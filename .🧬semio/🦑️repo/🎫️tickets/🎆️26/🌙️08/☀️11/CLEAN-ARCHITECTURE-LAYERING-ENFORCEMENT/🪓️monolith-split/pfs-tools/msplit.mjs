// usage: node msplit.mjs <spec.json> [APPLY]
// Splits one TypeScript module along top-level statements into child modules (<folder>/🟦️.ts next to it).
// spec: { file, label, children: [{ folder, doc, stmts: ["19-117", ...] }], parentDoc?, sha1? }
// stmts ranges are START LINES of top-level statements (getStart, comments excluded). Unassigned statements stay in the parent.
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import fs from "node:fs";
import path from "node:path";
import crypto from "node:crypto";

const REPO = "/Users/ueli/Documents/semio";
const specPath = process.argv[2];
const APPLY = process.argv[3] === "APPLY";
const spec = JSON.parse(fs.readFileSync(specPath, "utf8"));
const file = path.resolve(REPO, spec.file);
const dir = path.dirname(file);
const text = fs.readFileSync(file, "utf8");
const sha = crypto.createHash("sha1").update(text).digest("hex");
if (spec.sha1 && spec.sha1 !== sha) throw new Error(`source changed: pinned ${spec.sha1}, live ${sha}`);

const kind = file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS;
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, kind);
const host = ts.createCompilerHost({});
host.getSourceFile = (n) => (n === file ? sf : undefined);
host.fileExists = (n) => n === file;
host.readFile = (n) => (n === file ? text : undefined);
const program = ts.createProgram([file], { noResolve: true, noLib: true, target: ts.ScriptTarget.Latest, jsx: ts.JsxEmit.ReactJSX }, host);
const checker = program.getTypeChecker();
const lineOf = (pos) => sf.getLineAndCharacterOfPosition(pos).line + 1;

const stmts = [...sf.statements];
const isImport = (s) => ts.isImportDeclaration(s);
const regionRe = /^[ \t]*\/\/\s*#(end)?region\s+(.*?)\s*$/;
const isBlank = (l) => l.trim() === "";
const isEndRegion = (l) => { const m = l.match(regionRe); return !!(m && m[1]); };

const firstImport = stmts.find(isImport);
const preamble = firstImport ? text.slice(0, firstImport.getStart(sf)) : "";

const units = [];
for (let k = 0; k < stmts.length; k++) {
  const s = stmts[k];
  const prevEnd = k === 0 ? 0 : stmts[k - 1].getEnd();
  const gap = text.slice(prevEnd, s.getStart(sf));
  let cut = 0;
  if (k > 0) {
    const nl = gap.indexOf("\n");
    if (nl < 0) cut = gap.length === 0 ? 0 : gap.trim() === "" ? gap.length : 0;
    else {
      cut = nl + 1;
      for (;;) {
        const e = gap.indexOf("\n", cut);
        if (e < 0) break;
        const line = gap.slice(cut, e);
        if (isBlank(line) || isEndRegion(line)) cut = e + 1;
        else break;
      }
    }
  } else cut = 0;
  units.push({ k, stmt: s, gapPrev: gap.slice(0, cut), pre: gap.slice(cut), body: text.slice(s.getStart(sf), s.getEnd()), post: "" });
}
for (let k = 0; k < units.length; k++) {
  if (k + 1 < units.length) units[k].post = units[k + 1].gapPrev;
  else units[k].post = text.slice(stmts[k].getEnd());
}
if (!firstImport) {
  units[0].pre = text.slice(0, stmts[0].getStart(sf));
}
const importEndGap = firstImport ? (() => { const lastImp = [...stmts].filter(isImport).pop(); const idx = stmts.indexOf(lastImp); return idx + 1 < units.length ? units[idx + 1].gapPrev : ""; })() : "";
if (firstImport) {
  const lastImpIdx = stmts.indexOf([...stmts].filter(isImport).pop());
  if (lastImpIdx + 1 < units.length) units[lastImpIdx + 1].gapPrev = units[lastImpIdx + 1].gapPrev;
}

