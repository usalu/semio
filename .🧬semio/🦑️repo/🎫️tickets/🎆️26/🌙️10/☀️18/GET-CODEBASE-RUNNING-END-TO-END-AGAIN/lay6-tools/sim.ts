import { readFileSync } from "node:fs";
const sc = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const W = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/";
const c = sc.find((x: any) => !x.renderer && x.size > 20);
const cut = new Set(process.argv.slice(3).map((a) => a.split(">")));
const s = (p: string) => p.replace(W, "").replace(/\/🟦️\.ts$/, "");
const members: string[] = c.members; const g = new Map<string, string[]>(); for (const m of members) g.set(m, []);
for (const e of c.edges) { if ([...cut].some(([a, b]) => s(e.from) === a && (b === "*" || s(e.to) === b))) continue; g.get(e.from)!.push(e.to); }
let idx = 0; const index = new Map<string, number>(), low = new Map<string, number>(), on = new Set<string>(), st: string[] = [], out: string[][] = [];
function sc2(v: string) { const work: [string, number][] = [[v, 0]]; index.set(v, idx); low.set(v, idx++); st.push(v); on.add(v);
  while (work.length) { const [n, i] = work[work.length - 1]!; const es = g.get(n)!; if (i < es.length) { work[work.length - 1]![1]++; const w = es[i]!; if (!index.has(w)) { index.set(w, idx); low.set(w, idx++); st.push(w); on.add(w); work.push([w, 0]); } else if (on.has(w)) low.set(n, Math.min(low.get(n)!, index.get(w)!)); } else { work.pop(); if (work.length) { const p = work[work.length - 1]![0]; low.set(p, Math.min(low.get(p)!, low.get(n)!)); } if (low.get(n) === index.get(n)) { const comp: string[] = []; let w: string; do { w = st.pop()!; on.delete(w); comp.push(w); } while (w !== n); if (comp.length > 1) out.push(comp); } } } }
for (const m of members) if (!index.has(m)) sc2(m);
console.log(out.length ? out.map((o) => o.length + ": " + o.map(s).join(", ")).join("\n") : "ACYCLIC");
