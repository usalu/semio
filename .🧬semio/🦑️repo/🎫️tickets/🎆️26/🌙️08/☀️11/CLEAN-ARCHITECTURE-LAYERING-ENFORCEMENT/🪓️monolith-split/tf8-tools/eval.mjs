import { load, scc } from "./graph.mjs";
const [file, cfgPath] = process.argv.slice(2);
const cfg = (await import(cfgPath)).default;
const { tops } = load(file);
const modOfRegion = new Map(); for (const m of cfg.modules) for (const r of m.regions ?? []) modOfRegion.set(r, m.name);
const modOfName = new Map(); for (const m of cfg.modules) for (const n of m.names ?? []) modOfName.set(n, m.name);
const assign = new Map();
const unassigned = [];
for (const t of tops) {
  if (t.isImport) continue;
  if (t.names.some((n) => (cfg.drop ?? []).includes(n))) continue;
  const byName = t.names.map((n) => modOfName.get(n)).find(Boolean);
  const m = byName ?? modOfRegion.get(t.region);
  if (!m) { unassigned.push(`${t.names[0] ?? t.kind}@${t.region}`); continue; }
  assign.set(t, m);
}
if (unassigned.length) console.log("UNASSIGNED", unassigned.join(" "));
const order = cfg.modules.map((m) => m.name);
const size = {};
for (const [t, m] of assign) size[m] = (size[m] ?? 0) + t.end - t.start + 1;
console.log("SIZES", JSON.stringify(size));
const succ = (m) => { const s = new Set(); for (const [t, tm] of assign) if (tm === m) for (const d of t.deps.keys()) { const dm = assign.get(d); if (dm && dm !== m) s.add(dm); } return [...s]; };
const comps = scc(order, succ);
const bad = comps.filter((c) => c.length > 1);
console.log("MODULE SCCs:", bad.length ? bad.map((c) => c.join(" | ")).join("  ;;  ") : "none (acyclic)");
if (bad.length) {
  const inBad = new Set(bad.flat());
  const rank = new Map(order.map((m, i) => [m, i]));
  const back = new Map();
  for (const [t, tm] of assign) if (inBad.has(tm)) for (const [d, v] of t.deps) { const dm = assign.get(d); if (dm && dm !== tm && inBad.has(dm) && rank.get(dm) > rank.get(tm)) { const k = `${tm}(${t.names[0]}) -> ${dm}(${d.names[0]})${v.typeOnly ? " [type]" : ""}`; back.set(k, 1); } }
  console.log("BACKWARD (vs order):"); for (const k of back.keys()) console.log("  ", k);
}
const edgeList = new Map();
for (const [t, tm] of assign) for (const [d, v] of t.deps) { const dm = assign.get(d); if (dm && dm !== tm) { const k = tm + " -> " + dm; edgeList.set(k, (edgeList.get(k) ?? 0) + 1); } }
if (process.argv[4] === "edges") for (const [k, v] of edgeList) console.log(k, v);
