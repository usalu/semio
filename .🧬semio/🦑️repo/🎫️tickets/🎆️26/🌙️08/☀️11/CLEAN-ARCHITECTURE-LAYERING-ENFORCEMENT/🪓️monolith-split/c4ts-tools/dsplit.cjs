// usage: node dsplit.cjs <spec.json> [APPLY]
// Test-file splitter. A test file holds imports, optional top-level helpers and one or more top-level `describe` calls (or one giant describe with a shared body).
// spec: {
//   file, childRoot?,                     // childRoot defaults to the parent's grandparent (tests/ dir): children are sibling folders of the parent folder
//   describeLine?: N,                     // start line of the top-level describe whose BODY is split (omit when top-level describes are split whole)
//   kit?: { dir, doc, lines:[[a,b]...] }, // statements (top-level or describe body, by first line) moved into an exported kit file
//   shared?: [[a,b]...],                  // describe-body statements copied into every child describe (let, beforeEach, afterEach) and kept in the parent
//   children: [{ dir, doc, title?, lines:[[a,b]...] }]   // body statements (or whole top-level statements when describeLine is absent) moved to a child
// }
// Dry run prints sizes and problems; APPLY writes children, kit and the new parent. Imports are pruned per file by identifier use.
const fs = require("fs");
const path = require("path");
const ts = require("/Users/ueli/Documents/semio/node_modules/typescript");

const spec = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const APPLY = process.argv[3] === "APPLY";
const file = spec.file;
const text = fs.readFileSync(process.env.ORIG_TEXT || file, "utf8");
const isTsx = file.endsWith(".tsx");
const ext = path.extname(file);
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, isTsx ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const lines = text.split("\n");
const errors = [];
const warn = [];
const lineOf = (pos) => sf.getLineAndCharacterOfPosition(pos).line + 1;
const regionRe = /^[ \t]*\/\/\s*#(end)?region\b/;

const leadLine = (node) => {
  const full = node.getFullStart();
  const start = node.getStart(sf, false);
  const trivia = text.slice(full, start);
  const m = trivia.match(/^\s*/)[0].length;
  return lineOf(full + m);
};
const mk = (node) => ({ node, startLine: leadLine(node), endLine: lineOf(node.getEnd()), codeLine: lineOf(node.getStart(sf, false)) });
const inRanges = (ranges, l) => (ranges || []).some(([a, b]) => l >= a && l <= b);

const top = sf.statements.map(mk);
let describeStmt = null;
let bodyStmts = [];
let cbNode = null;
if (spec.describeLine) {
  describeStmt = top.find((s) => s.startLine === spec.describeLine || s.codeLine === spec.describeLine);
  if (!describeStmt) throw new Error("describeLine not found");
  const call = describeStmt.node.expression;
  cbNode = call.arguments[call.arguments.length - 1];
  if (!ts.isArrowFunction(cbNode) && !ts.isFunctionExpression(cbNode)) throw new Error("describe callback not a function");
  bodyStmts = cbNode.body.statements.map(mk);
}
for (let i = 0; i < lines.length; i++) if (regionRe.test(lines[i])) warn.push(`region marker at line ${i + 1}`);

const importStmts = top.filter((s) => ts.isImportDeclaration(s.node));
const kidsSpec = spec.children;

const owner = new Map();
const classify = (s, where) => {
  if (spec.kit && inRanges(spec.kit.lines, s.startLine)) return "kit";
  for (let c = 0; c < kidsSpec.length; c++) if (inRanges(kidsSpec[c].lines, s.startLine)) return "c" + c;
  return "parent";
};
const topOwner = top.map((s) => (ts.isImportDeclaration(s.node) ? "import" : s === describeStmt ? "describe" : classify(s)));
const bodyOwner = bodyStmts.map((s) => (inRanges(spec.shared, s.startLine) ? "shared" : classify(s)));

const nameOfDecl = (n) => {
  if (ts.isVariableStatement(n)) return n.declarationList.declarations.flatMap((d) => (ts.isIdentifier(d.name) ? [d.name.text] : []));
  if (ts.isFunctionDeclaration(n) || ts.isClassDeclaration(n) || ts.isInterfaceDeclaration(n) || ts.isTypeAliasDeclaration(n) || ts.isEnumDeclaration(n)) return n.name ? [n.name.text] : [];
  return [];
};

const sliceLines = (a, b) => lines.slice(a - 1, b);
const dedent = (ls, n) => ls.map((l) => (l.startsWith(" ".repeat(n)) ? l.slice(n) : l));
const indent = (ls, n) => ls.map((l) => (l.length ? " ".repeat(n) + l : l));
const indentOf = bodyStmts.length ? (lines[bodyStmts[0].codeLine - 1].match(/^\s*/)[0].length) : 0;

