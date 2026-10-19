/**
 * 🧭 Re-points every `no-cross-package-relative-*` edge to an existing package export entry (identity of the deep origin per imported name), else records the missing entry.
 *
 *   bun ncpr-all.ts <edges.json> <plan.json> [apply]
 */
import ts from "typescript";
import { existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { ROOT, cliClosure, isFrozen, posix } from "../repoapi-tools/lib.ts";
import { isBootstrapFile, isConfigFile } from "../repoapi-tools/rules.ts";
import { nearestManifest, packageIndex, type Pkg } from "../repoapi-tools/packages.ts";

const [edgesPath, planPath, mode, ...excludePaths] = process.argv.slice(2) as string[];
const excluded = new Set<string>();
for (const x of excludePaths) { const j = JSON.parse(readFileSync(x, "utf8")); for (const e of Array.isArray(j) ? j : j.reverted) excluded.add(e.file + "\0" + e.before); }
const apply = mode === "apply";
const SKIP_FROM = process.env.LAY15_SKIP_FROM ? new RegExp(process.env.LAY15_SKIP_FROM, "u") : null;
const edges = (JSON.parse(readFileSync(edgesPath, "utf8")) as any[]).filter(e => /^no-cross-technology-/u.test(e.rule) && e.specifier);
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
function namespaceMembers(sf: ts.SourceFile, local: string): string[] | "*" {
  const members = new Set<string>();
  let escapes = false;
  const visit = (n: ts.Node): void => {
    if (ts.isImportDeclaration(n) || ts.isExportDeclaration(n)) return;
    if (ts.isPropertyAccessExpression(n) && ts.isIdentifier(n.expression) && n.expression.text === local) { members.add(n.name.text); return; }
    if (ts.isElementAccessExpression(n) && ts.isIdentifier(n.expression) && n.expression.text === local) { if (ts.isStringLiteralLike(n.argumentExpression)) members.add(n.argumentExpression.text); else escapes = true; return; }
    if (ts.isQualifiedName(n) && ts.isIdentifier(n.left) && n.left.text === local) { members.add(n.right.text); return; }
    if (ts.isTypeQueryNode(n) && ts.isIdentifier(n.exprName) && n.exprName.text === local) { escapes = true; return; }
    if (ts.isIdentifier(n) && n.text === local) { const p = n.parent; if (!(ts.isImportClause(p) || ts.isNamespaceImport(p))) escapes = true; return; }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  return escapes ? "*" : [...members];
}
function usesOf(file: string, specifier: string): Use[] {
  const sf = parse(file);
  const uses: Use[] = [];
  const visit = (n: ts.Node): void => {
    if (ts.isImportDeclaration(n) && ts.isStringLiteral(n.moduleSpecifier) && n.moduleSpecifier.text === specifier) {
      const c = n.importClause; const names: string[] = []; let star: string[] | "*" | null = null;
      if (!c) star = "*";
      else { if (c.name) names.push("default"); if (c.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) star = namespaceMembers(sf, c.namedBindings.name.text); else for (const e of c.namedBindings.elements) names.push((e.propertyName ?? e.name).text); } }
      uses.push({ names: star === "*" ? "*" : [...names, ...(star ?? [])], node: n.moduleSpecifier });
    } else if (ts.isExportDeclaration(n) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier) && n.moduleSpecifier.text === specifier) {
      const names: string[] = []; let star = false;
      if (!n.exportClause || ts.isNamespaceExport(n.exportClause)) star = true; else for (const e of n.exportClause.elements) names.push((e.propertyName ?? e.name).text);
      uses.push({ names: star ? "*" : names, node: n.moduleSpecifier });
    } else if (ts.isCallExpression(n) && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0]) && n.arguments[0].text === specifier && (n.expression.kind === ts.SyntaxKind.ImportKeyword || (ts.isIdentifier(n.expression) && n.expression.text === "require"))) {
      let p: ts.Node = n.parent;
      while (ts.isParenthesizedExpression(p) || ts.isAwaitExpression(p) || ts.isAsExpression(p)) p = p.parent;
      let names: string[] | "*" = "*";
      if (ts.isVariableDeclaration(p) && ts.isObjectBindingPattern(p.name) && !p.name.elements.some(e => e.dotDotDotToken)) names = p.name.elements.map(e => (e.propertyName && ts.isIdentifier(e.propertyName) ? e.propertyName : ts.isIdentifier(e.name) ? e.name : null)?.text ?? "*");
      if (Array.isArray(names) && names.includes("*")) names = "*";
      uses.push({ names, node: n.arguments[0] });
    } else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal) && n.argument.literal.text === specifier) {
      const q = n.qualifier; const first = q ? (function f(x: ts.EntityName): string { return ts.isIdentifier(x) ? x.text : f(x.left); })(q) : "*";
      uses.push({ names: first === "*" ? "*" : [first], node: n.argument.literal });
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  return uses;
}

