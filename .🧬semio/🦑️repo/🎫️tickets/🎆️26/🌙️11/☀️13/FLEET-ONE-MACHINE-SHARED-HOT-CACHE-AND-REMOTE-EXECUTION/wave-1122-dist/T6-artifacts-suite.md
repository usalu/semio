# 🧪 T6: native-build driver artifacts suite (2026-11-24 20:11-22:15, lane T6)

Lane T6 follows T4. All runs used `repo test-file`, one at a time, at host load 46-85 (the budget was clamped to 1800 s on every run). No sub-agents were used and nothing was restarted. Nothing was written inside `/Users/ueli/.sinc/play`. I took the lock `lock.tests-native-build` and released it. The landing is recorded in `T/landings.md` under "T6". The scripts are in `wave-1122-dist/T6-scripts/`. My logs (`🗑️generated/T6`) are deleted.

Abbreviations: P = `🧰️framework/🔨️modules/🏃️process/`, CP = `P📦️artifacts/🏗️native-build/🧩️component-pass/`.

## 1. Real wasm32 build: a generated `component.wasm` is refused as an edit during the build

**Root cause.** The suspected causes do not apply: the output is not in a source directory, the watch root is not the problem, and this is not `fileKinds` drift. The stage file `<generation>/w/<key>/component.wasm` is already a declared output: the provenance records it as `stagedPath` and hashes it.

The fault is in the order of work:
- The driver keyed completions by package name only: `${name}:stage`, `${name}:<stage>`, `failComponent(name)` (`CP🟦️.ts` 572-590, 810-812, 1143, 1156).
- The real test plans the same packages in two passes (release and prototype). Both run in parallel lanes.
- So the first pass to stage `comp-a` resolved the `comp-a:stage` completion of the other pass too.
- The other pass's provenance then observed its own stage path before that path was written. The path went into the owner's `missing` set (`P../📁️filesystem/🧾️observation/📁️current/🟦️.ts:70`).
- When the stage landed, `recheckStructure` (`:99`) refused it with "Current physical absent input appeared before transfer".
- The plan contract admits the same package in several passes (`📋️contract/🟦️.ts:50` only refuses duplicates within one pass), so the driver has to support it.

**Fix (`CP🟦️.ts`).**
- Every component completion is now keyed by its planned pass: `<pass>/<package>:<stage>`. Batches map to their parent pass. The key lives in `componentKey` inside `executePasses`, beside `batches`.
- The public names keep their meaning: `<package>:<stage>` and `components:<stage>` now wait for every instance of that package.

**Regression test.** I added the stub test "a component planned in two passes stages, runs its stages and is observed per pass…" (`CP🧪️tests/🌊️streaming/🟦️.ts`, fixture `repeatedComponents`):
- Two profiles plan the same two packages.
- Every stage write after the first is delayed by 1.5 s.
- A declared `check` stage asserts that its own stage file exists.
- It passes on the final code. With the old shared key, `check` would run on the other pass's completion. That last point is reasoned from the code: I did not run the old code to show it red.

**Play release build.** The log `/Users/ueli/Documents/semio/.🧬semio/🌐hub/play-cdn-1122/pinned/site-build.log` (3021 lines, read at 20:13) contains none of these: "appeared before transfer", "Current physical", "ConcurrentEdit". It shows 0/73 components staged at 96 min. Play builds one mode (one profile), with 27 passes and 73 component builds, so a package name is unlikely to repeat across its passes. I did not check this against the play plan itself. Play can only hit this refusal if two of its passes name the same package.

**The real wasm32 test itself is still unverified.** Both runs (load 55-85) ran into the test's own 600 s body deadline ("Original resource loan deadline exhausted" at `arm`). The isolated child does not forward its stdout, so my temporary `[DEBUG]` lines (now removed, the file is byte-identical to before) showed nothing. Where it stalls is not proven. Candidates are the nested `schedulerBinaries()` `repo cargo` build waiting behind the shared cargo leases, or the cold wasm32 compiles at load ~70.

## 2. Splitting the native-artifacts suite

`CP🧪️tests/🟦️.ts` was about 1400 lines with about 70 tests (counting fixture rows). It is now split by topic, with one suite per file, each with a 300 s budget.

- **Shared helper.** The stubs (`artifacts`, `header`, `stubCargo`, `stubWorkspace`, `runStub`, `stubJob`) moved to the helper `🧪️tests/🎭️stub/🟦️.ts`. It imports only `expect`, so the route table does not discover it as a test file.
- **Shared fixtures.** Fixtures are built once per file. The `schedulerBinaries` cache is per process and only the `🦀️real` file uses it.
- **No assertion changed.** Only imports and paths changed (one extra `../`), applied by `T6-scripts/split-component-pass-tests.py`, which checks the starting file's sha256 first.
- **Route table and project.** `P🧪️testing/🗂️suites/🔣️.json` and `P📋️project.json` gained 7 new suites, each with its `test-<id>` target. `component-reclaim` and `native-queued-cancellation` now point at their own files, and their name filters are gone.
- **Route-table law check.** The suites table law test shows no duplicated, filtered or missing entries from these changes. Its 3 failures are other lanes' unrouted or aggregated files (`🆓️free-space`, `🏗️build/💽️disk`, `🧹️scratch`, the admission fixtures, `process-watchdog-linux`).

