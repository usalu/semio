# W11: what restarts a Ready stage when code lands, and the fix (2026-11-24, 17:46 to 19:10)

`B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`, `R` = `B🌐️remote-execution/🛰️sidecar/🚦️ready/`. Evidence read-only from `/Users/Shared/semio-store/v1/origins/g1-*/{ready.json,ready-timings.json,ready-log-workspace*.txt,slice-memory.json}`; `repo fleet status` on the coordinator timed out its worker/fleet readers (load 59 to 71), so the origins files are the source. Lock `lock.worker-ready` (released). Landings: `T/landings.md` "W11". Nothing restarted by hand, no scratch build, no sub-agent.

## 1. What restarts a stage when code changes (item 1)
Three candidates, checked one by one:
- **Stage supervisor code check (`R🟦️.ts:1134`, `readyCodeStopV1` + `readySemanticsChangedV1`)**: stops a stage only when a declared semantics source changed (`selected-exchange` revision, `observation` record stamp) AND (holder wait 10 min OR reading 30 min with 5 min quiet). Both sources were last written 11-23 (03:17 and 23:18): **0 code stops today**. Every other file edit never stops a stage (schema text of `R🌲️process-tree/🧬️schema/🔣️.json` says so; the code agrees).
- **Sidecar self-restart killing its child**: no. The `ready` role restarts for every change of its closure (quiet 90 s, max deferral 10 min, `🛰️sidecar/🧬️schema/🔣️.json` `Restart.ready`) but the stage runs under a detached wrapper and the next supervisor ADOPTS it (`running.json`, `R🟦️.ts:894-1153`). Every stall log of today names the adoption ("started … by supervisor A, adopted by supervisor B at T").
- **Graph read refusing across a tool-closure change**: not seen. The slice read forgets only owners whose inputs changed (W9: 13 of 639); graph reads run under a detached runner that also survives restarts.

**The real mechanism (root cause): the stall clock ran while nobody watched.** After a landing the old supervisor detaches, the launcher starts the new code, the new supervisor computes digests, memory, lease and decision, and only then adopts. Until today the adopter restored `stallAt` from the last checkpoint and judged `now - stallAt`, so the restart gap counted as quiet time; and `readyDecideV1` could even hold the adoption behind a memory wait (up to `memoryWaitMaximumMilliseconds` 10 min) or a back-off while the stage ran unwatched. Evidence (latest stall log per worker, before the fix):

| worker | stage started | adopted by new supervisor | stopped as stalled | stop after adoption |
|---|---|---|---|---|
| m08 | 16:10:48Z | 16:36:36Z | 16:36:43Z | 7 s |
| m01 | 16:33:09Z | 16:44:05Z | 16:44:31Z | 26 s (10 s probe + grace) |
| m10 | 16:14:41Z | 16:31:05Z | 16:32:15Z | 70 s |
| m12 | 16:16:50Z | 16:30:59Z | 16:32:51Z | 112 s |
| m03 | 16:10:05Z | 16:30:53Z | 16:33:43Z | 170 s |
| m06 | 09:08:06Z | 09:29:29Z | 09:32:01Z | 152 s |
| m11 | 16:00:08Z | 16:13:28Z | 16:24:37Z | 669 s |

A kill within seconds of adoption needs a `stallAt` already 10 min old, which the old supervisor (ticking every 250 ms) would have acted on itself had it been watching: the quiet minutes were the restart gap. Today's workspace stops (all `stalled`/SIGTERM, from `ready-timings.json`, 00:00 to 17:45): m01 37, m03 27, m06 15, m07 15, m08 28, m10 18, m11 27, m12 36; stage digest `d5981b7a5a72` unchanged all day. Only the newest stall log survives per worker, so how many of the ~200 were adoption-proximate cannot be counted; 7 of 7 latest are.

