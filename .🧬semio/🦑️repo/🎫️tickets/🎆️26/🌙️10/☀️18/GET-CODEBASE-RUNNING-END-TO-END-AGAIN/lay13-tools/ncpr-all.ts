/**
 * 🧭 Re-points every `no-cross-package-relative-*` edge to an existing package export entry (identity of the deep origin per imported name), else records the missing entry.
 *
 *   bun ncpr-all.ts <edges.json> <plan.json> [apply]
 */
import ts from "typescript";
import { existsSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { ROOT, cliClosure, isFrozen, posix } from "../repoapi-tools/lib.ts";
import { isBootstrapFile, isConfigFile } from "../repoapi-tools/rules.ts";
import { nearestManifest, packageIndex, type Pkg } from "../repoapi-tools/packages.ts";

const [edgesPath, planPath, mode, ...excludePaths] = process.argv.slice(2) as string[];
const excluded = new Set<string>();
for (const x of excludePaths) { const j = JSON.parse(readFileSync(x, "utf8")); for (const e of Array.isArray(j) ? j : j.reverted) excluded.add(e.file + "\0" + e.before); }
const apply = mode === "apply";
const edges = (JSON.parse(readFileSync(edgesPath, "utf8")) as any[]).filter(e => e.rule.startsWith("no-cross-package-relative-") && e.specifier);
const index = packageIndex();
const closure = cliClosure();
const abs = (p: string): string => resolve(ROOT, p);
const rel = (p: string): string => posix(relative(ROOT, p));

const entryFile = (pkg: Pkg, value: unknown): string | null => {
  if (typeof value === "string") return value;
  if (Array.isArray(value)) { for (const v of value) { const r = entryFile(pkg, v); if (r) return r; } return null; }
  if (value && typeof value === "object") {
    const o = value as Record<string, unknown>;
    for (const k of ["semio-source", "import", "default", "types", "node"]) if (k in o) { const r = entryFile(pkg, o[k]); if (r) return r; }
    for (const v of Object.values(o)) { const r = entryFile(pkg, v); if (r) return r; }
  }
  return null;
};
const entriesOf = (pkg: Pkg): { sub: string; file: string }[] => {
  const out: { sub: string; file: string }[] = [];
  for (const [sub, value] of Object.entries(pkg.exports)) {
    if (!sub.startsWith(".") || sub.includes("*")) continue;
    const t = entryFile(pkg, value);
    if (t && t.startsWith("./")) out.push({ sub, file: rel(resolve(abs(pkg.dir), t)) });
  }
  return out;
};
const entryCache = new Map<string, { sub: string; file: string }[]>();
const entries = (pkg: Pkg) => entryCache.get(pkg.name) ?? (entryCache.set(pkg.name, entriesOf(pkg)), entryCache.get(pkg.name)!);

type Origin = { file: string; name: string; typeKind: boolean; via: boolean };
const originCache = new Map<string, Map<string, Origin> | null>();
const sources = ["", ".ts", ".tsx", ".mts", ".js", ".mjs", ".cjs", ".json", "/🟦️.ts", "/🟦️.tsx"];
const resolveRel = (fromFile: string, spec: string): string | null => {
  if (!spec.startsWith(".")) return null;
  const base = resolve(dirname(abs(fromFile)), spec.split("?")[0]!);
  const alts = [base.replace(/\.js$/u, ".ts"), base.replace(/\.js$/u, ".tsx")];
  for (const c of [...sources.map(s => base + s), ...alts]) { try { if (statSync(c).isFile()) return rel(c); } catch { /* next */ } }
  return null;
};
const parse = (file: string): ts.SourceFile => ts.createSourceFile(file, readFileSync(abs(file), "utf8"), ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const has = (n: ts.Node, k: ts.SyntaxKind): boolean => !!ts.getModifiers(n as ts.HasModifiers)?.some(m => m.kind === k);

function originsOf(file: string, depth = 0): Map<string, Origin> | null {
  if (originCache.has(file)) return originCache.get(file)!;
  originCache.set(file, null);
  if (depth > 8 || !/\.(ts|tsx|mts|js|mjs|cjs)$/u.test(file)) return null;
  let sf: ts.SourceFile;
  try { sf = parse(file); } catch { return null; }
  const out = new Map<string, Origin>();
  const self = (name: string, typeKind = false): void => { out.set(name, { file, name, typeKind, via: false }); };
  for (const st of sf.statements) {
    if (ts.isExportDeclaration(st)) {
      const target = st.moduleSpecifier && ts.isStringLiteral(st.moduleSpecifier) ? st.moduleSpecifier.text : null;
      const resolved = target ? resolveRel(file, target) : null;
      const sub = resolved ? originsOf(resolved, depth + 1) : null;
      if (st.exportClause && ts.isNamedExports(st.exportClause)) {
        for (const e of st.exportClause.elements) {
          if (!target) { self(e.name.text); continue; }
          const orig = (e.propertyName ?? e.name).text;
          const base = sub?.get(orig) ?? { file: resolved ?? "bare:" + target, name: orig, typeKind: false, via: false };
          out.set(e.name.text, { ...base, via: base.via || st.isTypeOnly || e.isTypeOnly });
        }
      } else if (st.exportClause && ts.isNamespaceExport(st.exportClause)) out.set(st.exportClause.name.text, { file: resolved ?? "bare:" + target, name: "*", typeKind: false, via: st.isTypeOnly });
      else if (target) {
        if (sub) { for (const [k, v] of sub) if (k !== "default" && !out.has(k)) out.set(k, { ...v, via: v.via || st.isTypeOnly }); }
        else out.set("*bare:" + target, { file: "bare:" + target, name: "*", typeKind: false, via: false });
      }
    } else if (ts.isExportAssignment(st)) self("default");
    else if (has(st, ts.SyntaxKind.ExportKeyword)) {
      if (ts.isVariableStatement(st)) for (const d of st.declarationList.declarations) { if (ts.isIdentifier(d.name)) self(d.name.text); else for (const b of d.name.elements) if (ts.isBindingElement(b) && ts.isIdentifier(b.name)) self(b.name.text); }
      else if (ts.isFunctionDeclaration(st) || ts.isClassDeclaration(st)) { if (has(st, ts.SyntaxKind.DefaultKeyword)) self("default"); else if (st.name) self(st.name.text); }
      else if (ts.isInterfaceDeclaration(st) || ts.isTypeAliasDeclaration(st)) self(st.name.text, true);
      else if (ts.isEnumDeclaration(st) || ts.isModuleDeclaration(st)) self((st.name as ts.Identifier).text);
    }
  }
  originCache.set(file, out);
  return out;
}

type Use = { names: string[] | "*"; node: ts.StringLiteralLike };
function usesOf(file: string, specifier: string): Use[] {
  const sf = parse(file);
  const uses: Use[] = [];
  const visit = (n: ts.Node): void => {
    if (ts.isImportDeclaration(n) && ts.isStringLiteral(n.moduleSpecifier) && n.moduleSpecifier.text === specifier) {
      const c = n.importClause; const names: string[] = []; let star = false;
      if (!c) star = true;
      else { if (c.name) names.push("default"); if (c.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) star = true; else for (const e of c.namedBindings.elements) names.push((e.propertyName ?? e.name).text); } }
      uses.push({ names: star ? "*" : names, node: n.moduleSpecifier });
    } else if (ts.isExportDeclaration(n) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier) && n.moduleSpecifier.text === specifier) {
      const names: string[] = []; let star = false;
      if (!n.exportClause || ts.isNamespaceExport(n.exportClause)) star = true; else for (const e of n.exportClause.elements) names.push((e.propertyName ?? e.name).text);
      uses.push({ names: star ? "*" : names, node: n.moduleSpecifier });
    } else if (ts.isCallExpression(n) && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0]) && n.arguments[0].text === specifier && (n.expression.kind === ts.SyntaxKind.ImportKeyword || (ts.isIdentifier(n.expression) && n.expression.text === "require"))) uses.push({ names: "*", node: n.arguments[0] });
    else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal) && n.argument.literal.text === specifier) {
      const q = n.qualifier; const first = q ? (function f(x: ts.EntityName): string { return ts.isIdentifier(x) ? x.text : f(x.left); })(q) : "*";
      uses.push({ names: first === "*" ? "*" : [first], node: n.argument.literal });
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  return uses;
}

