// usage: node barrelsplit.cjs <spec.json> [APPLY]
// Splits a package entry module made of imports plus region-wise `export { .. }` lists into region-group children; the entry keeps its docstring and the lines named in `keep`, and re-exports every child with `export * from "./<dir>/🟦️.ts"` (the package export map points at the entry, so its API surface stays the same).
// spec: { source (snapshot), file (live entry), keep: [[a,b]...] (root lines), children: [{ dir, doc, ranges }] }
// Imports are rebuilt per child from the identifiers its statements use (export specifiers included); an import declaration none of whose bindings is used by any statement is dropped (a named import without a binding use has no runtime effect under isolatedModules; side-effect-only imports stay in the first child); relative specifiers get one more `../`.
const fs = require("fs");
const path = require("path");
const ts = require("/Users/ueli/Documents/semio/node_modules/typescript");

const spec = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const APPLY = process.argv[3] === "APPLY";
const text = fs.readFileSync(spec.source, "utf8");
const lines = text.split("\n");
const dir = path.dirname(spec.file);
const isTsx = spec.file.endsWith(".tsx");
const sf = ts.createSourceFile(spec.file, text, ts.ScriptTarget.Latest, true, isTsx ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;

const inRange = (n, ranges) => ranges.some(([a, b]) => n >= a && n <= b);
const imports = sf.statements.filter((s) => ts.isImportDeclaration(s));
const nonImports = sf.statements.filter((s) => !ts.isImportDeclaration(s));
const groupOf = new Map();
const keepIdx = new Set();
nonImports.forEach((s, i) => {
  const start = L(s.getStart(sf, true));
  if (inRange(start, spec.keep)) { keepIdx.add(i); return; }
  const gi = spec.children.findIndex((c) => inRange(start, c.ranges));
  if (gi < 0) throw new Error(`statement at line ${start} belongs to no group`);
  groupOf.set(i, gi);
});

const bindingLocalNames = (decl) => {
  const c = decl.importClause;
  const names = [];
  if (!c) return names;
  if (c.name) names.push(c.name.text);
  if (c.namedBindings && ts.isNamespaceImport(c.namedBindings)) names.push(c.namedBindings.name.text);
  if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const el of c.namedBindings.elements) names.push(el.name.text);
  return names;
};

// identifiers used by statements (the module-level scope: any identifier occurrence that is not a declaration name of an import)
const idsOf = (stmt) => {
  const ids = new Set();
  if (ts.isExportDeclaration(stmt) && stmt.moduleSpecifier) return ids;
  const visit = (n) => {
    if (ts.isIdentifier(n)) {
      const p = n.parent;
      const isMemberName = p && ((ts.isPropertyAccessExpression(p) && p.name === n) || (ts.isPropertyAssignment(p) && p.name === n) || (ts.isPropertySignature(p) && p.name === n) || (ts.isQualifiedName(p) && p.right === n) || (ts.isExportSpecifier(p) && p.propertyName && p.name === n));
      if (!isMemberName) ids.add(n.text);
    }
    ts.forEachChild(n, visit);
  };
  visit(stmt);
  return ids;
};
const stmtIds = nonImports.map(idsOf);
const usedBy = new Map(); // import local name -> Set(group index)
for (const decl of imports) for (const nm of bindingLocalNames(decl)) usedBy.set(nm, new Set());
nonImports.forEach((s, i) => {
  const gi = keepIdx.has(i) ? -1 : groupOf.get(i);
  for (const id of stmtIds[i]) if (usedBy.has(id)) usedBy.get(id).add(gi);
});

