// usage: node split-module.cjs <spec.json> [APPLY]
// Moves a trailing run of top-level statements of one TypeScript module into a new child module in a sub-folder.
// Dry run prints the plan; APPLY writes the child, the shrunken parent and the re-pointed importers.
const fs = require("fs");
const path = require("path");
const { analyze, ts } = require("./units.cjs");

const spec = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const APPLY = process.argv[3] === "APPLY";
const file = spec.file;
const dir = path.dirname(file);
const childFile = path.join(dir, spec.childDir, "🟦️.ts");
const { sf, text, stmts, edges, checker } = analyze(file);
const from = spec.from, to = spec.to;
if (to !== stmts.length - 1) throw new Error("only trailing runs are supported");
const moved = new Set(); for (let i = from; i <= to; i++) moved.add(i);
const regionRe = /^[ \t]*\/\/\s*#(end)?region\s+(.*?)\s*$/;
function regionBalance(t) {
  const stack = []; const unmatchedEnds = [];
  for (const line of t.split("\n")) {
    const m = line.match(regionRe);
    if (!m) continue;
    if (!m[1]) stack.push(m[2]);
    else if (stack.length && stack[stack.length - 1] === m[2]) stack.pop();
    else unmatchedEnds.push(m[2]);
  }
  return { open: stack, unmatchedEnds };
}
let blockStart = sf.statements[from - 1].getEnd();
{
  let cursor = blockStart;
  let open = regionBalance(text.slice(0, blockStart)).open;
  for (;;) {
    const nl = text.indexOf("\n", cursor);
    if (nl < 0) break;
    let scan = nl + 1, found = false;
    while (scan <= text.length) {
      let e = text.indexOf("\n", scan); if (e < 0) e = text.length;
      const line = text.slice(scan, e);
      if (line.trim() === "") { scan = e + 1; if (e >= text.length) break; continue; }
      const m = line.match(regionRe);
      if (m && m[1] && open.length && open[open.length - 1] === m[2]) { open = open.slice(0, -1); cursor = e; found = true; }
      break;
    }
    if (!found) break;
  }
  blockStart = cursor;
}
const blockText = text.slice(blockStart);
const parentText0 = text.slice(0, blockStart);

function specOf(imp) { return imp.moduleSpecifier.text; }
function relFrom(base, target) {
  let r = path.relative(base, target).split(path.sep).join("/");
  if (!r.startsWith(".")) r = "./" + r;
  return r;
}
function rebase(spec0) {
  if (!spec0.startsWith(".")) return spec0;
  return relFrom(path.dirname(childFile), path.resolve(dir, spec0));
}

const importSpecIndex = new Map();
sf.statements.forEach((s, i) => {
  if (!ts.isImportDeclaration(s)) return;
  const c = s.importClause;
  if (!c) return;
  if (c.name) importSpecIndex.set(checker.getSymbolAtLocation(c.name), { decl: s, kind: "default", node: c });
  if (c.namedBindings && ts.isNamespaceImport(c.namedBindings)) importSpecIndex.set(checker.getSymbolAtLocation(c.namedBindings.name), { decl: s, kind: "ns", node: c.namedBindings });
  if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const el of c.namedBindings.elements) importSpecIndex.set(checker.getSymbolAtLocation(el.name), { decl: s, kind: "named", node: el });
});
const usedIn = (idxs) => {
  const used = new Set();
  const visit = (n) => {
    if (ts.isIdentifier(n)) {
      const sy = checker.getSymbolAtLocation(n);
      if (sy && importSpecIndex.has(sy) && !ts.isImportSpecifier(n.parent) && !ts.isImportClause(n.parent) && !ts.isNamespaceImport(n.parent)) used.add(sy);
    }
    ts.forEachChild(n, visit);
  };
  for (const i of idxs) if (!ts.isImportDeclaration(sf.statements[i])) visit(sf.statements[i]);
  return used;
};
const movedIdx = [...moved];
const restIdx = stmts.map((s, i) => i).filter((i) => !moved.has(i));
const usedChild = usedIn(movedIdx), usedRest = usedIn(restIdx);

const needFromParent = new Set();
for (const i of movedIdx) for (const j of edges[i]) if (!moved.has(j)) needFromParent.add(j);
for (const i of restIdx) for (const j of edges[i]) if (moved.has(j)) throw new Error(`parent statement ${i} depends on moved statement ${j}`);

const parentNeedNames = [...needFromParent].map((j) => stmts[j].names).flat();
const parentEdits = [];
for (const j of needFromParent) if (!stmts[j].exported) parentEdits.push({ pos: sf.statements[j].getStart(sf), ins: "export " });

function importLine(decl, usedSet) {
  const c = decl.importClause;
  const parts = [];
  let def = null, ns = null; const named = [];
  if (c.name && usedSet.has(checker.getSymbolAtLocation(c.name))) def = c.name.text;
  if (c.namedBindings && ts.isNamespaceImport(c.namedBindings) && usedSet.has(checker.getSymbolAtLocation(c.namedBindings.name))) ns = c.namedBindings.name.text;
  if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const el of c.namedBindings.elements) if (usedSet.has(checker.getSymbolAtLocation(el.name))) named.push(el.getText(sf));
  if (!def && !ns && !named.length) return null;
  const all = c.isTypeOnly ? "type " : "";
  const bits = [];
  if (def) bits.push(def);
  if (ns) bits.push(`* as ${ns}`);
  if (named.length) bits.push(`{ ${named.join(", ")} }`);
  return { def, ns, named, typeOnly: c.isTypeOnly, bits, all };
}