const kitDecls = [];
const kitItems = [];
if (spec.kit) {
  top.forEach((s, i) => { if (topOwner[i] === "kit") kitItems.push({ lines: sliceLines(s.startLine, s.endLine), names: nameOfDecl(s.node), node: s.node, indent: 0 }); });
  bodyStmts.forEach((s, i) => { if (bodyOwner[i] === "kit") kitItems.push({ lines: dedent(sliceLines(s.startLine, s.endLine), indentOf), names: nameOfDecl(s.node), node: s.node, indent: indentOf }); });
  for (const it of kitItems) {
    if (!it.names.length) errors.push(`kit statement at ${lineOf(it.node.getStart(sf))} declares no name`);
    kitDecls.push(...it.names);
  }
}

const collectIds = (srcText, kind) => {
  const f = ts.createSourceFile("x" + ext, srcText, ts.ScriptTarget.Latest, true, isTsx ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const used = new Set();
  const visit = (n) => {
    if (ts.isImportDeclaration(n)) return;
    if (ts.isIdentifier(n)) {
      const p = n.parent;
      const skip = p && ((ts.isPropertyAccessExpression(p) && p.name === n) || (ts.isPropertyAssignment(p) && p.name === n) || (ts.isQualifiedName(p) && p.right === n) || ((ts.isMethodDeclaration(p) || ts.isPropertyDeclaration(p) || ts.isPropertySignature(p) || ts.isMethodSignature(p)) && p.name === n) || (ts.isJsxAttribute(p) && p.name === n) || (ts.isBindingElement(p) && p.propertyName === n));
      if (!skip) used.add(n.text);
    }
    ts.forEachChild(n, visit);
  };
  visit(f);
  return used;
};

const prunedImports = (usedIds, extraSpecFix) => {
  const out = [];
  for (const s of importStmts) {
    const n = s.node;
    const c = n.importClause;
    const q = n.moduleSpecifier.getText(sf)[0];
    const specText = n.moduleSpecifier.text;
    const spec2 = extraSpecFix ? extraSpecFix(specText) : specText;
    if (!c) { out.push({ text: `import ${q}${spec2}${q};`, side: true }); continue; }
    const def = c.name && usedIds.has(c.name.text) ? c.name.text : null;
    let ns = null; const named = [];
    if (c.namedBindings && ts.isNamespaceImport(c.namedBindings) && usedIds.has(c.namedBindings.name.text)) ns = c.namedBindings.name.text;
    if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const el of c.namedBindings.elements) if (usedIds.has(el.name.text)) named.push(el.getText(sf));
    if (!def && !ns && !named.length) continue;
    const bits = [];
    if (def) bits.push(def);
    if (ns) bits.push(`* as ${ns}`);
    if (named.length) bits.push(`{ ${named.join(", ")} }`);
    out.push({ text: `import ${c.isTypeOnly ? "type " : ""}${bits.join(", ")} from ${q}${spec2}${q};` });
  }
  return out;
};
const rebase = (fromFile) => (specText) => {
  if (!specText.startsWith(".")) return specText;
  if (fromFile === file) return specText;
  let r = path.relative(path.dirname(fromFile), path.resolve(path.dirname(file), specText)).split(path.sep).join("/");
  if (!r.startsWith(".")) r = "./" + r;
  return r;
};
const relTo = (fromFile, toFile) => {
  let r = path.relative(path.dirname(fromFile), toFile).split(path.sep).join("/");
  if (!r.startsWith(".")) r = "./" + r;
  return r;
};

const childRoot = spec.childRoot || path.dirname(path.dirname(file));
const kitFile = spec.kit ? path.join(childRoot, spec.kit.dir, "🟦️" + (spec.kit.ext || ".ts")) : null;
const childFiles = kidsSpec.map((k) => path.join(childRoot, k.dir, "🟦️" + (k.ext || ext)));
const outputs = [];

const importBlock = (outFile, bodyText) => {
  const used = collectIds(bodyText);
  const imps = prunedImports(used, rebase(outFile));
  const lines2 = imps.filter((i) => !i.side || outFile === file).map((i) => i.text);
  if (spec.kit && outFile !== kitFile) {
    const names = kitDecls.filter((n) => used.has(n)).sort();
    if (names.length) lines2.push(`import { ${names.join(", ")} } from "${relTo(outFile, kitFile)}";`);
  }
  return lines2;
};

const sharedItems = bodyStmts.filter((s, i) => bodyOwner[i] === "shared");
const sharedText = sharedItems.map((s) => sliceLines(s.startLine, s.endLine).join("\n"));
if (spec.kit) {
  const sharedNames = new Set(sharedItems.flatMap((s) => nameOfDecl(s.node)));
  for (const it of kitItems) { const u = collectIds(it.lines.join("\n")); for (const n of sharedNames) if (u.has(n)) errors.push(`kit statement ${it.names.join(",")} uses shared ${n}`); }
}

const describeHead = (k) => {
  const call = describeStmt.node.expression;
  const title = k.title ?? text.slice(call.arguments[0].getStart(sf), call.arguments[0].getEnd());
  const t = k.title !== undefined ? JSON.stringify(k.title) : title;
  return `${call.expression.getText(sf)}(${t}, () => {`;
};

