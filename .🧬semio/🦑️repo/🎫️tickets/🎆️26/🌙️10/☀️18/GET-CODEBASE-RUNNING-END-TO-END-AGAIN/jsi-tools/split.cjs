// 🪓 usage: node split.cjs <spec.cjs> <out-dir>: splits the monolithic build inference module into wired component files.
// Statement-level and AST-driven: statements move verbatim; the tool only adds imports, the injected dependency block, `export` keywords and `loaded.` access for bootstrap state.
const ts = require("/Users/ueli/.cache/jsi/nm/node_modules/typescript");
const fs = require("fs");
const path = require("path");
const crypto = require("crypto");

const spec = require(path.resolve(process.argv[2]));
const outDir = path.resolve(process.argv[3]);
const sourcePath = path.resolve(spec.source);
const text = fs.readFileSync(sourcePath, "utf8");
const FE0F = "️";
const program = ts.createProgram([sourcePath], { allowJs: true, checkJs: false, noResolve: true, target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, noLib: true });
const sf = program.getSourceFile(sourcePath);
const checker = program.getTypeChecker();

const statements = sf.statements.map((s, index) => ({ s, index }));
const declNames = new Map();
const topLevelOf = new Map();

function bindingNames(name, out) {
  if (ts.isIdentifier(name)) out.push(name);
  else if (ts.isObjectBindingPattern(name) || ts.isArrayBindingPattern(name)) for (const e of name.elements) if (ts.isBindingElement(e)) bindingNames(e.name, out);
}
function statementDeclarations(s) {
  const names = [];
  if (ts.isVariableStatement(s)) for (const d of s.declarationList.declarations) bindingNames(d.name, names);
  else if ((ts.isFunctionDeclaration(s) || ts.isClassDeclaration(s)) && s.name) names.push(s.name);
  else if (ts.isImportDeclaration(s) && s.importClause) {
    const c = s.importClause;
    if (c.name) names.push(c.name);
    if (c.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) names.push(c.namedBindings.name); else for (const e of c.namedBindings.elements) names.push(e.name); }
  }
  return names;
}
for (const row of statements) {
  row.decls = statementDeclarations(row.s);
  for (const id of row.decls) { declNames.set(id.text, row); }
}

function rowOfDeclaration(decl) {
  let n = decl;
  while (n && n.parent && !ts.isSourceFile(n.parent)) n = n.parent;
  return n && n.parent && ts.isSourceFile(n.parent) ? statements.find((r) => r.s === n) : undefined;
}

const lineOf = (pos) => sf.getLineAndCharacterOfPosition(pos).line + 1;

// statement text starts at its first JSDoc; plain comments between statements travel with the following statement
function blockRange(row) {
  const prevEnd = row.index === 0 ? 0 : statements[row.index - 1].s.end;
  const start = row.s.getStart(sf, true);
  const gap = text.slice(prevEnd, start);
  const lead = /\S/.test(gap) ? gap.replace(/^\s*\n/, "").replace(/\s+$/, "") : "";
  return { start, end: row.s.end, lead, bare: row.s.getStart(sf) - start };
}

// references -------------------------------------------------------------
function isInsideFunction(n, stop) {
  for (let p = n.parent; p; p = p.parent) {
    if (ts.isFunctionLike(p) || ts.isClassLike(p)) return true;
    if (p === stop) break;
  }
  return false;
}
function collectReferences(row) {
  const refs = [];
  const visit = (n) => {
    if (ts.isIdentifier(n)) {
      const p = n.parent;
      const isPropName = (ts.isPropertyAccessExpression(p) && p.name === n) || (ts.isPropertyAssignment(p) && p.name === n) || (ts.isMethodDeclaration(p) && p.name === n) || (ts.isGetAccessor(p) && p.name === n) || (ts.isSetAccessor(p) && p.name === n) || (ts.isPropertySignature(p) && p.name === n) || (ts.isBindingElement(p) && p.propertyName === n) || (ts.isMetaProperty(p)) || (ts.isLabeledStatement(p));
      if (!isPropName) {
        let symbol;
        let shorthand = false;
        if (ts.isShorthandPropertyAssignment(p) && p.name === n) { symbol = checker.getShorthandAssignmentValueSymbol(p); shorthand = true; }
        else symbol = checker.getSymbolAtLocation(n);
        const decl = symbol && symbol.declarations && symbol.declarations[0];
        if (decl) {
          const target = rowOfDeclaration(decl);
          if (target && declNames.get(n.text) === target && target !== row) refs.push({ node: n, name: n.text, target, shorthand, deferred: isInsideFunction(n, row.s), assignedTo: ts.isBinaryExpression(p) && p.left === n && p.operatorToken.kind >= ts.SyntaxKind.FirstAssignment && p.operatorToken.kind <= ts.SyntaxKind.LastAssignment });
        }
      }
    }
    ts.forEachChild(n, visit);
  };
  visit(row.s);
  return refs;
}

