#!/usr/bin/env bun
import { existsSync, mkdirSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = join(import.meta.dir, "../../../../../..");
const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");
const cadPkg = join(pluginsRoot, "📐️cad/📦️packages/🟦️typescript/package.json");

function listDirs(dir) {
  if (!existsSync(dir)) return [];
  return readdirSync(dir).filter((n) => statSync(join(dir, n)).isDirectory());
}

const template = existsSync(cadPkg) ? JSON.parse(readFileSync(cadPkg, "utf8")) : null;
let created = 0;
for (const plugin of listDirs(pluginsRoot)) {
  const pkgDir = join(pluginsRoot, plugin, "📦️packages/🟦️typescript");
  if (existsSync(join(pkgDir, "package.json"))) continue;
  mkdirSync(pkgDir, { recursive: true });
  const id = plugin.replace(/[^\x00-\x7f]/g, "") || "plugin";
  const pkg = template
    ? { ...template, name: `@semio-tech/${id}-js`, repository: { ...template.repository, directory: `✏️s/🔌️plugins/${plugin}/📦️packages/🟦️typescript` } }
    : { name: `@semio-tech/${id}-js`, type: "module", exports: { ".": "./📦️index.ts" } };
  writeFileSync(join(pkgDir, "package.json"), JSON.stringify(pkg, null, 2));
  writeFileSync(join(pkgDir, "📦️index.ts"), `/** ${id} facet WASM facades — re-export artifact 🟦️component.ts leaves. */\nexport {};\n`);
  created++;
}
console.log(`[DEBUG] scaffold-plugin-ts-packages created ${created} packages`);
