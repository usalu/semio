import ts from "typescript";
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve, relative, sep } from "node:path";
import { ROOT, workspacePackages, type Pkg } from "./pkgs.ts";

const [, , reportPath, closurePath, mode, outPath] = process.argv;
const apply = mode === "apply";
const SRC = "🧰️framework/🛍️products/🦑️repo/";
const TGT = "🧰️framework/🔨️modules/🏃️process/";
const FROZEN = ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/"];
const closure = new Set(readFileSync(closurePath!, "utf8").split("\n").filter(Boolean));
const report = JSON.parse(readFileSync(reportPath!, "utf8"));
const mods = new Map<string, any>(report.modules.map((m: any) => [m.source, m]));
const pkgs = workspacePackages();

const v = (report.summary.violations as any[]).filter((x) => x.rule.name.startsWith("no-cross-package-relative-") && x.from.startsWith(SRC) && x.to.startsWith(TGT) && !FROZEN.some((f) => x.from.startsWith(f)));
type Edge = { from: string; to: string; specifier: string; pkgName: string };
const edges = new Map<string, Edge>();
for (const x of v) {
  const dep = mods.get(x.from)?.dependencies.find((d: any) => d.resolved === x.to);
  if (!dep) continue;
  const k = `${x.from}\u0000${dep.module}`;
  if (!edges.has(k)) edges.set(k, { from: x.from, to: x.to, specifier: dep.module, pkgName: x.rule.name.replace("no-cross-package-relative-", "") });
}

const cfgPath = resolve(ROOT, SRC, "tsconfig.json");
const cfg = ts.getParsedCommandLineOfConfigFile(cfgPath, {}, { ...ts.sys, onUnRecoverableConfigFileDiagnostic: (d) => { throw new Error(String(d.messageText)); } })!;
const roots = new Set(cfg.fileNames);
for (const e of edges.values()) roots.add(ROOT + e.from);
const program = ts.createProgram({ rootNames: [...roots], options: { ...cfg.options, noEmit: true, incremental: false, tsBuildInfoFile: undefined } });
const checker = program.getTypeChecker();

const cache = new Map<string, Map<string, ts.Symbol> | null>();
function exportsOf(file: string): Map<string, ts.Symbol> | null {
  if (cache.has(file)) return cache.get(file)!;
  const sf = program.getSourceFile(file);
  const sym = sf && checker.getSymbolAtLocation(sf);
  const res = sym ? new Map(checker.getExportsOfModule(sym).map((s) => [s.name, s])) : null;
  cache.set(file, res);
  return res;
}
const resolveAlias = (s: ts.Symbol): ts.Symbol => (s.flags & ts.SymbolFlags.Alias ? checker.getAliasedSymbol(s) : s);

