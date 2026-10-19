// usage: node ksplit.cjs <spec.json> [APPLY]
// Test-registration splitter: decomposes one giant `registerTests(vitest, dependencies, source)` function into a kit factory (shared fixtures, harnesses, destructured dependencies),
// plain registration groups (`registerXTests(kit)`) and describe-splits (wrapper + helpers factory + parts), keeping the registration order and every statement verbatim.
// spec: { file, function, kit: { dir, doc, name, emoji, fdoc, typeName }, groups: [ { dir, doc, name, emoji, fdoc, from, to } | { dir, doc, name, emoji, fdoc, describeLine, helpers?: { dir, doc, name, emoji, fdoc, typeName }, parts: [ { dir, doc, name, emoji, fdoc, from, to } ] } ], rootKeep: [line...] }
//   from/to = start lines of the first and last registration statement of a group (depth 1 of the function, or depth 1 of the describe callback for parts).
// Placement of declarations: types and pure functions/classes become kit-module exports (types via `import type`), declarations used by one group move into it, everything else is returned by the kit factory.
const fs = require("fs");
const path = require("path");
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");

const spec = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const APPLY = process.argv[3] === "APPLY";
const file = spec.file;
const text = fs.readFileSync(process.env.ORIG_TEXT || file, "utf8");
const lines = text.split("\n");
const errors = [];
const sfOpts = { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true, noEmit: true, skipLibCheck: true, types: [], noResolve: true, strict: true };
const host = ts.createCompilerHost(sfOpts);
const origGet = host.getSourceFile.bind(host);
host.getSourceFile = (fn, lv, ...r) => (fn === file ? ts.createSourceFile(fn, text, lv, true, ts.ScriptKind.TS) : origGet(fn, lv, ...r));
const program = ts.createProgram([file], sfOpts, host);
const sf = program.getSourceFile(file);
const checker = program.getTypeChecker();
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
const offsetOfLine = (line) => (line <= 1 ? 0 : lines.slice(0, line - 1).join("\n").length + 1);
const regionRe = /^\s*\/\/\s*#(end)?region\b/;

const F = sf.statements.find((s) => ts.isFunctionDeclaration(s) && s.name && s.name.text === spec.function);
if (!F) throw new Error("function not found");
const fnAsync = !!F.modifiers?.some((m) => m.kind === ts.SyntaxKind.AsyncKeyword);

const mkStmts = (nodes, openLine) => nodes.map((n, i, a) => ({ node: n, index: i, a: L(n.getStart(sf)), b: L(n.getEnd()), gapFrom: (i === 0 ? openLine : L(a[i - 1].getEnd())) + 1 }));
const stmts = mkStmts([...F.body.statements], L(F.body.getStart(sf)));
stmts.forEach((s, i) => { if (i > 0 && stmts[i - 1].b === s.a) errors.push(`statements share line ${s.a}`); });

const bindingNames = (n, out = []) => {
  if (ts.isIdentifier(n)) out.push(n);
  else if (ts.isObjectBindingPattern(n) || ts.isArrayBindingPattern(n)) n.elements.forEach((e) => { if (!ts.isOmittedExpression(e)) bindingNames(e.name, out); });
  return out;
};
const paramSyms = new Map();
for (const p of F.parameters) paramSyms.set(checker.getSymbolAtLocation(p.name), p.name.text);

const kindOf = (s) => {
  const n = s.node;
  if (ts.isTypeAliasDeclaration(n) || ts.isInterfaceDeclaration(n)) return "type";
  if (ts.isVariableStatement(n)) {
    const d = n.declarationList.declarations;
    if (d.length === 1 && d[0].initializer && ts.isIdentifier(d[0].initializer) && ts.isObjectBindingPattern(d[0].name) && paramSyms.has(checker.getSymbolAtLocation(d[0].initializer))) return "destruct";
    return "decl";
  }
  if (ts.isFunctionDeclaration(n) || ts.isClassDeclaration(n)) return "decl";
  return "reg";
};
stmts.forEach((s) => (s.kind = kindOf(s)));
for (const line of spec.kitInit || []) { const s = stmts.find((x) => x.a === line); if (s) s.kind = "kitinit"; else errors.push(`kitInit: no statement at ${line}`); }

const valueDecl = new Map();
const typeDecl = new Map();
for (const p of paramSyms.keys()) valueDecl.set(p, { param: true, name: paramSyms.get(p) });
const registerDecls = (s, valueTable, typeTable) => {
  s.decls = [];
  const add = (id, v, t) => { const sym = checker.getSymbolAtLocation(id); if (sym) { s.decls.push({ name: id.text, sym }); if (v) valueTable.set(sym, s); if (t) typeTable.set(sym, s); } };
  if (ts.isVariableStatement(s.node)) for (const d of s.node.declarationList.declarations) bindingNames(d.name).forEach((id) => add(id, true, false));
  else if (ts.isFunctionDeclaration(s.node) && s.node.name) add(s.node.name, true, false);
  else if (ts.isClassDeclaration(s.node) && s.node.name) add(s.node.name, true, true);
  else if ((ts.isTypeAliasDeclaration(s.node) || ts.isInterfaceDeclaration(s.node)) && s.node.name) add(s.node.name, false, true);
};
stmts.forEach((s) => registerDecls(s, valueDecl, typeDecl));

