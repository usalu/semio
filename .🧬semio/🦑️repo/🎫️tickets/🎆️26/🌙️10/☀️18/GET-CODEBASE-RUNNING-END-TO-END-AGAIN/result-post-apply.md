# Result: post-apply (executor)

Write pause entered after the last edit below; no repo writes outside C/ since.

## Root causes and fixes
1. `lease-fair/lease fairness law` and 2. `cargo-pressure/admission read suite`: not code bugs. Both suites are envelope-gated and throw "Explicit ticket generated artifact directory required" / "Original diagnostic fixture directory required" when `SEMIO_TEST_ARTIFACT_DIR` is unset; the staged-merge verify runner spawned commands without it. Fix: `staged-merge-tools/plan.ts` verify step now passes `SEMIO_TEST_ARTIFACT_DIR` (inherited if set, else a `art` dir inside the per-verification temp dir that is removed afterwards).
3. `env-pass2/patch set idempotent`: `env-pass2-tools/apply.py` `patched()` only recognised an already-applied span edit at its original offset. After the import edit (start 580) changed the file length, later spans moved, the exact test failed and the fallback count anchor `receiveScriptProcessInvocation(process.env,` saw 2 of 4 (two other hits are inside child-script template strings that must stay). Fix: new `shifted_applied()` accepts spans at their cumulative length-shifted offsets (only when the unapplied check fails and every edit is span-keyed).
4. tsc (new, from cargo-pressure): `CargoSourceAdmissionPortV1` now requires `recheck`; the two port literals in `library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🧪️tests/🕸️units/🟦️.ts` lacked it. Added `recheck:async()=>undefined`.

## Files
- C/staged-merge-tools/plan.ts
- C/env-pass2-tools/apply.py
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🧪️tests/🕸️units/🟦️.ts

## Verification
- `SEMIO_TEST_ARTIFACT_DIR=<ticket gen> bun test ./🧰️framework/🔨️modules/🏃️process/🔒️leases/⚖️fairness/🧪️tests/🟦️.ts`: 11 pass, 0 fail.
- same env, `bun test ./…/🏗️native-build/📥️source/🎟️admission/📖️read/🧪️tests/🟦️.ts`: 2 pass, 0 fail.
- `sh env-pass2-tools/apply.sh check`: ok 41 files; `apply --dry-run`: 0 of 41 files would change (no-op).
- `bun test ./C/staged-merge-tools/plan.test.ts`: 5 pass; `bun build --target=bun plan.ts`: ok.
- tsc `node node_modules/typescript/bin/tsc -p 🧰️framework/🛍️products/🦑️repo/tsconfig.json --noEmit --pretty false`: 13 errors before the units fix; after it the `units` errors are gone (grep count 0). Remaining list re-count not captured after the fix.

## Pending (tsc errors, library files, not fixed because of the pause)
- `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/👓️reading/🟦️.ts(68)`: TS2345 test stub lacks `observed` in the scope type.
- `🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️sources/🟦️.ts(122,129,142)`: TS2769 literal widening (`missing` string vs union, `version: number` vs `1`, `string[]|undefined`); add `as const` / non-null.
- `🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️workspace/🟦️.ts(40)`: TS2352 cast `Record<string,string>`; cast via `unknown`.
- `🏗️build/🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts(20,24,69)`: TS2769 `"compile"|"freshness"` literal, TS7053 index on `{browser,codec}`.
- `🖥️server/🎛️coordinator/🏗️builder/🎟️admission/🧪️tests/🟦️.ts(8)`: TS2459 imports `ScriptInvocation`, which the process invocation module declares but does not export.
- Not re-run: the other sets.json verifications (unchanged by these fixes), heavy tsc gate until the pending list is fixed.
