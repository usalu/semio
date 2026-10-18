// usage: node live.cjs <file> <fn> [minSpanLines]: long-lived body-level names (decl line, last use line, last use stmt head)
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const fs = require("fs");
const [file, name, minSpan = "150"] = process.argv.slice(2);
const text = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
const fn = sf.statements.find((s) => ts.isFunctionDeclaration(s) && s.name && s.name.text === name);
const stmts = fn.body.statements;
const decl = new Map();
stmts.forEach((s, i) => {
  if (ts.isVariableStatement(s)) for (const d of s.declarationList.declarations) { const walk = (n) => { if (ts.isIdentifier(n)) decl.set(n.text, i); else if (ts.isObjectBindingPattern(n) || ts.isArrayBindingPattern(n)) n.elements.forEach((e) => { if (!ts.isOmittedExpression(e)) walk(e.name); }); }; walk(d.name); }
});
const last = new Map();
stmts.forEach((s, i) => { const visit = (n) => { if (ts.isIdentifier(n) && decl.has(n.text) && decl.get(n.text) !== i) last.set(n.text, i); ts.forEachChild(n, visit); }; visit(s); });
for (const [n, i] of decl) { const l = last.get(n); if (l === undefined) { console.log(`UNUSED ${n} @${L(stmts[i].getStart(sf))}`); continue; } const a = L(stmts[i].getStart(sf)), b = L(stmts[l].getStart(sf)); if (b - a >= +minSpan) console.log(`${n}\t${a} -> ${b}`); }
