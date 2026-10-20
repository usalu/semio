// usage: node ladder.cjs <spec.json> [APPLY]
// Route-ladder splitter: a method/function body that is a ladder of `if (...) { ...; return; }` statements becomes group functions `(context, segments) => Promise<boolean>`
// spec: { file, className?, method, context: { dir, doc, name, fields: [{ name, type }], importLines: [text], thisFields: [names], thisCalls?: [names] }, groups: [ { dir, doc, name, emoji, fdoc, from, to } ], keepFrom: line, asyncGroups?: true }
//   from/to = start lines of the first/last statement of a group; keepFrom = start line of the first statement that stays in the method; `this.<field>` becomes `context.<field>` inside the groups.
const fs = require("fs");
const path = require("path");
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const VS = "️";
const spec = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const APPLY = process.argv[3] === "APPLY";
const file = spec.file;
const text = fs.readFileSync(process.env.ORIG_TEXT || file, "utf8");
const lines = text.split("\n");
const errors = [];
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
let method = null;
const find = (n) => {
  if (method) return;
  if (ts.isMethodDeclaration(n) && n.name.getText(sf) === spec.method && n.body && (!spec.className || (n.parent.name && n.parent.name.text === spec.className))) { method = n; return; }
  if (!spec.className && ts.isFunctionDeclaration(n) && n.name && n.name.text === spec.method) { method = n; return; }
  ts.forEachChild(n, find);
};
find(sf);
if (!method) throw new Error("method not found");
const stmts = [...method.body.statements].map((node) => ({ node, a: L(node.getStart(sf)), b: L(node.getEnd()) }));
const byLine = new Map(stmts.map((s) => [s.a, s]));
const isAsync = !!method.modifiers?.some((m) => m.kind === ts.SyntaxKind.AsyncKeyword);
const keep = byLine.get(spec.keepFrom);
if (!keep) errors.push(`keepFrom ${spec.keepFrom} is not a statement start`);
const lineOff = [0];
for (let i = 0; i < text.length; i++) if (text[i] === "\n") lineOff.push(i + 1);
const offOfLine = (l) => lineOff[l - 1];