const forceRelative = (file: string): boolean => closure.has(file) || /\.(mjs|cjs)$/u.test(file) || file.endsWith("📜️script.ts") || isConfigFile(file) || isBootstrapFile(file);
const specifierFor = (from: string, pkg: Pkg, e: { sub: string; file: string }): string => {
  if (forceRelative(from)) { let r = posix(relative(dirname(abs(from)), abs(e.file))); if (!r.startsWith(".")) r = "./" + r; return r; }
  return pkg.name + (e.sub === "." ? "" : e.sub.slice(1));
};

type Edit = { file: string; before: string; after: string; start: number; end: number; pkg: string; sub: string; to: string };
type Skip = { from: string; to: string; specifier: string; pkg: string; reason: string; names: string[] | "*" };
const edits: Edit[] = [];
const skipped: Skip[] = [];
const depGaps = new Map<string, Set<string>>();
const groups = new Map<string, any[]>();
for (const e of edges) { const k = e.from + "\0" + e.specifier; (groups.get(k) ?? groups.set(k, []).get(k)!).push(e); }

for (const group of groups.values()) {
  const e0 = group[0];
  const pkgNames = group.map(g => (g.rule as string).slice("no-cross-package-relative-".length));
  const targets = pkgNames.map(n => index.byName.get(n)).filter((p): p is Pkg => !!p);
  const to: string = e0.to;
  const skip = (reason: string, names: string[] | "*" = "*"): void => { skipped.push({ from: e0.from, to, specifier: e0.specifier, pkg: pkgNames.join(","), reason, names }); };
  if (isFrozen(e0.from)) { skip("frozen"); continue; }
  if (excluded.has(e0.from + "\0" + e0.specifier)) { skip("reverted: rewrite introduced another violation, a cycle or a barrel in the zero-touch closure"); continue; }
  if (e0.specifier.includes("?")) { skip("query import"); continue; }
  if (!targets.length) { skip("target package unknown"); continue; }
  if (!existsSync(abs(e0.from))) { skip("source missing"); continue; }
  let uses: Use[];
  try { uses = usesOf(e0.from, e0.specifier); } catch { skip("parse failure"); continue; }
  if (!uses.length) { skip("no import node found"); continue; }
  const sourceOwners = new Set(targets.map(t => t.sourceOwner));
  const candidates = index.all.filter(p => sourceOwners.has(p.sourceOwner));
  const toOrigins = to.endsWith(".json") ? null : originsOf(to);
  const allNames = [...new Set(uses.flatMap(u => (u.names === "*" ? ["*"] : u.names)))];
  let failure = "";
  const chosen: { u: Use; pkg: Pkg; entry: { sub: string; file: string } }[] = [];
  for (const u of uses) {
    let best: { pkg: Pkg; entry: { sub: string; file: string }; score: number } | null = null;
    for (const p of candidates) for (const en of entries(p)) {
      let score = 0;
      if (en.file === to) score = 1e9;
      else if (toOrigins) {
        const eo = originsOf(en.file);
        if (!eo) continue;
        const needed = u.names === "*" ? [...toOrigins.keys()].filter(k => !k.startsWith("*bare:")) : u.names;
        if (!needed.length) continue;
        const ok = needed.every(n => { const a = toOrigins.get(n), b = eo.get(n); return a && b && a.file === b.file && a.name === b.name && !(b.via && !b.typeKind && !(a.via && !a.typeKind)); });
        if (!ok) continue;
        score = 1e6 - eo.size;
      } else continue;
      if (!best || score > best.score) best = { pkg: p, entry: en, score };
    }
    if (!best) { failure = u.names === "*" ? "namespace, side-effect or dynamic use without covering entry" : "no existing entry exposes " + u.names.join(","); break; }
    chosen.push({ u, pkg: best.pkg, entry: best.entry });
  }
  if (failure) { skip(failure, allNames.includes("*") ? "*" : allNames); continue; }
  const sf = parse(e0.from);
  for (const c of chosen) {
    const after = specifierFor(e0.from, c.pkg, c.entry);
    edits.push({ file: e0.from, before: e0.specifier, after, start: c.u.node.getStart(sf) + 1, end: c.u.node.getEnd() - 1, pkg: c.pkg.name, sub: c.entry.sub, to });
    if (!after.startsWith(".")) {
      const mf = nearestManifest(e0.from, index.all);
      if (mf && mf.name !== c.pkg.name && !mf.deps.has(c.pkg.name)) (depGaps.get(mf.packageJson) ?? depGaps.set(mf.packageJson, new Set()).get(mf.packageJson)!).add(c.pkg.name);
    }
  }
}

