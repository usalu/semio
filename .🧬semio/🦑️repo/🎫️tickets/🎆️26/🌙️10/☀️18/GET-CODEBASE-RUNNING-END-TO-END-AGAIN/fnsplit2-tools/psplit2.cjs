// usage: node psplit.cjs <spec.json> [APPLY]
// Phase splitter: decomposes one giant function body into phase functions (one file per semantic group), keeping the original depth-1 statements verbatim and in order.
// spec: { file, function, childRoot?, helpers?: { dir, doc, items: [{ line, as }] }, keep: [line...], files: [{ dir, doc, phases: [{ name, emoji, doc, from, to }] }], typeOverrides?: { name: "type text" } }
//   line = first code line of a depth-1 statement; phases cover contiguous statement runs [from..to]; keep = statements that stay in the root; helpers = pure const arrows hoisted to module level (renamed with `as`).
// Cross-phase locals are returned by the producing phase and passed to the consuming phase as one destructured object parameter; the root threads them (`const { a } = producer(); consumer({ a })`).
// DRY RUN prints sizes, flows and problems; APPLY writes the children and rewrites the root.
const fs = require("fs");
const path = require("path");
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");

const spec = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const APPLY = process.argv[3] === "APPLY";
const file = spec.file;
const text = fs.readFileSync(process.env.ORIG_TEXT || file, "utf8");
const lines = text.split("\n");
const errors = [];
const sfOpts = { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true, noEmit: true, skipLibCheck: true, types: [], noResolve: !spec.resolve, strict: true, jsx: ts.JsxEmit.ReactJSX };
const host = ts.createCompilerHost(sfOpts);
const orig = host.getSourceFile.bind(host);
host.getSourceFile = (fn, lv, ...r) => (fn === file ? ts.createSourceFile(fn, text, lv, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS) : orig(fn, lv, ...r));
const program = ts.createProgram([file], sfOpts, host);
const sf = program.getSourceFile(file);
const checker = program.getTypeChecker();
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;

let fn = sf.statements.find((s) => ts.isFunctionDeclaration(s) && s.name && s.name.text === spec.function);
let outer = fn;
if (!fn && spec.arrowVar) {
  outer = sf.statements.find((s) => ts.isVariableStatement(s) && s.declarationList.declarations.some((d) => ts.isIdentifier(d.name) && d.name.text === spec.function));
  const d = outer && outer.declarationList.declarations.find((x) => ts.isIdentifier(x.name) && x.name.text === spec.function);
  fn = d && d.initializer && ts.isArrowFunction(d.initializer) ? d.initializer : null;
}
if (!fn || !outer) throw new Error("function not found");
const isAsync = !!fn.modifiers?.some((m) => m.kind === ts.SyntaxKind.AsyncKeyword);
const stmts = fn.body.statements.map((node, index) => ({ node, index, a: L(node.getStart(sf)), b: L(node.getEnd()) }));
const bodyOpen = L(fn.body.getStart(sf));
stmts.forEach((s, i) => {
  s.gapFrom = (i === 0 ? bodyOpen : stmts[i - 1].b) + 1;
  if (i > 0 && stmts[i - 1].b === s.a) errors.push(`statements share line ${s.a}`);
});
const byLine = new Map(stmts.map((s) => [s.a, s]));
const stmtAt = (line, what) => { const s = byLine.get(line); if (!s) errors.push(`${what}: no statement starts at line ${line}`); return s; };

