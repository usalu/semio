// usage: node repoint.cjs <root.ts> <importers.json> [APPLY]
// For a split module (root + direct child folders holding 🟦️.ts): re-points every named import/export of the root or of a child to the file that exports that name now.
const fs = require("fs");
const path = require("path");
const ts = require("/Users/ueli/Documents/semio/node_modules/typescript");
const root = process.argv[2];
const importers = JSON.parse(fs.readFileSync(process.argv[3], "utf8"));
const APPLY = process.argv[4] === "APPLY";
const dir = path.dirname(root);
const members = [root];
for (const e of fs.readdirSync(dir, { withFileTypes: true })) if (e.isDirectory()) { const f = path.join(dir, e.name, "🟦️.ts"); if (fs.existsSync(f) && !(process.env.SKIP || "").split(",").includes(e.name)) members.push(f); }
const kind = (p) => (p.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const owner = new Map();
for (const m of members) {
  const sf = ts.createSourceFile(m, fs.readFileSync(m, "utf8"), ts.ScriptTarget.Latest, true, kind(m));
  for (const s of sf.statements) {
    const exp = !!(ts.getCombinedModifierFlags(s) & ts.ModifierFlags.Export);
    if (exp) {
      if (ts.isVariableStatement(s)) for (const d of s.declarationList.declarations) owner.set(d.name.getText(sf), m);
      else if (s.name) owner.set(s.name.text, m);
    } else if (ts.isExportDeclaration(s) && !s.moduleSpecifier && s.exportClause && ts.isNamedExports(s.exportClause)) for (const el of s.exportClause.elements) owner.set(el.name.text, m);
  }
}
const resolveSpec = (from, spec) => { const base = path.resolve(path.dirname(from), spec); for (const c of [base, base + ".ts", path.join(base, "🟦️.ts")]) if (fs.existsSync(c) && fs.statSync(c).isFile()) return c; return null; };
const relFrom = (fromDir, target) => { let r = path.relative(fromDir, target).split(path.sep).join("/"); if (!r.startsWith(".")) r = "./" + r; return r; };
const memberSet = new Set(members);
let touched = 0; const manual = [];
for (const p of importers) {
  if (!fs.existsSync(p) || memberSet.has(p) && process.env.CHILDREN !== "1") continue;
  const t = fs.readFileSync(p, "utf8");
  const sf = ts.createSourceFile(p, t, ts.ScriptTarget.Latest, true, kind(p));
  const edits = [];
  for (const node of sf.statements) {
    if (!(ts.isImportDeclaration(node) || ts.isExportDeclaration(node)) || !node.moduleSpecifier || !ts.isStringLiteral(node.moduleSpecifier) || !node.moduleSpecifier.text.startsWith(".")) continue;
    const target = resolveSpec(p, node.moduleSpecifier.text);
    if (!target || !memberSet.has(target)) continue;
    const clause = ts.isImportDeclaration(node) ? node.importClause : node.exportClause;
    const nb = ts.isImportDeclaration(node) ? clause && clause.namedBindings : clause;
    if (!nb || !(ts.isNamedImports(nb) || ts.isNamedExports(nb)) || (ts.isImportDeclaration(node) && clause.name)) { manual.push(p + ": " + node.getText(sf).slice(0, 80)); continue; }
    const groups = new Map();
    let unknown = false;
    for (const el of nb.elements) {
      const orig = (el.propertyName ?? el.name).text;
      const o = owner.get(orig) || (memberSet.has(target) ? null : null);
      if (!o) { unknown = true; manual.push(p + ": unknown name " + orig); continue; }
      if (!groups.has(o)) groups.set(o, []);
      groups.get(o).push(el);
    }
    if (unknown) continue;
    if (groups.size === 1 && groups.has(target)) continue;
    const q = node.moduleSpecifier.getText(sf)[0];
    const head = ts.isImportDeclaration(node) ? "import " + (clause.isTypeOnly ? "type " : "") : "export " + (node.isTypeOnly ? "type " : "");
    const reps = [...groups.entries()].sort((a, b) => members.indexOf(a[0]) - members.indexOf(b[0])).map(([o, els]) => `${head}{ ${els.map((e) => e.getText(sf)).join(", ")} } from ${q}${relFrom(path.dirname(p), o)}${q};`);
    const ls = t.lastIndexOf("\n", node.getStart(sf));
    const indent = t.slice(ls + 1, node.getStart(sf));
    edits.push({ start: node.getStart(sf), end: node.getEnd(), text: reps.join("\n" + indent) });
  }
  if (!edits.length) continue;
  let r = t;
  for (const e of edits.sort((a, b) => b.start - a.start)) r = r.slice(0, e.start) + e.text + r.slice(e.end);
  touched++;
  console.log((APPLY ? "write " : "would write ") + path.relative("/Users/ueli/Documents/semio", p) + " (" + edits.length + ")");
  if (APPLY) fs.writeFileSync(p, r);
}
console.log(JSON.stringify({ names: owner.size, touched, manual }, null, 1));
