// usage: node bx.cjs <spec.json> [APPLY]
// Block extractor: moves chosen blocks of a (hook/closure) function into module-level functions in child folders.
// item kinds: "if"    = the then-block of an `if` statement starting at `line` that ends in `return ...;` or `continue;`
//             "range" = the statements from the one starting at `line` to the one ending at `to` (same block); declared names used afterwards come back as a destructured result
// spec: { file, scopeLine, coreStmtLine?, coreType?, types?: {name: "type"}, narrowKeep?: [names], files: [{ id, dir, doc, imports?: [text] }], items: [{ id, kind, line, to?, name, emoji, doc, file, retType? }] }
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
const { options } = ts.convertCompilerOptionsFromJson({ ...cfg.compilerOptions, noEmit: true, incremental: false, types: (process.env.TS_TYPES ? process.env.TS_TYPES.split(",") : []) }, REPO);
options.noEmit = true;
const host = ts.createCompilerHost(options);
const origGet = host.getSourceFile.bind(host);
host.getSourceFile = (fn, lv, ...r) => (path.resolve(fn) === path.resolve(file) ? ts.createSourceFile(fn, text, lv, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS) : origGet(fn, lv, ...r));
const program = ts.createProgram([file], options, host);
const sf = program.getSourceFile(file);
const checker = program.getTypeChecker();
const lines = text.split("\n");
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
const lineStartOff = [0];
for (let i = 0; i < text.length; i++) if (text[i] === "\n") lineStartOff.push(i + 1);
const offOfLine = (l) => lineStartOff[l - 1];

