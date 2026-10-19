import { readFileSync } from "node:fs";
import ts from "typescript";
const file = process.argv[2]!, text = readFileSync(file, "utf8"), source = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true);
const out: string[] = [];
for (const node of source.statements) {
  if (!ts.isExpressionStatement(node) || !ts.isCallExpression(node.expression)) continue;
  const call = node.expression;
  if (!ts.isIdentifier(call.expression) || call.expression.text !== "test" || call.arguments.length < 2) continue;
  const first = call.arguments[0]!;
  if (ts.isStringLiteralLike(first) && /withBuildTestInvocationV1/.test(call.arguments[1]!.getText(source))) out.push(first.text);
}
console.log(JSON.stringify(out));
