#!/usr/bin/env bun
/**
 * 🏗️ Records the `📊️metrics` language-agnostic fixtures from the REAL `git` CLI.
 *
 * It builds a throw-away repository in a temporary directory whose commits are fully
 * deterministic (fixed author, fixed committer, fixed dates), captures exactly what
 * `git log --numstat` answers, and derives the golden projections with a parser written here from
 * the specification — independently of the Rust and Go implementations under test.
 *
 * Run: `bun ./.🧬semio/…/🏗️record-metrics-fixtures.ts`
 */
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";

const REPO_ROOT = process.env.SEMIO_REPO_ROOT ?? process.cwd();
const FIXTURES = join(REPO_ROOT, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "📊️metrics", "🧫️fixtures");

const AUTHOR = { name: "Ada Lovelace", email: "ada@semio-tech.com" };
const SECOND_AUTHOR = { name: "Grace Hopper", email: "grace@semio-tech.com" };
const SIDE_CONTENT = "pub fn side() -> u8 { 1 }" + String.fromCharCode(10);

type Change = { path: string; content?: string; bytes?: number[]; rename?: string; delete?: boolean };
type Recipe = { message: string; when: number; author: { name: string; email: string }; changes: Change[] };

const RECIPE: Recipe[] = [
  {
    message: "seed the tree",
    when: 1_767_225_600, // 2026-01-01T00:00:00Z
    author: AUTHOR,
    changes: [
      { path: "🧬️alpha.ts", content: "export const one = 1;\nexport const two = 2;\nexport const three = 3;\n" },
      { path: "README.md", content: "# Fixture\n\nA recorded repository.\n" },
      { path: "data/config.json", content: '{\n  "a": 1,\n  "b": { "c": 2 }\n}\n' },
    ],
  },
  {
    message: "grow the code",
    when: 1_767_232_800, // +2h
    author: SECOND_AUTHOR,
    changes: [
      { path: "🧬️alpha.ts", content: "export const one = 1;\nexport const two = 22;\nexport const three = 3;\nexport const four = 4;\n" },
      { path: "pkg/main.go", content: "package main\n\nfunc main() {}\n" },
    ],
  },
  {
    message: "rename into src",
    when: 1_767_312_000, // next day
    author: AUTHOR,
    changes: [{ path: "src/🧬️beta.ts", rename: "🧬️alpha.ts", content: "export const one = 1;\nexport const two = 22;\nexport const three = 33;\nexport const four = 4;\nexport const five = 5;\n" }],
  },
  {
    message: "add a binary asset and a hidden file",
    when: 1_767_398_400, // next day
    author: SECOND_AUTHOR,
    changes: [
      { path: "assets/logo.png", bytes: [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x01, 0x02, 0x03] },
      { path: ".hidden/secret.ts", content: "export const hidden = true;\n" },
      { path: ".🧬semio/🦑️repo/note.md", content: "meta only\n" },
    ],
  },
  {
    message: "drop the readme, extend the data",
    when: 1_769_904_000, // 2026-02-01
    author: AUTHOR,
    changes: [
      { path: "README.md", delete: true },
      { path: "data/config.json", content: '{\n  "a": 1,\n  "b": { "c": 2, "d": [1, 2, { "e": 3 }] },\n  "f": 4\n}\n' },
      { path: "py/tool.py", content: "def main():\n    return 0\n" },
    ],
  },
];

function git(cwd: string, args: string[], env: Record<string, string> = {}): string {
  const result = spawnSync("git", args, { cwd, env: { ...process.env, ...env }, encoding: "buffer", maxBuffer: 64 * 1024 * 1024 });
  if (result.status !== 0) throw new Error(`git ${args.join(" ")} failed: ${result.stderr?.toString("utf8")}`);
  return result.stdout.toString("utf8");
}

