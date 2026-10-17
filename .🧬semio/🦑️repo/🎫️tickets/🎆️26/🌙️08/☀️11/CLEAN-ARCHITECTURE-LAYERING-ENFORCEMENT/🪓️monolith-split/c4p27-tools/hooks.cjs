// usage: node hooks.cjs analyze <plan.json>            -> statement table with declared names, bag reads and forward references
//        node hooks.cjs emit <plan.json> [APPLY]       -> dry run (prints sizes/checks) or writes hook modules and the new parent
// plan: { file, fn, propsType, hookRoot, parentDoc?, hooks: [{ dir, name, doc, upto }] }  (`upto` = exclusive statement index; the last hook ends where the final `return` starts)
// Splits the body of one React function component into hooks that pass a shared `core` bag (same scheme as the NodeGraph and ShellHost hook decompositions).
const fs = require("fs");
const path = require("path");
const ts = require("/Users/ueli/Documents/semio/node_modules/typescript");

const plan = JSON.parse(fs.readFileSync(process.argv[3], "utf8"));
const mode = process.argv[2];
const APPLY = process.argv[4] === "APPLY";
const file = plan.file;
const dir = path.dirname(file);
const text = fs.readFileSync(plan.source || file, "utf8");
const isTsx = file.endsWith(".tsx");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, isTsx ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const host = ts.createCompilerHost({});
host.getSourceFile = (n) => (n === file ? sf : undefined);
host.fileExists = (n) => n === file;
host.readFile = (n) => (n === file ? text : undefined);
const program = ts.createProgram([file], { noResolve: true, noLib: true, target: ts.ScriptTarget.Latest, jsx: ts.JsxEmit.Preserve }, host);
const checker = program.getTypeChecker();

const fnDecl = sf.statements.find((s) => ts.isFunctionDeclaration(s) && s.name && s.name.text === plan.fn);
if (!fnDecl) throw new Error("function not found");
const body = fnDecl.body;
const bstmts = body.statements;
const retIdx = bstmts.length - 1;
if (!ts.isReturnStatement(bstmts[retIdx])) throw new Error("last statement is not a return");
const tailFrom = plan.parentTail ?? retIdx;
const lineOf = (pos) => sf.getLineAndCharacterOfPosition(pos).line + 1;

const declOwner = new Map();
const declaredNames = bstmts.map(() => []);
bstmts.forEach((s, i) => {
  const add = (nameNode) => {
    if (ts.isIdentifier(nameNode)) { const sy = checker.getSymbolAtLocation(nameNode); declaredNames[i].push(nameNode.text); if (sy) declOwner.set(sy, i); }
    else for (const el of nameNode.elements) if (!ts.isOmittedExpression(el)) add(el.name);
  };
  if (ts.isVariableStatement(s)) for (const d of s.declarationList.declarations) add(d.name);
  else if (ts.isFunctionDeclaration(s) && s.name) { declaredNames[i].push(s.name.text); const sy = checker.getSymbolAtLocation(s.name); if (sy) declOwner.set(sy, i); }
});
const propSyms = new Map();
const param = fnDecl.parameters[0];
const propNames = [];
if (ts.isObjectBindingPattern(param.name)) for (const el of param.name.elements) { const sy = checker.getSymbolAtLocation(el.name); propSyms.set(sy, el.name.text); propNames.push(el.name.text); }
const importSyms = new Map();
sf.statements.forEach((s, i) => {
  if (!ts.isImportDeclaration(s) || !s.importClause) return;
  const c = s.importClause;
  if (c.name) importSyms.set(checker.getSymbolAtLocation(c.name), i);
  if (c.namedBindings && ts.isNamespaceImport(c.namedBindings)) importSyms.set(checker.getSymbolAtLocation(c.namedBindings.name), i);
  if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const el of c.namedBindings.elements) importSyms.set(checker.getSymbolAtLocation(el.name), i);
});