const applied: Edit[] = [], stale: Edit[] = [], unresolved: Edit[] = [];
if (apply) {
  const byFile = new Map<string, Edit[]>();
  for (const ed of edits) (byFile.get(ed.file) ?? byFile.set(ed.file, []).get(ed.file)!).push(ed);
  for (const [file, list] of byFile) {
    let text = readFileSync(abs(file), "utf8");
    const ok = list.filter(ed => text.slice(ed.start, ed.end) === ed.before);
    stale.push(...list.filter(ed => !ok.includes(ed)));
    for (const ed of ok.sort((a, b) => b.start - a.start)) {
      if (!ed.after.startsWith(".")) { try { Bun.resolveSync(ed.after, dirname(abs(file))); } catch { unresolved.push(ed); continue; } }
      else if (!existsSync(resolve(dirname(abs(file)), ed.after))) { unresolved.push(ed); continue; }
      text = text.slice(0, ed.start) + ed.after + text.slice(ed.end);
      applied.push(ed);
    }
    if (text !== readFileSync(abs(file), "utf8")) writeFileSync(abs(file), text);
  }
}

const reasons: Record<string, number> = {};
for (const s of skipped) { const k = s.reason.replace(/exposes .*/u, "exposes ..."); reasons[k] = (reasons[k] ?? 0) + 1; }
writeFileSync(planPath, JSON.stringify({ edits, skipped, depGaps: Object.fromEntries([...depGaps].map(([k, v]) => [k, [...v]])), applied: applied.length, stale: stale.length, unresolved: unresolved.map(u => ({ file: u.file, after: u.after })) }, null, 1));
console.log(JSON.stringify({ groups: groups.size, edits: edits.length, skipped: skipped.length, reasons, applied: applied.length, stale: stale.length, unresolved: unresolved.length, files: new Set(edits.map(e => e.file)).size }));
