// usage: node testsplit.cjs <spec.json> [APPLY]
// Splits one vitest file into child test modules nested next to it plus ONE shared support module; the original path becomes an aggregator that imports every child for its side effects (the test path, its registrations and its test ids stay as they were).
// spec: { source (snapshot of the original), file (the live original), support: { dir, doc, ranges }, children: [{ dir, doc, ranges, wrapDescribe? }], rootDoc, keepRootImports? }
// - ranges are 1-based inclusive line ranges of the source, copied verbatim (comments and JSDoc included); wrapDescribe wraps the ranges in `describe("<name>", () => { ... });`.
// - imports are rebuilt per file from the identifiers the file uses (type modifiers kept); relative specifiers and relative path string literals get one more `../` because the child sits one folder deeper.
// - top-level declarations of the support ranges are exported when a child uses them; children import them from the support module.
const fs = require("fs");
const path = require("path");
const ts = require("/Users/ueli/Documents/semio/node_modules/typescript");

const spec = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const APPLY = process.argv[3] === "APPLY";
const text = fs.readFileSync(spec.source, "utf8");
const lines = text.split("\n");
const dir = path.dirname(spec.file);
const sf = ts.createSourceFile(spec.file, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);

const imports = sf.statements.filter((s) => ts.isImportDeclaration(s));
const importBindings = [];
for (const decl of imports) {
  const c = decl.importClause;
  const spec0 = decl.moduleSpecifier.text;
  if (!c) { importBindings.push({ decl, spec0, side: true }); continue; }
  if (c.name) importBindings.push({ decl, spec0, kind: "default", local: c.name.text, typeOnly: c.isTypeOnly });
  if (c.namedBindings && ts.isNamespaceImport(c.namedBindings)) importBindings.push({ decl, spec0, kind: "ns", local: c.namedBindings.name.text, typeOnly: c.isTypeOnly });
  if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const el of c.namedBindings.elements) importBindings.push({ decl, spec0, kind: "named", local: el.name.text, imported: el.propertyName ? el.propertyName.text : el.name.text, typeOnly: c.isTypeOnly || el.isTypeOnly });
}

