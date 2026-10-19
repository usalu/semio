// 🔎 Lossless check for dsplit: expands the part calls of the new describe callback with the bodies of the part functions and compares the token stream with the original callback body.
// usage: node verify-expand.cjs <newRoot> <snapshot> [nth module-level describe, default 0]   (relative string literals are normalised: dsplit rebases them on purpose)
const ts = require(process.env.TYPESCRIPT_PATH || "/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const fs = require("fs");
const path = require("path");
const [newRoot, snapshot, describeLine] = process.argv.slice(2);
const REPO = "/Users/ueli/Documents/semio/";
const abs = (p) => (path.isAbsolute(p) ? p : REPO + p);
const parse = (file, textOverride) => { const text = textOverride ?? fs.readFileSync(file, "utf8"); return ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS); };
const callbackOf = (sf, nth) => {
  const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
  const st = sf.statements.filter((s) => ts.isExpressionStatement(s) && ts.isCallExpression(s.expression) && /^describe\b/.test(s.expression.expression.getText(sf)))[+nth || 0];
  const call = st.expression;
  return call.arguments[call.arguments.length - 1];
};
const tokens = (sf, nodes) => {
  const out = [];
  for (const n of nodes) {
    const w = (x) => {
      if (x.getChildCount(sf) === 0 && x.kind !== ts.SyntaxKind.EndOfFileToken) {
        let t = x.getText(sf);
        if (ts.isStringLiteralLike(x) && /^["'`]\.{1,2}\//.test(t)) t = "<rel>";
        out.push(t.includes("\n") ? t.replace(/\s*\n\s*\*?\s*/g, " ") : t);
      } else x.getChildren(sf).forEach(w);
    };
    w(n);
  }
  return out;
};
const oldSf = parse(abs(snapshot));
const oldCb = callbackOf(oldSf, describeLine);
const oldStmts = [...oldCb.body.statements].map((st) => tokens(oldSf, [st]));
const oldTokens = oldStmts.flat();
const newSf = parse(abs(newRoot));
const newCb = callbackOf(newSf, describeLine);
const imports = new Map();
for (const s of newSf.statements) if (ts.isImportDeclaration(s) && s.moduleSpecifier.text.startsWith(".") && s.importClause?.namedBindings && ts.isNamedImports(s.importClause.namedBindings)) for (const el of s.importClause.namedBindings.elements) imports.set(el.name.text, path.resolve(path.dirname(abs(newRoot)), s.moduleSpecifier.text));
const expanded = [];
for (const s of newCb.body.statements) {
  if (ts.isExpressionStatement(s) && ts.isCallExpression(s.expression) && ts.isIdentifier(s.expression.expression) && imports.has(s.expression.expression.text)) {
    const f = imports.get(s.expression.expression.text);
    const sf = parse(f);
    const fn = sf.statements.find((x) => ts.isFunctionDeclaration(x) && x.name.text === s.expression.expression.text);
    expanded.push(...tokens(sf, [...fn.body.statements]));
  } else expanded.push(...tokens(newSf, [s]));
}
const sharedFiles = [];
{
  const seen = new Set([abs(newRoot)]);
  const queue = [...new Set([...imports.values()])];
  while (queue.length) {
    const f = queue.shift();
    if (seen.has(f) || !fs.existsSync(f)) continue;
    seen.add(f);
    sharedFiles.push(f);
    const sf = parse(f);
    for (const st of sf.statements) if (ts.isImportDeclaration(st) && st.moduleSpecifier.text.startsWith(".")) { const r = path.resolve(path.dirname(f), st.moduleSpecifier.text); if (/🟦️\.tsx?$/.test(r) && r.startsWith(path.dirname(abs(newRoot)))) queue.push(r); }
  }
}
const sharedPool = [];
for (const f of sharedFiles) {
  const sf = parse(f);
  for (const st of sf.statements) { const t = tokens(sf, [st]); { const k = t.slice(0, 4).indexOf("export"); sharedPool.push(k >= 0 ? [...t.slice(0, k), ...t.slice(k + 1)] : t); } }
}
const eq = (a, b) => a.length === b.length && a.every((t, i) => t === b[i]);
let pos = 0, hoistedCount = 0, ok = true, detail = "";
for (const st of oldStmts) {
  if (eq(expanded.slice(pos, pos + st.length), st)) { pos += st.length; continue; }
  if (sharedPool.some((t) => eq(t, st))) { hoistedCount += 1; continue; }
  ok = false; detail = JSON.stringify(st.slice(0, 12)); break;
}
ok = ok && pos === expanded.length;
console.log(`callback tokens old=${oldTokens.length} expanded=${expanded.length} statements=${oldStmts.length} hoisted-to-shared=${hoistedCount} ${ok ? "LOSSLESS OK" : "DIFFERENT " + detail}`);
process.exit(ok ? 0 : 1);
