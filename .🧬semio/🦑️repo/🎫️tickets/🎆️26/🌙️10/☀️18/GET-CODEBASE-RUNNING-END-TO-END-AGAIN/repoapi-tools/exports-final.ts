import { relative, resolve } from "node:path";
import { GENERATED, LIB_REL, PKG_DIR, PKG_NAME, ROOT, TICKET, posix, readJson, writeJson } from "./lib.ts";
import { classify } from "./classify.ts";
import { buildGlue } from "./glue.ts";
import { DOMAINS, libRelOf } from "./rules.ts";
import { loadSpecs, type SpecRow } from "./specs.ts";
import { packageIndex, type Pkg } from "./packages.ts";

const OUT = resolve(TICKET, "repoapi-tools/exports-final.json");
const PKG_JSON_REL = posix(relative(ROOT, resolve(PKG_DIR, "package.json")));

type Entry = {
  subpath: string;
  target: string;
  glueFile: string | null;
  targetFile: string;
  kind: "module" | "json" | "asset";
  symbols: string[] | "*";
  importers: string[];
  sources: string[];
  action: "new-entry" | "extend-existing-glue";
  alreadyExported: boolean;
  existing?: { package: string; subpath: string };
};

const strip = (s: string): string => s.replace(/[^\x00-\x7f]/gu, "").replace(/\/+/gu, "/").replace(/^\/|\/$/gu, "");

/** Subpath choice: an existing key for the same target, else the most frequent proposal, else the shortest. JSON documents keep a `.json` suffix. */
function chooseSubpath(rows: SpecRow[], isJson: boolean, existing: string | undefined): string {
  if (existing) return existing;
  const votes = new Map<string, number>();
  for (const r of rows) {
    let sp = r.subpath;
    if (isJson && !sp.endsWith(".json")) sp += ".json";
    if (!isJson && sp.endsWith("/.mjs")) sp = sp.slice(0, -"/.mjs".length);
    votes.set(sp, (votes.get(sp) ?? 0) + 1);
  }
  return [...votes].sort((a, b) => b[1] - a[1] || a[0].length - b[0].length || a[0].localeCompare(b[0]))[0]![0];
}

