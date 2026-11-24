# W9: why no macOS worker reaches Ready at 16:40, what was landed (2026-11-24, 16:40 to 17:20)

Evidence (read-only): `/Users/Shared/semio-store/v1/origins/g1-*/{ready.json,ready-timings.json,slice-memory.json,attest.json,view.txt,ready-log-workspace*.txt}`, `repo fleet status`, the replica's inventories. Nothing restarted, nothing run on a worker, no scratch build.

## Verdict
1. The stage does not chase a moving tree or a moving code identity. The stage code digest of every macOS worker is `d5981b7a5a72` for all of today's 20-39 runs (no change since last evening; fleet status: all macOS workers on the tree's code `c179514e`). The ~30 landings of the day did not restart a stage.
2. Every run ends the same way: `stalled`, 12 to 47 min, growth 0 (last three of m01/m03/m08/m10/m11/m12 all `stalled`). m06/m07 hold ADOPTED runs of 2.7 h and 9.2 h.
3. The read is blocked on the storage mapping of the mounted image, not on edits. In every `slice-memory.json` (m01, m08, m10, m07) the presence session's `postVerify` has `mappingGrant` ACTIVE with 101.9 s (m01), 156.9 s (m08), 170.7 s (m10), and m07 967 s over 4 sessions (`mappingGrant` 814 s); `preVerify` 0.9 s (m01, m08), 39 s (m10), 272 s (m07). Looks: m01 15 done avg 6.2 s, m08 15 avg 3.4 s, m10 10 avg 32 s, m07 63 avg 65.6 s. `taken` shows 10, 15, 15, 57 of 608: ledger `present` = `done`, `wanted 0`, `running 0`, `fleet.*` all 0.
4. Why that freezes `taken`: `createPresenceCoalescerV1` (`B👁️observation/📥️reuse/🧺️coalesce/🟨️.mjs`) holds `draining` until the session callback returns, and the session (`withRequiredRepositoryCacheStoreConsumerV1`, `⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts:109-121`) ends with `verify("postVerify")` = `reservation.recheck` = journal read + `physicalGateVerify` + `mappingGrant` (shared, FIFO behind any queued exclusive mapping writer, with units, puller children (6 on m10) and collectors using the same image). Every ask that arrives while it waits queues; no new session opens until it is granted. A queue of 100 to 970 s per session, 8 looks in flight, hence `taken` standing still and the 10-min stall rule firing (m10 stood at `taken 10` for 1,737 s). Inference (GR16-19) is not on this path: `fleet.computed 0`, `derived 0`.
5. Each stall stop then repeats the file inventory (about 220 to 260 s, 195-211k observations) and the owner rounds from the persisted state; m06 shows the other effect: `a new pass over the owners starts (round 2): 1 path(s) changed under the read, 13 owner(s) forgotten` (round 3 at 9,786 s): one changed path forgets only 13 owners, so the snapshot-then-top-up behaviour already exists in the read; it is not what stops convergence.

## Per macOS worker (16:40)
| worker | last three runs | now | taken / settled | code digest |
|---|---|---|---|---|
| m01 | stalled 33 min / 16 min / 43 min, growth 0 | attempt 11, 544 s | 15 of 608, 2.4/min (15 in 375 s) | d5981b7a5a72 |
| m03 | stalled 12 / 28 / 32 min | attempt 4, file inventory (0 of 600 at 91 s), then round 1 17 of 3600 | none yet | same |
| m06 | stalled 24 min, failed 149 min, failed 53 min (13:55), then adopted | 2.7 h adopted run, round 3, 41 of 3600 | rounds 1-3, none taken | same |
| m07 | stalled 41 / 23 / 11 min (07:24), then adopted | 9.2 h adopted run | 57 of 608 (frozen; report file last written 08:15) | same |
| m08 | stalled 22 / 47 / 15 min | attempt 11, 479 s | 15 of 608 | same |
| m10 | stalled 27 / 23 / 18 min | attempt 19, 1,737 s | 10 of 608 (frozen 20 min) | same |
| m11 | stalled 19 / 16 / 24 min | attempt 3, round 1 45 of 3600 | none yet | same |
| m12 | stalled 19 / 17 / 24 min | attempt 0, round 1 4 of 3600 | none yet | same |
Settled owners do NOT grow across attempts: each attempt starts at 10 to 15 present (the records are valid, but are admitted only one presence session at a time) and the stage is killed before the pass ends. Resume works at the record layer (GR16 law), the pass cannot outrun the stall rule.
Owner records: 605 to 608 are needed; the ledger shows 10 to 57 verified present per attempt; 0 refused, 0 stale, no fleet fetch, so the identity of the selection did not move (no `stale`, `reasked` or `refused` count anywhere in the four slice reports). I could not count the fleet's graph records on the cache server without a tree walk; the per-worker ledgers above are the evidence.