const childImports = [];
const parentImportRewrites = [];
for (const s of sf.statements) {
  if (!ts.isImportDeclaration(s) || !s.importClause) continue;
  const spec0 = specOf(s);
  const cl = importLine(s, usedChild);
  if (cl) childImports.push(`import ${cl.all}${cl.bits.join(", ")} from "${rebase(spec0)}";`);
  const droppedByMove = new Set([...usedChild].filter((sy) => !usedRest.has(sy)));
  const keep = new Set([...importSpecIndex.keys()].filter((sy) => importSpecIndex.get(sy).decl === s && !droppedByMove.has(sy)));
  const kl = importLine(s, keep);
  const orig = s.getText(sf);
  if (!kl) parentImportRewrites.push({ start: s.getStart(sf), end: s.getEnd(), text: null });
  else {
    const line = `import ${kl.all}${kl.bits.join(", ")} from ${s.moduleSpecifier.getText(sf)};`;
    if (line !== orig) parentImportRewrites.push({ start: s.getStart(sf), end: s.getEnd(), text: line });
  }
}
const parentRel = relFrom(path.dirname(childFile), file);
if (parentNeedNames.length) childImports.push(`import { ${parentNeedNames.join(", ")} } from "${parentRel}";`);

const pb = regionBalance(parentText0), cb = regionBalance(blockText);
if (cb.open.length) throw new Error("child leaves regions open: " + cb.open.join(","));

let parentNew = parentText0;
const edits = [...parentEdits.map((e) => ({ start: e.pos, end: e.pos, text: e.ins })), ...parentImportRewrites].sort((a, b) => b.start - a.start);
for (const e of edits) {
  if (e.text === null) {
    let end = e.end; if (parentNew[end] === "\n") end++;
    parentNew = parentNew.slice(0, e.start) + parentNew.slice(end);
  } else parentNew = parentNew.slice(0, e.start) + e.text + parentNew.slice(e.end);
}
parentNew = parentNew.replace(/\s+$/, "") + "\n";
for (const name of pb.open.reverse()) parentNew += `//#endregion ${name}\n`;

const regionPrefix = cb.unmatchedEnds.reverse().map((n) => `//#region ${n}\n`).join("");
const childText = `${spec.doc}\n${childImports.join("\n")}\n\n${regionPrefix}${blockText.replace(/^\s+/, "")}`.replace(/\s+$/, "") + "\n";

const movedNames = new Set(); for (const i of movedIdx) if (stmts[i].exported) for (const n of stmts[i].names) movedNames.add(n);
const lineCount = (t) => t.split("\n").length - 1;
console.log(JSON.stringify({ parentLines: lineCount(parentNew), childLines: lineCount(childText), movedExports: [...movedNames], parentNeeds: parentNeedNames, parentAutoExported: parentEdits.length, regionsClosedInParent: pb.open, regionsOpenedInChild: cb.unmatchedEnds }, null, 1));

function rewriteImporter(p) {
  const t = fs.readFileSync(p, "utf8");
  const s2 = ts.createSourceFile(p, t, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const outs = [];
  for (const st of s2.statements) {
    if (!(ts.isImportDeclaration(st) || ts.isExportDeclaration(st)) || !st.moduleSpecifier || !ts.isStringLiteral(st.moduleSpecifier)) continue;
    if (!st.moduleSpecifier.text.startsWith(".") || path.resolve(path.dirname(p), st.moduleSpecifier.text) !== file) continue;
    const clause = ts.isImportDeclaration(st) ? st.importClause && st.importClause.namedBindings : st.exportClause;
    if (!clause || !(ts.isNamedImports(clause) || ts.isNamedExports(clause))) throw new Error("unsupported import form in " + p);
    const els = [...clause.elements];
    const mv = els.filter((e) => movedNames.has((e.propertyName ?? e.name).text));
    if (!mv.length) continue;
    const kp = els.filter((e) => !mv.includes(e));
    const q = st.moduleSpecifier.getText(s2)[0];
    const newSpec = relFrom(path.dirname(p), childFile);
    const head = ts.isImportDeclaration(st) ? "import " + (st.importClause.isTypeOnly ? "type " : "") : "export " + (st.isTypeOnly ? "type " : "");
    const mk = (list, sp) => `${head}{ ${list.map((e) => e.getText(s2)).join(", ")} } from ${q}${sp}${q};`;
    let rep = mk(mv, newSpec);
    if (kp.length) rep = mk(kp, st.moduleSpecifier.text) + "\n" + rep;
    outs.push({ start: st.getStart(s2), end: st.getEnd(), text: rep });
  }
  if (!outs.length) return null;
  let r = t;
  for (const e of outs.sort((a, b) => b.start - a.start)) r = r.slice(0, e.start) + e.text + r.slice(e.end);
  return r;
}
const importerWrites = [];
for (const imp of spec.importers || []) {
  const r = rewriteImporter(imp);
  if (r === null) { console.log("no change needed:", imp); continue; }
  importerWrites.push([imp, r]);
}
console.log("importers rewritten:", importerWrites.length);
if (APPLY) {
  if (fs.existsSync(childFile)) throw new Error("child exists: " + childFile);
  fs.mkdirSync(path.dirname(childFile), { recursive: true });
  fs.writeFileSync(childFile, childText);
  fs.writeFileSync(file, parentNew);
  for (const [p, r] of importerWrites) fs.writeFileSync(p, r);
  console.log("APPLIED");
}
