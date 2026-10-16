import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import fs from "node:fs";
const [file, startLine, depth = "2"] = process.argv.slice(2);
const src = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, src, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const line = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
const callName = (n) => ts.isExpressionStatement(n) && ts.isCallExpression(n.expression) ? n.expression.expression.getText(sf) : null;
function walk(n, d, ind) {
  if (!ts.isExpressionStatement(n) && !ts.isVariableStatement(n) && !ts.isFunctionDeclaration(n)) { ts.forEachChild(n, (c) => walk(c, d, ind)); return; }
  const cn = callName(n);
  if (ts.isExpressionStatement(n) && cn && /^(describe|it|test|beforeEach|afterEach|describe\.each)/.test(cn)) {
    const a = n.expression.arguments[0];
    console.log(`${ind}${line(n.getStart(sf))}-${line(n.end)} ${cn} ${a ? a.getText(sf).slice(0, 90) : ""}`);
    if (d > 1 && /^describe/.test(cn)) { const fn = n.expression.arguments.find((x) => ts.isArrowFunction(x) || ts.isFunctionExpression(x)); if (fn && fn.body && ts.isBlock(fn.body)) for (const s of fn.body.statements) walk(s, d - 1, ind + "  "); }
  } else if (ts.isVariableStatement(n) || ts.isFunctionDeclaration(n)) {
    console.log(`${ind}${line(n.getStart(sf))}-${line(n.end)} ${ts.isFunctionDeclaration(n) ? "function " + n.name?.text : "var " + n.declarationList.declarations.map((x) => x.name.getText(sf)).join(",").slice(0, 60)}`);
  }
}
const fn = sf.statements.find((s) => ts.isFunctionDeclaration(s) && line(s.getStart(sf)) >= +startLine);
for (const s of fn.body.statements) walk(s, +depth, "");
