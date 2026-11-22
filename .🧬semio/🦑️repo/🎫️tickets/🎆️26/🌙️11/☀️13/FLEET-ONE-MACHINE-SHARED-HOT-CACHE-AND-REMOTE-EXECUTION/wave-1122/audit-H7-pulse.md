# 🩺 Wave 1122 Audit H7 (Pulse-3): Fleet, Cache, Remote Execution, Coordinator

Read-only. Raw outputs kept in the lane scratch folder (not in the worktree). Values are verbatim unless marked as converted.

Timing: snapshot A fleet read 11:19:34 CET, cache and execution reads 11:20 to 11:21, origins 11:21, host 11:21. Snapshot B fleet read 11:37:14 CET, cache and execution reads 11:37 to 11:38, origins 11:39, host 11:39 to 11:40. The gap is 17 min 40 s by the fleet clock (18 min by the other reads), slightly short of 20 min because the wait was split across calls.

## Snapshot A (about 11:20 CET)

### A1 Workers and fleet (`repo fleet status --json`)

Counts: ready 1, working 10, idle 0, silent 0, offline 1, absent 2. Summary: running 3, blocked 0, waiting 0, alerts 0, warnings 4.
Memory: pressure warn, available 6481 MB, swap used 28568 MB, load 36.68, admitted 6656 MB, observed 7110 MB, under-reserved 1273 MB, grants 3, waiters 0.
Fleet totals: cacheHit 212022, cacheMiss 382240, service cache hits 29108 / misses 10608 / puts 5314, offloadedIn 2, offloadedOut 2, claims active 33. The storage reader failed ("The reader did not answer within 5 s"); all other readers ok.

| Machine | Platform | Liveness | Recorded | Stage (att) | Refusal | Worker / why | Headroom | Load | Running | OffIn | OffOut | Hit | Miss | Skipped | Top skip reasons |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| g1-m01 | macos | working | running | workspace (2) | - | refusing / disk-full | no | 12.94 | 1 | 0 | 0 | 2781 | 7161 | 3351 | unrecorded 3344 |
| g1-m02 | macos | working | waiting | tools (15) | sidecar-restarting | idle | yes | 5.31 | 0 | 1 | 0 | 17452 | 43217 | 18158 | unrecorded 17832, upload-expired 161, upload-expired-down 83 |
| g1-m03 | macos | working | running | tools (10) | - | idle | yes | 8.21 | 1 | 1 | 0 | 21665 | 29575 | 1040 | none |
| g1-m04 | linux | working | waiting | dependencies (10) | disk-consumers | refusing / disk-full | no | 7.96 | 0 | 0 | 0 | 15547 | 52909 | 4436 | none |
| g1-m05 | windows | offline | running | dependencies (19) | - | not listed | not listed | - | - | - | - | - | - | - | record stale 79 min |
| g1-m06 | macos | working | running | workspace (0) | - | idle | yes | 15.06 | 6 | 0 | 0 | 42912 | 45718 | 21335 | unrecorded 20828, no-compiler-identity 471 |
| g1-m07 | macos | working | running | tools (8) | - | refusing / no-headroom | no | 16.31 | 5 | 0 | 0 | 15569 | 29911 | 14423 | unrecorded 14410 |
| g1-m08 | macos | working | waiting | workspace (8) | sidecar-restarting | refusing / disk-full | no | 17 | 0 | 0 | 0 | 6156 | 10308 | 1852 | unrecorded 1651, key-incomplete 175 |
| g1-m09 | - | absent | - | - | - | - | - | - | - | - | - | - | - | - | enrolled, never wrote ready.json |
| g1-m10 | macos | working | running | workspace (5) | - | idle | yes | 6.25 | 4 | 0 | 0 | 28335 | 57875 | 16477 | unrecorded 16359, diverged 65 |
| g1-m11 | macos | ready | ready | - | - | refusing / disk-full | no | 6.17 | 2 | 0 | 0 | 22038 | 50663 | 15997 | unrecorded 15740, spool-full 197 |
| g1-m12 | macos | working | running | workspace (1) | - | refusing / disk-full | no | 12.69 | 2 | 2 | 0 | 23921 | 31551 | 7625 | spool-full 836, upload-expired-down 681, breaker-down 238 |
| g1-m13 | macos | working | waiting | dependencies (28) | sidecar-restarting | refusing / - | yes | 7.33 | 0 | 0 | 0 | 0 | 0 | 0 | none |
| g1-server | - | absent | - | - | - | refusing / no-headroom | no | 35.87 | 3 | 0 | 0 | 15646 | 23352 | 5685 | unrecorded 5677 |

