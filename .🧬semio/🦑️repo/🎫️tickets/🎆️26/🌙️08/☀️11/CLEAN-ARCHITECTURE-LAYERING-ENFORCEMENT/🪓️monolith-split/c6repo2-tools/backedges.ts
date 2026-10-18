import { parseFile } from "./lib.ts";
import { readFileSync } from "node:fs";
const plan = JSON.parse(readFileSync(process.argv[2], "utf8"));
const p = parseFile(plan.file);
const n = p.units.length;
const home: number[] = new Array(n).fill(-1);
plan.chunks.forEach((c: any, ci: number) => {
  for (const [a, b] of c.ranges ?? []) for (let i = a; i <= b && i < n; i++) if (home[i] < 0) home[i] = ci;
  for (const u of c.units ?? []) home[u] = ci;
});
const nm = (i: number) => [...p.units[i].names][0] ?? "?";
const groups = new Map<string, string[]>();
for (const u of p.units) for (const d of u.deps) if (home[d] > home[u.id]) {
  const k = `${plan.chunks[home[u.id]].dir} -> ${plan.chunks[home[d]].dir}`;
  (groups.get(k) ?? groups.set(k, []).get(k)!).push(`${nm(u.id)}->${nm(d)}`);
}
for (const [k, v] of groups) console.log(`${v.length}\t${k}\t${v.slice(0, 4).join(" ")}`);
