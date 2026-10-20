import { readFileSync } from "node:fs";
import { resolve } from "node:path";
const g = JSON.parse(readFileSync(resolve(import.meta.dir, "../🗑️generated/cycle-break-x/graph.json"), "utf8")).edges as any[];
const isTest = (f: string): boolean => /(^|\/)🧪️|\.(test|spec|stories|story)\.[cm]?[jt]sx?$|\/📖️stories\/|(^|\/)📜️script\.ts$|vite(st)?\.config|\/🏗️builder\/|\/📖️/u.test(f);
const [a, b, mode = "all"] = process.argv.slice(2);
const sel = g.filter(e => e.kind !== "decl" && e.from === a && e.to === b && (mode === "all" || (mode === "runtime") === !isTest(e.file)));
const by = new Map<string, Set<string>>();
for (const e of sel) { const k = e.target ?? e.spec; (by.get(k) ?? by.set(k, new Set()).get(k)!).add(e.file); }
console.log(a, "->", b, mode, sel.length, "edges", by.size, "targets");
for (const [k, v] of [...by].sort((x, y) => y[1].size - x[1].size)) console.log(" ", v.size, k, "<=", [...v].slice(0, 2).join(" | "));
