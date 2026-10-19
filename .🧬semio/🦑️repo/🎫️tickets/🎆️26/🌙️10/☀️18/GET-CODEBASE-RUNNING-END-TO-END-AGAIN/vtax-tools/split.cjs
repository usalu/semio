// usage: node split.cjs [APPLY]
// Decomposes validateTaxonomy (snapshot orig/taxonomy-authority.ts) into rule-group validators in sibling taxonomy folders and rewrites the authority module.
// Statements stay verbatim and in order. DRY RUN prints sizes and problems; APPLY writes the folders and the new authority module.
const fs = require("fs");
const path = require("path");
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const spec = require("./spec.cjs");
const APPLY = process.argv[2] === "APPLY";
const ORIG = path.join(__dirname, "orig/taxonomy-authority.ts");
const FAKE = path.join(spec.DISCOVERY, spec.AUTHORITY_DIR, "🟦️.ts");
const text = fs.readFileSync(ORIG, "utf8");
const errors = [];
const opts = { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true, noEmit: true, skipLibCheck: true, types: [], noResolve: true, strict: true };
const host = ts.createCompilerHost(opts);
const orig = host.getSourceFile.bind(host);
host.getSourceFile = (fn, lv, ...r) => (fn === FAKE ? ts.createSourceFile(fn, text, lv, true, ts.ScriptKind.TS) : orig(fn, lv, ...r));
const program = ts.createProgram([FAKE], opts, host);
const sf = program.getSourceFile(FAKE);
const ck = program.getTypeChecker();
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
const V = spec.V;

const fn = sf.statements.find((s) => ts.isFunctionDeclaration(s) && s.name && s.name.text === "validateTaxonomy");
const scopeFn = sf.statements.find((s) => ts.isFunctionDeclaration(s) && s.name && s.name.text === "validateSchemaScopeVocabulary");
const stmts = fn.body.statements.map((node, i) => ({ node, i, start: node.getStart(sf), end: node.getEnd() }));

const importDecls = sf.statements.filter(ts.isImportDeclaration);
const importOfSymbol = new Map();
for (const d of importDecls) {
  const named = d.importClause && d.importClause.namedBindings;
  if (named && ts.isNamedImports(named)) for (const el of named.elements) { const sym = ck.getSymbolAtLocation(el.name); if (sym) importOfSymbol.set(sym, { decl: d, el }); }
}
const topFns = new Map(sf.statements.filter((s) => ts.isFunctionDeclaration(s) && s.name).map((s) => [ck.getSymbolAtLocation(s.name), s.name.text]));
const symOwner = new Map();
for (const s of stmts) {
  const add = (id) => { const sym = ck.getSymbolAtLocation(id); if (sym) symOwner.set(sym, { stmt: s, name: id.text }); };
  const walk = (n) => { if (ts.isIdentifier(n)) add(n); else if (ts.isObjectBindingPattern(n) || ts.isArrayBindingPattern(n)) n.elements.forEach((e) => { if (!ts.isOmittedExpression(e)) walk(e.name); }); };
  if (ts.isVariableStatement(s.node)) for (const d of s.node.declarationList.declarations) walk(d.name);
}
const paramSym = ck.getSymbolAtLocation(fn.parameters[0].name);

function refsOf(nodes) {
  const imports = new Map(), ports = new Set(), members = new Set(), outer = new Set();
  let scopeCall = false;
  const visit = (n) => {
    if (ts.isIdentifier(n)) {
      const p = n.parent;
      const skip = (ts.isPropertyAccessExpression(p) && p.name === n) || (ts.isPropertyAssignment(p) && p.name === n) || (ts.isQualifiedName(p) && p.right === n) || ((ts.isMethodDeclaration(p) || ts.isPropertyDeclaration(p) || ts.isPropertySignature(p)) && p.name === n);
      if (!skip) {
        let sym = ck.getSymbolAtLocation(n);
        if (ts.isShorthandPropertyAssignment(p) && p.name === n) sym = ck.getShorthandAssignmentValueSymbol(p) || sym;
        if (sym) {
          const imp = importOfSymbol.get(sym);
          if (imp) imports.set(imp.el.name.text, imp);
          else if (topFns.has(sym)) { const name = topFns.get(sym); if (spec.PORTS.includes(name)) ports.add(name); else if (name === "validateSchemaScopeVocabulary") scopeCall = true; else errors.push(`unexpected top-level function ${name}`); }
          else if (sym === paramSym) members.add("taxonomy");
          else { const o = symOwner.get(sym); if (o) outer.add(o); }
        }
      }
    }
    ts.forEachChild(n, visit);
  };
  nodes.forEach(visit);
  return { imports, ports, members, outer, scopeCall };
}

