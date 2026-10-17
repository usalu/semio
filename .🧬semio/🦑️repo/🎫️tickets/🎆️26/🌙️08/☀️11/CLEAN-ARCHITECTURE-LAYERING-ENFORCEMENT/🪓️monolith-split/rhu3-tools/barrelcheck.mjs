import fs from "node:fs";
const txt = fs.readFileSync(process.argv[2], "utf8").split("\n");
const ex = new Set(JSON.parse(fs.readFileSync(process.argv[3], "utf8"))[process.argv[4]]);
let cur = null, to = null; const edges = [];
for (let i = 0; i < txt.length; i++) {
  const l = txt[i];
  if (l.startsWith("E/")) cur = l; else if (l.startsWith("   -> ")) to = l.slice(6); else if (l.startsWith("      ")) {
    const names = l.trim().split(", ").filter(Boolean).map(n => n.replace(/^type /, "")).map(n => n.replace(/^export\{(.*)\}$/, "$1"));
    edges.push({ from: cur, to, names });
  }
}
const miss = new Map(); let ok = 0, bad = 0;
for (const e of edges) {
  const flat = e.names.flatMap(n => n.includes(",") ? n.split(",") : [n]);
  const m = flat.filter(n => !ex.has(n) && !n.startsWith("<"));
  if (m.length === 0 && !flat.some(n => n.startsWith("<"))) ok++; else { bad++; const k = e.to; const s = miss.get(k) ?? new Set(); m.forEach(x => s.add(x)); if (flat.some(n => n.startsWith("<"))) s.add("<dyn/side>"); miss.set(k, s); }
}
console.log("edges", edges.length, "all-in-barrel", ok, "with-missing", bad);
for (const [k, s] of [...miss].sort()) console.log(k, "\n    ", [...s].join(", "));
