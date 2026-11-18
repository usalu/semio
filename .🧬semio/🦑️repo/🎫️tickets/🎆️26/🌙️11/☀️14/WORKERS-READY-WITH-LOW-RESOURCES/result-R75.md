# R75: owner pool throughput (measured 04:26 to 05:13 UTC, landed 04:41)

No worker reached `ready: true` (ready table 05:12). The fleet rate did NOT recover in the 30 min watch (see Measured after). Scripts: `T/🧰️scripts/R75/` (`stage.py`, `land.py`, `snap.py`, `snap-*.txt`, `fail-before.txt`).

## Measured before
- pool.json 04:33: g1-m01/m03/server `nothing-fits` with 1 child (m01 4); m02/m10 13-15 children, target 0 (`pressure`), 3-4 running; m06/m07/m08 6-10 children, 0-4 running. 2 to 3 owners/min per machine, `stored` 6 to 17 times `published` (a puller walks mostly over records that are already stored, so a low published rate does not mean a low claim rate; the share of the wave still unstored was NOT measured).
- Server host view (this machine, `readRemoteExecutionHostViewV1`): available 15.6 GB, but grants `show` reserved 14.3 GB observed 2.7 GB and seven `graph-farm` grants of 1.5 GB with `observedMegabytes` 0 forever. The pool subtracts reserved minus observed in full: 22 GB "unrealized" against 15.6 GB free, so no child fits. A worker (g1-m01, worker log) reports 75 % system memory free while the pool says `nothing-fits`: the same mechanism.
- Child price: `childMegabytes` flaps 524 .. 2158 .. 2774 between machines and minutes (maximum of the last 5 samples x 1.2): m10 priced 524 MB, started 13 children, hit `pressure`; m07/m08 priced 2 to 2.7 GB after one giant owner, so only 1 to 2 children.
- Child start is NOT expensive: a real child (probe on the server, load 29) is ready in 993 ms at 232 MB. An owner takes it to 1.1 to 1.4 GB (+1 GB). The server's own child sat 22 min in one owner at 70 % CPU (7 CPU-min, stack in `lstat`), a heavy-tailed owner cost.
- Churn: the pool restarts on every edit of its closure (pool quiet 5 s, max 30 s): `🏗️build/📖️texts/🔣️.json` was edited every 30 to 60 s at 04:59 to 05:02 and restarted the pool 6 times in 3 min; five more restarts per hour from code landings (R73, R74, R75 itself). Wave ids change too (04011a -> 4783e0). After each restart pub counters are 0 and children (up to 9 per machine) are all still opening. At `restartPending` the runner sets target 0 (reason `null`): 7 of 10 machines showed `children target 0, reason None` at 04:42 and 05:01.
- g1-m04 (Linux) saw only wave 4d9db5 with `EDQUOT: unknown error, write` as lastError at 04:33; after the landing it sees 04011a/4783e0 (lastError `None`), but still publishes 0 (1 child, 1 running, `yielding`).

## Causes (ranked)
1. Pool restarts and wave churn (above): measured, NOT fixed (see Not done).
2. Memory pricing: unmeasured farm grants and the machine's own reservations counted in full (measured on the server, argued on workers); child price jumpy (measured).
3. Heavy-tailed owner cost under load 30 to 36 on 8 to 10 cores at nice 10 (readiness stage, git storms of the replica daemon): per-owner wall time in minutes (server child). Not changed.
4. Child start cost (hypothesis 4/(1)): refuted, 1 s.
5. m04: wave files written to the temp dir failed with EDQUOT (cause of the missing wave read from lastError and code; not reproduced).

