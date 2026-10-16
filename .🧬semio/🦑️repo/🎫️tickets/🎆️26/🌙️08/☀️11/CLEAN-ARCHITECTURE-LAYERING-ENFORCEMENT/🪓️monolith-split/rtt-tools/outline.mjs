import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import fs from "node:fs";
const file = process.argv[2];
const src = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, src, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const line = (pos) => sf.getLineAndCharacterOfPosition(pos).line + 1;
const name = (s) => {
  if (ts.isFunctionDeclaration(s) || ts.isClassDeclaration(s) || ts.isInterfaceDeclaration(s) || ts.isTypeAliasDeclaration(s) || ts.isEnumDeclaration(s)) return s.name?.text;
  if (ts.isVariableStatement(s)) return s.declarationList.declarations.map((d) => d.name.getText()).join(",").slice(0, 60);
  if (ts.isExpressionStatement(s)) return s.expression.getText().slice(0, 70).replace(/\n/g, " ");
  if (ts.isImportDeclaration(s)) return s.moduleSpecifier.getText().slice(-50);
  return "";
};
const mode = process.argv[3] ?? "top";
if (mode === "top") {
  for (const s of sf.statements) {
    console.log(`${line(s.getStart())}-${line(s.end)} ${ts.SyntaxKind[s.kind]} ${name(s)}`);
  }
} else {
  const fnName = process.argv[4];
  const fn = sf.statements.find((s) => ts.isFunctionDeclaration(s) && s.name?.text === fnName);
  for (const s of fn.body.statements) {
    let extra = "";
    if (ts.isExpressionStatement(s) && ts.isCallExpression(s.expression)) {
      const a = s.expression.arguments[0];
      extra = a && (ts.isStringLiteralLike(a) || ts.isTemplateLiteral(a)) ? a.getText().slice(0, 100) : "";
    }
    console.log(`${line(s.getStart())}-${line(s.end)} ${ts.SyntaxKind[s.kind]} ${name(s)} ${extra}`);
  }
}
