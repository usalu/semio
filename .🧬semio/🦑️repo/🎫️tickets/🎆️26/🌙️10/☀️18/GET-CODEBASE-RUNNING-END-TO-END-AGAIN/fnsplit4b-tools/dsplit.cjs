// usage: node dsplit.cjs <spec.json> [APPLY]
// Module-level describe splitter: decomposes the callback of ONE module-level `describe(...)` statement into part functions (one folder per part), keeping every registration statement verbatim and in order.
// spec: { file, describeLine, shared?: { dir, doc }, parts: [ { dir, doc, name, emoji, fdoc, from, to } ] }
//   from/to = start lines (without comments) of the first and last statement of a part at depth 1 of the describe callback; parts cover the callback body contiguously.
//   Declarations of the callback used by exactly one part move into it; module-level non-import declarations that parts use move into the shared file (exported); mutable module-level bindings and shared callback declarations are refused.
const fs = require("fs");
const path = require("path");
const ts = require(process.env.TYPESCRIPT_PATH || "/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");

const spec = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const APPLY = process.argv[3] === "APPLY";
const file = spec.file;
const text = fs.readFileSync(file, "utf8");
const lines = text.split("\n");
const errors = [];
const sfOpts = { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true, noEmit: true, skipLibCheck: true, types: [], noResolve: true, strict: true, jsx: ts.JsxEmit.ReactJSX };
const host = ts.createCompilerHost(sfOpts);
const origGet = host.getSourceFile.bind(host);
host.getSourceFile = (fn, lv, ...r) => (fn === file ? ts.createSourceFile(fn, text, lv, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS) : origGet(fn, lv, ...r));
const program = ts.createProgram([file], sfOpts, host);
const sf = program.getSourceFile(file);
const checker = program.getTypeChecker();
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
const ext = path.extname(file);
const V = "️";
const stripV = (e) => e.replace(/️/g, "");

const dstmt = sf.statements.find((s) => L(s.getStart(sf)) === spec.describeLine);
if (!dstmt || !ts.isExpressionStatement(dstmt) || !ts.isCallExpression(dstmt.expression)) throw new Error("describe statement not found at " + spec.describeLine);
const call = dstmt.expression;
const cb = call.arguments[call.arguments.length - 1];
if (!(ts.isArrowFunction(cb) || ts.isFunctionExpression(cb)) || cb.parameters.length || !ts.isBlock(cb.body)) throw new Error("describe callback must be a parameterless block function");
const inner = [...cb.body.statements].map((n, i, a) => ({ node: n, index: i, a: L(n.getStart(sf)), b: L(n.getEnd()), gapFrom: (i === 0 ? L(cb.body.getStart(sf)) : L(a[i - 1].getEnd())) + 1 }));
const innerByLine = new Map(inner.map((s) => [s.a, s]));
inner.forEach((s, i) => { if (i > 0 && inner[i - 1].b === s.a) errors.push(`callback statements share line ${s.a}`); });
const regionRe = /^\s*\/\/\s*#(end)?region\b/;

