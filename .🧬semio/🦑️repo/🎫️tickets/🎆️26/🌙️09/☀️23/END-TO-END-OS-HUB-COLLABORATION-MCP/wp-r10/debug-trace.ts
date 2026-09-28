#!/usr/bin/env bun
/**
 * 🏷️ R10 one-off codemod (session 14b; coordinator decision GO): retires the `[DEBUG]` tag from the tree. AGENTS.md reserves
 * `[DEBUG]` for temporary logs; the tree uses it as a permanent channel. Per tracked line holding `[DEBUG]`:
 *   error        — a thrown error / Error / super / reject / panic / Rust `.expect(` / test `expect(…, "msg")` message → the
 *                  prefix is dropped (an error message is no log line);
 *   test-print   — a print statement (console.*, eprintln!/println!, process.std*.write, t.Logf, fmt.Print*) in a test file
 *                  that starts and closes on its own line → the line is deleted (temporary by definition);
 *   test-note    — such a print whose deletion would leave a binding it reads unused → the tag is dropped instead;
 *   script-print — a print in a `📜️script.ts` → the tag is dropped (a status line of a permanent script);
 *   trace        — everything else (runtime-armed diagnostics, their consumers — classifiers, filters, asserted lines — and
 *                  the comments/fixtures that name the channel) → `[TRACE]`;
 *   manual       — a line holding several statements, code inside a string literal, or the format string of a print
 *                  spanning lines → never in `now`; in window 3 the tag is dropped in test files and becomes `[TRACE]` elsewhere.
 * Coupling: every `[DEBUG]` line whose text after the tag matches the literal skeleton of an error / print site (its text with
 * interpolations as wildcards) follows that site's action — a fixture or assertion quoting a thrown message drops the
 * prefix with it. A group lands `now` only when every member is open during the guest freeze (preamble 14 rule 2) and the
 * group holds no `trace` / `script-print` / `manual` member; everything else lands in window 3 in one pass (`--scope all`),
 * so no consumer ever sees a mixed channel.
 * Usage: bun debug-trace.ts [--scope now|all] [--apply]   (writes generated/debug-trace-<scope>.{json,diff})
 */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { join } from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const OUT = join(import.meta.dir, "generated");
const flag = (name: string): string | undefined => (process.argv.includes(name) ? process.argv[process.argv.indexOf(name) + 1] : undefined);
const scope = (flag("--scope") ?? "now") as "now" | "all";
const apply = process.argv.includes("--apply");

type Kind = "error" | "test-print" | "test-note" | "script-print" | "trace" | "manual";
type Site = { id: number; path: string; line: number; text: string; kind: Kind; open: boolean; group: number; now: boolean; replacement: string | null };