const isTypePos = (n) => { for (let p = n.parent; p; p = p.parent) if (ts.isTypeNode(p) && !ts.isExpressionWithTypeArguments(p)) return true; return false; };
const declSyms = new Map();
const declOf = (s) => {
  const names = [];
  const add = (id) => { const sym = checker.getSymbolAtLocation(id); if (sym) { names.push({ name: id.text, sym, id }); declSyms.set(sym, s); } };
  const walk = (n) => { if (ts.isIdentifier(n)) add(n); else if (ts.isObjectBindingPattern(n) || ts.isArrayBindingPattern(n)) n.elements.forEach((e) => { if (!ts.isOmittedExpression(e)) walk(e.name); }); };
  if (ts.isVariableStatement(s.node)) for (const d of s.node.declarationList.declarations) walk(d.name);
  else if ((ts.isFunctionDeclaration(s.node) || ts.isClassDeclaration(s.node)) && s.node.name) add(s.node.name);
  s.decls = names;
};
stmts.forEach(declOf);
const PARAM = { param: true, index: -1, a: 0, decls: [] };
for (const p of fn.parameters) { const walkP = (n) => { if (ts.isIdentifier(n)) { const sym = checker.getSymbolAtLocation(n); if (sym) declSyms.set(sym, PARAM); } else if (ts.isObjectBindingPattern(n) || ts.isArrayBindingPattern(n)) n.elements.forEach((e) => { if (!ts.isOmittedExpression(e)) walkP(e.name); }); }; walkP(p.name); }

const refsOf = (s) => {
  const used = new Map();
  const visit = (n) => {
    if (ts.isIdentifier(n)) {
      const p = n.parent;
      const skip = (ts.isPropertyAccessExpression(p) && p.name === n) || (ts.isPropertyAssignment(p) && p.name === n) || (ts.isQualifiedName(p) && p.right === n) || ((ts.isMethodDeclaration(p) || ts.isPropertyDeclaration(p)) && p.name === n);
      if (!skip) {
        let sym = checker.getSymbolAtLocation(n);
        if (ts.isShorthandPropertyAssignment(p) && p.name === n) sym = checker.getShorthandAssignmentValueSymbol(p) || sym;
        const owner = sym && declSyms.get(sym);
        if (owner && owner !== s) { if (!used.has(sym)) used.set(sym, []); used.get(sym).push(n); }
      }
    }
    ts.forEachChild(n, visit);
  };
  visit(s.node);
  return used;
};
stmts.forEach((s) => { s.refs = refsOf(s); s.hasAwait = false; const w = (n) => { if (ts.isAwaitExpression(n) || (ts.isForOfStatement(n) && n.awaitModifier)) s.hasAwait = true; if (!ts.isFunctionLike(n) || n === s.node) ts.forEachChild(n, w); }; w(s.node); });

const returnStmt = stmts.find((s) => ts.isReturnStatement(s.node) && s.index === stmts.length - 1);
if (!returnStmt) errors.push("function must end with a return statement");

const owner = new Map();
const units = [];
const helperItems = [];
const renames = new Map();
if (spec.helpers) {
  for (const it of spec.helpers.items) {
    const s = stmtAt(it.line, "helper");
    if (!s) continue;
    if (s.refs.size) errors.push(`helper ${s.decls.map((d) => d.name)} uses body locals ${[...s.refs.keys()].map((k) => k.name)}`);
    owner.set(s, "helper");
    for (const d of s.decls) renames.set(d.sym, it.as);
    helperItems.push({ s, as: it.as });
  }
}
for (const line of spec.keep) { const s = stmtAt(line, "keep"); if (s) { owner.set(s, "keep"); } }
if (returnStmt) owner.set(returnStmt, "return");
const fileSpecs = spec.files;
const phaseUnits = [];
for (const [fi, f] of fileSpecs.entries()) {
  for (const ph of f.phases) {
    const from = stmtAt(ph.from, `phase ${ph.name}`), to = stmtAt(ph.to, `phase ${ph.name}`);
    if (!from || !to) continue;
    const members = stmts.filter((s) => s.index >= from.index && s.index <= to.index);
    const u = { kind: "phase", spec: ph, fileIndex: fi, members, from, to };
    for (const m of members) {
      if (owner.has(m) && owner.get(m) !== "helper") errors.push(`statement at ${m.a} claimed twice (${owner.get(m)} and ${ph.name})`);
      if (owner.get(m) === "helper") { u.holes = u.holes || []; u.holes.push(m); } else owner.set(m, u);
    }
    phaseUnits.push(u);
  }
}
for (const s of stmts) if (!owner.has(s)) errors.push(`statement at line ${s.a} (${s.node.getText(sf).slice(0, 50)}) is not covered`);
const unitOf = (s) => { const o = owner.get(s); return o && typeof o === "object" ? o : o === "keep" ? { kind: "keep", s } : null; };
const order = [];
for (const s of stmts) {
  const o = owner.get(s);
  if (o === "helper" || o === "return") continue;
  if (o === "keep") order.push({ kind: "keep", members: [s], s });
  else if (!order.includes(o)) order.push(o);
}
let prev = -1;
for (const u of order) { const first = u.members[0].index; if (first < prev) errors.push(`unit order broken at ${u.kind === "phase" ? u.spec.name : u.s.a}`); prev = u.members[u.members.length - 1].index; }
const unitIdx = new Map(order.map((u, i) => [u, i]));
const memberUnit = new Map();
for (const u of order) for (const m of u.members) memberUnit.set(m, u);