| file (suite) | result | causes of the red tests |
|---|---|---|
| `🧪️tests/🟦️.ts` laws (native-artifacts) | **16/0**, 8.7 s | – |
| `🌊️streaming` (native-component-streaming) | **6/0**, 148 s | Run 1 was 0/7 because of my first misplaced `componentKey` (ReferenceError), fixed before run 2. |
| `♻️reclaim` (component-reclaim) | **2/0**, 38 s | – |
| `💥️failures` (native-component-failures) | **7/0**, 115 s | – |
| `🔁️retry` (native-component-retry) | **10/0**, 196 s | – |
| `🔏️seal` (native-component-seal) | **12/0**, 197 s | – |
| `🧩️closure` (native-component-closure) | **4/0**, 65 s | – |
| `🏃️jobs` (native-component-jobs) | **7/2**, 154-256 s | See "Jobs file" below. |
| `🛑️cancellation` (native-queued-cancellation) | **3/2**, 114 s | See "Cancellation file" below. |
| `🦀️real` (native-component-real) | **1/2**, 606 s | The optimizer test passes. The real wasm32 build hits its 600 s body deadline (see section 1); the second red is its `(unnamed)` isolated twin. Unverified. |

**Jobs file: 2 red tests.**
- `reserved nested compiler processes…` is stale or order-dependent; the cause is not proven:
  - Run 1: `parallelism` was 3 instead of 4.
  - Runs 2 and 3: the "reserved" row carries a jobserver.
  - The test takes the rows of `jobs.jsonl` in line order. The rows suggest the `shared` lane job finished first, against the declared lane order.
- `a job failing at once…` (counted twice, once as its isolated twin) gets the raw job error "exit 3 (attempt 1, …)" instead of "Native build job failed … job:broken":
  - The raw error is recorded as a `callback`-phase failure before the driver's own rejection.
  - My change (the job wait now stops the driver with the named error) did not change the result.
  - A second attempt (job failures no longer rethrown into the admission scope) did not change it either, so I reverted it.
  - Not traced to the end.

**Cancellation file: 2 red tests**, `pending stages…` and `the original finite caller budget…`. Both are load-bound, and the label fix works (section 3):
- With the fixture's 18 s budget, the body cutoff leaves 8 s (18 s minus the 10 s reserve). At load ~70, Cargo planning takes longer than that:
  - the stage is never entered (`entered=false`);
  - the unit-graph child never starts (`unit-calls.jsonl` absent).
- What I tried, both reverted:
  - Stretching the budgets by load (up to 7×) tripped the 10-minute "no progress" exit of `test-file`.
  - 30 s budgets made the file hang for 30 min, cause unknown.

## 3. `queued-cancellation`: wrong label on the finite-budget error

**Verdict: product, not test.** The caller must see the label of the operation that owned the deadline. Three layers lost it, all now fixed:

1. **`P🎛️owned-execution/🟦️.ts`.** When a command's signal ended with `ResourceLoanDeadlineExhaustedV1` (the body cutoff), it rejected with a generic `cancelled … cargo:planning-capture`. That error is not handled by `track`, so it closed the Original. Proof: run 2 of the file cascaded into "Original process admission closed - first failure: cancelled …". The command now rejects with that deadline object. New test in `⏳️deadline`: 5/0. Main file: 7/0.
2. **`CP🟦️.ts`, outer catch.** Any loan exhaustion leaving the driver is now passed through `ended()` and labelled "Original native build deadline exhausted". Before, this only happened when the signal was aborted, but a budget that runs out does not abort the signal.
3. **The law kit `withLawStorageV1` (`P📦️artifacts/🏗️native-build/🧪️tests/🦀️cargo/🟦️.ts`).** Its `finally { await storage.retire() }` replaced the body's failure with the retirement's refusal under the cut-off control. The body failure now stays primary, and a refusal caused by that same stop is dropped (the image's accepted cleanup reclaims the bytes).

**Result.** The driver and the storage layer now reject with "Original native build deadline exhausted" (run 5, via temporary `[DEBUG]` lines, now removed). The two tests are still red only for the budget reason in section 2.

## Open items

1. The real wasm32 test: find where the 600 s go. Start with the scheduler build, which needs a test-visible progress line outside the isolated child.
2. Jobs: the order of `reserved` vs `shared` lane jobs, and where the raw job error escapes ahead of the named driver error.
3. Cancellation budgets: give the finite cases a planning-independent clock (for example, start the budget after planning admission) instead of a bigger fixed number. The 30 s hang is not explained.
4. I did not run the old code to show the regression test red.
