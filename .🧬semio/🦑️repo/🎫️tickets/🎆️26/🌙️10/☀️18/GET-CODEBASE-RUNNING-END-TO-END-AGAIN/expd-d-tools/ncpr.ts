/**
 * expc-d: re-points `no-cross-package-relative-*` edges to explicit narrow package facades (creates facade + export key when no existing entry exposes the imported names).
 *
 *   bun ncpr.ts <edges.json> <scope-regex> [apply] [--only=<regex>]   (edges.json = {edges:[{rule,from,to}]})
 */
import ts from "typescript";
import { existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { GENERATED, ROOT, posix } from "../repoapi-tools/lib.ts";
import { packageIndex, type Pkg } from "../repoapi-tools/packages.ts";
import { DOMAINS, domainOf } from "../repoapi-tools/rules.ts";
import { LIB_REL } from "../repoapi-tools/lib.ts";

mkdirSync(GENERATED, { recursive: true });
const [edgesPath, scopeSource, mode, ...flags] = process.argv.slice(2) as string[];
const apply = mode === "apply";
const only = flags.find(f => f.startsWith("--only="))?.slice(7);
const scope = new RegExp(scopeSource!, "u");
const PREFIX = "no-cross-package-relative-";
const abs = (p: string): string => resolve(ROOT, p);
const rel = (p: string): string => posix(relative(ROOT, p));
const raw = JSON.parse(readFileSync(edgesPath!, "utf8"));
const edges = (Array.isArray(raw) ? raw : raw.edges).filter((e: any) => String(e.rule).startsWith(PREFIX) && scope.test(e.from) && (!only || new RegExp(only, "u").test(e.to)));
const index = packageIndex();
const SRC = [".ts", ".tsx", ".mts", ".js", ".mjs", ".cjs", ".json", "/🟦️.ts", "/🟦️.tsx", "/🟨️.mjs", "/🟨️.cjs"];
const isFile = (p: string): boolean => { try { return statSync(p).isFile(); } catch { return false; } };
const resolveRel = (fromFile: string, spec: string): string | null => {
  if (!spec.startsWith(".")) return null;
  const base = resolve(dirname(abs(fromFile)), spec.split("?")[0]!);
  for (const c of [base, ...SRC.map(s => base + s), base.replace(/\.js$/u, ".ts"), base.replace(/\.js$/u, ".tsx")]) if (isFile(c)) return rel(c);
  return null;
};
const parse = (file: string, text = readFileSync(abs(file), "utf8")): ts.SourceFile => ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const has = (n: ts.Node, k: ts.SyntaxKind): boolean => !!ts.getModifiers(n as ts.HasModifiers)?.some(m => m.kind === k);

type Origin = { file: string; name: string; typeKind: boolean; via: boolean };
const originCache = new Map<string, Map<string, Origin> | null>();
function originsOf(file: string, depth = 0): Map<string, Origin> | null {
  if (originCache.has(file)) return originCache.get(file)!;
  originCache.set(file, null);
  if (depth > 8 || !/\.(ts|tsx|mts|js|mjs|cjs)$/u.test(file)) return null;
  let sf: ts.SourceFile;
  try { sf = parse(file); } catch { return null; }
  const out = new Map<string, Origin>();
  const self = (name: string, typeKind = false): void => { if (!(typeKind && out.has(name) && !out.get(name)!.typeKind)) out.set(name, { file, name, typeKind, via: false }); };
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

type Use = { names: string[] | "*"; node: ts.StringLiteralLike; specifier: string; typeOnly: boolean };
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
function allUses(file: string): Use[] {
  const sf = parse(file);
  const uses: Use[] = [];
  const visit = (n: ts.Node): void => {
    if (ts.isImportDeclaration(n) && ts.isStringLiteral(n.moduleSpecifier)) {
      const c = n.importClause; const names: string[] = []; let star: string[] | "*" | null = null;
      if (!c) star = "*";
      else { if (c.name) names.push("default"); if (c.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) star = namespaceMembers(sf, c.namedBindings.name.text); else for (const e of c.namedBindings.elements) names.push((e.propertyName ?? e.name).text); } }
      uses.push({ names: star === "*" ? "*" : [...names, ...(star ?? [])], node: n.moduleSpecifier, specifier: n.moduleSpecifier.text, typeOnly: !!c?.isTypeOnly });
    } else if (ts.isExportDeclaration(n) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) {
      const names: string[] = []; let star = false;
      if (!n.exportClause || ts.isNamespaceExport(n.exportClause)) star = true; else for (const e of n.exportClause.elements) names.push((e.propertyName ?? e.name).text);
      uses.push({ names: star ? "*" : names, node: n.moduleSpecifier, specifier: n.moduleSpecifier.text, typeOnly: n.isTypeOnly });
    } else if (ts.isCallExpression(n) && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0]) && (n.expression.kind === ts.SyntaxKind.ImportKeyword || (ts.isIdentifier(n.expression) && n.expression.text === "require"))) {
      let p: ts.Node = n.parent;
      while (ts.isParenthesizedExpression(p) || ts.isAwaitExpression(p) || ts.isAsExpression(p)) p = p.parent;
      let names: string[] | "*" = "*";
      if (ts.isVariableDeclaration(p) && ts.isObjectBindingPattern(p.name) && !p.name.elements.some(e => e.dotDotDotToken)) names = p.name.elements.map(e => (e.propertyName && ts.isIdentifier(e.propertyName) ? e.propertyName : ts.isIdentifier(e.name) ? e.name : null)?.text ?? "*");
      if (Array.isArray(names) && names.includes("*")) names = "*";
      uses.push({ names, node: n.arguments[0], specifier: n.arguments[0].text, typeOnly: false });
    } else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal)) {
      const q = n.qualifier; const first = q ? (function f(x: ts.EntityName): string { return ts.isIdentifier(x) ? x.text : f(x.left); })(q) : "*";
      uses.push({ names: first === "*" ? "*" : [first], node: n.argument.literal, specifier: n.argument.literal.text, typeOnly: true });
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  return uses;
}

