import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
const require = createRequire(import.meta.url);
export const ts: typeof import("typescript") = require("/Users/ueli/.cache/c6repo2/typescript/lib/typescript.js");

export interface ImportBinding { local: string; imported: string; typeOnly: boolean; spec: string; kind: "named" | "default" | "namespace"; stmt: number; }
export interface Unit {
  id: number;
  stmts: number[];
  names: Set<string>;
  start: number;
  end: number;
  startLine: number;
  endLine: number;
  lines: number;
  region: string;
  deps: Set<number>;
  uses: Set<string>;
  refNames: Map<number, Set<string>>;
  usesImportLocals: Set<string>;
  effect: boolean;
  exported: boolean;
  usesMeta: boolean;
  kind: string;
}
export interface Parsed {
  file: string;
  text: string;
  sf: import("typescript").SourceFile;
  bindings: Map<string, ImportBinding>;
  importStmts: import("typescript").ImportDeclaration[];
  reexports: import("typescript").ExportDeclaration[];
  localExportLists: import("typescript").ExportDeclaration[];
  units: Unit[];
  stmtUnit: number[];
  stmtInfo: StmtInfo[];
}
export interface StmtInfo { node: import("typescript").Statement; names: string[]; index: number; kind: string; effect: boolean; exported: boolean; }

function bindingNames(name: import("typescript").BindingName, out: string[]) {
  if (ts.isIdentifier(name)) { out.push(name.text); return; }
  for (const el of name.elements) if (ts.isBindingElement(el)) bindingNames(el.name, out);
}

export function declaredNames(st: import("typescript").Statement): string[] {
  const out: string[] = [];
  if (ts.isVariableStatement(st)) for (const d of st.declarationList.declarations) bindingNames(d.name, out);
  else if ((ts.isFunctionDeclaration(st) || ts.isClassDeclaration(st) || ts.isInterfaceDeclaration(st) || ts.isTypeAliasDeclaration(st) || ts.isEnumDeclaration(st)) && st.name) out.push(st.name.text);
  else if (ts.isModuleDeclaration(st) && ts.isIdentifier(st.name)) out.push(st.name.text);
  return out;
}

function hasExport(st: import("typescript").Statement) {
  const mods = ts.canHaveModifiers(st) ? ts.getModifiers(st) : undefined;
  return !!mods?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword);
}

