/**
 * 🧭 Re-points boundary-violating imports to an existing public entry of the admitted package (identity of the deep origin per imported name).
 *
 *   bun sdkfix.ts <edges.tsv> <plan.json> <scope> [apply]
 *
 * scope `sdk`: candidates are the entries of `@semio-tech/framework` only (plugins-framework-sdk-only);
 * scope `any`: every workspace package (no-cross-technology). `📜️script.ts`, `.mjs/.cjs` and config files keep a relative path to the glue file.
 */
import ts from "typescript";
import { existsSync, readFileSync, readdirSync, realpathSync, statSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const [edgesPath, planPath, scope, mode, policyPath] = process.argv.slice(2) as [string, string, "sdk" | "any", string | undefined, string | undefined];
const policy = JSON.parse(readFileSync(policyPath ?? ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/lay16-run/policy.json", "utf8"));
const arr = (v: unknown): string[] => (v === undefined ? [] : Array.isArray(v) ? v : [v as string]);
const rx = (v: unknown): RegExp[] => arr(v).map(x => new RegExp(x, "u"));
const anyRx = (rs: RegExp[], x: string): boolean => rs.some(r => r.test(x));
const rules = policy.forbidden
  .filter((r: any) => !/^no-cross-package-relative/u.test(r.name) && r.name !== "no-circular" && !r.to.circular && !/^not-to-unlisted|^no-escaping/u.test(r.name))
  .map((r: any) => ({ name: r.name, fromPath: rx(r.from?.path), fromNot: rx(r.from?.pathNot), toPath: rx(r.to?.path), toNot: rx(r.to?.pathNot), types: arr(r.to?.dependencyTypes), typesNot: arr(r.to?.dependencyTypesNot) }));
const violationsOf = (from: string, to: string, types: string[]): string[] => rules.filter((rule: any) => anyRx(rule.fromPath, from) && !anyRx(rule.fromNot, from) && (!rule.toPath.length || anyRx(rule.toPath, to)) && !anyRx(rule.toNot, to) && (!rule.types.length || rule.types.some((t: string) => types.includes(t))) && !rule.typesNot.some((t: string) => types.includes(t))).map((rule: any) => rule.name);

const apply = mode === "apply";
const SDK = "@semio-tech/framework";
const abs = (p: string): string => resolve(ROOT, p);
const rel = (p: string): string => relative(ROOT, p).split("\\").join("/");

type Pkg = { name: string; dir: string; sourceOwner: string; exports: Record<string, unknown> };
const pkgs = new Map<string, Pkg>();
for (const e of readdirSync(resolve(ROOT, "node_modules/@semio-tech"))) {
  try {
    const dir = realpathSync(resolve(ROOT, "node_modules/@semio-tech", e));
    const j = JSON.parse(readFileSync(resolve(dir, "package.json"), "utf8"));
    const d = rel(dir);
    pkgs.set(j.name, { name: j.name, dir: d, sourceOwner: d.includes("/📦️packages/") ? d.split("/📦️packages/")[0]! : d, exports: typeof j.exports === "object" && j.exports ? j.exports : {} });
  } catch {}
}
const entryFile = (value: unknown): string | null => {
  if (typeof value === "string") return value;
  if (Array.isArray(value)) { for (const v of value) { const r = entryFile(v); if (r) return r; } return null; }
  if (value && typeof value === "object") {
    const o = value as Record<string, unknown>;
    for (const k of ["semio-source", "import", "default", "types", "node"]) if (k in o) { const r = entryFile(o[k]); if (r) return r; }
  }
  return null;
};
type Entry = { sub: string; file: string };
const entriesOf = (p: Pkg): Entry[] => {
  const out: Entry[] = [];
  for (const [sub, value] of Object.entries(p.exports)) {
    if (!sub.startsWith(".") || sub.includes("*")) continue;
    const t = entryFile(value);
    if (t && t.startsWith("./")) out.push({ sub, file: rel(resolve(abs(p.dir), t)) });
  }
  return out;
};
const entryCache = new Map<string, Entry[]>();
const entries = (p: Pkg): Entry[] => entryCache.get(p.name) ?? (entryCache.set(p.name, entriesOf(p)), entryCache.get(p.name)!);

type Origin = { file: string; name: string; typeKind: boolean; via: boolean };
const originCache = new Map<string, Map<string, Origin> | null>();
const sources = ["", ".ts", ".tsx", ".mts", ".js", ".mjs", ".cjs", ".json", "/🟦️.ts", "/🟦️.tsx"];
const resolveWorkspace = (spec: string): string | null => {
  const clean = spec.split("?")[0]!;
  const parts = clean.split("/");
  const p = pkgs.get(parts.slice(0, 2).join("/"));
  if (!p) return null;
  const sub = "." + clean.slice(parts.slice(0, 2).join("/").length);
  return entries(p).find(e => e.sub === sub)?.file ?? null;
};
const resolveRel = (fromFile: string, spec: string): string | null => {
  if (spec.startsWith("@semio-tech/")) return resolveWorkspace(spec);
  if (!spec.startsWith(".")) return null;
  const base = resolve(dirname(abs(fromFile)), spec.split("?")[0]!);
  const alts = [base.replace(/\.js$/u, ".ts"), base.replace(/\.js$/u, ".tsx")];
  for (const c of [...sources.map(s => base + s), ...alts]) { try { if (statSync(c).isFile()) return rel(c); } catch {} }
  return null;
};
const parse = (file: string): ts.SourceFile => ts.createSourceFile(file, readFileSync(abs(file), "utf8"), ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const has = (n: ts.Node, k: ts.SyntaxKind): boolean => !!ts.getModifiers(n as ts.HasModifiers)?.some(m => m.kind === k);

function originsOf(file: string, depth = 0): Map<string, Origin> | null {
  if (originCache.has(file)) return originCache.get(file)!;
  originCache.set(file, null);
  if (depth > 10 || !/\.(ts|tsx|mts|js|mjs|cjs)$/u.test(file)) return null;
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
function usesOf(file: string, specifier: string): { uses: Use[]; loose: ts.StringLiteralLike[] } {
  const sf = parse(file);
  const uses: Use[] = [];
  const claimed = new Set<ts.Node>();
  const visit = (n: ts.Node): void => {
    if (ts.isImportDeclaration(n) && ts.isStringLiteral(n.moduleSpecifier) && n.moduleSpecifier.text === specifier) {
      const c = n.importClause; const names: string[] = []; let star: string[] | "*" | null = null;
      if (!c) star = "*";
      else { if (c.name) names.push("default"); if (c.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) star = namespaceMembers(sf, c.namedBindings.name.text); else for (const e of c.namedBindings.elements) names.push((e.propertyName ?? e.name).text); } }
      uses.push({ names: star === "*" ? "*" : [...names, ...(star ?? [])], node: n.moduleSpecifier }); claimed.add(n.moduleSpecifier);
    } else if (ts.isExportDeclaration(n) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier) && n.moduleSpecifier.text === specifier) {
      const names: string[] = []; let star = false;
      if (!n.exportClause || ts.isNamespaceExport(n.exportClause)) star = true; else for (const e of n.exportClause.elements) names.push((e.propertyName ?? e.name).text);
      uses.push({ names: star ? "*" : names, node: n.moduleSpecifier }); claimed.add(n.moduleSpecifier);
    } else if (ts.isCallExpression(n) && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0]) && n.arguments[0].text === specifier && (n.expression.kind === ts.SyntaxKind.ImportKeyword || (ts.isIdentifier(n.expression) && n.expression.text === "require"))) {
      let p: ts.Node = n.parent;
      while (ts.isParenthesizedExpression(p) || ts.isAwaitExpression(p) || ts.isAsExpression(p)) p = p.parent;
      let names: string[] | "*" = "*";
      if (ts.isVariableDeclaration(p) && ts.isObjectBindingPattern(p.name) && !p.name.elements.some(e => e.dotDotDotToken)) names = p.name.elements.map(e => (e.propertyName && ts.isIdentifier(e.propertyName) ? e.propertyName : ts.isIdentifier(e.name) ? e.name : null)?.text ?? "*");
      if (Array.isArray(names) && names.includes("*")) names = "*";
      uses.push({ names, node: n.arguments[0] }); claimed.add(n.arguments[0]);
    } else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal) && n.argument.literal.text === specifier) {
      const q = n.qualifier; const first = q ? (function f(x: ts.EntityName): string { return ts.isIdentifier(x) ? x.text : f(x.left); })(q) : "*";
      uses.push({ names: first === "*" ? "*" : [first], node: n.argument.literal }); claimed.add(n.argument.literal);
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  const loose: ts.StringLiteralLike[] = [];
  const scan = (n: ts.Node): void => { if (ts.isStringLiteralLike(n) && n.text === specifier && !claimed.has(n)) loose.push(n); ts.forEachChild(n, scan); };
  scan(sf);
  return { uses, loose };
}

const exportSourceCache = new Map<string, Set<string>>();
function exportSources(entry: string): Set<string> {
  const hit = exportSourceCache.get(entry);
  if (hit) return hit;
  const out = new Set<string>();
  try { for (const st of parse(entry).statements) if (ts.isExportDeclaration(st) && st.moduleSpecifier && ts.isStringLiteral(st.moduleSpecifier)) { const r = resolveRel(entry, st.moduleSpecifier.text); if (r) out.add(r); } } catch {}
  exportSourceCache.set(entry, out);
  return out;
}

const tailMatch = (a: string, b: string): number => {
  const x = a.split("/"), y = b.split("/");
  let n = 0;
  while (n < x.length && n < y.length && x[x.length - 1 - n] === y[y.length - 1 - n]) n++;
  return n;
};

const GENERIC_SEGMENTS = new Set(["🧰️framework", "🔨️modules", "📦️packages", "🟦️typescript", "🟦️.ts", "🟦️.tsx", "🛍️products", "🧪️tests", "🧪️testing"]);
const sharesDomain = (a: string, b: string): boolean => { const x = new Set(a.split("/").filter(seg => !GENERIC_SEGMENTS.has(seg))); return b.split("/").some(seg => !GENERIC_SEGMENTS.has(seg) && x.has(seg)); };

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
  } catch {}
  jsonCache.set(key, ok);
  return ok;
}