## 2. Landed for item 1 (`R🟦️.ts`, `R🧬️schema/🔣️.json`, schema first)
a. **Only watched time is quiet time.** The watchdog checkpoint carries `watchedAt` (written at least every `watchdogIdleWriteMilliseconds` 30 s while nothing changes, every `watchdogWriteMilliseconds` 10 s while something does, and at every detach) and `unwatchedMilliseconds`. On adoption `unwatched += now - prior.watchedAt`; any progress resets it; the stall stop is `now - stallAt - unwatched >= stallStopMilliseconds` (`R🟦️.ts` stall line next to `readyStalledV1`). `stallAt` itself is kept, so the law "quiet time survives repeated detaches" stays (the test of that still passes). The hard cap still counts from the start: a stage on old code is bounded by the stage hard cap (workspace 1 h soft x 8 = 8 h, `stageHardCapMultipliers`), and results that a new code would not accept are still stopped by the semantics-source rule.
b. **An in-flight stage is adopted at once** (`readyDecideV1`, input `inflight`): it is never held behind `memory-low` or a back-off (it already holds its memory; its back-off was served before it started). Human-running and a live foreign lease still observe.
c. Not changed, already in place: Ready stays Ready while a stale stage refreshes (`readyRefreshV1`, reason `refreshing` "Ready, refreshing stage {stage} in the background" EN + DE; m02 shows it now), remote actions are served by the `agent` role which is unaffected, settled owner records survive (GR16 law).
d. Not done: running the stage from a sealed code snapshot. The sealed implementation store exists (`B🏛️sealed`, `SEMIO_REPO_SEAL`) but its default mode is `live` and the stored entry is 703 h old with explicit promotion; switching the fleet's stage to it needs a promotion step per stage start and is its own lane. With (a) and (b) a landing no longer costs a stage; mixed live code inside one stage remains bounded by the semantics rule and the hard cap.

Tests (`repo test-file`, one at a time): `R🧪️tests/💓️watchdog` 10/0 (new case `gap`: no supervisor for longer than the stall window, the re-adopted stage is NOT stopped, `unwatchedMilliseconds` ≥ the gap, `stallAt` unchanged, and it is stopped once the WATCHED quiet reaches the window; the `quiet` case keeps the stop across three detaches). `R🧪️tests/🟦️.ts` 57/1, twice: the decide fixture with 2 new in-flight cases equals the Python oracle (`🧫️fixtures/🐍️oracle.py`, updated) and a named assertion; the 1 failure is "a silent stage that spends processor time is alive…": its bash busy loop got 0.45 s CPU per 5 s at load 71 (measured with `/usr/bin/time`), under the 0.5 s liveness threshold, so it is host load, not this change. Differential for `gap` not run on the old code (reverting would restart every sidecar again); by construction the old rule stops at the first tick after adoption (3 s quiet > 2.5 s window) and `detach()` would get a non-detached result.

## 3. Exclusive mapping writer (item 2): not landed
- The holder probe of W10 (`mappingQueue.head`, landed 17:36) has not published a holder on any worker during this lane: `head` null on m01/m03/m10/m11/m12 at 18:21, absent on m06/m07/m08 (older code). Long shared grant waits continue (m01 127 s and m11 120 s waiting at 18:00; finished maxima m12 95 s, m08 74 s; others 20 to 551 ms), with `holders.exclusive 0` in-process, i.e. a kernel holder in another process.
- The exclusive writers are `acquireNativeStorageMappingWriterV1` in `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts:200` (persistent child preparation, held until `releaseMapping()` after handoff), `:232` (funded child), `:243` (`retirePersistentNativeFundedChildV1`, held for the whole retirement), `🧱️fixed/🟦️.ts:187,250,261`, and the maintenance writer in `📤️publication/🌐️platform/🟦️.ts:50`. These are single 2 to 30 kB proof-carrying lines that attach, create and retire nested DMG images; splitting them needs tests that create and attach images, which the binding rule (no image attach/detach/resize) forbids in this wave, and without the holder's name it is unknown which of the five is the 160 to 192 s hold. Next step: read `mappingQueue.head.holderCommand` once a worker on current code waits, then split exactly that writer (prepare outside, exclusive only for attach/commit).