const reads = bstmts.map(() => ({ bag: new Set(), props: new Set(), imports: new Set(), forward: new Set() }));
const visit = (n, i) => {
  if (ts.isIdentifier(n)) {
    let sy = checker.getSymbolAtLocation(n);
    if (!sy && n.parent && ts.isShorthandPropertyAssignment(n.parent)) sy = checker.getShorthandAssignmentValueSymbol(n.parent);
    if (sy) {
      if (declOwner.has(sy)) { const j = declOwner.get(sy); if (j !== i) { if (j < i) reads[i].bag.add(n.text); else reads[i].forward.add(n.text + "@" + j); } }
      else if (propSyms.has(sy)) reads[i].props.add(propSyms.get(sy));
      else if (importSyms.has(sy) && !ts.isImportSpecifier(n.parent) && !ts.isImportClause(n.parent) && !ts.isNamespaceImport(n.parent)) reads[i].imports.add(sy);
    }
  }
  ts.forEachChild(n, (c) => visit(c, i));
};
bstmts.forEach((s, i) => visit(s, i));

if (mode === "analyze") {
  bstmts.forEach((s, i) => {
    const a = lineOf(s.getStart(sf)), b = lineOf(s.getEnd());
    console.log([i, `${a}-${b}`, declaredNames[i].join(",") || ("<" + ts.SyntaxKind[s.kind] + ">" + text.slice(s.getStart(sf), s.getStart(sf) + 40).replace(/\s+/g, " ")), "bag:" + [...reads[i].bag].length, "props:" + [...reads[i].props].join(","), reads[i].forward.size ? "FWD:" + [...reads[i].forward].join(",") : ""].join("\t"));
  });
  console.log("props:", propNames.join(","));
  process.exit(0);
}

const fullStart = (i) => bstmts[i].getFullStart();
const hooks = plan.hooks.map((h, k) => ({ ...h, from: k === 0 ? 0 : plan.hooks[k - 1].upto, to: h.upto === undefined ? retIdx : h.upto }));
hooks[hooks.length - 1].to = tailFrom;
const hasJsx = (a, b) => { let f = false; const v = (n) => { if (ts.isJsxElement(n) || ts.isJsxSelfClosingElement(n) || ts.isJsxFragment(n)) f = true; if (!f) ts.forEachChild(n, v); }; for (let i = a; i < b; i++) v(bstmts[i]); return f; };
const hookFileExt = hooks.map((h) => (hasJsx(h.from, h.to) ? ".tsx" : ".ts"));
const hookOfStmt = new Array(bstmts.length).fill(-1);
hooks.forEach((h, k) => { for (let i = h.from; i < h.to; i++) hookOfStmt[i] = k; });
const problems = [];
const usedLater = hooks.map(() => new Set());
const declaredIn = hooks.map((h) => { const s = new Set(); for (let i = h.from; i < h.to; i++) declaredNames[i].forEach((n) => s.add(n)); return s; });
const nameHook = new Map(); declaredIn.forEach((s, k) => s.forEach((n) => nameHook.set(n, k)));
const hookReads = hooks.map(() => ({ bag: new Set(), props: new Set(), imports: new Set() }));
const retReads = { bag: new Set(), props: new Set(), imports: new Set(), forward: new Set() };
for (let i = tailFrom; i <= retIdx; i++) for (const key of ["bag", "props", "imports", "forward"]) reads[i][key].forEach((x) => retReads[key].add(x));
for (let i = 0; i <= retIdx; i++) {
  const k = i >= tailFrom ? hooks.length : hookOfStmt[i];
  for (const n of reads[i].bag) {
    const hk = nameHook.get(n);
    if (hk === undefined) problems.push(`stmt ${i}: unresolved ${n}`);
    else if (hk !== k) { usedLater[hk].add(n); if (k < hooks.length) hookReads[k].bag.add(n); }
  }
  for (const f of reads[i].forward) { const [n, j] = f.split("@"); if (hookOfStmt[+j] !== k) problems.push(`stmt ${i} (hook ${k}) forward-reads ${n} declared in stmt ${j} (hook ${hookOfStmt[+j]})`); }
  if (k < hooks.length) { reads[i].props.forEach((p) => hookReads[k].props.add(p)); reads[i].imports.forEach((p) => hookReads[k].imports.add(p)); }
}
const retProps = [...retReads.props];
const retBag = [...retReads.bag];
const retImports = retReads.imports;
const allProps = new Set(); hookReads.forEach((r) => r.props.forEach((p) => allProps.add(p))); retProps.forEach((p) => allProps.add(p));

