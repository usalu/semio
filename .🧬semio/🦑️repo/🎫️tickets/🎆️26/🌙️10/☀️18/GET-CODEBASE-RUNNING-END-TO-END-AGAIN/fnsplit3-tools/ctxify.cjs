// usage: node ctxify.cjs <spec.json> [APPLY]
// Closure factory -> explicit context. The function declarations of a factory move to child modules, their closure state (host members, `post`, `let` variables) becomes `ctx`, calls between them pass `ctx`.
// spec: { file, factoryLine, hostStmtLine, ctxConsts: [{line, name, type, create}], lets: [{line}], sharedStmts: [line], sharedDir, sharedDoc, ctxTypeName, stateTypeName, createName, hostType, hostParam,
//         files: [{ id, dir, doc, fns: [{name, emoji, doc?}] }], keepFns: [name], rootEmoji? }
const fs = require("fs");
const path = require("path");
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");

const REPO = "/Users/ueli/Documents/semio";
const VS = "️";
const spec = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const APPLY = process.argv[3] === "APPLY";
const file = path.isAbsolute(spec.file) ? spec.file : path.join(REPO, spec.file);
const text = fs.readFileSync(process.env.ORIG_TEXT || file, "utf8");
const errors = [];
const warns = [];
const cfg = JSON.parse(fs.readFileSync(path.join(REPO, "tsconfig.json"), "utf8"));
const { options } = ts.convertCompilerOptionsFromJson({ ...cfg.compilerOptions, noEmit: true, incremental: false, types: [] }, REPO);
options.noEmit = true;
const host = ts.createCompilerHost(options);
const origGet = host.getSourceFile.bind(host);
host.getSourceFile = (fn, lv, ...r) => (path.resolve(fn) === path.resolve(file) ? ts.createSourceFile(fn, text, lv, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS) : origGet(fn, lv, ...r));
const program = ts.createProgram([file], options, host);
const sf = program.getSourceFile(file);
const checker = program.getTypeChecker();
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
const ext = path.extname(file);

let factory = null;
{
  const visit = (n) => { if (factory) return; if (ts.isFunctionDeclaration(n) && n.body && L(n.getStart(sf)) === spec.factoryLine) { factory = n; return; } ts.forEachChild(n, visit); };
  visit(sf);
}
if (!factory) throw new Error("factory not found");
const stmts = factory.body.statements;
const stmtAtLine = (line) => stmts.find((s) => L(s.getStart(sf)) === line);
const declSym = (id) => checker.getSymbolAtLocation(id);
const declOf = (sym) => sym && (sym.valueDeclaration || (sym.declarations && sym.declarations[0]));
const inFactory = (d) => d && d.getSourceFile() === sf && d.pos >= factory.body.pos && d.end <= factory.body.end;
const hostParamSym = declSym(factory.parameters[0].name);