const forcedHead = [];
const forcedTail = [];
{
  const ranges = (spec.parentHeadLines ?? []).map((r) => ["head", ...String(r).split("-").map(Number)]).concat((spec.parentTailLines ?? []).map((r) => ["tail", ...String(r).split("-").map(Number)]));
  const lineStart = [0];
  for (let i = 0; i < text.length; i++) if (text[i] === "\n") lineStart.push(i + 1);
  const take = (piece, offset) => {
    const lines = piece.split("\n");
    let abs = offset;
    const keep = [];
    lines.forEach((ln, idx) => {
      const no = lineOf(abs);
      const hit = ranges.find(([, a0, b0]) => no >= a0 && no <= (b0 ?? a0) && idx < lines.length - 1);
      if (hit) (hit[0] === "head" ? forcedHead : forcedTail).push({ no, text: ln });
      else keep.push(ln);
      abs += ln.length + 1;
    });
    return keep.join("\n");
  };
  for (const u of units) {
    if (isImport(u.stmt)) continue;
    const preOff = u.stmt.getStart(sf) - u.pre.length;
    u.pre = take(u.pre, preOff);
    const postOff = u.stmt.getEnd();
    u.post = take(u.post, postOff);
  }
  forcedHead.sort((a, b) => a.no - b.no);
  forcedTail.sort((a, b) => a.no - b.no);
}
const nonImport = units.filter((u) => !isImport(u.stmt));
const parseRanges = (list) => list.map((r) => { const [a, b] = String(r).split("-").map(Number); return [a, b ?? a]; });
const clusterOf = new Map();
const childByName = new Map();
for (const c of spec.children) {
  childByName.set(c.folder, c);
  const ranges = parseRanges(c.stmts ?? []);
  const wanted = new Set(c.names ?? []);
  for (const u of nonImport) {
    const start = lineOf(u.stmt.getStart(sf));
    const declared = (ts.isVariableStatement(u.stmt) ? u.stmt.declarationList.declarations.flatMap((d) => (ts.isIdentifier(d.name) ? [d.name.text] : [])) : u.stmt.name ? [u.stmt.name.text] : []);
    if (ranges.some(([a, b]) => start >= a && start <= b) || declared.some((n) => wanted.has(n))) {
      if (clusterOf.has(u.k)) throw new Error(`statement at line ${start} assigned twice (${clusterOf.get(u.k)} and ${c.folder})`);
      clusterOf.set(u.k, c.folder);
    }
  }
}
const PARENT = "<parent>";
for (const u of nonImport) if (!clusterOf.has(u.k)) clusterOf.set(u.k, PARENT);

const declNames = (s) => {
  if (ts.isVariableStatement(s)) return s.declarationList.declarations.flatMap((d) => (ts.isIdentifier(d.name) ? [d.name] : []));
  if ((ts.isFunctionDeclaration(s) || ts.isClassDeclaration(s) || ts.isInterfaceDeclaration(s) || ts.isTypeAliasDeclaration(s) || ts.isEnumDeclaration(s)) && s.name) return [s.name];
  return [];
};
const owner = new Map();
const nameOfSym = new Map();
const typeOnlySym = new Map();
for (const u of nonImport) {
  for (const id of declNames(u.stmt)) {
    const sy = checker.getSymbolAtLocation(id);
    if (!sy) continue;
    owner.set(sy, u.k);
    nameOfSym.set(sy, id.text);
    typeOnlySym.set(sy, ts.isInterfaceDeclaration(u.stmt) || ts.isTypeAliasDeclaration(u.stmt));
  }
}
const importSym = new Map();
for (const s of stmts) {
  if (!isImport(s) || !s.importClause) continue;
  const c = s.importClause;
  if (c.name) importSym.set(checker.getSymbolAtLocation(c.name), { decl: s, kind: "default", name: c.name.text, typeOnly: c.isTypeOnly });
  if (c.namedBindings && ts.isNamespaceImport(c.namedBindings)) importSym.set(checker.getSymbolAtLocation(c.namedBindings.name), { decl: s, kind: "ns", name: c.namedBindings.name.text, typeOnly: c.isTypeOnly });
  if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const el of c.namedBindings.elements) importSym.set(checker.getSymbolAtLocation(el.name), { decl: s, kind: "named", el, name: el.name.text, typeOnly: c.isTypeOnly || el.isTypeOnly });
}