const jsonCache = new Map<string, boolean>();
function jsonGlueExposes(entry: string, json: string): boolean {
  const key = entry + "\0" + json;
  if (jsonCache.has(key)) return jsonCache.get(key)!;
  let ok = false;
  try {
    if (/\.(ts|tsx|mts)$/u.test(entry)) {
      const sf = parse(entry); let reaches = false, exports = false;
      for (const st of sf.statements) {
        if ((ts.isImportDeclaration(st) || ts.isExportDeclaration(st)) && st.moduleSpecifier && ts.isStringLiteral(st.moduleSpecifier) && resolveRel(entry, st.moduleSpecifier.text) === json) reaches = true;
        if (ts.isExportAssignment(st)) exports = true;
        if (ts.isExportDeclaration(st) && st.exportClause && ts.isNamedExports(st.exportClause) && st.exportClause.elements.some(e => e.name.text === "default")) exports = true;
      }
      ok = reaches && exports;
    }
  } catch { ok = false; }
  jsonCache.set(key, ok);
  return ok;
}

const forceRelative = (file: string): boolean => closure.has(file) || /\.(mjs|cjs)$/u.test(file) || file.endsWith("📜️script.ts") || isConfigFile(file) || isBootstrapFile(file);
const specifierFor = (from: string, pkg: Pkg, e: { sub: string; file: string }): string => {
  if (forceRelative(from)) { let r = posix(relative(dirname(abs(from)), abs(e.file))); if (!r.startsWith(".")) r = "./" + r; return r; }
  return pkg.name + (e.sub === "." ? "" : e.sub.slice(1));
};

type Edit = { file: string; before: string; after: string; start: number; end: number; pkg: string; sub: string; to: string; names: string[] | "*"; narrow?: boolean };
type Skip = { from: string; to: string; specifier: string; pkg: string; reason: string; names: string[] | "*" };
const edits: Edit[] = [];
const skipped: Skip[] = [];
const depGaps = new Map<string, Set<string>>();
const groups = new Map<string, any[]>();
for (const e of edges) { const k = e.from + "\0" + e.specifier; (groups.get(k) ?? groups.set(k, []).get(k)!).push(e); }

