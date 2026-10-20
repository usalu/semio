// 🔬 Resolves every static and string-literal dynamic import of the changed files inside the mirror and checks that each named import is exported by its target. usage: bun verify-imports.ts <mirror>
import {existsSync, readFileSync, statSync} from "node:fs";
import {dirname, join, resolve} from "node:path";
import ts from "typescript";
import {C} from "./vfs.ts";

const mirror = resolve(process.argv[2]!);
const ops = JSON.parse(readFileSync(`${C}/tax-stage-tools/staged/ops.json`, "utf8")).ops as {path: string; kind: string}[];
const changed = ops.filter((o) => o.kind === "write" && /\.(ts|tsx|mts|mjs|js)$/u.test(o.path)).map((o) => join(mirror, o.path));
const cache = new Map<string, Set<string> | "any">();
const parse = (file: string): ts.SourceFile => ts.createSourceFile(file, readFileSync(file, "utf8"), ts.ScriptTarget.Latest, true);
const target = (from: string, spec: string): string | null => {
  if (spec.startsWith("node:") || spec.startsWith("bun:")) return null;
  if (spec.startsWith(".")) { const p = resolve(dirname(from), spec); return existsSync(p) && statSync(p).isFile() ? p : "MISSING:" + p; }
  try { return Bun.resolveSync(spec, dirname(from)); } catch { return "MISSING:" + spec; }
};
function exportsOf(file: string): Set<string> | "any" {
  if (cache.has(file)) return cache.get(file)!;
  cache.set(file, new Set());
  if (!/\.(ts|tsx|mts)$/u.test(file)) { cache.set(file, "any"); return "any"; }
  const out = new Set<string>();
  let any = false;
  for (const s of parse(file).statements) {
    const mods = ts.canHaveModifiers(s) ? ts.getModifiers(s) : undefined, exported = !!mods?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword);
    if (exported) {
      if (ts.isVariableStatement(s)) for (const d of s.declarationList.declarations) { const names = (b: ts.BindingName): void => { if (ts.isIdentifier(b)) out.add(b.text); else for (const e of b.elements) if (!ts.isOmittedExpression(e)) names(e.name); }; names(d.name); }
      else if ((ts.isFunctionDeclaration(s) || ts.isClassDeclaration(s) || ts.isTypeAliasDeclaration(s) || ts.isInterfaceDeclaration(s) || ts.isEnumDeclaration(s)) && s.name) out.add(s.name.text);
      else if (ts.isModuleDeclaration(s)) out.add(s.name.getText());
    }
    if (ts.isExportDeclaration(s)) {
      if (s.exportClause && ts.isNamedExports(s.exportClause)) for (const e of s.exportClause.elements) out.add(e.name.text);
      else if (s.exportClause && ts.isNamespaceExport(s.exportClause)) out.add(s.exportClause.name.text);
      else if (s.moduleSpecifier && ts.isStringLiteral(s.moduleSpecifier)) { const t = target(file, s.moduleSpecifier.text); if (!t || t.startsWith("MISSING")) any = true; else { const e = exportsOf(t); if (e === "any") any = true; else for (const n of e) out.add(n); } }
    }
    if (ts.isExportAssignment(s)) out.add("default");
  }
  const r = any ? "any" : out;
  cache.set(file, r);
  return r;
}
let problems = 0, checked = 0;
const report = (file: string, text: string): void => { problems++; console.log(`PROBLEM ${file.replace(mirror + "/", "").slice(-120)}: ${text}`); };
for (const file of changed) {
  if (!existsSync(file)) { report(file, "missing in mirror"); continue; }
  const f = parse(file), check = (spec: string, names: string[]): void => {
    checked++;
    const t = target(file, spec);
    if (!t) return;
    if (t.startsWith("MISSING")) { report(file, `unresolved ${spec}`); return; }
    if (!/\.(ts|tsx|mts)$/u.test(t) || !names.length) return;
    const e = exportsOf(t);
    if (e === "any") return;
    for (const n of names) if (n !== "default" && n !== "*" && !e.has(n)) report(file, `${spec} does not export ${n}`);
  };
  const visit = (n: ts.Node): void => {
    if (ts.isImportDeclaration(n) && ts.isStringLiteral(n.moduleSpecifier)) {
      const c = n.importClause, names: string[] = [];
      if (c && !c.isTypeOnly) { if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const e of c.namedBindings.elements) names.push((e.propertyName ?? e.name).text); }
      else if (c?.isTypeOnly && c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const e of c.namedBindings.elements) names.push((e.propertyName ?? e.name).text);
      check(n.moduleSpecifier.text, names);
    } else if (ts.isExportDeclaration(n) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) {
      const names: string[] = [];
      if (n.exportClause && ts.isNamedExports(n.exportClause)) for (const e of n.exportClause.elements) names.push((e.propertyName ?? e.name).text);
      check(n.moduleSpecifier.text, names);
    } else if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteral(n.arguments[0]!)) {
      const names: string[] = [];
      const par = n.parent?.parent;
      const decl = ts.isAwaitExpression(n.parent) && ts.isVariableDeclaration(n.parent.parent) ? n.parent.parent : ts.isParenthesizedExpression(n.parent) ? undefined : undefined;
      if (decl && ts.isObjectBindingPattern(decl.name)) for (const e of decl.name.elements) names.push((e.propertyName ?? e.name).getText());
      void par;
      check((n.arguments[0] as ts.StringLiteral).text, names);
    }
    ts.forEachChild(n, visit);
  };
  visit(f);
}
console.log(JSON.stringify({files: changed.length, importsChecked: checked, problems}));

