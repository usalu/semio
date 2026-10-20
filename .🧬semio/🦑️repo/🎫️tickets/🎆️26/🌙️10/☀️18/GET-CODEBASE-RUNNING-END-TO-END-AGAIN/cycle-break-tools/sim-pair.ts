import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { build } from "./sim.ts";
const plan = JSON.parse(readFileSync(resolve(import.meta.dir, process.argv[2]!), "utf8"));
const [a, b, mode = "runtime"] = process.argv.slice(3);
const es = build(plan).filter(e => e.from === a && e.to === b && (mode === "all" || (mode === "runtime") === !e.test));
const by = new Map<string, Set<string>>();
for (const e of es) { const k = e.target; (by.get(k) ?? by.set(k, new Set()).get(k)!).add(e.file); }
console.log(a, "->", b, mode, es.length);
for (const [t, f] of [...by].sort((x, y) => y[1].size - x[1].size)) console.log(" ", f.size, t.replace("🧰️framework/", ""), "<=", [...f].slice(0, 3).map(x => x.replace("🧰️framework/", "")).join(" | "));
