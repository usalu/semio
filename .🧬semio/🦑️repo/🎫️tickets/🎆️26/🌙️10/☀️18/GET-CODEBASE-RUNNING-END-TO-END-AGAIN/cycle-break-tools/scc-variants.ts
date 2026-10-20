import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { scc } from "./pkg-graph.ts";
const g = JSON.parse(readFileSync(resolve(import.meta.dir, "../🗑️generated/cycle-break-x/graph.json"), "utf8")).edges as any[];
const run = (label: string, pred: (e: any) => boolean): void => {
  const adj = new Map<string, Set<string>>(), nodes = new Set<string>();
  for (const e of g) if (pred(e)) { nodes.add(e.from); nodes.add(e.to); (adj.get(e.from) ?? adj.set(e.from, new Set()).get(e.from)!).add(e.to); }
  const comps = scc([...nodes], adj).filter(c => c.length > 1).sort((a, b) => b.length - a.length);
  console.log(label, "nodes", nodes.size, "cyclic", comps.map(c => c.length).join(","));
};
const isScript = (e: any): boolean => /📜️script\.ts$/u.test(e.file ?? "");
run("decl only", e => e.kind === "decl");
run("decl+bare", e => e.kind !== "rel");
run("decl+bare+rel no scripts", e => !isScript(e));
run("bare+rel no scripts no decl", e => e.kind !== "decl" && !isScript(e));
run("all", () => true);
