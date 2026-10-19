import ts from "typescript";
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { ROOT, workspacePackages, type Pkg } from "./pkgs.ts";

const [, , reportPath, mode, outPath] = process.argv;
const apply = mode === "apply";
const OS = "🧰️framework/🛍️products/💻️os/";
const REND = OS + "🔨️modules/📺️renderer/";
const HOST_ROOT = new RegExp("^" + REND.replace(/[.*+?^${}()|[\]\\]/g, "\\$&") + "🧑‍🎨engine/(🎯️targets/⚛️react|🧱️elements)/");
const HOST_PUBLIC_ENTRY_ROOT = new RegExp("^" + (OS + "📦️packages/🟦️typescript/🔨️modules/📺️renderer/").replace(/[.*+?^${}()|[\]\\]/g, "\\$&") + "🧑‍🎨engine/(🎯️targets/⚛️react|🧱️elements)/");
const HOST_ALLOWED = [HOST_ROOT, HOST_PUBLIC_ENTRY_ROOT, /^🧰️framework\/🔨️modules\/🖱️ui\//, /^🧰️framework\/📦️packages\//];
const STUBBED = new Set(["@semio-tech/framework-os-node-graph-rs", "@semio-tech/framework-surface-rs", "@semio-tech/framework-editor-rs", "@semio-tech/flow-core"]);
const report = JSON.parse(readFileSync(reportPath!, "utf8"));
const mods = new Map<string, any>(report.modules.map((m: any) => [m.source, m]));
const pkgs = workspacePackages();

type Edge = { from: string; to: string; specs: Set<string>; rulePkgs: Set<string>; hostRule: boolean };
const edges = new Map<string, Edge>();
for (const x of report.summary.violations as any[]) {
  if (!x.from.startsWith(REND)) continue;
  const isN = x.rule.name.startsWith("no-cross-package-relative-"), isH = x.rule.name === "renderer-hosts-only-ui";
  if (!isN && !isH) continue;
  const k = x.from + "\u0000" + x.to;
  const e = edges.get(k) ?? { from: x.from, to: x.to, specs: new Set(), rulePkgs: new Set(), hostRule: false };
  for (const d of mods.get(x.from)?.dependencies ?? []) if (d.resolved === x.to) e.specs.add(d.module);
  if (isN) e.rulePkgs.add(x.rule.name.replace("no-cross-package-relative-", "")); else e.hostRule = true;
  edges.set(k, e);
}

type Entry = { pkg: Pkg; sub: string; abs: string; rel: string };
const entries: Entry[] = [];
for (const p of pkgs) for (const [sub, rel] of Object.entries(p.exports)) {
  if (!rel || sub.includes("*") || rel.includes("*")) continue;
  const abs = resolve(ROOT, p.owner, rel);
  if (existsSync(abs)) entries.push({ pkg: p, sub, abs, rel: abs.slice(ROOT.length) });
}

const cfgPath = resolve(ROOT, OS, "tsconfig.json");
const cfg = ts.getParsedCommandLineOfConfigFile(cfgPath, {}, { ...ts.sys, onUnRecoverableConfigFileDiagnostic: (d) => { throw new Error(String(d.messageText)); } })!;
const extra = [...new Set(entries.map((e) => e.abs).filter((f) => /\.tsx?$/.test(f)))];
const program = ts.createProgram({ rootNames: [...new Set([...cfg.fileNames, ...extra])], options: { ...cfg.options, noEmit: true, incremental: false, tsBuildInfoFile: undefined } });
const checker = program.getTypeChecker();
const exportCache = new Map<string, Map<string, ts.Symbol> | null>();
function exportsOf(file: string): Map<string, ts.Symbol> | null {
  if (exportCache.has(file)) return exportCache.get(file)!;
  const sf = program.getSourceFile(file);
  const sym = sf && checker.getSymbolAtLocation(sf);
  const res = sym ? new Map(checker.getExportsOfModule(sym).map((s) => [s.name, s])) : null;
  exportCache.set(file, res);
  return res;
}
const real = (s: ts.Symbol): ts.Symbol => (s.flags & ts.SymbolFlags.Alias ? checker.getAliasedSymbol(s) : s);

type Use = { names: string[] | "*"; spec: ts.StringLiteralLike };
function usesIn(sf: ts.SourceFile, specifier: string): Use[] {
  const uses: Use[] = [];
  const visit = (n: ts.Node): void => {
    if (ts.isImportDeclaration(n) && ts.isStringLiteral(n.moduleSpecifier) && n.moduleSpecifier.text === specifier) {
      const c = n.importClause; const names: string[] = []; let star = false;
      if (!c) star = true; else { if (c.name) names.push("default"); if (c.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) star = true; else for (const e of c.namedBindings.elements) names.push((e.propertyName ?? e.name).text); } }
      uses.push({ names: star ? "*" : names, spec: n.moduleSpecifier });
    } else if (ts.isExportDeclaration(n) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier) && n.moduleSpecifier.text === specifier) {
      const names: string[] = []; let star = false;
      if (!n.exportClause || ts.isNamespaceExport(n.exportClause)) star = true; else for (const e of n.exportClause.elements) names.push((e.propertyName ?? e.name).text);
      uses.push({ names: star ? "*" : names, spec: n.moduleSpecifier });
    } else if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0]) && n.arguments[0].text === specifier) {
      let names: string[] | "*" = "*";
      const aw = n.parent, decl = aw && ts.isAwaitExpression(aw) ? aw.parent : undefined;
      if (n.arguments.length === 1 && decl && ts.isVariableDeclaration(decl) && ts.isObjectBindingPattern(decl.name) && !decl.name.elements.some((e) => e.dotDotDotToken)) {
        const picked = decl.name.elements.map((e) => (e.propertyName && ts.isIdentifier(e.propertyName) ? e.propertyName.text : ts.isIdentifier(e.name) ? e.name.text : "*"));
        if (!picked.includes("*") && picked.length) names = picked;
      }
      uses.push({ names, spec: n.arguments[0] });
    } else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal) && n.argument.literal.text === specifier) {
      const q = n.qualifier; const first = q ? (function f(x: ts.EntityName): string { return ts.isIdentifier(x) ? x.text : f(x.left); })(q) : "*";
      uses.push({ names: first === "*" ? "*" : [first], spec: n.argument.literal });
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  return uses;
}

