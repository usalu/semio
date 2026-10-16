import ts from "typescript";
import { readFileSync } from "node:fs";

const file = process.argv[2]!;
const text = readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const fn = sf.statements.find((s) => ts.isFunctionDeclaration(s) && s.name?.text === "registerTests1") as ts.FunctionDeclaration;
const body = fn.body!.statements;

const declared = new Map<string, number>();
const destructured = new Set<string>();
const declNames = (s: ts.Statement): string[] => {
  const out: string[] = [];
  if (ts.isVariableStatement(s)) for (const d of s.declarationList.declarations) {
    if (ts.isIdentifier(d.name)) out.push(d.name.text);
    else if (ts.isObjectBindingPattern(d.name)) for (const e of d.name.elements) if (ts.isIdentifier(e.name)) out.push(e.name.text);
  }
  else if ((ts.isFunctionDeclaration(s) || ts.isTypeAliasDeclaration(s) || ts.isClassDeclaration(s)) && s.name) out.push(s.name.text);
  return out;
};
body.forEach((s, i) => { for (const n of declNames(s)) declared.set(n, i); });
for (const s of sf.statements) if (ts.isImportDeclaration(s) && s.importClause) {
  const c = s.importClause;
  if (c.name) declared.set(c.name.text, -1);
  if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const e of c.namedBindings.elements) declared.set(e.name.text, -1);
}
for (const s of sf.statements) if (ts.isTypeAliasDeclaration(s)) declared.set(s.name.text, -1);

const idents = (n: ts.Node, out: Set<string>) => {
  if (ts.isIdentifier(n)) {
    const p = n.parent;
    const isProp = (ts.isPropertyAccessExpression(p) && p.name === n) || (ts.isPropertyAssignment(p) && p.name === n) || (ts.isQualifiedName(p) && p.right === n);
    if (!isProp) out.add(n.text);
  }
  n.forEachChild((c) => idents(c, out));
};
const line = (pos: number) => sf.getLineAndCharacterOfPosition(pos).line + 1;

const rows: any[] = [];
body.forEach((s, i) => {
  const used = new Set<string>();
  idents(s, used);
  const own = new Set(declNames(s));
  let title = "";
  if (ts.isExpressionStatement(s) && ts.isCallExpression(s.expression)) {
    const a = s.expression.arguments[0];
    title = s.expression.expression.getText(sf) + (a && ts.isStringLiteralLike(a) ? " " + a.text : "");
  }
  rows.push({ i, start: line(s.getStart(sf)), end: line(s.end), kind: ts.SyntaxKind[s.kind], title, own: [...own], uses: [...used].filter((u) => declared.has(u) && !own.has(u)).sort() });
});
console.log(JSON.stringify(rows, null, 0).replace(/\},\{/g, "},\n{"));
