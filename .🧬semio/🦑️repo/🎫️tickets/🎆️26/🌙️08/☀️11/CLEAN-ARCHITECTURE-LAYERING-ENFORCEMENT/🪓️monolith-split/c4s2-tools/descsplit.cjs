// usage: node descsplit.cjs <spec.json> [APPLY]
// Splits a `registerTests1(vitest, dependencies, source)` suite: the `describe` blocks of the function body move into child
// `registerXTests` functions (one folder per group), declarations used by several groups move into a kit module, the parent
// keeps an orchestrator that calls the children in the original order.
// spec: { file, fn, kit?: { dir, doc, fn, moveTop: [names of top-level declarations that move into the kit] },
//         groups: [{ dir, fn, doc, lines: [[a,b]...] }], depsType?: "text of the dependencies parameter type in the children" }
// A body statement belongs to the group whose line range contains its first token line. Anything else is a declaration.
const fs = require("fs");
const path = require("path");
const ts = require("/Users/ueli/Documents/semio/node_modules/typescript");

const spec = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const APPLY = process.argv[3] === "APPLY";
const file = spec.file;
const dir = path.dirname(file);
const text = fs.readFileSync(process.env.ORIG_TEXT || file, "utf8");
const isTsx = file.endsWith(".tsx");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, isTsx ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const host = ts.createCompilerHost({});
host.getSourceFile = (n) => (n === file ? sf : undefined);
host.fileExists = (n) => n === file;
host.readFile = (n) => (n === file ? text : undefined);
const program = ts.createProgram([file], { noResolve: true, noLib: true, target: ts.ScriptTarget.Latest, jsx: ts.JsxEmit.Preserve }, host);
const checker = program.getTypeChecker();
const lineOf = (pos) => sf.getLineAndCharacterOfPosition(pos).line + 1;
const errors = [];
const warn = [];
const regionRe = /^[ \t]*\/\/\s*#(end)?region\s+(.*?)\s*$/;

const top = sf.statements;
const fnNode = top.find((s) => ts.isFunctionDeclaration(s) && s.name && s.name.text === spec.fn);
if (!fnNode) throw new Error("function not found: " + spec.fn);
const body = fnNode.body.statements;
const fnIndexInTop = top.indexOf(fnNode);

// ---- body statements
const kids = spec.groups;
const bstmts = body.map((s, i) => {
  let names = [];
  if (ts.isVariableStatement(s)) names = s.declarationList.declarations.flatMap((d) => (ts.isIdentifier(d.name) ? [d.name.text] : d.name.elements.map((e) => e.name.getText(sf))));
  else if (ts.isFunctionDeclaration(s) || ts.isClassDeclaration(s) || ts.isTypeAliasDeclaration(s) || ts.isInterfaceDeclaration(s)) names = s.name ? [s.name.text] : [];
  const startLine = lineOf(s.getStart(sf, false));
  let group = -1;
  kids.forEach((k, c) => { if (group < 0 && k.lines.some(([a, b]) => startLine >= a && startLine <= b)) group = c; });
  return { i, node: s, names, startLine, group, isType: ts.isTypeAliasDeclaration(s) || ts.isInterfaceDeclaration(s) };
});
const groupIdx = bstmts.filter((b) => b.group >= 0).map((b) => b.group);
for (let k = 1; k < groupIdx.length; k++) if (groupIdx[k] < groupIdx[k - 1]) errors.push("groups are not contiguous and ascending near statement " + bstmts.filter((b) => b.group >= 0)[k].startLine);
for (const b of bstmts) if (b.group < 0 && ts.isExpressionStatement(b.node) && /^\s*(describe|it|test)\b/.test(b.node.getText(sf))) errors.push(`unassigned block at line ${b.startLine}`);

