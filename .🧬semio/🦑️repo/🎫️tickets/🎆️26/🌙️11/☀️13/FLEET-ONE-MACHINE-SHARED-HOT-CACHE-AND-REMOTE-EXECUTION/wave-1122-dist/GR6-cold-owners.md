# GR6: cold cargo owners (2026-11-23, 16:05 to 17:30 Mac clock)

Code lane GR6. No code was changed, no lock was taken, nothing landed. Scripts kept in `wave-1122-dist/GR6/` (`profile-summary.py`: CPU profile summary that separates stalls from compute; `count-preload.mjs`: bun `--preload` that counts intervals, fetches and fetch paths).

## Verdict
1. The premise (`cargo metadata` per owner) is false again (E42 said the same). No Cargo process runs under a graph inference child, and the harness run made zero `cargo` spawns. The heavy child of the live read (pid 35722) was one JS thread at about 98 % CPU, 47 CPU-minutes in 72 minutes, 3.4 GB RSS. GR4's manifest-closure module is therefore NOT on the owner path and was not used. The Cargo path closure is already derived from manifests (`nativeDependencyRoots`).
2. A cold owner costs about 90 to 115 s CPU (user 61 to 86 s, **sys 126 to 132 s**) and 218 to 244 s wall, in isolation, at load 38 to 58. More than half of the profile wall time is stalls, not code.
3. Running the profile under `taskpolicy -b` starved it completely (10 CPU-seconds in 14 minutes at load 58); do not profile with it.

## 1. Profile of ONE cold owner, in isolation (task 1)
Owner `@semio-tech/norm-eurocode-structural-plugin` (one of the 6 slow owners, 1,540 to 1,700 s in the live read), owner harness `--late`, private staging, `bun --cpu-prof`, two runs: wall 244 s / cpu 115 s (2,131 rows, record 129,967 bytes, digest c35505f8...), wall 218 s / cpu 89 s (same digest). Peak RSS 2.3 GB (footprint 2.6 GB), 593k page reclaims, 0 swaps.

| bucket | seconds (of 252 s sampled) | what |
|---|---|---|
| 44 stalls of more than 200 ms (sample gaps) | 138 (55 %) | the process is descheduled or blocked in the kernel and the time lands on whichever frame wakes (`renew`, `readFileSync`, `machinePathV1`, `jsonSchemaSubsetValueEquals`, `scriptProviderGroupV1`: pure JS frames, so it is not their cost). Consistent with sys time 126 to 132 s and a host at load 38 to 58 with 1.7 GB swap. |
| command-script import closure walk (`closurePackagesV1` 19.1, `walkImportsV1` 15.0, TS parse inside) | 19 incl | reading and analysing the tool's own sources, `observeReadSourceV1` 25 incl, `readFileSync` 17.7 self |
| closure-layer store: `entries` pack/zlib/index (`layerBatchesSync` 12.3, `zlibBufferSync` 8.3, `indexSync` 7.8, `writeSync` 7.9) | about 22 incl | compressing and writing closure layers |
| observation row verification (`projectObservationHoldsV1` 9.8, `observationLayerHoldsAsyncV1` 8.9) | about 10 | |
| `set` of the bounded cache (`🧊️bounded` 7.7 self, 9.0 incl) | 9 | |
| `stringify`/`parse` 6, gc 4.5, `statSync`/`lstatSync`/`existsSync` 8.9, `rmSync` 4.3 (harness close 2.9) | about 24 | |
| fleet exchange (`renew`, claim, lookup) | 0 compute | 846 fetches in the run: 279 GET `ac`, 274 claim, 274 release, 14 renew; all holds released (274 of 274). The 67 s that the raw profile shows under `renew` -> fleet facts read is stall time ending in that frame: the same facts read costs 0.22 ms per call in isolation (200 calls). |

The raw, unfiltered profile points at fleet facts (`facted`/`readLocalFleetCacheFactsV1` 67 s) and at `cargo`-free JS; both are artefacts of crediting a stall to the frame that resumed. Splitting samples at 200 ms gives the table above (113 s compute, matching the 89 to 115 s CPU).

## 2. Root cause and fix (task 2)
- There is no single dominant defect to remove: compute is spread over the closure walk (about 19 s), layer store (about 22 s), row verification (about 10 s) and file syscalls, and the other half of the time is the kernel/host (sys time). Byte-identical differential records already hold: both runs give digest c35505f8..., and E42 recorded identical digests across 4 owners and 3 modes.
- A manifest-only derivation cannot replace this work: the 2,131 rows of the owner are mostly command-script closure rows, not Cargo manifests (E42: the Cargo path closure is 7 % of a cold owner).
- Not done, ranked by evidence: (a) make the closure layer store avoid recompressing a layer whose bytes are unchanged (`entries` `pack`, about 8 s of zlib); (b) share the command-script closure walk across all owners of a process (already memoised, `walkImportsV1` 3,424 claims, 0 duplicate computations in E42); (c) lower sys time: the sys share (126 s of 186 CPU-s) suggests page faults and per-file syscalls on a 2.3 GB heap under pressure; fewer, larger reads would help but is a design change, not a point fix. I did not land any of these: none has a measured gain I could prove byte-identical in the time available, and the checkout is under a play release.

## 3. Stall watch (task 3)
`createGraphReadWatchV1` (`🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts`) already counts: completed phase counters, shared closures, ledger counts (`owners N of M`), and an increase of the row count of a RUNNING owner (`rowsMoved`) as progress; waits, age and concurrency do not. The stall bound today is 25 minutes (`inferStallMilliseconds` 1,500,000) and the absolute bound 12 h (`inferMaximumMilliseconds`), so the "5 minutes" text of the task is an older value. In the live read an owner at 1,707 s with 209 rows did not trip it because other owners arrived (118, 119, 120 ... 127 of 128 over 40 minutes). Phase progress inside an owner is NOT counted (a row-less cold walk shows 0 rows for 4 minutes and counts as silence). Not changed; the lever is to emit an owner phase row (walk/verify/pack) and count it in `rowsMoved`; the genuine-hang bound stays the 25 min silence plus the 12 h cap.

## 4. Finishing the `workspace` selection read (task 4)
Not done, deliberately. The read that runs is NOT the selection read: it is the whole-graph refresh (read 34600, reason `refresh`, started 14:23, runner 34597). Observed over time: wave 1 owners 98 -> 118 (16:08) -> 119 -> 120 -> 122 -> 123 -> 126 -> 127 of 128 (about 17:25), mix 91 present, 22 fetched from the fleet, 14 inferred here; whole-graph phase `show owners 141/5222`, "about 37.7 h left" in `repo fleet status`; peak heap child 3.4 GB RSS (grant 3,072 MB, 2.1 to 3.4 GB held); memory normal, swap 1.7 GB, load 53. No head is published ("Generation: none published", `Graph: no builder`). A second (selection) read would queue behind the whole read's custody (E33: custody held 6,800 s), so I did not start one. Decision needed: either let the whole read finish waves (the cold owners it infers are stored per owner and fetched by the selection read afterwards) or cancel it and run the selection (`repo run-many norm-din4108,norm-din4108-rs,repo-mcp --targets=build`) right after wave 1, when 127 of 128 owners are stored.

## Not verified
- Whether the stall share is page-in (swap 1.7 GB) or CPU descheduling: sys time says kernel work, `time -l` shows 0 swaps; no `fs_usage` or `sample` was taken.
- Peak memory of a fetch-only selection read (no head exists).
- No Windows or Linux measurement.
