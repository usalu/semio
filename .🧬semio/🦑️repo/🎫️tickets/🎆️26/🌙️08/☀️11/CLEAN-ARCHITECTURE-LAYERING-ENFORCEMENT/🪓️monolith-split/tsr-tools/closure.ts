import ts from "typescript";
import { readFileSync, existsSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
const root = "/Users/ueli/Documents/semio";
const tax = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8"));
const profile = tax.generatorContracts["wgpu-frame-worker"].packageGeneration.browserProfile;
const listed: string[] = profile.sourceModulePaths;
const set = new Set(listed);
const missing: string[] = [];
const queue = [...listed];
const seen = new Set<string>();
while (queue.length) {
  const path = queue.shift()!;
  if (seen.has(path)) continue;
  seen.add(path);
  if (!/\.(ts|mjs)$/u.test(path)) continue;
  if (!existsSync(join(root, path))) { console.log("MISSING FILE", path); continue; }
  const emitted = ts.transpileModule(readFileSync(join(root, path), "utf8"), { compilerOptions: { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext } }).outputText;
  const source = ts.createSourceFile(path, emitted, ts.ScriptTarget.Latest, true);
  for (const d of source.statements) {
    if (!(ts.isImportDeclaration(d) || ts.isExportDeclaration(d)) || !d.moduleSpecifier || !ts.isStringLiteral(d.moduleSpecifier) || !d.moduleSpecifier.text.startsWith(".")) continue;
    const target = relative(root, resolve(root, dirname(path), d.moduleSpecifier.text)).replaceAll("\\", "/");
    if (!set.has(target)) { set.add(target); missing.push(target + "   <= " + path); queue.push(target); }
    else queue.push(target);
  }
}
console.log("listed", listed.length, "missing", missing.length);
for (const m of missing) console.log(m);
