import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
import { dirname, join, relative, normalize, sep, posix } from "node:path";

const require = createRequire(import.meta.url);
export const ts: typeof import("typescript") = require(`${process.env.HOME}/.bun/install/cache/typescript@5.9.3@@@1/lib/typescript.js`);
export const ROOT = "/Users/ueli/Documents/semio";

export type Decl = {
  index: number;
  names: string[];
  exported: boolean;
  kind: string;
  start: number;
  end: number;
  startLine: number;
  endLine: number;
  deps: Set<string>;
  typeOnly: boolean;
};

export type ImportBinding = { local: string; imported: string; typeOnly: boolean; kind: "named" | "default" | "namespace" };
export type ImportInfo = { index: number; spec: string; start: number; end: number; bindings: ImportBinding[]; declTypeOnly: boolean; sideEffect: boolean; text: string };

export type FileAnalysis = {
  path: string;
  text: string;
  sf: import("typescript").SourceFile;
  imports: ImportInfo[];
  decls: Decl[];
  exportsFrom: { index: number; spec: string | null; names: string[]; typeOnly: boolean; star: boolean; start: number; end: number; text: string }[];
  topNames: Map<string, number>;
};

export function parse(path: string, text?: string): import("typescript").SourceFile {
  const src = text ?? readFileSync(join(ROOT, path), "utf8");
  const kind = path.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS;
  return ts.createSourceFile(path, src, ts.ScriptTarget.Latest, true, kind);
}

function declaredNames(node: import("typescript").Node): string[] {
  const out: string[] = [];
  const addBinding = (b: import("typescript").BindingName) => {
    if (ts.isIdentifier(b)) out.push(b.text);
    else for (const e of b.elements) if (!ts.isOmittedExpression(e)) addBinding(e.name);
  };
  if (ts.isVariableStatement(node)) for (const d of node.declarationList.declarations) addBinding(d.name);
  else if ((ts.isFunctionDeclaration(node) || ts.isClassDeclaration(node) || ts.isInterfaceDeclaration(node) || ts.isTypeAliasDeclaration(node) || ts.isEnumDeclaration(node) || ts.isModuleDeclaration(node)) && node.name) out.push(node.name.text);
  return out;
}

function hasExport(node: import("typescript").Node): boolean {
  return !!(ts.canHaveModifiers(node) && ts.getModifiers(node)?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword));
}

export function collectIdentifiers(node: import("typescript").Node, into: Set<string>): void {
  const visit = (n: import("typescript").Node) => {
    if (ts.isIdentifier(n)) {
      const p = n.parent;
      if (p && ts.isPropertyAccessExpression(p) && p.name === n) return;
      if (p && ts.isQualifiedName(p) && p.right === n) return;
      if (p && (ts.isPropertyAssignment(p) || ts.isPropertySignature(p) || ts.isMethodDeclaration(p) || ts.isMethodSignature(p) || ts.isPropertyDeclaration(p) || ts.isEnumMember(p)) && p.name === n) return;
      if (p && ts.isBindingElement(p) && p.propertyName === n) return;
      into.add(n.text);
    }
    ts.forEachChild(n, visit);
  };
  visit(node);
}

function exactDeps(checker: import("typescript").TypeChecker, sf: import("typescript").SourceFile, node: import("typescript").Node, into: Set<string>): void {
  const visit = (n: import("typescript").Node) => {
    if (ts.isIdentifier(n)) {
      const p = n.parent;
      const skip = (p && ts.isPropertyAccessExpression(p) && p.name === n) || (p && ts.isQualifiedName(p) && p.right === n) || (p && (ts.isPropertyAssignment(p) || ts.isPropertySignature(p) || ts.isMethodSignature(p) || ts.isEnumMember(p)) && p.name === n) || (p && (ts.isMethodDeclaration(p) || ts.isPropertyDeclaration(p) || ts.isGetAccessor(p) || ts.isSetAccessor(p)) && p.name === n) || (p && ts.isBindingElement(p) && p.propertyName === n) || (p && ts.isLabeledStatement(p)) || (p && (ts.isBreakOrContinueStatement(p)));
      if (!skip) {
        let sym = p && ts.isShorthandPropertyAssignment(p) && p.name === n ? checker.getShorthandAssignmentValueSymbol(p) : checker.getSymbolAtLocation(n);
        if (sym) {
          const decls = sym.declarations ?? [];
          const topLevel = decls.some((d) => d.getSourceFile() === sf && (d.parent === sf || (ts.isVariableDeclaration(d) && d.parent.parent.parent === sf) || ts.isImportSpecifier(d) || ts.isImportClause(d) || ts.isNamespaceImport(d)));
          if (topLevel) into.add(n.text);
        } else if (p && !ts.isVariableDeclaration(p) && !ts.isParameter(p)) {
          into.add("?" + n.text);
        }
      }
    }
    ts.forEachChild(n, visit);
  };
  visit(node);
}

