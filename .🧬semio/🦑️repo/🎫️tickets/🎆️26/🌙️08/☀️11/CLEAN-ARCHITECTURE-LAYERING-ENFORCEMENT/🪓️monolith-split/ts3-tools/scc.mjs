import fs from "node:fs";
const rows = fs.readFileSync(process.argv[2], "utf8").trim().split("\n").map((l) => { const [i, rng, kind, names, edges] = l.split("\t"); return { i: +i, rng, kind, names, edges: edges.replace("-> ", "").split(",").filter(Boolean).map(Number) }; });
let index = 0; const st = [], on = new Set(), idx = new Map(), low = new Map(), sccs = [];
const sc = (v) => { idx.set(v, index); low.set(v, index); index++; st.push(v); on.add(v);
  for (const w of rows[v].edges) { if (!idx.has(w)) { sc(w); low.set(v, Math.min(low.get(v), low.get(w))); } else if (on.has(w)) low.set(v, Math.min(low.get(v), idx.get(w))); }
  if (low.get(v) === idx.get(v)) { const c = []; let w; do { w = st.pop(); on.delete(w); c.push(w); } while (w !== v); sccs.push(c); } };
for (const r of rows) if (!idx.has(r.i)) sc(r.i);
for (const c of sccs.filter((c) => c.length > 1)) console.log("SCC", c.sort((a, b) => a - b).map((i) => `${i}:${rows[i].names}(${rows[i].rng})`).join(" | "));
