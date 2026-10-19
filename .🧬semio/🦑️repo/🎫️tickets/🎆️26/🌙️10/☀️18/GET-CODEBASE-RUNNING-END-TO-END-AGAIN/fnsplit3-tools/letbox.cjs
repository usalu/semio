// usage: node letbox.cjs <spec.json> [APPLY]
// Boxes chosen `let` variables of a scope function into one state object: `const <stateName>: {...} = {...};` and every reference becomes `<stateName>.<name>`.
// spec: { file, scopeLine, scopeParam?, stateName, lets: [names], insertBeforeLine? }
const fs = require("fs");
const path = require("path");
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const REPO = "/Users/ueli/Documents/semio";
const spec = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const APPLY = process.argv[3] === "APPLY";
const file = path.isAbsolute(spec.file) ? spec.file : path.join(REPO, spec.file);
const text = fs.readFileSync(file, "utf8");
const errors = [];
const cfg = JSON.parse(fs.readFileSync(path.join(REPO, "tsconfig.json"), "utf8"));
const { options } = ts.convertCompilerOptionsFromJson({ ...cfg.compilerOptions, noEmit: true, incremental: false, types: [] }, REPO);
options.noEmit = true;
const host = ts.createCompilerHost(options);
const origGet = host.getSourceFile.bind(host);
host.getSourceFile = (fn, lv, ...r) => (path.resolve(fn) === path.resolve(file) ? ts.createSourceFile(fn, text, lv, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS) : origGet(fn, lv, ...r));
const program = ts.createProgram([file], options, host);
const sf = program.getSourceFile(file);
const checker = program.getTypeChecker();
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
let scope = null;
const find = (n) => { if (scope) return; if (ts.isFunctionLike(n) && n.body && L(n.getStart(sf)) === spec.scopeLine && (!spec.scopeParam || (n.parameters[0] && n.parameters[0].name.getText(sf) === spec.scopeParam))) { scope = n; return; } ts.forEachChild(n, find); };
find(sf);
if (!scope) throw new Error("scope not found");
const want = new Set(spec.lets);
const boxed = new Map();
const declStmts = new Map();
const visitDecl = (n) => {
  if (ts.isVariableDeclarationList(n) && (n.flags & ts.NodeFlags.Let) && ts.isVariableStatement(n.parent)) {
    for (const d of n.declarations) if (ts.isIdentifier(d.name) && want.has(d.name.text)) {
      const sym = checker.getSymbolAtLocation(d.name);
      const type = d.type ? d.type.getText(sf) : checker.typeToString(checker.getBaseTypeOfLiteralType(checker.getTypeOfSymbolAtLocation(sym, d)), scope, ts.TypeFormatFlags.NoTruncation);
      boxed.set(sym, { name: d.name.text, type, init: d.initializer ? d.initializer.getText(sf) : "undefined", decl: d });
      if (!declStmts.has(n.parent)) declStmts.set(n.parent, n);
    }
  }
  ts.forEachChild(n, visitDecl);
};
visitDecl(scope.body);
for (const nm of want) if (![...boxed.values()].some((b) => b.name === nm)) errors.push(`let ${nm} not found`);
const edits = [];
const declNameNodes = new Set([...boxed.values()].map((b) => b.decl.name));
const walk = (n) => {
  if (ts.isIdentifier(n) && !declNameNodes.has(n)) {
    const p = n.parent;
    const skip = (ts.isPropertyAccessExpression(p) && p.name === n) || (ts.isPropertyAssignment(p) && p.name === n) || (ts.isQualifiedName(p) && p.right === n) || ((ts.isMethodDeclaration(p) || ts.isPropertyDeclaration(p) || ts.isPropertySignature(p) || ts.isMethodSignature(p)) && p.name === n);
    if (!skip) {
      let sym = checker.getSymbolAtLocation(n);
      if (ts.isShorthandPropertyAssignment(p) && p.name === n) sym = checker.getShorthandAssignmentValueSymbol(p) || sym;
      const b = sym && boxed.get(sym);
      if (b) {
        if (ts.isShorthandPropertyAssignment(p)) edits.push({ s: n.getStart(sf), e: n.getEnd(), t: `${n.text}: ${spec.stateName}.${n.text}` });
        else if (ts.isTypeQueryNode(p.parent) || ts.isTypeQueryNode(p)) edits.push({ s: n.getStart(sf), e: n.getEnd(), t: `${spec.stateName}.${n.text}` });
        else edits.push({ s: n.getStart(sf), e: n.getEnd(), t: `${spec.stateName}.${n.text}` });
      }
    }
  }
  ts.forEachChild(n, walk);
};
walk(scope.body);
let firstStmtStart = Infinity;
for (const [stmt, list] of declStmts) {
  const remaining = list.declarations.filter((d) => !declNameNodes.has(d.name));
  firstStmtStart = Math.min(firstStmtStart, stmt.getStart(sf));
  if (remaining.length === 0 && spec.lateInit) {
    const lead = ts.getLeadingCommentRanges(text, stmt.getFullStart()) || [];
    const s0 = stmt.getStart(sf);
    const ind = /^\s*/.exec(text.slice(text.lastIndexOf("\n", s0) + 1))[0];
    const assigns = list.declarations.filter((d) => d.initializer).map((d) => `${spec.stateName}.${d.name.text} = ${d.initializer.getText(sf)};`);
    edits.push({ s: s0, e: stmt.getEnd(), t: assigns.length ? assigns.join("\n" + ind) : "", lateAssign: true });
    if (!assigns.length) { const ls = text.lastIndexOf("\n", s0) + 1; edits.pop(); edits.push({ s: ls, e: stmt.getEnd() + (text[stmt.getEnd()] === "\n" ? 1 : 0), t: "", del: true }); }
  } else if (remaining.length === 0) {
    const lead = ts.getLeadingCommentRanges(text, stmt.getFullStart()) || [];
    const start = lead.length ? lead[0].pos : stmt.getStart(sf);
    const lineStart = text.lastIndexOf("\n", start) + 1;
    const hasOnlyWs = text.slice(lineStart, start).trim() === "";
    edits.push({ s: hasOnlyWs ? lineStart : start, e: stmt.getEnd() + (text[stmt.getEnd()] === "\n" ? 1 : 0), t: "", del: true });
  } else {
    errors.push(`statement at line ${L(stmt.getStart(sf))} mixes boxed and unboxed declarators`);
  }
}
const indent = (/^\s*/.exec(text.slice(text.lastIndexOf("\n", firstStmtStart) + 1))[0]);
const members = [...boxed.values()];
const typeText = `{ ${members.map((b) => `${b.name}: ${b.type}`).join("; ")} }`;
const stateDecl = spec.lateInit ? `${indent}const ${spec.stateName} = {} as ${typeText};\n` : `${indent}const ${spec.stateName}: ${typeText} = { ${members.map((b) => `${b.name}: ${b.init}`).join(", ")} };\n`;
const insertAt = text.lastIndexOf("\n", firstStmtStart) + 1;
edits.push({ s: insertAt, e: insertAt, t: stateDecl });
let out = text;
for (const e of edits.sort((a, b) => b.s - a.s || b.e - a.e)) out = out.slice(0, e.s) + e.t + out.slice(e.e);
console.log(JSON.stringify({ boxed: members.map((b) => `${b.name}: ${b.type}`), refsRewritten: edits.filter((e) => !e.del && !e.lateAssign && e.t !== stateDecl).length, errors }, null, 1));
if (process.env.DUMP) fs.writeFileSync(process.env.DUMP, out);
if (APPLY) {
  if (errors.length) { console.error("REFUSED"); process.exit(2); }
  if (fs.readFileSync(file, "utf8") !== text) { console.error("REFUSED: changed"); process.exit(5); }
  fs.writeFileSync(file, out);
  console.log("APPLIED");
}