// ---- symbols
const declOwner = new Map(); // symbol -> { kind: "body", i, name } | { kind: "import", stmt }
const topOwner = new Map();
bstmts.forEach((b) => {
  const s = b.node;
  const add = (id) => { const sy = checker.getSymbolAtLocation(id); if (sy) declOwner.set(sy, { kind: "body", i: b.i, name: id.text }); };
  if (ts.isVariableStatement(s)) s.declarationList.declarations.forEach((d) => { if (ts.isIdentifier(d.name)) add(d.name); else d.name.elements.forEach((e) => add(e.name)); });
  else if (s.name && ts.isIdentifier(s.name)) add(s.name);
});
top.forEach((s, ti) => {
  if (ts.isImportDeclaration(s) && s.importClause) {
    const c = s.importClause;
    const add = (id) => { const sy = checker.getSymbolAtLocation(id); if (sy) topOwner.set(sy, { kind: "import", ti, name: id.text }); };
    if (c.name) add(c.name);
    if (c.namedBindings && ts.isNamespaceImport(c.namedBindings)) add(c.namedBindings.name);
    if (c.namedBindings && ts.isNamedImports(c.namedBindings)) c.namedBindings.elements.forEach((e) => add(e.name));
  } else if (ti !== fnIndexInTop) {
    const add = (id) => { const sy = checker.getSymbolAtLocation(id); if (sy) topOwner.set(sy, { kind: "top", ti, name: id.text }); };
    if (ts.isVariableStatement(s)) s.declarationList.declarations.forEach((d) => { if (ts.isIdentifier(d.name)) add(d.name); });
    else if ((ts.isFunctionDeclaration(s) || ts.isTypeAliasDeclaration(s) || ts.isInterfaceDeclaration(s) || ts.isClassDeclaration(s)) && s.name) add(s.name);
  }
});
const fnParams = fnNode.parameters.map((p) => ({ name: p.name.getText(sf), sym: checker.getSymbolAtLocation(p.name), type: p.type ? p.type.getText(sf) : "any" }));
const paramBySym = new Map(fnParams.map((p) => [p.sym, p.name]));

// references of a node: body stmts, top-level owners, params
function refsOf(node) {
  const body = new Set(), imports = new Set(), tops = new Set(), params = new Set(), importBindings = new Set();
  const visit = (n) => {
    if (ts.isIdentifier(n)) {
      let sy = checker.getSymbolAtLocation(n);
      if (n.parent && ts.isShorthandPropertyAssignment(n.parent) && n.parent.name === n) sy = checker.getShorthandAssignmentValueSymbol(n.parent) || sy;
      if (sy) {
        if (declOwner.has(sy)) body.add(declOwner.get(sy).i);
        else if (topOwner.has(sy)) { const o = topOwner.get(sy); if (o.kind === "import") { imports.add(o.ti); importBindings.add(sy); } else tops.add(o.ti); }
        else if (paramBySym.has(sy)) params.add(paramBySym.get(sy));
      }
    }
    ts.forEachChild(n, visit);
  };
  visit(node);
  return { body, imports, tops, params, importBindings };
}
const stmtRefs = bstmts.map((b) => refsOf(b.node));

// destructuring bindings of `dependencies` / `vitest`
const bindingSource = new Map(); // body stmt index -> param name
bstmts.forEach((b) => {
  const s = b.node;
  if (ts.isVariableStatement(s) && s.declarationList.declarations.length === 1) {
    const d = s.declarationList.declarations[0];
    if (ts.isObjectBindingPattern(d.name) && d.initializer && ts.isIdentifier(d.initializer) && fnParams.some((p) => p.name === d.initializer.text)) bindingSource.set(b.i, d.initializer.text);
  }
});

