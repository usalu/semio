/** 🗺️ Derives the SDK surface (facades and package exports) from the violating plugin imports. Read-only: writes only the plan JSON.
 * Usage: bun plan.ts <scan.json> <plan.json> [scope-prefix]
 */
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { ROOT, SDK_DIR, SDK_PACKAGE, sdkExportTargets } from "./lib.ts";

const [scanPath, planPath, scopeArg] = process.argv.slice(2);
const SCOPE = scopeArg ?? "🧰️framework/🔨️modules/";
const MODULES = "🧰️framework/🔨️modules/";
const recs: any[] = JSON.parse(readFileSync(scanPath, "utf8")).filter((r: any) => r.resolved.startsWith(SCOPE));

const sdkPkg = JSON.parse(readFileSync(join(ROOT, SDK_DIR, "package.json"), "utf8"));
const existingExports: Record<string, string> = sdkPkg.exports;
const existingFacades = new Map<string, string>();
for (const [sub, t] of Object.entries(existingExports)) existingFacades.set((SDK_DIR + "/" + (t as string).slice(2)).normalize("NFC"), sub);

const isAsset = (p: string) => !/\.(ts|tsx)$/.test(p);
const targets = [...new Set(recs.map(r => r.resolved as string))];
const tsTargets = targets.filter(t => !isAsset(t));

const options: ts.CompilerOptions = {
  target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler,
  allowImportingTsExtensions: true, resolveJsonModule: true, customConditions: ["semio-source"], jsx: ts.JsxEmit.ReactJSX,
  skipLibCheck: true, noEmit: true, types: [], esModuleInterop: true, isolatedModules: true,
  paths: {
    "dom-accessibility-api": ["../../../node_modules/dom-accessibility-api/dist/index.d.ts"],
    "fast-json-stable-stringify": [join(ROOT, SDK_DIR, "🌿️ambient/🔤️fast-json-stable-stringify/🟦️.d.ts")],
  },
  baseUrl: join(ROOT, SDK_DIR),
};
const roots = [...existingFacades.keys(), ...tsTargets].map(p => join(ROOT, p)).filter(existsSync);
const t0 = Date.now();
const program = ts.createProgram(roots, options);
const checker = program.getTypeChecker();
console.error(`[plan] program ${program.getSourceFiles().length} files in ${Date.now() - t0}ms`);

function moduleExports(rel: string): Map<string, ts.Symbol> {
  const sf = program.getSourceFile(join(ROOT, rel));
  const out = new Map<string, ts.Symbol>();
  if (!sf) return out;
  const sym = checker.getSymbolAtLocation(sf);
  if (!sym) return out;
  for (const e of checker.getExportsOfModule(sym)) out.set(e.name, e);
  return out;
}
function resolveAlias(s: ts.Symbol): ts.Symbol { return s.flags & ts.SymbolFlags.Alias ? checker.getAliasedSymbol(s) : s; }
function keyOf(s: ts.Symbol): string | null {
  const d = s.declarations?.[0];
  return d ? d.getSourceFile().fileName + ":" + d.pos : null;
}
function isValue(s: ts.Symbol): boolean {
  let cur: ts.Symbol | undefined = s;
  while (cur && cur.flags & ts.SymbolFlags.Alias) {
    const d = cur.declarations?.[0];
    if (d && ts.isExportSpecifier(d) && (d.isTypeOnly || d.parent.parent.isTypeOnly)) return false;
    if (d && ts.isImportSpecifier(d) && (d.isTypeOnly || d.parent.parent.isTypeOnly)) return false;
    if (d && ts.isNamespaceExport(d) && d.parent.isTypeOnly) return false;
    cur = checker.getImmediateAliasedSymbol(cur);
  }
  return !!cur && (cur.flags & ts.SymbolFlags.Value) !== 0;
}

const facadeIndex = new Map<string, { sub: string; name: string }[]>();
const facadeExports = new Map<string, Map<string, ts.Symbol>>();
for (const [fac, sub] of existingFacades) {
  if (sub === ".") continue;
  const ex = moduleExports(fac);
  facadeExports.set(fac, ex);
  for (const [name, s] of ex) { const k = keyOf(resolveAlias(s)); if (!k) continue; const arr = facadeIndex.get(k) ?? []; arr.push({ sub, name }); facadeIndex.set(k, arr); }
}