for (const group of groups.values()) {
  const e0 = group[0];
  const owner = nearestManifest(e0.to, index.all);
  const pkgNames = owner ? [owner.name] : [];
  const targets = owner && index.byName.has(owner.name) ? [index.byName.get(owner.name)!] : [];
  const widen = !targets.length;
  const to: string = e0.to;
  const skip = (reason: string, names: string[] | "*" = "*"): void => { skipped.push({ from: e0.from, to, specifier: e0.specifier, pkg: pkgNames.join(","), reason, names }); };
  if (isFrozen(e0.from)) { skip("frozen"); continue; }
  if (SKIP_FROM && SKIP_FROM.test(e0.from)) { skip("scope: owned by another executor"); continue; }
  if (excluded.has(e0.from + "\0" + e0.specifier)) { skip("reverted: rewrite introduced another violation, a cycle or a barrel in the zero-touch closure"); continue; }
  if (e0.specifier.includes("?")) { skip("query import"); continue; }
  if (!existsSync(abs(e0.from))) { skip("source missing"); continue; }
  let uses: Use[];
  try { uses = usesOf(e0.from, e0.specifier); } catch { skip("parse failure"); continue; }
  if (!uses.length) { skip("no import node found"); continue; }
  const sourceOwners = new Set(targets.map(t => t.sourceOwner));
  const candidates = widen ? index.all.filter(p => !/^\.🧬semio\//u.test(p.dir)) : index.all.filter(p => sourceOwners.has(p.sourceOwner));
  const toOrigins = to.endsWith(".json") ? null : originsOf(to);
  const allNames = [...new Set(uses.flatMap(u => (u.names === "*" ? ["*"] : u.names)))];
  let failure = "";
  const chosen: { u: Use; pkg: Pkg; entry: { sub: string; file: string } }[] = [];
  for (const u of uses) {
    let best: { pkg: Pkg; entry: { sub: string; file: string }; score: number } | null = null;
    for (const p of candidates) for (const en of entries(p)) {
      let score = 0;
      if (en.file === to) score = 1e9;
      else if (to.endsWith(".json")) { if ((u.names === "*" || (u.names.length === 1 && u.names[0] === "default")) && jsonGlueExposes(en.file, to)) score = 1e6; else continue; }
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
    edits.push({ file: e0.from, before: e0.specifier, after, start: c.u.node.getStart(sf) + 1, end: c.u.node.getEnd() - 1, pkg: c.pkg.name, sub: c.entry.sub, to, names: c.u.names });
    if (!after.startsWith(".")) {
      const mf = nearestManifest(e0.from, index.all);
      if (mf && mf.name !== c.pkg.name && !mf.deps.has(c.pkg.name)) (depGaps.get(mf.packageJson) ?? depGaps.set(mf.packageJson, new Set()).get(mf.packageJson)!).add(c.pkg.name);
    }
  }
}



const BARREL_SUBS = new Set(["./artifacts", "./authoring", "./build", "./cargo", "./dev", "./inventory", "./laws", "./policy", "./process", "./storage", "./testing"]);
const LIB_ROOT = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const LIB_GLUE = LIB_ROOT + "/📦️packages/🟦️typescript";
const DOMAIN_DIRS: Record<string, string> = { "./artifacts": "📦️artifacts", "./authoring": "🏗️authoring", "./build": "🏗️build", "./cargo": "🦀️cargo", "./dev": "🎮️playground", "./inventory": "📇️inventory", "./laws": "⚖️laws", "./policy": "🚨️policy", "./process": "🏃️process", "./storage": "💾️storage", "./testing": "🧪️testing" };
const narrowKeys: { key: string; glue: string; target: string; domain: string; names: string[]; importers: string[] }[] = [];
const barrelDropped: { file: string; before: string; sub: string; to: string; reason: string }[] = [];
if (mode === "narrow") {
  const keep: Edit[] = [];
  const groupsByTarget = new Map<string, Edit[]>();
  for (const ed of edits) {
    if (ed.pkg === "@semio-tech/repo-lib" && BARREL_SUBS.has(ed.sub) && ed.to.startsWith(LIB_ROOT + "/")) (groupsByTarget.get(ed.to) ?? groupsByTarget.set(ed.to, []).get(ed.to)!).push(ed);
    else keep.push(ed);
  }
  for (const [to, list] of groupsByTarget) {
    const domainDir = DOMAIN_DIRS[list[0]!.sub]!;
    const relTarget = to.slice(LIB_ROOT.length + 1).split("/");
    const at = relTarget.indexOf(domainDir);
    const restSegs = (at >= 0 ? relTarget.slice(at + 1) : relTarget).filter(x => !/^🟦️\.(ts|tsx)$/u.test(x));
    const leafFile = (at >= 0 ? relTarget.slice(at + 1) : relTarget).find(x => /^🟦️\.(ts|tsx)$/u.test(x)) ?? null;
    const dirSegs = restSegs.length || leafFile ? restSegs : [];
    const base = to.endsWith("🟦️.ts") || to.endsWith("🟦️.tsx") ? dirSegs : dirSegs.slice(0, -1).concat([dirSegs[dirSegs.length - 1]!.replace(/\.[^.]+$/u, "")]);
    const glue = [LIB_GLUE, domainDir, ...base, "🟦️.ts"].join("/");
    const ascii = (x: string): string => x.replace(/[^\x00-\x7f]/gu, "");
    const key = list[0]!.sub + "/" + (base.map(ascii).filter(Boolean).join("/") || "root");
    const names = new Set<string>(); let star = false;
    for (const ed of list) { if (ed.names === "*") star = true; else for (const n of ed.names) names.add(n); }
    const origins = originsOf(to);
    if (!origins) { for (const ed of list) barrelDropped.push({ file: ed.file, before: ed.before, sub: ed.sub, to, reason: "target unreadable" }); continue; }
    const need = star ? [...origins.keys()].filter(k => !k.startsWith("*bare:") && k !== "default") : [...names];
    const missing = need.filter(n => !origins.has(n));
    if (missing.length || !need.length) { for (const ed of list) barrelDropped.push({ file: ed.file, before: ed.before, sub: ed.sub, to, reason: missing.length ? "names not exported by target: " + missing.join(",") : "no names" }); continue; }
    const values = need.filter(n => { const o = origins.get(n)!; return !(o.typeKind || o.via); }).sort((a, b) => (a < b ? -1 : a > b ? 1 : 0));
    const types = need.filter(n => { const o = origins.get(n)!; return o.typeKind || o.via; }).sort((a, b) => (a < b ? -1 : a > b ? 1 : 0));
    const spec = posix(relative(dirname(abs(glue)), abs(to))); const specifier = spec.startsWith(".") ? spec : "./" + spec;
    const lines = [`/** 🧩️ Public \`@semio-tech/repo-lib${key.slice(1)}\` entry: explicit re-exports of the library module that owns each member. */`];
    if (values.length) lines.push(`export { ${values.join(", ")} } from "${specifier}";`);
    if (types.length) lines.push(`export type { ${types.join(", ")} } from "${specifier}";`);
    if (apply && !existsSync(abs(glue))) { mkdirSync(dirname(abs(glue)), { recursive: true }); writeFileSync(abs(glue), lines.join("\n") + "\n"); }
    else if (apply && existsSync(abs(glue))) {
      const have = originsOf(glue);
      const lacking = need.filter(n => !have?.has(n));
      if (lacking.length) { const t = readFileSync(abs(glue), "utf8").replace(/\n*$/u, "\n"); writeFileSync(abs(glue), t + `export { ${lacking.filter(n => values.includes(n)).join(", ")} } from "${specifier}";\n`.replace("export {  }", "")); }
    }
    narrowKeys.push({ key, glue, target: to, domain: list[0]!.sub, names: need, importers: [...new Set(list.map(l => l.file))] });
    for (const ed of list) {
      if (forceRelative(ed.file)) { let r = posix(relative(dirname(abs(ed.file)), abs(glue))); if (!r.startsWith(".")) r = "./" + r; keep.push({ ...ed, after: r, sub: key, narrow: true }); }
      else barrelDropped.push({ file: ed.file, before: ed.before, sub: ed.sub, to, reason: "bare specifier needs manifest key " + key });
    }
  }
  edits.length = 0; edits.push(...keep);
}

const closureDropped: { file: string; before: string; after: string }[] = [];
if (mode === "narrow" || mode === "apply") {
  const keep: Edit[] = [];
  for (const ed of edits) {
    const glueTarget = !ed.after.startsWith(".") || ed.after.includes("📦️packages/🟦️typescript/");
    if (closure.has(ed.file) && glueTarget && !ed.narrow) closureDropped.push({ file: ed.file, before: ed.before, after: ed.after });
    else keep.push(ed);
  }
  edits.length = 0; edits.push(...keep);
}

const glueMode = mode === "glue";
const glueReport: { glue: string; target: string; added: string[]; types: string[]; conflicts: string[] }[] = [];
const unplannedGaps: { to: string; pkg: string; names: string[] | "*" }[] = [];
if (glueMode) {
  const final = JSON.parse(readFileSync(resolve(import.meta.dir, "../repoapi-tools/exports-final.json"), "utf8"));
  const wanted = new Map<string, { glue: string; names: Set<string> | "*"; pkg: string }>();
  for (const s of skipped) {
    if (!/^(no existing entry exposes|namespace)/u.test(s.reason)) continue;
    const pkgs = s.pkg.split(",").map(n => index.byName.get(n)).filter((p): p is Pkg => !!p);
    if (!pkgs.length) continue;
    const owners = new Set(pkgs.map(p => p.sourceOwner));
    let glueFile: string | undefined, pkgName = "";
    for (const p of index.all.filter(q => owners.has(q.sourceOwner))) {
      const entry = Object.values(final.packages[p.name]?.entries ?? {}).find((e: any) => e.targetFile === s.to && e.glueFile && existsSync(abs(e.glueFile))) as any;
      if (entry) { glueFile = entry.glueFile; pkgName = p.name; break; }
    }
    if (!glueFile) { unplannedGaps.push({ to: s.to, pkg: s.pkg, names: s.names }); continue; }
    const row = wanted.get(s.to) ?? wanted.set(s.to, { glue: glueFile, names: new Set(), pkg: pkgName }).get(s.to)!;
    if (s.names === "*" || row.names === "*") row.names = "*"; else for (const n of s.names) row.names.add(n);
  }
  for (const [to, row] of wanted) {
    if (to.endsWith(".json")) continue;
    const targetOrigins = originsOf(to);
    const glueOrigins = originsOf(row.glue);
    if (!targetOrigins || !glueOrigins) continue;
    const need = row.names === "*" ? [...targetOrigins.keys()].filter(k => !k.startsWith("*bare:") && k !== "default") : [...row.names];
    const missing: string[] = [], conflicts: string[] = [];
    for (const n of need) {
      const t = targetOrigins.get(n);
      if (!t) { conflicts.push(n + " (not exported by target)"); continue; }
      const g = glueOrigins.get(n);
      if (!g) { missing.push(n); continue; }
      if (g.file !== t.file || g.name !== t.name) conflicts.push(n + " (different origin in glue)");
    }
    if (!missing.length) { if (conflicts.length) glueReport.push({ glue: row.glue, target: to, added: [], types: [], conflicts }); continue; }
    const values = missing.filter(n => { const o = targetOrigins.get(n)!; return !(o.typeKind || o.via); }).sort();
    const types = missing.filter(n => { const o = targetOrigins.get(n)!; return o.typeKind || o.via; }).sort();
    let text = readFileSync(abs(row.glue), "utf8");
    const spec = posix(relative(dirname(abs(row.glue)), abs(to))); const specifier = spec.startsWith(".") ? spec : "./" + spec;
    const gsf = parse(row.glue);
    const merge = (names: string[], typeOnly: boolean): void => {
      if (!names.length) return;
      for (const st of gsf.statements) {
        if (!ts.isExportDeclaration(st) || !st.moduleSpecifier || !ts.isStringLiteral(st.moduleSpecifier) || st.moduleSpecifier.text !== specifier || st.isTypeOnly !== typeOnly || !st.exportClause || !ts.isNamedExports(st.exportClause) || st.attributes) continue;
        const existing = st.exportClause.elements.map(e => e.getText(gsf));
        const all = [...new Set([...existing, ...names])].sort((a, b) => (a < b ? -1 : a > b ? 1 : 0));
        const replacement = `export ${typeOnly ? "type " : ""}{ ${all.join(", ")} } from "${specifier}";`;
        const start = st.getStart(gsf), end = st.getEnd();
        edits2.push({ start, end, text: replacement });
        return;
      }
      edits2.push({ start: -1, end: -1, text: `export ${typeOnly ? "type " : ""}{ ${names.join(", ")} } from "${specifier}";` });
    };
    const edits2: { start: number; end: number; text: string }[] = [];
    merge(values, false); merge(types, true);
    const inplace = edits2.filter(e => e.start >= 0).sort((a, b) => b.start - a.start);
    for (const e of inplace) text = text.slice(0, e.start) + e.text + text.slice(e.end);
    const appended = edits2.filter(e => e.start < 0).map(e => e.text);
    if (appended.length) text = text.replace(/\n*$/u, "\n") + appended.join("\n") + "\n";
    writeFileSync(abs(row.glue), text);
    glueReport.push({ glue: row.glue, target: to, added: values, types, conflicts });
  }
  writeFileSync(planPath.replace(/\.json$/u, "-glue.json"), JSON.stringify({ glueReport, unplannedGaps }, null, 1));
  console.log(JSON.stringify({ glueFilesEdited: glueReport.filter(r => r.added.length || r.types.length).length, namesAdded: glueReport.reduce((n, r) => n + r.added.length + r.types.length, 0), conflicts: glueReport.filter(r => r.conflicts.length).length, unplanned: unplannedGaps.length }));
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
writeFileSync(planPath, JSON.stringify({ edits, skipped, narrowKeys, barrelDropped, closureDropped, depGaps: Object.fromEntries([...depGaps].map(([k, v]) => [k, [...v]])), applied: applied.length, stale: stale.length, unresolved: unresolved.map(u => ({ file: u.file, after: u.after })) }, null, 1));
console.log(JSON.stringify({ groups: groups.size, edits: edits.length, skipped: skipped.length, reasons, applied: applied.length, stale: stale.length, unresolved: unresolved.length, files: new Set(edits.map(e => e.file)).size }));
