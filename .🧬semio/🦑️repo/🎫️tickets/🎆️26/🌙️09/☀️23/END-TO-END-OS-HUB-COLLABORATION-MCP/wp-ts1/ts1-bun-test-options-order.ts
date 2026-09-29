/** ⏱️ TS1 one-off codemod: `bun:test` takes options after the body — `test(label, fn, { timeout })` — as `@types/bun`
 * declares; suites written `test(label, { timeout }, fn)` are reordered. usage: bun ts1-bun-test-options-order.ts <file…> [--apply] */
import ts from "typescript";
import { readFileSync, writeFileSync } from "node:fs";

const args = process.argv.slice(2);
const apply = args.includes("--apply");
for (const file of args.filter((arg) => arg !== "--apply")) {
  const text = readFileSync(file, "utf8");
  const source = ts.createSourceFile(file, text, ts.ScriptTarget.ESNext, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const edits: { start: number; end: number; text: string }[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node) && node.arguments.length === 3) {
      const callee = node.expression.getText(source);
      const [label, options, body] = node.arguments;
      if (/^(test|it)(\.(skip|only|todo|serial|concurrent))?$/.test(callee) && ts.isObjectLiteralExpression(options!) && (ts.isArrowFunction(body!) || ts.isFunctionExpression(body!))) {
        edits.push({ start: options!.getStart(source), end: body!.getEnd(), text: `${body!.getText(source)}, ${options!.getText(source)}` });
        void label;
        return;
      }
    }
    ts.forEachChild(node, visit);
  };
  visit(source);
  let out = text;
  for (const edit of edits.sort((a, b) => b.start - a.start)) out = out.slice(0, edit.start) + edit.text + out.slice(edit.end);
  if (edits.length && apply) writeFileSync(file, out);
  console.log(`${edits.length ? (apply ? "wrote" : "would write") : "unchanged"} ${file.replace(/^.*\/(🧪️tests|📜️script)/, "$1")} (${edits.length})`);
}