const metaSites = [];
for (const row of statements) {
  row.refs = collectReferences(row);
  const visit = (n) => { if (ts.isMetaProperty(n)) metaSites.push({ row, line: lineOf(n.getStart(sf)), text: n.parent.getText(sf).slice(0, 80) }); ts.forEachChild(n, visit); };
  visit(row.s);
}

// assignment -----------------------------------------------------------
const assigned = new Map();
const componentOf = new Map();
spec.components.forEach((c, ci) => {
  c.index = ci;
  c.dirName = c.emoji + FE0F + c.slug;
  for (const name of c.names) {
    const row = declNames.get(name);
    if (!row) throw new Error(`Unknown name ${name} in ${c.slug}`);
    if (assigned.has(row) && assigned.get(row) !== c) throw new Error(`Statement of ${name} assigned twice: ${assigned.get(row).slug}, ${c.slug}`);
    assigned.set(row, c);
    componentOf.set(name, c);
  }
});
for (const row of statements) {
  if (row.decls.length === 0) continue;
  if (ts.isImportDeclaration(row.s)) continue;
  if (row.s === observedRowStatement()) continue;
  const owners = new Set(row.decls.map((d) => componentOf.get(d.text) && componentOf.get(d.text).slug));
  if (owners.has(undefined) && !(spec.rootOwned || []).includes(row.decls[0].text) && !(spec.dropped || []).includes(row.decls[0].text)) throw new Error(`Unassigned declaration ${row.decls.map((d) => d.text).join(",")} at line ${lineOf(row.s.getStart(sf))}`);
  if (owners.size > 1) throw new Error(`Split statement ${row.decls.map((d) => d.text)}`);
}
function observedRowStatement() { const r = declNames.get("existsSync"); return r && r.s; }
// statements without declarations are an error unless listed
for (const row of statements) {
  if (row.decls.length === 0 && !ts.isImportDeclaration(row.s)) {
    const head = row.s.getText(sf);
    if (!(spec.rootStatements || []).some((prefix) => head.startsWith(prefix))) throw new Error(`Statement without declaration at line ${lineOf(row.s.getStart(sf))}: ${head.slice(0, 80)}`);
  }
}

// imports -----------------------------------------------------------------
const importRows = statements.filter((r) => ts.isImportDeclaration(r.s));
const importBinding = new Map();
for (const row of importRows) {
  const spec_ = row.s.moduleSpecifier.text;
  const c = row.s.importClause;
  if (!c) continue;
  if (c.name) importBinding.set(c.name.text, { row, kind: "default", imported: "default", local: c.name.text });
  if (c.namedBindings) {
    if (ts.isNamespaceImport(c.namedBindings)) importBinding.set(c.namedBindings.name.text, { row, kind: "namespace", local: c.namedBindings.name.text });
    else for (const e of c.namedBindings.elements) importBinding.set(e.name.text, { row, kind: "named", imported: (e.propertyName || e.name).text, local: e.name.text });
  }
}
const observedRow = declNames.get("existsSync");
const observedNames = observedRow.decls.map((d) => d.text);

function rebase(specifier, depth) {
  if (!specifier.startsWith(".")) return specifier;
  return "../".repeat(depth) + specifier;
}

// emission ------------------------------------------------------------------
const rewriteLoaded = new Set(spec.loaded);
const report = { components: {}, meta: metaSites.map((m) => `${m.line}: ${m.text}`) };

