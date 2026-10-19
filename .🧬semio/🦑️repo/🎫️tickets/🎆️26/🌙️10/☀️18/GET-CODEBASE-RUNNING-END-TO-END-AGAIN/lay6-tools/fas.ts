import { readFileSync } from "node:fs";
const sc = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const W = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/";
const c = sc.find((x: any) => !x.renderer && x.size > 2);
const s = (p: string) => p.replace(W, "").replace(/\/🟦️\.ts$/, "");
const M: string[] = c.members.map(s);
const rt = c.edges.filter((e: any) => !e.typeOnly).map((e: any) => [s(e.from), s(e.to)]);
const ty = c.edges.filter((e: any) => e.typeOnly).map((e: any) => [s(e.from), s(e.to)]);
// order: dependency first. random restarts + local search on position among runtime topological constraints
function topoRandom(): string[] {
  const indeg = new Map<string, number>(); const rev = new Map<string, string[]>(); for (const m of M) { indeg.set(m, 0); rev.set(m, []); }
  for (const [a, b] of rt) { indeg.set(a, indeg.get(a)! + 1); rev.get(b)!.push(a); }
  const ready = M.filter((m) => indeg.get(m) === 0); const order: string[] = [];
  while (ready.length) { const i = Math.floor(Math.random() * ready.length); const n = ready.splice(i, 1)[0]!; order.push(n); for (const p of rev.get(n)!) { indeg.set(p, indeg.get(p)! - 1); if (indeg.get(p) === 0) ready.push(p); } }
  return order;
}
const score = (o: string[]) => { const pos = new Map(o.map((m, i) => [m, i])); return ty.filter(([a, b]) => pos.get(b!)! > pos.get(a!)!).length; };
let best = topoRandom(), bs = score(best);
for (let i = 0; i < 20000; i++) { const o = topoRandom(); const sc2 = score(o); if (sc2 < bs) { best = o; bs = sc2; } }
const pos = new Map(best.map((m, i) => [m, i]));
console.log("min back type edges (heuristic):", bs);
console.log(best.join("\n"));
console.log("--- back edges (cut these)");
for (const [a, b] of ty) if (pos.get(b!)! > pos.get(a!)!) console.log(a, "->", b);
