# R57: the parent of the owners wave (observe, refs) and what really blocks it

T = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/`, B = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`. Scripts in `T/🧰️scripts/R57/` (`land.py`, `rehearse*.py`, `measure-*.mjs`, `laws-*.ts`). Trace used: `/var/folders/pm/zdg449hs6r33w_2gtsn23byc0000gn/T/semio-footprint/8889.log`. The running read (8889, 91760, 91761) was not touched.

## 1. What `observe` and `refs` are (verified in code)

- The owner clock (`B/🧠️memory/🦶️footprint/🟨️.mjs`, `createOwnerClockV1`) is a wall clock per owner, not a CPU profile. `observe` is charged at `B/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:313` as `Date.now() - inferStartedAt - farm - inline`: everything between the start of `observeProjectInferenceV1` and its return that is not the farm child. `refs` is `resolveClosureRefsV1` (`:315`, `B/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs:485`). Both are awaited inside one of ~15 concurrent owner loops of ONE event loop, so each includes every stretch in which the thread ran something else: a 656 s wall for an owner whose child needed 35 s is the contention of 15 owners plus the stalls below, not 260 s plus 360 s of work of that owner.
- `observe` after the farm: `observeInferenceComputationV1` (`B/👁️observation/🟨️.mjs:949`): `lookup()` reads the adopted record, `settleClosureRowsV1` (brings the shared closure entries named by `closure` rows up to date in the parent, `:437`), then `current(rows)` (`:843`) twice (candidate and final), then `inferenceReusableV1`.
- `refs`: `closureApiV1(...).resolve` (`B/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs:1495`): per recipe `ensureClosureEntryV1` (derive once per process and recipe in the PARENT: TypeScript parse and import walk, `walkImportsV1`, saved to the durable store; otherwise load plus `closureEntryHoldsAsyncV1`, cooperative and memoised per validation round), then the owner's row list through the row log (a list of ids per layer, written once per layer).

## 2. Measured (real owners, real code, 7 owners, `T/🧰️scripts/R57/measure-*.mjs`; machine load average 41 to 54, swap 4.2 of 5.1 GB used)

| what | number |
|---|---|
| rows in a record of the wave (late closures) | 29, 119, 72, 75, 71, 79 (the closures are separate entries) |
| rows in a record without late closures | 25 202; `current()` over them: 125 to 225 ms, 18 400 syscalls (11 069 stat, 4 844 lstat, 2 378 exists) |
| parent `observe` of an owner whose closure entries are warm | 7 to 37 ms |
| parent `refs`, warm, with the row log (the real path), 30 repeats | 11 to 612 ms (median about 40 ms) |
| parent `refs`, first derivation of a closure | 1.8 s to 52 s (test-runner-go 10 to 18 s, test-domain 18 to 52 s) |
| child inference of the same 7 owners | 57 to 67 s in total; the parent needed 55 to 125 s for observe plus refs |

So the premise "the parent re-stats the files the child read" does not hold for the wave: late records carry about a hundred rows, so there are about a hundred checks per owner, 10 to 40 ms. I did NOT build the "child returns observations, parent merges, zero synchronous stats" path: it would save at most tens of milliseconds per owner and add a second source of truth. The `refs` and the large `observe` values are (a) the first derivation of a shared closure in the parent thread (TypeScript parsing: `parseListElement`, `walkImportsV1`, `readFileSync`, `update`/`Hash` in the profile) while 14 other owners wait for the same in-flight entry, and (b) the stalls of the thread.

The throughput of the live wave from its trace: 0 owners at 18:11 UTC, 2 745 of 5 101 at 18:57 UTC, 60 owners per minute on average (595 / 595 / 610 / 220 per 10 minutes). The "1.2 owners per minute" does not appear in the trace; the slowest 10-minute window was 18:41 to 18:53 with about 18 per minute.

## 3. What blocks the thread (the finding that matters)

From the trace (4 160 s, 309 samples): the sum of the reported loop lags is **2 691 s = 65 % of the wall time**; 228 starved rows, 2 341 s blocked; single lags of 20 to 52 s. System: swap 4.2 of 5.1 GB used, the process holds 2.1 GB resident against a physical footprint of 3.4 GB (the rest is compressed or swapped), 10.5 million heap objects. Everything that walks the whole heap therefore touches pages that must come back, and it runs on the one thread. Three such walks run in the parent by design and each had no bound on its share of the wall time:

