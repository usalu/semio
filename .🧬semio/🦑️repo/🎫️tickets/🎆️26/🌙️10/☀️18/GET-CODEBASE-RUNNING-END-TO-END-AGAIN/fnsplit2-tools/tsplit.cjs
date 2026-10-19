// usage: node tsplit.cjs <spec.json> [APPLY]
// Template splitter: cuts the ONE template literal a function returns into chunk functions (one folder each) at line starts; the root returns the chunks joined with "".
// spec: { file, function, parts: [ { dir, doc, name, emoji, fdoc, fromLine } ] }   fromLine = first source line of the chunk; the first part starts at the template's own line.
const fs = require("fs");
const path = require("path");
const ts = require(process.env.TYPESCRIPT_PATH || "/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");

const spec = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const APPLY = process.argv[3] === "APPLY";
const file = spec.file;
const text = fs.readFileSync(file, "utf8");
const lines = text.split("\n");
const errors = [];
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
const lineStart = (line) => (line <= 1 ? 0 : lines.slice(0, line - 1).join("\n").length + 1);
const ext = path.extname(file);
const V = "️";
const stripV = (e) => e.replace(/️/g, "");

const fn = sf.statements.find((s) => ts.isFunctionDeclaration(s) && s.name && s.name.text === spec.function);
if (!fn || fn.body.statements.length !== 1 || !ts.isReturnStatement(fn.body.statements[0])) throw new Error("function must consist of one return statement");
const ret = fn.body.statements[0];
const tpl = ret.expression;
if (!ts.isTemplateExpression(tpl) && !ts.isNoSubstitutionTemplateLiteral(tpl)) throw new Error("return must be a template literal");
const tStart = tpl.getStart(sf), tEnd = tpl.getEnd();
const spans = ts.isTemplateExpression(tpl) ? tpl.templateSpans.map((sp) => ({ s: sp.expression.getStart(sf) - 2, e: sp.literal.getStart(sf) + 1 })) : [];
const cuts = spec.parts.map((p, i) => (i === 0 ? tStart + 1 : lineStart(p.fromLine)));
const ends = cuts.map((c, i) => (i + 1 < cuts.length ? cuts[i + 1] : tEnd - 1));
cuts.forEach((c, i) => {
  if (i > 0) {
    if (c <= tStart || c >= tEnd) errors.push(`cut ${i} is outside the template`);
    if (text[c - 1] !== "\n") errors.push(`cut ${i} is not at a line start`);
    if (text[c - 2] === "\\") errors.push(`cut ${i} follows a line continuation`);
    for (const sp of spans) if (c > sp.s && c < sp.e) errors.push(`cut ${i} splits an interpolation`);
  }
  if (i > 0 && c <= cuts[i - 1]) errors.push(`cuts are not increasing at ${i}`);
});
const importStmts = sf.statements.filter(ts.isImportDeclaration);
const childPath = (dir) => path.join(path.dirname(file), dir, "🟦️" + ext);
const rebase = (fromFile) => (specText) => {
  if (!specText.startsWith(".")) return specText;
  let r = path.relative(path.dirname(fromFile), path.resolve(path.dirname(file), specText)).split(path.sep).join("/");
  if (!r.startsWith(".")) r = "./" + r;
  return r;
};
const relTo = (fromFile, toFile) => { let r = path.relative(path.dirname(fromFile), toFile).split(path.sep).join("/"); if (!r.startsWith(".")) r = "./" + r; return r; };
const identsOf = (code) => {
  const f = ts.createSourceFile("x" + ext, code, ts.ScriptTarget.Latest, true);
  const used = new Set();
  const visit = (n) => {
    if (ts.isImportDeclaration(n)) return;
    if (ts.isIdentifier(n)) {
      const p = n.parent;
      const skip = p && ((ts.isPropertyAccessExpression(p) && p.name === n) || (ts.isPropertyAssignment(p) && p.name === n) || (ts.isQualifiedName(p) && p.right === n));
      if (!skip) used.add(n.text);
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
    out.push(`import ${c.isTypeOnly ? "type " : ""}${bits.join(", ")} from ${q}${sp}${q};`);
  }
  return out;
};
const outputs = [];
const emojis = new Map();
const claim = (e, who) => { const k = stripV(e); if (emojis.has(k)) errors.push(`emoji ${e} repeated (${emojis.get(k)}, ${who})`); emojis.set(k, who); };
const interpolationCode = (a, b) => spans.filter((sp) => sp.s >= a && sp.e <= b).map((sp) => text.slice(sp.s, sp.e)).join("\n");
spec.parts.forEach((p, i) => {
  claim(p.emoji, p.name);
  claim(p.dir.split(V)[0], "folder " + p.dir);
  const raw = text.slice(cuts[i], ends[i]);
  const out = childPath(p.dir);
  const code = [`/** ${p.emoji} ${p.fdoc} */`, `export function ${p.name}(): string {`, `  return \`${raw}\`;`, "}"].join("\n");
  const used = identsOf(`const __x = ${"`"}${raw}${"`"};` + "\n" + interpolationCode(cuts[i], ends[i]));
  const imps = prunedImports(used, rebase(out));
  if (!/^\/\*\*\s+\S/.test(p.doc)) errors.push(`doc must start with /** : ${p.doc}`);
  outputs.push({ path: out, text: `${p.doc}\n${imps.join("\n")}${imps.length ? "\n\n" : ""}${code}\n`, kind: "part" });
});
{
  const call = `[${spec.parts.map((p) => `${p.name}()`).join(", ")}].join("")`;
  const bodyStart = fn.body.getStart(sf) + 1, bodyEnd = fn.body.getEnd() - 1;
  let out = text.slice(0, bodyStart) + `\n  return ${call};\n` + text.slice(bodyEnd);
  const f2 = ts.createSourceFile(file, out, ts.ScriptTarget.Latest, true);
  const importsNow = f2.statements.filter(ts.isImportDeclaration);
  const rest = out.slice(importsNow.length ? importsNow[importsNow.length - 1].getEnd() : 0);
  const usedNow = identsOf(rest);
  const edits = [];
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
    edits.push({ s: n.getStart(f2), e: n.getEnd(), t: bits.length ? `import ${c.isTypeOnly ? "type " : ""}${bits.join(", ")} from ${q}${n.moduleSpecifier.text}${q};` : "" });
  }
  const adds = spec.parts.map((p) => `import { ${p.name} } from "${relTo(file, childPath(p.dir))}";`);
  const lastImport = importsNow.length ? importsNow[importsNow.length - 1] : null;
  const at = lastImport ? lastImport.getEnd() : 0;
  edits.push({ s: at, e: at, t: (lastImport ? "\n" : "") + adds.join("\n") + (lastImport ? "" : "\n") });
  for (const e of edits.sort((a, b) => b.s - a.s)) out = out.slice(0, e.s) + e.t + out.slice(e.e);
  out = out.replace(/\n{3,}/g, "\n\n");
  outputs.unshift({ path: file, text: out, kind: "root" });
}
const lc = (t) => t.split("\n").length - 1;
console.log(JSON.stringify({ outputs: outputs.map((o) => ({ path: path.relative(path.dirname(file), o.path), kind: o.kind, lines: lc(o.text) })), errors }, null, 1));
if (APPLY) {
  if (errors.length) { console.error("REFUSED: errors present"); process.exit(2); }
  for (const o of outputs.slice(1)) if (fs.existsSync(o.path)) { console.error("REFUSED: child exists " + o.path); process.exit(4); }
  if (fs.readFileSync(file, "utf8") !== text) { console.error("REFUSED: root changed during run"); process.exit(5); }
  for (const o of outputs.slice(1)) { fs.mkdirSync(path.dirname(o.path), { recursive: true }); fs.writeFileSync(o.path, o.text); }
  fs.writeFileSync(file, outputs[0].text);
  console.log("APPLIED");
}
