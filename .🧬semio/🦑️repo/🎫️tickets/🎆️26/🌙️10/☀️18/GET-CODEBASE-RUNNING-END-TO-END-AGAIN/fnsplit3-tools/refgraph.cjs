// usage: node refgraph.cjs <file> <functionName> : depth-1 statements of the function with the other statements they reference (forward refs flagged)
const fs = require("fs"), path = require("path");
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const file = process.argv[2], fname = process.argv[3];
const text = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
let fn = null;
const find = (n) => { if (fn) return; if (ts.isFunctionDeclaration(n) && n.name && n.name.text === fname) { fn = n; return; } ts.forEachChild(n, find); };
find(sf);
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
const stmts = fn.body.statements;
const declared = new Map();
stmts.forEach((s, i) => {
  const names = [];
  const bn = (n) => { if (ts.isIdentifier(n)) names.push(n.text); else if (n.elements) n.elements.forEach((e) => !ts.isOmittedExpression(e) && bn(e.name)); };
  if (ts.isVariableStatement(s)) s.declarationList.declarations.forEach((d) => bn(d.name));
  else if ((ts.isFunctionDeclaration(s) || ts.isClassDeclaration(s)) && s.name) names.push(s.name.text);
  s._names = names; names.forEach((n) => declared.set(n, i));
});
stmts.forEach((s, i) => {
  const refs = new Set();
  const w = (n) => { if (ts.isIdentifier(n) && declared.has(n.text) && !(ts.isPropertyAccessExpression(n.parent) && n.parent.name === n) && !(ts.isPropertyAssignment(n.parent) && n.parent.name === n)) { const j = declared.get(n.text); if (j !== i) refs.add(j); } ts.forEachChild(n, w); };
  w(s);
  const fwd = [...refs].filter((j) => j > i);
  console.log(`${String(i).padStart(2)} L${L(s.getStart(sf))}-${L(s.getEnd())} (${L(s.getEnd()) - L(s.getStart(sf)) + 1}) ${s._names.join(",") || ts.SyntaxKind[s.kind]}  -> back [${[...refs].filter((j) => j < i).map((j) => stmts[j]._names.join("+") || j)}]${fwd.length ? "  FWD [" + fwd.map((j) => stmts[j]._names.join("+")) + "]" : ""}`);
});