const isTypeMeaning = (n) => {
  for (let p = n.parent; p; p = p.parent) {
    if (ts.isTypeQueryNode(p)) return false;
    if (ts.isExpressionWithTypeArguments(p)) { const h = p.parent; return ts.isHeritageClause(h) && (h.token === ts.SyntaxKind.ImplementsKeyword || ts.isInterfaceDeclaration(h.parent)); }
    if (ts.isTypeNode(p)) return true;
    if (ts.isStatement(p)) return false;
  }
  return false;
};
const refsOf = (node, vtable, ttable, self) => {
  const v = new Map(), t = new Map();
  const visit = (n) => {
    if (ts.isIdentifier(n)) {
      const p = n.parent;
      const declName = (ts.isTypeAliasDeclaration(p) || ts.isInterfaceDeclaration(p) || ts.isClassDeclaration(p) || ts.isFunctionDeclaration(p) || ts.isVariableDeclaration(p) || ts.isParameter(p) || ts.isBindingElement(p)) && p.name === n;
      const skip = declName || (ts.isPropertyAccessExpression(p) && p.name === n) || (ts.isPropertyAssignment(p) && p.name === n) || (ts.isQualifiedName(p) && p.right === n) || ((ts.isMethodDeclaration(p) || ts.isPropertyDeclaration(p) || ts.isPropertySignature(p) || ts.isMethodSignature(p)) && p.name === n) || (ts.isBindingElement(p) && p.propertyName === n);
      if (!skip) {
        let sym = checker.getSymbolAtLocation(n);
        if (ts.isShorthandPropertyAssignment(p) && p.name === n) sym = checker.getShorthandAssignmentValueSymbol(p) || sym;
        if (sym) {
          const typeSide = isTypeMeaning(n);
          const table = typeSide ? ttable : vtable;
          const dst = typeSide ? t : v;
          if (table && table.has(sym) && table.get(sym) !== self) { if (!dst.has(sym)) dst.set(sym, []); dst.get(sym).push(n); }
        }
      }
    }
    ts.forEachChild(n, visit);
  };
  visit(node);
  return { v, t };
};
stmts.forEach((s) => { const r = refsOf(s.node, valueDecl, typeDecl, s); s.refs = r.v; s.trefs = r.t; });
const hasAwaitOutsideFns = (node) => { let r = false; const w = (n) => { if (ts.isAwaitExpression(n) || (ts.isForOfStatement(n) && n.awaitModifier)) r = true; if (!ts.isFunctionLike(n) || n === node) ts.forEachChild(n, w); }; w(node); return r; };

const byLine = new Map(stmts.map((s) => [s.a, s]));
const stmtAt = (line, what, table = byLine) => { const s = table.get(line); if (!s) errors.push(`${what}: no statement starts at line ${line}`); return s; };

const groups = [];
const owner = new Map();
for (const g of spec.groups) {
  const grp = { spec: g, kind: g.describeLine ? "split" : "plain", regs: [] };
  if (grp.kind === "plain") {
    const a = stmtAt(g.from, g.name), b = stmtAt(g.to, g.name);
    if (a && b) for (const s of stmts) if (s.index >= a.index && s.index <= b.index && s.kind === "reg") { grp.regs.push(s); if (owner.has(s)) errors.push(`reg at ${s.a} claimed twice`); owner.set(s, grp); }
  } else {
    const d = stmtAt(g.describeLine, g.name);
    if (d) { grp.regs.push(d); owner.set(d, grp); grp.describe = d; }
  }
  if (!grp.regs.length) errors.push(`group ${g.name} has no registration statement`);
  groups.push(grp);
}
const rootKeep = [];
for (const line of spec.rootKeep || []) { const s = stmtAt(line, "rootKeep"); if (s) { owner.set(s, "root"); rootKeep.push(s); } }
for (const s of stmts) if (s.kind === "reg" && !owner.has(s)) errors.push(`registration statement at ${s.a} is not covered: ${s.node.getText(sf).slice(0, 60)}`);
for (const s of stmts) if (s.kind === "reg" && hasAwaitOutsideFns(s.node)) errors.push(`reg at ${s.a} awaits`);
groups.forEach((g, i) => { if (i > 0 && Math.min(...g.regs.map((s) => s.index)) < Math.max(...groups[i - 1].regs.map((s) => s.index))) errors.push(`group ${g.spec.name} out of statement order`); });

