import { load, scc } from "./graph.mjs";
const { tops } = load(process.argv[2]);
const regions = [...new Set(tops.map((t) => t.region))];
const E = new Map();
for (const t of tops) for (const [d, v] of t.deps) if (d.region !== t.region) { const k = t.region + " -> " + d.region; if (!E.has(k)) E.set(k, new Set()); E.get(k).add(`${t.names[0]}>${d.names[0]}${v.typeOnly ? "(t)" : ""}`); }
for (const [k, v] of E) console.log(k, "::", [...v].slice(0, 8).join(", "));
