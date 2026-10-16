const { analyze, ts } = require("./units.cjs");
for (const f of process.argv.slice(2)) {
  const { sf, checker } = analyze(f);
  const decls = new Map();
  for (const s of sf.statements) {
    if (!ts.isImportDeclaration(s) || !s.importClause) continue;
    const c = s.importClause;
    if (c.name) decls.set(checker.getSymbolAtLocation(c.name), c.name.text);
    if (c.namedBindings && ts.isNamespaceImport(c.namedBindings)) decls.set(checker.getSymbolAtLocation(c.namedBindings.name), c.namedBindings.name.text);
    if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const el of c.namedBindings.elements) decls.set(checker.getSymbolAtLocation(el.name), el.name.text);
  }
  const used = new Set();
  const visit = (n) => {
    if (ts.isIdentifier(n)) { const sy = checker.getSymbolAtLocation(n); if (sy && decls.has(sy) && !(ts.isImportSpecifier(n.parent) || ts.isImportClause(n.parent) || ts.isNamespaceImport(n.parent))) used.add(sy); }
    ts.forEachChild(n, visit);
  };
  for (const s of sf.statements) if (!ts.isImportDeclaration(s)) visit(s);
  const unused = [...decls].filter(([sy]) => !used.has(sy)).map(([, n]) => n);
  console.log(f.split("/").slice(-2).join("/"), "unused imports:", unused.join(",") || "none");
}