// ---- which groups use which declaration (direct), then closure
const decls = bstmts.filter((b) => b.group < 0);
const usedBy = new Map(decls.map((d) => [d.i, new Set()]));
const depsOf = (i) => [...stmtRefs[i].body];
bstmts.forEach((b) => {
  if (b.group < 0) return;
  for (const j of stmtRefs[b.i].body) if (usedBy.has(j)) usedBy.get(j).add(b.group);
});
// transitive: a declaration used by a decl that is used by groups inherits those groups
let changed = true;
while (changed) {
  changed = false;
  for (const d of decls) for (const j of depsOf(d.i)) if (usedBy.has(j) && j !== d.i) for (const g of usedBy.get(d.i)) if (!usedBy.get(j).has(g)) { usedBy.get(j).add(g); changed = true; }
}
const shared = new Set(decls.filter((d) => usedBy.get(d.i).size >= 2 && !bindingSource.has(d.i) && !(spec.copySharedTypes && d.isType)).map((d) => d.i));
// everything a shared declaration needs (other than bindings) is shared too (already true through the closure)
const sharedList = decls.filter((d) => shared.has(d.i));
const sharedValues = sharedList.filter((d) => !d.isType);
const sharedTypes = sharedList.filter((d) => d.isType);
for (const d of sharedValues) if (ts.isVariableStatement(d.node) && (d.node.declarationList.flags & ts.NodeFlags.Let)) errors.push(`shared mutable let at line ${d.startLine}`);
const unusedDecls = decls.filter((d) => !bindingSource.has(d.i) && usedBy.get(d.i).size === 0);
for (const d of unusedDecls) warn.push(`declaration used by no group: ${d.names.join(",") || d.node.getText(sf).slice(0, 40)} (line ${d.startLine}) stays in the orchestrator`);

// ---- region trivia of body statements
const lines = text.split("\n");
const regionTexts = new Map();
const stackAt = new Array(body.length);
const trivia = new Array(body.length);
{
  let stack = [];
  let prevEnd = body.length ? fnNode.body.getStart(sf) + 1 : 0;
  const evalTrivia = (from, to) => {
    const seg = text.slice(from, to);
    const out = [];
    const segLines = seg.split("\n");
    const keepIdx = new Set();
    { const st2 = []; segLines.forEach((l, idx) => { const m = l.match(regionRe); if (!m) return; if (!m[1]) st2.push({ name: m[2], idx }); else if (st2.length && st2[st2.length - 1].name === m[2]) { keepIdx.add(st2.pop().idx); keepIdx.add(idx); } }); }
    for (const [lineIdx, l] of segLines.entries()) {
      if (keepIdx.has(lineIdx)) { out.push(l); continue; }
      const m = l.match(regionRe);
      if (m) {
        if (!regionTexts.has(m[2])) regionTexts.set(m[2], {});
        const rt = regionTexts.get(m[2]);
        if (!m[1]) { stack.push(m[2]); rt.start = rt.start || l.trim(); }
        else { if (stack.length && stack[stack.length - 1] === m[2]) stack.pop(); else errors.push(`unmatched #endregion ${m[2]}`); rt.end = rt.end || l.trim(); }
      } else out.push(l);
    }
    return out.join("\n");
  };
  for (let i = 0; i < body.length; i++) {
    const s = body[i];
    const lead = evalTrivia(prevEnd, s.getStart(sf, true));
    const docRaw = text.slice(s.getStart(sf, true), s.getStart(sf, false));
    const docClean = evalTrivia(s.getStart(sf, true), s.getStart(sf, false));
    stackAt[i] = [...stack];
    trivia[i] = { lead, doc: docClean, docMarker: docClean !== docRaw };
    prevEnd = s.getEnd();
  }
  const tail = evalTrivia(prevEnd, fnNode.body.getEnd() - 1);
  trivia.tail = tail;
  trivia.finalStack = [...stack];
}
if (trivia.finalStack.length) errors.push("regions left open at the end of the function: " + trivia.finalStack.join(","));

