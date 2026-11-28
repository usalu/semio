# Audit A6: Live Effect After Wave 1127 (measured 2026-11-27 10:28-10:31 CET)

Read-only lane. Raw outputs were captured in `wave-1127/🗑️generated/A6/` (fleet-status.json, remote-cache-status.txt, remote-execution-status.txt, ready-log counts, ready-timings aggregate, warden rings) and that folder is deleted at the end of this audit (done after this report was written). Comparison baselines: A5 (06:19-06:25 CET) and A4 (03:10-03:12 CET), as recorded in audit-A5-effect.md.

## 1. Ready per origin

Ready 11 of 14 workers (A5: 10, A4: 6): m01, m02, m03, m06, m07, m08, m11, m12, m13, m14, m15.

| Origin | Liveness | Stage | Non-ready reason |
|---|---|---|---|
| m04 (linux) | working | tools (stage 4/5) | adopted run, heartbeat 3 s old, no refusal |
| m05 (windows) | silent | tools (stage 4/5) | record 3931 s old, heartbeat 205 s old, 1 timeout in 12 h; memory mode observe |
| m10 (macos) | working | workspace (stage 5/5) | attempt 35; 12 h: 31 failed, 6 stalled; warden memory guard hold |
| m09 | absent | - | no worker row |
| g1-server | absent | - | coordinator, no worker row |

Fleet summary: running 3, blocked 0, waiting 0, alerts 1 (`worker-warden-intervened` on m14: slice-worker cancelled at about 10:23 CET, overdue), warnings 44 (unit-key-machine-dependent, typegen).

## 2. Windowed rates (sum over origins, 15 min / 60 min; A5 15 min in brackets)

- Unit cache, local: hit 389,389 / 759,674 [A5 233,148]; miss 37,249 / 102,916 [A5 31,192].
- Unit cache skips, 15 min / 60 min: diverged 3 / 10 [A5 0 / 1]; path-dependent 8 / 116 [A5 14 / 42]; other 16 / 63 [A5 11 / 664]; upload-entry-gone, upload-failed, upload-expired-down, upload-expired, spool-full, read-only, breaker-denied, key-incomplete, upload-dropped: 0 / 0. The skip storm stays stopped.
- Offload out 858 / 1,207 [A5 494 / 573]; fellback 3 / 4; refused 1 / 1.
- Declines by reason, 15 min / 60 min: local-slot 1,357 / 10,699; cheap 1,073 / 3,368; not-portable 54 / 277; unmapped-path 2 / 5; infrastructure 1 / 2; queue-timeout 1 / 1; other 2 / 2; no-idle-machine, pin-failed, unmeasured, worker-policy, incomplete-inputs: 0 / 0. (A5 15 min: local-slot 1108, cheap 1169.)
- Test-file offloads: not visible in the fleet rates (no test-file key). Remote-execution counters since the 10:07 CET server start: queued 0, executing 1, completed 1,096, failed 14, refused 11, offloadRejected 13, cancelled 1.
- cache.remote, 15 min / 60 min: ac hit 13,420 / 61,439 [A5 5,375 / 40,789]; ac miss 15,653 / 75,494 [A5 15,732 / 58,221]; ac put 4,556 / 20,436 [A5 3,612 / 11,616]; ac conflict 3 / 13; cas hit 374,470 / 707,060 [A5 225,860 / 337,139]; cas miss 36,133 / 96,915 [A5 30,483 / 79,292]; cas put 35,092 / 95,762 [A5 30,564 / 69,661]; refused 0 / 0.
- CAS store: 78.6 % used (usedBasisPoints 7860; A5 93.7 %), last collection 255 s before the snapshot, 3,921 leased and 159 expired entries.

## 3. Nx remote cache and ready logs

- Server cumulative Nx counters: hits 20, misses 742, puts 278, conflicts 7 (A5: 14 / 605 / 225; A4: 4 / 456 / 171). Delta since A5: +6 hits, +137 misses, +53 puts. Since the 09:01 UTC server start: hits 5, misses 27, puts 10.
- Client windows: Nx hit 2 / 5, miss 8 / 32, put 5 / 16 (15 min / 60 min).
- Ready-log files `ready-log-workspace*.txt` (64 files across 15 origins): "cache promotion withheld" lines 0, "remote cache hit" lines 0.
- ENOSPC / "No space left" lines: 0 in every current workspace log. A5's two m02 ENOSPC lines are no longer in `ready-log-workspace.txt` (rewritten at 10:06 CET); not a proven fix.

