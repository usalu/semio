// usage: node freevars.cjs <file> <startLine> <endLine>: identifiers referenced in the line range that are declared in an enclosing function scope (not module scope, not imports)
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const fs = require("fs");
const [file, a, b] = process.argv.slice(2);
const text = fs.readFileSync(file, "utf8");
const sfOpts = { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true, noEmit: true, skipLibCheck: true, types: [], noResolve: true, strict: true };
const host = ts.createCompilerHost(sfOpts);
const orig = host.getSourceFile.bind(host);
host.getSourceFile = (fn, lv, ...r) => (fn === file ? ts.createSourceFile(fn, text, lv, true, (file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS)) : orig(fn, lv, ...r));
const program = ts.createProgram([file], sfOpts, host);
const sf = program.getSourceFile(file);
const checker = program.getTypeChecker();
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
const res = new Map();
const visit = (n) => {
  if (ts.isIdentifier(n) && L(n.getStart(sf)) >= +a && L(n.getStart(sf)) <= +b) {
    const p = n.parent;
    const skip = (ts.isPropertyAccessExpression(p) && p.name === n) || (ts.isPropertyAssignment(p) && p.name === n) || (ts.isQualifiedName(p) && p.right === n);
    if (!skip) {
      const s = checker.getSymbolAtLocation(n) || (ts.isShorthandPropertyAssignment(p) ? checker.getShorthandAssignmentValueSymbol(p) : null);
      const d = s && s.declarations && s.declarations[0];
      if (d && d.getSourceFile() === sf && (L(d.getStart(sf)) < +a || L(d.getStart(sf)) > +b)) {
        let top = d; while (top.parent && !ts.isSourceFile(top.parent)) top = top.parent;
        if (!ts.isSourceFile(d.parent) && !(ts.isVariableDeclaration(d) && ts.isSourceFile(d.parent.parent.parent)) && !ts.isImportSpecifier(d) && !ts.isImportClause(d) && !ts.isNamespaceImport(d)) res.set(n.text, (res.get(n.text) || 0) + 1);
      }
    }
  }
  ts.forEachChild(n, visit);
};
visit(sf);
console.log([...res.entries()].map(([k, v]) => `${k}:${v}`).join(" "));
