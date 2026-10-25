import { readFileSync } from "node:fs";
const [sccPath, idx, cutsPath] = process.argv.slice(2) as [string, string, string];
const rows = JSON.parse(readFileSync(sccPath, "utf8"));
const row = rows[Number(idx)];
const cuts: string[][] = readFileSync(cutsPath, "utf8").split("\n").filter((l) => l.trim() && !l.startsWith("#")).map((l) => l.split(" -> ").map((s) => s.trim()));
const strip = (p: string) => p.replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/", "L/");
const edges: [string, string, string][] = row.edges.filter(([a, b]: [string, string]) => !cuts.some(([x, y]) => (x!.startsWith("=") ? strip(a) === x!.slice(1) : strip(a).includes(x!)) && (y!.startsWith("=") ? strip(b) === y!.slice(1) : strip(b).includes(y!))));
const adj = new Map<string, string[]>();
for (const f of row.files) adj.set(f, []);
for (const [a, b] of edges) adj.get(a)!.push(b);
let index = 0; const idxm = new Map<string, number>(), low = new Map<string, number>(), on = new Set<string>(), st: string[] = [], out: string[][] = [];
const rec = (v: string) => { idxm.set(v, index); low.set(v, index); index++; st.push(v); on.add(v); for (const w of adj.get(v)!) { if (!idxm.has(w)) { rec(w); low.set(v, Math.min(low.get(v)!, low.get(w)!)); } else if (on.has(w)) low.set(v, Math.min(low.get(v)!, idxm.get(w)!)); } if (low.get(v) === idxm.get(v)) { const c: string[] = []; let w: string; do { w = st.pop()!; on.delete(w); c.push(w); } while (w !== v); if (c.length > 1) out.push(c); } };
for (const f of row.files) if (!idxm.has(f)) rec(f);
console.log("remaining sccs", out.map((c) => c.length).join(","));
for (const c of out) { const s = new Set(c); console.log("SCC", c.length); for (const f of c) console.log("  F", strip(f)); for (const [a, b, k] of edges) if (s.has(a) && s.has(b)) console.log("  E", strip(a), "->", strip(b), k); }