## 4. Remote cache divergences per hour and kind (CET, since 04:00)

Total in status: 1,277 (A5: 1,257; +20 in about 3.5 h). By class: unit 1,208, owner-pool 41, project-inference 16, closure 12.

| Hour (CET) | Count | By kind |
|---|---|---|
| 04:00 | 93 | rlib+rmeta 23, stream 70 |
| 05:00 | 0 | - |
| 06:00 | 10 | dep-info+rlib+rmeta+stream 3, rlib+rmeta+stream 3, rmeta 1, stream 3 |
| 07:00 | 0 | - (H49 landed) |
| 08:00 | 1 | stream 1 (H55 key change landed) |
| 09:00 | 3 | stream 3 |
| 10:00 (partial, to 10:15) | 5 | dep-info+rlib+rmeta 1, stream 4 |

Since 04:00 CET: 112. Since the 07:00 CET fix: 9 in about 3.5 h (about 2.6/h), all stream except one. Current bucket: 7. The status field `lastHour` (8) and `hourly` disagree with the current bucket by design (not reconciled).

## 5. ENOSPC after 07:00 CET per origin

0 lines in every origin's workspace log. Nothing to report per origin.

## 6. Stalls and stage failures since 08:00 CET (ready-timings.json)

34 runs ended: ok 10, stage-failed 20, stalled 4 (A5 since 05:00: 35 runs, 28 stage-failed, 5 stalled). Per origin: m02 8 stage-failed (workspace); m07 6 stage-failed + 1 stalled; m10 3 stalled + 1 stage-failed; m12 2 stage-failed (workspace, dependencies); m06, m08, m15 1 stage-failed each. m11, m13, m14 clean.

## 7. Warden peaks (memory.json alarm rings, 8 per origin)

- actor_input_decoder: no alarm in any current ring. A5 peak of 1836 MB (m14, 06:19 CET) has rotated out. Not proven lower.
- actor_input_wire: no alarm in any current ring. Not proven lower (A4: 1633 MB).
- process3d_host: no alarm of that crate. One remote-unit bun alarm on m11 (1744 MB, time not extracted) mentions process3d_host in its text; owner is the remote-execution bun, not confirmed as the host.
- Largest alarms now: rustc `semio_s_artifact_stdio_pdf_pdf_snapshot_value_tests` on m12, 5451 MB (09:22 CET); slice-worker on m02, 4505 MB (08:50 CET). A5 largest was slice-worker 6146 MB on m07 (rotated out).
- Rings hold 50 alarms in total across 10 origins. The rings rotate fast, so absence is weak evidence.

## 8. Coordinator

- Host: semio-g1-m08 (hostname; also the warden origin shown in fleet status). Load 41.47 / 38.99 / 38.40 (A5: 57). Swap 14,864 MB used of 16,384 MB (A5: 12.2 / 13.3 GB). Free RAM 12.8 GB (fleet memory), spare 7.2 GB.
- Free disk on `/`: 94 GiB available, 13 % used (host free 100.99 GB per fleet status). Cache volume: CAS 78.6 % of cap.
- Cache server (192.168.178.70:6094): started 10:01 CET (startedAt 09:01:16 UTC), uptime about 29 min at measurement; ledger restarts 41 (cumulative).
- Remote-execution server (192.168.178.70:8980/8981): started 10:07 CET, about 23 min.

## Summary vs A5 and A4

- Better: Ready 11 (A5 10, A4 6); offload out 858 per 15 min (A5 494); cache.remote ac hits and cas hits up; CAS usage down from 93.7 % to 78.6 %; zero ENOSPC; zero skip storm; divergences after 07:00 CET about 2.6/h.
- Worse or unchanged: Nx remote still almost no hits (cumulative 20 hits / 742 misses, +6 hits in 4.5 h); zero "remote cache hit" and zero "cache promotion withheld" lines in worker logs; local-slot declines rose to 1,357 per 15 min (A5 1,108); 60 min stalls/stage-failures still high (20 stage-failed, 4 stalled in 34 runs since 08:00 CET); divergences still present (stream kind) after H55.
- Not verified: actor_input_decoder and actor_input_wire peaks (absent from current rings); test-file offloads (not in status output).