1. **The heap census of the sampler** (`createHeapSamplerV1`): `bun:jsc heapStats` walks the whole heap. Measured: 0.55 to 0.73 s for 6 M live objects, 1.5 s for 36 M (unswapped). `starved()` ran one in the hot code of the parent at every pulse (3 s) while the loop was starved, which is exactly when it hurts most (228 starved rows plus 309 regular rows in 69 minutes): the diagnostic lengthened the stalls it reports.
2. **`releaseIfGrownV1`** (called from `yieldInferenceV1`, every 16 yields): a full collection, `Bun.shrink`, a second full collection whenever heap in use plus external grew by 256 MB, garbage included, without regard to how much of the grant the process holds (the trace shows 3.4 GB of a 14.3 GB grant).
3. **`createParentCollectorV1.afterProject`**: a full collection after a project when the managed bytes passed live plus 384 MB, at most every 3 s regardless of how long the previous one took.

## 4. What was built and landed (R57 entry in `T/landings.md`, 2026-11-15 19:56)

Schema first: three declared numbers, `censusDutyPercent` 5 and `releaseDutyPercent` 10 (`B/🧠️memory/🦶️footprint/🧬️schema/🔣️.json`), `collectDutyPercent` 10 (`B/🧠️memory/🧬️schema/🔣️.json`, fixture `B/🧠️memory/🧪️tests/🔣️.json`). Code:
- sampler: a count that took C ms is followed by at least C x 19 ms without one; a starved row in between still carries the stack, the blocked time and the LAST counts; a regular sample waits (`B/🧠️memory/🦶️footprint/🟨️.mjs`, `createHeapSamplerV1`).
- `releaseIfGrownV1`: no release while the footprint gate says `run` (below `softPercent` of the grant), and after a release that took C ms the next waits C x 9 ms; `state`, `now`, `decision` injectable.
- `createParentCollectorV1`: after a collection that took C ms the next waits max(3 s, C x 9 ms).
No behaviour of the graph changes: only when memory is given back and when it is counted. Nothing was weakened in soundness (no cache, no verification, no observation touched).

Laws (language-agnostic numbers, red before / green after, rehearsed outside the tree by `rehearse.py` / `rehearse_memory.py`, then appended to `🧪️tests/🟦️.ts` of footprint and `🧪️tests/🧹️parent-budget/🟦️.ts`):
- census: over 10 simulated minutes of a permanently starved loop with a census costing 2 s, the share spent counting is **0.40 before, at most 0.055 after**; every starved row is still written (rows = pulses), rows between counts carry the last counts.
- sample waits for the quiet time and speaks once it passed.
- release: share of wall time in forced releases (3 s each, garbage returning at once) **0.75 before, at most 0.11 after**; never below the soft limit, runs from it; the default decision reads the footprint gate the process holds.
- collector: share **0.73 before, at most 0.12 after** (8 s per collection, garbage returning at once); a cheap collection keeps the old pace (law passes before and after).
Red before is by absence of the declared numbers/options (4 of 4 and 1 of 2 red on the old module, verified with `R57_BASE=1`).

Tests run on the landed tree (all in the landing entry): footprint (with its existing tests), clocks, memory root and its seven sub-directories, cut, settle, plan-stage, closure-kinds dependencies, phases, project-nodes slow-last / inferred / away, the standing preflight and heavy-questions. Green except the memory root test "the process tree reader sums the resident memory of the process and all its descendants and nothing else": red before my change as well (verified on the untouched tree), environment dependent, not mine. The fixture test ("policy equals its declaration") had caught my first attempt (missing key in the language-agnostic fixture): the landing script restored the files and landed the second time.

## 5. Before and after on real owners (harness `measure-duty.mjs`, 7 owners in one process with the footprint gate and sampler bound, same load)

| | before | after |
|---|---|---|
| forced collections (`Bun.gc`) | 50 calls, 7.5 s | **0 calls, 0 s** (footprint far below soft) |
| heap census | 33 calls, 9.3 s | **14 calls, 6.5 s** (small heap here: 0.28 to 0.47 s each) |
| wall of the run | 132.8 s | 144.1 s |
| parent observe + refs | 18.4 + 48.2 s | 13.9 + 55.7 s |
| loop lag, sum / max | 96.8 s / 20.5 s | 103.7 s / 19.4 s |

Plainly: **no end-to-end improvement is measured in the harness.** The forced work it removes is 16.8 s of 133 s there (13 %), within the noise of a machine at load average 50 (the `refs` of the first derivations alone varied between 40 and 56 s from run to run). The harness heap is small (a few million objects, unswapped), so it cannot show the effect that matters in the server: walks of a 10-million-object heap whose pages are partly swapped. That effect is the argument, not a measurement.

## 6. NOT verified