function segName(seg: string): string {
  return seg.replace(/^(?:\p{Extended_Pictographic}(?:️|‍\p{Extended_Pictographic}️?)*)+/u, "").replace(/^️/, "");
}
function subpathOf(facadeRel: string): string {
  const inside = facadeRel.slice(SDK_DIR.length + 1).replace(/\/🟦️\.ts$/, "");
  return "./" + inside.split("/").map(segName).join("/");
}
function mirrorFacade(t: string): string {
  let rel = t.slice(MODULES.length);
  rel = rel.replace("/📦️packages/🟦️typescript", "");
  rel = rel.replace(/\/🧪️tests\//g, "/🧪️testing/");
  if (isAsset(rel)) {
    const fixtureAt = rel.indexOf("/🧪️testing/🧫️fixtures/");
    rel = (fixtureAt >= 0 ? rel.slice(0, fixtureAt) + "/🧪️testing" : dirname(rel)) + "/🟦️.ts";
  }
  rel = rel.replace(/\/🟦️\.tsx$/, "/🟦️.ts");
  return (SDK_DIR + "/" + rel).normalize("NFC");
}
function camel(s: string): string { return s.replace(/-(\w)/g, (_, c) => c.toUpperCase()); }
function jsonExportName(t: string): string {
  const isFixture = t.includes("/🧪️tests/🧫️fixtures/") || t.includes("/🧫️fixtures/");
  const rawDir = dirname(t.slice(MODULES.length)).replace("/📦️packages/🟦️typescript", "");
  const fixtureTail = isFixture ? rawDir.split("/🧫️fixtures").slice(1).join("/") : "";
  const modulePart = isFixture ? rawDir.split("/🧪️tests")[0] : rawDir;
  const dir = (modulePart + (fixtureTail ? "/" + fixtureTail : "")).split("/").map(segName).filter(s => s && s !== "schema" && s !== "fixtures" && s !== "tests");
  const base = t.split("/").pop()!.replace(/\.[^.]+$/, "");
  const baseName = /^\p{Extended_Pictographic}/u.test(base) ? "" : segName(base);
  let parts = [...dir.slice(-2), ...(baseName ? [baseName] : [])];
  while (parts.length > 1 && /^\d/.test(parts[0])) parts = parts.slice(1);
  const n = camel(parts.join("-"));
  return n.charAt(0).toLowerCase() + n.slice(1) + (t.endsWith(".sql") ? "Sql" : isFixture ? "Fixture" : "Schema");
}

interface TargetUse { names: Map<string, { value: boolean; type: boolean }>; jsonDefault: boolean; defaultTs: boolean; flags: string[]; files: Set<string> }
const uses = new Map<string, TargetUse>();
for (const r of recs) {
  const u = uses.get(r.resolved) ?? { names: new Map(), jsonDefault: false, defaultTs: false, flags: [], files: new Set() };
  uses.set(r.resolved, u);
  u.files.add(r.file);
  const add = (n: string, typeOnly: boolean) => { const e = u.names.get(n) ?? { value: false, type: false }; if (typeOnly) e.type = true; else e.value = true; u.names.set(n, e); };
  for (const n of r.named) add(n.imported, r.typeOnly || n.typeOnly);
  if (r.members) for (const m of r.members) add(m, r.kind === "importType");
  if (r.defaultName) { if (isAsset(r.resolved)) u.jsonDefault = true; else u.defaultTs = true; }
  if (r.membersUnknown) u.flags.push(`membersUnknown ${r.file}:${r.line}`);
  if (r.exportStar) u.flags.push(`exportStar ${r.file}:${r.line}`);
  if (r.namespaceName && isAsset(r.resolved)) u.flags.push(`namespace-asset ${r.file}:${r.line}`);
}

const facades = new Map<string, { facade: string; subpath: string; existing: boolean; adds: Map<string, { from: string; names: Map<string, boolean>; json: { exportName: string; file: string; attr?: string }[] }> }>();
const targetMap: Record<string, { facade: string; subpath: string; jsonExport?: string }> = {};
const problems: string[] = [];

for (const [t, u] of uses) {
  if (u.defaultTs) problems.push(`default import of TS module ${t}`);
  for (const f of u.flags) problems.push(`${t}: ${f}`);
  let chosen: string | null = null;
  if (!isAsset(t)) {
    const own = moduleExports(t);
    const covered: { fac: string; ok: boolean }[] = [];
    for (const [fac, ex] of facadeExports) {
      let ok = true;
      for (const n of u.names.keys()) {
        const s = own.get(n);
        if (!s) { ok = false; break; }
        const k = keyOf(resolveAlias(s));
        const hit = ex.get(n);
        if (!hit || keyOf(resolveAlias(hit)) !== k) { ok = false; break; }
      }
      covered.push({ fac, ok });
    }
    const hits = covered.filter(c => c.ok);
    if (u.names.size > 0 && hits.length) chosen = hits.sort((a, b) => facadeExports.get(a.fac)!.size - facadeExports.get(b.fac)!.size)[0].fac;
    for (const n of u.names.keys()) if (!own.has(n)) problems.push(`${t}: name ${n} not exported by target`);
  }
  let facade = chosen ?? mirrorFacade(t);
  const existing = existingFacades.has(facade) || existsSync(join(ROOT, facade));
  const subpath = existingFacades.get(facade) ?? subpathOf(facade);
  const fe = facades.get(facade) ?? { facade, subpath, existing, adds: new Map() };
  facades.set(facade, fe);
  if (!chosen) {
    const add = fe.adds.get(t) ?? { from: t, names: new Map(), json: [] };
    fe.adds.set(t, add);
    if (!isAsset(t)) {
      const own = moduleExports(t);
      for (const [n, usage] of u.names) {
        const s = own.get(n);
        if (!s) continue;
        if (facadeExports.get(facade)?.has(n)) continue;
        add.names.set(n, isValue(s));
      }
    } else if (u.jsonDefault) {
      add.json.push({ exportName: jsonExportName(t), file: t, ...(t.endsWith(".sql") ? { attr: "text" } : {}) });
    }
  }
  targetMap[t] = { facade, subpath, ...(u.jsonDefault ? { jsonExport: jsonExportName(t) } : {}) };
}

const out = { facades: [...facades.values()].map(f => ({ facade: f.facade, subpath: f.subpath, existing: f.existing, adds: [...f.adds.values()].map(a => ({ from: a.from, names: [...a.names].map(([name, value]) => ({ name, value })), json: a.json })) })), targetMap, problems };
writeFileSync(planPath, JSON.stringify(out, null, 1));
console.log(`targets=${uses.size} facades=${out.facades.length} newFacades=${out.facades.filter(f => !f.existing).length} problems=${problems.length}`);
for (const p of problems) console.log("PROBLEM", p);
