import ts from "typescript";
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { resolve, dirname } from "node:path";

const [, , reportPath, mode, outPath] = process.argv;
const apply = mode === "apply";
const ROOT = "/Users/ueli/Documents/semio/";
const roots = ["✏️s/🧑‍💻dev/", "✏️s/🧪️tests/", "♻️mit-bestand/"];
type Pkg = { name: string; owner: string; sourceOwner: string; exports: Record<string, string | null> };
function loadPkgs(): Pkg[] {
  const seen = new Map<string, Pkg>();
  for (const base of ["", "✏️s/", "♻️mit-bestand/"]) {
    const rootJson = JSON.parse(readFileSync(ROOT + base + "package.json", "utf8"));
    for (const w of (rootJson.workspaces ?? []) as string[]) {
      const owner = resolve(ROOT + base, w).slice(ROOT.length);
      const f = ROOT + owner + "/package.json";
      if (!existsSync(f) || seen.has(owner)) continue;
      const j = JSON.parse(readFileSync(f, "utf8"));
      const ex: Record<string, string | null> = {};
      for (const [k, v] of Object.entries(j.exports ?? {})) ex[k] = typeof v === "string" ? v : typeof v === "object" && v !== null ? ((Object.values(v as any).find((x) => typeof x === "string") as string | undefined) ?? null) : null;
      seen.set(owner, { name: j.name, owner, sourceOwner: owner.includes("/📦️packages/") ? owner.split("/📦️packages/")[0]! : owner, exports: ex });
    }
  }
  return [...seen.values()];
}
const pkgs = loadPkgs();
const report = JSON.parse(readFileSync(reportPath!, "utf8"));
const mods = new Map<string, any>(report.modules.map((m: any) => [m.source, m]));
type Edge = { from: string; to: string; specifier: string; rules: string[] };
const edges = new Map<string, Edge>();
for (const x of report.summary.violations as any[]) {
  if (!roots.some((p) => x.from.startsWith(p))) continue;
  if (!/^no-cross-(package-relative|technology)/.test(x.rule.name)) continue;
  const dep = mods.get(x.from)?.dependencies.find((d: any) => d.resolved === x.to);
  if (!dep) continue;
  const k = `${x.from}\0${dep.module}\0${x.to}`;
  const e = edges.get(k) ?? { from: x.from, to: x.to, specifier: dep.module, rules: [] };
  e.rules.push(x.rule.name); edges.set(k, e);
}

const targetOwner = (to: string): string | null => {
  let best: string | null = null;
  for (const p of pkgs) if ((to.startsWith(p.sourceOwner + "/") || to.startsWith(p.owner + "/")) && (!best || p.sourceOwner.length > best.length)) best = p.sourceOwner;
  return best;
};

const cfg = ts.getParsedCommandLineOfConfigFile(ROOT + "tsconfig.json", {}, { ...ts.sys, onUnRecoverableConfigFileDiagnostic: (d) => { throw new Error(String(d.messageText)); } })!;
const rootNames = new Set<string>();
for (const e of edges.values()) {
  rootNames.add(ROOT + e.from);
  if (/\.(tsx?|mts|cts)$/.test(e.to)) rootNames.add(ROOT + e.to);
  const so = targetOwner(e.to);
  if (so) for (const p of pkgs.filter((q) => q.sourceOwner === so)) for (const rel of Object.values(p.exports)) if (rel && !rel.includes("*") && /\.(tsx?|mts|cts)$/.test(rel)) rootNames.add(resolve(ROOT, p.owner, rel));
}
const program = ts.createProgram({ rootNames: [...rootNames].filter((f) => existsSync(f)), options: { ...cfg.options, noEmit: true, incremental: false, tsBuildInfoFile: undefined } });
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
const alias = (s: ts.Symbol): ts.Symbol => (s.flags & ts.SymbolFlags.Alias ? checker.getAliasedSymbol(s) : s);