const edges = new Map();
const usedImports = new Map();
const warnings = [];
const mutated = [];
for (const u of nonImport) {
  const cl = clusterOf.get(u.k);
  if (!edges.has(cl)) edges.set(cl, new Map());
  if (!usedImports.has(cl)) usedImports.set(cl, new Set());
  const visit = (n) => {
    if (ts.isIdentifier(n)) {
      let sy = checker.getSymbolAtLocation(n);
      if (!sy && n.parent && ts.isShorthandPropertyAssignment(n.parent)) sy = checker.getShorthandAssignmentValueSymbol(n.parent);
      if (sy) {
        if (importSym.has(sy) && !ts.isImportSpecifier(n.parent)) usedImports.get(cl).add(sy);
        if (owner.has(sy)) {
          const j = owner.get(sy);
          if (j !== u.k) {
            const tcl = clusterOf.get(j);
            if (tcl !== cl) {
              const m = edges.get(cl);
              if (!m.has(tcl)) m.set(tcl, new Map());
              m.get(tcl).set(sy, { name: nameOfSym.get(sy), stmt: j, from: u.k });
              const p = n.parent;
              const assigned = (ts.isBinaryExpression(p) && p.left === n && p.operatorToken.kind >= ts.SyntaxKind.FirstAssignment && p.operatorToken.kind <= ts.SyntaxKind.LastAssignment) || (ts.isPrefixUnaryExpression(p) && (p.operator === ts.SyntaxKind.PlusPlusToken || p.operator === ts.SyntaxKind.MinusMinusToken)) || (ts.isPostfixUnaryExpression(p));
              if (assigned) mutated.push(`${nameOfSym.get(sy)} assigned in ${cl} but owned by ${tcl}`);
            }
          }
        }
      }
    }
    if (n.kind === ts.SyntaxKind.MetaProperty) warnings.push(`line ${lineOf(n.getStart(sf))}: ${n.getText(sf)} in ${cl}`);
    if (ts.isIdentifier(n) && (n.text === "__dirname" || n.text === "__filename")) warnings.push(`line ${lineOf(n.getStart(sf))}: ${n.text} in ${cl}`);
    ts.forEachChild(n, visit);
  };
  visit(u.stmt);
}

const clusters = [...new Set([...spec.children.map((c) => c.folder), PARENT])];
const graph = new Map(clusters.map((c) => [c, new Set()]));
for (const [a, m] of edges) for (const b of m.keys()) graph.get(a).add(b);
{
  let index = 0; const st = [], on = new Set(), idx = new Map(), low = new Map(), sccs = [];
  const sc = (v) => { idx.set(v, index); low.set(v, index); index++; st.push(v); on.add(v);
    for (const w of graph.get(v)) { if (!idx.has(w)) { sc(w); low.set(v, Math.min(low.get(v), low.get(w))); } else if (on.has(w)) low.set(v, Math.min(low.get(v), idx.get(w))); }
    if (low.get(v) === idx.get(v)) { const c = []; let w; do { w = st.pop(); on.delete(w); c.push(w); } while (w !== v); sccs.push(c); } };
  for (const c of clusters) if (!idx.has(c)) sc(c);
  const bad = sccs.filter((c) => c.length > 1);
  if (bad.length) {
    console.log("CLUSTER CYCLES:");
    for (const c of bad) {
      console.log("  ", c.join(" <-> "));
      for (const a of c) for (const b of c) if (a !== b && edges.get(a)?.has(b)) console.log(`     ${a} -> ${b}: ${[...edges.get(a).get(b).values()].map((e) => `${e.name}@L${lineOf(stmts[e.stmt].getStart(sf))}(from L${lineOf(stmts[e.from].getStart(sf))})`).join(", ")}`);
    }
    if (APPLY) throw new Error("cluster cycles");
  }
}
if (mutated.length) { console.log("CROSS-CLUSTER MUTATION:", mutated); if (APPLY) throw new Error("mutation"); }

const needExport = new Set();
for (const m of edges.values()) for (const t of m.values()) for (const e of t.values()) needExport.add(e.stmt);

const relFrom = (base, target) => { let r = path.relative(base, target).split(path.sep).join("/"); if (!r.startsWith(".")) r = "./" + r; return r; };
const childRoot = spec.childRoot ? path.resolve(REPO, spec.childRoot) : dir;
const folderFile = (c) => (c === PARENT ? file : path.join(childRoot, c, "🟦️" + path.extname(file)));
const rebaseFor = (cl, spec0) => (spec0.startsWith(".") ? relFrom(path.dirname(folderFile(cl)), path.resolve(dir, spec0)) : spec0);

