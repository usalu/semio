import ts from "typescript";
import { readFileSync, readdirSync, statSync, existsSync } from "node:fs";
import { dirname, resolve } from "node:path";
const ROOT = "/Users/ueli/Documents/semio/";
const P = ROOT + "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/";
const runtimeOnly = process.argv[2] === "rt";
const files: string[] = [];
(function walk(d: string) { for (const e of readdirSync(d)) { if (e === "node_modules" || e === "target" || e === "dist") continue; const p = d + "/" + e; const st = statSync(p); if (st.isDirectory()) walk(p); else if (/\.(ts|tsx)$/.test(e) && !e.endsWith(".d.ts")) files.push(p); } })(P.slice(0, -1));
const set = new Set(files);
const exts = ["", ".ts", ".tsx", "/🟦️.ts", "/🟦️.tsx", ".js"];
function res(from: string, spec: string): string | null {
  if (!spec.startsWith(".")) return null;
  const base = resolve(dirname(from), spec);
  for (const e of exts) { const c = base + e; if (set.has(c)) return c; }
  return null;
}
const g = new Map<string, Map<string, boolean>>();
for (const f of files) {
  const sf = ts.createSourceFile(f, readFileSync(f, "utf8"), ts.ScriptTarget.Latest, true, f.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const m = new Map<string, boolean>();
  const add = (spec: string, typeOnly: boolean) => { const r = res(f, spec); if (r) m.set(r, (m.get(r) ?? true) && typeOnly); };
  const visit = (n: ts.Node): void => {
    if (ts.isImportDeclaration(n) && ts.isStringLiteral(n.moduleSpecifier)) {
      const c = n.importClause; let t = !!c?.isTypeOnly;
      if (c && !t && !c.name && c.namedBindings && ts.isNamedImports(c.namedBindings) && c.namedBindings.elements.length && c.namedBindings.elements.every((e) => e.isTypeOnly)) t = true;
      add(n.moduleSpecifier.text, t);
    } else if (ts.isExportDeclaration(n) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) add(n.moduleSpecifier.text, n.isTypeOnly);
    else if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0])) add(n.arguments[0].text, false);
    else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal)) add(n.argument.literal.text, true);
    ts.forEachChild(n, visit);
  };
  visit(sf);
  g.set(f, m);
}
const adj = new Map<string, string[]>();
for (const [f, m] of g) adj.set(f, [...m].filter(([, t]) => !(runtimeOnly && t)).map(([k]) => k));
let idx = 0; const index = new Map<string, number>(), low = new Map<string, number>(), on = new Set<string>(), stack: string[] = [], sccs: string[][] = [];
function strong(v: string) {
  const work: [string, number][] = [[v, 0]]; index.set(v, idx); low.set(v, idx); idx++; stack.push(v); on.add(v);
  while (work.length) {
    const [n, i] = work[work.length - 1]!; const es = adj.get(n) ?? [];
    if (i < es.length) {
      work[work.length - 1]![1]++;
      const w = es[i]!;
      if (!index.has(w)) { index.set(w, idx); low.set(w, idx); idx++; stack.push(w); on.add(w); work.push([w, 0]); }
      else if (on.has(w)) low.set(n, Math.min(low.get(n)!, index.get(w)!));
    } else {
      work.pop();
      if (work.length) { const p = work[work.length - 1]![0]; low.set(p, Math.min(low.get(p)!, low.get(n)!)); }
      if (low.get(n) === index.get(n)) { const c: string[] = []; let w: string; do { w = stack.pop()!; on.delete(w); c.push(w); } while (w !== n); if (c.length > 1 || es.includes(n)) sccs.push(c); }
    }
  }
}
for (const v of adj.keys()) if (!index.has(v)) strong(v);
sccs.sort((a, b) => b.length - a.length);
const short = (p: string) => p.replace(P, "").replace("🧑‍🎨engine/🧱️elements/", "").replace(/\/🟦️\.(tsx?)$/, "/#");
console.log("files", files.length, "sccs", sccs.map((c) => c.length).join(","));
if (process.argv[3]) for (const c of sccs) console.log("--", c.length, "\n  " + c.map(short).sort().join("\n  "));
if (process.argv[4]) require("node:fs").writeFileSync(process.argv[4], JSON.stringify({ sccs, edges: [...g].flatMap(([f, m]) => [...m].map(([t, ty]) => ({ from: f, to: t, typeOnly: ty }))) }));
