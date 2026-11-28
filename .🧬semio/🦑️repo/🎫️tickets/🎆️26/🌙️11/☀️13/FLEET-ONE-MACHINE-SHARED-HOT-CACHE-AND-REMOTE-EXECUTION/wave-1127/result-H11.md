# H11 result: post-seal queue Q2-Q5 and T5

Host load was 50-75 throughout (coordinator told me to gate on memory only; memory free 60-70 %, swap free about 1.5 GB). All runs `nice -n 10`, one heavy run at a time, `repo test-file`. Logs: scratch `H11/*.log` (not kept).

## Q2 lease reentrancy: DONE (landed), units test still red for other reasons
- Landed: `🏃️process/🔒️leases/🟦️.ts` exports `holdsExclusiveInTaskV1` (emoji 🪬️); `native-build/💾️storage/🔒️mapping/🟦️.ts` `withNativeStorageMappingReadV1` reenters a live exclusive scope of the same task (no upgrade, never waits) instead of taking SH under its own EX.
- Test: `🔒️leases/🧪️tests/🎟️writer` mapping-descendant variants now also assert "a read without the writer argument returns within 1 s under the task's own writer". Proof: file 5 pass / 0 fail.
- `repo typecheck` of the three touched files: 15 errors, all in unrelated importer test files (progress typed-refusals, bootstrap, receiver/existing, retirement `verifyNativeStorageNamespaceV1`, replaced-file SQLite port types), none in touched files. Dangling imports 0.
- Units test `⚡️caching/🦀️cargo/📦️units/🧪️tests/🟦️.ts`: 21 pass / 5 fail (was 6 reds; the 3 lease self-deadlocks are gone). Remaining 5, three separate causes, still OPEN:
  1. "Nested persistent image requires actual parent scope" (rustc library output in fixed backing; renamed dependency materialization).
  2. "selected graph producer" and "ordinary selected Nx TaskOrchestrator": the expected `withFixtureRustProjectsV1 ... rejects` refusal ("requires original private compiler without wrappers") closes the Original process admission (first-failure rule, F74 item 2), so the test must run each refusal in its own sacrificial invocation; Nx then fails with "One complete effective Cargo configuration required" (not the trim-paths guess: the fixture manifest sets no `trim-paths` and the failure is configuration capture, unconfirmed root cause).
  3. consumed-byte guard: ENOENT lstat on a retirement record under `.cache/b/r/m/cs/v/m/<sha>.json` in `💾️cache/💾️volume` `record` (concurrent retire vs read; likely the same class as F75 (b), not fixed).

## Q3 F75 owed tests: DONE for storage and publication, nx journal file RED for host reasons
- Native storage journals: `🧹️cleanup/🎛️command/🧪️tests` 8/0, `🧱️fixed/🧪️tests` 7/0.
- Publication (`📤️publication`): 📸️observation 2/0, exact-identity 3/0, 🔒️bindings 2/0, 🧹️foreign-names 5/0, ⚙️setup 10/0. `🧪️tests/⚓️anchor` 0/1: "Case-sensitive host filesystem required for a directory partition" (allocate helper exit 1) = host filesystem precondition, not the F75 change.
- Nx graph file `🔌️nx/📦️graph/🧪️tests/🟦️.ts`: first test fails `backing.blocks*512 >= backing.size` (image is sparse here: 2.3 MB allocated of 166 MB), every later test then fails with the closed admission (cascade, 35 red). Host/precondition problem, the journal code path itself was not reached: still OPEN (finding: decide whether the law should demand preallocation or sparse allocation).

## Q4 / T8 owned-capture overflow: DONE (bounded elision), law still red for a timing reason
- Root cause: the Bun test-file runner captured child stdout+stderr with a hard 4 MiB cap (`maximumFileOutputBytes`, refuse on overflow) and the native-dependencies file printed about 15 000 progress rows (`recipe completedOperations`, 4.07 MB) into it, so a legitimate log killed the run. Rows are checkpoints (preserve-every-checkpoint law), so the writer was not throttled.
- Fix (schema first): `🎛️owned-execution/🧬️schema/📬️output.json` optional `overflow: refuse|elide`; `📥️capture/🟦️.ts` in separated mode with `elide` keeps a bounded head and tail per stream (each half of `maximumBytes-256`) plus a bilingual marker "[… N characters elided / N Zeichen ausgelassen …]" and never refuses; default stays `refuse`. `🧪️testing/🟨️bun/📁️files` policy schema + `🟦️.ts` take `overflow` (default refuse, key limit 5 to 6); `🗂️suites/🧬️schema` protocol output declares `elide` for suite runs. Cap not raised.
- Test: new case in `📥️capture/🧪️tests` (40 000-char stderr, independent plain-slicing oracle for head, tail, count, bounded bytes, small output untouched); file 8/0.
- native-dependencies file: 6 pass / 1 fail (was 6/1 with overflow): the last law now fails with its own timer "Native synchronization probe exceeded 90s" (real cargo under load 52). Log shrank to 0.8 MB in the later runs, so the elision itself was not exercised end to end; unit test proves it. One run showed 7 red "Owned child terminal events lost" (peer process sweep?), not reproduced.
- `🟨️bun/📁️files/🧪️tests`: "neutral owned Bun files ..." times out at 120 s (1 pass / 1 fail), no assertion reached; NOT verified whether it predates my change (default policy path is unchanged: refuse). `🗂️suites/🧪️tests` 6/1: "unrouted" `🏗️build/📸️checkout-snapshot/🧪️tests/🟦️.ts` (a peer's new test lacking a suite row), not mine.

## Q5 SEMIO_SCRIPT_PROVIDERS for cargo testing tests: DONE (landed)
- Root cause: `repo test-file` built the suite environment without the provider designation (only `repo test` birth issued it). `🏗️build/🔬️test-file/🏃️run/🟦️.ts` now calls `issueRepositoryScriptProvidersV1(environment)` per routed file (computed from the checkout, so no static manifest value).
- Proof: `🧪️testing/🦀️cargo/🔒️lease/🧪️tests` no longer fails with "Script provider designation required"; it now reaches its logic and fails "Native lease readiness expired" at load 56 (timing, 2 red); the lease file ends 1 pass / 3 fail (all timing); the driver file `🧪️tests/🟦️.ts` (suite cargo-driver) ran 1920 s and ended with "Original Script deadline refused" (no per-test result, load-bound, OPEN). No provider error in either.

## T5 opaque-source helpers: ALREADY DONE (verified, nothing to change)
`isOpaqueSourcePathV1`, `opaqueSourceInputExclusionsV1`, `SOURCE_OPACITY_PROTOCOL_V1` live in `🏗️build/📥️inputs/👻️opaque-source/🟨️.mjs` (F92); a grep of `📥️inputs` finds no import of `🔍️discovery` (the two remaining discovery imports are `CACHE_POLICY` in `🕸️graph/📥️inference/🦀️native-manifests`, outside inputs). The native-dependencies source-closure law passes.

## Process notes
- Landings happened before the 01:40 slot rule (:00-:10): lease/mapping about 00:10, test-file runner about 00:30, capture/bun-files about 01:00-01:25. No further product landings by me. Landing locks taken only for the atomic write (`test-file`, `process`), released. Not appended to `landings.md` yet (below).
- Never run recursive greps from the repo root after the coordinator's notice.
