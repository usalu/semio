/**
 * 🧹️ S4-GATES: the AGENTS source rules over the lines this ticket's window added to the files its reports name (successor of
 * `🧪️s3-gates-source-rules.ts`, batched `git diff` so 2000+ files finish inside one Bash call).
 *
 * Window: every working-tree line `git diff -U0 <base>` reports as added (a file absent at base is added whole); base = the last
 * auto-commit before the ticket opened (`3eeee4f9119`). Rules: docstrings without a leading emoji (`docstringHitsOfText`, the
 * `verify docstrings emoji-first` census), per-file docstring-emoji uniqueness (a docstring opener on an added line whose emoji
 * another opener of the same file already uses — coordinator decision 10-04: uniqueness is enforced per touched file), indented
 * line comments inside definitions, and the temporary-log tag. Input `🗑️generated/s3-gates/ticket-files.txt`; output
 * `🗑️generated/s4-gates/source-rules.json`.
 * @see /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts
 */
import { spawnSync } from "node:child_process";
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { docstringHitsOfText } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts";

const ROOT = "/Users/ueli/Documents/semio";
const T = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING";
const BASE = process.argv[2] ?? "3eeee4f9119";
const TAG = ["[", "DEBUG", "]"].join("");
const OPENER = /^\s*(?:\/\/\/|\/\/!|\/\*\*|\*)\s*(\S+)/u;
const EMOJI = /^(?:\p{Extended_Pictographic}|\p{So}|\p{Regional_Indicator})(?:️|‍(?:\p{Extended_Pictographic}|\p{So}))*️?/u;

const files = readFileSync(join(ROOT, T, "🗑️generated/s3-gates/ticket-files.txt"), "utf8").split("\n").filter((path) => /\.(rs|ts|tsx)$/u.test(path) && existsSync(join(ROOT, path)));
const git = (args: readonly string[]): string => spawnSync("git", [...args], { cwd: ROOT, encoding: "utf8", maxBuffer: 1 << 30 }).stdout;
const atBase = new Set<string>();
const windows = new Map<string, Set<number>>();
for (let start = 0; start < files.length; start += 200) {
  const chunk = files.slice(start, start + 200);
  for (const path of git(["ls-tree", "-r", "-z", "--name-only", BASE, "--", ...chunk]).split("\0").filter(Boolean)) atBase.add(path);
  let current: Set<number> | undefined;
  for (const line of git(["diff", "-U0", "--no-color", "--no-renames", BASE, "--", ...chunk]).split("\n")) {
    const file = /^\+\+\+ b\/(.*)$/u.exec(line);
    if (file) {
      current = new Set<number>();
      windows.set(file[1]!, current);
      continue;
    }
    const hunk = /^@@ -\S+ \+(\d+)(?:,(\d+))? @@/u.exec(line);
    if (hunk && current) for (let at = Number(hunk[1]); at < Number(hunk[1]) + (hunk[2] === undefined ? 1 : Number(hunk[2])); at += 1) current.add(at);
  }
}

/** 🔎️ Each docstring opener's line and leading emoji: the first content line of a `///`/`//!` run or a `/** … *\/` block. */
function docstringOpeners(path: string, lines: readonly string[]): { line: number; emoji: string }[] {
  const found: { line: number; emoji: string }[] = [];
  let inRun = false;
  let inBlock = false;
  let pending = false;
  lines.forEach((raw, index) => {
    const line = raw.trimStart();
    const lineDoc = path.endsWith(".rs") && /^\/\/[/!](?!\/)/u.test(line);
    if (lineDoc) {
      if (!inRun) pending = true;
      inRun = true;
    } else inRun = false;
    if (!inBlock && line.startsWith("/**") && !line.startsWith("/**/") && !line.startsWith("/***")) {
      inBlock = !line.includes("*/");
      pending = true;
    } else if (inBlock && line.includes("*/")) inBlock = false;
    if (!pending) return;
    const content = OPENER.exec(raw)?.[1];
    if (!content) return;
    pending = false;
    const emoji = EMOJI.exec(content)?.[0];
    if (emoji) found.push({ line: index + 1, emoji: emoji.replace(/️/gu, "") });
  });
  return found;
}

const report: Record<string, { docstrings: unknown[]; duplicateEmoji: unknown[]; comments: unknown[]; debug: unknown[] }> = {};
let totals = { files: 0, docstrings: 0, duplicateEmoji: 0, comments: 0, debug: 0 };
for (const path of files) {
  const text = readFileSync(join(ROOT, path), "utf8");
  const lines = text.split("\n");
  const window = atBase.has(path) ? (windows.get(path) ?? new Set<number>()) : new Set(lines.map((_, index) => index + 1));
  const docstrings = docstringHitsOfText(path, text).filter((hit) => window.has(hit.line));
  const first = new Map<string, number>();
  const duplicateEmoji: { line: number; emoji: string; first: number }[] = [];
  for (const opener of docstringOpeners(path, lines)) {
    const seen = first.get(opener.emoji);
    if (seen === undefined) first.set(opener.emoji, opener.line);
    else if (window.has(opener.line)) duplicateEmoji.push({ line: opener.line, emoji: opener.emoji, first: seen });
  }
  const comments = lines.flatMap((raw, index) => {
    if (!window.has(index + 1)) return [];
    const trimmed = raw.trimStart();
    if (raw.length === trimmed.length || !trimmed.startsWith("//")) return [];
    if (/^\/\/(?:\/|!|#region|#endregion)/u.test(trimmed) || /^\/\/\s*(?:@ts-|eslint-|biome-|prettier-|SAFETY:)/u.test(trimmed)) return [];
    return [{ line: index + 1, text: trimmed.slice(0, 120) }];
  });
  const debug = lines.flatMap((raw, index) => (window.has(index + 1) && raw.includes(TAG) ? [{ line: index + 1, text: raw.trim().slice(0, 140) }] : []));
  totals = { files: totals.files + 1, docstrings: totals.docstrings + docstrings.length, duplicateEmoji: totals.duplicateEmoji + duplicateEmoji.length, comments: totals.comments + comments.length, debug: totals.debug + debug.length };
  if (docstrings.length + duplicateEmoji.length + comments.length + debug.length > 0) report[path] = { docstrings, duplicateEmoji, comments, debug };
}
writeFileSync(join(ROOT, T, "🗑️generated/s4-gates/source-rules.json"), `${JSON.stringify({ base: BASE, totals, report }, null, 1)}\n`);
console.log(JSON.stringify(totals));