const relativeKept = (file: string): boolean => /\.(mjs|cjs)$/u.test(file) || file.endsWith("📜️script.ts") || /(^|\/)(?:⚙️|🧪️)?(?:vite|vitest)\.config\.[cm]?[jt]s$/u.test(file) || /(^|\/)🚀️bootstrap\//u.test(file);
const specifierFor = (from: string, p: Pkg, e: Entry, before: string): string => {
  if (before.startsWith(".") && relativeKept(from)) { let r = rel(abs(e.file)); r = relative(dirname(abs(from)), abs(e.file)).split("\\").join("/"); return r.startsWith(".") ? r : "./" + r; }
  return p.name + (e.sub === "." ? "" : e.sub.slice(1));
};

type Edit = { file: string; before: string; after: string; start: number; end: number; pkg: string; sub: string; to: string; names: string[] | "*" };
type Skip = { from: string; to: string; specifier: string; reason: string; names: string[] | "*"; rules: string[]; glueGap?: { entry: string; pkg: string; missing: string[] } };
const edits: Edit[] = [];
const skipped: Skip[] = [];
const rows = readFileSync(edgesPath, "utf8").split("\n").filter(Boolean).map(l => { const [rule, from, to, spec, types] = l.split("\t"); return { rule: rule!, from: from!, to: to!, specifier: spec!, types: types! }; });
const groups = new Map<string, typeof rows>();
for (const r of rows) { const k = r.from + "\0" + r.specifier; (groups.get(k) ?? groups.set(k, []).get(k)!).push(r); }
const candidatePackages = (): Pkg[] => (scope === "sdk" ? [pkgs.get(SDK)!] : [...pkgs.values()]);
const glueWrites = new Map<string, { entry: string; to: string; values: Set<string>; types: Set<string> }>();

