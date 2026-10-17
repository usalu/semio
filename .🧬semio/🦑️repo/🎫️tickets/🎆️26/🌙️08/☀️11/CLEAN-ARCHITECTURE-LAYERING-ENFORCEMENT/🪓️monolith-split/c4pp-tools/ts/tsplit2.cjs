// usage: node tsplit2.cjs <spec.json> [APPLY]
// Splits one vitest file into sibling test files (`<siblingDir>/<dir>/🟦️.<ext>`) plus ONE shared support module.
// spec: { source, siblingDir, support: { dir, doc, ranges }, children: [{ dir, doc, ranges, root?: true }], ext }
// - ranges are 1-based inclusive line ranges of the source; the text is copied verbatim.
// - support: every top-level declaration it holds is exported when a child (or nothing else) uses it; children import the names they use from it.
// - open `//#region` markers are re-opened at the start of a range and closed at its end, so every output file is balanced.
// - a child with `root: true` is written over the source file itself (written last).
// - a top-level declaration used by a child but declared outside its ranges and outside the support ranges is an error.
const fs = require("fs");
const path = require("path");
const ts = require("/Users/ueli/Documents/semio/node_modules/typescript");
const spec = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const APPLY = process.argv[3] === "APPLY";
const text = fs.readFileSync(spec.source, "utf8");
const lines = text.split("\n");
const sf = ts.createSourceFile(spec.source, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX);
const lineOf = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
const errors = [];
const regionRe = /^[ \t]*\/\/\s*#(end)?region\s+(.*?)\s*$/;

const host = ts.createCompilerHost({});
host.getSourceFile = (n) => (n === spec.source ? sf : undefined);
host.fileExists = (n) => n === spec.source;
host.readFile = (n) => (n === spec.source ? text : undefined);
const program = ts.createProgram([spec.source], { noResolve: true, noLib: true, target: ts.ScriptTarget.Latest, jsx: ts.JsxEmit.Preserve }, host);
const checker = program.getTypeChecker();
const imports = sf.statements.filter(ts.isImportDeclaration);
const ownerOfSym = new Map();
const importOfSym = new Map();
for (const s of sf.statements) {
  if (ts.isImportDeclaration(s)) {
    const c = s.importClause; if (!c) continue;
    if (c.name) importOfSym.set(checker.getSymbolAtLocation(c.name), c.name.text);
    if (c.namedBindings && ts.isNamespaceImport(c.namedBindings)) importOfSym.set(checker.getSymbolAtLocation(c.namedBindings.name), c.namedBindings.name.text);
    if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const el of c.namedBindings.elements) importOfSym.set(checker.getSymbolAtLocation(el.name), el.name.text);
    continue;
  }
  const addName = (nameNode) => { if (ts.isIdentifier(nameNode)) { const sy = checker.getSymbolAtLocation(nameNode); if (sy) ownerOfSym.set(sy, nameNode.text); } else for (const el of nameNode.elements) if (!ts.isOmittedExpression(el)) addName(el.name); };
  if (ts.isVariableStatement(s)) for (const d of s.declarationList.declarations) addName(d.name);
  else if (s.name) { const sy = checker.getSymbolAtLocation(s.name); if (sy) ownerOfSym.set(sy, s.name.getText(sf)); }
}
const refsOfStmt = new Map();
for (const s of sf.statements) {
  if (ts.isImportDeclaration(s)) continue;
  const tops = new Set(), imps = new Set();
  const visit = (n) => {
    if (ts.isIdentifier(n)) {
      let sy = checker.getSymbolAtLocation(n);
      if (!sy && n.parent && ts.isShorthandPropertyAssignment(n.parent)) sy = checker.getShorthandAssignmentValueSymbol(n.parent);
      if (sy && ownerOfSym.has(sy)) tops.add(ownerOfSym.get(sy));
      if (sy && importOfSym.has(sy) && !ts.isImportSpecifier(n.parent)) imps.add(importOfSym.get(sy));
    }
    ts.forEachChild(n, visit);
  };
  visit(s);
  refsOfStmt.set(lineOf(s.getStart(sf, true)), { tops, imps });
}
const refsInRanges = (rs) => {
  const tops = new Set(), imps = new Set();
  for (const [start, r] of refsOfStmt) if (rs.some(([a, b]) => start >= a && start <= b)) { r.tops.forEach((x) => tops.add(x)); r.imps.forEach((x) => imps.add(x)); }
  return { tops, imps };
};
const topDecls = new Map();
const stmtInfo = [];
for (const s of sf.statements) {
  if (ts.isImportDeclaration(s)) continue;
  const names = ts.isVariableStatement(s)
    ? s.declarationList.declarations.flatMap((d) => (ts.isIdentifier(d.name) ? [d.name.text] : d.name.elements.map((e) => e.name.getText(sf))))
    : s.name ? [s.name.getText(sf)] : [];
  const info = { start: lineOf(s.getStart(sf, true)), bodyStart: lineOf(s.getStart(sf, false)), end: lineOf(s.getEnd()), names };
  stmtInfo.push(info);
  for (const n of names) topDecls.set(n, info);
}

const markerText = new Map();
const stackAt = new Array(lines.length + 2);
{
  const stack = [];
  for (let i = 1; i <= lines.length; i++) {
    stackAt[i] = [...stack];
    const m = lines[i - 1].match(regionRe);
    if (!m) continue;
    if (!m[1]) { stack.push(m[2]); if (!markerText.has("s:" + m[2])) markerText.set("s:" + m[2], lines[i - 1].trim()); }
    else { if (stack[stack.length - 1] === m[2]) stack.pop(); else errors.push(`unbalanced endregion ${m[2]} at ${i}`); if (!markerText.has("e:" + m[2])) markerText.set("e:" + m[2], lines[i - 1].trim()); }
  }
  stackAt[lines.length + 1] = [...stack];
}

