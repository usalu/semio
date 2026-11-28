# Audit A8: Final Effect Of Wave 1127 (measured 2026-11-27 23:03-23:07 CET)

Read-only lane. Sources: `repo fleet status --json` (summary and `--detail`), `repo remote-cache status`, `repo remote-execution status`, origin files under `/Users/Shared/semio-store/v1/origins/<o>/` (ready.json, ready-timings.json, memory.json, sidecar-code.json, ready-log-*.txt). All times CET. Raw outputs were in `wave-1127/🗑️generated/A8/`, deleted at the end.

Columns: 00:00 values are the "before" figures recorded in audit-A4; "n/r" means not recorded. Windows are 15 min / 60 min unless marked.

## 1. Ready per origin

| Measure | 00:00 | A4 03:15 | A5 06:25 | A6 10:30 | Now 23:06 |
|---|---|---|---|---|---|
| Ready workers | 8 | 6 | 10 | 11 | 9 (m02, m06, m07, m08, m11, m12, m13, m14, m15) |

Non-ready origins now:
- g1-m01: offline 6.0 h. ready.json still says ready (stale record, 21539 s old), so it is not counted as ready.
- g1-m03: backoff, stage workspace failed (exit 1), 15 attempts, peak 10473 MB. Failure: `failed to run custom build command for serde_core v1.0.228` inside `@semio-tech/norm-din4108-rs:build:prototype`. Fleet shows it as "working" with refusal stage-failed.
- g1-m04: running, stage tools (4 of 5), adopted, 3 attempts. No failure line.
- g1-m05: running (Windows), stage workspace (5 of 5), adopted, 4 attempts, tree diverged.
- g1-m10: waiting, stage workspace, refusal image-short, 42 attempts. Needs 1619 MB free in `.cache/b/r/m` but only 535 of 16183 MB free after in-place collection. Retried every 5 min.
- g1-m09 and g1-server: absent, no ready.json.

## 2. Unit cache, offload, cache.remote, Nx

| Measure | 00:00 | A4 03:15 | A5 06:25 | A6 10:30 | Now 23:06 |
|---|---|---|---|---|---|
| cache.local hit (15 / 60) | n/r (m01 cum. hit 82.8k) | 68775 / 228756 | 233148 / 362098 | 389389 / 759674 | 18380 / 183052 |
| cache.local miss (15 / 60) | n/r (m01 cum. 270k) | 27401 / 111356 | 31192 / 83585 | 37249 / 102916 | 24009 / 97974 |
| skip upload-entry-gone (15 / 60) | n/r (cum. 18.0k) | 0 / 42 | 3 / 6 | 0 / 0 | 877 / 905 |
| skip path-dependent (15 / 60) | n/r | 17 / 82 | 14 / 42 | 8 / 116 | 37 / 63 |
| skip diverged (15 / 60) | n/r | 15 / 51 | 0 / 1 | 3 / 10 | 0 / 0 (257 in 24 h) |
| skip upload-failed / spool-full (15 / 60) | n/r (cum. 14.7k / 27.9k) | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 |
| offload out (15 / 60) | n/r (cum. 6531) | 37 / 196 | 494 / 573 | 858 / 1207 | 0 / 38 |
| offload fellback (15 / 60) | n/r (cum. 3029) | 3 / 13 | 3 / 9 | 3 / 4 | 137 / 158 |
| offload refused (15 / 60) | n/r (cum. 895) | 9 / 16 | 1 / 4 | 1 / 1 | 0 / 0 (104 in 24 h) |
| decline local-slot (15 / 60) | n/r (cum. 12.8k) | 1653 / 7777 | 1108 / 5609 | 1357 / 10699 | 2361 / 7619 |
| decline cheap (15 / 60) | n/r (cum. 5.1k) | 1608 / 4284 | 1169 / 2756 | 1073 / 3368 | 321 / 1928 |
| cache.remote AC hit (15 / 60) | n/r | 15113 / 48371 | 5375 / 40789 | 13420 / 61439 | 1054 / 11483 |
| cache.remote AC miss (15 / 60) | n/r | 22573 / 81619 | 15732 / 58221 | 15653 / 75494 | 15878 / 78424 |
| cache.remote CAS hit (15 / 60) | n/r | 51085 / 168106 | 225860 / 337139 | 374470 / 707060 | 18345 / 161388 |
| cache.remote CAS miss (15 / 60) | n/r | 21530 / 78195 | 30483 / 79292 | 36133 / 96915 | 24019 / 97936 |
| Nx remote hit / miss / put (15 min client) | n/r (server 4 / 403) | 0 / 12 / 8 | 0 / 11 / 6 | 2 / 8 / 5 | 0 / 5 / 0 |
| Nx server cumulative hit / miss / put | 4 / 403 | 4 / 456 / 171 | 14 / 605 / 225 | 20 / 742 / 278 | 23 / 975 / 372 |

Notes:
- The unit-cache skip storm of A4 (upload-entry-gone, upload-failed, spool-full) was zero through A6. It is back: 877 upload-entry-gone in the last 15 min, almost all from g1-m10 (493) and g1-m15 (365), both 24 h-flat bursts.
- Offload out dropped to 0 in the last 15 min after 858 at A6. The fellback spike (137 in 15 min) matches the infrastructure decline spike (137).
- The remote-execution server restarted at 21:12 CET and the cache server at 21:59 CET. Their counters restarted, so cumulative values are not comparable across the restart. Cache server session since 21:59: Nx hit 0 / miss 15 / put 1.

