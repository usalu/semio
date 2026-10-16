import ts from "typescript";
import { readFileSync } from "node:fs";
const text = readFileSync(process.argv[2]!, "utf8");
const sf = ts.createSourceFile("x.ts", text, ts.ScriptTarget.Latest, true);
const line = (p: number) => sf.getLineAndCharacterOfPosition(p).line + 1;
const lets = new Set<string>();
for (const s of sf.statements) if (ts.isVariableStatement(s) && (s.declarationList.flags & ts.NodeFlags.Let)) for (const d of s.declarationList.declarations) if (ts.isIdentifier(d.name)) lets.add(d.name.text);
const topName = (n: ts.Node): string => {
  let cur: ts.Node = n;
  while (cur.parent && cur.parent !== sf) cur = cur.parent;
  const st = cur as ts.Statement;
  if (ts.isVariableStatement(st)) return st.declarationList.declarations.map((d) => d.name.getText(sf)).join(",");
  if ((ts.isFunctionDeclaration(st) || ts.isClassDeclaration(st)) && st.name) return st.name.text;
  return `<${ts.SyntaxKind[st.kind]}@${line(st.getStart(sf))}>`;
};
const writers = new Map<string, Set<string>>();
const visit = (n: ts.Node) => {
  if (ts.isBinaryExpression(n) && n.operatorToken.kind >= ts.SyntaxKind.FirstAssignment && n.operatorToken.kind <= ts.SyntaxKind.LastAssignment && ts.isIdentifier(n.left) && lets.has(n.left.text)) {
    (writers.get(n.left.text) ?? writers.set(n.left.text, new Set()).get(n.left.text)!).add(topName(n) + `@${line(n.getStart(sf))}`);
  }
  if ((ts.isPrefixUnaryExpression(n) || ts.isPostfixUnaryExpression(n)) && (n.operator === ts.SyntaxKind.PlusPlusToken || n.operator === ts.SyntaxKind.MinusMinusToken) && ts.isIdentifier(n.operand) && lets.has(n.operand.text)) {
    (writers.get(n.operand.text) ?? writers.set(n.operand.text, new Set()).get(n.operand.text)!).add(topName(n) + `@${line(n.getStart(sf))}`);
  }
  n.forEachChild(visit);
};
visit(sf);
for (const v of lets) console.log(v.padEnd(18), [...(writers.get(v) ?? [])].join("  "));