function buildRepository(): { dir: string; shas: string[] } {
  const dir = mkdtempSync(join(tmpdir(), "semio-metrics-"));
  git(dir, ["init", "--quiet", "--initial-branch=⛳️wip"]);
  git(dir, ["config", "user.name", AUTHOR.name]);
  git(dir, ["config", "user.email", AUTHOR.email]);
  git(dir, ["config", "core.quotepath", "true"]);
  git(dir, ["config", "diff.renames", "true"]);
  const shas: string[] = [];
  for (const commit of RECIPE) {
    for (const change of commit.changes) {
      const target = join(dir, change.path);
      if (change.delete) {
        git(dir, ["rm", "--quiet", "--", change.path]);
        continue;
      }
      mkdirSync(dirname(target), { recursive: true });
      if (change.rename !== undefined) git(dir, ["mv", change.rename, change.path]);
      if (change.bytes !== undefined) writeFileSync(target, Buffer.from(change.bytes));
      else writeFileSync(target, change.content ?? "", "utf8");
      git(dir, ["add", "--", change.path]);
    }
    const stamp = `${commit.when} +0000`;
    git(dir, ["commit", "--quiet", "-m", commit.message], {
      GIT_AUTHOR_NAME: commit.author.name,
      GIT_AUTHOR_EMAIL: commit.author.email,
      GIT_AUTHOR_DATE: stamp,
      GIT_COMMITTER_NAME: commit.author.name,
      GIT_COMMITTER_EMAIL: commit.author.email,
      GIT_COMMITTER_DATE: stamp,
    });
    shas.push(git(dir, ["rev-parse", "HEAD"]).trim());
  }
  // 🔀️ A side branch merged back, so the transcript also carries a merge commit.
  git(dir, ["checkout", "--quiet", "-b", "side", shas[1]!]);
  writeFileSync(join(dir, "side.rs"), "pub fn side() -> u8 { 1 }\n", "utf8");
  git(dir, ["add", "--", "side.rs"]);
  git(dir, ["commit", "--quiet", "-m", "side work"], { GIT_AUTHOR_NAME: SECOND_AUTHOR.name, GIT_AUTHOR_EMAIL: SECOND_AUTHOR.email, GIT_AUTHOR_DATE: "1767318000 +0000", GIT_COMMITTER_NAME: SECOND_AUTHOR.name, GIT_COMMITTER_EMAIL: SECOND_AUTHOR.email, GIT_COMMITTER_DATE: "1767318000 +0000" });
  git(dir, ["checkout", "--quiet", "⛳️wip"]);
  git(dir, ["merge", "--quiet", "--no-ff", "-m", "merge side", "side"], { GIT_AUTHOR_NAME: AUTHOR.name, GIT_AUTHOR_EMAIL: AUTHOR.email, GIT_AUTHOR_DATE: "1769990400 +0000", GIT_COMMITTER_NAME: AUTHOR.name, GIT_COMMITTER_EMAIL: AUTHOR.email, GIT_COMMITTER_DATE: "1769990400 +0000" });
  shas.push(git(dir, ["rev-parse", "HEAD"]).trim());
  return { dir, shas };
}

const PRETTY = "COMMIT%x09%H%x09%aN%x09%aE%x09%at";
const BASE_ARGS = ["log", "--numstat", "--first-parent", "--reverse", `--pretty=format:${PRETTY}`];

function record(dir: string, shas: string[]): Record<string, unknown> {
  const logs: Record<string, string> = {};
  logs[""] = git(dir, [...BASE_ARGS.slice(0, 1), "--no-merges", ...BASE_ARGS.slice(1)]);
  logs["⛳️wip"] = git(dir, [...BASE_ARGS.slice(0, 1), "--no-merges", ...BASE_ARGS.slice(1), "⛳️wip"]);
  logs["🔀️with-merges"] = git(dir, [...BASE_ARGS, "⛳️wip"]);
  const tracked: Record<string, string[]> = {};
  const blobs: Record<string, string> = {};
  const refs = ["", ...shas];
  for (const ref of refs) {
    const raw = ref === "" ? git(dir, ["ls-files", "-z"]) : git(dir, ["ls-tree", "-r", "--name-only", "-z", ref]);
    const paths = raw.split("\0").filter(Boolean);
    tracked[ref] = paths;
    for (const path of paths) {
      const result = spawnSync("git", ref === "" ? ["show", `HEAD:${path}`] : ["show", `${ref}:${path}`], { cwd: dir, encoding: "buffer", maxBuffer: 64 * 1024 * 1024 });
      if (result.status !== 0) continue;
      const buffer = result.stdout;
      if (buffer.includes(0)) continue;
      blobs[`${ref}${path}`] = buffer.toString("utf8");
    }
  }
  return { id: "metrics-recorded-repository", logs, tracked, blobs };
}

//#region 🔬️Independent reference implementation (the golden generator)
const AGG_MARKUP = "Markup";
const AGG_DATA = "Data";
const CODE = ["TypeScript", "Go", "C#", "Python", "Rust"];