// ---- text of a statement with relative import specifiers rebased
const REPO = "/Users/ueli/Documents/semio";
function rebaseRel(specText, toDir) {
  if (!specText.startsWith(".")) return specText;
  let r = path.relative(toDir, path.resolve(dir, specText)).split(path.sep).join("/");
  if (!r.startsWith(".")) r = "./" + r;
  return r;
}
function stmtText(node, toDir, withDoc) {
  const start = node.getStart(sf, false);
  const end = node.getEnd();
  const edits = [];
  const visit = (n) => {
    if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal)) edits.push([n.argument.literal.getStart(sf) + 1, n.argument.literal.getEnd() - 1, rebaseRel(n.argument.literal.text, toDir)]);
    else if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteral(n.arguments[0])) edits.push([n.arguments[0].getStart(sf) + 1, n.arguments[0].getEnd() - 1, rebaseRel(n.arguments[0].text, toDir)]);
    ts.forEachChild(n, visit);
  };
  visit(node);
  let out = text.slice(start, end);
  for (const [a, b, rep] of edits.sort((x, y) => y[0] - x[0])) out = out.slice(0, a - start) + rep + out.slice(b - start);
  if (/import\.meta\.(dir|dirname|url|filename)|__dirname|__filename/.test(out)) warn.push(`path-relative reference inside the text at line ${lineOf(start)}: ${out.slice(0, 80).replace(/\n/g, " ")}`);
  return out;
}

// ---- imports
const importDecls = top.map((s, ti) => ({ s, ti })).filter((x) => ts.isImportDeclaration(x.s));
function importLine(ti, usedSyms, toDir) {
  const s = top[ti];
  const c = s.importClause;
  const specText = s.moduleSpecifier.text;
  const newSpec = rebaseRel(specText, toDir);
  const q = s.moduleSpecifier.getText(sf)[0];
  const specOut = q + newSpec + q;
  if (!c) return null;
  let def = null, ns = null;
  const named = [];
  if (c.name && usedSyms.has(checker.getSymbolAtLocation(c.name))) def = c.name.text;
  if (c.namedBindings && ts.isNamespaceImport(c.namedBindings) && usedSyms.has(checker.getSymbolAtLocation(c.namedBindings.name))) ns = c.namedBindings.name.text;
  if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const el of c.namedBindings.elements) if (usedSyms.has(checker.getSymbolAtLocation(el.name))) named.push(el.getText(sf));
  if (!def && !ns && !named.length) return null;
  const bits = [];
  if (def) bits.push(def);
  if (ns) bits.push(`* as ${ns}`);
  if (named.length) bits.push(`{ ${named.join(", ")} }`);
  return `import ${c.isTypeOnly ? "type " : ""}${bits.join(", ")} from ${specOut};`;
}

// ---- kit
const kit = spec.kit;
const moveTop = new Set(kit ? kit.moveTop || [] : []);
const topMoved = top.map((s, ti) => ({ s, ti })).filter(({ s, ti }) => ti !== fnIndexInTop && !ts.isImportDeclaration(s) && (() => {
  let nm = [];
  if (ts.isVariableStatement(s)) nm = s.declarationList.declarations.flatMap((d) => (ts.isIdentifier(d.name) ? [d.name.text] : []));
  else if (s.name) nm = [s.name.text];
  return nm.some((n) => moveTop.has(n));
})());
const topMovedIdx = new Set(topMoved.map((x) => x.ti));

// ---- file generation
const regionLinesFor = (open, target) => {
  let k = 0; while (k < open.length && k < target.length && open[k] === target[k]) k++;
  const out = [];
  for (let x = open.length - 1; x >= k; x--) out.push(regionTexts.get(open[x]).end);
  for (let x = k; x < target.length; x++) out.push(regionTexts.get(target[x]).start);
  return out;
};