for (const u of order) {
  u.needs = new Map();
  for (const m of u.members) for (const [sym, ids] of m.refs) {
    const o = declSyms.get(sym);
    if (owner.get(o) === "helper") continue;
    if (o === PARAM) { u.needs.set(sym, "param"); continue; }
    const pu = memberUnit.get(o);
    if (pu === u) continue;
    if (!pu) { errors.push(`unit ${u.spec?.name ?? u.s.a} references ${sym.name} declared in a non-unit statement`); continue; }
    if (unitIdx.get(pu) > unitIdx.get(u)) errors.push(`unit ${u.spec?.name ?? u.s.a} uses ${sym.name} declared later`);
    { const vd = sym.valueDeclaration; if (vd && ts.isVariableDeclaration(vd) && !(ts.getCombinedNodeFlags(vd) & ts.NodeFlags.Const) && !(spec.allowLets || []).includes(sym.name)) errors.push(`mutable ${sym.name} (declared with let/var) crosses from ${pu.spec?.name ?? pu.s?.a} to ${u.spec?.name ?? u.s.a}`); }
    u.needs.set(sym, pu);
  }
}
const finalNeeds = new Map();
for (const [sym] of returnStmt.refs) { const o = declSyms.get(sym); if (o === PARAM || owner.get(o) === "helper") continue; const pu = memberUnit.get(o); if (pu) finalNeeds.set(sym, pu); }
for (const u of order) u.produces = new Set();
for (const u of order) for (const [sym, pu] of u.needs) if (pu !== "param") pu.produces.add(sym);
for (const [sym, pu] of finalNeeds) pu.produces.add(sym);
for (const u of order) if (u.members.some((m) => m.hasAwait) && !isAsync) errors.push(`await inside ${u.spec?.name ?? u.s.a} of a non-async function`);
for (const u of order) if (u.kind === "phase" && u.members.some((m) => m.hasAwait)) u.async = true;

