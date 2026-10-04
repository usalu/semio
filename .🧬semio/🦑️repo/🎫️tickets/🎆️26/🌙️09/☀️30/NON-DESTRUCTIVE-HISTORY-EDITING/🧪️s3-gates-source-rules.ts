/**
 * 🧹️ S3-GATES: the AGENTS source rules over the lines this ticket's window added to the files its reports name.
 *
 * Window: every line of the working tree that `git diff -U0 <base>` reports as added, base = the last auto-commit before the
 * ticket opened (`3eeee4f9119`, 2026-09-30 00:11). Rules: the repo docstring census (`docstringHitsOfText`, the gate
 * `verify docstrings emoji-first` uses), a leading emoji repeated by another docstring opener of the same file, a line comment
 * inside a definition (indented `//` that is no doc comment and no `//#region`/`//#endregion` marker), and the temporary-log tag.
 * Input: `🗑️generated/s3-gates/ticket-files.txt` (from `🧪️s3-gates-ticket-files.py`); output: `🗑️generated/s3-gates/source-rules.json`.
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

const added = (path: string): Set<number> => {
  const lines = new Set<number>();
  const tracked = spawnSync("git", ["cat-file", "-e", `${BASE}:${path}`], { cwd: ROOT }).status === 0;
  const total = readFileSync(join(ROOT, path), "utf8").split("\n").length;
  if (!tracked) {
    for (let line = 1; line <= total; line += 1) lines.add(line);
    return lines;
  }
  const diff = spawnSync("git", ["diff", "-U0", BASE, "--", path], { cwd: ROOT, encoding: "utf8", maxBuffer: 1 << 28 }).stdout;
  for (const match of diff.matchAll(/^@@ -\S+ \+(\d+)(?:,(\d+))? @@/gmu)) {
    const start = Number(match[1]);
    const count = match[2] === undefined ? 1 : Number(match[2]);
    for (let line = start; line < start + count; line += 1) lines.add(line);
  }
  return lines;
};

const files = readFileSync(join(ROOT, T, "🗑️generated/s3-gates/ticket-files.txt"), "utf8").split("\n").filter((path) => /\.(rs|ts|tsx)$/u.test(path) && existsSync(join(ROOT, path)));
const report: Record<string, { docstrings: unknown[]; duplicateEmoji: unknown[]; comments: unknown[]; debug: unknown[] }> = {};
let totals = { files: 0, docstrings: 0, duplicateEmoji: 0, comments: 0, debug: 0 };
for (const path of files) {
  const text = readFileSync(join(ROOT, path), "utf8");
  const lines = text.split("\n");
  const window = added(path);
  const docstrings = docstringHitsOfText(path, text).filter((hit) => window.has(hit.line));
  const openers = new Map<string, number[]>();
  for (const hit of docstringOpeners(path, lines)) {
    const emoji = hit.emoji;
    openers.set(emoji, [...(openers.get(emoji) ?? []), hit.line]);
  }
  const duplicateEmoji = [...openers].filter(([, at]) => at.length > 1 && at.some((line) => window.has(line))).map(([emoji, at]) => ({ emoji, lines: at.filter((line) => window.has(line)), all: at.length }));
  const comments = lines.flatMap((raw, index) => {
    if (!window.has(index + 1)) return [];
    const trimmed = raw.trimStart();
    if (raw.length === trimmed.length || !trimmed.startsWith("//")) return [];
    if (/^\/\/(?:\/|!|#region|#endregion)/u.test(trimmed) || /^\/\/\s*(?:@ts-|eslint-|biome-|prettier-|SAFETY:)/u.test(trimmed)) return [];
    return [{ line: index + 1, text: trimmed.slice(0, 120) }];
  });
  const debug = lines.flatMap((raw, index) => (window.has(index + 1) && raw.includes(TAG) ? [{ line: index + 1, text: raw.trim().slice(0, 140) }] : []));
  totals = { files: totals.files + 1, docstrings: totals.docstrings + docstrings.length, duplicateEmoji: totals.duplicateEmoji + duplicateEmoji.reduce((sum, row) => sum + row.lines.length, 0), comments: totals.comments + comments.length, debug: totals.debug + debug.length };
  if (docstrings.length + duplicateEmoji.length + comments.length + debug.length > 0) report[path] = { docstrings, duplicateEmoji, comments, debug };
}
writeFileSync(join(ROOT, T, "🗑️generated/s3-gates/source-rules.json"), `${JSON.stringify({ base: BASE, totals, report }, null, 1)}\n`);
console.log(JSON.stringify(totals));

/** 🔎️ Each docstring opener's line and leading emoji: the first content line of a `///`/`//!` run or a `/** … *\/` block. */
function docstringOpeners(path: string, lines: string[]): { line: number; emoji: string }[] {
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
    if (emoji) found.push({ line: index + 1, emoji });
  });
  return found;
}
