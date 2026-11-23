# T2a test sweep: caching (unit cache, stage-fleet, cargo, storage, bootstrap, graph, pruning) + process/native-build

Method: `repo test-file --json --lang=en <file>`, one file at a time, sequential (runner outside the ticket, private scratch dir), host load 30-55. 673 test files (every `bun:test` file under `LIB/⚡️caching/` incl. unit-cache subfolders, stage-fleet, cargo, storage, and `🔨️modules/🏃️process/📦️artifacts/🏗️native-build/`). Full per-file table (first run + rerun, pass/fail counts, seconds, first error): `T2a-table.tsv` (same folder).

## Totals
- First run: 548 passed files, 125 failed/unparsed. Total wall 7205 s (~2 h).
- After fixes + one rerun of every failure under 150 s: 553 files pass, 120 fail (3918 passing test cases, 465 failing cases in the final column of the table).
- Tonight's fleet lanes (D2 K1-K6 R1 R2 O1-O9 F1-F4 N1 S1 C2 CAS1): every failure inside the unit-cache tree was caused by them or by load, and is now green (below). Everything else red is NOT from tonight's lanes.

## Failures caused by tonight, fixed
| file | first | cause | fix | final |
|---|---|---|---|---|
| `unit-cache/🏗️buildscript/☁️offload/🧪️tests/🟦️.ts` | 8/2 (52 s) | O5/C2 tests assert `offload-run` events with crate `wrapped build script` / `light build script`, but the fixture environment names every package `tiny` (offload label = `<CARGO_PKG_NAME> build script`): stale expectation | test env now sets `CARGO_PKG_NAME` `wrapped` / `light` (test only) | 10/0 |
| `unit-cache/🧪️tests/🟦️.ts` (main) | 27/1 | R2 added `retention` (Declaration v2) to build-script run entries (`BuildEntryV1.retention`) but the unit-cache schema `BuildEntry` (`additionalProperties:false`) lacked it: product schema bug, `validator("BuildEntry")` refused every run entry | schema-first fix: `retention` property added to `BuildEntry` in `unit-cache/🧬️schema/🔣️.json` | 28/0 |
| `unit-cache/🛰️fleet/🧪️tests/🥀️stale/🟦️.ts` | 0/1 | R2 e2e fixture races the 50 ms retention grace: the whole test is ~115 ms, the old-tree unit can still be "young" at `collect()` under load | test waits 150 ms (> grace) before `collect()` | 1/0 |
| `unit-cache/🧹️gc/🧱️units/🧪️tests/🟦️.ts` | 12/1 | free-space delta measured around a no-capacity reclaim goes negative while peers consume disk (load/timing), source untouched tonight | none, rerun | 13/0 |
| `storage/🔌️nx/📦️outputs/👁️observation/🧪️tests/🟦️.ts` | 14/1 | load/timing | none, rerun | 15/0 |

Landings (sha256 12 chars before -> after): offload test 25efb3a759b6 -> 6844de7ae48c; schema 657af85c85ea -> e653581cb916; stale test 23c27596fa7e -> 63a355d36918. No product code changed (the schema is the product-side fix).

## Still red, not from tonight's lanes (evidence: asserted code not touched by D2/K/R/O/F/N/S/C2/CAS1)
120 files; first error classes (column `first_error` in the table):
1. 36 files: `Original ... required` / `Actual ... required` / `Exact ... required` / `A test reached the live repository store ... give it a private repository` / `invalid process.testing.capabilities-differ` / `Undeclared original Repo build selection`. These suites need the genuine `repo test <suite>` birth/native-caller envelope; `repo test-file` does not give it (already T1 section 2, a test-file routing gap). Pre-existing.
2. 5 files `nested repo cargo` (native-build admission, jobserver, telemetry, frontend-threads, units/capture memo): the nested `repo cargo ...` hits `Internal error: Undeclared original Repo build selection`. Same family as 1.
3. 6 files `ReferenceError ... not defined` (`refreshStaleImageBeforeOpenV1`, `createIssuedResourceControlV1`, `admitNativeImageRelocationV1`, `withProvisionedPythonV1`, `withoutJobserverEnvironmentV1`, `schema`): a peer stream is mid-edit (new untracked/added `📥️opening/🔄️refresh/`, PY1 Python binding `withProvisionedPythonV1` in `native-build/📥️inputs/`). Python-binding class tagged: `native-build/📥️inputs/🧪️tests/📥️consumers` (PY1), `suites` test (see below).
4. 61 `other assertion` failures in the storage/graph/native-build images, closures, headroom, retirement, nx-graph, sdk-derivative, path-envelope, source, component-pass trees: all in the APFS/DMG storage + bounded-graph subsystem which none of tonight's lanes touched (`💾️storage/*` is only modified by the `📥️opening/🔄️refresh` addition and `🧬️lifetime`, another stream). Not attributed individually.
5. 12 `unparsed` (stdout empty under `--json`): re-run in text mode, `nx-output-observation/🔑️kernel` and `🩹️patches` pass (3/0, 8/0); `authorizers`, `📥️invocation`, `🏭️producer`, nx-plugin `📤️arguments`, `caller-environment`, `playground-preparation`, `pruning/🔒️incremental`, `pruning/🔒️store`, `📦️artifacts/🔒️bounded`, `🔏️seal/📥️journal` fail (exit 1) in text mode too. Note: `--json` printing nothing for passing files (kernel, patches) is a test-file defect worth a ticket (stdout empty while exit 0); not investigated.

Specific items worth a look by owners:
- `🗂️suites/🧪️tests` (4/1): suite `bootstrap` (15 s budget) contains process-spawning `🚀️bootstrap/🔌️nx-plugin/🧊️engine-references/🧪️tests/🟦️.ts` (created 11-22 17:14, before the wave); the budget-class oracle demands >= quick (300 s). Pre-existing, one-line suites-json fix, but `🗂️suites/🔣️.json` is staged by someone else, left alone.
- `🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🧪️tests` (11/1): after cutting the spawn-only reference the engine closure still contains `🏗️build/📜️script.ts`: some file in the engine closure imports it non-spawn-only now. `🏗️build/📜️script.ts` was edited tonight (03:17, dynamic `./🗄️cas/🆘️full-disk/🟦️.ts` import, not one of the listed lanes). Unattributed; owner of the full-disk reclaim should verify.
- Python binding (PY1) tag: none of the 120 files printed `Independent Python oracle required`; the Python-class failures here are `withProvisionedPythonV1 is not defined` (NB inputs/consumers) and the engine/staleness python oracles are exercised only through remote-cache (T2b).
- `🚀️bootstrap/🛰️fleet-fetch/🦀️crates` (3/1): `current package believes it's in a workspace when it's not` (standalone crate vs root workspace), not touched tonight.

## Notes
- The shared scratchpad directory is shared between lanes; T2b's `run.sh`/`order.txt`/`results.tsv` overwrote my first runner at 05:51 (I lost ~10 min and the first row), then moved to a private subdir. Whoever uses the scratchpad should use a lane subfolder.
- Per-file cost: 1-60 s typically, outliers 757 s (`native-build/🧾️receipt/📦️retention`, queued behind a shared bun slot), 180-350 s for storage closures.
- Nothing restarted, no live stores touched, `lock.tests-a` taken and released for each edit.
