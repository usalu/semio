# 🩺 Wave 1122 Audit H5 (Pulse-2): Fleet, Cache, Remote Execution

Read-only. Two snapshots 20 minutes apart. Sources: `repo fleet status --json`, `repo remote-cache status`, `repo remote-execution status`, `S/origins/*/ready.json` and their stage logs, `df`, `uptime`, `vm.swapusage`, `fseventsd`. Raw outputs were kept in a generated folder and deleted afterwards; the values below are verbatim.

## Snapshot A (03:09:16 CET)

### A1 Fleet and workers (`repo fleet status --json`, 03:09)

| Machine | Platform | Liveness | Recorded | Stage (index) | Attempts | Refusal | Exec headroom | Exec offloadedIn | Exec running |
|---|---|---|---|---|---|---|---|---|---|
| g1-m01 | macos | working | running | workspace (5) | 0 | - | no | 8 | 1 |
| g1-m02 | macos | working | running | workspace (5) | 8 | - | no | 0 | 0 |
| g1-m03 | macos | working | running | workspace (5) | 3 | - | yes | 17 | 1 |
| g1-m04 | linux | working | waiting | dependencies (3) | 10 | disk-collecting | no | 0 | 0 |
| g1-m05 | windows | silent | running | dependencies (3) | 14 | - | not listed | not listed | not listed |
| g1-m06 | macos | working | running | tools (4) | 7 | - | no | 0 | 0 |
| g1-m07 | macos | working | running | workspace (5) | 0 | - | no | 1 | 0 |
| g1-m08 | macos | working | running | workspace (5) | 0 | - | no | 0 | 0 |
| g1-m09 | - | absent | - | - | - | - | not listed | not listed | not listed |
| g1-m10 | macos | working | running | workspace (5) | 1 | - | no | 16 | 1 |
| g1-m11 | macos | silent | backoff | tools (4) | 21 | stage-failed | yes | 12 | 0 |
| g1-m12 | macos | working | running | tools (4) | 29 | - | no | 0 | 0 |
| g1-m13 | macos | working | running | tools (4) | 0 | - | no | 0 | 0 |
| g1-server | - | absent | - | - | - | - | no | 7 | 0 |

Counts: ready 0, working 10, silent 2, absent 2. Summary: running 6, blocked 0, waiting 1, alerts 0, warnings 216. Memory: pressure warn, available 8387 MB, swap used 20103 MB, load 40.22, admitted 8818 MB, observed 5990 MB, under-reserved 1287 MB, grants 6, waiters 1.
Cache hit/miss/skipped per machine and offloadedOut: not measured. The `fleet` reader failed (`The reader did not answer within 5 s`).
Findings (30): memory-pressure (warn), under-reserved (warn, pid 8636: 1591 MB used, 720 MB admitted), 28 x unit-key-machine-dependent (warn), sealed-drifted (info, 2300 changes), worker-absent (info, g1-m09).
unit-key-machine-dependent detail: 14 on g1-m02,g1-m06 (args.diagnostics), 8 on g1-m11,g1-m12 (args.diagnostics), 3 and 1 on g1-m02,g1-m06 with extra components (args.metadata, extern:syn, extern:arrayvec, extern:smallvec, extern:bytemuck_derive), 1 on g1-m02,g1-m12. Crates include build_script_build (9), bitflags (2), adler2, allocator_api2, hashbrown, kurbo.

### A2 Remote cache (`repo remote-cache status`, 03:09)

| Item | Value |
|---|---|
| Server reachable | yes (probes 404 reachable, authorised) |
| entries / bytes | 9591 / 10519455 |
| AC hits / misses / puts | 750 / 6517 / 539 (conflicts 0, rejected 0, stale 0) |
| CAS hits / misses / puts | 99684 / 7722 / 5427 (conflicts 0, rejected 105, stale 0) |
| nx hits / misses / puts | 0 / 6 / 0 |
| Peers tier hits / misses / errors / skipped | 0 / 0 / 0 / 3603 |
| Divergences count | 98 (byClass: closure 1, owner-pool 1, project-inference 16, unit 80) |
| Evidence | 93, 267900069 bytes (project-inference 27, unit 64, closure 1, owner-pool 1) |
| Superseded | project-inference 3 |
| Advisory | none |

### A3 Remote execution (`repo remote-execution status`, 03:09)

