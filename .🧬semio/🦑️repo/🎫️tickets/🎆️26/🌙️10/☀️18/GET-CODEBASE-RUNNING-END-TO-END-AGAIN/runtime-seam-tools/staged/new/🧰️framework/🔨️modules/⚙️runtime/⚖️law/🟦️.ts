import { readFileSync } from "node:fs";
import { join } from "node:path";
import { isMainModuleV1 } from "../🏁️main/🟦️.ts";
import { runtimeMessageV1 } from "../🗣️messages/🟦️.ts";
import { runtimeSpawnSyncV1 } from "../🚀️process/🟦️.ts";
import { maskSourceV1 } from "../🪶️portable/🔬️source/🟦️.ts";

/** ⚖️ Names the rule a finding breaks: the global `Bun`, a `bun` module specifier or program word, the string `Bun` (as in `"Bun" in globalThis`), `process.versions.bun`, a Bun-only `import.meta` member, or the same inside an embedded template payload. */
export type RuntimeLawRuleV1 = "Bun" | "specifier" | "process.versions.bun" | "import.meta" | "embedded";
/** 🔍️ One place where code names a specific runtime. */
export interface RuntimeLawFindingV1 { readonly line: number; readonly column: number; readonly rule: RuntimeLawRuleV1; readonly text: string }
/** 📋️ The outcome of one law run. */
export interface RuntimeLawReportV1 { readonly scanned: number; readonly findings: number; readonly violations: string[] }

const SPECIFIER = /^(?:bun(?::[a-z]*)?|Bun)$/u;
const CODE_RULES: readonly (readonly [RuntimeLawRuleV1, RegExp])[] = [
  ["Bun", /(?<![\w$])Bun(?![\w$])/gu],
  ["process.versions.bun", /(?<![\w$])versions\s*\??\.\s*bun(?![\w$])/gu],
  ["import.meta", /import\s*\.\s*meta\s*\.\s*(?:dir|path|main)(?![\w$])/gu],
];
const EMBEDDED_RULES: readonly RegExp[] = [/(?<![\w$])Bun\.[A-Za-z_$]/gu, /(?<![\w$])versions\s*\??\.\s*bun(?![\w$])/gu, /import\s*\.\s*meta\s*\.\s*(?:dir|path|main)(?![\w$])/gu, /["'`]bun:[a-z]+["'`]/gu];
/** 🔍️ Finds every place one source names a specific runtime. */
export function scanRuntimeLawSourceV1(source: string): RuntimeLawFindingV1[] {
  const masked = maskSourceV1(source), code = masked.code, specifiers = masked.strings.filter(row => SPECIFIER.test(row.value)).map(row => ({ index: row.start, value: row.value })), templates = masked.templates.map(row => ({ index: row.start, body: row.body })), starts: number[] = [0];
  for (let at = 0; at < source.length; at++) if (source[at] === "\n") starts.push(at + 1);
  const where = (index: number): { line: number; column: number } => { let low = 0, high = starts.length - 1; while (low < high) { const middle = (low + high + 1) >> 1; if (starts[middle]! <= index) low = middle; else high = middle - 1; } return { line: low + 1, column: index - starts[low]! + 1 }; };
  const findings: RuntimeLawFindingV1[] = [];
  for (const [rule, expression] of CODE_RULES) for (const match of code.matchAll(expression)) findings.push({ ...where(match.index!), rule, text: match[0] });
  for (const row of specifiers) findings.push({ ...where(row.index), rule: "specifier", text: row.value });
  for (const template of templates) {
    const seen = new Set<string>();
    for (const expression of EMBEDDED_RULES) for (const match of template.body.matchAll(expression)) { const key = String(match.index); if (!seen.has(key)) { seen.add(key); findings.push({ ...where(template.index), rule: "embedded", text: match[0] }); } }
  }
  return findings.sort((a, b) => a.line - b.line || a.column - b.column);
}

/** 🏠️ Tells whether a path is inside a runtime adapter directory: a directory named `🍞️bun` holds code written for that runtime and is the only place the rules may be broken. */
export function isRuntimeAdapterPathV1(path: string): boolean { return path.split("/").includes("🍞️bun"); }
/** 🧩️ Tells whether a path belongs to the runtime seam itself, where only the registry's runtime names may appear as strings. */
export function isRuntimeSeamPathV1(path: string): boolean { return path.includes("/⚙️runtime/") || path.startsWith("🧰️framework/🔨️modules/⚙️runtime/"); }
/** 🧪️ Tells whether a path is test code, which the law does not cover. */
export function isRuntimeLawExemptPathV1(path: string): boolean { return path.split("/").some(segment => segment === "🧪️tests" || segment === "node_modules" || segment === "🗑️generated") || /\.(?:test|spec)\.[cm]?[jt]sx?$/u.test(path); }

const SOURCE_FILE = /\.(?:[cm]?[jt]sx?)$/u;

/** 📋️ Lists the git-visible source files below one root. */
export function listRuntimeLawFilesV1(repositoryRoot: string, area: string): string[] {
  const result = runtimeSpawnSyncV1(["git", "ls-files", "-co", "--exclude-standard", "-z", "--", area], { cwd: repositoryRoot });
  if (result.exitCode !== 0) throw new Error(runtimeMessageV1("lawListFailed", { detail: result.stderr.toString() }));
  return result.stdout.toString().split("\0").filter(path => path !== "" && SOURCE_FILE.test(path) && !isRuntimeLawExemptPathV1(path));
}

/** ⚖️ Checks files against the law: every finding outside the Bun adapter is a violation, and there is no list of tolerated exceptions. */
export function checkRuntimeLawV1(files: readonly { readonly path: string; readonly source: string }[]): RuntimeLawReportV1 {
  const violations: string[] = [];
  let total = 0;
  for (const file of files) {
    if (isRuntimeAdapterPathV1(file.path)) continue;
    const seam = isRuntimeSeamPathV1(file.path);
    for (const finding of scanRuntimeLawSourceV1(file.source)) {
      if (seam && finding.rule === "specifier") continue;
      total++;
      violations.push(`${file.path}:${finding.line}:${finding.column} ${finding.rule} ${finding.text}`);
    }
  }
  return { scanned: files.length, findings: total, violations };
}

if (isMainModuleV1(import.meta.url)) {
  const root = process.argv[2] ?? process.cwd(), area = process.argv[3] ?? "🧰️framework";
  const files = listRuntimeLawFilesV1(root, area).map(path => ({ path, source: readFileSync(join(root, path), "utf8") }));
  const report = checkRuntimeLawV1(files);
  console.log(JSON.stringify({ scanned: report.scanned, findings: report.findings, violations: report.violations.length }));
  for (const row of report.violations.slice(0, 50)) console.log("VIOLATION", row);
  process.exitCode = report.violations.length === 0 ? 0 : 1;
}
