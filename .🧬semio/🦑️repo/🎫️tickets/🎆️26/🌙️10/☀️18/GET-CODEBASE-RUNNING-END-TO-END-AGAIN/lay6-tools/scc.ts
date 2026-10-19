import { readFileSync, writeFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const OS = "🧰️framework/🛍️products/💻️os/";
const rend = OS + "🔨️modules/📺️renderer/";
const g = new Map<string, { to: string; dynamic: boolean; typeOnly: boolean; module: string }[]>();
for (const m of r.modules) {
  if (!m.source.startsWith(OS)) continue;
  g.set(m.source, m.dependencies.filter((d: any) => d.resolved?.startsWith(OS) && !d.coreModule && !d.couldNotResolve).map((d: any) => ({ to: d.resolved, dynamic: !!d.dynamic, typeOnly: !!d.dependencyTypes?.includes("type-only"), module: d.module })));
}
let idx = 0; const index = new Map<string, number>(), low = new Map<string, number>(), on = new Set<string>(), stack: string[] = [], sccs: string[][] = [];
function strong(v: string) {
  const work: [string, number][] = [[v, 0]]; index.set(v, idx); low.set(v, idx); idx++; stack.push(v); on.add(v);
  while (work.length) {
    const [n, i] = work[work.length - 1]!; const es = g.get(n) ?? [];
    if (i < es.length) {
      work[work.length - 1]![1]++;
      const w = es[i]!.to; if (!g.has(w)) continue;
      if (!index.has(w)) { index.set(w, idx); low.set(w, idx); idx++; stack.push(w); on.add(w); work.push([w, 0]); }
      else if (on.has(w)) low.set(n, Math.min(low.get(n)!, index.get(w)!));
    } else {
      work.pop();
      if (work.length) { const p = work[work.length - 1]![0]; low.set(p, Math.min(low.get(p)!, low.get(n)!)); }
      if (low.get(n) === index.get(n)) { const c: string[] = []; let w: string; do { w = stack.pop()!; on.delete(w); c.push(w); } while (w !== n); if (c.length > 1 || (g.get(n) ?? []).some((e) => e.to === n)) sccs.push(c); }
    }
  }
}
for (const v of g.keys()) if (!index.has(v)) strong(v);
const s = (p: string) => p.replace(OS, "");
sccs.sort((a, b) => b.length - a.length);
const out = sccs.map((c) => { const set = new Set(c); return { renderer: c.some((m) => m.startsWith(rend)), size: c.length, members: c, edges: c.flatMap((m) => g.get(m)!.filter((e) => set.has(e.to)).map((e) => ({ from: m, ...e }))) }; });
writeFileSync(process.argv[3]!, JSON.stringify(out, null, 1));
for (const c of out) console.log(c.renderer ? "R" : "N", c.size, "members", c.edges.length, "edges", "e.g.", s(c.members[0]!));
