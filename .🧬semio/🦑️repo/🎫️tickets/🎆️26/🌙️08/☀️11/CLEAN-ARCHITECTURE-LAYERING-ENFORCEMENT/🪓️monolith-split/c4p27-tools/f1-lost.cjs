const fs = require("fs");
const ts = require("/Users/ueli/Documents/semio/node_modules/typescript");
const [orig, ...news] = process.argv.slice(2);
const norm = (l) => l.trim().replace(/^export\s+(default\s+)?/, "");
function lines(p, dropExportFrom) {
  const text = fs.readFileSync(p, "utf8");
  const sf = ts.createSourceFile(p, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX);
  let out = text; const cuts = [];
  sf.statements.forEach((s) => { if (ts.isImportDeclaration(s) || (dropExportFrom && ts.isExportDeclaration(s) && s.moduleSpecifier)) cuts.push([s.getStart(sf), s.getEnd()]); });
  cuts.sort((a, b) => b[0] - a[0]).forEach(([a, b]) => { out = out.slice(0, a) + out.slice(b); });
  return out.split("\n").map((l) => l.trim()).filter((l) => l && !/^\/\/\s*#(end)?region/.test(l)).map(norm);
}
const count = (a) => { const m = new Map(); a.forEach((x) => m.set(x, (m.get(x) || 0) + 1)); return m; };
const o = count(lines(orig, true)); const n = new Map();
for (const p of news) for (const [k, v] of count(lines(p, true))) n.set(k, (n.get(k) || 0) + v);
let lost = 0, added = 0; const L = [], A = [];
for (const [k, v] of o) { const w = n.get(k) || 0; if (v > w) { lost += v - w; L.push(k); } }
for (const [k, v] of n) { const w = o.get(k) || 0; if (v > w) { added += v - w; A.push(k); } }
console.log("excluding import and export-from declarations: lost", lost, "added", added);
L.forEach((x) => console.log("  LOST ", x.slice(0, 150)));
A.forEach((x) => console.log("  ADDED", x.slice(0, 110)));