const relFrom = (fromDir, target) => { let r = path.relative(fromDir, target).split(path.sep).join("/"); if (!r.startsWith(".")) r = "./" + r; return r; };
function importLine(declIdx, used, toDir) {
  const s = sf.statements[declIdx];
  const c = s.importClause;
  const specText = s.moduleSpecifier.text;
  const newSpec = specText.startsWith(".") ? relFrom(toDir, path.resolve(dir, specText)) : specText;
  const q = s.moduleSpecifier.getText(sf)[0];
  let def = null, ns = null; const named = [];
  if (c.name && used.has(checker.getSymbolAtLocation(c.name))) def = c.name.text;
  if (c.namedBindings && ts.isNamespaceImport(c.namedBindings) && used.has(checker.getSymbolAtLocation(c.namedBindings.name))) ns = c.namedBindings.name.text;
  if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const el of c.namedBindings.elements) if (used.has(checker.getSymbolAtLocation(el.name))) named.push(el.getText(sf));
  if (!def && !ns && !named.length) return null;
  const total = (c.name ? 1 : 0) + (c.namedBindings ? (ts.isNamespaceImport(c.namedBindings) ? 1 : c.namedBindings.elements.length) : 0);
  const keptN = (def ? 1 : 0) + (ns ? 1 : 0) + named.length;
  const bits = []; if (def) bits.push(def); if (ns) bits.push(`* as ${ns}`); if (named.length) bits.push(`{ ${named.join(", ")} }`);
  return { all: total === keptN && newSpec === specText, spec: q + newSpec + q, line: `import ${c.isTypeOnly ? "type " : ""}${bits.join(", ")} from ${q}${newSpec}${q};`, decl: declIdx };
}
function importsFor(usedSyms, toDir, extra) {
  const byDecl = new Map();
  for (const sy of usedSyms) byDecl.set(importSyms.get(sy), (byDecl.get(importSyms.get(sy)) || new Set()).add(sy));
  const lines = [];
  for (const [d, set] of [...byDecl].sort((a, b) => a[0] - b[0])) { const l = importLine(d, set, toDir); if (l) lines.push(l.line); }
  return lines;
}

const propsTypeImportIdx = (() => { for (const [sy, i] of importSyms) if (sy.name === plan.propsType) return i; return -1; })();
const typeSym = [...importSyms.keys()].find((sy) => sy.name === plan.propsType);

const bodyText = (a, b) => text.slice(fullStart(a), bstmts[b - 1].getEnd());
const outputs = [];
const typeName = (k) => plan.coreType + (k + 1);
hooks.forEach((h, k) => {
  const hdir = path.join(dir, h.dir);
  const out = path.join(hdir, "🟦️" + hookFileExt[k]);
  const names = [...usedLater[k]].sort();
  const reading = [...hookReads[k].bag].sort();
  const lines = [];
  const importSet = new Set(hookReads[k].imports);
  const extraImports = [];
  if (k === 0) { if (typeSym) importSet.add(typeSym); }
  const importLines = importsFor(importSet, hdir);
  const prevImport = k > 0 ? `import type { ${typeName(k - 1)} } from "${relFrom(hdir, path.join(dir, hooks[k - 1].dir, "🟦️" + hookFileExt[k - 1]))}";` : null;
  const doc = h.doc;
  const head = [doc, ...importLines, ...(prevImport ? [prevImport] : [])].join("\n");
  const propsList = [...hookReads[k].props].sort();
  let sig, prelude = "";
  if (k === 0) {
    const outside = new Set([...retProps]);
    hookReads.forEach((r, kk) => { if (kk > 0) r.props.forEach((x) => outside.add(x)); });
    const inside = new Set([...hookReads[0].props, ...outside]);
    const elems = param.name.elements.filter((el) => inside.has(el.name.text)).map((el) => el.getText(sf));
    sig = `export function ${h.name}({ ${elems.join(", ")} }: ${plan.propsType})`;
    outside.forEach((x) => names.push(x));
    names.sort();
  } else {
    sig = `export function ${h.name}(core: ${typeName(k - 1)})`;
    const need = [...new Set([...reading, ...propsList])].sort();
    prelude = need.length ? `  const { ${need.join(", ")} } = core;\n` : "";
  }
  const ret = names.length ? `  return { ${names.join(", ")} };\n` : "";
  const code = `// #region ${plan.region}\n${h.docFn ? h.docFn + "\n" : ""}${sig} {\n${prelude}${bodyText(h.from, h.to).replace(/^\n+/, "")}\n${ret}}\n\nexport type ${typeName(k)} = ${k > 0 ? typeName(k - 1) + " & " : ""}ReturnType<typeof ${h.name}>;\n// #endregion ${plan.region}\n`;
  outputs.push({ path: out, text: `${head}\n\n${code}`, lines: (head + code).split("\n").length, hook: h.name, reads: reading.length + propsList.length, outs: names.length });
});