type Fix = { from: string; to: string; specifier: string; replacement: string; pkg: string; sub: string; viaOtherPackage: boolean };
type Skip = { from: string; to: string; specifier: string; rulePkgs: string[]; hostRule: boolean; reason: string; names: string[] | "*"; ownerPkg: string | null };
const fixes: Fix[] = [], skipped: Skip[] = [];
const edits = new Map<string, { start: number; end: number; text: string }[]>();
const ownerOf = (to: string): Pkg | null => {
  let best: Pkg | null = null;
  for (const p of pkgs) { const so = p.sourceOwner === "" ? "" : p.sourceOwner + "/"; if (to.startsWith(so) && (!best || so.length > best.sourceOwner.length + 1)) { if (!best || p.sourceOwner.length > best.sourceOwner.length) best = p; } }
  return best;
};

for (const e of edges.values()) {
  const fromAbs = ROOT + e.from, toAbs = ROOT + e.to;
  const sf = program.getSourceFile(fromAbs);
  const isHost = HOST_ROOT.test(e.from);
  const owner = ownerOf(e.to);
  const fromOwner = ownerOf(e.from);
  const skipAll = (reason: string, names: string[] | "*" = "*") => skipped.push({ from: e.from, to: e.to, specifier: [...e.specs][0] ?? "", rulePkgs: [...e.rulePkgs], hostRule: e.hostRule, reason, names, ownerPkg: owner?.name ?? null });
  if (!sf) { skipAll("source not in program"); continue; }
  if (e.from.includes("🎯️targets/🧊️wgpu/")) { skipAll("wgpu browser bundle: imports are schema-owned (profile.workspaceImports), a bare specifier needs the profile binding first"); continue; }
  const toExports = toAbs.endsWith(".json") ? null : exportsOf(toAbs);
  const ruleGroupOwners = new Set([...e.rulePkgs].map((n) => pkgs.find((p) => p.name === n)?.sourceOwner).filter((x) => x !== undefined));
  for (const specifier of e.specs) {
    const uses = usesIn(sf, specifier);
    if (!uses.length) { skipAll("no import node found"); continue; }
    const decided: { spec: ts.StringLiteralLike; text: string; pkg: string; sub: string; other: boolean }[] = [];
    let reason = "";
    for (const u of uses) {
      let chosen: { en: Entry; score: number } | null = null;
      for (const en of entries) {
        if (STUBBED.has(en.pkg.name)) continue;
        if (!ruleGroupOwners.has(en.pkg.sourceOwner) && (en.pkg.sourceOwner + "/").startsWith(REND)) continue;
        if (fromOwner && fromOwner.sourceOwner === en.pkg.sourceOwner) continue;
        if (isHost && !HOST_ALLOWED.some((r) => r.test(en.rel))) continue;
        let ok = en.abs === toAbs;
        if (!ok && toAbs.endsWith(".json") && /\.tsx?$/.test(en.abs) && u.names !== "*" && u.names.length === 1 && u.names[0] === "default") {
          const m = /from\s+"([^"]+\.json)"/.exec(readFileSync(en.abs, "utf8"));
          ok = !!m && resolve(dirname(en.abs), m[1]!) === toAbs;
        }
        if (!ok && toExports) {
          const ee = exportsOf(en.abs);
          if (ee) { const needed = u.names === "*" ? [...toExports.keys()] : u.names; ok = needed.length > 0 && needed.every((n) => { const a = toExports.get(n), b = ee.get(n); return a && b && real(a) === real(b); }); }
        }
        if (!ok) continue;
        const same = ruleGroupOwners.has(en.pkg.sourceOwner) ? 0 : 1;
        const score = (en.abs === toAbs ? 0 : 10) + same * 5 + (en.sub === "." ? 0 : 1);
        if (!chosen || score < chosen.score) chosen = { en, score };
      }
      if (!chosen) { reason = u.names === "*" ? "namespace/dynamic/side-effect/json use without exact entry" : "no existing export entry exposes " + u.names.join(","); break; }
      const sub = chosen.en.sub === "." ? "" : chosen.en.sub.slice(1);
      decided.push({ spec: u.spec, text: chosen.en.pkg.name + sub, pkg: chosen.en.pkg.name, sub: chosen.en.sub, other: !ruleGroupOwners.has(chosen.en.pkg.sourceOwner) });
    }
    if (reason) { const all = uses.some((u) => u.names === "*") ? "*" : [...new Set(uses.flatMap((u) => u.names as string[]))]; skipAll(reason, all); continue; }
    for (const d of decided) {
      const list = edits.get(fromAbs) ?? []; list.push({ start: d.spec.getStart(sf) + 1, end: d.spec.getEnd() - 1, text: d.text }); edits.set(fromAbs, list);
      const decl = d.spec.parent;
      if (toAbs.endsWith(".json") && (ts.isImportDeclaration(decl) || ts.isExportDeclaration(decl)) && decl.attributes) { list.push({ start: d.spec.getEnd(), end: decl.attributes.getEnd(), text: "" }); }
      fixes.push({ from: e.from, to: e.to, specifier, replacement: d.text, pkg: d.pkg, sub: d.sub, viaOtherPackage: d.other });
    }
  }
}
if (apply) for (const [file, list] of edits) {
  let s = readFileSync(file, "utf8");
  for (const ed of list.sort((a, b) => b.start - a.start)) s = s.slice(0, ed.start) + ed.text + s.slice(ed.end);
  writeFileSync(file, s);
}
writeFileSync(outPath!, JSON.stringify({ fixes, skipped }, null, 1));
console.log("edges", edges.size, "fixes", fixes.length, "files", edits.size, "skipped", skipped.length);
const rc: Record<string, number> = {};
for (const s of skipped) { const k = (s.ownerPkg ?? "?") + " | " + (s.hostRule ? "H" : "") + (s.rulePkgs.length ? "N" : "") + " | " + s.reason.replace(/exposes .*/, "exposes ..."); rc[k] = (rc[k] ?? 0) + 1; }
console.log(Object.entries(rc).sort((a, b) => b[1] - a[1]).slice(0, 40));