- Whether the three bounds lift the live wave. The running read keeps its old code; its trace (`8889.log`) is the "before": 65 % of the wall in loop lag, 60 owners/min. After the next real wave start: compare in its trace the sum of `loop lag` over the span (target well below 65 %), the number of `starved sample` rows, the end-of-wave row of the collector (`collections`, `collectMilliseconds`, via `graphWaveMeterV1.parent`) and owners per minute.
- That the collections/census are the dominant stalls (the parent-side profile in the harness shows `gc` 19 % of a 178 s run, forced from `yieldInferenceV1`, and `heapStats` as measured above; for the live process I have no profile: macOS `sample` on pid 8889 returns unsymbolised Bun frames, the main thread was 100 % busy for 10 s). Other stalls exist for sure (the first derivation of closures in the parent: TypeScript parse, 2 to 52 s each, JSON/brotli of entries `save`/`listDigest`, 8 to 12 s of a 178 s profile).
- Not built, and the structural step that remains: **derive the shared closure entries in the farm children**, not in the parent. The pieces exist: the project-farm service worker already opens the same session (`lateClosures`), `produceClosureEntryV1` can run there into the staging store, the farm already adopts leaves (a `ce` leaf would be adopted like a record), `closureTravelV1` already moves entries as bundles, and the parent then only does `store.load` + `closureEntryHoldsAsyncV1` (cooperative, memoised per round). The walk farm of the derived nodes (`closure-kinds/🏭️walk-farm`) is the other precedent. This touches the settle/closure code R53 is editing and the farm protocol, needs a job kind in the schema, adoption of `ce` leaves, a hand-off so that 15 owners waiting on one recipe wait on the child, and a differential law (entry from the child equals entry from the parent on a fixture workspace). Not started.
- A real `repo` build command through the changed path: `repo cache status` boots and prints (exit 0 of the pipeline) but it does not exercise the owners wave; the changed modules were exercised by the harness (real owners, real gate and sampler) only.
- Linux and Windows: the code is platform neutral, nothing was run there.

## 7. For the other packages

