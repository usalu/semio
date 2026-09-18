#!/usr/bin/env bun
// T1 corruption census — find emoji glyphs spliced INSIDE ASCII identifiers.
//
// Signature of the 2026-09-03 rename-plan codemod incident
// (memory: project-codex-rename-plan-codemod-incident): an emoji (+ optional VS16)
// was inserted in the middle of an ASCII identifier, e.g.
//     WasiCliEnvironmen🔬️t029   (should be WasiCliEnvironment029)
//
// Detection: an ASCII word char, immediately followed by one Extended_Pictographic
// char (plus optional VS16 / ZWJ sequence), immediately followed by an ASCII word char.
//
// Usage:  bun <ticket>/🐍️t1-corruption-census.ts [rootDir] > <ticket>/🗑️generated/t1-corruption.txt

import { readdirSync, readFileSync, statSync } from "node:fs";
import { join, relative, dirname } from "node:path";

const ROOT = process.argv[2] ?? process.cwd();

const SKIP_DIR_NAMES = new Set([
  "node_modules",
  ".git",
  "target",
  "⚡️cache",
  ".venv",
  ".nx",
  ".turbo",
]);

// Default scope = the TypeScript/JavaScript program (what `tsc -p tsconfig.json` parses).
// `--all` widens to every text extension; only TS/JS corruption is a *parse* failure.
const WIDE = process.argv.includes("--all");
const TS_EXT = [".ts", ".tsx", ".mts", ".cts", ".js", ".jsx", ".mjs", ".cjs"];
const TEXT_EXT = new Set(
  WIDE
    ? [...TS_EXT, ".rs", ".json", ".jsonc", ".toml", ".wit", ".md", ".css", ".html", ".py", ".sh", ".yml", ".yaml"]
    : TS_EXT,
);

// word-char, emoji(+modifiers), word-char
const CORRUPT =
  /[A-Za-z0-9_](\p{Extended_Pictographic}(?:️|︎|‍\p{Extended_Pictographic}|[\u{1F3FB}-\u{1F3FF}])*)[A-Za-z0-9_]/u;
const CORRUPT_G = new RegExp(CORRUPT.source, "gu");

// Legitimate repo idiom: emoji-prefixed *path segments* carrying a numeric label,
// e.g. "🎆️26🌙️09☀️18", "🔖️1", "🚩️639". Those look like <digit><emoji><digit>.
// They are only a false positive when BOTH neighbours are digits.
function isDateLikePath(m: string): boolean {
  return /^[0-9]/.test(m) && /[0-9]$/.test(m);
}

// The repo legitimately uses emoji as a word separator INSIDE string/label values
// ("🏗️fem◻️2d-model", "⚙️setup🪟️native"). Only a splice in *code* position is the
// 2026-09-03 codemod corruption, and only that makes tsc emit TS1127/TS1434.
// Heuristic: count unescaped quotes/backticks before the column; odd => inside a string.
function inStringLiteral(line: string, col: number): boolean {
  let q: string | null = null;
  for (let i = 0; i < col && i < line.length; i++) {
    const c = line[i]!;
    if (c === "\\") { i++; continue; }
    if (q) { if (c === q) q = null; continue; }
    if (c === '"' || c === "'" || c === "`") q = c;
    else if (c === "/" && line[i + 1] === "/") return false; // line comment: code-ish, but not a string
  }
  return q !== null;
}

type Hit = {
  file: string;
  line: number;
  col: number;
  emoji: string;
  token: string;
  text: string;
  inString: boolean;
};

const hits: Hit[] = [];
let filesScanned = 0;

function walk(dir: string) {
  let entries;
  try {
    entries = readdirSync(dir, { withFileTypes: true });
  } catch {
    return;
  }
  for (const e of entries) {
    const p = join(dir, e.name);
    if (e.isDirectory()) {
      if (SKIP_DIR_NAMES.has(e.name)) continue;
      walk(p);
      continue;
    }
    if (!e.isFile()) continue;
    const dot = e.name.lastIndexOf(".");
    const ext = dot < 0 ? "" : e.name.slice(dot);
    if (!TEXT_EXT.has(ext)) continue;
    let st;
    try {
      st = statSync(p);
    } catch {
      continue;
    }
    if (st.size > 32 * 1024 * 1024) continue;
    scan(p);
  }
}

function scan(p: string) {
  let buf: string;
  try {
    buf = readFileSync(p, "utf8");
  } catch {
    return;
  }
  filesScanned++;
  if (!/\p{Extended_Pictographic}/u.test(buf)) return;
  const lines = buf.split("\n");
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i]!;
    if (!CORRUPT.test(line)) continue;
    CORRUPT_G.lastIndex = 0;
    let m: RegExpExecArray | null;
    while ((m = CORRUPT_G.exec(line)) !== null) {
      const whole = m[0]!;
      if (isDateLikePath(whole)) continue;
      // surrounding ASCII-identifier token, for reporting
      let s = m.index;
      while (s > 0 && /[A-Za-z0-9_]/.test(line[s - 1]!)) s--;
      let e = CORRUPT_G.lastIndex;
      while (e < line.length && /[A-Za-z0-9_]/.test(line[e]!)) e++;
      hits.push({
        file: relative(ROOT, p),
        line: i + 1,
        col: m.index + 1,
        emoji: m[1]!,
        token: line.slice(s, e),
        text: line.trim().slice(0, 200),
        inString: inStringLiteral(line, m.index),
      });
      // avoid overlapping rescans of the same char
      if (CORRUPT_G.lastIndex === m.index) CORRUPT_G.lastIndex++;
    }
  }
}