Hit ratio per machine, cumulative counters at A (hits / (hits + misses)): m01 28.0%, m02 28.8%, m03 42.3%, m04 22.7%, m06 48.4%, m07 34.2%, m08 37.4%, m10 32.9%, m11 30.3%, m12 43.1%, g1-server 40.1%, m13 not measured (0 / 0), fleet 35.7%. These are not a window ratio.

Findings (6): memory-pressure (warn); source-failed storage (warn); under-reserved pid 50632 ("uses 3321 MB but only 2048 MB are admitted", warn); worker-offline g1-m05 (79 min, warn); sealed-drifted (info, 2308 changes); worker-absent g1-m09 (info). unit-key-machine-dependent: 0 findings (fleet reader ok).

### A2 Remote cache (`repo remote-cache status`)

Endpoint http://192.168.178.70:6094, configured true, upload true, token present false (the status reads the endpoint from fleet-facts). Probes: nx-self-hosted 404 reachable authorised, bazel-http 404 reachable authorised, nx-self-hosted 404 reachable authorised. serverFault null. Remote block: reachable true, authorised true.

| Item | Value |
|---|---|
| entries / bytes | 33778 / 39354598 (maximumBytes 2576875520) |
| evicted (total since start) | 70 entries / 63819 bytes |
| startedAt | 2026-11-22T10:14:53.438Z (11:14:53 CET) |
| AC hits / misses / puts | 1253 / 5005 / 193 (conflicts 0, rejected 0, stale 0) |
| CAS hits / misses / puts | 28271 / 6275 / 5376 (conflicts 0, rejected 0, stale 0) |
| nx hits / misses / puts | 0 / 14 / 1 |
| Divergences count | 130; byClass closure 9, owner-pool 25, project-inference 16, unit 80; since 01:53:11 CET |
| Divergence evidence | 135 (253445031 bytes; closure 8, owner-pool 15, project-inference 64, unit 48) |
| Superseded / advisory | {} / {} |
| Peers hits / misses / errors / skipped / absent | 0 / 0 / 0 / 113 / 4 |
| Claims | active 34, waiting 1, oldest 112 s |
| Remote hits / misses (fleet view) | 29524 / 11294 |

### A3 Remote execution (`repo remote-execution status`)

startedAt 2026-11-22T10:14:46.929Z (11:14:46 CET). grpc reachable, probe 26 ms. cache.answers true.
Counters: queued 0, executing 0, completed 3, cached 0, failed 0, cancelled 0, retried 0, deduplicated 0, refused 0, offloadRejected 0.
Workers registered 13. Headroom true: g1-m13, g1-m02, g1-m06, g1-m03, g1-m10, g1-m05 (6). Per worker offloadedIn: m13 0, m02 2, m06 0, m03 1, m10 0, m05 0, m11 0, m12 0, m01 0, m08 0, m07 0, server 0, m04 0 (sum 3).

### A4 Origins (`S/origins/*/ready.json`, last line of newest ready-log)

| Origin | Ready | State | Stage (idx) | Att | Exit | Refusal | Age of ready.json (s) | Newest log: last line (log age s) |
|---|---|---|---|---|---|---|---|---|
| g1-m01 | false | running | workspace (5) | 2 | - | - | 7 | `.../.cache/b/t/m: 5160 of 8191` (9) |
| g1-m02 | false | running | tools (4) | 15 | - | - | 5 | `.../.cache/b/t/m: 5403 of 8191` (1255, ready-log-tools-failed) |
| g1-m03 | false | running | tools (4) | 10 | - | - | 5 | `.../.cache/b/t/m: 5394 of 8191` (5) |
| g1-m04 | false | waiting | dependencies (3) | 10 | 1 | disk-collecting | 23 | `repository:command failed with exit code 1` (32909, ready-log-tools-failed) |
| g1-m05 | false | running | dependencies (3) | 19 | - | - | 7322 | `C:\semio: 117453 of 803224` (7439) |
| g1-m06 | false | running | workspace (5) | 0 | - | - | 3 | `.../.cache/b/t/m: 5490 of 8191` (1) |
| g1-m07 | false | running | tools (4) | 8 | - | - | 4 | `.../.cache/b/t/m: 5340 of 8191` (4) |
| g1-m08 | false | backoff | workspace (5) | 8 | 1 | stage-failed | 17 | `.../.cache/b/t/m: 5417 of 8191` (482, ready-log-workspace-failed) |
| g1-m09 | no ready.json | - | - | - | - | - | - | - |
| g1-m10 | false | running | workspace (5) | 5 | - | - | 5 | `.../.cache/b/t/m: 5281 of 8191` (7) |
| g1-m11 | true | ready | - | 0 | - | - | 27 | `.../.cache/b/t/m: 5372 of 8191` (406, stale log) |
| g1-m12 | false | running | workspace (5) | 1 | - | - | 4 | `.../.cache/b/t/m: 5323 of 8191` (3) |
| g1-m13 | false | backoff | dependencies (3) | 28 | 1 | stage-failed | 11 | `.../.cache/b/t/m: 5649 of 8191` (557, ready-log-dependencies-failed) |
| g1-server | no ready.json | - | - | - | - | - | - | - |