type Use = { names: string[] | "*"; spec: ts.StringLiteralLike };
function usesIn(sf: ts.SourceFile, specifier: string): Use[] {
  const uses: Use[] = [];
  const visit = (n: ts.Node): void => {
    if (ts.isImportDeclaration(n) && ts.isStringLiteral(n.moduleSpecifier) && n.moduleSpecifier.text === specifier) {
      const c = n.importClause; const names: string[] = []; let star = false;
      if (!c) star = true;
      else {
        if (c.name) names.push("default");
        if (c.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) { const m = memberUses(sf, c.namedBindings.name); if (m === "*") star = true; else names.push(...m); } else for (const e of c.namedBindings.elements) names.push((e.propertyName ?? e.name).text); }
      }
      uses.push({ names: star ? "*" : names, spec: n.moduleSpecifier });
    } else if (ts.isExportDeclaration(n) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier) && n.moduleSpecifier.text === specifier) {
      const names: string[] = []; let star = false;
      if (!n.exportClause || ts.isNamespaceExport(n.exportClause)) star = true; else for (const e of n.exportClause.elements) names.push((e.propertyName ?? e.name).text);
      uses.push({ names: star ? "*" : names, spec: n.moduleSpecifier });
    } else if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0]) && n.arguments[0].text === specifier) {
      uses.push({ names: dynamicNames(n), spec: n.arguments[0] });
    } else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal) && n.argument.literal.text === specifier) {
      const q = n.qualifier; const first = q ? (ts.isIdentifier(q) ? q.text : ts.isQualifiedName(q) ? (function f(x: ts.EntityName): string { return ts.isIdentifier(x) ? x.text : f(x.left); })(q) : "*") : "*";
      uses.push({ names: first === "*" ? "*" : [first], spec: n.argument.literal });
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  return uses;
}
function memberUses(sf: ts.SourceFile, id: ts.Identifier): string[] | "*" {
  const sym = checker.getSymbolAtLocation(id);
  if (!sym) return "*";
  const names = new Set<string>(); let bad = false;
  const visit = (n: ts.Node): void => {
    if (bad) return;
    if (ts.isIdentifier(n) && n !== id && n.text === id.text && checker.getSymbolAtLocation(n) === sym) {
      const p = n.parent;
      if (ts.isPropertyAccessExpression(p) && p.expression === n) names.add(p.name.text);
      else if (ts.isQualifiedName(p) && p.left === n) names.add(p.right.text);
      else if (ts.isElementAccessExpression(p) && p.expression === n && ts.isStringLiteralLike(p.argumentExpression)) names.add(p.argumentExpression.text);
      else if (ts.isShorthandPropertyAssignment(p) || ts.isExportSpecifier(p)) bad = true;
      else bad = true;
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  return bad ? "*" : [...names];
}
function dynamicNames(call: ts.CallExpression): string[] | "*" {
  let p: ts.Node = call.parent;
  if (ts.isAwaitExpression(p)) {
    const d = p.parent;
    if (ts.isVariableDeclaration(d) && ts.isIdentifier(d.name)) return memberUses(d.getSourceFile(), d.name);
    if (ts.isVariableDeclaration(d) && ts.isObjectBindingPattern(d.name)) {
      const names: string[] = [];
      for (const e of d.name.elements) { if (e.dotDotDotToken || !ts.isIdentifier(e.propertyName ?? e.name)) return "*"; names.push(((e.propertyName ?? e.name) as ts.Identifier).text); }
      return names;
    }
    return "*";
  }
  if (ts.isPropertyAccessExpression(p) && p.name.text === "then") {
    const c = p.parent;
    if (ts.isCallExpression(c) && c.arguments[0] && (ts.isArrowFunction(c.arguments[0]) || ts.isFunctionExpression(c.arguments[0]))) {
      const fn = c.arguments[0]; const param = fn.parameters[0];
      if (param && ts.isObjectBindingPattern(param.name)) {
        const names: string[] = [];
        for (const e of param.name.elements) { if (e.dotDotDotToken || !ts.isIdentifier(e.propertyName ?? e.name)) return "*"; names.push(((e.propertyName ?? e.name) as ts.Identifier).text); }
        return names;
      }
    }
  }
  return "*";
}

type Fix = { from: string; specifier: string; to: string; replacement: string; subpath: string; mode: string };
const fixes: Fix[] = [], skipped: any[] = [];
const edits = new Map<string, { start: number; end: number; text: string }[]>();
const procPkgs = pkgs.filter((p) => p.sourceOwner === "🧰️framework/🔨️modules/🏃️process");

for (const e of edges.values()) {
  const fromAbs = ROOT + e.from, toAbs = ROOT + e.to;
  const sf = program.getSourceFile(fromAbs);
  if (!sf) { skipped.push({ ...e, reason: "source not in program" }); continue; }
  const uses = usesIn(sf, e.specifier);
  if (!uses.length) { skipped.push({ ...e, reason: "no import node found" }); continue; }
  const toExports = toAbs.endsWith(".json") ? null : exportsOf(toAbs);
  const decided: { spec: ts.StringLiteralLike; text: string; sub: string; mode: string }[] = [];
  let reason = "";
  for (const u of uses) {
    let chosen: { p: Pkg; sub: string; entryAbs: string } | null = null;
    for (const p of procPkgs) {
      for (const [sub, rel] of Object.entries(p.exports)) {
        if (!rel || sub.includes("*")) continue;
        const entryAbs = resolve(ROOT, p.owner, rel);
        if (entryAbs === toAbs) { chosen = { p, sub, entryAbs }; break; }
        if (!toExports) continue;
        const ee = exportsOf(entryAbs); if (!ee) continue;
        const needed = u.names === "*" ? [...toExports.keys()] : u.names;
        const ok = needed.every((n) => { const a = toExports.get(n), b = ee.get(n); return a && b && resolveAlias(a) === resolveAlias(b); });
        if (ok && !chosen) chosen = { p, sub, entryAbs };
      }
      if (chosen) break;
    }
    if (!chosen) { reason = u.names === "*" ? "namespace/side-effect/dynamic use without exact entry" : "no existing export entry exposes " + u.names.join(","); break; }
    if (closure.has(e.from)) {
      let rel = relative(dirname(fromAbs), chosen.entryAbs).split(sep).join("/");
      if (!rel.startsWith(".")) rel = "./" + rel;
      decided.push({ spec: u.spec, text: rel, sub: chosen.sub, mode: "relative" });
    } else {
      const sub = chosen.sub === "." ? "" : chosen.sub.slice(1);
      decided.push({ spec: u.spec, text: chosen.p.name + sub, sub: chosen.sub, mode: "bare" });
    }
  }
  if (reason) { const all = uses.some((u) => u.names === "*") ? "*" : [...new Set(uses.flatMap((u) => u.names as string[]))]; skipped.push({ ...e, reason, names: all }); continue; }
  for (const d of decided) {
    const list = edits.get(fromAbs) ?? []; list.push({ start: d.spec.getStart(sf) + 1, end: d.spec.getEnd() - 1, text: d.text }); edits.set(fromAbs, list);
    fixes.push({ from: e.from, specifier: e.specifier, to: e.to, replacement: d.text, subpath: d.sub, mode: d.mode });
  }
}
if (apply) for (const [file, list] of edits) {
  let s = readFileSync(file, "utf8");
  for (const ed of list.sort((a, b) => b.start - a.start)) s = s.slice(0, ed.start) + ed.text + s.slice(ed.end);
  writeFileSync(file, s);
}
writeFileSync(outPath!, JSON.stringify({ fixes, skipped }, null, 1));
console.log("edges", edges.size, "fixable edges", new Set(fixes.map((f) => f.from + f.specifier)).size, "skipped", skipped.length, "files", edits.size, "relative", fixes.filter((f) => f.mode === "relative").length, "bare", fixes.filter((f) => f.mode === "bare").length);
const rc: Record<string, number> = {}; for (const s of skipped) { const k = s.reason.replace(/exposes .*/, "exposes ..."); rc[k] = (rc[k] ?? 0) + 1; }
console.log(rc);
