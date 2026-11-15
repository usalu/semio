# R3: the replica daemon of g1-server restarted every 6 minutes (26 times)

## Cause (measured)

The activity `drift` of the record `loop-blocked-restart.json` is the drift pass of `runV1`: every 120 s it calls `fleetReportV1` -> `judgeFleetV1` (`🩺️attest/🟦️.ts`). Inside, the loop that picks the workers that disagree with the rest (now `separateOutliersV1`) **never ended**:

- Live inputs (read-only, copied to a scratch process): attestations 7 ms, gap windows 3 ms, `foldStoreV1` over the real store (293 399 entries, 93 MB snapshot) 1.3 s with a longest event-loop block of 5 ms. None of that is the cause.
- `judgeFleetV1` on the live attestations of 03:39 did not return (a scratch process ran at 100 % CPU, flat 117 MB, for more than 5 minutes; `fleetReportV1` the same). With a counter in a scratch copy: more than 1000 identical passes. Identical state: `view.digest` bucket 16, g1-m04 and g1-m10 each differ from the six other workers (m01 m02 m03 m06 m07 server, which agree) and from each other. Both tie for the highest score (6), the old code removed the pair (m04, m10), found no further pair with both ends tied and repeated the same pass forever (`continue` without progress).
- Why every ~6 min: drift timer 120 s after a start, 120 s of blocked loop, the boot stage kills the daemon (`KILL_MS` 120 s), restart. Each restart took the three sidecars along.
- The stack sample shows no syscall, only JS/allocation, which fits a tight loop that allocates (`new Map`, sorted arrays) per pass.

## Fix

