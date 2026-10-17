// usage: node tsplit.cjs <spec.json> [APPLY]
// Splits a self-contained vitest file into sibling test files by line ranges (whole-file text copy).
// spec: { source, siblingDir, header: "<path comment>", children: [{ dir, doc, ranges: [[a,b]...] }], replace?: [[from,to]], deleteSource?: true }
// Every child gets: Header region (path comment + docstring), Adapters region (the original import declarations narrowed to the identifiers the child text uses), then its ranges.
// A top-level declaration used by a child but declared outside its ranges is an error.
const fs = require("fs");
const path = require("path");
const ts = require("/Users/ueli/Documents/semio/node_modules/typescript");
const spec = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const APPLY = process.argv[3] === "APPLY";
const text = fs.readFileSync(spec.source, "utf8");
const lines = text.split("\n");
const sf = ts.createSourceFile(spec.source, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const lineOf = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
const errors = [];

const imports = sf.statements.filter(ts.isImportDeclaration);
const topDecls = new Map();
for (const s of sf.statements) {
  if (ts.isImportDeclaration(s)) continue;
  const names = ts.isVariableStatement(s) ? s.declarationList.declarations.map((d) => d.name.getText(sf)) : s.name ? [s.name.getText(sf)] : [];
  for (const n of names) topDecls.set(n, { start: lineOf(s.getStart(sf, true)), end: lineOf(s.getEnd()) });
}

function narrowImports(bodyText) {
  const used = new Set();
  const bsf = ts.createSourceFile("x.ts", bodyText, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const visit = (n) => { if (ts.isIdentifier(n)) used.add(n.text); ts.forEachChild(n, visit); };
  visit(bsf);
  const out = [];
  for (const imp of imports) {
    const c = imp.importClause;
    const q = imp.moduleSpecifier.getText(sf);
    if (!c) { out.push(imp.getText(sf)); continue; }
    const def = c.name && used.has(c.name.text) ? c.name.text : null;
    let ns = null;
    const named = [];
    if (c.namedBindings && ts.isNamespaceImport(c.namedBindings) && used.has(c.namedBindings.name.text)) ns = c.namedBindings.name.text;
    if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const el of c.namedBindings.elements) if (used.has(el.name.text)) named.push(el.getText(sf));
    if (!def && !ns && !named.length) continue;
    const bits = [];
    if (def) bits.push(def);
    if (ns) bits.push(`* as ${ns}`);
    if (named.length) bits.push(`{ ${named.join(", ")} }`);
    out.push(`import ${c.isTypeOnly ? "type " : ""}${bits.join(", ")} from ${q};`);
  }
  return out;
}

const outputs = [];
for (const k of spec.children) {
  const chunks = [];
  const covered = new Set();
  for (const [a, b] of k.ranges) {
    chunks.push(lines.slice(a - 1, b).join("\n"));
    for (let i = a; i <= b; i++) covered.add(i);
  }
  let body = chunks.join("\n\n");
  for (const [from, to] of spec.replace || []) body = body.split(from).join(to);
  const used = new Set();
  const bsf = ts.createSourceFile("x.ts", body, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const visit = (n) => { if (ts.isIdentifier(n)) used.add(n.text); ts.forEachChild(n, visit); };
  visit(bsf);
  const declaredHere = new Set();
  const bodyDecls = [];
  for (const s of bsf.statements) {
    const names = ts.isVariableStatement(s) ? s.declarationList.declarations.map((d) => d.name.getText(bsf)) : s.name ? [s.name.getText(bsf)] : [];
    names.forEach((n) => declaredHere.add(n));
  }
  for (const [n, d] of topDecls) {
    if (used.has(n) && !declaredHere.has(n)) {
      const inside = [...covered].some((l) => l >= d.start && l <= d.end);
      if (!inside) errors.push(`child ${k.dir}: uses ${n} declared at ${d.start}-${d.end} outside its ranges`);
    }
  }
  const imps = narrowImports(body);
  const hdr = `// #region 🧲️Header\n// 🎨️ ${spec.header.replace("<dir>", k.dir)}\n${k.doc}\n// #endregion 🧲️Header\n\n// #region 🔌️Adapters\n${imps.join("\n")}\n// #endregion 🔌️Adapters\n\n`;
  outputs.push({ path: path.join(spec.siblingDir, k.dir, "🟦️.ts"), text: hdr + body.replace(/\s+$/, "") + "\n" });
}
for (const o of outputs) {
  if (!/^\/\*\*\s*\S/.test(o.text.split("\n")[2] || "")) errors.push(`${o.path}: docstring missing`);
}
console.log(JSON.stringify({ errors, outputs: outputs.map((o) => ({ path: path.relative(spec.siblingDir, o.path), lines: o.text.split("\n").length - 1 })) }, null, 1));
if (APPLY) {
  if (errors.length) { console.error("REFUSED"); process.exit(2); }
  if (fs.readFileSync(spec.source, "utf8") !== text) { console.error("source changed"); process.exit(5); }
  for (const o of outputs) if (fs.existsSync(o.path)) { console.error("exists " + o.path); process.exit(4); }
  for (const o of outputs) { fs.mkdirSync(path.dirname(o.path), { recursive: true }); fs.writeFileSync(o.path, o.text); }
  if (spec.deleteSource) fs.unlinkSync(spec.source);
  console.log("APPLIED");
} else if (process.env.DUMP) {
  fs.mkdirSync(process.env.DUMP, { recursive: true });
  for (const o of outputs) fs.writeFileSync(path.join(process.env.DUMP, path.basename(path.dirname(o.path)) + ".ts"), o.text);
}
