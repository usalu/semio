import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { scc } from "./pkg-graph.ts";
const g = JSON.parse(readFileSync(resolve(import.meta.dir, "../🗑️generated/cycle-break-x/graph.json"), "utf8")).edges as any[];
export const isTest = (f: string): boolean => /(^|\/)🧪️|\.(test|spec|stories|story)\.[cm]?[jt]sx?$|\/📖️stories\/|(^|\/)📜️script\.ts$|vite(st)?\.config|\/🏗️builder\/|\/📖️/u.test(f);
const mode = process.argv[2] ?? "runtime";
const adj = new Map<string, Set<string>>(), nodes = new Set<string>(), cnt = new Map<string, number>();
for (const e of g) {
  if (e.kind === "decl") continue;
  if (mode === "runtime" && isTest(e.file)) continue;
  if (mode === "test" && !isTest(e.file)) continue;
  nodes.add(e.from); nodes.add(e.to); (adj.get(e.from) ?? adj.set(e.from, new Set()).get(e.from)!).add(e.to);
  const k = e.from + ">" + e.to; cnt.set(k, (cnt.get(k) ?? 0) + 1);
}
const comps = scc([...nodes], adj).filter(c => c.length > 1).sort((a, b) => b.length - a.length);
console.log(mode, "nodes", nodes.size, "cyclic components", comps.map(c => c.length).join(","));
for (const c of comps) { const s = new Set(c); console.log("\nSCC", c.length); for (const a of c) for (const b of adj.get(a) ?? []) if (s.has(b)) console.log(`  ${a} -> ${b} ${cnt.get(a + ">" + b)}`); }
