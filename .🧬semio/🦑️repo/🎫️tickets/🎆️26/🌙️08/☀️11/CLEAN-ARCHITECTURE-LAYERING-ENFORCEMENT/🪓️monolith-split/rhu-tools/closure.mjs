import fs from "node:fs";
const g = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const OS = "🧰️framework/🛍️products/💻️os/";
const ENG = OS + "🔨️modules/📺️renderer/";
const adj = new Map();
for (const m of g.modules) adj.set(m.source, (m.dependencies ?? []).filter(d => d.resolved && !d.coreModule && !d.couldNotResolve).map(d => d.resolved));
const roots = process.argv.slice(3);
const seen = new Set(); const stack = [...roots];
while (stack.length) { const n = stack.pop(); if (seen.has(n)) continue; seen.add(n); if (n.startsWith(ENG)) continue; for (const d of adj.get(n) ?? []) stack.push(d); }
const osIn = [...seen].filter(p => p.startsWith(OS) && !roots.includes(p));
const renderer = [...seen].filter(p => p.startsWith(ENG));
console.log("closure", seen.size, "os-internal", osIn.length, "renderer", renderer.length);
const by = new Map(); for (const p of osIn) { const k = p.slice(OS.length).split("/").slice(0, 3).join("/"); by.set(k, (by.get(k) ?? 0) + 1); }
for (const [k, v] of [...by].sort((a, b) => b[1] - a[1]).slice(0, 25)) console.log(v, k);
