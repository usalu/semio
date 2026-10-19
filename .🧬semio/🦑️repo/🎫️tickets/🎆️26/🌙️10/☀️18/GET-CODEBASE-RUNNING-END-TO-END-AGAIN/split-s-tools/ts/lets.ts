import { analyze, ts } from "./ast.ts";
const a = analyze(process.argv[2]);
const lets = new Set<string>();
for (const st of a.sf.statements) if (ts.isVariableStatement(st) && (st.declarationList.flags & ts.NodeFlags.Let)) for (const d of st.declarationList.declarations) if (ts.isIdentifier(d.name)) lets.add(d.name.text);
const writes = new Map<string, Set<string>>();
const visit = (n: import("typescript").Node, owner: string) => {
  const target = (e: import("typescript").Expression) => ts.isIdentifier(e) && lets.has(e.text) ? e.text : undefined;
  if (ts.isBinaryExpression(n) && n.operatorToken.kind >= ts.SyntaxKind.FirstAssignment && n.operatorToken.kind <= ts.SyntaxKind.LastAssignment) { const t = target(n.left); if (t) (writes.get(t) ?? writes.set(t, new Set()).get(t)!).add(owner); }
  if ((ts.isPrefixUnaryExpression(n) || ts.isPostfixUnaryExpression(n)) && (n.operator === ts.SyntaxKind.PlusPlusToken || n.operator === ts.SyntaxKind.MinusMinusToken)) { const t = target(n.operand); if (t) (writes.get(t) ?? writes.set(t, new Set()).get(t)!).add(owner); }
  ts.forEachChild(n, (c) => visit(c, owner));
};
a.decls.forEach((d) => visit(a.sf.statements[d.index], d.names.join(",") || "stmt@" + d.startLine));
for (const l of lets) console.log(l, "writers:", [...(writes.get(l) ?? [])].join(" | "));