function emitComponent(c) {
  const rows = statements.filter((r) => assigned.get(r) === c);
  const bodyEdits = [];
  const foreign = new Map();
  const importsUsed = new Map();
  const observedUsed = new Set();
  const problems = [];
  const rowsText = [];
  const exported = new Set();
  for (const row of rows) {
    const { start, end, lead, bare } = blockRange(row);
    let body = text.slice(start, end);
    const edits = [];
    for (const ref of row.refs) {
      const targetComponent = assigned.get(ref.target);
      if (importBinding.has(ref.name) && ref.target.s.kind === ts.SyntaxKind.ImportDeclaration) {
        const b = importBinding.get(ref.name);
        if (!importsUsed.has(b.row)) importsUsed.set(b.row, new Set());
        importsUsed.get(b.row).add(ref.name);
        continue;
      }
      if (ref.target === observedRow) { observedUsed.add(ref.name); continue; }
      if (rewriteLoaded.has(ref.name)) {
        if (targetComponent === c) { problems.push(`loaded name ${ref.name} declared in ${c.slug}`); continue; }
        edits.push({ at: ref.node.getStart(sf) - start, length: ref.node.getEnd() - ref.node.getStart(sf), to: ref.shorthand ? `${ref.name}: loaded.${ref.name}` : `loaded.${ref.name}` });
        foreign.set("loaded", { component: "bootstrap", name: "loaded" });
        continue;
      }
      if (spec.identityRevision && ref.name === spec.identityRevision) {
        edits.push({ at: ref.node.getStart(sf) - start, length: ref.node.getEnd() - ref.node.getStart(sf), to: ref.shorthand ? `${ref.name}: revisionIdentity.revision` : "revisionIdentity.revision" });
        foreign.set("revisionIdentity", { component: "@registry", name: "revisionIdentity" });
        continue;
      }
      if (!targetComponent) { if ((spec.dropped || []).includes(ref.name)) { problems.push(`dropped name ${ref.name} referenced at ${lineOf(ref.node.getStart(sf))}`); } else problems.push(`unassigned ${ref.name} at ${lineOf(ref.node.getStart(sf))}`); continue; }
      if (targetComponent === c) continue;
      const kind = ref.target.s;
      if (ts.isVariableStatement(kind) && (kind.declarationList.flags & ts.NodeFlags.Let) && !(spec.crossLet || []).includes(ref.name)) problems.push(`cross-component let ${ref.name} (${targetComponent.slug}) at ${lineOf(ref.node.getStart(sf))}`);
      if (!ref.deferred && !(spec.wireInit && spec.wireInit.includes(row.decls[0] && row.decls[0].text))) problems.push(`import-time use of foreign ${ref.name} (${targetComponent.slug}) at ${lineOf(ref.node.getStart(sf))} in ${row.decls.map((d) => d.text).join(",")}`);
      foreign.set(ref.name, { component: targetComponent.slug, name: ref.name });
      targetComponent.exportNames = targetComponent.exportNames || new Set();
      targetComponent.exportNames.add(ref.name);
    }
    edits.sort((a, b) => b.at - a.at);
    for (const e of edits) body = body.slice(0, e.at) + e.to + body.slice(e.at + e.length);
    rowsText.push({ row, lead, body, bare, edits });
  }
  return { rows, rowsText, foreign, importsUsed, observedUsed, problems };
}

const emitted = spec.components.map((c) => ({ c, ...emitComponent(c) }));
// exports requested by the root, invocation or other hand-written code
for (const name of spec.exportsForRoot || []) {
  const c = componentOf.get(name);
  if (!c) throw new Error(`exportsForRoot unknown ${name}`);
  c.exportNames = c.exportNames || new Set();
  c.exportNames.add(name);
}
for (const extra of spec.extraInject || []) {
  const target = spec.components.find((c) => c.slug === extra.from);
  target.exportNames = target.exportNames || new Set();
  for (const n of extra.names) target.exportNames.add(n);
}

fs.rmSync(outDir, { recursive: true, force: true });
fs.mkdirSync(outDir, { recursive: true });

