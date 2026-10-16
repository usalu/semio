/** 🗺️ Derives the public facades, direct exports and target map for the flagged cross-package edges of the selected owners. Read-only: writes only the plan JSON.
 * Usage: bun plan.ts <policy.json> <scan.json> <plan.json> --owner <sourceOwner>... [--exclude-target <prefix>]... [--ts-package <sourceOwner>=<dir>:<name>]...
 * An owner without a TypeScript package is planned only when `--ts-package` names the package to create.
 */
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { ROOT } from "../sdkb-tools/lib.ts";
import { loadPolicy, groupsOf, segName, isAsset, nfc, type Edge, type Group, type PolicyPkg } from "./model.ts";
import { edgeSkip } from "./skip.ts";

const args = process.argv.slice(2);
const [policyPath, scanPath, planPath] = args;
const multi = (n: string) => args.flatMap((a, i) => (a === n ? [args[i + 1]] : []));
const owners = new Set(multi("--owner").map(nfc));
const excludeTarget = multi("--exclude-target").map(nfc);
const newPkgs = new Map(multi("--ts-package").map(s => { const [o, rest] = s.split("="); const [dir, name] = rest.split(":"); return [nfc(o), { dir: nfc(dir), name }]; }));
const policy = loadPolicy(policyPath);
const groups = new Map(groupsOf(policy).map(g => [g.sourceOwner, g]));
const problems: string[] = [];
const edges: Edge[] = (JSON.parse(readFileSync(scanPath, "utf8")) as Edge[]).filter(e => owners.has(e.sourceOwner) && !edgeSkip(e) && !excludeTarget.some(p => e.resolved.startsWith(p)));

interface PkgRef { name: string; dir: string; created: boolean }
const exporterOf = (g: Group): PkgRef | null => {
  if (g.tsPackage) return { name: g.tsPackage.name, dir: g.tsPackage.owner, created: false };
  const n = newPkgs.get(g.sourceOwner);
  return n ? { name: n.name, dir: n.dir, created: true } : null;
};

const exportKey = (dirRel: string, base: string): string => {
  const segs = dirRel.split("/").filter(Boolean).map(segName);
  const b = base === "🟦️.ts" || base === "🟦️.tsx" ? null : segName(base.replace(/\.[^.]+$/u, ""));
  const all = [...segs, ...(b ? [b] : [])].filter(Boolean);
  return all.length ? "./" + all.join("/") : ".";
};

