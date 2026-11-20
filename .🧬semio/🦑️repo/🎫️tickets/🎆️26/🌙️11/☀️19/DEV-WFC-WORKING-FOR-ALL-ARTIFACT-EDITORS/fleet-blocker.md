# Fleet blocker diagnosis (read-only)

Taken 2026-11-19, 00:25 to 00:30 local. Nothing was changed and no process was stopped.

## Verdict

The graph load is advancing, but slowly and mostly waiting rather than computing. It is not dead. The "stall" for `repo run` and `repo show targets` is the shared machine admission queue plus one idle run-many grant, not memory exhaustion.

## Who leads the graph load

| Role | PID | Notes |
|---|---|---|
| Graph builder (the leader) | 19040 | bun, child of 18968. Holds exclusive leases `repository-graph-facts:...:shard:24` and `build-graph-state:...:m:selected:24`. Owner `graph`, priority `agent`. Not this session. |
| Graph planner | 18968 | `bun --eval ...`, child of 18933. Holds a 2048 MB grant (reserved), uses 1131 MB. |
| Service host | 18933 | `repo service host run`, orphaned (ppid 1). |
| Fleet supervisor | 2528, 2535 | Started from `~/Library/Application Support/semio/fleet/entry/boot.mjs`. This is the ancestor of the whole tree. |
| Idle run-many | 18913 | `run-many --projects=@semio-tech/norm-din4108,...`. Holds a 512 MB grant (56 MB used) for about 32 min at 0.1% CPU. |
| This session's `repo run` | 31857 | Your `activate-bitmap-react-prototype`. Already admitted, now waiting on the graph. |

## Progress samples (inference counter, "of 591")

| Time | Inference | Source |
|---|---|---|
| about 00:25 | 4 | your report |
| 00:27 | 6 | `repo leases status` |
| 00:29:01 | 7 | `repo leases status` |
| 00:29:46 | 13 | `repo leases status` |

- Rate is lumpy: roughly 2 to 8 units per minute. At about 5 to 8 per minute, the remaining 578 units need about 70 to 115 minutes.
- CPU on 19040 over 45 s was 2.2 s, about 5% of one core. Its state is `Ss`, RSS steady at about 1.1 to 1.2 GB.
- One `ps` sample showed state `Us` (uninterruptible), which points to disk I/O or image attach waits. Two images (`nx`, `products`) are unmounted, and `native-fixed-image` leases are held.
- Conclusion: progressing, but I/O- or wait-bound. Not stuck, but it will take a long time.

## Memory pressure

Not the constraint.

- Host: 16 GB total, 5.9 to 6.5 GB free, pressure "normal", swap 98 MB with 0 MB/s swap-out.
- Admission is the constraint. The 2048 MB floor plus reservations that are not yet resident leave about 1.7 GB spare.
- The cargo queue needs 3354 MB per request. Four cargo requests (33110, 33246, 33469, 33555) are waiting, behind the 5271 MB cargo grant of 32943 and the stale reservations of 18968 and 18913.

## What `repo doctor` suggests

- Ask the owner of run-many 18913 whether it still works. Stopping it returns 512 MB. Do not stop it without the owner's say.
- Ask the owner of the graph builder 19040 whether it is still progressing. Long exclusive holds are the main queue cause.
- Safe cleanup: `repo doctor --fix --only=leases` (preview with `--dry-run` first). It removes registry rows of dead processes and folders only.
- Other notes:
  - 6 lock-order violations, which can deadlock under load.
  - Graph code changed after generation 248, so the next fresh read rebuilds everything it touches. This explains the long load.
  - `repo cache gc --dry-run` lists one undeclared cache entry.

## Single-project bypass

No, not with the current state.

- `repo show project @semio-tech/framework-os-dev` went through `door.capacity`. It queued behind the same 3354 MB cargo requests and was killed by SIGTERM at the 90 s timeout.
- `--graph-policy=current` would serve the newest sealed graph without waiting, but `repo fleet status` reports "Generation: none published". There is nothing sealed to serve, so this probably fails. Not tested.

## Command caveats

- `repo fleet status`, `repo leases status`, `repo doctor`, and `repo capacity status --json` worked.
- Some commands returned "unknown command" when run in a loop (`capacity status`, `scheduler status`, `service host status`, `session list`). The same subcommand worked when run directly or with `--json`. The CLI behaviour looks inconsistent. `repo session status` worked and shows this session's state as live.
- `repo show targets` was not re-run, to avoid another capacity queue.

## Process details (for the owner)

- This session (`a1rg9xl3don`) holds cargo pid 33165 (about 760 MB RSS, 83% CPU). It is working, not waiting.
- The four waiting cargo commands are `timeout ... repo cargo ... test` runs from this session's shell, which is why this session's own tests are queued.
- `repo doctor` marked 18913 (run-many) as the root-cause holder of memory for the cargo queue.

## Suggested next steps (none taken)

1. Ask the owner of run-many 18913 and of the graph builder 19040 whether they are still active.
2. Re-check `repo leases status` every few minutes. Expect the inference counter to keep climbing.
3. Run `repo doctor --fix --only=leases --dry-run`, then the real fix if the output looks right.

## 2026-11-19 14:20 — cargo offload to the fleet hangs; native fallback

`withBuildCapacityV1` (🏗️build/🌡️host-capacity/🟦️.ts) calls `offloadFanOutJobsV1`. When local unit admission says "wait" and a fleet execution endpoint is published, cargo stops queueing locally and its units are sent to the fleet ("[capacity] cargo does not queue for this machine … flow to the fleet").

Effects observed:
- bitmap-test19, wfc2d build 23 and job-headless-t1 sat 20–40 min with no compile output.
- wfc3d test23 died with "disk I/O error".

Fallback: export `SEMIO_OFFLOAD=never` (switch `offloadModeV1`, schema `🚀️offload/🧬️schema/🔣️.json`). All ticket agents now run cargo with it, and local runs queue on host memory as before. Follow-up for the fleet team: an offloaded build with no fleet progress should fall back to local admission instead of waiting without limit.