const hostNames = new Map();
{
  const st = stmtAtLine(spec.hostStmtLine);
  for (const el of st.declarationList.declarations[0].name.elements) hostNames.set(declSym(el.name), el.name.text);
}
const ctxConstSyms = new Map();
const ctxConstStmts = new Set();
for (const c of spec.ctxConsts || []) {
  const st = stmtAtLine(c.line);
  ctxConstStmts.add(st);
  ctxConstSyms.set(declSym(st.declarationList.declarations[0].name), c);
}
const letSyms = new Map();
const letStmts = new Set();
const letInfo = [];
for (const l of spec.lets || []) {
  const st = stmtAtLine(l.line);
  letStmts.add(st);
  const d = st.declarationList.declarations[0];
  const sym = declSym(d.name);
  const type = d.type ? d.type.getText(sf) : checker.typeToString(checker.getBaseTypeOfLiteralType(checker.getTypeOfSymbolAtLocation(sym, d)));
  const init = d.initializer ? d.initializer.getText(sf) : "undefined";
  letSyms.set(sym, d.name.text);
  letInfo.push({ name: d.name.text, type, init, leading: leadingComments(st) });
}
function leadingComments(node) {
  const r = (ts.getLeadingCommentRanges(text, node.getFullStart()) || []);
  return r.length ? text.slice(r[0].pos, node.getStart(sf)).replace(/\s+$/, "") : "";
}
const sharedStmtSet = new Set((spec.sharedStmts || []).map((l) => { const s = stmtAtLine(l); if (!s) throw new Error("shared stmt line " + l); return s; }));
const fnDecls = stmts.filter((s) => ts.isFunctionDeclaration(s) && s.name);
const fnByName = new Map(fnDecls.map((f) => [f.name.text, f]));
const fnSyms = new Map(fnDecls.map((f) => [declSym(f.name), f.name.text]));
const fileOfFn = new Map();
for (const f of spec.files) for (const fn of f.fns) { if (!fnByName.has(fn.name)) errors.push(`function ${fn.name} not found`); fileOfFn.set(fn.name, { file: f, spec: fn }); }
for (const f of fnDecls) if (!fileOfFn.has(f.name.text)) errors.push(`function ${f.name.text} has no file`);
const returnStmt = stmts.find((s) => ts.isReturnStatement(s));

const skipId = (n) => {
  const p = n.parent;
  return (ts.isPropertyAccessExpression(p) && p.name === n) || (ts.isPropertyAssignment(p) && p.name === n) || (ts.isQualifiedName(p) && p.right === n) || ((ts.isMethodDeclaration(p) || ts.isPropertyDeclaration(p) || ts.isPropertySignature(p) || ts.isMethodSignature(p) || ts.isGetAccessor(p) || ts.isSetAccessor(p)) && p.name === n) || (ts.isBindingElement(p) && p.propertyName === n) || (ts.isLabeledStatement(p) && p.label === n) || (ts.isBreakOrContinueStatement(p) && p.label === n) || (ts.isParameter(p) && p.name === n) || (ts.isFunctionDeclaration(p) && p.name === n) || (ts.isVariableDeclaration(p) && p.name === n) || (ts.isBindingElement(p) && p.name === n);
};
const symOf = (n) => {
  const p = n.parent;
  if (ts.isShorthandPropertyAssignment(p) && p.name === n) return checker.getShorthandAssignmentValueSymbol(p) || checker.getSymbolAtLocation(n);
  return checker.getSymbolAtLocation(n);
};

const refsOf = (node) => {
  const out = [];
  const w = (n) => { if (ts.isIdentifier(n) && !skipId(n)) { const s = symOf(n); if (s) out.push({ n, s }); } ts.forEachChild(n, w); };
  w(node);
  return out;
};
const needsCtx = new Set();
const fnRefs = new Map();
for (const f of fnDecls) fnRefs.set(f.name.text, refsOf(f));
let changed = true;
while (changed) {
  changed = false;
  for (const f of fnDecls) {
    if (needsCtx.has(f.name.text)) continue;
    const hit = fnRefs.get(f.name.text).some(({ s }) => hostNames.has(s) || s === hostParamSym || ctxConstSyms.has(s) || letSyms.has(s) || (fnSyms.has(s) && needsCtx.has(fnSyms.get(s)) && fnSyms.get(s) !== f.name.text));
    if (hit) { needsCtx.add(f.name.text); changed = true; }
  }
}