function bodyOf(idxs, toDir, indent = "  ") {
  const parts = [];
  let open = [];
  for (const i of idxs) {
    const s = body[i];
    const rl = regionLinesFor(open, stackAt[i]);
    open = [...stackAt[i]];
    const lead = trivia[i].lead;
    const leadTrim = lead.replace(/^\s+/, "").replace(/\s+$/, "");
    const ws = lead.match(/^\s*/)[0];
    const blankBefore = parts.length > 0 && ((ws.match(/\n/g) || []).length >= 2 || rl.length > 0);
    const docRaw = trivia[i].doc;
    const bodyTxt = stmtText(s, toDir);
    const chunk = [];
    if (leadTrim) chunk.push(indent + leadTrim);
    if (trivia[i].docMarker) {
      if (docRaw.trim()) chunk.push(indent + docRaw.trim());
      if (rl.length) chunk.push(rl.map((l) => indent + l.trim()).join("\n"));
      chunk.push(indent + bodyTxt);
    } else {
      if (rl.length) chunk.unshift(rl.map((l) => indent + l.trim()).join("\n"));
      chunk.push(indent + docRaw.replace(/^[ \t]+/, "") + bodyTxt);
    }
    parts.push({ text: chunk.join("\n"), blank: blankBefore });
  }
  const tail = regionLinesFor(open, []);
  let out = "";
  parts.forEach((p, k) => { out += (k === 0 ? "" : p.blank ? "\n\n" : "\n") + p.text; });
  if (tail.length) out += "\n" + tail.map((l) => indent + l.trim()).join("\n");
  return out;
}

const outputs = [];
const kitName = kit ? kit.fn : null;
const kitFile = kit ? path.join(dir, kit.dir, "🟦️" + (isTsx ? ".tsx" : ".ts")) : null;
const childFile = kids.map((k) => path.join(dir, k.dir, "🟦️" + (k.ext || (isTsx ? ".tsx" : ".ts"))));
const relFrom = (fromDir, target) => { let r = path.relative(fromDir, target).split(path.sep).join("/"); if (!r.startsWith(".")) r = "./" + r; return r; };
const paramTypeText = (name) => fnParams.find((p) => p.name === name).type;

// closure over declaration dependencies (the decls a text needs)
function needClosure(startSet) {
  const need = new Set(startSet);
  let ch = true;
  while (ch) { ch = false; for (const i of [...need]) for (const j of depsOf(i)) if (!need.has(j) && j !== i) { need.add(j); ch = true; } }
  return need;
}

// bindings needed for a set of body statements: which names of each binding statement
function bindingsNeeded(allStmtIdx) {
  const byBinding = new Map(); // binding stmt idx -> Set(names)
  const used = new Set();
  for (const i of allStmtIdx) {
    const visit = (n) => {
      if (ts.isIdentifier(n)) {
        let sy = checker.getSymbolAtLocation(n);
        if (n.parent && ts.isShorthandPropertyAssignment(n.parent) && n.parent.name === n) sy = checker.getShorthandAssignmentValueSymbol(n.parent) || sy;
        if (sy && declOwner.has(sy)) { const o = declOwner.get(sy); if (bindingSource.has(o.i)) { if (!byBinding.has(o.i)) byBinding.set(o.i, new Set()); byBinding.get(o.i).add(sy); } }
      }
      ts.forEachChild(n, visit);
    };
    visit(body[i]);
  }
  return byBinding;
}
function bindingLines(byBinding) {
  const out = [];
  for (const [i, syms] of [...byBinding].sort((a, b) => a[0] - b[0])) {
    const d = body[i].declarationList.declarations[0];
    const els = d.name.elements.filter((e) => syms.has(checker.getSymbolAtLocation(e.name)));
    out.push({ source: bindingSource.get(i), line: `  const { ${els.map((e) => e.getText(sf)).join(", ")} } = ${bindingSource.get(i)};` });
  }
  return out;
}