function extensionOf(path: string): string {
  const base = path.slice(Math.max(path.lastIndexOf("/"), path.lastIndexOf("\\")) + 1);
  const dot = base.lastIndexOf(".");
  return dot < 0 ? "" : base.slice(dot).toLowerCase();
}

function classify(path: string): string {
  const extension = extensionOf(path);
  if ([".ts", ".tsx", ".cts", ".mts", ".mtsx"].includes(extension)) return "TypeScript";
  if (extension === ".go") return "Go";
  if (extension === ".cs") return "C#";
  if (extension === ".py") return "Python";
  if (extension === ".rs") return "Rust";
  if ([".html", ".htm", ".xhtml", ".md", ".markdown", ".mdown", ".mkd", ".mdx", ".mdc", ".svx", ".svxtheme"].includes(extension)) return AGG_MARKUP;
  if ([".json", ".jsonc", ".yaml", ".yml", ".toml", ".csv", ".xml", ".ini", ".cfg", ".conf", ".properties", ".editorconfig", ".gitattributes", ".gitmodules"].includes(extension)) return AGG_DATA;
  return "";
}

function unquote(raw: string): string {
  const trimmed = raw.trim();
  if (trimmed.length < 2 || !trimmed.startsWith('"') || !trimmed.endsWith('"')) return raw;
  const inner = trimmed.slice(1, -1);
  const bytes: number[] = [];
  for (let index = 0; index < inner.length; ) {
    if (inner[index] !== "\\") {
      for (const byte of Buffer.from(inner[index]!, "utf8")) bytes.push(byte);
      index += 1;
      continue;
    }
    index += 1;
    const escape = inner[index]!;
    index += 1;
    const simple: Record<string, number> = { n: 10, t: 9, r: 13, a: 7, b: 8, f: 12, v: 11, '"': 34, "\\": 92 };
    if (escape in simple) {
      bytes.push(simple[escape]!);
      continue;
    }
    if (escape >= "0" && escape <= "7") {
      let value = escape.charCodeAt(0) - 48;
      for (let taken = 1; taken < 3 && index < inner.length && inner[index]! >= "0" && inner[index]! <= "7"; taken += 1) {
        value = value * 8 + (inner[index]!.charCodeAt(0) - 48);
        index += 1;
      }
      bytes.push(value & 0xff);
      continue;
    }
    for (const byte of Buffer.from(escape, "utf8")) bytes.push(byte);
  }
  return Buffer.from(bytes).toString("utf8");
}

function resolvePath(field: string): { path: string; renameFrom: string | null } {
  const open = field.indexOf("{");
  const close = field.indexOf("}");
  if (open >= 0 && close > open) {
    const middle = field.slice(open + 1, close);
    const arrow = middle.indexOf(" => ");
    if (arrow >= 0) {
      const prefix = field.slice(0, open);
      const suffix = field.slice(close + 1);
      const collapse = (value: string): string => value.replace(/\/{2,}/g, "/");
      return { path: unquote(collapse(prefix + middle.slice(arrow + 4) + suffix)), renameFrom: unquote(collapse(prefix + middle.slice(0, arrow) + suffix)) };
    }
  }
  const arrow = field.indexOf(" => ");
  if (arrow >= 0) return { path: unquote(field.slice(arrow + 4).trim()), renameFrom: unquote(field.slice(0, arrow).trim()) };
  return { path: unquote(field), renameFrom: null };
}