## Fixes landed (04:38 to 04:41 UTC, one lock hold, `land.py`; 9 files, none in a tool closure: both standing preflight tests green)
- `ownerPoolChildMegabytesV1`: 75th percentile (nearest rank) of the last 32 child sizes plus 15 % instead of the maximum of the last 5 plus 20 %; schema `peakSamples 32, peakPercentile 75, peakMarginPercent 15`.
- `ownerPoolHostOfV1` (sidecar `🏊️pool`): a `graph-farm` grant's untouched reservation counts `farmUnrealizedShareBasisPoints` 2500 (25 %); the machine's own grants in full.
- `ownerPoolRecycleV1`: a child is recycled after an owner only when it passed the limit AND grew 600 MB past its ready size (`recycleGrowthMegabytes`). Low value (child start is 1 s), but harmless.
- `writeOwnerPoolWaveFilesV1`: wave files go to the next directory (`.cache/f/owner-pool-waves/<id>`) when the temp dir fails (EDQUOT, ENOSPC); wave dirs are removed when the pool ends.
- Fixture bug found: the owner-pool fixture policy lacked `yieldAfterMilliseconds`/`yieldUntilMilliseconds`, so the Python oracle crashed and the law skipped it silently (`if(python.status===0)`). Fixed; the law now requires the oracle (exit 0) and its output equal to ours.

## Laws, red before / green after (measured under the lock, new tests against old code)
- Red before: sidecar `🏊️pool` "a farmed owner that holds a reservation nobody measures..." and the owner-pool test file (import of the new exports fails; new fixture/oracle cases for the percentile price and the recycle rule, flap case [524, 2100, 2200, ...] -> 610 not 2640; wave-file fallback with injected EDQUOT).
- Green after: owner-pool tests, sidecar `🏊️pool`, sidecar `🧪️tests`, both standing preflight tests. Entry in `landings.md` (R75).

