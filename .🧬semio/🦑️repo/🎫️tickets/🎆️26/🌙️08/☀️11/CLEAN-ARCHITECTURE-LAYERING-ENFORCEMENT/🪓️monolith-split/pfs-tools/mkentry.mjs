// usage: node mkentry.mjs <map.json> <original.txt> <entry-out> <header-text-file>   writes an entry that re-exports exactly the exported names of the original, per child, in original order
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import fs from "node:fs";
import path from "node:path";
const R = "/Users/ueli/Documents/semio";
const [mapPath, origPath, outPath, headerPath] = process.argv.slice(2);
const map = JSON.parse(fs.readFileSync(mapPath, "utf8"));
const text = fs.readFileSync(origPath, "utf8");
const sf = ts.createSourceFile("x.ts", text, ts.ScriptTarget.Latest, true);
const groups = new Map(Object.keys(map.children).map((c) => [c, []]));
const outDir = path.dirname(path.resolve(R, map.file));
for (const s of sf.statements) {
  if (!(ts.getCombinedModifierFlags(s) & ts.ModifierFlags.Export)) continue;
  const typeOnly = ts.isInterfaceDeclaration(s) || ts.isTypeAliasDeclaration(s);
  const names = ts.isVariableStatement(s) ? s.declarationList.declarations.map((d) => d.name.text) : [s.name.text];
  for (const n of names) {
    const dest = map.names[n];
    const child = Object.entries(map.children).find(([, p]) => p === dest)?.[0];
    if (!child) throw new Error("no child for " + n);
    groups.get(child).push(typeOnly ? "type " + n : n);
  }
}
let out = fs.readFileSync(headerPath, "utf8").replace(/\s+$/, "") + "\n\n";
let total = 0;
for (const [child, items] of groups) {
  if (!items.length) continue;
  const rel = path.relative(outDir, path.resolve(R, map.children[child])).split(path.sep).join("/");
  out += `export { ${items.join(", ")} } from "./${rel}";\n`;
  total += items.length;
}
fs.writeFileSync(outPath, out);
console.log("entry names:", total);
