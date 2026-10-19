import ts from "typescript";
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { dirname } from "node:path";

const [src, outJson, outTs] = process.argv.slice(2) as [string, string, string];
const text = readFileSync(src, "utf8");
const sf = ts.createSourceFile(src, text, ts.ScriptTarget.ES2022, true);
const strings = (n: ts.Node): string[] => {
  let e: ts.Node = n;
  while (ts.isAsExpression(e) || ts.isParenthesizedExpression(e)) e = e.expression;
  if (!ts.isArrayLiteralExpression(e)) throw Error("not an array: " + ts.SyntaxKind[e.kind]);
  return e.elements.map((x) => { if (!ts.isStringLiteral(x)) throw Error("not a string"); return x.text; });
};
let nativeDecl: ts.VariableStatement | undefined;
let lawsStmt: ts.VariableStatement | undefined;
const visit = (n: ts.Node) => {
  if (ts.isVariableStatement(n)) {
    for (const d of n.declarationList.declarations) {
      if (ts.isIdentifier(d.name) && d.name.text === "FEM2D_VERIFICATION_LAWS") nativeDecl = n;
      if (ts.isIdentifier(d.name) && d.name.text === "laws" && d.initializer && ts.isConditionalExpression(d.initializer)) lawsStmt = n;
    }
  }
  ts.forEachChild(n, visit);
};
visit(sf);
if (!nativeDecl || !lawsStmt) throw Error("anchors missing");
const nd = (nativeDecl as ts.VariableStatement).declarationList.declarations[0]!;
const native = strings(nd.initializer!);
const sets: Record<string, string[]> = {};
let cur = (lawsStmt as ts.VariableStatement).declarationList.declarations[0]!.initializer as ts.Expression;
while (ts.isConditionalExpression(cur)) {
  const c = cur.condition as ts.BinaryExpression;
  const key = (c.right as ts.StringLiteral).text;
  sets[key] = strings(cur.whenTrue);
  cur = cur.whenFalse;
}
if (!ts.isIdentifier(cur) || cur.text !== "FEM2D_VERIFICATION_LAWS") throw Error("tail is not the native list");
const out = { native, mounted: sets.mounted!, numerical: sets.numerical!, presence: sets.presence! };
for (const k of ["mounted", "numerical", "presence"]) if (!out[k as keyof typeof out]) throw Error("missing " + k);
mkdirSync(dirname(outJson), { recursive: true });
writeFileSync(outJson, JSON.stringify(out, null, 2) + "\n");
const ns = (nativeDecl as ts.VariableStatement);
const ls = (lawsStmt as ts.VariableStatement);
const a0 = ns.getStart(sf), a1 = ns.getEnd();
const b0 = ls.getStart(sf), b1 = ls.getEnd();
console.log(JSON.stringify({ native: native.length, mounted: out.mounted.length, numerical: out.numerical.length, presence: out.presence.length, nativeSpan: [sf.getLineAndCharacterOfPosition(a0).line + 1, sf.getLineAndCharacterOfPosition(a1).line + 1], lawsLine: sf.getLineAndCharacterOfPosition(b0).line + 1, lawsEnd: sf.getLineAndCharacterOfPosition(b1).line + 1 }));