Ready count: 1 (g1-m11, readyMs 11:14:48 CET, startedMs 11:15:05 CET, so readyMs is earlier than startedMs in the record, verbatim).

### A5 Coordinator graph

`.cache/b/g/m/generations/current.json`: `{"generation":260,"manifest":null,"parent":"a09127b692d0a7b02a627d07329d84db479232b01a05dc7446bc89894da27601","reason":"closure-missing","version":1}`, mtime 2026-11-20 03:59:59. The newest graph admission and authority files (`selected/0bcdb8621838...`) are from 10:58 and the closure directories from 10:39.
Running reads (fleet runs): pid 74175 `show`, phase plan, ageSeconds 5194 (about 1 h 27 min). Its child pid 74385 at 1.0 GB RSS, 30.6% CPU at B. Graph builder pid 50632 `graph-plan`, phase `boot-recovery:observe:0/0`, ageSeconds 6117, etaOverdue true, memory admitted 2048 MB, observed 3321 MB, exclusive scope `.cache/b/g/m:scope:2157`.
Progress file: none found under `.cache/b/g` written in the last 30 min (only `m/optional.sqlite` at 11:20 and `fixed.dmg`).

### A6 Host

| Item | Value |
|---|---|
| `/System/Volumes/Data` | 926Gi size, 848Gi used, 26Gi avail, 98% (inodes 4%) |
| uptime (11:21) | load averages 36.64 34.34 32.11 |
| `vm.swapusage` | total 29696.00M, used 28883.44M, free 812.56M |
| fseventsd | pid 551, RSS 3745984 KB, 98.3% CPU |
| Cache server (listens 192.168.178.70:6094) | pid 78055, bun, started 11:14:50, elapsed 07:06; all threads 31T, nice 0 |
| Stall lines in build-cache.log | 370 in total (first 2026-11-22T02:03:11Z, last 04:53:20Z); 0 in the last 30 min |

## Snapshot B (about 11:38 CET)

### B1 Workers and fleet

Counts: ready 1, working 10, idle 0, silent 1, offline 0, absent 2. Summary: running 3, blocked 0, waiting 0, alerts 0, warnings 4.
Memory: pressure warn, available 5885 MB, swap used 29459 MB, load 35.09, admitted 6656 MB, observed 6324 MB, under-reserved 350 MB, spare 1789 MB, grants 3, waiters 0.
The fleet reader failed ("The reader did not answer within 5 s") and so did the storage reader. Per-machine cache hit/miss, skip reasons, offloadedOut and worker why/headroom are not measured in B.
Findings (6): memory-pressure (warn), source-failed fleet (warn), source-failed storage (warn), worker-silent g1-m13 (warn), sealed-drifted (info), worker-absent g1-m09 (info). unit-key-machine-dependent: not present (fleet reader failed, so not measured).

| Machine | Liveness | Recorded | Stage (att) | Refusal | Heartbeat age (s) |
|---|---|---|---|---|---|
| g1-m01 | working | running | workspace (2) | - | 0 |
| g1-m02 | working | backoff | tools (16) | stage-failed | 0 |
| g1-m03 | working | running | tools (10) | - | 0 |
| g1-m04 | working | waiting | dependencies (10) | disk-collecting | 0 |
| g1-m05 | working | running | dependencies (19) | - | 893 |
| g1-m06 | working | running | workspace (0) | - | 1 |
| g1-m07 | working | backoff | tools (9) | stage-failed | 2 |
| g1-m08 | working | running | workspace (8) | - | 2 |
| g1-m09 | absent | - | - | - | - |
| g1-m10 | working | running | workspace (5) | - | 3 |
| g1-m11 | ready | ready | - | - | 1 |
| g1-m12 | working | running | workspace (2) | - | 4 |
| g1-m13 | silent | backoff | dependencies (29) | stage-failed | 1 |
| g1-server | absent | - | - | - | - |