## 3. Divergences (cache server, CET hour, since 12:00)

Total 1304 (00:00 971-1038; A4 1135; A5 1257; A6 1277). Current bucket since the 21:58 restart: 0.

| Hour (CET) | Count | By kind |
|---|---|---|
| 12:00 | 2 | stream 2 |
| 13:00 | 8 | dep-info+rlib+rmeta 1, dep-info+rlib+rmeta+stream 7 |
| 14:00, 15:00 | 0 | - |
| 16:00 | 5 | dep-info+rlib+rmeta 1, dep-info+rlib+rmeta+stream 4 |
| 17:00 | 1 | stream 1 |
| 18:00 | 1 | dep-info+rlib+rmeta 1 |
| 19:00 | 0 | - |
| 20:00 | 1 | dep-info+rlib+rmeta 1 |
| 21:00-23:00 | 0 | - |

Since 12:00: 18 divergences. The hourly feed gives kinds only, not classes. Total by class: unit 1235, owner-pool 41, project-inference 16, closure 12.

## 4. ENOSPC and stage failures since 16:00

- ENOSPC / "No space left" / os error 28 lines: 0 in every ready-log file (A5: 6 lines, A6: 0 after 07:00). The 23:00 m10 wait is an image-short refusal, not an ENOSPC line.
- ready-timings.json (ring of 40 runs per origin, covers back to before 16:00 on every origin). Runs ended since 16:00: 22 total. stage-failed 12, interrupted 6, ok 3, stalled 1, stage-timeout 0.
- Per origin: m03 stage-failed 6, interrupted 1; m10 stage-failed 1, interrupted 5; m05 stage-failed 4; m08 stage-failed 1, stalled 1, ok 1; m07 ok 1; m14 ok 1.
- Comparison: A4 since 02:00 (46 runs) 39 stage-failed, 4 stalled; A5 since 05:00 (35 runs) 28 stage-failed, 5 stalled; A6 since 08:00 (34 runs) 20 stage-failed, 4 stalled. Now since 16:00 (22 runs) 12 stage-failed, 1 stalled.
- New fields: `failureClass` is absent from every run row. `stopBy` is present on one run (g1-m10, supervisor cancel, 22:22 CET).

## 5. Warden peaks (top 5 of 93 alarm entries, by MB)

| Rank | Origin | MB | Time (CET) | Action / kind | Owner |
|---|---|---|---|---|---|
| 1 | g1-m11 | 11524 | 22:37 | suspend, granted | bun remote-execution unit |
| 2 | g1-m08 | 5776 | 21:32 | suspend, granted | bun `door` |
| 3 | g1-m02 | 5017 | 23:05 | suspend, granted | bun --eval |
| 4 | g1-m02 | 4844 | 23:06 | resume (same pid) | bun --eval |
| 5 | g1-m14 | 4813 | 23:03 | cancel, granted | bun `door` |

Crate-level peaks (from the rings, for comparison with 00:00 and A4): actor_input_decoder 2054 MB (g1-m12, 21:38, remote-unit report); actor_input_wire 2172 MB (g1-m12, 20:32). process3d_host: no alarm in the rings. 00:00 values were process3d_host 1750 MB and actor_input host 1558 MB. Largest alarm overall is now 11524 MB, up from A5's 6146 MB and A6's 5451 MB.

## 6. Coordinator and code convergence

Coordinator (`semio-g1-server.fritz.box`, 32 GB RAM; A6 named semio-g1-m08 as coordinator, which does not match the current findings):
- Load 70.81 (A6 41.47 on m08). Available RAM 5755 MB, pressure warn, swap 25202 MB (76 % of memory). Admitted 15445 MB, observed 16560 MB.
- Free disk: host 72.5 GB (floor 16.1 GB; A6 94 GiB on `/`). CAS 54.3 GB used of an effective cap of 47.0 GB (115 %, reserve distance -7.1 GB). Graph image 95 % full.

Sidecar code (`S/origins/*/sidecar-code.json`, 00:00 n/r):
- Current on all roles: g1-m01 only. Partly current: g1-m04 (spool), g1-m05 (spool), g1-m13 (graph, pool, ready, spool), g1-server (scheduler, pool, spool).
- 11 origins still run agent code `c7f12f52` while disk is `cc5dd436`, the same fleet treeCode gap: g1-m02, m03, m06, m07, m08, m10, m11, m12, m14, m15, g1-server. Graph, pool and ready roles on those origins are also stale against disk.
- g1-m09 has no sidecar.

## Summary

- Better than A6: ENOSPC is zero; the 15-min CAS hit rate is high relative to 00:00 (see table, with restart caveats). Nx cumulative counters (23 hits / 975 misses) are not comparable across the 21:59 cache-server restart.
- Worse than A6: Ready 9 (A6 11); skip upload-entry-gone storm back (877 in 15 min); offload out 0 in 15 min; coordinator swap and load high; warden peak 11524 MB; CAS over its effective cap; only 1 of 14 origins fully on current code.
- Not proven: stall rate since 16:00 is low (1 stalled), but ring coverage is 22 runs, so this is a small sample.
