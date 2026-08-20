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
  kind: "call-add-await" | "call-remove-await" | "def-remove-async" | "test-remove-async";
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
function findFutureExprSpan(d: RustcDiagnostic): RustcSpan | null {
  const allSpans: RustcSpan[] = [...d.spans, ...d.children.flatMap((c) => c.spans)];
  for (const s of allSpans) {
    if (s.label && /this expression has type `?impl Future/i.test(s.label)) return s;
  }
  for (const s of allSpans) {
    if (s.label && /found (`?impl )?[Ff]uture/i.test(s.label)) return s;
  }
  return d.spans.find((s) => s.is_primary) ?? d.spans[0] ?? null;
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
        const calleeName = extractCalleeNameBackward(clean, start + before.length);
        if (calleeName && PATTERN_CONSTRUCTOR_DENYLIST.has(calleeName)) {
          plan.residue.push({ file: absPath, message: d.message, reason: `refused: extracted callee "${calleeName}" is a pattern-position enum constructor, not a real call — likely a tuple/pattern mismatch diagnostic, needs human triage` });
          continue;
        }
        const handled = calleeName ? planDeasyncDefinition(absPath, calleeName, plan, d, "suggestion-driven") : false;
        if (!handled) {
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
    const futureSpan = findFutureExprSpan(d);
    if (!futureSpan) {
      plan.residue.push({ file: "?", message: d.message, reason: "no-suggestion fallback pattern matched but diagnostic has no span" });
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
    if (!call) {
      plan.residue.push({ file: absPath, message: d.message, reason: `no-suggestion fallback: could not forward-parse a call expression at Future-expr span start byte=${futureSpan.byte_start} char=${primaryStartChar} (likely a bare variable holding an un-awaited Future assigned elsewhere — needs def-use triage)` });
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

async function runCrate(crate: string, opts: { dryRun: boolean; maxIterations: number; target?: string }): Promise<void> {
  const runId = randomRunId();
  console.log(`[run ${runId}] crate=${crate} dryRun=${opts.dryRun} target=${opts.target ?? "(host)"}`);

  let prevErrorCount = Infinity;
  for (let iteration = 1; iteration <= opts.maxIterations; iteration++) {
    fileSrcCache.clear();
    fileCleanCache.clear();
    quoteRangeCache.clear();
    byteToCharCache.clear();
    invalidateGlobalAsyncFnIndex();

    const diags = runCargoCheckJson(crate, { tests: true, target: opts.target });
    const totalErrors = countErrors(diags);
    const asyncDiags = diags.filter(isAsyncClassDiagnostic);

    console.log(`[run ${runId}] iteration ${iteration}: total errors=${totalErrors}, async-class=${asyncDiags.length}`);

    if (totalErrors >= prevErrorCount && iteration > 1) {
      console.log(`[run ${runId}] MONOTONIC GUARD TRIPPED: ${totalErrors} >= previous ${prevErrorCount}. Reverting iteration ${iteration - 1} and stopping.`);
      revertRun(runId, iteration - 1);
      return;
    }
    prevErrorCount = totalErrors;

    if (asyncDiags.length === 0) {
      console.log(`[run ${runId}] no async-class diagnostics remain (total errors=${totalErrors}, presumably other bug classes or zero). Stopping.`);
      return;
    }

    const plan: PlanResult = { edits: [], residue: [] };
    for (const d of asyncDiags) planEditsForDiagnostic(d, plan);

    if (plan.edits.length === 0) {
      console.log(`[run ${runId}] ${asyncDiags.length} async-class diagnostics but zero automatable edits planned. Residue:`);
      for (const r of plan.residue.slice(0, 20)) console.log(`  - ${r.file}: ${r.reason} :: ${r.message.slice(0, 120)}`);
      console.log(`  (${plan.residue.length} total residue entries this iteration)`);
      return;
    }

    console.log(`[run ${runId}] iteration ${iteration}: planned ${plan.edits.length} edits (${plan.residue.length} residue)`);

    if (opts.dryRun) {
      const byKind: Record<string, number> = {};
      for (const e of plan.edits) byKind[e.kind] = (byKind[e.kind] ?? 0) + 1;
      console.log(`[run ${runId}] DRY RUN — edits by kind:`, byKind);
      for (const e of plan.edits.slice(0, 10)) {
        console.log(`  ${e.kind} ${e.file}:${e.start}-${e.end} ${JSON.stringify(e.before)} -> ${JSON.stringify(e.after)}`);
      }
      if (plan.residue.length > 0) {
        console.log(`[run ${runId}] DRY RUN — residue (${plan.residue.length}):`);
        for (const r of plan.residue.slice(0, 20)) console.log(`  - ${r.file}: ${r.reason} :: ${r.message.slice(0, 120)}`);
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

  rmSync(dir, { recursive: true, force: true });
  console.log("ALL SELFTESTS PASSED");
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
    });
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
