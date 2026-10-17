const fs = require("fs"), path = require("path");
const ts = require("/Users/ueli/Documents/semio/node_modules/typescript");
const files = new Set(process.argv.slice(2));
const edges = new Map();
const resolve = (from, spec) => { const b = path.resolve(path.dirname(from), spec); for (const c of [b, b + ".ts", b + ".tsx"]) if (files.has(c)) return c; return null; };
for (const f of files) {
  const t = fs.readFileSync(f, "utf8");
  const sf = ts.createSourceFile(f, t, ts.ScriptTarget.Latest, true, f.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const out = new Set();
  sf.statements.forEach((s) => {
    if ((ts.isImportDeclaration(s) || ts.isExportDeclaration(s)) && s.moduleSpecifier && s.moduleSpecifier.text.startsWith(".")) {
      if (ts.isImportDeclaration(s) && s.importClause && s.importClause.isTypeOnly) return;
      const r = resolve(f, s.moduleSpecifier.text); if (r) out.add(r);
    }
  });
  edges.set(f, out);
}
let index = 0; const stack = [], idx = new Map(), low = new Map(), on = new Set(), sccs = [];
const strong = (v) => { idx.set(v, index); low.set(v, index); index++; stack.push(v); on.add(v); for (const w of edges.get(v)) { if (!idx.has(w)) { strong(w); low.set(v, Math.min(low.get(v), low.get(w))); } else if (on.has(w)) low.set(v, Math.min(low.get(v), idx.get(w))); } if (low.get(v) === idx.get(v)) { const c = []; let w; do { w = stack.pop(); on.delete(w); c.push(w); } while (w !== v); if (c.length > 1) sccs.push(c); } };
for (const v of files) if (!idx.has(v)) strong(v);
console.log("files", files.size, "SCCs>1:", sccs.length);
for (const c of sccs) console.log(c.map((x) => x.split("elements/")[1]));
