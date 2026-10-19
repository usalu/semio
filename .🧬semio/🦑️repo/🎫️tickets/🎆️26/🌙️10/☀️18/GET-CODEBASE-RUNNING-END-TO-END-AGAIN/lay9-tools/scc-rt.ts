import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const mode = process.argv[3] ?? "rt";
const OS = "🧰️framework/🛍️products/💻️os/";
const P = OS + "🔨️modules/📺️renderer/";
const g = new Map<string, string[]>();
for (const m of r.modules) {
  if (!m.source.startsWith(P)) continue;
  g.set(m.source, m.dependencies.filter((d: any) => d.resolved?.startsWith(P) && !d.coreModule && !d.couldNotResolve && (mode === "all" || !d.dependencyTypes.some((t: string) => t === "type-only" || t === "type-import"))).map((d: any) => d.resolved));
}
let idx = 0; const index = new Map<string, number>(), low = new Map<string, number>(), on = new Set<string>(), stack: string[] = [], sccs: string[][] = [];
function strong(v: string) {
  const work: [string, number][] = [[v, 0]]; index.set(v, idx); low.set(v, idx); idx++; stack.push(v); on.add(v);
  while (work.length) {
    const [n, i] = work[work.length - 1]!; const es = g.get(n) ?? [];
    if (i < es.length) {
      work[work.length - 1]![1]++;
      const w = es[i]!; if (!g.has(w)) continue;
      if (!index.has(w)) { index.set(w, idx); low.set(w, idx); idx++; stack.push(w); on.add(w); work.push([w, 0]); }
      else if (on.has(w)) low.set(n, Math.min(low.get(n)!, index.get(w)!));
    } else {
      work.pop();
      if (work.length) { const p = work[work.length - 1]![0]; low.set(p, Math.min(low.get(p)!, low.get(n)!)); }
      if (low.get(n) === index.get(n)) { const c: string[] = []; let w: string; do { w = stack.pop()!; on.delete(w); c.push(w); } while (w !== n); if (c.length > 1 || (g.get(n) ?? []).some((e) => e === n)) sccs.push(c); }
    }
  }
}
for (const v of g.keys()) if (!index.has(v)) strong(v);
sccs.sort((a, b) => b.length - a.length);
for (const c of sccs) { console.log(c.length); } if (process.argv[4]) (await import("node:fs")).writeFileSync(process.argv[4], JSON.stringify(sccs[0]));
