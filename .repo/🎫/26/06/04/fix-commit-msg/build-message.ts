import { readFileSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import {
  parseCommitBundleBody,
  labelPathTokens,
  extractBundleDateLineFromCommit,
  formatBundleDateLine,
  formatBundleSubject,
  normalizeBundleScopeLabel,
  pathsFromNumstatRow,
} from "../../../../../../repo/lib/js/src/commit.ts";
import {
  gitRangeNumstat,
  shouldSkipPathForUloc,
  sumGitLangDeltas,
  accumulateGitDeltasFromNumstat,
  gitDeltaLineTotal,
  buildMicroCommitMetricsForRange,
  formatMicroCommitMetricsLines,
  formatBundleUlocSuffix,
} from "../../../../../../repo/lib/js/src/uloc-metrics.ts";

function formatBundleHeaderLine(label: string, total: GitDeltaSum): string {
  return `${normalizeBundleScopeLabel(label)}${formatBundleUlocSuffix(total)}`;
}

const root = "/Users/ueli/Documents/compose";
const wip = "6831b35f5e7b0edc35e97c8631c2458bada62bf5";
const head = "a9ba9ef2c";
const microHead = "4d17010754182decd30a2dda29b12a86fe7c115b";

function git(args: string[]): string {
  const r = spawnSync("git", args, { cwd: root, encoding: "utf8" });
  return r.status === 0 ? (r.stdout ?? "").trim() : "";
}

function add(a: GitDeltaSum, b: GitDeltaSum): GitDeltaSum {
  return { added: a.added + b.added, removed: a.removed + b.removed, edited: a.edited + b.edited };
}

type GitDeltaSum = { added: number; removed: number; edited: number };

function findRemainderBundleIndex(bundles: CommitBundleSection[]): number {
  const i = bundles.findIndex((b) => /workspace/i.test(normalizeBundleScopeLabel(b.label)));
  if (i < 0) throw new Error("commit: missing 🌐workspace… remainder bundle in body");
  return i;
}

function assignPrimary(bundles: CommitBundleSection[], path: string, remainderIdx: number): number {
  const pl = path.toLowerCase();
  let best = -1;
  let bestScore = 0;
  for (let i = 0; i < bundles.length; i++) {
    if (i === remainderIdx) continue;
    const tokens = labelPathTokens(bundles[i]!.label);
    if (!tokens.every((t) => pl.includes(t))) continue;
    if (tokens.length > bestScore) {
      bestScore = tokens.length;
      best = i;
    }
  }
  return best >= 0 ? best : remainderIdx;
}

function lastCommitShaTouchingPath(base: string, headRef: string, path: string): string | null {
  if (shouldSkipPathForUloc(root, path)) return null;
  const out = git(["log", "-1", "--format=%H", `${base}..${headRef}`, "--", path]);
  return out || null;
}

function dateLineForSha(sha: string): string | null {
  const subject = git(["log", "-1", "--format=%s", sha]);
  const body = git(["log", "-1", "--format=%B", sha]);
  return extractBundleDateLineFromCommit(subject, body);
}

type CommitBundleSection = ReturnType<typeof parseCommitBundleBody>[number];

function extractBodyFromMessage(raw: string): string {
  const bodyLines: string[] = [];
  let inBody = false;
  for (const line of raw.split("\n")) {
    if (line.startsWith("🐙") && line.endsWith("🔀")) {
      inBody = true;
      continue;
    }
    if (!inBody) continue;
    if (/^📊uloc➕/.test(line) && !/^🎆/.test(line) && bodyLines.length > 0) break;
    const stripped = line.replace(/📊uloc.*$/u, "").trim();
    if (!stripped) {
      if (bodyLines.length) bodyLines.push("");
      continue;
    }
    bodyLines.push(stripped);
  }
  while (bodyLines.at(-1) === "") bodyLines.pop();
  return `${bodyLines.join("\n")}\n`;
}

const remainderBody = `
🌐workspace🧩other
🎆26🌙06☀️04
🌐Framework hosts, UI assets, infinite cavas, agents skills, and monorepo paths outside named bundles
🎆26🌙06☀️03
🌐Mathematical graph crates, CAD JSON, reasoning notes, and compose core outside fixtures
🎆26🌙06☀️02
🌐Storybook, cursor plans, and workspace scaffolding touched during the sprint
`;

const raw = readFileSync("/tmp/current-msg.txt", "utf8");
let body = extractBodyFromMessage(raw);
if (!body.includes("🌐workspace")) body += remainderBody;
let bundles = parseCommitBundleBody(body);
const remainderIdx = findRemainderBundleIndex(bundles);

const headers = bundles.map(() => ({ added: 0, removed: 0, edited: 0 }));
const dateDeltas: Map<string, GitDeltaSum>[] = bundles.map(() => new Map());

for (const row of gitRangeNumstat(root, wip, head)) {
  const rowPaths = pathsFromNumstatRow(row.path);
  if (!rowPaths.length || rowPaths.every((p) => shouldSkipPathForUloc(root, p))) continue;
  const chunk = sumGitLangDeltas(
    accumulateGitDeltasFromNumstat(root, [{ path: row.path, added: row.added, removed: row.removed }]),
  );
  if (gitDeltaLineTotal(chunk) === 0) continue;
  const bi = assignPrimary(bundles, rowPaths[0] ?? row.path, remainderIdx);
  headers[bi] = add(headers[bi]!, chunk);
  let sha: string | null = null;
  for (const p of rowPaths) {
    sha = lastCommitShaTouchingPath(wip, microHead, p) ?? lastCommitShaTouchingPath(wip, head, p);
    if (sha) break;
  }
  let dateLine = sha ? dateLineForSha(sha) : null;
  if (!dateLine) dateLine = bundles[bi]!.dates[0]?.dateLine ?? "🎆26🌙06☀️04";
  const m = dateDeltas[bi]!;
  m.set(dateLine, add(m.get(dateLine) ?? { added: 0, removed: 0, edited: 0 }, chunk));
}

for (let bi = 0; bi < bundles.length; bi++) {
  let allDays: GitDeltaSum = { added: 0, removed: 0, edited: 0 };
  for (const d of dateDeltas[bi]!.values()) allDays = add(allDays, d);
  const h = headers[bi]!;
  if (allDays.added !== h.added || allDays.removed !== h.removed || allDays.edited !== h.edited) {
    throw new Error(`internal: range day map != header for ${bundles[bi]!.label}`);
  }
}

for (let bi = 0; bi < bundles.length; bi++) {
  const bundle = bundles[bi]!;
  const fallbackDay = bundle.dates[0]?.dateLine;
  if (!fallbackDay) throw new Error(`bundle ${bundle.label} has no 🎆 dates`);
  for (const [dateLine, d] of [...dateDeltas[bi]!.entries()]) {
    if (bundle.dates.some((s) => s.dateLine === dateLine)) continue;
    if (gitDeltaLineTotal(d) === 0) {
      dateDeltas[bi]!.delete(dateLine);
      continue;
    }
    dateDeltas[bi]!.set(fallbackDay, add(dateDeltas[bi]!.get(fallbackDay) ?? { added: 0, removed: 0, edited: 0 }, d));
    dateDeltas[bi]!.delete(dateLine);
  }
}

const ranked = bundles
  .map((bundle, i) => ({ bundle, i, total: gitDeltaLineTotal(headers[i]!) }))
  .sort((a, b) => b.total - a.total);
bundles = ranked.map((r) => r.bundle);
const sortedHeaders = ranked.map((r) => headers[r.i]!);
const sortedDateDeltas = ranked.map((r) => dateDeltas[r.i]!);

const contributor = { alias: "ueli", emoji: "🐙", name: "Ueli Saluz", email: "ueli@semio-tech.com" };
const lines: string[] = [formatBundleSubject(contributor, new Date("2026-06-04")), ""];
for (let bi = 0; bi < bundles.length; bi++) {
  const bundle = bundles[bi]!;
  lines.push(formatBundleHeaderLine(bundle.label, sortedHeaders[bi]!));
  for (const section of bundle.dates) {
    const dayDelta = sortedDateDeltas[bi]!.get(section.dateLine) ?? { added: 0, removed: 0, edited: 0 };
    lines.push(formatBundleDateLine(section.dateLine, dayDelta));
    lines.push(...section.bullets);
  }
  if (bi < bundles.length - 1) lines.push("");
}
const metrics = formatMicroCommitMetricsLines(buildMicroCommitMetricsForRange(root, wip, head));
if (metrics.length) lines.push(...metrics, "");
lines.push("Signed-off-by: Ueli Saluz <ueli@semio-tech.com>");
const message = `${lines.join("\n")}\n`;

for (let bi = 0; bi < bundles.length; bi++) {
  let listed: GitDeltaSum = { added: 0, removed: 0, edited: 0 };
  for (const s of bundles[bi]!.dates) {
    listed = add(listed, sortedDateDeltas[bi]!.get(s.dateLine) ?? { added: 0, removed: 0, edited: 0 });
  }
  const h = sortedHeaders[bi]!;
  if (listed.added !== h.added || listed.removed !== h.removed || listed.edited !== h.edited) {
    throw new Error(`listed days != header for ${bundles[bi]!.label}`);
  }
}

writeFileSync("/Users/ueli/Documents/compose/.git/compose-commit-message", message);
process.stdout.write(message);