type Use = { names: string[] | "*"; spec: ts.StringLiteralLike };
function usesIn(sf: ts.SourceFile, specifier: string): Use[] {
  const uses: Use[] = [];
  const visit = (n: ts.Node): void => {
    if (ts.isImportDeclaration(n) && ts.isStringLiteral(n.moduleSpecifier) && n.moduleSpecifier.text === specifier) {
      const c = n.importClause; const names: string[] = []; let star = false;
      if (!c) star = true;
      else {
        if (c.name) names.push("default");
        if (c.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) star = true; else for (const e of c.namedBindings.elements) names.push((e.propertyName ?? e.name).text); }
      }
      uses.push({ names: star ? "*" : names, spec: n.moduleSpecifier });
    } else if (ts.isExportDeclaration(n) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier) && n.moduleSpecifier.text === specifier) {
      const names: string[] = []; let star = false;
      if (!n.exportClause || ts.isNamespaceExport(n.exportClause)) star = true; else for (const e of n.exportClause.elements) names.push((e.propertyName ?? e.name).text);
      uses.push({ names: star ? "*" : names, spec: n.moduleSpecifier });
    } else if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0]) && n.arguments[0].text === specifier) {
      uses.push({ names: "*", spec: n.arguments[0] });
    } else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal) && n.argument.literal.text === specifier) {
      const q = n.qualifier; const first = q ? (ts.isIdentifier(q) ? q.text : "*") : "*";
      uses.push({ names: first === "*" ? "*" : [first], spec: n.argument.literal });
    } else if (ts.isCallExpression(n) && ts.isIdentifier(n.expression) && n.expression.text === "require" && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0]) && n.arguments[0].text === specifier) {
      uses.push({ names: "*", spec: n.arguments[0] });
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  return uses;
}

type Fix = { from: string; specifier: string; to: string; replacement: string };
const fixes: Fix[] = [];
const skipped: { from: string; to: string; specifier: string; reason: string; names?: string[] | "*"; rules: string[]; owner: string | null }[] = [];
const edits = new Map<string, { start: number; end: number; text: string }[]>();
for (const e of edges.values()) {
  const fromAbs = ROOT + e.from, toAbs = ROOT + e.to;
  const so = targetOwner(e.to);
  const sf = program.getSourceFile(fromAbs);
  const base = { from: e.from, to: e.to, specifier: e.specifier, rules: e.rules, owner: so };
  if (!sf) { skipped.push({ ...base, reason: "source not in program" }); continue; }
  const uses = usesIn(sf, e.specifier);
  if (!uses.length) { skipped.push({ ...base, reason: "no import node found" }); continue; }
  if (!so) { skipped.push({ ...base, reason: "target not owned by any workspace package" }); continue; }
  const group = pkgs.filter((p) => p.sourceOwner === so);
  const toExports = /\.(tsx?|mts|cts)$/.test(e.to) ? exportsOf(toAbs) : null;
  const decided: { spec: ts.StringLiteralLike; text: string }[] = [];
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
        if (!needed.length) continue;
        if (needed.every((n) => { const a = toExports.get(n), b = ee.get(n); return a && b && alias(a) === alias(b); }) && !chosen) chosen = { p, sub };
      }
      if (chosen) break;
    }
    if (!chosen) { reason = u.names === "*" ? "namespace/side-effect/dynamic use without exact entry" : "no existing export entry exposes " + u.names.join(","); break; }
    decided.push({ spec: u.spec, text: chosen.p.name + (chosen.sub === "." ? "" : chosen.sub.slice(1)) });
  }
  if (reason) { skipped.push({ ...base, reason, names: uses.some((u) => u.names === "*") ? "*" : [...new Set(uses.flatMap((u) => u.names as string[]))] }); continue; }
  for (const d of decided) {
    const list = edits.get(fromAbs) ?? []; list.push({ start: d.spec.getStart(sf) + 1, end: d.spec.getEnd() - 1, text: d.text }); edits.set(fromAbs, list);
    fixes.push({ from: e.from, specifier: e.specifier, to: e.to, replacement: d.text });
  }
}
if (apply) for (const [file, list] of edits) {
  let s = readFileSync(file, "utf8");
  for (const ed of list.sort((a, b) => b.start - a.start)) s = s.slice(0, ed.start) + ed.text + s.slice(ed.end);
  writeFileSync(file, s);
}
writeFileSync(outPath!, JSON.stringify({ fixes, skipped }, null, 1));
console.log("edges", edges.size, "fixable", fixes.length, "files", edits.size, "skipped", skipped.length);
const rc: Record<string, number> = {};
for (const s of skipped) { const k = (s.owner ?? "-").slice(-50) + " | " + s.reason.replace(/exposes .*/, "exposes ..."); rc[k] = (rc[k] ?? 0) + 1; }
console.log(rc);
