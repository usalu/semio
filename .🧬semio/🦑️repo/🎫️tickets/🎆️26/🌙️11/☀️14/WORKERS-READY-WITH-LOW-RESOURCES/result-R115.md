# R115: what a hanging owner waits on (2026-11-17)

## Evidence (server, 05:32 to 05:40 UTC, read pid 99197 `show projects`, front door 82785)
- No lease or file lock is involved. The activity rows of the read are `build:graph-phase:inference ... mode shared elapsedMs 0`; the log shows no `native-artifact-input-read`, `lease-sweep` or `dependencies:` wait, and no nested `repo` command (children of 99197 are only `--semio-role=graph-farm-worker` processes).
- The nine farm workers under 99197 were idle for 3 to 4 minutes (`sample`: main thread in `kevent64`, no JS, 9 to 25 s CPU in 4 minutes) while 8 to 9 owners (`shooting-shooting-rs`, `fem-2d-rs`, `fem-3d-rs`, `bim-model-rs`; language:rust) were "running 100 to 205 s, 0 rows". So the owners are not in a farm child: they are queued behind, or running on, the ONE thread of the parent.
- The parent: 8.5 to 9.1 GB resident growing about 30 MB/s, about 25 % of one core at load 40, state `U`/`R` (paging: 803 M pageins, swap 2.4 of 4 GB, 5813 free pages), heap census 4.2 to 4.6 GB managed plus 3.9 to 4.3 GB external, 2.6 M objects, three forced full collections in this read.
- The capacity governor kept the read itself 936 s at the front (`waiting 936s for show: need 14336 MB, -1098 MB spare above the 4096 MB floor`) and then kept the farm children (1536 MB each) out (`waiting for exec ... behind projects (14336 MB)`, `admitted exec with 1 of 3 job(s): the host cannot carry more right now`), so on an overcommitted host the farm is empty and every owner runs inline in the parent's governor (`waiting for memory headroom (4011 of 4520 MB reserved, 6 projects running)`, later `9787 of 8734`).
- Workers (g1-m08 04:3x, `ready-log-workspace-failed.txt`): `waiting: the memory governor (waiting: 3738 of 3...` with two owners 2446 s and 2358 s at 0 rows: the same inline path; the CPU inside it was the stale-record settle (R111) and the 89 953 file hashes of the native authority (R112), both landed after that run.
- Pool children (nice 10, `owner-pool-children`): three orphans 51 to 60 minutes old have 18 to 21 CPU minutes each (about 35 % of a core, they compute, they do not wait); at nice 10 and load 40 they get a fraction of that (R113).
- The progress text of a parked owner named four different waits in one sentence ("a farm child, queued for one, the memory governor, or this process"), so nobody could tell which.

## What changed (landed 06:01 to 06:03 UTC, fix 06:04, see landings.md)
1. Parked-owner line names the actual wait, EN and DE: `stagedPrepareV1` marks the farm wait, then the memory-governor wait after the farm child is done, then the in-process stage. New test `project-nodes/🧪️tests/🧭️stage-names` (red before: missing export).
2. Walk-farm children (`closure-kinds/walk-farm/👷️child`) start the same parent-gone watchdog thread as project-farm workers (R108). New test `walk-farm/🧪️tests/🪦️busy-orphan` with a real child whose parent is SIGKILLed while its loop is held (red before: child outlived its read 17 s, green after: ends in about 1 s).
3. Not changed: pool child priority (R113 owns the pool child lifecycle; `childPriority` 10 in the owner-pool schema is the candidate), the memory governor law.

## Other long-lived spawn sites in the build library (gap list for the watchdog)
- Has a parent-gone mechanism: project-farm worker, owner-pool child (R108), resident slice-pool (reaper by token), walk-farm child (this ticket).
- Gap, child waits on stdin/stdout only, killed only by the parent's own timers: `graph/📥️inference/🧭️script-inputs/🪆️heavy-questions` script child (parent kills on exit, timeout 15 min and silence 60 s; a SIGKILLed parent leaves it until it ends on its own), `🧭️invocation/🕸️graph/🧠️memo/🧮️derive/♨️warm` (spawns warm helper children), `🏡️resident/🏊️delegate` warm pool and cold delegate children, `🌐️remote-execution/🛰️sidecar/🕸️graph/🔭️read` (read child), `🌐️remote-execution/🛰️sidecar/🚦️ready/🏃️wrapper`. Each needs the same watchdog or a documented reason (not done here).

## Unverified
- The farm-empty-because-of-capacity cause is read from the log and `sample` (idle workers), not from a JS stack of the parent (bun gives none); the CPU split inside the parent was not measured after the R107/R111/R112 landings.
- Whether pool children at nice 10 should run at 0 (not changed).
- The landing script's compare step keyed all rows as `.` and printed GREEN with one failing assertion; fixed by hand at 06:04.
