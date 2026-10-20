import { loadPackages } from "../exports-stage-tools/lib.ts";
import { scc } from "./pkg-graph.ts";
const pk = loadPackages();
const fields = process.argv.slice(2).length ? process.argv.slice(2) : ["dependencies", "devDependencies", "peerDependencies", "optionalDependencies"];
const adj = new Map<string, Set<string>>(), nodes = new Set<string>(), kind = new Map<string, string>();
for (const p of pk.all) for (const f of fields) for (const [k, v] of Object.entries<string>(p.json[f] ?? {})) if (p.name && k !== p.name && pk.byName.has(k)) { nodes.add(p.name); nodes.add(k); (adj.get(p.name) ?? adj.set(p.name, new Set()).get(p.name)!).add(k); kind.set(p.name + ">" + k, f); }
const comps = scc([...nodes], adj).filter(c => c.length > 1);
for (const c of comps) { const s = new Set(c); console.log("SCC", c.length); for (const a of c) for (const b of adj.get(a) ?? []) if (s.has(b)) console.log("  ", a, "->", b, kind.get(a + ">" + b)); }
