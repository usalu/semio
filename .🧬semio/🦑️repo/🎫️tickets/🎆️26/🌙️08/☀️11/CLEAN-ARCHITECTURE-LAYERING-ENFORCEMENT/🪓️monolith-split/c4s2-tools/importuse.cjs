// usage: node importuse.cjs <file> <module-specifier-substring>  : which top-level statements use bindings imported from the module
const fs = require("fs"), ts = require("/Users/ueli/Documents/semio/node_modules/typescript");
const [file, mod] = process.argv.slice(2);
const sf = ts.createSourceFile(file, fs.readFileSync(file, "utf8"), ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const names = new Set();
for (const s of sf.statements) if (ts.isImportDeclaration(s) && s.moduleSpecifier.text.includes(mod) && s.importClause) { const c = s.importClause; if (c.name) names.add(c.name.text); if (c.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) names.add(c.namedBindings.name.text); else c.namedBindings.elements.forEach((e) => names.add(e.name.text)); } }
console.log("bindings:", [...names].join(","));
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
for (const s of sf.statements) { if (ts.isImportDeclaration(s)) continue; const used = new Set(); const v = (n) => { if (ts.isIdentifier(n) && names.has(n.text)) used.add(n.text); ts.forEachChild(n, v); }; v(s);
  let nm = ts.isVariableStatement(s) ? s.declarationList.declarations.map((d) => d.name.getText(sf)).join(",") : s.name ? s.name.getText(sf) : ""; if (used.size) console.log(L(s.getStart(sf, true)) + "-" + L(s.getEnd()), nm, [...used].join(",")); }
