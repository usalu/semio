/**
 * 🏷️ One-shot fixture migration to the typed `PropertyValue` enum: `{"kind":"Text","text":"x"}` becomes `{"Text":{"value":"x"}}`
 * (booleans read their `flag`, every other kind its `number`) in every committed JSON document under the artifact subset.
 * Usage: `bun r6-z-mutations-property-value.ts <subset-dir> [--check]`. Documents with a kind/slot mismatch are reported, not changed.
 */
import { readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { field, object, parse, print, walk, type Node } from "./r6-z-mutations-rawjson.ts";

const KINDS = ["Text", "Real", "Integer", "Boolean", "Length", "Area", "Volume", "Angle"];
const SLOT: Record<string, string> = { Text: "text", Boolean: "flag" };
const root = process.argv[2];
const check = process.argv.includes("--check");
const problems: string[] = [];

function files(dir: string): string[] {
  return readdirSync(dir).flatMap((name) => {
    const path = join(dir, name);
    if (statSync(path).isDirectory()) return files(path);
    return name.endsWith(".json") ? [path] : [];
  });
}

let migrated = 0;
for (const path of files(root)) {
  const text = readFileSync(path, "utf8");
  if (!/"kind":\s*"(Text|Real|Integer|Boolean|Length|Area|Volume|Angle)"/.test(text)) continue;
  if (path.includes("schema") && /"\$schema"/.test(text)) continue;
  let changed = false;
  const next = walk(parse(text), (node: Node) => {
    if (node.k !== "obj") return undefined;
    const kind = field(node, "kind");
    if (!kind || kind.k !== "raw") return undefined;
    const name = kind.v.replaceAll('"', "");
    if (!KINDS.includes(name) || !node.v.every(([key]) => ["kind", "text", "number", "flag"].includes(key))) return undefined;
    const slot = SLOT[name] ?? "number";
    const value = field(node, slot);
    if (!value || node.v.length !== 2) {
      problems.push(`${path}: ${name} with slots ${node.v.map(([key]) => key).join(",")}`);
      return undefined;
    }
    changed = true;
    return object([name, object(["value", value])]);
  });
  if (changed) {
    migrated++;
    if (!check) writeFileSync(path, print(next) + "\n");
  }
}
console.log(`${check ? "would migrate" : "migrated"} ${migrated} documents; ${problems.length} problems`);
for (const problem of problems) console.log(problem);
