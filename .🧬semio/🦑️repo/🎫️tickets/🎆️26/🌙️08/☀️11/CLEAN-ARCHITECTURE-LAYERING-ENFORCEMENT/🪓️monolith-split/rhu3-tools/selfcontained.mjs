import fs from "node:fs";
const g = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const edges = fs.readFileSync(process.argv[3], "utf8").trim().split("\n").map(l => JSON.parse(l));
const OS = "🧰️framework/🛍️products/💻️os/";
const ENG = OS + "🔨️modules/📺️renderer/";
const isTest = p => /\/🧪️tests\//.test(p) || /\/🧫️fixtures\//.test(p) || /\.test\.tsx?$/.test(p) || /📖️stories/.test(p);
const adj = new Map(), rev = new Map();
for (const m of g.modules) { const ds = (m.dependencies ?? []).filter(d => d.resolved && !d.coreModule && !d.couldNotResolve).map(d => d.resolved); adj.set(m.source, ds); for (const d of ds) { const a = rev.get(d) ?? []; a.push(m.source); rev.set(d, a); } }
const targets = new Map();
for (const e of edges) { if (isTest(e.from) || e.to.startsWith(".cache") || !e.to.startsWith(OS) || e.to.startsWith(ENG)) continue; targets.set(e.to, (targets.get(e.to) ?? 0) + 1); }
for (const [t, n] of [...targets].sort((a, b) => b[1] - a[1])) {
  const seen = new Set(); const st = [t]; let hitRenderer = false;
  while (st.length) { const x = st.pop(); if (seen.has(x)) continue; seen.add(x); for (const d of adj.get(x) ?? []) { if (d.startsWith(ENG)) { hitRenderer = true; continue; } st.push(d); } }
  const osIn = [...seen].filter(p => p.startsWith(OS) && p !== t && !p.startsWith(ENG));
  const imp = [...new Set(rev.get(t) ?? [])]; const outside = imp.filter(p => !p.startsWith(ENG) && !isTest(p)); 
  console.log(String(n).padStart(3), "os-closure", String(osIn.length).padStart(3), "other-src", String(outside.length).padStart(3), "→renderer", hitRenderer ? "Y" : "n", t.replace(OS, "OS/"));
}
