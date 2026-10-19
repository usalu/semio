import ts from "typescript";
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve, join, relative } from "node:path";
import { ROOT, workspacePackages, type Pkg } from "../lay6-tools/pkgs.ts";

const [, , reportPath, mode, outPath] = process.argv;
const apply = mode === "apply";
const OS = "🧰️framework/🛍️products/💻️os/";
const rend = OS + "🔨️modules/📺️renderer/";
const report = JSON.parse(readFileSync(reportPath!, "utf8"));
const mods = new Map<string, any>(report.modules.map((m: any) => [m.source, m]));
const pkgs = workspacePackages();
const byName = new Map(pkgs.map((p) => [p.name, p]));

const v = (report.summary.violations as any[]).filter((x) => x.rule.name.startsWith("no-cross-package-relative-") && !x.from.startsWith(rend) && !x.to.startsWith(OS));
type Edge = { from: string; to: string; specifier: string; pkgName: string };
const edges = new Map<string, Edge>();
for (const x of v) {
  const dep = mods.get(x.from)?.dependencies.find((d: any) => d.resolved === x.to);
  if (!dep) continue;
  const pkgName = x.rule.name.replace("no-cross-package-relative-", "");
  const k = `${x.from}\u0000${dep.module}`;
  const prev = edges.get(k);
  if (!prev) edges.set(k, { from: x.from, to: x.to, specifier: dep.module, pkgName });
  else prev.pkgName += "," + pkgName;
}

const cfgPath = resolve(ROOT, OS, "tsconfig.json");
const cfg = ts.getParsedCommandLineOfConfigFile(cfgPath, {}, { ...ts.sys, onUnRecoverableConfigFileDiagnostic: (d) => { throw new Error(String(d.messageText)); } })!;
const program = ts.createProgram({ rootNames: cfg.fileNames, options: { ...cfg.options, noEmit: true, incremental: false, tsBuildInfoFile: undefined } });
const checker = program.getTypeChecker();

const entrySymbolCache = new Map<string, Map<string, ts.Symbol> | null>();
function exportsOf(file: string): Map<string, ts.Symbol> | null {
  if (entrySymbolCache.has(file)) return entrySymbolCache.get(file)!;
  const sf = program.getSourceFile(file);
  const sym = sf && checker.getSymbolAtLocation(sf);
  const res = sym ? new Map(checker.getExportsOfModule(sym).map((s) => [s.name, s])) : null;
  entrySymbolCache.set(file, res);
  return res;
}
const resolveAlias = (s: ts.Symbol): ts.Symbol => (s.flags & ts.SymbolFlags.Alias ? checker.getAliasedSymbol(s) : s);

type Use = { names: string[] | "*"; node: ts.Node; spec: ts.StringLiteralLike };
function usesIn(sf: ts.SourceFile, specifier: string): Use[] {
  const uses: Use[] = [];
  const visit = (n: ts.Node): void => {
    if (ts.isImportDeclaration(n) && ts.isStringLiteral(n.moduleSpecifier) && n.moduleSpecifier.text === specifier) {
      const c = n.importClause; const names: string[] = [];
      let star = false;
      if (!c) star = true;
      else {
        if (c.name) names.push("default");
        if (c.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) star = true; else for (const e of c.namedBindings.elements) names.push((e.propertyName ?? e.name).text); }
      }
      uses.push({ names: star ? "*" : names, node: n, spec: n.moduleSpecifier });
    } else if (ts.isExportDeclaration(n) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier) && n.moduleSpecifier.text === specifier) {
      const names: string[] = []; let star = false;
      if (!n.exportClause || ts.isNamespaceExport(n.exportClause)) star = true; else for (const e of n.exportClause.elements) names.push((e.propertyName ?? e.name).text);
      uses.push({ names: star ? "*" : names, node: n, spec: n.moduleSpecifier });
    } else if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0]) && n.arguments[0].text === specifier) {
      uses.push({ names: "*", node: n, spec: n.arguments[0] });
    } else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal) && n.argument.literal.text === specifier) {
      const q = n.qualifier; const first = q ? (ts.isIdentifier(q) ? q.text : ts.isQualifiedName(q) ? (function f(x: ts.EntityName): string { return ts.isIdentifier(x) ? x.text : f(x.left); })(q) : "*") : "*";
      uses.push({ names: first === "*" ? "*" : [first], node: n, spec: n.argument.literal });
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  return uses;
}

