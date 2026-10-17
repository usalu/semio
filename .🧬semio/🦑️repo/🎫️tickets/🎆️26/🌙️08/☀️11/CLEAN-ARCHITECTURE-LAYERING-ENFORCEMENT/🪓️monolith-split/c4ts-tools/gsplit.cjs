// usage: node gsplit.cjs <spec.json> [APPLY]
// General TypeScript module splitter (parent keeps what is not picked; picked top-level statements move to children).
// spec: { file, children: [{ dir, doc, lines?: [[a,b]...], names?: [..] }], importers?: "auto" | [paths], keepInParent?: [names] }
// A statement belongs to the first child whose selector matches (statement start line, JSDoc included, inside a line range, or a declared name).
// Dry run prints sizes, cross-file edges and violations. APPLY writes children, the new parent and the re-pointed importers.
const fs = require("fs");
const path = require("path");
const cp = require("child_process");
const ts = require("/Users/ueli/Documents/semio/node_modules/typescript");

const spec = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const APPLY = process.argv[3] === "APPLY";
const REPO = "/Users/ueli/Documents/semio";
const file = spec.file;
const dir = path.dirname(file);
const text = fs.readFileSync(process.env.ORIG_TEXT || file, "utf8");
const REPOINT = process.argv[3] === "REPOINT";
const isTsx = file.endsWith(".tsx");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, isTsx ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const host = ts.createCompilerHost({});
host.getSourceFile = (n) => (n === file ? sf : undefined);
host.fileExists = (n) => n === file;
host.readFile = (n) => (n === file ? text : undefined);
const program = ts.createProgram([file], { noResolve: true, noLib: true, target: ts.ScriptTarget.Latest, jsx: ts.JsxEmit.Preserve }, host);
const checker = program.getTypeChecker();
const N = sf.statements.length;
const lineOf = (pos) => sf.getLineAndCharacterOfPosition(pos).line + 1;
const errors = [];
const warn = [];

const regionRe = /^[ \t]*\/\/\s*#(end)?region\s+(.*?)\s*$/;

const stmts = sf.statements.map((s, i) => {
  let names = [];
  if (ts.isVariableStatement(s)) names = s.declarationList.declarations.flatMap((d) => (ts.isIdentifier(d.name) ? [d.name.text] : d.name.elements ? d.name.elements.map((e) => e.name.getText(sf)) : []));
  else if (ts.isFunctionDeclaration(s) || ts.isClassDeclaration(s) || ts.isInterfaceDeclaration(s) || ts.isTypeAliasDeclaration(s) || ts.isEnumDeclaration(s)) names = s.name ? [s.name.text] : [];
  const exported = !!(ts.getCombinedModifierFlags(s) & ts.ModifierFlags.Export);
  const isImport = ts.isImportDeclaration(s);
  const typeOnlyDecl = ts.isInterfaceDeclaration(s) || ts.isTypeAliasDeclaration(s);
  return { i, node: s, names, exported, isImport, typeOnlyDecl, startLine: lineOf(s.getStart(sf, true)), endLine: lineOf(s.getEnd()) };
});

const declSyms = new Map();
const nameOfSym = new Map();
for (const st of stmts) {
  const s = st.node;
  const syms = [];
  if (ts.isVariableStatement(s)) for (const d of s.declarationList.declarations) { if (ts.isIdentifier(d.name)) { const sy = checker.getSymbolAtLocation(d.name); if (sy) syms.push([sy, d.name.text]); } else warn.push(`destructuring declaration at line ${st.startLine}`); }
  else if (s.name && ts.isIdentifier(s.name)) { const sy = checker.getSymbolAtLocation(s.name); if (sy) syms.push([sy, s.name.text]); }
  for (const [sy, nm] of syms) { declSyms.set(sy, st.i); nameOfSym.set(sy, nm); }
}

