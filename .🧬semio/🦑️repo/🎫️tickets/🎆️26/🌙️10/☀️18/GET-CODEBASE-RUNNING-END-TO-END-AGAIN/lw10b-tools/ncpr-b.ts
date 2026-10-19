import ts from "typescript";
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { resolve, relative, dirname } from "node:path";
import { ROOT, workspacePackages, type Pkg } from "../lay6-tools/pkgs.ts";

const [, , reportPath, closurePath, mode, outPath, only] = process.argv;
const apply = mode === "apply" || mode === "apply-closure";
const closureMode = mode === "plan-closure" || mode === "apply-closure";
const PR = "🧰️framework/🛍️products/";
const SCOPES = ["🦑️repo/", "❓️quiz/", "🎤️presentation/", "🐾️pets/", "📓️print/", "🖥️server/"].map((s) => PR + s);
const PROCESS = "🧰️framework/🔨️modules/🏃️process/";
const FROZEN = ["⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/", "🏗️build/📥️inputs/🔏️revision/"];
const closure = new Set(readFileSync(closurePath!, "utf8").split("\n").filter(Boolean));
const report = JSON.parse(readFileSync(reportPath!, "utf8"));
const mods = new Map<string, any>(report.modules.map((m: any) => [m.source, m]));
const pkgs = workspacePackages();
const only_ = only ? new Set(readFileSync(only, "utf8").split("\n").filter(Boolean)) : null;

type Edge = { from: string; to: string; specifier: string; pkgNames: string[] };
const edges = new Map<string, Edge>();
for (const x of report.summary.violations as any[]) {
  if (!x.rule.name.startsWith("no-cross-package-relative-")) continue;
  if (!SCOPES.some((s) => x.from.startsWith(s)) || closure.has(x.from) !== closureMode) continue;
  if (x.to.startsWith(PROCESS) && x.from.startsWith(SCOPES[0]!)) continue;
  if (only_ && !only_.has(x.from)) continue;
  if (FROZEN.some((f) => x.from.includes(f))) continue;
  const dep = mods.get(x.from)?.dependencies.find((d: any) => d.resolved === x.to);
  if (!dep) continue;
  const k = `${x.from}\u0000${dep.module}`;
  const name = x.rule.name.slice("no-cross-package-relative-".length);
  const prev = edges.get(k);
  if (!prev) edges.set(k, { from: x.from, to: x.to, specifier: dep.module, pkgNames: [name] });
  else if (!prev.pkgNames.includes(name)) prev.pkgNames.push(name);
}
console.log("edges", edges.size);

const cfgPath = resolve(ROOT, "🧰️framework/🛍️products/🦑️repo/tsconfig.json");
const cfg = ts.getParsedCommandLineOfConfigFile(cfgPath, {}, { ...ts.sys, onUnRecoverableConfigFileDiagnostic: (d) => { throw new Error(String(d.messageText)); } })!;
const roots = new Set(cfg.fileNames);
for (const e of edges.values()) roots.add(ROOT + e.from);
for (const p of pkgs) for (const rel of Object.values(p.exports)) if (rel && /\.tsx?$/u.test(rel)) { const f = resolve(ROOT, p.owner, rel); if (existsSync(f)) roots.add(f); }
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
      else { if (c.name) names.push("default"); if (c.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) star = true; else for (const e of c.namedBindings.elements) names.push((e.propertyName ?? e.name).text); } }
      uses.push({ names: star ? "*" : names, spec: n.moduleSpecifier });
    } else if (ts.isExportDeclaration(n) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier) && n.moduleSpecifier.text === specifier) {
      const names: string[] = []; let star = false;
      if (!n.exportClause || ts.isNamespaceExport(n.exportClause)) star = true; else for (const e of n.exportClause.elements) names.push((e.propertyName ?? e.name).text);
      uses.push({ names: star ? "*" : names, spec: n.moduleSpecifier });
    } else if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0]) && n.arguments[0].text === specifier) {
      uses.push({ names: "*", spec: n.arguments[0] });
    } else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal) && n.argument.literal.text === specifier) {
      const q = n.qualifier; const first = q ? (function f(x: ts.EntityName): string { return ts.isIdentifier(x) ? x.text : f(x.left); })(q) : "*";
      uses.push({ names: first === "*" ? "*" : [first], spec: n.argument.literal });
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  return uses;
}

