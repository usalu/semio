/** 🗺️ Derives the public facades and package `exports` of the three owning packages from the flagged import edges. Read-only: writes only the plan JSON.
 * Usage: bun plan.ts <scan.json> <plan.json>
 */
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { ROOT } from "../sdkb-tools/lib.ts";
import { OWNERS, OWNER_DIR, PKG_NAME, segName, isAsset, nfc, type Edge, type OwnerKey } from "./model.ts";

const [scanPath, planPath] = process.argv.slice(2);
const edges: Edge[] = JSON.parse(readFileSync(scanPath, "utf8"));
const problems: string[] = [];

function mapTarget(owner: OwnerKey, target: string): { facade: string; subpath: string } {
  const S = OWNERS[owner].sourceOwner;
  let rel = target.slice(S.length + 1);
  rel = rel.replace(/\/📦️packages\/🟦️typescript(?=\/|$)/gu, "");
  let dirRel: string;
  if (isAsset(rel)) {
    const at = rel.indexOf("/🧪️tests/🧫️fixtures/");
    const at2 = rel.indexOf("/🧪️tests/");
    dirRel = at >= 0 ? rel.slice(0, at) + "/🧪️testing" : at2 >= 0 ? rel.slice(0, at2) + "/🧪️testing" : dirname(rel);
  } else {
    const base = rel.split("/").pop()!;
    if (base !== "🟦️.ts") problems.push(`target ${target}: not a 🟦️.ts entry`);
    dirRel = dirname(rel).replace(/(^|\/)🧪️tests(?=\/|$)/gu, "$1🧪️testing");
  }
  if (dirRel === ".") return { facade: OWNER_DIR[owner] + "/🟦️.ts", subpath: "." };
  const facade = nfc(OWNER_DIR[owner] + "/" + dirRel + "/🟦️.ts");
  const subpath = "./" + dirRel.split("/").map(segName).join("/");
  return { facade, subpath };
}

const camel = (s: string) => s.replace(/-(\w)/gu, (_, c) => c.toUpperCase());
function jsonExportName(owner: OwnerKey, target: string): string {
  const S = OWNERS[owner].sourceOwner;
  const rel = target.slice(S.length + 1);
  const isFixture = rel.includes("/🧪️tests/");
  const dirRel = dirname(rel);
  const modulePart = isFixture ? rel.split("/🧪️tests")[0] : dirRel;
  const fixtureTail = isFixture ? dirRel.split("/🧫️fixtures").slice(1).join("/") : "";
  const dir = (modulePart + (fixtureTail ? "/" + fixtureTail : "")).split("/").map(segName).filter(s => s && s !== "schema" && s !== "fixtures" && s !== "tests");
  const base = target.split("/").pop()!.replace(/\.[^.]+$/u, "");
  const baseName = /^\p{Extended_Pictographic}/u.test(base) ? "" : segName(base);
  let parts = [...dir.slice(-2), ...(baseName ? [baseName] : [])];
  while (parts.length > 1 && /^\d/u.test(parts[0])) parts = parts.slice(1);
  if (!parts.length) parts = [segName(OWNERS[owner].sourceOwner.split("/").pop()!)];
  const n = camel(parts.join("-"));
  return n.charAt(0).toLowerCase() + n.slice(1) + (isFixture ? "Fixture" : "Schema");
}

const options: ts.CompilerOptions = {
  target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler,
  allowImportingTsExtensions: true, resolveJsonModule: true, customConditions: ["semio-source"], jsx: ts.JsxEmit.ReactJSX,
  skipLibCheck: true, noEmit: true, types: [], esModuleInterop: true, isolatedModules: true,
};

