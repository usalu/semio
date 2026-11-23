# Cold owner discovery: where the time goes, what changed, what did not help

Scope: `repo run` reads the project graph slice first; PID 17996 spent 7 298 s in cold owner discovery, 2 868 s in inference, 2 247 s in discovery again after a worker replacement. Goal: make discovery and repair rounds scale. Result: **a small, safe reduction of redundant probes landed, but the replays show no large CPU saving, and the structural fix (resume/skip by listing fingerprint, parallel subtrees) was NOT done.** The numbers below say why.

## What the persisted memo says about discovery (scope `171969…admission.json`, 425 owners)

* 10 owners select targets; the other ~415 are cheap (median 61 rows, declaration only). Cold discovery time is dominated by a handful of huge owners: `framework-rs` 97 248 rows, `s-fem-composition-tests` 94 029, `framework` 59 737, `framework-actor-rs` 42 735, `ui-rs` 41 927, ... (769 436 owner-row references, 162 021 distinct).
* Per kind (distinct / references): lstat 52 931 / 311 309 (5.9x), readdir 31 697 / 157 246 (5.0x), analysis 17 755 / 77 111, exists 14 677 / 58 932, resolve 13 457 / 69 710, fileExists 13 141 / 44 337. The reference counts are the rows each owner's *verification* walks (shared by the round cache), not extra discovery syscalls.

## Isolated replays (scripts in `🧪️graph-read-perf/`: `owner-discovery-replay.ts`, `discovery-walk.ts`, `owner-rows.ts`, `owner-kinds.ts`, `owner-with-targets.ts`, `prewarm.ts`)

`owner-discovery-replay.ts` runs the real `discoverBuildOwnerAdmissionFactsV1` for owners taken from the memo inside one `settleObservedComputationV1` round. Fidelity check: the row counts equal the memo's (`framework-os` 21 055 vs 21 061, `framework-os-dev` 17 809 vs 17 805, `framework-os-kernel` 21 033 vs 21 033, `ui-styling-tokens` 11 763 vs 11 767).

| replay | wall | CPU (user+sys) |
| --- | --- | --- |
| `ui-styling-tokens`, 11 763 rows, first touch (cold file cache) | 44.4 s | 7.6 s |
| same owner again (warm) | 11.9-13.1 s | 6.6 s |
| 3 owners (`framework-os`, `-kernel`, `-dev`) in one round, before / after my change (warm) | 29.3 s / 14.8 s (noisy host) | 5.25 s / 6.06 s |
| syscall counts of that replay, before = after | lstat 16 322, readdir 17 464, readFile 55, stat 86 | exists 302 -> 83 |

Findings:

1. **Discovery CPU is not redundant.** The second and third owner of the same tree cost 1.9-9 s against 36 s for the first: sharing across owners already works. Syscall counts for lstat/readdir are identical before and after my change, so the 5x row-reference ratio is verification, not discovery.
2. **Wall time is first-touch I/O latency, not CPU**: 44 s wall for 7.6 s CPU cold against 12 s warm for the same owner (host saturated, see `graph-read-performance.md`). CPU profile of the warm replay: `readdirSync` 32 %, `lstatSync` 10 %, `readFileSync` 6 %, TypeScript parse/intern ~10 %; no single user-space hot spot.
3. The 2 247 s second discovery after the worker replacement is the owner checkpoint/memo failing its hold (peers edited rows an owner consumed; the admission format key also flips with edits to its 8 files), so owners are re-observed wholesale. Fixing that needs per-owner partial re-verification (re-run only when a changed listing name matches one of the owner's input patterns) or a watcher journal; I did not build either.

## Change (kept)

File `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs` (+ `🟨️.d.mts`):

* `observedFs.readdirSync(path,{withFileTypes:true})` is answered from the round's typed-listing cache (`typedListingV1`) that the hollow-directory check already fills, returning a copy of the array. Per round a directory is listed once. Isolated walk (`discovery-walk.ts`, 1 523 directories): readdir syscalls 3 047 -> 1 524 for one walk and 6 095 -> 1 524 for three walks, identical 3 048 recorded rows.
* New `roundAnswerV1`: `existsSync`, the TypeScript host's `fileExists`/`directoryExists`, and `observedRequireResolve` are answered once per round (existsSync syscalls in the 3-owner replay 302 -> 83). Facts are still recorded for every call.
* New `forgetRoundReadsV1()`, called from the owner retry hook in `🕸️graph/📥️load/🟦️.ts` (`onRetry`), so a retried owner never re-reads a stale answer of the same round.
* Correctness: a cached answer can only be older than a fresh probe within one settle round; the round's rows are re-verified by the settle/seal verification (which uses the uncached `facts.*` probes), exactly as every fact recorded earlier in a long round already is. Outside a round nothing is cached.

## Tests

New `👁️observation/🧪️tests/🗂️discovery-reads/` (`🔣️.json` language-agnostic scenarios, `🐍️.py` independent model of listing counts and answers, `🟦️.ts`): 3 pass / 0 fail (counted fs port: one listing per directory over N walks, repeated existence asks answer identically, new round re-reads, `forgetRoundReadsV1` re-reads, a path that appears after forgetting is seen).
Regression runs: `round-listings` 3/3, `listed-children` 4/4, `validation-round` 4/4 (needs `--timeout 90000` on this host), `check-memo` 2/2, `conflict` 2/2, `resolution` 5/5, `stale-row` 2/2, load `settle-churn` 4/4, `inventory` 10/10, owner-discovery `observation` 4/4. Pre-existing failures unrelated to this change (verified against the previous file for the first): owner-discovery `authority-round` (digest mismatch), `owner-scale` (child import path of an emoji path), load `membership-follow` (needs `SEMIO_TEST_ARTIFACT_DIR`).

## Not done / recommendations

1. Per-owner partial re-verification: when an owner's hold fails only on `readdir` rows, check whether the changed names match the owner's patterns before re-observing (needs the scan patterns persisted with the owner entry).
2. Checkpoint format key narrower than the 8-file hash (each edit today makes every persisted owner fact a miss).
3. Parallel I/O for first touch: a worker-thread/async pre-listing of the scope's trees before discovery. A quick `prewarm.ts` test on small trees was inconclusive (OS cache and host noise dominate); needs a measurement on a truly cold big tree before building it.
4. Host: Defender exclusions and the stdin-hanging `python3 -` processes remain the largest lever on wall time.

No `[DEBUG]` prints were added; generated logs/profiles under `🗑️generated` were removed. No process other than my own replay (PID 58920, started by me) was killed.
