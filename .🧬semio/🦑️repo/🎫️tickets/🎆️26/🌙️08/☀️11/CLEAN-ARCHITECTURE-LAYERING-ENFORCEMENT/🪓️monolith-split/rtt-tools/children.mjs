import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import fs from "node:fs";
const [file, startLine] = process.argv.slice(2);
const src = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, src, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const line = (pos) => sf.getLineAndCharacterOfPosition(pos).line + 1;
function find(n) {
  if (ts.isExpressionStatement(n) && line(n.getStart()) === Number(startLine)) return n;
  return ts.forEachChild(n, find);
}
const stmt = find(sf);
const call = stmt.expression;
const cb = call.arguments.find((a) => ts.isArrowFunction(a) || ts.isFunctionExpression(a));
for (const s of cb.body.statements) {
  let extra = "";
  if (ts.isExpressionStatement(s) && ts.isCallExpression(s.expression)) {
    const a = s.expression.arguments[0];
    extra = s.expression.expression.getText().slice(0, 30) + " " + (a && (ts.isStringLiteralLike(a) || ts.isTemplateLiteral(a)) ? a.getText().slice(0, 110) : "");
  } else if (ts.isVariableStatement(s)) extra = s.declarationList.declarations.map((d) => d.name.getText().slice(0, 40)).join(",");
  else if (ts.isFunctionDeclaration(s)) extra = s.name?.text;
  console.log(`${line(s.getStart())}-${line(s.end)} ${ts.SyntaxKind[s.kind]} ${extra}`);
}
