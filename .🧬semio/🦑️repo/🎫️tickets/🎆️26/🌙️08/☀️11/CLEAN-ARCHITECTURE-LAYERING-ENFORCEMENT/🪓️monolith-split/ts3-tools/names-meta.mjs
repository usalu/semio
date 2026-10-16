// usage: node names-meta.mjs <snapshot.ts.txt> <map.json> <out.json> -> name -> {exported, type, file}
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import fs from "node:fs";
const [snap, mapPath, out] = process.argv.slice(2);
const map = JSON.parse(fs.readFileSync(mapPath, "utf8"));
const text = fs.readFileSync(snap, "utf8");
const sf = ts.createSourceFile("x.ts", text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const meta = {};
for (const s of sf.statements) {
  const exported = !!(ts.getCombinedModifierFlags(s) & ts.ModifierFlags.Export);
  const type = ts.isInterfaceDeclaration(s) || ts.isTypeAliasDeclaration(s);
  const names = ts.isVariableStatement(s) ? s.declarationList.declarations.map((d) => d.name.getText(sf)) : s.name ? [s.name.getText(sf)] : [];
  for (const n of names) meta[n] = { exported, type, file: map.names[n] };
}
fs.writeFileSync(out, JSON.stringify(meta, null, 1));
console.log(Object.keys(meta).length, "names;", Object.values(meta).filter((m) => m.exported).length, "exported");
