// usage: node unbarrel.cjs <parent.tsx> <importers.json> [APPLY]
// Removes the `export { ... } from "<child>"` re-export lists of <parent> and re-points every importer to the module that owns each name.
// Named imports and named re-exports are rewritten; any other use of the parent (namespace import, default import, dynamic import, import type(), export *) is reported as manual.
const fs = require("fs");
const path = require("path");
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const parent = process.argv[2];
const importers = JSON.parse(fs.readFileSync(process.argv[3], "utf8"));
const APPLY = process.argv[4] === "APPLY";
const text = fs.readFileSync(parent, "utf8");
const kind = (p) => (p.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const sf = ts.createSourceFile(parent, text, ts.ScriptTarget.Latest, true, kind(parent));
const resolveSpec = (from, spec) => {
  const base = path.resolve(path.dirname(from), spec);
  for (const c of [base, base + ".ts", base + ".tsx", path.join(base, "🟦️.ts"), path.join(base, "🟦️.tsx")]) if (fs.existsSync(c) && fs.statSync(c).isFile()) return c;
  throw new Error("cannot resolve " + spec + " from " + from);
};
const owner = new Map();
const removals = [];
for (const s of sf.statements) {
  if (!ts.isExportDeclaration(s) || !s.moduleSpecifier || !s.exportClause || !ts.isNamedExports(s.exportClause)) continue;
  const spec = s.moduleSpecifier.text;
  if (!spec.startsWith(".")) continue;
  const target = resolveSpec(parent, spec);
  for (const el of s.exportClause.elements) owner.set(el.name.text, { target, original: (el.propertyName ?? el.name).text, type: el.isTypeOnly || s.isTypeOnly });
  removals.push([s.getFullStart(), s.getEnd()]);
}
const relFrom = (fromDir, target) => { let r = path.relative(fromDir, target).split(path.sep).join("/"); if (!r.startsWith(".")) r = "./" + r; return r; };
const manual = [];
const writes = [];
for (const p of importers) {
  if (p === parent || !fs.existsSync(p)) continue;
  const t = fs.readFileSync(p, "utf8");
  const s2 = ts.createSourceFile(p, t, ts.ScriptTarget.Latest, true, kind(p));
  const edits = [];
  const targetsParent = (spec) => spec.startsWith(".") && (() => { try { return resolveSpec(p, spec) === parent; } catch { return false; } })();
  const walk = (node) => {
    if ((ts.isImportDeclaration(node) || ts.isExportDeclaration(node)) && node.moduleSpecifier && ts.isStringLiteral(node.moduleSpecifier) && targetsParent(node.moduleSpecifier.text)) {
      const isImp = ts.isImportDeclaration(node);
      const clause = isImp ? node.importClause : node.exportClause;
      if (isImp && !node.importClause) {}
      else if (!isImp && !node.exportClause) manual.push(`${p}: export * from parent`);
      else {
        const nb = isImp ? clause.namedBindings : clause;
        if (isImp && clause.name) manual.push(`${p}: default import of parent`);
        if (nb && (ts.isNamespaceImport(nb) || ts.isNamespaceExport(nb))) manual.push(`${p}: namespace use of parent`);
        else if (nb) {
          const keep = [], groups = new Map();
          for (const e of nb.elements) {
            const nm = (e.propertyName ?? e.name).text;
            if (owner.has(nm)) { const o = owner.get(nm); if (!groups.has(o.target)) groups.set(o.target, []); groups.get(o.target).push({ e, o }); } else keep.push(e);
          }
          if (groups.size) {
            const q = node.moduleSpecifier.getText(s2)[0];
            const head = isImp ? "import " + (clause.isTypeOnly ? "type " : "") : "export " + (node.isTypeOnly ? "type " : "");
            const reps = [];
            const mk = (items, sp) => `${head}{ ${items.join(", ")} } from ${q}${sp}${q};`;
            if (keep.length) reps.push(mk(keep.map((e) => e.getText(s2)), node.moduleSpecifier.text));
            for (const [target, list] of [...groups].sort((a, b) => a[0].localeCompare(b[0]))) {
              const items = list.map(({ e, o }) => {
                const asLocal = e.name.text;
                const spelled = o.original === asLocal ? asLocal : `${o.original} as ${asLocal}`;
                return (e.isTypeOnly ? "type " : "") + spelled;
              });
              reps.push(mk(items, relFrom(path.dirname(p), target)));
            }
            const ls = t.lastIndexOf("\n", node.getStart(s2));
            const indent = t.slice(ls + 1, node.getStart(s2));
            edits.push({ start: node.getStart(s2), end: node.getEnd(), text: reps.join("\n" + indent) });
          }
        }
      }
    }
    if (ts.isCallExpression(node) && node.arguments[0] && ts.isStringLiteral(node.arguments[0]) && (node.expression.kind === ts.SyntaxKind.ImportKeyword || (ts.isPropertyAccessExpression(node.expression) && node.expression.expression.getText(s2) === "vi" && ["mock", "doMock", "importActual"].includes(node.expression.name.text))) && targetsParent(node.arguments[0].text))
      manual.push(`${p}: ${node.expression.getText(s2)}() of parent at line ${s2.getLineAndCharacterOfPosition(node.getStart(s2)).line + 1}`);
    if (ts.isImportTypeNode(node) && ts.isLiteralTypeNode(node.argument) && ts.isStringLiteral(node.argument.literal) && targetsParent(node.argument.literal.text)) manual.push(`${p}: import() type of parent`);
    ts.forEachChild(node, walk);
  };
  walk(s2);
  if (!edits.length) continue;
  let r = t;
  for (const e of edits.sort((a, b) => b.start - a.start)) r = r.slice(0, e.start) + e.text + r.slice(e.end);
  writes.push({ path: p, text: r, count: edits.length });
}
let newParent = text;
for (const [a, b] of removals.sort((x, y) => y[0] - x[0])) newParent = newParent.slice(0, a) + newParent.slice(b);
newParent = newParent.replace(/\n{3,}/g, "\n\n");
console.log(JSON.stringify({ owners: owner.size, removedStatements: removals.length, manual, rewritten: writes.map((w) => path.basename(path.dirname(w.path)) + " (" + w.count + ")"), parentLines: newParent.split("\n").length }, null, 1));
if (APPLY) {
  if (manual.length && !process.env.ALLOW_MANUAL) { console.error("REFUSED: manual sites present"); process.exit(3); }
  if (fs.readFileSync(parent, "utf8") !== text) { console.error("parent changed"); process.exit(5); }
  for (const w of writes) fs.writeFileSync(w.path, w.text);
  fs.writeFileSync(parent, newParent);
  console.log("APPLIED");
}