const importBinding = new Map();
sf.statements.forEach((s, i) => {
  if (!ts.isImportDeclaration(s) || !s.importClause) return;
  const c = s.importClause;
  if (c.name) importBinding.set(checker.getSymbolAtLocation(c.name), { decl: i, kind: "default" });
  if (c.namedBindings && ts.isNamespaceImport(c.namedBindings)) importBinding.set(checker.getSymbolAtLocation(c.namedBindings.name), { decl: i, kind: "ns" });
  if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const el of c.namedBindings.elements) importBinding.set(checker.getSymbolAtLocation(el.name), { decl: i, kind: "named", el });
});

const refs = stmts.map(() => ({ stmtRefs: new Map(), importRefs: new Set(), assigned: new Set() }));
const visit = (n, i) => {
  if (ts.isIdentifier(n)) {
    let sy = checker.getSymbolAtLocation(n);
    if (!sy && n.parent && ts.isShorthandPropertyAssignment(n.parent)) sy = checker.getShorthandAssignmentValueSymbol(n.parent);
    if (sy) {
      if (declSyms.has(sy)) {
        const j = declSyms.get(sy);
        if (j !== i) {
          const par = n.parent;
          const isAssign = par && ((ts.isBinaryExpression(par) && par.left === n && par.operatorToken.kind >= ts.SyntaxKind.FirstAssignment && par.operatorToken.kind <= ts.SyntaxKind.LastAssignment) || ts.isPostfixUnaryExpression(par) || ts.isPrefixUnaryExpression(par) && (par.operator === ts.SyntaxKind.PlusPlusToken || par.operator === ts.SyntaxKind.MinusMinusToken));
          if (isAssign) refs[i].assigned.add(j);
          if (!refs[i].stmtRefs.has(j)) refs[i].stmtRefs.set(j, new Set());
          refs[i].stmtRefs.get(j).add(nameOfSym.get(sy));
        }
      } else if (importBinding.has(sy) && !ts.isImportSpecifier(n.parent) && !ts.isImportClause(n.parent) && !ts.isNamespaceImport(n.parent)) refs[i].importRefs.add(sy);
    }
  }
  ts.forEachChild(n, (c) => visit(c, i));
};
stmts.forEach((st, i) => { if (!st.isImport) visit(st.node, i); });

// export lists (export { a, b }) without module specifier
stmts.forEach((st) => { if (ts.isExportDeclaration(st.node) && !st.node.moduleSpecifier) warn.push(`export list at line ${st.startLine} (kept in parent; locals referenced must stay or be exported manually)`); });

// ---- assignment of statements to files (0 = parent, 1.. = children)
const kids = spec.children;
const fileOf = new Array(N).fill(0);
const kidSel = kids.map((k) => ({ lines: k.lines || [], names: new Set(k.names || []), res: (k.re || []).map((r) => new RegExp(r, "u")) }));
stmts.forEach((st) => {
  if (st.isImport) return;
  for (let c = 0; c < kids.length; c++) {
    const sel = kidSel[c];
    if (sel.lines.some(([a, b]) => st.startLine >= a && st.startLine <= b)) { fileOf[st.i] = c + 1; return; }
    if (st.names.length && st.names.some((nm) => sel.names.has(nm) || sel.res.some((re) => re.test(nm)))) { fileOf[st.i] = c + 1; return; }
  }
});
for (const nm of spec.keepInParent || []) { stmts.forEach((st) => { if (st.names.includes(nm)) fileOf[st.i] = 0; }); }
for (const st of stmts) if (st.isImport) fileOf[st.i] = 0;
const shared = stmts.map((st) => !st.isImport && (spec.sharedLines || []).some(([a, b]) => st.startLine >= a && st.startLine <= b));
stmts.forEach((st) => { if (shared[st.i]) fileOf[st.i] = 0; });
for (const st of stmts) if (st.names.length > 1 && new Set(st.names.map(() => 0)).size) { /* multi-declarator statements move as a unit */ }

