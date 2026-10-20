// usage: node hoist-decls.cjs <file> <callStartLine> [APPLY] [--max N]
// Hoists the depth-1 declarations of a module-level call's callback (function declarations, type aliases and consts) that reference no callback-scoped binding (parameters, non-hoisted locals) to module level, before the call, in their original order.
const fs = require("fs");
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const [file, callLine, ...rest] = process.argv.slice(2);
const APPLY = rest.includes("APPLY");
const text = fs.readFileSync(file, "utf8");
const sfOpts = { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true, noEmit: true, skipLibCheck: true, types: [], noResolve: true, strict: true };
const host = ts.createCompilerHost(sfOpts);
const orig = host.getSourceFile.bind(host);
host.getSourceFile = (fn, lv, ...r) => (fn === file ? ts.createSourceFile(fn, text, lv, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS) : orig(fn, lv, ...r));
const program = ts.createProgram([file], sfOpts, host);
const sf = program.getSourceFile(file);
const checker = program.getTypeChecker();
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
const stmt = sf.statements.find((s) => L(s.getStart(sf)) === +callLine);
let call = stmt.expression; if (ts.isAwaitExpression(call)) call = call.expression;
const cb = call.arguments[call.arguments.length - 1];
const body = cb.body;
const stmts = [...body.statements];
const declSyms = new Map();
const addSym = (id, st) => { const sym = checker.getSymbolAtLocation(id); if (sym) declSyms.set(sym, st); };
for (const st of stmts) {
  if (ts.isFunctionDeclaration(st) && st.name) addSym(st.name, st);
  else if (ts.isTypeAliasDeclaration(st) || ts.isInterfaceDeclaration(st)) addSym(st.name, st);
  else if (ts.isVariableStatement(st)) for (const d of st.declarationList.declarations) { const walk = (n) => { if (ts.isIdentifier(n)) addSym(n, st); else if (ts.isObjectBindingPattern(n) || ts.isArrayBindingPattern(n)) n.elements.forEach((e) => { if (!ts.isOmittedExpression(e)) walk(e.name); }); }; walk(d.name); }
}
const paramSyms = new Set();
for (const p of cb.parameters) { const walk = (n) => { if (ts.isIdentifier(n)) { const s = checker.getSymbolAtLocation(n); if (s) paramSyms.add(s); } else if (ts.isObjectBindingPattern(n) || ts.isArrayBindingPattern(n)) n.elements.forEach((e) => { if (!ts.isOmittedExpression(e)) walk(e.name); }); }; walk(p.name); }
const candidates = stmts.filter((st) => ts.isFunctionDeclaration(st) || ts.isTypeAliasDeclaration(st) || ts.isInterfaceDeclaration(st) || (ts.isVariableStatement(st) && (st.declarationList.flags & ts.NodeFlags.Const)));
const refs = (node) => { const out = new Set(); const visit = (n) => { if (ts.isIdentifier(n)) { const p = n.parent; const skip = (ts.isPropertyAccessExpression(p) && p.name === n) || (ts.isPropertyAssignment(p) && p.name === n) || (ts.isQualifiedName(p) && p.right === n); if (!skip) { const s = checker.getSymbolAtLocation(n) || (ts.isShorthandPropertyAssignment(p) ? checker.getShorthandAssignmentValueSymbol(p) : null); if (s) out.add(s); } } ts.forEachChild(n, visit); }; visit(node); return out; };
const hoist = new Set(candidates);
for (let changed = true; changed; ) {
  changed = false;
  for (const st of [...hoist]) {
    for (const s of refs(st)) {
      if (paramSyms.has(s)) { hoist.delete(st); changed = true; break; }
      const d = declSyms.get(s);
      if (d && d !== st && !hoist.has(d)) { hoist.delete(st); changed = true; break; }
    }
  }
}
const picked = stmts.filter((st) => hoist.has(st));
const lines = text.split("\n");
const size = picked.reduce((n, st) => n + L(st.getEnd()) - L(st.getStart(sf)) + 1, 0);
console.log("callback statements", stmts.length, "hoisting", picked.length, "lines", size);
console.log(picked.map((st) => `${L(st.getStart(sf))}:${ts.isFunctionDeclaration(st) ? st.name.text : st.getText(sf).slice(0, 40).replace(/\n.*/s, "")}`).join(" | "));
if (APPLY) {
  const ranges = picked.map((st) => [st.getFullStart(), st.getEnd()]);
  const bodyInd = (picked[0] ? text.slice(text.lastIndexOf("\n", picked[0].getStart(sf)) + 1, picked[0].getStart(sf)) : "").length;
  const dd = (s) => s.split("\n").map((l) => l.slice(Math.min(bodyInd, l.match(/^ */)[0].length))).join("\n");
  const moved = picked.map((st) => dd(text.slice(st.getFullStart(), st.getEnd()).replace(/^\n+/, "")));
  let out = text;
  for (const [s, e] of ranges.sort((a, b) => b[0] - a[0])) out = out.slice(0, s) + out.slice(e);
  const callStart = stmt.getFullStart();
  const shiftBefore = 0;
  const insertAt = stmt.getStart(sf);
  out = out.slice(0, insertAt) + moved.join("\n\n") + "\n\n" + out.slice(insertAt);
  fs.writeFileSync(file, out);
  console.log("APPLIED");
}