const TEST_PATH = /(^|\/)(🧪️tests|tests|🧫️fixtures|benches|examples)\/|_test\.go$|\.test\.[jt]sx?$/u;
const ERROR = /(throw\s|new Error\(|\bError\(|super\(|reject\(|panic!\(|unreachable!\(|\.expect\(|bail!\(|anyhow!\(|format_err!\(|\bexpect\([^)]*,\s*)[^\n]*\[DEBUG\]/u;
const PRINT = /^\s*(console\.(log|error|warn|info|debug)\(|e?println!\(|e?print!\(|process\.std(out|err)\.write\(|t\.Logf?\(|fmt\.Print(f|ln)?\()/u;
const PRINT_ANYWHERE = /console\.(log|error|warn|info|debug)\(|e?println!\(|process\.std(out|err)\.write\(/gu;

/** 🧊️ Whether a path is open during the guest freeze: host TS under 💻️os outside the plugin, wgpu-target, flow-wasm and script
 * trees, repo tooling tests, the hub and the non-framework trees; everything the chain builds or executes is frozen. */
function isOpen(path: string): boolean {
  if (path.startsWith("✏️s/🔌️plugins/") || path.includes("/🔌️plugin/")) return false;
  if (path.endsWith("📜️script.ts")) return false;
  if (path.startsWith("🌎️hub/")) return true;
  if (path.endsWith(".rs")) return false;
  if (path.startsWith("🧰️framework/🔨️modules/")) return false;
  if (path.includes("/🎯️targets/🧊️wgpu/") || path.includes("/🌊️flow/🕸️wasm/")) return false;
  if (path.startsWith("🧰️framework/🛍️products/🦑️repo/")) return TEST_PATH.test(path);
  return true;
}

/** ✂️ Whether a print starts at the line's first token and closes on the same line with nothing after its terminator. */
function selfContained(text: string): boolean {
  if (!PRINT.test(text)) return false;
  let depth = 0;
  let quote: string | null = null;
  for (let index = 0; index < text.length; index += 1) {
    const char = text[index]!;
    if (quote) {
      if (char === "\\") index += 1;
      else if (char === quote) quote = null;
      continue;
    }
    if (char === '"' || char === "'" || char === "`") quote = char;
    else if (char === "(") depth += 1;
    else if (char === ")") {
      depth -= 1;
      if (depth === 0) return /^;?\s*(\/\/.*)?$/u.test(text.slice(index + 1));
    }
  }
  return false;
}

function classify(path: string, text: string): Kind {
  const trimmed = text.trim();
  const prints = [...text.matchAll(PRINT_ANYWHERE)].length;
  if (/^["'`]/u.test(trimmed) || prints > 1 || (prints === 1 && ERROR.test(text) && !PRINT.test(text)) || (text.split(";").length > 3 && !trimmed.startsWith("//") && !trimmed.startsWith("*"))) return "manual";
  if (ERROR.test(text)) return "error";
  if (TEST_PATH.test(path) && selfContained(text)) return "test-print";
  if (path.endsWith("📜️script.ts") && PRINT.test(text)) return "script-print";
  return "trace";
}

/** ✂️ The line without the tag: an emptied `"[DEBUG] " +` operand or `"[DEBUG]",` argument goes with it. */
const dropTag = (text: string): string => text.replace(/(["'`])\[DEBUG\]\s*\1\s*\+\s*/gu, "").replace(/(["'`])\[DEBUG\]\1\s*,\s*/gu, "").replaceAll("[DEBUG] ", "").replaceAll("[DEBUG]", "");

function replacementOf(kind: Kind, text: string, path: string): string | null {
  if (kind === "test-print") return null;
  if (kind === "error" || kind === "script-print" || kind === "test-note") return dropTag(text);
  if (kind === "manual") return TEST_PATH.test(path) ? dropTag(text) : text.replaceAll("[DEBUG]", "[TRACE]");
  return text.replaceAll("[DEBUG]", "[TRACE]");
}

/** 🔗️ The anchored skeleton of a message after the tag: literal runs kept, `${…}` / `{…}` / `" + … + "` joints as wildcards,
 * cut at the closing quote; `null` when fewer than 12 literal characters remain (too generic to couple on). */
function skeletonOf(text: string): RegExp | null {
  const at = text.indexOf("[DEBUG]");
  const tail = text.slice(at + 7).replace(/^\s+/u, "");
  const end = tail.search(/(?<!\\)["'`](?!\s*\+)/u);
  const body = end >= 0 ? tail.slice(0, end) : tail;
  const parts = body.split(/\$\{[^}]*\}|\{[^}]*\}|["'`]\s*\+[^+]*\+\s*["'`]/u).map((part) => part.trim()).filter(Boolean);
  if (parts.join("").length < 12) return null;
  return new RegExp(`^\\s*${parts.map((part) => part.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")).join(".*?")}`, "u");
}

const { DEBUG_TAG_CENSUS_EXEMPT } = await import(`${ROOT}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts`);
const listed = spawnSync("git", ["grep", "-n", "-I", "-F", "[DEBUG]", "--", ":!.🧬semio", ":!*.md", ":!.cursor", ...(DEBUG_TAG_CENSUS_EXEMPT as string[]).map((path) => `:!${path}`)], { cwd: ROOT, encoding: "utf8", maxBuffer: 1 << 30 });
const fileLines = new Map<string, string[]>();
const linesOf = (path: string): string[] => fileLines.get(path) ?? fileLines.set(path, readFileSync(join(ROOT, path), "utf8").split("\n")).get(path)!;
/** 🧷️ Whether deleting line `line` would leave its predecessor dangling (an arrow, a brace-less `if`/`else`, an open call). */
const dangles = (path: string, line: number): boolean => {
  const lines = linesOf(path);
  const next = lines.slice(line).find((candidate) => candidate.trim().length > 0) ?? "";
  if (/^\s*(\.|\?\.|\)|\]|,|\+|&&|\|\||\?|:)/u.test(next) && !/^\s*\}/u.test(next)) return true;
  for (let index = line - 2; index >= 0; index -= 1) {
    const previous = lines[index]!.replace(/\/\/.*$/u, "").trimEnd();
    if (previous.trim().length === 0) continue;
    return !/[{;}]$|^\s*(\/\*|\*)|\*\/$|^\s*\/\//u.test(previous) || /\b(if|else|for|while)\b[^{;]*$/u.test(previous);
  }
  return false;
};
/** 🪢️ Whether deleting a print would leave a binding it reads unused (a Go compile error, a Rust warning): an identifier of
 * its arguments that occurs at most once in the file outside this line. */
const orphans = (path: string, text: string): boolean => {
  const open = text.indexOf("(");
  const argumentsText = text.slice(open + 1).replace(/"(?:[^"\\]|\\.)*"|'(?:[^'\\]|\\.)*'/gu, (literal) => [...literal.matchAll(/\{([A-Za-z_]\w*)/gu)].map((match) => ` ${match[1]} `).join(""));
  const identifiers = new Set([...argumentsText.replace(/`[^`]*`/gu, (literal) => [...literal.matchAll(/\$\{\s*([A-Za-z_]\w*)/gu)].map((match) => ` ${match[1]} `).join("")).matchAll(/(?<![.\w])([A-Za-z_]\w*)/gu)].map((match) => match[1]!));
  const rest = linesOf(path).filter((candidate) => candidate !== text).join("\n");
  return [...identifiers].some((identifier) => !/^(true|false|null|undefined|Some|None|self|Self|len|format|String|JSON|Math|Number|Object|Array)$/u.test(identifier) && (rest.match(new RegExp(`\\b${identifier}\\b`, "gu"))?.length ?? 0) <= 1);
};
const sites: Site[] = listed.stdout.split("\n").filter(Boolean).map((row, id) => {
  const first = row.indexOf(":");
  const second = row.indexOf(":", first + 1);
  const path = row.slice(0, first);
  const text = row.slice(second + 1);
  const line = Number(row.slice(first + 1, second));
  const classified = classify(path, text);
  const kind: Kind = classified === "test-print" && dangles(path, line) ? "manual" : classified === "test-print" && orphans(path, text) ? "test-note" : classified;
  return { id, path, line, text, kind, open: isOpen(path), group: id, now: false, replacement: null };
});

const parent = sites.map((site) => site.id);
const find = (id: number): number => (parent[id] === id ? id : (parent[id] = find(parent[id]!)));
const tails = sites.map((site) => site.text.slice(site.text.indexOf("[DEBUG]") + 7));
for (const site of sites) {
  if (site.kind !== "error" && site.kind !== "test-print") continue;
  const skeleton = skeletonOf(site.text);
  if (!skeleton) continue;
  for (const other of sites) if (other.id !== site.id && skeleton.test(tails[other.id]!)) parent[find(other.id)] = find(site.id);
}
const groups = new Map<number, Site[]>();
for (const site of sites) {
  site.group = find(site.id);
  (groups.get(site.group) ?? groups.set(site.group, []).get(site.group)!).push(site);
}
for (const members of groups.values()) {
  const leader = members.find((member) => member.kind === "error") ?? members.find((member) => member.kind === "test-print");
  if (members.length > 1 && leader) for (const member of members) if (member.kind === "trace" && leader.kind === "error") member.kind = "error";
  const now = members.every((member) => member.open && (member.kind === "error" || member.kind === "test-print" || member.kind === "test-note"));
  for (const member of members) {
    member.now = now;
    member.replacement = replacementOf(member.kind, member.text, member.path);
  }
}

const selected = sites.filter((site) => (scope === "all" ? true : site.now && site.kind !== "manual"));
const KINDS: Kind[] = ["error", "test-print", "test-note", "script-print", "trace", "manual"];
const counts = (list: Site[]) => Object.fromEntries(KINDS.map((kind) => [kind, list.filter((site) => site.kind === kind).length]));
const coupledGroups = [...groups.values()].filter((members) => members.length > 1);
const summary = { scope, sites: sites.length, selected: selected.length, files: new Set(selected.map((site) => site.path)).size, byKind: counts(selected), all: counts(sites), open: sites.filter((site) => site.open).length, coupledGroups: coupledGroups.length, coupledSites: coupledGroups.reduce((total, members) => total + members.length, 0) };
mkdirSync(OUT, { recursive: true });
writeFileSync(join(OUT, `debug-trace-${scope}.json`), JSON.stringify({ summary, sites: selected, manual: sites.filter((site) => site.kind === "manual"), groups: coupledGroups.map((members) => members.map((member) => `${member.path}:${member.line} [${member.kind}${member.open ? "" : ",frozen"}]`)) }, null, 1));

const byFile = new Map<string, Site[]>();
for (const site of selected) (byFile.get(site.path) ?? byFile.set(site.path, []).get(site.path)!).push(site);
const diff: string[] = [];
let applied = 0;
let stale = 0;
const pending: [string, string][] = [];
for (const [path, list] of byFile) {
  const absolute = join(ROOT, path);
  const lines = readFileSync(absolute, "utf8").split("\n");
  const drop = new Set<number>();
  for (const site of list) {
    if (lines[site.line - 1] !== site.text) {
      stale += 1;
      diff.push(`!! stale ${path}:${site.line}`);
      continue;
    }
    diff.push(`--- ${path}:${site.line} [${site.kind}]`, `- ${site.text.trim()}`);
    if (site.replacement === null) drop.add(site.line - 1);
    else {
      lines[site.line - 1] = site.replacement;
      diff.push(`+ ${site.replacement.trim()}`);
    }
    applied += 1;
  }
  pending.push([absolute, lines.filter((_, index) => !drop.has(index)).join("\n")]);
}
if (apply && stale === 0) for (const [absolute, content] of pending) writeFileSync(absolute, content);
writeFileSync(join(OUT, `debug-trace-${scope}.diff`), diff.join("\n") + "\n");
console.log(JSON.stringify({ ...summary, applied, stale, wrote: apply && stale === 0 }, null, 1));
