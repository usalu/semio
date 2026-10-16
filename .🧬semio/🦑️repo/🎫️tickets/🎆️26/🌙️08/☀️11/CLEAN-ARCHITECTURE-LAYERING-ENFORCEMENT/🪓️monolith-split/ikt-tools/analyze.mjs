import { fileURLToPath } from "node:url";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import fs from "node:fs";
import path from "node:path";

export function analyze(file) {
  const text = process.env.SPLIT_SOURCE ? fs.readFileSync(process.env.SPLIT_SOURCE, "utf8") : fs.readFileSync(file, "utf8");
  const options = {
    jsx: ts.JsxEmit.ReactJSX, target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext,
    moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true, noResolve: true,
    noEmit: true, skipLibCheck: true, types: [], lib: [],
  };
  const host = ts.createCompilerHost(options);
  const getSourceFile = host.getSourceFile.bind(host);
  host.getSourceFile = (f, l, ...r) => (f === file ? ts.createSourceFile(f, text, l, true, ts.ScriptKind.TSX) : getSourceFile(f, l, ...r));
  const program = ts.createProgram([file], options, host);
  const sf = program.getSourceFile(file);
  const checker = program.getTypeChecker();
  const lineOf = (pos) => sf.getLineAndCharacterOfPosition(pos).line + 1;

  const imports = new Map(); // local name -> {decl index, imported, typeOnly, kind}
  const importDecls = [];
  const statements = [];
  const topOfNode = (n) => { while (n && n.parent !== sf) n = n.parent; return n; };

  sf.statements.forEach((st, i) => {
    if (ts.isImportDeclaration(st)) {
      const d = { spec: st.moduleSpecifier.text, start: st.getStart(), end: st.end, typeOnly: !!st.importClause?.isTypeOnly, names: [], defaultName: null, nsName: null, attrs: st.attributes ? st.attributes.getText() : null };
      const ic = st.importClause;
      if (ic) {
        if (ic.name) { d.defaultName = ic.name.text; imports.set(ic.name.text, { d, kind: "default" }); }
        if (ic.namedBindings) {
          if (ts.isNamespaceImport(ic.namedBindings)) { d.nsName = ic.namedBindings.name.text; imports.set(d.nsName, { d, kind: "ns" }); }
          else for (const el of ic.namedBindings.elements) {
            const e = { local: el.name.text, imported: (el.propertyName ?? el.name).text, typeOnly: el.isTypeOnly || d.typeOnly };
            d.names.push(e); imports.set(e.local, { d, kind: "named", e });
          }
        }
      } else d.bare = true;
      importDecls.push(d);
    }
  });

  const units = [];
  const nameToUnit = new Map();
  let prevEnd = 0;
  sf.statements.forEach((st, i) => {
    const full = st.getFullStart();
    if (ts.isImportDeclaration(st)) { prevEnd = st.end; return; }
    const u = { idx: units.length, stmt: st, kind: ts.SyntaxKind[st.kind], names: [], exported: false, isType: false, fullStart: full, start: st.getStart(), end: st.end, exportDecl: false };
    const mods = ts.canHaveModifiers(st) ? ts.getModifiers(st) : undefined;
    u.exported = !!mods?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword);
    u.isDefault = !!mods?.some((m) => m.kind === ts.SyntaxKind.DefaultKeyword);
    if (ts.isVariableStatement(st)) for (const d of st.declarationList.declarations) { if (ts.isIdentifier(d.name)) u.names.push(d.name.text); else collectBinding(d.name, u.names); }
    else if ((ts.isFunctionDeclaration(st) || ts.isClassDeclaration(st) || ts.isEnumDeclaration(st)) && st.name) u.names.push(st.name.text);
    else if (ts.isInterfaceDeclaration(st) || ts.isTypeAliasDeclaration(st)) { u.names.push(st.name.text); u.isType = true; }
    else if (ts.isModuleDeclaration(st)) u.names.push(st.name.text);
    else if (ts.isExportDeclaration(st)) u.exportDecl = true;
    else if (ts.isExportAssignment(st)) u.exportDecl = true;
    units.push(u);
    prevEnd = st.end;
  });
  function collectBinding(b, out) { for (const el of b.elements) { if (ts.isOmittedExpression(el)) continue; if (ts.isIdentifier(el.name)) out.push(el.name.text); else collectBinding(el.name, out); } }

  // merge units with equal names (overloads, interface+const merges)
  const parent = units.map((_, i) => i);
  const find = (x) => (parent[x] === x ? x : (parent[x] = find(parent[x])));
  const owner = new Map();
  for (const u of units) for (const n of u.names) { if (owner.has(n)) parent[find(u.idx)] = find(owner.get(n)); else owner.set(n, u.idx); }

  const stmtToUnit = new Map(units.map((u) => [u.stmt, u]));
  const importRefs = units.map(() => new Set());
  const unitRefs = units.map(() => new Set());
  const nameRefs = units.map(() => new Map());
  const jsx = units.map(() => false);
  const impNames = units.map(() => new Set());
  const valueUse = units.map(() => new Set());

  const visit = (u, n) => {
    if (ts.isIdentifier(n)) {
      const p = n.parent;
      if (p && ts.isPropertyAccessExpression(p) && p.name === n) return;
      if (p && ts.isQualifiedName(p) && p.right === n) return;
      if (p && (ts.isPropertyAssignment(p) || ts.isPropertySignature(p) || ts.isMethodDeclaration(p) || ts.isPropertyDeclaration(p) || ts.isMethodSignature(p) || ts.isEnumMember(p) || ts.isGetAccessor(p) || ts.isSetAccessor(p)) && p.name === n) return;
      if (p && ts.isBindingElement(p) && p.propertyName === n) return;
      if (p && (ts.isJsxAttribute(p)) && p.name === n) return;
      if (p && ts.isLabeledStatement(p)) return;
      let sym = checker.getSymbolAtLocation(n);
      if (p && ts.isShorthandPropertyAssignment(p) && p.name === n) sym = checker.getShorthandAssignmentValueSymbol(p);
      if (!sym) return;
      const decls = sym.declarations ?? [];
      if (decls.length === 0) return;
      for (const d of decls) {
        if (ts.isImportSpecifier(d) || ts.isImportClause(d) || ts.isNamespaceImport(d)) { importRefs[u.idx].add(sym.name); break; }
        const top = topOfNode(d);
        if (top && stmtToUnit.has(top)) {
          const tu = stmtToUnit.get(top);
          // exclude local shadowing: only count if declaration is directly the top-level one
          if (isTopLevelDecl(d, top)) { unitRefs[u.idx].add(tu.idx); const set = nameRefs[u.idx].get(tu.idx) ?? new Set(); set.add(sym.name); nameRefs[u.idx].set(tu.idx, set); }
        }
      }
    }
    if (ts.isJsxElement(n) || ts.isJsxSelfClosingElement(n) || ts.isJsxFragment(n)) jsx[u.idx] = true;
    ts.forEachChild(n, (c) => visit(u, c));
  };
  function isTopLevelDecl(d, top) {
    if (ts.isVariableDeclaration(d)) return d.parent.parent === top;
    if (ts.isFunctionDeclaration(d) || ts.isClassDeclaration(d) || ts.isInterfaceDeclaration(d) || ts.isTypeAliasDeclaration(d) || ts.isEnumDeclaration(d) || ts.isModuleDeclaration(d)) return d === top;
    if (ts.isBindingElement(d)) { let p = d; while (p && !ts.isVariableDeclaration(p)) p = p.parent; return p && p.parent.parent === top; }
    return false;
  }
  for (const u of units) {
    if (u.exportDecl) {
      // export { a, b } [from "x"]
      continue;
    }
    ts.forEachChild(u.stmt, (c) => visit(u, c));
  }
  return { text, sf, units, importDecls, imports, importRefs, unitRefs, nameRefs, jsx, find, lineOf, ts };
}

if (path.resolve(process.argv[1]) === fileURLToPath(import.meta.url) && process.argv[2]) {
  const a = analyze(process.argv[2]);
  const out = a.units.map((u) => ({ idx: u.idx, kind: u.kind, names: u.names, exported: u.exported, isType: u.isType, lines: [a.lineOf(u.fullStart + (a.text.slice(u.fullStart).match(/^\s*/)[0].length)), a.lineOf(u.end)], refs: [...a.unitRefs[u.idx]], imps: [...a.importRefs[u.idx]] }));
  fs.writeFileSync(process.argv[3], JSON.stringify(out));
  console.log("units", out.length, "imports", a.importDecls.length);
}