const ctxName = spec.ctxTypeName;
const rewrite = (node, { addParam }) => {
  const edits = [];
  const usedHost = new Set();
  for (const { n, s } of refsOf(node)) {
    const p = n.parent;
    if (letSyms.has(s)) {
      if (ts.isShorthandPropertyAssignment(p)) { errors.push(`shorthand use of let ${n.text}`); continue; }
      edits.push({ s: n.getStart(sf), e: n.getEnd(), t: `ctx.state.${n.text}` });
    } else if (ctxConstSyms.has(s)) {
      if (ts.isShorthandPropertyAssignment(p)) { errors.push(`shorthand use of ${n.text}`); continue; }
      edits.push({ s: n.getStart(sf), e: n.getEnd(), t: `ctx.${n.text}` });
    } else if (s === hostParamSym) {
      edits.push({ s: n.getStart(sf), e: n.getEnd(), t: "ctx.host" });
    } else if (hostNames.has(s)) {
      if (ts.isShorthandPropertyAssignment(p)) { errors.push(`shorthand use of host member ${n.text}`); continue; }
      usedHost.add(hostNames.get(s));
    } else if (fnSyms.has(s) && needsCtx.has(fnSyms.get(s))) {
      if (ts.isShorthandPropertyAssignment(p) && spec.dropName) {
        edits.push({ s: n.getStart(sf), e: n.getEnd(), t: `${n.text}: (...args: ${spec.dropName}<Parameters<typeof ${n.text}>>) => ${n.text}(ctx, ...args)` });
      } else if (ts.isCallExpression(p) && p.expression === n) {
        const open = p.arguments.pos;
        edits.push({ s: open, e: open, t: "ctx" + (p.arguments.length ? ", " : "") });
      } else if (!(ts.isFunctionDeclaration(p) && p.name === n)) errors.push(`context function ${n.text} used as a value at line ${L(n.getStart(sf))}`);
    }
  }
  return { edits, usedHost };
};
const applyEdits = (src, base, edits) => {
  let out = src;
  for (const e of [...edits].sort((a, b) => b.s - a.s || b.e - a.e)) out = out.slice(0, e.s - base) + e.t + out.slice(e.e - base);
  return out;
};

