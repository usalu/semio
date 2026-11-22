# Builtin-Read Import Coverage Refusal (2026-11-22, ~15:40)

## Symptom
Every `repo cargo` call failed before compiling with `Closed compiler builtin-read import coverage refused: the rewrite did not count every built-in import` (`builtin-read-coverage-refused`, raised by `redirectCapturedBuiltinReadsV1` in `🏗️build/📥️source/🔏️closure/🍞️bun/🟦️.ts`).

## Cause
A peer's temporary probe in `⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🟦️.ts` (`recheck`, line 70): `require('node:fs').appendFileSync('/private/tmp/claude-501/dbg.txt','[DEBUG] dirchange …')`. The capture rewrites static imports and dynamic `import()` of `node:fs`; in ESM mode a `require("node:fs")` call is seen by the character scanner but not by the parser's import count (or the reverse), so the counts differ and the capture refuses the whole closure.

## Fix
Removed only that probe statement (the guarded `if(...){require(...).appendFileSync(...);}`); the peer's other edits in the file are untouched.

## Oracle
[builtin-read-coverage-scan.ts](./builtin-read-coverage-scan.ts) runs the real `redirectCapturedBuiltinReadsV1` over library sources changed in the last N hours (arg, default 6) and prints each refusal. After the fix: 253 files checked, no production refusal (two test files are refused, but tests are not part of a compiler capture).
Run: `<repo bun> builtin-read-coverage-scan.ts 6` from the repo root.