if (process.argv.includes("--global")) {
  const REPO = "/Users/ueli/Documents/semio";
  const files = readFileSync(`${C}/tax-stage-tools/index/grep-files.txt`, "utf8").split("\n").filter((f) => /\.(ts|tsx|mts|mjs)$/u.test(f));
  let scanned = 0, bad = 0;
  const deleted = new Set(ops.filter((o) => o.kind === "delete").map((o) => o.path));
  for (const rel of files) {
    if (deleted.has(rel)) continue;
    const real = existsSync(join(mirror, rel)) ? join(mirror, rel) : join(REPO, rel);
    let text: string; try { text = readFileSync(real, "utf8"); } catch { continue; }
    if (!text.includes("📜️script.ts")) continue;
    scanned++;
    const f = ts.createSourceFile(real, text, ts.ScriptTarget.Latest, true);
    const dir = dirname(join(mirror, rel));
    const visit = (n: ts.Node): void => {
      let spec: string | null = null, names: string[] = [];
      if (ts.isImportDeclaration(n) && ts.isStringLiteral(n.moduleSpecifier)) { spec = n.moduleSpecifier.text; if (n.importClause?.namedBindings && ts.isNamedImports(n.importClause.namedBindings)) names = n.importClause.namedBindings.elements.map((e) => (e.propertyName ?? e.name).text); }
      else if (ts.isExportDeclaration(n) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) { spec = n.moduleSpecifier.text; if (n.exportClause && ts.isNamedExports(n.exportClause)) names = n.exportClause.elements.map((e) => (e.propertyName ?? e.name).text); }
      else if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteral(n.arguments[0]!)) { spec = (n.arguments[0] as ts.StringLiteral).text; const d = ts.isAwaitExpression(n.parent) && ts.isVariableDeclaration(n.parent.parent) ? n.parent.parent : undefined; if (d && ts.isObjectBindingPattern(d.name)) names = d.name.elements.map((e) => (e.propertyName ?? e.name).getText()); }
      if (spec && spec.startsWith(".") && spec.endsWith("📜️script.ts") && names.length) {
        const t = resolve(dir, spec);
        if (!existsSync(t)) { bad++; console.log(`GLOBAL missing target ${rel.slice(-100)} -> ${spec.slice(-60)}`); }
        else { const e = exportsOf(t); if (e !== "any") for (const name of names) if (!e.has(name)) { bad++; console.log(`GLOBAL ${rel.slice(-110)} imports ${name} from ${spec.slice(-70)} but the script no longer exports it`); } }
      }
      ts.forEachChild(n, visit);
    };
    visit(f);
  }
  console.log(JSON.stringify({globalScanned: scanned, globalProblems: bad}));
}
