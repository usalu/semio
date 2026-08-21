#!/usr/bin/env bun
// 🔧️r13-deasync-codemod.ts
// TEMPORARY compiler-driven codemod for ticket INTERACTIVE-JOB-RUNTIME-REFACTOR,
// packet PHASE-1-5-DE-ASYNC-REPAIR-SWEEP, work packet R13.
// Lives only inside the ticket folder — NOT wired into 📜️script.ts, NOT a permanent script.
//
// Repairs the "async fn called without .await" bug class (AGENTS.md:44's async convention
// mechanically marked ~53,000 functions async; Phase 0 found 88.28% never suspend) by driving
// edits off `cargo check --message-format=json` diagnostics, never off name-keyed text search.
//
// Decision rule per diagnosed call site:
//   - callee genuinely suspends (has its own real `.await`)              -> add `.await`
//   - callee never suspends (zero own `.await` in its body)              -> DE-ASYNC the callee
//     (remove `async` from ITS signature; the call site needs no edit — its expression type
//     changes from `impl Future<Output=T>` to `T` automatically once the callee is fixed)
// Guards (never de-async):
//   - fn sits inside an `impl <Trait> for <Type>` block (external trait signature)
//   - fn sits lexically inside a `quote!{...}` / `quote_spanned!{...}` macro body (generated code)
//   - fn carries #[tokio::test] or #[...::async_test] (legitimate async test harness)
//   - proc-macro entry points (#[proc_macro*]) are the one case that's ALWAYS a fix, never a skip
//     -> handled by the same de-async path once flagged (their body must be sync by construction)
//
// Safety: every edit is journaled (file, byte span, before, after, motivating diagnostic) to
// 📝️r13-journal.jsonl in this folder, back-to-front applied per file per iteration so earlier
// edits never shift later spans. `revert` replays the journal backwards (see algorithm note in
// `revertRun`). `--dry-run` reports proposed edits without writing. Iteration requires the
// crate's own error count to strictly decrease; on stall/regression the tool reverts that single
// iteration and stops rather than thrashing.
//
// Usage:
//   bun 🔧️r13-deasync-codemod.ts run --crate=<cargo-package-name> [--dry-run] [--tests] [--max-iterations=20] [--target=<triple>]
//   bun 🔧️r13-deasync-codemod.ts revert --run=<runId>
//   bun 🔧️r13-deasync-codemod.ts selftest

import { readFileSync, writeFileSync, existsSync, appendFileSync, mkdtempSync, rmSync, readdirSync, statSync } from "fs";
import { join, resolve, isAbsolute } from "path";
import { tmpdir } from "os";
import { spawnSync } from "child_process";
import { cleanRustSource, findBodyOrDecl } from "../🔧️async-census.ts";

//#region Config

const ROOT = "/Users/ueli/Documents/semio";
const TICKET_DIR = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-5-DE-ASYNC-REPAIR-SWEEP";
const JOURNAL_PATH = join(TICKET_DIR, "📝️r13-journal.jsonl");
const EXCLUDE_PATH_SUBSTR = ["/compose/", "/target/", "/node_modules/"];

// SAFETY: rustc's diagnostics can carry spans into files OUTSIDE this repo entirely — a real
// example hit during this packet: a diagnostic against `semio-s-plugin-stdio` produced a
// machine-applicable suggestion whose span pointed into
// `~/.rustup/toolchains/.../library/core/src/macros/mod.rs` (the std library's OWN source, as
// distributed with the toolchain — e.g. from a `matches!`/`assert!`-style macro expansion site
// rustc attributes back to its expansion origin). Editing that file would be pointless (it's
// not part of compilation output, just installed source) and actively dangerous (mutating the
// toolchain installation itself, shared across every project on this machine). Every editable
// path MUST be positively confirmed to live inside this repo's ROOT before ANY read/slice/write
// touches it — a substring blacklist alone is not sufficient, since it can never enumerate every
// out-of-repo location rustc might name (toolchain source, ~/.cargo/registry vendored crates,
// etc.). isEditableFile() is the single required gate; EXCLUDE_PATH_SUBSTR narrows further
// WITHIN the repo (compose/target/node_modules).
function isEditableFile(absPath: string): boolean {
  if (!absPath.startsWith(ROOT + "/")) return false;
  return !EXCLUDE_PATH_SUBSTR.some((sub) => absPath.includes(sub));
}

//#endregion

//#region Types

interface Edit {
  ts: string;
  runId: string;
  iteration: number;
  crate: string;
  file: string; // absolute path
  start: number; // byte offset, ORIGINAL file state at start of this iteration
  end: number;
  before: string;
  after: string;
  kind: "call-add-await" | "call-remove-await" | "def-remove-async" | "test-remove-async" | "defuse-call-add-await";
  diagnosticCode: string | null;
  diagnosticMessage: string;
}

interface RustcSpan {
  file_name: string;
  byte_start: number;
  byte_end: number;
  is_primary: boolean;
  label: string | null;
  suggested_replacement: string | null;
  suggestion_applicability: string | null;
  text: { text: string; highlight_start: number; highlight_end: number }[];
}

interface RustcDiagnostic {
  message: string;
  code: { code: string } | null;
  level: string;
  spans: RustcSpan[];
  children: RustcDiagnostic[];
  rendered: string | null;
}

//#endregion

//#region Journal I/O

function appendJournal(edit: Edit): void {
  appendFileSync(JOURNAL_PATH, JSON.stringify(edit) + "\n");
}

function readJournal(): Edit[] {
  if (!existsSync(JOURNAL_PATH)) return [];
  const lines = readFileSync(JOURNAL_PATH, "utf8").split("\n").filter((l) => l.trim().length > 0);
  return lines.map((l) => JSON.parse(l) as Edit);
}

//#endregion

//#region cargo check invocation

function runCargoCheckJson(crate: string, opts: { tests: boolean; target?: string }): RustcDiagnostic[] {
  const args = ["check", "-p", crate, "--all-targets", "--message-format=json"];
  if (opts.target) args.splice(2, 0, "--target", opts.target);
  const res = spawnSync("cargo", args, { cwd: ROOT, maxBuffer: 1024 * 1024 * 1024, encoding: "utf8" });
  const out = res.stdout ?? "";
  const diags: RustcDiagnostic[] = [];
  for (const line of out.split("\n")) {
    if (!line.trim()) continue;
    let obj: any;
    try {
      obj = JSON.parse(line);
    } catch {
      continue;
    }
    if (obj.reason === "compiler-message" && obj.message) {
      diags.push(obj.message as RustcDiagnostic);
    }
  }
  return diags;
}

function countErrors(diags: RustcDiagnostic[]): number {
  return diags.filter((d) => d.level === "error" && !isExcludedDiag(d)).length;
}

function isExcludedDiag(d: RustcDiagnostic): boolean {
  const f = d.spans.find((s) => s.is_primary)?.file_name ?? d.spans[0]?.file_name ?? "";
  if (!f) return true;
  const abs = isAbsolute(f) ? f : resolve(ROOT, f);
  return !isEditableFile(abs);
}

//#endregion

//#region Async-class recognition

const ASYNC_SIGNATURE_PATTERNS = [
  /async functions cannot be used for tests/i,
  /no method named .* found for opaque type `?impl Future/i,
  /cannot apply unary operator .* to type `?impl Future/i,
  /the `?\??`? operator can only be applied to values that implement `?Try/i,
  /is not an iterator/i,
  /cannot (add|subtract|multiply|divide) .*`?impl Future/i,
  /binary operation .* cannot be applied to type `?impl Future/i,
  /no field .* on type `?impl Future/i,
  /mismatched types/i,
  /has an incompatible type for trait/i,
  /recursion in an async fn requires boxing/i,
  /is not a `?future`?/i,
];

function mentionsFutureOrAwait(d: RustcDiagnostic): boolean {
  const blob = (d.rendered ?? "") + " " + d.message + " " + d.children.map((c) => c.message + " " + (c.rendered ?? "")).join(" ");
  return /Future|\.await|await/i.test(blob);
}

function isAsyncClassDiagnostic(d: RustcDiagnostic): boolean {
  if (d.level !== "error") return false;
  if (isExcludedDiag(d)) return false;
  if (!mentionsFutureOrAwait(d)) return false;
  return ASYNC_SIGNATURE_PATTERNS.some((re) => re.test(d.message) || re.test(d.rendered ?? ""));
}

//#endregion

//#region Machine-applicable suggestion extraction

interface Suggestion {
  span: RustcSpan;
  applicability: string;
}

function collectSuggestions(d: RustcDiagnostic, out: Suggestion[]): void {
  for (const s of d.spans) {
    if (s.suggested_replacement !== null && s.suggested_replacement !== undefined) {
      out.push({ span: s, applicability: s.suggestion_applicability ?? "Unspecified" });
    }
  }
  for (const c of d.children) collectSuggestions(c, out);
}

//#endregion

//#region Fresh callee-suspends lookup (reuses Phase 0 census primitives directly, not its cache)

interface CalleeInfo {
  file: string;
  line: number;
  suspends: boolean; // has its own genuine .await
  guarded: string | null; // non-null => do not touch definition, reason recorded
}

const quoteRangeCache = new Map<string, [number, number][]>();
const fileSrcCache = new Map<string, string>();
const fileCleanCache = new Map<string, string>();
const byteToCharCache = new Map<string, Uint32Array>();

// rustc's diagnostic spans are UTF-8 BYTE offsets. `src`/`clean` are JS strings indexed in
// UTF-16 code units. This repo's source files routinely contain multi-byte characters (the
// emoji docstring convention, emoji in comments) BEFORE arbitrary code positions, so byte
// offset N is frequently NOT the same as string index N — confirmed empirically (byte 5079 in
// one fixture landed 50 code units off). Every rustc span must be converted through this map
// before it is used to slice or compare against `src`/`clean`; never index them directly.
function utf8ByteLen(codePoint: number): number {
  if (codePoint < 0x80) return 1;
  if (codePoint < 0x800) return 2;
  if (codePoint < 0x10000) return 3;
  return 4;
}

function buildByteToCharMap(src: string): Uint32Array {
  const byteLen = Buffer.byteLength(src, "utf8");
  const map = new Uint32Array(byteLen + 1);
  let byteIdx = 0;
  let charIdx = 0;
  while (charIdx < src.length) {
    const cp = src.codePointAt(charIdx)!;
    const bLen = utf8ByteLen(cp);
    const cLen = cp > 0xffff ? 2 : 1;
    for (let b = 0; b < bLen; b++) map[byteIdx + b] = charIdx;
    byteIdx += bLen;
    charIdx += cLen;
  }
  map[byteLen] = charIdx;
  return map;
}

function loadFile(absPath: string): { src: string; clean: string } {
  if (fileSrcCache.has(absPath)) return { src: fileSrcCache.get(absPath)!, clean: fileCleanCache.get(absPath)! };
  const src = readFileSync(absPath, "utf8");
  const clean = cleanRustSource(src);
  fileSrcCache.set(absPath, src);
  fileCleanCache.set(absPath, clean);
  byteToCharCache.set(absPath, buildByteToCharMap(src));
  return { src, clean };
}

// Converts a rustc UTF-8 byte offset for `absPath` into a JS string char index. Always call
// this on byte_start/byte_end BEFORE using them with `src`/`clean` from loadFile().
function toCharOffset(absPath: string, byteOffset: number): number {
  loadFile(absPath); // ensure map is populated
  const map = byteToCharCache.get(absPath)!;
  if (byteOffset < 0) return 0;
  if (byteOffset >= map.length) return map[map.length - 1];
  return map[byteOffset];
}

function invalidateFile(absPath: string): void {
  fileSrcCache.delete(absPath);
  fileCleanCache.delete(absPath);
  quoteRangeCache.delete(absPath);
  byteToCharCache.delete(absPath);
  invalidateDefUseCaches(absPath);
}