// ---- kit module
let kitText = null;
let kitExports = [];
const kitSharedNames = [];
if (kit) {
  const exportedTypes = sharedTypes;
  const idxs = new Set(sharedList.map((d) => d.i));
  for (const d of sharedList) for (const j of depsOf(d.i)) if (!idxs.has(j) && !bindingSource.has(j)) errors.push(`shared ${d.names.join(",")} needs the unshared ${bstmts[j].names.join(",")}`);
  const allIdx = [...idxs];
  const byB = bindingsNeeded(allIdx);
  const blines = bindingLines(byB);
  const syms = new Set();
  const topRefImports = new Set();
  const allNodes = [...allIdx.map((i) => body[i]), ...topMoved.map((x) => x.s)];
  for (const n of allNodes) { const r = refsOf(n); r.importBindings.forEach((x) => syms.add(x)); }
  const impLines = [];
  for (const { ti } of importDecls) { const l = importLine(ti, syms, path.dirname(kitFile)); if (l) impLines.push(l); }
  const topLines = topMoved.map(({ s }) => { const doc = text.slice(s.getStart(sf, true), s.getStart(sf, false)).replace(/^\s+/, ""); return doc + "export " + stmtText(s, path.dirname(kitFile)).replace(/^export\s+/, ""); });
  const typeLines = exportedTypes.map((d) => "export " + stmtText(d.node, path.dirname(kitFile)));
  const valueIdx = sharedValues.map((d) => d.i);
  const valueNames = sharedValues.flatMap((d) => d.names);
  kitSharedNames.push(...valueNames, ...exportedTypes.flatMap((d) => d.names));
  const factory = [];
  if (valueIdx.length) {
    const paramList = fnParams.filter((p) => blines.some((b) => b.source === p.name) || valueIdx.some((i) => stmtRefs[i].params.has(p.name))).map((p) => p.name);
    const paramDecl = paramList.map((p) => `${p}: ${p === "dependencies" && spec.depsType ? spec.depsType : p === "source" && spec.sourceType ? spec.sourceType : paramTypeText(p)}`);
    factory.push(`export function ${kit.fn}(${paramDecl.join(", ")}) {`);
    for (const b of blines) factory.push(b.line);
    if (blines.length) factory.push("");
    factory.push(bodyOf(valueIdx, path.dirname(kitFile)));
    factory.push("");
    factory.push(`  return { ${valueNames.join(", ")} };`);
    factory.push("}");
    kitExports.push({ fn: kit.fn, params: paramList, names: valueNames });
  }
  const typeBlock = typeLines.join("\n");
  kitText = `${kit.doc}\n${impLines.join("\n")}${impLines.length ? "\n\n" : ""}${[topLines.join("\n"), kit.extraText || "", typeBlock, factory.join("\n")].filter(Boolean).join("\n\n")}`.replace(/\s+$/, "") + "\n";
  outputs.push({ path: kitFile, text: kitText });
}

