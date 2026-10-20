/** 🔎 Hill-climbing search for the package order with the fewest upward file edges inside the cyclic components. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { build } from "./sim.ts";
import { scc } from "./pkg-graph.ts";
const plan = JSON.parse(readFileSync(resolve(import.meta.dir, "carves.json"), "utf8"));
const es = build(plan).filter(e => !e.test);
const adj = new Map<string, Set<string>>(), nodes = new Set<string>();
for (const e of es) { nodes.add(e.from); nodes.add(e.to); (adj.get(e.from) ?? adj.set(e.from, new Set()).get(e.from)!).add(e.to); }
const comp = scc([...nodes], adj).filter(c => c.length > 6).sort((a, b) => b.length - a.length)[0]!;
const set = new Set(comp);
const w = new Map<string, number>();
const files = new Map<string, Set<string>>();
for (const e of es) if (set.has(e.from) && set.has(e.to)) { const k = e.from + ">" + e.to; (files.get(k) ?? files.set(k, new Set()).get(k)!).add(e.file); }
for (const [k, v] of files) w.set(k, v.size);
const cost = (order: string[]): number => { const pos = new Map(order.map((v, i) => [v, i])); let c = 0; for (const [k, n] of w) { const [a, b] = k.split(">") as [string, string]; if (pos.get(a)! > pos.get(b)!) c += n; } return c; };
// order[0] is the lowest layer: an arc a->b (a depends on b) is fine when b is lower (pos(b) < pos(a))
let best: string[] = [], bestC = Infinity;
for (let r = 0; r < 400; r++) {
  const order = [...comp].sort(() => Math.random() - 0.5);
  let c = cost(order), improved = true;
  while (improved) { improved = false; for (let i = 0; i < order.length; i++) for (let j = 0; j < order.length; j++) { if (i === j) continue; const t = [...order]; const [x] = t.splice(i, 1); t.splice(j, 0, x!); const c2 = cost(t); if (c2 < c) { order.splice(0, order.length, ...t); c = c2; improved = true; } } }
  if (c < bestC) { bestC = c; best = [...order]; }
}
console.log("component", comp.length, "best feedback files", bestC);
console.log(best.map((v, i) => `${i} ${v}`).join("\n"));
const pos = new Map(best.map((v, i) => [v, i]));
for (const [k, n] of [...w].sort((a, b) => b[1] - a[1])) { const [a, b] = k.split(">") as [string, string]; if (pos.get(a)! > pos.get(b)!) { console.log(String(n).padStart(3), a, "->", b, [...files.get(k)!].slice(0, 2).map(f => f.replace("🧰️framework/🛍️products/", "")).join(" | ")); } }