for (const group of groups.values()) {
  const e0 = group[0]!;
  const to = e0.to;
  const rules = [...new Set(group.map(g => g.rule))];
  const skip = (reason: string, names: string[] | "*" = "*", glueGap?: Skip["glueGap"]): void => { skipped.push({ from: e0.from, to, specifier: e0.specifier, reason, names, rules, glueGap }); };
  if (e0.specifier.includes("?")) { skip("query import"); continue; }
  if (!existsSync(abs(e0.from))) { skip("source missing"); continue; }
  let found: { uses: Use[]; loose: ts.StringLiteralLike[] };
  try { found = usesOf(e0.from, e0.specifier); } catch { skip("parse failure"); continue; }
  const { uses, loose } = found;
  if (!uses.length) { skip("no import node found"); continue; }
  const toOrigins = to.endsWith(".json") ? null : originsOf(to);
  const chosen: { u: Use; pkg: Pkg; entry: Entry }[] = [];
  let failure = ""; let gap: Skip["glueGap"] | undefined; let failNames: string[] | "*" = "*";
  for (const u of uses) {
    let best: { pkg: Pkg; entry: Entry; score: number; gap?: { missing: string[]; modules: Map<string, string[]> } } | null = null;
    for (const p of candidatePackages()) for (const en of entries(p)) {
      const kept = e0.specifier.startsWith(".") && relativeKept(e0.from);
      if (violationsOf(e0.from, en.file, kept ? ["local"] : ["aliased", "aliased-workspace", "local"]).length) continue;
      let score = 0;
      if (en.file === to) score = 1e9;
      else if (to.endsWith(".json")) { if ((u.names === "*" || (u.names.length === 1 && u.names[0] === "default")) && jsonGlueExposes(en.file, to)) score = 1e6; else continue; }
      else if (toOrigins) {
        const eo = originsOf(en.file);
        if (!eo) continue;
        const needed = u.names === "*" ? [...toOrigins.keys()].filter(k => !k.startsWith("*bare:")) : u.names;
        if (!needed.length) continue;
        const sources = exportSources(en.file);
        const direct = sources.has(to);
        const tail = tailMatch(en.file, to) * 1000;
        const ok = needed.every(n => { const a = toOrigins.get(n), b = eo.get(n); return a && b && a.file === b.file && a.name === b.name && !(b.via && !b.typeKind && !(a.via && !a.typeKind)); });
        if (ok) score = 1e6 + (direct ? 5e5 : 0) + tail - eo.size;
        else if (u.names !== "*" && needed.every(n => toOrigins.get(n))) {
          const missing = needed.filter(n => { const a = toOrigins.get(n)!, b = eo.get(n); return !b; });
          const conflicting = needed.filter(n => { const a = toOrigins.get(n)!, b = eo.get(n); return b && !(a.file === b.file && a.name === b.name); });
          if (conflicting.length || !missing.length || !sharesDomain(en.file, to)) continue;
          const modules = new Map<string, string[]>();
          let cover = true;
          for (const n of missing) { const o = toOrigins.get(n)!; let m: string | null = sources.has(to) ? to : sources.has(o.file) ? o.file : null; if (!m) for (const src of sources) { const so = originsOf(src)?.get(n); if (so && so.file === o.file && so.name === o.name) { m = src; break; } } if (!m) { cover = false; break; } (modules.get(m) ?? modules.set(m, []).get(m)!).push(n); }
          if (!cover) continue;
          score = 7e5 + (direct ? 5e5 : 0) + tail - eo.size;
          if (!best || score > best.score) { best = { pkg: p, entry: en, score, gap: { missing, modules } }; }
          continue;
        } else continue;
      } else continue;
      if (!best || score > best.score) best = { pkg: p, entry: en, score };
    }
    if (best?.gap) {
      for (const [m, names] of best.gap.modules) {
        const kk = best.entry.file + "\0" + m;
        const w = glueWrites.get(kk) ?? glueWrites.set(kk, { entry: best.entry.file, to: m, values: new Set(), types: new Set() }).get(kk)!;
        for (const n of names) { const o = toOrigins!.get(n)!; const exp = o.name === n ? n : `${o.name} as ${n}`; (o.typeKind || o.via ? w.types : w.values).add(exp); }
      }
      gap = { entry: best.entry.file, pkg: best.pkg.name, missing: best.gap.missing };
    }
    if (!best) { failure = u.names === "*" ? "namespace, side-effect or dynamic use without covering entry" : "no existing entry exposes " + u.names.join(","); failNames = u.names; break; }
    chosen.push({ u, pkg: best.pkg, entry: best.entry });
  }
  if (failure) { skip(failure, failNames); continue; }
  if (loose.length && new Set(chosen.map(c => specifierFor(e0.from, c.pkg, c.entry, e0.specifier))).size > 1) { skip("string uses of the specifier and several target entries"); continue; }
  const sf = parse(e0.from);
  for (const c of chosen) {
    const after = specifierFor(e0.from, c.pkg, c.entry, e0.specifier);
    edits.push({ file: e0.from, before: e0.specifier, after, start: c.u.node.getStart(sf) + 1, end: c.u.node.getEnd() - 1, pkg: c.pkg.name, sub: c.entry.sub, to, names: c.u.names });
  }
  for (const l of loose) edits.push({ file: e0.from, before: e0.specifier, after: specifierFor(e0.from, chosen[0]!.pkg, chosen[0]!.entry, e0.specifier), start: l.getStart(sf) + 1, end: l.getEnd() - 1, pkg: chosen[0]!.pkg.name, sub: chosen[0]!.entry.sub, to, names: "*" });
  void gap;
}

