import { existsSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
import {
  accumulateGitDeltasFromNumstat,
  buildMicroCommitMetricsForRange,
  formatBundleUlocSuffix,
  formatMicroCommitMetricsLines,
  gitDeltaLineTotal,
  gitDeltaSumsEqual,
  gitRangeNumstat,
  gitRepoRoot,
  pathUnderPrefixes,
  shouldSkipPathForUloc,
  sumGitLangDeltas,
  sumMicroCommitLangMetrics,
  validateMicroCommitLangMetricsDeltaSum,
  type UlocRunner,
} from "./uloc-metrics.ts";
import { bulletEmojiValidationError } from "./micro-commit.ts";

export type CommitLevel =
  | "prepare-only"
  | "prepare-and-tag"
  | "prepare-and-tag-and-squash"
  | "prepare-and-tag-and-squash-and-push";

export type CommitSteps = { tag: boolean; squash: boolean; push: boolean };

type Contributor = { alias: string; emoji: string; name: string; email: string; emails?: string[] };

export type CommitBundleDateSection = { dateLine: string; bullets: string[] };
export type CommitBundleSection = { label: string; dates: CommitBundleDateSection[] };

export const BUNDLE_WIP_SUBJECT_RE = /^(.+🎆\d{2}🌙\d{2}☀️\d{2})🔀$/u;
export const BUNDLE_DATE_SECTION_RE = /^🎆\d{2}🌙\d{2}☀️\d{2}$/u;
const EMOJI_CLUSTER_RE = /^(\p{Extended_Pictographic}(?:\uFE0F|\u200D\p{Extended_Pictographic})*)/u;
const BUNDLE_SCOPE_RESERVED_RE = /🔀|🚩|📊uloc|🔢/u;
const LABEL_TOKEN_BLOCKLIST = new Set(["uloc", "repo", "the", "and"]);
const MICRO_COMMIT_BULLET_RE = /^(?:\p{Extended_Pictographic}(?:\uFE0F|\u200D\p{Extended_Pictographic})*)+\S/u;

function pad2(n: number): string {
  return String(n).padStart(2, "0");
}

function git(root: string, args: string[]): { ok: boolean; out: string } {
  const r = spawnSync("git", args, { cwd: root, encoding: "utf8" });
  if (r.status !== 0) return { ok: false, out: (r.stderr ?? r.stdout ?? "").trim() };
  return { ok: true, out: (r.stdout ?? "").trim() };
}

function gitDir(root: string): string {
  const out = git(root, ["rev-parse", "--git-dir"]).out;
  return out.startsWith("/") ? out : join(root, out);
}

function branchAllowed(root: string): boolean {
  const b = git(root, ["branch", "--show-current"]).out;
  return b.includes("⛳wip") || b.includes("🏗️dev");
}

function gitEmail(root: string): string {
  return git(root, ["config", "user.email"]).out;
}

function findContributor(root: string): Contributor | null {
  const email = gitEmail(root).toLowerCase();
  if (!email) return null;
  const dir = join(root, ".repo", "🧑‍💻");
  if (!existsSync(dir)) return null;
  for (const name of readdirSync(dir, { withFileTypes: true })) {
    if (!name.isDirectory()) continue;
    const path = join(dir, name.name, "contributor.json");
    if (!existsSync(path)) continue;
    const c = JSON.parse(readFileSync(path, "utf8")) as Contributor & { emails?: string[] };
    const emails = [c.email, ...(c.emails ?? [])]
      .filter((e): e is string => typeof e === "string" && e.length > 0)
      .map((e) => e.toLowerCase());
    if (emails.includes(email)) return c;
  }
  return null;
}

/** 🧭Parses explicit `ct` / `cs` / `cp` step flags from argv segments. */
export function parseCommitSteps(segments: string[]): CommitSteps {
  const token = segments.join(" ").toLowerCase();
  return {
    tag: /\b(ct|ctag|tag!|\+tag)\b/.test(token),
    squash: /\b(cs|csquash|squash!|\+squash)\b/.test(token),
    push: /\b(cp|cpush|push!|\+push)\b/.test(token),
  };
}

function commitStepsFromLevel(level: CommitLevel): CommitSteps {
  switch (level) {
    case "prepare-and-tag":
      return { tag: true, squash: false, push: false };
    case "prepare-and-tag-and-squash":
      return { tag: true, squash: true, push: false };
    case "prepare-and-tag-and-squash-and-push":
      return { tag: true, squash: true, push: true };
    default:
      return { tag: false, squash: false, push: false };
  }
}

function loadCommitSteps(root: string, contributor: Contributor, segments: string[]): CommitSteps {
  const explicit = parseCommitSteps(segments);
  if (explicit.tag || explicit.squash || explicit.push) return explicit;
  const path = join(root, ".repo", "🧑‍💻", contributor.alias, "commit.json");
  if (existsSync(path)) {
    const j = JSON.parse(readFileSync(path, "utf8")) as { level?: string };
    const allowed: CommitLevel[] = [
      "prepare-only",
      "prepare-and-tag",
      "prepare-and-tag-and-squash",
      "prepare-and-tag-and-squash-and-push",
    ];
    if (allowed.includes(j.level as CommitLevel)) return commitStepsFromLevel(j.level as CommitLevel);
  }
  return { tag: false, squash: false, push: false };
}

export function isCommitPrepareOnly(segments: string[]): boolean {
  const token = segments.join(" ").toLowerCase();
  if (/\b(c\.|cprepare|prepare!|\+prepare)\b/.test(token)) return true;
  const steps = parseCommitSteps(segments);
  return !steps.tag && !steps.squash && !steps.push;
}

function shSingleQuote(s: string): string {
  return `'${s.replace(/'/g, `'\"'\"'`)}'`;
}

function formatCommitPrepareCommandBlock(command: string): string {
  return `\`\`\`\n${command}\n\`\`\``;
}

/** 🏷️GPG-signed annotated tag; tag object name and `-m` message are the same (`…🚩`). */
export function formatGitSignedTagCommand(tagName: string, head = "HEAD"): string {
  const q = shSingleQuote(tagName);
  return `git tag -s -m ${q} ${q} ${head}`;
}

/** 📋Four copy-paste ``` blocks: signed tag, squash, push, all-in-one. */
export function formatCommitPrepareCommands(opts: {
  tagName: string;
  wipSha: string;
  messageFile?: string;
}): string {
  const msg = opts.messageFile ?? ".git/semio-commit-message";
  const tag = formatGitSignedTagCommand(opts.tagName);
  const squash = `git reset --soft ${opts.wipSha} && git commit -S -F ${shSingleQuote(msg)}`;
  const push = `git push --follow-tags`;
  const all = `${tag} && git reset --soft ${opts.wipSha} && git commit -S -F ${shSingleQuote(msg)} && git push --follow-tags`;
  return `${[tag, squash, push, all].map(formatCommitPrepareCommandBlock).join("\n\n")}\n`;
}

/** 📋Prepare-only agent reply: four `git` blocks, then tag name, then full commit message. */
export function formatCommitPrepareAgentReply(opts: {
  tagName: string;
  wipSha: string;
  messageFile?: string;
  commitMessage: string;
}): string {
  const commands = formatCommitPrepareCommands({
    tagName: opts.tagName,
    wipSha: opts.wipSha,
    messageFile: opts.messageFile,
  });
  const tagNameBlock = formatCommitPrepareCommandBlock(opts.tagName.trim());
  const messageBlock = formatCommitPrepareCommandBlock(opts.commitMessage.trimEnd());
  return `${commands}${tagNameBlock}\n\n${messageBlock}\n`;
}

/** 🔀Finds the newest commit whose subject is a bundle/WIP marker (`…🔀`). */
export function findLastBundleWipCommit(root: string): { sha: string; subject: string } | null {
  root = gitRepoRoot(root);
  const r = spawnSync("git", ["log", "--format=%H%x00%s%x00", "-n", "500"], {
    cwd: root,
    encoding: "utf8",
    maxBuffer: 16 * 1024 * 1024,
  });
  if (r.status !== 0) return null;
  const parts = (r.stdout ?? "").split("\0").filter(Boolean);
  for (let i = 0; i + 1 < parts.length; i += 2) {
    const sha = parts[i]!.trim();
    const subject = parts[i + 1]!.trim();
    if (BUNDLE_WIP_SUBJECT_RE.test(subject)) return { sha, subject };
  }
  return null;
}

/** 🏷️Signed tag name for the micro-commit tip before squash (`…🚩`). */
export function formatBundleTagName(contributor: Contributor, now = new Date()): string {
  const yy = pad2(now.getFullYear() % 100);
  const mm = pad2(now.getMonth() + 1);
  const dd = pad2(now.getDate());
  return `${contributor.emoji}${contributor.alias}🎆${yy}🌙${mm}☀️${dd}🚩`;
}

/** 🔀Bundle squash commit subject (`…🔀`). */
export function formatBundleSubject(contributor: Contributor, now = new Date()): string {
  const yy = pad2(now.getFullYear() % 100);
  const mm = pad2(now.getMonth() + 1);
  const dd = pad2(now.getDate());
  return `${contributor.emoji}${contributor.alias}🎆${yy}🌙${mm}☀️${dd}🔀`;
}

/** 🔢Leading emoji clusters on a line. */
export function leadingEmojiClusterCount(line: string): number {
  let n = 0;
  let rest = line.trim();
  while (true) {
    const m = EMOJI_CLUSTER_RE.exec(rest);
    if (!m) break;
    n++;
    rest = rest.slice(m[0].length);
  }
  return n;
}

/** 🔢Emoji clusters anywhere on a line (bundle labels interleave emoji + text). */
export function emojiClusterCountInLine(line: string): number {
  const re = /\p{Extended_Pictographic}(?:\uFE0F|\u200D\p{Extended_Pictographic})*/gu;
  return [...line.matchAll(re)].length;
}

/** 🧹Strips hand-written uloc and reserved bundle/tag emojis from a scope line. */
export function normalizeBundleScopeLabel(line: string): string {
  return line
    .trim()
    .replace(/📊uloc.*$/u, "")
    .replace(/🔀|🚩/gu, "")
    .trim();
}

/** 🏷️ASCII tokens from a bundle emoji label (for matching git paths internally). */
export function labelPathTokens(label: string): string[] {
  const text = normalizeBundleScopeLabel(label).replace(/\p{Extended_Pictographic}/gu, " ").trim().toLowerCase();
  return text.split(/[^a-z0-9]+/).filter((t) => t.length >= 2 && !LABEL_TOKEN_BLOCKLIST.has(t));
}

/** 🚫Validates bundle scope label before parse. */
export function bundleScopeLabelError(line: string): string | null {
  const raw = line.trim();
  if (raw.includes("|") || raw.includes("/")) {
    return "commit: bundle scope must be emoji + area name only — no paths or `|`";
  }
  if (BUNDLE_SCOPE_RESERVED_RE.test(raw)) {
    return "commit: bundle scope must not include 🔀 🚩 📊uloc or 🔢 — script adds subject and uloc";
  }
  const norm = normalizeBundleScopeLabel(raw);
  if (!norm) return "commit: empty bundle scope after removing reserved emojis";
  const tokens = labelPathTokens(norm);
  if (tokens.length === 0) {
    return "commit: bundle scope needs an area name after emojis (e.g. 🏘️semio✍️sketchpad, 🥅framework, 🖱️ui⚛️react)";
  }
  if (!isBundleScopeLine(norm)) {
    return `commit: invalid bundle scope: ${raw}`;
  }
  return null;
}

function changedPathsInRange(root: string, base: string, head: string): string[] {
  const r = git(root, ["diff", "--name-only", `${base}..${head}`]);
  if (!r.ok || !r.out) return [];
  return r.out.split("\n").filter(Boolean);
}

function longestCommonPathPrefix(paths: string[]): string {
  if (paths.length === 0) return "";
  const split = paths.map((p) => p.split("/"));
  const first = split[0]!;
  let len = 0;
  for (let i = 0; i < first.length; i++) {
    const seg = first[i]!;
    if (split.every((parts) => parts[i] === seg)) len = i + 1;
    else break;
  }
  if (len === 0) return paths[0]!.split("/")[0] ?? "";
  return first.slice(0, len).join("/");
}

/** 📂Infers repo path prefixes for a bundle label from the revision range (not shown in messages). */
export function inferPathPrefixesForBundleLabel(
  root: string,
  base: string,
  head: string,
  label: string,
  assignedPaths?: string[],
): string[] {
  const tokens = labelPathTokens(label);
  if (tokens.length === 0) return [];
  const pool = assignedPaths ?? changedPathsInRange(root, base, head);
  const matched = pool.filter((p) => {
    const pl = p.toLowerCase();
    return tokens.every((t) => pl.includes(t));
  });
  if (matched.length === 0) return [];
  const prefix = longestCommonPathPrefix(matched);
  return prefix ? [prefix] : [];
}

function assignPathToBundleIndex(bundles: CommitBundleSection[], path: string): number {
  const pl = path.toLowerCase();
  let best = -1;
  let bestScore = 0;
  for (let i = 0; i < bundles.length; i++) {
    const tokens = labelPathTokens(bundles[i]!.label);
    if (tokens.length === 0) continue;
    if (!tokens.every((t) => pl.includes(t))) continue;
    const score = tokens.length;
    if (score > bestScore) {
      bestScore = score;
      best = i;
    }
  }
  return best;
}

function assignChangedPathsToBundles(
  root: string,
  base: string,
  head: string,
  bundles: CommitBundleSection[],
): string[][] {
  const paths = changedPathsInRange(root, base, head);
  const assigned = bundles.map(() => [] as string[]);
  for (const p of paths) {
    const bi = assignPathToBundleIndex(bundles, p);
    if (bi >= 0) assigned[bi]!.push(p);
  }
  return assigned;
}

type GitDeltaSum = { added: number; removed: number; edited: number };

function addGitDeltaSums(a: GitDeltaSum, b: GitDeltaSum): GitDeltaSum {
  return { added: a.added + b.added, removed: a.removed + b.removed, edited: a.edited + b.edited };
}

const BUNDLE_COMMIT_TIMESTAMP_LINE_RE = /^🎆\d{2}🌙\d{2}☀️\d{2}⏰/u;

/** 🎆Calendar day from a micro-commit subject (`🎆YY🌙MM☀️DD`). */
export function extractBundleDateLineFromSubject(subject: string): string | null {
  const m = /🎆\d{2}🌙\d{2}☀️\d{2}/u.exec(subject.trim());
  return m?.[0] ?? null;
}

/** 🎆Calendar day from the micro-commit body timestamp line (`🎆YY🌙MM☀️DD⏰…`). */
export function extractBundleDateLineFromCommitBody(body: string): string | null {
  for (const raw of body.split("\n")) {
    const line = raw.trim();
    if (!BUNDLE_COMMIT_TIMESTAMP_LINE_RE.test(line)) continue;
    const m = /^🎆\d{2}🌙\d{2}☀️\d{2}/u.exec(line);
    return m?.[0] ?? null;
  }
  return null;
}

/** 🎆Calendar day for bundle per-day uloc (body timestamp, else subject). */
export function extractBundleDateLineFromCommit(subject: string, body: string): string | null {
  return extractBundleDateLineFromCommitBody(body) ?? extractBundleDateLineFromSubject(subject);
}

/** 📂Paths from one numstat row (rename rows may join old/new with tabs or `=>`). */
export function pathsFromNumstatRow(pathField: string): string[] {
  const parts = pathField
    .split("\t")
    .map((p) => p.trim())
    .filter(Boolean);
  if (parts.length === 0) return [];
  const out = new Set<string>();
  for (const part of parts) {
    const brace = /^(.*)\{(.+?)\s*=>\s*(.+?)\}(.*)$/u.exec(part);
    if (brace) {
      const a = brace[2]!.trim();
      const b = brace[3]!.trim();
      if (a) out.add(a);
      if (b) out.add(b);
      continue;
    }
    const arrow = /\s*=>\s*/u.exec(part);
    if (arrow) {
      const [from, to] = part.split(/\s*=>\s*/u);
      if (from?.trim()) out.add(from.trim());
      if (to?.trim()) out.add(to.trim());
      continue;
    }
    out.add(part);
  }
  return [...out];
}

/** 📂Prefix set for a bundle: all range paths, inferred roots, and token-matching parents (renames). */
export function buildBundlePathPrefixSets(
  root: string,
  base: string,
  head: string,
  bundles: CommitBundleSection[],
): string[][] {
  const assignments = assignChangedPathsToBundles(root, base, head, bundles);
  return bundles.map((bundle, i) => {
    const assigned = assignments[i] ?? [];
    const prefixes = new Set<string>();
    const tokens = labelPathTokens(bundle.label);
    for (const p of assigned) {
      prefixes.add(p);
      if (tokens.length === 0) continue;
      const segments = p.split("/");
      let acc = "";
      for (const seg of segments) {
        acc = acc ? `${acc}/${seg}` : seg;
        const pl = acc.toLowerCase();
        if (tokens.every((t) => pl.includes(t))) prefixes.add(acc);
      }
    }
    return [...prefixes];
  });
}

/** 📂Whether a path belongs to a bundle (assigned path prefixes, else label token scoring). */
export function pathMatchesBundleIndex(
  path: string,
  bundleIndex: number,
  prefixSets: string[][],
  bundles: CommitBundleSection[],
): boolean {
  const prefixes = prefixSets[bundleIndex] ?? [];
  if (prefixes.length > 0) return pathUnderPrefixes(path, prefixes);
  return assignPathToBundleIndex(bundles, path) === bundleIndex;
}

/** 🧹Strips hand-written per-day uloc from a date section line. */
export function normalizeBundleDateLine(line: string): string {
  return line.trim().replace(/📊uloc.*$/u, "").trim();
}

/** 🎆Bundle squash only: `🎆YY🌙MM☀️DD` + per-day git delta suffix (micro-commit uses timestamp + footer uloc only). */
export function formatBundleDateLine(dateLine: string, d: GitDeltaSum): string {
  return `${normalizeBundleDateLine(dateLine)}${formatBundleUlocSuffix(d)}`;
}

export type BundleDateDeltasMap = Map<number, Map<string, GitDeltaSum>>;

function gitCommitShasInRange(root: string, base: string, head: string): string[] {
  const r = git(root, ["rev-list", "--reverse", `${base}..${head}`]);
  if (!r.ok || !r.out) return [];
  return r.out.split("\n").filter(Boolean);
}

function addNumstatRowToBundleDateMap(
  map: BundleDateDeltasMap,
  row: { path: string; added: number; removed: number },
  dateLine: string,
  bi: number,
  root: string,
): void {
  const chunk = sumGitLangDeltas(
    accumulateGitDeltasFromNumstat(root, [{ path: row.path, added: row.added, removed: row.removed }]),
  );
  if (gitDeltaLineTotal(chunk) === 0) return;
  const bundleMap = map.get(bi)!;
  const prev = bundleMap.get(dateLine) ?? { added: 0, removed: 0, edited: 0 };
  bundleMap.set(dateLine, addGitDeltaSums(prev, chunk));
}

/** 📊Per-bundle per-day git deltas: sum each micro-commit parent..sha row on its body 🎆 day (sums to range partition). */
export function buildBundleDateDeltasMap(
  root: string,
  base: string,
  head: string,
  bundles: CommitBundleSection[],
): BundleDateDeltasMap {
  root = gitRepoRoot(root);
  const map: BundleDateDeltasMap = new Map();
  for (let i = 0; i < bundles.length; i++) map.set(i, new Map());
  const prefixSets = buildBundlePathPrefixSets(root, base, head, bundles);
  for (const sha of gitCommitShasInRange(root, base, head)) {
    const parent = `${sha}^`;
    const subject = git(root, ["log", "-1", "--format=%s", sha]).out;
    const body = git(root, ["log", "-1", "--format=%B", sha]).out;
    const dateLine = extractBundleDateLineFromCommit(subject, body);
    if (!dateLine) continue;
    for (const row of gitRangeNumstat(root, parent, sha)) {
      const rowPaths = pathsFromNumstatRow(row.path);
      if (rowPaths.length === 0 || rowPaths.every((p) => shouldSkipPathForUloc(root, p))) continue;
      const owners = resolveBundleIndicesForNumstatRow(row.path, prefixSets, bundles);
      if (owners.length === 0) {
        throw new Error(
          `commit: changed path is not attributed to any bundle — ${row.path}; add a bundle scope or fix labels`,
        );
      }
      if (owners.length > 1) {
        const names = owners.map((i) => bundles[i]!.label).join(", ");
        throw new Error(`commit: changed path matches multiple bundles (${names}) — ${row.path}`);
      }
      addNumstatRowToBundleDateMap(map, row, dateLine, owners[0]!, root);
    }
  }
  return map;
}

function bundleGitDeltasForPaths(
  root: string,
  base: string,
  head: string,
  pathPrefixes: string[],
): { added: number; removed: number; edited: number } {
  const rows = gitRangeNumstat(root, base, head);
  const assigned =
    pathPrefixes.length > 0
      ? rows.filter((r) => pathsFromNumstatRow(r.path).some((p) => pathUnderPrefixes(p, pathPrefixes)))
      : [];
  return sumGitLangDeltas(accumulateGitDeltasFromNumstat(root, assigned));
}

function formatBundleHeaderLine(label: string, total: GitDeltaSum): string {
  return `${normalizeBundleScopeLabel(label)}${formatBundleUlocSuffix(total)}`;
}

/** 📊Orders bundles by descending 🟰 (➕+✏️+➖) from assigned path diffs. */
export function sortCommitBundlesByEditTotal(
  root: string,
  base: string,
  head: string,
  bundles: CommitBundleSection[],
  pathAssignments: string[][],
): { bundles: CommitBundleSection[]; pathAssignments: string[][]; pathPrefixSets: string[][] } {
  const prefixSets = buildBundlePathPrefixSets(root, base, head, bundles);
  const ranked = bundles.map((bundle, i) => ({
    bundle,
    paths: pathAssignments[i] ?? [],
    prefixes: prefixSets[i] ?? [],
    total: gitDeltaLineTotal(bundleGitDeltasForPaths(root, base, head, prefixSets[i] ?? [])),
  }));
  ranked.sort((a, b) => b.total - a.total);
  return {
    bundles: ranked.map((r) => r.bundle),
    pathAssignments: ranked.map((r) => r.paths),
    pathPrefixSets: ranked.map((r) => r.prefixes),
  };
}

/** 🏷️Bundle scope line: two+ emojis, or one emoji + lowercase technology slug (`🥅framework`). */
export function isBundleScopeLine(line: string): boolean {
  const t = normalizeBundleScopeLabel(line);
  if (!t || BUNDLE_DATE_SECTION_RE.test(t)) return false;
  if (t.includes("/") || t.includes("|")) return false;
  if (BUNDLE_SCOPE_RESERVED_RE.test(t)) return false;
  if (emojiClusterCountInLine(t) >= 2) return true;
  if (emojiClusterCountInLine(t) !== 1) return false;
  const after = t.replace(/^\p{Extended_Pictographic}(?:\uFE0F|\u200D\p{Extended_Pictographic})*/u, "");
  return /^[a-z][a-z0-9]{2,24}$/u.test(after);
}

/** 🚫Validates stdin is bundle body only, not a full commit message. */
export function commitBundleBodyError(text: string): string | null {
  const first = text.trim().split("\n").find((l) => l.trim().length > 0)?.trim() ?? "";
  if (BUNDLE_WIP_SUBJECT_RE.test(first)) {
    return "commit: stdin must not include the bundle subject (…🔀) — script adds it";
  }
  if (/^🐙|^🧑/.test(first) && first.includes("🚩")) {
    return "commit: stdin must not include micro-commit subject lines";
  }
  if (/^📊uloc/m.test(text) || /^🔢[\dk]/m.test(text)) {
    return "commit: stdin must not include the 📊uloc footer — script adds it";
  }
  if (/^🎆\d{2}🌙\d{2}☀️\d{2}📊uloc/m.test(text)) {
    return "commit: stdin must not include per-day 📊uloc — script adds it to each 🎆 line";
  }
  return null;
}

function validateBulletLine(b: string): void {
  if (!MICRO_COMMIT_BULLET_RE.test(b)) {
    throw new Error(`commit: bullet must start with {emoji} then description (no space after emoji): ${b}`);
  }
  const err = bulletEmojiValidationError([b]);
  if (err) throw new Error(err.replace(/^micro-commit:/, "commit:"));
  if (BUNDLE_DATE_SECTION_RE.test(b.trim())) {
    throw new Error("commit: use a date section line `🎆YY🌙MM☀️DD` on its own, not as a bullet");
  }
}

/** 📦Parses LLM bundle body (emoji-only scope lines, dates, bullets). */
export function parseCommitBundleBody(text: string): CommitBundleSection[] {
  const bundles: CommitBundleSection[] = [];
  let current: CommitBundleSection | null = null;
  let dateSection: CommitBundleDateSection | null = null;

  for (const raw of text.split("\n")) {
    const line = raw.trim();
    if (!line || line.startsWith("#")) continue;
    if (line.startsWith("📊uloc") || line.startsWith("🔢") || line.startsWith("Signed-off-by:")) continue;

    const dateCandidate = normalizeBundleDateLine(line);
    if (BUNDLE_DATE_SECTION_RE.test(dateCandidate)) {
      if (!current) throw new Error(`commit: date section before bundle scope: ${line}`);
      if (dateSection) current.dates.push(dateSection);
      dateSection = { dateLine: dateCandidate, bullets: [] };
      continue;
    }

    if (dateSection) {
      if (isBundleScopeLine(line)) {
        const scopeErr = bundleScopeLabelError(line);
        if (scopeErr) throw new Error(scopeErr);
        current.dates.push(dateSection);
        bundles.push(current);
        current = { label: normalizeBundleScopeLabel(line), dates: [] };
        dateSection = null;
        continue;
      }
      validateBulletLine(line);
      dateSection.bullets.push(line);
      continue;
    }

    if (isBundleScopeLine(line)) {
      const scopeErr = bundleScopeLabelError(line);
      if (scopeErr) throw new Error(scopeErr);
      if (current) bundles.push(current);
      current = { label: normalizeBundleScopeLabel(line), dates: [] };
      continue;
    }

    if (!current) throw new Error(`commit: expected emoji bundle scope (two+ emojis, no paths), got: ${line}`);
    throw new Error(`commit: expected 🎆YY🌙MM☀️DD date section before bullet: ${line}`);
  }

  if (current) {
    if (dateSection) current.dates.push(dateSection);
    bundles.push(current);
  }
  if (bundles.length === 0) throw new Error("commit: at least one emoji bundle scope is required");
  for (const b of bundles) {
    if (b.dates.length === 0) throw new Error(`commit: bundle ${b.label} needs at least one date section`);
    for (const d of b.dates) {
      if (d.bullets.length === 0) throw new Error(`commit: ${d.dateLine} in ${b.label} needs at least one bullet`);
    }
  }
  return bundles;
}

function formatGitDeltaSumBrief(d: GitDeltaSum): string {
  return `➕${d.added}✏️${d.edited}➖${d.removed}🟰${gitDeltaLineTotal(d)}`;
}

function assertGitDeltaSumsEqual(a: GitDeltaSum, b: GitDeltaSum, message: string): void {
  if (gitDeltaSumsEqual(a, b)) return;
  throw new Error(`${message} — ${formatGitDeltaSumBrief(a)} vs ${formatGitDeltaSumBrief(b)}`);
}

/** 📂Bundle indices that own a numstat row (0 or 1 after validation). */
export function resolveBundleIndicesForNumstatRow(
  pathField: string,
  prefixSets: string[][],
  bundles: CommitBundleSection[],
): number[] {
  const matched = new Set<number>();
  for (const path of pathsFromNumstatRow(pathField)) {
    for (let bi = 0; bi < bundles.length; bi++) {
      if (pathMatchesBundleIndex(path, bi, prefixSets, bundles)) matched.add(bi);
    }
  }
  return [...matched];
}

function rangeGitDeltaTotal(root: string, base: string, head: string): GitDeltaSum {
  return sumGitLangDeltas(accumulateGitDeltasFromNumstat(root, gitRangeNumstat(root, base, head)));
}

/** 📊Partition WIP-range numstat across bundles (one bundle per row). */
export function partitionRangeDeltasByBundle(
  root: string,
  base: string,
  head: string,
  bundles: CommitBundleSection[],
  prefixSets: string[][],
): { bundleTotals: GitDeltaSum[]; rangeTotal: GitDeltaSum } {
  const bundleTotals = bundles.map(() => ({ added: 0, removed: 0, edited: 0 }));
  let rangeTotal: GitDeltaSum = { added: 0, removed: 0, edited: 0 };
  for (const row of gitRangeNumstat(root, base, head)) {
    const rowPaths = pathsFromNumstatRow(row.path);
    if (rowPaths.length === 0 || rowPaths.every((p) => shouldSkipPathForUloc(root, p))) continue;
    const chunk = sumGitLangDeltas(
      accumulateGitDeltasFromNumstat(root, [{ path: row.path, added: row.added, removed: row.removed }]),
    );
    if (gitDeltaLineTotal(chunk) === 0) continue;
    rangeTotal = addGitDeltaSums(rangeTotal, chunk);
    const owners = resolveBundleIndicesForNumstatRow(row.path, prefixSets, bundles);
    if (owners.length === 0) {
      throw new Error(
        `commit: changed path is not attributed to any bundle — ${row.path}; add a bundle scope or fix labels`,
      );
    }
    if (owners.length > 1) {
      const names = owners.map((i) => bundles[i]!.label).join(", ");
      throw new Error(`commit: changed path matches multiple bundles (${names}) — ${row.path}`);
    }
    const bi = owners[0]!;
    bundleTotals[bi] = addGitDeltaSums(bundleTotals[bi]!, chunk);
  }
  return { bundleTotals, rangeTotal };
}

/** 🚫Per-day uloc must sum to bundle header totals; missing/extra dates imply attribution mistakes. */
export function validateBundleDayDeltasAttribution(
  bundles: CommitBundleSection[],
  prefixSets: string[][],
  dateDeltas: BundleDateDeltasMap,
  bundleTotals: GitDeltaSum[],
): void {
  for (let bi = 0; bi < bundles.length; bi++) {
    const bundle = bundles[bi]!;
    const total = bundleTotals[bi] ?? { added: 0, removed: 0, edited: 0 };
    const listedDates = new Set(bundle.dates.map((s) => s.dateLine));
    let daySum: GitDeltaSum = { added: 0, removed: 0, edited: 0 };
    for (const dateLine of listedDates) {
      const d = dateDeltas.get(bi)?.get(dateLine) ?? { added: 0, removed: 0, edited: 0 };
      daySum = addGitDeltaSums(daySum, d);
    }
    const perDay = dateDeltas.get(bi);
    if (perDay) {
      for (const [dateLine, d] of perDay) {
        if (listedDates.has(dateLine)) continue;
        if (gitDeltaLineTotal(d) === 0) continue;
        throw new Error(
          `commit: ${bundle.label} has micro-commit changes on ${dateLine} (${formatGitDeltaSumBrief(d)}) but that day is missing from your bundle body — add a 🎆 section or fix attribution`,
        );
      }
    }
    if (
      daySum.added !== total.added ||
      daySum.edited !== total.edited ||
      daySum.removed !== total.removed
    ) {
      throw new Error(
        `commit: per-day 📊uloc for ${bundle.label} does not add up to the bundle total — days ${formatGitDeltaSumBrief(daySum)} vs bundle ${formatGitDeltaSumBrief(total)}; re-read log + diff and fix bundle/date attribution`,
      );
    }
  }
}

/** 🚫All bundle-commit uloc constraints (days→bundle, bundles→range, languages→range). */
export function validateBundleCommitAttribution(
  root: string,
  base: string,
  head: string,
  bundles: CommitBundleSection[],
  ulocRunner?: UlocRunner,
): void {
  root = gitRepoRoot(root);
  const prefixSets = buildBundlePathPrefixSets(root, base, head, bundles);
  const { bundleTotals: partitioned, rangeTotal } = partitionRangeDeltasByBundle(root, base, head, bundles, prefixSets);
  const dateDeltas = buildBundleDateDeltasMap(root, base, head, bundles);
  validateBundleDayDeltasAttribution(bundles, prefixSets, dateDeltas, partitioned);
  for (let bi = 0; bi < bundles.length; bi++) {
    let allDays: GitDeltaSum = { added: 0, removed: 0, edited: 0 };
    const perDay = dateDeltas.get(bi);
    if (perDay) {
      for (const d of perDay.values()) allDays = addGitDeltaSums(allDays, d);
    }
    assertGitDeltaSumsEqual(
      allDays,
      partitioned[bi] ?? { added: 0, removed: 0, edited: 0 },
      `commit: all micro-commit days for ${bundles[bi]!.label} do not add up to the bundle total`,
    );
  }
  let bundleSum: GitDeltaSum = { added: 0, removed: 0, edited: 0 };
  for (const t of partitioned) bundleSum = addGitDeltaSums(bundleSum, t);
  assertGitDeltaSumsEqual(
    bundleSum,
    rangeTotal,
    "commit: all bundle header totals do not add up to the WIP range 📊uloc — fix bundle attribution",
  );
  const metrics = buildMicroCommitMetricsForRange(root, base, head, undefined, ulocRunner);
  validateMicroCommitLangMetricsDeltaSum(metrics);
  const langTotal = sumMicroCommitLangMetrics(metrics);
  assertGitDeltaSumsEqual(
    { added: langTotal.added, edited: langTotal.edited, removed: langTotal.removed },
    rangeTotal,
    "commit: footer per-language 📊uloc does not add up to the WIP range total",
  );
}

export function buildCommitMessage(
  root: string,
  contributor: Contributor,
  bundles: CommitBundleSection[],
  wipSha: string,
  head = "HEAD",
  ulocRunner?: UlocRunner,
  now = new Date(),
): string {
  root = gitRepoRoot(root);
  const pathAssignments = assignChangedPathsToBundles(root, wipSha, head, bundles);
  const sorted = sortCommitBundlesByEditTotal(root, wipSha, head, bundles, pathAssignments);
  bundles = sorted.bundles;
  validateBundleCommitAttribution(root, wipSha, head, bundles, ulocRunner);
  const prefixSets = buildBundlePathPrefixSets(root, wipSha, head, bundles);
  const { bundleTotals } = partitionRangeDeltasByBundle(root, wipSha, head, bundles, prefixSets);
  const dateDeltas = buildBundleDateDeltasMap(root, wipSha, head, bundles);
  const lines: string[] = [formatBundleSubject(contributor, now), ""];
  for (let bi = 0; bi < bundles.length; bi++) {
    const bundle = bundles[bi]!;
    lines.push(formatBundleHeaderLine(bundle.label, bundleTotals[bi] ?? { added: 0, removed: 0, edited: 0 }));
    const perDay = dateDeltas.get(bi);
    for (const section of bundle.dates) {
      const dayDelta = perDay?.get(section.dateLine) ?? { added: 0, removed: 0, edited: 0 };
      lines.push(formatBundleDateLine(section.dateLine, dayDelta));
      lines.push(...section.bullets);
    }
    if (bi < bundles.length - 1) lines.push("");
  }
  const metrics = formatMicroCommitMetricsLines(buildMicroCommitMetricsForRange(root, wipSha, head, undefined, ulocRunner));
  if (metrics.length > 0) lines.push(...metrics, "");
  lines.push(`Signed-off-by: ${contributor.name} <${contributor.email}>`);
  return `${lines.join("\n")}\n`;
}

function readBodyInput(root: string, file: string | null): string {
  if (file) {
    const path = file.startsWith("/") ? file : join(root, file);
    return readFileSync(path, "utf8");
  }
  if (process.stdin.isTTY) return "";
  return readFileSync(0, "utf8");
}

function assertCleanWorktree(root: string): void {
  const st = git(root, ["status", "--porcelain"]);
  if (!st.ok) return;
  if (st.out.trim()) {
    console.error("commit: working tree must be clean before tag/squash/push");
    process.exit(1);
  }
}

function emitStdout(message: string): void {
  process.stdout.write(message.endsWith("\n") ? message : `${message}\n`);
}

const COMMIT_DIFF_MAX_BYTES = 1_500_000;

export type CommitBundleRange = { wip: { sha: string; subject: string }; range: string };

/** 🔀Resolves last bundle WIP and revision range for analysis. */
export function resolveCommitBundleRange(root: string): CommitBundleRange | null {
  root = gitRepoRoot(root);
  const wip = findLastBundleWipCommit(root);
  if (!wip) return null;
  return { wip, range: `${wip.sha}..HEAD` };
}

function normalizeCompareLine(s: string): string {
  return s.trim().replace(/\s+/g, " ").toLowerCase();
}

/** 📜Collects comparable lines from commit bodies in range (for copy detection). */
export function commitHistoryCompareLines(root: string, base: string, head: string): Set<string> {
  const r = spawnSync("git", ["log", `--format=%B%x00`, `${base}..${head}`], {
    cwd: gitRepoRoot(root),
    encoding: "utf8",
    maxBuffer: 32 * 1024 * 1024,
  });
  const lines = new Set<string>();
  if (r.status !== 0) return lines;
  for (const body of (r.stdout ?? "").split("\0")) {
    if (!body.trim()) continue;
    for (const raw of body.split("\n")) {
      const line = raw.trim();
      if (line.length < 12) continue;
      if (BUNDLE_WIP_SUBJECT_RE.test(line)) continue;
      if (BUNDLE_DATE_SECTION_RE.test(normalizeBundleDateLine(line))) continue;
      if (line.startsWith("📊uloc") || line.startsWith("🔢")) continue;
      if (line.startsWith("Signed-off-by:")) continue;
      if (/^🎆\d{2}🌙\d{2}☀️\d{2}⏰/u.test(line)) continue;
      lines.add(normalizeCompareLine(line));
    }
  }
  return lines;
}

/** 🚫Whether a bullet verbatim-matches a line from a prior commit body in the range. */
export function bulletMatchesCommitHistory(bullet: string, history: Set<string>): boolean {
  return history.has(normalizeCompareLine(bullet));
}

/** 🚫Ensures bullets are newly written from diff analysis, not pasted from prior commits. */
export function validateBundleBulletsFresh(root: string, base: string, head: string, bundles: CommitBundleSection[]): void {
  const history = commitHistoryCompareLines(root, base, head);
  for (const bundle of bundles) {
    for (const section of bundle.dates) {
      for (const bullet of section.bullets) {
        if (bulletMatchesCommitHistory(bullet, history)) {
          throw new Error(
            `commit: bullet copies a prior commit message line — rewrite from git diff only: ${bullet}`,
          );
        }
      }
    }
  }
}

function emitCommitBundleAttributionNote(): void {
  console.error(
    "commit: bundles and file→bundle mapping are NOT automatic — folder layout and bundle boundaries change between WIPs",
  );
  console.error(
    "commit: you must (1) read log for last bundle/WIP state, (2) read diff --stat + full diff for every path, (3) decide scopes/dates/bullets, then prepare stdin",
  );
  console.error(
    "commit: script only adds subject, uloc suffixes, sort order, footer, Signed-off-by — never invents bundles or bullets",
  );
  console.error(
    "commit: prepare/check fail unless days→bundle, bundles→range, and languages→range (all ➕✏️➖🟰); run: bun ./script.ts commit check",
  );
}

function emitCommitAnalysisHint(): void {
  console.error("commit: write NEW bundle scopes, dates, and bullets on prepare stdin after log + diff attribution");
}

/** 📜Prior commit messages since last WIP (context for dates only — not for copying bullets). */
export function emitCommitLog(root: string): void {
  root = gitRepoRoot(root);
  const resolved = resolveCommitBundleRange(root);
  if (!resolved) {
    console.error("commit: no prior bundle WIP commit (subject …🔀) found in recent history");
    process.exit(1);
  }
  const { wip, range } = resolved;
  console.error(`Last bundle WIP: ${wip.subject} (${wip.sha.slice(0, 7)})`);
  console.error(`Range: ${range}\n`);
  emitCommitBundleAttributionNote();
  console.error("\n=== prior commit messages (context only — do not copy bullets; old format may be wrong) ===\n");
  const log = git(root, ["log", `--format=commit %h%n%s%n%b%n---`, range]);
  if (log.ok && log.out) console.error(log.out);
  emitCommitAnalysisHint();
}

/** 📊Git diff since last WIP (primary source for new bundle summaries). */
export function emitCommitDiff(root: string): void {
  root = gitRepoRoot(root);
  const resolved = resolveCommitBundleRange(root);
  if (!resolved) {
    console.error("commit: no prior bundle WIP commit (subject …🔀) found in recent history");
    process.exit(1);
  }
  const { wip, range } = resolved;
  console.error(`Last bundle WIP: ${wip.subject} (${wip.sha.slice(0, 7)})`);
  console.error(`Range: ${range}\n`);
  const stat = git(root, ["diff", "--stat", range]);
  emitCommitBundleAttributionNote();
  console.error("\n=== git diff --stat (overview) ===\n");
  if (stat.ok && stat.out) console.error(`${stat.out}\n`);
  const patch = git(root, ["diff", range]);
  console.error("=== git diff (write bullets from this — not from prior commit text) ===\n");
  if (!patch.ok || !patch.out) {
    console.error(patch.out || "(empty diff)\n");
    emitCommitAnalysisHint();
    return;
  }
  const bytes = Buffer.byteLength(patch.out, "utf8");
  if (bytes > COMMIT_DIFF_MAX_BYTES) {
    console.error(patch.out.slice(0, COMMIT_DIFF_MAX_BYTES));
    console.error(`\n[commit diff truncated at ${COMMIT_DIFF_MAX_BYTES} bytes of ${bytes} — inspect locally: git diff ${range}]\n`);
  } else {
    console.error(`${patch.out}\n`);
  }
  emitCommitAnalysisHint();
}

function emitCommitAnalysis(root: string): void {
  emitCommitLog(root);
  console.error("\n");
  emitCommitDiff(root);
}

export function runCommit(root: string, segments: string[]): void {
  root = gitRepoRoot(root);
  const cmd = segments[0] ?? "prepare";
  if (!branchAllowed(root)) {
    console.error("commit: branch must contain ⛳wip or 🏗️dev");
    process.exit(1);
  }
  const contributor = findContributor(root);
  if (!contributor) {
    console.error(`commit: no contributor for git user.email ${gitEmail(root) || "(unset)"}`);
    process.exit(1);
  }

  if (cmd === "log") {
    emitCommitLog(root);
    process.exit(0);
  }

  if (cmd === "diff") {
    emitCommitDiff(root);
    process.exit(0);
  }

  if (cmd === "analyze") {
    emitCommitAnalysis(root);
    process.exit(0);
  }

  if (cmd === "check") {
    const dash = segments.indexOf("--");
    const bodyFile = dash >= 0 ? (segments[dash + 1] ?? null) : null;
    const body = readBodyInput(root, bodyFile);
    if (!body.trim()) {
      console.error("commit check: pass bundle body on stdin or after -- body.txt");
      process.exit(1);
    }
    const bodyErr = commitBundleBodyError(body);
    if (bodyErr) {
      console.error(bodyErr);
      process.exit(1);
    }
    const wip = findLastBundleWipCommit(root);
    if (!wip) {
      console.error("commit: no prior bundle WIP commit (subject …🔀) found");
      process.exit(1);
    }
    const ahead = git(root, ["rev-list", "--count", `${wip.sha}..HEAD`]);
    if (!ahead.ok || Number(ahead.out) === 0) {
      console.error("commit: no commits after last bundle WIP — nothing to check");
      process.exit(1);
    }
    try {
      const bundles = parseCommitBundleBody(body);
      validateBundleBulletsFresh(root, wip.sha, "HEAD", bundles);
      validateBundleCommitAttribution(root, wip.sha, "HEAD", bundles);
    } catch (e) {
      console.error(e instanceof Error ? e.message : String(e));
      process.exit(1);
    }
    const range = rangeGitDeltaTotal(root, wip.sha, "HEAD");
    console.error(`commit check: OK — ${formatGitDeltaSumBrief(range)}`);
    console.error("commit check: per-bundle days → bundle headers → WIP range total; per-language footer → same total");
    process.exit(0);
  }

  if (cmd !== "prepare") {
    console.error("[commit] usage: bun ./script.ts commit <log|diff|analyze|check|prepare> [ct|cs|cp|…] [-- body.txt]");
    process.exit(1);
  }

  const dash = segments.indexOf("--");
  const levelSegments = dash >= 0 ? segments.slice(1, dash) : segments.slice(1);
  const bodyFile = dash >= 0 ? (segments[dash + 1] ?? null) : null;
  const steps = loadCommitSteps(root, contributor, levelSegments);
  const prepareOnly = isCommitPrepareOnly(levelSegments);
  const messagePath = join(gitDir(root), "semio-commit-message");

  const wip = findLastBundleWipCommit(root);
  if (!wip) {
    console.error("commit: no prior bundle WIP commit (subject …🔀) found");
    process.exit(1);
  }

  const ahead = git(root, ["rev-list", "--count", `${wip.sha}..HEAD`]);
  if (!ahead.ok || Number(ahead.out) === 0) {
    console.error("commit: no commits after last bundle WIP — nothing to bundle");
    process.exit(1);
  }

  const body = readBodyInput(root, bodyFile);
  let message: string;
  if (body.trim()) {
    const bodyErr = commitBundleBodyError(body);
    if (bodyErr) {
      console.error(bodyErr);
      process.exit(1);
    }
    let bundles: CommitBundleSection[];
    try {
      bundles = parseCommitBundleBody(body);
      validateBundleBulletsFresh(root, wip.sha, "HEAD", bundles);
    } catch (e) {
      console.error(e instanceof Error ? e.message : String(e));
      process.exit(1);
    }
    try {
      message = buildCommitMessage(root, contributor, bundles, wip.sha);
    } catch (e) {
      console.error(e instanceof Error ? e.message : String(e));
      process.exit(1);
    }
    writeFileSync(messagePath, message);
  } else if (existsSync(messagePath)) {
    message = readFileSync(messagePath, "utf8");
    if (!message.trim()) {
      console.error("commit: semio-commit-message is empty — run prepare with bundle body first");
      process.exit(1);
    }
  } else {
    emitCommitAnalysis(root);
    process.exit(1);
  }

  if (prepareOnly) {
    if (!body.trim()) {
      emitCommitAnalysis(root);
      process.exit(1);
    }
    const tagName = formatBundleTagName(contributor);
    emitStdout(
      formatCommitPrepareAgentReply({
        tagName,
        wipSha: wip.sha,
        messageFile: ".git/semio-commit-message",
        commitMessage: message,
      }),
    );
    process.exit(0);
  }

  if (steps.tag || steps.squash || steps.push) assertCleanWorktree(root);

  if (steps.tag) {
    const tagName = formatBundleTagName(contributor);
    const tag = spawnSync("git", ["tag", "-s", "-m", tagName, tagName, "HEAD"], { cwd: root, encoding: "utf8" });
    if (tag.status !== 0) {
      console.error((tag.stderr ?? tag.stdout ?? "git tag failed").trim());
      process.exit(tag.status ?? 1);
    }
  }

  if (steps.squash) {
    const reset = spawnSync("git", ["reset", "--soft", wip.sha], { cwd: root, encoding: "utf8" });
    if (reset.status !== 0) {
      console.error((reset.stderr ?? reset.stdout ?? "git reset --soft failed").trim());
      process.exit(reset.status ?? 1);
    }
    writeFileSync(join(gitDir(root), "COMMIT_EDITMSG"), message);
    const commit = spawnSync("git", ["commit", "-S", "-F", join(gitDir(root), "COMMIT_EDITMSG")], {
      cwd: root,
      encoding: "utf8",
    });
    if (commit.status !== 0) {
      console.error((commit.stderr ?? commit.stdout ?? "git commit failed").trim());
      process.exit(commit.status ?? 1);
    }
  }

  if (steps.push) {
    const push = spawnSync("git", ["push", "--follow-tags"], { cwd: root, encoding: "utf8" });
    if (push.status !== 0) {
      console.error((push.stderr ?? push.stdout ?? "git push failed").trim());
      process.exit(push.status ?? 1);
    }
  }

  emitStdout(message);
  process.exit(0);
}
