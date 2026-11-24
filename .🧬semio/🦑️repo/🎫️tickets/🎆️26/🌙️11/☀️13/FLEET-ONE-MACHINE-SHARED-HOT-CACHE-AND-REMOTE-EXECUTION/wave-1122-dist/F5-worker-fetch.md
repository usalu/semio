# F5: why worker fetch took 50-60 s per remote unit (2026-11-24 12:05)

## Cause (evidence)
- The worker blob cache bound is `remoteExecution.worker.maximumBlobBytes` = 268435456 (256 MiB, `LIB/🔣️policy.json:574`, unchanged since 2026-10-23). `RemoteExecutionBlobCacheV1.prune` (`LIB/🌐️remote-execution/👷️worker/🟦️.ts`, called at start and every 30 s after an action) evicts least recently used blobs down to 90 % of the bound, also blobs of a running action.
- The input closure of the release units grew: median `bytesShipped` per remote unit in the play log 83 MB (10:30) -> 498 MB (11:00) -> 576 MB (11:30), 527-778 MB for every unit 11:42-11:57. A closure above the bound can never stay cached, so every action re-downloads (almost) its whole closure: fetch time 1.0 s (10:30) -> 18 s -> 24 s -> 48 s median, effective 7-33 MB/s (median 13 MB/s) with 11 workers pulling at once. Not a fixed wait: the constancy is closure size / shared bandwidth.
- Server is not the cause: from the coordinator 10 sequential real GETs 3-80 ms (50 MB in 80 ms), 40 parallel 131 ms total; the sidecar logged no stall >= 400 ms after the 11:52 restart (last `readCasTransientsV1` stalls 10:56, before CAS6's reader change; GET path never touches transients, only HEAD with a lease header does).
- "away from it" = `marks[2]-marks[1]-phases.workerMilliseconds` (`LIB/🌐️remote-execution/🚀️offload/🟦️.ts:159`): submit-to-answer time minus the worker's own time, i.e. scheduler queue wait + dispatch. The ~40 s is queueing behind workers whose slots sat in 50 s fetches. The 6 queue-timeouts at ~122 s (11:44:46-11:45:18, procedural_3d finals) are `waitMs=queueSaturatedMilliseconds`/pin queue (`🚀️offload/🟦️.ts:120`) + 2 s: every slot was busy, consistent with fetch occupation; I did not find the scheduler holding a unit while advertising an idle slot.
- Also found: directories of the input tree were fetched over HTTP for every action (never cached), and file placement concurrency multiplied per nesting level (16 files x 8 sub-directories x depth), unbounded in practice.

## Fix (landed 12:02, see landings.md)
1. `maximumBlobBytes` 256 MiB -> 12 GiB (schema max allows 1 PiB).
2. `prune` never removes a blob used (mtime, refreshed on every hit) within the last 10 minutes: a bound below one closure cannot cause refetching of the running/prepared action's inputs.
3. Directories go through the local blob cache (`blobs.bytes`), so a repeat tree costs no request.
4. One 32-wide gate for all placements of an action (replaces nested unbounded fan-out).
Existing behaviour kept: existence check by digest is the local stat, hits are cloned (APFS) into the private root.
Server: in-memory/hot path already clean after CAS6 (no per-request parsing on GET); new test proves 64 parallel GETs beside 300 leased uploads answer within bounds.

## Tests
worker 13/0 (new second-action test: after the first action and a prune under a 1 KiB bound, the second action sharing 99 inputs issues <= 8 requests), server admission 2/0 (new parallel test), remote-execution main 17/0, disk 8/0 (one flake under load, green on rerun).

## Before / after
Before (play log, successful remote units 11:42-11:57, n=24): median total 94.7 s, median worker fetch 50.4 s. 11:57-12:02 (n=22, before my code runs anywhere): median total 17.8 s, fetch 9.5 s (smaller closures from other lanes / warm caches; not my change).
After: gated by worker self-restart (5 quiet minutes after the code-digest change at 12:02, so from about 12:07; workers restart themselves when idle-quiet, the sidecar hands over). Re-measure with the read-only count of `offload.jsonl` after that time; not yet available at report time.