interface Use { owner: OwnerKey; names: Set<string>; all: boolean; sideEffectOnly?: boolean; jsonDefault: boolean; files: Set<string>; flags: string[] }
const uses = new Map<string, Use>();
for (const e of edges) {
  const u = uses.get(e.resolved) ?? { owner: e.owner, names: new Set(), all: false, jsonDefault: false, files: new Set(), flags: [] };
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
    if (e.defaultName) u.flags.push(`default-ts ${e.file}:${e.line}`);
    if (e.exportStar || e.membersUnknown || (e.namespaceName && !e.members?.length && !e.membersUnknown && false)) u.all = true;
    if (e.sideEffect) u.flags.push(`side-effect ${e.file}:${e.line}`);
  }
}

const tsTargets = [...uses.keys()].filter(t => !isAsset(t) && existsSync(join(ROOT, t)));
const existingFacades = (Object.keys(OWNER_DIR) as OwnerKey[]).flatMap(o => {
  const pkg = join(ROOT, OWNER_DIR[o], "package.json");
  if (!existsSync(pkg)) return [];
  const ex = JSON.parse(readFileSync(pkg, "utf8")).exports ?? {};
  return Object.values(ex).filter((v): v is string => typeof v === "string").map(v => nfc(OWNER_DIR[o] + "/" + v.slice(2))).filter(p => existsSync(join(ROOT, p)));
});
const program = ts.createProgram([...tsTargets, ...existingFacades].map(p => join(ROOT, p)), options);
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

type Add = { from: string; names: { name: string; value: boolean }[]; json: { exportName: string; file: string }[]; sideEffect?: boolean };
const facades = new Map<string, { owner: OwnerKey; facade: string; subpath: string; adds: Map<string, Add> }>();
const targetMap: Record<string, { owner: OwnerKey; facade: string; subpath: string; jsonExport?: string }> = {};

for (const [t, u] of uses) {
  for (const f of u.flags) problems.push(`${t}: ${f}`);
  const { facade, subpath } = mapTarget(u.owner, t);
  const fe = facades.get(facade) ?? { owner: u.owner, facade, subpath, adds: new Map() };
  facades.set(facade, fe);
  const add = fe.adds.get(t) ?? { from: t, names: [], json: [] };
  fe.adds.set(t, add);
  if (!isAsset(t)) {
    if (!existsSync(join(ROOT, t))) { problems.push(`${t}: target file missing`); continue; }
    const own = moduleExports(t);
    const want = u.all ? [...own.keys()].filter(n => n !== "default") : [...u.names];
    if (u.all && !want.length) add.sideEffect = true;
    for (const n of want) {
      const s = own.get(n);
      if (!s) { problems.push(`${t}: name ${n} not exported by target`); continue; }
      add.names.push({ name: n, value: isValue(s) });
    }
  } else if (u.jsonDefault) {
    add.json.push({ exportName: jsonExportName(u.owner, t), file: t });
  }
  targetMap[t] = { owner: u.owner, facade, subpath, ...(u.jsonDefault ? { jsonExport: jsonExportName(u.owner, t) } : {}) };
}

const subpathOwner = new Map<string, string>();
for (const f of facades.values()) {
  const key = f.owner + " " + f.subpath;
  const prev = subpathOwner.get(key);
  if (prev && prev !== f.facade) problems.push(`subpath collision ${key}: ${prev} vs ${f.facade}`);
  subpathOwner.set(key, f.facade);
}

const out = { owners: Object.fromEntries((Object.keys(OWNER_DIR) as OwnerKey[]).map(o => [o, { dir: OWNER_DIR[o], pkg: PKG_NAME[o] }])), facades: [...facades.values()].map(f => ({ owner: f.owner, facade: f.facade, subpath: f.subpath, adds: [...f.adds.values()].map(a => ({ ...a, names: a.names.sort((x, y) => (x.name < y.name ? -1 : 1)) })) })), targetMap, problems };
writeFileSync(planPath, JSON.stringify(out, null, 1));
const perOwner = (k: OwnerKey) => out.facades.filter(f => f.owner === k).length;
console.log(`targets=${uses.size} facades=${out.facades.length} (value ${perOwner("value")}, validator ${perOwner("validator")}, schema ${perOwner("schema")}) problems=${problems.length}`);
for (const p of problems) console.log("PROBLEM", p);