const byName = new Map(spec.components.map((c) => [c.slug, c]));
const lineCounts = {};
for (const e of emitted) {
  const { c, rowsText, foreign, importsUsed, observedUsed, problems } = e;
  if (c.manual) {
    const dir = path.join(outDir, c.dirName);
    fs.mkdirSync(dir, { recursive: true });
    fs.copyFileSync(path.join(__dirname, "manual", `${c.slug}.mjs`), path.join(dir, "🟨️.mjs"));
    lineCounts[c.slug] = fs.readFileSync(path.join(dir, "🟨️.mjs"), "utf8").split("\n").length - 1;
    report.components[c.slug] = { lines: lineCounts[c.slug], needs: ["coordinates"], problems: [], exports: [], foreign: [], rows: [] };
    c.needs = ["coordinates"];
    continue;
  }
  for (const dropped of (spec.dropInject || {})[c.slug] || []) foreign.delete(dropped);
  for (const extra of spec.extraInject || []) if (extra.in === c.slug) for (const n of extra.names) foreign.set(n, { component: extra.from, name: n });
  for (const n of (spec.extraRegistry || {})[c.slug] || []) foreign.set(n, { component: "@registry", name: n });
  const depth = 1;
  const needs = [...new Set([...foreign.values()].filter((f) => f.component !== "@registry").map((f) => f.component))];
  const order = (slug) => byName.get(slug).index;
  for (const need of needs) if (order(need) >= c.index) problems.push(`needs later or same component ${need}`);
  c.needs = needs;
  const out = [];
  out.push(`/** ${c.emoji}${FE0F} ${c.doc} */`);
  const imports = new Map();
  const addImport = (specifier, part) => { if (!imports.has(specifier)) imports.set(specifier, { default: undefined, namespace: undefined, named: [] }); const entry = imports.get(specifier); if (part.default) entry.default = part.default; if (part.namespace) entry.namespace = part.namespace; for (const n of part.named || []) if (!entry.named.includes(n)) entry.named.push(n); };
  for (const [row, usedNames] of importsUsed) {
    for (const dropped of (spec.dropImports || {})[c.slug] || []) usedNames.delete(dropped);
    if (!usedNames.size) continue;
    const clause = row.s.importClause;
    const part = { named: [] };
    if (clause.name && usedNames.has(clause.name.text)) part.default = clause.name.text;
    if (clause.namedBindings) {
      if (ts.isNamespaceImport(clause.namedBindings)) { if (usedNames.has(clause.namedBindings.name.text)) part.namespace = clause.namedBindings.name.text; }
      else part.named = clause.namedBindings.elements.filter((el) => usedNames.has(el.name.text)).map((el) => (el.propertyName ? `${el.propertyName.text} as ${el.name.text}` : el.name.text));
    }
    addImport(rebase(row.s.moduleSpecifier.text, depth), part);
  }
  if (observedUsed.size) {
    const observationRow = [...importBinding.values()].find((b) => b.local === "observedFs").row;
    addImport(rebase(observationRow.s.moduleSpecifier.text, depth), { named: ["observedFs"] });
  }
  const importLines = [...imports].map(([specifier, entry]) => {
    const parts = [];
    if (entry.default) parts.push(entry.default);
    if (entry.namespace) parts.push(`* as ${entry.namespace}`);
    if (entry.named.length) parts.push(`{ ${entry.named.join(", ")} }`);
    return `import ${parts.join(", ")} from ${JSON.stringify(specifier)};`;
  });
  importLines.sort((a, b) => a.localeCompare(b));
  out.push(...importLines);
  if (observedUsed.size) out.push(`const { ${observedNames.filter((n) => observedUsed.has(n)).join(", ")} } = observedFs;`);
  const wireInit = rowsText.filter(({ row }) => (spec.wireInit || []).includes(row.decls[0] && row.decls[0].text));
  const injected = [...foreign.values()].filter((f) => f.component !== "@registry");
  const hasRegistry = [...foreign.values()].some((f) => f.component === "@registry");
  const injectedNames = injected.map((f) => f.name);
  const wireSections = [];
  const grouped = new Map();
  for (const f of injected) { if (!grouped.has(f.component)) grouped.set(f.component, []); if (!grouped.get(f.component).includes(f.name)) grouped.get(f.component).push(f.name); }
  const registryNames = [...foreign.values()].filter((f) => f.component === "@registry").map((f) => f.name);
  out.push("");
  out.push(`/** 🏷️ Names this component as its roster entry does. */`);
  out.push(`export const component = ${JSON.stringify(c.slug)};`);
  const declLets = [...new Set([...injectedNames, ...registryNames])];
  if (declLets.length) out.push(`let ${declLets.join(", ")};`);
  const wireLines = [];
  for (const [comp, names] of grouped) wireLines.push(`  ({ ${names.join(", ")} } = registry.${comp.replace(/-([a-z])/g, (_, ch) => ch.toUpperCase())});`);
  if (registryNames.length) wireLines.push(`  ({ ${registryNames.join(", ")} } = registry);`);
  const wireInitNames = [];
  const bodyOut = [];
  for (const { row, lead, body, bare, edits } of rowsText) {
    const first = row.decls[0] && row.decls[0].text;
    if ((spec.wireInit || []).includes(first)) {
      wireInitNames.push(first);
      const doc = body.slice(0, bare), rest = body.slice(bare);
      const m = /^(?:export\s+)?const\s+([A-Za-z0-9_$]+)\s*=\s*([\s\S]*?);?\s*$/.exec(rest);
      if (!m) throw new Error(`wireInit statement ${first} must be a simple const`);
      wireLines.push((lead ? lead.replace(/^/gm, "  ") + "\n" : "") + (doc.trim() ? doc.trim().replace(/^/gm, "  ") + "\n" : "") + `  ${m[1]} = ${m[2]};`);
      continue;
    }
    bodyOut.push({ row, lead, body, first, bare, edits });
  }
  if (wireInitNames.length) out.push(`export let ${wireInitNames.join(", ")};`);
  if (wireLines.length) {
    out.push("");
    out.push(`/** 🔌️ Binds this component's dependencies from the revision registry once its needs are wired. */`);
    out.push(`export function wire(registry) {`);
    out.push(...wireLines);
    out.push(`}`);
  } else {
    out.push("");
    out.push(`/** 🔌️ Declares that this component needs no sibling. */`);
    out.push(`export function wire() {}`);
  }
  out.push("");
  let previousRow;
  for (const { row, lead, body, bare } of bodyOut) {
    let b = body;
    const exportThis = row.decls.some((d) => c.exportNames && c.exportNames.has(d.text)) || row.decls.some((d) => (spec.alwaysExport || []).includes(d.text));
    if (exportThis && !/^export\s/.test(b.slice(bare))) b = b.slice(0, bare) + "export " + b.slice(bare);
    const adjacent = previousRow && previousRow.index + 1 === row.index && !lead && !/\n[ \t]*\n/.test(text.slice(previousRow.s.end, row.s.getStart(sf, true)));
    out.push(adjacent ? "\u0001" : "");
    out.push((lead ? lead + "\n" : "") + b);
    previousRow = row;
  }
  let content = out.join("\n").replace(/\n\u0001\n/g, "\n").replace(/\n{3,}/g, "\n\n").replace(/\s+$/, "") + "\n";
  for (const append of (spec.appends || []).filter((a) => a.in === c.slug)) content += "\n" + append.text.replace(/\s+$/, "") + "\n";
  for (const patch of (spec.patches || []).filter((p) => p.in === c.slug)) {
    const parts = content.split(patch.find);
    const count = parts.length - 1;
    if (count !== (patch.count ?? 1)) { problems.push(`patch count ${count} != ${patch.count ?? 1}: ${patch.find.slice(0, 60)}`); continue; }
    content = parts.join(patch.replace);
  }
  const dir = path.join(outDir, c.dirName);
  fs.mkdirSync(dir, { recursive: true });
  fs.writeFileSync(path.join(dir, "🟨️.mjs"), content);
  lineCounts[c.slug] = content.split("\n").length - 1;
  report.components[c.slug] = { lines: lineCounts[c.slug], needs, problems, exports: [...(c.exportNames || [])].sort(), foreign: [...foreign.keys()].sort(), wireInit: wireInitNames, rows: rowsText.map(({ row }) => `${lineOf(row.s.getStart(sf))}:${row.decls.map((d) => d.text).join(",") || "-"}`) };
}
{
  const manifestDir = path.join(outDir, "🗂" + FE0F + "components");
  fs.mkdirSync(manifestDir, { recursive: true });
  const rank = new Map(spec.components.map((c, i) => [c.slug, i]));
  const manifest = { version: 1, components: spec.components.map((c) => ({ name: c.slug, path: `./${c.dirName}/🟨️.mjs`, needs: [...c.needs].sort((a, b) => rank.get(a) - rank.get(b)) })) };
  fs.writeFileSync(path.join(manifestDir, "🔣️.json"), JSON.stringify(manifest, null, 2) + "\n");
}
fs.writeFileSync(path.join(outDir, "report.json"), JSON.stringify(report, null, 1));
let bad = 0;
for (const [slug, r] of Object.entries(report.components)) {
  console.log(`${slug}\t${r.lines}\tneeds=[${r.needs.join(",")}]\tproblems=${r.problems.length}`);
  for (const p of r.problems) { console.log(`   PROBLEM ${p}`); bad++; }
}
console.log("meta sites:\n" + report.meta.join("\n"));
console.log(bad ? `PROBLEMS ${bad}` : "OK");
process.exit(bad ? 1 : 0);
