# F38 result

## 1. F33 storage rescan: re-landed
The rescan was right, the `📏️paths` test was wrong. The test fires each mutation once; a bounded rescan then sees a stable second walk and accepts (by design: the failure on m05 was a one-off change). The test's law is "one walk refuses every change", so it now runs the scan with `maximumScanAttempts` 1 (injected schema copy). The bounded rescan is pinned by the new `🧪️tests/🔁️rescan`. Landed storage `🟦️.ts`, `📥️install.json`, rescan test, paths test. Tests: rescan 1/0, paths 3/0, edge-scan 1/0, ancestry 7/0, fence 8/0, main 47/0, work 3/0, progress 2/0.

## 2. Red files: classification (none caused by the train's patch)
| File | Cause | Train? | Action |
|---|---|---|---|
| budget admission 5/1 | wait notice printed only the environment's language (`noticeLanguagesV1`), test expected both | no, language protocol drift | test pins the language env, 6/0 |
| native-storage borrower 1/9, receiver existing 0/5 | suite `native-storage` granted no `artifacts` while these files declare `artifacts:true` and use the artifact dir (`capabilities-differ`) | no (suite table gap) | suite gets `capabilities.artifacts`; the 4 files that declared `{}` now declare `{artifacts:true}`; borrower 9/0, receiver 4/0, operation 12/0, mapping 5/0, journal 1/0, contract bootstrap 3/0 |
| selected-state-recovery 0/2 | extracted `openRepositoryCachePartitionAttemptV1` lacked ports `refreshStaleImageBeforeOpenV1`/`keepRepositoryImageReserveV1`; then the test expected "not issued" after close, product says "already closed" (typed refusals) | no | ports added, expectation updated, 1/0 |
| native-build storage main 5/1 | `buildCargoArtifacts` writes the phase ledger to `<cwd>/.cache` before the refusal | no (phase ledger 17:03) | test ignores `.cache`, 6/0 |
| F31 engine-digest 11/1 | `disk-recovery/real` (17:03) and `idle/loop` reference `📜️script.ts` by URL and are not in the spawn-only table; ready imports `🩺️doctor` for real | no | two spawn-only rows, corpus preflight->doctor and doctor off mustLeave; 12/0 |
| headroom (timeouts) | load-dependent | not touched |
Not fixed, pre-existing, listed: `🧪️testing/🗂️suites/🧪️tests` 3 laws fail (new unrouted test files `🆓️free-space/*`, `🏗️build/🏠️machine-state/*`, `🏗️build/💽️disk`, `🧹️scratch`, `🎟️capacity/🧮️units/🧪️tests/🛰️fleet-jobs`; test files that import sibling test files in `📁️directory`, `📚️witness`, `⏳️admission`, `🔎️resolve`); owner of those tests must route them. `📥️invocation/🧪️tests` types step and nx-plugin `📤️arguments` were not analysed.

## 3. dangling-imports.py
Rewritten (`T/🧰️scripts/coordinator/dangling-imports.py`): a small tokeniser drops comments, strings, template literals (with nested `${}`) and regexes, and counts only `from "x"`, `import "x"`, `import("x")`, `require("x")` with relative specifier; decodes `🧰`/`\u{..}` escapes, strips `?url`/`?inline`, tries `.ts/.tsx/.mts/.mjs/.js/.json/index.*` and `.js`->`.ts`, skips `dist`, `target`, `.cache`, `🤖️generated`, `*.d.ts`. Selftest `T/🧰️scripts/F38/dangling-imports-selftest.py` (13 cases, ok). Whole `🧰️framework`: old 10052 hits, new 0; a real broken import is still reported.

## Not verified
Real suite runner (not `repo test-file`) for the changed suite table; effect of the two new spawn-only rows on live Nx task hashes (the surface does not change, only the closure).
