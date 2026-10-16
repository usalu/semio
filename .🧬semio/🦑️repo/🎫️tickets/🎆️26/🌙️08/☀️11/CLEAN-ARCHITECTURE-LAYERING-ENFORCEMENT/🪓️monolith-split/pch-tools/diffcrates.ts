// 🔬️ Resolution differential for a dedupe: the consumers of the working tree plus their new owner crates, mounted as modules of one synthetic crate, against the consumers of a git base as they were (owner sources mounted by path). Zero onlyCombined diagnostics means the move introduced no unresolved name. Sandboxes live outside the repository.
// usage: bun diffcrates.ts <spec.json> <sandbox-dir>
// spec: { baseRef, pluginDirs: [repo-relative dirs to copy], consumers: [{ident, lib}], owners: [{ident, lib}] } — every lib is a repo-relative lib root file; ident is the extern crate identifier the consumers use for it.
import {existsSync, mkdirSync, readFileSync, rmSync, writeFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {dirname, join, resolve} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const C = join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree");
const {buildSandbox, patchSandboxSources, runResolutionOracle, diffDiagnostics, resolveCrateModules} = await import(join(C, "🔬️verify/🟦️.ts"));
const {rewriteForCombined, external, normalizeCombined, mountedOutsideDirectories} = await import(join(C, "🧫️sandbox/🟦️.ts"));
type Crate = {ident: string; lib: string; existing?: boolean};
const spec = JSON.parse(readFileSync(process.argv[2]!, "utf8")) as {baseRef: string; pluginDirs: string[]; consumers: Crate[]; owners: Crate[]; extraFlags?: string[]};
const sandboxRoot = resolve(process.argv[3]!);
if (sandboxRoot.startsWith(ROOT)) throw new Error("sandbox must be outside the repository");
const filesOf = (root: string, lib: string): string[] => [...new Set([lib, ...resolveCrateModules(root, join(root, lib), file => existsSync(file) ? readFileSync(file, "utf8") : "").flatMap(m => m.file !== null && m.file !== "unresolved" ? [m.file] : [])])];
const measureAll = async (crates: Crate[]) => {
  const result = new Map<string, string[]>();
  const outside = new Set<string>();
  for (const crate of crates) {
    const files = filesOf(ROOT, crate.lib);
    result.set(crate.ident, files);
    for (const path of files) if (!spec.pluginDirs.some(dir => path.startsWith(`${dir}/`))) outside.add(path);
  }
  return {result, outside};
};
const rewrite = (text: string, module: string, map: ReadonlyMap<string, string>): string => {
  let t = rewriteForCombined(text, module, map);
  for (const [ident, mod] of map) t = t.replace(new RegExp(`(\\buse\\s+)${ident}(\\s+as\\b)`, "gu"), `$1crate::${mod}$2`);
  return t.replace(/(\buse\s+)crate(\s+as\b)/gu, `$1crate::${module}$2`);
};
const all = [...spec.consumers, ...spec.owners];
const {result: measured, outside} = await measureAll(all);
const directories = [...new Set([...spec.pluginDirs, ...[...outside].map(p => dirname(p)).filter(d => existsSync(join(ROOT, d))), ...spec.pluginDirs.flatMap(d => [...mountedOutsideDirectories(ROOT, d)])])];
const prefixes = ["semio_", "serde", "protocol::", "store::", "dsl::", "pack::", "value_derive::", "framework_schema::", "tokio"];
const combined = `${sandboxRoot}-combined`, base = `${sandboxRoot}-base`;
buildSandbox(ROOT, combined, directories);
const moduleOf = new Map(all.map(c => [c.ident, `__c_${c.ident}`]));
const owned = new Set<string>();
const mounts: string[] = [];
for (const crate of all) {
  const mod = moduleOf.get(crate.ident)!;
  mounts.push(`#[path = ${JSON.stringify(crate.lib)}]\npub mod ${mod};`);
  for (const path of measured.get(crate.ident)!) {
    if (!path.endsWith(".rs") || owned.has(path)) continue;
    owned.add(path);
    const file = join(combined, path);
    if (existsSync(file)) writeFileSync(file, rewrite(readFileSync(file, "utf8"), mod, moduleOf));
  }
}
writeFileSync(join(combined, "__combined.rs"), mounts.join("\n") + "\n");
patchSandboxSources(combined, prefixes);
buildSandbox(ROOT, base, directories);
mkdirSync(base, {recursive: true});
for (const dir of spec.pluginDirs) execFileSync("sh", ["-c", `cd '${ROOT}' && git -c core.quotepath=false archive ${spec.baseRef} '${dir}' | tar -x -C '${base}'`]);
const baseMounts: string[] = [];
const baseCrates = [...spec.consumers, ...spec.owners.filter(o => o.existing)];
const baseModules = new Map(baseCrates.map(c => [c.ident, `__c_${c.ident}`]));
const baseMeasured = new Map(baseCrates.map(c => [c.ident, filesOf(base, c.lib)]));
const consumerFiles = new Set(spec.consumers.flatMap(c => baseMeasured.get(c.ident)!));
const seen = new Set<string>();
for (const crate of baseCrates) {
  const isOwner = !spec.consumers.includes(crate);
  const files = baseMeasured.get(crate.ident)!;
  if (isOwner && files.some(f => consumerFiles.has(f))) { console.error(`base: existing owner ${crate.ident} overlaps a consumer file; left out of the base`); continue; }
  const mod = baseModules.get(crate.ident)!;
  baseMounts.push(`#[path = ${JSON.stringify(crate.lib)}]\npub mod ${mod};`);
  for (const path of files) {
    if (!path.endsWith(".rs") || seen.has(path)) continue;
    seen.add(path);
    const file = join(base, path);
    if (existsSync(file)) writeFileSync(file, rewrite(readFileSync(file, "utf8"), mod, baseModules));
  }
}
writeFileSync(join(base, "__combined.rs"), baseMounts.join("\n") + "\n");
patchSandboxSources(base, prefixes);
const report: Record<string, unknown> = {baseRef: spec.baseRef, consumers: spec.consumers.map(c => c.ident), owners: spec.owners.map(o => o.ident)};
let failed = false;
for (const flags of [[], ["--test", ...(spec.extraFlags ?? [])]] as string[][]) {
  const baseline = runResolutionOracle(base, join(base, "__combined.rs"), "combined", flags);
  const after = runResolutionOracle(combined, join(combined, "__combined.rs"), "combined", flags);
  const strip = (list: string[]): string[] => [...new Set(list.filter(external).map(normalizeCombined).map(line => line.split("\t").slice(0, 2).join("\t")))].filter(line => !line.includes("aborting due to"));
  const delta = diffDiagnostics(strip(baseline.diagnostics), strip(after.diagnostics));
  const key = flags.includes("--test") ? "--test" : "native";
  report[key] = {base: baseline.diagnostics.length, combined: after.diagnostics.length, onlyCombined: delta.onlyCandidate.length, onlyBase: delta.onlyOriginal.length, sampleBase: delta.onlyOriginal.slice(0, 6), sample: delta.onlyCandidate.slice(0, 12).map((line: string) => { const at = after.diagnostics.findIndex((entry: string) => strip([entry])[0] === line); return at >= 0 ? `${line}  @ ${(after.locations[at] ?? "").replace(combined + "/", "")}` : line; })};
  if (delta.onlyCandidate.length) failed = true;
}
console.log(JSON.stringify(report, null, 1));
if (!process.argv.includes("--keep")) { rmSync(combined, {recursive: true, force: true}); rmSync(base, {recursive: true, force: true}); }
process.exit(failed ? 1 : 0);
