/** expc-d: restores the original specifiers of the edits that landed on a barrel facade (more than 4 modules) so that ncpr.ts can re-point them to narrow facades. bun unbarrel.ts <plan.json...> -- writes edges to <out> */
import ts from "typescript";
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, normalize, relative, resolve } from "node:path";
const root = "/Users/ueli/Documents/semio";
const [out, ...plans] = process.argv.slice(2) as string[];
const BARRELS = ["🎮️playground/🟦️.ts", "🖥️server/📦️packages/🟦️typescript/🟦️.ts", "📚️course/📦️packages/🟦️typescript/🟦️.ts", "📦️packages/🟦️typescript/🦀️cargo/🟦️.ts", "🏗️builder/🌐️vite/🟦️.ts"];
const rows: { file: string; before: string; after: string }[] = plans.flatMap(p => JSON.parse(readFileSync(p, "utf8")).edits);
const byFile = new Map<string, typeof rows>();
for (const r of rows) { const t = normalize(resolve(root, dirname(r.file), r.after)); if (!BARRELS.some(b => t.endsWith(b))) continue; (byFile.get(r.file) ?? byFile.set(r.file, []).get(r.file)!).push(r); }
const edges: { rule: string; from: string; to: string }[] = [];
for (const [file, list] of byFile) {
  let text = readFileSync(resolve(root, file), "utf8");
  const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true);
  const nodes: ts.StringLiteralLike[] = [];
  const visit = (n: ts.Node): void => {
    if ((ts.isImportDeclaration(n) || ts.isExportDeclaration(n)) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) nodes.push(n.moduleSpecifier);
    else if (ts.isCallExpression(n) && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0]) && (n.expression.kind === ts.SyntaxKind.ImportKeyword || (ts.isIdentifier(n.expression) && n.expression.text === "require"))) nodes.push(n.arguments[0]);
    ts.forEachChild(n, visit);
  };
  visit(sf);
  const repl: { start: number; end: number; text: string }[] = [];
  const queues = new Map<string, typeof rows>();
  for (const r of list) (queues.get(r.after) ?? queues.set(r.after, []).get(r.after)!).push(r);
  for (const n of nodes) { const q = queues.get(n.text); if (!q?.length) continue; const r = q.shift()!; repl.push({ start: n.getStart(sf) + 1, end: n.getEnd() - 1, text: r.before }); const target = normalize(relative(root, resolve(root, dirname(file), r.before))); edges.push({ rule: "RULE", from: file, to: target }); }
  for (const x of repl.sort((a, b) => b.start - a.start)) text = text.slice(0, x.start) + x.text + text.slice(x.end);
  writeFileSync(resolve(root, file), text);
}
writeFileSync(out!, JSON.stringify({ ok: true, edges }, null, 1));
console.log(byFile.size, "files,", edges.length, "specifiers restored");
