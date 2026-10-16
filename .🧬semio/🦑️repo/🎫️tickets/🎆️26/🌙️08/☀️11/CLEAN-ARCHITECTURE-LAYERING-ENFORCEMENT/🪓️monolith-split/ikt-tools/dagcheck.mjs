import fs from "node:fs";
import path from "node:path";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
const root = process.argv[2];
const walk = (d) => fs.readdirSync(d, { withFileTypes: true }).flatMap((e) => (e.isDirectory() ? (["🧪️tests", "🧫️fixtures", "node_modules", "📖️stories", "🎯️targets", "🧬️schema"].includes(e.name) ? [] : walk(path.join(d, e.name))) : /^🟦️\.tsx?$/.test(e.name) ? [path.join(d, e.name)] : []));
const files = walk(root);
const set = new Set(files);
const adj = new Map();
for (const f of files) {
  const sf = ts.createSourceFile(f, fs.readFileSync(f, "utf8"), ts.ScriptTarget.ESNext, true, ts.ScriptKind.TSX);
  const deps = new Set();
  for (const st of sf.statements) {
    const spec = (ts.isImportDeclaration(st) || ts.isExportDeclaration(st)) && st.moduleSpecifier ? st.moduleSpecifier.text : null;
    if (!spec || !spec.startsWith(".")) continue;
    const abs = path.resolve(path.dirname(f), spec);
    if (set.has(abs)) deps.add(abs);
  }
  adj.set(f, deps);
}
let idx = 0; const I = new Map(), L = new Map(), st = [], on = new Set(), sccs = [];
const sc = (v) => { I.set(v, idx); L.set(v, idx++); st.push(v); on.add(v); for (const w of adj.get(v)) { if (!I.has(w)) { sc(w); L.set(v, Math.min(L.get(v), L.get(w))); } else if (on.has(w)) L.set(v, Math.min(L.get(v), I.get(w))); } if (L.get(v) === I.get(v)) { const c = []; let w; do { w = st.pop(); on.delete(w); c.push(w); } while (w !== v); sccs.push(c); } };
for (const f of files) if (!I.has(f)) sc(f);
const bad = sccs.filter((c) => c.length > 1);
console.log("files", files.length, "SCCs>1:", bad.length);
for (const c of bad) console.log(" cycle:", c.map((f) => path.relative(root, f)).join(" <-> "));
if (process.argv[3] === "edges") for (const f of files) console.log(path.relative(root, f), "->", [...adj.get(f)].map((x) => path.relative(root, path.dirname(x))).join(" "));
