import { load, scc } from "./graph.mjs";
const { tops } = load(process.argv[2]);
const regions = [...new Set(tops.map((t) => t.region))];
const rsucc = (r) => { const s = new Set(); for (const t of tops) if (t.region === r) for (const d of t.deps.keys()) if (d.region !== r) s.add(d.region); return [...s]; };
const comps = scc(regions, rsucc);
console.log("REGION SCCs (size>1):");
for (const c of comps) if (c.length > 1) console.log(c.length, c.join(" | "));
console.log("singletons:", comps.filter((c) => c.length === 1).length);
let eagerCross = 0;
for (const t of tops) for (const [d, v] of t.deps) if (v.eager && d.region !== t.region) { eagerCross++; console.log("EAGER", t.names[0] ?? t.kind, t.region, "->", d.names[0], d.region); }
console.log("eager cross", eagerCross);
const ssucc = (t) => [...t.deps.keys()];
const sc = scc(tops, ssucc).filter((c) => c.length > 1);
console.log("STATEMENT SCCs>1:", sc.length, sc.map((c) => c.length).join(","));
