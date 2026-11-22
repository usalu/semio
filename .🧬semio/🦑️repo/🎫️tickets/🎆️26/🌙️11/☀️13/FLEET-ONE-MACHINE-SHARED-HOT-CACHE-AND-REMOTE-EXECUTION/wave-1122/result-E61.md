# Result E61 (slice-loop-lag + watchdog), 15:45 CET

## 1. Watchdog 8 of 9 red: fixed (landed, T/landings.md "Wave 1122 E61")
Not caused by E55's code-stop (its diff only touches the code-stop branch). Cause: `runOwnedReadyStageV1` built `readyGraphCounterTrackerV1(treePolicy)` without a prior on every adoption, so on tick 1 (250 ms) the already-logged `taken 1 of 9` counted as an advance and called `moved()` (stallAt +258..290 ms, the exact red delta). Real consequence: every supervisor restart (code lands every few minutes) reset the quiet clock of a stalled read once. Second bug on the same line: `tracker.observe(recent)||graphCounter.observe(recent)` short-circuited, so the counter lagged a poll (found with a scratch copy of the test dumping the records after each detach: counter stayed 1 while new lines were seen).
Fix in `🚦️ready/🟦️.ts`: counter restored from the watchdog record (`graphCounter`, optional integer in the ready schema), seeded from the tail on a first adoption (log mtime rule as for lines), written at every checkpoint, both trackers always observed. Test file: the semantics row sets `codeQuietMilliseconds` 900 (E55's rule needs a quiet stage). Law: a progressing stage is never stopped for new code (code-stop 4/0, stale-stage 14/0) and a stalled stage is detected through adoptions (watchdog 9/0, was 1/8).
Other tests: ready main 57/0, graph-progress 3/0, process-tree 16/0, process-tree watchdog 2/0. tools-grace handoff 0/1: times out at its 29 s ttl under load 27-60; log has no `taken` line and its watchdog record exists, so the changed path is not reached; baseline not re-run (no revert in the replicated tree); its fixture is a peer's uncommitted edit.

## 2. Slice "loop lag": measured, premise corrected, NO product fix landed
Measurements (slice-memory samples are written by a 30 s timer, so the gap between consecutive samples is the timer's lateness; polled every 5 s for 7 min, 15:37-15:44):
| worker | sample gaps (s) | slice CPU / uptime |
|---|---|---|
| m06 | 27-34, 14 samples, steady | n/a |
| m01 | 30,31,29,58,59 | 6.2 % |
| m10 | 178,33,30,41,57,72 | 11.9 % |
| m08 | 1114,166,42,134 | 10.0 % |
| m12 | 1 sample in 7 min (last 15:33:03) | 12.0 % |
- The loop IS late on m08/m10/m12 (timer 30 s fires after 1114 s on m08), but the process spends only 6-12 % of its wall time on CPU. So JS compute cannot be what blocks it: the process is not scheduled or is paged out (16 GB Macs: stage tree peak 10.2 GB on m08 plus a pool puller with 5 slots of ~1 GB children, swap in use). m06 and m01 (no puller pressure at the time) are on time.
- m01's metadata calls average 15.5 s (284 calls, 4412 s) while its timer is on time: that wait is real fleet wait, not loop lag. E55's "event loop blocked by synchronous work" is therefore not the general cause; late timers on m08/m10/m12 are starvation.
- Checked and cleared as blockers: report build (`heapStats` 160 ms on a 219 MB synthetic heap, `memoryUsage` 0 ms, JSON of 900k objects 115 ms: 0.5 s every 30 s at most); the 2 s "Reading the project graph slice" lines come from the host process, not the slice, so they say nothing about it.
- Not done: before/after loop-lag numbers (no code change, because the cause is machine starvation, not a chunkable loop). A reproduction on this Mac is impossible: no small selected read can be started (load 27-60, whole read running, harness inputs of R87 gone).

## 3. Live check after rollout: not possible yet
At 15:44 every slice process started before 15:20 (m01 13:57, m06 14:27, m08 14:47, m10 14:45, m12 14:49), so none runs E55's late-timeout retry yet. Cumulative old-code counters: m01 looks 557 / unavailable 277, m08 1026 / 830, m10 766 / 213, m12 532 / 21 (m12 last sample 15:33). Compare again once a sample has `atMs - uptimeSeconds` after 15:20.

## NOT verified
Whether the E55 retry raises hit rate; whether a worker reaches Ready; handoff test baseline.

## What others must know
- Lever for the lateness is machine memory/CPU admission on 16 GB Macs (stage 10 GB peak + puller children together), owned by the worker admission lane (E56), not a loop-yield in the slice.
- Host load is 60 at 15:44 (above the 40 gate): I stopped heavy runs.
