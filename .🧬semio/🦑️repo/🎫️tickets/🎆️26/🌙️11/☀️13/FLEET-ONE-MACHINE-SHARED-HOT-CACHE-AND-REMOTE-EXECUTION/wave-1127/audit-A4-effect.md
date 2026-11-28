# Audit A4: Live Effect After Wave 1127 (measured 2026-11-27 03:10-03:12 CET)

Read-only lane. Raw outputs were captured in `wave-1127/🗑️generated/A4/` (fleet-status.json, remote-cache-status.txt, remote-execution-status.txt) and that folder is deleted at the end of this audit.

## 1. Ready per origin

Ready 6 of 14 workers: g1-m03, g1-m06, g1-m07, g1-m10, g1-m13, g1-m15. Fleet counts: ready 6, working 7, silent 1 (m05), absent 2 (m09, g1-server). Before: ready 8 at 00:00.

| Origin | State | Current stage / failure line |
|---|---|---|
| m01 | running | workspace, adopted. Last failed log 01:33 "Bounded dense Source owner rows required" (69 > 64), fixed by H26 at 02:35 |
| m02 | backoff | workspace exit 1. Last failed 03:00: Owned Nx invocation failed (124), no progress for 10 min (run-many, deadline) |
| m03 | ready | workspace refreshing in background (inputs changed). Last failed 03:01: stage-timeout, SIGTERM |
| m04 | running | tools, adopted. Last failed log is stale (11-25 12:53): EACCES applying patch file in dependencies |
| m05 | running | native (Windows). Reason: inputs of stage native changed. Last failed 11-26 20:10: EPERM unlink esbuild.exe, "Directory partition capacity exceeded" |
| m08 | running | workspace, adopted. Last failed 02:25: "Bounded dense Source owner rows required" (69 > 64) |
| m11 | backoff | workspace exit 1. Last failed 03:04: "No space left on device (os error 28)" writing .d under .cache/b/r/m (cache volume full) |
| m12 | backoff | workspace exit 1. Last failed 02:54: Owned Nx invocation failed (124), no progress for 10 min |
| m14 | running | workspace, adopted. Last failed 01:46 workspace-failed; log tail shows cache partitions at capacity (b/r/m 16465 of 40959) |
| g1-server | n/a | no ready.json (server role, reported absent in fleet status) |

## 2. Windowed rates (sum over 15 origins; 15 min / 60 min / 24 h)

Unit cache (client-side):
- cache.local.hit 68775 / 228756 / 517337; miss 27401 / 111356 / 210914. Before cumulative mix on m01: hit 82.8k, miss 270k.
- Skips: spool-full 0 / 0 / (none in 24 h); upload-failed 0 / 0; upload-expired-down 0 / 0 / 1; upload-entry-gone 0 / 42 / 149; diverged 15 / 51 / 86; path-dependent 17 / 82 / 189; other 19 / 70 / 166. The big before-skip reasons (spool-full 27.9k, upload-entry-gone 18.0k, upload-failed 14.7k) are cumulative and have stopped growing.
- Fleet cumulative per-machine skip totals: spool-full 209.7k, upload-failed 80.1k, upload-entry-gone 77.1k, upload-expired-down 38.7k (all historic).

Offload:
- Out 37 / 196 / 274 (cumulative 6654, before 6531). Refused 9 / 16 / 56 (cumulative 898, before 895). FellBack 3 / 13 / 39 (cumulative 3045, before 3029).
- Declines 15 min / 60 min: local-slot 1653 / 7777 (cumulative 14023, before 12.8k), cheap 1608 / 4284 (cumulative 5385, before 5.1k), not-portable 96 / 346, no-idle-machine 9 / 15, infrastructure 2 / 11, unmeasured 3 / 6. unmapped-path, pin-failed, queue-timeout and incomplete-inputs are flat (0 to 1 per window).

cache.remote.* (15 min / 60 min / 24 h):
- ac hit 15113 / 48371 / 138942; ac miss 22573 / 81619 / 181026; ac put 4914 / 14452 / 30439; ac conflict 12 / 53 / 86.
- cas hit 51085 / 168106 / 385237; cas miss 21530 / 78195 / 145471; cas put 20564 / 59039 / 122401.
- nx hit 0 / 0 / 2; nx miss 12 / 41 / 112; nx put 8 / 8 / 38. cache.remote.refused 0.

## 3. Nx remote cache

Server cumulative: hits 4, misses 456 (before 4/403). That is 53 more misses and 0 new hits. Session (since 23:34 restart): hits 1, misses 81.

## 4. Remote cache divergences

Count 1135 (before about 971-1038). Newest divergence 2026-11-27 03:11:08 CET (unit class, ac kind). By class (cumulative): unit 1066, owner-pool 41, project-inference 16, closure 12. Last hour: 53. Current window: 95 unit divergences. Divergences are still occurring at about 50 per hour.

## 5. Warden peaks (lastAlarms, 8 per origin)

- process3d_host: no alarm in the current lastAlarms ring (before 1750 MB).
- actor_input_decoder: host 1806 MB (m12, 03:10 CET, still climbing); wasm: no alarm in ring (before 2280 MB wasm).
- actor_input_wire: host 1633 MB (m01) and 1628 MB (m12); wasm: no alarm in ring (before 2269).
- Host-level before value 1558 MB is now exceeded by the decoder (1806 MB), so the host actor_input peak has grown, not shrunk.
- Largest alarm overall: slice-worker 6807 MB on m07 (cancelled 03:09).

## 6. Stalls in ready-timings.json (runs ended since 02:00 CET)

- Since 02:00 CET: 46 runs. stalled 4 (m13 2, m07 1, m14 1), stage-timeout 2, stage-failed 39, ok 1.
- Whole file (ring): 547 runs, stalled 64, stage-timeout 15, stage-failed 428, ok 40. Before 75 stalls / 541 runs. The 4 stalls since 02:00 are a clear drop from the pre-fix rate.

## Summary of effect

- Positive: unit-cache skip storm (spool-full, upload-failed, upload-expired-down) is at 0 in 15 and 60 min windows. Offload fellback and refusals are almost flat. Stalls dropped to 4 since 02:00.
- Negative: Ready fell to 6 (from 8); m11 and m12 hit disk-full or Nx deadline; no new Nx remote hits (still 4); divergences keep climbing (1135, newest 03:11); the actor_input decoder host peak rose to 1806 MB.
- Not verified: the warden process3d_host peak (no current alarm) and wasm actor_input peaks (no current alarm) could not be confirmed as reduced, only as absent from the ring.