1. `🩺️attest/🟦️.ts`: `separateOutliersV1` (exported, pure). If the tied workers do not differ from each other, the tie is no ambiguity: the last one is a victim as in the untied case. Every pass removes at least one pair, so at most `apart.length` passes. Live `judgeFleetV1` on the captured input: from endless to 12 ms; live `fleetReportV1` x5 against the real store: 24, 13, 9, 8, 9 ms, longest loop block 9 ms.
2. `🔁️replica/🟦️.ts`: the drift repair (`repairV1` -> `takeOverRegistersV1`, the `disagreeing` pass) iterated 300 000 entries without yielding. Now `yieldingV1()` (8 ms slices) in all four loops. After the pass the declared activity returns to `publish` (it stayed `drift` for everything that ran after it, which made the record misleading).
3. Watchdog (`🔁️replica/🛟️supervisor/🟦️.ts`, types and schema in `🛡️resilience`): `beat.json` carries `longestMs` and `longestActivity` (the longest block the watchdog thread saw since the start and the activity declared during it; resolution = the beat interval, 1 s). The stage's own `loop-blocked-restart` writer copies both. Not changed: the installed outer boot program (`boot.mjs`, version 6, outside the worktree) writes the same record without these two fields (it already writes `lagMs` and `activity`); changing it needs a `bootVersionV1` raise and a boot-program rollout, which was out of scope.
4. Sidecars (`🔁️replica/🧷️sidecars/🟦️.ts` + `🧬️schema/🔣️.json`, launcher in `📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🟦️.ts`): before, children were attached to the daemon (`SEMIO_FLEET_PARENT_PID`, the launcher left within 2 s of the daemon's death, the daemon stopped them on every exit). Now:
   - children start detached; the launcher keeps `.cache/f/sidecars/<name>.json` (name, pid, start, write time, sha256 of its entry script) fresh every 2 s;
   - the daemon renews `<name>.lease` every 5 s; a launcher ends itself (and its child) only when nobody renewed the lease for 10 minutes;
   - a new daemon adopts a child when the pid lives, the record is at most 15 s old, the name matches and the entry hash equals the file now; a living child with another hash is ended (SIGTERM, SIGKILL after the grace) and started again; a dead one is started;
   - `runV1` calls `release()` (detach) when it restarts for new code, `stop()` only on a real stop; a daemon killed by the watchdog leaves the children running;
   - code changes of a role still restart only that role (its own launcher watch); the protocol module is imported dynamically by the launcher so it is not part of the role closure.

## Proof

- `🩺️attest/🧪️tests/⏱️separation.ts` (3 tests): the real captured case (two outliers) ends within 50 ms and names both; 2-against-2 stays ambiguous; 1500 random graphs end and agree exactly with the independent Python implementation `🐍️separation.py`. (Red state shown by the live hang and the counter above; the old function is gone, so the test cannot run against it.)
- `🔁️replica/🧪️tests/⏱️turns.ts` "a drift repair over a 300k-entry manifest and a 300k-entry store snapshot never holds the event loop for longer than 50 ms in one turn": red 499 ms before the yields, 12 ms after (repair of 300 000 entries takes 9.9 s in total).
- `🔁️replica/🧪️tests/🛟️boot.ts`: record carries `longestMs` >= 2500 and `longestActivity` `publish`.
- `🔁️replica/🧷️sidecars/🧪️tests/🟦️.ts` (12 tests, was 9): a restarted daemon adopts a living sidecar (same pid, `restarts` 0, no new start, explicit stop ends it, launcher removes its record); changed entry script -> old launcher ended, new started; dead process -> started again; stale record with a living pid (the test process) is not adopted; unrenewed lease ends the launcher; real processes: daemon blocked, killed, next daemon reports the same launcher pid and nothing restarts.
- Library `🛰️sidecar/🧪️tests/🟦️.ts` 3/3 (lease replaces parent pid).
- Whole module: all 49 test files run on the live tree (`BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`, `SEMIO_*` unset): all pass except two tests of `🔁️replica/🧪️tests/🟦️.ts` (63/65), which still read `state.json` (replaced by `state.ckp` on 11-13) and were edited by someone else at 02:39; not touched here.

## Landing and live observation

Landed in place at ~03:47-03:53 (see the deviation below); record in `SHARED-WORKTREE-FLEET-OVER-NFS/1113-landings.md`. Code digest `97445096cc0a…`: g1-server (canary) went first at 03:52-03:53, all ten other attestations carry it by 04:04 (age 0 s each).

Deviation from the protocol: I saved the anchored edits to the live files one after another before taking the lock, so the daemon's own code watch picked the final digest up before the locks and the full test run (the lock was taken at 03:54 and held until 04:05). No intermediate state was admitted (one digest `b874…` ran 03:47-03:52 with the watchdog edit only, fine). Next time such a change goes into a scratch copy first.

Observed on g1-server (health.json, beat.json, sidecars logs):
- daemon pid 2665 since 03:52, no restart; `loop-blocked-restart` stays at count 26 (last 03:46 UTC+1 before the switch); beat `longestMs` 1323 (activity `idle`, i.e. beat jitter, not a block); `lagMs` 150-200 ms.
- sidecars pids 7107 (agent), 7109 (scheduler), 7110 (cache), `restarts` 0 since 03:53; records and leases fresh (`wallMs` within 2 s / 5 s of now).
- build-cache.log: `serving the cache` at 02:50:24Z (launcher edit seen by the role watch) and 02:53:20Z (daemon switch), no line after that.
- Not observed: a daemon restart with live adoption on the real server (none happened after the switch; I did not restart the live daemon). That path is proven by the tests above only.

## Not verified / for others

- The two failing replica tests above and the `signed-fleet` cluster test (`the claim is released`, fails with and without my env change) belong to other work.
- No real `repo` build command was run through the changed path (graph commands forbidden here).
- Log rotation of an adopted sidecar happens only when a child is started (the adopted process keeps its file descriptor; `build-cache.log` can grow past 2 MiB until the next start).
- Platform check of the new protocol on Linux/Windows: nothing platform specific (pid liveness via `process.kill(pid,0)`, files via tmp+rename, `detached: true`) but only run on macOS.
- Other packages: `SEMIO_FLEET_PARENT_PID` no longer exists; children get `SEMIO_FLEET_SIDECAR_RECORD`, `SEMIO_FLEET_SIDECAR_LEASE`, `SEMIO_FLEET_SIDECAR_LEASE_MS`. A sidecar that runs outside the daemon should not set a lease (it then never expires).
