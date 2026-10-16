import ts from "typescript";
import { readFileSync } from "node:fs";
const [file, startLine] = [process.argv[2]!, Number(process.argv[3])];
const text = readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const line = (pos: number) => sf.getLineAndCharacterOfPosition(pos).line + 1;
let target: ts.Block | null = null;
const find = (n: ts.Node) => {
  if (ts.isCallExpression(n) && line(n.getStart(sf)) === startLine) { const cb = n.arguments[n.arguments.length - 1]; if (cb && (ts.isArrowFunction(cb) || ts.isFunctionExpression(cb)) && ts.isBlock(cb.body)) target = cb.body; }
  n.forEachChild(find);
};
find(sf);
const body = (target as ts.Block | null)!.statements;
const names = (s: ts.Statement): string[] => {
  const out: string[] = [];
  if (ts.isVariableStatement(s)) for (const d of s.declarationList.declarations) { if (ts.isIdentifier(d.name)) out.push(d.name.text); else if (ts.isObjectBindingPattern(d.name)) for (const e of d.name.elements) if (ts.isIdentifier(e.name)) out.push(e.name.text); }
  else if ((ts.isFunctionDeclaration(s) || ts.isTypeAliasDeclaration(s) || ts.isClassDeclaration(s)) && s.name) out.push(s.name.text);
  return out;
};
const declared = new Map<string, number>();
body.forEach((s, i) => names(s).forEach((n) => declared.set(n, i)));
const idents = (n: ts.Node, out: Set<string>) => { if (ts.isIdentifier(n)) { const p = n.parent; if (!((ts.isPropertyAccessExpression(p) && p.name === n) || (ts.isPropertyAssignment(p) && p.name === n))) out.add(n.text); } n.forEachChild((c) => idents(c, out)); };
body.forEach((s, i) => {
  const used = new Set<string>(); idents(s, used);
  const own = names(s);
  let title = "";
  if (ts.isExpressionStatement(s) && ts.isCallExpression(s.expression)) { const a = s.expression.arguments[0]; title = s.expression.expression.getText(sf) + " " + (a && ts.isStringLiteralLike(a) ? a.text.slice(0, 70) : ""); }
  console.log(`${i}\t${line(s.getStart(sf))}-${line(s.end)}\t${title || own.join(",")}\tuses-local: ${[...used].filter((u) => declared.has(u) && !own.includes(u)).join(",")}`);
});