const fnTexts = new Map();
for (const f of fnDecls) {
  const name = f.name.text;
  const lead = (ts.getLeadingCommentRanges(text, f.getFullStart()) || []);
  const start = lead.length ? lead[0].pos : f.getStart(sf);
  const { edits, usedHost } = rewrite(f, {});
  if (needsCtx.has(name)) edits.push({ s: f.parameters.pos, e: f.parameters.pos, t: `ctx: ${ctxName}` + (f.parameters.length ? ", " : "") });
  let body = applyEdits(text.slice(start, f.getEnd()), start, edits);
  if (usedHost.size) {
    const open = bodyOpenIndex(f, start, edits);
    const names = [...usedHost];
    body = body.slice(0, open + 1) + `\n  const { ${names.join(", ")} } = ctx.host;` + body.slice(open + 1);
  }
  body = body.split("\n").filter((l) => !/^\s*\/\/\s*#(end)?region\b/.test(l)).join("\n");
  fnTexts.set(name, { text: body, hostUsed: usedHost, lead: text.slice(start, f.getStart(sf)) });
}
function bodyOpenIndex(f, start, edits) {
  const off = f.body.getStart(sf) - start;
  let shift = 0;
  for (const e of edits) if (e.e <= f.body.getStart(sf)) shift += e.t.length - (e.e - e.s);
  return off + shift;
}
const { edits: retEdits, usedHost: retHost } = rewrite(returnStmt, {});
let retText = applyEdits(text.slice(returnStmt.getStart(sf), returnStmt.getEnd()), returnStmt.getStart(sf), retEdits);

const importStmts = sf.statements.filter(ts.isImportDeclaration);
const identsOf = (code) => {
  const f = ts.createSourceFile("x" + ext, code, ts.ScriptTarget.Latest, true);
  const used = new Set();
  const visit = (n) => {
    if (ts.isImportDeclaration(n)) return;
    if (ts.isIdentifier(n)) {
      const p = n.parent;
      const skip = p && ((ts.isPropertyAccessExpression(p) && p.name === n) || (ts.isPropertyAssignment(p) && p.name === n) || (ts.isQualifiedName(p) && p.right === n) || ((ts.isMethodDeclaration(p) || ts.isPropertyDeclaration(p) || ts.isPropertySignature(p) || ts.isMethodSignature(p)) && p.name === n));
      if (!skip) used.add(n.text);
    }
    ts.forEachChild(n, visit);
  };
  visit(f);
  return used;
};
const relTo = (fromFile, toFile) => { let r = path.relative(path.dirname(fromFile), toFile).split(path.sep).join("/"); if (!r.startsWith(".")) r = "./" + r; return r; };
const rebase = (fromFile) => (specText) => {
  if (!specText.startsWith(".")) return specText;
  let r = path.relative(path.dirname(fromFile), path.resolve(path.dirname(file), specText)).split(path.sep).join("/");
  if (!r.startsWith(".")) r = "./" + r;
  return r;
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

const topDecls = new Map();
for (const st of sf.statements) {
  if (st === factory || ts.isImportDeclaration(st)) continue;
  const names = [];
  if (ts.isVariableStatement(st)) st.declarationList.declarations.forEach((d) => ts.isIdentifier(d.name) && names.push(d.name.text));
  else if ((ts.isFunctionDeclaration(st) || ts.isTypeAliasDeclaration(st) || ts.isInterfaceDeclaration(st) || ts.isClassDeclaration(st) || ts.isEnumDeclaration(st)) && st.name) names.push(st.name.text);
  for (const n of names) topDecls.set(n, st);
}
const innerShared = [...sharedStmtSet];
const innerSharedNames = new Map();
for (const st of innerShared) {
  const names = [];
  if (ts.isVariableStatement(st)) st.declarationList.declarations.forEach((d) => ts.isIdentifier(d.name) && names.push(d.name.text));
  else if ((ts.isTypeAliasDeclaration(st) || ts.isInterfaceDeclaration(st)) && st.name) names.push(st.name.text);
  for (const n of names) innerSharedNames.set(n, st);
}

const sharedPath = path.join(path.dirname(file), spec.sharedDir, "🟦️" + ext);
const sharedBodyParts = [];
const movedTop = new Set();
{
  const frontier = [...fnTexts.values()].map((f) => f.text).concat([...innerShared].map((s) => s.getText(sf)), letInfo.map((l) => l.type + " " + l.init));
  const seen = new Set();
  let work = frontier;
  while (work.length) {
    const next = [];
    for (const t of work) for (const n of identsOf(t)) {
      const st = topDecls.get(n);
      if (!st || seen.has(st)) continue;
      seen.add(st); movedTop.add(st);
      next.push(st.getText(sf));
    }
    work = next;
  }
}
const topTexts = [...movedTop].sort((a, b) => a.getStart(sf) - b.getStart(sf)).map((st) => {
  let t = st.getFullText(sf).replace(/^\s+/, "");
  t = t.replace(/^((?:\/\*\*[\s\S]*?\*\/\s*|\/\/[^\n]*\n\s*)*)(const|let|function|async function|class|type|interface|enum)\b/, (m, c, kw) => `${c}export ${kw}`);
  return t.trimEnd();
});
const innerTexts = innerShared.sort((a, b) => a.getStart(sf) - b.getStart(sf)).map((st) => {
  const lead = (ts.getLeadingCommentRanges(text, st.getFullStart()) || []);
  const start = lead.length ? lead[0].pos : st.getStart(sf);
  let t = text.slice(start, st.getEnd()).split("\n").filter((l) => !/^\s*\/\/\s*#(end)?region\b/.test(l)).join("\n").replace(/^\s+/, "");
  t = t.replace(/^((?:\/\*\*[\s\S]*?\*\/\s*|\/\/[^\n]*\n\s*)*)(const|type|interface)\b/, (m, c, kw) => `${c}export ${kw}`);
  return t;
});
const stateMembers = letInfo.map((l) => `${l.leading ? l.leading.split("\n").map((x) => "  " + x.trim().replace(/^\*/, " *")).join("\n") + "\n" : ""}  ${l.name}: ${l.type};`).join("\n");
const stateInit = letInfo.map((l) => `${l.name}: ${l.init}`).join(", ");
const ctxLines = [`  readonly host: ${spec.hostType};`, ...(spec.ctxConsts || []).map((c) => `  readonly ${c.name}: ${c.type};`), `  readonly state: ${spec.stateTypeName};`];
const sharedCode = [
  ...topTexts,
  ...innerTexts,
  `/** ${spec.stateEmoji + VS} ${spec.stateDoc} */\nexport interface ${spec.stateTypeName} {\n${stateMembers}\n}`,
  `/** ${spec.ctxEmoji + VS} ${spec.ctxDoc} */\nexport interface ${ctxName} {\n${ctxLines.join("\n")}\n}`,
  ...(spec.dropName ? [`/** ${spec.dropEmoji + VS} ${spec.dropDoc} */\nexport type ${spec.dropName}<A extends readonly unknown[]> = A extends readonly [${ctxName}, ...infer R] ? R : never;`] : []),
  `/** ${spec.createEmoji + VS} ${spec.createDoc} */\nexport function ${spec.createName}(${spec.hostParam}: ${spec.hostType}): ${ctxName} {\n  return {\n    ${spec.hostParam},\n${(spec.ctxConsts || []).map((c) => `    ${c.name}: ${c.create},`).join("\n")}\n    state: { ${stateInit} },\n  };\n}`,
].join("\n\n");
const sharedUsed = identsOf(sharedCode);
const sharedImps = prunedImports(sharedUsed, rebase(sharedPath));
const sharedExports = new Set([...(spec.dropName ? [spec.dropName] : []), ...[...movedTop].flatMap((st) => { const ns = []; if (ts.isVariableStatement(st)) st.declarationList.declarations.forEach((d) => ts.isIdentifier(d.name) && ns.push(d.name.text)); else if (st.name) ns.push(st.name.text); return ns; }), ...innerSharedNames.keys(), spec.stateTypeName, ctxName, spec.createName]);
const sharedOut = { path: sharedPath, text: `/** ${spec.sharedDoc.replace(/^\/\*\*\s*/, "").replace(/\s*\*\/$/, "")} */\n${sharedImps.join("\n")}${sharedImps.length ? "\n\n" : ""}${sharedCode}\n` };

const outFiles = [];
for (const f of spec.files) {
  const outPath = path.join(path.dirname(file), f.dir, "🟦️" + ext);
  const fnList = f.fns.filter((x) => fnTexts.has(x.name));
  const parts = fnList.map((x) => {
    const info = fnTexts.get(x.name);
    let t = info.text;
    t = t.replace(/^((?:\/\*\*[\s\S]*?\*\/\s*|\/\/[^\n]*\n\s*)*)(async function|function)\b/, (m, c, kw) => `${c}export ${kw}`);
    if (x.doc && !/^\s*\/\*\*/.test(t)) t = `/** ${x.emoji + VS} ${x.doc} */\n` + t;
    return t;
  });
  const body = parts.join("\n\n");
  const used = identsOf(body);
  const imps = prunedImports(used, rebase(outPath));
  const sharedNeed = [...sharedExports].filter((n) => used.has(n));
  if (sharedNeed.length) {
    const vals = [], tys = [];
    for (const n of sharedNeed) { const st = topDecls.get(n) || innerSharedNames.get(n); const isType = st ? (ts.isTypeAliasDeclaration(st) || ts.isInterfaceDeclaration(st)) : (n === spec.stateTypeName || n === ctxName); (isType ? tys : vals).push(n); }
    if (vals.length) imps.push(`import { ${vals.sort().join(", ")} } from "${relTo(outPath, sharedPath)}";`);
    if (tys.length) imps.push(`import type { ${tys.sort().join(", ")} } from "${relTo(outPath, sharedPath)}";`);
  }
  for (const g of spec.files) if (g !== f) {
    const need = g.fns.map((x) => x.name).filter((n) => new RegExp(`\\b${n}\\b`).test(body));
    if (need.length) imps.push(`import { ${need.sort().join(", ")} } from "${relTo(outPath, path.join(path.dirname(file), g.dir, "🟦️" + ext))}";`);
  }
  outFiles.push({ path: outPath, text: `/** ${f.doc.replace(/^\/\*\*\s*/, "").replace(/\s*\*\/$/, "")} */\n${imps.join("\n")}${imps.length ? "\n\n" : ""}${body}\n`, fns: fnList.map((x) => x.name), f });
}

let retHostBlock = retHost.size ? `  const { ${[...retHost].join(", ")} } = host;\n` : "";
const factorySig = text.slice(factory.getStart(sf), factory.body.getStart(sf) + 1);
const leadFactory = leadingComments(factory);
const rootBodyCore = `${factorySig}\n  const ctx = ${spec.createName}(${spec.hostParam});\n${retHostBlock}${spec.rootRegionOpen ? "  " + spec.rootRegionOpen + "\n" : ""}  ${retText.replace(/\n/g, "\n")}\n${spec.rootRegionClose ? "  " + spec.rootRegionClose + "\n" : ""}}`;
const rootUsedProvisional = identsOf(rootBodyCore);
let newImports = [];
{
  const lastImportEnd = importStmts[importStmts.length - 1].getEnd();
  const head = text.slice(0, lastImportEnd);
  const keepTop = sf.statements.filter((st) => st !== factory && !ts.isImportDeclaration(st) && !movedTop.has(st)).map((st) => st.getFullText(sf).trim());
  const allMoved = new Set([...outFiles.flatMap((o) => [...identsOf(o.text)]), ...identsOf(sharedOut.text)]);
  const bodyOnly = rootBodyCore + "\n" + keepTop.join("\n");
  const usedRoot = identsOf(bodyOnly);
  const keepSpec = (name) => usedRoot.has(name) || !allMoved.has(name);
  const impLines = [];
  for (const n of importStmts) {
    const c = n.importClause;
    const q = n.moduleSpecifier.getText(sf)[0];
    if (!c) { impLines.push(n.getText(sf)); continue; }
    const def = c.name && keepSpec(c.name.text) ? c.name.text : null;
    let ns = null; const named = [];
    if (c.namedBindings && ts.isNamespaceImport(c.namedBindings) && keepSpec(c.namedBindings.name.text)) ns = c.namedBindings.name.text;
    if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const el of c.namedBindings.elements) if (keepSpec(el.name.text)) named.push(el.getText(sf));
    if (!def && !ns && !named.length) continue;
    const bits = [];
    if (def) bits.push(def);
    if (ns) bits.push(`* as ${ns}`);
    if (named.length) bits.push(`{ ${named.join(", ")} }`);
    impLines.push(`import ${c.isTypeOnly ? "type " : ""}${bits.join(", ")} from ${q}${n.moduleSpecifier.text}${q};`);
  }
  const sharedNeedRoot = [...sharedExports].filter((n) => usedRoot.has(n));
  const vals = [], tys = [];
  for (const n of sharedNeedRoot) { const st = topDecls.get(n) || innerSharedNames.get(n); const isType = st ? (ts.isTypeAliasDeclaration(st) || ts.isInterfaceDeclaration(st)) : (n === spec.stateTypeName || n === ctxName); (isType ? tys : vals).push(n); }
  if (vals.length) impLines.push(`import { ${vals.sort().join(", ")} } from "${relTo(file, sharedPath)}";`);
  if (tys.length) impLines.push(`import type { ${tys.sort().join(", ")} } from "${relTo(file, sharedPath)}";`);
  for (const g of spec.files) {
    const need = g.fns.map((x) => x.name).filter((n) => new RegExp(`\\b${n}\\b`).test(rootBodyCore));
    if (need.length) impLines.push(`import { ${need.sort().join(", ")} } from "${relTo(file, path.join(path.dirname(file), g.dir, "🟦️" + ext))}";`);
  }
  newImports = impLines;
  var keepTopTexts = keepTop;
  var rootHeadComment = text.slice(0, importStmts[0].getFullStart() === 0 ? importStmts[0].getStart(sf) : importStmts[0].getStart(sf)).trimEnd();
}
const rootText = `${rootHeadComment ? rootHeadComment + "\n" : ""}${newImports.join("\n")}\n\n${keepTopTexts.length ? keepTopTexts.join("\n\n") + "\n\n" : ""}${leadFactory ? leadFactory + "\n" : ""}${rootBodyCore}\n`;

{
  const owner = new Map();
  for (const o of outFiles) for (const n of o.fns) owner.set(n, o.f.id);
  const edges = new Map();
  for (const o of outFiles) for (const n of o.fns) for (const { s } of fnRefs.get(n)) if (fnSyms.has(s)) { const to = owner.get(fnSyms.get(s)); if (to && to !== o.f.id) { if (!edges.has(o.f.id)) edges.set(o.f.id, new Set()); edges.get(o.f.id).add(to); } }
  const cyc = [];
  for (const [a, tos] of edges) for (const b of tos) if (edges.get(b)?.has(a) && a < b) cyc.push(`${a} <-> ${b}`);
  if (cyc.length) warns.push("group cycles: " + cyc.join("; "));
  warns.push("group edges: " + [...edges].map(([a, t]) => `${a}->${[...t].join("+")}`).join(" "));
}
{
  const names = fnDecls.map((f) => f.name.text);
  const adj = new Map(names.map((n) => [n, new Set()]));
  for (const n of names) for (const { s } of fnRefs.get(n)) if (fnSyms.has(s) && fnSyms.get(s) !== n) adj.get(n).add(fnSyms.get(s));
  let idx = 0; const stack = []; const on = new Set(); const index = new Map(); const low = new Map(); const sccs = [];
  const strong = (v) => { index.set(v, idx); low.set(v, idx); idx++; stack.push(v); on.add(v); for (const w of adj.get(v)) { if (!index.has(w)) { strong(w); low.set(v, Math.min(low.get(v), low.get(w))); } else if (on.has(w)) low.set(v, Math.min(low.get(v), index.get(w))); } if (low.get(v) === index.get(v)) { const c = []; let w; do { w = stack.pop(); on.delete(w); c.push(w); } while (w !== v); sccs.push(c); } };
  for (const n of names) if (!index.has(n)) strong(n);
  if (process.env.FNGRAPH) for (const n of names) console.error(n + " -> " + [...adj.get(n)].join(", "));
  warns.push("sccs: " + sccs.filter((c) => c.length > 1).map((c) => `[${c.join(",")}]`).join(" "));
}
const outputs = [sharedOut, ...outFiles];
const lc = (t) => t.split("\n").length;
console.log(JSON.stringify({ rootLines: lc(rootText), needsCtx: [...needsCtx].length, ofTotal: fnDecls.length, files: outputs.map((o) => ({ file: path.relative(path.dirname(file), o.path), lines: lc(o.text), fns: o.fns })), movedTop: [...movedTop].length, errors, warns }, null, 1));
if (process.env.DUMP) { fs.mkdirSync(process.env.DUMP, { recursive: true }); fs.writeFileSync(path.join(process.env.DUMP, "root" + ext), rootText); outputs.forEach((o, i) => fs.writeFileSync(path.join(process.env.DUMP, `${i}-${path.basename(path.dirname(o.path))}${ext}`), o.text)); }
if (APPLY) {
  if (errors.length) { console.error("REFUSED: errors present"); process.exit(2); }
  for (const o of outputs) if (fs.existsSync(o.path)) { console.error("REFUSED: child exists " + o.path); process.exit(4); }
  if (fs.readFileSync(file, "utf8") !== text) { console.error("REFUSED: root changed during run"); process.exit(5); }
  for (const o of outputs) { fs.mkdirSync(path.dirname(o.path), { recursive: true }); fs.writeFileSync(o.path, o.text); }
  fs.writeFileSync(file, rootText);
  console.log("APPLIED");
}