const nearest = (file: string): string => pkgs.filter((p) => file.startsWith(p.sourceOwner + "/")).sort((a, b) => b.sourceOwner.length - a.sourceOwner.length)[0]?.name ?? "";
type Fix = { from: string; specifier: string; to: string; replacement: string };
const fixes: Fix[] = [], skipped: { from: string; to: string; specifier: string; pkgs: string[]; reason: string; names?: string[] | "*" }[] = [];
const edits = new Map<string, { start: number; end: number; text: string; old: string }[]>();

for (const e of edges.values()) {
  const fromAbs = ROOT + e.from; const toAbs = ROOT + e.to;
  const sf = program.getSourceFile(fromAbs);
  if (!sf) { skipped.push({ ...e, pkgs: e.pkgNames, reason: "source not in program" }); continue; }
  const uses = usesIn(sf, e.specifier);
  if (uses.length === 0) { skipped.push({ ...e, pkgs: e.pkgNames, reason: "no import node found" }); continue; }
  const toExports = toAbs.endsWith(".json") ? null : exportsOf(toAbs);
  const decided: { spec: ts.StringLiteralLike; text: string }[] = [];
  let reason = "";
  for (const u of uses) {
    let chosen: { p: Pkg; sub: string } | null = null;
    const cands = pkgs.filter((p) => e.pkgNames.includes(p.name) || toAbs.startsWith(ROOT + p.sourceOwner + "/") || p.sourceOwner === "").sort((a, b) => b.sourceOwner.length - a.sourceOwner.length);
    for (const p of cands) {
      for (const [sub, rel] of Object.entries(p.exports)) {
        if (!rel || sub.includes("*") || sub === "./package.json") continue;
        const entryAbs = resolve(ROOT, p.owner, rel);
        if (entryAbs === toAbs) { chosen = { p, sub }; break; }
        if (!toExports) continue;
        const ee = exportsOf(entryAbs); if (!ee) continue;
        const needed = u.names === "*" ? [...toExports.keys()] : u.names;
        if (needed.length && needed.every((n) => { const a = toExports.get(n), b = ee.get(n); return a && b && resolveAlias(a) === resolveAlias(b); })) { chosen = { p, sub }; break; }
      }
      if (chosen) break;
    }
    if (!chosen) { reason = u.names === "*" ? "namespace/side-effect/dynamic use without exact entry" : "no existing export entry exposes " + u.names.join(","); break; }
    const entryRel = relative(dirname(fromAbs), resolve(ROOT, chosen.p.owner, chosen.p.exports[chosen.sub]!)).split("\\").join("/");
    decided.push({ spec: u.spec, text: closureMode ? (entryRel.startsWith(".") ? entryRel : "./" + entryRel) : chosen.p.name + (chosen.sub === "." ? "" : chosen.sub.slice(1)) });
  }
  if (reason) { const all = uses.some((u) => u.names === "*") ? "*" : [...new Set(uses.flatMap((u) => u.names as string[]))]; skipped.push({ ...e, pkgs: e.pkgNames, reason, names: all }); continue; }
  for (const d of decided) {
    const list = edits.get(fromAbs) ?? []; list.push({ start: d.spec.getStart(sf) + 1, end: d.spec.getEnd() - 1, text: d.text, old: d.spec.text }); edits.set(fromAbs, list);
    fixes.push({ from: e.from, specifier: e.specifier, to: e.to, replacement: d.text });
  }
}
if (apply) for (const [file, list] of edits) {
  let s = readFileSync(file, "utf8");
  for (const ed of list.sort((a, b) => b.start - a.start)) { if (s.slice(ed.start, ed.end) !== ed.old) { console.log("STALE", file, ed.old); continue; } s = s.slice(0, ed.start) + ed.text + s.slice(ed.end); }
  writeFileSync(file, s);
}
writeFileSync(outPath!, JSON.stringify({ fixes, skipped }, null, 1));
console.log("edges", edges.size, "fixable", fixes.length, "skipped", skipped.length, "files", edits.size);
const rc: Record<string, number> = {}; for (const s of skipped) { const k = s.pkgs.join(",") + " | " + s.reason.replace(/exposes .*/, "exposes ..."); rc[k] = (rc[k] ?? 0) + 1; }
console.log(rc);
