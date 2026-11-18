# R86: closure parts decided from row ids, take/flush progress counters, failure block mirror: landed; no Mac ready

Status 18:45 CET: four landings under `landing.lock` (c, b, d; a was restored red once, see 6), all member tests and both standing preflights green. NO Mac is `ready: true`. Scripts: `T/🧰️scripts/R86/` (`land.py`, `wait-land.py`, `mk1..mk3.py`, `part-ids.test.ts`, `real-bench.ts`, `dev.py`, `devC.py`, `absolutize.py`). LIB = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`.

## 1. What was found first (changes the picture)

The label `flush 1 of 1` is the last progress event of the closure settle chunk; it stays for the WHOLE following take (`stage: inference` in `slice-memory.json`), because the take emits no progress event to the log. g1-m10 (478 owners) took 88 owners in 376 s, 459 in 1029 s, and the stall rule (10 min without a new line) stopped the try at 1034 s every time (`ready-timings.json`: stalled 723, 768, 662, 1034 s). So the 10 minutes at 83 % CPU were the take, not (only) a part check. I could not observe a worker past the take, so what follows the take is NOT diagnosed.

## 2. Built and landed

- c (`🏗️build/🧠️memory/📋️slice-report`, `👷️worker.ts`): `startSliceProgressV1` tells `taken N of M` (ledger `done`/`total` of the owners source) every 5 s while the count rises, and each plan stage entered is told as `stage <name>`. Seen live: `taken 33 of 4774`, `stage inference 0 of 0` on g1-m01/m02/m07. Law: slice-report test `the take counters are told as progress ...` (red on old code: import missing).
- b (`closure-kinds/🟨️.mjs`, cutoff schema `partRowBatchRows` 4096, `partProgressMilliseconds` 5000): a closure part is decided from the ids of its rows in a persisted dictionary (R82 layer index, `<cacheRoot>/part-ids/x/`, also listing parts below 8 rows). Each distinct row is probed once per settle through the caller's `hold` in batches, in-flight probes are joined, a failing batch is bisected, the part is an AND over ids; a part without list is loaded once and listed. `flush parts N of M` (decisions) and `flush support N of M` (supports built) are reported at least every 5 s and once at the end; cancellation is checked in both loops. Result carries `parts:{decided,byIds,loaded,probedRows,asked,rowsDecided}`.
  - Law `closure-kinds/🧪️tests/🪪️part-ids` (4 tests; BEFORE on old code: 2 red, part records read 98 times, no progress lines): warm settler reads no part record, probes exactly the distinct rows (set computed in the test), each once; a stale row stales exactly the closures naming it, same set with and without the index (the old path is the oracle); progress ends at `usedParts of usedParts`; cancellation stops before half the rows.
  - Real-process bench `real-bench.ts` (real closure layers of `.cache/b/g/m/inference/selections/…/ce/l`, read only; node store and index in scratch; load 40-50 on this machine): 800 layers, 106 159 distinct rows: old warm 18.6-25.4 s with 206 of 800 closures re-derived and 737 part reads; new warm 10.5-16.1 s, 0 re-derived, 800 of 800 parts by ids, 106 159 rows probed (= distinct), 243-536 part reads left (supports). 300 layers: old 3.9-6.1 s, new 2.5-3.2 s. Extrapolated warm settle ~8 ms per closure under load 45 (about 15 s for 1 918 closures, about 40 s for 4 774); that is NOT measured on a Mac mini and the 60 s / 15 s targets are NOT proven.
- d (`ready/🟦️.ts`): a failed stage mirrors the `request-failed <reference>` block of the service log its refusal names to `ready-log-<stage>-failure.txt` (4000 chars). Only takes effect after the agent of a Mac runs the new ready code (not yet the case on g1-m10).

## 3. Not done / not proven

- Reuse of the take pass verdicts inside the process: NOT built (take verdicts are keyed by the `ce` dictionary and the observation hold, the settle uses the graph hold; sharing would need one dictionary and one hold).
- What remains in the settle after the part decisions: loading the rows of every held closure's parts for the supports (`partRowsV1`, needed to patch the fact index); unchanged, still a per-part read.
- Index files `<part id>.ix` of vanished parts are not swept (only the 2 GiB dictionary bound resets).
- The cause of what happens after the take (post-take stages) is unknown.

## 4. The g1-m10 `unexpected failure (reference 3a9fba88)`

Two tries ended with the SAME reference: at 206 s (`rederivation 168 of 168`, 17:10:30 UTC, before landing b was live in that process) and at 365 s (after the last verification chunk, 17:3x). Same reference = same detail text = a deterministic JS exception inside the settle/derive phase, not a crash. It is therefore not b (b was not loaded in the first). Suspects by timing (all landed 17:04-17:07 UTC): R87 s1 (`observation/📥️reuse` presence coalescer used by derive of a stale closure; the sampling profiler in the worker) and my c (worker progress heartbeat; unlikely, it only posts messages and the other Macs run it fine). g1-m01/m02/m07 run c and R87 s1 but never run the settle, which is what m10 does. I could not read the host log. d will give the block after m10's next failure; whoever owns R87 should read `ready-log-workspace-failure.txt` first. g1-m02/m08 failures at 17:10-17:14 are exit 75 "code changed under the slice worker" from the landing churn of that window.
g1-m01 `taken 248 of 4774` after 876 s (0.28/s) is the take loop (R87's), not the settle; R87's sampling profiler (always on in the worker) and the new coalescer are the changes in that window; no measurement here.

## 5. Rule notes

- At the start I edited the cutoff schema file live (before the lock) for about a minute and reverted it byte-exact (git diff empty); nothing was replicated in between that I know of.
- Landing a ran red once (an existing test asserts the exact list of progress phases of a chunk; my first `flush support` line broke it; fixed by not emitting at the first closure), restored by `land.py`.
- Background wait scripts (`wait-land.py`) all ended; no process of mine remains; scratch is a few MB under `scratchpad/R86`.