Counters: queued 1, executing 3, completed 30, cached 0, failed 72, cancelled 65, retried 1, deduplicated 0, refused 2, offloadRejected 72. Cache answers: true.
Workers (headroom, running, offloadedIn, refused): g1-m04 (no, 0, 0, 0), g1-m10 (no, 1, 16, 0), g1-m11 (yes, 0, 12, 1), g1-m08 (no, 0, 0, 0), g1-m12 (no, 0, 0, 0), g1-server (no, 0, 7, 0), g1-m07 (no, 0, 1, 1), g1-m13 (no, 0, 0, 0), g1-m02 (no, 0, 0, 0), g1-m01 (no, 1, 8, 0), g1-m03 (yes, 1, 17, 0), g1-m06 (no, 0, 0, 0). g1-m05 is not a registered exec worker in this snapshot. Offloaded in total: 61.

### A4 Origins (`S/origins`, 03:09)

| Origin | Ready | Stage | State | Refusal | Last line of current stage log |
|---|---|---|---|---|---|
| g1-m01 | false | workspace | running | - | `…/.cache/b/t/m: 5239 of 8191` |
| g1-m02 | false | workspace | running | - | `…/.cache/b/t/m: 5436 of 8191` |
| g1-m03 | false | workspace | running | - | `…/.cache/b/t/m: 5394 of 8191` |
| g1-m04 | false | dependencies | waiting | disk-collecting | `[script] … "stage":"complete"` (stale) |
| g1-m05 | false | dependencies | running | - | `[repository:command] still running after 980 s` |
| g1-m06 | false | tools | running | - | `…/.cache/b/t/m: 5575 of 8191` |
| g1-m07 | false | workspace | running | - | `…/.cache/b/t/m: 5340 of 8191` |
| g1-m08 | false | workspace | running | - | `…/.cache/b/t/m: 5469 of 8191` |
| g1-m09 | no ready.json | - | - | - | - |
| g1-m10 | false | workspace | running | - | `…/.cache/b/t/m: 5381 of 8191` |
| g1-m11 | false | tools | waiting | sidecar-restarting | `…/.cache/b/t/m: 5496 of 8191` |
| g1-m12 | false | tools | running | - | `…/.cache/b/t/m: 5542 of 8191` |
| g1-m13 | false | tools | running | - | `…/.cache/b/t/m: 5649 of 8191` |
| g1-server | no ready.json | - | - | - | - |

### A5 Host (03:09)

`/System/Volumes/Data`: 926Gi size, 846Gi used, 36Gi avail, 96%. Uptime load averages 39.24 43.34 42.65. Swap total 21504.00M, used 20103.19M, free 1400.81M. fseventsd RSS 6743872 KB, 104.9% CPU.

## Snapshot B (03:29:50 CET, 20 min 34 s after A)

### B1 Fleet and workers (03:29)

| Machine | Platform | Liveness | Recorded | Stage (index) | Attempts | Refusal | Exec headroom | Exec offloadedIn | Exec running |
|---|---|---|---|---|---|---|---|---|---|
| g1-m01 | macos | working | running | workspace (5) | 0 | - | no | 36 | 1 |
| g1-m02 | macos | working | running | workspace (5) | 8 | - | yes | 12 | 0 |
| g1-m03 | macos | working | running | workspace (5) | 3 | - | yes | 76 | 0 |
| g1-m04 | linux | working | waiting | dependencies (3) | 10 | disk-collecting | no | 0 | 0 |
| g1-m05 | windows | working | running | dependencies (3) | 14 | - | yes | 0 | 0 |
| g1-m06 | macos | working | running | tools (4) | 7 | - | no | 0 | 0 |
| g1-m07 | macos | working | running | workspace (5) | 0 | - | yes | 19 | 0 |
| g1-m08 | macos | working | running | workspace (5) | 0 | - | no | 0 | 0 |
| g1-m09 | - | absent | - | - | - | - | not listed | not listed | not listed |
| g1-m10 | macos | working | running | workspace (5) | 1 | - | no | 143 | 0 |
| g1-m11 | macos | working | backoff | tools (4) | 22 | stage-failed | no | 24 | 0 |
| g1-m12 | macos | working | running | workspace (5) | 0 | - | no | 3 | 0 |
| g1-m13 | macos | working | running | tools (4) | 3 | - | no | 23 | 0 |
| g1-server | - | absent | - | - | - | - | no | 55 | 0 |