const entryFile = (value: unknown): string | null => {
  if (typeof value === "string") return value;
  if (Array.isArray(value)) { for (const v of value) { const r = entryFile(v); if (r) return r; } return null; }
  if (value && typeof value === "object") {
    const o = value as Record<string, unknown>;
    for (const k of ["semio-source", "import", "default", "types", "node"]) if (k in o) { const r = entryFile(o[k]); if (r) return r; }
    for (const v of Object.values(o)) { const r = entryFile(v); if (r) return r; }
  }
  return null;
};
const entriesCache = new Map<string, { sub: string; file: string }[]>();
function entriesOf(pkg: Pkg): { sub: string; file: string }[] {
  const hit = entriesCache.get(pkg.packageJson);
  if (hit) return hit;
  const current = JSON.parse(readFileSync(abs(pkg.packageJson), "utf8")).exports ?? {};
  const out: { sub: string; file: string }[] = [];
  for (const [sub, value] of Object.entries(current)) {
    if (!sub.startsWith(".") || sub.includes("*")) continue;
    const t = entryFile(value);
    if (t && t.startsWith("./")) out.push({ sub, file: rel(resolve(abs(pkg.dir), t)) });
  }
  entriesCache.set(pkg.packageJson, out);
  return out;
}
const jsonGlueExposes = (entry: string, json: string): boolean => {
  if (!/\.(ts|tsx|mts)$/u.test(entry) || !isFile(abs(entry))) return false;
  const sf = parse(entry); let reaches = false, exports = false;
  for (const st of sf.statements) {
    if ((ts.isImportDeclaration(st) || ts.isExportDeclaration(st)) && st.moduleSpecifier && ts.isStringLiteral(st.moduleSpecifier) && resolveRel(entry, st.moduleSpecifier.text) === json) reaches = true;
    if (ts.isExportAssignment(st)) exports = true;
    if (ts.isExportDeclaration(st) && st.exportClause && ts.isNamedExports(st.exportClause) && st.exportClause.elements.some(e => e.name.text === "default")) exports = true;
  }
  return reaches && exports;
};

