import fs from "node:fs";
import path from "node:path";
const [origFile, root, extraList] = process.argv.slice(2);
const REG = /^[ \t]*\/\/\s*#(end)?region\b\s*(.*)$/;
const walk = (d) => fs.readdirSync(d, { withFileTypes: true }).flatMap((e) => (e.isDirectory() ? (["🧪️tests","🧫️fixtures","📖️stories","🎯️targets","🧬️schema","node_modules"].includes(e.name) ? [] : walk(path.join(d, e.name))) : /^🟦️\.tsx?$/.test(e.name) ? [path.join(d, e.name)] : []));
const files = [...walk(root), ...(extraList ? extraList.split(",") : [])];
const count = (t) => { const s = new Map(); t.split("\n").forEach((l) => { const m = REG.exec(l); if (m && !m[1]) s.set(m[2].trim(), (s.get(m[2].trim()) ?? 0) + 1); }); return s; };
const o = count(fs.readFileSync(origFile, "utf8"));
const all = new Map();
let bad = 0;
for (const f of files) {
  const t = fs.readFileSync(f, "utf8");
  for (const [n, c] of count(t)) all.set(n, (all.get(n) ?? 0) + c);
  const st = []; let mism = 0;
  t.split("\n").forEach((l) => { const m = REG.exec(l); if (!m) return; const n = m[2].trim(); if (!m[1]) st.push(n); else if (st[st.length - 1] === n) st.pop(); else mism++; });
  if (st.length || mism) { bad++; console.log("UNBALANCED", f, st, mism); }
}
const missing = [...o.keys()].filter((n) => !all.has(n));
console.log("files", files.length, "orig distinct", o.size, "tree distinct", all.size, "missing", missing, "unbalanced files", bad);