/** Glue and manifest target of one source file inside a package: the file itself when the manifest sits in the source owner, else a glue file mirroring the owner-relative path. */
function targetOf(pkg: Pkg, targetFile: string, proposedGlue: string | undefined): { target: string; glue: string | null } {
  if (pkg.dir === pkg.sourceOwner || targetFile.startsWith(pkg.dir + "/")) return { target: "./" + posix(relative(pkg.dir, targetFile)), glue: null };
  let ownerRel = targetFile.startsWith(pkg.sourceOwner + "/") ? targetFile.slice(pkg.sourceOwner.length + 1) : targetFile;
  if (pkg.name === "@semio-tech/framework") ownerRel = ownerRel.replace(/^🔨️modules\//u, "");
  if (targetFile.endsWith(".json")) {
    const stem = ownerRel.split("/").pop()!.replace(/\.json$/u, "").replace(/[^\x00-\x7f]/gu, "");
    ownerRel = [...ownerRel.split("/").slice(0, -1), "🔣️json", ...(stem ? [stem] : []), "🟦️.ts"].join("/");
  } else if (proposedGlue) ownerRel = proposedGlue.replace(/^📦️packages\/[^/]+\//u, "");
  ownerRel = ownerRel.replace(/(^|\/)🧪️tests(?=\/|$)/gu, "$1🧪️testing");
  return { target: "./" + ownerRel, glue: posix(`${pkg.dir}/${ownerRel}`) };
}

const { rows: allRows, files: specFiles } = loadSpecs();
const index = packageIndex();
const exportTargets = new Map<string, { package: string; subpath: string }>();
for (const p of index.all) for (const [subpath, value] of Object.entries(p.exports)) if (typeof value === "string") { const abs = posix(resolve(ROOT, p.dir, value)); if (!exportTargets.has(abs)) exportTargets.set(abs, { package: p.name, subpath }); }
const classification = classify();
const glue = buildGlue(classification);

const repoLibRows = allRows.filter(r => r.pkg === PKG_NAME);
const otherRows = allRows.filter(r => r.pkg !== PKG_NAME);

/** repo-lib: domain entries replace every per-file proposal. */
const edgesByTarget = new Map<string, Set<string>>();
for (const rep of classification.reports) for (const [t] of rep.b) (edgesByTarget.get(libRelOf(t)) ?? edgesByTarget.set(libRelOf(t), new Set()).get(libRelOf(t))!).add(rep.file);
const domainTargetByLib = new Map<string, string>();
for (const g of glue) for (const m of g.members) domainTargetByLib.set(m.target, g.key);
const aTargets = new Map<string, Set<string>>();
const dead = new Set<string>(), open = new Map<string, Set<string>>();
for (const rep of classification.reports) {
  for (const a of rep.a) (aTargets.get(libRelOf(a.target)) ?? aTargets.set(libRelOf(a.target), new Set()).get(libRelOf(a.target))!).add(a.to);
  for (const d of rep.dead) dead.add(libRelOf(d.target));
  for (const o of rep.open) (open.get(libRelOf(o.target)) ?? open.set(libRelOf(o.target), new Set()).get(libRelOf(o.target))!).add(rep.file);
}
const specCoverage = new Map<string, { disposition: string; detail: string[]; specImporters: number; sources: string[] }>();
for (const r of repoLibRows) {
  const lib = r.targetFile.startsWith(LIB_REL + "/") ? r.targetFile.slice(LIB_REL.length + 1) : r.targetFile;
  const row = specCoverage.get(lib) ?? { disposition: "", detail: [], specImporters: 0, sources: [] };
  specCoverage.set(lib, row);
  row.specImporters += r.importers.length;
  if (!row.sources.includes(r.source)) row.sources.push(r.source);
}
for (const [lib, row] of specCoverage) {
  const parts: string[] = [];
  if (domainTargetByLib.has(lib)) parts.push(`b:${domainTargetByLib.get(lib)}`);
  if (aTargets.has(lib)) parts.push(`a:${[...aTargets.get(lib)!].join("|")}`);
  if (open.has(lib)) parts.push("c:fixture-coupling");
  if (dead.has(lib)) parts.push("c:dead-import");
  row.disposition = parts.length ? parts.join(" + ") : "settled: no importer outside the library remains (fixed by a peer, moved into the library, or bootstrap-internal)";
  row.detail = parts;
}

const repoLib = {
  package: PKG_NAME,
  packageJson: PKG_JSON_REL,
  exportsToAdd: Object.fromEntries(glue.map(g => [g.key, "./" + posix(relative(PKG_DIR, g.abs))])),
  domains: Object.fromEntries(glue.map(g => {
    const importers = new Set<string>();
    for (const m of g.members) for (const f of edgesByTarget.get(m.target) ?? []) importers.add(f);
    return [g.key, {
      domain: g.domain,
      purpose: DOMAINS[g.domain].purpose,
      glueFile: posix(relative(ROOT, g.abs)),
      format: g.glue,
      names: g.members.reduce((n, m) => n + m.names.length + m.types.length, 0),
      importerFiles: importers.size,
      members: g.members.map(m => ({ from: m.target, values: m.names, types: m.types, json: m.json })),
    }];
  })),
  specCoverage: Object.fromEntries([...specCoverage].sort().map(([k, v]) => [k, v])),
};

/** every other package: merge rows per target, resolve one subpath each, derive glue. */
const byPkg = new Map<string, Map<string, SpecRow[]>>();
for (const r of otherRows) {
  const m = byPkg.get(r.pkg) ?? new Map<string, SpecRow[]>();
  byPkg.set(r.pkg, m);
  (m.get(r.targetFile) ?? m.set(r.targetFile, []).get(r.targetFile)!).push(r);
}
const packages: Record<string, { packageJson: string | null; packageDir: string | null; sourceOwner: string | null; existingExports: number; entries: Record<string, Entry>; collisions: string[]; unresolved: boolean }> = {};
const exportsToAdd: Record<string, Record<string, string>> = { [PKG_JSON_REL]: repoLib.exportsToAdd };
const exportsMissing: Record<string, Record<string, string>> = {};
for (const [k, v] of Object.entries(repoLib.exportsToAdd)) if (index.byName.get(PKG_NAME)!.exports[k] !== v) (exportsMissing[PKG_JSON_REL] ??= {})[k] = v;
const glueToCreate: { file: string; package: string; subpath: string; symbols: string[] | "*"; action: string }[] = [];
for (const [name, targets] of [...byPkg].sort((a, b) => a[0].localeCompare(b[0]))) {
  const pkg = index.byName.get(name);
  const entry = { packageJson: pkg?.packageJson ?? null, packageDir: pkg?.dir ?? null, sourceOwner: pkg?.sourceOwner ?? null, existingExports: pkg ? Object.keys(pkg.exports).length : 0, entries: {} as Record<string, Entry>, collisions: [] as string[], unresolved: !pkg };
  packages[name] = entry;
  const taken = new Map<string, string>();
  for (const [targetFile, rows] of [...targets].sort((a, b) => a[0].localeCompare(b[0]))) {
    const isJson = targetFile.endsWith(".json");
    if (!/\.(?:[cm]?[jt]sx?|json)$/u.test(targetFile)) {
      entry.entries[chooseSubpath(rows, false, undefined)] = { subpath: chooseSubpath(rows, false, undefined), target: "(asset: the owner decides how a stylesheet or media file is exported)", glueFile: null, targetFile, kind: "asset", symbols: "*", importers: [...new Set(rows.flatMap(r => r.importers))].sort(), sources: [...new Set(rows.map(r => r.source))].sort(), action: "new-entry", alreadyExported: true };
      continue;
    }
    const where = pkg ? targetOf(pkg, targetFile, rows.find(r => r.glueFile)?.glueFile) : { target: "(unresolved package)", glue: null };
    const existingAnywhere = exportTargets.get(posix(resolve(ROOT, targetFile)));
    const existingKey = pkg ? Object.entries(pkg.exports).find(([, v]) => typeof v === "string" && pkg && posix(resolve(ROOT, pkg.dir, v)) === posix(resolve(ROOT, where.glue ?? targetFile)))?.[0] : undefined;
    let subpath = chooseSubpath(rows, isJson, existingKey ?? existingAnywhere?.subpath);
    if (taken.has(subpath) && taken.get(subpath) !== targetFile) {
      entry.collisions.push(`${subpath}: ${taken.get(subpath)} vs ${targetFile}`);
      subpath = subpath.replace(/(\.json)?$/u, m => `-${strip(targetFile.split("/").slice(-2, -1)[0] ?? "x") || "x"}${m}`);
    }
    taken.set(subpath, targetFile);
    const symbols = rows.some(r => r.symbols === "*") ? "*" : [...new Set(rows.flatMap(r => r.symbols as string[]))].sort();
    const action = rows.some(r => r.action === "extend-existing-glue") && !rows.some(r => r.action === "new-entry") ? "extend-existing-glue" : "new-entry";
    entry.entries[subpath] = {
      subpath, target: where.target, glueFile: where.glue, targetFile, kind: isJson ? "json" : "module", symbols,
      importers: [...new Set(rows.flatMap(r => r.importers))].sort(), sources: [...new Set(rows.map(r => r.source))].sort(), action, alreadyExported: !!existingKey || !!existingAnywhere, ...(existingAnywhere && existingAnywhere.package !== name ? { existing: existingAnywhere } : {}),
    };
    const elsewhere = !!existingAnywhere && existingAnywhere.package !== name;
    if (pkg && !elsewhere && action === "new-entry") {
      (exportsToAdd[pkg.packageJson] ??= {})[subpath] = where.target;
      if (pkg.exports[subpath] !== where.target) (exportsMissing[pkg.packageJson] ??= {})[subpath] = where.target;
    }
    if (where.glue && !existingAnywhere) glueToCreate.push({ file: where.glue, package: name, subpath, symbols, action });
  }
}

const summary = [
  { package: PKG_NAME, newExportEntries: glue.length, importerEdges: [...edgesByTarget.values()].reduce((n, s) => n + s.size, 0), note: "domain entries replace per-file proposals" },
  ...Object.entries(packages).map(([p, v]) => ({ package: p, newExportEntries: v.packageJson ? Object.keys(exportsToAdd[v.packageJson] ?? {}).length : 0, importerEdges: new Set(Object.values(v.entries).flatMap(e => e.importers)).size })),
].sort((a, b) => b.importerEdges - a.importerEdges);

const final = {
  generatedAt: new Date().toISOString(),
  freeze: "exportsToAdd is the complete set of manifest lines; exportsMissing is the part the live manifests do not carry yet (empty once the deps executor applied them); then run repoapi-tools/apply.ts",
  specs: { files: specFiles, rowsRead: allRows.length, repoLibRows: repoLibRows.length, otherRows: otherRows.length },
  summary,
  exportsToAdd,
  exportsMissing,
  repoLib,
  packages,
  byPackage: Object.fromEntries(Object.entries(packages).map(([p, v]) => [p, Object.fromEntries(Object.values(v.entries).map(e => [e.targetFile, { targetFile: e.targetFile, proposedSubpath: e.subpath, proposedGlueFile: e.glueFile, symbols: e.symbols, importers: e.importers, action: e.action }]))])),
  glueToCreate,
  classification: {
    filesPlanned: classification.reports.length,
    filesChanged: classification.reports.filter(r => r.changed).length,
    frameworkNames: classification.reports.reduce((n, r) => n + r.a.length, 0),
    deadImports: classification.reports.reduce((n, r) => n + r.dead.length, 0),
    openFixtureCouplings: classification.reports.flatMap(r => r.open.map(o => ({ importer: r.file, target: libRelOf(o.target), reason: o.reason }))),
    failures: classification.reports.flatMap(r => r.failures.map(f => `${r.file}: ${f}`)),
  },
};
writeJson(OUT, final);
void GENERATED;
console.log(`spec files ${specFiles.length}, rows ${allRows.length}; repo-lib domains ${glue.length}; other packages ${Object.keys(packages).length}; new keys ${Object.values(exportsToAdd).reduce((n, m) => n + Object.keys(m).length, 0)}`);
for (const s of summary.slice(0, 40)) console.log(" ", s.package.padEnd(40), String(s.newExportEntries).padStart(4), String(s.importerEdges).padStart(5));
const unresolved = Object.entries(packages).filter(([, v]) => v.unresolved).map(([k]) => k);
if (unresolved.length) console.log("UNRESOLVED PACKAGES", unresolved);
for (const [p, v] of Object.entries(packages)) if (v.collisions.length) console.log("COLLISIONS", p, v.collisions);
if (index.duplicates.size) console.log("DUPLICATE PACKAGE NAMES", [...index.duplicates].map(([k, v]) => `${k}: ${v.join(", ")}`));