### B2 Remote cache

Probes and reachability: reachable true, authorised true (remote block). serverFault null.

| Item | Value |
|---|---|
| entries / bytes | 33308 / 39124300 |
| evicted (total since start) | 1423 entries / 1615983 bytes |
| AC hits / misses / puts | 12799 / 18667 / 1322 (conflicts 2, rejected 0, stale 42) |
| CAS hits / misses / puts | 281762 / 34064 / 27035 (conflicts 0, rejected 102, stale 0) |
| nx hits / misses / puts | 0 / 17 / 2 |
| Divergences count | 130; byClass closure 9, owner-pool 25, project-inference 16, unit 80 (unchanged from A) |
| Divergence evidence | 135 (253445031 bytes) |
| Superseded / advisory | {} / {} |
| Peers hits / misses / errors / skipped / absent | 0 / 0 / 0 / 488 / 4 |
| Claims | active 53, waiting 0, oldest 1161 s |
| Remote hits / misses (fleet view) | 294561 / 52748 |

### B3 Remote execution

startedAt 2026-11-22T10:14:46.929Z (unchanged). cache.answers true.
Counters: queued 0, executing 0, completed 179, cached 0, failed 24, cancelled 7, retried 3, deduplicated 0, refused 1, offloadRejected 22.
Workers registered 12 (g1-m04 no longer listed). Headroom true: g1-m02, g1-m03, g1-m10, g1-m13, g1-m05 (5). Per worker offloadedIn: m13 66, m03 64, m02 39, m10 17, m06 2, m07 2, m01 0, m05 0, m08 0, m11 0, m12 0, server 0 (sum 190).

### B4 Origins

| Origin | Ready | State | Stage (idx) | Att | Exit | Refusal | Age of ready.json (s) | Newest log: last line (log age s) |
|---|---|---|---|---|---|---|---|---|
| g1-m01 | false | running | workspace (5) | 2 | - | - | 4 | `.../.cache/b/t/m: 5160 of 8191` (4) |
| g1-m02 | false | backoff | tools (4) | 16 | 1 | stage-failed | 87 | `.../.cache/b/t/m: 5403 of 8191` (911, ready-log-tools-failed) |
| g1-m03 | false | running | tools (4) | 10 | - | - | 2 | `.../.cache/b/t/m: 5394 of 8191` (10) |
| g1-m04 | false | waiting | dependencies (3) | 10 | 1 | disk-collecting | 135 | `repository:command failed with exit code 1` (33983, ready-log-tools-failed) |
| g1-m05 | false | running | dependencies (3) | 19 | - | - | 8 | `C:\semio: 100218 of 803224` (67) |
| g1-m06 | false | running | workspace (5) | 0 | - | - | 1 | `.../.cache/b/t/m: 5490 of 8191` (5) |
| g1-m07 | false | backoff | tools (4) | 9 | 1 | stage-failed | 93 | `.../.cache/b/t/m: 5340 of 8191` (820, ready-log-tools-failed) |
| g1-m08 | false | running | workspace (5) | 8 | - | - | 1 | `.../.cache/b/t/m: 5417 of 8191` (6) |
| g1-m09 | no ready.json | - | - | - | - | - | - | - |
| g1-m10 | false | running | workspace (5) | 5 | - | - | 3 | `.../.cache/b/t/m: 5281 of 8191` (1) |
| g1-m11 | true | ready | - | 0 | - | - | 81 | `.../.cache/b/t/m: 5372 of 8191` (1480, stale log) |
| g1-m12 | false | running | workspace (5) | 2 | - | - | 0 | `.../.cache/b/t/m: 5323 of 8191` (0) |
| g1-m13 | false | backoff | dependencies (3) | 29 | 1 | stage-failed | 3 | `.../.cache/b/t/m: 5649 of 8191` (363, ready-log-dependencies-failed) |
| g1-server | no ready.json | - | - | - | - | - | - | - |

### B5 Coordinator graph

current.json unchanged (generation 260, reason closure-missing, mtime 2026-11-20 03:59:59).
Running reads: pid 74175 `show`, plan, etime 01:46:42, child pid 74385 at 1010656 KB RSS, 30.6% CPU. Graph builder pid 50632 `graph-plan`, phase `boot-recovery:observe:0/0` still, ageSeconds 7178 (was 6117), heldSeconds 3444 (was 2384), etaOverdue true, observed 2398 MB. No progress file found.
`.cache/b/g/fixed.dmg`: 19326345216 bytes at 11:21 and at 11:39 (same size, mtime moved to 11:39:37).

