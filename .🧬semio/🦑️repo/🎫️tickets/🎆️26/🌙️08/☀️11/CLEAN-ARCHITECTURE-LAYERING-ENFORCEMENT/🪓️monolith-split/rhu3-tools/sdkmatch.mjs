import fs from "node:fs";
const txt = fs.readFileSync(process.argv[2], "utf8").split("\n");
const all = JSON.parse(fs.readFileSync(process.argv[3], "utf8"));
const entries = JSON.parse(fs.readFileSync(process.argv[4], "utf8"));
const sub = new Map(entries.map(([k, v]) => [v, k]));
const sets = Object.entries(all).map(([f, names]) => ({ f, sub: sub.get(f), set: new Set(names) }));
let cur = null, to = null; const edges = [];
for (const l of txt) { if (l.startsWith("E/")) cur = l; else if (l.startsWith("   -> ")) to = l.slice(6); else if (l.startsWith("      ")) { const names = l.trim().split(", ").filter(Boolean).map(n => n.replace(/^type /, "")).flatMap(n => { const m = n.match(/^export\{(.*)\}$/); return m ? m[1].split(",") : [n]; }); edges.push({ from: cur, to, names }); } }
let ok = 0; const unresolved = new Map();
for (const e of edges) {
  const need = e.names.filter(n => !n.startsWith("<"));
  const hit = sets.find(s => need.length > 0 && need.every(n => s.set.has(n)));
  if (hit) { ok++; if (process.argv[5] === "v") console.log("OK ", hit.sub, "<-", e.from.slice(0,90), need.join(",").slice(0,80)); } else { const k = e.to; const s = unresolved.get(k) ?? new Set(); need.forEach(n => s.add(n)); unresolved.set(k, s); }
}
console.log("edges", edges.length, "single-entry-satisfiable", ok);
for (const [k, s] of [...unresolved].sort()) console.log(k.replace("F/🔨️modules/", ""), "::", [...s].slice(0, 12).join(", "));