const symOf = (id) => checker.getSymbolAtLocation(id);
const kindOfDecl = (sym) => {
  const d = sym.declarations && sym.declarations[0];
  if (!d) return "unknown";
  if (d.getSourceFile() !== sf) return "global";
  if (ts.isImportSpecifier(d) || ts.isImportClause(d) || ts.isNamespaceImport(d)) return "import";
  const top = ts.isVariableDeclaration(d) ? d.parent && d.parent.parent && d.parent.parent.parent : d.parent;
  if (top && ts.isSourceFile(top) && (ts.isVariableDeclaration(d) || ts.isFunctionDeclaration(d) || ts.isClassDeclaration(d) || ts.isTypeAliasDeclaration(d) || ts.isInterfaceDeclaration(d) || ts.isEnumDeclaration(d))) {
    if (ts.isVariableDeclaration(d)) return ts.getCombinedNodeFlags(d) & ts.NodeFlags.Const ? "module-const" : "module-let";
    if (ts.isFunctionDeclaration(d) || ts.isClassDeclaration(d)) return "module-fn";
    if (ts.isTypeAliasDeclaration(d) || ts.isInterfaceDeclaration(d)) return "module-type";
    return "module-enum";
  }
  let p = d;
  while (p && p.parent && !(ts.isBlock(p.parent) && p.parent.parent === cb)) p = p.parent;
  if (p && p.parent && ts.isBlock(p.parent) && p.parent.parent === cb) return "callback-local";
  return "inner";
};
const refsOf = (node) => {
  const out = [];
  const visit = (n) => {
    if (ts.isIdentifier(n)) {
      const p = n.parent;
      const declName = (ts.isTypeAliasDeclaration(p) || ts.isInterfaceDeclaration(p) || ts.isClassDeclaration(p) || ts.isFunctionDeclaration(p) || ts.isVariableDeclaration(p) || ts.isParameter(p) || ts.isBindingElement(p)) && p.name === n;
      const skip = declName || (ts.isPropertyAccessExpression(p) && p.name === n) || (ts.isPropertyAssignment(p) && p.name === n) || (ts.isQualifiedName(p) && p.right === n) || ((ts.isMethodDeclaration(p) || ts.isPropertyDeclaration(p) || ts.isPropertySignature(p) || ts.isMethodSignature(p)) && p.name === n);
      if (!skip) {
        let sym = symOf(n);
        if (ts.isShorthandPropertyAssignment(p) && p.name === n) sym = checker.getShorthandAssignmentValueSymbol(p) || sym;
        if (sym) out.push({ id: n, sym, kind: kindOfDecl(sym), name: n.text });
      }
    }
    ts.forEachChild(n, visit);
  };
  visit(node);
  return out;
};
inner.forEach((s) => {
  s.refs = refsOf(s.node);
  s.isDecl = ts.isVariableStatement(s.node) || ts.isFunctionDeclaration(s.node) || ts.isClassDeclaration(s.node);
  s.declSyms = [];
  if (ts.isVariableStatement(s.node)) for (const d of s.node.declarationList.declarations) { const w = (n) => { if (ts.isIdentifier(n)) { const sym = symOf(n); if (sym) s.declSyms.push(sym); } else n.elements && n.elements.forEach((e) => !ts.isOmittedExpression(e) && w(e.name)); }; w(d.name); }
  else if ((ts.isFunctionDeclaration(s.node) || ts.isClassDeclaration(s.node)) && s.node.name) { const sym = symOf(s.node.name); if (sym) s.declSyms.push(sym); }
});
const endRe = /^\s*\/\/\s*#endregion\b/;
inner.forEach((s, i) => {
  s.tailB = s.b;
  const gapEnd = i + 1 < inner.length ? inner[i + 1].a - 1 : L(cb.body.getEnd()) - 1;
  let last = -1;
  for (let l = s.b + 1; l <= gapEnd; l++) if (endRe.test(lines[l - 1])) last = l;
  if (last > 0) { s.tailB = last; if (i + 1 < inner.length) inner[i + 1].gapFrom = Math.max(inner[i + 1].gapFrom, last + 1); }
});

