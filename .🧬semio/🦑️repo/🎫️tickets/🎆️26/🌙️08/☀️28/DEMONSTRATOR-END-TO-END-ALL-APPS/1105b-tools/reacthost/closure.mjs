import { readFileSync, existsSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
const root = "/Users/ueli/Documents/semio";
const tax = JSON.parse(readFileSync(`${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`, "utf8"));
const set = new Set(tax.generatorContracts["wgpu-frame-worker"].packageGeneration.browserProfile.sourceModulePaths);
const added = [];
const queue = [...set];
while (queue.length) {
  const path = queue.pop();
  if (!/\.(ts|mjs)$/u.test(path)) continue;
  const emitted = ts.transpileModule(readFileSync(`${root}/${path}`, "utf8"), { compilerOptions: { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext } }).outputText;
  const source = ts.createSourceFile(path, emitted, ts.ScriptTarget.Latest, true);
  for (const d of source.statements) {
    if (!(ts.isImportDeclaration(d) || ts.isExportDeclaration(d)) || !d.moduleSpecifier || !ts.isStringLiteral(d.moduleSpecifier) || !d.moduleSpecifier.text.startsWith(".")) continue;
    const target = relative(root, resolve(root, dirname(path), d.moduleSpecifier.text)).replaceAll("\\", "/");
    if (!set.has(target)) { if (!existsSync(`${root}/${target}`)) { console.error("MISSING FILE", target, "from", path); continue; } set.add(target); added.push(target); queue.push(target); }
  }
}
console.log(JSON.stringify(added));