const joinBlocks = (blocks) => blocks.filter((b) => b.length).join("\n\n");

if (spec.kit) {
  const body = kitItems.map((it) => {
    const ls = it.lines.slice();
    const idx = ls.findIndex((l) => /^\s*(const|let|var|function|class|interface|type|enum|async function)\b/.test(l));
    if (idx < 0) { errors.push("kit: no declaration line in " + it.names.join(",")); return ls.join("\n"); }
    if (!/^export\b/.test(ls[idx])) ls[idx] = "export " + ls[idx].trimStart();
    return ls.join("\n");
  });
  const bodyText = body.join("\n\n");
  const imps = importBlock(kitFile, bodyText);
  outputs.push({ path: kitFile, text: `${spec.kit.doc}\n${imps.join("\n")}${imps.length ? "\n\n" : ""}${bodyText}`.replace(/\s+$/, "") + "\n", kind: "kit" });
}

kidsSpec.forEach((k, c) => {
  let blocks;
  if (describeStmt) {
    const its = bodyStmts.filter((s, i) => bodyOwner[i] === "c" + c).map((s) => sliceLines(s.startLine, s.endLine).join("\n"));
    if (!its.length) errors.push(`child ${k.dir} has no statements`);
    const inner = [...sharedText, ...its].join("\n\n");
    blocks = [`${describeHead(k)}\n${inner}\n});`];
  } else {
    const items = top.filter((s, i) => topOwner[i] === "c" + c).map((s) => sliceLines(s.startLine, s.endLine).join("\n"));
    if (!items.length) errors.push(`child ${k.dir} has no statements`);
    blocks = [items.join("\n\n")];
  }
  const bodyText = blocks.join("\n\n");
  const imps = importBlock(childFiles[c], bodyText);
  if (!k.doc || !/^\/\*\*\s*\S/.test(k.doc)) errors.push(`child ${k.dir}: missing docstring`);
  outputs.push({ path: childFiles[c], text: `${k.doc}\n${imps.join("\n")}${imps.length ? "\n\n" : ""}${bodyText}`.replace(/\s+$/, "") + "\n", kind: "child" });
});

{
  const remove = new Array(lines.length + 2).fill(false);
  top.forEach((s, i) => { if (/^(kit|c\d+)$/.test(topOwner[i])) for (let l = s.startLine; l <= s.endLine; l++) remove[l] = true; });
  bodyStmts.forEach((s, i) => { if (/^(kit|c\d+)$/.test(bodyOwner[i])) for (let l = s.startLine; l <= s.endLine; l++) remove[l] = true; });
  const firstImport = importStmts.length ? importStmts[0].codeLine : null;
  const lastImport = importStmts.length ? importStmts[importStmts.length - 1].endLine : null;
  const keptText = lines.filter((l, idx) => !remove[idx + 1] && !(firstImport && idx + 1 >= firstImport && idx + 1 <= lastImport)).join("\n");
  const header = firstImport ? lines.slice(0, firstImport - 1) : [];
  const nonImportLines = lines.map((l, idx) => ({ l, n: idx + 1 })).filter((x) => !remove[x.n] && (!firstImport || x.n < firstImport || x.n > lastImport)).map((x) => x.l);
  const bodyOnly = nonImportLines.slice(header.length).join("\n");
  const imps = importBlock(file, bodyOnly);
  let outText = header.join("\n") + (header.length ? "\n" : "") + imps.join("\n") + "\n" + bodyOnly;
  outText = outText.replace(/\n{3,}/g, "\n\n").replace(/\{\n\n+/g, "{\n").replace(/\n\n+\}\);/g, "\n});").replace(/\s+$/, "") + "\n";
  outputs.unshift({ path: file, text: outText, kind: "parent" });
}

const lc = (t) => t.split("\n").length - 1;
console.log(JSON.stringify({ outputs: outputs.map((o) => ({ path: path.relative(path.dirname(file), o.path), lines: lc(o.text) })), warnings: warn, errors, kitDecls }, null, 1));
if (APPLY) {
  if (errors.length) { console.error("REFUSED: errors present"); process.exit(2); }
  for (const o of outputs.slice(1)) if (fs.existsSync(o.path)) { console.error("REFUSED: child exists " + o.path); process.exit(4); }
  if (fs.readFileSync(file, "utf8") !== text) { console.error("REFUSED: parent changed during run"); process.exit(5); }
  for (const o of outputs.slice(1)) { fs.mkdirSync(path.dirname(o.path), { recursive: true }); fs.writeFileSync(o.path, o.text); }
  fs.writeFileSync(file, outputs[0].text);
  console.log("APPLIED");
} else if (process.env.DUMP) {
  fs.mkdirSync(process.env.DUMP, { recursive: true });
  for (const o of outputs) fs.writeFileSync(path.join(process.env.DUMP, path.basename(path.dirname(o.path)) + ".out" + ext), o.text);
}
