import fs from "node:fs";
import path from "node:path";
const root = path.resolve(process.argv[2]);
const files = [];
(function walk(dir) { for (const entry of fs.readdirSync(dir, { withFileTypes: true })) { const full = path.join(dir, entry.name); if (entry.isDirectory()) { if (entry.name !== "node_modules") walk(full); } else if (/\.tsx?$/.test(entry.name)) files.push(full); } })(root);
const set = new Set(files);
const edges = new Map();
const importRe = /(?:import|export)\s[^"';]*?from\s*"([^"]+)"|import\s*"([^"]+)"|import\(\s*"([^"]+)"\s*\)/g;
for (const file of files) {
  const source = fs.readFileSync(file, "utf8");
  const out = new Set();
  for (const match of source.matchAll(importRe)) {
    const spec = match[1] ?? match[2] ?? match[3];
    if (!spec.startsWith(".")) continue;
    const target = path.resolve(path.dirname(file), spec);
    if (set.has(target)) out.add(target);
  }
  edges.set(file, out);
}
let index = 0; const stack = []; const state = new Map(); const sccs = [];
function strong(v) {
  state.set(v, { index, low: index, on: true }); index += 1; stack.push(v);
  for (const w of edges.get(v) ?? []) {
    if (!state.has(w)) { strong(w); state.get(v).low = Math.min(state.get(v).low, state.get(w).low); }
    else if (state.get(w).on) state.get(v).low = Math.min(state.get(v).low, state.get(w).index);
  }
  if (state.get(v).low === state.get(v).index) { const comp = []; let w; do { w = stack.pop(); state.get(w).on = false; comp.push(w); } while (w !== v); if (comp.length > 1 || edges.get(v).has(v)) sccs.push(comp); }
}
for (const f of files) if (!state.has(f)) strong(f);
let n = 0; for (const s of edges.values()) n += s.size;
console.log(`files=${files.length} edges=${n} cycles=${sccs.length}`);
for (const c of sccs) console.log(c.map((f) => path.relative(root, f)).join("\n  "));
