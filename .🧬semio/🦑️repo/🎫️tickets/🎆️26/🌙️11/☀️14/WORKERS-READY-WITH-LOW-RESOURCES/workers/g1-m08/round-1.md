# Round 1 — g1-m08 — Not Ready

The single prescribed foreground bootstrap ended with **native_rc=1** and no completion marker. The monitor observed **system memory pressure level 2** at `2026-11-14T12:16:12Z` and sent **one SIGINT** to this attempt’s own process group. The required one-minute wait was completed before this report. No bootstrap retry was made.

## Machine And Replica Code

| Measurement | Value |
| --- | --- |
| Origin | g1-m08 |
| Computer name | Ueli’s Mac mini |
| Hostname | ueli-workstation-mac-mini-m4-16gb-256gb-05-wlan.fritz.box |
| Hardware model | Mac16,10 |
| CPU | Apple M4 |
| Architecture | arm64 |
| OS | macOS 27.2, build 26B123 |
| Physical memory | 17179869184 bytes (16 GiB) |
| Free home disk before round | 122024956 KiB (116.372 GiB), measured 2026-11-14T11:00:34Z |
| Free home disk after round | 111945556 KiB (106.760 GiB), measured 2026-11-14T12:23:07Z |
| Code digest from status | `1c863927612f`, last attested prefix, explicitly stale |
| Currently running daemon digest | Not measured: status reports daemon not running |
| Native start | 2026-11-14T11:03:41Z (2026-11-14T12:03:41+01:00) |
| Native end | 2026-11-14T12:16:12Z (2026-11-14T13:16:12+01:00) |
| Native wall time | 4351 seconds |
| Logged native_rc | 1 |
| Outer foreground wrapper exit | 130 |
| Completion marker exists | false |
| Ready | false |
| Peak sampled process RSS | 6477936 KiB = 6633406464 bytes = 6.177841 GiB |
| RSS samples | 144; 2026-11-14T11:04:54Z through 2026-11-14T12:17:07Z |
| Observed sample intervals | 30–31 seconds |
| Hard stop | System pressure warning, level 2; native exit and signal are in the same UTC second; their sub-second ordering is not measured |
| Post-signal observation complete | 2026-11-14T12:17:13Z |

The `native_rc` above is the value actually emitted by the supplied wrapper, not a replacement of its result with the outer shell’s exit 130. System pressure returned to level 1 after interruption. The RSS limit was not the observed trigger: the largest 30-second sample was below 8,000,000,000 bytes. Unsampled transient RSS peaks are not measured. The pressure interpretation and its primary-source check are in [memory-pressure-check.md](memory-pressure-check.md).

## Failed / Interrupted Task

- `workspace:setup`: interrupted under the round’s system-pressure guard. The last observed nested phase was `cache:entry` / “Verifying build tools”. No other failed-task summary was emitted before interruption.
- First native error line: **not emitted / not measured**. The log contains no native error line for which 15 predecessor lines can be identified. The external monitor’s failing condition, verbatim, was:

```
system memory pressure level 2
```

The following are the **15 native-log lines immediately preceding the logged end**, supplied as interruption context rather than represented as a nonexistent error-line excerpt:

```
[repo] still working after 3440 s; last phase started: source.served
[script] {"owner":"repository:build","command":"cache:entry","stage":"running","activity":{"resource":"nx:source-verification","mode":"shared","elapsedMs":0}}
[script] cache:entry · Verifying build tools
[script] {"owner":"repository:build","command":"cache:entry","stage":"running","activity":{"resource":"nx:source-verification","mode":"shared","elapsedMs":0}}
[script] cache:entry · Verifying build tools
[script] {"owner":"repository:build","command":"cache:entry","stage":"running","activity":{"resource":"nx:source-verification","mode":"shared","elapsedMs":0}}
[script] cache:entry · Verifying build tools
[script] {"owner":"repository:build","command":"cache:entry","stage":"running","activity":{"resource":"nx:source-verification","mode":"shared","elapsedMs":0}}
[script] cache:entry · Verifying build tools
[script] {"owner":"repository:build","command":"cache:entry","stage":"running","activity":{"resource":"nx:source-verification","mode":"shared","elapsedMs":0}}
[script] cache:entry · Verifying build tools
[script] {"owner":"repository:build","command":"cache:entry","stage":"running","activity":{"resource":"nx:source-verification","mode":"shared","elapsedMs":0}}
[script] cache:entry · Verifying build tools
[script] {"owner":"repository:build","command":"cache:entry","stage":"running","activity":{"resource":"nx:source-verification","mode":"shared","elapsedMs":0}}
[script] cache:entry · Verifying build tools
```