export function parseFile(file: string, override?: string): Parsed {
  const text = override ?? readFileSync(file, "utf8");
  const options: import("typescript").CompilerOptions = { noResolve: true, noLib: true, target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, allowJs: false, types: [], skipLibCheck: true, isolatedModules: true, jsx: ts.JsxEmit.Preserve };
  const host = ts.createCompilerHost(options, true);
  const orig = host.getSourceFile.bind(host);
  host.getSourceFile = (fn, lang, ...rest) => (fn === file ? ts.createSourceFile(fn, text, lang, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS) : orig(fn, lang, ...rest));
  const program = ts.createProgram([file], options, host);
  const sf = program.getSourceFile(file)!;
  const checker = program.getTypeChecker();
  const stmts = sf.statements;
  const bindings = new Map<string, ImportBinding>();
  const importStmts: import("typescript").ImportDeclaration[] = [];
  const reexports: import("typescript").ExportDeclaration[] = [];
  const localExportLists: import("typescript").ExportDeclaration[] = [];
  const stmtInfo: StmtInfo[] = [];
  const topSymbolStmt = new Map<import("typescript").Symbol, number>();
  const importSymbolLocal = new Map<import("typescript").Symbol, string>();

  stmts.forEach((st, i) => {
    if (ts.isImportDeclaration(st)) {
      importStmts.push(st);
      const spec = (st.moduleSpecifier as import("typescript").StringLiteral).text;
      const ic = st.importClause;
      if (ic) {
        const clauseType = ic.isTypeOnly;
        if (ic.name) bindings.set(ic.name.text, { local: ic.name.text, imported: "default", typeOnly: clauseType, spec, kind: "default", stmt: i });
        if (ic.namedBindings) {
          if (ts.isNamespaceImport(ic.namedBindings)) bindings.set(ic.namedBindings.name.text, { local: ic.namedBindings.name.text, imported: "*", typeOnly: clauseType, spec, kind: "namespace", stmt: i });
          else for (const el of ic.namedBindings.elements) bindings.set(el.name.text, { local: el.name.text, imported: (el.propertyName ?? el.name).text, typeOnly: clauseType || el.isTypeOnly, spec, kind: "named", stmt: i });
        }
      }
    }
    if (ts.isExportDeclaration(st)) (st.moduleSpecifier ? reexports : localExportLists).push(st);
  });

  stmts.forEach((st, i) => {
    const names = declaredNames(st);
    const decl = ts.isImportDeclaration(st) || ts.isExportDeclaration(st);
    const isDecl = names.length > 0 || ts.isExportAssignment(st);
    let kind = ts.SyntaxKind[st.kind];
    if (ts.isVariableStatement(st)) kind = (st.declarationList.flags & ts.NodeFlags.Const) ? "const" : (st.declarationList.flags & ts.NodeFlags.Let) ? "let" : "var";
    stmtInfo.push({ node: st, names, index: i, kind, effect: !decl && !isDecl, exported: hasExport(st) });
  });

  const symbolStmt = (sym: import("typescript").Symbol): number | undefined => {
    for (const d of sym.declarations ?? []) {
      let n: import("typescript").Node = d;
      while (n.parent && n.parent !== sf) n = n.parent;
      if (n.parent === sf) {
        const idx = stmts.indexOf(n as import("typescript").Statement);
        if (idx >= 0 && !ts.isImportDeclaration(n)) return idx;
      }
    }
    return undefined;
  };
  const allSymbolStmts = (sym: import("typescript").Symbol): number[] => {
    const out: number[] = [];
    for (const d of sym.declarations ?? []) {
      let n: import("typescript").Node = d;
      while (n.parent && n.parent !== sf) n = n.parent;
      if (n.parent === sf) {
        const idx = stmts.indexOf(n as import("typescript").Statement);
        if (idx >= 0 && !ts.isImportDeclaration(n) && !out.includes(idx)) out.push(idx);
      }
    }
    return out;
  };

  const stmtRefs: { stmts: Set<number>; imports: Set<string>; meta: boolean; assignedTop: Set<number>; names: Map<number, Set<string>> }[] = stmts.map(() => ({ stmts: new Set(), imports: new Set(), meta: false, assignedTop: new Set(), names: new Map() }));

  const isAssignTarget = (id: import("typescript").Identifier) => {
    let n: import("typescript").Node = id;
    let p = n.parent;
    while (p && (ts.isParenthesizedExpression(p) || ts.isArrayLiteralExpression(p) || ts.isSpreadElement(p) || ts.isObjectLiteralExpression(p) || ts.isPropertyAssignment(p) || ts.isShorthandPropertyAssignment(p))) {
      if (ts.isPropertyAssignment(p) && p.name === n) return false;
      n = p; p = n.parent;
    }
    if (!p) return false;
    if (ts.isBinaryExpression(p) && p.left === n && p.operatorToken.kind >= ts.SyntaxKind.FirstAssignment && p.operatorToken.kind <= ts.SyntaxKind.LastAssignment) return true;
    if ((ts.isPrefixUnaryExpression(p) || ts.isPostfixUnaryExpression(p)) && (p.operator === ts.SyntaxKind.PlusPlusToken || p.operator === ts.SyntaxKind.MinusMinusToken)) return true;
    if ((ts.isForOfStatement(p) || ts.isForInStatement(p)) && p.initializer === n) return true;
    return false;
  };

  const visit = (node: import("typescript").Node, si: number) => {
    if (ts.isIdentifier(node)) {
      const par = node.parent;
      let skip = false;
      if (par) {
        if ((ts.isPropertyAccessExpression(par) && par.name === node) || (ts.isQualifiedName(par) && par.right === node)) skip = true;
        else if ((ts.isPropertyAssignment(par) || ts.isPropertySignature(par) || ts.isMethodSignature(par) || ts.isPropertyDeclaration(par) || ts.isMethodDeclaration(par) || ts.isGetAccessor(par) || ts.isSetAccessor(par) || ts.isEnumMember(par)) && par.name === node) skip = true;
        else if (ts.isBindingElement(par) && par.propertyName === node) skip = true;
        else if ((ts.isImportSpecifier(par) || ts.isExportSpecifier(par))) skip = true;
        else if (ts.isLabeledStatement(par) || ts.isBreakOrContinueStatement(par)) skip = true;
      }
      if (!skip) {
        let sym: import("typescript").Symbol | undefined;
        if (par && ts.isShorthandPropertyAssignment(par) && par.name === node) sym = checker.getShorthandAssignmentValueSymbol(par);
        else sym = checker.getSymbolAtLocation(node);
        if (par && ts.isExportSpecifier(par)) sym = undefined;
        if (sym) {
          if (sym.flags & ts.SymbolFlags.Alias) {
            const d = sym.declarations?.[0];
            if (d && (ts.isImportSpecifier(d) || ts.isImportClause(d) || ts.isNamespaceImport(d))) {
              const local = ts.isImportSpecifier(d) ? d.name.text : ts.isImportClause(d) ? d.name!.text : d.name.text;
              stmtRefs[si].imports.add(local);
            }
          } else {
            const targets = allSymbolStmts(sym);
            for (const t of targets) {
              if (t !== si) { stmtRefs[si].stmts.add(t); (stmtRefs[si].names.get(t) ?? stmtRefs[si].names.set(t, new Set()).get(t)!).add(sym.name); }
              if (isAssignTarget(node) && stmtInfo[t].kind === "let" || isAssignTarget(node) && stmtInfo[t].kind === "var") stmtRefs[si].assignedTop.add(t);
            }
          }
        } else if (bindings.has(node.text) && !skip) {
          // unresolved: fall through
        }
      }
    }
    if (ts.isMetaProperty(node) && node.keywordToken === ts.SyntaxKind.ImportKeyword) stmtRefs[si].meta = true;
    if (ts.isIdentifier(node) && (node.text === "__dirname" || node.text === "__filename")) stmtRefs[si].meta = true;
    ts.forEachChild(node, (c) => visit(c, si));
  };
  stmts.forEach((st, i) => {
    if (ts.isImportDeclaration(st)) return;
    visit(st, i);
  });

  const nameStmts = new Map<string, number[]>();
  stmtInfo.forEach((s) => s.names.forEach((n) => { (nameStmts.get(n) ?? nameStmts.set(n, []).get(n)!).push(s.index); }));

  const parent = stmts.map((_, i) => i);
  const find = (x: number): number => (parent[x] === x ? x : (parent[x] = find(parent[x])));
  const union = (a: number, b: number) => { a = find(a); b = find(b); if (a !== b) parent[Math.max(a, b)] = Math.min(a, b); };
  for (const [, arr] of nameStmts) for (let k = 1; k < arr.length; k++) union(arr[0], arr[k]);
  stmtRefs.forEach((r, i) => r.assignedTop.forEach((t) => union(i, t)));

  const groups = new Map<number, number[]>();
  stmts.forEach((st, i) => {
    if (ts.isImportDeclaration(st) || ts.isExportDeclaration(st)) return;
    const g = find(i);
    (groups.get(g) ?? groups.set(g, []).get(g)!).push(i);
  });

  const lineOf = (pos: number) => sf.getLineAndCharacterOfPosition(pos).line + 1;
  const regionAt = regionIndex(text);
  const units: Unit[] = [];
  const stmtUnit: number[] = stmts.map(() => -1);
  for (const [, members] of [...groups.entries()].sort((a, b) => a[1][0] - b[1][0])) {
    const id = units.length;
    members.sort((a, b) => a - b);
    const first = stmts[members[0]], last = stmts[members[members.length - 1]];
    const start = first.getStart(sf), end = last.getEnd();
    const u: Unit = {
      id, stmts: members, names: new Set(), start, end,
      startLine: lineOf(start), endLine: lineOf(end), lines: 0,
      region: regionAt(start), deps: new Set(), uses: new Set(), refNames: new Map(), usesImportLocals: new Set(),
      effect: members.every((m) => stmtInfo[m].effect), exported: members.some((m) => stmtInfo[m].exported),
      usesMeta: members.some((m) => stmtRefs[m].meta), kind: stmtInfo[members[0]].kind,
    };
    for (const m of members) { stmtInfo[m].names.forEach((n) => u.names.add(n)); stmtUnit[m] = id; }
    u.lines = members.reduce((s, m) => s + (lineOf(stmts[m].getEnd()) - lineOf(stmts[m].getStart(sf)) + 1), 0);
    units.push(u);
  }
  for (const u of units) {
    for (const m of u.stmts) {
      stmtRefs[m].stmts.forEach((t) => { const tu = stmtUnit[t]; if (tu >= 0 && tu !== u.id) u.deps.add(tu); });
      stmtRefs[m].names.forEach((ns, t) => { const tu = stmtUnit[t]; if (tu >= 0 && tu !== u.id) { const cur = u.refNames.get(tu) ?? u.refNames.set(tu, new Set()).get(tu)!; ns.forEach((n) => cur.add(n)); } });
      stmtRefs[m].imports.forEach((l) => u.usesImportLocals.add(l));
    }
  }
  return { file, text, sf, bindings, importStmts, reexports, localExportLists, units, stmtUnit, stmtInfo };
}