- Anything that wants a number from the heap in the hot path must pay for it by wall time, not by event count: use the duty-share pattern (cost times (100 - d) / d) instead of a fixed interval.
- `observe=` and `refs=` in the owner lines are wall clocks inside a shared thread; do not read them as per-owner work. The sum of the `loop lag` fields of the `sample step` rows divided by the span is the honest utilisation figure (65 % lag in the server read, trace 17:47 to 18:57 UTC).
- Harness: `measure-parent-path.mjs` (infers N real owners with `lateClosures`, then runs the parent path `observeProjectInferenceV1` + `resolveClosureRefsV1` over them; `R57_WARM=n` repeats the last owner's refs; pass a row log to match the wave, the non-log branch is 50 to 100 times slower and misleading).

---

# Follow-up (coordinator: the read was killed by our own watchdog at 18:59:07 UTC)

Second landing (entry "Package R57 landing 2" in `T/landings.md`, script `T/🧰️scripts/R57/land3.py`). The read note was `read-cancel-1859.md`.

## (1) Watchdog: any forward progress is progress, and a stop says why first (landed)

- Cause confirmed in code: `createGraphReadWatchV1` (`B/🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts`) took only a higher generation number or a "paused after" line as progress. The owners wave publishes no generation until it ends, so a read advancing at 60 owners/min was "stalled" after 25 min.
- Now progress is also any counter of a `"resource":"build:graph-phase:<phase>:n1:n2:..."` line of the read's stderr that passes the highest value seen for its phase (owners finished, facts verified, directories and files stamped, closures verified, doubt, membership); a counter that went back (running owners 15 to 9) or stands still does not count; a phase seen first counts. `counters()` returns the highest values.
- A stop first writes `read-stop.json` next to the read record (schema `ReadStop` in the wanted schema: verdict, silent milliseconds, newest generation, all counters, runner and read pid) and says the same through `say` (the sidecar log); only then SIGINT. I did not write into `read-record.json` itself: the runner owns and rewrites it every beat, so a second writer would race; the stop row is its own file, removed by the next start.
- The texts changed to "nothing advanced for N min (no wave, no counter of any phase)" and the German twin.
- Law tests (red before, green after): `R57 law: a read that finishes owners but publishes no wave for longer than the stall limit is alive; a read whose counters stand still is stopped` (60 simulated minutes of owners finished, no manifest: `go`; 24 minutes of frozen or falling counters: `go`; 25 minutes: `stalled`; one more owner: `go`; red on the old module: `stalled`), and the stop-row law (equals the schema). The old law and the sidecar law that asserted the old text were updated.
- Limits: the absolute maximum (12 h) and the memory-overrun rule are unchanged. A read whose parent is blocked and prints nothing for 25 minutes is still stopped, with the reason now written.
- Rollout caveat: the watchdog runs in the sidecar, not in the detached read. The sidecar picks up the new code "when the serve in flight is done (at most 10 min)" (build-agent.log 19:14), then adopts the read, whose watch starts with a fresh stall window. The read started at 19:00 was under the OLD rule until that restart; I could not see the handover happen before reporting.

## (2) Evidence and work

- Done: `read.log` is no longer truncated. `rotateGraphReadLogV1` renames it to `read.<mtime>.log` before a new read starts and keeps the newest `readLogsKept` (5, declared in the wanted schema, law `a new read never truncates the log of the previous one`). Footprint traces were already per pid.
- Measured, not fixed: how much work survives a dead read. In the selection's record directory (`.cache/b/g/m/inference/selections/8d8f…/np`) there are 1 183 owner records; modified in the 3 hours of the dead read: **39** (`find -mmin 180`), against 2 745 owners finished. So almost nothing the dead read finished is in the local store. The optional-leaf ledger (`optional.sqlite`) shows 562.8 MB used of a 1 073.7 MB maximum (low-water 512 MiB), the image has 4.0 GiB free: the quota is not full, so I cannot name the reason (declined optional publication under the low-water/retirement rule, the `current()` and `racy` conditions before a record is written, or the owners were taken from the fleet and never written locally); the decline counters are not persisted and the dead read's log is gone. The workers' records live in the fleet cache and the farm children's leaves are adopted, so a new read may still take many without inference; the number is **not known**.
- How to get the number: the new read (runner 80989, read 80992, eval child 81610) was still in `graph.plan.settle` at 19:15 UTC and had published the owner-pool manifest `7c9e70db1a7e` (4 775 owners) at 19:12; its owners phase had not started when I stopped. At its end the publisher writes `pool.json` (`consumedByHost`, owners per minute) and the farm prints `[graph-farm] {... adopted ...}`; inferred = `onInferred(missed.size)`. Owners taken without inference = owners (5 101) minus missed. Check them with `python3 T/🧰️scripts/R49/evidence.py` and the trace `semio-footprint/81610.log`.
- Not done: raising the optional quota or persisting records another way. R59 changes the record form (digest references instead of gigabyte records), which is the real fix for the size of what has to be kept; I did not touch records or quotas.

## (3) Shared closure entries off the parent: NOT built

What I verified: every derivation of a late closure entry runs `produceClosureEntryV1` in the parent (`B/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs`, around line 1311), deduplicated per recipe by `closureInflightV1`, so 15 owners asking `resolve` for target-specific recipes (`native-target`, `generic-target`: hundreds for `repo-policy`) all wait on derivations that run one after the other on the parent's thread; `primeSharedClosuresV1` primes only the recipes that no target decides (`native-command`, provider ports, tool closures). The R53 walk farm (`closure-kinds/🏭️walk-farm`) answers questions of the derived nodes, not these entries.
Why I stopped: the hand-over of an entry from a child has no channel today. The project farm adopts staged leaves through `adoptStagedGraphLeavesV1`, which admits only the optional leaf names (`np/ip/rr/...`, schema `OptionalPath`); closure entries (`ce`, 2.7 GB in this selection) are written by `createClosureStoreV1` under their own publication rules, and a child's store is read-only by design (`bindClosureStoreV1`). Doing it needs (a) a `closure` job kind in the farm worker/service that runs `ensureClosureEntryV1` into a writable private `ce` of the staging, (b) a declared adoption path for `ce` leaves (or `closureTravelV1` bundles over the reply), (c) a parent hook in `produceClosureEntryV1` that asks the farm first and falls back to deriving itself, loading and verifying the adopted entry cooperatively, (d) a differential law (entry from the child equals the parent's on a fixture workspace, same rows and digests), and it touches the farm protocol and the storage rules that R59 is changing. I judged a half-built channel in the parent's hot path worse than none. The measured basis for doing it: first derivations of one closure cost 2 to 52 s of parent thread (7 owners, `measure-parent-path.mjs`), 1 691 s of `resolve-closure-refs` for `repo-policy` in the dead read, while farm children needed 16 to 70 s per owner.

## Proven and unproven in this follow-up

- Proven by tests that ran green on the landed tree: the watchdog counts owner progress and still stops frozen counters; the stop row equals its schema; logs are rotated and capped. Standing preflight and heavy-questions green; wanted, wanted/server, wave-rows, sidecar read/graph/stall/wave-* green; one known-red test (tree/overrun) red on the untouched tree too.
- Unproven: that the next read is not stopped (needs a real run longer than 25 minutes without a wave under the new code), the handover of the running read to the new watchdog, how many owners the new read takes without inference, the cause of the 39-of-2 745 persistence, any effect of (3) (not built).