const rangeText = (ranges) => ranges.map(([a, b]) => lines.slice(a - 1, b).join("\n")).join("\n\n");
const identifiers = (body) => {
  const s = ts.createSourceFile("x.ts", body, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const ids = new Set();
  const visit = (n) => { if (ts.isIdentifier(n)) ids.add(n.text); ts.forEachChild(n, visit); };
  visit(s);
  return ids;
};
const topLevelDecls = (body) => {
  const s = ts.createSourceFile("x.ts", body, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const names = [];
  for (const st of s.statements) {
    if (ts.isVariableStatement(st)) for (const d of st.declarationList.declarations) { if (ts.isIdentifier(d.name)) names.push(d.name.text); }
    else if ((ts.isFunctionDeclaration(st) || ts.isClassDeclaration(st) || ts.isInterfaceDeclaration(st) || ts.isTypeAliasDeclaration(st) || ts.isEnumDeclaration(st)) && st.name) names.push(st.name.text);
  }
  return names;
};
const rebaseSpec = (s) => (s.startsWith(".") ? "../" + s : s);
const rebaseLiterals = (body) => body.replace(/(["'`])(\.\.\/[^"'`\n]*)\1/gu, (m, q, p) => q + "../" + p + q);

const files = [];
const supportBody = rangeText(spec.support.ranges);
const supportDecls = topLevelDecls(supportBody);
const supportIds = identifiers(supportBody);
const bodies = spec.children.map((c) => {
  let body = rangeText(c.ranges);
  if (c.wrapDescribe) body = `describe(${JSON.stringify(c.wrapDescribe)}, () => {\n${body}\n});`;
  return body;
});
const usedSupport = new Set();
spec.children.forEach((c, i) => { const ids = identifiers(bodies[i]); for (const d of supportDecls) if (ids.has(d)) usedSupport.add(d); });

function importsFor(ids, selfDecls) {
  const out = [];
  const byDecl = new Map();
  for (const b of importBindings) {
    if (b.side) continue;
    if (!ids.has(b.local)) continue;
    if (selfDecls && selfDecls.has(b.local)) continue;
    if (!byDecl.has(b.decl)) byDecl.set(b.decl, []);
    byDecl.get(b.decl).push(b);
  }
  for (const [decl, bs] of byDecl) {
    const specOut = rebaseSpec(bs[0].spec0);
    const def = bs.find((b) => b.kind === "default");
    const ns = bs.find((b) => b.kind === "ns");
    const named = bs.filter((b) => b.kind === "named");
    const allTypeOnly = decl.importClause.isTypeOnly;
    const parts = [];
    if (def) parts.push(def.local);
    if (ns) parts.push(`* as ${ns.local}`);
    if (named.length) parts.push("{ " + named.map((b) => `${!allTypeOnly && b.typeOnly ? "type " : ""}${b.imported !== b.local ? b.imported + " as " : ""}${b.local}`).join(", ") + " }");
    out.push(`import ${allTypeOnly ? "type " : ""}${parts.join(", ")} from ${JSON.stringify(specOut)};`);
  }
  return out;
}

const result = [];
// support module
{
  const ids = supportIds;
  const hdr = importsFor(ids, new Set(supportDecls));
  let body = rebaseLiterals(supportBody);
  const sfS = ts.createSourceFile("s.ts", body, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const edits = [];
  for (const st of sfS.statements) {
    const names = ts.isVariableStatement(st) ? st.declarationList.declarations.filter((d) => ts.isIdentifier(d.name)).map((d) => d.name.text) : st.name ? [st.name.text] : [];
    if (names.some((n) => usedSupport.has(n)) && !(ts.getCombinedModifierFlags(st) & ts.ModifierFlags.Export)) edits.push(st.getStart(sfS, false));
  }
  for (const pos of edits.sort((a, b) => b - a)) body = body.slice(0, pos) + "export " + body.slice(pos);
  result.push({ rel: `${spec.support.dir}/🟦️.ts`, text: spec.support.doc + "\n" + hdr.join("\n") + (hdr.length ? "\n\n" : "\n") + body + "\n" });
}
spec.children.forEach((c, i) => {
  const body = rebaseLiterals(bodies[i]);
  const ids = identifiers(bodies[i]);
  const selfDecls = new Set(topLevelDecls(bodies[i]));
  const hdr = importsFor(ids, selfDecls);
  const sup = [...usedSupport].filter((d) => ids.has(d) && !selfDecls.has(d)).sort();
  if (sup.length) hdr.push(`import { ${sup.join(", ")} } from ${JSON.stringify("../" + spec.support.dir + "/🟦️.ts")};`);
  result.push({ rel: `${c.dir}/🟦️.ts`, text: c.doc + "\n" + hdr.join("\n") + (hdr.length ? "\n\n" : "\n") + body + "\n" });
});
const rootText = spec.rootDoc + "\n" + spec.children.map((c) => `import ${JSON.stringify("./" + c.dir + "/🟦️.ts")};`).join("\n") + "\n";
const out = { outputs: result.map((r) => ({ path: r.rel, lines: r.text.split("\n").length - 1 })).concat([{ path: "🟦️.ts (root)", lines: rootText.split("\n").length - 1 }]), usedSupport: [...usedSupport] };
console.log(JSON.stringify(out, null, 1));
if (APPLY) {
  const cur = fs.readFileSync(spec.file, "utf8");
  if (cur !== text) throw new Error("live file differs from the snapshot; refusing to write");
  for (const r of result) {
    const p = path.join(dir, r.rel);
    if (fs.existsSync(p)) throw new Error("exists " + p);
  }
  for (const r of result) { const p = path.join(dir, r.rel); fs.mkdirSync(path.dirname(p), { recursive: true }); fs.writeFileSync(p, r.text); }
  fs.writeFileSync(spec.file + ".tmp", rootText);
  fs.renameSync(spec.file + ".tmp", spec.file);
  console.log("APPLIED");
}
