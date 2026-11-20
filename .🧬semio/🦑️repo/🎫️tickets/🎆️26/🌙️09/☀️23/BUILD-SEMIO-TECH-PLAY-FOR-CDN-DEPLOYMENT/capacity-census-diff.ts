import { readFileSync } from "node:fs";
import ts from "typescript";
/** 🧮️ Prints capacity source census rows that differ from the frozen fixture. Usage: capacity-census-diff.ts <fixture.json> */
const fixture = JSON.parse(readFileSync(process.argv[2]!, "utf8")), names = ["withCompilerCapacityScopeV1", "acquireCompilerCapacityLeaseV1", "withCompilerCapacityConsumptionV1"];
for (const [i, path] of (fixture.callers as string[]).entries()) {
  const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true), forwarded: { callee: string; argument: string }[] = [];
  let literalCalls = 0;
  const walk = (node: ts.Node) => { if (ts.isCallExpression(node) && names.includes(node.expression.getText(source))) { const options = node.arguments[0]; if (options && ts.isObjectLiteralExpression(options)) literalCalls++; else forwarded.push({ callee: node.expression.getText(source), argument: options?.getText(source) ?? "" }); } ts.forEachChild(node, walk); };
  walk(source);
  const row = { path, literalCalls, forwarded };
  if (JSON.stringify(row) !== JSON.stringify(fixture.census.rows[i])) console.log(JSON.stringify({ observed: row, frozen: fixture.census.rows[i] }));
}
