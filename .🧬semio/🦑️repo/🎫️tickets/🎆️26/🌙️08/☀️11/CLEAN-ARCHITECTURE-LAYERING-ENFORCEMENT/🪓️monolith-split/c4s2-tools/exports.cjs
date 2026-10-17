// usage: node exports.cjs <snapshot> <new-file...>  : every name exported by the snapshot is exported by exactly one new file
const fs = require("fs"), ts = require("/Users/ueli/Documents/semio/node_modules/typescript");
const names = (p) => { const sf = ts.createSourceFile(p, fs.readFileSync(p, "utf8"), ts.ScriptTarget.Latest, true, p.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS); const out = new Set();
  for (const s of sf.statements) {
    const exp = ts.canHaveModifiers(s) && (ts.getModifiers(s) || []).some((m) => m.kind === ts.SyntaxKind.ExportKeyword);
    if (exp) { if (ts.isVariableStatement(s)) s.declarationList.declarations.forEach((d) => out.add(d.name.getText(sf))); else if (s.name) out.add(s.name.text); }
    if (ts.isExportDeclaration(s) && s.exportClause && ts.isNamedExports(s.exportClause)) s.exportClause.elements.forEach((e) => out.add(e.name.text));
  } return out; };
const [snap, ...news] = process.argv.slice(2);
const orig = names(snap); const where = new Map();
for (const f of news) for (const n of names(f)) where.set(n, [...(where.get(n) || []), f]);
let bad = 0;
for (const n of orig) { const w = where.get(n) || []; if (w.length !== 1) { bad++; console.log("PROBLEM", n, w.length); } }
console.log(`orig exports ${orig.size}, new files ${news.length}, problems ${bad}`);