const decls = stmts.filter((s) => s.kind === "decl");
const typeStmts = stmts.filter((s) => s.kind === "type");
const destructs = stmts.filter((s) => s.kind === "destruct");

const users = new Map();
const addUser = (d, g) => { if (!users.has(d)) users.set(d, new Set()); const set = users.get(d); if (set.has(g)) return false; set.add(g); return true; };
for (const s of stmts) if (s.kind === "reg") for (const sym of s.refs.keys()) { const d = valueDecl.get(sym); if (d && !d.param && d.kind === "decl") addUser(d, owner.get(s)); }
for (let changed = true; changed; ) {
  changed = false;
  for (const s of decls) if (users.has(s)) for (const sym of s.refs.keys()) { const d = valueDecl.get(sym); if (d && !d.param && d.kind === "decl") for (const g of users.get(s)) if (addUser(d, g)) changed = true; }
}
const pure = new Set();
{
  const cand = new Set(decls.filter((d) => ts.isFunctionDeclaration(d.node) || ts.isClassDeclaration(d.node)));
  for (let changed = true; changed; ) {
    changed = false;
    for (const d of [...cand]) for (const sym of d.refs.keys()) { const t = valueDecl.get(sym); if (t && (t.param || t.kind === "destruct" || (t.kind === "decl" && !cand.has(t)))) { cand.delete(d); changed = true; break; } }
  }
  cand.forEach((d) => pure.add(d));
}
for (const d of decls) {
  const u = users.get(d) || new Set();
  const only = u.size === 1 ? [...u][0] : null;
  if (only && only !== "root" && only.kind === "plain") { d.place = "group"; d.group = only; }
  else if (only === "root") d.place = "root";
  else if (pure.has(d)) d.place = "module";
  else d.place = "kit";
}
for (const t of typeStmts) t.place = "module";
{
  const typeUsers = new Map();
  const addTU = (t, who) => { if (!typeUsers.has(t)) typeUsers.set(t, new Set()); const set = typeUsers.get(t); if (set.has(who)) return false; set.add(who); return true; };
  for (const s of stmts) {
    if (s.kind === "reg") for (const sym of s.trefs.keys()) { const t = typeDecl.get(sym); if (t && t.kind === "type") addTU(t, owner.get(s)); }
    if (s.kind === "decl") for (const sym of s.trefs.keys()) { const t = typeDecl.get(sym); if (t && t.kind === "type") addTU(t, s.place === "group" ? s.group : s.place === "root" ? "root" : "kit"); }
    if (s.kind === "destruct") for (const sym of s.trefs.keys()) { const t = typeDecl.get(sym); if (t && t.kind === "type") addTU(t, "kit"); }
  }
  for (let ch = true; ch; ) { ch = false; for (const t of typeStmts) if (typeUsers.has(t)) for (const sym of t.trefs.keys()) { const u = typeDecl.get(sym); if (u && u.kind === "type") for (const who of typeUsers.get(t)) if (addTU(u, who)) ch = true; } }
  for (const t of typeStmts) { const u = typeUsers.get(t); if (u && u.size === 1) { const only = [...u][0]; if (only && only !== "kit" && only !== "root" && only.kind === "plain") { t.place = "group"; t.group = only; } else if (only === "kit") { t.place = "kit-local"; } } }
  for (const t of typeStmts) if (t.place === "module") for (const sym of t.trefs.keys()) { const u = typeDecl.get(sym); if (u && u.kind === "type" && u.place === "kit-local") errors.push(`module type ${t.decls.map((x) => x.name)} uses kit-local type ${u.decls.map((x) => x.name)}`); }
  for (const t of typeStmts) if (t.place === "group") for (const sym of t.trefs.keys()) { const u = typeDecl.get(sym); if (u && u.kind === "type" && u.place === "module") errors.push(`group type ${t.decls.map((x) => x.name)} uses module type ${u.decls.map((x) => x.name)}`); }
}
const moduleDecls = decls.filter((d) => d.place === "module");
const kitDecls = decls.filter((d) => d.place === "kit");
for (const d of decls) if (d.place === "root") errors.push(`decl ${d.decls.map((x) => x.name)} is used only by root-kept statements`);
for (const d of decls) if (d.place === "group") for (const sym of d.trefs.keys()) { const t = typeDecl.get(sym); if (t && t.kind === "decl" && t.place === "kit") errors.push(`class ${t.decls.map((x) => x.name)} is a kit-scope value used as a type`); }

const importStmts = sf.statements.filter(ts.isImportDeclaration);
const ext = path.extname(file);
const childRoot = spec.childRoot || path.dirname(file);
const childPath = (dir) => path.join(childRoot, dir, "🟦️" + ext);
const kitPath = childPath(spec.kit.dir);
const rebase = (fromFile) => (specText) => {
  if (!specText.startsWith(".")) return specText;
  let r = path.relative(path.dirname(fromFile), path.resolve(path.dirname(file), specText)).split(path.sep).join("/");
  if (!r.startsWith(".")) r = "./" + r;
  return r;
};
const relTo = (fromFile, toFile) => { let r = path.relative(path.dirname(fromFile), toFile).split(path.sep).join("/"); if (!r.startsWith(".")) r = "./" + r; return r; };

