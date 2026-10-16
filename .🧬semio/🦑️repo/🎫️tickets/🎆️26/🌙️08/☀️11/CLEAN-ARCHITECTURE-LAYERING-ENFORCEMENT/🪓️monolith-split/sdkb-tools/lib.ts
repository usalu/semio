/** 🔬️ Shared AST import extraction and resolution for the SDK-boundary codemod tools. Read-only helpers. */
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import { readFileSync, existsSync, statSync, realpathSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";

export const ROOT = "/Users/ueli/Documents/semio";
export const FRAMEWORK = "🧰️framework/";
export const SDK_PACKAGE = "@semio-tech/framework";
export const SDK_DIR = "🧰️framework/📦️packages/🟦️typescript";

export interface Named { imported: string; local: string; typeOnly: boolean }
export interface ImportRecord {
  file: string;
  kind: "import" | "export" | "dynamic" | "require" | "importType";
  spec: string;
  litStart: number;
  litEnd: number;
  stmtStart: number;
  stmtEnd: number;
  typeOnly: boolean;
  defaultName?: string;
  namespaceName?: string;
  named: Named[];
  sideEffect: boolean;
  exportStar: boolean;
  members?: string[];
  membersUnknown?: boolean;
  line: number;
  resolved: string | null;
}

const compilerOptions: ts.CompilerOptions = {
  target: ts.ScriptTarget.ESNext,
  module: ts.ModuleKind.ESNext,
  moduleResolution: ts.ModuleResolutionKind.Bundler,
  allowImportingTsExtensions: true,
  resolveJsonModule: true,
  customConditions: ["semio-source"],
  jsx: ts.JsxEmit.ReactJSX,
};

const cache = new Map<string, string | null>();

export function repoRelative(abs: string): string {
  return relative(ROOT, abs).normalize("NFC");
}

function isFile(p: string): boolean { try { return statSync(p).isFile(); } catch { return false; } }

export function resolveSpec(fileRel: string, spec: string): string | null {
  const key = dirname(fileRel) + "\0" + spec;
  if (cache.has(key)) return cache.get(key)!;
  let out: string | null = null;
  const clean = spec.replace(/\?.*$/, "");
  const from = join(ROOT, fileRel);
  if (clean.startsWith(".")) {
    const base = resolve(dirname(from), clean);
    for (const cand of [base, base + ".ts", base + ".tsx", base + "/🟦️.ts", base + "/🟦️.tsx"]) if (isFile(cand)) { out = repoRelative(realpathSync(cand)); break; }
  } else {
    const r = ts.resolveModuleName(clean, from, compilerOptions, ts.sys).resolvedModule;
    if (r) { try { out = repoRelative(realpathSync(r.resolvedFileName)); } catch { out = repoRelative(r.resolvedFileName); } }
    if (out && out.startsWith("node_modules/")) out = null;
  }
  cache.set(key, out);
  return out;
}

export function scanSource(fileRel: string, text: string): ImportRecord[] {
  const kind = fileRel.endsWith(".tsx") || fileRel.endsWith(".jsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS;
  const sf = ts.createSourceFile(fileRel, text, ts.ScriptTarget.ESNext, true, kind);
  const out: ImportRecord[] = [];
  const push = (rec: Omit<ImportRecord, "file" | "resolved" | "line">, lit: ts.StringLiteralLike) => {
    const line = sf.getLineAndCharacterOfPosition(lit.getStart(sf)).line + 1;
    out.push({ ...rec, file: fileRel, line, resolved: resolveSpec(fileRel, rec.spec) });
  };
  const visit = (node: ts.Node) => {
    if (ts.isImportDeclaration(node) && ts.isStringLiteral(node.moduleSpecifier)) {
      const lit = node.moduleSpecifier;
      const clause = node.importClause;
      const named: Named[] = [];
      let defaultName: string | undefined, namespaceName: string | undefined;
      if (clause) {
        if (clause.name) defaultName = clause.name.text;
        const nb = clause.namedBindings;
        if (nb && ts.isNamespaceImport(nb)) namespaceName = nb.name.text;
        else if (nb && ts.isNamedImports(nb)) for (const el of nb.elements) named.push({ imported: (el.propertyName ?? el.name).text, local: el.name.text, typeOnly: el.isTypeOnly });
      }
      push({ kind: "import", spec: lit.text, litStart: lit.getStart(sf), litEnd: lit.getEnd(), stmtStart: node.getStart(sf), stmtEnd: node.getEnd(), typeOnly: !!clause?.isTypeOnly, defaultName, namespaceName, named, sideEffect: !clause, exportStar: false }, lit);
    } else if (ts.isExportDeclaration(node) && node.moduleSpecifier && ts.isStringLiteral(node.moduleSpecifier)) {
      const lit = node.moduleSpecifier;
      const named: Named[] = [];
      let namespaceName: string | undefined;
      const ec = node.exportClause;
      if (ec && ts.isNamedExports(ec)) for (const el of ec.elements) named.push({ imported: (el.propertyName ?? el.name).text, local: el.name.text, typeOnly: el.isTypeOnly });
      if (ec && ts.isNamespaceExport(ec)) namespaceName = ec.name.text;
      push({ kind: "export", spec: lit.text, litStart: lit.getStart(sf), litEnd: lit.getEnd(), stmtStart: node.getStart(sf), stmtEnd: node.getEnd(), typeOnly: node.isTypeOnly, namespaceName, named, sideEffect: false, exportStar: !ec }, lit);
    } else if (ts.isCallExpression(node) && node.arguments.length >= 1 && ts.isStringLiteralLike(node.arguments[0])) {
      const lit = node.arguments[0] as ts.StringLiteralLike;
      if (node.expression.kind === ts.SyntaxKind.ImportKeyword) push({ kind: "dynamic", spec: lit.text, litStart: lit.getStart(sf), litEnd: lit.getEnd(), stmtStart: node.getStart(sf), stmtEnd: node.getEnd(), typeOnly: false, named: [], sideEffect: false, exportStar: false }, lit);
      else if (ts.isIdentifier(node.expression) && node.expression.text === "require") push({ kind: "require", spec: lit.text, litStart: lit.getStart(sf), litEnd: lit.getEnd(), stmtStart: node.getStart(sf), stmtEnd: node.getEnd(), typeOnly: false, named: [], sideEffect: false, exportStar: false }, lit);
    } else if (ts.isImportTypeNode(node) && ts.isLiteralTypeNode(node.argument) && ts.isStringLiteral(node.argument.literal)) {
      const lit = node.argument.literal;
      const q = node.qualifier; let left: ts.EntityName | undefined = q; while (left && ts.isQualifiedName(left)) left = left.left;
      push({ kind: "importType", spec: lit.text, litStart: lit.getStart(sf), litEnd: lit.getEnd(), stmtStart: node.getStart(sf), stmtEnd: node.getEnd(), typeOnly: true, named: [], sideEffect: false, exportStar: false, members: left && ts.isIdentifier(left) ? [left.text] : undefined, membersUnknown: !left }, lit);
    }
    ts.forEachChild(node, visit);
  };
  visit(sf);
  const nsAccess = (ns: string): { names: string[]; unknown: boolean } => {
    const names = new Set<string>();
    let unknown = false;
    const walk = (n: ts.Node) => {
      if (ts.isPropertyAccessExpression(n) && ts.isIdentifier(n.expression) && n.expression.text === ns) names.add(n.name.text);
      else if (ts.isQualifiedName(n) && ts.isIdentifier(n.left) && n.left.text === ns) names.add(n.right.text);
      else if (ts.isIdentifier(n) && n.text === ns) {
        const p = n.parent;
        const okParent = (ts.isPropertyAccessExpression(p) && p.expression === n) || (ts.isQualifiedName(p) && p.left === n) || ts.isNamespaceImport(p) || (ts.isTypeQueryNode(p) && false);
        if (!okParent && !(ts.isImportClause(p) || ts.isNamespaceImport(p))) unknown = true;
      }
      ts.forEachChild(n, walk);
    };
    walk(sf);
    return { names: [...names], unknown };
  };
  for (const r of out) {
    if (r.kind === "import" && r.namespaceName) { const a = nsAccess(r.namespaceName); r.members = a.names; r.membersUnknown = a.unknown; }
  }
  const dynFind = (n: ts.Node) => {
    if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0])) {
      const lit = n.arguments[0] as ts.StringLiteralLike;
      const rec = out.find(o => o.kind === "dynamic" && o.litStart === lit.getStart(sf));
      if (rec) {
        let cur: ts.Node = n; let up = cur.parent;
        while (up && (ts.isAwaitExpression(up) || ts.isParenthesizedExpression(up))) { cur = up; up = cur.parent; }
        if (up && ts.isVariableDeclaration(up) && up.initializer === cur && ts.isObjectBindingPattern(up.name)) rec.members = up.name.elements.map(e => (e.propertyName ?? e.name as ts.Identifier) && ((e.propertyName as ts.Identifier | undefined)?.text ?? (e.name as ts.Identifier).text));
        else if (up && ts.isPropertyAccessExpression(up) && up.expression === cur) rec.members = [up.name.text];
        else if (up && ts.isVariableDeclaration(up) && up.initializer === cur && ts.isIdentifier(up.name)) { const a = nsAccess(up.name.text); rec.members = a.names; rec.membersUnknown = a.unknown; }
        else rec.membersUnknown = true;
      }
    }
    ts.forEachChild(n, dynFind);
  };
  dynFind(sf);
  return out;
}

export function sdkExportTargets(): Set<string> {
  const pkg = JSON.parse(readFileSync(join(ROOT, SDK_DIR, "package.json"), "utf8"));
  const out = new Set<string>();
  const collect = (v: unknown) => { if (typeof v === "string") out.add((SDK_DIR + "/" + v.slice(2)).normalize("NFC")); else if (Array.isArray(v)) v.forEach(collect); else if (v && typeof v === "object") Object.values(v).forEach(collect); };
  collect(pkg.exports);
  return out;
}

export function isFrameworkTarget(t: string | null): boolean { return !!t && t.startsWith(FRAMEWORK); }

export function readLines(path: string): string[] { return readFileSync(path, "utf8").split("\n").filter(Boolean); }

export { existsSync };
