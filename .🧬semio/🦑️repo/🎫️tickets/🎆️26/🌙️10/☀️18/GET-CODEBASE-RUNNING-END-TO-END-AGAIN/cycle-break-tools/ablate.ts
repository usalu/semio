import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { build } from "./sim.ts";
import { scc } from "./pkg-graph.ts";
const plan = JSON.parse(readFileSync(resolve(import.meta.dir, "carves.json"), "utf8")) as { name: string; owners: string[] }[];
const keep = new Set(["@semio-tech/repo-lib", "@semio-tech/repo-coordinator", "@semio-tech/repo-test", "@semio-tech/nx-bootstrap-tools", "@semio-tech/ui-react", "@semio-tech/ui-styling", "@semio-tech/framework-ui-viewport-ts", "@semio-tech/ui-locale-rs"]);
const run = (p: typeof plan): string => {
  const es = build(p).filter(e => !e.test);
  const adj = new Map<string, Set<string>>(), nodes = new Set<string>();
  for (const e of es) { nodes.add(e.from); nodes.add(e.to); (adj.get(e.from) ?? adj.set(e.from, new Set()).get(e.from)!).add(e.to); }
  const comps = scc([...nodes], adj).filter(c => c.length > 1);
  return `${comps.map(c => c.length).join(",")} fwInScc=${comps.some(c => c.includes("@semio-tech/framework"))}`;
};
console.log("all:", run(plan));
for (const c of plan) { if (keep.has(c.name)) continue; console.log("without", c.name.padEnd(48), run(plan.filter(x => x !== c))); }
