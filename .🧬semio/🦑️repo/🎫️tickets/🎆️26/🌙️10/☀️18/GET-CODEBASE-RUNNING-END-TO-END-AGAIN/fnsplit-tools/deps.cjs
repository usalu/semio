// usage: node deps.cjs <file> <function-name>: depth-1 statements of the function body with declared names and used body-level names
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const fs = require("fs");
const [file, name] = process.argv.slice(2);
const text = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
const fn = sf.statements.find((s) => ts.isFunctionDeclaration(s) && s.name && s.name.text === name);
const stmts = fn.body.statements;
const declared = new Map();
const decls = stmts.map((s, i) => {
  const names = [];
  if (ts.isVariableStatement(s)) for (const d of s.declarationList.declarations) { const walk = (n) => { if (ts.isIdentifier(n)) names.push(n.text); else if (ts.isObjectBindingPattern(n) || ts.isArrayBindingPattern(n)) n.elements.forEach((e) => { if (!ts.isOmittedExpression(e)) walk(e.name); }); }; walk(d.name); }
  if (ts.isFunctionDeclaration(s) && s.name) names.push(s.name.text);
  for (const n of names) declared.set(n, i);
  return names;
});
const rows = stmts.map((s, i) => {
  const used = new Set();
  const visit = (n) => { if (ts.isIdentifier(n) && declared.has(n.text) && declared.get(n.text) !== i) { const p = n.parent; if (!(ts.isPropertyAccessExpression(p) && p.name === n) && !(ts.isPropertyAssignment(p) && p.name === n)) used.add(n.text); } ts.forEachChild(n, visit); };
  visit(s);
  return { a: L(s.getStart(sf, true)), b: L(s.getEnd()), kind: ts.SyntaxKind[s.kind].replace("Declaration", ""), decl: decls[i], used: [...used], head: s.getText(sf).split("\n")[0].slice(0, 90) };
});
for (const r of rows) console.log(`${r.a}-${r.b}\t${r.b - r.a + 1}\t${r.kind}\tdecl=[${r.decl}]\tuse=[${r.used.map((u) => u + "@" + (stmts.length && declared.get(u))).join(",")}]\t${r.head}`);