const scope = (() => {
  let found = null;
  const visit = (n) => {
    if (found) return;
    if (ts.isFunctionLike(n) && n.body && L(n.getStart(sf)) === spec.scopeLine) { found = n; return; }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  return found;
})();
if (!scope) throw new Error("scope function not found at line " + spec.scopeLine);
const inScope = (n) => n && n.getSourceFile() === sf && n.pos >= scope.pos && n.end <= scope.end;

const coreNames = new Set();
const coreSyms = new Map();
const coreStmts = spec.coreStmts || (spec.coreStmtLine ? [{ line: spec.coreStmtLine, type: spec.coreType }] : []);
const coreVia = new Map();
for (const cs of coreStmts) {
  if (cs.via) coreVia.set(cs.type, cs.via);
  const st = scope.body.statements.find((s) => L(s.getStart(sf)) === cs.line);
  if (!st || !ts.isVariableStatement(st)) throw new Error("core statement not found at " + cs.line);
  const d = st.declarationList.declarations[0];
  for (const el of d.name.elements) { coreNames.add(el.name.text); const s = checker.getSymbolAtLocation(el.name); if (s) coreSyms.set(s, cs.type); }
}

const isTypePos = (n) => { for (let p = n.parent; p; p = p.parent) { if (ts.isTypeNode(p) && !ts.isExpressionWithTypeArguments(p)) return true; if (ts.isStatement(p)) return false; } return false; };
const skipId = (n) => {
  const p = n.parent;
  return (ts.isPropertyAccessExpression(p) && p.name === n) || (ts.isPropertyAssignment(p) && p.name === n) || (ts.isQualifiedName(p) && p.right === n) || ((ts.isMethodDeclaration(p) || ts.isPropertyDeclaration(p) || ts.isPropertySignature(p) || ts.isMethodSignature(p) || ts.isGetAccessor(p) || ts.isSetAccessor(p)) && p.name === n) || (ts.isBindingElement(p) && p.propertyName === n) || (ts.isLabeledStatement(p) && p.label === n) || ((ts.isBreakOrContinueStatement(p)) && p.label === n) || (ts.isEnumMember(p) && p.name === n) || (ts.isJsxAttribute(p) && p.name === n);
};
const symOf = (n) => {
  const p = n.parent;
  if (ts.isShorthandPropertyAssignment(p) && p.name === n) return checker.getShorthandAssignmentValueSymbol(p) || checker.getSymbolAtLocation(n);
  return checker.getSymbolAtLocation(n);
};
const declOfSym = (sym) => (sym && (sym.valueDeclaration || (sym.declarations && sym.declarations[0])));
const isWrite = (n) => {
  const p = n.parent;
  if (ts.isBinaryExpression(p) && p.left === n && p.operatorToken.kind >= ts.SyntaxKind.FirstAssignment && p.operatorToken.kind <= ts.SyntaxKind.LastAssignment) return true;
  if ((ts.isPrefixUnaryExpression(p) || ts.isPostfixUnaryExpression(p)) && (p.operator === ts.SyntaxKind.PlusPlusToken || p.operator === ts.SyntaxKind.MinusMinusToken)) return true;
  if (ts.isArrayLiteralExpression(p) || ts.isShorthandPropertyAssignment(p)) { for (let q = p; q.parent; q = q.parent) { if (ts.isBinaryExpression(q.parent) && q.parent.left === q && q.parent.operatorToken.kind === ts.SyntaxKind.EqualsToken) return true; if (!(ts.isArrayLiteralExpression(q.parent) || ts.isObjectLiteralExpression(q.parent) || ts.isPropertyAssignment(q.parent) || ts.isShorthandPropertyAssignment(q.parent))) break; } }
  return false;
};
const containsAwait = (nodes) => { let r = false; const w = (n) => { if (ts.isAwaitExpression(n) || (ts.isForOfStatement(n) && n.awaitModifier)) r = true; if (!ts.isFunctionLike(n)) ts.forEachChild(n, w); }; nodes.forEach(w); return r; };

const importStmts = sf.statements.filter(ts.isImportDeclaration);
const identsOf = (code) => {
  const f = ts.createSourceFile("x" + path.extname(file), code, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const used = new Set();
  const visit = (n) => {
    if (ts.isImportDeclaration(n)) return;
    if (ts.isIdentifier(n) && !skipIdIn(n)) used.add(n.text);
    ts.forEachChild(n, visit);
  };
  visit(f);
  return used;
};
const skipIdIn = (n) => {
  const p = n.parent;
  return p && ((ts.isPropertyAccessExpression(p) && p.name === n) || (ts.isPropertyAssignment(p) && p.name === n) || (ts.isQualifiedName(p) && p.right === n) || ((ts.isMethodDeclaration(p) || ts.isPropertyDeclaration(p) || ts.isPropertySignature(p) || ts.isMethodSignature(p)) && p.name === n));
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
const importedNames = new Set();
for (const n of importStmts) { const c = n.importClause; if (!c) continue; if (c.name) importedNames.add(c.name.text); if (c.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) importedNames.add(c.namedBindings.name.text); else c.namedBindings.elements.forEach((e) => importedNames.add(e.name.text)); } }

const typeFlags = ts.TypeFormatFlags.NoTruncation | ts.TypeFormatFlags.UseAliasDefinedOutsideCurrentScope;
const itemResults = [];

for (const item of spec.items) {
  const r = { item, errors: [] };
  itemResults.push(r);
  let nodes = [];
  let thenBlock = null;
  let range = null;
  if (item.kind === "if") {
    let ifs = null;
    const visit = (n) => { if (ifs) return; if (ts.isIfStatement(n) && L(n.getStart(sf)) === item.line && inScope(n)) { ifs = n; return; } ts.forEachChild(n, visit); };
    visit(scope.body);
    if (!ifs) { errors.push(`${item.id}: no if at line ${item.line}`); continue; }
    if (!ts.isBlock(ifs.thenStatement)) { errors.push(`${item.id}: then is not a block`); continue; }
    thenBlock = ifs.thenStatement;
    nodes = [...thenBlock.statements];
    const last = nodes[nodes.length - 1];
    if (!last || !(ts.isReturnStatement(last) || ts.isContinueStatement(last))) { errors.push(`${item.id}: then-block does not end in return/continue`); continue; }
    r.terminal = ts.isReturnStatement(last) ? "return" : "continue";
    r.ifStmt = ifs;
  } else {
    let first = null, lastStmt = null, parentBlock = null;
    const visit = (n) => {
      if (first) return;
      const list = ts.isBlock(n) || ts.isSourceFile(n) || ts.isCaseClause(n) ? n.statements : null;
      if (list) {
        const a = list.findIndex((s) => L(s.getStart(sf)) === item.line);
        if (a >= 0) {
          const b = list.findIndex((s, i) => i >= a && L(s.getEnd()) === item.to);
          if (b >= 0) { first = list[a]; lastStmt = list[b]; parentBlock = list; nodes = list.slice(a, b + 1); return; }
        }
      }
      ts.forEachChild(n, visit);
    };
    visit(scope.body);
    if (!first) { errors.push(`${item.id}: no statement range ${item.line}-${item.to}`); continue; }
    range = { first, lastStmt, parentBlock };
    r.range = range;
  }
  r.nodes = nodes;
  const lead = (ts.getLeadingCommentRanges(text, nodes[0].getFullStart()) || []).filter((c) => text.slice(offOfLine(L(c.pos)), c.pos).trim() === "");
  const rangeStart = nodes[0].getStart(sf), rangeEnd = nodes[nodes.length - 1].getEnd();
  r.leadStart = lead && lead.length ? lead[0].pos : rangeStart;
  r.rangeStart = rangeStart; r.rangeEnd = rangeEnd;
  const free = new Map();
  const writes = new Set();
  const localTypes = new Set();
  const hasReturnInRange = [];
  const interiorContinues = [];
  const exitReturns = [];
  const walk = (n, loopDepth, fnDepth) => {
    if (ts.isIdentifier(n) && !skipId(n)) {
      const sym = symOf(n);
      const d = declOfSym(sym);
      if (sym && d && inScope(d) && !(d.end <= rangeEnd && d.getStart(sf) >= rangeStart)) {
        const tp = isTypePos(n) && !(sym.flags & ts.SymbolFlags.Value);
        if (tp) localTypes.add(sym.name);
        else {
          if (!free.has(sym)) free.set(sym, []);
          free.get(sym).push(n);
          if (isWrite(n)) writes.add(sym);
        }
      }
    }
    if (ts.isReturnStatement(n) && fnDepth === 0 && n !== nodes[nodes.length - 1]) { hasReturnInRange.push(L(n.getStart(sf))); exitReturns.push(n); }
    if (ts.isContinueStatement(n) && fnDepth === 0 && loopDepth === 0 && n !== nodes[nodes.length - 1]) interiorContinues.push(n);
    const isLoop = ts.isForStatement(n) || ts.isForOfStatement(n) || ts.isForInStatement(n) || ts.isWhileStatement(n) || ts.isDoStatement(n);
    ts.forEachChild(n, (c) => walk(c, loopDepth + (isLoop ? 1 : 0), fnDepth + (ts.isFunctionLike(n) ? 1 : 0)));
  };
  nodes.forEach((n) => walk(n, 0, 0));
  r.free = free; r.writes = writes; r.localTypes = localTypes; r.interiorContinues = interiorContinues;
  r.rangeReturns = item.kind === "range" && item.returns ? exitReturns : [];
  if (item.kind === "range" && hasReturnInRange.length && !item.returns) r.errors.push(`return inside range at ${hasReturnInRange}`);

  if (item.kind === "if" && r.terminal === "continue" && exitReturns.some((n) => n.expression)) r.errors.push(`function-level return with a value inside a continue block at ${hasReturnInRange}`);
  r.exitReturns = item.kind === "if" && r.terminal === "continue" ? exitReturns : [];
  if (item.kind === "if" && r.terminal === "return") { /* interior returns are fine: they return from the handler and the root returns that value */ }
  if (localTypes.size) r.errors.push(`local types used: ${[...localTypes].join(", ")}`);
  const thisUse = []; const wt = (n) => { if (n.kind === ts.SyntaxKind.ThisKeyword || (ts.isIdentifier(n) && n.text === "arguments")) thisUse.push(1); if (!ts.isFunctionDeclaration(n) && !ts.isFunctionExpression(n) && !ts.isClassLike(n)) ts.forEachChild(n, wt); }; nodes.forEach(wt);
  if (thisUse.length) r.errors.push("this/arguments used");
  r.isAsync = containsAwait(nodes);
  if (item.kind === "range") {
    const declared = [];
    for (const n of nodes) {
      if (ts.isVariableStatement(n)) for (const d of n.declarationList.declarations) { const w = (b) => { if (ts.isIdentifier(b)) declared.push({ sym: checker.getSymbolAtLocation(b), kind: n.declarationList.flags & ts.NodeFlags.Const ? "const" : "let" }); else b.elements.forEach((e) => !ts.isOmittedExpression(e) && w(e.name)); }; w(d.name); }
      else if ((ts.isFunctionDeclaration(n) || ts.isClassDeclaration(n)) && n.name) declared.push({ sym: checker.getSymbolAtLocation(n.name), kind: "const" });
    }
    const usedAfter = [];
    const wa = (n) => {
      if (ts.isIdentifier(n) && !skipId(n)) {
        const sym = symOf(n);
        const hit = declared.find((d) => d.sym === sym);
        if (hit && (n.getStart(sf) < rangeStart || n.getStart(sf) >= rangeEnd) && !usedAfter.includes(hit)) usedAfter.push(hit);
      }
      ts.forEachChild(n, wa);
    };
    wa(scope.body);
    r.produces = usedAfter;
    if (item.returns && usedAfter.length) r.errors.push("a range with returns cannot also produce values");
    if (item.returns && !item.terminal && exitReturns.some((n) => !n.expression)) r.errors.push("bare return inside a range with returns");
    if (usedAfter.some((u) => u.kind === "let")) {
      const syms = usedAfter.filter((u) => u.kind === "let").map((u) => u.sym.name);
      warns.push(`${item.id}: produces let ${syms.join(", ")} (root re-declares as let)`);
    }
    for (const w of [...writes]) if ((spec.paramWrites || []).includes(w.name)) writes.delete(w);
    if (writes.size) r.errors.push(`writes free variable(s) ${[...writes].map((s) => s.name).join(", ")}`);
  } else if (writes.size) {
    if (r.terminal === "continue" && writes.size === 1) { r.writeBack = [...writes][0]; }
    else r.errors.push(`writes free variable(s) ${[...writes].map((s) => s.name).join(", ")}`);
  }
  r.coreUsed = []; r.localUsed = [];
  for (const [sym, ids] of free) {
    const d = declOfSym(sym);
    const isParam = ts.isParameter(d);
    const fromCore = coreSyms.has(sym);
    (fromCore ? r.coreUsed : r.localUsed).push({ sym, ids, isParam, coreType: coreSyms.get(sym) });
  }
}

const nameSet = new Set();
for (const r of itemResults) { if (nameSet.has(r.item.name)) errors.push(`duplicate name ${r.item.name}`); nameSet.add(r.item.name); }
for (const r of itemResults) if (r.errors.length) errors.push(...r.errors.map((e) => `${r.item.id}: ${e}`));

const coreType = spec.coreType;
const guardKeys = (cond, name) => {
  const keys = [];
  const visit = (c) => {
    if (ts.isParenthesizedExpression(c)) return visit(c.expression);
    if (ts.isBinaryExpression(c) && c.operatorToken.kind === ts.SyntaxKind.BarBarToken) { const a = visit(c.left), b = visit(c.right); return a && b; }
    if (ts.isBinaryExpression(c) && c.operatorToken.kind === ts.SyntaxKind.AmpersandAmpersandToken) { const a = visit(c.left), b = visit(c.right); return a || b; }
    if (ts.isBinaryExpression(c) && c.operatorToken.kind === ts.SyntaxKind.InKeyword && ts.isStringLiteral(c.left) && ts.isIdentifier(c.right) && c.right.text === name) { keys.push({ kind: "in", key: c.left.text }); return true; }
    if (ts.isBinaryExpression(c) && c.operatorToken.kind === ts.SyntaxKind.EqualsEqualsEqualsToken && ts.isPropertyAccessExpression(c.left) && ts.isIdentifier(c.left.expression) && c.left.expression.text === name && ts.isStringLiteral(c.right)) { keys.push({ kind: "eq", prop: c.left.name.text, value: c.right.text }); return true; }
    return false;
  };
  return visit(cond) ? keys : null;
};
const typeOfSym = (r, sym, ids) => {
  if (spec.types && spec.types[sym.name]) return spec.types[sym.name];
  if (r.item.types && r.item.types[sym.name]) return r.item.types[sym.name];
  if (r.ifStmt) {
    const keys = guardKeys(r.ifStmt.expression, sym.name);
    if (keys && keys.length) {
      const declT = checker.typeToString(checker.getTypeOfSymbolAtLocation(sym, declOfSym(sym)), scope, typeFlags);
      if (/^[A-Za-z_][A-Za-z0-9_]*$/.test(declT)) return keys.map((k) => `Extract<${declT}, Readonly<${k.kind === "in" ? `Record<"${k.key}", unknown>` : `{ ${k.prop}: "${k.value}" }`}>>`).join(" | ");
    }
  }
  const texts = [...new Set(ids.map((id) => checker.typeToString(checker.getTypeAtLocation(id), scope, typeFlags)))];
  let t = texts[0];
  if (texts.length > 1) {
    const d = declOfSym(sym);
    const dt = checker.typeToString(checker.getTypeOfSymbolAtLocation(sym, d), scope, typeFlags);
    warns.push(`${r.item.id}: ${sym.name} has ${texts.length} narrowings (${texts.map((x) => x.slice(0, 40)).join(" | ")}); using declared ${dt.slice(0, 60)}`);
    t = dt;
  }
  if (/import\(/.test(t)) errors.push(`${r.item.id}: type of ${sym.name} contains import(): ${t.slice(0, 100)}`);
  r.pendingChecks = r.pendingChecks || new Map();
  r.pendingChecks.set(sym.name, t);
  if (/^any$/.test(t) && !spec.allowAny) errors.push(`${r.item.id}: type of ${sym.name} is any`);
  return t;
};

const out = [];
const checkText = (r, name, t) => {
  const stripped = t.replace(/import\("[^"]*"\)\.[A-Za-z0-9_]+/g, "").replace(/[A-Za-z0-9_]+\.[A-Z][A-Za-z0-9_]*/g, "");
  for (const nm of new Set(stripped.match(/\b[A-Z][A-Za-z0-9_]*\b/g) || [])) {
    if (spec.typeImports && spec.typeImports[nm]) continue;
    let ok = true;
    try { ok = !!checker.resolveName(nm, scope.body, ts.SymbolFlags.Type | ts.SymbolFlags.Value | ts.SymbolFlags.Namespace, false); } catch { ok = true; }
    if (!ok) errors.push(`${r.item.id}: type of ${name} names ${nm} which is not in scope (${t.slice(0, 90)})`);
  }
};
const bodyDedent = (nodesLines, base) => nodesLines.map((l) => (l.startsWith(base) ? l.slice(base.length) : l));
const fnTexts = new Map();
const rootEdits = [];
for (const r of itemResults) {
  if (r.errors.length || !r.nodes) continue;
  const { item, nodes } = r;
  const firstLine = L(r.leadStart);
  let lastLine = L(nodes[nodes.length - 1].getEnd());
  const startOff = offOfLine(firstLine);
  const endOff = nodes[nodes.length - 1].getEnd();
  let bodyText = text.slice(startOff, endOff);
  const baseIndent = /^\s*/.exec(lines[firstLine - 1])[0];
  const multiLine = (() => { let bad = false; const w = (n) => { if ((ts.isTemplateExpression(n) || ts.isNoSubstitutionTemplateLiteral(n)) && L(n.getStart(sf)) !== L(n.getEnd())) bad = true; ts.forEachChild(n, w); }; nodes.forEach(w); return bad; })();
  if (multiLine) { errors.push(`${item.id}: contains a multi-line template literal`); continue; }
  let bodyLines = bodyText.split("\n");
  if (r.terminal === "continue") {
    const lastNode = nodes[nodes.length - 1];
    const cl = L(lastNode.getStart(sf));
    const idx = cl - firstLine;
    bodyLines = bodyLines.slice(0, idx);
    while (bodyLines.length && !bodyLines[bodyLines.length - 1].trim()) bodyLines.pop();
    if (!bodyLines.length) bodyLines = [baseIndent + "// empty"];
  }
  for (const c of r.interiorContinues) {
    const li = L(c.getStart(sf)) - firstLine;
    if (li < bodyLines.length) bodyLines[li] = bodyLines[li].replace(/\bcontinue;/, "return;");
  }
  for (const n of r.exitReturns) {
    const li = L(n.getStart(sf)) - firstLine;
    const col = n.getStart(sf) - offOfLine(L(n.getStart(sf)));
    const line = bodyLines[li];
    if (line.slice(col, col + 7) !== "return;") { errors.push(`${item.id}: cannot rewrite exit return at line ${li + firstLine}`); continue; }
    bodyLines[li] = line.slice(0, col) + (r.writeBack ? "return null;" : 'return "stop";') + line.slice(col + 7);
  }
  bodyLines = bodyDedent(bodyLines, baseIndent).map((l) => (l ? "  " + l : l));
  const coreNamesUsed = r.coreUsed.map((u) => u.sym.name).filter((n) => n);
  const parts = [];
  const narrowedCore = [];
  const pickNames = [];
  const pickByType = new Map();
  for (const u of r.coreUsed) {
    const decl = checker.typeToString(checker.getTypeOfSymbolAtLocation(u.sym, declOfSym(u.sym)), scope, typeFlags);
    const t = typeOfSym(r, u.sym, u.ids);
    const override = (spec.types && spec.types[u.sym.name]) || (item.types && item.types[u.sym.name]);
    if (!override && t !== decl && !(spec.narrowKeep || []).includes(u.sym.name)) { narrowedCore.push({ name: u.sym.name, type: t }); } else { if (!coreVia.has(u.coreType)) pickNames.push(u.sym.name); if (!pickByType.has(u.coreType)) pickByType.set(u.coreType, []); pickByType.get(u.coreType).push(u.sym.name); }
  }
  const viaObjects = [];
  for (const [ct, names] of pickByType) {
    if (coreVia.has(ct)) { viaObjects.push({ via: coreVia.get(ct), type: ct, names }); continue; }
    parts.push(`Pick<${ct}, ${names.map((n) => `"${n}"`).join(" | ")}>`);
  }
  r.viaObjects = viaObjects;
  const members = [...viaObjects.map((v) => ({ name: v.via, type: v.type })), ...narrowedCore];
  for (const u of r.localUsed) members.push({ name: u.sym.name, type: typeOfSym(r, u.sym, u.ids) });
  if (members.length) parts.push(`{\n${members.map((m) => `  readonly ${m.name}: ${m.type};`).join("\n")}\n}`);
  for (const m of members) checkText(r, m.name, m.type);
  const allNames = [...pickNames, ...members.map((m) => m.name)];
  const params = allNames.length ? `{ ${allNames.join(", ")} }: ${parts.join(" & ")}` : "";
  const argsText = allNames.length ? `{ ${allNames.join(", ")} }` : "";
  let ret = "";
  let retLines = [];
  if (r.range) {
    const prod = r.produces.map((p) => p.sym.name);
    if (prod.length) retLines = [`  return { ${prod.join(", ")} };`];
    r.prodNames = prod;
  }
  if (r.writeBack) {
    const wn = r.writeBack.name;
    retLines = [`  return ${wn};`];
  }
  const isAsync = r.isAsync;
  const emoji = item.emoji + VS;
  const viaPrologue = r.viaObjects.map((v) => `  const { ${v.names.join(", ")} } = ${v.via};`);
  const fnText = `/** ${emoji} ${item.doc} */\nexport ${isAsync ? "async " : ""}function ${item.name}(${params}) {\n${viaPrologue.length ? viaPrologue.join("\n") + "\n" : ""}${bodyLines.join("\n")}${retLines.length ? "\n" + retLines.join("\n") : ""}\n}`;
  fnTexts.set(item.id, fnText);
  const call = `${isAsync ? "await " : ""}${item.name}(${argsText})`;
  const ind = baseIndent;
  let repl;
  if (item.kind === "if") {
    if (r.terminal === "return") repl = `${ind}return ${call};`;
    else if (r.writeBack && r.exitReturns.length) repl = `${ind}const ${r.writeBack.name}Next = ${call};\n${ind}if (${r.writeBack.name}Next === null) return;\n${ind}${r.writeBack.name} = ${r.writeBack.name}Next;\n${ind}continue;`;
    else if (r.writeBack) repl = `${ind}${r.writeBack.name} = ${call};\n${ind}continue;`;
    else if (r.exitReturns.length) repl = `${ind}if (${call} === "stop") return;\n${ind}continue;`;
    else repl = `${ind}${call};\n${ind}continue;`;
  } else if (item.returns) {
    repl = item.terminal ? `${ind}return ${call};` : `${ind}const ${item.id}Outcome = ${call};\n${ind}if (${item.id}Outcome !== undefined) return ${item.id}Outcome;`;
  } else if (r.prodNames && r.prodNames.length) {
    const kw = r.produces.some((p) => p.kind === "let") ? "let" : "const";
    repl = `${ind}${kw} { ${r.prodNames.join(", ")} } = ${call};`;
  } else repl = `${ind}${call};`;
  rootEdits.push({ start: startOff, end: endOff, text: repl, id: item.id });
  r.argsLen = argsText.length;
}

const fileSpecs = spec.files;
const outFiles = [];
const stripV = (e) => e.replace(/️/g, "");
const allEmoji = new Map();
for (const f of fileSpecs) { const e = stripV(f.dir.split(VS)[0]); if (allEmoji.has(e)) errors.push(`folder emoji ${e} repeated`); allEmoji.set(e, "folder " + f.dir); }
for (const r of itemResults) { const e = stripV(r.item.emoji); if (allEmoji.has(e)) errors.push(`emoji ${e} repeated (${allEmoji.get(e)}, ${r.item.name})`); allEmoji.set(e, r.item.name); }
for (const f of fileSpecs) {
  const items = itemResults.filter((r) => r.item.file === f.id && fnTexts.has(r.item.id));
  const outPath = path.join(path.dirname(file), f.dir, "🟦️" + path.extname(file));
  const body = items.map((r) => fnTexts.get(r.item.id)).join("\n\n");
  const used = identsOf(body);
  const imps = prunedImports(used, rebase(outPath));
  if (f.imports) imps.push(...f.imports);
  const byModule = new Map();
  for (const [nm, mod] of Object.entries(spec.typeImports || {})) if (used.has(nm)) { if (!byModule.has(mod)) byModule.set(mod, []); byModule.get(mod).push(nm); }
  for (const [mod, names] of byModule) imps.push(`import type { ${names.sort().join(", ")} } from "${mod}";`);
  const topUsed = [];
  for (const st of sf.statements) {
    if (ts.isImportDeclaration(st) || st === scope || (st.pos <= scope.pos && st.end >= scope.end)) continue;
    const names = [];
    if (ts.isVariableStatement(st)) st.declarationList.declarations.forEach((d) => ts.isIdentifier(d.name) && names.push(d.name.text));
    else if ((ts.isFunctionDeclaration(st) || ts.isTypeAliasDeclaration(st) || ts.isInterfaceDeclaration(st) || ts.isClassDeclaration(st) || ts.isEnumDeclaration(st)) && st.name) names.push(st.name.text);
    for (const n of names) if (used.has(n)) topUsed.push(n);
  }
  if (topUsed.length) errors.push(`file ${f.id}: uses root module-level declarations ${topUsed.join(", ")}`);
  const doc = `/** ${f.doc.replace(/^\/\*\*\s*/, "").replace(/\s*\*\/$/, "")} */`;
  outFiles.push({ path: outPath, text: `${doc}\n${imps.join("\n")}${imps.length ? "\n\n" : ""}${body}\n`, items, f });
}

let newText = text;
for (const e of [...rootEdits].sort((a, b) => b.start - a.start)) newText = newText.slice(0, e.start) + e.text + newText.slice(e.end);
const usedMovedNames = new Set(outFiles.flatMap((o) => o.items.map((r) => r.item.name)));
{
  const rootBodyUsed = identsOf(newText.split("\n").filter((l) => !/^import\b/.test(l)).join("\n"));
  const lastImport = importStmts[importStmts.length - 1];
  const insert = outFiles.map((o) => `import { ${o.items.map((r) => r.item.name).filter((n) => rootBodyUsed.has(n)).sort().join(", ")} } from "${relTo(file, o.path)}";`).join("\n");
  const movedIdents = new Set(outFiles.flatMap((o) => [...identsOf(o.text)]));
  const unusedAfter = [];
  for (const n of importStmts) {
    const c = n.importClause;
    if (!c) continue;
    const keep = (name) => rootBodyUsed.has(name) || !movedIdents.has(name);
    if (c.name && !keep(c.name.text)) unusedAfter.push(c.name.text);
    if (c.namedBindings && ts.isNamedImports(c.namedBindings)) c.namedBindings.elements.forEach((el) => { if (!keep(el.name.text)) unusedAfter.push(el.name.text); });
    if (c.namedBindings && ts.isNamespaceImport(c.namedBindings) && !keep(c.namedBindings.name.text)) unusedAfter.push(c.namedBindings.name.text);
  }
  const edits2 = [];
  for (const n of importStmts) {
    const c = n.importClause;
    if (!c) continue;
    const dropName = (name) => unusedAfter.includes(name);
    const def = c.name && !dropName(c.name.text) ? c.name.text : null;
    let ns = null; const named = [];
    if (c.namedBindings && ts.isNamespaceImport(c.namedBindings) && !dropName(c.namedBindings.name.text)) ns = c.namedBindings.name.text;
    if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const el of c.namedBindings.elements) if (!dropName(el.name.text)) named.push(el.getText(sf));
    const orig = [c.name, ...(c.namedBindings ? (ts.isNamedImports(c.namedBindings) ? c.namedBindings.elements : [c.namedBindings]) : [])].filter(Boolean).length;
    const now = (def ? 1 : 0) + (ns ? 1 : 0) + named.length;
    if (now === orig) continue;
    const q = n.moduleSpecifier.getText(sf)[0];
    let t = "";
    if (now) {
      const bits = [];
      if (def) bits.push(def);
      if (ns) bits.push(`* as ${ns}`);
      if (named.length) bits.push(`{ ${named.join(", ")} }`);
      t = `import ${c.isTypeOnly ? "type " : ""}${bits.join(", ")} from ${q}${n.moduleSpecifier.text}${q};`;
    }
    edits2.push({ start: n.getStart(sf), end: n.getEnd(), text: t });
  }
  let t2 = newText;
  const shift = (pos) => pos;
  edits2.push({ start: lastImport.getEnd(), end: lastImport.getEnd(), text: "\n" + insert });
  for (const e of edits2.sort((a, b) => b.start - a.start)) t2 = t2.slice(0, e.start) + e.text + t2.slice(e.end);
  newText = t2;
}

const lc = (t) => t.split("\n").length;
const scopeLen = (() => { const sfNew = ts.createSourceFile("n" + path.extname(file), newText, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS); let len = 0; const v = (n) => { if (ts.isFunctionLike(n) && n.body && n.name && scope.name && n.name.text === scope.name.text) { len = sfNew.getLineAndCharacterOfPosition(n.getEnd()).line - sfNew.getLineAndCharacterOfPosition(n.getStart(sfNew)).line + 1; return; } ts.forEachChild(n, v); }; v(sfNew); return len; })();
const report = { rootLines: lc(newText), scopeLines: scopeLen, files: outFiles.map((o) => ({ file: path.relative(path.dirname(file), o.path), lines: lc(o.text), fns: o.items.map((r) => `${r.item.name}:${fnTexts.get(r.item.id).split("\n").length}`) })), errors, warns };
console.log(JSON.stringify(report, null, 1));
if (process.env.DUMP) { fs.mkdirSync(process.env.DUMP, { recursive: true }); fs.writeFileSync(path.join(process.env.DUMP, "root" + path.extname(file)), newText); outFiles.forEach((o, i) => fs.writeFileSync(path.join(process.env.DUMP, `${i}-${path.basename(path.dirname(o.path))}${path.extname(file)}`), o.text)); }
if (APPLY) {
  if (errors.length) { console.error("REFUSED: errors present"); process.exit(2); }
  for (const o of outFiles) if (fs.existsSync(o.path)) { console.error("REFUSED: child exists " + o.path); process.exit(4); }
  if (fs.readFileSync(file, "utf8") !== text) { console.error("REFUSED: root changed during run"); process.exit(5); }
  for (const o of outFiles) { fs.mkdirSync(path.dirname(o.path), { recursive: true }); fs.writeFileSync(o.path, o.text); }
  fs.writeFileSync(file, newText);
  console.log("APPLIED");
}
