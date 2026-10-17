// usage: node units.cjs <file>   (top-level statements with reference edges inside the file)
const ts = require("/Users/ueli/Documents/semio/node_modules/typescript");
const fs = require("fs");
function analyze(file) {
  const text = fs.readFileSync(file, "utf8");
  const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const host = ts.createCompilerHost({});
  host.getSourceFile = (n) => (n === file ? sf : undefined);
  host.fileExists = (n) => n === file;
  host.readFile = (n) => (n === file ? text : undefined);
  const program = ts.createProgram([file], { noResolve: true, noLib: true, target: ts.ScriptTarget.Latest }, host);
  const checker = program.getTypeChecker();
  const lineOf = (pos) => sf.getLineAndCharacterOfPosition(pos).line + 1;
  const stmts = sf.statements.map((s, i) => {
    let names;
    if (ts.isVariableStatement(s)) names = s.declarationList.declarations.map((d) => d.name.getText(sf));
    else if (s.name && s.name.getText) names = [s.name.getText(sf)];
    else if (ts.isImportDeclaration(s)) names = ["<import>"];
    else names = ["<" + ts.SyntaxKind[s.kind] + ">"];
    return {
      i,
      full: lineOf(s.getFullStart() + (s.getFullText(sf).match(/^\s*/)[0].length ? 0 : 0)),
      start: lineOf(s.getStart(sf, true)),
      end: lineOf(s.getEnd()),
      kind: ts.SyntaxKind[s.kind],
      names,
      exported: !!(ts.getCombinedModifierFlags(s) & ts.ModifierFlags.Export),
    };
  });
  const owner = new Map();
  sf.statements.forEach((s, i) => {
    const syms = [];
    if (ts.isVariableStatement(s)) for (const d of s.declarationList.declarations) { const sy = checker.getSymbolAtLocation(d.name); if (sy) syms.push(sy); }
    else if (s.name) { const sy = checker.getSymbolAtLocation(s.name); if (sy) syms.push(sy); }
    for (const sy of syms) owner.set(sy, i);
  });
  const edges = stmts.map(() => new Set());
  const visit = (n, i) => {
    if (ts.isIdentifier(n)) {
      let sy = checker.getSymbolAtLocation(n);
      if (!sy && n.parent && ts.isShorthandPropertyAssignment(n.parent)) sy = checker.getShorthandAssignmentValueSymbol(n.parent);
      if (sy && owner.has(sy)) { const j = owner.get(sy); if (j !== i) edges[i].add(j); }
    }
    ts.forEachChild(n, (c) => visit(c, i));
  };
  sf.statements.forEach((s, i) => { if (!ts.isImportDeclaration(s)) visit(s, i); });
  return { sf, text, stmts, edges, checker };
}
module.exports = { analyze, ts };
if (require.main === module) {
  const { stmts, edges } = analyze(process.argv[2]);
  for (const s of stmts) console.log(`${s.i}\t${s.start}-${s.end}\t${s.kind}\t${s.exported ? "export " : ""}${s.names.join(",")}\t-> ${[...edges[s.i]].join(",")}`);
}
