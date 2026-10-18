import { readFileSync } from "node:fs";
import { analyze, ts, ROOT } from "./ast.ts";
import { resolveRel } from "./exports.ts";
const L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const targets = new Set([L + "🟦️.ts", L + "📦️packages/🟦️typescript/🟦️.ts"]);
const S = "/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad";
const edges = JSON.parse(readFileSync(S + "/edges.json", "utf8"));
const symbols = new Map<string, number>();
const odd: string[] = [];
const files = new Set<string>();
for (const [f, rows] of Object.entries<any>(edges)) {
  if (!rows.some(([s, r]: [string, string | null]) => (r && targets.has(r)) || s === "@semio-tech/repo-lib")) continue;
  files.add(f);
  const a = analyze(f);
  const check = (spec: string) => (resolveRel(f, spec) !== null && targets.has(resolveRel(f, spec)!)) || spec === "@semio-tech/repo-lib";
  for (const imp of a.imports) {
    if (!check(imp.spec)) continue;
    if (imp.sideEffect) odd.push(`${f}: side-effect import`);
    for (const b of imp.bindings) {
      if (b.kind !== "named") odd.push(`${f}: ${b.kind} ${b.local}`);
      else symbols.set(b.imported, (symbols.get(b.imported) ?? 0) + 1);
    }
  }
  for (const e of a.exportsFrom) if (e.spec && check(e.spec)) odd.push(`${f}: export-from ${e.star ? "*" : e.names.join(",")}`);
  const visit = (n: import("typescript").Node) => {
    if (ts.isCallExpression(n) && n.arguments.length >= 1 && ts.isStringLiteralLike(n.arguments[0])) {
      const isDyn = n.expression.kind === ts.SyntaxKind.ImportKeyword;
      const isReq = ts.isIdentifier(n.expression) && n.expression.text === "require";
      if ((isDyn || isReq) && check((n.arguments[0] as any).text)) odd.push(`${f}: ${isDyn ? "dynamic-import" : "require"} :: ${n.parent.getText().slice(0, 140).replace(/\n/g, " ")}`);
    }
    if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal) && check(n.argument.literal.text)) odd.push(`${f}: import-type :: ${n.getText().slice(0, 140)}`);
    ts.forEachChild(n, visit);
  };
  visit(a.sf);
}
console.log("files", files.size);
console.log("symbols", symbols.size);
for (const [s, n] of [...symbols].sort((a, b) => b[1] - a[1])) console.log(n, s);
console.log("ODD");
for (const o of odd) console.log(o);