function unitText(u, cl) {
  const edits = [];
  if (needExport.has(u.k) && !(ts.getCombinedModifierFlags(u.stmt) & ts.ModifierFlags.Export) && cl !== undefined) {
    edits.push({ pos: u.stmt.getStart(sf) - u.stmt.getStart(sf), ins: "export " });
  }
  let body = u.body;
  const base = u.stmt.getStart(sf);
  const rewrites = [];
  const visit = (n) => {
    if (cl !== PARENT) {
      if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0]) && n.arguments[0].text.startsWith(".")) rewrites.push({ lit: n.arguments[0] });
      if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal) && n.argument.literal.text.startsWith(".")) rewrites.push({ lit: n.argument.literal });
      if (ts.isExportDeclaration(n) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier) && n.moduleSpecifier.text.startsWith(".")) rewrites.push({ lit: n.moduleSpecifier });
    }
    ts.forEachChild(n, visit);
  };
  visit(u.stmt);
  const all = [...rewrites.map((r) => ({ start: r.lit.getStart(sf) + 1 - base, end: r.lit.getEnd() - 1 - base, text: rebaseFor(cl, r.lit.text) })), ...edits.map((e) => ({ start: 0, end: 0, text: e.ins }))].sort((a, b) => b.start - a.start);
  for (const e of all) body = body.slice(0, e.start) + e.text + body.slice(e.end);
  return u.pre + body + u.post;
}

function balance(t) {
  const stack = []; const unmatchedEnds = [];
  for (const line of t.split("\n")) {
    const m = line.match(regionRe);
    if (!m) continue;
    if (!m[1]) stack.push(m[2]);
    else if (stack.length && stack[stack.length - 1] === m[2]) stack.pop();
    else unmatchedEnds.push(m[2]);
  }
  return { open: stack, unmatchedEnds: unmatchedEnds.filter((x) => x !== null) };
}

function importLines(cl) {
  const used = usedImports.get(cl) ?? new Set();
  const out = [];
  for (const s of stmts) {
    if (!isImport(s)) continue;
    const c = s.importClause;
    const spec0 = s.moduleSpecifier.text;
    if (!c) { if (cl === PARENT) out.push(`import "${spec0}";`); continue; }
    let def = null, ns = null; const named = [];
    if (c.name && used.has(checker.getSymbolAtLocation(c.name))) def = c.name.text;
    if (c.namedBindings && ts.isNamespaceImport(c.namedBindings) && used.has(checker.getSymbolAtLocation(c.namedBindings.name))) ns = c.namedBindings.name.text;
    if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const el of c.namedBindings.elements) if (used.has(checker.getSymbolAtLocation(el.name))) named.push(el.getText(sf));
    if (!def && !ns && !named.length) continue;
    const bits = [];
    if (def) bits.push(def);
    if (ns) bits.push(`* as ${ns}`);
    if (named.length) bits.push(`{ ${named.join(", ")} }`);
    out.push(`import ${c.isTypeOnly ? "type " : ""}${bits.join(", ")} from "${rebaseFor(cl, spec0)}";`);
  }
  const m = edges.get(cl);
  if (m) {
    const order = new Map(clusters.map((c, i) => [c, i]));
    for (const tcl of [...m.keys()].sort((a, b) => order.get(a) - order.get(b))) {
      const items = [...m.get(tcl).values()].sort((a, b) => a.stmt - b.stmt || a.name.localeCompare(b.name));
      const syms = [...m.get(tcl).entries()].sort((a, b) => a[1].stmt - b[1].stmt || a[1].name.localeCompare(b[1].name));
      const bits = syms.map(([sy, e]) => (typeOnlySym.get(sy) ? `type ${e.name}` : e.name));
      const allType = syms.every(([sy]) => typeOnlySym.get(sy));
      const target = relFrom(path.dirname(folderFile(cl)), folderFile(tcl));
      if (allType) out.push(`import type { ${syms.map(([, e]) => e.name).join(", ")} } from "${target}";`);
      else out.push(`import { ${bits.join(", ")} } from "${target}";`);
      void items;
    }
  }
  return out;
}