## Measured after (pool.json, fleet owners/min = sum of ownersPerMinute, 5-min window)
04:42 11.2 (restart), 04:46 13.0, 04:49 8.2, 04:55 19.0, 05:01 3.0 (pool restarts by the texts.json edits and new wave 4783e0), 05:07 0.8, 05:12 4.8. Before (04:33) about 17. Not improved; the numbers are dominated by restarts. `nothing-fits` is still shown by m03, m08 and the server after the landing, so the memory price/unrealized rule alone does not free memory there (their host views cannot be read from the server; only the server's own view was read).

## Not done, proposed next (the real lever)
1. Do not restart the pool for edits that cannot change what a child computes (texts.json) and do not stop children at `restartPending`: take the pool role out of the `restartPending` target-0 rule, restart only when due with a quiet period of 90 s like the other roles (the pool has 5 s / 30 s), and keep children alive across the supervisor restart (adopt them) or give them the full owner time instead of 20 s drain grace before SIGKILL.
2. Stop publishing a new wave id for unrelated landings (wave ids changed 4 times in 1.5 h; every change makes every machine reload 25 MB units and restart its children).
3. Measure the unstored remainder of the wave (claim sample with abandon) before sizing anything else: `stored` is 6 to 17 times `published`, so the refill may be much smaller than 5 000.
4. Owner cost under load: per-owner wall time in minutes on loaded hosts; the pool runs at nice 10 behind the readiness stage by design (R74), so its rate is whatever the stage leaves.

## Proven / not proven
Proven: the four laws above (red before, green after, oracle equal), no tool closure touched, the diagnosis of restart churn from the sidecar log and pool.json, child open 1 s. Not proven: any throughput gain; that the farm 25 % rule frees memory on a worker; that the EDQUOT fallback fixes m04 (cause inferred from lastError); the remaining unstored share; claim spread/duplicates between machines (not examined); Linux and Windows pool behaviour.

# R75c: resume after the coordinator crash (12:20 to 12:40 UTC)
Scripts: `T/🧰️scripts/R75c/` (`snap.py` sampler, `land.py`). Landed once under `landing.lock` at 13:3x local (tests under the lock: owner-pool 29 green, sidecar pool, sidecar, both standing preflight tests; entry in `landings.md`). Members were edited live before the lock (own members only).

## Why seven machines sat at `no-wave` for 7 hours (found, fixed)
1. The pointer drops every wave after `manifestMaximumAgeMilliseconds` (3 h). The last wave was published 05:33; at 08:33 the pointer was empty for everybody. The runner then deleted its wave (`!ids.has(id) && children===0`) and kept nothing on disk (wave files in a temp dir, removed when the pool ends), so `no-wave` followed even though every record of that wave was still valid.
2. The slice readers that inferred 10 GB locally never publish: the publisher and the fleet port are created only `if(!options?.ownerAdmission)` (project-nodes), and a slice reader infers the selected targets of a slice, so its units/record keys differ from the full reader's. Letting them publish as a wave would publish owners the pool children cannot reproduce; NOT changed. A worker therefore had no way to start a wave on its own.
3. Fix (law red before, green after): `ownerPoolKeepWaveV1` (a wave stays while the pointer lists none; it goes only when the pointer lists others or its revision declined it, and never while a child works on it), wave files live in `.cache/f/owner-pool-waves/<id16>` and stay when the pool ends, the runner adopts the newest two at start (`adopt`), a wave older than the pointer age reopens every 10 min (`staleReopenMilliseconds`, `ownerPoolReopenMillisecondsV1`) instead of every minute. Laws: pure keep/reopen law; real-process law "a pool whose pointer has no wave works the wave it kept on disk and does not delete it" (12 owners all stored). Fail-before is by construction (old runner reads only the pointer and removed unlisted waves; the new exports do not exist in the old code); I did NOT run the old code against the new tests.
4. Also fixed: the child registry counted any live pid, so a reused pid (typical after a day on Windows/Linux) counted as a running child and held `have+passed < target` back. `ownerPoolLiveChildrenV1` now ignores rows older than child lifetime plus drain (40 min) and the runner deletes them. Suspected, NOT proven, for g1-m05 (Windows): it showed `children 0, target 1, reason yielding, claimAttempts 0` from 12:17 to 12:28 and then started a child.

## Measured 12:24 to 12:35 UTC (11 of 30 min; sampler continues to 13:05 in `🗑️generated/snap.txt`, one row per minute)
- The server read published wave 25d571 (4 494 owners) at about 12:17, a new wave aa2494 at about 12:30. All pools adopted it within a poll.
- Fleet owners/min (sum of `ownersPerMinute`, 5-min window): 1.0 .. 2.8, mean about 1.6. Far below 100/min.
- Children vs cores: 5 to 56 children fleet-wide (counts swing as children end after `doneJumps`), `running` only 1 to 5. Machines show 7 to 11 children with target 7 to 11 and 0 running: the children are walking, not computing.
- Per puller (about 100 to 560 attempts each): stored about 70 to 80 %, held about 20 to 25 %, granted 1 to 3 %. So the unstored remainder of the wave was NOT 4 494; roughly three quarters of the lookups are already stored, a quarter are held by another claim, and only a few percent were free. The rate is limited by lack of claimable work, not by children or memory. The `held` quarter (about 1 000 owners if representative) is more than the 1 to 5 running owners can explain: lapsed claims of killed children (TTL 5 min), or claims of the server read; NOT examined.
- Restarts: all 9 pools restarted together at 12:17 (R78) and again at 12:27 (a landing of another package); none between 12:27 and 12:35 (the 90 s quiet handover held). Children survived. Wasted claims: not measurable from pool.json.
- g1-m06: `pool.json` is 10 400 s old (started 08:57, wave 1dd2d9, `nothing-fits`, 1 child): its supervisor is hung or dead since about 09:40. Not examined further (no access to that host from here).

## Not done
- 100 owners/min and one child per core cannot be shown while the wave is mostly stored: it needs a cold wave. Next step when one appears: claim-sample the wave with abandon to count the free share, and count the holders of the `held` quarter.
- The new code is in the tree, but the pools started before the landing (12:27) run the old runner until their next handover.
- Slice readers still publish nothing (see 2); R76 owns their memory.