const multiLineLiteralLines = (node) => {
  const set = new Set();
  const w = (n) => { if (ts.isTemplateExpression(n) || ts.isNoSubstitutionTemplateLiteral(n) || ts.isStringLiteral(n)) { const a = L(n.getStart(sf)), b = L(n.getEnd()); for (let l = a + 1; l <= b; l++) set.add(l); } ts.forEachChild(n, w); };
  w(node);
  return set;
};
const importSpecEdits = (node, file2) => {
  const edits = [];
  const fix = rebase(file2);
  const lit = (l) => { if (l && ts.isStringLiteral(l) && l.text.startsWith(".")) { const q = l.getText(sf)[0]; edits.push({ s: l.getStart(sf), e: l.getEnd(), t: q + fix(l.text) + q }); } };
  const w = (n) => {
    if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword) lit(n.arguments[0]);
    if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument)) lit(n.argument.literal);
    ts.forEachChild(n, w);
  };
  w(node);
  return edits;
};
const stmtText = (s, file2, dedent, markers) => {
  const lit = multiLineLiteralLines(s.node);
  const base = offsetOfLine(s.gapFrom);
  let chunk = lines.slice(s.gapFrom - 1, s.b).join("\n");
  for (const e of importSpecEdits(s.node, file2).map((x) => ({ s: x.s - base, e: x.e - base, t: x.t })).sort((x, y) => y.s - x.s)) chunk = chunk.slice(0, e.s) + e.t + chunk.slice(e.e);
  const out = [];
  chunk.split("\n").forEach((l, i) => {
    const lineNo = s.gapFrom + i;
    if (regionRe.test(l) && lineNo < s.a) { if (markers) markers.push({ line: lineNo, text: l.trim() }); return; }
    if (dedent && !lit.has(lineNo) && l.startsWith(" ".repeat(dedent))) l = l.slice(dedent);
    out.push(l);
  });
  while (out.length && !out[0].trim()) out.shift();
  return out.join("\n");
};

