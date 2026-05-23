import { existsSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";

//#region Types
export type MicroCommitLangMetrics = {
  lang: string;
  emoji: string;
  code: number;
  edited: number;
  added: number;
  removed: number;
};

export type UlocByLanguage = Record<string, number>;

/** 📊Repo-wide unified LOC per language (tracked git files; JSON uses key count). */
export type UlocRunner = {
  countRepoByLanguage(root: string): UlocByLanguage;
};
//#endregion Types

//#region Constants
const METRICS_LOCK_FILES = new Set([
  "package-lock.json",
  "yarn.lock",
  "pnpm-lock.yaml",
  "go.sum",
  "uv.lock",
  "bun.lockb",
  "cargo.lock",
]);

const METRICS_LICENSE_TEMPLATE_BASENAMES = new Set([
  "LICENSE",
  "LICENSE.md",
  "LICENSE.txt",
  "COPYING",
  "COPYING.md",
  "NOTICE",
  "NOTICE.md",
  "UNLICENSE",
  "UNLICENSE.md",
]);

const LANG_EMOJI: Record<string, string> = {
  TypeScript: "🟦",
  JavaScript: "🟨",
  Go: "🔵",
  "C#": "🟣",
  Python: "🐍",
  Rust: "🦀",
  Shell: "🐚",
  Dockerfile: "🐳",
  Makefile: "🔧",
  CSS: "🎨",
  SQL: "🛢️",
  HTML: "🌐",
  Markdown: "📝",
  JSON: "🧾",
  YAML: "📋",
  TOML: "⚙️",
  XML: "📄",
  CSV: "📑",
  "Bourne Shell": "🐚",
  "Bourne Again Shell": "🐚",
  PowerShell: "💠",
  Docker: "🐳",
};

const ULOC_EXCLUDE_DIRS = [
  ".repo",
  "node_modules",
  "dist",
  "build",
  "target",
  ".git",
  ".nx",
  "coverage",
  ".cache",
  ".turbo",
  ".next",
  "out",
  "vendor",
  "third_party",
  "Carthage",
];

const MAX_METRICS_FILE_BYTES = 8 * 1024 * 1024;
//#endregion Constants

//#region Path rules
function normalizeRepoPath(path: string): string {
  return path.replace(/\\/g, "/").replace(/^\.\//, "");
}

function metricsPathBasename(rel: string): string {
  return rel.slice(rel.lastIndexOf("/") + 1);
}

function hasHiddenDotPathSegment(rel: string): boolean {
  for (const seg of rel.split("/")) {
    if (!seg || seg === "." || seg === "..") continue;
    if (seg.startsWith(".")) return true;
  }
  return false;
}

function isMetricsLockOrGenerated(rel: string): boolean {
  const base = metricsPathBasename(rel);
  if (METRICS_LOCK_FILES.has(base)) return true;
  if (base.endsWith(".generated.go") || base.endsWith(".pb.go")) return true;
  return false;
}

function isMetricsLicenseTemplateFile(rel: string): boolean {
  const base = metricsPathBasename(rel);
  if (METRICS_LICENSE_TEMPLATE_BASENAMES.has(base)) return true;
  if (base.startsWith("LICENSE.")) return true;
  return false;
}

/** 🗂️Whether paths must be excluded from uloc/metrics (dot paths, license templates, vendor, lockfiles). */
export function shouldSkipPathForUloc(root: string, relPath: string): boolean {
  const rel = normalizeRepoPath(relPath);
  if (!rel || rel === ".repo" || rel.startsWith(".repo/")) return true;
  if (hasHiddenDotPathSegment(rel)) return true;
  if (isMetricsLicenseTemplateFile(rel)) return true;
  if (isMetricsLockOrGenerated(rel)) return true;
  const ignored = spawnSync("git", ["check-ignore", "-q", "--", rel], { cwd: gitRepoRoot(root) });
  if (ignored.status === 0) return true;
  for (const dir of ULOC_EXCLUDE_DIRS) {
    if (rel === dir || rel.startsWith(`${dir}/`)) return true;
  }
  return false;
}

/** 🏷️Maps a repo path to a metrics language bucket (code langs + JSON/YAML/… formats). */
export function classifyPathForMetrics(path: string): string {
  const rel = normalizeRepoPath(path);
  const base = rel.slice(rel.lastIndexOf("/") + 1).toLowerCase();
  if (base === "dockerfile" || base.startsWith("dockerfile.")) return "Dockerfile";
  if (base === "makefile" || base === "justfile") return "Makefile";
  const ext = rel.slice(rel.lastIndexOf(".")).toLowerCase();
  switch (ext) {
    case ".ts":
    case ".tsx":
    case ".cts":
    case ".mts":
    case ".mtsx":
      return "TypeScript";
    case ".js":
    case ".mjs":
    case ".cjs":
      return "JavaScript";
    case ".go":
      return "Go";
    case ".cs":
      return "C#";
    case ".py":
      return "Python";
    case ".rs":
      return "Rust";
    case ".sh":
    case ".bash":
    case ".zsh":
      return "Shell";
    case ".ps1":
      return "PowerShell";
    case ".css":
    case ".scss":
    case ".sass":
      return "CSS";
    case ".sql":
      return "SQL";
    case ".html":
    case ".htm":
    case ".xhtml":
      return "HTML";
    case ".md":
    case ".markdown":
    case ".mdown":
    case ".mkd":
    case ".mdx":
    case ".mdc":
    case ".svx":
      return "Markdown";
    case ".json":
    case ".jsonc":
      return "JSON";
    case ".yaml":
    case ".yml":
      return "YAML";
    case ".toml":
      return "TOML";
    case ".csv":
      return "CSV";
    case ".xml":
      return "XML";
    default:
      return "";
  }
}

/** 🎨Emoji for a metrics language row. */
export function langMetricsEmoji(lang: string): string {
  return LANG_EMOJI[lang] ?? "📎";
}

/** 🌳Resolves the git worktree root (never a subdirectory of the monorepo). */
export function gitRepoRoot(start: string): string {
  const r = spawnSync("git", ["rev-parse", "--show-toplevel"], { cwd: start, encoding: "utf8" });
  if (r.status === 0) {
    const top = (r.stdout ?? "").trim();
    if (top) return top;
  }
  return start;
}

function gitTrackedPaths(root: string): string[] {
  const repoRoot = gitRepoRoot(root);
  const r = spawnSync("git", ["ls-files", "-z"], { cwd: repoRoot, maxBuffer: 256 * 1024 * 1024 });
  if (r.status !== 0) return [];
  return (r.stdout ?? Buffer.alloc(0))
    .toString("utf8")
    .split("\0")
    .filter(Boolean);
}
//#endregion Path rules

//#region Uloc counting
function countJsonKeysValue(v: unknown): number {
  if (v === null || typeof v !== "object") return 0;
  if (Array.isArray(v)) {
    let n = 0;
    for (const item of v) n += countJsonKeysValue(item);
    return n;
  }
  const o = v as Record<string, unknown>;
  let n = Object.keys(o).length;
  for (const vv of Object.values(o)) n += countJsonKeysValue(vv);
  return n;
}

/** 🧮Counts JSON object keys recursively (aligned with repo `loc` Data JSON rules). */
export function countJsonKeys(text: string): number {
  const t = text.trim();
  if (!t) return 0;
  try {
    return countJsonKeysValue(JSON.parse(t));
  } catch {
    return 0;
  }
}

function physicalLineCount(text: string): number {
  if (!text.length) return 0;
  return text.split(/\r?\n/).length;
}

/** 📏LOC for one tracked file body (JSON key count when applicable). */
export function countUnifiedLocForFile(rel: string, data: string): number {
  const ext = rel.slice(rel.lastIndexOf(".")).toLowerCase();
  if (ext === ".json" || ext === ".jsonc") {
    const keys = countJsonKeys(data);
    if (keys > 0) return keys;
    if (!data.trim()) return 0;
    return physicalLineCount(data);
  }
  return physicalLineCount(data);
}

function gitDir(root: string): string {
  const repoRoot = gitRepoRoot(root);
  const r = spawnSync("git", ["rev-parse", "--git-dir"], { cwd: repoRoot, encoding: "utf8" });
  if (r.status !== 0) return join(repoRoot, ".git");
  const dir = (r.stdout ?? "").trim();
  return dir.startsWith("/") ? dir : join(repoRoot, dir);
}

const ULOC_CACHE_VERSION = 3;

type UlocCacheFile = {
  version: number;
  head: string;
  trackedFiles: number;
  totalLoc: number;
  langCount: number;
  counts: UlocByLanguage;
};

function ulocCachePath(root: string): string {
  return join(gitDir(root), "semio-uloc-cache.json");
}

function ulocCacheStats(counts: UlocByLanguage): { totalLoc: number; langCount: number } {
  let totalLoc = 0;
  let langCount = 0;
  for (const n of Object.values(counts)) {
    if (n > 0) {
      totalLoc += n;
      langCount++;
    }
  }
  return { totalLoc, langCount };
}

/** 🧪Whether cached uloc looks like a complete repo scan (rejects partial/stale caches). */
export function isUlocCachePlausible(root: string, counts: UlocByLanguage): boolean {
  const { totalLoc, langCount } = ulocCacheStats(counts);
  if (totalLoc <= 0 || langCount === 0) return false;
  const tracked = gitTrackedPaths(root).length;
  if (tracked === 0) return true;
  if (tracked < 100) return totalLoc > 0 && (langCount >= 2 || totalLoc >= 50);
  if (langCount < 6 && tracked >= 300) return false;
  if (totalLoc < tracked * 3) return false;
  return true;
}

function readUlocCache(root: string): UlocByLanguage | null {
  const head = spawnSync("git", ["rev-parse", "HEAD"], { cwd: root, encoding: "utf8" });
  if (head.status !== 0) return null;
  const h = (head.stdout ?? "").trim();
  if (!h) return null;
  try {
    const raw = readFileSync(ulocCachePath(root), "utf8");
    const parsed = JSON.parse(raw) as Partial<UlocCacheFile>;
    if (parsed.version !== ULOC_CACHE_VERSION) return null;
    if (parsed.head !== h || !parsed.counts || typeof parsed.counts !== "object") return null;
    const tracked = gitTrackedPaths(root).length;
    if (typeof parsed.trackedFiles === "number" && Math.abs(tracked - parsed.trackedFiles) > 50) return null;
    if (!isUlocCachePlausible(root, parsed.counts)) return null;
    const stats = ulocCacheStats(parsed.counts);
    if (typeof parsed.totalLoc === "number" && parsed.totalLoc !== stats.totalLoc) return null;
    if (typeof parsed.langCount === "number" && parsed.langCount !== stats.langCount) return null;
    return parsed.counts;
  } catch {
    return null;
  }
}

function writeUlocCache(root: string, counts: UlocByLanguage): void {
  const head = spawnSync("git", ["rev-parse", "HEAD"], { cwd: root, encoding: "utf8" });
  if (head.status !== 0) return;
  const h = (head.stdout ?? "").trim();
  if (!h) return;
  const { totalLoc, langCount } = ulocCacheStats(counts);
  const payload: UlocCacheFile = {
    version: ULOC_CACHE_VERSION,
    head: h,
    trackedFiles: gitTrackedPaths(root).length,
    totalLoc,
    langCount,
    counts,
  };
  writeFileSync(ulocCachePath(root), JSON.stringify(payload));
}

/** 📊Scans all git-tracked paths and sums unified LOC per language bucket. */
export function scanRepoUnifiedLocUncached(root: string): UlocByLanguage {
  const repoRoot = gitRepoRoot(root);
  const out: UlocByLanguage = {};
  for (const rel of gitTrackedPaths(root)) {
    if (shouldSkipPathForUloc(repoRoot, rel)) continue;
    const lang = classifyPathForMetrics(rel);
    if (!lang) continue;
    const fp = join(repoRoot, rel);
    if (!existsSync(fp)) continue;
    try {
      const st = statSync(fp);
      if (!st.isFile() || st.size > MAX_METRICS_FILE_BYTES) continue;
    } catch {
      continue;
    }
    let data: string;
    try {
      const buf = readFileSync(fp);
      if (buf.includes(0)) continue;
      data = buf.toString("utf8");
    } catch {
      continue;
    }
    const loc = countUnifiedLocForFile(rel, data);
    if (loc <= 0) continue;
    out[lang] = (out[lang] ?? 0) + loc;
  }
  return out;
}

/** 📊Repo uloc with per-HEAD cache under `.git/semio-uloc-cache.json`. */
export function scanRepoUnifiedLoc(root: string): UlocByLanguage {
  const cached = readUlocCache(root);
  if (cached) return cached;
  const counts = scanRepoUnifiedLocUncached(root);
  writeUlocCache(root, counts);
  return counts;
}

/** 📊Default uloc runner (tracked-file unified scan). */
export function createDefaultUlocRunner(): UlocRunner {
  return { countRepoByLanguage: scanRepoUnifiedLoc };
}
//#endregion Uloc counting

//#region Git deltas
/** ✂️Splits git numstat into replaced (edited), net added, and net removed lines. */
export function splitGitNumstatDelta(
  added: number,
  removed: number,
): { edited: number; added: number; removed: number } {
  const a = Math.max(0, added);
  const r = Math.max(0, removed);
  const edited = Math.min(a, r);
  return { edited, added: a - edited, removed: r - edited };
}

function parseGitNumstatZ(stdout: Buffer | string): { path: string; added: number; removed: number }[] {
  const raw = typeof stdout === "string" ? stdout : stdout.toString("utf8");
  if (!raw) return [];
  const out: { path: string; added: number; removed: number }[] = [];
  for (const entry of raw.split("\0")) {
    if (!entry) continue;
    const parts = entry.split("\t");
    if (parts.length < 3) continue;
    const added = parts[0] === "-" ? 0 : Number(parts[0]) || 0;
    const removed = parts[1] === "-" ? 0 : Number(parts[1]) || 0;
    out.push({ path: parts.slice(2).join("\t"), added, removed });
  }
  return out;
}

function gitCachedNumstat(root: string): { path: string; added: number; removed: number }[] {
  const r = spawnSync("git", ["diff", "--cached", "--numstat", "-z"], { cwd: gitRepoRoot(root) });
  if (r.status !== 0) return [];
  return parseGitNumstatZ(r.stdout ?? Buffer.alloc(0));
}

/** 📂Whether a repo-relative path lies under any normalized prefix. */
export function pathUnderPrefixes(rel: string, prefixes: string[]): boolean {
  if (prefixes.length === 0) return true;
  const n = normalizeRepoPath(rel);
  for (const raw of prefixes) {
    const p = normalizeRepoPath(raw).replace(/\/$/, "");
    if (!p) continue;
    if (n === p || n.startsWith(`${p}/`)) return true;
  }
  return false;
}

/** 📈Git numstat between two revisions. */
export function gitRangeNumstat(root: string, base: string, head: string): { path: string; added: number; removed: number }[] {
  const repoRoot = gitRepoRoot(root);
  const r = spawnSync("git", ["diff", "--numstat", "-z", `${base}..${head}`], {
    cwd: repoRoot,
    maxBuffer: 256 * 1024 * 1024,
  });
  if (r.status !== 0) return [];
  return parseGitNumstatZ(r.stdout ?? Buffer.alloc(0));
}

/** ➕Accumulates per-language git deltas from numstat rows (optional path prefixes). */
export function accumulateGitDeltasFromNumstat(
  root: string,
  rows: { path: string; added: number; removed: number }[],
  pathPrefixes?: string[],
): Map<string, { added: number; removed: number; edited: number }> {
  const m = new Map<string, { added: number; removed: number; edited: number }>();
  for (const { path, added, removed } of rows) {
    if (shouldSkipPathForUloc(root, path)) continue;
    if (pathPrefixes && !pathUnderPrefixes(path, pathPrefixes)) continue;
    const lang = classifyPathForMetrics(path);
    if (!lang) continue;
    const d = splitGitNumstatDelta(added, removed);
    const row = m.get(lang) ?? { added: 0, removed: 0, edited: 0 };
    row.edited += d.edited;
    row.added += d.added;
    row.removed += d.removed;
    m.set(lang, row);
  }
  return m;
}

function accumulateGitDeltas(root: string): Map<string, { added: number; removed: number; edited: number }> {
  return accumulateGitDeltasFromNumstat(root, gitCachedNumstat(root));
}

/** ➕Sums git delta counters across all languages. */
export function sumGitLangDeltas(deltas: Map<string, { added: number; removed: number; edited: number }>): {
  added: number;
  removed: number;
  edited: number;
} {
  let added = 0;
  let removed = 0;
  let edited = 0;
  for (const d of deltas.values()) {
    added += d.added;
    removed += d.removed;
    edited += d.edited;
  }
  return { added, removed, edited };
}

/** 🟰Sum of net added, edited (replaced), and removed line counts from git numstat. */
export function gitDeltaLineTotal(d: { added: number; removed: number; edited: number }): number {
  return d.added + d.edited + d.removed;
}

/** 📊Appends `➕` `✏️` `➖` and total `🟰` (sum of the three) when non-zero. */
export function appendGitDeltaSuffix(line: string, d: { added: number; removed: number; edited: number }): string {
  if (d.added > 0) line += `➕${d.added}`;
  if (d.edited > 0) line += `✏️${d.edited}`;
  if (d.removed > 0) line += `➖${d.removed}`;
  const sum = gitDeltaLineTotal(d);
  if (sum > 0) line += `🟰${sum}`;
  return line;
}

/** 📊Compact `📊uloc➕…✏️…➖…🟰…` suffix from git deltas (bundle header). */
export function formatBundleUlocSuffix(d: { added: number; removed: number; edited: number }): string {
  return appendGitDeltaSuffix("📊uloc", d);
}
//#endregion Git deltas

//#region Format
/** 🔢Formats uncommented LOC counts (e.g. 200000 → 200k). */
export function formatMetricLocCount(n: number): string {
  const v = Math.max(0, Math.round(n));
  if (v >= 1000) return `${Math.round(v / 1000)}k`;
  return String(v);
}

function sortMetricLanguages(codeByLang: UlocByLanguage, deltas: Map<string, { edited: number }>): string[] {
  const langs = new Set<string>();
  for (const [lang, n] of Object.entries(codeByLang)) {
    if (n > 0) langs.add(lang);
  }
  for (const lang of deltas.keys()) langs.add(lang);
  return [...langs].sort((a, b) => (codeByLang[b] ?? 0) - (codeByLang[a] ?? 0) || a.localeCompare(b));
}

function buildMicroCommitMetricsFromDeltas(
  codeByLang: UlocByLanguage,
  deltas: Map<string, { added: number; removed: number; edited: number }>,
): MicroCommitLangMetrics[] {
  const rows: MicroCommitLangMetrics[] = [];
  for (const lang of sortMetricLanguages(codeByLang, deltas)) {
    const d = deltas.get(lang) ?? { added: 0, removed: 0, edited: 0 };
    const code = codeByLang[lang] ?? 0;
    if (code === 0 && d.edited === 0 && d.added === 0 && d.removed === 0) continue;
    rows.push({
      lang,
      emoji: langMetricsEmoji(lang),
      code,
      edited: d.edited,
      added: d.added,
      removed: d.removed,
    });
  }
  return rows;
}

/** 📊Builds micro-commit metrics: repo uloc per language + staged git deltas. */
export function buildMicroCommitMetrics(root: string, ulocRunner: UlocRunner = createDefaultUlocRunner()): MicroCommitLangMetrics[] {
  const repoRoot = gitRepoRoot(root);
  const deltas = accumulateGitDeltas(repoRoot);
  const codeByLang = ulocRunner.countRepoByLanguage(repoRoot);
  return buildMicroCommitMetricsFromDeltas(codeByLang, deltas);
}

/** 📊Builds uloc metrics for a git revision range (optional path prefixes). */
export function buildMicroCommitMetricsForRange(
  root: string,
  base: string,
  head = "HEAD",
  pathPrefixes?: string[],
  ulocRunner: UlocRunner = createDefaultUlocRunner(),
): MicroCommitLangMetrics[] {
  const repoRoot = gitRepoRoot(root);
  const deltas = accumulateGitDeltasFromNumstat(repoRoot, gitRangeNumstat(repoRoot, base, head), pathPrefixes);
  const codeByLang = ulocRunner.countRepoByLanguage(repoRoot);
  return buildMicroCommitMetricsFromDeltas(codeByLang, deltas);
}

/** 📊Micro-commit metrics block header (unified repo LOC). */
export const MICRO_COMMIT_ULOC_HEADER = "📊uloc";

/** 🔢Emoji for the aggregate total row (first line after the header). */
export const MICRO_COMMIT_ULOC_TOTAL_EMOJI = "🔢";

/** 📊Sums per-language uloc rows into one total. */
export function sumMicroCommitLangMetrics(metrics: MicroCommitLangMetrics[]): MicroCommitLangMetrics {
  const total: MicroCommitLangMetrics = {
    lang: "Total",
    emoji: MICRO_COMMIT_ULOC_TOTAL_EMOJI,
    code: 0,
    edited: 0,
    added: 0,
    removed: 0,
  };
  for (const m of metrics) {
    total.code += m.code;
    total.edited += m.edited;
    total.added += m.added;
    total.removed += m.removed;
  }
  return total;
}

/** 📊Formats one uloc row; omits zero ➕✏️➖; appends 🟰 sum when any delta is non-zero. */
export function formatMicroCommitMetricLine(m: MicroCommitLangMetrics): string {
  return appendGitDeltaSuffix(`${m.emoji}${formatMetricLocCount(m.code)}`, m);
}

/** 📊Renders the unified LOC block: `📊uloc➕…✏️…➖…🟰…` total, then per-language rows. */
export function formatMicroCommitMetricsLines(metrics: MicroCommitLangMetrics[]): string[] {
  if (metrics.length === 0) return [];
  const total = sumMicroCommitLangMetrics(metrics);
  const header = formatBundleUlocSuffix({ added: total.added, edited: total.edited, removed: total.removed });
  return [header, ...metrics.map(formatMicroCommitMetricLine)];
}

/** ✅Whether two git delta sums match on ➕ ✏️ ➖ (🟰 follows). */
export function gitDeltaSumsEqual(
  a: { added: number; removed: number; edited: number },
  b: { added: number; removed: number; edited: number },
): boolean {
  return a.added === b.added && a.edited === b.edited && a.removed === b.removed;
}

/** 🚫Per-language ➕✏️➖ must sum to the footer `📊uloc` total line. */
export function validateMicroCommitLangMetricsDeltaSum(metrics: MicroCommitLangMetrics[]): void {
  if (metrics.length === 0) return;
  const total = sumMicroCommitLangMetrics(metrics);
  let added = 0;
  let edited = 0;
  let removed = 0;
  for (const m of metrics) {
    added += m.added;
    edited += m.edited;
    removed += m.removed;
  }
  if (!gitDeltaSumsEqual({ added, edited, removed }, total)) {
    throw new Error(
      `commit: per-language 📊uloc deltas do not sum to the footer total — languages ➕${added}✏️${edited}➖${removed}🟰${gitDeltaLineTotal({ added, edited, removed })} vs footer ➕${total.added}✏️${total.edited}➖${total.removed}🟰${gitDeltaLineTotal(total)}`,
    );
  }
}
//#endregion Format