Counts: ready 0, working 12, silent 0, absent 2. Summary: running 5, blocked 0, waiting 0, alerts 0, warnings 3. Memory: pressure warn, available 7002 MB, swap used 20698 MB, load 66.86, spare 1049 MB, grants 4, waiters 0.
Fleet reader failed again (`The reader did not answer within 5 s`) and so did the storage reader. Cache hit/miss/skipped per machine: not measured. Findings reduced to 5: memory-pressure, source-failed (fleet), source-failed (storage), sealed-drifted (2301 changes), worker-absent (g1-m09). unit-key-machine-dependent: not present in this read (the fleet reader failed, so the list is probably partial; not measured).

### B2 Remote cache (03:29 and re-check 03:32:21)

`repo remote-cache status` did not answer: `serverFault` reason `unreachable`, message `The cache server did not answer the status request in time.` Probes (3) reachable false, status null. Endpoint 192.168.178.70:6094. Counters, entries, bytes and divergences: not measured. The 03:32:21 re-check gave the same result.

### B3 Remote execution (03:29)

Counters: queued 0, executing 1, completed 267, cached 0, failed 176, cancelled 387, retried 19, deduplicated 0, refused 8, offloadRejected 174. Cache answers: false (the cache server is unreachable from the scheduler).
Workers (headroom, running, offloadedIn, refused): g1-server (no, 0, 55, 2), g1-m03 (yes, 0, 76, 1), g1-m12 (no, 0, 3, 0), g1-m01 (no, 1, 36, 0), g1-m02 (yes, 0, 12, 0), g1-m11 (no, 0, 24, 0), g1-m04 (no, 0, 0, 0), g1-m07 (yes, 0, 19, 3), g1-m10 (no, 0, 143, 0), g1-m13 (no, 0, 23, 0), g1-m08 (no, 0, 0, 0), g1-m06 (no, 0, 0, 0), g1-m05 (yes, 0, 0, 0). Offloaded in total: 391.

### B4 Origins (03:29, stage and logs re-read at 03:32)

| Origin | Ready | Stage | State | Refusal | Last line of current stage log (03:29) | Same line at 03:32 |
|---|---|---|---|---|---|---|
| g1-m01 | false | workspace | running | - | `…/.cache/b/t/m: 5239 of 8191` | `5239 of 8191` |
| g1-m02 | false | workspace | running | - | `…/.cache/b/t/m: 5436 of 8191` | `5436 of 8191` |
| g1-m03 | false | workspace | running | - | `…/.cache/b/t/m: 5394 of 8191` | `5394 of 8191` |
| g1-m04 | false | dependencies | waiting | disk-collecting | `[script] … "stage":"complete"` | unchanged |
| g1-m05 | false | dependencies | running | - | `C:\semio: 2086 of 803224` | `C:\semio: 0 of 803224` |
| g1-m06 | false | tools | running | - | `…/.cache/b/t/m: 5575 of 8191` | `5575 of 8191` |
| g1-m07 | false | workspace | running | - | `…/.cache/b/t/m: 5340 of 8191` | `5340 of 8191` |
| g1-m08 | false | workspace | running | - | `…/.cache/b/t/m: 5469 of 8191` | `5468 of 8191` |
| g1-m09 | no ready.json | - | - | - | - | - |
| g1-m10 | false | workspace | running | - | `…/.cache/b/t/m: 5381 of 8191` | `5381 of 8191` |
| g1-m11 | false | tools | backoff | stage-failed | `…/.cache/b/t/m: 5496 of 8191` | `5496 of 8191` (log mtime 03:28:51) |
| g1-m12 | false | workspace | running | - | `…/.cache/b/t/m: 5520 of 8191` | `5520 of 8191` |
| g1-m13 | false | tools | running | - | `…/.cache/b/t/m: 5649 of 8191` | `5649 of 8191` |
| g1-server | no ready.json | - | - | - | - | - |

Ready count: 0 in both snapshots.

### B5 Host (03:31, read at the end of the snapshot)

`/System/Volumes/Data`: 926Gi size, 824Gi used, 57Gi avail, 94%. Uptime load averages 66.01 51.73 42.47. Swap total 21504.00M, used 20876.12M, free 627.88M. fseventsd RSS 7982256 KB, 104.4% CPU.

## Deltas (A to B, 20 min 34 s)