const owner = new Map();
const parts = spec.parts.map((p) => ({ spec: p, stmts: [] }));
parts.forEach((p) => {
  const a = innerByLine.get(p.spec.from), b = innerByLine.get(p.spec.to);
  if (!a || !b) { errors.push(`part ${p.spec.name}: no statement at ${p.spec.from} or ${p.spec.to}`); return; }
  for (const s of inner) if (s.index >= a.index && s.index <= b.index) { if (owner.has(s)) errors.push(`statement ${s.a} claimed twice`); owner.set(s, p); p.stmts.push(s); }
});
for (const s of inner) if (!owner.has(s)) errors.push(`statement at ${s.a} is not covered`);
const symOwners = new Map();
for (const s of inner) for (const sym of s.declSyms) symOwners.set(sym, s);
const hoisted = new Set();
for (let changed = true; changed; ) {
  changed = false;
  for (const s of inner) for (const r of s.refs) if (r.kind === "callback-local") {
    const d = symOwners.get(r.sym);
    if (!d || d === s || hoisted.has(d)) continue;
    if (owner.get(d) !== owner.get(s) || hoisted.has(s)) { hoisted.add(d); changed = true; }
  }
}
for (const d of hoisted) {
  const okKind = ts.isFunctionDeclaration(d.node) || ts.isClassDeclaration(d.node) || (ts.isVariableStatement(d.node) && (ts.getCombinedNodeFlags(d.node.declarationList) & ts.NodeFlags.Const));
  if (!okKind) errors.push(`callback-local declaration at line ${d.a} is shared across parts but is not a function, class or const`);
}
for (const s of inner) for (const r of s.refs) if (r.kind === "callback-local") {
  const d = symOwners.get(r.sym);
  if (d && d.index > s.index && !ts.isFunctionDeclaration(d.node) && !hoisted.has(d)) errors.push(`callback-local ${r.name} used before its declaration`);
}
const sharedDecls = new Map();
for (const s of inner) for (const r of s.refs) {
  if (r.kind === "module-let") errors.push(`mutable module binding ${r.name} used at line ${s.a}`);
  if (r.kind === "module-const" || r.kind === "module-fn" || r.kind === "module-type" || r.kind === "module-enum" || r.kind === "module-other") sharedDecls.set(r.sym, r);
}
const moduleStmtsOf = (sym) => sym.declarations.filter((d) => d.getSourceFile() === sf).map((d0) => { let d = d0; while (d.parent && !ts.isSourceFile(d.parent)) d = d.parent; return d; });
const closure = new Set(sharedDecls.keys());
for (let changed = true; changed; ) {
  changed = false;
  for (const sym of [...closure]) for (const r of moduleStmtsOf(sym).flatMap((st) => refsOf(st))) if (["module-const", "module-fn", "module-type", "module-enum", "module-other", "module-let"].includes(r.kind) && !closure.has(r.sym) && r.sym !== sym) { if (r.kind === "module-let") errors.push(`shared declaration ${sym.name} uses mutable ${r.name}`); closure.add(r.sym); changed = true; }
}
const sharedStmts = [...new Set([...closure].flatMap(moduleStmtsOf))].sort((x, y) => x.getStart(sf) - y.getStart(sf));
for (const st of sharedStmts) {
  if (st.modifiers && st.modifiers.some((m) => m.kind === ts.SyntaxKind.ExportKeyword)) errors.push(`shared declaration at line ${L(st.getStart(sf))} is exported from the root`);
  if (/import\.meta|__dirname|__filename/.test(st.getText(sf)) && !(spec.sharedRewrite || []).some(([from]) => st.getText(sf).includes(from))) errors.push(`shared declaration at line ${L(st.getStart(sf))} is anchored to the file location (import.meta); add a sharedRewrite entry`);
}
if ((sharedStmts.length || hoisted.size) && !spec.shared) errors.push(`parts use module declarations (${[...closure].map((s) => s.name)}) but the spec has no shared file`);
const otherModuleUsers = new Set();
for (const st of sf.statements) {
  if (st === dstmt || ts.isImportDeclaration(st) || sharedStmts.includes(st)) continue;
  for (const r of refsOf(st)) if (closure.has(r.sym)) otherModuleUsers.add(r.name);
}

