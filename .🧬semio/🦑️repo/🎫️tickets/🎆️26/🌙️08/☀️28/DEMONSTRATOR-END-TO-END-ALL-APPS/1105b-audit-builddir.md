# 1105b Audit: Build Directory Deletion and Disk Consumption

Lane: `aud-builddir` (read-only, foreground, no kills, no edits except this report). Date of audit: 2026-11-06, about 02:25 local.

## Summary

- The build directory of the failed proc-macro2 compile was a cargo lane, `.cache/b/r/m/u/l/p48d8a0`. It is gone now. The most likely deleter is the unit-cache pressure collector (`planGcV1` / `applyGcYieldingV1` / `removeLaneV1`), which ran at 02:10:05 (`u/gc.stamp`). That run also truncated `u/gc.log` to 0 bytes, so the decision log is lost. This is an inference from code plus timing, not a confirmed trace.
- Disk: the host volume lost about 98 GiB. The main cause is `.cache/b/r/fixed.dmg`, a 96 GiB image that is fully allocated on the host, while only 59.9 GiB is used inside it. Host space is not returned when the collector deletes files inside the image.

## (1) What deletes cargo build directories

Code location: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/`

| Deleter | Code | What it removes | Safety check |
|---|---|---|---|
| Pressure GC (`collectGarbageSingleFlightV1`, started by `watchUnitCachePressureV1`) | `🧹️gc/🟦️.ts` | Whole lane base (`rmSync(paths.base)`), incremental dirs, CAS objects | `removeLaneV1` retakes claim `lane-<id>` and re-checks `laneLiveV1` |
| Pool trim (on every lane acquire) | `🧵️lane/🏊️pool/🟦️.ts` `trimPoolV1` | Least-recently-used unclaimed lanes above `maximumLanes` 16 or lane quota | Same re-check in `removeLaneV1` |
| Stale unit collector | `🧹️gc/🧱️units/🟦️.ts` | `build/<crate>/<hash>` units inside lanes | Profile lease probe, per-unit lock probe, minimum age (1 day superseded, busy minimum for busy profiles) |

Mechanisms that let a live lane look idle:

1. `planGcV1` under pressure (need > 0) removes every idle lane (not live), oldest first, not only lanes older than `laneIdleMilliseconds` (24 h). The reserved default lane `d` is not excluded in `listLanesV1`. The pool excludes it, the GC does not.
2. `laneLiveV1` (`🧵️lane/🟦️.ts` line 55) deletes any live marker older than `liveMarkerStaleMilliseconds` (30 s) and reports the lane as not live, even when the holder pid is alive. Markers are refreshed by a `setInterval` heartbeat in the holder process. An event loop stalled for more than 30 s under fleet load makes the holder look dead.
3. Claims (`🔒️lock/🟦️.ts`) are taken over when older than `staleMilliseconds` (`stale()` line 28), not only when the holder pid is dead. Same heartbeat weakness.
4. The GC collector's pressure policy is aggressive: `highWaterShare` 0.875, `lowWaterShare` 0.75, `collectorIntervalMilliseconds` 300000. `u/fill.jsonl` (955 samples) shows the image swinging between about 7.7 GiB and 71.7 GiB free within hours, so collections run constantly and lanes of running builds are the eviction candidates.

Evidence from logs and disk:

- `W-aussuchen-smoke-13.txt` line 1672: `failed to write .../u/l/p48d8a0/prototype/build/proc-macro2/ace9eb4e95cdd3dc/fingerprint/invoked.timestamp: No such file or directory`. Exit code 1, finished 02:14:53.
- `u/l/p48d8a0` no longer exists. `u/l` now has 7 lanes (`p12a2a4 p66b2c0 p77771d p7b9915 pbe66bd pc3e1e8 pd6acad`).
- `u/gc.stamp` = 02:10:05. `u/gc.log` is 0 bytes, modified 02:10. `pruneEvents` truncates event logs above 4 MiB, so the GC log was wiped by design.
- 10 generated logs contain `[repo cargo] lane pool trimmed ... removed 1 idle lane(s)`. This confirms pool trims are real in this run, but these logs do not show the victim lane.
- No process named gc, prune or clean is running. Relevant live processes are repo wrappers, `semio-build-scheduler`, and cargo/rustc for other lanes.

Not checked: whether the victim lane had a live marker or claim at 02:10. The lane is gone, so this cannot be verified from disk now. The unit collector is a lower-likelihood path; it needs its `minimumAgeMilliseconds` value checked against a fresh unit.

Ticket tools were checked too. `1105b-tools/slot.sh` runs `rm -rf` on dead slot dirs only. `actinfra/stage3.ts` and `actinfra/stages.ts` remove their own output dirs. None touch `.cache/b`.

## (2) What consumed the space

Host volume `/` (`/dev/disk3s1s1`): 926 GiB total, 109 GiB free now.

Cache images (APFS volumes mounted from sparse-named `fixed.dmg` files under `.cache/b/<x>/`). Host allocation per image (`du -sk`) and the volume's own usage (`df`):

| Image | Host allocated | Volume size | Volume used | Born |
|---|---|---|---|---|
| `b/r/fixed.dmg` | 96.0 GiB | 96 GiB | 59.9 GiB | 11-05 10:24 |
| `b/p/fixed.dmg` | 24.0 GiB | 23.8 GiB | 15.0 GiB | 10-18 11:42 |
| `b/d/fixed.dmg` | 8.0 GiB | 7.8 GiB | 4.6 GiB | 10-18 02:15 |
| `b/t/fixed.dmg` | 8.0 GiB | 7.8 GiB | 5.0 GiB | 10-18 02:58 |
| `b/g/fixed.dmg` | 6.0 GiB | 5.8 GiB | 3.3 GiB | 10-22 01:52 |
| `b/o`, `b/n`, `b/l` | about 4 GiB | | | |

Total host allocation of cache images: about 146 GiB.

Inside `b/r/m` (the mounted r image, 50.5 GB by `du`):

- `u/l` (lanes): 40.7 GB (38.8 GiB). Largest lanes from `u/fill.jsonl`: `u/l/p66b2c0/prototype` 13.4 GiB, `u/l/pd6acad/prototype` 5.8 GiB, `u/l/p12a2a4/prototype` 3.8 GiB, `u/l/p7b9915/prototype` 3.2 GiB.
- `u` total: 50.4 GB, so content-addressed units and other data are about 9.7 GB.
- `t`: 1 MB (reserved target dir).

Growth mechanism: the r image is a fixed-size APFS-in-DMG file. Its host allocation is 96 GiB, but internal use is 59.9 GiB. Deletes inside the image (GC, lane removal) do not shrink the host file. So host free space tracks the image's high-water mark of written blocks, not current use.

Not measured: `du -sk .cache/b` reported 275 GiB, but it includes mounted volumes and double counts. `.🧬semio/🦑️repo/⚡️cache` is about 3.8 GiB.

## Recommendation

1. Never delete a lane that has a live marker or a claim without first checking pid liveness. Remove the age-only takeover in `laneLiveV1` (line 55) and `stale()` (line 28). Treat a lane as live while its holder pid is alive, and make the heartbeat independent of the main event loop.
2. Exclude reserved lane `d` (and any lane acquired in the last `laneIdleMilliseconds`) from the pressure path of `planGcV1`. The pressure path should only remove lanes that have been idle for minutes, not all idle lanes.
3. Never truncate `gc.log`. Write each removal (lane id, reason, live state) to the log before it happens, so the next audit can confirm the victim.
4. Host space: the r image should be sized to its real demand, or moved to a sparse or compactable format, so host free space stops tracking the high-water mark. Compaction (`hdiutil compact`) only works on sparse UDIF images, and this one is named fixed. Verify the format first. Not run in this audit.
5. Until 1 and 2 are fixed, pause new cargo lanes while a `gc` collection runs, or raise the pressure floor (`highWaterShare`) so collections are rarer.

Scratch files for this audit (du output) are in `/private/tmp/claude-501/-Users-ueli-Documents-semio/52893ade-82ad-4a46-aa3d-0c3ed23edafb/scratchpad/aud-builddir/`, outside the ticket folder. No repo files were changed.