- Hit ratio per machine in the window: not measured. The fleet reader failed in both snapshots and the cache server was unreachable in B.
- Hit ratio, cluster-wide, A-window (03:07 to 03:09, the only window with cache data): AC 650 hits and 5697 misses (10.2%); CAS 58597 hits and 4644 misses (92.6%). Per-consumer AC hits in that window: g1-m12 +258, g1-m08 +105, g1-m01 +70, g1-server +67, g1-m07 +47, g1-m03 +32, g1-m10 +24, g1-m13 +18, g1-m02 +15, g1-m11 +10, g1-m06 +2, g1-m04 +2, g1-m05 0. Sum 650.
- Offloads in the window: offloadedIn sum 61 to 391 (+330). Remote execution counters: completed 30 to 267 (+237), failed 72 to 176 (+104), cancelled 65 to 387 (+322), retried 1 to 19 (+18), refused 2 to 8 (+6), offloadRejected 72 to 174 (+102).
- Divergences added: none in the A-window (98 at 03:07 and 98 at 03:09). Evidence 92 to 93, superseded gained project-inference 3. B: not measured.
- Stage changes: g1-m12 tools (4, attempts 29) to workspace (5, attempts 0). g1-m05 silent to working. g1-m11 silent to working (still backoff, stage-failed, attempts 21 to 22). g1-m13 attempts 0 to 3. Other stages unchanged.
- Ready count: 0 to 0.
- Headroom: 2 workers (g1-m11, g1-m03) to 4 workers (g1-m03, g1-m02, g1-m07, g1-m05).
- Disk: available 36Gi to 57Gi. Swap used 20103 MB to 20876 MB. Load 1 min 39.24 to 66.01 (uptime). fseventsd RSS 6743872 KB to 7982256 KB.

## Anomalies

1. The remote cache server at 192.168.178.70:6094 became unreachable between 03:09 and 03:29 and was still unreachable at 03:32:21. The remote-execution service reports `cache.answers: false`. Fleet and storage readers timed out too. This is the top item: builds must fall back to local.
2. Stage progress counters are frozen. The workspace counters `…/.cache/b/t/m: N of 8191` are identical at 03:09, 03:29 and 03:32 for g1-m01, g1-m02, g1-m03, g1-m06, g1-m07, g1-m10, g1-m11, g1-m12 and g1-m13 (g1-m08 moved 5469 to 5468). The log mtimes are fresh (the writers are alive), so the progress reading is either stalled or not updating.
3. g1-m05 Windows dependencies counter reset: `C:\semio: 2086 of 803224` at 03:29 to `C:\semio: 0 of 803224` at 03:32. It was `[repository:command] still running after 980 s` at 03:09.
4. g1-m04 is still refused with `disk-collecting` (needs 6144 MB, 2624 MB free at 03:09). Its last stage-log line is a stale `stage: complete`.
5. g1-m11 is in backoff with `stage-failed` (exit status 1), attempts 21 to 22, in tools. g1-m09 and g1-server have no ready.json and are absent from the fleet.
6. Remote cache entries jumped from 118 (175299 bytes, at 03:07, `startedAt` 2026-11-22T02:06:32.255Z) to 9591 (10519455 bytes, at 03:09, same `startedAt`). Unexplained. Divergences are 98 now, not the 1000 measured at 00:38. The peers tier is 0/0/0 with 3603 skipped (earlier 0 hits, 704 misses, 20 errors).
7. Unit keys differ by machine: 28 `unit-key-machine-dependent` findings at 03:09 (g1-m02,g1-m06 and g1-m11,g1-m12; component args.diagnostics, sometimes with extern:syn or args.metadata). Not listed at 03:29 (fleet reader failed).
8. Fleet summary warnings fell from 216 (03:09) to 3 (03:29), while the findings list shrank from 30 to 5. This fits the reader failure, so the drop is probably measurement loss, not recovery.
9. Memory: swap 20.1 to 20.9 GB of 21.5 GB used, pressure warn, 1-minute load 40 to 67. fseventsd RSS grew from 6.4 GiB to 7.6 GiB (the brief said 9.6 GB at 00:38).
10. The remote-execution counters rose quickly (failed +104, cancelled +322 in 20 min). The cancelled count is high relative to completed (+237).

## Not measured

Per-machine cache hit/miss/skipped and offloadedOut (fleet reader failed in A and B). Cache counters and divergences at B (server unreachable). The unit-key findings at B. Hit ratio per machine.
