import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import fs from "node:fs";
import path from "node:path";
const [root, kidsFile, repoRel] = process.argv.slice(2);
const kids = JSON.parse(fs.readFileSync(kidsFile, "utf8"));
const parent = path.join(root, "🟦️.tsx");
const files = [parent, ...kids.map((k) => ["🟦️.ts", "🟦️.tsx"].map((f) => path.join(root, k, f)).find(fs.existsSync))];
if (files.some((f) => !f)) throw new Error("missing file");
const set = new Set(files);
const adj = new Map();
let maxBytes = 0, maxPath = "";
for (const f of files) {
  const rel = repoRel ? path.relative(repoRel, f) : f; const b = Buffer.byteLength(rel); if (b > maxBytes) { maxBytes = b; maxPath = rel; }
  const sf = ts.createSourceFile(f, fs.readFileSync(f, "utf8"), ts.ScriptTarget.ESNext, true, ts.ScriptKind.TSX);
  const deps = new Set();
  for (const st of sf.statements) {
    const spec = (ts.isImportDeclaration(st) || ts.isExportDeclaration(st)) && st.moduleSpecifier ? st.moduleSpecifier.text : null;
    if (!spec || !spec.startsWith(".")) continue;
    const abs = path.resolve(path.dirname(f), spec.replace(/\?.*$/, ""));
    if (set.has(abs)) deps.add(abs);
  }
  adj.set(f, deps);
}
let idx = 0; const I = new Map(), L = new Map(), st = [], on = new Set(), sccs = [];
const sc = (v) => { I.set(v, idx); L.set(v, idx++); st.push(v); on.add(v); for (const w of adj.get(v)) { if (!I.has(w)) { sc(w); L.set(v, Math.min(L.get(v), L.get(w))); } else if (on.has(w)) L.set(v, Math.min(L.get(v), I.get(w))); } if (L.get(v) === I.get(v)) { const c = []; let w; do { w = st.pop(); on.delete(w); c.push(w); } while (w !== v); sccs.push(c); } };
for (const f of files) if (!I.has(f)) sc(f);
const bad = sccs.filter((c) => c.length > 1);
const intoParent = files.slice(1).filter((f) => adj.get(f).has(files[0]));
console.log("files", files.length, "SCCs>1:", bad.length, "children importing parent:", intoParent.length, "max path bytes", maxBytes, maxPath);
for (const f of files) console.log(String(fs.readFileSync(f, "utf8").split("\n").length).padStart(5), path.relative(root, f), "->", [...adj.get(f)].map((x) => path.basename(path.dirname(x))).join(" "));