const lastK = hooks.length - 1;
const parentNeeds = [...new Set([...retBag])].sort();
const parentPropNeeds = retProps.sort();
const parentImportSyms = new Set(retImports);
const sigVisit = (n) => {
  if (ts.isIdentifier(n)) { const sy = checker.getSymbolAtLocation(n); if (sy && importSyms.has(sy)) parentImportSyms.add(sy); }
  ts.forEachChild(n, sigVisit);
};
if (fnDecl.type) sigVisit(fnDecl.type);
parameterTypeVisit: { if (param.type) sigVisit(param.type); }
sf.statements.forEach((s) => { if (s !== fnDecl && !ts.isImportDeclaration(s)) sigVisit(s); });
const hookCalls = hooks.map((h, k) => `  Object.assign(core, ${h.name}(${k === 0 ? "props" : "core"}));`).join("\n");
const hookImports = hooks.map((h, k) => `import { ${h.name}, type ${typeName(k)} } from "./${h.dir}/🟦️${hookFileExt[k]}";`);
const retText = text.slice(fullStart(tailFrom), bstmts[retIdx].getEnd());
const destruct = [...new Set([...parentNeeds, ...parentPropNeeds])].sort();
const fnLead = text.slice(fnDecl.getStart(sf, true), fnDecl.getStart(sf, false));
const newFn = `${fnLead}export function ${plan.fn}(props: ${plan.propsType})${fnDecl.type ? ": " + fnDecl.type.getText(sf) : ""} {\n  const core = {} as ${typeName(lastK)};\n${hookCalls}\n  const { ${destruct.join(", ")} } = core;\n${retText.replace(/^\n+/, "\n")}\n}`;

const importStmts = sf.statements.filter((s) => ts.isImportDeclaration(s));
const lastImportEnd = importStmts[importStmts.length - 1].getEnd();
const fnStartFull = fnDecl.getStart(sf, true);
let parentHead = "";
for (const s of importStmts) {
  parentHead += text.slice(s.getFullStart(), s.getStart(sf, false));
  const i = sf.statements.indexOf(s);
  const l = importLine(i, parentImportSyms, dir);
  if (l) {
    const raw = text.slice(s.getStart(sf, false), s.getEnd());
    parentHead += l.all ? raw : l.line;
  } else {
    const dropped = text.slice(s.getStart(sf, false), s.getEnd());
    void dropped;
  }
}
parentHead = parentHead.replace(/\n{3,}/g, "\n\n");
const regionBefore = text.slice(lastImportEnd, fnStartFull);
const tailAfterFn = text.slice(fnDecl.getEnd());
const parentOut = `${parentHead}\n${hookImports.join("\n")}${regionBefore}${newFn}${tailAfterFn}`;
outputs.push({ path: file, text: parentOut, lines: parentOut.split("\n").length, hook: "<parent>" });

console.log(JSON.stringify({ problems, hooks: outputs.map((o) => ({ path: o.path.slice(-70), lines: o.lines, hook: o.hook, reads: o.reads, outs: o.outs })), allProps: [...allProps].length, propNames: propNames.length, retBag: retBag.length, retProps: retProps.length }, null, 1));
if (APPLY && !problems.length) {
  for (const o of outputs) { fs.mkdirSync(path.dirname(o.path), { recursive: true }); fs.writeFileSync(o.path, o.text); }
  console.log("APPLIED");
}
