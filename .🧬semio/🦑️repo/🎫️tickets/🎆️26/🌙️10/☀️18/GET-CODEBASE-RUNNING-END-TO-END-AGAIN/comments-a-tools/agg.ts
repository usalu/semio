import { readFileSync, existsSync } from "node:fs";
const dirs = process.argv.slice(2);
const tot: Record<string, Record<string, number>> = {};
for (const d of dirs) {
  const s = JSON.parse(readFileSync(`${d}/${existsSync(`${d}/apply-summary.json`) ? "apply" : "dry"}-summary.json`, "utf8"));
  for (const [t, m] of Object.entries<any>(s)) for (const [k, v] of Object.entries<number>(m)) { tot[t] ??= {}; tot[t][k] = (tot[t][k] ?? 0) + v; }
}
const all: Record<string, number> = {};
for (const [t, m] of Object.entries(tot)) { console.log(t, JSON.stringify(m)); for (const [k, v] of Object.entries(m)) all[k] = (all[k] ?? 0) + v; }
console.log("ALL", JSON.stringify(all, null, 1));