const importStmts = sf.statements.filter(ts.isImportDeclaration);
const skipId = (n) => { const p = n.parent; return p && ((ts.isPropertyAccessExpression(p) && p.name === n) || (ts.isPropertyAssignment(p) && p.name === n) || (ts.isQualifiedName(p) && p.right === n) || ((ts.isMethodDeclaration(p) || ts.isPropertyDeclaration(p) || ts.isPropertySignature(p) || ts.isMethodSignature(p)) && p.name === n) || (ts.isBindingElement(p) && p.propertyName === n)); };
const identsOf = (code) => {
  const f = ts.createSourceFile("x" + path.extname(file), code, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const used = new Set();
  const visit = (n) => { if (ts.isImportDeclaration(n)) return; if (ts.isIdentifier(n) && !skipId(n)) used.add(n.text); ts.forEachChild(n, visit); };
  visit(f);
  return used;
};
const relTo = (fromFile, toFile) => { let r = path.relative(path.dirname(fromFile), toFile).split(path.sep).join("/"); if (!r.startsWith(".")) r = "./" + r; return r; };
const rebase = (fromFile) => (specText) => { if (!specText.startsWith(".")) return specText; let r = path.relative(path.dirname(fromFile), path.resolve(path.dirname(file), specText)).split(path.sep).join("/"); if (!r.startsWith(".")) r = "./" + r; return r; };
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

const baseDir = path.dirname(file);
const childPath = (dir) => path.join(baseDir, dir, "🟦️" + path.extname(file));
const ctxFile = childPath(spec.context.dir);
const thisFields = new Set(spec.context.thisFields);
const ctxName = spec.context.paramName || "context";

const groupOutputs = [];
let prevEnd = -1;
for (const g of spec.groups) {
  const a = byLine.get(g.from), b = byLine.get(g.to);
  if (!a || !b) { errors.push(`group ${g.name}: no statement starts at ${!a ? g.from : g.to}`); continue; }
  const members = stmts.filter((s) => s.node.pos >= a.node.pos && s.node.pos <= b.node.pos);
  if (members[0] !== a) errors.push(`group ${g.name} start mismatch`);
  if (prevEnd >= 0 && stmts.indexOf(a) !== prevEnd + 1) errors.push(`group ${g.name} is not contiguous with the previous group`);
  prevEnd = stmts.indexOf(b);
  const startOff = a.node.getStart(sf), endOff = b.node.getEnd();
  const edits = [];
  const walk = (n, fnDepth) => {
    if (n.kind === ts.SyntaxKind.ThisKeyword) {
      const p = n.parent;
      if (ts.isPropertyAccessExpression(p) && p.expression === n && thisFields.has(p.name.text)) edits.push({ s: n.getStart(sf), e: n.getEnd(), t: ctxName });
      else errors.push(`group ${g.name}: unsupported this use at line ${L(n.getStart(sf))}`);
    }
    if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteral(n.arguments[0]) && n.arguments[0].text.startsWith(".")) {
      const lit = n.arguments[0];
      edits.push({ s: lit.getStart(sf) + 1, e: lit.getEnd() - 1, t: rebase(childPath(g.dir))(lit.text) });
    }
    if (ts.isReturnStatement(n) && fnDepth === 0) {
      if (n.expression) errors.push(`group ${g.name}: return with value at line ${L(n.getStart(sf))}`);
      else edits.push({ s: n.getStart(sf), e: n.getEnd(), t: "return true;" });
    }
    if ((ts.isFunctionExpression(n) || ts.isFunctionDeclaration(n)) && n !== method) { let usesThis = false; const w = (x) => { if (x.kind === ts.SyntaxKind.ThisKeyword) usesThis = true; ts.forEachChild(x, w); }; ts.forEachChild(n, w); if (usesThis) errors.push(`group ${g.name}: function with own this at line ${L(n.getStart(sf))}`); }
    ts.forEachChild(n, (c) => walk(c, fnDepth + (ts.isFunctionLike(c) ? 1 : 0)));
  };
  members.forEach((m) => walk(m.node, 0));
  const multi = (() => { let bad = false; const w = (n) => { if ((ts.isTemplateExpression(n) || ts.isNoSubstitutionTemplateLiteral(n)) && L(n.getStart(sf)) !== L(n.getEnd())) bad = true; ts.forEachChild(n, w); }; members.forEach((m) => w(m.node)); return bad; })();
  if (multi) errors.push(`group ${g.name}: multi-line template literal`);
  let body = text.slice(offOfLine(a.a), endOff);
  const base = offOfLine(a.a);
  for (const e of edits.sort((x, y) => y.s - x.s)) body = body.slice(0, e.s - base) + e.t + body.slice(e.e - base);
  const indent = /^\s*/.exec(lines[a.a - 1])[0];
  const bodyLines = body.split("\n").map((l) => (l.startsWith(indent) ? l.slice(indent.length) : l)).map((l) => (l ? "  " + l : l));
  const fn = [`/** ${g.emoji}${VS} ${g.fdoc} */`, `export async function ${g.name}(${ctxName}: ${spec.context.name}, segments: string[]): Promise<boolean> {`, ...bodyLines, "  return false;", "}"].join("\n");
  groupOutputs.push({ g, fn, path: childPath(g.dir), members });
}
const outputs = [];
for (const o of groupOutputs) {
  const used = identsOf(o.fn);
  const imps = prunedImports(used, rebase(o.path));
  imps.push(`import type { ${spec.context.name} } from "${relTo(o.path, ctxFile)}";`);
  outputs.push({ path: o.path, text: `/** ${o.g.dir.split(VS)[0]}${VS} ${o.g.doc} */\n${imps.join("\n")}\n\n${o.fn}\n`, kind: "child" });
}
{
  const fieldsText = spec.context.fields.map((f) => `  readonly ${f.name}: ${f.type};`).join("\n");
  const imps = spec.context.importLines.map((l) => l.replace(/from "(\.[^"]*)"/, (m, sp) => `from "${rebase(ctxFile)(sp)}"`));
  outputs.push({ path: ctxFile, text: `/** ${spec.context.emoji}${VS} ${spec.context.doc} */\n${imps.join("\n")}\n\nexport type ${spec.context.name} = {\n${fieldsText}\n};\n`, kind: "ctx" });
}
{
  const first = stmts[0];
  const methodOpen = method.body.getStart(sf);
  const keepText = keep ? text.slice(offOfLine(keep.a), method.body.getEnd() - 1) : "";
  const ctxInit = spec.context.init;
  const callLines = groupOutputs.map((o) => `${o.g.name}`);
  const dispatch = `    for (const route of [${callLines.join(", ")}]) if (await route(${ctxName}, segments)) return;`;
  const newBody = `{\n    const ${ctxName}: ${spec.context.name} = ${ctxInit};\n${dispatch}\n${keepText.replace(/\s+$/, "")}\n  }`;
  let rootText = text.slice(0, methodOpen) + newBody + text.slice(method.body.getEnd());
  const sfNew = rootText;
  const usedRoot = identsOf(rootText.split("\n").filter((l) => !/^import\b/.test(l)).join("\n") + "\n" + callLines.join(" "));
  const imps = prunedImports(usedRoot, (s) => s);
  for (const o of groupOutputs) imps.push(`import { ${o.g.name} } from "${relTo(file, o.path)}";`);
  imps.push(`import type { ${spec.context.name} } from "${relTo(file, ctxFile)}";`);
  const headerEnd = importStmts[importStmts.length - 1].getEnd();
  const headerStart = importStmts[0].getFullStart();
  rootText = rootText.slice(0, importStmts[0].getStart(sf)) + imps.join("\n") + rootText.slice(headerEnd);
  outputs.unshift({ path: file, text: rootText, kind: "root" });
}
const lc = (t) => t.split("\n").length;
const emojis = new Map();
const stripV = (e) => e.replace(/️/g, "");
for (const o of groupOutputs) { const e = stripV(o.g.emoji); if (emojis.has(e)) errors.push(`emoji ${e} repeated`); emojis.set(e, o.g.name); const fe = stripV(o.g.dir.split(VS)[0]); if (emojis.has(fe)) errors.push(`folder emoji ${fe} repeated`); emojis.set(fe, o.g.dir); }
{ const fe = stripV(spec.context.dir.split(VS)[0]); if (emojis.has(fe)) errors.push(`folder emoji ${fe} repeated (context)`); }
console.log(JSON.stringify({ outputs: outputs.map((o) => ({ path: path.relative(baseDir, o.path), lines: lc(o.text) })), errors }, null, 1));
if (process.env.DUMP) { fs.mkdirSync(process.env.DUMP, { recursive: true }); outputs.forEach((o, i) => fs.writeFileSync(path.join(process.env.DUMP, `${i}-${path.basename(path.dirname(o.path))}${path.extname(file)}`), o.text)); }
if (APPLY) {
  if (errors.length) { console.error("REFUSED"); process.exit(2); }
  for (const o of outputs.slice(1)) if (fs.existsSync(o.path)) { console.error("REFUSED: child exists " + o.path); process.exit(4); }
  if (fs.readFileSync(file, "utf8") !== text) { console.error("REFUSED: root changed"); process.exit(5); }
  for (const o of outputs.slice(1)) { fs.mkdirSync(path.dirname(o.path), { recursive: true }); fs.writeFileSync(o.path, o.text); }
  fs.writeFileSync(file, outputs[0].text);
  console.log("APPLIED");
}
