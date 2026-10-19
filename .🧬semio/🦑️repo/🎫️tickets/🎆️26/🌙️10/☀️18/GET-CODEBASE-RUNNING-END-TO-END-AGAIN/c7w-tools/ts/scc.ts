import { analyze } from "./ast.ts";
const a = analyze(process.argv[2]);
const isState = (d: any) => d.kind === "FirstStatement";
const nodes = a.decls.map((d, i) => i);
const nameOf = new Map<string, number>();
a.decls.forEach((d, i) => d.names.forEach((n) => nameOf.set(n, i)));
const adj = new Map<number, Set<number>>();
for (const i of nodes) {
  const s = new Set<number>();
  for (const n of a.decls[i].deps) { const j = nameOf.get(n); if (j !== undefined && j !== i && !isState(a.decls[j])) s.add(j); }
  adj.set(i, s);
}
let idx = 0; const index = new Map<number, number>(), low = new Map<number, number>(), on = new Set<number>(), st: number[] = [], comps: number[][] = [];
const go = (v: number) => { index.set(v, idx); low.set(v, idx); idx++; st.push(v); on.add(v); for (const w of adj.get(v)!) { if (!index.has(w)) { go(w); low.set(v, Math.min(low.get(v)!, low.get(w)!)); } else if (on.has(w)) low.set(v, Math.min(low.get(v)!, index.get(w)!)); } if (low.get(v) === index.get(v)) { const c: number[] = []; let w: number; do { w = st.pop()!; on.delete(w); c.push(w); } while (w !== v); comps.push(c); } };
for (const n of nodes) if (!isState(a.decls[n]) && !index.has(n)) go(n);
for (const c of comps.filter((c) => c.length > 1)) console.log("SCC", c.sort((x, y) => x - y).map((i) => a.decls[i].names.join(",") + "@" + a.decls[i].startLine).join(" | "));