export function leadingStart(text: string, node: import("typescript").Node, sf: import("typescript").SourceFile, prevEnd: number): number {
  const full = node.getFullStart();
  let s = Math.max(full, prevEnd);
  const ranges = ts.getLeadingCommentRanges(text, s) ?? [];
  const nodeStart = node.getStart(sf);
  if (ranges.length) {
    s = ranges[0].pos;
  } else {
    s = nodeStart;
  }
  return s;
}

function exactReferences(path: string, text: string): { checker: import("typescript").TypeChecker; sf: import("typescript").SourceFile } {
  const options: import("typescript").CompilerOptions = { noResolve: true, noLib: true, target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, allowJs: false, skipLibCheck: true, types: [] };
  const host = ts.createCompilerHost(options, true);
  const kind = path.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS;
  const original = host.getSourceFile.bind(host);
  host.getSourceFile = (fileName, languageVersion, ...rest) => (fileName === path ? ts.createSourceFile(path, text, languageVersion, true, kind) : original(fileName, languageVersion, ...rest));
  host.fileExists = (f) => f === path;
  host.readFile = (f) => (f === path ? text : undefined);
  const program = ts.createProgram({ rootNames: [path], options, host });
  return { checker: program.getTypeChecker(), sf: program.getSourceFile(path)! };
}

export function analyze(path: string, textIn?: string, exact = true): FileAnalysis {
  const text = textIn ?? readFileSync(join(ROOT, path), "utf8");
  let sf = parse(path, text);
  let checker: import("typescript").TypeChecker | null = null;
  if (exact) {
    const r = exactReferences(path, text);
    sf = r.sf;
    checker = r.checker;
  }
  const imports: ImportInfo[] = [];
  const decls: Decl[] = [];
  const exportsFrom: FileAnalysis["exportsFrom"] = [];
  const topNames = new Map<string, number>();
  let prevEnd = 0;
  sf.statements.forEach((st, index) => {
    const start = leadingStart(text, st, sf, prevEnd);
    const end = st.getEnd();
    if (ts.isImportDeclaration(st)) {
      const bindings: ImportBinding[] = [];
      const ic = st.importClause;
      if (ic) {
        if (ic.name) bindings.push({ local: ic.name.text, imported: "default", typeOnly: ic.isTypeOnly, kind: "default" });
        if (ic.namedBindings) {
          if (ts.isNamespaceImport(ic.namedBindings)) bindings.push({ local: ic.namedBindings.name.text, imported: "*", typeOnly: ic.isTypeOnly, kind: "namespace" });
          else for (const e of ic.namedBindings.elements) bindings.push({ local: e.name.text, imported: (e.propertyName ?? e.name).text, typeOnly: ic.isTypeOnly || e.isTypeOnly, kind: "named" });
        }
      }
      imports.push({ index, spec: (st.moduleSpecifier as import("typescript").StringLiteral).text, start, end, bindings, declTypeOnly: !!ic?.isTypeOnly, sideEffect: !ic, text: text.slice(st.getStart(sf), end) });
    } else if (ts.isExportDeclaration(st)) {
      const names: string[] = [];
      let star = false;
      if (st.exportClause && ts.isNamedExports(st.exportClause)) for (const e of st.exportClause.elements) names.push(e.name.text);
      else if (!st.exportClause) star = true;
      else if (st.exportClause && ts.isNamespaceExport(st.exportClause)) names.push(st.exportClause.name.text);
      exportsFrom.push({ index, spec: st.moduleSpecifier ? (st.moduleSpecifier as import("typescript").StringLiteral).text : null, names, typeOnly: st.isTypeOnly, star, start, end, text: text.slice(st.getStart(sf), end) });
    } else {
      const names = declaredNames(st);
      const deps = new Set<string>();
      if (checker) exactDeps(checker, sf, st, deps);
      else collectIdentifiers(st, deps);
      const sl = sf.getLineAndCharacterOfPosition(start).line + 1;
      const el = sf.getLineAndCharacterOfPosition(end).line + 1;
      const typeOnly = ts.isInterfaceDeclaration(st) || ts.isTypeAliasDeclaration(st);
      decls.push({ index, names, exported: hasExport(st), kind: ts.SyntaxKind[st.kind], start, end, startLine: sl, endLine: el, deps, typeOnly });
      for (const n of names) topNames.set(n, decls.length - 1);
    }
    prevEnd = end;
  });
  return { path, text, sf, imports, decls, exportsFrom, topNames };
}

export function relSpec(fromFile: string, toFile: string): string {
  let r = posix.relative(dirname(fromFile).split(sep).join("/"), toFile.split(sep).join("/"));
  if (!r.startsWith(".")) r = "./" + r;
  return r;
}

export function nl(text: string): number {
  let c = 0;
  for (let i = 0; i < text.length; i++) if (text.charCodeAt(i) === 10) c++;
  return c;
}