## 4. `🪪️presence` (item 3): when and why it broke, fixed
- Broke with the change between the archive commits `b4927d2c311` (tests last changed) and `01324745f80` (2026-11-22): `missingInventoryNamesV1` (`B👁️observation/📥️reuse/🟨️.mjs`) treats a stable leaf whose bytes are not its name (`BUILD_INVENTORY_CORRUPT`: hash mismatch or invalid UTF-8/JSON) as missing, so it is recomputed (self-healing content-addressed cache), and `inventoryNamesSettledV1` looks again (≤ 3) after a concurrent change. The law still expected `refuse` for `hash`, `cold-utf8`, `ancestor-replaced` and `absence-reappears` (the last two write `{}` during the look: the second look sees a stable corrupt leaf). The first wrong expectation closed the law's process admission, so the other 35 failed as consequences ("Original process admission closed - first failure …"), with or without W10.
- The same change added the `held` session path, which counted the `reader` phase twice (`started 2`); and W10's early release answers callers before the session's `postVerify` tail, so a census taken right after the answer saw `body` active and `postVerify` not started.
- Fixed for the right reason: corpus `🧫️cases.json`: those four faults are `recompute`; new law "inference reuse recomputes a <fault> <kind> inventory under managed custody" (status 0, warm, `calls` 2, no refusal, the oracle says the record was unusable, and for the two mid-look faults `mutated` plus the "looking again 1 of" line); the child supports `exact-leaf-absent` (removed before the look) and recompute rows. Code: `missingInventoryNamesV1` opens its own phase batch only without a held session. Telemetry laws wait (≤ 60 s) for the session tail before taking the census.
- Runs: 4/36 (start) -> 39/3 (corpus) -> 6/36 (an unrelated `Rust cache envelope disagrees with repository total budget` refusal from a peer's in-flight budget edit) -> 38/4 (reader phase fixed; telemetry tail; `settles a replaced namespace` hit its 180 s deadline at load 70) -> 41/0 (final).

## 5. Watch (18:05 to 19:05, read-only)
Ready count stayed 1 of 11 (m02; it refreshed `dependencies` at 18:52 and stayed Ready). No workspace stage ran to completion. My ready landing (17:58-18:04) restarted every `ready` sidecar between about 18:07 and 18:17 (pids in `ready.json` changed, reason `adopted`) under the old rule for the last time.
| worker | workspace endings 18:05-19:01 | furthest `taken` seen |
|---|---|---|
| m01 | one run of 4,213 s (17:45 to 18:55, survived the restart wave), ended `stage-timeout` (soft cap 1 h, idle) | 188 at 18:21, 338 of 608 at 18:50 (best of the day on m01; before: 15 to 85 per attempt) |
| m03 | none: one adopted run of 4,948 s still running | 52 (frozen since about 17:50) |
| m06 | stalled 18:20 (1,736 s), 18:42 (1,289 s) | 121 at 18:33 |
| m07 | stalled 18:16, 18:29 | round 1 |
| m08 | stalled 18:15, 18:39 | 58 at 19:01 |
| m10 | stalled 18:21 (2,936 s), 18:36 | 73 then 10 |
| m11 | `stage-timeout` 18:30 (3,945 s), then 2 `stage-failed` after a `dependencies` change | 93 |
| m12 | stalled 18:22, 18:56 (2,012 s) | 161 at 18:21 |
Reading: runs now last 30 to 80 min (m01 70 min, m03 82 min, m10 49 min, m12 34 min) instead of being cut within seconds of an adoption; the stops left are genuine (`taken` frozen for 10 watched minutes, or the 1 h soft cap with no progress), i.e. the look/grant cost of W9/W10. The adoption-proximity of the 18:15-18:56 stops could not be checked: the `-stalled` mirrors on the origins were last rewritten 17:22Z. At about 18:40 a `bun.lock`/`package.json` landing re-ran `dependencies` on every worker (input change, `ok` in 71 to 339 s), which ends a workspace run by design (its inputs changed).

## 6. Tests
- `R🧪️tests/💓️watchdog/🟦️.ts` 10/0 (new `gap`).
- `R🧪️tests/🟦️.ts` 57/1 twice (the 1 = busy-loop liveness under load 71, see section 2).
- `B👁️observation/🧪️tests/🪪️presence/🟦️.ts` 41/0 (from 4/36; 8 new recompute laws).
- `B👁️observation/🧪️tests/🧺️presence-batch/🟦️.ts` 19/0 (touches `📥️reuse/🟨️.mjs`).

## Open
- Item 2 needs the holder name (section 3).
- Genuine stalls remain: a take pass whose `taken` stands for 10 watched minutes is still stopped (m10 at 73, m03 at 52); that is the look cost/grant queue of W9/W10, not code landings.
- Sealed stage code (2d).
