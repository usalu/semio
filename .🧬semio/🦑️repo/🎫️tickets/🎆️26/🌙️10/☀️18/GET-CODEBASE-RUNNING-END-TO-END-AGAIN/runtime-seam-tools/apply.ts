import { spawnSync } from "node:child_process";
import { chmodSync, copyFileSync, existsSync, mkdirSync, readFileSync, readdirSync, renameSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { importSpecifier, transformSource, type ManualV1 } from "./codemod.ts";
import { checkRuntimeLawV1, scanRuntimeLawSourceV1 } from "./staged/new/🧰️framework/🔨️modules/⚙️runtime/⚖️law/🟦️.ts";

interface ManualEditV1 { readonly old: string; readonly new: string; readonly count?: number }
interface ManualFileV1 { readonly path: string; readonly before?: boolean; readonly edits: readonly ManualEditV1[]; readonly imports?: readonly string[] }

const here = dirname(fileURLToPath(import.meta.url));
const arguments_ = process.argv.slice(2);
const flag = (name: string): boolean => arguments_.includes(name);
const option = (name: string): string | undefined => { const at = arguments_.indexOf(name); return at < 0 ? undefined : arguments_[at + 1]; };
const root = resolve(option("--root") ?? join(here, "../../../../../../../.."));
const overlay = option("--overlay");
const write = flag("--write");
const FACADE = "🧰️framework/🔨️modules/⚙️runtime/🟦️.ts";
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

function mergeFacadeImports(text: string, specifier: string, extra: readonly string[], include: boolean): string {
  const pattern = new RegExp(`^import\\s*\\{([^}]*)\\}\\s*from\\s*"${specifier.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")}";[ \\t]*\\r?\\n?`, "gmu");
  const names = new Set<string>(include ? extra : []);
  let first = -1;
  const stripped = text.replace(pattern, (match, list: string, offset: number) => {
    for (const name of list.split(",").map(part => part.trim()).filter(Boolean)) names.add(name);
    if (first < 0) first = offset;
    return "";
  });
  if (names.size === 0) return text;
  const statement = `import { ${[...names].sort().join(", ")} } from "${specifier}";\n`;
  if (first >= 0) return stripped.slice(0, first) + statement + stripped.slice(first);
  const imports = [...stripped.matchAll(/^import [^\n]*;[ \t]*$/gmu)], last = imports.at(-1);
  if (last === undefined) return statement + stripped;
  const at = last.index! + last[0].length;
  return `${stripped.slice(0, at)}\n${statement.trimEnd()}${stripped.slice(at)}`;
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
const manualFiles: readonly ManualFileV1[] = existsSync(manualPath) ? JSON.parse(readFileSync(manualPath, "utf8")).files : [];
const manualByPath = new Map(manualFiles.map(row => [row.path, row]));
const finals: { path: string; source: string }[] = [];
const pending: (() => void)[] = [];
const report = { files: 0, changed: 0, edits: 0, manualApplied: 0, refused: [] as string[], remaining: [] as { path: string; findings: ManualV1[] }[], created: 0, identical: 0, changedPaths: [] as string[] };
const backup = join(here, "../🗑️generated/runtime-seam/backup");
const output = (path: string): string => (overlay === undefined ? join(root, path) : join(overlay, path));

for (const path of listCandidates()) {
  report.files++;
  const original = readFileSync(join(root, path), "utf8");
  const manual = manualByPath.get(path);
  let applied = 0;
  const edit = (input: string, rows: readonly ManualEditV1[]): string => {
    let value = input;
    for (const row of rows) {
      if (row.new.includes(row.old) && value.includes(row.new)) continue;
      const occurrences = value.split(row.old).length - 1;
      if (occurrences === 0) { if (!value.includes(row.new)) report.refused.push(`${path}: anchor missing: ${row.old.slice(0, 80)}`); continue; }
      if (occurrences !== (row.count ?? 1)) { report.refused.push(`${path}: anchor occurs ${occurrences}x, expected ${row.count ?? 1}: ${row.old.slice(0, 80)}`); continue; }
      value = value.split(row.old).join(row.new); applied++;
    }
    return value;
  };
  const prepared = manual?.before === true ? edit(original, manual.edits) : original;
  const frozen = path.includes("/🔏️authorizers/") || path.endsWith(".d.ts");
  const result = frozen ? { text: prepared, edits: 0, manual: [], used: [] as string[] } : transformSource(path, prepared, importSpecifier(path, FACADE));
  let text = manual?.before === true || manual === undefined ? result.text : edit(result.text, manual.edits);
  if (text !== result.text || result.used.length > 0 || (manual?.imports?.length ?? 0) > 0 && applied > 0) text = mergeFacadeImports(text, importSpecifier(path, FACADE), manual?.imports ?? [], applied > 0);
  finals.push({ path, source: text });
  const findings = scanRuntimeLawSourceV1(text), leftover = findings.length;
  if (text !== original) {
    report.changedPaths.push(path); report.changed++; report.edits += result.edits; report.manualApplied += applied;
    if (write || overlay !== undefined) pending.push(() => {
      if (write && overlay === undefined && !flag("--no-backup")) { const saved = join(backup, path); if (!existsSync(saved)) { mkdirSync(dirname(saved), { recursive: true }); copyFileSync(join(root, path), saved); } }
      place(output(path), text, statSync(join(root, path)).mode & 0o7777);
    });
  }
  if (leftover > 0) report.remaining.push({ path, findings: findings.map(row => ({ line: row.line, column: row.column, rule: row.rule, snippet: text.split("\n")[row.line - 1]!.trim().slice(0, 160) })) });
}

for (const source of walk(STAGED)) {
  const path = relative(STAGED, source), target = output(path), content = readFileSync(source);
  if (existsSync(target)) { if (readFileSync(target).equals(content)) { report.identical++; continue; } report.refused.push(`${path}: differs from staged content`); continue; }
  report.created++;
  if (write || overlay !== undefined) pending.push(() => place(target, content, 0o644));
}
if (report.refused.length === 0) for (const action of pending) action();
else if (write || overlay !== undefined) console.log(`NOTHING WRITTEN: ${report.refused.length} anchors refused`);

const ledgerPath = join(here, "src/ledger.json");
const law = existsSync(ledgerPath) ? checkRuntimeLawV1(finals, JSON.parse(readFileSync(ledgerPath, "utf8")).rows) : undefined;
if (law !== undefined) { console.log(JSON.stringify({ law: { scanned: law.scanned, findings: law.findings, violations: law.violations.length, stale: law.stale.length } })); for (const row of law.violations.slice(0, 30)) console.log("LAW-VIOLATION", row); for (const row of law.stale.slice(0, 30)) console.log("LAW-STALE", row); }
const file = option("--report");
if (file !== undefined) writeFileSync(file, JSON.stringify(report, null, 1));
console.log(JSON.stringify({ mode: overlay !== undefined ? "overlay" : write ? "write" : "dry-run", files: report.files, changed: report.changed, edits: report.edits, manualApplied: report.manualApplied, created: report.created, identical: report.identical, refused: report.refused.length, remainingFiles: report.remaining.length, remainingFindings: report.remaining.reduce((total, row) => total + row.findings.length, 0) }));
for (const row of report.refused.slice(0, 40)) console.log("REFUSED", row);
process.exitCode = report.refused.length === 0 && (law === undefined || law.violations.length + law.stale.length === 0) ? 0 : 1;
