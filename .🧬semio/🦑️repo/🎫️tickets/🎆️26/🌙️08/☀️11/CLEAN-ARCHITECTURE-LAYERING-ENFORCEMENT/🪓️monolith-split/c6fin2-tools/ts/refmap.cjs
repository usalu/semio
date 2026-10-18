// 🕸️ usage: node refmap.cjs <file.mjs>: for every top-level declaration, which other top-level declarations of the file it references (names), plus uses of import.meta; JS module reference census for planning a split.
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const fs = require("fs");
const file = process.argv[2];
const text = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
const decl = new Map();
const rows = [];
for (const s of sf.statements) {
  let names = [];
  if (ts.isVariableStatement(s)) {
    for (const d of s.declarationList.declarations) {
      const walk = (b) => { if (ts.isIdentifier(b)) names.push(b.text); else if (ts.isObjectBindingPattern(b) || ts.isArrayBindingPattern(b)) b.elements.forEach((e) => { if (ts.isBindingElement(e)) walk(e.name); }); };
      walk(d.name);
    }
  } else if ((ts.isFunctionDeclaration(s) || ts.isClassDeclaration(s)) && s.name) names = [s.name.text];
  else if (ts.isImportDeclaration(s)) {
    const c = s.importClause;
    if (c) { if (c.name) names.push(c.name.text); if (c.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) names.push(c.namedBindings.name.text); else c.namedBindings.elements.forEach((e) => names.push(e.name.text)); } }
  }
  const row = { s, names, a: L(s.getStart(sf, true)), b: L(s.getEnd()), kind: ts.SyntaxKind[s.kind], imp: ts.isImportDeclaration(s) };
  rows.push(row);
  for (const n of names) decl.set(n, row);
}
for (const r of rows) {
  if (r.imp) continue;
  const used = new Set(); let meta = false;
  const visit = (n) => {
    if (ts.isIdentifier(n) && decl.has(n.text) && decl.get(n.text) !== r && !(ts.isPropertyAccessExpression(n.parent) && n.parent.name === n) && !(ts.isPropertyAssignment(n.parent) && n.parent.name === n)) used.add(n.text);
    if (ts.isMetaProperty(n)) meta = true;
    ts.forEachChild(n, visit);
  };
  visit(r.s);
  const own = r.names.join(",") || r.s.getText(sf).split("\n")[0].slice(0, 50);
  const dd = [...used].filter((u) => !decl.get(u).imp);
  console.log(`${r.a}-${r.b}\t${r.b - r.a + 1}\t${own.slice(0, 60)}\t${meta ? "[import.meta] " : ""}${dd.join(" ")}`);
}