const factorySet = new Set(spec.FACTORY_STATEMENTS);
const covered = new Set();
const gapText = (a, b) => text.slice(a.end, b.start);
function bodyText(indices) {
  const kept = indices.map((i) => stmts[i]);
  let out = "";
  kept.forEach((s, k) => {
    if (k > 0) {
      const prev = kept[k - 1];
      let blank;
      if (prev.i + 1 === s.i) { const gap = gapText(prev, s); const residue = gap.split("\n").filter((line) => line.trim() && !/^\s*\/\/#(end)?region /u.test(line)).join(""); if (residue) errors.push(`comment between statements ${prev.i} and ${s.i}`); blank = /\n\s*\n/.test(gap); }
      else blank = false;
      out += blank ? "\n\n" : "\n";
    }
    out += "  " + text.slice(s.start, s.end);
  });
  return out;
}
// original statements are indented by two spaces already; slice starts at the first token, so re-add the indentation above.

function importStatement(els, d) {
  const allTypes = els.every((e) => e.isTypeOnly || d.importClause.isTypeOnly);
  const names = els.map((e) => (allTypes && e.isTypeOnly ? e.name.text + (e.propertyName ? "" : "") : e.getText(sf)));
  return `import ${allTypes ? "type " : ""}{ ${names.join(", ")} } from ${d.moduleSpecifier.getText(sf)};`;
}
function importLines(usedImports) {
  const byDecl = new Map();
  for (const { decl, el } of usedImports.values()) { if (!byDecl.has(decl)) byDecl.set(decl, []); byDecl.get(decl).push(el); }
  const lines = [];
  for (const d of importDecls) {
    if (!byDecl.has(d)) continue;
    const els = byDecl.get(d).sort((a, b) => d.importClause.namedBindings.elements.indexOf(a) - d.importClause.namedBindings.elements.indexOf(b));
    lines.push(importStatement(els, d));
  }
  return lines;
}
const folder = (emoji, slug) => `${emoji}${V}${slug}`;
const doc = (emoji, text) => `/** ${emoji}${V} ${text} */`;

const outputs = new Map();
const MEMBERS_ORDER = spec.AUDIT_MEMBERS;
const groupsInfo = [];

for (const gr of spec.groups) {
  const indices = [];
  for (const [a, b] of gr.ranges) for (let i = a; i <= b; i++) { if (factorySet.has(i)) continue; if (covered.has(i)) errors.push(`statement ${i} claimed twice`); covered.add(i); indices.push(i); }
  const refs = refsOf(indices.map((i) => stmts[i].node));
  const mine = new Set(indices);
  for (const o of refs.outer) { if (mine.has(o.stmt.i)) continue; if (factorySet.has(o.stmt.i)) refs.members.add(o.name); else errors.push(`group ${gr.n} uses ${o.name} declared in statement ${o.stmt.i} outside the group`); }
  for (const m of refs.members) if (!MEMBERS_ORDER.includes(m)) errors.push(`group ${gr.n} unknown member ${m}`);
  const destructured = [...MEMBERS_ORDER.filter((m) => refs.members.has(m)), ...spec.PORTS.filter((p) => refs.ports.has(p))];
  const lines = importLines(refs.imports);
  const importsOut = [...lines, `import type { TaxonomyAudit } from "../${folder(spec.context.emoji, spec.context.slug)}/🟦️.ts";`];
  if (refs.scopeCall) importsOut.push(`import { validateSchemaScopeVocabulary } from "../${folder(spec.scope.emoji, spec.scope.slug)}/🟦️.ts";`);
  const code0 = `${importsOut.join("\n")}\n\n${doc(gr.emoji, gr.doc)}\nexport function ${gr.fn}(audit: TaxonomyAudit): void {\n  const { ${destructured.join(", ")} } = audit;\n${bodyText(indices)}\n}\n`;
  const code = refs.scopeCall ? code0.replace("validateSchemaScopeVocabulary(taxonomy)", "validateSchemaScopeVocabulary(audit)") : code0;
  if (refs.scopeCall && code === code0) errors.push("scope vocabulary call not rewritten");
  outputs.set(path.join(spec.DISCOVERY, folder(gr.emoji, gr.slug), "🟦️.ts"), code);
  groupsInfo.push({ n: gr.n, fn: gr.fn, dir: folder(gr.emoji, gr.slug), lines: code.split("\n").length - 1, stmts: indices.length, destructured, scopeCall: refs.scopeCall });
}
for (const s of stmts) if (!covered.has(s.i) && !factorySet.has(s.i) && s.i !== stmts.length - 1) errors.push(`statement ${s.i} (line ${L(s.start)}) is not covered`);
if (!ts.isReturnStatement(stmts[stmts.length - 1].node)) errors.push("last statement must be return");

// context module
{
  const factoryIdx = spec.FACTORY_STATEMENTS.slice();
  const refs = refsOf(factoryIdx.map((i) => stmts[i].node));
  for (const o of refs.outer) if (!factorySet.has(o.stmt.i)) errors.push(`factory uses ${o.name} outside factory`);
  const importsOut = fs.readFileSync(path.join(__dirname, "context-imports.txt"), "utf8").trim().split("\n");
  const body = bodyText(factoryIdx).replace(/^  const problems: string\[\] = \[\];/, "  const problems: string[] = [];");
  const ctxDir = folder(spec.context.emoji, spec.context.slug);
  const header = fs.readFileSync(path.join(__dirname, "context-header.ts"), "utf8");
  const code = `${importsOut.join("\n")}\n${header}${doc(spec.context.emoji, "Creates the audit context for one taxonomy, binding the shared problem list, the shared check helpers and the injected authority ports.")}\nexport function createTaxonomyAudit(taxonomy: Taxonomy, ports: TaxonomyAuditPorts): TaxonomyAudit {\n${body}\n  return { ${["taxonomy", "problems", "pathMatcher", "canonicalEmoji", "document", "record", "ids", "pattern", "fullPattern", "pathPattern", "workspacePath", "extensionChain", "exactKeys", "kebabId"].join(", ")}, ...ports };\n}\n`;
  outputs.set(path.join(spec.DISCOVERY, ctxDir, "🟦️.ts"), code);
  groupsInfo.push({ n: 0, fn: "createTaxonomyAudit", dir: ctxDir, lines: code.split("\n").length - 1, imports: [...refs.imports.keys()] });
}

// schema scope vocabulary module
{
  const refs = refsOf([scopeFn]);
  refs.imports.delete("Taxonomy");
  const lines = importLines(refs.imports);
  if (refs.ports.size !== 1 || !refs.ports.has("semanticDirectoryKindId")) errors.push(`scope vocabulary ports ${[...refs.ports]}`);
  let fnText = text.slice(scopeFn.getStart(sf), scopeFn.getEnd());
  fnText = fnText.replace("function validateSchemaScopeVocabulary(taxonomy: Taxonomy): string[] {", "export function validateSchemaScopeVocabulary(audit: TaxonomyAudit): string[] {\n  const { taxonomy, semanticDirectoryKindId } = audit;");
  const code = `${[...lines, `import type { TaxonomyAudit } from "../${folder(spec.context.emoji, spec.context.slug)}/🟦️.ts";`].join("\n")}\n\n${doc(spec.scope.emoji, spec.scope.doc)}\n${fnText}\n`;
  outputs.set(path.join(spec.DISCOVERY, folder(spec.scope.emoji, spec.scope.slug), "🟦️.ts"), code);
  groupsInfo.push({ n: 99, fn: "validateSchemaScopeVocabulary", dir: folder(spec.scope.emoji, spec.scope.slug), lines: code.split("\n").length - 1 });
}

// new authority module
{
  const validatorStart = fn.getFullStart();
  const docStart = fn.getStart(sf, true);
  const startIdx = ts.getLeadingCommentRanges(text, fn.getFullStart())?.[0]?.pos ?? fn.getStart(sf);
  const docText = text.slice(startIdx, fn.getStart(sf));
  const shell = `${docText}export function validateTaxonomy(taxonomy: Taxonomy = readTaxonomyUnchecked()): string[] {\n  const audit = createTaxonomyAudit(taxonomy, { ${spec.PORTS.join(", ")} });\n${spec.groups.map((gr) => `  ${gr.fn}(audit);`).join("\n")}\n  return audit.problems;\n}`;
  const scopeStart = (ts.getLeadingCommentRanges(text, scopeFn.getFullStart())?.[0]?.pos) ?? scopeFn.getStart(sf);
  const before = text.slice(0, scopeStart);
  const between = text.slice(scopeFn.getEnd(), startIdx);
  const after = text.slice(fn.getEnd());
  let body = before.replace(/\n+$/, "\n") + "\n" + shell + after;
  const importEnd = importDecls[importDecls.length - 1].getEnd();
  const rest = body.slice(importEnd);
  const addImports = spec.groups.map((gr) => `import { ${gr.fn} } from "../${folder(gr.emoji, gr.slug)}/🟦️.ts";`);
  addImports.push(`import { createTaxonomyAudit } from "../${folder(spec.context.emoji, spec.context.slug)}/🟦️.ts";`);
  const restSf = ts.createSourceFile("rest.ts", rest, ts.ScriptTarget.ESNext, true, ts.ScriptKind.TS);
  const idents = new Set();
  const collect = (n) => { if (ts.isIdentifier(n)) { const p = n.parent; if (!((ts.isPropertyAccessExpression(p) && p.name === n) || (ts.isPropertyAssignment(p) && p.name === n) || (ts.isQualifiedName(p) && p.right === n))) idents.add(n.text); } ts.forEachChild(n, collect); };
  collect(restSf);
  const keptImports = [];
  for (const d of importDecls) {
    const els = d.importClause.namedBindings.elements.filter((el) => idents.has(el.name.text));
    if (els.length) keptImports.push(importStatement(els, d));
  }
  const code = keptImports.join("\n") + "\n" + addImports.join("\n") + rest;
  outputs.set(FAKE + ".new", code);
}

for (const g of groupsInfo) console.log(String(g.n).padStart(2), g.fn.padEnd(52), String(g.lines).padStart(4), g.dir, g.destructured ? "{" + g.destructured.join(",") + "}" : "", g.scopeCall ? "+scope" : "");
const auth = outputs.get(FAKE + ".new");
console.log("authority lines", auth.split("\n").length - 1);
if (errors.length) { console.log("ERRORS:\n" + errors.join("\n")); process.exit(1); }
if (process.env.DUMP) { for (const [file, code] of outputs) { const out = path.join(process.env.DUMP, path.basename(path.dirname(file)) + (file.endsWith(".new") ? ".new.ts" : ".ts")); fs.mkdirSync(process.env.DUMP, { recursive: true }); fs.writeFileSync(out, code); } }
if (!APPLY) { console.log("DRY RUN OK"); process.exit(0); }
const BACKUP = "/Users/ueli/.cache/vtax/backup";
fs.mkdirSync(BACKUP, { recursive: true });
const liveNow = fs.readFileSync(FAKE, "utf8");
if (liveNow !== text) { console.log("authority changed since snapshot; refusing"); process.exit(2); }
fs.copyFileSync(FAKE, path.join(BACKUP, "authority.ts"));
for (const [file, code] of outputs) {
  if (file.endsWith(".new")) continue;
  if (fs.existsSync(file)) { console.log("exists", file); process.exit(3); }
  fs.mkdirSync(path.dirname(file), { recursive: true });
  fs.writeFileSync(file, code);
}
fs.writeFileSync(FAKE, auth);
console.log("APPLIED");
