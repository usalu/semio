import { readFileSync } from "node:fs";
import { analyze, ts } from "./ast.ts";
import { resolveRel } from "./exports.ts";
const L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const targets = new Set([L + "🟦️.ts", L + "📦️packages/🟦️typescript/🟦️.ts"]);
const S = "/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad";
const edges = JSON.parse(readFileSync(S + "/edges.json", "utf8"));
const forms = new Map<string, string[]>();
for (const [f, rows] of Object.entries<any>(edges)) {
  if (!rows.some(([s, r]: any) => r && targets.has(r))) continue;
  const a = analyze(f);
  const visit = (n: import("typescript").Node) => {
    if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && ts.isStringLiteralLike(n.arguments[0])) {
      const r = resolveRel(f, (n.arguments[0] as any).text);
      if (r && targets.has(r)) {
        let p: import("typescript").Node = n.parent;
        const aw = p;
        let g = aw.parent;
        let shape = ts.SyntaxKind[g.kind];
        if (ts.isVariableDeclaration(g)) shape += ":" + ts.SyntaxKind[g.name.kind];
        if (ts.isParenthesizedExpression(g)) shape += "→" + ts.SyntaxKind[g.parent.kind];
        const key = shape;
        (forms.get(key) ?? forms.set(key, []).get(key)!).push(f + " :: " + g.parent.getText().slice(0, 160).replace(/\s+/g, " "));
      }
    }
    ts.forEachChild(n, visit);
  };
  visit(a.sf);
}
for (const [k, v] of forms) { console.log(k, v.length); for (const x of v.slice(0, 3)) console.log("   ", x.slice(x.indexOf("::"))); }