### B6 Host (11:39 to 11:40 CET)

| Item | Value |
|---|---|
| `/System/Volumes/Data` | 926Gi size, 848Gi used, 24Gi avail, 98% (inodes 5%) |
| uptime (11:39) | load averages 27.72 33.35 33.36 |
| `vm.swapusage` | total 31744.00M, used 30692.69M, free 1051.31M (11:40: used 30237.56M, free 1506.44M) |
| fseventsd | RSS 4660304 KB, 107.3% CPU |
| Cache server pid 78055 | process pri 37, nice 0, CPU 38.5% |
| Stall lines | 370 total; 0 in the last 30 min (no new stall since 04:53:20Z) |

## Deltas (A to B, about 18 min)

- Hit ratio per machine in the window: not measured (fleet reader failed in B). Cluster level from the cache server counters (window = A to B): AC 11546 hits, 13662 misses, ratio 45.8%; CAS 253491 hits, 27789 misses, ratio 90.1%; AC puts +1129, CAS puts +21659, nx puts +1.
- Offloads: remote-execution offloadedIn sum 3 to 190 (+187). Counters: completed 3 to 179 (+176), failed 0 to 24, cancelled 0 to 7, retried 0 to 3, refused 0 to 1, offloadRejected 0 to 22.
- Divergences added in the window: 0 (130 to 130; byClass unchanged). Against H5 (98 at 03:09): +32 (closure 1 to 9, owner-pool 1 to 25, project-inference 16 to 16, unit 80 to 80). Superseded: H5 project-inference 3, now {}.
- Stage changes A to B: g1-m02 waiting (sidecar-restarting) to backoff (stage-failed, attempts 15 to 16, exit 1). g1-m05 offline (record stale 79 min) to running (ready.json updated 8 s before B). g1-m07 running (no-headroom) to backoff (stage-failed, attempts 8 to 9). g1-m08 waiting (sidecar-restarting) to running (attempts 8). g1-m12 attempts 1 to 2. g1-m13 waiting (sidecar-restarting, attempts 28) to backoff (stage-failed, attempts 29), liveness silent. Others unchanged.
- Ready count: 1 to 1 (g1-m11).
- Headroom (remote execution): 6 to 5 (g1-m06 dropped out; g1-m02, g1-m03, g1-m10, g1-m13, g1-m05 remain).
- Registered remote workers: 13 to 12 (g1-m04 gone from the registry).
- Workspace progress counters: identical A to B for g1-m01 (5160), g1-m02 (5403), g1-m03 (5394), g1-m06 (5490), g1-m07 (5340), g1-m08 (5417), g1-m10 (5281), g1-m12 (5323), g1-m13 (5649). g1-m05 went 117453 to 100218 (decreased).
- Host: available memory 6481 to 5885 MB; swap used 28568 to 29459 MB (fleet memory), 28883.44 to 30692.69 MB (vm.swapusage); disk avail 26Gi to 24Gi; load 36.64 to 27.72 (1 min); fseventsd RSS 3745984 to 4660304 KB, CPU 98.3% to 107.3%.

## Comparison with H5 (03:09 and 03:29 CET)

| Metric | H5 (03:09 / 03:29) | H7 A (11:19 to 11:21) | H7 B (11:37 to 11:40) |
|---|---|---|---|
| Ready | 0 / 0 | 1 (g1-m11) | 1 (g1-m11) |
| Working / silent / offline / absent | 10 / 2 / 0 / 2 (03:09) ; 12 / 0 / 0 / 2 (03:29) | 10 / 0 / 1 / 2 | 10 / 1 / 0 / 2 |
| Remote cache | 9591 entries, 10519455 bytes (03:09) ; unreachable (03:29) | 33778 entries, 39354598 bytes, reachable | 33308 entries, 39124300 bytes, reachable |
| AC hits / misses | 750 / 6517 (03:09) | 1253 / 5005 (since 11:14:53 restart) | 12799 / 18667 |
| CAS hits / misses | 99684 / 7722 (03:09) | 28271 / 6275 | 281762 / 34064 |
| Divergences | 98 | 130 | 130 |
| Superseded | project-inference 3 | {} | {} |
| Remote execution counters | completed 30 to 267, failed 72 to 176 | completed 3, failed 0 (restart) | completed 179, failed 24 |
| Offloaded in (sum) | 61 to 391 | 3 | 190 |
| Headroom workers | 2 to 4 | 6 | 5 |
| Unit-key-machine-dependent findings | 28 (03:09) | 0 | not measured |
| Swap used | 20103 / 21504 MB ; 20876 MB at 03:31 | 28883 of 29696 MB | 30692 of 31744 MB |
| Disk avail | 36Gi (03:09) ; 57Gi (03:31) | 26Gi | 24Gi |
| fseventsd | 6743872 KB (03:09) ; 7982256 KB (03:31) | 3745984 KB | 4660304 KB |
| Load (1 min) | 39.24 ; 66.01 | 36.64 | 27.72 |
| Event loop stalls | not measured | 370 total, 0 in 30 min | 370 total, 0 in 30 min |