export function regionIndex(text: string): (pos: number) => string {
  const lines = text.split("\n");
  const marks: { pos: number; stack: string[] }[] = [];
  let pos = 0;
  const stack: string[] = [];
  for (const line of lines) {
    const m = line.match(/^\s*\/\/#region\s+(.*)$/);
    if (m) { stack.push(m[1].trim()); marks.push({ pos, stack: [...stack] }); }
    else if (/^\s*\/\/#endregion/.test(line)) { stack.pop(); marks.push({ pos, stack: [...stack] }); }
    pos += line.length + 1;
  }
  return (p: number) => {
    let lo = 0, hi = marks.length - 1, ans = -1;
    while (lo <= hi) { const mid = (lo + hi) >> 1; if (marks[mid].pos <= p) { ans = mid; lo = mid + 1; } else hi = mid - 1; }
    if (ans < 0) return "";
    return marks[ans].stack.join(" > ");
  };
}

export function sccs(n: number, edges: (i: number) => Iterable<number>): number[][] {
  const index = new Array(n).fill(-1), low = new Array(n).fill(0), on = new Array(n).fill(false);
  const st: number[] = [];
  let idx = 0;
  const out: number[][] = [];
  const strong = (v0: number) => {
    const work: { v: number; it: Iterator<number> }[] = [];
    const enter = (v: number) => { index[v] = low[v] = idx++; st.push(v); on[v] = true; work.push({ v, it: edges(v)[Symbol.iterator]() }); };
    enter(v0);
    while (work.length) {
      const top = work[work.length - 1];
      const nx = top.it.next();
      if (!nx.done) {
        const w = nx.value;
        if (index[w] < 0) enter(w);
        else if (on[w]) low[top.v] = Math.min(low[top.v], index[w]);
      } else {
        work.pop();
        if (work.length) { const p = work[work.length - 1].v; low[p] = Math.min(low[p], low[top.v]); }
        if (low[top.v] === index[top.v]) {
          const comp: number[] = [];
          let w: number;
          do { w = st.pop()!; on[w] = false; comp.push(w); } while (w !== top.v);
          out.push(comp);
        }
      }
    }
  };
  for (let v = 0; v < n; v++) if (index[v] < 0) strong(v);
  return out;
}
