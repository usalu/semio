import fs from "node:fs";
import path from "node:path";
const [dir, root] = process.argv.slice(2);
const REG = /^[ \t]*\/\/\s*#(end)?region\b\s*(.*)$/;
const walk = (d) => fs.readdirSync(d, { withFileTypes: true }).flatMap((e) => (e.isDirectory() ? walk(path.join(d, e.name)) : [path.join(d, e.name)]));
let bad = 0;
for (const f of walk(dir)) {
  const t = fs.readFileSync(f, "utf8");
  const disk = fs.readFileSync(path.join(root, path.relative(dir, f)), "utf8");
  const stripped = t.split("\n").filter((l) => !REG.test(l) || l.includes("🧲️Header")).join("\n");
  const diskStripped = disk.split("\n").filter((l) => !REG.test(l) || l.includes("🧲️Header")).join("\n");
  const same = stripped === diskStripped;
  const st = []; let mism = 0;
  t.split("\n").forEach((l) => { const m = REG.exec(l); if (!m) return; const n = m[2].trim(); if (!m[1]) st.push(n); else if (st[st.length - 1] === n) st.pop(); else mism++; });
  if (!same || st.length || mism) bad++;
  console.log(path.relative(dir, f).slice(0, 50).padEnd(50), "markers-only-diff:", same, "unclosed", st.length, "mismatch", mism);
}
console.log("bad files", bad);