function rangeText(a, b, exportLines) {
  const local = [...stackAt[a]];
  const out = [];
  for (const n of local) out.push(markerText.get("s:" + n));
  for (let i = a; i <= b; i++) {
    let l = lines[i - 1];
    const m = l.match(regionRe);
    if (m) { if (!m[1]) local.push(m[2]); else if (local[local.length - 1] === m[2]) local.pop(); }
    if (exportLines && exportLines.has(i)) l = "export " + l;
    out.push(l);
  }
  for (let k = local.length - 1; k >= 0; k--) out.push(markerText.get("e:" + local[k]));
  return out.join("\n");
}

const identifiersOf = (bodyText) => {
  const used = new Set();
  const bsf = ts.createSourceFile("x.tsx", bodyText, ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX);
  const visit = (n) => { if (ts.isIdentifier(n)) used.add(n.text); ts.forEachChild(n, visit); };
  visit(bsf);
  return used;
};
const declaredIn = (rs) => {
  const set = new Set();
  for (const [n, d] of topDecls) if (rs.some(([a, b]) => d.start >= a && d.end <= b)) set.add(n);
  return set;
};

function narrowImports(used) {
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
    if (imp.attributes) { out.push(imp.getText(sf)); continue; }
    const bits = [];
    if (def) bits.push(def);
    if (ns) bits.push(`* as ${ns}`);
    if (named.length) bits.push(`{ ${named.join(", ")} }`);
    out.push(`import ${c.isTypeOnly ? "type " : ""}${bits.join(", ")} from ${q};`);
  }
  return out;
}

const ext = spec.ext || "tsx";
const supportNames = declaredIn(spec.support.ranges);
const childNames = spec.children.map((k) => declaredIn(k.ranges));
const supportTexts = [];
const usedFromSupport = new Set();
const outputs = [];
const supportDirAbs = path.join(spec.siblingDir, spec.support.dir);
const supportFile = path.join(supportDirAbs, "🟦️." + ext);

for (const [ci, k] of spec.children.entries()) {
  const body = k.ranges.map(([a, b]) => rangeText(a, b)).join("\n\n");
  const refs = refsInRanges(k.ranges);
  const used = new Set([...refs.tops, ...refs.imps]);
  const fromSupport = [];
  for (const n of refs.tops) {
    if (!topDecls.has(n) || childNames[ci].has(n)) continue;
    if (supportNames.has(n)) { fromSupport.push(n); usedFromSupport.add(n); }
    else errors.push(`child ${k.dir}: uses ${n} declared at ${topDecls.get(n).start}-${topDecls.get(n).end} outside its ranges and the support ranges`);
  }
  k._body = body; k._used = used; k._fromSupport = fromSupport.sort();
}
const exportLines = new Set();
for (const n of usedFromSupport) exportLines.add(topDecls.get(n).bodyStart);
{
  const body = spec.support.ranges.map(([a, b]) => rangeText(a, b, exportLines)).join("\n\n");
  const refs = refsInRanges(spec.support.ranges);
  const used = new Set([...refs.tops, ...refs.imps]);
  for (const n of refs.tops) if (!supportNames.has(n)) errors.push(`support: uses ${n} declared outside the support ranges`);
  const imps = narrowImports(used);
  outputs.push({ path: supportFile, text: `${spec.support.doc}\n${imps.join("\n")}\n\n${body.replace(/\s+$/, "")}\n` });
}
for (const k of spec.children) {
  const imps = narrowImports(k._used);
  if (k._fromSupport.length) imps.push(`import { ${k._fromSupport.join(", ")} } from "../${spec.support.dir}/🟦️.${ext}";`);
  const file = k.root ? spec.source : path.join(spec.siblingDir, k.dir, "🟦️." + ext);
  outputs.push({ path: file, root: !!k.root, text: `${k.doc}\n${imps.join("\n")}\n\n${k._body.replace(/\s+$/, "")}\n` });
}
for (const o of outputs) if (!/^\/\*\*\s*\S/.test(o.text)) errors.push(`${o.path}: docstring missing`);
const unused = [...supportNames].filter((n) => !usedFromSupport.has(n));
console.log(JSON.stringify({ errors, supportExports: [...usedFromSupport].length, supportPrivate: unused, outputs: outputs.map((o) => ({ path: path.relative(spec.siblingDir, o.path), lines: o.text.split("\n").length - 1 })) }, null, 1));
if (APPLY) {
  if (errors.length) { console.error("REFUSED"); process.exit(2); }
  if (fs.readFileSync(spec.source, "utf8") !== text) { console.error("source changed"); process.exit(5); }
  for (const o of outputs) if (!o.root && fs.existsSync(o.path)) { console.error("exists " + o.path); process.exit(4); }
  for (const o of outputs.filter((x) => !x.root)) { fs.mkdirSync(path.dirname(o.path), { recursive: true }); fs.writeFileSync(o.path, o.text); }
  for (const o of outputs.filter((x) => x.root)) fs.writeFileSync(o.path, o.text);
  console.log("APPLIED");
} else if (process.env.DUMP) {
  fs.mkdirSync(process.env.DUMP, { recursive: true });
  for (const o of outputs) fs.writeFileSync(path.join(process.env.DUMP, path.basename(path.dirname(o.path)) + "." + ext), o.text);
}
