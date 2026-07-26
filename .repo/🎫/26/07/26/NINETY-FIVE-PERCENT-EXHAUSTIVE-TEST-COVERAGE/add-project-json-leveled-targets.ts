#!/usr/bin/env bun
/** 📊One-off codemod: appends `test-quick`/`test-long`/`test-exhaustive` nx targets (cloned from each
 * project.json's own `test` target, with the level word spliced into its `bun ./script.ts test …` command)
 * to every project.json that has a `test` target but is missing some/all leveled siblings — closes the gap
 * where `nx run-many -t test-exhaustive` silently skipped ~55% of test-bearing projects. Only touches files
 * whose `test.options.command` starts with the standard `bun ./script.ts test` prefix; anything else (a raw
 * `dotnet test …` bypassing script.ts) is reported, not touched — that is a separate pre-existing violation
 * of the "project.json MUST only call script.ts" rule, not a mechanical gap to paper over here.
 * Ticket: 26/07/26/NINETY-FIVE-PERCENT-EXHAUSTIVE-TEST-COVERAGE. */
import { readFileSync, writeFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";

const LEVELS = ["quick", "long", "exhaustive"] as const;

function findProjectJsonFiles(dir: string, out: string[] = []): string[] {
  for (const name of readdirSync(dir)) {
    if (name === "node_modules" || name === ".claude" || name === ".repo" || name === "target" || name.startsWith(".git")) continue;
    const full = join(dir, name);
    const st = statSync(full);
    if (st.isDirectory()) findProjectJsonFiles(full, out);
    else if (name === "project.json") out.push(full);
  }
  return out;
}

/** Finds the character span `[start, end)` of the `"test": { ... }` object value within `text`, via brace counting (handles nested objects/arrays/strings). Returns null if not found. */
function findTestTargetSpan(text: string): { keyStart: number; valueStart: number; valueEnd: number } | null {
  const keyMatch = /"test"\s*:\s*/.exec(text);
  if (!keyMatch) return null;
  const keyStart = keyMatch.index;
  const valueStart = keyMatch.index + keyMatch[0].length;
  if (text[valueStart] !== "{") return null;
  let depth = 0;
  let inString = false;
  let escaped = false;
  for (let i = valueStart; i < text.length; i++) {
    const ch = text[i];
    if (inString) {
      if (escaped) escaped = false;
      else if (ch === "\\") escaped = true;
      else if (ch === '"') inString = false;
      continue;
    }
    if (ch === '"') inString = true;
    else if (ch === "{") depth++;
    else if (ch === "}") {
      depth--;
      if (depth === 0) return { keyStart, valueStart, valueEnd: i + 1 };
    }
  }
  return null;
}

const repoRoot = process.cwd();
const files = findProjectJsonFiles(repoRoot);

let changed = 0;
let flagged = 0;
let skipped = 0;
for (const path of files) {
  const text = readFileSync(path, "utf8");
  let data: any;
  try {
    data = JSON.parse(text);
  } catch {
    console.error(`[PARSE-ERROR] ${path}`);
    continue;
  }
  const targets = data.targets ?? {};
  if (!("test" in targets)) continue;
  const needed = LEVELS.filter((level) => !(`test-${level}` in targets));
  if (needed.length === 0) {
    skipped++;
    continue;
  }
  const testTarget = targets.test;
  const command: string = testTarget?.options?.command ?? "";
  if (!command.startsWith("bun ./script.ts test")) {
    console.log(`[FLAG] ${path.slice(repoRoot.length + 1)} — non-standard test command ${JSON.stringify(command)}, needs manual leveled targets.`);
    flagged++;
    continue;
  }

  const span = findTestTargetSpan(text);
  if (!span) {
    console.error(`[MISS] ${path} — could not locate "test": { … } span in raw text.`);
    continue;
  }
  const lineStart = text.lastIndexOf("\n", span.keyStart) + 1;
  const indent = text.slice(lineStart, span.keyStart);
  const testBlockText = text.slice(span.valueStart, span.valueEnd);

  const newBlocks = needed
    .map((level) => {
      const levelCommand = command === "bun ./script.ts test" ? `bun ./script.ts test ${level}` : command.replace("bun ./script.ts test", `bun ./script.ts test ${level}`);
      const block = testBlockText.replace(/"command":\s*"[^"]*"/, `"command": ${JSON.stringify(levelCommand)}`);
      return `${indent}"test-${level}": ${block}`;
    })
    .join(",\n");

  const insertAt = span.valueEnd;
  const next = `${text.slice(0, insertAt)},\n${newBlocks}${text.slice(insertAt)}`;
  writeFileSync(path, next);
  // Round-trip validate.
  try {
    JSON.parse(readFileSync(path, "utf8"));
  } catch (e) {
    writeFileSync(path, text); // revert on malformed output
    console.error(`[REVERTED] ${path} — codemod produced invalid JSON: ${e}`);
    continue;
  }
  console.log(`[ok]   ${path.slice(repoRoot.length + 1)} (+${needed.map((l) => `test-${l}`).join(", ")})`);
  changed++;
}
console.log(`\n${changed} changed, ${flagged} flagged for manual follow-up, ${skipped} already complete.`);