// ---- cross-file edges
const fileNames = ["parent", ...kids.map((k) => k.dir)];
const fileEdges = kids.map(() => new Map());
const parentEdges = new Map();
const need = fileNames.map(() => new Map());
stmts.forEach((st) => {
  const a = fileOf[st.i];
  for (const [j, nameSet] of refs[st.i].stmtRefs) {
    const b = fileOf[j];
    if (a === b || shared[j]) continue;
    if (!need[a].has(b)) need[a].set(b, new Map());
    for (const nm of nameSet) need[a].get(b).set(nm, { j });
  }
  for (const j of refs[st.i].assigned) if (fileOf[j] !== a) errors.push(`stmt line ${st.startLine} (${fileNames[a]}) assigns to ${stmts[j].names.join(",")} (${fileNames[fileOf[j]]}): imported bindings cannot be assigned`);
});
// child importing from parent is a violation
for (let a = 1; a < fileNames.length; a++) if (need[a].has(0) && !spec.childrenImportParent) errors.push(`${fileNames[a]} needs from parent: ${[...need[a].get(0).keys()].join(", ")}`);
// cycle detection between children
{
  const color = new Array(fileNames.length).fill(0);
  const stack = [];
  const dfs = (u) => {
    color[u] = 1; stack.push(u);
    for (const v of need[u].keys()) {
      if (color[v] === 1) errors.push(`file cycle: ${stack.slice(stack.indexOf(v)).concat(v).map((x) => fileNames[x]).join(" -> ")}`);
      else if (color[v] === 0) dfs(v);
    }
    stack.pop(); color[u] = 2;
  };
  for (let u = 0; u < fileNames.length; u++) if (color[u] === 0) dfs(u);
}

// ---- region stacks
const lines = text.split("\n");
const lineStartOffsets = []; { let off = 0; for (const l of lines) { lineStartOffsets.push(off); off += l.length + 1; } }
const regionTexts = new Map(); // name -> {start, end}
const stackAtStmtStart = new Array(N);
const trivia = new Array(N); // { text (marker lines removed), blankBefore }
{
  let stack = [];
  let prevEnd = 0;
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
        if (!m[1]) { stack.push(m[2]); if (!regionTexts.has(m[2])) regionTexts.set(m[2], {}); regionTexts.get(m[2]).start = regionTexts.get(m[2]).start || l.trim(); }
        else {
          if (stack.length && stack[stack.length - 1] === m[2]) stack.pop();
          else errors.push(`unmatched #endregion ${m[2]} near offset ${from}`);
          if (!regionTexts.has(m[2])) regionTexts.set(m[2], {}); regionTexts.get(m[2]).end = regionTexts.get(m[2]).end || l.trim();
        }
      } else out.push(l);
    }
    return out.join("\n");
  };
  for (let i = 0; i < N; i++) {
    const s = sf.statements[i];
    const lead = evalTrivia(prevEnd, s.getStart(sf, true));
    const docRawI = text.slice(s.getStart(sf, true), s.getStart(sf, false));
    const docClean = evalTrivia(s.getStart(sf, true), s.getStart(sf, false));
    stackAtStmtStart[i] = [...stack];
    trivia[i] = { lead, doc: docClean, docMarker: docClean !== docRawI };
    const stmtText = text.slice(s.getStart(sf, true), s.getEnd());
    // regions inside a statement body must balance on their own; skip nested markers
    prevEnd = s.getEnd();
  }
  const tail = evalTrivia(prevEnd, text.length);
  trivia.tail = tail;
  trivia.finalStack = [...stack];
}
if (trivia.finalStack.length) errors.push("regions left open at EOF: " + trivia.finalStack.join(","));
for (const [nm, v] of regionTexts) { if (!v.start || !v.end) errors.push(`region ${nm} lacks a start or end marker`); }