const importStmts = sf.statements.filter(ts.isImportDeclaration);
const childRoot = spec.childRoot || path.dirname(file);
const childPath = (dir) => path.join(childRoot, dir, "🟦️" + ext);
const rebase = (fromFile) => (specText) => {
  if (!specText.startsWith(".")) return specText;
  let r = path.relative(path.dirname(fromFile), path.resolve(path.dirname(file), specText)).split(path.sep).join("/");
  if (!r.startsWith(".")) r = "./" + r;
  return r;
};
const relTo = (fromFile, toFile) => { let r = path.relative(path.dirname(fromFile), toFile).split(path.sep).join("/"); if (!r.startsWith(".")) r = "./" + r; return r; };
const identsOf = (code) => {
  const f = ts.createSourceFile("x" + ext, code, ts.ScriptTarget.Latest, true, ext === ".tsx" ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const used = new Set();
  const visit = (n) => {
    if (ts.isImportDeclaration(n)) return;
    if (ts.isIdentifier(n)) {
      const p = n.parent;
      const skip = p && ((ts.isPropertyAccessExpression(p) && p.name === n) || (ts.isPropertyAssignment(p) && p.name === n) || (ts.isQualifiedName(p) && p.right === n) || ((ts.isMethodDeclaration(p) || ts.isPropertyDeclaration(p) || ts.isPropertySignature(p) || ts.isMethodSignature(p)) && p.name === n));
      if (!skip) used.add(n.text);
      if (ts.isJsxOpeningElement(p) || ts.isJsxSelfClosingElement(p) || ts.isJsxClosingElement(p)) used.add(n.text);
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
    const attrs = n.attributes ? ` ${n.attributes.getText(sf)}` : "";
    out.push(`import ${c.isTypeOnly ? "type " : ""}${bits.join(", ")} from ${q}${sp}${q}${attrs};`);
  }
  return out;
};
const multiLineLiteralLines = (node) => {
  const set = new Set();
  const w = (n) => { if (ts.isTemplateExpression(n) || ts.isNoSubstitutionTemplateLiteral(n) || ts.isStringLiteral(n)) { const a = L(n.getStart(sf)), b = L(n.getEnd()); for (let l = a + 1; l <= b; l++) set.add(l); } ts.forEachChild(n, w); };
  w(node);
  return set;
};
const offsetOfLine = (line) => (line <= 1 ? 0 : lines.slice(0, line - 1).join("\n").length + 1);
const importSpecEdits = (node, toFile) => {
  const edits = [];
  const fix = rebase(toFile);
  const lit = (l) => { if (l && ts.isStringLiteral(l) && l.text.startsWith(".")) { const q = l.getText(sf)[0]; edits.push({ s: l.getStart(sf), e: l.getEnd(), t: q + fix(l.text) + q }); } };
  const w = (n) => {
    if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword) lit(n.arguments[0]);
    if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument)) lit(n.argument.literal);
    if (ts.isNewExpression(n) && n.expression.getText(sf) === "URL") { /* URL(import.meta.url) based reads resolve relative to the file: flagged below */ }
    ts.forEachChild(n, w);
  };
  w(node);
  return edits;
};
const urlRelative = (node) => { let hit = false; const w = (n) => { if (ts.isNewExpression(n) && n.expression.getText(sf) === "URL" && n.arguments && n.arguments.length === 2 && /import\.meta\.url|import\.meta\.dir/.test(n.arguments[1].getText(sf)) && ts.isStringLiteral(n.arguments[0]) && n.arguments[0].text.startsWith(".")) hit = true; if (/import\.meta\.(dir|dirname|url|path)/.test(n.getText(sf)) && ts.isMetaProperty(n)) hit = true; ts.forEachChild(n, w); }; w(node); return hit; };
const stmtChunk = (s, toFile, dedent, startLine) => {
  const lit = multiLineLiteralLines(s.node);
  const from = startLine ?? s.gapFrom;
  const base = offsetOfLine(from);
  let chunk = lines.slice(from - 1, s.tailB ?? s.b).join("\n");
  for (const e of importSpecEdits(s.node, toFile).map((x) => ({ s: x.s - base, e: x.e - base, t: x.t })).sort((x, y) => y.s - x.s)) chunk = chunk.slice(0, e.s) + e.t + chunk.slice(e.e);
  const out = [];
  chunk.split("\n").forEach((l, i) => { const lineNo = from + i; if (dedent && !lit.has(lineNo) && l.startsWith(" ".repeat(dedent))) l = l.slice(dedent); out.push(l); });
  while (out.length && !out[0].trim()) out.shift();
  return out.join("\n");
};
for (const s of inner) if (urlRelative(s.node)) errors.push(`statement at ${s.a} derives paths from import.meta / relative URL (moves with the file)`);