function skipped(rel: string): boolean {
  const path = rel.replace(/\\/g, "/").replace(/^\.\//, "");
  if (!path || path === ".🧬semio" || path.startsWith(".🧬semio/")) return true;
  return path.split("/").some((segment) => segment && segment !== "." && segment !== ".." && segment.startsWith("."));
}

type ParsedCommit = { sha: string; when_unix: number; author: string; author_mail: string; files: unknown[]; delta: Record<string, { added: number; removed: number }> };

function parseNumstat(stdout: string, weights: Set<string>): ParsedCommit[] {
  const commits: ParsedCommit[] = [];
  let current: ParsedCommit | null = null;
  for (const raw of stdout.split("\n")) {
    const line = raw.replace(/\r$/, "");
    if (!line) continue;
    if (line.startsWith("COMMIT")) {
      const fields = line.split("\t");
      if (fields.length < 5) continue;
      if (current) commits.push(current);
      current = { sha: fields[1]!, author: fields[2]!, author_mail: fields[3]!, when_unix: Number.parseInt(fields[4]!, 10), files: [], delta: {} };
      continue;
    }
    if (!current) continue;
    const parts = line.split("\t");
    if (parts.length < 3) continue;
    const field = parts.slice(2).join("\t");
    const binary = parts[0] === "-" || parts[1] === "-";
    const added = binary ? 0 : Number.parseInt(parts[0]!, 10);
    const removed = binary ? 0 : Number.parseInt(parts[1]!, 10);
    if (!binary && (Number.isNaN(added) || Number.isNaN(removed))) continue;
    const { path, renameFrom } = resolvePath(field);
    const language = classify(path);
    const bucket = binary || skipped(path) || !weights.has(language) ? null : language;
    if (bucket) {
      const entry = (current.delta[bucket] ??= { added: 0, removed: 0 });
      entry.added += added;
      entry.removed += removed;
    }
    const file: Record<string, unknown> = { path, added, removed, binary };
    if (renameFrom !== null) file.rename_from = renameFrom;
    if (bucket) file.bucket = bucket;
    current.files.push(file);
  }
  if (current) commits.push(current);
  return commits;
}

function countJsonKeysValue(value: unknown): number {
  if (value === null || typeof value !== "object") return 0;
  if (Array.isArray(value)) return value.reduce<number>((total, item) => total + countJsonKeysValue(item), 0);
  const record = value as Record<string, unknown>;
  return Object.keys(record).length + Object.values(record).reduce<number>((total, item) => total + countJsonKeysValue(item), 0);
}

function unifiedLoc(rel: string, text: string): number {
  const physical = text.length === 0 ? 0 : text.split("\n").length - (text.endsWith("\n") ? 0 : 0) + (text.endsWith("\n") ? 0 : 0);
  const lines = text.length === 0 ? 0 : (text.match(/\n/g)?.length ?? 0) + 1;
  void physical;
  const extension = extensionOf(rel);
  if (extension === ".json" || extension === ".jsonc") {
    if (!text.trim()) return 0;
    try {
      const keys = countJsonKeysValue(JSON.parse(text));
      if (keys > 0) return keys;
    } catch {
      /* falls through to physical lines */
    }
  }
  return lines;
}

function scanCounts(transcript: { tracked: Record<string, string[]>; blobs: Record<string, string> }, ref: string, weights: Set<string>): Record<string, number> {
  const counts: Record<string, number> = {};
  for (const bucket of weights) counts[bucket] = 0;
  for (const rel of transcript.tracked[ref] ?? []) {
    if (skipped(rel)) continue;
    const bucket = classify(rel);
    if (!bucket || !weights.has(bucket)) continue;
    const body = transcript.blobs[`${ref}${rel}`];
    if (body === undefined) continue;
    counts[bucket] = (counts[bucket] ?? 0) + unifiedLoc(rel, body);
  }
  return counts;
}

type Row = { loc: number; percent: number; wip_percent: number; edited: number; added: number; removed: number };

function compose(cumulative: Record<string, { added: number; removed: number }>, scan: Record<string, number>, languages: string[], wipDenominator: number): Record<string, Row> {
  const weights = new Set([...languages, AGG_MARKUP, AGG_DATA]);
  const rows: Record<string, Row> = {};
  const make = (key: string): Row => {
    const pair = cumulative[key] ?? { added: 0, removed: 0 };
    return { loc: scan[key] ?? 0, percent: 0, wip_percent: 0, edited: pair.added + pair.removed, added: pair.added, removed: pair.removed };
  };
  const code: Row = { loc: 0, percent: 0, wip_percent: 0, edited: 0, added: 0, removed: 0 };
  for (const language of CODE) {
    if (!weights.has(language)) continue;
    const row = make(language);
    rows[language] = row;
    code.loc += row.loc;
    code.added += row.added;
    code.removed += row.removed;
    code.edited += row.edited;
  }
  const markup = make(AGG_MARKUP);
  const data = make(AGG_DATA);
  rows[AGG_MARKUP] = markup;
  rows[AGG_DATA] = data;
  rows.Code = code;
  rows.Total = { loc: code.loc + markup.loc + data.loc, percent: 0, wip_percent: 0, edited: code.edited + markup.edited + data.edited, added: code.added + markup.added + data.added, removed: code.removed + markup.removed + data.removed };
  const denominator = rows.Total.loc;
  for (const [key, row] of Object.entries(rows)) row.percent = denominator <= 0 ? 0 : key === "Total" ? 100 : Math.round((10000 * row.loc) / denominator) / 100;
  const churn = wipDenominator > 0 ? wipDenominator : rows.Total.edited;
  for (const row of Object.values(rows)) row.wip_percent = churn <= 0 ? 0 : Math.round((10000 * row.edited) / churn) / 100;
  return rows;
}

function bucketKey(unix: number, bucket: string): string {
  const date = new Date(unix * 1000);
  const year = date.getUTCFullYear();
  const month = `${date.getUTCMonth() + 1}`.padStart(2, "0");
  const day = `${date.getUTCDate()}`.padStart(2, "0");
  const hour = `${date.getUTCHours()}`.padStart(2, "0");
  if (bucket === "hour") return `${year}-${month}-${day}T${hour}Z`;
  if (bucket === "day") return `${year}-${month}-${day}`;
  if (bucket === "month") return `${year}-${month}`;
  if (bucket === "year") return `${year}`;
  if (bucket === "week") {
    const thursday = new Date(Date.UTC(year, date.getUTCMonth(), date.getUTCDate()));
    thursday.setUTCDate(thursday.getUTCDate() + 3 - ((thursday.getUTCDay() + 6) % 7));
    const isoYear = thursday.getUTCFullYear();
    const januaryFirst = Date.UTC(isoYear, 0, 1);
    const week = Math.floor((thursday.getTime() - januaryFirst) / (7 * 86_400_000)) + 1;
    return `${isoYear}-W${`${week}`.padStart(2, "0")}`;
  }
  return new Date(unix * 1000).toISOString().replace(/\.\d{3}Z$/, "Z");
}

function bucketStart(unix: number, bucket: string): number {
  const date = new Date(unix * 1000);
  if (bucket === "hour") return unix - (unix % 3600);
  if (bucket === "day") return Math.floor(unix / 86_400) * 86_400;
  if (bucket === "month") return Date.UTC(date.getUTCFullYear(), date.getUTCMonth(), 1) / 1000;
  if (bucket === "year") return Date.UTC(date.getUTCFullYear(), 0, 1) / 1000;
  if (bucket === "week") {
    const weekday = ((date.getUTCDay() + 6) % 7) + 1;
    return (Math.floor(unix / 86_400) - (weekday - 1)) * 86_400;
  }
  return unix;
}

function bucketCommits(commits: ParsedCommit[], bucket: string): unknown[] {
  const order: string[] = [];
  const groups = new Map<string, { key: string; start_unix: number; commits: string[]; delta: Record<string, { added: number; removed: number }> }>();
  for (const commit of commits) {
    const key = bucketKey(commit.when_unix, bucket);
    let group = groups.get(key);
    if (!group) {
      group = { key, start_unix: bucketStart(commit.when_unix, bucket), commits: [], delta: {} };
      groups.set(key, group);
      order.push(key);
    }
    group.commits.push(commit.sha);
    for (const [name, pair] of Object.entries(commit.delta)) {
      const total = (group.delta[name] ??= { added: 0, removed: 0 });
      total.added += pair.added;
      total.removed += pair.removed;
    }
  }
  return order.map((key) => groups.get(key)!);
}

const BENCHMARK_LANGUAGES = ["Typescript", "Python", "Go", "C#", "Rust"];

function parseBenchmark(lang: string, output: string): { test: string; lang: string; time: string }[] {
  const results: { test: string; lang: string; time: string }[] = [];
  for (const raw of output.split("\n")) {
    const trimmed = raw.trim();
    if (!trimmed) continue;
    const parts = trimmed.split(",");
    if (parts.length !== 2 || parts[0]!.includes("warning") || parts[0]!.includes(":") || parts[0]!.includes("/") || parts[0]!.includes("\\")) continue;
    results.push({ test: parts[0]!, lang, time: parts[1]! });
  }
  return results;
}

function durationSeconds(raw: string): number | null {
  const text = raw.trim();
  let factor = 1;
  let body = text;
  if (body.endsWith("ms")) [body, factor] = [body.slice(0, -2), 1e-3];
  else if (body.endsWith("us") || body.endsWith("µs")) [body, factor] = [body.slice(0, -2), 1e-6];
  else if (body.endsWith("ns")) [body, factor] = [body.slice(0, -2), 1e-9];
  else if (body.endsWith("s")) body = body.slice(0, -1);
  const value = Number.parseFloat(body.trim());
  return Number.isNaN(value) ? null : value * factor;
}

function summarizeBenchmarks(results: { test: string; lang: string; time: string }[]): unknown {
  const tests = [...new Set(results.map((entry) => entry.test))].sort();
  const rows = tests.map((test) => {
    const timings: Record<string, string> = {};
    for (const language of BENCHMARK_LANGUAGES) {
      const found = results.find((entry) => entry.test === test && entry.lang === language);
      if (found) timings[language] = found.time;
    }
    let fastest: string | undefined;
    let best = Number.POSITIVE_INFINITY;
    for (const [language, time] of Object.entries(timings).sort(([left], [right]) => left.localeCompare(right))) {
      const seconds = durationSeconds(time);
      if (seconds !== null && seconds < best) {
        best = seconds;
        fastest = language;
      }
    }
    return fastest === undefined ? { test, timings } : { test, timings, fastest };
  });
  return { tests, languages: BENCHMARK_LANGUAGES, rows };
}
//#endregion 🔬️Independent reference implementation

const BENCHMARK_OUTPUTS: Record<string, string> = {
  Typescript: "warning: something,ignored\nserialize,0.412s\ndeserialize,0.208s\nC:/tmp/path,1.0s\n",
  Python: "serialize,1.204s\ndeserialize,0.930s\n",
  Go: "serialize,0.180s\ndeserialize,0.096s\nround-trip,0.400s\n",
  "C#": "serialize,0.350s\ndeserialize,0.201s\n",
  Rust: "serialize,0.090s\ndeserialize,0.044s\nround-trip,0.150s\n",
};

function main(): void {
  const { dir, shas } = buildRepository();
  try {
    const transcript = record(dir, shas) as { id: string; logs: Record<string, string>; tracked: Record<string, string[]>; blobs: Record<string, string> };
    mkdirSync(FIXTURES, { recursive: true });
    const write = (name: string, value: unknown): void => writeFileSync(join(FIXTURES, name), `${JSON.stringify(value, null, 2)}\n`, "utf8");

    write("🌱️repository-recipe.json", { author: AUTHOR, secondAuthor: SECOND_AUTHOR, branch: "⛳️wip", commits: RECIPE, side: { from: 1, branch: "side", path: "side.rs", content: SIDE_CONTENT, message: "side work", when: 1_767_318_000, author: SECOND_AUTHOR, mergeMessage: "merge side", mergeWhen: 1_769_990_400, mergeAuthor: AUTHOR } });
    write("🎞️git-transcript.json", transcript);

    const languages = [...CODE];
    const weights = new Set([...languages, AGG_MARKUP, AGG_DATA]);
    const commits = parseNumstat(transcript.logs[""]!, weights);
    const merged = parseNumstat(transcript.logs["🔀️with-merges"]!, weights);
    write("📤️numstat-deltas.json", { noMerges: commits, withMerges: merged.map((commit) => ({ sha: commit.sha, when_unix: commit.when_unix, fileCount: commit.files.length, delta: commit.delta })) });

    const cumulative: Record<string, { added: number; removed: number }> = {};
    for (const bucket of weights) cumulative[bucket] = { added: 0, removed: 0 };
    for (const commit of commits) {
      for (const [name, pair] of Object.entries(commit.delta)) {
        cumulative[name]!.added += pair.added;
        cumulative[name]!.removed += pair.removed;
      }
    }
    const wipDenominator = Object.values(cumulative).reduce((total, pair) => total + pair.added + pair.removed, 0);
    const scan = scanCounts(transcript, "", weights);
    write("📤️loc-report.json", { branch: "", snapshot: compose(cumulative, scan, languages, wipDenominator), scan, cumulative, wipDenominator });

    write("📤️time-buckets.json", Object.fromEntries(["commit", "hour", "day", "week", "month", "year"].map((bucket) => [bucket, bucketCommits(commits, bucket)])));

    const results = Object.entries(BENCHMARK_OUTPUTS).flatMap(([language, output]) => parseBenchmark(language, output));
    write("⏱️benchmark-output.json", BENCHMARK_OUTPUTS);
    write("📤️benchmark-summary.json", { results, summary: summarizeBenchmarks(results) });

    console.log(`[record] wrote 7 fixture(s) to ${FIXTURES}`);
    console.log(`[record] ${commits.length} no-merge commit(s), ${merged.length} with merges, ${Object.keys(transcript.blobs).length} blob(s)`);
  } finally {
    if (existsSync(dir)) rmSync(dir, { recursive: true, force: true });
  }
}

main();
