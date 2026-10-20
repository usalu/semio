import { readFileSync } from "node:fs";
import { resolve } from "node:path";
const g = JSON.parse(readFileSync(resolve(import.meta.dir, "../🗑️generated/cycle-break-x/graph.json"), "utf8")).edges as any[];
const [a, b, mode] = process.argv.slice(2);
const sel = g.filter(e => e.kind !== "decl" && e.from === a && e.to === b);
console.log(sel.length, "edges");
const by = new Map<string, Set<string>>();
for (const e of sel) { const k = mode === "files" ? e.file : (e.target ?? e.spec); (by.get(k) ?? by.set(k, new Set()).get(k)!).add(e.file); }
for (const [k, v] of [...by].sort((x, y) => y[1].size - x[1].size)) console.log(v.size, k, mode === "files" ? "" : [...v].slice(0, 2).join(" | "));