function assemble(cl, docstring) {
  const mine = nonImport.filter((u) => clusterOf.get(u.k) === cl);
  let body = "";
  for (const u of mine) {
    const t = unitText(u, cl);
    if (body && !/\n[ \t]*\n$/.test(body)) body += "\n";
    body += t;
  }
  body = body.replace(/^\s+/, "").replace(/\s+$/, "") + "\n";
  if (cl === PARENT && forcedTail.length) body += forcedTail.map((l) => l.text).join("\n") + "\n";
  const imports = importLines(cl).join("\n");
  let head;
  if (cl === PARENT) head = (preamble + imports + (imports ? "\n" : "") + importEndGap + (forcedHead.length ? forcedHead.map((l) => l.text).join("\n") + "\n" : "")).replace(/\s+$/, "\n") + "\n";
  else head = `${docstring}\n${imports}${imports ? "\n\n" : "\n"}`;
  const bal = balance(cl === PARENT ? head + body : body);
  const prefix = bal.unmatchedEnds.slice().reverse().map((n) => `//#region ${n}\n`).join("");
  const suffix = bal.open.slice().reverse().map((n) => `//#endregion ${n}\n`).join("");
  return { text: head + prefix + body + suffix, synthetic: bal.unmatchedEnds.length + bal.open.length };
}

const outputs = new Map();
for (const c of spec.children) outputs.set(c.folder, assemble(c.folder, c.doc));
const parentOut = assemble(PARENT);
outputs.set(PARENT, parentOut);

const pathBytes = (p) => Buffer.byteLength(path.relative(REPO, p));
console.log(`source ${path.relative(REPO, file)} sha1=${sha} lines=${text.split("\n").length - 1}`);
const rows = [];
for (const [cl, o] of outputs) rows.push(`${cl === PARENT ? "(parent)" : cl}\tlines=${o.text.split("\n").length - 1}\tsyntheticRegions=${o.synthetic}\tpathBytes=${pathBytes(folderFile(cl))}`);
console.log(rows.join("\n"));
console.log("cluster edges:");
for (const [a, m] of edges) for (const [b, t] of m) console.log(`  ${a === PARENT ? "(parent)" : a} -> ${b === PARENT ? "(parent)" : b}: ${[...t.values()].map((e) => e.name).slice(0, 8).join(",")}${t.size > 8 ? ",…" + t.size : ""}`);
console.log(`exports added: ${[...needExport].filter((k) => !(ts.getCombinedModifierFlags(stmts[k]) & ts.ModifierFlags.Export)).length}`);
if (warnings.length) console.log("WARNINGS:\n  " + warnings.join("\n  "));

if (process.env.PRINT) { const o = outputs.get(process.env.PRINT === "parent" ? PARENT : process.env.PRINT); console.log("=====" + process.env.PRINT + "\n" + o.text); }
if (spec.deleteParent && nonImport.some((u) => clusterOf.get(u.k) === PARENT)) { console.log("PARENT STILL OWNS:", nonImport.filter((u) => clusterOf.get(u.k) === PARENT).map((u) => `L${lineOf(u.stmt.getStart(sf))}`).join(",")); if (APPLY) throw new Error("parent owns statements"); }
const mapping = {};
for (const u of nonImport) {
  const cl = clusterOf.get(u.k);
  for (const id of declNames(u.stmt)) mapping[id.text] = path.relative(REPO, folderFile(cl));
}
const mapPath = path.resolve(REPO, `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/pfs-${spec.label}-map.json`);
fs.writeFileSync(mapPath, JSON.stringify({ file: path.relative(REPO, file), sha1: sha, names: mapping, children: Object.fromEntries(spec.children.map((c) => [c.folder, path.relative(REPO, folderFile(c.folder))])) }, null, 1));

if (APPLY) {
  const live = fs.readFileSync(file, "utf8");
  if (crypto.createHash("sha1").update(live).digest("hex") !== sha) throw new Error("source changed while splitting");
  for (const c of spec.children) {
    const f = folderFile(c.folder);
    if (fs.existsSync(f)) throw new Error("child exists: " + f);
    if (pathBytes(f) > 240) throw new Error("path too long: " + f);
  }
  const origDir = path.resolve(REPO, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/pfs-orig");
  fs.mkdirSync(origDir, { recursive: true });
  fs.writeFileSync(path.join(origDir, `${spec.label}${path.extname(file)}.txt`), text);
  for (const c of spec.children) {
    const f = folderFile(c.folder);
    fs.mkdirSync(path.dirname(f), { recursive: true });
    fs.writeFileSync(f, outputs.get(c.folder).text);
  }
  if (spec.deleteParent) {
    if (nonImport.some((u) => clusterOf.get(u.k) === PARENT)) throw new Error("deleteParent but the parent still owns statements");
    fs.unlinkSync(file);
  } else fs.writeFileSync(file, parentOut.text);
  console.log("APPLIED");
}
