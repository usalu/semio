import { readFileSync } from "node:fs";
import { join } from "node:path";
import { isMainModuleV1, runtimeSpawnSyncV1 } from "../🟦️.ts";

/** ⚖️ Names the rule a finding breaks: the global `Bun`, a `bun` module specifier, `process.versions.bun`, a Bun-only `import.meta` member, or the same inside an embedded template payload. */
export type RuntimeLawRuleV1 = "Bun" | "specifier" | "process.versions.bun" | "import.meta" | "embedded";
/** 🔍️ One place where code names a specific runtime. */
export interface RuntimeLawFindingV1 { readonly line: number; readonly column: number; readonly rule: RuntimeLawRuleV1; readonly text: string }
/** 📒️ One ledgered file: a finding class tolerated for a stated reason until its owner removes it. */
export interface RuntimeLawLedgerRowV1 { readonly path: string; readonly rule: RuntimeLawRuleV1; readonly count: number; readonly class: string; readonly owner: string; readonly reason: string }
/** 📋️ The outcome of one law run. */
export interface RuntimeLawReportV1 { readonly scanned: number; readonly findings: number; readonly violations: string[]; readonly stale: string[] }

const SPECIFIER = /^bun(?::[a-z]*)?$/u;
const CODE_RULES: readonly (readonly [RuntimeLawRuleV1, RegExp])[] = [
  ["Bun", /(?<![\w$])Bun(?![\w$])/gu],
  ["process.versions.bun", /process\s*\.\s*versions\s*\.\s*bun(?![\w$])/gu],
  ["import.meta", /import\s*\.\s*meta\s*\.\s*(?:dir|path|main)(?![\w$])/gu],
];
const EMBEDDED_RULES: readonly RegExp[] = [/(?<![\w$])Bun\.[A-Za-z_$]/gu, /process\s*\.\s*versions\s*\.\s*bun/gu, /import\s*\.\s*meta\s*\.\s*(?:dir|path|main)(?![\w$])/gu, /["'`]bun:[a-z]+["'`]/gu];
const REGEX_PREVIOUS = /[(,=:[!&|?{};+\-*%<>~^]$/u;
const REGEX_KEYWORDS = /(?:^|[^\w$])(?:return|typeof|case|do|else|in|of|void|delete|throw|new|yield|await)$/u;

/** 🔬️ Masks comments and string contents of one source so rules see code only, and collects template bodies for the embedded rule. */
function mask(source: string): { readonly code: string; readonly specifiers: { index: number; value: string }[]; readonly templates: { index: number; body: string }[] } {
  const output = source.split("");
  const specifiers: { index: number; value: string }[] = [], templates: { index: number; body: string }[] = [];
  const blank = (from: number, to: number): void => { for (let at = from; at < to; at++) if (output[at] !== "\n" && output[at] !== "\r") output[at] = " "; };
  let index = 0;
  const scanCode = (stopAtBrace: boolean): void => {
    let depth = 0, previous = "";
    while (index < source.length) {
      const character = source[index]!, next = source[index + 1];
      if (character === "/" && next === "/") { const end = source.indexOf("\n", index); const stop = end < 0 ? source.length : end; blank(index, stop); index = stop; continue; }
      if (character === "/" && next === "*") { const end = source.indexOf("*/", index + 2); const stop = end < 0 ? source.length : end + 2; blank(index, stop); index = stop; continue; }
      if (character === '"' || character === "'") {
        const start = index++;
        while (index < source.length && source[index] !== character && source[index] !== "\n") { if (source[index] === "\\") index++; index++; }
        const value = source.slice(start + 1, index);
        if (SPECIFIER.test(value)) specifiers.push({ index: start, value });
        blank(start + 1, index); index++; previous = "x"; continue;
      }
      if (character === "`") {
        const start = index++;
        let bodyStart = index;
        const parts: string[] = [];
        while (index < source.length && source[index] !== "`") {
          if (source[index] === "\\") { index += 2; continue; }
          if (source[index] === "$" && source[index + 1] === "{") {
            parts.push(source.slice(bodyStart, index)); blank(bodyStart, index);
            index += 2; scanCode(true); bodyStart = index; continue;
          }
          index++;
        }
        parts.push(source.slice(bodyStart, index)); blank(bodyStart, index);
        templates.push({ index: start, body: source.slice(start + 1, index) });
        index++; previous = "x"; continue;
      }
      if (character === "/" && (previous === "" || REGEX_PREVIOUS.test(previous) || REGEX_KEYWORDS.test(previous))) {
        const start = index++;
        let inClass = false;
        while (index < source.length && source[index] !== "\n") { const c = source[index]!; if (c === "\\") { index += 2; continue; } if (c === "[") inClass = true; else if (c === "]") inClass = false; else if (c === "/" && !inClass) break; index++; }
        blank(start + 1, index); index++; previous = "x"; continue;
      }
      if (stopAtBrace) { if (character === "{") depth++; else if (character === "}") { if (depth === 0) { index++; return; } depth--; } }
      if (!/\s/u.test(character)) previous = (previous + character).slice(-24);
      index++;
    }
  };
  scanCode(false);
  return { code: output.join(""), specifiers, templates };
}

/** 🔍️ Finds every place one source names a specific runtime. */
export function scanRuntimeLawSourceV1(source: string): RuntimeLawFindingV1[] {
  const { code, specifiers, templates } = mask(source), starts: number[] = [0];
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

/** 🏠️ Tells whether a path is inside the one Bun adapter directory, the only place any rule may be broken. */
export function isRuntimeAdapterPathV1(path: string): boolean { return path.includes("/⚙️runtime/🍞️bun/") || path.startsWith("🧰️framework/🔨️modules/⚙️runtime/🍞️bun/"); }
/** 🧩️ Tells whether a path belongs to the runtime seam itself, where only the registry's runtime names may appear as strings. */
export function isRuntimeSeamPathV1(path: string): boolean { return path.includes("/⚙️runtime/") || path.startsWith("🧰️framework/🔨️modules/⚙️runtime/"); }
/** 🧪️ Tells whether a path is test code, which the law does not cover. */
export function isRuntimeLawExemptPathV1(path: string): boolean { return path.split("/").some(segment => segment === "🧪️tests" || segment === "node_modules" || segment === "🗑️generated") || /\.(?:test|spec)\.[cm]?[jt]sx?$/u.test(path); }

const SOURCE_FILE = /\.(?:[cm]?[jt]sx?)$/u;

/** 📋️ Lists the git-visible source files below one root. */
export function listRuntimeLawFilesV1(repositoryRoot: string, area: string): string[] {
  const result = runtimeSpawnSyncV1(["git", "ls-files", "-co", "--exclude-standard", "-z", "--", area], { cwd: repositoryRoot });
  if (result.exitCode !== 0) throw new Error(`git ls-files failed: ${result.stderr.toString()}`);
  return result.stdout.toString().split("\0").filter(path => path !== "" && SOURCE_FILE.test(path) && !isRuntimeLawExemptPathV1(path));
}

/** ⚖️ Checks files against the law and the ledger: unledgered findings are violations, ledger rows without findings are stale. */
export function checkRuntimeLawV1(files: readonly { readonly path: string; readonly source: string }[], ledger: readonly RuntimeLawLedgerRowV1[]): RuntimeLawReportV1 {
  const violations: string[] = [], stale: string[] = [], tolerated = new Map<string, RuntimeLawLedgerRowV1>();
  for (const row of ledger) tolerated.set(`${row.path}\0${row.rule}`, row);
  const actual = new Map<string, number>();
  let total = 0;
  for (const file of files) {
    if (isRuntimeAdapterPathV1(file.path)) continue;
    const seam = isRuntimeSeamPathV1(file.path);
    for (const finding of scanRuntimeLawSourceV1(file.source)) {
      if (seam && finding.rule === "specifier") continue;
      total++;
      const key = `${file.path}\0${finding.rule}`, count = (actual.get(key) ?? 0) + 1;
      actual.set(key, count);
      const row = tolerated.get(key);
      if (row === undefined || count > row.count) violations.push(`${file.path}:${finding.line}:${finding.column} ${finding.rule} ${finding.text}`);
    }
  }
  for (const [key, row] of tolerated) if ((actual.get(key) ?? 0) < row.count) stale.push(`${row.path} ${row.rule} ledger ${row.count} actual ${actual.get(key) ?? 0}`);
  return { scanned: files.length, findings: total, violations, stale };
}

/** 📒️ Builds ledger rows from current findings so a reviewer can classify them; classes default to `pending`. */
export function draftRuntimeLawLedgerV1(files: readonly { readonly path: string; readonly source: string }[]): RuntimeLawLedgerRowV1[] {
  const rows = new Map<string, { path: string; rule: RuntimeLawRuleV1; count: number }>();
  for (const file of files) {
    if (isRuntimeAdapterPathV1(file.path)) continue;
    const seam = isRuntimeSeamPathV1(file.path);
    for (const finding of scanRuntimeLawSourceV1(file.source)) { if (seam && finding.rule === "specifier") continue; const key = `${file.path}\0${finding.rule}`, row = rows.get(key) ?? { path: file.path, rule: finding.rule, count: 0 }; row.count++; rows.set(key, row); }
  }
  return [...rows.values()].sort((a, b) => (a.path < b.path ? -1 : a.path > b.path ? 1 : a.rule < b.rule ? -1 : 1)).map(row => ({ ...row, class: "pending", owner: "runtime-seam", reason: "unclassified" }));
}

if (isMainModuleV1(import.meta.url)) {
  const root = process.argv[2] ?? process.cwd(), area = process.argv[3] ?? "🧰️framework";
  const files = listRuntimeLawFilesV1(root, area).map(path => ({ path, source: readFileSync(join(root, path), "utf8") }));
  const ledgerPath = process.argv[4];
  const ledger = ledgerPath === undefined ? [] : (JSON.parse(readFileSync(ledgerPath, "utf8")).rows as RuntimeLawLedgerRowV1[]);
  const report = checkRuntimeLawV1(files, ledger);
  console.log(JSON.stringify({ scanned: report.scanned, findings: report.findings, violations: report.violations.length, stale: report.stale.length }));
  for (const row of report.violations.slice(0, 50)) console.log("VIOLATION", row);
  for (const row of report.stale.slice(0, 50)) console.log("STALE", row);
  process.exitCode = report.violations.length + report.stale.length === 0 ? 0 : 1;
}
