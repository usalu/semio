# Result E30 (tests-build-2), partial

## Verdict
Part 1 is solved (root cause was not the operation). Part 2 is only partly done: the fresh table covers 164 of 662 test files under B (142 passed at first run, 22 failed); the run is about 8 files per 3 minutes under load 25-55, so the remaining 498 files were NOT run (stopped on purpose, nothing left running). Table: `T/🧰️scripts/E30/table.md` (failures only), raw rows `T/🧰️scripts/E30/results-part1.jsonl`, file list `T/🧰️scripts/E30/all-test-files.txt`, runner `T/🧰️scripts/E30/run-table.py` (batches, waits while load > 45, resumable: `python3 run-table.py <seconds> <batch> <start index> <result name>`).

## 1. `repo test-file` on task-view cargo (5/5 now)
Cause: every repo process gets its TMPDIR below the checkout (`.cache/scratch/<pid>-process-X`, the declared scratch home). The test built its cargo workspaces there, so the checkout's `.cargo/config.toml` (`rustc = ingress script`) applied. The suite runner behaves identically; the operation was right. Fix: `🏞️task-view/🧪️tests/🧰️world` scratch is now below `SEMIO_TEST_ARTIFACT_DIR` (outside the checkout), row `build-task-view` declares `scratch: sandbox`, new law test in `🏞️task-view/🧪️tests/🟦️.ts` (row is sandbox, world follows the artifact dir). task-view cargo 5/5, task-view tests 13/13.
General rule found: a test that needs a root OUTSIDE the checkout (cargo config, workspace-root discovery, source snapshots that skip `.cache`, path-length budgets) must not use `tmpdir()`; use `SEMIO_TEST_ARTIFACT_DIR` (or `/tmp` for short-path laws).

## 2. Fixes (each proven with `repo test-file`)
| file | cause | fix | result |
|---|---|---|---|
| `⏱️profile/📊️cpu/🧪️tests` | stale: launcher now adds the graph child env for graph-reading commands and a partition pin for sealed mode | expectation uses `graphChildEnvironmentV1()`, sealed port `readPartitionPinV1` + pin env | 5/5 |
| `⚙️operations/🧪️tests/🧭️help-offline` | 116 sequential `spawnSync` of 3-13 s under load, 120 s timeout | async pool of 6, 240 s cap (inside 300 s budget) | 1/1 (119 s at load 30) |
| `🌐️remote-cache/🧶️nx/🚦️single-flight` | measured client time excludes client start (730 ms < 800) | one-sided: test-side elapsed >= 900 ms and client wait > 400 ms | 15/15 |
| `🔧️tools/🏗️bazel/📐️path-plan/🧪️tests` | corpus expectations stale (host-source 243 vs 227 units) | recomputed from the corpus's own Python oracle, TS and Python agree | 5/5 |
| `…/📐️path-plan/🧪️tests/🔒️real-bazel` | path-length law; checkout under `.cache/scratch`/artifact dir too long | short base `/tmp` (tmpdir on Windows) | 1/1 |
| `…/📸️snapshot/🧪️tests/🔒️planned` | sources below `.cache` not captured | artifact dir base; row `build-bazel-snapshots` `scratch: sandbox` | 2/2 |
| `…/🧪️tests/📥️paths` | same (`.cache` sources skipped) | artifact dir base | 11/11 |
| `🔧️tools/🏗️bazel/🧪️tests` | stale: target moved from root `📋️project.json` to the typescript package project | path updated | 15/15 |
| `🔭️witness/🧪️tests` | scratch below `/.cache` | artifact dir base | 13/13 |
| `🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests` | stale: adopt repairs a damaged closure leaf (documented product behaviour) instead of throwing | expects repair and checks the bytes | 30/30 |
| `🌐️remote-execution/🚀️offload/🧪️tests/*` (6 files + workspace helper) | in-checkout scratch | artifact dir base | main file 28/30 (was 27/30), see below |

Landing record: `T/landings.md` "Wave 1122 E30" (round 1 entry; the later fixes above are all test-side except the two suite rows; lock `suites` taken and released).

## Still failing (analysed, not fixed)
- `🚀️offload/🧪️tests/⏱️timeouts` (upload-hang test: outcome detail is a gRPC transport error instead of `upload`) and `🚀️offload/🧪️tests/🟦️.ts` ("cache endpoint probed once": probe counter 0, i.e. the offload does not reach the test's cache server); `🚀️offload/🧪️tests/🌐️cluster` 8 of 9 fail ("the remote compiler runs"). Not caused by scratch location; suspected: the offload resolves the cache/endpoint from somewhere other than the test environment under the suite envelope. Needs the remote-execution owner.
- `🕸️graph/📥️inference/🔖️stamps` (source-text greps of observation code no longer match; E29 is editing it), `🕸️graph/📥️inference/🧪️tests/🎓️course-inputs` (proctored run, error stream, 187 s) and `🚶️repository-walks` (settler vs oracle mismatch, 44 s): graph lane.
- `🛰️sidecar/🚦️ready/⏳️tools-grace/*` (Original envelope refusal in the test process), `🌲️process-tree` graph-progress/containment (extended vs timeout; lease owner token): sidecar, E27's area, not touched.
- `🔬️test-file/🧪️tests/🧫️fixtures/🔴️failing`: intentional failing fixture; it imports bun:test so a census counts it, not a defect.

## Not done / not verified
- 498 of 662 files never run (list = `all-test-files.txt` minus `results-part1.jsonl`).
- `🗂️workspaces/🦀️cargo/🧪️tests` is NOT under B (it lives in `📚️library/🗂️workspaces`). It is run by its project script (`contract-check`, via `runRepositoryCommand` with the cargo preparation storage), not by a suite table, so `repo test-file` runs it `plain` and it fails on "Original short preparation custody storage required". Routing it needs a suite-row environment provider (or a project-script route in test-file), not a row; I did not add one. E24 (caching) was not contacted.
- Skipped as touched in the last hour by active lanes: `🌐️remote-execution/🛰️sidecar/🧪️tests/🪜️priority` (E27), `🏡️resident/🧪️tests/🛟️key-keeper` and `🚪️client/🗝️key`, `🕸️graph/📥️inference/🧭️script-inputs/🧪️tests/package-metadata-edits.test.ts` (E29), `🌐️remote-cache/🛰️fleet/🪪️identity/🧪️tests`, `🧪️tests/🕸️graph/🩹️repair-rounds`, `🧪️tests/🕸️graph/🌱️native-scope` (details in findings-E30.md).
- Windows/Linux not run.