const gluesEdited: { entry: string; values: string[]; types: string[]; to: string }[] = [];
if (apply) {
  for (const w of glueWrites.values()) {
    const spec = relative(dirname(abs(w.entry)), abs(w.to)).split("\\").join("/");
    const specifier = spec.startsWith(".") ? spec : "./" + spec;
    const gsf = parse(w.entry);
    let text = readFileSync(abs(w.entry), "utf8");
    const patches: { start: number; end: number; text: string }[] = [];
    const appended: string[] = [];
    const merge = (names: string[], typeOnly: boolean): void => {
      if (!names.length) return;
      for (const st of gsf.statements) {
        if (!ts.isExportDeclaration(st) || !st.moduleSpecifier || !ts.isStringLiteral(st.moduleSpecifier) || st.moduleSpecifier.text !== specifier || st.isTypeOnly !== typeOnly || !st.exportClause || !ts.isNamedExports(st.exportClause) || st.attributes) continue;
        const existing = st.exportClause.elements.map(e => e.getText(gsf));
        const all = [...new Set([...existing, ...names])].sort((a, b) => (a < b ? -1 : a > b ? 1 : 0));
        patches.push({ start: st.getStart(gsf), end: st.getEnd(), text: `export ${typeOnly ? "type " : ""}{ ${all.join(", ")} } from "${specifier}";` });
        return;
      }
      appended.push(`export ${typeOnly ? "type " : ""}{ ${names.sort((a, b) => (a < b ? -1 : a > b ? 1 : 0)).join(", ")} } from "${specifier}";`);
    };
    merge([...w.values], false); merge([...w.types], true);
    for (const p of patches.sort((a, b) => b.start - a.start)) text = text.slice(0, p.start) + p.text + text.slice(p.end);
    if (appended.length) text = text.replace(/\n*$/u, "\n") + appended.join("\n") + "\n";
    writeFileSync(abs(w.entry), text);
    gluesEdited.push({ entry: w.entry, values: [...w.values], types: [...w.types], to: w.to });
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
writeFileSync(planPath, JSON.stringify({ edits, skipped, gluesEdited, glueWrites: [...glueWrites.values()].map(w => ({ ...w, values: [...w.values], types: [...w.types] })), applied: applied.length, stale: stale.length, unresolved: unresolved.map(u => ({ file: u.file, after: u.after })) }, null, 1));
console.log(JSON.stringify({ groups: groups.size, edits: edits.length, skipped: skipped.length, reasons, glueFiles: glueWrites.size, applied: applied.length, stale: stale.length, unresolved: unresolved.length, files: new Set(edits.map(e => e.file)).size }));