The exact last 200 log lines are in [round-1.tail.txt](round-1.tail.txt). Full machine-local log: `/Users/ueli/native-bootstrap-round-1.log`.

## Capacity And Graph Lines — Verbatim

The following exactly matches every raw-log line containing `[capacity]` or `graph:`. No `graph: refreshing N projects` or `Inferring projects: … of M projects` message above 300 was observed; a numeric project count was not measured.

```
[capacity] run: sized at 46% of the default from 1 earlier run(s) of its class
[capacity] admitted run with 7 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 52% of the default from 1 earlier run(s) of its class
[capacity] run: sized at 52% of the default from 1 earlier run(s) of its class
[capacity] run: sized at 52% of the default from 1 earlier run(s) of its class
[capacity] admitted dependencies: 1 job(s), 6144 MB reserved (9904 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 1 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (9724 MB available, 2048 MB floor)
[capacity] run: work has to run, waiting 0s to grow the reservation from 2048 MB to 8190 MB: not enough free memory (-1117 MB spare above the 2048 MB floor; 4 running, 0 ahead); behind graph-plan (running, 1 job(s), 3123 MB), graph-farm (running, 1 job(s), 1536 MB), graph-farm (running, 1 job(s), 0 MB)
[capacity] run: work has to run, waiting 6s to grow the reservation from 2048 MB to 8190 MB: not enough free memory (-1004 MB spare above the 2048 MB floor; 4 running, 0 ahead); behind graph-plan (running, 1 job(s), 3123 MB), graph-farm (running, 1 job(s), 1536 MB), graph-farm (running, 1 job(s), 0 MB)
[capacity] run: work has to run, waiting 11s to grow the reservation from 2048 MB to 8190 MB: not enough free memory (-1070 MB spare above the 2048 MB floor; 4 running, 0 ahead); behind graph-plan (running, 1 job(s), 5523 MB), graph-farm (running, 1 job(s), 1536 MB), graph-farm (running, 1 job(s), 0 MB)
[capacity] run: work has to run, waiting 16s to grow the reservation from 2048 MB to 8190 MB: not enough free memory (-1135 MB spare above the 2048 MB floor; 4 running, 0 ahead); behind graph-plan (running, 1 job(s), 5523 MB), graph-farm (running, 1 job(s), 1536 MB), graph-farm (running, 1 job(s), 0 MB)
[capacity] run: work has to run, waiting 21s to grow the reservation from 2048 MB to 8190 MB: not enough free memory (-635 MB spare above the 2048 MB floor; 4 running, 0 ahead); behind graph-plan (running, 1 job(s), 5523 MB), graph-farm (running, 1 job(s), 1536 MB), graph-farm (running, 1 job(s), 0 MB)
[capacity] run: work has to run, waiting 26s to grow the reservation from 2048 MB to 8190 MB: not enough free memory (3009 MB spare above the 2048 MB floor; 4 running, 0 ahead); behind graph-plan (running, 1 job(s), 2975 MB), graph-farm (running, 1 job(s), 0 MB), graph-farm (running, 1 job(s), 0 MB)
[capacity] run: reservation grown from 2048 MB to 3510 MB with 2 of 8 job(s) after 31s: the host cannot carry more right now (4357 MB available, 2048 MB floor); the run grows to more jobs once memory and cores free up
[capacity] run: grown from 2 to 3 of 8 job(s) after 795s, reservation 3510 MB -> 3510 MB: memory freed up and nothing of ours waits (2472 MB available, 2048 MB floor); work that starts from now on uses the new jobs
[capacity] run: grown from 3 to 4 of 8 job(s) after 3270s, reservation 3510 MB -> 4446 MB: memory freed up and nothing of ours waits (4449 MB available, 2048 MB floor); work that starts from now on uses the new jobs
[capacity] run: sized at 69% of the default from 1 earlier run(s) of its class
```

## Wall Time Per Phase

Phase transitions below are observed at log polling time (about one-second resolution). Nested intervals are included in `workspace:setup` and must not be added to it. Early messages were present before the first monitor sample; their individual timings are not measured.

