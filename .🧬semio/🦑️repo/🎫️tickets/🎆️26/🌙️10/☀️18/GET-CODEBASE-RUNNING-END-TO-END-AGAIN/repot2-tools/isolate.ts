// usage: bun isolate.ts <test-file> <test name>...   rewrites top-level test("name", fn[, timeout]) into registerIsolatedBuildTestV1(import.meta.path, "name", timeout, fn)
import { readFileSync, writeFileSync } from "node:fs";
import ts from "typescript";
const [file, ...names] = process.argv.slice(2);
const text = readFileSync(file!, "utf8"), source = ts.createSourceFile(file!, text, ts.ScriptTarget.Latest, true);
const edits: { start: number; end: number; text: string }[] = [];
for (const node of source.statements) {
  if (!ts.isExpressionStatement(node) || !ts.isCallExpression(node.expression)) continue;
  const call = node.expression;
  if (!ts.isIdentifier(call.expression) || call.arguments.length < 2) continue;
  const first = call.arguments[0]!;
  if (!ts.isStringLiteralLike(first) || !names.includes(first.text)) continue;
  const plans: Record<string, number> = { "sdk-fence": 30000, "sdk-runtime": 120000, "profile-oracle": 120000, "receiver-proof": 2000, "watch-lifecycle": 15000, "caller-birth": 15000 };
  if (call.expression.text === "test") {
    const fn = call.arguments[1]!.getText(source), timeout = call.arguments[2]?.getText(source) ?? "120000";
    edits.push({ start: call.getStart(source), end: call.getEnd(), text: `registerIsolatedBuildTestV1(import.meta.path,${first.getText(source)},${timeout},${fn})` });
  } else if (call.expression.text === "registerBuildTestLawV1" && call.arguments.length === 3 && ts.isStringLiteralLike(call.arguments[1]!)) {
    const plan = plans[(call.arguments[1] as ts.StringLiteral).text]!, body = call.arguments[2]!.getText(source);
    edits.push({ start: call.getStart(source), end: call.getEnd(), text: `registerIsolatedBuildTestV1(import.meta.path,${first.getText(source)},${plan},()=>withBuildTestInvocationV1(${body}))` });
  } else continue;
}
let out = text;
for (const edit of edits.sort((a, b) => b.start - a.start)) out = out.slice(0, edit.start) + edit.text + out.slice(edit.end);
const found = edits.length;
if (found !== names.length) { console.error("matched " + found + " of " + names.length); process.exit(1); }
if (!out.includes("registerIsolatedBuildTestV1}") && !/registerIsolatedBuildTestV1[,}]/.test(text)) {
  const m = /import\s*\{([^}]*)\}\s*from\s*"([^"]*🧪️testing\/📥️invocation\/🟦️\.ts)"/u.exec(out);
  if (m) out = out.replace(m[0], `import{${m[1]!.trim()},registerIsolatedBuildTestV1}from"${m[2]}"`);
  else { console.error("invocation import not found"); process.exit(1); }
}
writeFileSync(file!, out);
console.log("isolated " + found);