// ---- imports bookkeeping
const importDecls = stmts.filter((s) => s.isImport);
function rebaseSpec(spec0, toFile) {
  if (!spec0.startsWith(".")) return spec0;
  if (toFile === file) return spec0;
  return relFrom(path.dirname(toFile), path.resolve(dir, spec0));
}
function importLineFor(declStmt, usedSet, toFile) {
  const s = declStmt.node;
  const c = s.importClause;
  const specText = s.moduleSpecifier.text;
  const newSpec = rebaseSpec(specText, toFile || file);
  const q = s.moduleSpecifier.getText(sf)[0];
  const specOut = newSpec === specText ? s.moduleSpecifier.getText(sf) : q + newSpec + q;
  if (!c) return { text: newSpec === specText ? s.getText(sf) : `import ${specOut};`, side: true };
  let def = null, ns = null; const named = [];
  if (c.name && usedSet.has(checker.getSymbolAtLocation(c.name))) def = c.name.text;
  if (c.namedBindings && ts.isNamespaceImport(c.namedBindings) && usedSet.has(checker.getSymbolAtLocation(c.namedBindings.name))) ns = c.namedBindings.name.text;
  let allNamed = 0, keptNamed = 0;
  if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const el of c.namedBindings.elements) { allNamed++; if (usedSet.has(checker.getSymbolAtLocation(el.name))) { named.push(el.getText(sf)); keptNamed++; } }
  const totalBindings = (c.name ? 1 : 0) + (c.namedBindings && ts.isNamespaceImport(c.namedBindings) ? 1 : 0) + allNamed;
  const kept = (def ? 1 : 0) + (ns ? 1 : 0) + keptNamed;
  if (!kept) return null;
  if (kept === totalBindings && newSpec === specText) return { text: s.getText(sf) };
  const bits = [];
  if (def) bits.push(def);
  if (ns) bits.push(`* as ${ns}`);
  if (named.length) bits.push(`{ ${named.join(", ")} }`);
  return { text: `import ${c.isTypeOnly ? "type " : ""}${bits.join(", ")} from ${specOut};` };
}

// used import bindings per output file
const usedImports = fileNames.map(() => new Set());
stmts.forEach((st) => { for (const sy of refs[st.i].importRefs) { usedImports[fileOf[st.i]].add(sy); if (shared[st.i]) usedImports.forEach((u) => u.add(sy)); } });

// ---- extension for children
const jsxIn = (nodeList) => { let f = false; const v = (n) => { if (ts.isJsxElement(n) || ts.isJsxSelfClosingElement(n) || ts.isJsxFragment(n)) f = true; if (!f) ts.forEachChild(n, v); }; nodeList.forEach(v); return f; };
const childExt = kids.map((k, c) => (isTsx && jsxIn(stmts.filter((s) => fileOf[s.i] === c + 1).map((s) => s.node)) ? ".tsx" : ".ts"));
const childFile = kids.map((k, c) => path.join(spec.childRoot || dir, k.dir, "🟦️" + childExt[c]));
const relFrom = (fromDir, target) => { let r = path.relative(fromDir, target).split(path.sep).join("/"); if (!r.startsWith(".")) r = "./" + r; return r; };
const fileAbs = [file, ...childFile];

// ---- needed `export` on moved statements referenced from other files
const exportAdds = new Set();
for (let a = 0; a < fileNames.length; a++) for (const [b, nm] of need[a]) for (const [name, { j }] of nm) if (!stmts[j].exported) exportAdds.add(j);