const glueModules = (entry: string): number => {
  if (!isFile(abs(entry))) return 0;
  const modules = new Set<string>();
  for (const st of parse(entry).statements) { if (!ts.isExportDeclaration(st) || !st.moduleSpecifier || !ts.isStringLiteral(st.moduleSpecifier)) return 0; const r = resolveRel(entry, st.moduleSpecifier.text); if (r) modules.add(r); }
  return modules.size;
};
const asciiSeg = (segment: string): string => segment.replace(/\.[a-z]+$/u, "").replace(/[^\x00-\x7f]/gu, "");
const BOM = new Set<string>();
type Plan = { pkg: Pkg; sub: string; glue: string; target: string; values: Set<string>; types: Set<string>; json: boolean; reuse: boolean; extend?: boolean; keyOnly?: boolean };
const plans = new Map<string, Plan>();
const skipped: { from: string; to: string; specifier: string; reason: string }[] = [];
type Edit = { file: string; start: number; end: number; before: string; to: string; planKey: string; text: string };
const edits: Edit[] = [];

function packageForTarget(target: string, ruleNames: string[]): Pkg | null {
  const candidates = ruleNames.map(n => index.byName.get(n)).filter((p): p is Pkg => !!p);
  if (!candidates.length) return null;
  const owner = candidates[0]!.sourceOwner;
  const siblings = index.all.filter(p => p.sourceOwner === owner && p.dir.includes("🟦️typescript"));
  return siblings[0] ?? candidates[0]!;
}
function glueRelOf(pkg: Pkg, target: string): string | null {
  if (!target.startsWith(pkg.sourceOwner + "/")) return null;
  let r = target.slice(pkg.sourceOwner.length + 1);
  if (pkg.sourceOwner === "🧰️framework") r = r.replace(/^🔨️modules\//u, "");
  return r;
}
function planFor(pkg: Pkg, target: string, glueRel: string, json: boolean): { sub: string; glue: string } | null {
  const dirSegs = glueRel.split("/").slice(0, -1);
  const fileName = glueRel.split("/").pop()!;
  const stem = /^🟦️\.(ts|tsx|mjs|cjs)$/u.test(fileName) ? null : asciiSeg(fileName);
  const keySegs = [...dirSegs.map(asciiSeg), ...(stem ? [stem] : [])].filter(Boolean);
  const keyBase = "./" + (keySegs.join("/") || "root");
  const sub = json ? keyBase + ".json" : keyBase;
  const glueDir = [...dirSegs, ...(stem ? [fileName.replace(/\.[^.]+$/u, "")] : [])];
  const glue = posix([pkg.dir, ...glueDir, json ? "🔣️json/🟦️.ts" : /\.mjs$/u.test(fileName) ? "🟨️.mjs" : /\.cjs$/u.test(fileName) ? "🟨️.cjs" : "🟦️.ts"].join("/"));
  return { sub, glue };
}

const byFrom = new Map<string, any[]>();
for (const e of edges) (byFrom.get(e.from) ?? byFrom.set(e.from, []).get(e.from)!).push(e);

for (const [from, group] of byFrom) {
  if (!existsSync(abs(from))) continue;
  const targets = new Map<string, string[]>();
  for (const e of group) (targets.get(e.to) ?? targets.set(e.to, []).get(e.to)!).push(String(e.rule).slice(PREFIX.length));
  const uses = allUses(from).map(u => ({ u, resolved: resolveRel(from, u.specifier) })).filter(x => x.resolved && targets.has(x.resolved));
  const seenSpec = new Set(uses.map(x => x.resolved));
  for (const t of targets.keys()) if (!seenSpec.has(t)) skipped.push({ from, to: t, specifier: "?", reason: "no resolvable import node (query, alias or generated)" });
  for (const { u, resolved } of uses) {
    const target = resolved!;
    if (u.specifier.includes("?")) { skipped.push({ from, to: target, specifier: u.specifier, reason: "query import" }); continue; }
    const pkg = packageForTarget(target, targets.get(target)!);
    if (!pkg) { skipped.push({ from, to: target, specifier: u.specifier, reason: "target package unknown" }); continue; }
    const owners = index.all.filter(p => p.sourceOwner === pkg.sourceOwner);
    const json = target.endsWith(".json");
    const toOrigins = json ? null : originsOf(target);
    const needed = json ? ["default"] : u.names === "*" ? [...(toOrigins?.keys() ?? [])].filter(k => !k.startsWith("*bare:") && k !== "default") : u.names;
    let best: { pkg: Pkg; entry: { sub: string; file: string }; score: number } | null = null;
    for (const p of owners) for (const en of entriesOf(p)) {
      if (en.file !== target && /\.(ts|tsx|mts)$/u.test(en.file) && glueModules(en.file) > 4) continue;
      let score = 0;
      if (en.file === target) score = 1e9;
      else if (json) { if (jsonGlueExposes(en.file, target)) score = 1e6; else continue; }
      else if (toOrigins && needed.length) {
        const eo = originsOf(en.file);
        if (!eo) continue;
        const ok = needed.every(n => { const a = toOrigins.get(n), b = eo.get(n); return a && b && a.file === b.file && a.name === b.name && !(b.via && !b.typeKind && !(a.via && !a.typeKind)); });
        if (!ok) continue;
        score = 1e6 - eo.size;
      } else continue;
      if (!best || score > best.score) best = { pkg: p, entry: en, score };
    }
    let planKey: string;
    if (best) {
      planKey = best.pkg.name + "\0" + best.entry.sub;
      if (!plans.has(planKey)) plans.set(planKey, { pkg: best.pkg, sub: best.entry.sub, glue: best.entry.file, target, values: new Set(), types: new Set(), json, reuse: true });
    } else {
      if (target.startsWith(pkg.dir + "/")) {
        const under = target.slice(pkg.dir.length + 1).split("/");
        const underFile = under[under.length - 1]!;
        const underStem = json || !/^🟦️\.(ts|tsx|mjs|cjs)$/u.test(underFile) ? asciiSeg(underFile) : "";
        const keySegs = [...under.slice(0, -1).map(asciiSeg), underStem].filter(Boolean);
        const sub = keySegs.length ? "./" + keySegs.join("/") + (json ? ".json" : "") : ".";
        planKey = pkg.name + "\0" + sub;
        const currentExports = JSON.parse(readFileSync(abs(pkg.packageJson), "utf8")).exports ?? {};
        if (sub in currentExports && !plans.has(planKey)) { skipped.push({ from, to: target, specifier: u.specifier, reason: "facade key exists elsewhere " + sub }); continue; }
        if (!plans.has(planKey)) plans.set(planKey, { pkg, sub, glue: target, target, values: new Set(), types: new Set(), json, reuse: false, keyOnly: true });
        edits.push({ file: from, start: -1, end: -1, before: u.specifier, to: target, planKey, text: "" });
        continue;
      }
      const glueRel = glueRelOf(pkg, target);
      if (!glueRel) { skipped.push({ from, to: target, specifier: u.specifier, reason: "target outside the package source owner " + pkg.sourceOwner }); continue; }
      let planned = planFor(pkg, target, glueRel, json);
      if (!planned) { skipped.push({ from, to: target, specifier: u.specifier, reason: "no planned key" }); continue; }
      let narrow: { sub: string; file: string } | null = null;
      if (!json) for (const p of owners) for (const en of entriesOf(p)) {
        if (en.file === target || !/\.(ts|tsx|mts)$/u.test(en.file) || glueModules(en.file) !== 1) continue;
        const go = originsOf(en.file);
        if (go && [...go.values()].some(o => o.file === target)) narrow = { sub: en.sub, file: en.file };
      }
      if (narrow) planned = { sub: narrow.sub, glue: narrow.file };
      planKey = pkg.name + "\0" + planned.sub;
      const existing = plans.get(planKey);
      if (existing && existing.target !== target) { skipped.push({ from, to: target, specifier: u.specifier, reason: "key collision " + planned.sub }); continue; }
      const currentExports = JSON.parse(readFileSync(abs(pkg.packageJson), "utf8")).exports ?? {};
      let extendGlue: string | null = narrow ? narrow.file : null;
      if (!existing && !narrow && planned.sub in currentExports) {
        const value = entryFile(currentExports[planned.sub]);
        const glueExisting = value ? rel(resolve(abs(pkg.dir), value)) : null;
        const go = glueExisting && !json ? originsOf(glueExisting) : null;
        if (go && [...go.values()].some(o => o.file === target) && glueModules(glueExisting!) <= 4) extendGlue = glueExisting;
        else { skipped.push({ from, to: target, specifier: u.specifier, reason: "key exists with other target " + planned.sub }); continue; }
      }
      if (!plans.has(planKey)) plans.set(planKey, { pkg, sub: planned.sub, glue: extendGlue ?? planned.glue, target, values: new Set(), types: new Set(), json, reuse: false, extend: !!extendGlue });
      const plan = plans.get(planKey)!;
      if (plan.reuse) { plan.reuse = false; plan.extend = true; }
      if (!json && !toOrigins) { skipped.push({ from, to: target, specifier: u.specifier, reason: "target unreadable" }); continue; }
      let bad = false;
      for (const n of needed) {
        const o = toOrigins?.get(n);
        if (!json && !o) { skipped.push({ from, to: target, specifier: u.specifier, reason: "name not exported by target: " + n }); bad = true; continue; }
        if (!json) (o!.typeKind || o!.via ? plan.types : plan.values).add(n);
      }
      if (bad) continue;
      if (!json && !needed.length) { skipped.push({ from, to: target, specifier: u.specifier, reason: "no names" }); continue; }
    }
    const plan = plans.get(planKey)!;
    const glueAbs = abs(plan.glue);
    let r = posix(relative(dirname(abs(from)), glueAbs));
    if (!r.startsWith(".")) r = "./" + r;
    const sf = parse(from);
    edits.push({ file: from, start: u.node.getStart(sf) + 1, end: u.node.getEnd() - 1, before: u.specifier, to: target, planKey, text: r });
    if (json) {
      const parent = u.node.parent;
      const attributes = (ts.isImportDeclaration(parent) || ts.isExportDeclaration(parent)) ? parent.attributes : ts.isCallExpression(parent) && parent.arguments.length === 2 && parent.arguments[0] === u.node ? parent.arguments[1] : undefined;
      if (attributes) { const s = u.node.getEnd(), e = ts.isCallExpression(parent) ? (parent.arguments[1] as ts.Node).getEnd() : attributes.getEnd(); const cut = ts.isCallExpression(parent) ? sf.text.slice(s, e) : sf.text.slice(s, e); edits.push({ file: from, start: s, end: e, before: cut, to: target, planKey, text: "" }); }
    }
  }
}

const reportPlans = [...plans.values()].map(p => ({ extend: !!p.extend, keyOnly: !!p.keyOnly, pkg: p.pkg.name, sub: p.sub, glue: p.glue, target: p.target, reuse: p.reuse, values: [...p.values].sort(), types: [...p.types].sort() }));
const created: string[] = [];
if (apply) {
  const touched = new Map<string, Record<string, string>>();
  for (const p of plans.values()) {
    if (p.reuse) {
      if (!p.json && (p.values.size || p.types.size)) throw Error("reuse with names");
      continue;
    }
    const glueAbs = abs(p.glue);
    if (p.keyOnly) {
      const pj = abs(p.pkg.packageJson), doc = JSON.parse(readFileSync(pj, "utf8")), text = readFileSync(pj, "utf8");
      const roundTripped = JSON.stringify(doc, null, 2) + "\n" === text;
      doc.exports ??= {};
      if (!(p.sub in doc.exports)) {
        if (!roundTripped) throw Error("manifest does not round-trip " + p.pkg.packageJson);
        doc.exports[p.sub] = "./" + posix(relative(abs(p.pkg.dir), glueAbs));
        writeFileSync(pj, JSON.stringify(doc, null, 2) + "\n");
      }
      continue;
    }
    if (p.extend) {
      const have = originsOf(p.glue)!;
      const specRaw = posix(relative(dirname(glueAbs), abs(p.target)));
      const specifier = specRaw.startsWith(".") ? specRaw : "./" + specRaw;
      const missing = (set: Set<string>) => [...set].filter(n => { const o = have.get(n), t = originsOf(p.target)!.get(n)!; return !(o && o.file === t.file && o.name === t.name); }).sort();
      const mv = missing(p.values), mt = missing(p.types);
      let text = readFileSync(glueAbs, "utf8");
      const merge = (names: string[], typeOnly: boolean): void => {
        if (!names.length) return;
        const gsf = parse(p.glue, text);
        for (const st of gsf.statements) {
          if (!ts.isExportDeclaration(st) || !st.moduleSpecifier || !ts.isStringLiteral(st.moduleSpecifier) || st.moduleSpecifier.text !== specifier || st.isTypeOnly !== typeOnly || !st.exportClause || !ts.isNamedExports(st.exportClause) || st.attributes) continue;
          const all = [...new Set([...st.exportClause.elements.map(e => e.getText(gsf)), ...names])].sort();
          text = text.slice(0, st.getStart(gsf)) + `export ${typeOnly ? "type " : ""}{ ${all.join(", ")} } from "${specifier}";` + text.slice(st.getEnd());
          return;
        }
        text = text.replace(/\n*$/u, "\n") + `export ${typeOnly ? "type " : ""}{ ${names.join(", ")} } from "${specifier}";\n`;
      };
      merge(mv, false);
      if (mt.length) { const g2 = parse(p.glue, text); void g2; merge(mt, true); }
      if (text !== readFileSync(glueAbs, "utf8")) writeFileSync(glueAbs, text);
      continue;
    }
    const specRaw = posix(relative(dirname(glueAbs), abs(p.target)));
    const specifier = specRaw.startsWith(".") ? specRaw : "./" + specRaw;
    const key = p.sub.slice(1);
    let body: string;
    if (p.json) body = `/** 🧩️ Public \`${p.pkg.name}${key}\` entry: the JSON document, re-exported unchanged. */\nimport document from "${specifier}" with { type: "json" };\nexport default document;\n`;
    else {
      const lines = [`/** 🧩️ Public \`${p.pkg.name}${key}\` entry: explicit re-exports of the module that owns each member. */`];
      const values = [...p.values].sort(), types = [...p.types].sort();
      if (values.length) lines.push(`export { ${values.join(", ")} } from "${specifier}";`);
      if (types.length) lines.push(`export type { ${types.join(", ")} } from "${specifier}";`);
      body = lines.join("\n") + "\n";
    }
    if (p.glue.length > 250) throw Error("path too long " + p.glue);
    if (existsSync(glueAbs)) {
      const have = readFileSync(glueAbs, "utf8");
      if (have !== body) throw Error("glue exists with other content " + p.glue);
    } else { mkdirSync(dirname(glueAbs), { recursive: true }); writeFileSync(glueAbs, body); created.push(p.glue); if (/\.mjs$/u.test(p.glue)) { const declaration = glueAbs.replace(/\.mjs$/u, ".d.mts"); if (!existsSync(declaration)) { writeFileSync(declaration, body); created.push(rel(declaration)); } } }
    const pj = abs(p.pkg.packageJson);
    const doc = JSON.parse(readFileSync(pj, "utf8"));
    doc.exports ??= {};
    if (!(p.sub in doc.exports)) {
      const text = readFileSync(pj, "utf8");
      const roundTrip = JSON.stringify(doc, null, 2) + "\n";
      doc.exports[p.sub] = "./" + posix(relative(abs(p.pkg.dir), glueAbs));
      if (roundTrip !== text) throw Error("manifest does not round-trip " + p.pkg.packageJson);
      writeFileSync(pj, JSON.stringify(doc, null, 2) + "\n");
    }
    touched.set(pj, doc.exports);
  }
}
const applied: Edit[] = [], stale: Edit[] = [];
if (apply) {
  const byFile = new Map<string, Edit[]>();
  for (const ed of edits) (byFile.get(ed.file) ?? byFile.set(ed.file, []).get(ed.file)!).push(ed);
  for (const [file, list] of byFile) {
    let text = readFileSync(abs(file), "utf8");
    const ok = list.filter(ed => ed.start >= 0 && text.slice(ed.start, ed.end) === ed.before);
    stale.push(...list.filter(ed => !ok.includes(ed)));
    for (const ed of ok.sort((a, b) => b.start - a.start)) { text = text.slice(0, ed.start) + ed.text + text.slice(ed.end); applied.push(ed); }
    if (text !== readFileSync(abs(file), "utf8")) writeFileSync(abs(file), text);
  }
}
const reasons: Record<string, number> = {};
for (const s of skipped) { const k = s.reason.replace(/:.*/u, ""); reasons[k] = (reasons[k] ?? 0) + 1; }
writeFileSync(edgesPath!.replace(/\.[a-z]+$/u, "") + `.plan-${apply ? "applied" : "dry"}.json`, JSON.stringify({ plans: reportPlans, skipped, edits: edits.map(e => ({ file: e.file, before: e.before, after: e.text })), created, applied: applied.length, stale: stale.length }, null, 1));
console.log(JSON.stringify({ edges: edges.length, files: byFrom.size, plans: plans.size, newKeys: reportPlans.filter(p => !p.reuse).length, reused: reportPlans.filter(p => p.reuse).length, edits: edits.length, skipped: skipped.length, reasons, applied: applied.length, stale: stale.length, created: created.length }));
