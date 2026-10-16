import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import { readFileSync } from "node:fs";
const MARKER = /^\s*\/\/ ?#(end)?region (.*)$/;
export function load(file, textOverride) {
  const text = textOverride ?? readFileSync(file, "utf8");
  const host = ts.createCompilerHost({});
  const sfOrig = host.getSourceFile;
  const fname = file;
  host.getSourceFile = (name, lang, ...rest) => (name === fname ? ts.createSourceFile(name, text, lang, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS) : sfOrig.call(host, name, lang, ...rest));
  host.fileExists = (n) => n === fname || ts.sys.fileExists(n);
  host.readFile = (n) => (n === fname ? text : ts.sys.readFile(n));
  const program = ts.createProgram([file], { noResolve: true, target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true, noEmit: true, skipLibCheck: true, resolveJsonModule: true, jsx: ts.JsxEmit.Preserve, types: [] }, host);
  const checker = program.getTypeChecker();
  const sf = program.getSourceFile(file);
  const lines = text.split("\n");
  const lineStarts = []; { let o = 0; for (const l of lines) { lineStarts.push(o); o += l.length + 1; } }
  const lineOf = (pos) => sf.getLineAndCharacterOfPosition(pos).line + 1;
  const regions = []; const stack = [];
  lines.forEach((l, i) => {
    const m = l.match(MARKER);
    if (!m) return;
    if (!m[1]) { const r = { name: m[2], start: i + 1, startText: l, parent: stack[stack.length - 1] ?? null }; regions.push(r); stack.push(r); }
    else { const idx = [...stack].reverse().findIndex((x) => x.name === m[2]); if (idx >= 0) { const r = stack[stack.length - 1 - idx]; r.end = i + 1; r.endText = l; stack.splice(stack.length - 1 - idx); } }
  });
  const markerLines = new Set(); for (const r of regions) { markerLines.add(r.start); if (r.end) markerLines.add(r.end); }
  const regionPathAt = (line) => { let best = null; for (const r of regions) if (r.end && line > r.start && line < r.end && (!best || r.start > best.start)) best = r; const path = []; for (let r = best; r; r = r.parent) path.unshift(r); return path; };
  const tops = [];
  const declToTop = new Map();
  for (const st of sf.statements) {
    const comments = ts.getLeadingCommentRanges(text, st.getFullStart()) ?? [];
    let spanPos = st.getStart();
    let first = null;
    for (const c of comments) { const body = text.slice(c.pos, c.end); if (MARKER.test(body.split("\n")[0]) && c.kind === ts.SyntaxKind.SingleLineCommentTrivia) { first = null; continue; } if (first === null) first = c.pos; }
    if (first !== null) spanPos = first;
    const startLine = lineOf(spanPos), endLine = lineOf(st.end), codeLine = lineOf(st.getStart());
    const names = [];
    if (ts.isVariableStatement(st)) for (const d of st.declarationList.declarations) { const walk = (n) => { if (ts.isIdentifier(n)) names.push(n.text); else n.elements?.forEach((e) => walk(e.name)); }; walk(d.name); }
    else if ((ts.isFunctionDeclaration(st) || ts.isClassDeclaration(st) || ts.isInterfaceDeclaration(st) || ts.isTypeAliasDeclaration(st) || ts.isEnumDeclaration(st)) && st.name) names.push(st.name.text);
    const exported = !!st.modifiers?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword);
    const path = regionPathAt(codeLine);
    const t = { st, startLine, endLine, codeLine, spanPos, names, exported, path, region: path.length ? path[path.length - 1].name : "-", kind: ts.SyntaxKind[st.kind], deps: new Map(), refs: new Map(), importUses: new Map(), isType: ts.isTypeAliasDeclaration(st) || ts.isInterfaceDeclaration(st), isImport: ts.isImportDeclaration(st), start: startLine, end: endLine };
    tops.push(t); declToTop.set(st, t);
  }
  const isTopLevelDeclaration = (d) => { if (d.parent === sf) return true; if (ts.isVariableDeclaration(d) && d.parent && ts.isVariableDeclarationList(d.parent) && d.parent.parent && ts.isVariableStatement(d.parent.parent) && d.parent.parent.parent === sf) return true; if (ts.isBindingElement(d)) { let p = d; while (p && !ts.isVariableDeclaration(p)) p = p.parent; return !!p && isTopLevelDeclaration(p); } return false; };
  const topOfSymbol = (sym) => { if (!sym) return null; if (sym.flags & ts.SymbolFlags.Alias) return null; for (const d of sym.declarations ?? []) { if (!isTopLevelDeclaration(d)) continue; let p = d; while (p && p.parent && p.parent !== sf) p = p.parent; const t = declToTop.get(p); if (t && !t.isImport) return t; } return null; };
  const importDeclOfSymbol = (sym) => { if (!sym || !(sym.flags & ts.SymbolFlags.Alias)) return null; for (const d of sym.declarations ?? []) { if (ts.isImportSpecifier(d) || ts.isImportClause(d) || ts.isNamespaceImport(d)) return d; } return null; };
  for (const t of tops) {
    const walk = (n, lazy, inT) => {
      if (ts.isIdentifier(n)) {
        let sym = checker.getSymbolAtLocation(n);
        if (n.parent && ts.isShorthandPropertyAssignment(n.parent) && n.parent.name === n) sym = checker.getShorthandAssignmentValueSymbol(n.parent);
        const d = topOfSymbol(sym);
        if (d && d !== t) {
          const prev = t.deps.get(d);
          t.deps.set(d, { eager: (prev?.eager ?? false) || (!lazy && !inT), typeOnly: (prev?.typeOnly ?? true) && inT });
          const pr = t.refs.get(n.text);
          t.refs.set(n.text, { top: d, typeOnly: (pr?.typeOnly ?? true) && inT });
        } else {
          const imp = importDeclOfSymbol(sym);
          if (imp) { const pu = t.importUses.get(imp); t.importUses.set(imp, { typeOnly: (pu?.typeOnly ?? true) && inT }); }
        }
      }
      let l = lazy, it = inT;
      if (ts.isFunctionLike(n) || ts.isMethodDeclaration(n) || ts.isGetAccessor(n) || ts.isSetAccessor(n) || ts.isConstructorDeclaration(n)) l = true;
      if (ts.isTypeNode(n) && !ts.isExpressionWithTypeArguments(n)) it = true;
      if (ts.isTypeAliasDeclaration(n) || ts.isInterfaceDeclaration(n)) it = true;
      ts.forEachChild(n, (c) => walk(c, l, it));
    };
    if (!t.isImport) walk(t.st, false, false);
  }
  return { tops, sf, text, lines, lineStarts, regions, markerLines, checker, program, ts };
}
export function scc(nodes, succ) {
  let idx = 0; const stack = [], on = new Set(), index = new Map(), low = new Map(), out = [];
  const strong = (v) => { index.set(v, idx); low.set(v, idx); idx++; stack.push(v); on.add(v); for (const w of succ(v)) { if (!index.has(w)) { strong(w); low.set(v, Math.min(low.get(v), low.get(w))); } else if (on.has(w)) low.set(v, Math.min(low.get(v), index.get(w))); } if (low.get(v) === index.get(v)) { const c = []; let w; do { w = stack.pop(); on.delete(w); c.push(w); } while (w !== v); out.push(c); } };
  for (const v of nodes) if (!index.has(v)) strong(v);
  return out;
}