// ---- generate a file
function emitStatements(fi, bodyFn, afterIdx, afterLines) {
  const parts = [];
  let open = [];
  const closeTo = (target) => {
    let k = 0; while (k < open.length && k < target.length && open[k] === target[k]) k++;
    const out = [];
    for (let x = open.length - 1; x >= k; x--) out.push(regionTexts.get(open[x]).end);
    for (let x = k; x < target.length; x++) out.push(regionTexts.get(target[x]).start);
    open = [...target];
    return out;
  };
  for (let i = 0; i < N; i++) {
    if (fileOf[i] !== fi && !shared[i]) continue;
    const st = stmts[i];
    const s = st.node;
    const regionLines = closeTo(stackAtStmtStart[i]);
    const lead = trivia[i].lead;
    const leadTrim = lead.replace(/^\s+/, "").replace(/\s+$/, "");
    const ws = lead.match(/^\s*/)[0];
    const blankBefore = parts.length > 0 && ((ws.match(/\n/g) || []).length >= 2 || regionLines.length > 0);
    const docRaw = trivia[i].doc;
    let body = text.slice(s.getStart(sf, false), s.getEnd());
    if (exportAdds.has(i)) body = "export " + body;
    if (bodyFn) body = bodyFn(i, body);
    const chunk = [];
    if (leadTrim) chunk.push(leadTrim);
    if (trivia[i].docMarker) {
      if (docRaw.trim()) chunk.push(docRaw.trim());
      if (regionLines.length) chunk.push(regionLines.join("\n"));
      if (body !== null) chunk.push(body);
    } else {
      if (regionLines.length) chunk.unshift(regionLines.join("\n"));
      if (body !== null) chunk.push(docRaw + body);
      else if (docRaw.trim()) chunk.push(docRaw.trim());
    }
    if (chunk.length) parts.push({ text: chunk.join("\n"), blank: blankBefore });
    if (i === afterIdx && afterLines && afterLines.length) parts.push({ text: afterLines.join("\n"), blank: false });
  }
  const tailLines = closeTo([]);
  return { parts, tailLines };
}
function join(parts, tailLines) {
  let out = "";
  parts.forEach((p, idx) => { out += (idx === 0 ? "" : p.blank ? "\n\n" : "\n") + p.text; });
  if (tailLines.length) out += "\n" + tailLines.join("\n");
  return out;
}

function crossImports(fi) {
  const res = [];
  for (const [b, nm] of [...need[fi]].sort((x, y) => x[0] - y[0])) {
    const specs = [...nm.entries()].map(([name, { j }]) => ({ name, type: stmts[j].typeOnlyDecl })).sort((x, y) => x.name.localeCompare(y.name));
    const allType = specs.every((x) => x.type);
    const target = relFrom(path.dirname(fileAbs[fi]), fileAbs[b]);
    if (allType) res.push(`import type { ${specs.map((x) => x.name).join(", ")} } from "${target}";`);
    else res.push(`import { ${specs.map((x) => (x.type ? "type " : "") + x.name).join(", ")} } from "${target}";`);
  }
  return res;
}