| Phase | Observed start (UTC) | End / next observation (UTC) | Wall time / limitation |
| --- | --- | --- | --- |
| Initial launcher / Rust / native-command setup | 2026-11-14T11:03:41Z | 2026-11-14T11:04:54Z | Individual phase wall times not measured; these messages were already in the log when monitoring began. |
| Dependency installation (observed portion) | 2026-11-14T11:04:54Z | 2026-11-14T11:18:50Z | 836 s observed; exact start not measured. |
| workspace:setup | 2026-11-14T11:18:50Z | 2026-11-14T12:16:12Z | 3442 s from observed phase start to logged end. |
| Graph-slice read (within workspace:setup) | not measured | not measured | At least 3262 s reported by the native heartbeat; exact final interval not measured. |
| build:source-admission (nested observation) | 2026-11-14T12:13:43Z | 2026-11-14T12:13:45Z | 2 s observed. |
| build:implementation-source (nested observation) | 2026-11-14T12:13:45Z | 2026-11-14T12:14:16Z | 31 s observed. |
| repository:command (nested observation) | 2026-11-14T12:14:16Z | 2026-11-14T12:15:26Z | 70 s observed. |
| build:implementation-source (nested observation) | 2026-11-14T12:15:26Z | 2026-11-14T12:15:46Z | 20 s observed. |
| cache:entry (nested observation) | 2026-11-14T12:15:46Z | 2026-11-14T12:16:12Z | 26 s observed. |
| Required post-SIGINT wait | 2026-11-14T12:16:12Z | 2026-11-14T12:17:13Z | 61 s. |

## Sampling Method And Limits

Round 1 used the existing toolchains and caches left by earlier join work. A pre-protocol native setup attempt was cancelled with exit 130 before this round began; its caches were preserved. The two previously observed native-process PIDs were absent before round 1 started. No cache reset was performed.

The requested `ps -axo rss,command | sort -nr | head -3` query was sampled approximately every 30 seconds. The reported peak is the largest process RSS **seen in those machine-wide samples**, not a continuous native-process high-water mark. Sampling began 73 seconds after native start; earlier RSS peaks and exact early-phase timings are **not measured**. Credential-like command-line fields were redacted before storage if present.

Initial observations were written within this worker folder. At `2026-11-14T11:57:41Z`, only the monitoring helper was replaced with a buffered helper; the native attempt continued without restart or interruption. Later observations were held in memory and written after the attempt and grace interval, reducing monitoring writes during the graph operation. Both segments are preserved in [round-1-ps.jsonl](round-1-ps.jsonl) and [round-1-phases.jsonl](round-1-phases.jsonl). Hard-stop checks remained active, and only one SIGINT was sent to the native attempt. [round-1-monitor-result.json](round-1-monitor-result.json) records the measured result.

## Replica Status And Report Delivery

```
Replica g1-m08 of group g1, store /Volumes/semio-store: reachable
Synchronization: on
Daemon: not running
278647 files tracked; queued: 10 incoming, ? outgoing; 26 overwritten files kept in .cache/f/lost
Journal: 5.47 MiB in 11 segments of 9 origins.
Newest snapshot by g1-server: 1862s old, 89.18 MiB.
Reclaimable now: 0 journal segments (0 B).
Last compaction 11325s ago: 861 blobs (27.00 GiB) removed; 88 blobs (129.23 MiB) were still too young.
Git state: shared (stage and commits replicate); stage tree 59f8d6e4d9b1935a6afc28cfa10608ad9c946325.
Background tiers, last pass (cost, duty cycle of one core): hot 22 ms 6.7%, directory sweep 910 ms 10%, file sweep 3083 ms 2.9%, tree walk 27521 ms 1.9%, git view 30640 ms 5%.
  g1-m08 macos 5176s 8/0 cp:5176s on 1c863927612f 33/12/36 -/-|129949/574785 stale (attestation too old)
```

Full observations: [before](round-1-status-before.txt), [during](round-1-status-during.txt), and [after](round-1-status-after.txt). The daemon log recorded a stop after code changed; the LaunchAgent subsequently had a new running PID, while status still reported no running daemon and stale attestation. These are observations, not a service repair or a claim that the new process had completed startup.

A read-only, SHA-256-verified inspection of the existing fleet-store snapshot plus retained numeric journal segments found the same protocol and `round: 1`, no machine-specific `next.md`, and no published copy of this worker’s interim progress file. Therefore delivery of these local reports to the coordinator is not confirmed. No store or worktree file was modified by that inspection.

No sudo, mutating Git command, pkill/killall, build-code edit, daemon restart, or source fix was performed for this round. Direct report-file writes are confined to this worker folder; the explicitly prescribed bootstrap performed its own native setup/cache/log operations.

## Next Action

`ready` is false. As required by the protocol, wait for a higher round, checking every 60 seconds for at most six hours from this report. Do not run round 1 again. Preflight is not run because protocol step 5 applies when ready.