const rebase = (s) => (s.startsWith(".") ? "../" + s : s);
const importLinesFor = (gi) => {
  const out = [];
  imports.forEach((decl) => {
    const c = decl.importClause;
    const spec0 = decl.moduleSpecifier.text;
    if (!c) { if (gi === 0) out.push(`import ${JSON.stringify(rebase(spec0))};`); return; }
    const names = bindingLocalNames(decl);
    const anyUsed = names.some((nm) => usedBy.get(nm).size > 0);
    if (!anyUsed) return;
    const mine = (nm) => usedBy.get(nm).has(gi);
    const parts = [];
    if (c.name && mine(c.name.text)) parts.push(c.name.text);
    if (c.namedBindings && ts.isNamespaceImport(c.namedBindings) && mine(c.namedBindings.name.text)) parts.push(`* as ${c.namedBindings.name.text}`);
    if (c.namedBindings && ts.isNamedImports(c.namedBindings)) {
      const els = c.namedBindings.elements.filter((el) => mine(el.name.text));
      if (els.length) parts.push("{ " + els.map((el) => `${el.isTypeOnly && !c.isTypeOnly ? "type " : ""}${el.propertyName ? el.propertyName.text + " as " : ""}${el.name.text}`).join(", ") + " }");
    }
    if (!parts.length) return;
    out.push(`import ${c.isTypeOnly ? "type " : ""}${parts.join(", ")} from ${JSON.stringify(rebase(spec0))};`);
  });
  return out;
};

// statements text: from the first statement start (comments included) through the last statement end of each contiguous run, keeping the original blank/comment/region lines between statements of the group's ranges
const importSpans = imports.map((d) => [L(d.getStart(sf, false)), L(d.getEnd())]);
const groupText = (gi) => {
  const parts = [];
  for (const [ra, rb] of spec.children[gi].ranges) {
    const keep = [];
    for (let n = ra; n <= rb; n += 1) { if (!importSpans.some(([x, y]) => n >= x && n <= y)) keep.push(lines[n - 1]); }
    parts.push(keep.join("\n"));
  }
  return parts.join("\n\n").replace(/\n{3,}/gu, "\n\n");
};

const rebaseExports = (body) => body.replace(/(from\s+)(["'])(\.\.?\/[^"']*)\2/gu, (m, f, q, p) => f + q + "../" + p + q).replace(/(import\(\s*)(["'])(\.\.?\/[^"']*)\2/gu, (m, f, q, p) => f + q + "../" + p + q);

const outs = spec.children.map((c, gi) => {
  const body = rebaseExports(groupText(gi));
  const hdr = importLinesFor(gi);
  return { rel: `${c.dir}/🟦️.${c.ext ?? "ts"}`, text: c.doc + "\n" + (hdr.length ? hdr.join("\n") + "\n\n" : "\n") + body + "\n" };
});
const keepText = spec.keep.map(([a, b]) => lines.slice(a - 1, b).join("\n")).join("\n");
const rootText = keepText + "\n\n" + spec.children.map((c) => `export * from ${JSON.stringify("./" + c.dir + "/🟦️." + (c.ext ?? "ts"))};`).join("\n") + "\n";

const unusedDecls = imports.filter((d) => d.importClause && bindingLocalNames(d).every((nm) => usedBy.get(nm).size === 0));
const keepUses = [...usedBy].filter(([nm, s]) => s.has(-1)).map(([nm]) => nm);
console.log(JSON.stringify({ outputs: outs.map((o) => ({ path: o.rel, lines: o.text.split("\n").length - 1 })).concat([{ path: "🟦️.tsx (entry)", lines: rootText.split("\n").length - 1 }]), unusedImportDeclarations: unusedDecls.length, importsUsedByKeptRootLines: keepUses }, null, 1));
if (APPLY) {
  if (fs.readFileSync(spec.file, "utf8") !== text) throw new Error("live file differs from the snapshot; refusing to write");
  for (const o of outs) { const p = path.join(dir, o.rel); if (fs.existsSync(p)) throw new Error("exists " + p); }
  for (const o of outs) { const p = path.join(dir, o.rel); fs.mkdirSync(path.dirname(p), { recursive: true }); fs.writeFileSync(p, o.text); }
  fs.writeFileSync(spec.file + ".tmp", rootText);
  fs.renameSync(spec.file + ".tmp", spec.file);
  console.log("APPLIED");
}
