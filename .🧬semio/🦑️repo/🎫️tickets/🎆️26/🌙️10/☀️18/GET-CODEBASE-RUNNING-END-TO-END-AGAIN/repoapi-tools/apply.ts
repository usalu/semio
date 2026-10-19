/**
 * Rewrites every deep importer to the public entries once the exports of `exports-final.json` exist in the manifests.
 *
 *   bun repoapi-tools/apply.ts dry   [--relative] [--only=<substring>] [--skip-others] [--skip-repo-lib] [--no-scan]
 *   bun repoapi-tools/apply.ts apply [same flags]   (first writes the missing glue files of the other packages: --no-glue skips that)
 *
 * dry   : plans against the current tree with the manifest keys treated as present (virtual), writes only
 *         `🗑️generated/repoapi/apply-plan.json` and `manifests-needed.json`, touches no source file.
 * apply : refuses to run while a manifest key it needs is missing (use --relative to target the glue files directly),
 *         then rewrites the files and resolves every new specifier with Bun.resolveSync.
 */
import { existsSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { GENERATED, LIB_REL, PKG_DIR, PKG_NAME, ROOT, cliClosure, isFrozen, isOutsidePackage, libTargets, posix, readJson, rel, writeJson, type Edge } from "./lib.ts";
import { scanEdges } from "./scan.ts";
import { plan, type Ctx, type Report } from "./engine.ts";
import { frameworkRules, isBootstrapFile, isConfigFile, jsonNames, libRelOf } from "./rules.ts";
import { exportsOf } from "./exports.ts";
import { nearestManifest, packageIndex } from "./packages.ts";
import { buildOtherGlue } from "./glue-others.ts";

const [mode = "dry", ...flags] = process.argv.slice(2);
if (mode !== "dry" && mode !== "apply") throw Error("usage: apply.ts <dry|apply> [--relative] [--only=<substring>] [--skip-others] [--skip-repo-lib] [--no-scan]");
const flag = (name: string): boolean => flags.includes(name);
const only = flags.find(f => f.startsWith("--only="))?.slice(7);
const write = mode === "apply";

const finalPath = resolve(import.meta.dir, "exports-final.json");
if (!existsSync(finalPath)) throw Error("exports-final.json missing: run bun repoapi-tools/exports-final.ts first");
const final = readJson<any>(finalPath);
const specsNewer = [...new Bun.Glob("*-tools/exports-needed.json").scanSync(resolve(import.meta.dir, ".."))].filter(f => statSync(resolve(import.meta.dir, "..", f)).mtimeMs > statSync(finalPath).mtimeMs);
if (specsNewer.length) console.log("WARNING: specs newer than exports-final.json (rerun exports-final.ts):", specsNewer.join(", "));

const index = packageIndex();
const repoLibKeys: Record<string, string> = final.repoLib.exportsToAdd;
const repoLibPkg = index.byName.get(PKG_NAME)!;
const missingRepoLib = Object.keys(repoLibKeys).filter(k => !(k in repoLibPkg.exports));
const relative_ = flag("--relative");
if (write && !relative_ && !flag("--skip-repo-lib") && missingRepoLib.length) {
  console.error(`refusing to apply: ${PKG_NAME} lacks ${missingRepoLib.length} manifest keys (${missingRepoLib.join(", ")}); the deps executor applies exports-final.json first, or pass --relative`);
  process.exit(2);
}

type OtherEntry = { owners: string[]; pkg: string; subpath: string; targetFile: string; glueAbs?: string; directAbs?: string; kind: string; symbols: string[] | "*"; keyExists: boolean };
const pendingGlue = new Set<string>();
const collisionKeys: string[] = [];
const others = new Map<string, OtherEntry>();
for (const [pkgName, p] of Object.entries<any>(final.packages)) {
  const live = index.byName.get(pkgName);
  for (const e of Object.values<any>(p.entries)) {
    if (e.action === "extend-existing-glue") continue;
    if (e.kind === "asset") continue;
    if (e.targetFile.startsWith(LIB_REL + "/") && !e.targetFile.startsWith(LIB_REL + "/⚡️caching/🚀️bootstrap/")) continue;
    if (e.existing) { others.set(e.targetFile, { owners: [pkgName, e.existing.package], pkg: e.existing.package, subpath: e.existing.subpath, targetFile: e.targetFile, directAbs: resolve(ROOT, e.targetFile), kind: e.kind, symbols: e.symbols, keyExists: true }); continue; }
    const sameTarget = (a: unknown, b: unknown): boolean => typeof a === "string" && typeof b === "string" && a.replace(/\/\/+/gu, "/") === b.replace(/\/\/+/gu, "/");
    const keyExists = !!live && sameTarget(live.exports[e.subpath], e.target);
    if (live && e.subpath in live.exports && !keyExists) { collisionKeys.push(`${pkgName}${e.subpath.slice(1)} (live ${JSON.stringify(live.exports[e.subpath])}, spec ${e.target})`); continue; }
    if (e.glueFile && !existsSync(resolve(ROOT, e.glueFile))) pendingGlue.add(resolve(ROOT, e.glueFile));
    others.set(e.targetFile, { owners: [pkgName], pkg: pkgName, subpath: e.subpath, targetFile: e.targetFile, glueAbs: e.glueFile ? resolve(ROOT, e.glueFile) : undefined, directAbs: e.glueFile ? undefined : resolve(ROOT, e.targetFile), kind: e.kind, symbols: e.symbols, keyExists });
  }
}
const pending = [...others.values()].filter(o => !o.keyExists);
if (write && !relative_ && !flag("--skip-others") && pending.length) console.log(`note: ${pending.length} package entries are not in the manifests yet; their importers are skipped`);

const refreshTree = write || flag("--refresh");
const known = new Set<string>([...libTargets(refreshTree), ...others.keys()]);
const scanFile = resolve(GENERATED, "apply-edges.json");
let scan: { edges: Edge[]; frozen: Edge[] };
if (flag("--no-scan") && existsSync(scanFile)) scan = readJson(scanFile);
else {
  const t0 = Date.now();
  const s = scanEdges(known, file => !isFrozen(file) && (!only || file.includes(only)), refreshTree);
  scan = { edges: s.edges, frozen: s.frozen };
  writeJson(scanFile, scan);
  console.log(`scanned ${s.files} files, ${s.edges.length} edges, ${Date.now() - t0} ms`);
}

const generated = flag("--skip-others") ? { create: [], extend: [], unnamed: [] as string[] } : buildOtherGlue();
const unnamedEntries = new Set(generated.unnamed.map(u => u.split(" ")[0]!));
const extension = new Map<string, Set<string>>();
for (const x of generated.extend) extension.set(resolve(ROOT, x.file), new Set([...x.add.values, ...x.add.types]));
if (collisionKeys.length) console.log("manifest keys that exist with another target (entries skipped):", collisionKeys.join("; "));
if (write && !flag("--no-glue") && !flag("--skip-others")) {
  const { create, extend } = generated as ReturnType<typeof buildOtherGlue>;
  const { mkdirSync } = await import("node:fs");
  for (const g of create) { const abs = resolve(ROOT, g.file); if (existsSync(abs)) continue; mkdirSync(dirname(abs), { recursive: true }); writeFileSync(abs, g.content); }
  const { extensionText } = await import("./glue-others.ts");
  for (const x of extend) writeFileSync(resolve(ROOT, x.file), extensionText(x, final));
  console.log(`glue: created ${create.length} files, extended ${extend.length}`);
  pendingGlue.clear();
}

const closure = cliClosure();
const { readRelativeClosureV1 } = await import(resolve(ROOT, LIB_REL, "🕸️dependencies/🧭️direction/🔗️relative-closure/🟦️.ts"));
for (const f of readRelativeClosureV1(ROOT, LIB_REL + "/🏗️build/📜️script.ts").files as string[]) closure.add(f);
const ctx: Ctx = {
  known: libTargets(), fw: frameworkRules(), closure, relative: relative_,
  jsonNames: jsonNames([...libTargets()].filter(t => t.endsWith(".json") && t.startsWith(LIB_REL + "/")).map(libRelOf)), blocked: new Set(),
};

const publicEntryPrefix = LIB_REL + "/📦️packages/";
const byFile = new Map<string, Edge[]>();
for (const e of scan.edges.filter(x => !x.target.startsWith(publicEntryPrefix))) (byFile.get(e.file) ?? byFile.set(e.file, []).get(e.file)!).push(e);

const summary = { closureSkipped: 0, files: 0, changed: 0, repoLibFiles: 0, otherEdges: 0, otherSkipped: [] as string[], a: 0, dead: 0, open: [] as string[], failures: [] as string[], unresolved: [] as string[], manifests: new Map<string, Set<string>>() };
const results: { file: string; before: string; after: string; report?: Report }[] = [];

const forceRelative = (file: string): boolean => relative_ || closure.has(file) || /\.(mjs|cjs)$/u.test(file) || file.endsWith("📜️script.ts") || isConfigFile(file) || isBootstrapFile(file);
const noteManifest = (file: string, pkgName: string): void => {
  const m = nearestManifest(file, index.all);
  if (m && m.name !== pkgName && !m.deps.has(pkgName)) (summary.manifests.get(m.packageJson) ?? summary.manifests.set(m.packageJson, new Set()).get(m.packageJson)!).add(pkgName);
};

for (const [file, edges] of [...byFile].sort((a, b) => a[0].localeCompare(b[0]))) {
  summary.files++;
  const abs = resolve(ROOT, file);
  if (closure.has(file)) { summary.closureSkipped++; continue; }
  if (!existsSync(abs)) { summary.otherSkipped.push(`${file} (file vanished since the scan)`); continue; }
  let text = readFileSync(abs, "utf8");
  const before = text;
  const replacements: { start: number; end: number; text: string }[] = [];
  if (!flag("--skip-others")) {
    for (const e of edges) {
      const o = others.get(e.target);
      if (!o) continue;
      if (!/^["'`]/u.test(text.slice(e.start, e.end)) || text.slice(e.start + 1, e.end - 1) !== e.spec) { summary.otherSkipped.push(`${file} -> ${e.spec} (file changed since the scan)`); continue; }
      if (o.owners.some(n => { const owner = index.byName.get(n); return owner && (file === owner.sourceOwner || file.startsWith(owner.sourceOwner + "/")); })) continue;
      if (!o.keyExists && !(mode === "dry") && !relative_) { summary.otherSkipped.push(`${file} -> ${o.pkg}${o.subpath.slice(1)} (key missing)`); continue; }
      if (unnamedEntries.has(`${o.pkg}${o.subpath.slice(1)}`)) { summary.otherSkipped.push(`${file} -> ${o.pkg}${o.subpath.slice(1)} (no named symbol: nothing to re-export)`); continue; }
      if (e.spec.includes("?") && o.glueAbs) { summary.otherSkipped.push(`${file} -> ${e.spec} (query import reads the source file itself, a glue file would change its text)`); continue; }
      const entryAbs = o.glueAbs ?? o.directAbs!;
      if (o.glueAbs && existsSync(o.glueAbs) && e.star && !/export \*/u.test(readFileSync(o.glueAbs, "utf8"))) { summary.otherSkipped.push(`${file} -> ${o.pkg}${o.subpath.slice(1)} (whole-module use needs an export * glue)`); continue; }
      if (o.glueAbs && existsSync(o.glueAbs) && !e.star && e.names.length) {
        const ex = exportsOf(o.glueAbs);
        const lacking = e.names.filter(n => !ex.values.has(n) && !ex.types.has(n) && !(mode === "dry" && extension.get(o.glueAbs!)?.has(n)));
        if (lacking.length) { summary.otherSkipped.push(`${file} -> ${o.pkg}${o.subpath.slice(1)} (glue lacks ${lacking.join(",")})`); continue; }
      }
      if (o.glueAbs && !existsSync(o.glueAbs) && write && !relative_) { summary.otherSkipped.push(`${file} -> ${o.pkg}${o.subpath.slice(1)} (glue file missing)`); continue; }
      const specText = forceRelative(file) ? (() => { const r = posix(relative(dirname(abs), entryAbs)); return r.startsWith(".") ? r : "./" + r; })() : (o.subpath === "." ? o.pkg : `${o.pkg}/${o.subpath.slice(2)}`);
      if (!forceRelative(file)) noteManifest(file, o.pkg);
      let end = e.end;
      if (o.kind === "json" && o.glueAbs) {
        const tail = text.slice(e.end);
        const attribute = tail.match(/^\s*with\s*\{[^}]*\}/u) ?? tail.match(/^\s*,\s*\{\s*with\s*:\s*\{[^}]*\}\s*\}/u);
        if (attribute) end = e.end + attribute[0].length;
      }
      const query = e.spec.includes("?") ? "?" + e.spec.split("?").slice(1).join("?") : "";
      replacements.push({ start: e.start, end, text: JSON.stringify(specText + query) });
      summary.otherEdges++;
    }
    for (const r of replacements.sort((a, b) => b.start - a.start)) text = text.slice(0, r.start) + r.text + text.slice(r.end);
  }
  let report: Report | undefined;
  if (!flag("--skip-repo-lib") && isOutsidePackage(file) && edges.some(e => e.target.startsWith(LIB_REL + "/"))) {
    report = plan(file, text, ctx);
    summary.repoLibFiles++;
    summary.a += report.a.length;
    summary.dead += report.dead.length;
    for (const o of report.open) summary.open.push(`${file} -> ${libRelOf(o.target)} (${o.reason})`);
    for (const f of report.failures) summary.failures.push(`${file}: ${f}`);
    if (report.changed) {
      text = report.text;
      for (const a of report.a) void a;
      for (const b of report.b.keys()) void b;
    }
  }
  if (text !== before) {
    summary.changed++;
    results.push({ file, before, after: text, report });
    if (report) {
      const rewritten = [...text.matchAll(/["'](@semio-tech\/[^"']+)["']/gu)].map(m => m[1]!);
      for (const spec of new Set(rewritten)) { const pkgName = spec.split("/").slice(0, 2).join("/"); if (spec.startsWith(PKG_NAME + "/") || spec.startsWith("@semio-tech/process/")) noteManifest(file, pkgName); }
    }
  }
}

const unresolved: string[] = [];
for (const r of results) {
  const specs = [...r.after.matchAll(/(?:from\s*|import\s*\(\s*|require\(\s*)["']([^"']+)["']/gu)].map(m => m[1]!);
  const beforeSpecs = new Set([...r.before.matchAll(/(?:from\s*|import\s*\(\s*|require\(\s*)["']([^"']+)["']/gu)].map(m => m[1]!));
  for (let s of specs) {
    if (beforeSpecs.has(s)) continue;
    if (s.startsWith("@semio-tech/") || s.startsWith("repo-vscode")) {
      s = s.split("?")[0]!;
      const parts = s.split("/");
      const scoped = s.startsWith("@");
      const name = parts.slice(0, scoped ? 2 : 1).join("/");
      const sub = "./" + parts.slice(scoped ? 2 : 1).join("/");
      const pkg = index.byName.get(name);
      const virtualOk = name === PKG_NAME ? sub in repoLibKeys : [...others.values()].some(o => o.pkg === name && o.subpath === sub);
      const liveOk = !!pkg && (sub in pkg.exports || (sub === "./" && "." in pkg.exports));
      if (!liveOk && !(mode === "dry" && virtualOk)) unresolved.push(`${r.file}: ${s}`);
      else if (write) {
        try { Bun.resolveSync(s, dirname(resolve(ROOT, r.file))); } catch { unresolved.push(`${r.file}: ${s} (Bun.resolveSync failed)`); }
      }
    } else if (s.startsWith(".")) {
      const target = resolve(dirname(resolve(ROOT, r.file)), s.split("?")[0]!);
      if (!existsSync(target) && !pendingGlue.has(target)) unresolved.push(`${r.file}: ${s} (relative target missing)`);
    }
  }
}

if (write) for (const r of results) {
  const current = readFileSync(resolve(ROOT, r.file), "utf8");
  if (current !== r.before) { console.log("SKIPPED (changed meanwhile):", r.file); continue; }
  writeFileSync(resolve(ROOT, r.file), r.after);
}

writeJson(resolve(GENERATED, "overlay.json"), Object.fromEntries(results.map(r => [r.file, r.after])));
const show = flags.find(f => f.startsWith("--show="))?.slice(7);
if (show) for (const r of results.filter(x => x.file.includes(show)).slice(0, 6)) {
  const a = resolve(GENERATED, "before.txt"), b = resolve(GENERATED, "after.txt");
  writeFileSync(a, r.before); writeFileSync(b, r.after);
  console.log("=== " + r.file);
  Bun.spawnSync(["diff", "-u", "--label", "before", "--label", "after", a, b], { stdout: "inherit" });
}
const manifests = Object.fromEntries([...summary.manifests].sort().map(([k, v]) => [k, [...v].sort()]));
writeJson(resolve(GENERATED, "apply-plan.json"), { mode, flags, summary: { ...summary, manifests: undefined, otherSkipped: summary.otherSkipped.slice(0, 400) }, changedFiles: results.map(r => r.file), unresolved });
writeJson(resolve(import.meta.dir, "manifests-needed.json"), { note: "dependencies the importers' nearest package.json should list once the freeze lifts (bare workspace specifiers)", manifests });
console.log(JSON.stringify({ mode, scannedImporters: summary.files, filesChanged: summary.changed, repoLibFiles: summary.repoLibFiles, frameworkNames: summary.a, deadImports: summary.dead, otherPackageEdges: summary.otherEdges, otherSkipped: summary.otherSkipped.length, openFixtureCouplings: summary.open.length, failures: summary.failures.length, unresolvedSpecifiers: unresolved.length, manifestsNeedingDependencies: Object.keys(manifests).length }, null, 1));
for (const f of summary.failures.slice(0, 10)) console.log("FAILURE", f);
for (const u of unresolved.slice(0, 10)) console.log("UNRESOLVED", u);
for (const o of summary.open) console.log("OPEN", o);
void PKG_DIR; void rel;
