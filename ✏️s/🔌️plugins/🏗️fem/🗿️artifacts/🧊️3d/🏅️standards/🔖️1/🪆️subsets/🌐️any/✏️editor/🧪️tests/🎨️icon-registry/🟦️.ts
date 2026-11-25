/** 🎨️ Every icon id the FEM 3D editor authors resolves in the generated framework icon registry, so no row or action paints its raw id as text (F12: `ruler`, and the 2D `mouse-pointer-click`/`timer`). */
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join, resolve } from "node:path";
import { expect, it } from "vitest";

const editorRoot = resolve(__dirname, "../..");
const registrySource = join(__dirname, "../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🟦️icons.ts");

function rustSources(directory: string): string[] {
  return readdirSync(directory).flatMap((name) => {
    const path = join(directory, name);
    if (name === "🧪️tests" || name === "node_modules" || name === "target") return [];
    return statSync(path).isDirectory() ? rustSources(path) : name.endsWith(".rs") ? [path] : [];
  });
}

const ICON_LITERALS = [
  /ActionKind::\w+,\s*"([a-z0-9-]+)"/g,
  /granularity\(\s*[A-Z0-9_]+,\s*"[^"]*",\s*"[^"]*",\s*"([a-z0-9-]+)"/g,
  /entity_row\([^;]*?,\s*"([a-z0-9-]+)",\s*(?:false|true|[a-z_()&.]+)\)/g,
  /entity_item\([^;]*?,\s*"([a-z0-9-]+)",\s*(?:false|true|[a-z_()&.]+)\)/g,
  /icon_id\("([a-z0-9-]+)"\)/g,
  /icon_text\("([a-z0-9-]+)"\)/g,
  /\.icon\(ui_text\("([a-z0-9-]+)"\)/g,
];

it("resolves every authored FEM 3D icon id in the generated icon registry", () => {
  const registry = new Set([...readFileSync(registrySource, "utf8").matchAll(/^ {2}"([a-z0-9-]+)": `/gm)].map((match) => match[1]!));
  expect(registry.size).toBeGreaterThan(100);
  const used = new Map<string, string>();
  for (const file of rustSources(editorRoot)) {
    const source = readFileSync(file, "utf8");
    for (const pattern of ICON_LITERALS) for (const match of source.matchAll(pattern)) used.set(match[1]!, file);
  }
  expect(used.size).toBeGreaterThan(10);
  expect([...used.keys()].filter((id) => !registry.has(id)).sort()).toEqual([]);
});
