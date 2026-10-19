import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const cyc = new Set<string>();
const adj = new Map<string, Set<string>>();
for (const x of r.summary.violations) if (x.rule.name === "no-circular") for (const c of x.cycle ?? []) cyc.add(c.name);
const edgeKinds = new Map<string, string>();
for (const m of r.modules) { if (!cyc.has(m.source)) continue; for (const d of m.dependencies) if (cyc.has(d.resolved) && d.circular) { (adj.get(m.source) ?? adj.set(m.source, new Set()).get(m.source)!).add(d.resolved); edgeKinds.set(m.source + ">" + d.resolved, `${d.module} ${d.dependencyTypes.join(",")}${d.typeOnly ? " TYPE" : ""}${d.dynamic ? " DYN" : ""}`); } }
let idx = 0; const stack: string[] = [], on = new Set<string>(), index = new Map<string, number>(), low = new Map<string, number>(), sccs: string[][] = [];
const strong = (v: string) => { index.set(v, idx); low.set(v, idx++); stack.push(v); on.add(v);
  for (const w of adj.get(v) ?? []) { if (!index.has(w)) { strong(w); low.set(v, Math.min(low.get(v)!, low.get(w)!)); } else if (on.has(w)) low.set(v, Math.min(low.get(v)!, index.get(w)!)); }
  if (low.get(v) === index.get(v)) { const c: string[] = []; let w; do { w = stack.pop()!; on.delete(w); c.push(w); } while (w !== v); if (c.length > 1) sccs.push(c); } };
for (const v of adj.keys()) if (!index.has(v)) strong(v);
const short = (s: string) => s.replace("🧰️framework/🛍️products/", "").replace(/🔨️modules\/📚️library\//, "L/");
for (const c of sccs) { console.log("SCC", c.length); for (const v of c) for (const w of adj.get(v) ?? []) if (c.includes(w)) console.log("  ", short(v), "->", short(w), "|", edgeKinds.get(v + ">" + w)); }