const typeText = (sym) => {
  if (spec.typeOverrides?.[sym.name]) return spec.typeOverrides[sym.name];
  const decl = sym.valueDeclaration || sym.declarations[0];
  const t = checker.getBaseTypeOfLiteralType(checker.getTypeOfSymbolAtLocation(sym, decl));
  const s = checker.typeToString(t, fn, ts.TypeFormatFlags.NoTruncation | ts.TypeFormatFlags.UseAliasDefinedOutsideCurrentScope);
  if (/\bany\b|import\(|unknown/.test(s)) errors.push(`type of ${sym.name} not nameable (${s.slice(0, 80)}); add a typeOverrides entry`);
  return s;
};

const sliceLines = (a, b) => lines.slice(a - 1, b);
const regionRe = /^\s*\/\/\s*#(end)?region\b/;
const segmentLines = (members) => {
  const first = members[0], last = members[members.length - 1];
  let ls = sliceLines(first.gapFrom, last.b);
  const out = [];
  let skipping = false;
  for (const m of members) {
    if (owner.get(m) === "helper") continue;
  }
  const hole = new Set((members.filter((m) => owner.get(m) === "helper")).flatMap((m) => { const r = []; for (let l = m.gapFrom; l <= m.b; l++) r.push(l); return r; }));
  for (let l = first.gapFrom; l <= last.b; l++) if (!hole.has(l)) out.push(lines[l - 1]);
  while (out.length && !out[0].trim()) out.shift();
  return out;
};
const phaseDecls = (u) => u.members.filter((m) => owner.get(m) === u).flatMap((m) => m.decls);

const identsOf = (code) => {
  const f = ts.createSourceFile("x" + path.extname(file), code, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
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
const importStmts = sf.statements.filter(ts.isImportDeclaration);
const rebase = (fromFile) => (specText) => {
  if (!specText.startsWith(".")) return specText;
  let r = path.relative(path.dirname(fromFile), path.resolve(path.dirname(file), specText)).split(path.sep).join("/");
  if (!r.startsWith(".")) r = "./" + r;
  return r;
};
const relTo = (fromFile, toFile) => { let r = path.relative(path.dirname(fromFile), toFile).split(path.sep).join("/"); if (!r.startsWith(".")) r = "./" + r; return r; };
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

const childRoot = spec.childRoot || path.dirname(file);
const ext = path.extname(file);
const childPath = (dir) => path.join(childRoot, dir, "🟦️" + ext);
const helperFile = spec.helpers ? childPath(spec.helpers.dir) : null;

const applyRenames = (member, code) => {
  const edits = [];
  for (const [sym, ids] of member.refs) if (renames.has(sym)) for (const id of ids) {
    const p = id.parent;
    if (ts.isShorthandPropertyAssignment(p) && p.name === id) { errors.push(`shorthand use of renamed helper at line ${L(id.getStart(sf))}`); continue; }
    edits.push({ s: id.getStart(sf), e: id.getEnd(), t: renames.get(sym) });
  }
  return edits;
};
const renamedText = (members, textLines, startLine) => {
  const base = lines.slice(0, startLine - 1).join("\n").length + (startLine > 1 ? 1 : 0);
  const edits = members.flatMap((m) => applyRenames(m)).sort((x, y) => y.s - x.s);
  let out = textLines.join("\n");
  const offs = base;
  for (const e of edits) out = out.slice(0, e.s - offs) + e.t + out.slice(e.e - offs);
  return out.split("\n");
};
const phaseBody = (u) => {
  const first = u.members[0], last = u.members[u.members.length - 1];
  let ls = [];
  const hole = new Set((u.holes || []).flatMap((m) => { const r = []; for (let l = m.gapFrom; l <= m.b; l++) r.push(l); return r; }));
  if (hole.size) errors.push(`phase ${u.spec.name} spans a hoisted helper`);
  ls = sliceLines(first.gapFrom, last.b);
  ls = renamedText(u.members, ls, first.gapFrom);
  while (ls.length && !ls[0].trim()) ls.shift();
  return ls;
};

const outputs = [];
const flowDecl = (set) => [...set].map((sym) => ({ name: sym.name, type: typeText(sym) }));
const paramText = (needs) => {
  if (!needs.size) return "";
  const names = [...needs.keys()];
  if (spec.typesFromProducers) {
    const byProducer = new Map();
    for (const [sym, pu] of needs) { const key = pu === "param" ? "param" : pu; if (!byProducer.has(key)) byProducer.set(key, []); byProducer.get(key).push(sym); }
    const parts = [];
    for (const [pu, syms] of byProducer) {
      if (pu === "param") parts.push(`{ ${syms.map((s) => `readonly ${s.name}: ${typeText(s)}`).join("; ")} }`);
      else parts.push(`Pick<${pu.async ? "Awaited<ReturnType<typeof " + pu.spec.name + ">>" : "ReturnType<typeof " + pu.spec.name + ">"}, ${syms.map((s) => `"${s.name}"`).join(" | ")}>`);
    }
    return `{ ${names.map((s) => s.name).join(", ")} }: ${parts.join(" & ")}`;
  }
  const inner = names.map((s) => `  readonly ${s.name}: ${typeText(s)};`).join("\n");
  return `{ ${names.map((s) => s.name).join(", ")} }: {\n${inner}\n}`;
};

const rootCalls = [];
for (const u of order) {
  if (u.kind === "keep") { rootCalls.push({ lines: renamedText(u.members, sliceLines(u.members[0].gapFrom, u.members[0].b), u.members[0].gapFrom).filter((l, i, a) => !(i === 0 && !l.trim())) }); continue; }
  const args = u.needs.size ? `{ ${[...u.needs.keys()].map((s) => s.name).join(", ")} }` : "";
  const prod = [...u.produces].map((s) => s.name);
  const call = `${isAsync && u.async ? "await " : ""}${u.spec.name}(${args})`;
  rootCalls.push({ unit: u, lines: [prod.length ? `  const { ${prod.join(", ")} } = ${call};` : `  ${call};`] });
}
for (const [fi, f] of fileSpecs.entries()) {
  const out = childPath(f.dir);
  const funcs = [];
  for (const ph of f.phases) {
    const u = phaseUnits.find((x) => x.spec === ph);
    if (!u) continue;
    const body = phaseBody(u);
    const prod = [...u.produces].map((s) => s.name);
    const ret = prod.length ? [`  return { ${prod.join(", ")} };`] : [];
    const params = paramText(u.needs);
    funcs.push([`/** ${ph.emoji} ${ph.doc} */`, `export ${u.async ? "async " : ""}function ${ph.name}(${params}) {`, ...body, ...ret, "}"].join("\n"));
    for (const [sym] of u.needs) if (!sym) continue;
  }
  const bodyText = funcs.join("\n\n");
  const used = identsOf(bodyText);
  const imps = prunedImports(used, rebase(out));
  if (spec.typesFromProducers) {
    const own = new Set(f.phases.map((p) => p.name));
    const foreign = new Map();
    for (const [fj, g] of fileSpecs.entries()) if (fj !== fi) for (const p of g.phases) if (new RegExp(`typeof ${p.name}>`).test(bodyText)) { if (!foreign.has(g)) foreign.set(g, []); foreign.get(g).push(p.name); }
    for (const [g, names] of foreign) imps.push(`import type { ${names.sort().join(", ")} } from "${relTo(out, childPath(g.dir))}";`);
  }
  if (helperFile && spec.helpers.items.some((h) => used.has(h.as))) imps.push(`import { ${spec.helpers.items.map((h) => h.as).filter((n) => used.has(n)).sort().join(", ")} } from "${relTo(out, helperFile)}";`);
  if (!/^\/\*\*\s+\S/.test(f.doc)) errors.push(`file ${f.dir}: docstring must start with /** `);
  outputs.push({ path: out, text: `${f.doc}\n${imps.join("\n")}${imps.length ? "\n\n" : ""}${bodyText}\n`, kind: "child" });
}
if (helperFile) {
  const items = helperItems.map(({ s, as }) => {
    let ls = sliceLines(s.a, s.b);
    const multiLineTemplate = (() => { let bad = false; const w = (n) => { if ((ts.isTemplateExpression(n) || ts.isNoSubstitutionTemplateLiteral(n)) && L(n.getStart(sf)) !== L(n.getEnd())) bad = true; ts.forEachChild(n, w); }; w(s.node); return bad; })();
    if (multiLineTemplate) errors.push(`helper ${as} contains a multi-line template`);
    ls = ls.map((l) => (l.startsWith("  ") ? l.slice(2) : l));
    const edits = [];
    for (const d of s.decls) { const id = d.id; edits.push({ line: L(id.getStart(sf)), col: id.getStart(sf) - (lines.slice(0, L(id.getStart(sf)) - 1).join("\n").length + (L(id.getStart(sf)) > 1 ? 1 : 0)), name: d.name }); }
    ls[0] = ls[0].replace(new RegExp(`^(const|function)\\s+${s.decls[0].name}\\b`), (m, k) => `export ${k} ${as}`);
    const hs = spec.helpers.items.find((h) => h.as === as);
    if (hs.doc) ls.unshift(`/** ${hs.emoji} ${hs.doc} */`);
    return { ls, s };
  });
  const bodyText = items.map((i) => i.ls.join("\n")).join("\n\n");
  const used = identsOf(bodyText);
  const imps = prunedImports(used, rebase(helperFile));
  outputs.push({ path: helperFile, text: `${spec.helpers.doc}\n${imps.join("\n")}${imps.length ? "\n\n" : ""}${bodyText}\n`, kind: "helper" });
}

// 🔗 module-level declarations of the root file that moved code uses: moved into the shared file
const topDecls = new Map();
for (const st of sf.statements) {
  if (st === fn || st === outer || ts.isImportDeclaration(st)) continue;
  const names = [];
  const bn = (n) => { if (ts.isIdentifier(n)) names.push(n.text); else if (ts.isObjectBindingPattern(n) || ts.isArrayBindingPattern(n)) n.elements.forEach((e) => !ts.isOmittedExpression(e) && bn(e.name)); };
  if (ts.isVariableStatement(st)) st.declarationList.declarations.forEach((d) => bn(d.name));
  else if ((ts.isFunctionDeclaration(st) || ts.isClassDeclaration(st) || ts.isTypeAliasDeclaration(st) || ts.isInterfaceDeclaration(st) || ts.isEnumDeclaration(st)) && st.name) names.push(st.name.text);
  for (const n of names) topDecls.set(n, st);
}
const movedStmts = new Set();
{
  const childOutputs = outputs.filter((o) => o.kind === "child" || o.kind === "helper");
  let frontier = childOutputs.map((o) => o.text);
  const seen = new Set();
  while (frontier.length) {
    const next = [];
    for (const t of frontier) for (const n of identsOf(t)) {
      const st = topDecls.get(n);
      if (!st || seen.has(st)) continue;
      seen.add(st);
      movedStmts.add(st);
      next.push(st.getFullText(sf));
    }
    frontier = next;
  }
}
const movedNames = [];
let sharedOut = null;
if (movedStmts.size) {
  if (!spec.shared) errors.push(`phases use module declarations of the root file (${[...topDecls].filter(([, st]) => movedStmts.has(st)).map(([n]) => n).join(", ")}) but the spec has no shared file`);
  else {
    const sharedFile = path.join(childRoot, spec.shared.dir, "🟦️" + ext);
    const ordered = [...movedStmts].sort((a, b) => a.getStart(sf) - b.getStart(sf));
    const texts = ordered.map((st) => {
      if (!spec.allowExportedMoves && st.modifiers?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword)) errors.push(`moved declaration at line ${L(st.getStart(sf))} is exported from the root`);
      let t = st.getFullText(sf).trim();
      for (const e of []) t = t;
      for (const [from, to] of spec.sharedRewrite || []) t = t.split(from).join(to);
      t = t.split("\n").filter((l) => !/^\s*\/\/\s*#(end)?region\b/.test(l)).join("\n").trim();
      t = t.replace(/^(\s*)(const|let|var|function|async function|class|type|interface|enum)\b/m, (mm, ws, kw) => `${ws}export ${kw}`);
      return t;
    });
    for (const st of ordered) {
      if (ts.isVariableStatement(st)) st.declarationList.declarations.forEach((d) => { const w = (n) => { if (ts.isIdentifier(n)) movedNames.push({ name: n.text, type: false }); else n.elements && n.elements.forEach((e) => !ts.isOmittedExpression(e) && w(e.name)); }; w(d.name); });
      else if (st.name) movedNames.push({ name: st.name.text, type: ts.isTypeAliasDeclaration(st) || ts.isInterfaceDeclaration(st) });
    }
    const code = texts.join("\n\n");
    const imps = prunedImports(identsOf(code), rebase(sharedFile));
    sharedOut = { path: sharedFile, text: `${spec.shared.doc}\n${imps.join("\n")}${imps.length ? "\n\n" : ""}${code}\n`, kind: "helper" };
    for (const o of outputs) {
      if (o.kind !== "child" && o.kind !== "helper") continue;
      const used = identsOf(o.text);
      const need = movedNames.filter((m) => used.has(m.name));
      if (!need.length) continue;
      const vals = need.filter((m) => !m.type).map((m) => m.name), tys = need.filter((m) => m.type).map((m) => m.name);
      const add = [vals.length ? `import { ${vals.sort().join(", ")} } from "${relTo(o.path, sharedFile)}";` : "", tys.length ? `import type { ${tys.sort().join(", ")} } from "${relTo(o.path, sharedFile)}";` : ""].filter(Boolean).join("\n");
      const ls = o.text.split("\n");
      let last = 0;
      ls.forEach((l, i) => { if (/^import\s/.test(l)) last = i + 1; });
      ls.splice(last || 1, 0, add);
      o.text = ls.join("\n");
    }
  }
}
{
  const docLines = [];
  const fnDocStart = outer.getFullStart();
  const jsdocs = ts.getJSDocCommentsAndTags(outer).filter(ts.isJSDoc);
  const rootRet = renamedText([returnStmt], sliceLines(returnStmt.a, returnStmt.b), returnStmt.a);
  const exported = outer.modifiers?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword);
  const arrowVar = !!spec.arrowVar && outer !== fn;
  const closeTxt = arrowVar ? "}" + text.slice(fn.body.getEnd(), outer.getEnd()) : "}";
  const sig = arrowVar ? text.slice(outer.getStart(sf), fn.body.getStart(sf) + 1) : `${exported ? "export " : ""}${isAsync ? "async " : ""}function ${spec.function}(${fn.parameters.map((p) => p.getText(sf)).join(", ")})${fn.type ? ": " + fn.type.getText(sf) : ""} {`;
  const bodyParts = [];
  for (const c of rootCalls) bodyParts.push(c.lines.join("\n"));
  const bodyText = [...bodyParts, rootRet.join("\n")].join("\n");
  const phaseNames = [];
  const imps = [];
  const movedSet = movedStmts;
  const headerEnd0 = importStmts.length ? importStmts[importStmts.length - 1].getEnd() : 0;
  const fnStart0 = jsdocs.length ? jsdocs[0].getStart(sf) : outer.getStart(sf);
  const keepText = (from, to) => sf.statements.filter((st) => st.getFullStart() >= from && st.getEnd() <= to && !movedSet.has(st) && !ts.isImportDeclaration(st)).map((st) => st.getFullText(sf).trim()).join("\n\n");
  const betweenKept = keepText(headerEnd0, fnStart0);
  const afterKept = keepText(outer.getEnd(), text.length);
  const usedRoot = identsOf(bodyText + "\n" + sig + "\n" + betweenKept + "\n" + afterKept);
  imps.push(...prunedImports(usedRoot, (s) => s));
  for (const [fi, f] of fileSpecs.entries()) {
    const names = f.phases.map((p) => p.name).filter((n) => usedRoot.has(n)).sort();
    if (names.length) imps.push(`import { ${names.join(", ")} } from "${relTo(file, childPath(f.dir))}";`);
  }
  const headerEnd = importStmts.length ? importStmts[importStmts.length - 1].getEnd() : 0;
  const fnStart = jsdocs.length ? jsdocs[0].getStart(sf) : outer.getStart(sf);
  const between = movedStmts.size ? betweenKept : text.slice(headerEnd, fnStart).trim();
  const afterFn = movedStmts.size ? afterKept : text.slice(outer.getEnd()).trim();
  let rootText = `${imps.join("\n")}\n\n${between ? between + "\n" : ""}${jsdocs.length ? jsdocs.map((d) => d.getText(sf)).join("\n") + "\n" : ""}${sig}\n${bodyText}\n${closeTxt}\n${afterFn ? "\n" + afterFn + "\n" : ""}`.replace(/\n{3,}/g, "\n\n");
  if (sharedOut) {
    const need = movedNames.filter((m) => usedRoot.has(m.name));
    if (need.length) {
      const vals = need.filter((m) => !m.type).map((m) => m.name), tys = need.filter((m) => m.type).map((m) => m.name);
      const add = [vals.length ? `import { ${vals.sort().join(", ")} } from "${relTo(file, sharedOut.path)}";` : "", tys.length ? `import type { ${tys.sort().join(", ")} } from "${relTo(file, sharedOut.path)}";` : ""].filter(Boolean).join("\n");
      rootText = rootText.replace(/^((?:import [^\n]*\n)+)/, (m) => m + add + "\n");
    }
    outputs.push(sharedOut);
  }
  outputs.unshift({ path: file, text: rootText, kind: "root" });
}

const allEmojis = new Map();
const stripV = (e) => e.replace(/\uFE0F/g, "");
for (const f of fileSpecs) allEmojis.set(stripV(f.dir.split("\uFE0F")[0]), "folder " + f.dir);
if (spec.helpers) { allEmojis.set(stripV(spec.helpers.dir.split("\uFE0F")[0]), "folder " + spec.helpers.dir); for (const h of spec.helpers.items) if (h.emoji) { if (allEmojis.has(stripV(h.emoji))) errors.push(`helper emoji ${h.emoji} repeated`); allEmojis.set(stripV(h.emoji), h.as); } }
for (const f of fileSpecs) for (const p of f.phases) { p.emoji = p.emoji; if (allEmojis.has(stripV(p.emoji))) { errors.push(`phase emoji ${p.emoji} repeated (${allEmojis.get(stripV(p.emoji))}, ${p.name})`); continue; } allEmojis.set(stripV(p.emoji), p.name); }
for (const f of fileSpecs) for (const p of []) { if (allEmojis.has(p.emoji)) errors.push(`phase emoji ${p.emoji} repeated (${allEmojis.get(p.emoji)}, ${p.name})`); allEmojis.set(p.emoji, p.name); }
const lc = (t) => t.split("\n").length - 1;
const report = outputs.map((o) => ({ path: path.relative(childRoot, o.path), lines: lc(o.text) }));
console.log(JSON.stringify({ outputs: report, errors }, null, 1));
const flows = order.filter((u) => u.kind === "phase" && (u.needs.size || u.produces.size)).map((u) => `${u.spec.name}: needs [${[...u.needs.keys()].map((s) => s.name)}] produces [${[...u.produces].map((s) => s.name)}]`);
console.log(flows.join("\n"));
if (process.env.DUMP) { fs.mkdirSync(process.env.DUMP, { recursive: true }); outputs.forEach((o, i) => fs.writeFileSync(path.join(process.env.DUMP, `${i}-${path.basename(path.dirname(o.path))}${ext}`), o.text)); }
if (APPLY) {
  if (errors.length) { console.error("REFUSED: errors present"); process.exit(2); }
  for (const o of outputs.slice(1)) if (fs.existsSync(o.path)) { console.error("REFUSED: child exists " + o.path); process.exit(4); }
  if (fs.readFileSync(file, "utf8") !== text) { console.error("REFUSED: root changed during run"); process.exit(5); }
  for (const o of outputs.slice(1)) { fs.mkdirSync(path.dirname(o.path), { recursive: true }); fs.writeFileSync(o.path, o.text); }
  fs.writeFileSync(file, outputs[0].text);
  console.log("APPLIED");
}