const identsOf = (code) => {
  const f = ts.createSourceFile("x.ts", code, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const used = new Set();
  const visit = (n) => {
    if (ts.isImportDeclaration(n)) return;
    if (ts.isIdentifier(n)) {
      const p = n.parent;
      const skip = p && ((ts.isPropertyAccessExpression(p) && p.name === n) || (ts.isPropertyAssignment(p) && p.name === n) || (ts.isQualifiedName(p) && p.right === n) || ((ts.isMethodDeclaration(p) || ts.isPropertyDeclaration(p) || ts.isPropertySignature(p) || ts.isMethodSignature(p)) && p.name === n) || (ts.isBindingElement(p) && p.propertyName === n));
      if (!skip) used.add(n.text);
    }
    ts.forEachChild(n, visit);
  };
  visit(f);
  return used;
};
const prunedImports = (used, fix) => {
  const out = [];
  for (const n of importStmts) {
    const c = n.importClause;
    const q = n.moduleSpecifier.getText(sf)[0];
    const sp = fix(n.moduleSpecifier.text);
    if (!c) continue;
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
    out.push(`import ${c.isTypeOnly ? "type " : ""}${bits.join(", ")} from ${q}${sp}${q};`);
  }
  return out;
};

const topLevelOthers = sf.statements.filter((s) => s !== F && !ts.isImportDeclaration(s));
const topNames = new Map();
for (const s of topLevelOthers) {
  const names = [];
  if (ts.isVariableStatement(s)) for (const d of s.declarationList.declarations) bindingNames(d.name).forEach((i) => names.push(i.text));
  else if ((ts.isTypeAliasDeclaration(s) || ts.isFunctionDeclaration(s) || ts.isClassDeclaration(s) || ts.isInterfaceDeclaration(s)) && s.name) names.push(s.name.text);
  const isType = ts.isTypeAliasDeclaration(s) || ts.isInterfaceDeclaration(s);
  for (const n of names) topNames.set(n, { s, isType });
}

const kitValueSyms = new Map();
for (const p of paramSyms.keys()) kitValueSyms.set(p, paramSyms.get(p));
for (const s of destructs) for (const d of s.decls) kitValueSyms.set(d.sym, d.name);
for (const d of kitDecls) for (const x of d.decls) kitValueSyms.set(x.sym, x.name);
const moduleValueSyms = new Map();
for (const d of moduleDecls) for (const x of d.decls) moduleValueSyms.set(x.sym, x.name);
const moduleTypeSyms = new Map();
for (const s of typeStmts) if (s.place === "module") for (const x of s.decls) moduleTypeSyms.set(x.sym, x.name);
for (const d of moduleDecls) if (ts.isClassDeclaration(d.node)) for (const x of d.decls) moduleTypeSyms.set(x.sym, x.name);

const uses = (refList, trefList) => {
  const kitNames = new Set(), valueImports = new Set(), typeImports = new Set();
  for (const r of refList) for (const sym of r.keys()) { if (kitValueSyms.has(sym)) kitNames.add(kitValueSyms.get(sym)); else if (moduleValueSyms.has(sym)) valueImports.add(moduleValueSyms.get(sym)); }
  for (const r of trefList) for (const sym of r.keys()) { if (moduleTypeSyms.has(sym)) { const n = moduleTypeSyms.get(sym); if ([...moduleDecls].some((d) => d.decls.some((x) => x.name === n))) valueImports.add(n); else typeImports.add(n); } }
  return { kitNames, valueImports, typeImports };
};
const kitImportLines = (outFile, code, u, extra) => {
  const used = identsOf(code);
  const out = [];
  const values = new Set(u.valueImports), types = new Set(u.typeImports);
  for (const [n, info] of topNames) if (used.has(n)) (info.isType ? types : values).add(n);
  if (values.size) out.push(`import { ${[...values].sort().join(", ")} } from "${relTo(outFile, kitPath)}";`);
  if (types.size || extra) out.push(`import type { ${[...types, ...(extra ? [extra] : [])].sort().join(", ")} } from "${relTo(outFile, kitPath)}";`);
  return out;
};

const outputs = [];
const docTop = (doc) => { if (!/^\/\*\*\s+\S/.test(doc)) errors.push(`doc must start with /** : ${doc}`); return doc; };
const fdocOf = (o) => `/** ${o.emoji} ${o.fdoc} */`;
const emojis = new Map();
const claim = (e, who) => { const k = e.replace(/️/g, ""); if (emojis.has(k)) errors.push(`emoji ${e} repeated (${emojis.get(k)}, ${who})`); emojis.set(k, who); };
const claimDir = (dir) => claim(dir.split("️")[0], "folder " + dir);
const markersByStmt = new Map();
stmts.forEach((s) => { const m = []; stmtText(s, file, 0, m); if (m.length) markersByStmt.set(s, m); });
const kitType = spec.kit.typeName;
const emitFile = (o) => outputs.push(o);
const mergeRefs = (list) => { const m = new Map(); for (const r of list) for (const [k, v] of r) { if (!m.has(k)) m.set(k, []); m.get(k).push(...v); } return m; };
const exposeNames = new Set();
const expose = (names) => names.forEach((n) => exposeNames.add(n));

for (const g of groups) {
  const s = g.spec;
  claim(s.emoji, s.name);
  claimDir(s.dir);
  if (g.kind === "plain") {
    const outFile = childPath(s.dir);
    const gDecls = decls.filter((d) => d.place === "group" && d.group === g);
    const gTypes = typeStmts.filter((t) => t.place === "group" && t.group === g);
    const items = [...gTypes, ...gDecls, ...g.regs];
    const u = uses(items.map((x) => x.refs), items.map((x) => x.trefs));
    expose([...u.kitNames]);
    const body = items.map((x) => stmtText(x, outFile, 0));
    const pro = u.kitNames.size ? [`  const { ${[...u.kitNames].sort().join(", ")} } = kit;`] : [];
    g.async = items.some((x) => hasAwaitOutsideFns(x.node));
    if (g.async && !fnAsync) errors.push(`group ${s.name} awaits but the function is not async`);
    const code = [fdocOf(s), `export ${g.async ? "async function" : "function"} ${s.name}(kit: ${kitType}): ${g.async ? "Promise<void>" : "void"} {`, ...pro, ...body, "}"].join("\n");
    const imps = [...prunedImports(identsOf(code), rebase(outFile)), ...kitImportLines(outFile, code, u, kitType)];
    emitFile({ path: outFile, text: `${docTop(s.doc)}\n${imps.join("\n")}\n\n${code}\n`, kind: "group" });
    continue;
  }
  const d = g.describe;
  const call = d.node.expression;
  if (!ts.isCallExpression(call)) { errors.push(`describe at ${d.a} is not a call`); continue; }
  const cb = call.arguments[call.arguments.length - 1];
  if (!(ts.isArrowFunction(cb) || ts.isFunctionExpression(cb)) || cb.parameters.length || cb.modifiers?.length) { errors.push(`describe callback at ${d.a} must be a plain sync function`); continue; }
  const inner = mkStmts([...cb.body.statements], L(cb.body.getStart(sf)));
  const innerByLine = new Map(inner.map((x) => [x.a, x]));
  const iValue = new Map();
  inner.forEach((x) => { x.kind = ts.isFunctionDeclaration(x.node) || ts.isClassDeclaration(x.node) || ts.isVariableStatement(x.node) ? "decl" : "reg"; registerDecls(x, iValue, new Map()); });
  inner.forEach((x) => { const r = refsOf(x.node, iValue, null, x); x.irefs = r.v; const o = refsOf(x.node, valueDecl, typeDecl, null); x.refs = o.v; x.trefs = o.t; });
  const parts = s.parts.map((p) => ({ spec: p, regs: [] }));
  const own = new Map();
  parts.forEach((p) => {
    const a = stmtAt(p.spec.from, p.spec.name, innerByLine), b = stmtAt(p.spec.to, p.spec.name, innerByLine);
    if (a && b) for (const x of inner) if (x.index >= a.index && x.index <= b.index && x.kind === "reg") { p.regs.push(x); own.set(x, p); }
    claim(p.spec.emoji, p.spec.name);
    claimDir(p.spec.dir);
  });
  for (const x of inner) if (x.kind === "reg" && !own.has(x)) errors.push(`describe ${d.a}: registration statement at ${x.a} not covered`);
  const iUsers = new Map();
  const addU = (x, p) => { if (!iUsers.has(x)) iUsers.set(x, new Set()); const set = iUsers.get(x); if (set.has(p)) return false; set.add(p); return true; };
  for (const x of inner) if (x.kind === "reg") for (const sym of x.irefs.keys()) addU(iValue.get(sym), own.get(x));
  for (let ch = true; ch; ) { ch = false; for (const x of inner) if (x.kind === "decl" && iUsers.has(x)) for (const sym of x.irefs.keys()) for (const p of iUsers.get(x)) if (addU(iValue.get(sym), p)) ch = true; }
  const helperDecls = [], partDecls = new Map();
  for (const x of inner) if (x.kind === "decl") {
    const uu = iUsers.get(x) || new Set();
    if (uu.size === 1) { const p = [...uu][0]; if (!partDecls.has(p)) partDecls.set(p, []); partDecls.get(p).push(x); x.place = "part"; } else { helperDecls.push(x); x.place = "helper"; }
  }
  const helperSyms = new Map();
  helperDecls.forEach((x) => x.decls.forEach((dd) => helperSyms.set(dd.sym, dd.name)));
  const hs = s.helpers;
  if (helperDecls.length && !hs) errors.push(`describe ${d.a} has shared helpers (${helperDecls.map((x) => x.decls.map((y) => y.name))}) but no helpers spec`);
  if (hs && !helperDecls.length) errors.push(`describe ${d.a}: helpers spec given but no shared helpers`);
  const hFile = hs && helperDecls.length ? childPath(hs.dir) : null;
  const wFile = childPath(s.dir);
  if (hs) { claim(hs.emoji, hs.name); claimDir(hs.dir); }
  const partFile = (p) => childPath(p.spec.dir);
  for (const p of parts) {
    const outFile = partFile(p);
    const pDecls = partDecls.get(p) || [];
    const items = [...pDecls, ...p.regs];
    const u = uses(items.map((x) => x.refs), items.map((x) => x.trefs));
    expose([...u.kitNames]);
    const hNames = [...new Set(items.flatMap((x) => [...x.irefs.keys()]).filter((sym) => helperSyms.has(sym)).map((sym) => helperSyms.get(sym)))].sort();
    p.usesHelpers = hNames.length > 0;
    const body = items.map((x) => stmtText(x, outFile, 2));
    const pro = [];
    if (u.kitNames.size) pro.push(`  const { ${[...u.kitNames].sort().join(", ")} } = kit;`);
    if (hNames.length) pro.push(`  const { ${hNames.join(", ")} } = helpers;`);
    const code = [fdocOf(p.spec), `export function ${p.spec.name}(kit: ${kitType}${hNames.length ? `, helpers: ${hs.typeName}` : ""}): void {`, ...pro, ...body, "}"].join("\n");
    const imps = [...prunedImports(identsOf(code), rebase(outFile)), ...kitImportLines(outFile, code, u, kitType)];
    if (hNames.length) imps.push(`import type { ${hs.typeName} } from "${relTo(outFile, hFile)}";`);
    emitFile({ path: outFile, text: `${docTop(p.spec.doc)}\n${imps.join("\n")}\n\n${code}\n`, kind: "part" });
  }
  if (hFile) {
    const u = uses(helperDecls.map((x) => x.refs), helperDecls.map((x) => x.trefs));
    expose([...u.kitNames]);
    const body = helperDecls.map((x) => stmtText(x, hFile, 2));
    const names = helperDecls.flatMap((x) => x.decls.map((y) => y.name));
    const pro = u.kitNames.size ? [`  const { ${[...u.kitNames].sort().join(", ")} } = kit;`] : [];
    const code = [fdocOf(hs), `export function ${hs.name}(kit: ${kitType}) {`, ...pro, ...body, `  return { ${names.join(", ")} };`, "}", `export type ${hs.typeName} = ReturnType<typeof ${hs.name}>;`].join("\n");
    const imps = [...prunedImports(identsOf(code), rebase(hFile)), ...kitImportLines(hFile, code, u, kitType)];
    emitFile({ path: hFile, text: `${docTop(hs.doc)}\n${imps.join("\n")}\n\n${code}\n`, kind: "helpers" });
  }
  const calleeRefs = refsOf(call.expression, valueDecl, typeDecl, null);
  const cu = uses([calleeRefs.v], []);
  expose([...cu.kitNames]);
  const calleeText = call.expression.getText(sf);
  const argTexts = call.arguments.slice(0, -1).map((a) => a.getText(sf));
  const callLines = [];
  if (hFile) callLines.push(`    const helpers = ${hs.name}(kit);`);
  for (const p of parts) callLines.push(`    ${p.spec.name}(kit${p.usesHelpers ? ", helpers" : ""});`);
  const code = [fdocOf(s), `export function ${s.name}(kit: ${kitType}): void {`, `  const { ${[...cu.kitNames].sort().join(", ")} } = kit;`, `  ${calleeText}(${argTexts.join(", ")}, () => {`, ...callLines, "  });", "}"].join("\n");
  const imps = [`import type { ${kitType} } from "${relTo(wFile, kitPath)}";`];
  if (hFile) imps.push(`import { ${hs.name} } from "${relTo(wFile, hFile)}";`);
  parts.forEach((p) => imps.push(`import { ${p.spec.name} } from "${relTo(wFile, partFile(p))}";`));
  emitFile({ path: wFile, text: `${docTop(s.doc)}\n${imps.join("\n")}\n\n${code}\n`, kind: "wrapper" });
}
for (const s of rootKeep) expose([...uses([s.refs], []).kitNames]);

{
  claim(spec.kit.emoji, spec.kit.name);
  claimDir(spec.kit.dir);
  const outFile = kitPath;
  const head = topLevelOthers.map((s) => {
    const m = { gapFrom: L(s.getStart(sf, true)), a: L(s.getStart(sf)), b: L(s.getEnd()), node: s };
    let t = stmtText(m, outFile, 0);
    if (!/^(\/\*\*[\s\S]*?\*\/\s*)?export\b/.test(t)) t = t.replace(/(^|\*\/\s*)(const|function|class|type|interface)\b/, (mm, pre, kw) => `${pre}export ${kw}`);
    return t;
  });
  const typeTexts = typeStmts.filter((s) => s.place === "module").map((s) => stmtText(s, outFile, 2).replace(/^(type|interface)\b/, "export $1"));
  const modTexts = moduleDecls.map((d) => stmtText(d, outFile, 2).replace(/^((?:\/\*\*[\s\S]*?\*\/\s*)?)(async function|function|class)\b/, "$1export $2"));
  const destructTexts = destructs.map((s) => stmtText(s, outFile, 0));
  const kitDeclTexts = [...kitDecls, ...stmts.filter((x) => x.kind === "kitinit"), ...typeStmts.filter((t) => t.place === "kit-local")].sort((x, y) => x.index - y.index).map((d) => stmtText(d, outFile, 0));
  const paramText = F.parameters.map((p) => {
    let t = p.getText(sf);
    const base = p.getStart(sf);
    for (const e of importSpecEdits(p, outFile).sort((x, y) => y.s - x.s)) t = t.slice(0, e.s - base) + e.t + t.slice(e.e - base);
    return t;
  }).join(", ");
  const returned = [...exposeNames].filter((n) => ![...moduleValueSyms.values()].includes(n)).sort();
  const kitAsync = [...destructs, ...kitDecls, ...stmts.filter((x) => x.kind === "kitinit")].some((x) => hasAwaitOutsideFns(x.node));
  if (kitAsync && !fnAsync) errors.push("the kit awaits but the function is not async");
  spec.kit.async = kitAsync;
  const factory = [fdocOf(spec.kit), `export ${kitAsync ? "async " : ""}function ${spec.kit.name}(${paramText}) {`, ...destructTexts, ...kitDeclTexts, `  return { ${returned.join(", ")} };`, "}", `export type ${kitType} = ${kitAsync ? "Awaited<" : ""}ReturnType<typeof ${spec.kit.name}>${kitAsync ? ">" : ""};`].join("\n");
  const code = [head.join("\n"), typeTexts.join("\n"), modTexts.join("\n\n"), factory].filter((x) => x).join("\n\n");
  const imps = prunedImports(identsOf(code), rebase(outFile));
  emitFile({ path: outFile, text: `${docTop(spec.kit.doc)}\n${imps.join("\n")}\n\n${code}\n`, kind: "kit" });
  spec.kit.returned = returned;
}

{
  const events = [];
  for (const g of groups) events.push({ index: Math.min(...g.regs.map((r) => r.index)), kind: "group", g, first: g.regs.reduce((m, r) => (r.index < m.index ? r : m)) });
  for (const s of rootKeep) events.push({ index: s.index, kind: "keep", s, first: s });
  events.sort((a, b) => a.index - b.index);
  const firstOf = new Map(events.map((e) => [e.first, e]));
  const eventMarkers = new Map();
  let pend = [];
  for (const s of stmts) {
    pend.push(...(markersByStmt.get(s) || []));
    const ev = firstOf.get(s);
    if (ev) { eventMarkers.set(ev, pend); pend = []; }
    else if (s.kind === "reg" && pend.length) { errors.push(`region marker(s) at ${pend.map((x) => x.line)} sit inside a group before statement ${s.a}`); pend = []; }
  }
  if (pend.length) errors.push(`dangling region markers ${pend.map((x) => x.line)}`);
  const callLines = [];
  for (const ev of events) {
    for (const m of eventMarkers.get(ev) || []) callLines.push(`  ${m.text}`);
    if (ev.kind === "group") callLines.push(`  ${ev.g.async ? "await " : ""}${ev.g.spec.name}(kit);`);
    else callLines.push(stmtText(ev.s, file, 0));
  }
  { const lastB = stmts.length ? stmts[stmts.length - 1].b : 0; const closeL = L(F.body.getEnd()); for (let l = lastB + 1; l < closeL; l++) if (/^\s*\/\/\s*#endregion\b/.test(lines[l - 1])) callLines.push("  " + lines[l - 1].trim()); }
  const ku = uses(rootKeep.map((s) => s.refs), []);
  const sigParams = F.parameters.map((p) => p.getText(sf)).join(", ");
  const kitArgs = F.parameters.map((p) => p.name.getText(sf)).join(", ");
  const bodyLines = [`  const kit = ${spec.kit.async ? "await " : ""}${spec.kit.name}(${kitArgs});`];
  if (ku.kitNames.size) bodyLines.push(`  const { ${[...ku.kitNames].sort().join(", ")} } = kit;`);
  bodyLines.push(...callLines);
  const jsdocs = ts.getJSDocCommentsAndTags(F).filter(ts.isJSDoc);
  const sig = `export ${fnAsync ? "async " : ""}function ${spec.function}(${sigParams})${F.type ? ": " + F.type.getText(sf) : ""} {`;
  const code = `${jsdocs.length ? jsdocs.map((x) => x.getText(sf)).join("\n") + "\n" : ""}${sig}\n${bodyLines.join("\n")}\n}`;
  const used = identsOf(code);
  const imps = prunedImports(used, (x) => x);
  imps.push(`import { ${spec.kit.name} } from "${relTo(file, kitPath)}";`);
  const rootTypes = [...topNames].filter(([n, info]) => used.has(n) && info.isType).map(([n]) => n);
  if (rootTypes.length) imps.push(`import type { ${rootTypes.sort().join(", ")} } from "${relTo(file, kitPath)}";`);
  for (const g of groups) imps.push(`import { ${g.spec.name} } from "${relTo(file, childPath(g.spec.dir))}";`);
  outputs.unshift({ path: file, text: `${imps.join("\n")}\n\n${code}\n`, kind: "root" });
}

for (const d of decls) if (!d.place) errors.push(`decl ${d.decls.map((x) => x.name)} not placed`);
const lc = (t) => t.split("\n").length - 1;
const placement = { module: moduleDecls.map((d) => d.decls.map((x) => x.name)).flat(), kit: kitDecls.map((d) => d.decls.map((x) => x.name)).flat(), group: decls.filter((d) => d.place === "group").map((d) => `${d.decls.map((x) => x.name)}->${d.group.spec.name}`), kitReturned: spec.kit.returned };
console.log(JSON.stringify({ outputs: outputs.map((o) => ({ path: path.relative(childRoot, o.path), kind: o.kind, lines: lc(o.text) })), errors, placement }, null, 1));
if (process.env.DUMP) { fs.mkdirSync(process.env.DUMP, { recursive: true }); outputs.forEach((o, i) => fs.writeFileSync(path.join(process.env.DUMP, `${i}-${path.basename(path.dirname(o.path))}${ext}`), o.text)); }
if (APPLY) {
  if (errors.length) { console.error("REFUSED: errors present"); process.exit(2); }
  for (const o of outputs.slice(1)) if (fs.existsSync(o.path)) { console.error("REFUSED: child exists " + o.path); process.exit(4); }
  if (fs.readFileSync(file, "utf8") !== text) { console.error("REFUSED: root changed during run"); process.exit(5); }
  for (const o of outputs.slice(1)) { fs.mkdirSync(path.dirname(o.path), { recursive: true }); fs.writeFileSync(o.path, o.text); }
  fs.writeFileSync(file, outputs[0].text);
  console.log("APPLIED");
}