function computeQuoteRanges(clean: string): [number, number][] {
  const ranges: [number, number][] = [];
  const re = /\b(quote|quote_spanned)!\s*\{/g;
  let m: RegExpExecArray | null;
  while ((m = re.exec(clean))) {
    const braceStart = clean.indexOf("{", m.index);
    if (braceStart < 0) continue;
    let depth = 1;
    let i = braceStart + 1;
    while (i < clean.length && depth > 0) {
      if (clean[i] === "{") depth++;
      else if (clean[i] === "}") depth--;
      i++;
    }
    ranges.push([braceStart, i]);
  }
  return ranges;
}

function getQuoteRanges(absPath: string): [number, number][] {
  if (quoteRangeCache.has(absPath)) return quoteRangeCache.get(absPath)!;
  const { clean } = loadFile(absPath);
  const ranges = computeQuoteRanges(clean);
  quoteRangeCache.set(absPath, ranges);
  return ranges;
}

function insideAny(ranges: [number, number][], pos: number): boolean {
  return ranges.some(([s, e]) => pos >= s && pos < e);
}

function lineOfOffset(src: string, offset: number): number {
  let line = 1;
  for (let i = 0; i < offset && i < src.length; i++) if (src[i] === "\n") line++;
  return line;
}

// Scans backward from a fn's `async` keyword offset for the nearest enclosing `impl` header
// at a shallower/equal textual nesting, to guard against rewriting a trait-impl signature.
function isInsideTraitImpl(clean: string, asyncPos: number): boolean {
  const windowStart = Math.max(0, asyncPos - 8000);
  const window = clean.slice(windowStart, asyncPos);
  const implRe = /\bimpl(?:<[^>]*>)?\s+([A-Za-z_][\w:.<>, '&]*?)\s+for\s+/g;
  let lastMatch: RegExpExecArray | null = null;
  let m: RegExpExecArray | null;
  while ((m = implRe.exec(window))) lastMatch = m;
  if (!lastMatch) return false;
  // verify no closing top-level `}` of that impl block appears between the impl header and asyncPos
  const implHeaderEnd = windowStart + lastMatch.index + lastMatch[0].length;
  const braceOpen = clean.indexOf("{", implHeaderEnd);
  if (braceOpen < 0 || braceOpen > asyncPos) return false;
  let depth = 1;
  let i = braceOpen + 1;
  while (i < asyncPos && depth > 0) {
    if (clean[i] === "{") depth++;
    else if (clean[i] === "}") depth--;
    i++;
  }
  return depth > 0; // still inside the impl block at asyncPos
}

function hasAsyncTestAttribute(src: string, asyncFnLine: number): boolean {
  const lines = src.split("\n");
  for (let l = Math.max(0, asyncFnLine - 4); l < asyncFnLine - 1 && l < lines.length; l++) {
    if (/#\[\s*tokio::test/.test(lines[l]) || /async_test\s*\]/.test(lines[l]) || /#\[\s*.*async_test/.test(lines[l])) return true;
  }
  return false;
}

// Locate the `async fn NAME` occurrence whose reported line matches `expectedLine`, verified
// against the actual file content (span-keyed to file+line+name, never a blind global replace).
function locateAsyncKeyword(absPath: string, name: string, expectedLine: number): { start: number; end: number } | null {
  const { src, clean } = loadFile(absPath);
  const escaped = name.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const re = new RegExp(`\\basync\\s+fn\\s+${escaped}\\b`, "g");
  let m: RegExpExecArray | null;
  while ((m = re.exec(clean))) {
    const line = lineOfOffset(src, m.index);
    if (line === expectedLine) {
      const asyncKw = /^async\s+/.exec(m[0])!;
      return { start: m.index, end: m.index + asyncKw[0].length };
    }
  }
  return null;
}

function ownBodyHasAwait(clean: string, bodyStart: number): boolean {
  // Reuse census's own-level scan by re-importing scanBody indirectly is not exported; do a
  // minimal, guard-consistent re-scan here: brace-match, skip nested `fn` items, look for a
  // top-level `.await`.
  const n = clean.length;
  const stack: boolean[] = [false];
  let i = bodyStart + 1;
  while (i < n) {
    const ch = clean[i];
    if (ch === "{") {
      stack.push(stack[stack.length - 1]);
      i++;
      continue;
    }
    if (ch === "}") {
      stack.pop();
      if (stack.length === 0) return false;
      i++;
      continue;
    }
    if (/[A-Za-z_]/.test(ch)) {
      let j = i;
      while (j < n && /[A-Za-z0-9_]/.test(clean[j])) j++;
      const word = clean.slice(i, j);
      if (word === "fn") {
        let k = j;
        while (k < n && /\s/.test(clean[k])) k++;
        if (/[A-Za-z_]/.test(clean[k] ?? "")) {
          let nameEnd = k;
          while (nameEnd < n && /[A-Za-z0-9_]/.test(clean[nameEnd])) nameEnd++;
          const sig = findBodyOrDecl(clean, nameEnd);
          if (sig && sig.hasBody && sig.bodyStart !== undefined) {
            i = sig.bodyStart;
            stack.push(true);
            i++;
            continue;
          } else if (sig && !sig.hasBody && sig.declEnd !== undefined) {
            i = sig.declEnd + 1;
            continue;
          }
        }
        i = j;
        continue;
      }
      i = j;
      continue;
    }
    if (ch === "." && clean.slice(i, i + 6) === ".await" && !stack[stack.length - 1]) {
      return true;
    }
    i++;
  }
  return false;
}

// Same own-level scan as ownBodyHasAwait, but returns the callee name at each own-level
// `.await` instead of a boolean. Used only by the E0733 mutual-recursion handler, to verify a
// function's awaits are ALL directed at fellow members of its own recursion cycle (i.e. no
// OTHER genuine suspension hiding in the same body) before bulk-de-asyncing the whole cycle.
function ownAwaitCalleeNames(clean: string, bodyStart: number): string[] {
  const n = clean.length;
  const stack: boolean[] = [false];
  const names: string[] = [];
  let i = bodyStart + 1;
  while (i < n) {
    const ch = clean[i];
    if (ch === "{") {
      stack.push(stack[stack.length - 1]);
      i++;
      continue;
    }
    if (ch === "}") {
      stack.pop();
      if (stack.length === 0) return names;
      i++;
      continue;
    }
    if (/[A-Za-z_]/.test(ch)) {
      let j = i;
      while (j < n && /[A-Za-z0-9_]/.test(clean[j])) j++;
      const word = clean.slice(i, j);
      if (word === "fn") {
        let k = j;
        while (k < n && /\s/.test(clean[k])) k++;
        if (/[A-Za-z_]/.test(clean[k] ?? "")) {
          let nameEnd = k;
          while (nameEnd < n && /[A-Za-z0-9_]/.test(clean[nameEnd])) nameEnd++;
          const sig = findBodyOrDecl(clean, nameEnd);
          if (sig && sig.hasBody && sig.bodyStart !== undefined) {
            i = sig.bodyStart;
            stack.push(true);
            i++;
            continue;
          } else if (sig && !sig.hasBody && sig.declEnd !== undefined) {
            i = sig.declEnd + 1;
            continue;
          }
        }
        i = j;
        continue;
      }
      i = j;
      continue;
    }
    if (ch === "." && clean.slice(i, i + 6) === ".await" && !stack[stack.length - 1]) {
      const name = extractCalleeNameBackward(clean, i);
      names.push(name ?? "<unresolved>");
      i += 6;
      continue;
    }
    i++;
  }
  return names;
}

// Finds the async fn definition matching `name`, closest by line proximity if multiple exist
// in the same file (dedupes generic names like `hash_bytes` per R12's lesson — every candidate
// is independently checked, never blindly assumed unique).
function resolveCalleeInFile(absPath: string, name: string): CalleeInfo | null {
  const { src, clean } = loadFile(absPath);
  const escaped = name.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const re = new RegExp(`\\basync\\s+fn\\s+${escaped}\\b`, "g");
  const candidates: { pos: number; line: number }[] = [];
  let m: RegExpExecArray | null;
  while ((m = re.exec(clean))) candidates.push({ pos: m.index, line: lineOfOffset(src, m.index) });
  if (candidates.length === 0) return null;
  // Only safe to resolve automatically when exactly one candidate exists in this file; multiple
  // same-named async fns in one file is rare and ambiguous — treat as unresolved (guarded).
  if (candidates.length > 1) return { file: absPath, line: -1, suspends: true, guarded: "ambiguous: multiple same-named async fn in file" };
  const cand = candidates[0];
  const ranges = getQuoteRanges(absPath);
  if (insideAny(ranges, cand.pos)) return { file: absPath, line: cand.line, suspends: true, guarded: "inside quote!{} macro body" };
  if (isInsideTraitImpl(clean, cand.pos)) return { file: absPath, line: cand.line, suspends: true, guarded: "inside external trait impl block" };
  if (hasAsyncTestAttribute(src, cand.line)) return { file: absPath, line: cand.line, suspends: true, guarded: "async test harness attribute" };
  const fnKwIdx = clean.indexOf("fn", cand.pos);
  const afterName = fnKwIdx + 2;
  const sig = findBodyOrDecl(clean, afterName);
  if (!sig || !sig.hasBody || sig.bodyStart === undefined) return { file: absPath, line: cand.line, suspends: true, guarded: "signature scan failed" };
  const suspends = ownBodyHasAwait(clean, sig.bodyStart);
  return { file: absPath, line: cand.line, suspends, guarded: null };
}

//#region Repo-wide callee index (cross-file resolution, uniqueness-gated)

// A best-effort, name-keyed repo-wide index of `async fn NAME` occurrences, used ONLY as a
// last resort when the call site's own file has zero local candidates for `name` — e.g. the
// callee lives in a different crate/module (the common case: registry code calling into a
// shared manifest/engine crate). This is exactly the kind of lookup R12 warned is dangerous
// when trusted blindly (two unrelated `hash_bytes` functions existed repo-wide) — so it is
// used ONLY to find the candidate location(s); the actual guard/suspends decision is always
// re-derived by resolveCalleeInFile() against LIVE file content at that location, and if more
// than one repo-wide candidate exists for the name, resolution is refused (guarded) rather
// than guessing. Rebuilt fresh every iteration (see runCrate) so it never resolves against a
// location this same run has already edited incorrectly.
interface GlobalIndexEntry {
  file: string;
  line: number;
}

const GLOBAL_INDEX_MANDATORY_EXCLUDE = new Set(["compose", "target", "node_modules"]);
const GLOBAL_INDEX_EXTRA_EXCLUDE = new Set([".🧬semio", "storybook-static", "♻️mit-bestand", ".nx", ".git", "⚡️cache"]);

function buildGlobalAsyncFnIndex(): Map<string, GlobalIndexEntry[]> {
  const index = new Map<string, GlobalIndexEntry[]>();
  function walk(dir: string): void {
    let entries: string[];
    try {
      entries = readdirSync(dir);
    } catch {
      return;
    }
    for (const entry of entries) {
      if (GLOBAL_INDEX_MANDATORY_EXCLUDE.has(entry) || GLOBAL_INDEX_EXTRA_EXCLUDE.has(entry)) continue;
      const full = join(dir, entry);
      let st;
      try {
        st = statSync(full);
      } catch {
        continue;
      }
      if (st.isDirectory()) {
        walk(full);
      } else if (st.isFile() && entry.endsWith(".rs")) {
        let src: string;
        try {
          src = readFileSync(full, "utf8");
        } catch {
          continue;
        }
        const re = /\basync\s+fn\s+([A-Za-z_][A-Za-z0-9_]*)/g;
        let m: RegExpExecArray | null;
        while ((m = re.exec(src))) {
          const name = m[1];
          const line = lineOfOffset(src, m.index);
          if (!index.has(name)) index.set(name, []);
          index.get(name)!.push({ file: full, line });
        }
      }
    }
  }
  walk(ROOT);
  return index;
}

let globalAsyncFnIndex: Map<string, GlobalIndexEntry[]> | null = null;

function getGlobalAsyncFnIndex(): Map<string, GlobalIndexEntry[]> {
  if (!globalAsyncFnIndex) globalAsyncFnIndex = buildGlobalAsyncFnIndex();
  return globalAsyncFnIndex;
}

function invalidateGlobalAsyncFnIndex(): void {
  globalAsyncFnIndex = null;
}

//#endregion

function resolveCallee(callSiteAbsPath: string, name: string): CalleeInfo | null {
  const local = resolveCalleeInFile(callSiteAbsPath, name);
  if (local) return local;
  const idx = getGlobalAsyncFnIndex();
  const candidates = idx.get(name);
  if (!candidates || candidates.length === 0) return null;
  if (candidates.length > 1) {
    const distinctFiles = new Set(candidates.map((c) => c.file));
    if (distinctFiles.size > 1 || candidates.length > 1) {
      return { file: callSiteAbsPath, line: -1, suspends: true, guarded: `ambiguous: ${candidates.length} repo-wide async fn named ${name} across ${distinctFiles.size} file(s)` };
    }
  }
  const target = candidates[0];
  if (target.file === callSiteAbsPath) return null; // already covered by the local search above
  return resolveCalleeInFile(target.file, name);
}

//#endregion

//#region Callee-name extraction from a call expression ending at a given offset

// SAFETY: `Some(x)` / `Ok(x)` / `Err(x)` / `None` are enum-variant CONSTRUCTORS that are
// lexically indistinguishable from a function call — but they routinely appear in PATTERN
// position (`let (Some(a), Some(b)) = expr else { .. }`, `match x { Some(y) => .. }`), where a
// `.await` inserted right after them is not merely wrong, it is a syntax error. Confirmed as a
// REAL corruption during this packet's os-infinite run: `findFutureExprSpan()` picked a pattern
// sub-span (rustc had labelled it "found future" as part of a tuple-pattern-vs-tuple-of-futures
// mismatch) instead of the actual Future-typed expression, and forward-extraction happily
// treated `Some(src_ep)` as a callable, producing `let (Some(src_ep).await, ...) = .. else {`.
// resolveCallee() already returns null for these (no user code defines `async fn Some`), which
// used to fall through to "must be external, await it" — exactly backwards for a pattern. This
// denylist is checked at every call-add-await site right before the edit is queued; on a hit,
// the diagnostic is refused into residue instead of guessed at.
const PATTERN_CONSTRUCTOR_DENYLIST = new Set(["Some", "None", "Ok", "Err"]);

// SAFETY: a SECOND real corruption shape, distinct from the pattern-constructor one, found on
// `semio-s-plugin-stdio`. rustc's OWN "consider awaiting" suggestion — trusted verbatim as
// authoritative per this tool's design — inserted `.await` immediately after a STRUCT-LITERAL
// SHORTHAND FIELD reference: `CsvSnapshot { schema: ..., has_header, records }` (where
// `records` means `records: records`) became `CsvSnapshot { ..., has_header, records.await }`,
// a syntax error (`records.await` cannot be a struct field). This is NOT a callee-resolution
// problem (there is no call at all here) and NOT limited to the suggestion-driven path — it can
// happen anywhere this tool inserts `.await` after a bare identifier. General, span-local
// heuristic: refuse an `.await` insertion when the immediately preceding token (skipping
// whitespace) is a BARE identifier with no trailing call parens, AND that identifier is itself
// immediately preceded by `{` or `,` AND immediately followed (after the insertion) by `,` or
// `}` — i.e. it sits exactly where a struct/tuple-literal element or shorthand field would.
// A genuine `field: expr.await` is unaffected (preceded by `:`, not `{`/`,`). Like the pattern-
// constructor guard, a false refusal only produces residue, never a wrong edit.
function isRiskyBareIdentifierAwaitInsertion(clean: string, insertionStart: number): boolean {
  let after = insertionStart;
  while (after < clean.length && /\s/.test(clean[after])) after++;
  if (clean[after] !== "," && clean[after] !== "}") return false;
  let p = insertionStart - 1;
  while (p >= 0 && /\s/.test(clean[p])) p--;
  if (clean[p] === ")") return false; // a real call precedes — not this shape
  if (!/[A-Za-z0-9_]/.test(clean[p] ?? "")) return false;
  while (p >= 0 && /[A-Za-z0-9_]/.test(clean[p])) p--;
  while (p >= 0 && /\s/.test(clean[p])) p--;
  return clean[p] === "{" || clean[p] === ",";
}

// Defense-in-depth backstop, independent of root cause: NEVER insert `.await` immediately
// after text that already ends with `.await` (or, for the dot-prefix "await." insertion shape,
// immediately before text that already starts with "await"). This is what let the
// findFutureExprSpan mis-selection (fixed above) accumulate up to 6 stacked `.await`s across
// repeated runs instead of failing loudly on the first one — each run's fresh diagnostic scan
// re-found the "same" unresolved error and piled another `.await` on top rather than recognising
// the position was already touched. This check is cheap and unconditional: even if some other,
// not-yet-found bug someday computes a wrong insertion point again, it cannot stack.
function wouldStackAwait(clean: string, insertionStart: number): boolean {
  const before = clean.slice(Math.max(0, insertionStart - 6), insertionStart);
  const after = clean.slice(insertionStart, insertionStart + 6);
  return before === ".await" || after === "await.";
}

function extractCalleeNameBackward(clean: string, callEndExclusive: number): string | null {
  let p = callEndExclusive - 1;
  while (p >= 0 && /\s/.test(clean[p])) p--;
  // Some suggestions insert `await.` at a zero-width point right after an EXISTING method-chain
  // dot (turning `.ok()` into `.await.ok()`) rather than after a call's closing paren. Skip one
  // such separating dot before requiring the closing paren of the actual future-producing call.
  if (clean[p] === ".") {
    p--;
    while (p >= 0 && /\s/.test(clean[p])) p--;
  }
  if (clean[p] !== ")") return null;
  let depth = 1;
  p--;
  while (p >= 0 && depth > 0) {
    if (clean[p] === ")") depth++;
    else if (clean[p] === "(") depth--;
    p--;
  }
  let nameEnd = p + 1;
  let nameStart = nameEnd;
  while (nameStart > 0 && /[A-Za-z0-9_]/.test(clean[nameStart - 1])) nameStart--;
  const name = clean.slice(nameStart, nameEnd);
  return name.length > 0 ? name : null;
}

// Forward variant: for diagnostic shapes where the primary span STARTS at the callee
// identifier (e.g. E0277 "is not an iterator" over a `for x in callee(...)` expression).
// Balances parens/turbofish from the span start to find the true call end, independent of
// the diagnostic's own (sometimes off-by-a-few-bytes) span end.
function extractCallForward(clean: string, spanStart: number): { name: string; callEnd: number } | null {
  let i = spanStart;
  if (!/[A-Za-z_]/.test(clean[i] ?? "")) return null;
  const nameStart = i;
  while (i < clean.length && /[A-Za-z0-9_]/.test(clean[i])) i++;
  const name = clean.slice(nameStart, i);
  while (clean[i] === ":" && clean[i + 1] === ":") {
    i += 2;
    if (clean[i] === "<") {
      let depth = 1;
      i++;
      while (i < clean.length && depth > 0) {
        if (clean[i] === "<") depth++;
        else if (clean[i] === ">") depth--;
        i++;
      }
    } else break;
  }
  if (clean[i] !== "(") return null;
  let depth = 1;
  i++;
  while (i < clean.length && depth > 0) {
    if (clean[i] === "(") depth++;
    else if (clean[i] === ")") depth--;
    i++;
  }
  return { name, callEnd: i };
}

//#endregion

//#region R14: def-use resolution for bare let-bound Future locals
//
// R13's no-suggestion fallback (findFutureExprSpan + extractCallForward) correctly and safely
// refuses to guess when the Future-typed expression named by a diagnostic is a BARE, let-bound
// local variable rather than a fresh call — this was, by a wide margin, the dominant residue
// shape across every large crate R13 touched (~1,250+ diagnostics). R14 resolves that specific
// shape, and ONLY that shape: trace the variable back to its OWN binding statement within the
// SAME enclosing function body, and apply R13's identical decision rule to the binding's RHS
// call — never at the later use site (that is both the semantically correct fix location and
// what avoids the receiver-span class of bug R13's incident #4 found).
//
// This is deliberately narrower than a general data-flow pass. It resolves exactly:
//   let (mut)? NAME (: TYPE)? = CALL(...);      ... (same fn, no reassignment/shadow/closure) ... NAME
// and refuses — with a specific, logged reason, never a guess — every shape enumerated in the
// R14 task brief: shadowing, function-parameter/pattern/closure-argument bindings, reassignment
// or mutable-borrow between binding and use, RHS that is not a bare resolvable call (macro,
// method chain, block/conditional expression), a binding that crosses a closure or nested-fn
// boundary, and a binding hidden behind an unevaluated `#[cfg(...)]`. Every one of R13's existing
// guards (PATTERN_CONSTRUCTOR_DENYLIST, wouldStackAwait, isRiskyBareIdentifierAwaitInsertion,
// isEditableFile, the trait-impl/quote!/async-test guards inside resolveCallee) is re-applied
// unchanged at the resolved binding site — R14 adds a smarter way to LOCATE the edit, not a new
// way to decide whether an edit is safe.

interface EnclosingFn {
  name: string;
  isAsync: boolean;
  paramsStart: number; // char offset, first char inside '('
  paramsEnd: number; // char offset of matching ')'
  bodyStart: number; // char offset of the fn's opening '{'
  bodyEnd: number; // char offset of the fn's matching closing '}'
}

// Cheap memoization: the list of `fn NAME` candidate offsets in a file never changes within one
// planning pass over a fixed snapshot of that file, and large files can have thousands of fns.
const fnCandidateCache = new Map<string, { idx: number; name: string; afterName: number }[]>();

function invalidateDefUseCaches(absPath: string): void {
  fnCandidateCache.delete(absPath);
}

function matchBraceForward(clean: string, openBraceIdx: number): number {
  let depth = 1;
  let i = openBraceIdx + 1;
  const n = clean.length;
  while (i < n && depth > 0) {
    if (clean[i] === "{") depth++;
    else if (clean[i] === "}") depth--;
    i++;
  }
  return depth === 0 ? i - 1 : n; // n (out of range) signals "unbalanced" to callers
}

// Finds the innermost `fn ... { ... }` whose body textually contains `pos`. Scanning backward
// from the nearest preceding `fn` keyword and taking the FIRST candidate whose body actually
// contains `pos` is correct by construction: any enclosing function's `fn` keyword must precede
// `pos`, and among all such enclosing functions the innermost one's `fn` keyword is the closest
// preceding one (nesting means the inner function's text starts after the outer's `fn` keyword
// but before `pos`) — so the nearest-first scan finds the innermost match first.
function findEnclosingFunction(absPath: string, clean: string, pos: number): EnclosingFn | null {
  let candidates = fnCandidateCache.get(absPath);
  if (!candidates) {
    candidates = [];
    const fnRe = /\bfn\s+([A-Za-z_][A-Za-z0-9_]*)/g;
    let m: RegExpExecArray | null;
    while ((m = fnRe.exec(clean))) candidates.push({ idx: m.index, name: m[1], afterName: m.index + m[0].length });
    fnCandidateCache.set(absPath, candidates);
  }
  for (let k = candidates.length - 1; k >= 0; k--) {
    const c = candidates[k];
    if (c.idx >= pos) continue;
    let i = c.afterName;
    while (i < clean.length && /\s/.test(clean[i])) i++;
    if (clean[i] === "<") {
      let depth = 1;
      i++;
      while (i < clean.length && depth > 0) {
        if (clean[i] === "<") depth++;
        else if (clean[i] === ">") {
          if (clean[i - 1] !== "-") depth--;
        }
        i++;
      }
    }
    while (i < clean.length && /\s/.test(clean[i])) i++;
    if (clean[i] !== "(") continue;
    const parenStart = i;
    let depth = 1;
    i = parenStart + 1;
    while (i < clean.length && depth > 0) {
      if (clean[i] === "(") depth++;
      else if (clean[i] === ")") depth--;
      i++;
    }
    if (depth !== 0) continue;
    const paramsEnd = i - 1;
    const sig = findBodyOrDecl(clean, paramsEnd + 1);
    if (!sig || !sig.hasBody || sig.bodyStart === undefined) continue;
    const bodyEnd = matchBraceForward(clean, sig.bodyStart);
    if (bodyEnd >= clean.length) continue; // unbalanced — refuse silently, caller sees null
    if (pos > sig.bodyStart && pos < bodyEnd) {
      let asyncCheck = c.idx - 1;
      while (asyncCheck >= 0 && /\s/.test(clean[asyncCheck])) asyncCheck--;
      let asyncWordEnd = asyncCheck + 1;
      let asyncWordStart = asyncWordEnd;
      while (asyncWordStart > 0 && /[A-Za-z0-9_]/.test(clean[asyncWordStart - 1])) asyncWordStart--;
      const isAsync = clean.slice(asyncWordStart, asyncWordEnd) === "async";
      return { name: c.name, isAsync, paramsStart: parenStart + 1, paramsEnd, bodyStart: sig.bodyStart, bodyEnd };
    }
  }
  return null;
}

// Simple-identifier parameter names only (top-level comma split, `&`/`&mut`/`mut` stripped,
// `self` variants excluded). Destructuring params are intentionally NOT enumerated here — a
// variable bound by a tuple/struct-pattern parameter never matches a bare-identifier name via
// this parser, so it naturally falls through to the "no let-binding found" refusal instead,
// which is safe (a refusal either way).
// R14: a real bug this packet's OWN dry-run/monotonic-guard caught (not merely a theoretical
// one) — a `.await` insertion can be textually correct (real call, real callee) while still
// being WRONG for a reason none of R13's existing guards check: the insertion point may not be
// lexically inside ANY async context at all. `.await` is legal Rust syntax anywhere inside an
// `async fn`, an `async {}` block, or an `async move? |..| {}` closure — but a Future-typed LOCAL
// VARIABLE can perfectly legally exist and be passed around inside an ordinary SYNC function
// too (it just can never be `.await`ed there). Confirmed on `semio-framework-surface`: the
// generic "mismatched types" diagnostic pattern (broad by necessity — see ASYNC_SIGNATURE_
// PATTERNS) misattributed a completely unrelated diagnostic to `let plane_normal =
// Vec3::new(0.0, 0.0, 1.0);` inside an ordinary, never-async pointer-picking function; `Vec3::new`
// is fully synchronous. Both R13's pre-existing call-add-await paths AND the new R14 def-use path
// share this exposure — a diagnostic misattribution can point ANY of them at a position that was
// never inside async code, and rustc's response to an illegal `.await` there
// ("`await` is only allowed inside `async` functions and blocks") is a different failure mode
// from the pattern-position/struct-shorthand/stacking corruptions R13's incidents already
// catalogue. Caught here by the monotonic guard exactly as designed (reverted, zero corruption
// on disk) — but rather than rely on catch-and-revert forever, this closes the gap at all three
// `.await`-insertion sites (both pre-existing R13 sites and the new R14 site) with a shared,
// conservative, span-local check: an insertion point is accepted only if its innermost enclosing
// named function is itself `async`, OR it sits inside a nested `async {}` / `async move? |..|`
// block/closure within that function. Anything else refuses into residue with a specific reason.
function hasEnclosingAsyncBlockOrClosure(clean: string, regionStart: number, regionEnd: number, pos: number): boolean {
  const re = /\basync\b/g;
  re.lastIndex = regionStart;
  let m: RegExpExecArray | null;
  while ((m = re.exec(clean)) && m.index < pos) {
    let i = m.index + 5;
    while (i < regionEnd && /\s/.test(clean[i])) i++;
    if (clean.slice(i, i + 4) === "move" && !/[A-Za-z0-9_]/.test(clean[i + 4] ?? "")) {
      i += 4;
      while (i < regionEnd && /\s/.test(clean[i])) i++;
    }
    let zoneEnd = -1;
    if (clean[i] === "{") {
      const be = matchBraceForward(clean, i);
      zoneEnd = be >= clean.length ? regionEnd : be + 1;
    } else if (clean[i] === "|") {
      let depth = 0;
      let j = i + 1;
      let found = -1;
      while (j < regionEnd) {
        const cj = clean[j];
        if (cj === "(" || cj === "[") depth++;
        else if (cj === ")" || cj === "]") depth--;
        else if (cj === "|" && depth === 0 && clean[j + 1] !== "|" && clean[j - 1] !== "|") {
          found = j;
          break;
        } else if (cj === ";" && depth === 0) break;
        j++;
      }
      if (found > 0) {
        let k = found + 1;
        while (k < regionEnd && /\s/.test(clean[k])) k++;
        if (clean[k] === "{") {
          const be = matchBraceForward(clean, k);
          zoneEnd = be >= clean.length ? regionEnd : be + 1;
        } else {
          let d = 0;
          let z = k;
          while (z < regionEnd) {
            const cz = clean[z];
            if (cz === "(" || cz === "[" || cz === "{") d++;
            else if (cz === ")" || cz === "]" || cz === "}") {
              if (d === 0) break;
              d--;
            } else if ((cz === "," || cz === ";") && d === 0) break;
            z++;
          }
          zoneEnd = z;
        }
      }
    } else {
      continue; // e.g. `async fn` — the named-function case, handled separately via EnclosingFn.isAsync
    }
    if (zoneEnd > pos) return true;
  }
  return false;
}

// The shared gate: refuses any `.await`-insertion position that is not lexically inside async
// code. `absPath`/`clean` must already be loaded (loadFile) by the caller.
function isInsideAsyncContext(absPath: string, clean: string, pos: number): boolean {
  const fn = findEnclosingFunction(absPath, clean, pos);
  if (!fn) return false; // cannot even locate an enclosing function — refuse rather than assume
  if (fn.isAsync) return true;
  return hasEnclosingAsyncBlockOrClosure(clean, fn.bodyStart, fn.bodyEnd, pos);
}

function paramNamesOf(clean: string, paramsStart: number, paramsEnd: number): Set<string> {
  const text = clean.slice(paramsStart, paramsEnd);
  const names = new Set<string>();
  let depth = 0;
  let last = 0;
  const parts: string[] = [];
  for (let i = 0; i < text.length; i++) {
    const c = text[i];
    if (c === "(" || c === "[" || c === "<") depth++;
    else if (c === ")" || c === "]") depth--;
    else if (c === ">") {
      if (text[i - 1] !== "-" && depth > 0) depth--;
    } else if (c === "," && depth === 0) {
      parts.push(text.slice(last, i));
      last = i + 1;
    }
  }
  parts.push(text.slice(last));
  for (const raw of parts) {
    let p = raw.trim();
    if (!p || /^(&\s*(mut\s+)?)?mut\s+self$/.test(p) || /^&?\s*self$/.test(p)) continue;
    p = p.replace(/^&\s*(mut\s+)?/, "").replace(/^mut\s+/, "");
    const colonIdx = p.indexOf(":");
    const namePart = (colonIdx >= 0 ? p.slice(0, colonIdx) : p).trim();
    if (/^[A-Za-z_][A-Za-z0-9_]*$/.test(namePart)) names.add(namePart);
  }
  return names;
}

interface Zone {
  start: number;
  end: number;
}

// Best-effort closure-span detector. `|` is lexically ambiguous with bitwise-or, logical-or
// (`||`), and or-patterns in match arms — this deliberately does NOT try to be a real Rust
// parser. It only fires when a `|` (or `||`, the empty-params case) is IMMEDIATELY preceded
// (after whitespace) by `move`, or by one of `( , = { ;`, or is at the very start of the scanned
// region — contexts where a bitwise/logical-or genuinely cannot appear (an operator needs a
// left-hand operand token directly before it, not an opening delimiter). False negatives (a real
// closure this misses) fall through to the general "no let-binding found" refusal downstream if
// it hides the binding/use; false positives just widen a zone, which can only ever cause an
// EXTRA refusal, never a wrong edit — the same false-refusal-is-cheap philosophy as R13's other
// guards.
function computeClosureZones(clean: string, regionStart: number, regionEnd: number): Zone[] {
  const zones: Zone[] = [];
  let i = regionStart;
  while (i < regionEnd) {
    if (clean[i] !== "|") {
      i++;
      continue;
    }
    let p = i - 1;
    while (p >= regionStart && /\s/.test(clean[p])) p--;
    const prevCh = p >= regionStart ? clean[p] : "";
    let isMove = false;
    if (/[A-Za-z_]/.test(prevCh)) {
      let ws = p;
      while (ws >= regionStart && /[A-Za-z0-9_]/.test(clean[ws])) ws--;
      if (clean.slice(ws + 1, p + 1) === "move") isMove = true;
    }
    const introducerOk = isMove || prevCh === "" || "(,={;".includes(prevCh);
    if (!introducerOk) {
      i++;
      continue;
    }

    let paramsEndExclusive: number;
    if (clean[i + 1] === "|") {
      paramsEndExclusive = i + 2; // empty-params closure `||`
    } else {
      let depth = 0;
      let j = i + 1;
      let found = -1;
      while (j < regionEnd && j - i < 2000) {
        const cj = clean[j];
        if (cj === "(" || cj === "[") depth++;
        else if (cj === ")" || cj === "]") depth--;
        else if (cj === "|" && depth === 0 && clean[j + 1] !== "|" && clean[j - 1] !== "|") {
          found = j;
          break;
        } else if (cj === ";" && depth === 0) break;
        j++;
      }
      if (found < 0) {
        i++;
        continue;
      }
      paramsEndExclusive = found + 1;
    }

    let k = paramsEndExclusive;
    while (k < regionEnd && /\s/.test(clean[k])) k++;
    let zoneEnd: number;
    if (clean[k] === "{") {
      const be = matchBraceForward(clean, k);
      zoneEnd = be >= clean.length ? regionEnd : be + 1;
    } else {
      let d = 0;
      let z = k;
      while (z < regionEnd) {
        const cz = clean[z];
        if (cz === "(" || cz === "[" || cz === "{") d++;
        else if (cz === ")" || cz === "]" || cz === "}") {
          if (d === 0) break;
          d--;
        } else if ((cz === "," || cz === ";") && d === 0) break;
        z++;
      }
      zoneEnd = z;
    }
    zones.push({ start: i, end: zoneEnd });
    i = zoneEnd;
  }
  return zones;
}

function insideZones(zones: Zone[], pos: number): boolean {
  return zones.some((z) => pos >= z.start && pos < z.end);
}

interface LetBinding {
  letStart: number;
  eqPos: number; // char offset of the `=`
  stmtEnd: number; // char offset of the terminating `;`
}

// Enumerates `let (mut)? NAME (: TYPE)? = <rhs>;` bindings of `varName` inside [bodyStart,
// bodyEnd), skipping the bodies of any NESTED named `fn` items (a different function's own
// scope — mirrors the same nested-fn skip census's scanBody() uses for `.await` accounting).
// Destructuring/enum-variant patterns (`let (a, b) = ..`, `let Some(x) = ..`) never match: the
// identifier captured right after `let`/`mut` is the pattern's own leading token ("Some", not
// "x"), so they correctly produce zero bindings for the pattern-bound name.
function findLetBindingsSkippingNestedFns(clean: string, bodyStart: number, bodyEnd: number, varName: string): LetBinding[] {
  const results: LetBinding[] = [];
  let i = bodyStart + 1;
  while (i < bodyEnd) {
    const ch = clean[i];
    if (!/[A-Za-z_]/.test(ch)) {
      i++;
      continue;
    }
    let j = i;
    while (j < bodyEnd && /[A-Za-z0-9_]/.test(clean[j])) j++;
    const word = clean.slice(i, j);

    if (word === "fn") {
      let k = j;
      while (k < bodyEnd && /\s/.test(clean[k])) k++;
      if (/[A-Za-z_]/.test(clean[k] ?? "")) {
        let nameEnd = k;
        while (nameEnd < bodyEnd && /[A-Za-z0-9_]/.test(clean[nameEnd])) nameEnd++;
        const sig = findBodyOrDecl(clean, nameEnd);
        if (sig && sig.hasBody && sig.bodyStart !== undefined) {
          const nestedEnd = matchBraceForward(clean, sig.bodyStart);
          i = nestedEnd < bodyEnd ? nestedEnd + 1 : bodyEnd;
          continue;
        } else if (sig && !sig.hasBody && sig.declEnd !== undefined) {
          i = sig.declEnd + 1;
          continue;
        }
      }
      i = j;
      continue;
    }

    if (word === "let") {
      let k = j;
      while (k < bodyEnd && /\s/.test(clean[k])) k++;
      if (clean.slice(k, k + 3) === "mut" && /\s/.test(clean[k + 3] ?? "")) {
        k += 3;
        while (k < bodyEnd && /\s/.test(clean[k])) k++;
      }
      const nameStart = k;
      let nameEnd = k;
      while (nameEnd < bodyEnd && /[A-Za-z0-9_]/.test(clean[nameEnd])) nameEnd++;
      const name = clean.slice(nameStart, nameEnd);
      let p = nameEnd;
      while (p < bodyEnd && /\s/.test(clean[p])) p++;
      if (clean[p] === ":") {
        p++;
        let depth = 0;
        let stopped = false;
        while (p < bodyEnd) {
          const cp = clean[p];
          if (cp === "(" || cp === "[" || cp === "<") depth++;
          else if (cp === ")" || cp === "]") depth--;
          else if (cp === ">") {
            if (clean[p - 1] !== "-" && depth > 0) depth--;
          } else if (cp === "=" && depth === 0) {
            stopped = true;
            break;
          } else if (cp === ";" && depth === 0) {
            stopped = true;
            break;
          }
          p++;
        }
        if (!stopped) p = bodyEnd;
      }
      while (p < bodyEnd && /\s/.test(clean[p])) p++;
      if (clean[p] === "=" && clean[p + 1] !== "=") {
        const eqPos = p;
        let d = 0;
        let q = eqPos + 1;
        while (q < bodyEnd) {
          const cq = clean[q];
          if (cq === "(" || cq === "[" || cq === "{") d++;
          else if (cq === ")" || cq === "]" || cq === "}") d--;
          else if (cq === ";" && d === 0) break;
          q++;
        }
        if (name === varName && q < bodyEnd) {
          results.push({ letStart: i, eqPos, stmtEnd: q });
        }
      }
      i = nameEnd > i ? nameEnd : j;
      continue;
    }

    i = j;
  }
  return results;
}

function checkReassignedOrMutated(clean: string, varName: string, from: number, to: number): string | null {
  if (to <= from) return null;
  const region = clean.slice(from, to);
  const escaped = varName.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const reassignRe = new RegExp(`\\b${escaped}\\b\\s*(=[^=]|\\+=|-=|\\*=|/=|%=|&=|\\|=|\\^=|<<=|>>=)`);
  if (reassignRe.test(region)) return `variable "${varName}" is reassigned between its binding and this use — cannot assume it is still the same Future, refused`;
  const mutBorrowRe = new RegExp(`&\\s*mut\\s+${escaped}\\b`);
  if (mutBorrowRe.test(region)) return `variable "${varName}" is mutably borrowed between its binding and this use — cannot assume it is unchanged, refused`;
  return null;
}

function checkCfgAbove(src: string, letCharOffset: number): boolean {
  const line = lineOfOffset(src, letCharOffset);
  const lines = src.split("\n");
  for (let l = Math.max(0, line - 4); l < line - 1 && l < lines.length; l++) {
    if (/#\[\s*cfg\s*\(/.test(lines[l])) return true;
  }
  return false;
}

const RHS_BLOCK_KEYWORDS = new Set(["if", "match", "unsafe", "loop", "while", "for", "async", "move"]);

// R14: Rust reserved words are lexically indistinguishable from identifiers, so a diagnostic's
// Future-expr span can start at one (e.g. `crate::Foo::bar()?` — the `?`-operator diagnostic's
// span landed on the leading `crate` keyword in real code this packet touched). None of these
// are ever a bindable local variable; def-use is never attempted for them, and the residue
// reason says so explicitly instead of the misleading generic "no let-binding found" message.
const RUST_RESERVED_WORDS = new Set([
  "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in",
  "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct", "super",
  "trait", "true", "type", "unsafe", "use", "where", "while", "async", "await", "dyn", "abstract", "become", "box",
  "do", "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try", "union", "'static",
]);

interface RhsCall {
  calleeName: string;
  callEnd: number;
}

type RhsParseResult = { ok: true; call: RhsCall } | { ok: false; reason: string };

// Requires the RHS to be EXACTLY a bare, possibly path-qualified, possibly turbofished call —
// `[Path::]*NAME[::<T>](args)` — with nothing else in the statement. Anything else (a macro
// `foo!(..)`, a method chain `recv.method()`, a trailing `?`, a block/conditional expression) is
// refused rather than guessed at, per the task brief's explicit refusal list. Only the trailing
// bare identifier is returned as the callee name — consistent with how resolveCallee()/
// extractCalleeNameBackward() already resolve callees by bare name only, never by full path.
function parseSimpleCallRhs(clean: string, rhsStart: number, stmtEnd: number): RhsParseResult {
  let i = rhsStart;
  while (i < stmtEnd && /\s/.test(clean[i])) i++;
  if (!/[A-Za-z_]/.test(clean[i] ?? "")) {
    return { ok: false, reason: "RHS does not start with an identifier (block/literal/reference/other expression) — not a plain call" };
  }
  const firstWordStart = i;
  let j = i;
  while (j < stmtEnd && /[A-Za-z0-9_]/.test(clean[j])) j++;
  const firstWord = clean.slice(firstWordStart, j);
  if (RHS_BLOCK_KEYWORDS.has(firstWord)) {
    return { ok: false, reason: `RHS is a "${firstWord}" block/conditional expression, not a plain call` };
  }
  let lastSegStart = firstWordStart;
  let lastSegEnd = j;
  i = j;
  while (clean[i] === ":" && clean[i + 1] === ":") {
    const k = i + 2;
    if (clean[k] === "<") break; // turbofish — handled generically below
    if (!/[A-Za-z_]/.test(clean[k] ?? "")) break;
    let k2 = k;
    while (k2 < stmtEnd && /[A-Za-z0-9_]/.test(clean[k2])) k2++;
    lastSegStart = k;
    lastSegEnd = k2;
    i = k2;
  }
  if (clean[i] === ":" && clean[i + 1] === ":" && clean[i + 2] === "<") {
    let depth = 1;
    i += 3;
    while (i < stmtEnd && depth > 0) {
      if (clean[i] === "<") depth++;
      else if (clean[i] === ">") {
        if (clean[i - 1] !== "-") depth--;
      }
      i++;
    }
  }
  while (i < stmtEnd && /\s/.test(clean[i])) i++;
  if (clean[i] !== "(") {
    if (clean[i] === "!") return { ok: false, reason: "RHS is a macro invocation, not a resolvable fn call" };
    if (clean[i] === ".") return { ok: false, reason: "RHS is a method chain — cannot attribute the receiver's type, not a resolvable call" };
    return { ok: false, reason: "RHS is not a plain call expression" };
  }
  let depth = 1;
  let p = i + 1;
  while (p < stmtEnd && depth > 0) {
    if (clean[p] === "(") depth++;
    else if (clean[p] === ")") depth--;
    p++;
  }
  if (depth !== 0) return { ok: false, reason: "RHS call parens did not balance within the statement — needs human triage" };
  const callEnd = p;
  let q = callEnd;
  while (q < stmtEnd && /\s/.test(clean[q])) q++;
  if (q !== stmtEnd) {
    return { ok: false, reason: "RHS has trailing tokens after the call (method chain, `?`, or binary operator) — cannot attribute a single callee, not a resolvable call" };
  }
  return { ok: true, call: { calleeName: clean.slice(lastSegStart, lastSegEnd), callEnd } };
}

interface DefUseResolution {
  ok: boolean;
  reason?: string;
  calleeName?: string;
  callEnd?: number; // char offset, same coordinate space as loadFile(absPath).clean/.src
}

// The single entry point: given a use-site (absPath, char offset of the bare identifier, its
// name), trace it to its unique, unambiguous, same-function `let` binding and return the
// binding's RHS call — or a specific, logged refusal. Never guesses.
function resolveDefUse(absPath: string, useCharOffset: number, varName: string): DefUseResolution {
  const { clean, src } = loadFile(absPath);
  const fn = findEnclosingFunction(absPath, clean, useCharOffset);
  if (!fn) return { ok: false, reason: "could not locate an enclosing function body for the use site" };

  const params = paramNamesOf(clean, fn.paramsStart, fn.paramsEnd);
  if (params.has(varName)) {
    return { ok: false, reason: `"${varName}" is a function parameter of ${fn.name}, not a local let-binding — refused` };
  }

  const closureZones = computeClosureZones(clean, fn.bodyStart, fn.bodyEnd);
  const bindings = findLetBindingsSkippingNestedFns(clean, fn.bodyStart, fn.bodyEnd, varName);

  if (bindings.length === 0) {
    const bodyText = clean.slice(fn.bodyStart, fn.bodyEnd);
    const escaped = varName.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
    const patternish =
      new RegExp(`\\b(for|if\\s+let|while\\s+let|match)\\b[^;{]{0,200}\\b${escaped}\\b`).test(bodyText) ||
      closureZones.some((z) => new RegExp(`\\b${escaped}\\b`).test(clean.slice(z.start, Math.min(z.end, z.start + 200))));
    return {
      ok: false,
      reason: patternish
        ? `"${varName}" appears to be introduced by a match/if-let/for/closure-argument pattern, not a plain "let ${varName} = <call>;" binding — refused`
        : `no plain "let ${varName} = <call>;" binding found in enclosing function ${fn.name} — refused`,
    };
  }

  if (bindings.length > 1) {
    return { ok: false, reason: `"${varName}" is bound ${bindings.length} times in ${fn.name} (shadowing) — ambiguous which binding governs this use, refused` };
  }

  const b = bindings[0];

  if (insideZones(closureZones, b.letStart)) {
    return { ok: false, reason: `binding of "${varName}" occurs inside a closure — crosses closure boundary, refused` };
  }
  if (insideZones(closureZones, useCharOffset)) {
    return { ok: false, reason: `use of "${varName}" occurs inside a closure — crosses closure boundary, refused` };
  }

  if (checkCfgAbove(src, b.letStart)) {
    return { ok: false, reason: `binding of "${varName}" sits behind a #[cfg(...)] attribute this tool cannot evaluate — refused` };
  }

  if (useCharOffset < b.stmtEnd) {
    return { ok: false, reason: `use of "${varName}" occurs textually before its own binding statement — likely loop-carried or otherwise non-linear control flow, refused` };
  }

  const mutReason = checkReassignedOrMutated(clean, varName, b.stmtEnd, useCharOffset);
  if (mutReason) return { ok: false, reason: mutReason };

  const rhs = parseSimpleCallRhs(clean, b.eqPos + 1, b.stmtEnd);
  if (rhs.ok === false) return { ok: false, reason: `RHS of "let ${varName} = ..." is not a resolvable call: ${rhs.reason}` };

  return { ok: true, calleeName: rhs.call.calleeName, callEnd: rhs.call.callEnd };
}

//#endregion

//#region Per-diagnostic edit planning

interface PlannedEdit {
  file: string;
  start: number;
  end: number;
  before: string;
  after: string;
  kind: Edit["kind"];
  diagnosticCode: string | null;
  diagnosticMessage: string;
}

interface PlanResult {
  edits: PlannedEdit[];
  residue: { file: string; message: string; reason: string }[];
}

function resolveFilePath(fileName: string): string {
  return isAbsolute(fileName) ? fileName : resolve(ROOT, fileName);
}

// The "is_primary" span is NOT always the actual Future-typed expression — e.g. for
// `if let Some(out) = outward { .. }` where `outward` holds an un-awaited Future, rustc marks
// the DESTRUCTURING PATTERN `Some(out)` as primary and the variable `outward` as a secondary
// span carrying the label "this expression has type `impl Future<Output = ...>`". Blindly
// forward-extracting a call from the primary span in that shape would treat `Some(...)` as a
// callee and insert a syntactically broken `.await` — confirmed as a real, distinct diagnostic
// shape in semio-framework-os-infinite. This picks the span that rustc itself LABELS as the
// Future-typed expression when one exists, falling back to the primary span otherwise (the
// shape most of the other no-suggestion fallback patterns, e.g. "is not an iterator", actually
// have — there the primary span IS the call). extractCallForward() still refuses to extract
// anything from a bare identifier (no following `(`), so even a wrong span choice here fails
// safely into residue rather than corrupting code — this is a precision improvement, not a
// correctness requirement of the safety model.
interface FutureSpanResult {
  span: RustcSpan;
  // 🛡️ true when the span was independently identified BY RUSTC as the Future-typed expression
  // itself (a "this expression has type impl Future"/"found impl Future" label, or a structural
  // byte-adjacent sibling to the primary span) — safe to forward-extract a call from. false when
  // it is the bare primary span reached only via the last-resort fallback, with no corroborating
  // signal that it is the RECEIVER rather than a trailing method/operator name applied TO the
  // receiver — see the R15 comment at its one call site for the real-world corruption this caught.
  trusted: boolean;
}

function findFutureExprSpan(d: RustcDiagnostic): FutureSpanResult | null {
  const allSpans: RustcSpan[] = [...d.spans, ...d.children.flatMap((c) => c.spans)];
  for (const s of allSpans) {
    if (s.label && /this expression has type `?impl Future/i.test(s.label)) return { span: s, trusted: true };
  }
  for (const s of allSpans) {
    if (s.label && /found (`?impl )?[Ff]uture/i.test(s.label)) return { span: s, trusted: true };
  }
  const primary = d.spans.find((s) => s.is_primary) ?? d.spans[0] ?? null;
  // CONFIRMED REAL BUG, fixed here: for the "`T` is not an iterator" shape (and siblings), the
  // primary span covers only the short trailing failing method/operator (e.g. the ~9 bytes of
  // `into_iter`), NOT the Future-producing expression — that is a SEPARATE, non-primary,
  // unlabeled sibling span whose byte_end exactly abuts the primary span's byte_start. Blindly
  // falling back to primary here inserted `.await` AFTER the trailing method instead of before
  // it (`X.into_iter().await` instead of `X.await.into_iter()`), which does not fix the
  // underlying type error — so a LATER run's fresh diagnostic scan found the "same" error again
  // and stacked ANOTHER `.await` on top. Confirmed on `semio-framework-surface`
  // (`visible_tile_coords(..).into_iter().await.await`) and reproduced up to 6-deep elsewhere.
  // Prefer the byte-adjacent sibling span when one exists in the same file — a structural
  // signal, not a guess.
  if (primary) {
    const abutting = d.spans.find((s) => !s.is_primary && s.file_name === primary.file_name && s.byte_end === primary.byte_start);
    if (abutting) return { span: abutting, trusted: true };
  }
  // R15: NOT every "is not an iterator"/"no field ... on type impl Future"/etc. diagnostic carries
  // the abutting sibling span the fix above relies on — confirmed at scale on real
  // `semio-s-plugin-stdio` diagnostics (168 single-span occurrences in the lib target alone, e.g.
  // `<X as Mutation<Y>>::inverse(m, b).into_iter().map(...)` where rustc emits ONLY the 9-byte
  // `into_iter` span, no sibling at all). Falling back to this untrusted primary and forward-
  // extracting from it silently reproduces the SAME bug the abutting-span fix above was built to
  // close, except as a fresh misattribution rather than a re-diagnosed stack: `extractCallForward`
  // happily parses `into_iter()` itself as "the call" and inserts `.await` after ITS closing paren
  // (`X.into_iter().await`) — syntactically clean, so no self-test or parse-error sweep would have
  // caught it, and it does not fix the underlying error, which is why the caller must refuse this
  // specific shape rather than trust an untrusted span the way it trusts the other three.
  return primary ? { span: primary, trusted: false } : null;
}

// Plans a definition-edit ("remove async") for `calleeName`, expected near/at `nearOffset`
// in `absPath`, using the shared resolveCallee() guard pipeline (trait-impl / quote! / async-
// test guards all apply here too — this is the SAME safety gate as the call-site-driven path).
function planDeasyncDefinition(absPath: string, calleeName: string, plan: PlanResult, d: RustcDiagnostic, contextNote: string): boolean {
  const info = resolveCallee(absPath, calleeName);
  if (!info || info.guarded !== null || info.suspends) return false;
  const kw = locateAsyncKeyword(info.file, calleeName, info.line);
  if (!kw) {
    plan.residue.push({ file: absPath, message: d.message, reason: `${contextNote}: callee ${calleeName} classified non-suspending but async keyword not re-locatable at line ${info.line}` });
    return true; // handled as residue, do not also fall through to an await-insertion
  }
  const defSrc = loadFile(info.file).src;
  plan.edits.push({
    file: info.file,
    start: kw.start,
    end: kw.end,
    before: defSrc.slice(kw.start, kw.end),
    after: "",
    kind: "def-remove-async",
    diagnosticCode: d.code?.code ?? null,
    diagnosticMessage: `${d.message} [${contextNote}: de-async callee ${calleeName}]`,
  });
  return true;
}

// E0053: `fn NAME` in a trait impl marked `async` where the trait's own declared signature
// is plain `fn`. This is an AUTHORITATIVE, compiler-verified signal — stronger than (and an
// explicit override of) the general trait-impl guard elsewhere in this file, which exists to
// avoid guessing at traits whose real signature we haven't confirmed. Here rustc's own child
// note gives us the expected (sync) and found (async, `impl Future<...>`) signatures directly.
function tryPlanE0053(d: RustcDiagnostic, plan: PlanResult): boolean {
  if (!/has an incompatible type for trait/i.test(d.message)) return false;
  const nameMatch = /method `([A-Za-z_][A-Za-z0-9_]*)`/.exec(d.message);
  if (!nameMatch) {
    plan.residue.push({ file: "?", message: d.message, reason: "E0053 but could not extract method name from message" });
    return true;
  }
  const name = nameMatch[1];
  const primary = d.spans.find((s) => s.is_primary) ?? d.spans[0];
  if (!primary) {
    plan.residue.push({ file: "?", message: d.message, reason: "E0053 but no span" });
    return true;
  }
  const childBlob = d.children.map((c) => c.message).join(" \n ");
  const expectedMatch = /expected signature `([^`]*)`/.exec(childBlob);
  const foundMatch = /found signature `([^`]*)`/.exec(childBlob);
  if (!expectedMatch || !foundMatch) {
    plan.residue.push({ file: resolveFilePath(primary.file_name), message: d.message, reason: "E0053 but expected/found signature notes not found — needs human triage" });
    return true;
  }
  if (/Future/.test(expectedMatch[1])) {
    // trait itself genuinely wants an async-shaped signature; something else is wrong. Do not touch.
    plan.residue.push({ file: resolveFilePath(primary.file_name), message: d.message, reason: "E0053 but trait's OWN expected signature already mentions Future — not the async-fn-without-await bug class, needs human triage" });
    return true;
  }
  if (!/Future/.test(foundMatch[1])) {
    plan.residue.push({ file: resolveFilePath(primary.file_name), message: d.message, reason: "E0053 but found signature does not mention Future — not this bug class" });
    return true;
  }
  const absPath = resolveFilePath(primary.file_name);
  if (!isEditableFile(absPath)) return true;
  if (!existsSync(absPath)) {
    plan.residue.push({ file: absPath, message: d.message, reason: "E0053 file not found on disk" });
    return true;
  }
  const { src, clean } = loadFile(absPath);
  const escaped = name.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const re = new RegExp(`\\basync\\s+fn\\s+${escaped}\\b`, "g");
  const primaryStartChar = toCharOffset(absPath, primary.byte_start);
  const primaryEndChar = toCharOffset(absPath, primary.byte_end);
  const windowStart = Math.max(0, primaryStartChar - 400);
  const windowEnd = Math.min(clean.length, primaryEndChar + 400);
  let best: RegExpExecArray | null = null;
  let m: RegExpExecArray | null;
  while ((m = re.exec(clean))) {
    if (m.index >= windowStart && m.index <= windowEnd) {
      best = m;
      break;
    }
  }
  if (!best) {
    plan.residue.push({ file: absPath, message: d.message, reason: `E0053: could not locate "async fn ${name}" near primary span [${primary.byte_start},${primary.byte_end})` });
    return true;
  }
  const asyncKw = /^async\s+/.exec(best[0])!;
  const start = best.index;
  const end = best.index + asyncKw[0].length;
  const line = lineOfOffset(src, start);
  if (hasAsyncTestAttribute(src, line)) {
    plan.residue.push({ file: absPath, message: d.message, reason: "E0053 target carries an async-test attribute — guarded, not touched" });
    return true;
  }
  plan.edits.push({
    file: absPath,
    start,
    end,
    before: src.slice(start, end),
    after: "",
    kind: "def-remove-async",
    diagnosticCode: d.code?.code ?? null,
    diagnosticMessage: `${d.message} [E0053: trait declares sync signature]`,
  });
  return true;
}

// E0733 "recursion in an async fn requires boxing": NOT in the task's enumerated bug-class
// list, but its own diagnostic spans hand us the exact recursion cycle for free — the top-level
// diagnostic's spans and every child "which leads to this async fn" note's spans each contain
// one member fn's full signature text (`async fn NAME(...)`). If EVERY member's own-level
// `.await`s target ONLY fellow members of this same cycle (i.e. there is no OTHER genuine
// suspension hiding in any of these bodies — verified via ownAwaitCalleeNames, not assumed),
// the cycle can be de-asynced as one atomic edit set: plain mutual/self recursion needs no
// Box::pin at all, so the error disappears rather than needing the boxing fix. If any member
// has an award to something outside the cycle, this is refused entirely (residue) rather than
// guessing — a genuinely mixed recursive+suspending function needs a human's Box::pin call.
function tryPlanE0733(d: RustcDiagnostic, plan: PlanResult): boolean {
  if (!/recursion in an async fn requires boxing/i.test(d.message)) return false;

  const allSpans: RustcSpan[] = [...d.spans, ...d.children.flatMap((c) => c.spans)];
  interface Member {
    absPath: string;
    name: string;
    asyncStart: number;
    asyncEnd: number;
    bodyStart: number;
  }
  const members: Member[] = [];
  const seen = new Set<string>();
  for (const s of allSpans) {
    const absPath = resolveFilePath(s.file_name);
    if (!isEditableFile(absPath)) continue;
    if (!existsSync(absPath)) continue;
    const { clean } = loadFile(absPath);
    const startChar = toCharOffset(absPath, s.byte_start);
    const endChar = toCharOffset(absPath, s.byte_end);
    const text = clean.slice(startChar, endChar);
    const m = /\basync\s+fn\s+([A-Za-z_][A-Za-z0-9_]*)/.exec(text);
    if (!m) continue;
    const name = m[1];
    const key = `${absPath}:${name}`;
    if (seen.has(key)) continue;
    seen.add(key);
    const asyncStart = startChar + m.index;
    const asyncKw = /^async\s+/.exec(m[0])!;
    const asyncEnd = asyncStart + asyncKw[0].length;
    const fnKwIdx = clean.indexOf("fn", asyncStart);
    const sig = findBodyOrDecl(clean, fnKwIdx + 2);
    if (!sig || !sig.hasBody || sig.bodyStart === undefined) continue;
    members.push({ absPath, name, asyncStart, asyncEnd, bodyStart: sig.bodyStart });
  }

  if (members.length < 2) {
    plan.residue.push({ file: members[0]?.absPath ?? "?", message: d.message, reason: "E0733: could not extract at least 2 cycle members from diagnostic spans — needs human triage" });
    return true;
  }

  const memberNames = new Set(members.map((m) => m.name));
  for (const mem of members) {
    const { clean } = loadFile(mem.absPath);
    const calleeNames = ownAwaitCalleeNames(clean, mem.bodyStart);
    const outside = calleeNames.filter((n) => !memberNames.has(n));
    if (outside.length > 0) {
      plan.residue.push({ file: mem.absPath, message: d.message, reason: `E0733: cycle member ${mem.name} awaits outside the cycle (${outside.join(", ")}) — genuinely mixed recursion+suspension, needs a human Box::pin decision` });
      return true;
    }
    if (isInsideTraitImpl(clean, mem.asyncStart)) {
      plan.residue.push({ file: mem.absPath, message: d.message, reason: `E0733: cycle member ${mem.name} is inside a trait impl — guarded, not touched` });
      return true;
    }
    const ranges = getQuoteRanges(mem.absPath);
    if (insideAny(ranges, mem.asyncStart)) {
      plan.residue.push({ file: mem.absPath, message: d.message, reason: `E0733: cycle member ${mem.name} is inside a quote!{} macro body — guarded, not touched` });
      return true;
    }
  }

  for (const mem of members) {
    const src = loadFile(mem.absPath).src;
    plan.edits.push({
      file: mem.absPath,
      start: mem.asyncStart,
      end: mem.asyncEnd,
      before: src.slice(mem.asyncStart, mem.asyncEnd),
      after: "",
      kind: "def-remove-async",
      diagnosticCode: d.code?.code ?? null,
      diagnosticMessage: `${d.message} [E0733: de-async whole recursion cycle {${[...memberNames].join(", ")}} — no member awaits outside the cycle]`,
    });
  }
  return true;
}

function planEditsForDiagnostic(d: RustcDiagnostic, plan: PlanResult): void {
  if (tryPlanE0053(d, plan)) return;
  if (tryPlanE0733(d, plan)) return;

  const suggestions: Suggestion[] = [];
  collectSuggestions(d, suggestions);

  const usable = suggestions.filter(
    (s) => s.applicability === "MachineApplicable" || (s.applicability === "MaybeIncorrect" && /await/i.test(s.span.suggested_replacement ?? ""))
  );

  if (usable.length > 0) {
    for (const sug of usable) {
      const absPath = resolveFilePath(sug.span.file_name);
      if (!isEditableFile(absPath)) continue;
      if (!existsSync(absPath)) {
        plan.residue.push({ file: absPath, message: d.message, reason: "suggestion file not found on disk" });
        continue;
      }
      const { src, clean } = loadFile(absPath);
      const start = toCharOffset(absPath, sug.span.byte_start);
      const end = toCharOffset(absPath, sug.span.byte_end);
      const before = src.slice(start, end);
      const after = sug.span.suggested_replacement ?? "";
      const addsAwait = /await/i.test(after) && !/await/i.test(before);
      const removesAwait = /await/i.test(before) && !/await/i.test(after);

      if (removesAwait) {
        plan.edits.push({ file: absPath, start, end, before, after, kind: "call-remove-await", diagnosticCode: d.code?.code ?? null, diagnosticMessage: d.message });
        continue;
      }

      if (addsAwait) {
        if (isRiskyBareIdentifierAwaitInsertion(clean, start + before.length)) {
          plan.residue.push({ file: absPath, message: d.message, reason: "refused: .await would land on a bare identifier immediately preceded by `{`/`,` and followed by `,`/`}` — looks like a struct-literal shorthand field or tuple/array element, not a real expression tail; needs human triage" });
          continue;
        }
        if (wouldStackAwait(clean, start)) {
          plan.residue.push({ file: absPath, message: d.message, reason: "refused: this position already has a .await immediately adjacent — inserting another would stack rather than fix; the underlying error is likely mis-diagnosed (wrong span) and needs human triage" });
          continue;
        }
        const calleeName = extractCalleeNameBackward(clean, start + before.length);
        if (calleeName && PATTERN_CONSTRUCTOR_DENYLIST.has(calleeName)) {
          plan.residue.push({ file: absPath, message: d.message, reason: `refused: extracted callee "${calleeName}" is a pattern-position enum constructor, not a real call — likely a tuple/pattern mismatch diagnostic, needs human triage` });
          continue;
        }
        const handled = calleeName ? planDeasyncDefinition(absPath, calleeName, plan, d, "suggestion-driven") : false;
        if (!handled) {
          if (!isInsideAsyncContext(absPath, clean, start)) {
            plan.residue.push({ file: absPath, message: d.message, reason: "refused: .await insertion point is not lexically inside an async fn/block/closure — a Future-typed value can legally exist in sync code too; the diagnostic likely misattributed this position (see isInsideAsyncContext), needs human triage" });
            continue;
          }
          plan.edits.push({ file: absPath, start, end, before, after, kind: "call-add-await", diagnosticCode: d.code?.code ?? null, diagnosticMessage: d.message });
        }
        continue;
      }

      // Suggestion present but not an obvious add/remove-await shape — apply verbatim; it is
      // machine-applicable per rustc, and still within the Future/await diagnostic family.
      plan.edits.push({ file: absPath, start, end, before, after, kind: "call-remove-await", diagnosticCode: d.code?.code ?? null, diagnosticMessage: d.message });
    }
    return;
  }

  // No machine-applicable suggestion. Handle the "#[test] + async fn" shape explicitly; log
  // everything else as residue rather than guess at an edit without compiler-authoritative spans.
  if (/async functions cannot be used for tests/i.test(d.message)) {
    const primary = d.spans.find((s) => s.is_primary) ?? d.spans[0];
    if (!primary) {
      plan.residue.push({ file: "?", message: d.message, reason: "no span on test diagnostic" });
      return;
    }
    const absPath = resolveFilePath(primary.file_name);
    if (!isEditableFile(absPath)) return;
    const { src, clean } = loadFile(absPath);
    const searchFrom = toCharOffset(absPath, primary.byte_end);
    const re = /\basync\s+fn\s+([A-Za-z_][A-Za-z0-9_]*)/;
    const window = clean.slice(searchFrom, searchFrom + 500);
    const m = re.exec(window);
    if (!m) {
      plan.residue.push({ file: absPath, message: d.message, reason: "could not locate async fn after #[test] attribute within 500-char window" });
      return;
    }
    const asyncStart = searchFrom + m.index;
    const asyncKw = /^async\s+/.exec(m[0])!;
    const start = asyncStart;
    const end = asyncStart + asyncKw[0].length;
    plan.edits.push({ file: absPath, start, end, before: src.slice(start, end), after: "", kind: "test-remove-async", diagnosticCode: d.code?.code ?? null, diagnosticMessage: d.message });
    return;
  }

  // "opaque `impl Future` used where a plain value was expected" shapes that rustc gives NO
  // suggestion for at all (e.g. E0277 "is not an iterator", or "mismatched types" against an
  // opaque Future with no `.await`ing help offered). Locate the actual Future-typed expression
  // via findFutureExprSpan() (NOT blindly the primary span — see its comment for the
  // `if let Some(out) = outward` counter-example) and extract a call forward from its start.
  // Apply the same decision rule: de-async a non-suspending callee; otherwise insert `.await`
  // right after the call. When the Future-typed expression is a bare variable rather than a
  // fresh call (the common case for this residue: a `let`-bound Future used later), forward
  // extraction correctly fails and this falls through to residue rather than guessing.
  const NO_SUGGESTION_FALLBACK_PATTERNS = [/is not an iterator/i, /no field .* on type `?impl Future/i, /cannot apply unary operator .* to type `?impl Future/i, /the `?\??`? operator can only be applied/i, /binary operation .* cannot be applied to type `?impl Future/i, /cannot (add|subtract|multiply|divide) .*`?impl Future/i, /mismatched types/i];
  if (NO_SUGGESTION_FALLBACK_PATTERNS.some((re) => re.test(d.message))) {
    const futureSpanResult = findFutureExprSpan(d);
    if (!futureSpanResult) {
      plan.residue.push({ file: "?", message: d.message, reason: "no-suggestion fallback pattern matched but diagnostic has no span" });
      return;
    }
    const futureSpan = futureSpanResult.span;
    // R15: SIXTH real bug, a second manifestation of the FIFTH failure mode listed for this
    // packet ("diagnostics misattributed to fully synchronous expressions") that isInsideAsyncContext
    // does NOT catch, because it only asks "is this position lexically inside an async fn/block?" —
    // true here, since the enclosing fn genuinely IS async. Confirmed on real `semio-s-plugin-stdio`:
    // `write_box(b"moov", &[build_mvhd(&snapshot.movie), traks, build_udta(&snapshot.movie)].concat())`
    // — an array literal mixing two un-awaited async calls with one ordinary `Vec<u8>` local
    // (`traks`). rustc's element-type unification picks the FIRST element's type (the un-awaited
    // Future from `build_mvhd(..)`) as "expected" for the whole array, then reports the mismatch at
    // the first element that doesn't match — `traks`, an entirely innocent bystander — with the
    // unmistakable, self-describing label `"expected future, found `Vec<u8>`"`. That label PROVES
    // the span is the CONCRETE ("found") side, not the future-producing expression, so treating it
    // as one (as def-use/extractCallForward would) can only be wrong; the real fix (awaiting the
    // Future-typed SIBLING elements earlier in the same array/call) needs sibling-expression
    // reasoning this span-local tool deliberately does not attempt. Refuse rather than guess.
    if (futureSpan.label && /^expected\s+(`?impl\s+)?[Ff]uture.*,\s*found\s+/i.test(futureSpan.label)) {
      plan.residue.push({
        file: resolveFilePath(futureSpan.file_name),
        message: d.message,
        reason: `no-suggestion fallback: Future-expr span's own label is "${futureSpan.label}" — this proves the span is the CONCRETE ("found") side of an element-type-unification mismatch (e.g. an array/tuple/call-argument literal mixing un-awaited async calls with an ordinary value), not the Future-producing expression itself; the real fix is on an unidentified sibling expression, which needs reasoning this tool does not attempt — refused, needs human triage`,
      });
      return;
    }
    const absPath = resolveFilePath(futureSpan.file_name);
    if (!isEditableFile(absPath)) return;
    if (!existsSync(absPath)) {
      plan.residue.push({ file: absPath, message: d.message, reason: "no-suggestion fallback: file not found on disk" });
      return;
    }
    const { src, clean } = loadFile(absPath);
    const primaryStartChar = toCharOffset(absPath, futureSpan.byte_start);
    const call = extractCallForward(clean, primaryStartChar);
    // R15: FIFTH real bug, found applying this packet to real `semio-s-plugin-stdio` diagnostics
    // (168 occurrences in the lib target alone). When findFutureExprSpan() had to fall back to
    // the untrusted bare primary span (no rustc label, no abutting sibling — see its comment) AND
    // that span is immediately preceded by `.`, it is a TRAILING method/field name in a dot chain
    // (e.g. `<T as Trait>::inverse(m, b).into_iter().map(...)`, primary span = `into_iter` only,
    // no sibling at all). extractCallForward() then happily parses `into_iter()` itself as "the
    // call" and would insert `.await` after ITS OWN closing paren — syntactically clean
    // (`X.into_iter().await.map(...)`), so no parse-error sweep catches it, but it does not fix
    // the underlying error (the receiver, not `.into_iter()`'s result, is the un-awaited Future).
    // This is bug #2's exact symptom (misattributed trailing-method `.await`) rediscovered via a
    // different mechanism (a genuinely single-span diagnostic, not a mis-selected sibling), so it
    // needs its own refusal rather than being folded into the existing abutting-span preference,
    // which has nothing to select from here. A trusted span (label-matched or abutting-sibling)
    // legitimately CAN be dot-preceded — e.g. `self.fetch_data()` where `fetch_data` really is the
    // Future-producing callee — so this guard only fires on the untrusted-fallback case.
    if (call && !futureSpanResult.trusted && primaryStartChar > 0 && clean[primaryStartChar - 1] === ".") {
      plan.residue.push({
        file: absPath,
        message: d.message,
        reason: `no-suggestion fallback: Future-expr span is an untrusted primary-fallback (no rustc label, no abutting sibling span) immediately preceded by "." — forward-extracting "${call.name}(...)" here would treat a trailing dot-chained method/field as the callee, misattributing .await to it instead of the actual Future-producing receiver earlier in the chain (bug#2's symptom, rediscovered on a genuinely single-span diagnostic); refused pending a real receiver-locating heuristic, needs human triage`,
      });
      return;
    }
    if (!call) {
      // R14: the dominant residue shape — the Future-typed expression is a bare, let-bound
      // local (no `(` follows the identifier), not a fresh call. Trace it to its own binding
      // statement in the same enclosing function and apply the identical decision rule to the
      // BINDING's RHS call (never the use site — see the R14 region's header comment).
      const identMatch = /^[A-Za-z_][A-Za-z0-9_]*/.exec(clean.slice(primaryStartChar));
      if (identMatch && !RUST_RESERVED_WORDS.has(identMatch[0])) {
        const varName = identMatch[0];
        // R15: EIGHTH real bug, the mirror image of the sixth. `extractCallForward` already
        // failed at this exact position (that is why we are in this branch at all), which only
        // proves the identifier is not IMMEDIATELY followed by `(` — it says nothing about
        // whether the identifier is a bare variable USE (def-use's whole premise) versus the
        // RECEIVER of a method chain, e.g. the Future-expr span covers `myc.nodes()` in full (a
        // genuinely async `.nodes()` call on an ordinary, already-synchronous `myc: Storage<..>`)
        // and `identMatch` only captures its leading `myc`. Confirmed on real `semio-s-plugin-
        // stdio` (`graph::operators-internals`): def-use dutifully traced "myc" to its own
        // (non-async, already-correct) binding `let myc = mycielskian(&g);` and inserted
        // `.await` THERE — `mycielskian(&g).await` — which rustc's own SUBSEQUENT diagnostic
        // (`` `graph_core::Storage<Normal, Undirected>` is not a future ``, appearing only once
        // the bad `.await` was actually on disk) proved is not a future at all; the real fix was
        // `.await` after `myc.nodes()`'s OWN closing paren, an ordinary call-add-await on `nodes`
        // this branch never gets a chance to consider because def-use hijacks the bare leading
        // identifier first. Caught by the monotonic guard tripping (13091 >= 13091) on iteration
        // 3, root-caused against the pre-edit diagnostic JSON (`mycielskian` confirmed plain
        // `pub fn`, never `async`, by grep), not guessed at. Refuse rather than pick a side.
        if (clean[primaryStartChar + identMatch[0].length] === ".") {
          plan.residue.push({
            file: absPath,
            message: d.message,
            reason: `def-use: identifier "${varName}" at the Future-expr span start is immediately followed by "." — the span covers a RECEIVER.method(..) chain, not a bare variable use; def-use's bare-variable premise does not hold (the Future may come from the trailing method, not this identifier's own binding) — refused, needs human triage`,
          });
          return;
        }
        const defUse = resolveDefUse(absPath, primaryStartChar, varName);
        if (defUse.ok && defUse.calleeName && defUse.callEnd !== undefined) {
          if (PATTERN_CONSTRUCTOR_DENYLIST.has(defUse.calleeName)) {
            plan.residue.push({ file: absPath, message: d.message, reason: `def-use: resolved callee "${defUse.calleeName}" is a pattern-position enum constructor, not a real call — refused` });
            return;
          }
          const handled = planDeasyncDefinition(absPath, defUse.calleeName, plan, d, "def-use");
          if (!handled) {
            if (isRiskyBareIdentifierAwaitInsertion(clean, defUse.callEnd)) {
              plan.residue.push({ file: absPath, message: d.message, reason: `def-use: .await insertion at the binding-site call would land on a bare-identifier struct-shorthand/tuple-element position — refused` });
              return;
            }
            if (wouldStackAwait(clean, defUse.callEnd)) {
              plan.residue.push({ file: absPath, message: d.message, reason: `def-use: binding-site insertion position already has .await immediately adjacent — would stack, refused` });
              return;
            }
            if (!isInsideAsyncContext(absPath, clean, defUse.callEnd)) {
              plan.residue.push({ file: absPath, message: d.message, reason: `def-use: binding-site of "${varName}" is not lexically inside an async fn/block/closure — a Future-typed local can legally exist in sync code too; the diagnostic likely misattributed this use site, refused` });
              return;
            }
            plan.edits.push({
              file: absPath,
              start: defUse.callEnd,
              end: defUse.callEnd,
              before: "",
              after: ".await",
              kind: "defuse-call-add-await",
              diagnosticCode: d.code?.code ?? null,
              diagnosticMessage: `${d.message} [def-use: .await inserted at binding-site RHS call for bare variable "${varName}", callee ${defUse.calleeName}]`,
            });
          }
          return;
        }
        plan.residue.push({ file: absPath, message: d.message, reason: `def-use refused for bare variable "${varName}": ${defUse.reason ?? "unknown"}` });
        return;
      }
      if (identMatch && RUST_RESERVED_WORDS.has(identMatch[0])) {
        plan.residue.push({ file: absPath, message: d.message, reason: `no-suggestion fallback: Future-expr span starts at the reserved word "${identMatch[0]}" (keyword/path-segment, not a variable) — def-use not attempted, needs human triage` });
        return;
      }
      plan.residue.push({ file: absPath, message: d.message, reason: `no-suggestion fallback: could not forward-parse a call expression at Future-expr span start byte=${futureSpan.byte_start} char=${primaryStartChar}` });
      return;
    }
    if (PATTERN_CONSTRUCTOR_DENYLIST.has(call.name)) {
      plan.residue.push({ file: absPath, message: d.message, reason: `refused: forward-extracted callee "${call.name}" is a pattern-position enum constructor, not a real call — likely a tuple/pattern mismatch diagnostic, needs human triage` });
      return;
    }
    const handled = planDeasyncDefinition(absPath, call.name, plan, d, "no-suggestion-fallback");
    if (!handled) {
      // callee genuinely suspends (or is guarded/unresolved) — insert `.await` right after the
      // call. (isRiskyBareIdentifierAwaitInsertion will always read false here in practice —
      // extractCallForward already required a real `(...)` call at this position — kept for
      // uniformity with the suggestion-driven site and as a defense-in-depth backstop.)
      if (isRiskyBareIdentifierAwaitInsertion(clean, call.callEnd)) {
        plan.residue.push({ file: absPath, message: d.message, reason: "refused: .await insertion point looks like a struct-literal shorthand/tuple-element tail — needs human triage" });
        return;
      }
      if (wouldStackAwait(clean, call.callEnd)) {
        plan.residue.push({ file: absPath, message: d.message, reason: "refused: this position already has a .await immediately adjacent — inserting another would stack rather than fix; the underlying error is likely mis-diagnosed (wrong span) and needs human triage" });
        return;
      }
      if (!isInsideAsyncContext(absPath, clean, call.callEnd)) {
        plan.residue.push({ file: absPath, message: d.message, reason: "refused: .await insertion point is not lexically inside an async fn/block/closure — a Future-typed value can legally exist in sync code too; the diagnostic likely misattributed this position, needs human triage" });
        return;
      }
      plan.edits.push({ file: absPath, start: call.callEnd, end: call.callEnd, before: "", after: ".await", kind: "call-add-await", diagnosticCode: d.code?.code ?? null, diagnosticMessage: d.message });
    }
    return;
  }

  const primary = d.spans.find((s) => s.is_primary) ?? d.spans[0];
  plan.residue.push({ file: primary ? resolveFilePath(primary.file_name) : "?", message: d.message, reason: "no machine-applicable suggestion and no recognised fallback pattern; needs human triage" });
}

//#endregion

//#region Apply / dry-run one iteration

function dedupeAndOrderEdits(edits: PlannedEdit[]): Map<string, PlannedEdit[]> {
  const byFile = new Map<string, PlannedEdit[]>();
  const seen = new Set<string>();
  for (const e of edits) {
    const key = `${e.file}:${e.start}:${e.end}`;
    if (seen.has(key)) continue;
    seen.add(key);
    if (!byFile.has(e.file)) byFile.set(e.file, []);
    byFile.get(e.file)!.push(e);
  }
  for (const [, list] of byFile) list.sort((a, b) => b.start - a.start); // back-to-front
  return byFile;
}

function applyEditsToFile(absPath: string, edits: PlannedEdit[]): void {
  let src = readFileSync(absPath, "utf8");
  // detect overlaps; drop later (already-seen) overlapping edits defensively
  const nonOverlapping: PlannedEdit[] = [];
  let lastStart = Infinity;
  for (const e of edits) {
    // edits are sorted descending by start; ensure end <= lastStart (no overlap with previous kept edit)
    if (e.end <= lastStart) {
      nonOverlapping.push(e);
      lastStart = e.start;
    }
  }
  for (const e of nonOverlapping) {
    const actual = src.slice(e.start, e.end);
    if (actual !== e.before) {
      throw new Error(`span mismatch in ${absPath} [${e.start},${e.end}): expected ${JSON.stringify(e.before)} got ${JSON.stringify(actual)} — file changed underneath us, aborting this edit`);
    }
    src = src.slice(0, e.start) + e.after + src.slice(e.end);
  }
  writeFileSync(absPath, src);
  invalidateFile(absPath);
}

//#endregion

//#region Run loop

function nowIso(): string {
  return new Date().toISOString();
}

function randomRunId(): string {
  return "r13-" + Math.random().toString(36).slice(2, 10);
}

// R14: buckets a residue reason string into a short, stable category label for auditing — the
// task brief requires "the complete refusal taxonomy with counts", which needs aggregation
// across potentially thousands of residue entries, not just the first-20 console sample below.
function residueCategory(reason: string): string {
  if (reason.startsWith('def-use refused for bare variable')) {
    const sub = reason.split(": ").slice(1).join(": ");
    if (/function parameter/.test(sub)) return "def-use: function parameter";
    if (/match\/if-let\/for\/closure-argument pattern/.test(sub)) return "def-use: pattern binding (match/if-let/for/closure-arg)";
    if (/shadowing/.test(sub)) return "def-use: shadowed binding";
    if (/crosses closure boundary/.test(sub)) return "def-use: crosses closure boundary";
    if (/#\[cfg/.test(sub)) return "def-use: binding behind #[cfg(...)]";
    if (/not lexically inside an async/.test(sub)) return "def-use: binding site not inside async context";
    if (/reassigned/.test(sub)) return "def-use: reassigned between binding and use";
    if (/mutably borrowed/.test(sub)) return "def-use: mutably borrowed between binding and use";
    if (/loop-carried/.test(sub)) return "def-use: use precedes its own binding textually";
    if (/no plain "let/.test(sub)) return "def-use: no let-binding found";
    if (/macro invocation/.test(sub)) return "def-use: RHS is a macro invocation";
    if (/method chain/.test(sub)) return "def-use: RHS is a method chain";
    if (/block\/conditional/.test(sub)) return "def-use: RHS is a block/conditional expression";
    if (/RHS/.test(sub)) return "def-use: RHS not a resolvable call (other)";
    if (/could not locate an enclosing function/.test(sub)) return "def-use: no enclosing function located";
    return "def-use: other";
  }
  if (reason.startsWith("def-use:") && /RECEIVER\.method\(\.\.\) chain/.test(reason)) return "R15: def-use identifier is a method-chain receiver, not a bare variable (eighth bug guard)";
  if (reason.startsWith("def-use:")) return "def-use: post-resolution guard (denylist/risky-insertion/stacking)";
  if (reason.startsWith("no-suggestion fallback") && /reserved word/.test(reason)) return "R13: Future-expr span starts at a reserved word (not a variable)";
  if (reason.startsWith("no-suggestion fallback") && /untrusted primary-fallback/.test(reason)) return "R15: untrusted primary-fallback dot-method/field misattribution (fifth bug guard)";
  if (reason.startsWith("no-suggestion fallback") && /CONCRETE \("found"\) side/.test(reason)) return "R15: span is the concrete side of an element-type-unification mismatch (sixth bug guard)";
  if (reason.startsWith("no-suggestion fallback")) return "R13: bare identifier, no def-use attempted (not this residue class)";
  if (reason.includes("not lexically inside an async")) return "R13/R14: .await insertion refused — not lexically inside async context";
  if (reason.startsWith("E0053")) return "R13: E0053 human-triage";
  if (reason.startsWith("E0733")) return "R13: E0733 human-triage";
  if (reason.startsWith("refused:")) return "R13: guard refusal (pattern-ctor/risky-insertion/stacking)";
  if (reason.includes("ambiguous")) return "R13: ambiguous callee resolution";
  return "other/unclassified";
}

async function runCrate(crate: string, opts: { dryRun: boolean; maxIterations: number; target?: string; verbose?: boolean }): Promise<void> {
  const runId = randomRunId();
  console.log(`[run ${runId}] crate=${crate} dryRun=${opts.dryRun} target=${opts.target ?? "(host)"}`);

  let prevErrorCount = Infinity;
  let prevDiagKeys: Set<string> = new Set();
  for (let iteration = 1; iteration <= opts.maxIterations; iteration++) {
    fileSrcCache.clear();
    fileCleanCache.clear();
    quoteRangeCache.clear();
    byteToCharCache.clear();
    fnCandidateCache.clear();
    invalidateGlobalAsyncFnIndex();

    const diags = runCargoCheckJson(crate, { tests: true, target: opts.target });
    const totalErrors = countErrors(diags);
    const asyncDiags = diags.filter(isAsyncClassDiagnostic);
    const errorDiags = diags.filter((d) => d.level === "error" && !isExcludedDiag(d));
    const diagKeys = new Set(
      errorDiags.map((d) => {
        const p = d.spans.find((s) => s.is_primary) ?? d.spans[0];
        return `${p?.file_name ?? "?"}:${p?.byte_start ?? "?"}:${d.message}`;
      })
    );

    console.log(`[run ${runId}] iteration ${iteration}: total errors=${totalErrors}, async-class=${asyncDiags.length}`);

    if (totalErrors >= prevErrorCount && iteration > 1) {
      console.log(`[run ${runId}] MONOTONIC GUARD TRIPPED: ${totalErrors} >= previous ${prevErrorCount}.`);
      const newlyAppeared = [...diagKeys].filter((k) => !prevDiagKeys.has(k));
      console.log(`[run ${runId}] ${newlyAppeared.length} newly-appeared diagnostics this iteration (sample up to 15):`);
      for (const k of newlyAppeared.slice(0, 15)) console.log(`  + ${k.slice(0, 220)}`);
      console.log(`[run ${runId}] Reverting iteration ${iteration - 1} and stopping.`);
      revertRun(runId, iteration - 1);
      return;
    }
    prevErrorCount = totalErrors;
    prevDiagKeys = diagKeys;

    if (asyncDiags.length === 0) {
      console.log(`[run ${runId}] no async-class diagnostics remain (total errors=${totalErrors}, presumably other bug classes or zero). Stopping.`);
      return;
    }

    const plan: PlanResult = { edits: [], residue: [] };
    for (const d of asyncDiags) planEditsForDiagnostic(d, plan);

    if (plan.edits.length === 0) {
      console.log(`[run ${runId}] ${asyncDiags.length} async-class diagnostics but zero automatable edits planned. Residue:`);
      const cats: Record<string, number> = {};
      for (const r of plan.residue) cats[residueCategory(r.reason)] = (cats[residueCategory(r.reason)] ?? 0) + 1;
      console.log(`[run ${runId}] residue by category:`, cats);
      for (const r of plan.residue.slice(0, opts.verbose ? plan.residue.length : 20)) console.log(`  - ${r.file}: ${r.reason} :: ${r.message.slice(0, 120)}`);
      console.log(`  (${plan.residue.length} total residue entries this iteration)`);
      return;
    }

    console.log(`[run ${runId}] iteration ${iteration}: planned ${plan.edits.length} edits (${plan.residue.length} residue)`);

    if (opts.dryRun) {
      const byKind: Record<string, number> = {};
      for (const e of plan.edits) byKind[e.kind] = (byKind[e.kind] ?? 0) + 1;
      console.log(`[run ${runId}] DRY RUN — edits by kind:`, byKind);
      for (const e of plan.edits.slice(0, opts.verbose ? plan.edits.length : 10)) {
        console.log(`  ${e.kind} ${e.file}:${e.start}-${e.end} ${JSON.stringify(e.before)} -> ${JSON.stringify(e.after)}`);
      }
      if (plan.residue.length > 0) {
        const cats: Record<string, number> = {};
        for (const r of plan.residue) cats[residueCategory(r.reason)] = (cats[residueCategory(r.reason)] ?? 0) + 1;
        console.log(`[run ${runId}] DRY RUN — residue by category:`, cats);
        console.log(`[run ${runId}] DRY RUN — residue (${plan.residue.length}):`);
        for (const r of plan.residue.slice(0, opts.verbose ? plan.residue.length : 20)) console.log(`  - ${r.file}: ${r.reason} :: ${r.message.slice(0, 120)}`);
      }
      return; // dry-run never iterates past the first plan
    }

    const byFile = dedupeAndOrderEdits(plan.edits);
    for (const [file, edits] of byFile) {
      applyEditsToFile(file, edits);
      for (const e of edits) {
        appendJournal({
          ts: nowIso(),
          runId,
          iteration,
          crate,
          file: e.file,
          start: e.start,
          end: e.end,
          before: e.before,
          after: e.after,
          kind: e.kind,
          diagnosticCode: e.diagnosticCode,
          diagnosticMessage: e.diagnosticMessage,
        });
      }
    }
  }
  console.log(`[run ${runId}] reached max-iterations=${opts.maxIterations} without reaching a fixpoint.`);
}

//#endregion

//#region Revert

// Reverts a single iteration (or, if `throughIteration` omitted, the entire run) by replaying
// journaled edits for that run in REVERSE chronological order (latest iteration first), and
// within each iteration's per-file edit set, in ASCENDING original-start order — the exact
// mirror of forward's descending-start application. See header comment for the offset-stability
// proof: because edits are non-overlapping and forward application processes descending start
// first, every edit's ORIGINAL start offset remains valid both immediately after forward
// application (mid-iteration) and, symmetrically, at revert time when undone in ascending order.
function revertRun(runId: string, throughIteration?: number): void {
  const all = readJournal().filter((e) => e.runId === runId);
  if (all.length === 0) {
    console.log(`no journal entries found for run ${runId}`);
    return;
  }
  const iterations = Array.from(new Set(all.map((e) => e.iteration))).sort((a, b) => b - a);
  for (const iter of iterations) {
    if (throughIteration !== undefined && iter !== throughIteration) continue;
    const entries = all.filter((e) => e.iteration === iter);
    const byFile = new Map<string, Edit[]>();
    for (const e of entries) {
      if (!byFile.has(e.file)) byFile.set(e.file, []);
      byFile.get(e.file)!.push(e);
    }
    for (const [file, edits] of byFile) {
      edits.sort((a, b) => a.start - b.start); // ascending = mirror of forward's descending
      let src = readFileSync(file, "utf8");
      for (const e of edits) {
        const actual = src.slice(e.start, e.start + e.after.length);
        if (actual !== e.after) {
          console.log(`REVERT MISMATCH in ${file} [${e.start}]: expected ${JSON.stringify(e.after)} got ${JSON.stringify(actual)} — skipping this entry (file may already be reverted or changed)`);
          continue;
        }
        src = src.slice(0, e.start) + e.before + src.slice(e.start + e.after.length);
      }
      writeFileSync(file, src);
      invalidateFile(file);
      console.log(`reverted ${edits.length} edit(s) in ${file} (run ${runId}, iteration ${iter})`);
    }
  }
}

//#endregion

//#region Self-test (revert correctness on a scratch fixture, small scale, no real crate touched)

function selftest(): void {
  const dir = mkdtempSync(join(tmpdir(), "r13-selftest-"));
  const f = join(dir, "fixture.rs");
  const original = `pub async fn helper_one() -> i32 { 1 }\n\npub async fn helper_two() -> i32 { 2 }\n\npub async fn caller() -> i32 {\n    let a = helper_one().await;\n    let b = helper_two().await;\n    a + b\n}\n`;
  writeFileSync(f, original);

  const runId = "selftest-run";
  const edits: Edit[] = [
    // simulate: helper_one has no genuine suspension elsewhere — de-async it (remove "async ")
    { ts: nowIso(), runId, iteration: 1, crate: "fixture", file: f, start: original.indexOf("async fn helper_one"), end: original.indexOf("async fn helper_one") + "async ".length, before: "async ", after: "", kind: "def-remove-async", diagnosticCode: null, diagnosticMessage: "selftest" },
  ];
  // apply forward (single edit, trivial ordering)
  let src = readFileSync(f, "utf8");
  for (const e of edits) {
    const actual = src.slice(e.start, e.end);
    if (actual !== e.before) throw new Error("selftest forward span mismatch");
    src = src.slice(0, e.start) + e.after + src.slice(e.end);
  }
  writeFileSync(f, src);
  for (const e of edits) appendFileSync(join(dir, "journal.jsonl"), JSON.stringify(e) + "\n");

  const afterEdit = readFileSync(f, "utf8");
  if (afterEdit.includes("async fn helper_one")) throw new Error("selftest: forward edit did not take effect");
  if (!afterEdit.includes("pub fn helper_one")) throw new Error("selftest: forward edit produced wrong text");
  console.log("selftest: forward edit OK");

  // revert using the same ascending-order algorithm as revertRun, standalone (no journal file dependency)
  let src2 = readFileSync(f, "utf8");
  const sorted = [...edits].sort((a, b) => a.start - b.start);
  for (const e of sorted) {
    const actual = src2.slice(e.start, e.start + e.after.length);
    if (actual !== e.after) throw new Error("selftest revert span mismatch");
    src2 = src2.slice(0, e.start) + e.before + src2.slice(e.start + e.after.length);
  }
  writeFileSync(f, src2);
  const afterRevert = readFileSync(f, "utf8");
  if (afterRevert !== original) {
    console.log("EXPECTED:", JSON.stringify(original));
    console.log("GOT:     ", JSON.stringify(afterRevert));
    throw new Error("selftest: revert did not restore original byte-for-byte");
  }
  console.log("selftest: revert restored original byte-for-byte — OK");

  // multi-edit ordering test: two non-overlapping edits in one file, forward descending / revert ascending
  const original2 = `fn a() { x() }\nfn b() { y() }\n`;
  writeFileSync(f, original2);
  const xPos = original2.indexOf("x()");
  const yPos = original2.indexOf("y()");
  const e1: Edit = { ts: nowIso(), runId, iteration: 1, crate: "fixture", file: f, start: xPos, end: xPos + 3, before: "x()", after: "x_renamed()", kind: "call-add-await", diagnosticCode: null, diagnosticMessage: "selftest2" };
  const e2: Edit = { ts: nowIso(), runId, iteration: 1, crate: "fixture", file: f, start: yPos, end: yPos + 3, before: "y()", after: "yy()", kind: "call-add-await", diagnosticCode: null, diagnosticMessage: "selftest2" };
  let src3 = original2;
  for (const e of [e2, e1].sort((a, b) => b.start - a.start)) {
    const actual = src3.slice(e.start, e.end);
    if (actual !== e.before) throw new Error("selftest2 forward span mismatch for " + e.before);
    src3 = src3.slice(0, e.start) + e.after + src3.slice(e.end);
  }
  const expected2 = `fn a() { x_renamed() }\nfn b() { yy() }\n`;
  if (src3 !== expected2) {
    console.log("EXPECTED:", JSON.stringify(expected2));
    console.log("GOT:     ", JSON.stringify(src3));
    throw new Error("selftest2: multi-edit forward application produced wrong result");
  }
  console.log("selftest2: multi-edit forward (descending-start) OK");
  let src4 = src3;
  for (const e of [e1, e2].sort((a, b) => a.start - b.start)) {
    const actual = src4.slice(e.start, e.start + e.after.length);
    if (actual !== e.after) throw new Error("selftest2 revert span mismatch for " + e.after);
    src4 = src4.slice(0, e.start) + e.before + src4.slice(e.start + e.after.length);
  }
  if (src4 !== original2) {
    console.log("EXPECTED:", JSON.stringify(original2));
    console.log("GOT:     ", JSON.stringify(src4));
    throw new Error("selftest2: multi-edit revert (ascending-start) did not restore original");
  }
  console.log("selftest2: multi-edit revert (ascending-start) restored original byte-for-byte — OK");

  // byte-offset -> char-offset mapping test: emoji BEFORE the target position must shift the
  // byte offset ahead of the char offset by exactly (utf8 bytes - utf16 units) per character —
  // this is the exact bug class this repo's emoji-laden source files exposed in real usage.
  const f3 = join(dir, "emoji_fixture.rs");
  // 🦀 = 4 UTF-8 bytes, 2 UTF-16 code units (astral); 🔨️ = crab-hammer variant sequence similarly multi-byte.
  const emojiSrc = "// 🦀️ component\npub async fn helper() -> i32 {\n    1\n}\n";
  writeFileSync(f3, emojiSrc);
  const byteBuf = Buffer.from(emojiSrc, "utf8");
  const targetCharIdx = emojiSrc.indexOf("async fn helper");
  const targetByteIdx = Buffer.byteLength(emojiSrc.slice(0, targetCharIdx), "utf8");
  if (targetByteIdx === targetCharIdx) throw new Error("selftest3: fixture does not actually exercise a byte/char offset divergence — fixture is wrong");
  const map = buildByteToCharMap(emojiSrc);
  const recovered = map[targetByteIdx];
  if (recovered !== targetCharIdx) {
    throw new Error(`selftest3: byte->char map wrong: byte ${targetByteIdx} -> char ${recovered}, expected ${targetCharIdx}`);
  }
  console.log(`selftest3: byte->char map correct under emoji offset divergence (byte ${targetByteIdx} -> char ${targetCharIdx}) — OK`);

  // Regression test for a REAL corruption found during this packet's os-infinite run:
  // `let (Some(src_ep), Some(tgt_ep)) = (foo(), bar()) else { .. }` — rustc's diagnostic
  // labelled the PATTERN sub-span (not the actual Future-typed RHS) as "found future", and
  // forward-extraction happily treated `Some(src_ep)` as a callable, producing the syntax
  // error `let (Some(src_ep).await, ...) = .. else {`. Prove both halves of the fix hold:
  // extractCallForward still successfully parses `Some(x)` as a call shape (it IS one,
  // lexically) — the fix is NOT in extraction, it's the denylist gate that must catch it.
  const patternFixtureSrc = "let (Some(src_ep), Some(tgt_ep)) = pair() else { return false; };";
  const somePos = patternFixtureSrc.indexOf("Some(src_ep)");
  const patternClean = cleanRustSource(patternFixtureSrc);
  const patternCall = extractCallForward(patternClean, somePos);
  if (!patternCall || patternCall.name !== "Some") {
    throw new Error("selftest4: fixture assumption broken — extractCallForward no longer parses `Some(x)` as call-shaped, update this test");
  }
  if (!PATTERN_CONSTRUCTOR_DENYLIST.has(patternCall.name)) {
    throw new Error(`selftest4: PATTERN_CONSTRUCTOR_DENYLIST does not cover "${patternCall.name}" — the real corruption this test guards against would reproduce`);
  }
  for (const ctor of ["Some", "None", "Ok", "Err"]) {
    if (!PATTERN_CONSTRUCTOR_DENYLIST.has(ctor)) throw new Error(`selftest4: PATTERN_CONSTRUCTOR_DENYLIST missing "${ctor}"`);
  }
  console.log("selftest4: pattern-constructor denylist catches the real Some(x).await corruption shape — OK");

  // Regression test for a REAL near-miss found during this packet's semio-s-plugin-stdio run:
  // a rustc suggestion span pointed into the installed toolchain's OWN std library source
  // (`~/.rustup/toolchains/.../library/core/src/macros/mod.rs`), which a substring-only
  // exclusion list can never fully enumerate. isEditableFile() must positively require the
  // repo ROOT prefix, not just blacklist known-bad substrings.
  if (isEditableFile("/Users/someone/.rustup/toolchains/nightly-x/lib/rustlib/src/rust/library/core/src/macros/mod.rs")) {
    throw new Error("selftest5: isEditableFile() accepted a path outside the repo ROOT — the toolchain-source near-miss would reproduce");
  }
  if (isEditableFile(join(ROOT, "compose", "client", "lib", "rs", "src", "lib.rs"))) {
    throw new Error("selftest5: isEditableFile() accepted a path under compose/ — out-of-scope guard broken");
  }
  if (!isEditableFile(join(ROOT, "🧰️framework", "📦️packages", "🦀️rust", "src", "lib.rs"))) {
    throw new Error("selftest5: isEditableFile() rejected an ordinary in-repo path — guard is too strict");
  }
  console.log("selftest5: repo-containment guard rejects out-of-repo and compose/ paths, accepts ordinary repo paths — OK");

  // Regression test for a THIRD real corruption shape, found on semio-s-plugin-stdio: rustc's
  // own "consider awaiting" suggestion inserted `.await` right after a struct-literal SHORTHAND
  // field (`CsvSnapshot { schema: .., has_header, records }` -> `.., records.await }`), a
  // syntax error. isRiskyBareIdentifierAwaitInsertion must catch this shape (preceded by `,`,
  // bare identifier, followed by `}`) and must NOT false-positive on an ordinary
  // `field: expr.await` (preceded by `:`, not `,`/`{`).
  const shorthandFixtureSrc = "CsvSnapshot { schema: X.into(), has_header, records }";
  const shorthandClean = cleanRustSource(shorthandFixtureSrc);
  const shorthandInsertionPoint = shorthandFixtureSrc.lastIndexOf("records") + "records".length;
  if (!isRiskyBareIdentifierAwaitInsertion(shorthandClean, shorthandInsertionPoint)) {
    throw new Error("selftest6: isRiskyBareIdentifierAwaitInsertion did not catch the real struct-shorthand `records.await` corruption shape");
  }
  const normalFieldSrc = "CsvSnapshot { schema: X.into(), records: fetch_records() }";
  const normalFieldClean = cleanRustSource(normalFieldSrc);
  const normalFieldInsertionPoint = normalFieldSrc.lastIndexOf("fetch_records()") + "fetch_records()".length;
  if (isRiskyBareIdentifierAwaitInsertion(normalFieldClean, normalFieldInsertionPoint)) {
    throw new Error("selftest6: isRiskyBareIdentifierAwaitInsertion false-positived on an ordinary `field: call().await` position");
  }
  console.log("selftest6: struct-shorthand-field bare-identifier guard catches the real records.await corruption shape without false-positiving on normal field: call() positions — OK");

  // Regression test for the FOURTH real bug, and the most damaging one: for the "`T` is not an
  // iterator" diagnostic shape, rustc's primary span covers only the trailing failing method
  // (e.g. `into_iter`, ~9 bytes), not the Future-producing expression — a separate, non-primary,
  // unlabeled sibling span that byte-abuts it. Reproduces the exact shape confirmed on
  // `semio-framework-surface`: `visible_tile_coords(..)\n    .into_iter()` where rustc's
  // primary span is just ".into_iter" and a sibling span covers the full receiver expression
  // ending exactly where the primary begins.
  const stackFixtureSrc = "visible_tile_coords(&camera, a, b)\n    .into_iter()";
  const intoIterStart = stackFixtureSrc.indexOf(".into_iter") + 1; // skip the dot itself, per real span shape
  const intoIterEnd = intoIterStart + "into_iter".length;
  const receiverEnd = intoIterStart; // per the real diagnostic dump, the non-primary span's byte_end (16148) EQUALS the primary span's byte_start (16148) exactly — the dot belongs to the receiver span, not the trailing-method span
  const fakeDiag: RustcDiagnostic = {
    message: "`impl Future<Output = Vec<(u32, u32, u32)>>` is not an iterator",
    code: { code: "E0277" },
    level: "error",
    rendered: null,
    children: [],
    spans: [
      { file_name: "fixture.rs", byte_start: 0, byte_end: receiverEnd, is_primary: false, label: "", suggested_replacement: null, suggestion_applicability: null, text: [] },
      { file_name: "fixture.rs", byte_start: intoIterStart, byte_end: intoIterEnd, is_primary: true, label: "`impl Future<Output = Vec<(u32, u32, u32)>>` is not an iterator", suggested_replacement: null, suggestion_applicability: null, text: [] },
    ],
  };
  const chosenResult = findFutureExprSpan(fakeDiag);
  if (!chosenResult || chosenResult.span.is_primary) {
    throw new Error("selftest7: findFutureExprSpan still picked the primary (trailing-method) span instead of the abutting receiver-expression span — the .into_iter().await.await stacking bug would reproduce");
  }
  if (chosenResult.span.byte_start !== 0 || chosenResult.span.byte_end !== receiverEnd) {
    throw new Error(`selftest7: findFutureExprSpan picked an unexpected span [${chosenResult.span.byte_start},${chosenResult.span.byte_end}), expected [0,${receiverEnd})`);
  }
  if (!chosenResult.trusted) {
    throw new Error("selftest7: findFutureExprSpan picked the abutting sibling span but did not mark it trusted");
  }
  // And the stacking backstop itself: given a position that already has `.await` immediately
  // before it, wouldStackAwait must refuse regardless of how the position was computed.
  const stackedSrc = "foo().await";
  const stackedClean = cleanRustSource(stackedSrc);
  if (!wouldStackAwait(stackedClean, stackedSrc.length)) {
    throw new Error("selftest7: wouldStackAwait did not catch an insertion point immediately after an existing .await");
  }
  if (wouldStackAwait(cleanRustSource("foo()"), "foo()".length)) {
    throw new Error("selftest7: wouldStackAwait false-positived on a position with no existing .await nearby");
  }
  console.log("selftest7: findFutureExprSpan prefers the byte-adjacent receiver span over the primary trailing-method span, and wouldStackAwait refuses to stack — OK");

  // R14 selftest8: happy-path def-use resolution. `fut` is a bare, let-bound local used later
  // (`use_it(fut)`) rather than a fresh call — the dominant residue shape. Must resolve to the
  // BINDING's RHS call (`helper_fn()`), not the use site, and the resolved callee must be
  // independently confirmed non-suspending via the same resolveCallee() pipeline R13 already
  // uses for def-remove-async — proving the full decision rule connects end to end.
  const d8dir = mkdtempSync(join(tmpdir(), "r14-defuse-selftest8-"));
  const f8 = join(d8dir, "fixture8.rs");
  const src8 = "pub async fn helper_fn() -> i32 { 1 }\n\npub fn caller() -> i32 {\n    let fut = helper_fn();\n    use_it(fut)\n}\n";
  writeFileSync(f8, src8);
  const useOffset8 = src8.indexOf("use_it(fut)") + "use_it(".length;
  const res8 = resolveDefUse(f8, useOffset8, "fut");
  if (!res8.ok) throw new Error(`selftest8: expected def-use resolution to succeed, got refusal: ${res8.reason}`);
  if (res8.calleeName !== "helper_fn") throw new Error(`selftest8: expected calleeName "helper_fn", got "${res8.calleeName}"`);
  const expectedCallEnd8 = src8.indexOf("helper_fn();") + "helper_fn()".length;
  if (res8.callEnd !== expectedCallEnd8) throw new Error(`selftest8: expected callEnd ${expectedCallEnd8}, got ${res8.callEnd}`);
  const calleeInfo8 = resolveCallee(f8, "helper_fn");
  if (!calleeInfo8 || calleeInfo8.suspends !== false) throw new Error("selftest8: expected helper_fn to be classified non-suspending (no own .await) via the shared resolveCallee pipeline");
  invalidateFile(f8);
  rmSync(d8dir, { recursive: true, force: true });
  console.log("selftest8: def-use resolves a bare let-bound variable to its binding-site RHS call, decision rule confirmed non-suspending — OK");

  // R14 selftest9: shadowing. Two `let x = ..;` bindings of the same name in the same function
  // must refuse — it is ambiguous which one governs the use without proper lexical scoping,
  // which this tool deliberately does not implement (refusal, not a guess).
  const d9dir = mkdtempSync(join(tmpdir(), "r14-defuse-selftest9-"));
  const f9 = join(d9dir, "fixture9.rs");
  const src9 = "pub fn caller() -> i32 {\n    let x = helper_a();\n    let x = helper_b();\n    use_it(x)\n}\n";
  writeFileSync(f9, src9);
  const useOffset9 = src9.indexOf("use_it(x)") + "use_it(".length;
  const res9 = resolveDefUse(f9, useOffset9, "x");
  if (res9.ok) throw new Error("selftest9: expected shadowing refusal, got a resolution");
  if (!/shadow|bound 2 times/i.test(res9.reason ?? "")) throw new Error(`selftest9: refusal reason does not mention shadowing: ${res9.reason}`);
  invalidateFile(f9);
  rmSync(d9dir, { recursive: true, force: true });
  console.log("selftest9: shadowed same-name binding refused — OK");

  // R14 selftest10: reassignment between binding and use. `x` is reassigned after its `let mut`
  // binding, so the value at the use site may not be the one the diagnostic's Future-type
  // judgement was even made against — must refuse.
  const d10dir = mkdtempSync(join(tmpdir(), "r14-defuse-selftest10-"));
  const f10 = join(d10dir, "fixture10.rs");
  const src10 = "pub fn caller() -> i32 {\n    let mut x = helper_a();\n    x = helper_b();\n    use_it(x)\n}\n";
  writeFileSync(f10, src10);
  const useOffset10 = src10.indexOf("use_it(x)") + "use_it(".length;
  const res10 = resolveDefUse(f10, useOffset10, "x");
  if (res10.ok) throw new Error("selftest10: expected reassignment refusal, got a resolution");
  if (!/reassign/i.test(res10.reason ?? "")) throw new Error(`selftest10: refusal reason does not mention reassignment: ${res10.reason}`);
  invalidateFile(f10);
  rmSync(d10dir, { recursive: true, force: true });
  console.log("selftest10: reassignment between binding and use refused — OK");

  // R14 selftest11: function parameter. `x` is a parameter of `caller`, not a local let-binding
  // — must refuse before even attempting a let-binding scan.
  const d11dir = mkdtempSync(join(tmpdir(), "r14-defuse-selftest11-"));
  const f11 = join(d11dir, "fixture11.rs");
  const src11 = "pub fn caller(x: SomeFuture) -> i32 {\n    use_it(x)\n}\n";
  writeFileSync(f11, src11);
  const useOffset11 = src11.indexOf("use_it(x)") + "use_it(".length;
  const res11 = resolveDefUse(f11, useOffset11, "x");
  if (res11.ok) throw new Error("selftest11: expected function-parameter refusal, got a resolution");
  if (!/function parameter/i.test(res11.reason ?? "")) throw new Error(`selftest11: refusal reason does not mention function parameter: ${res11.reason}`);
  invalidateFile(f11);
  rmSync(d11dir, { recursive: true, force: true });
  console.log("selftest11: function-parameter binding refused — OK");

  // R14 selftest12: closure boundary. `x` is bound OUTSIDE a `move || { .. }` closure but used
  // INSIDE it — must refuse rather than assume the captured value at the use site matches
  // straightforward same-scope reasoning (closures can outlive/re-invoke, captured-by-value vs
  // by-ref changes semantics, and this tool deliberately does not model capture semantics).
  const d12dir = mkdtempSync(join(tmpdir(), "r14-defuse-selftest12-"));
  const f12 = join(d12dir, "fixture12.rs");
  const src12 = "pub fn caller() -> i32 {\n    let x = helper_a();\n    let f = move || {\n        use_it(x)\n    };\n    f()\n}\n";
  writeFileSync(f12, src12);
  const useOffset12 = src12.indexOf("use_it(x)", src12.indexOf("move ||")) + "use_it(".length;
  const res12 = resolveDefUse(f12, useOffset12, "x");
  if (res12.ok) throw new Error("selftest12: expected closure-boundary refusal, got a resolution");
  if (!/closure/i.test(res12.reason ?? "")) throw new Error(`selftest12: refusal reason does not mention closure: ${res12.reason}`);
  invalidateFile(f12);
  rmSync(d12dir, { recursive: true, force: true });
  console.log("selftest12: closure-boundary crossing (use inside a move closure, binding outside) refused — OK");

  // R14 selftest13: RHS shapes that are not a bare resolvable call — macro invocation, method
  // chain, and block/conditional expression — must each refuse with a specific reason, never
  // guess at a callee.
  const d13dir = mkdtempSync(join(tmpdir(), "r14-defuse-selftest13-"));
  const f13macro = join(d13dir, "fixture13macro.rs");
  writeFileSync(f13macro, "pub fn caller() -> i32 {\n    let x = some_macro!(1, 2);\n    use_it(x)\n}\n");
  const resMacro = resolveDefUse(f13macro, readFileSync(f13macro, "utf8").indexOf("use_it(x)") + "use_it(".length, "x");
  if (resMacro.ok || !/macro/i.test(resMacro.reason ?? "")) throw new Error(`selftest13: macro RHS not refused correctly: ${JSON.stringify(resMacro)}`);

  const f13chain = join(d13dir, "fixture13chain.rs");
  writeFileSync(f13chain, "pub fn caller() -> i32 {\n    let x = receiver.method_call();\n    use_it(x)\n}\n");
  const resChain = resolveDefUse(f13chain, readFileSync(f13chain, "utf8").indexOf("use_it(x)") + "use_it(".length, "x");
  if (resChain.ok || !/method chain/i.test(resChain.reason ?? "")) throw new Error(`selftest13: method-chain RHS not refused correctly: ${JSON.stringify(resChain)}`);

  const f13block = join(d13dir, "fixture13block.rs");
  writeFileSync(f13block, "pub fn caller() -> i32 {\n    let x = if cond { helper_a() } else { helper_b() };\n    use_it(x)\n}\n");
  const resBlock = resolveDefUse(f13block, readFileSync(f13block, "utf8").indexOf("use_it(x)") + "use_it(".length, "x");
  if (resBlock.ok || !/block\/conditional/i.test(resBlock.reason ?? "")) throw new Error(`selftest13: block/conditional RHS not refused correctly: ${JSON.stringify(resBlock)}`);

  invalidateFile(f13macro);
  invalidateFile(f13chain);
  invalidateFile(f13block);
  rmSync(d13dir, { recursive: true, force: true });
  console.log("selftest13: non-resolvable RHS shapes (macro, method chain, block/conditional) each refused with a specific reason — OK");

  // R14 selftest14: async-context guard. A real bug this packet's own dry-run caught — a
  // Future-typed local (`plane_normal = Vec3::new(..)`, misattributed by an overly-broad
  // diagnostic pattern) can sit inside an ORDINARY SYNC function, where `.await` is simply
  // illegal Rust regardless of how correctly the callee/position were extracted. Both a plain
  // sync fn and an `async {}`-block-nested position must be told apart correctly.
  const d14dir = mkdtempSync(join(tmpdir(), "r14-defuse-selftest14-"));
  const f14 = join(d14dir, "fixture14.rs");
  const src14 =
    "pub fn sync_caller() -> i32 {\n    let x = helper_a();\n    use_it(x)\n}\n\npub fn sync_with_async_block() -> i32 {\n    let y = async {\n        let z = helper_a();\n        use_it(z)\n    };\n    0\n}\n\npub async fn real_async_caller() -> i32 {\n    let w = helper_a();\n    use_it(w)\n}\n";
  writeFileSync(f14, src14);
  const { clean: clean14 } = loadFile(f14);

  const useOffsetSync = src14.indexOf("use_it(x)") + "use_it(".length;
  if (isInsideAsyncContext(f14, clean14, useOffsetSync)) throw new Error("selftest14: expected an ordinary sync fn to be refused (not inside async context)");

  const useOffsetAsyncBlock = src14.indexOf("use_it(z)") + "use_it(".length;
  if (!isInsideAsyncContext(f14, clean14, useOffsetAsyncBlock)) throw new Error("selftest14: expected a position inside a nested `async {}` block (within a sync fn) to be accepted");

  const useOffsetAsyncFn = src14.indexOf("use_it(w)") + "use_it(".length;
  if (!isInsideAsyncContext(f14, clean14, useOffsetAsyncFn)) throw new Error("selftest14: expected a position inside a genuinely `async fn` to be accepted");

  invalidateFile(f14);
  rmSync(d14dir, { recursive: true, force: true });
  console.log("selftest14: async-context guard distinguishes sync fn (refused) from async fn / nested async{} block (accepted) — OK");

  // R15 selftest15: FIFTH real bug (see the guard's comment at its call site in
  // planEditsForDiagnostic). Reproduces the exact real-world `semio-s-plugin-stdio` shape: an
  // "is not an iterator" diagnostic with rustc emitting ONLY the primary span over the trailing
  // dot-method name (`into_iter`, 9 bytes), no sibling span at all — the untrusted-fallback case
  // findFutureExprSpan() now flags via `trusted: false`. Before this packet's fix,
  // extractCallForward() successfully (but wrongly) parsed `into_iter()` itself as "the call" and
  // this would have queued a `call-add-await` edit placing `.await` after `.into_iter()`'s own
  // closing paren — syntactically clean, so no parse-error sweep would have caught it, and it does
  // not fix the underlying error. Run through the FULL `planEditsForDiagnostic` (not just
  // `findFutureExprSpan` in isolation, unlike selftest7) to prove the guard actually blocks the
  // edit end to end, the same standard R14's own selftest8 set for its happy path.
  const d15dir = mkdtempSync(join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-5-DE-ASYNC-REPAIR-SWEEP", "r15-selftest15-"));
  const f15 = join(d15dir, "fixture15.rs");
  const src15 = "pub fn convert(m: i32, b: i32) -> Vec<i32> {\n    inverse(m, b).into_iter().map(wrap).collect()\n}\n";
  writeFileSync(f15, src15);
  const intoIterStart15 = src15.indexOf(".into_iter") + 1; // span starts AFTER the dot, per the real diagnostic shape
  const intoIterEnd15 = intoIterStart15 + "into_iter".length;
  const fakeDiag15: RustcDiagnostic = {
    message: "`impl Future<Output = Vec<i32>>` is not an iterator",
    code: { code: "E0599" },
    level: "error",
    rendered: null,
    children: [],
    spans: [
      { file_name: f15, byte_start: intoIterStart15, byte_end: intoIterEnd15, is_primary: true, label: "`impl Future<Output = Vec<i32>>` is not an iterator", suggested_replacement: null, suggestion_applicability: null, text: [] },
    ],
  };
  const plan15: PlanResult = { edits: [], residue: [] };
  planEditsForDiagnostic(fakeDiag15, plan15);
  if (plan15.edits.length !== 0) {
    throw new Error(`selftest15: expected zero edits (untrusted dot-preceded fallback must be refused), got ${JSON.stringify(plan15.edits)}`);
  }
  if (plan15.residue.length !== 1 || !/untrusted primary-fallback/i.test(plan15.residue[0]!.reason)) {
    throw new Error(`selftest15: expected exactly one residue entry citing the untrusted-primary-fallback guard, got ${JSON.stringify(plan15.residue)}`);
  }
  invalidateFile(f15);
  rmSync(d15dir, { recursive: true, force: true });
  console.log("selftest15: untrusted-primary-fallback dot-method guard refuses the real `.into_iter()`-as-callee misattribution end to end — OK");

  // R15 selftest16: SIXTH real bug — a second manifestation of the FIFTH failure mode
  // (misattribution to a fully synchronous expression) that isInsideAsyncContext does NOT catch,
  // because the enclosing fn genuinely IS async this time. Reproduces the exact real-world
  // `semio-s-plugin-stdio` shape: an array literal `[async_call_a(), plain_local, async_call_b()]`
  // where rustc's element-type unification blames the innocent, ordinary `plain_local` with the
  // self-describing label "expected future, found `Vec<u8>`" — the span is the CONCRETE side, not
  // the Future-producing expression. Before this packet's fix, def-use would have resolved
  // `plain_local`'s own (entirely synchronous) binding and — since its callee doesn't resolve as
  // async — inserted `.await` after it, producing invalid code that awaits a non-Future value.
  const d16dir = mkdtempSync(join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-5-DE-ASYNC-REPAIR-SWEEP", "r15-selftest16-"));
  const f16 = join(d16dir, "fixture16.rs");
  const src16 = "pub async fn build_moov() -> Vec<u8> {\n    let mut plain_local = Vec::new();\n    plain_local.extend(vec![1u8]);\n    write_box(async_call_a(), plain_local, async_call_b())\n}\n";
  writeFileSync(f16, src16);
  const plainLocalUseStart16 = src16.indexOf("write_box(") + "write_box(async_call_a(), ".length;
  const plainLocalUseEnd16 = plainLocalUseStart16 + "plain_local".length;
  const fakeDiag16: RustcDiagnostic = {
    message: "mismatched types",
    code: { code: "E0308" },
    level: "error",
    rendered: null,
    children: [
      { children: [], code: null, level: "note", message: "expected opaque type `impl Future<Output = Vec<u8>>`\n                found struct `Vec<u8>`", rendered: null, spans: [] },
    ],
    spans: [
      {
        file_name: f16,
        byte_start: plainLocalUseStart16,
        byte_end: plainLocalUseEnd16,
        is_primary: true,
        label: "expected future, found `Vec<u8>`",
        suggested_replacement: null,
        suggestion_applicability: null,
        text: [],
      },
    ],
  };
  const plan16: PlanResult = { edits: [], residue: [] };
  planEditsForDiagnostic(fakeDiag16, plan16);
  if (plan16.edits.length !== 0) {
    throw new Error(`selftest16: expected zero edits (the concrete side of an element-type-unification mismatch must be refused, not awaited), got ${JSON.stringify(plan16.edits)}`);
  }
  if (plan16.residue.length !== 1 || !/CONCRETE \("found"\) side/.test(plan16.residue[0]!.reason)) {
    throw new Error(`selftest16: expected exactly one residue entry citing the concrete-side guard, got ${JSON.stringify(plan16.residue)}`);
  }
  invalidateFile(f16);
  rmSync(d16dir, { recursive: true, force: true });
  console.log('selftest16: "expected future, found <concrete type>" guard refuses the real array-literal-unification misattribution (traks/build_moov shape) end to end — OK');

  // R15 selftest17: EIGHTH real bug — the mirror image of the sixth, and the one that actually
  // tripped the monotonic guard on real `semio-s-plugin-stdio` (`graph::operators-internals`,
  // run `r13-o9bdzimk`, iteration 3: "13091 >= previous 13091"). Reproduces the exact shape:
  // `let myc = mycielskian(&g);` (`mycielskian` a PLAIN, never-async fn — confirmed by grep
  // against the real source) followed later by `for n in myc.nodes() { .. }` where `.nodes()`
  // (not `myc` itself) is the genuinely async, un-awaited call. Before this packet's fix, the
  // def-use fallback matched only the leading identifier "myc" of the `myc.nodes()` Future-expr
  // span, traced it to its own (already-correct, non-async) binding, and inserted `.await`
  // there — `mycielskian(&g).await` — which does not fix the error and, worse, awaits a value
  // that plainly is not a Future at all (confirmed only by a LATER compiler run once the bad
  // edit was on disk, exactly why this needed a proper guard rather than a plausible-looking
  // heuristic).
  const d17dir = mkdtempSync(join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-5-DE-ASYNC-REPAIR-SWEEP", "r15-selftest17-"));
  const f17 = join(d17dir, "fixture17.rs");
  const src17 = "pub fn mycielskian(g: &Storage) -> Storage {\n    g.clone()\n}\n\npub fn use_it() {\n    let myc = mycielskian(&some_graph());\n    for n in myc.nodes() {\n        touch(n);\n    }\n}\n";
  writeFileSync(f17, src17);
  const mycNodesStart17 = src17.indexOf("myc.nodes()");
  const mycNodesEnd17 = mycNodesStart17 + "myc.nodes()".length;
  const fakeDiag17: RustcDiagnostic = {
    message: "`impl Future<Output = impl Iterator<Item = u64>>` is not an iterator",
    code: { code: "E0277" },
    level: "error",
    rendered: null,
    children: [],
    spans: [
      {
        file_name: f17,
        byte_start: mycNodesStart17,
        byte_end: mycNodesEnd17,
        is_primary: true,
        label: "`impl Future<Output = impl Iterator<Item = u64>>` is not an iterator",
        suggested_replacement: null,
        suggestion_applicability: null,
        text: [],
      },
    ],
  };
  const plan17: PlanResult = { edits: [], residue: [] };
  planEditsForDiagnostic(fakeDiag17, plan17);
  if (plan17.edits.length !== 0) {
    throw new Error(`selftest17: expected zero edits (the method-chain-receiver identifier must be refused, never traced to its own binding), got ${JSON.stringify(plan17.edits)}`);
  }
  if (plan17.residue.length !== 1 || !/RECEIVER\.method\(\.\.\) chain/.test(plan17.residue[0]!.reason)) {
    throw new Error(`selftest17: expected exactly one residue entry citing the method-chain-receiver guard, got ${JSON.stringify(plan17.residue)}`);
  }
  invalidateFile(f17);
  rmSync(d17dir, { recursive: true, force: true });
  console.log("selftest17: method-chain-receiver guard refuses the real mycielskian/myc.nodes() misattribution (the one that tripped the monotonic guard) end to end — OK");

  rmSync(dir, { recursive: true, force: true });
  console.log("ALL SELFTESTS PASSED");
}

//#endregion

//#region Repair: strip accumulated stacked-.await corruption (see findFutureExprSpan fix)

// ONE-TIME repair for damage accumulated BEFORE the findFutureExprSpan abutting-span fix and
// the wouldStackAwait backstop existed: repeated runs, each re-diagnosing the SAME never-
// actually-fixed error, piled up to 6 `.await`s at a single wrong position. Since the correct
// insertion point can be arbitrarily far back through a method chain (not reliably "one call
// back" — confirmed by `actual.iter().zip(expected)` where the real receiver is `actual`, two
// calls back), this does NOT try to guess the right position. It strips the stack back to the
// original (still-broken, but never mis-edited) source, so a fresh `run` with the FIXED span
// selection can re-diagnose and repair each one correctly and safely, exactly like any other
// crate this tool has processed. Journalled (kind "strip-stacked-await") and revertible like
// every other edit in this file — `revert --run=<id>` undoes it the same way.
function stripStackedAwaits(dryRun: boolean): void {
  const runId = randomRunId();
  console.log(`[strip ${runId}] scanning repo for stacked .await corruption (dryRun=${dryRun})`);
  const found: string[] = [];
  function walk(dir: string): void {
    let entries: string[];
    try {
      entries = readdirSync(dir);
    } catch {
      return;
    }
    for (const entry of entries) {
      if (GLOBAL_INDEX_MANDATORY_EXCLUDE.has(entry) || GLOBAL_INDEX_EXTRA_EXCLUDE.has(entry)) continue;
      const full = join(dir, entry);
      let st;
      try {
        st = statSync(full);
      } catch {
        continue;
      }
      if (st.isDirectory()) walk(full);
      else if (st.isFile() && entry.endsWith(".rs")) found.push(full);
    }
  }
  walk(ROOT);

  let totalStripped = 0;
  for (const absPath of found) {
    if (!isEditableFile(absPath)) continue;
    const src = readFileSync(absPath, "utf8");
    const clean = cleanRustSource(src);
    const re = /(\.await){2,}/g;
    const edits: PlannedEdit[] = [];
    let m: RegExpExecArray | null;
    while ((m = re.exec(clean))) {
      edits.push({ file: absPath, start: m.index, end: m.index + m[0].length, before: src.slice(m.index, m.index + m[0].length), after: "", kind: "call-remove-await", diagnosticCode: null, diagnosticMessage: "strip-stacked-await repair: remove ALL N>=2 stacked .await (even one remaining is at the wrong position), to be re-diagnosed and correctly re-fixed fresh" });
    }
    if (edits.length === 0) continue;
    console.log(`[strip ${runId}] ${absPath}: ${edits.length} stacked-await site(s)`);
    totalStripped += edits.length;
    if (dryRun) continue;
    edits.sort((a, b) => b.start - a.start);
    let cur = src;
    for (const e of edits) {
      const actual = cur.slice(e.start, e.end);
      if (actual !== e.before) throw new Error(`strip: span mismatch in ${absPath} — file changed underneath us`);
      cur = cur.slice(0, e.start) + e.after + cur.slice(e.end);
    }
    writeFileSync(absPath, cur);
    invalidateFile(absPath);
    for (const e of edits) {
      appendJournal({ ts: nowIso(), runId, iteration: 1, crate: "<repo-wide-repair>", file: e.file, start: e.start, end: e.end, before: e.before, after: e.after, kind: e.kind, diagnosticCode: e.diagnosticCode, diagnosticMessage: e.diagnosticMessage });
    }
  }
  console.log(`[strip ${runId}] ${totalStripped} stacked-await site(s) ${dryRun ? "found (dry-run, none written)" : "fully stripped back to zero .await, ready for fresh re-diagnosis"}`);
}

//#endregion

//#region CLI

function parseArgs(argv: string[]): { cmd: string; opts: Record<string, string | boolean> } {
  const cmd = argv[0] ?? "";
  const opts: Record<string, string | boolean> = {};
  for (const a of argv.slice(1)) {
    if (a.startsWith("--")) {
      const [k, v] = a.slice(2).split("=");
      opts[k] = v === undefined ? true : v;
    }
  }
  return { cmd, opts };
}

async function main(): Promise<void> {
  const { cmd, opts } = parseArgs(process.argv.slice(2));
  if (cmd === "selftest") {
    selftest();
    return;
  }
  if (cmd === "run") {
    const crate = String(opts.crate ?? "");
    if (!crate) throw new Error("--crate=<package-name> required");
    await runCrate(crate, {
      dryRun: !!opts["dry-run"],
      maxIterations: opts["max-iterations"] ? Number(opts["max-iterations"]) : 20,
      target: opts.target ? String(opts.target) : undefined,
      verbose: !!opts.verbose,
    });
    return;
  }
  if (cmd === "strip-stacked-awaits") {
    stripStackedAwaits(!!opts["dry-run"]);
    return;
  }
  if (cmd === "revert") {
    const runId = String(opts.run ?? "");
    if (!runId) throw new Error("--run=<runId> required");
    revertRun(runId);
    return;
  }
  console.log("usage: bun 🔧️r13-deasync-codemod.ts run --crate=<name> [--dry-run] [--max-iterations=N] [--target=<triple>]");
  console.log("       bun 🔧️r13-deasync-codemod.ts revert --run=<runId>");
  console.log("       bun 🔧️r13-deasync-codemod.ts selftest");
}

if (import.meta.main) {
  main().catch((e) => {
    console.error(e);
    process.exit(1);
  });
}

//#endregion