// ---- children
const childSummaries = [];
kids.forEach((k, c) => {
  const toDir = path.dirname(childFile[c]);
  const own = bstmts.filter((b) => b.group === c).map((b) => b.i);
  const localDecls = decls.filter((d) => !bindingSource.has(d.i) && !shared.has(d.i) && (spec.copySharedTypes && d.isType ? usedBy.get(d.i).has(c) : usedBy.get(d.i).size === 1 && usedBy.get(d.i).has(c)));
  const localTypes = localDecls.filter((d) => d.isType);
  const localValues = localDecls.filter((d) => !d.isType);
  const allIdx = [...own, ...localDecls.map((d) => d.i)];
  const byB = bindingsNeeded(allIdx);
  const sharedUsed = sharedValues.filter((d) => allIdx.some((i) => stmtRefs[i].body.has(d.i)));
  const sharedTypesUsed = sharedTypes.filter((d) => allIdx.some((i) => stmtRefs[i].body.has(d.i)));
  const blines = bindingLines(byB);
  const usedImports = new Set();
  for (const i of allIdx) stmtRefs[i].importBindings.forEach((x) => usedImports.add(x));
  for (const i of allIdx) for (const ti of stmtRefs[i].tops) if (!topMovedIdx.has(ti)) errors.push(`${k.dir} uses the top-level declaration at statement ${ti} (${top[ti].getText(sf).slice(0, 40).replace(/\n/g, " ")}) that stays in the parent`);
  const usesParam = (p) => allIdx.some((i) => stmtRefs[i].params.has(p)) || blines.some((b) => b.source === p) || (sharedUsed.length && kitExports[0] && kitExports[0].params.includes(p));
  const params = fnParams.filter((p) => usesParam(p.name));
  const imps = [];
  for (const { ti } of importDecls) { const l = importLine(ti, usedImports, toDir); if (l) imps.push(l); }
  const kitImportNames = [...sharedUsed.flatMap((d) => (kitName ? [] : [])), ...sharedTypesUsed.flatMap((d) => d.names)];
  for (const p of params) {
    const t = p.name === "dependencies" && spec.depsType ? spec.depsType : p.name === "source" && spec.sourceType ? spec.sourceType : p.type;
    p.out = t;
  }
  if (kit) {
    const nm = [];
    if (sharedUsed.length) nm.push(kitName);
    for (const d of sharedTypesUsed) nm.push(...d.names.map((n) => "type " + n));
    for (const n of kit.typeExports || []) if (params.some((p) => new RegExp("\\b" + n + "\\b").test(p.out))) nm.push("type " + n);
    if (nm.length) imps.push(`import { ${nm.join(", ")} } from "${relFrom(toDir, kitFile)}";`);
  }
  const lines2 = [];
  lines2.push(k.doc);
  lines2.push(...imps);
  if (kit && params.some((p) => p.name === "dependencies" && spec.depsTypeImport)) lines2.push(spec.depsTypeImport.replace("%KIT%", relFrom(toDir, kitFile)));
  lines2.push("");
  // the original parameter type text may use relative specifiers: rebase them
  const rebaseTypeText = (t) => t.replace(/import\((["'])(\.[^"']*)\1\)/g, (m, q, s) => `import(${q}${rebaseRel(s, toDir)}${q})`);
  lines2.push(`export async function ${k.fn}(${params.map((p) => `${p.name}: ${rebaseTypeText(p.out)}`).join(", ")}): Promise<void> {`);
  for (const b of blines) lines2.push(b.line);
  if (sharedUsed.length) {
    const names = sharedUsed.flatMap((d) => d.names);
    lines2.push(`  const { ${names.join(", ")} } = ${kitName}(${kitExports[0].params.join(", ")});`);
  }
  if (blines.length || sharedUsed.length) lines2.push("");
  const declIdx = localDecls.map((d) => d.i);
  const order = [...declIdx, ...own].sort((a, b) => a - b);
  // keep original relative order, but declarations always precede the blocks they serve
  const localFirst = [...declIdx.sort((a, b) => a - b), ...own.sort((a, b) => a - b)];
  lines2.push(bodyOf(localFirst, toDir));
  lines2.push("}");
  const out = lines2.join("\n").replace(/\n{3,}/g, "\n\n").replace(/\s+$/, "") + "\n";
  outputs.push({ path: childFile[c], text: out });
  childSummaries.push({ dir: k.dir, fn: k.fn, params: params.map((p) => p.name), lines: out.split("\n").length - 1, local: localDecls.map((d) => d.names.join(",")), shared: sharedUsed.flatMap((d) => d.names) });
});

// ---- orchestrator (parent)
{
  const keepStmts = top.map((s, ti) => ({ s, ti })).filter(({ s, ti }) => !ts.isImportDeclaration(s) && ti !== fnIndexInTop && !topMovedIdx.has(ti));
  const orchestratorBody = [];
  const calls = [];
  childSummaries.forEach((cs, c) => { calls.push(`  await ${cs.fn}(${cs.params.join(", ")});`); });
  // untouched declarations that no group uses stay in the orchestrator
  const orphan = unusedDecls.map((d) => d.i);
  const bl = bindingsNeeded(orphan);
  const syms = new Set();
  for (const { s } of keepStmts) refsOf(s).importBindings.forEach((x) => syms.add(x));
  for (const i of orphan) stmtRefs[i].importBindings.forEach((x) => syms.add(x));
  const imps = [];
  for (const { ti } of importDecls) { const l = importLine(ti, syms, dir); if (l) imps.push(l); }
  const childImports = childSummaries.map((cs, c) => `import { ${cs.fn} } from "${relFrom(dir, childFile[c])}";`);
  const kitImportsParent = [];
  if (kit && topMoved.length) {
    const names = topMoved.flatMap(({ s }) => (ts.isVariableStatement(s) ? s.declarationList.declarations.map((d) => d.name.text) : [s.name.text]));
    const used = names.filter((n) => keepStmts.some(({ s }) => new RegExp("\\b" + n + "\\b").test(s.getText(sf))) || spec.parentUses && spec.parentUses.includes(n));
    const typed = [...(kit.typeExports || [])].filter((n) => (spec.parentTypeUses || []).includes(n));
    if (used.length || typed.length) kitImportsParent.push(`import { ${[...used.filter((n) => !typed.includes(n)), ...typed.map((n) => "type " + n)].join(", ")} } from "${relFrom(dir, kitFile)}";`);
  }
  const sig = fnNode.getText(sf).slice(0, fnNode.body.getStart(sf) - fnNode.getStart(sf)).trimEnd();
  const sigNew = spec.parentSig || sig;
  const fnTxt = `${sigNew} {\n${[...calls].join("\n")}\n}`;
  const keepTxt = keepStmts.map(({ s }) => stmtTextTop(s)).join("\n");
  function stmtTextTop(s) { return text.slice(s.getStart(sf, true), s.getEnd()); }
  const parts = [];
  const docTop = text.slice(0, top[0].getStart(sf, true));
  const before = keepStmts.filter(({ ti }) => ti < fnIndexInTop).map(({ s }) => stmtTextTop(s));
  const after = keepStmts.filter(({ ti }) => ti > fnIndexInTop).map(({ s }) => stmtTextTop(s));
  const importBlock = [...imps, ...childImports, ...kitImportsParent].join("\n");
  const outText = `${importBlock}\n\n${before.join("\n\n")}${before.length ? "\n\n" : ""}${fnTxt}\n\n${after.join("\n")}\n`;
  outputs.unshift({ path: file, text: outText.replace(/\n{3,}/g, "\n\n") });
}

// ---- report
console.log("kit:", kit ? `${kit.dir} (${kitText.split("\n").length - 1} lines)` : "none", "shared values:", sharedValues.flatMap((d) => d.names).join(","), "shared types:", sharedTypes.flatMap((d) => d.names).join(","));
console.log("top-level moved to the kit:", topMoved.map(({ s }) => s.getText(sf).slice(0, 40).replace(/\n/g, " ")).join(" | "));
childSummaries.forEach((cs) => console.log(`${cs.dir.padEnd(34)} ${cs.fn.padEnd(40)} ${String(cs.lines).padStart(5)} lines  params=${cs.params.join(",")}  local=${cs.local.join(",")}  shared=${cs.shared.join(",")}`));
console.log("parent:", outputs[0].text.split("\n").length - 1, "lines");
for (const w of warn) console.log("WARN", w);
for (const e of errors) console.log("ERROR", e);
if (APPLY) {
  if (errors.length) { console.log("refusing to apply: errors"); process.exit(1); }
  const cur = fs.readFileSync(file, "utf8");
  if (process.env.ORIG_TEXT && cur !== text) { console.log("live file differs from the snapshot; refusing"); process.exit(1); }
  for (const o of outputs.slice(1)) { if (fs.existsSync(o.path)) { console.log("exists: " + o.path); process.exit(1); } }
  for (const o of outputs.slice(1)) { fs.mkdirSync(path.dirname(o.path), { recursive: true }); fs.writeFileSync(o.path, o.text); }
  fs.writeFileSync(file, outputs[0].text);
  console.log("applied", outputs.length, "files");
} else if (process.env.STAGE_DIR) {
  for (const o of outputs) { const p = path.join(process.env.STAGE_DIR, path.relative(REPO, o.path)); fs.mkdirSync(path.dirname(p), { recursive: true }); fs.writeFileSync(p, o.text); }
  console.log("staged under", process.env.STAGE_DIR);
}