## Question 2: "tree differs"
- Now 5 files on 10 of 11 workers (all `M`, in-flight peer edits: fem `receiver-tests.rs` and a `Cargo.toml`, os reactor turn `event-dispatch`/`task-resumes`, `app-declarations-fixture`); identical on m01/m02/m03/m06/m07/m08/m10/m11/m12 (452,562 files, same attested manifest), `only at g1-server`: 0. The coordinator is the one that lags (its view pass); the workers agree with each other.
- Only g1-m13 really differs (1,313 only at the coordinator, 13 at the worker, 452,399 files, offline 2.1 h).
- Replication lag of a landing to a worker, today: `applyMs` p50 464 to 609 ms, p95 1.9 to 3.7 s on all (m07 1.2 s / 6.1 s, m05 0.3 s / 33.6 s). Edits reach every worker in seconds, so lag is not the cause.
- Does the read restart on every changed input? No: a changed path forgets only the owners that read it (13 of 639 in m06), the pass restarts for those; the file inventory is bounded (GR/W4 lenient last attempt).

## Landed
- `readyGrantWaitV1` (process-tree) plus one tick check in the stage supervisor (`🚦️ready/🟦️.ts` next to the warden hold): while the stage's own fresh slice report (< 120 s old) shows a presence verification with an ACTIVE `mappingGrant` that has waited < 30 min, the stall clock does not run. A read queued in the image's grant queue no longer gets killed and sent through the inventory again; the 30-min maximum still ends a wedged grant. Policy keys `grantWaitFreshMilliseconds`, `grantWaitMaximumMilliseconds` in the process-tree schema. Tests: `grant-wait` 1/0 (new), process-tree 16/0, ready main 58/0. Takes effect when a worker's sidecar next restarts on the new code (replica copies it; not forced).
- Not landed (needs design, evidence in 3 and 4): the session's `postVerify` blocking the next ask (the coalescer should let the next session open while the previous one's tail waits, or verify once per take pass), and who holds the exclusive mapping writer in the queue (worker lease state is not published; add the queue head to `slice-memory` presence timings next).

## Answer to question 4: what stands between today and "all macOS workers Ready"
- Not code identity, not tree convergence, not record validity. It is the mapping-grant queue: the take pass advances 10 to 15 owners per attempt (2.4 owners/min at best on m01, 0.3/min on m10). 608 owners at 2.4/min is 4.2 h, at m10's rate 33 h, with the stall rule killing every 12 to 47 min. The remaining 3 steps after `taken` (admission, graph seal, build of `norm-din4108`, 23 min on m02) add about 0.5 h.
- If no further build-system edits land and the stall exemption is live: roughly 4 h on the least loaded workers (m01, m08), 10 h or more on m10/m07/m06, unless the grant queue is shortened (stop remote units and puller children on a worker during its Ready read, 3 to 7 units run per worker now). Without the exemption: never (every attempt restarts). I do not claim "Ready (updating)" semantics were added; the Ready worker m02 stays ready on a stage with unchanged input digests (it was `ready` all day through the edits).