## Anomalies

1. Restart at 11:14:46 to 11:14:53 CET. Remote execution startedAt 10:14:46.929Z, cache startedAt 10:14:53.438Z (cache pid 78055 started 11:14:50), and g1-m11 became Ready at readyMs 11:14:48. All remote counters are reset by this restart, so counter comparisons with H5 are not like-for-like. The cause is not in the sources read.
2. Workspace progress is frozen. Nine macOS workers show the same `N of 8191` value in A and B (about 17 min). Log mtimes are fresh, so the writers are alive but not advancing. The coordinator's graph-plan builder pid 50632 is stuck in `boot-recovery:observe:0/0` (heldSeconds 2384 to 3444, etaOverdue true). The g1-m10 ready.json tail says "Reading the project graph slice (2816 s): waiting for the f...". The workers appear to wait on the coordinator graph read.
3. Coordinator graph state: current.json still points to generation 260 (reason closure-missing, manifest null, written 2026-11-20 03:59:59). No progress file for the running `show` read was found.
4. `repo remote-cache status` and `repo remote-execution status` both streamed a `repository:build` run of `build:source-admission` (`still working after 5 s / 10 s / 20 s; last phase started: source.capture`) before the JSON. The status reads trigger source admission and are not free.
5. Remote cache evictions: 70 entries at A to 1423 at B (+1353 in about 18 min), while the store holds 39 MB of its 2576875520-byte maximum. Eviction is not driven by size pressure.
6. g1-m05 liveness is inconsistent: fleet worker row says `working` with heartbeatAgeSeconds 893 at B, while its ready.json was updated 8 s before B. At A it was `offline` with a 79-minute-stale record.
7. g1-m04 (Linux, disk-consumers / disk-collecting) is still refused: 6144 MB needed, 2154 MB free at A (the B free-space figure was not re-read). It is no longer in the remote-execution worker registry at B. Its newest ready-log is 33983 s old.
8. g1-m13 went silent (liveness silent, backoff stage-failed, 29 attempts) between A and B. g1-m02 and g1-m07 are in backoff with stage-failed exits in tools.
9. Divergences rose from 98 (H5) to 130 with closure and owner-pool classes growing. None were added in the A to B window.
10. Memory: swap 30692 of 31744 MB used at B with 5885 MB available. fseventsd is at 107% CPU and 4.4 GB RSS. The graph store image `fixed.dmg` is 19.3 GB.
11. Unit-key-machine-dependent findings went from 28 (H5) to 0 in A. The fleet read was ok in A, so the detector changed or the drift was resolved. Not verifiable from one snapshot.

## Process notes (lane)

- Snapshot A raw files and B raw files are in the lane scratch folder (`scratchpad/H7/A`, `scratchpad/H7/B`), not in the worktree.
- I created four stray files in the repo root (`fleet.json`, `fleet.err`, `rc.txt`, `rx.txt`) at 11:18 and 11:19 by a wrong redirect path. I deleted all four at 11:19 (verified by stat before removal). The replica may have carried them for about a minute.
- I ran one `git status --short` in the repo root at 11:37 and stopped it with TaskStop after about 2 minutes. It is read-only in intent but can refresh `.git/index`. No file was written by me otherwise.
- No process was killed by me except my own stopped task. No file other than this report was changed.

## Not measured

- Per-machine cache hit ratio in the window, per-machine offloadedOut and skip reasons at B (fleet reader failed).
- Per-machine worker why/headroom at B (same cause).
- Storage reader at B (failed in A and B).
- Progress file for the running `repo show projects` read (none found).
- Unit-key findings at B.
