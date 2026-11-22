# 🔎 Audit H9 (Haiku): live rollout of landings since 15:00

Snapshot 2026-11-22 ~16:14-16:16 CET, read-only. Load 43-48 on 10 cores (above the 40 gate, no tests run).

## Summary per item
| Item | Verdict | Evidence |
|---|---|---|
| E55/E61 fleet-lookup skip ratio | NOT VERIFIABLE from store | slice-memory.json holds only the latest sample per worker. m08 sample 16:12:43 (uptime 272 s, so started ~16:08) is in phase "reading the owners (round 1)" 13/3600 and carries no skipped/total lookup field. m11 sample 16:11:13 (uptime 476 s) has owners.looks counters but no skip ratio. Earlier 830/1026 and 277/557 cannot be reproduced. |
| E73 ready-stage-claim.json | NOT YET ROLLED OUT | file absent on g1-m01, g1-m08, g1-m10, g1-m12. Workers m01, m07, m10, m12 are `waiting`, refusal `sidecar-restarting` ("waits for the worker supervisor to restart with updated code"). m08 was started 16:08 and is `running`, reason `inputs-changed`. |
| E73 page-outs / owners progress | NOT VERIFIABLE | m08 owners 13/3600 at 16:12. m10 owners 3380/3600 round 1 at 15:46 (pass 1, ledger fetched 520, here 17, wanted 67, missing 67). m01 pass 2 at 16:03, taken 10/604, looks.count 296, observe phase 1432630 ms cumulative. No page-out counter read. |
| E74 unrecorded / spool-full | PARTIAL | ready-log-*.txt grep: 0 `spool-full` and 0 `unrecorded` lines on m01, m06, m08, m10, m12. Unit-cache skip counters not read (not in the 4 surfaces I queried). Brief's m06 spool-full 50412 figure not re-checked. |
| E66 divergence counters | LIVE, improved and still firing | remote-cache status: lifetime 148 (owner-pool 40, unit 80, closure 12, project-inference 16). current count 1 (owner-pool, 16:23 CET), lastHour count 2 (owner-pool). Last recorded divergence atMs 1795359587674 (~16:06). Some divergences remain on owner-pool and closure (kept g1-m12, refused g1-server closures). |
| E76 worker-lost / scheduler summary | NOT VERIFIED | did not read scheduler logs (grep over cache dirs timed out to background, stopped). Live scheduler status: queues empty, 0 waiting, longestWaitSeconds 0, budget 28672 MB, reserved 0. |
| E71/E77 disk-full | PARTIAL, rollout in progress | g1-m04 refused `disk-short`: needs 6144 MB, has 5762 MB in /home/deck/semio/.cache (stale 64 min, offline). g1-m01 disk-full (per coordination, not re-read). doctor: 3 dead p-<pid> publications hold 43.9 MB in graph; 1 retired root native:test-admission on disk. |
| Ready state per worker | see table | below |
| current.json generation | generation 260, parent a09127b6..., reason closure-missing, manifest null (generation is failing: closure missing) | |

## Worker state (fleet status --json, 16:14)
| worker | liveness | recorded state | stage (index) | attempts | note |
|---|---|---|---|---|---|
| g1-m01 | working | waiting | workspace (5/5) | 5 | sidecar-restarting |
| g1-m02 | working | running | tools (4/5) | 20 | adopted |
| g1-m03 | working | running | tools (4/5) | 0 | adopted |
| g1-m04 | offline 64 min | waiting | dependencies (3/5) | 11 | disk-short |
| g1-m05 | offline 22 min | running | dependencies (3/5) | 20 | heartbeat 2260 s old |
| g1-m06 | working | running | workspace (5/5) | 3 | adopted |
| g1-m07 | working | waiting | tools (4/5) | 11 | sidecar-restarting |
| g1-m08 | working | running | workspace (5/5) | 0 | inputs-changed |
| g1-m09 | absent | - | - | - | never wrote a record |
| g1-m10 | working | waiting | workspace (5/5) | 7 | sidecar-restarting |
| g1-m11 | working | running | workspace (5/5) | 0 | adopted |
| g1-m12 | working | waiting | workspace (5/5) | 15 | sidecar-restarting |
| g1-m13 | working | running | tools (4/5) | 0 | adopted |
| g1-server | absent (no liveness) | - | - | - | |

Ready: 0 of 13 (counts: working 10, offline 2, absent 2). The fleet-status `fleet` source failed (`/counters/cancelledQueued` required) and `storage` timed out at 5 s, so disk and cancellation counters are missing from this snapshot.

## Remote execution / cache / doctor
- remote-execution: endpoint reachable, gRPC capabilities and execution true, probe 28 ms. Pending load stats not in the probe.
- remote-cache: 44752 entries, 48.6 MB of 2.58 GB, hits 588720, misses 69866, fleet origins ok 14. Peers hits 0, misses 0, errors 0, skipped 491, absent 4. Token not present in env (not recorded here).
- doctor (caches, host, processes): verdict blocked by host.load-high (load 43.6 on 10 cores). Warnings: 3 dead publications (43.9 MB), 1 retired root. Orphan processes: repo publish (pid 18683, 20938 s old), repo service host run (pid 90452, 1698 s).

## Not verified (must be read by coordinator before relying on this)
- E55/E61 skip ratio time series, E73 page-out counters, E74 unit-skip reason counters, E76 scheduler summary and worker-lost counts, and the current disk-full list for m01/m08/m11/m12 are NOT read here.
- Each slice-memory.json keeps only the latest sample, so a post-15:20 time series needs the worker log or a new sampler.
