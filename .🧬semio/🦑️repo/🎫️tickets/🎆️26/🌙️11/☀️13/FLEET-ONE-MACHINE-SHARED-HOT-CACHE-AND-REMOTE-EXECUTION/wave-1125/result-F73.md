# F73: slice worker killed (exit code null) on g1-m08 / g1-m10

## 1. Who killed it: the host warden (SIGKILL cancel under memory pressure)
- m08: failure `0354e495` at 04:17:08Z. Metrics ring (`origins/g1-m08/metrics`, TS reader): at 04:17:00 `warden.cancelled` 0 -> 1, `warden.subjects.memory` 3.29 GB -> 89 MB, warden level 1..2 (danger) from 04:16:10, host available 2.9 GB of 16 GB, swap 5.2 GB, swap-out up to 376 MB/s. Failure surfaced 8 s later (worker heartbeat/exit lag).
- m10: failure `57bba7da` at 04:33:48.9Z. Ring: `warden.cancelled` 0 -> 1 at 04:33:48, level 2, available 2.9 GB, subjects 5.0 GB -> 1.96 GB (a ~3 GB subject freed).
- Not an OOM, not F59/F43, not a supervisor restart (supervisor adoption at 04:14 is earlier; no further restart at the kill time). Kind `slice-worker` is `capCritical`, cancel signal SIGKILL (warden schema `Kinds`), so exit = `null` + signal SIGKILL. Counters in `memory.json`: cancelled 1 (m08), 2 (m10).
- Slice worker tree size at the kill was ~3 GB, below its 6 GB cap: it was a dear victim only because the host was in danger (swap-heavy 16 GB host), with the graph pool children at ~1 GB each.

## 2. Fix (landed, see T/landings.md "Wave 1125 F73")
The pool treated only exit code 75 as restartable; a signal exit was a hard stage failure. Now (`👷️slice-pool/🧵️process/🟨️.mjs`, `🟨️.mjs`): exit code null with a signal => `slice-killed`, `restart:true`, cause "killed by SIGKILL from outside (resource-transient)". It joins the memory cut ladder (the same path as `memory-pressure`): the next worker starts one cut level smaller (half the concurrent projects, so less memory and a smaller target for the warden); at the last level it restarts with the existing backoff, bounded by `sliceRestartPolicy` (total maximum, settle time); the same-cause failure counter does not apply. Progress shows `cut` / `restarted-cause` decisions in both languages as before.
- Not changed: warden classification. The slice worker remains cancelable as the last resort (a hung host is worse), but its loss is now cheap (persisted records, resumed at a smaller cut). A dedicated protection during readiness would have to be a warden mode that outranks the pool children; not done, the retry makes it unnecessary.
- Peak heap: not re-measured; m08 `peakMegabytes` 10208 for the whole stage; the F19/F55 paging applies to the same worker code (no separate slice-worker change needed).

## Tests
`repo test-file` `🏡️resident/🧪️tests/✂️cut-stops` 7/0 (2 new: killed worker cut and retried; killed at the last level restarted; code 1 stays a failure), restart 8/0, restart-cause 14/0, cut-load 9/0, slice-share 5/0. dangling-imports 0; `repo typecheck`: no error in touched files (36 errors in untouched peer test files).

## 3. Fleet watch
Snapshots (Ready of 14 live origins; the 15th, g1-m15, is stale):
- 05:45 2 (m03, m11). 05:54 3 (+m14). 06:01 4 (+m01). 06:09 5 (+m02). Not yet 10; slice stage of m07/m08/m10/m12/m13 running, m06 in back-off.
- m04 dependencies `disk-short` (22 attempts), m05 Windows dependencies attempt 51 (`Private Nx engine import missing ../../../🛠️tools/🐹️go/🟦️.ts` seen earlier, source-transient), m15 stale for 40 min.
- m08 failed once more at 05:42 with exit null: its resident service still ran the pre-landing pool code (no "killed by" text); it is running again.
- New failure classes seen: (a) churn of the resident service during peers' edits: "Complete retained closure publication required" (m06 05:54), "Closure reference proof leaf changed" (m12 05:54): not retried in the stage; now classified `source-transient` (second landing, test with the exact texts 4/0), so the next attempt starts a debounce after the tree changed instead of the long back-off. The classification applies on machines after their supervisors pick up the code. (b) stale failure files `BUILD_SOURCE_UNPARSEABLE` (m03, m11) and `route-targets` (m02/m07/m13/m14) are the earlier incidents, not new.
- Not verified: the killed-worker retry on a real warden cancel (unit-proven only); Ready >= 10 not reached within 40 min.
