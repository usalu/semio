import { readFileSync } from "node:fs";
const rows = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const which = Number(process.argv[3] ?? 0);
const r = rows[which]; const short = (p: string) => p.replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/", "L/").replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/", "R/");
const adj = new Map<string, string[]>(); for (const f of r.files) adj.set(f, []); for (const [a, b] of r.edges) adj.get(a)!.push(b);
const nodes = [...adj.keys()]; const out = new Map(nodes.map((n) => [n, new Set(adj.get(n))])); const inn = new Map(nodes.map((n) => [n, new Set<string>()]));
for (const [a, bs] of out) for (const b of bs) inn.get(b)!.add(a);
const s1: string[] = [], s2: string[] = []; const remaining = new Set(nodes);
const rm = (n: string) => { remaining.delete(n); for (const b of out.get(n)!) inn.get(b)!.delete(n); for (const a of inn.get(n)!) out.get(a)!.delete(n); };
while (remaining.size) { let progressed = true; while (progressed) { progressed = false; for (const n of [...remaining]) { if (out.get(n)!.size === 0) { s2.unshift(n); rm(n); progressed = true; } else if (inn.get(n)!.size === 0) { s1.push(n); rm(n); progressed = true; } } }
  if (!remaining.size) break; let best = "", bd = -1e9; for (const n of remaining) { const d = out.get(n)!.size - inn.get(n)!.size; if (d > bd) { bd = d; best = n; } } s1.push(best); rm(best); }
const order = new Map([...s1, ...s2].map((n, i) => [n, i])); const back = r.edges.filter(([a, b]: [string, string]) => order.get(a)! > order.get(b)!);
console.log("scc", which, "size", r.size, "edges", r.edges.length, "feedback", back.length);
for (const [a, b] of back) console.log("  ", short(a), "->", short(b));
