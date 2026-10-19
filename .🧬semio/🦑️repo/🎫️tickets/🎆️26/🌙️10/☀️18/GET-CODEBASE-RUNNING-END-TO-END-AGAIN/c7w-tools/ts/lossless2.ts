import { readFileSync } from "node:fs";
import { ts, parse } from "./ast.ts";
// like lossless.ts but reverses the setter rewrite: setX(v) -> X = v for the declared lets, and reports the unit names that differ
const [origPath, listPath, letsCsv] = process.argv.slice(2);
const lets = new Set(letsCsv.split(","));
const norm = (t: string) => t.replace(/^export\s+/u, "");
function unrewrite(text: string): string {
  const sf = parse("x.ts", text);
  const edits: { s: number; e: number; r: (inner: (a: number, b: number) => string) => string }[] = [];
  const visit = (n: import("typescript").Node) => {
    if (ts.isCallExpression(n) && ts.isIdentifier(n.expression) && /^set[A-Z]/.test(n.expression.text)) {
      const name = n.expression.text[3].toLowerCase() + n.expression.text.slice(4);
      if (lets.has(name) && n.arguments.length === 1) {
        const a = n.arguments[0];
        edits.push({ s: n.getStart(sf), e: n.getEnd(), r: (inner) => `${name} = ${inner(a.getStart(sf), a.getEnd())}` });
      }
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  const render = (s: number, e: number): string => {
    let out = "", c = s;
    for (const ed of edits.filter((x) => x.s >= s && x.e <= e).sort((x, y) => x.s - y.s || y.e - x.e)) {
      if (ed.s < c) continue;
      out += text.slice(c, ed.s) + ed.r(render);
      c = ed.e;
    }
    return out + text.slice(c, e);
  };
  return render(0, text.length);
}
function units(path: string, text: string, rewrite: boolean): Map<string, string> {
  const sf = parse(path, text);
  const out = new Map<string, string>();
  let i = 0;
  for (const st of sf.statements) {
    if (ts.isImportDeclaration(st) || ts.isExportDeclaration(st)) continue;
    const t = norm(st.getText(sf));
    const isSetter = ts.isFunctionDeclaration(st) && st.name && /^set[A-Z]/.test(st.name.text) && lets.has(st.name.text[3].toLowerCase() + st.name.text.slice(4)) && rewrite;
    if (isSetter) continue;
    out.set(`${i++}`, rewrite ? unrewrite(t) : t);
  }
  return out;
}
const orig = [...units(origPath, readFileSync(origPath, "utf8"), false).values()];
const created: string[] = [];
for (const f of readFileSync(listPath, "utf8").split("\n").filter(Boolean)) created.push(...units(f, readFileSync(f.startsWith("/") ? f : "/Users/ueli/Documents/semio/" + f, "utf8"), true).values());
const count = (xs: string[]) => { const m = new Map<string, number>(); for (const x of xs) m.set(x, (m.get(x) ?? 0) + 1); return m; };
const a = count(orig), b = count(created);
const missing = [...a].filter(([k, n]) => (b.get(k) ?? 0) < n), extra = [...b].filter(([k, n]) => (a.get(k) ?? 0) < n);
console.log(`orig ${orig.length} new ${created.length} missing ${missing.length} extra ${extra.length}`);
for (const [k] of missing) console.log("MISSING", k.slice(0, 100).replace(/\n/g, " "));
for (const [k] of extra) console.log("EXTRA  ", k.slice(0, 100).replace(/\n/g, " "));