const outputs = [];
let lastImportIdx = -1; while (lastImportIdx + 1 < N && stmts[lastImportIdx + 1].isImport) lastImportIdx++;
{
  const importRewrites = new Map();
  for (const st of importDecls) importRewrites.set(st.i, importLineFor(st, usedImports[0]));
  const bodyFn = (i, body) => {
    if (!importRewrites.has(i)) return body;
    const r = importRewrites.get(i);
    return r === null ? null : r.text;
  };
  const crossLines = crossImports(0);
  if (spec.sideEffectImports) kids.forEach((k, c) => crossLines.push(`import "${relFrom(dir, childFile[c])}";`));
  const { parts, tailLines } = emitStatements(0, bodyFn, lastImportIdx, crossLines);
  let out = join(parts, tailLines);
  if (lastImportIdx < 0 && crossLines.length) out = crossLines.join("\n") + "\n\n" + out;
  outputs.push({ path: file, text: out.replace(/\s+$/, "") + "\n" });
}
kids.forEach((k, c) => {
  const fi = c + 1;
  const imps = [];
  for (const st of importDecls) { const r = importLineFor(st, usedImports[fi], childFile[c]); if (r) imps.push(r.text); }
  imps.push(...crossImports(fi));
  const merged = [];
  const seen = new Map();
  for (const l of imps) {
    const m = l.match(/^import (type )?\{ (.*) \} from (["'])(.*)\3;$/);
    if (!m) { merged.push(l); continue; }
    const key = (m[1] || "") + "|" + m[4];
    if (seen.has(key)) { const e = seen.get(key); for (const n of m[2].split(", ")) if (!e.names.includes(n)) e.names.push(n); }
    else { const e = { type: m[1] || "", q: m[3], spec: m[4], names: m[2].split(", "), idx: merged.length }; seen.set(key, e); merged.push(e); }
  }
  imps.length = 0;
  for (const e of merged) imps.push(typeof e === "string" ? e : `import ${e.type}{ ${e.names.join(", ")} } from ${e.q}${e.spec}${e.q};`);
  const { parts, tailLines } = emitStatements(fi, null);
  const body = join(parts, tailLines);
  const doc = k.doc;
  if (!doc || !/^\/\*\*\s*\S/.test(doc)) errors.push(`child ${k.dir}: missing docstring`);
  const textOut = `${doc}\n${imps.join("\n")}${imps.length ? "\n\n" : ""}${body}`.replace(/\s+$/, "") + "\n";
  outputs.push({ path: childFile[c], text: textOut });
});

// ---- importers
const movedExports = new Map(); // name -> child index
stmts.forEach((st) => { if (fileOf[st.i] > 0) for (const nm of st.names) if (st.exported || exportAdds.has(st.i)) movedExports.set(nm, fileOf[st.i] - 1); });
let importerFiles = spec.importers;
if (importerFiles === "auto" || !importerFiles) {
  const dirName = path.basename(dir) + "/" + path.basename(file, path.extname(file));
  const out = cp.spawnSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "grep", "--untracked", "-l", "-F", dirName, "--", ":!.🧬semio"], { cwd: REPO, encoding: "utf8", maxBuffer: 1 << 28 });
  const sub = cp.spawnSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "ls-files", "--cached", "--others", "--exclude-standard", "--", path.relative(REPO, dir)], { cwd: REPO, encoding: "utf8", maxBuffer: 1 << 28 });
  importerFiles = [...new Set([...out.stdout.split("\n"), ...sub.stdout.split("\n")].filter(Boolean))].map((p) => path.join(REPO, p)).filter((p) => /\.(tsx?|mts|cts)$/.test(p) && p !== file && fs.existsSync(p));
}
const importerWrites = [];
const manual = [];
const ownResolve = (p, spec0) => path.resolve(path.dirname(p), spec0);
const targetsParent = (p, spec0) => spec0.startsWith(".") && (ownResolve(p, spec0) === file || ownResolve(p, spec0) === file.replace(/\.tsx?$/, ""));
for (const p of importerFiles) {
  if (outputs.some((o) => o.path === p)) continue;
  const t = fs.readFileSync(p, "utf8");
  const s2 = ts.createSourceFile(p, t, ts.ScriptTarget.Latest, true, p.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const edits = [];
  const walk = (node) => {
    if ((ts.isImportDeclaration(node) || ts.isExportDeclaration(node)) && node.moduleSpecifier && ts.isStringLiteral(node.moduleSpecifier) && targetsParent(p, node.moduleSpecifier.text)) {
      const clause = ts.isImportDeclaration(node) ? node.importClause : node.exportClause;
      if (ts.isImportDeclaration(node) && !node.importClause) { /* side-effect import */ }
      else if (ts.isExportDeclaration(node) && !node.exportClause) manual.push(`${p}: export * from parent`);
      else {
        const nb = ts.isImportDeclaration(node) ? clause.namedBindings : clause;
        if (ts.isImportDeclaration(node) && clause.name) manual.push(`${p}: default import of parent`);
        if (nb && ts.isNamespaceExport(nb)) manual.push(`${p}: namespace export of parent (${nb.getText(s2)})`);
        else if (nb && ts.isNamespaceImport(nb)) {
          const nsName = nb.name.text;
          const members = new Map();
          let bare = false;
          const scan = (n) => {
            if (ts.isPropertyAccessExpression(n) && ts.isIdentifier(n.expression) && n.expression.text === nsName) { members.set(n.name.text, [...(members.get(n.name.text) || []), { s: n.getStart(s2), e: n.getEnd() }]); return; }
            if (ts.isQualifiedName(n) && ts.isIdentifier(n.left) && n.left.text === nsName) { members.set(n.right.text, [...(members.get(n.right.text) || []), { s: n.getStart(s2), e: n.getEnd() }]); return; }
            if (ts.isIdentifier(n) && n.text === nsName && n !== nb.name) {
              const par = n.parent;
              const isName = par && ((ts.isPropertyAccessExpression(par) && par.name === n) || (ts.isQualifiedName(par) && par.right === n) || ((ts.isPropertyAssignment(par) || ts.isPropertySignature(par) || ts.isMethodDeclaration(par) || ts.isPropertyDeclaration(par) || ts.isEnumMember(par)) && par.name === n) || (ts.isBindingElement(par) && par.propertyName === n));
              if (!isName) bare = true;
            }
            ts.forEachChild(n, scan);
          };
          ts.forEachChild(s2, (c) => { if (c !== node) scan(c); });
          if (bare && !(spec.allowBare || []).some((a) => p.includes(a))) manual.push(`${p}: namespace ${nsName} also used bare`);
          const movedMembers = [...members.keys()].filter((m) => movedExports.has(m));
          if (movedMembers.length) {
            const rest = [...members.keys()].filter((m) => !movedExports.has(m));
            const collide = movedMembers.filter((m) => new RegExp("(?<![.\\w])" + m.replace(/[$]/g, "\\$&") + "(?!\\w)", "u").test(t.replace(new RegExp("\\b" + nsName + "\\." + m + "\\b", "gu"), "")));
            {
              const aliasOf = (m) => (collide.includes(m) ? nsName + m : m);
              for (const m of movedMembers) for (const r of members.get(m)) edits.push({ start: r.s, end: r.e, text: aliasOf(m) });
              const groups2 = new Map();
              for (const m of movedMembers) { const c = movedExports.get(m); if (!groups2.has(c)) groups2.set(c, []); groups2.get(c).push(m); }
              const lines2 = [];
              if (rest.length) lines2.push(node.getText(s2));
              const q = node.moduleSpecifier.getText(s2)[0];
              for (const [c, list] of [...groups2.entries()].sort((a, b) => a[0] - b[0])) {
                const types = list.filter((m) => stmts.some((st) => st.names.includes(m) && st.typeOnlyDecl));
                const vals = list.filter((m) => !types.includes(m));
                const sp = relFrom(path.dirname(p), childFile[c]);
                const spec1 = (m, isType) => (isType ? "type " : "") + m + (aliasOf(m) !== m ? " as " + aliasOf(m) : "");
                if (vals.length) lines2.push(`import { ${[...vals.map((m) => spec1(m, false)), ...types.map((m) => spec1(m, true))].sort((a, b) => a.replace(/^type /, "").localeCompare(b.replace(/^type /, ""))).join(", ")} } from ${q}${sp}${q};`);
                else lines2.push(`import type { ${types.sort().map((m) => spec1(m, false)).join(", ")} } from ${q}${sp}${q};`);
              }
              edits.push({ start: node.getStart(s2), end: node.getEnd(), text: lines2.join("\n") });
            }
          }
        }
        else if (nb && (ts.isNamedImports(nb) || ts.isNamedExports(nb))) {
          const els = [...nb.elements];
          const groups = new Map();
          const keep = [];
          for (const e of els) {
            const orig = (e.propertyName ?? e.name).text;
            if (movedExports.has(orig)) { const c = movedExports.get(orig); if (!groups.has(c)) groups.set(c, []); groups.get(c).push(e); } else keep.push(e);
          }
          if (groups.size) {
            const q = node.moduleSpecifier.getText(s2)[0];
            const head = ts.isImportDeclaration(node) ? "import " + (clause.isTypeOnly ? "type " : "") : "export " + (node.isTypeOnly ? "type " : "");
            const mk = (list, sp) => `${head}{ ${list.map((e) => e.getText(s2)).join(", ")} } from ${q}${sp}${q};`;
            const reps = [];
            if (keep.length) reps.push(mk(keep, node.moduleSpecifier.text));
            for (const [c, list] of [...groups.entries()].sort((a, b) => a[0] - b[0])) reps.push(mk(list, relFrom(path.dirname(p), childFile[c])));
            const indent = (() => { const ls = t.lastIndexOf("\n", node.getStart(s2)); return t.slice(ls + 1, node.getStart(s2)); })();
            edits.push({ start: node.getStart(s2), end: node.getEnd(), text: reps.join("\n" + indent) });
          }
        } else if (nb) manual.push(`${p}: unsupported form ${nb.getText(s2)}`);
      }
    }
    if (ts.isCallExpression(node) && node.expression.kind === ts.SyntaxKind.ImportKeyword && node.arguments[0] && ts.isStringLiteral(node.arguments[0]) && targetsParent(p, node.arguments[0].text)) manual.push(`${p}: dynamic import of parent at line ${s2.getLineAndCharacterOfPosition(node.getStart(s2)).line + 1}`);
    if (ts.isImportTypeNode(node) && ts.isLiteralTypeNode(node.argument) && ts.isStringLiteral(node.argument.literal) && targetsParent(p, node.argument.literal.text)) manual.push(`${p}: import() type of parent`);
    ts.forEachChild(node, walk);
  };
  walk(s2);
  if (!edits.length) continue;
  let r = t;
  for (const e of edits.sort((a, b) => b.start - a.start)) r = r.slice(0, e.start) + e.text + r.slice(e.end);
  importerWrites.push({ path: p, text: r, count: edits.length });
}

const allowed = spec.allowManual || [];
for (let k = manual.length - 1; k >= 0; k--) if (allowed.some((a) => manual[k].includes(a))) { warn.push("allowed manual: " + manual[k]); manual.splice(k, 1); }

// ---- report
const lc = (t) => t.split("\n").length - 1;
const report = { outputs: outputs.map((o) => ({ path: path.relative(dir, o.path), lines: lc(o.text) })), warnings: warn, errors, manualImporters: manual, importersRewritten: importerWrites.map((w) => path.relative(REPO, w.path)), exportsAdded: [...exportAdds].map((j) => stmts[j].names.join(",")), crossEdges: fileNames.map((n, a) => [n, [...need[a].entries()].map(([b, m]) => `${fileNames[b]}:${[...m.keys()].join("|")}`)]) };
console.log(JSON.stringify(report, null, 1));
if (REPOINT) {
  if (errors.length) { console.error("REFUSED: errors present"); process.exit(2); }
  if (manual.length) { console.error("REFUSED: manual importers present"); process.exit(3); }
  for (const w of importerWrites) fs.writeFileSync(w.path, w.text);
  console.log("REPOINTED " + importerWrites.length);
} else if (APPLY) {
  if (errors.length) { console.error("REFUSED: errors present"); process.exit(2); }
  if (manual.length) { console.error("REFUSED: manual importers present"); process.exit(3); }
  for (const o of outputs.slice(1)) if (fs.existsSync(o.path)) { console.error("REFUSED: child exists " + o.path); process.exit(4); }
  if (fs.readFileSync(file, "utf8") !== text) { console.error("REFUSED: parent changed during run"); process.exit(5); }
  for (const w of importerWrites) { const cur = fs.readFileSync(w.path, "utf8"); w.orig = cur; }
  for (const o of outputs.slice(1)) { fs.mkdirSync(path.dirname(o.path), { recursive: true }); fs.writeFileSync(o.path, o.text); }
  fs.writeFileSync(file, outputs[0].text);
  for (const w of importerWrites) fs.writeFileSync(w.path, w.text);
  console.log("APPLIED");
} else if (process.env.DUMP) {
  const d = process.env.DUMP;
  fs.mkdirSync(d, { recursive: true });
  for (const o of outputs) fs.writeFileSync(path.join(d, path.basename(path.dirname(o.path)) + ".out" + path.extname(o.path)), o.text);
  for (const w of importerWrites) fs.writeFileSync(path.join(d, "imp-" + path.basename(path.dirname(w.path)) + path.extname(w.path)), w.text);
}