const mirrorRel = (g: Group, target: string): string => {
  let rel = target.slice(g.sourceOwner.length + 1);
  if (g.sourceOwner === "🧰️framework") rel = rel.replace(/^🔨️modules\//u, "");
  rel = rel.replace(/\/?📦️packages\/🟦️typescript(?=\/|$)/gu, "").replace(/^\//u, "");
  return rel;
};

interface Use { g: Group; names: Set<string>; all: boolean; jsonDefault: boolean; files: Set<string>; flags: string[]; sideEffect: boolean; hasDefault?: boolean }
const uses = new Map<string, Use>();
for (const e of edges) {
  const g = groups.get(e.sourceOwner)!;
  const u = uses.get(e.resolved) ?? { g, names: new Set(), all: false, jsonDefault: false, files: new Set(), flags: [], sideEffect: false };
  uses.set(e.resolved, u);
  u.files.add(e.file);
  for (const n of e.named) u.names.add(n);
  if (e.members) for (const m of e.members) u.names.add(m);
  if (isAsset(e.resolved)) {
    if (e.defaultName) u.jsonDefault = true;
    if (e.kind === "export" && e.named.includes("default")) u.jsonDefault = true;
    if (e.kind === "dynamic") { u.jsonDefault = true; if (!(e.members?.length === 1 && e.members[0] === "default")) u.flags.push(`dynamic-json-shape ${e.file}:${e.line}`); }
    if (e.namespaceName) u.flags.push(`namespace-asset ${e.file}:${e.line}`);
  } else {
    if (e.defaultName) u.hasDefault = true;
    if (e.exportStar || e.membersUnknown) u.all = true;
    if (e.sideEffect) { u.sideEffect = true; }
  }
}

const options: ts.CompilerOptions = {
  target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler,
  allowImportingTsExtensions: true, resolveJsonModule: true, customConditions: ["semio-source"], jsx: ts.JsxEmit.ReactJSX,
  skipLibCheck: true, noEmit: true, types: [], esModuleInterop: true, isolatedModules: true,
};

const existingExportFiles = new Map<string, string[]>();
const exportedFilesOf = (g: Group): { pkg: PolicyPkg; subpath: string; file: string }[] => {
  const out: { pkg: PolicyPkg; subpath: string; file: string }[] = [];
  for (const p of g.packages) for (const r of p.exportTargets) if (r.target && /\.(ts|tsx)$/u.test(r.target)) { const f = nfc(p.owner + "/" + r.target.slice(2)); if (existsSync(join(ROOT, f))) out.push({ pkg: p, subpath: r.subpath, file: f }); }
  return out;
};
const allExisting = [...new Set([...groups.values()].filter(g => owners.has(g.sourceOwner)).flatMap(g => exportedFilesOf(g).map(x => x.file)))];
const tsTargets = [...uses.keys()].filter(t => !isAsset(t) && existsSync(join(ROOT, t)));
const program = ts.createProgram([...tsTargets, ...allExisting].map(p => join(ROOT, p)), options);
const checker = program.getTypeChecker();

function moduleExports(rel: string): Map<string, ts.Symbol> {
  const sf = program.getSourceFile(join(ROOT, rel));
  const out = new Map<string, ts.Symbol>();
  const sym = sf && checker.getSymbolAtLocation(sf);
  if (!sym) return out;
  for (const e of checker.getExportsOfModule(sym)) out.set(e.name, e);
  return out;
}
function isValue(s: ts.Symbol): boolean {
  let cur: ts.Symbol | undefined = s;
  while (cur && cur.flags & ts.SymbolFlags.Alias) {
    const d = cur.declarations?.[0];
    if (d && ts.isExportSpecifier(d) && (d.isTypeOnly || d.parent.parent.isTypeOnly)) return false;
    if (d && ts.isImportSpecifier(d) && (d.isTypeOnly || d.parent.parent.isTypeOnly)) return false;
    cur = checker.getImmediateAliasedSymbol(cur);
  }
  return !!cur && (cur.flags & ts.SymbolFlags.Value) !== 0;
}

function ultimate(sym: ts.Symbol): ts.Symbol { try { return sym.flags & ts.SymbolFlags.Alias ? checker.getAliasedSymbol(sym) : sym; } catch { return sym; } }
function sameSymbol(facade: string, target: string, name: string): boolean {
  const a = moduleExports(facade).get(name), b = moduleExports(target).get(name);
  return !!a && !!b && ultimate(a) === ultimate(b);
}

interface Covering { facade: string; subpath: string; pkg: PolicyPkg; names: Set<string> | "all"; modules: number }
function coveringFacades(g: Group): Map<string, Covering[]> {
  const out = new Map<string, Covering[]>();
  for (const x of exportedFilesOf(g)) {
    const text = readFileSync(join(ROOT, x.file), "utf8");
    const sf = ts.createSourceFile(x.file, text, ts.ScriptTarget.ESNext, true);
    const rows: { hit: string; names: Set<string> | "all" }[] = [];
    for (const st of sf.statements) {
      if (!ts.isExportDeclaration(st) || !st.moduleSpecifier || !ts.isStringLiteral(st.moduleSpecifier)) continue;
      const spec = st.moduleSpecifier.text;
      if (!spec.startsWith(".")) continue;
      const base = resolve(join(ROOT, dirname(x.file)), spec);
      let hit: string | null = null;
      for (const c of [base, base + ".ts", base + ".tsx", base + "/🟦️.ts", base + "/🟦️.tsx"]) if (existsSync(c)) { hit = nfc(c.slice(ROOT.length + 1)); break; }
      if (!hit) continue;
      const names: Set<string> | "all" = !st.exportClause ? "all" : ts.isNamedExports(st.exportClause) ? new Set(st.exportClause.elements.map(el => el.name.text)) : new Set();
      rows.push({ hit, names });
    }
    if (x.subpath === ".") continue;
    const modules = new Set(rows.map(r => r.hit)).size;
    for (const r of rows) { const l = out.get(r.hit) ?? []; l.push({ facade: x.file, subpath: x.subpath, pkg: x.pkg, names: r.names, modules }); out.set(r.hit, l); }
  }
  for (const l of out.values()) l.sort((a, b) => a.modules - b.modules);
  return out;
}

type Add = { from: string; names: { name: string; value: boolean }[]; json: { exportName: string; file: string }[]; sideEffect?: boolean };
interface FacadePlan { pkg: string; pkgDir: string; facade: string; subpath: string; created: boolean; adds: Map<string, Add> }
const facades = new Map<string, FacadePlan>();
const directExports: { pkg: string; pkgDir: string; subpath: string; target: string }[] = [];
const targetMap: Record<string, { pkg: string; pkgDir: string; subpath: string; facade: string | null; direct: boolean; jsonExport?: string; hasDefault?: boolean }> = {};
const covering = new Map<string, Map<string, Covering[]>>();

const camel = (s: string) => s.replace(/-(\w)/gu, (_, c) => c.toUpperCase());
const TESTS_DIR = /(^|\/)\u{1F9EA}\uFE0F?tests(?=\/|$)/gu;
function jsonExportName(g: Group, target: string): string {
  const rel = target.slice(g.sourceOwner.length + 1);
  const isFixture = /(^|\/)\u{1F9EA}\uFE0F?tests?(\/|$)/u.test(rel);
  const dir = dirname(rel).split("/").map(segName).filter(s => s && !["schema", "fixtures", "tests", "testing", "modules", "packages", "typescript", "."].includes(s));
  const base = target.split("/").pop()!.replace(/\.[^.]+$/u, "");
  const baseName = segName(base);
  let parts = [...dir.slice(-2), ...(baseName ? [baseName] : [])];
  while (parts.length > 1 && /^\d/u.test(parts[0])) parts = parts.slice(1);
  if (!parts.length) parts = [segName(g.sourceOwner.split("/").pop()!)];
  const n = camel(parts.join("-"));
  return n.charAt(0).toLowerCase() + n.slice(1) + (isFixture ? "Fixture" : "Schema");
}

function isPureFacade(rel: string): boolean {
  try {
    const sf = ts.createSourceFile(rel, readFileSync(join(ROOT, rel), "utf8"), ts.ScriptTarget.ESNext, true);
    return sf.statements.length > 0 && sf.statements.every(st => (ts.isExportDeclaration(st) && !!st.moduleSpecifier) || (ts.isImportDeclaration(st) && !st.importClause));
  } catch { return false; }
}
const existingBySubpathEarly = new Map<string, string>();
for (const g of groups.values()) for (const p of g.packages) for (const r of p.exportTargets) if (r.target) existingBySubpathEarly.set(p.name + " " + r.subpath, nfc(p.owner + "/" + r.target.slice(2)));
const sortedUses = [...uses].sort((a, b) => (a[0] < b[0] ? -1 : 1));
for (const [t, u] of sortedUses) {
  for (const f of u.flags) problems.push(`${t}: ${f}`);
  const g = u.g;
  const ex = exporterOf(g);
  if (!ex) { problems.push(`${t}: owner ${g.sourceOwner} has no TypeScript package`); continue; }
  const asset = isAsset(t);
  const base = t.split("/").pop()!;
  if (asset && !t.endsWith(".json")) { problems.push(`${t}: unsupported asset or JavaScript target (${t.split(".").pop()})`); continue; }
  if (asset && !u.jsonDefault) { problems.push(`${t}: json target without a default import`); continue; }
  const names = new Map<string, boolean>();
  if (!asset) {
    if (!existsSync(join(ROOT, t))) { problems.push(`${t}: target file missing`); continue; }
    const own = moduleExports(t);
    const want = u.all ? [...own.keys()].filter(n => n !== "default") : [...u.names];
    if (u.hasDefault) want.push("default");
    for (const n of want) {
      const s = own.get(n);
      if (!s) { problems.push(`${t}: name ${n} not exported by target`); continue; }
      names.set(n, isValue(s));
    }
  }
  const rel = mirrorRel(g, t);
  const dirRel = dirname(rel) === "." ? "" : dirname(rel).replace(TESTS_DIR, "$1🧪️testing");
  let facadeName = "🟦️.ts";
  let mirrorProblem: string | null = null;
  if (asset) facadeName = "🟦️.ts";
  else if (base === "🟦️.ts" || base === "🟦️.tsx") facadeName = "🟦️.ts";
  else if (base === "📜️script.ts") facadeName = "📜️script.ts";
  else mirrorProblem = `${t}: unsupported entry name ${base}`;
  const taken = (sp: string, fac: string) => { const prev = existingBySubpathEarly.get(ex.name + " " + sp); return !!prev && prev !== fac; };
  let segs = dirRel ? dirRel.split("/") : [];
  let facade = nfc(ex.dir + "/" + (segs.length ? segs.join("/") + "/" : "") + facadeName);
  let subpath = exportKey(segs.join("/"), facadeName);
  if (taken(subpath, facade)) { segs = ["📥️source", ...segs]; facade = nfc(ex.dir + "/" + segs.join("/") + "/" + facadeName); subpath = exportKey(segs.join("/"), facadeName); }
  while (Buffer.byteLength(facade) > 236 && segs.length > 1) { segs = segs.slice(1); facade = nfc(ex.dir + "/" + segs.join("/") + "/" + facadeName); subpath = exportKey(segs.join("/"), facadeName); }
  if (!mirrorProblem && Buffer.byteLength(facade) > 240) mirrorProblem = `${t}: facade path too long ${Buffer.byteLength(facade)}`;
  if (!covering.has(g.sourceOwner)) covering.set(g.sourceOwner, coveringFacades(g));
  const cov = !asset ? covering.get(g.sourceOwner)!.get(t)?.filter(c => c.modules === 1 || c.facade === facade) : undefined;
  const covers = (c: Covering, n: string) => c.names === "all" || (c.names as Set<string>).has(n) || sameSymbol(c.facade, t, n);
  const fully = cov?.find(c => [...names.keys()].every(n => covers(c, n)));
  const partial = !fully ? cov?.[0] : undefined;
  const chosen = fully ?? partial;
  const inDir = t.startsWith(ex.dir + "/");
  const rustDir = g.packages.find(p => !p.owner.includes("/🟦️typescript") && t.startsWith(p.owner + "/"));
  if (chosen && !asset) {
    const facade = chosen.facade;
    const key = chosen.pkg.name + " " + chosen.subpath;
    void key;
    if (fully) { targetMap[t] = { pkg: chosen.pkg.name, pkgDir: chosen.pkg.owner, subpath: chosen.subpath, facade, direct: false }; continue; }
    const fe = facades.get(facade) ?? { pkg: chosen.pkg.name, pkgDir: chosen.pkg.owner, facade, subpath: chosen.subpath, created: false, adds: new Map() };
    facades.set(facade, fe);
    const add = fe.adds.get(t) ?? { from: t, names: [], json: [] };
    fe.adds.set(t, add);
    for (const [n, v] of names) if (!covers(chosen, n)) add.names.push({ name: n, value: v });
    targetMap[t] = { pkg: chosen.pkg.name, pkgDir: chosen.pkg.owner, subpath: chosen.subpath, facade, direct: false };
    continue;
  }
  if (rustDir && !inDir) {
    if (!(base === "📜️script.ts") && !asset) { problems.push(`${t}: lies in language package ${rustDir.owner} and is not a script`); continue; }
    const subpath = exportKey(dirname(t.slice(rustDir.owner.length + 1)) === "." ? "" : dirname(t.slice(rustDir.owner.length + 1)), base);
    directExports.push({ pkg: rustDir.name, pkgDir: rustDir.owner, subpath, target: t });
    targetMap[t] = { pkg: rustDir.name, pkgDir: rustDir.owner, subpath, facade: null, direct: true };
    continue;
  }
  if (inDir) {
    if (asset) { problems.push(`${t}: asset inside package directory ${ex.dir}`); continue; }
    const relInDir = t.slice(ex.dir.length + 1);
    const subpath = exportKey(dirname(relInDir) === "." ? "" : dirname(relInDir), base);
    directExports.push({ pkg: ex.name, pkgDir: ex.dir, subpath, target: t });
    targetMap[t] = { pkg: ex.name, pkgDir: ex.dir, subpath, facade: null, direct: true };
    continue;
  }
  if (mirrorProblem) { problems.push(mirrorProblem); continue; }
  if (existsSync(join(ROOT, facade)) && !facades.has(facade) && !isPureFacade(facade) && !g.packages.some(p => p.exportTargets.some(r => r.target && nfc(p.owner + "/" + r.target.slice(2)) === facade))) { problems.push(`${t}: facade path ${facade} exists and is not an export target`); continue; }
  const fe = facades.get(facade) ?? { pkg: ex.name, pkgDir: ex.dir, facade, subpath, created: !existsSync(join(ROOT, facade)), adds: new Map() };
  facades.set(facade, fe);
  const add = fe.adds.get(t) ?? { from: t, names: [], json: [] };
  fe.adds.set(t, add);
  if (!asset) {
    for (const [n, v] of names) add.names.push({ name: n, value: v });
    if (u.sideEffect && !names.size) add.sideEffect = true;
  } else if (u.jsonDefault) add.json.push({ exportName: jsonExportName(g, t), file: t });
  targetMap[t] = { pkg: ex.name, pkgDir: ex.dir, subpath, facade, direct: false, ...(u.hasDefault ? { hasDefault: true } : {}), ...(asset && u.jsonDefault ? { jsonExport: jsonExportName(g, t) } : {}) };
}

const seenKey = new Map<string, string>();
for (const f of facades.values()) {
  const key = f.pkg + " " + f.subpath;
  const prev = seenKey.get(key);
  if (prev && prev !== f.facade) problems.push(`subpath collision ${key}: ${prev} vs ${f.facade}`);
  seenKey.set(key, f.facade);
}
for (const d of directExports) {
  const key = d.pkg + " " + d.subpath;
  const prev = seenKey.get(key);
  if (prev && prev !== d.target) problems.push(`subpath collision ${key}: ${prev} vs ${d.target}`);
  seenKey.set(key, d.target);
}
const existingBySubpath = new Map<string, string>();
for (const g of groups.values()) for (const p of g.packages) for (const r of p.exportTargets) if (r.target) existingBySubpath.set(p.name + " " + r.subpath, nfc(p.owner + "/" + r.target.slice(2)));
for (const f of facades.values()) { const prev = existingBySubpath.get(f.pkg + " " + f.subpath); if (prev && prev !== f.facade) problems.push(`export key ${f.pkg} ${f.subpath} already targets ${prev}, not ${f.facade}`); }
for (const d of directExports) { const prev = existingBySubpath.get(d.pkg + " " + d.subpath); if (prev && prev !== d.target) problems.push(`export key ${d.pkg} ${d.subpath} already targets ${prev}, not ${d.target}`); }

const out = {
  facades: [...facades.values()].map(f => ({ pkg: f.pkg, pkgDir: f.pkgDir, facade: f.facade, subpath: f.subpath, created: f.created, adds: [...f.adds.values()].map(a => ({ ...a, names: a.names.sort((x, y) => (x.name < y.name ? -1 : 1)) })) })),
  directExports, targetMap, problems,
  newPackages: [...newPkgs].map(([o, v]) => ({ sourceOwner: o, ...v })),
};
writeFileSync(planPath, JSON.stringify(out, null, 1));
const reuse = Object.values(targetMap).filter(t => !t.direct && !facades.has(t.facade ?? "")).length;
console.log(`targets=${uses.size} edges=${edges.length} facades=${out.facades.length} (created ${out.facades.filter(f => f.created).length}) directExports=${directExports.length} reusedFacadeTargets=${reuse} problems=${problems.length}`);
for (const p of problems) console.log("PROBLEM", p);
