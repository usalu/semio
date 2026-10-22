import { spawnSync } from "node:child_process";
import { chmodSync, copyFileSync, existsSync, mkdirSync, readFileSync, readdirSync, renameSync, rmSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { importSpecifier, transformSource, type ManualV1 } from "./codemod.ts";
import { checkRuntimeLawV1, isRuntimeAdapterPathV1, isRuntimeLawExemptPathV1, scanRuntimeLawSourceV1 } from "./staged/new/🧰️framework/🔨️modules/⚙️runtime/⚖️law/🟦️.ts";

interface ManualEditV1 { readonly old: string; readonly new: string; readonly count?: number }
interface ExtractionV1 { readonly start: string; readonly end: string; readonly to: string; readonly symbols: readonly string[]; readonly reexport?: boolean; readonly prefix?: string }
interface ManualFileV1 { readonly path: string; readonly before?: boolean; readonly edits: readonly ManualEditV1[]; readonly imports?: readonly string[]; readonly extract?: readonly ExtractionV1[] }
interface MoveV1 { readonly from: string; readonly to: string }

const here = dirname(fileURLToPath(import.meta.url));
const arguments_ = process.argv.slice(2);
const flag = (name: string): boolean => arguments_.includes(name);
const option = (name: string): string | undefined => { const at = arguments_.indexOf(name); return at < 0 ? undefined : arguments_[at + 1]; };
const root = resolve(option("--root") ?? join(here, "../../../../../../../.."));
const overlay = option("--overlay");
const write = flag("--write");
const V = "\ufe0f";
const FAMILIES: Readonly<Record<string, readonly string[]>> = {
  [`🏷${V}identity`]: ["runtimeIdentityV1", "runtimeExecutableV1", "isRuntimeExecutableV1", "runtimeKeyV1", "runtimeTypeEnvironmentV1", "runtimeSupportsV1", "runtimeSupportLevelV1", "isRuntimeBuiltinSpecifierV1", "runtimeCollectGarbageV1", "runtimeExecutableForV1", "runtimeBuiltinNamesV1", "runtimeDetectKeyV1", "runtimePackageCommandV1", "runtimeProgramV1", "isHostRuntimeCommandV1", "runtimeCommandNameV1", "runtimeCommandNamesV1", "runtimeAdapterNameV1", "runtimeIdsV1"],
  [`🏁${V}main`]: ["isMainModuleV1"],
  [`📄${V}parse`]: ["runtimeParseTomlV1", "runtimeParseJsoncV1", "runtimeParseYamlV1"],
  [`🌐${V}glob`]: ["GlobV1"],
  [`🚀${V}process`]: ["runtimeWhichV1", "runtimeResolveV1", "runtimeSleepV1", "runtimeSpawnV1", "runtimeSpawnSyncV1", "runtimeStdinTextV1", "runtimeStdinStreamV1", "runtimeTestArgumentsV1", "runtimeTestApiV1", "runtimeObserveSpawnV1", "runtimeTestPreloadV1"],
  [`🌍${V}serve`]: ["runtimeFileV1", "runtimeServeV1"],
  [`🗃${V}sqlite`]: ["SqliteDatabaseV1"],
  [`🧲${V}native`]: ["runtimeNativeSystemV1", "runtimeNativeOpenV1", "runtimeNativeReadV1", "runtimeNativePointerV1", "runtimeNativeCallbackV1"],
  [`🔬${V}source`]: ["TranspilerV1", "runtimeBundleV1", "runtimeLoaderPluginV1", "runtimeSourceUrlV1"],
  [`🔢${V}semver`]: ["runtimeSemverSatisfiesV1", "runtimeSemverOrderV1"],
  [`🗣${V}messages`]: ["runtimeMessageV1", "RuntimeCapabilityMissingV1"],
  [`🧪${V}test`]: ["test", "describe", "expect", "beforeAll", "afterAll", "beforeEach", "afterEach"],
};
const FAMILY_OF = new Map<string, string>(Object.entries(FAMILIES).flatMap(([directory, names]) => names.map(name => [name, directory] as const)));
const CONTRACT = `${MODULE_PATH()}/📋${V}contract/🟦${V}.ts`;
function MODULE_PATH(): string { return "🧰️framework/🔨️modules/⚙️runtime"; }
const specifierFor = (file: string, name: string): string => {
  const directory = FAMILY_OF.get(name);
  return importSpecifier(file, directory === undefined ? CONTRACT : `${MODULE_PATH()}/${directory}/🟦${V}.ts`);
};
const RUNTIME_IMPORT = /^import\s+(type\s+)?\{([^}]*)\}\s*from\s*"([^"]*\/⚙️runtime\/(?:[^"\/]*\/)?🟦️\.ts)";[ \t]*\r?\n?/gmu;
const MODULE = "🧰️framework/🔨️modules/⚙️runtime";
const STAGED = join(here, "staged/new");
const PATTERN = "Bun|bun:|\"bun\"|'bun'|versions\\.bun|import\\.meta\\.(dir|path|main)";
const SOURCE = /\.(?:[cm]?[jt]sx?)$/u;
const exempt = (path: string): boolean => path.startsWith(`${MODULE}/`) || path.split("/").some(segment => segment === "🧪️tests" || segment === "node_modules" || segment === "🗑️generated") || /\.(?:test|spec)\.[cm]?[jt]sx?$/u.test(path);

function git(args: string[]): string {
  const result = spawnSync("git", args, { cwd: root, encoding: "utf8", maxBuffer: 1 << 29 });
  if (result.status !== 0 && result.status !== 1) throw new Error(`git ${args[0]} failed: ${result.stderr}`);
  return result.stdout;
}

function listCandidates(): string[] {
  const listed = option("--candidates");
  if (listed !== undefined) return readFileSync(listed, "utf8").split("\n").filter(path => path !== "" && SOURCE.test(path) && !exempt(path) && existsSync(join(root, path))).sort();
  const tracked = git(["grep", "-l", "-z", "-I", "-E", PATTERN, "--", "🧰️framework"]).split("\0");
  const untracked = git(["ls-files", "-o", "--exclude-standard", "-z", "--", "🧰️framework"]).split("\0");
  return [...new Set([...tracked, ...untracked])].filter(path => path !== "" && SOURCE.test(path) && !exempt(path) && existsSync(join(root, path))).sort();
}

function normalizeRuntimeImports(text: string, file: string, extra: readonly string[]): string {
  const names = new Set<string>(extra), types = new Set<string>();
  let first = -1;
  const stripped = text.replace(RUNTIME_IMPORT, (_match, type: string | undefined, list: string, _specifier: string, offset: number) => {
    for (const part of list.split(",").map(item => item.trim()).filter(Boolean)) (type === undefined ? names : types).add(part);
    if (first < 0) first = offset;
    return "";
  });
  if (names.size === 0 && types.size === 0) return text;
  const groups = new Map<string, { values: string[]; types: string[] }>();
  const bucket = (specifier: string) => groups.get(specifier) ?? groups.set(specifier, { values: [], types: [] }).get(specifier)!;
  for (const part of names) { const base = part.split(/\s+as\s+/u)[0]!.trim(); if (FAMILY_OF.has(base)) bucket(specifierFor(file, base)).values.push(part); else bucket(specifierFor(file, base)).types.push(part); }
  for (const part of types) bucket(specifierFor(file, part.split(/\s+as\s+/u)[0]!.trim())).types.push(part);
  const lines: string[] = [];
  for (const [specifier, row] of [...groups].sort(([left], [right]) => (left < right ? -1 : 1))) {
    const values = [...new Set(row.values)].sort(), kinds = [...new Set(row.types)].filter(item => !values.includes(item)).sort();
    if (values.length > 0) lines.push(`import { ${[...values, ...kinds.map(item => `type ${item}`)].join(", ")} } from "${specifier}";`);
    else if (kinds.length > 0) lines.push(`import type { ${kinds.join(", ")} } from "${specifier}";`);
  }
  const block = lines.join("\n") + "\n";
  if (first >= 0) return stripped.slice(0, first) + block + stripped.slice(first);
  const imports = [...stripped.matchAll(/^import [^\n]*;[ \t]*$/gmu)], last = imports.at(-1);
  if (last === undefined) return block + stripped;
  const at = last.index! + last[0].length;
  return `${stripped.slice(0, at)}\n${block.trimEnd()}${stripped.slice(at)}`;
}

function walk(directory: string): string[] {
  return readdirSync(directory, { withFileTypes: true }).flatMap(entry => (entry.isDirectory() ? walk(join(directory, entry.name)) : [join(directory, entry.name)]));
}

function place(target: string, text: string | Buffer, mode?: number): void {
  mkdirSync(dirname(target), { recursive: true });
  const temporary = `${target}.runtime-seam.tmp`;
  writeFileSync(temporary, text);
  if (mode !== undefined) chmodSync(temporary, mode);
  renameSync(temporary, target);
}

const manualPath = join(here, "manual-edits.json");
const manualDocument = existsSync(manualPath) ? JSON.parse(readFileSync(manualPath, "utf8")) : {};
const manualFiles: readonly ManualFileV1[] = manualDocument.files ?? [];
const moves: readonly MoveV1[] = manualDocument.moves ?? [];
const deletes: readonly string[] = manualDocument.deletes ?? [];
const manualByPath = new Map(manualFiles.map(row => [row.path, row]));
const moveTarget = new Map(moves.map(row => [row.from, row.to]));
const finals: { path: string; source: string }[] = [];
const pending: (() => void)[] = [];
const report = { files: 0, changed: 0, edits: 0, manualApplied: 0, refused: [] as string[], remaining: [] as { path: string; findings: ManualV1[] }[], created: 0, identical: 0, moved: 0, deleted: 0, extracted: 0, changedPaths: [] as string[] };
const backup = join(here, "../🗑️generated/runtime-seam/backup");
const output = (path: string): string => (overlay === undefined ? join(root, path) : join(overlay, path));
const saveBackup = (path: string): void => { if (write && overlay === undefined && !flag("--no-backup")) { const saved = join(backup, path); if (!existsSync(saved) && existsSync(join(root, path))) { mkdirSync(dirname(saved), { recursive: true }); copyFileSync(join(root, path), saved); } } };
const REBASE = /(\bfrom\s*|\bimport\s*\(\s*|\brequire\s*\(\s*|\bnew URL\(\s*|\bimport\s+)(["'])(\.{1,2}\/[^"'\n]*)\2/gu;

function rebase(text: string, from: string, to: string): string {
  return text.replace(REBASE, (_match, lead: string, quote: string, specifier: string) => {
    const target = resolve(dirname(join(root, from)), specifier), next = relative(dirname(join(root, to)), target).split(/[\\/]/u).join("/");
    return `${lead}${quote}${next.startsWith(".") ? next : `./${next}`}${quote}`;
  });
}

function pipeline(path: string, original: string): { text: string; edits: number; applied: number; created: { path: string; text: string }[] } {
  const manual = manualByPath.get(path);
  const moduleOf = (extraction: ExtractionV1): string => { const specifier = relative(dirname(join(root, path)), join(root, extraction.to)).split(/[\\/]/u).join("/"); return specifier.startsWith(".") ? specifier : `./${specifier}`; };
  const settled = (manual?.extract?.length ?? 0) > 0 && manual!.extract!.every(extraction => original.includes(`from "${moduleOf(extraction)}"`));
  let applied = 0;
  const edit = (input: string, rows: readonly ManualEditV1[]): string => {
    let value = input;
    for (const row of rows) {
      if (row.new.includes(row.old) && value.includes(row.new)) continue;
      const occurrences = value.split(row.old).length - 1;
      if (occurrences === 0) { if (!value.includes(row.new) && !settled) report.refused.push(`${path}: anchor missing: ${row.old.slice(0, 80)}`); continue; }
      if (occurrences !== (row.count ?? 1)) { report.refused.push(`${path}: anchor occurs ${occurrences}x, expected ${row.count ?? 1}: ${row.old.slice(0, 80)}`); continue; }
      value = value.split(row.old).join(row.new); applied++;
    }
    return value;
  };
  let text = manual?.before === true ? edit(original, manual.edits) : original, edits = 0, used = 0;
  if (SOURCE.test(path) && !path.endsWith(".d.ts") && !exempt(path)) {
    for (let pass = 0; pass < 3; pass++) { const result = transformSource(path, text, name => specifierFor(path, name)); edits += result.edits; used += result.used.length; if (result.text === text) break; text = result.text; }
  }
  if (manual !== undefined && manual.before !== true) text = edit(text, manual.edits);
  const created: { path: string; text: string }[] = [];
  for (const extraction of manual?.extract ?? []) {
    const module = moduleOf(extraction);
    const start = text.indexOf(extraction.start);
    if (start < 0) { if (!text.includes(`from "${module}"`)) report.refused.push(`${path}: extraction start missing: ${extraction.start.slice(0, 80)}`); continue; }
    const stop = text.indexOf(extraction.end, start + extraction.start.length);
    if (stop < 0) { report.refused.push(`${path}: extraction end missing: ${extraction.end.slice(0, 80)}`); continue; }
    const region = text.slice(start, stop + extraction.end.length), rest = text.slice(0, start) + text.slice(stop + extraction.end.length);
    const usedHere = extraction.symbols.some(symbol => new RegExp(`(?<![\\w$])${symbol}(?![\\w$])`, "u").test(rest));
    const statement = (extraction.reexport === true ? `export { ${extraction.symbols.join(", ")} } from "${module}";\n` : "") + (extraction.reexport !== true || usedHere ? `import { ${extraction.symbols.join(", ")} } from "${module}";\n` : "");
    text = text.slice(0, start) + statement + text.slice(stop + extraction.end.length);
    created.push({ path: extraction.to, text: `${extraction.prefix ?? ""}${/^(?:\/\*\*[\s\S]*?\*\/\s*)?export\s/u.test(region) ? region : `export ${region}`}\n` });
    report.extracted++;
    applied++;
  }
  if (text !== original || used > 0 || (manual?.imports?.length ?? 0) > 0 && applied > 0) text = normalizeRuntimeImports(text, path, applied > 0 ? manual?.imports ?? [] : []);
  return { text, edits, applied, created };
}

function process_(path: string): void {
  const original = readFileSync(join(root, path), "utf8");
  const result = pipeline(path, original);
  const destination = moveTarget.get(path) ?? path;
  const text = destination === path ? result.text : rebase(result.text, path, destination);
  finals.push({ path: destination, source: text });
  for (const row of result.created) { finals.push({ path: row.path, source: row.text }); pending.push(() => place(output(row.path), row.text, 0o644)); report.created++; }
  const findings = scanRuntimeLawSourceV1(text), leftover = isRuntimeAdapterPathV1(destination) ? 0 : findings.length;
  if (text !== original || destination !== path) {
    report.changedPaths.push(path); report.changed++; report.edits += result.edits; report.manualApplied += result.applied;
    pending.push(() => {
      saveBackup(path);
      place(output(destination), text, statSync(join(root, path)).mode & 0o7777);
      if (destination !== path && overlay === undefined && write) rmSync(join(root, path));
    });
    if (destination !== path) report.moved++;
  }
  if (leftover > 0) report.remaining.push({ path: destination, findings: findings.map(row => ({ line: row.line, column: row.column, rule: row.rule, snippet: text.split("\n")[row.line - 1]!.trim().slice(0, 160) })) });
}

const candidates = new Set(listCandidates());
for (const row of manualFiles) if (existsSync(join(root, row.path))) candidates.add(row.path);
for (const row of moves) if (existsSync(join(root, row.from))) candidates.add(row.from);
for (const path of [...candidates].sort()) { report.files++; process_(path); }
for (const path of deletes) {
  if (!existsSync(join(root, path))) continue;
  report.deleted++;
  pending.push(() => { saveBackup(path); if (overlay !== undefined) rmSync(output(path), { force: true }); else if (write) rmSync(join(root, path)); });
}

for (const source of walk(STAGED)) {
  const path = relative(STAGED, source), target = output(path), content = readFileSync(source);
  if (existsSync(target)) { if (readFileSync(target).equals(content)) { report.identical++; continue; } report.refused.push(`${path}: differs from staged content`); continue; }
  report.created++;
  if (write || overlay !== undefined) pending.push(() => place(target, content, 0o644));
}
if (report.refused.length === 0 && (write || overlay !== undefined)) for (const action of pending) action();
else if (write || overlay !== undefined) console.log(`NOTHING WRITTEN: ${report.refused.length} anchors refused`);

const law = checkRuntimeLawV1(finals.filter(row => SOURCE.test(row.path) && !isRuntimeLawExemptPathV1(row.path)));
console.log(JSON.stringify({ law: { scanned: law.scanned, findings: law.findings, violations: law.violations.length } })); for (const row of law.violations.slice(0, 400)) console.log("LAW-VIOLATION", row);
const file = option("--report");
if (file !== undefined) writeFileSync(file, JSON.stringify(report, null, 1));
console.log(JSON.stringify({ mode: overlay !== undefined ? "overlay" : write ? "write" : "dry-run", files: report.files, changed: report.changed, edits: report.edits, manualApplied: report.manualApplied, created: report.created, moved: report.moved, deleted: report.deleted, extracted: report.extracted, identical: report.identical, refused: report.refused.length, remainingFiles: report.remaining.length, remainingFindings: report.remaining.reduce((total, row) => total + row.findings.length, 0) }));
for (const row of report.refused.slice(0, 60)) console.log("REFUSED", row);
process.exitCode = report.refused.length === 0 && law.violations.length === 0 ? 0 : 1;
