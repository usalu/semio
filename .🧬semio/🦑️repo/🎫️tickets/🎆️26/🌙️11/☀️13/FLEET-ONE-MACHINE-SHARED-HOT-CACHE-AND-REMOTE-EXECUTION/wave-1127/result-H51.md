# H51: g1-m05 (Windows) to Ready (continuation of H7)

Started 07:07 CET 2026-11-27. Evidence only from `S/origins/g1-m05` (no SSH). Times CET.

## Timeline of stages (ready-timings.json, ready.json)
- dependencies: ok 11/26 21:52 (1218 s) after 60 failed attempts (EPERM esbuild.exe, fixed by H7 `retireBusyFileV1`).
- tools (first run): 11/26 22:12 to 11/27 02:54, 16945 s, outcome `stage-timeout` (`soft-cap-idle`, last activity `javascript-preserving-source:324`).
- native: 03:08 to 05:17 outcome `stalled` (exit 0; log ends with `Native bootstrap complete`), rerun 06:19 to 06:52 ok in 1967 s (Node 24.15.0 provisioned, playwright chromium, dotnet 8, rust nightly-2026-07-20 present). Native is done.
- tools (second run): started 06:31, ready.json at 06:59:45 `[repo] still working after 162 s; last phase started: door.capabilities` (a front-door phase that takes seconds on a Mac). Workspace stage not reached.
- From 07:00 (m05 ready.json, memory.json) and 07:13 (replica attest) m05 went completely silent while the host answers on tcp/135 (machine up, fleet processes not writing).

## Partition capacity (the `Directory partition capacity exceeded` of the dependencies failure)
partitions.json (06:53, 190 s census, H7/H28 code now reports it) shows no partition near its cap: `l` 8.9 MB of 67.1 MB (13 %), `n` 574 MB of 1.61 GB (36 %), `o` 12.8 MB of 2.15 GB, `r` 81 MB of 103 GB, `d` 18.8 MB of 38.7 GB (partial), `t` 2.3 MB of 8.6 GB, `g` 45 KB, `p` 29 KB, `c` 8.5 MB (no cap). The refusal came from a transient overflow of the `leases` partition that the H7 in-place relief now reclaims; no capacity change needed and none made.

## What is wrong with m05 now (not a capacity or tool problem)
1. The replica daemon of m05 is crippled: attest `hot` tier passes 0 (m04: 9884), `view` pass 767 s (m04: 10 s), `queue.incoming` 86 and unchanged for 20+ min, `loop-blocked-restart` 91 (82 at 00:00; macOS machines 1-30, m04 Linux 213). The daemon is ended after 120 s of event-loop block and restarts; its open/run phase on a 522k-file tree with Windows lstat cost never reaches a hot pass, so code (H7 fixes, deadline, CPU-only stall rule, census) reaches m05 late or never. This is the likely reason every m05 fix of this wave has no visible effect.
2. The ready sidecar mirror (ready.json, memory.json, partitions.json) stopped at 07:00; no new ready log; tools run status unknown.
3. The only blocker visible in logs is slowness, not an error: front door `door.capabilities` > 160 s, process table read 17 s, memory tick p95 25 s, census 25 files/s.

## Landed / staged by H51
Landed 08:00 (slot, landings.md "Wave 1127 H51"), tests 2+8+8+22+26 pass, typecheck 0 errors:
1. `mirrorLinesV1` (sidecar ready): the live log mirror (`ready-log-<stage>.txt`) drops machine rows and heartbeat sentences and keeps the newest raw line, so a hung stage's last real lines become visible (the 16945 s tools run showed only `still running` lines). Takes effect when m05's sidecar next restarts; the stage log on m05 holds the full text.
2. `loop-blocked-in-<activity>` (shared-worktree resilience + replica supervisor): the activity the last blocked replica daemon sat in is now in the failure list of attest.json of every machine. m05 has 93 loop-blocked restarts (91 at 07:07, +2 at 07:51), hot passes 0 after each restart, attest 10-30 min stale; the name of the blocking activity is what is needed next and appears after the next replica restart that carries this code.
No landing in graph/inputs/workspaces/units/inference (seal quiet window of 08:15 respected, none planned).

## Next steps (open)
- Read `loop-blocked-in-*` in S/origins/g1-m05/attest.json, then fix that activity for slow Windows disks (candidates by reading: sync lstat in hot/dir tiers, sync fsync in `writeJson`).
- When m05's sidecar restarts, read ready-log-tools.txt for the real stage lines.
- Workspace stage (norm-din4108, norm-din4108-rs, repo-mcp) not reached: tools has not finished since 06:31.
- Provisioning cannot add a Defender exclusion for C:\semio without elevation (readiness says elevated=False); likely a large share of the 25 files/s, a USER action.

## Not verified
Nothing on Windows ran under my control; all conclusions are from mirrored store files and code reading.
