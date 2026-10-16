import fs from "node:fs";
import path from "node:path";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
const [src, dir, out] = process.argv.slice(2);
const sf = ts.createSourceFile(src, fs.readFileSync(src, "utf8"), ts.ScriptTarget.ESNext, true, ts.ScriptKind.TS);
const map = {};
for (const st of sf.statements) if (ts.isExportDeclaration(st) && st.moduleSpecifier && st.exportClause && ts.isNamedExports(st.exportClause)) for (const el of st.exportClause.elements) map[el.name.text] = path.resolve(dir, st.moduleSpecifier.text);
fs.writeFileSync(out, JSON.stringify(map, null, 1));
console.log(Object.keys(map).length, "re-exported names");