type Fix = { from: string; specifier: string; to: string; replacement: string; pkg: string; subpath: string };
const fixes: Fix[] = [], skipped: { from: string; to: string; specifier: string; pkg: string; reason: string; names?: string[] | "*" }[] = [];
const edits = new Map<string, { start: number; end: number; text: string }[]>();

for (const e of edges.values()) {
  const fromAbs = ROOT + e.from; const toAbs = ROOT + e.to;
  const sf = program.getSourceFile(fromAbs);
  if (!sf) { skipped.push({ ...e, pkg: e.pkgName, reason: "source not in program" }); continue; }
  const uses = usesIn(sf, e.specifier);
  if (uses.length === 0) { skipped.push({ ...e, pkg: e.pkgName, reason: "no import node found" }); continue; }
  const target = pkgs.find((p) => e.pkgName.split(",").includes(p.name));
  if (!target) { skipped.push({ ...e, pkg: e.pkgName, reason: "target package unknown" }); continue; }
  const group = pkgs.filter((p) => p.sourceOwner === target.sourceOwner);
  const toExports = toAbs.endsWith(".json") ? null : exportsOf(toAbs);
  const decided: { spec: ts.StringLiteralLike; text: string; pkg: string; sub: string }[] = [];
  let reason = "";
  for (const u of uses) {
    let chosen: { p: Pkg; sub: string } | null = null;
    for (const p of group) {
      for (const [sub, rel] of Object.entries(p.exports)) {
        if (!rel || sub.includes("*")) continue;
        const entryAbs = resolve(ROOT, p.owner, rel);
        if (entryAbs === toAbs) { chosen = { p, sub }; break; }
        if (!toExports) continue;
        const ee = exportsOf(entryAbs); if (!ee) continue;
        const needed = u.names === "*" ? [...toExports.keys()] : u.names;
        const ok = needed.every((n) => { const a = toExports.get(n), b = ee.get(n); return a && b && resolveAlias(a) === resolveAlias(b); });
        if (ok && !chosen) chosen = { p, sub };
      }
      if (chosen) break;
    }
    if (!chosen) { reason = u.names === "*" ? "namespace/side-effect/dynamic use without exact entry" : "no existing export entry exposes " + u.names.join(","); break; }
    const sub = chosen.sub === "." ? "" : chosen.sub.slice(1);
    decided.push({ spec: u.spec, text: chosen.p.name + sub, pkg: chosen.p.name, sub: chosen.sub });
  }
  if (reason) { const all = uses.some((u) => u.names === "*") ? "*" : [...new Set(uses.flatMap((u) => u.names as string[]))]; skipped.push({ ...e, pkg: e.pkgName, reason, names: all }); continue; }
  for (const d of decided) {
    const list = edits.get(fromAbs) ?? []; list.push({ start: d.spec.getStart(sf) + 1, end: d.spec.getEnd() - 1, text: d.text }); edits.set(fromAbs, list);
    fixes.push({ from: e.from, specifier: e.specifier, to: e.to, replacement: d.text, pkg: d.pkg, subpath: d.sub });
  }
}
if (apply) for (const [file, list] of edits) {
  let s = readFileSync(file, "utf8");
  for (const ed of list.sort((a, b) => b.start - a.start)) s = s.slice(0, ed.start) + ed.text + s.slice(ed.end);
  writeFileSync(file, s);
}
writeFileSync(outPath!, JSON.stringify({ fixes, skipped }, null, 1));
console.log("edges", edges.size, "fixable", fixes.length, "skipped", skipped.length);
const rc: Record<string, number> = {}; for (const s of skipped) { const k = s.pkg + " | " + s.reason.replace(/exposes .*/, "exposes ..."); rc[k] = (rc[k] ?? 0) + 1; }
console.log(rc);
