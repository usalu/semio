import { ALLNAMES, buildGraph } from "./graph.ts";
import { root } from "./lib.ts";
const g = buildGraph(ALLNAMES);
const inc = root + "/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🔐️two-turn-presence-and-root-retirement/🦀️.rs";
const outer = root + "/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs";
for (const f of [outer, inc]) {
  const p = g.parsedCache.get(f);
  console.log(f.slice(-60), "parsed", !!p, "mods", (g.modsByFile.get(f) ?? []).length, "chains", p?.chains.length, "includes", p?.root.includes);
  if (p) for (const c of p.chains.slice(0, 3)) console.log("  chain", c.segs.join("::"), "instances", (g.modsByDecl.get(c.mod) ?? []).length);
}
import { makeResolver } from "./graph.ts";
const R = makeResolver(g);
const p = g.parsedCache.get(inc)!;
for (const c of p.chains) for (const m of g.modsByDecl.get(c.mod) ?? []) {
  console.log("inst", m.id, m.crate.crate.name, m.path(), "lead", c.lead);
  for (let k = 1; k < c.segs.length; k++) { const pr = R.resolvePath(m, c.segs.slice(0, k), c.lead); console.log("  prefix", c.segs.slice(0, k).join("::"), "->", pr ? pr.path() + " " + pr.crate.crate.name : null); }
}
