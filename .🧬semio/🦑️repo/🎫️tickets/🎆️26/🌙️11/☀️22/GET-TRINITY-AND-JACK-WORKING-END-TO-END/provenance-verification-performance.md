# Cargo provenance verification performance

## Status of the reported build
- PIDs 4392 and 16984 no longer exist. `🗑️generated/plugin-build7.log` was last written 07:03 and ends with the final `[phase]` table, so build 7 had finished (not stuck) when checked.
- Phase table of that build: `total=7147 s`, `cargo=1300 s`, `provenance=4671 s` (78 min, 65 % of the build), `setup=361 s`, `plan:sources=330 s`.
- The 524 `[cargo-provenance] reading build records` rows are progress rows (one per second at most), not artifacts. Their segments show 14 physical owners and about 293 000 physical operations: one owner with 81 862 operations and 2.9 GB, then about 200 single-file rows, then 3-4 full structural passes of about 57 000 operations each (7 MB, no file bodies).

## Root cause (confirmed by measurement)
`CargoProvenanceSessionV1.recheckCompletedFiles` (sealed branch, `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts`) verified the directory rosters and absent paths of every owner with

`shared.structures(path => [...this.originalClaims].some(claim => claim.startsWith(path + sep)))`

The keep predicate runs once per structural path (tens of thousands, over all 593 owners), and each call spread the whole claim set (every current input row, 10 000 to 40 000 paths) into a new array and scanned it with a fresh string concatenation per claim. That is O(paths x claims) CPU on the main thread, single core, with large garbage (RSS ~500 MB). It runs in the seal, in the finalize recheck, and again in every `view.recheck()` of the retained compilation (called before and after each consumer task), which matches "CPU-bound, one slow step repeated per consumed artifact".

Two further quadratic sites in `completedCargoClosureRosterV1` (called three times per attempt: snapshot, finalize decode, output claims):
- `captures`: for every compiler resource, two scans over all units with `JSON.stringify` of both messages inside the callback.
- `unit.inputs.some(...)` per dep-info source (O(sources x inputs) per unit).

## Fix
- `claimDirectoriesV1(claims)` (exported, native-build `🟦️.ts`): one linear pass collects every directory prefix of every claim; the keep predicate is a set lookup. Exactly equivalent to `claim.startsWith(path + sep)` (a prefix ending at a separator boundary).
- Roster: message JSON set built once (`messages.has(JSON.stringify(row.producerUnit))`), per-unit `inputPaths` set. Same rows, same order, same custody (checked: real 576-unit receipt gives 10 495 rows before and after).
- Verification guarantees unchanged: the same paths are rechecked, against the same baselines; only the predicate cost changed.

## Measurements (loaded shared machine, bun 1.4.2)
| Case | Before | After |
| --- | --- | --- |
| keep predicate, 20 000 claims x 20 000 paths | 12 592 ms (5 s on a quieter run) | 19 ms |
| keep predicate, 40 000 x 40 000 | 27 541 ms | 35 ms |
| roster of real 576-unit receipt (`cargo-artifacts-KGTQrx`) | 286 ms | 117 ms |
| synthetic roster 400 units x 40 sources | linear before and after (about 190 ms) | same |

Not reproducible without the real storage backend: the native-storage law harness (`withIssuedLawStorageV1`) hangs on this loaded host (native-storage-mapping lease, cleanup deadline), so the retention snapshot/finalize path could not be profiled end to end. Session write+close over the real 576-unit receipt units (no retention) is linear and fast (about 1.5 s CPU), so the cost is in the retention/seal/recheck half. Filesystem metadata ops on this host cost 3-10 ms each (the `enumerate` bench: 500 entries 4.5 s wall, 46 ms CPU), which multiplies any redundant structural pass. I could not attach a profiler to the original worker (it was gone), so the attribution to the keep predicate rests on the op trace plus the microbenchmark above, not on a CPU profile of the original run.

## Tests
`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/🧪️tests/⏱️linear/🟦️.ts` with language-agnostic fixture `⏱️linear/🧫️fixtures/🔣️.json`:
1. claimed-directory set equals the exhaustive scan on fixture vectors, cross-checked with lodash `some`;
2. 4x more claims and paths grows time less than 9x (quadratic would be 16x);
3. roster over a generated observation keeps its exact row count (1586) and retained count (85), and 4x units grows time less than 18x.
Run: `cd C:\semio; bun test "./🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/🧪️tests/⏱️linear/🟦️.ts"` (3 pass).
The existing `receipt/🧪️tests/🟦️.ts` session tests time out at 5 s on this host (I/O latency) before and after; they do not exercise the changed sealed retention branch.

## Files changed
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts` (claimDirectoriesV1, recheckCompletedFiles)
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/📋️inventory/🟦️.ts`
- new `🧾️receipt/🧪️tests/⏱️linear/🟦️.ts` and `🧫️fixtures/🔣️.json`
- ticket scripts (inputs): `provenance-bench.ts`, `replay-bench.ts`, `roster-bench.ts`, `real-receipt-bench.ts`, `keep-bench.ts`, `enumerate-bench.ts`, `exist-bench.ts`, `storage-bench.test.ts`

## Does the running build benefit?
No process is running; any build started after this edit picks it up. A running worker would not: the build executes a captured snapshot of the sources taken at launch, so it must be restarted.

## Remaining cost to watch
Even linear, each retained-compilation recheck repeats about 50 000 structural filesystem operations plus a hash of every retained file, and per-owner structures are not deduplicated (593 owners, many shared paths). If a consumer calls `view.recheck()` per artifact this is still O(artifacts x structure). Next step if the build is still slow: recheck unique structural paths once per pass across owners.