// ---- generator classification -------------------------------------------------
type Family =
  | "wasm-component/jco bindings"
  | "storybook-static"
  | "jcoprobe"
  | "dist/build-output"
  | "ticket-scratch"
  | "other";

function family(file: string): Family {
  if (/(^|\/)storybook-static\//.test(file)) return "storybook-static";
  if (/jcoprobe/.test(file)) return "jcoprobe";
  if (/_component\.d\.ts$/.test(file) || /(^|\/)interfaces\/.*\.d\.ts$/.test(file))
    return "wasm-component/jco bindings";
  if (/(^|\/)(dist|📤️dist|🗑️generated|🤖️generated|out-[a-z-]+)\//.test(file))
    return "dist/build-output";
  if (/🎫️tickets\//.test(file)) return "ticket-scratch";
  return "other";
}

walk(ROOT);

const byFile = new Map<string, Hit[]>();
for (const h of hits) {
  if (!byFile.has(h.file)) byFile.set(h.file, []);
  byFile.get(h.file)!.push(h);
}

const out: string[] = [];
const P = (s = "") => out.push(s);

P(`# T1 corruption census`);
P(`root: ${ROOT}`);
P(`generated: ${new Date().toISOString()}`);
P(`text files scanned: ${filesScanned}`);
P(`corrupted files: ${byFile.size}`);
P(`corrupted occurrences: ${hits.length}`);
P();

const codeHits = hits.filter((h) => !h.inString);
const strHits = hits.filter((h) => h.inString);
P(`occurrences in CODE position (real corruption): ${codeHits.length}`);
P(`occurrences inside string literals (repo emoji-separator idiom, not corruption): ${strHits.length}`);
P();

P(`## By generator family`);
const byFamily = new Map<Family, { files: Set<string>; hits: number; code?: number }>();
for (const [f, hs] of byFile) {
  const fam = family(f);
  if (!byFamily.has(fam)) byFamily.set(fam, { files: new Set(), hits: 0 });
  const e = byFamily.get(fam)!;
  e.files.add(f);
  e.hits += hs.length;
  e.code = (e.code ?? 0) + hs.filter((h) => !h.inString).length;
}
P(`| family | files | occurrences | in code position |`);
P(`|---|---:|---:|---:|`);
for (const [fam, e] of [...byFamily].sort((a, b) => (b[1].code ?? 0) - (a[1].code ?? 0)))
  P(`| ${fam} | ${e.files.size} | ${e.hits} | ${e.code ?? 0} |`);
P();

P(`## By directory`);
const byDir = new Map<string, { files: Set<string>; hits: number; code?: number }>();
for (const [f, hs] of byFile) {
  const d = dirname(f);
  if (!byDir.has(d)) byDir.set(d, { files: new Set(), hits: 0 });
  const e = byDir.get(d)!;
  e.files.add(f);
  e.hits += hs.length;
  e.code = (e.code ?? 0) + hs.filter((h) => !h.inString).length;
}
P(`| directory | family | files | occurrences | in code position |`);
P(`|---|---|---:|---:|---:|`);
for (const [d, e] of [...byDir].sort((a, b) => (b[1].code ?? 0) - (a[1].code ?? 0) || b[1].hits - a[1].hits))
  P(`| ${d} | ${family([...e.files][0]!)} | ${e.files.size} | ${e.hits} | ${e.code ?? 0} |`);
P();

P(`## Spliced emoji histogram`);
const byEmoji = new Map<string, number>();
for (const h of hits) byEmoji.set(h.emoji, (byEmoji.get(h.emoji) ?? 0) + 1);
for (const [em, n] of [...byEmoji].sort((a, b) => b[1] - a[1]))
  P(`  ${n}\t${em}\t(${[...em].map((c) => "U+" + c.codePointAt(0)!.toString(16).toUpperCase()).join(" ")})`);
P();

P(`## Every occurrence`);
for (const [f, hs] of [...byFile].sort((a, b) => b[1].length - a[1].length)) {
  const code = hs.filter((h) => !h.inString).length;
  P(`### ${f}  [${family(f)}]  ${hs.length} occurrence(s), ${code} in code position`);
  for (const h of hs)
    P(`  ${h.line}:${h.col}\t${h.inString ? "string" : "CODE  "}\ttoken=${h.token}\t| ${h.text}`);
}

console.log(out.join("\n"));
