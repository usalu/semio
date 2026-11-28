# Audit A5: Live Effect After Wave 1127 (measured 2026-11-27 06:19-06:25 CET)

Read-only lane. Raw outputs were captured in `wave-1127/🗑️generated/A5/` (fleet-status.json, remote-cache-status.txt, remote-execution-status.txt) and that folder is deleted at the end of this audit. Comparison baseline is A4 (03:10-03:12 CET).

## 1. Ready per origin

Ready 10 of 14 workers (A4: 6): m01, m03, m06, m08, m10, m11, m12, m13, m14, m15. Working 4: m02, m04, m05, m07. Absent 2: m09, g1-server (no ready.json).

| Origin | State | Current stage / wait or failure line |
|---|---|---|
| m02 | waiting (exit 1, 13 attempts) | workspace, `image-short`: needs 2131 MB free in the Rust cache image `.cache/b/r/m`, only 1506 of 21303 MB free after in-place collection. Looked at again every 5 min. Log tail: "still running after 800 s". |
| m04 | running (adopted) | tools: "still working after 18411 s; last phase started: source.served". No new failure line. |
| m05 | running | native: "showed nothing new for 10 minutes and was stopped; started again at once". |
| m07 | running (adopted), 56 attempts | workspace: "waiting 964 s for run-many: not enough free memory (need 512 MB; short by 1071 MB)". Peak 10662 MB. |

## 2. Windowed rates (sum over origins, 15 min / 60 min)

- Unit cache hit 233,148 / 362,098 (A4 15 min: 68,775). Miss 31,192 / 83,585 (A4: 27,401).
- Skips, 15 min / 60 min: spool-full 0 / 0; upload-failed 0 / 0; upload-expired-down 0 / 0; upload-entry-gone 3 / 6; diverged 0 / 1; path-dependent 14 / 42; other 11 / 664. The 60 min "other" total is a burst 15-60 min ago, not a current rate.
- Offload out 494 / 573 (A4: 37 / 196); refused 1 / 4; fellback 3 / 9.
- Declines, 15 min / 60 min: local-slot 1108 / 5609; cheap 1169 / 2756; not-portable 68 / 194; unmeasured 34 / 36; infrastructure 2 / 7; no-idle-machine 1 / 4; unmapped-path 3 / 3; other 3 / 5; queue-timeout 0 / 0.
- cache.remote, 15 min / 60 min: ac hit 5375 / 40789; ac miss 15732 / 58221; ac put 3612 / 11616; ac conflict 5 / 8; cas hit 225860 / 337139; cas miss 30483 / 79292; cas put 30564 / 69661; refused 0 / 0.
- CAS store: 93.7 % used (usedBasisPoints 9367), last collection 238 s before the snapshot.

## 3. Nx remote cache

- Server cumulative counters: hits 14, misses 605, puts 225 (A4: 4 / 456 / 171). Since the 02:36 server start: hits 10, misses 138, puts 54.
- Windowed: 15 min hit 0 / miss 11 / put 6; 60 min hit 4 / miss 50 / put 19.
- Worker logs: `promotion withheld` 0 lines and `remote cache hit` 0 lines across `S/origins/*/ready-log-*.txt`. The H37 bilingual "cache promotion withheld" line (landed 05:01) does not appear in any ready log yet.

## 4. Remote cache divergences

- Total 1257 (A4: 1135, so +122 in about 3.1 h, about +39/h, not +53/h).
- By class (cumulative): unit 1188, owner-pool 41, project-inference 16, closure 12.
- Current bucket (since 03:35 CET): 113, all unit class.
- The 20 most recent entries span 04:35 to 06:00 CET, all ac kind, unit sort (differing: rlib, rmeta, rmeta.semio-deps). The divergences continued after H15's threads=1 fix landed at 04:00 CET.
- The status has no timestamp per divergence, so an exact count newer than 04:00 CET by kind is not available. The status field `lastHour` (1) conflicts with the current bucket (113) and was not used.

## 5. ENOSPC / "No space left" after 06:00 CET

- 6 lines in 2 ready-log files in total. The log lines carry no timestamp.
- Only one file was written after 06:00 CET: `g1-m02/ready-log-workspace.txt` (last write 06:08 CET, 2 ENOSPC lines: cargo `mkdtemp` and `copyfile` under `.cache/b/r/m`). This matches the m02 `image-short` refusal in section 1. The other file was last written before 06:00 CET.

## 6. Stalls in ready-timings.json since 05:00 CET

- 35 runs ended since 05:00 CET: stage-failed 28, ok 2, stalled 5 (m02, m03, m05, m07, m14: one each).
- Failure-heavy origins: m12 7, m11 6, m15 4, m02 3.
- A4 window (since 02:00 CET) had 4 stalls in 46 runs. The stall count did not drop further; stage-failed dominates.

## 7. Warden peaks (memory.json lastAlarms rings, 8 per origin)

- actor_input_decoder: 1836 MB, m14 local-unit rustc `semio_plugin_actor_input_decoder` (about 06:19 CET; same pid also 1717 MB). A4 host peak was 1806 MB (m12). Up.
- actor_input_wire: no alarm in any ring (A4: 1633 MB m01, 1628 MB m12). Not confirmed as reduced, only absent from the ring.
- process3d_host: no alarm in any ring (A4: none in ring).
- Largest alarm in the rings: slice-worker 6146 MB on m07 (A4: 6807 MB on m07).

## Summary of effect vs A4

- Positive: Ready 10 (A4: 6); offload out 494 in 15 min (A4: 37); unit-cache skip storm (spool-full, upload-failed, upload-expired-down) still 0.
- Negative: divergences continue after H15 (about +39/h, 20 of the newest after 04:35 CET); Nx hits still 14 cumulative and no `remote cache hit` lines in any worker log; m02 blocked on image space in the `r` cache image (ENOSPC lines at 06:08 CET); stalls still 5 of 35 runs since 05:00 CET; actor_input_decoder peak up to 1836 MB.
- Not verified: actor_input_wire and process3d_host peaks (absent from current rings, not proven lower).
