# Owner read throughput: the "N of 3600" is seconds, and the real loss is the heap recycle at every pass boundary

Scope: `repo run-many --targets=test --projects=s-plugin-composition,gis-gisterrain,gis-gismap-js,gis-js` (pid 89217, `🗑️generated/excl-ts-test.log`, left running), service log `host-e129a0ba1fad.log`, warden state.

## 1. The premise: "164 of 3600 owners" is not an owner count
- `reading the owners (round 3) 164 of 3600` is the step line of `sliceReadStepV1("owners-round")`: `describeSliceReadStepV1` reports the seconds the step ran (`done`) and its declared bound 3600 s (`total`, `📋️slice-report/🧬️schema`, `owners-round.boundMilliseconds`). The count 164 to 322 over 120 s is the step clock of the worker (it freezes while the pool or the warden holds the worker with SIGSTOP: pid 56661 is in state `T`, RSS 26 MB, 18 min CPU in 16 min wall). Not owners per second.
- The real owner counts are in the service log: `[owners] owners 1373 of 1373 done: 1345 present, 9 fetched from the fleet, 19 inferred here`; the presence prefetch of 1246 records took 17.6 s. The owner read itself is not slow: when the owners are in the process memo, a whole second pass over all 1373 owners took 18 s in this run (`owners done 1373 of 1373` at 1412 s, `owners done (pass 2)` at 1430 s of the same worker).
- No per-owner process spawn, no per-owner git call: the slice worker (56661) has exactly one child (the discovery farm, 56674). The visible `git -c core.quotepath=false ... ls-files --others` children belong to other parents (Claude app helper 73712, fleet `--boot-child run` 15115). Lock contention: none found (the worker waits on the capacity gate, not on a lock).

## 2. The measured cause of 4 h 20 min: a handover (recycle) at every pass boundary
Sequence in `excl-ts-test.log`, twice:
1. The read finishes a pass; one path changed under it. `a new pass over the owners starts (round 1): 1 path(s) changed under the read, 0 owner(s) forgotten, live heap 2351 MB`, and in the second case `(round 2): 1 path(s), 33 owner(s) forgotten, live heap 2195 MB`.
2. `♻️pass-reuse` `boundary()` asks `sliceRecycleDecisionV1`, which looked only at the heap: 2351 MB and 2195 MB are above the 2048 MB threshold (the steady live heap of this read is 2.0 to 2.5 GB, the census shows 3.3 to 4.1 GB), so the worker ended itself: `the read continues in a fresh worker at the boundary between two passes (handover 1 of 3)`.
3. The fresh worker waits for the gate (60 s), then starts a cold inventory of the tree: every one of the 1415 owner checks and 435,000 verified memo rows again, and only then reaches the owner round.

Cost of one handover (time from the boundary to the first `reading the owners` line of the new worker):

| handover | forgotten owners | heap | gate wait | time to the first owner round |
|---|---|---|---|---|
| 1 (line 4542) | 0 of 1373 | 2351 MB | 60 s | 1151 s |
| 2 (line 5732) | 33 of 1373 | 2195 MB | 61 s | 1132 s |

Total 2283 s (38 min) for two handovers, each discarding 1340 to 1373 verified owners to re-observe 0 to 33. Together with the other restarts (warden cuts, service generations) the read never finished a pass in a tree that changes every second.

## 3. Fix (schema first)
- `🧠️memory/♻️recycle/🧬️schema/🔣️.json` Policy: `minimumEvictedBasisPoints` 2500 and `ceilingBasisPoints` 15000.
- `🧠️memory/♻️recycle/🟨️.mjs` `sliceRecycleDecisionV1({heapBytes, kept, evicted})`: above the heap threshold a handover happens only when the next pass observes at least 25 % of the owners again (`evicted * 10000 >= (kept + evicted) * minimumEvictedBasisPoints`, an empty memo counts as paying because nothing is lost), or when the heap is past 150 % of the threshold (3072 MB), so a growing heap still ends. `.d.mts` updated.
- `🕸️graph/📥️load/♻️pass-reuse/🟦️.ts` `boundary` passes the owners kept and forgotten to the decision. A boundary that forgets few owners (any change to one file) continues in the same worker with its trust set, its owner memo and its verified rows: only the forgotten owners are observed again. A change that touches the requested closure still forgets exactly the owners whose rows name it (existing `evictReferencing`), so correctness is unchanged: kept owners are verified row by row when the pass reaches them.
- Not changed: the number of handovers (3), the heap measurement, the memory release between owners, the capacity gate.

## 4. Tests
- `♻️recycle/🧪️tests/🟦️.ts` (5 pass, ran on g1-m11): the decision table over 8 heaps x 9 owner splits against an independent SQLite oracle, and the two live cases (0 of 1373 and 33 of 1373 stay, past the ceiling recycles).
- `📥️load/♻️pass-reuse/🧪️tests/🟦️.ts` (new, 3 pass, g1-m12): the live read stays and says its boundary line; a third of the owners forgotten hands over; past the ceiling hands over.
- `🧪️tests/🕸️graph/🩹️repair-rounds` 30 pass, 0 fail (recycle paths included; the two earlier failures no longer occur in the replica).

## 5. Before and after (owners per second at a pass boundary)
- Before (measured above): boundary to the first owner round of the new worker 1132 to 1151 s for 1373 owners, i.e. 1.2 owners/s, plus 60 s of gate wait, per handover.
- After: the boundary stays in the worker; the next pass is the in-process pass 2 measured in the same log: 1373 owners in 18 s (about 76 owners/s), and only forgotten owners are observed. For the two boundaries of this run that is 2283 s replaced by about 36 s, a factor of about 60.
- A live re-measurement needs the resident service generation restarted with this code (the pool and the slice worker load it at spawn); I did not restart services or touch the running pid 89217. The planner plan-only run (`--dry-run`) was not started either, to avoid a second 4 to 6 GB read on a host with 100 MB free.

## 6. What remains (not changed, evidence only)
- A handover that is still taken (heap past 3072 MB, or at least 25 % forgotten) restarts with a cold inventory (13 to 19 min at 435 k rows, about 500 rows/s while the worker is suspended half of the time). The vouch ledger and the owner checkpoint exist to shorten that; they could not be exercised because the host is memory starved (the worker was SIGSTOPped in a growth wait of 2604 MB; `repo capacity status` shows no spare above the 2048 MB floor).
- The step line prints seconds as `N of 3600` without a unit; consumers (`readyStepOverdueSecondsV1`, the failure-log test) read that shape, so I left it. A unit in the step text would need those consumers changed together.