const modList = sf.statements.map((st, i) => {
  const a = L(st.getStart(sf)), b = L(st.getEnd());
  const prevB = i === 0 ? 0 : L(sf.statements[i - 1].getEnd());
  const nextA = i + 1 < sf.statements.length ? L(sf.statements[i + 1].getStart(sf)) : lines.length + 1;
  return { st, a, b, prevB, nextA, tailB: b };
});
modList.forEach((m, i) => {
  let last = -1;
  for (let l = m.b + 1; l < m.nextA; l++) if (/^\s*\/\/\s*#endregion\b/.test(lines[l - 1])) last = l;
  if (last > 0) { m.tailB = last; if (i + 1 < modList.length) modList[i + 1].prevB = Math.max(modList[i + 1].prevB, last); }
});
const modInfo = new Map(modList.map((m) => [m.st, m]));
const outputs = [];
const emojis = new Map();
const claim = (e, who) => { const k = stripV(e); if (emojis.has(k)) errors.push(`emoji ${e} repeated (${emojis.get(k)}, ${who})`); emojis.set(k, who); };
const docTop = (doc) => { if (!/^\/\*\*\s+\S/.test(doc)) errors.push(`doc must start with /** : ${doc}`); return doc; };
const shared = spec.shared;
const sharedFile = shared ? childPath(shared.dir) : null;
const sharedNames = [];
if (shared && (sharedStmts.length || hoisted.size)) {
  claim(shared.dir.split(V)[0], "folder " + shared.dir);
  const hoistedItems = [...hoisted].sort((x, y) => x.index - y.index);
  const hoistedTexts = hoistedItems.map((hs) => {
    let t = stmtChunk(hs, sharedFile, 2);
    if (!/^(\/\*\*[\s\S]*?\*\/\s*)?export\b/.test(t)) t = t.replace(/(^|\*\/\s*)(const|let|var|function|async function|class)\b/, (mm, pre, kw) => `${pre}export ${kw}`);
    for (const sym of hs.declSyms) sharedNames.push(sym.name);
    return t;
  });
  const texts = sharedStmts.map((st) => {
    const mi = modInfo.get(st);
    const m = { gapFrom: mi.prevB + 1, a: mi.a, b: mi.b, tailB: mi.tailB, node: st };
    let t = stmtChunk(m, sharedFile, 0);
    for (const [from, to] of spec.sharedRewrite || []) t = t.split(from).join(to);
    if (!/^(\/\*\*[\s\S]*?\*\/\s*)?export\b/.test(t)) t = t.replace(/(^|\*\/\s*)(const|let|var|function|async function|class|type|interface|enum)\b/, (mm, pre, kw) => `${pre}export ${kw}`);
    if (ts.isVariableStatement(st)) for (const d of st.declarationList.declarations) if (ts.isIdentifier(d.name)) sharedNames.push(d.name.text);
    if ((ts.isFunctionDeclaration(st) || ts.isClassDeclaration(st) || ts.isTypeAliasDeclaration(st) || ts.isInterfaceDeclaration(st) || ts.isEnumDeclaration(st)) && st.name) sharedNames.push(st.name.text);
    return t;
  });
  const code = [...texts, ...hoistedTexts].join("\n\n");
  const imps = prunedImports(identsOf(code), rebase(sharedFile));
  outputs.push({ path: sharedFile, text: `${docTop(shared.doc)}\n${imps.join("\n")}${imps.length ? "\n\n" : ""}${code}\n`, kind: "shared" });
}
const sharedByName = new Map();
for (const sym of closure) for (const st of moduleStmtsOf(sym)) sharedByName.set(sym.name + (ts.isTypeAliasDeclaration(st) || ts.isInterfaceDeclaration(st) ? "#type" : ""), st);
for (const p of parts) {
  const s = p.spec;
  claim(s.emoji, s.name);
  claim(s.dir.split(V)[0], "folder " + s.dir);
  const outFile = childPath(s.dir);
  const body = p.stmts.filter((st) => !hoisted.has(st)).map((st) => stmtChunk(st, outFile, 0)).join("\n");
  const code = [`/** ${s.emoji} ${s.fdoc} */`, `export function ${s.name}(): void {`, body, "}"].join("\n");
  const used = identsOf(code);
  const imps = prunedImports(used, rebase(outFile));
  const fromShared = [...new Set(sharedNames)].filter((n) => used.has(n));
  if (fromShared.length) {
    const typeOnly = fromShared.filter((n) => !sharedByName.has(n) && sharedByName.has(n + "#type"));
    const values = fromShared.filter((n) => !typeOnly.includes(n));
    if (values.length) imps.push(`import { ${values.sort().join(", ")} } from "${relTo(outFile, sharedFile)}";`);
    if (typeOnly.length) imps.push(`import type { ${typeOnly.sort().join(", ")} } from "${relTo(outFile, sharedFile)}";`);
  }
  outputs.push({ path: outFile, text: `${docTop(s.doc)}\n${imps.join("\n")}${imps.length ? "\n\n" : ""}${code}\n`, kind: "part" });
}
{
  const replaceStart = cb.body.getStart(sf) + 1;
  const replaceEnd = cb.body.getEnd() - 1;
  const calls = parts.map((p) => `  ${p.spec.name}();`).join("\n");
  const removeRanges = sharedStmts.map((st) => { const mi = modInfo.get(st); const e = offsetOfLine(mi.tailB + 1) - 1; return { s: offsetOfLine(mi.prevB + 1), e: Math.max(st.getEnd(), Math.min(e, text.length)) }; });
  const edits = [{ s: replaceStart, e: replaceEnd, t: `\n${calls}\n` }, ...removeRanges.map((r) => ({ s: r.s, e: r.e, t: "" }))].sort((a, b) => b.s - a.s);
  let out = text;
  for (const e of edits) out = out.slice(0, e.s) + e.t + out.slice(e.e);
  const f2 = ts.createSourceFile(file, out, ts.ScriptTarget.Latest, true, ext === ".tsx" ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const head = [];
  const importEdits = [];
  const usedNow = identsOf(out.slice(f2.statements.filter(ts.isImportDeclaration).reduce((m, n) => Math.max(m, n.getEnd()), 0)));
  const importsNow = f2.statements.filter(ts.isImportDeclaration);
  for (const n of importsNow) {
    const c = n.importClause;
    if (!c) continue;
    const def = c.name && usedNow.has(c.name.text) ? c.name.text : null;
    let ns = null;
    const named = [];
    if (c.namedBindings && ts.isNamespaceImport(c.namedBindings) && usedNow.has(c.namedBindings.name.text)) ns = c.namedBindings.name.text;
    if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const el of c.namedBindings.elements) if (usedNow.has(el.name.text)) named.push(el.getText(f2));
    const q = n.moduleSpecifier.getText(f2)[0];
    const bits = [];
    if (def) bits.push(def);
    if (ns) bits.push(`* as ${ns}`);
    if (named.length) bits.push(`{ ${named.join(", ")} }`);
    const attrs = n.attributes ? ` ${n.attributes.getText(f2)}` : "";
    const replacement = bits.length ? `import ${c.isTypeOnly ? "type " : ""}${bits.join(", ")} from ${q}${n.moduleSpecifier.text}${q}${attrs};` : "";
    importEdits.push({ s: n.getStart(f2), e: n.getEnd(), t: replacement });
  }
  const adds = [];
  if (sharedFile) {
    const need = [...new Set(sharedNames)].filter((n) => usedNow.has(n));
    if (need.length) adds.push(`import { ${need.join(", ")} } from "${relTo(file, sharedFile)}";`);
  }
  for (const p of parts) adds.push(`import { ${p.spec.name} } from "${relTo(file, childPath(p.spec.dir))}";`);
  const lastImport = importsNow.length ? importsNow[importsNow.length - 1] : null;
  const insertAt = lastImport ? lastImport.getEnd() : 0;
  importEdits.push({ s: insertAt, e: insertAt, t: (lastImport ? "\n" : "") + adds.join("\n") + (lastImport ? "" : "\n") });
  for (const e of importEdits.sort((a, b) => b.s - a.s)) out = out.slice(0, e.s) + e.t + out.slice(e.e);
  out = out.replace(/^\n+/, "").replace(/\n{3,}/g, "\n\n").replace(/(\*\/)\n\n(import )/, "$1\n$2").replace(/(;)\n\n(import (?:\{|"|type|\w))/g, "$1\n$2");
  outputs.unshift({ path: file, text: out, kind: "root" });
}
for (const o of outputs) {
  const open = (o.text.match(/^\s*\/\/\s*#region\b/gm) || []).length, close = (o.text.match(/^\s*\/\/\s*#endregion\b/gm) || []).length;
  if (open !== close) errors.push(`region markers unbalanced in ${path.basename(path.dirname(o.path))} (${open} regions, ${close} endregions)`);
}
const lc = (t) => t.split("\n").length - 1;
console.log(JSON.stringify({ outputs: outputs.map((o) => ({ path: path.relative(childRoot, o.path), kind: o.kind, lines: lc(o.text) })), sharedNames, otherModuleUsers: [...otherModuleUsers], errors }, null, 1));
if (process.env.DUMP) { fs.mkdirSync(process.env.DUMP, { recursive: true }); outputs.forEach((o, i) => fs.writeFileSync(path.join(process.env.DUMP, `${i}-${path.basename(path.dirname(o.path))}${ext}`), o.text)); }
if (APPLY) {
  if (errors.length) { console.error("REFUSED: errors present"); process.exit(2); }
  for (const o of outputs.slice(1)) if (fs.existsSync(o.path)) { console.error("REFUSED: child exists " + o.path); process.exit(4); }
  if (fs.readFileSync(file, "utf8") !== text) { console.error("REFUSED: root changed during run"); process.exit(5); }
  for (const o of outputs.slice(1)) { fs.mkdirSync(path.dirname(o.path), { recursive: true }); fs.writeFileSync(o.path, o.text); }
  fs.writeFileSync(file, outputs[0].text);
  console.log("APPLIED");
}
