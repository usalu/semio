# Round 4 — g1-m08 — Complete

**Not ready.** Attempts 1 and 2 ended in the expected fleet refusal (native_rc=75). The coordinator superseded this round with round 5 while attempt 3 was running: one SIGINT was sent at 17:55:31 UTC, then the required minute was waited. Attempt 3 logged native_rc=7. No memory hard stop or repeated EPERM was observed.

## Pre-run observations

Supported graph host stop exited 0: `No repo graph service is running on this host.` Full command output: `round-4-service-stop.json`. Supported replica status showed daemon running PID 75485, synchronization on and a fresh code attestation prefix 39e3688eb315; manifest divergence was reported, without repair. Full output: `round-4-status-before.txt`.

```json
{
  "at": "2026-11-14T17:36:58.993445+00:00",
  "digest": "39e3688eb315d48831772ca305d78bcb6f35975794444da47750bc5fa7459759",
  "replica_pid": 75485,
  "matching_processes": [],
  "sidecar_candidate_pids": [],
  "sidecar_parented_show_requires_wait": false,
  "build_agent_last8": "2026-11-14T16:59:42.551Z worker registered: 43a99cad-5ce5-4e51-aaf3-7229d6a134f0 cache http://192.168.178.70:6094\n2026-11-14T17:23:41.256Z agent code changed, restarting\n2026-11-14T17:23:41.260Z worker stopped: ueli-workstation-mac-mini-m4-16gb-256gb-05-wlan.fritz.box-8813\n2026-11-14T17:23:42.218Z joining http://192.168.178.70:8981 offering rustc for 3 target(s)\n2026-11-14T17:23:42.260Z worker sandbox: darwin-sandbox confined network denied\n2026-11-14T17:23:42.275Z worker registered: 7d8219ae-52d0-4165-932e-b8e2f4922270 cache http://192.168.178.70:6094\n2026-11-14T17:28:41.398Z graph-server: alive, 0 machines seen, 0 requests held, read none\n2026-11-14T17:33:45.238Z graph-server: alive, 0 machines seen, 0 requests held, read none"
}
```

## Machine

Memory 17179869184 bytes (16 GiB); macOS 27.2 build 26B123, Apple M4 arm64. Free disk before:

```text
Filesystem   1024-blocks      Used Available Capacity iused      ifree %iused  Mounted on
/dev/disk3s5   239362496  84237096 117388872    42%  831487 1173888720    0%   /System/Volumes/Data
```

Free disk after:

```text
Filesystem   1024-blocks      Used Available Capacity iused      ifree %iused  Mounted on
/dev/disk3s5   239362496  84400000 116800000    42%  831651 1171413760    0%   /System/Volumes/Data
```

## Attempts

| Try | Start UTC | End UTC observed | Wall s | native_rc | Retry sleep s | Wanted / arrived / missing / fleet wait s |
|---|---|---|---:|---:|---:|---|
| 1 | 2026-11-14T17:38:24Z | 2026-11-14T17:43:39Z | 315.076 | 75 | 180.005 | 703 / 0 / 703 / 0 |
| 2 | 2026-11-14T17:46:39Z | 2026-11-14T17:47:25Z | 45.645 | 75 | 180.003 | 703 / 0 / 703 / 0 |
| 3 | 2026-11-14T17:50:25Z | 2026-11-14T17:55:35Z | 309.61 | 7 | 0 | Not measured in native log; interrupted |

Attempt 3 resident-service observations were 0 of 703 arrived through interruption; total missing and exact fleet wait at cancellation were not emitted. The maximum graph-read heartbeat was 272 seconds. No fourth attempt was started after the coordinator superseded this round.

## Resource observations and phase timings

Thirty-second samples used both required top-three commands, including full commands, PID, PPID and ancestry. Pressure was checked about every five seconds, warning streaks every 30 seconds. Run ownership included descendants and graph-host metadata newly started by each attempt; reused resident hosts may be absent from per-attempt owned-RSS counters, but are retained in machine-wide top-three samples.

### Try 1

Peak run-owned RSS 3406352 KiB; peak machine-wide sampled RSS 3400848 KiB. Samples: 11. All pressure observations normal (1): True. See `round-4-try-1-ps.json` for full ancestry and `round-4-try-1-pressure.json`.

Observed phase starts (approximately five-second log-poll resolution):
```text
2026-11-14T17:38:24Z elapsed=0.001s [semio] Building the repo launcher (it provisions the pinned JavaScript runtime itself)…
2026-11-14T17:38:29Z elapsed=5.096s [semio] Installing the native repository command…
2026-11-14T17:38:39Z elapsed=15.231s [semio] Installing the dependency scopes (the planner of every later run needs them)…
2026-11-14T17:41:43Z elapsed=198.637s [semio] workspace:setup…
```

Observed wall per phase:
- [semio] Building the repo launcher (it provisions the pinned JavaScript runtime itself)…: 5.095 seconds until next observed phase/end; exact internal wall not measured.
- [semio] Installing the native repository command…: 10.135 seconds until next observed phase/end; exact internal wall not measured.
- [semio] Installing the dependency scopes (the planner of every later run needs them)…: 183.406 seconds until next observed phase/end; exact internal wall not measured.
- [semio] workspace:setup…: 116.439 seconds until next observed phase/end; exact internal wall not measured.

Failed bootstrap phase: `workspace:setup`; no downstream Nx task list was emitted. First emitted refusal and 15 preceding lines, or interruption context if no error was emitted:
```text
Reading the project graph slice (86 s): flush 1 of 1.
Reading the project graph slice (88 s): verification 1425 of 2277.
Reading the project graph slice (90 s): verification 1524 of 2277.
Reading the project graph slice (92 s): verification 1663 of 2277.
Reading the project graph slice (94 s): verification 1721 of 2277.
Reading the project graph slice (96 s): verification 1721 of 2277.
Reading the project graph slice (98 s): rederivation 140 of 140.
Reading the project graph slice (100 s): flush 1 of 1.
Reading the project graph slice (102 s): verification 1965 of 2277.
[capacity] run: work has to run, waiting 0s to grow the reservation from 2048 MB to 8190 MB: not enough free memory (1678 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind graph-plan (running, 1 job(s), 1833 MB)
Reading the project graph slice (104 s): flush 1 of 1.
[repo] still working after 110 s; last phase started: door.script
Reading the project graph slice (106 s): verification 2107 of 2277.
[capacity] run: work has to run, waiting 6s to grow the reservation from 2048 MB to 8190 MB: not enough free memory (2886 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind graph-plan (running, 1 job(s), 3952 MB)
Reading the project graph slice (108 s): verification 2236 of 2277.
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 703 and delivered 0 within 0 s, 703 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

Required capacity/graph/fleet/wanted/arrived/serving/restored/restarting/adopted lines, verbatim:
```text
[capacity] run: sized at 8% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 8 job(s), 1709 MB reserved (6795 MB available, 2048 MB floor)
[capacity] run: sized at 8% of the default from 3 earlier run(s) of its class
[capacity] run: sized at 46% of the default from 1 earlier run(s) of its class
[capacity] run: sized at 46% of the default from 1 earlier run(s) of its class
  fleet status                                    Show one picture of the shared workspace: who runs what (phase, progress, age, priority, code freshness), the queues with waiting-time estimates, memory admitted against really used, the Bazel servers, storage against its budgets, the sealed implementation and the last graph generation.
  --json                                          Print lists as JSON instead of one entry per line. (used by: graph, show, incremental, concurrency, bench, capacity, fleet, scheduler, leases, cache, remote-cache, doctor, tools, conformance, units, service, session)
  --origin <id>                                   remote-execution workers: origin id of this machine in the fleet (default: the origin of the replica, else the host name); the scheduler never gives a machine the units it requested itself. (used by: remote-execution)
  --limit=<n>                                     fleet status: how many runs the text output lists (default: 20, the JSON output always lists all). (used by: fleet)
  repo fleet status
      Run the service with the shared store and one local worker: gRPC for build tools on 127.0.0.1:8980, HTTP for workers, status and the fleet load table on 127.0.0.1:8981; hermetic Bazel actions can then run remotely.
  fleet status                                    Show one picture of the shared workspace: who runs what (phase, progress, age, priority, code freshness), the queues with waiting-time estimates, memory admitted against really used, the Bazel servers, storage against its budgets, the sealed implementation and the last graph generation.
  --json                                          Print lists as JSON instead of one entry per line. (used by: graph, show, incremental, concurrency, bench, capacity, fleet, scheduler, leases, cache, remote-cache, doctor, tools, conformance, units, service, session)
  --origin <id>                                   remote-execution workers: origin id of this machine in the fleet (default: the origin of the replica, else the host name); the scheduler never gives a machine the units it requested itself. (used by: remote-execution)
  --limit=<n>                                     fleet status: how many runs the text output lists (default: 20, the JSON output always lists all). (used by: fleet)
  repo fleet status
      Run the service with the shared store and one local worker: gRPC for build tools on 127.0.0.1:8980, HTTP for workers, status and the fleet load table on 127.0.0.1:8981; hermetic Bazel actions can then run remotely.
[capacity] dependencies: sized at 30% of the default from 3 earlier run(s) of its class
[capacity] admitted dependencies: 1 job(s), 1843 MB reserved (6758 MB available, 2048 MB floor)
[capacity] dependencies: sized at 30% of the default from 3 earlier run(s) of its class
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":68.40545799999927}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":1.9365839999991294}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":73.15375000000131}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":1.1632920000010927}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":62.22383300000001}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.6763329999994312}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":47.580541999999696}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":1.6649170000018785}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":51.01262500000303}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":1.596957999994629}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":59.88812499999767}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.8568329999980051}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":53.45737499999814}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.8496670000022277}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":131.13383299999987}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":1.2667500000097789}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":54.850875000003725}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":1.2967499999940628}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":64.88295899999503}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":1.0033339999936288}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":57.358500000002095}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.5982500000100117}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":43.6617499999993}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.6473330000007991}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":42.68895800001337}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":1.2183329999970738}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":55.02812499999709}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.8397499999991851}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":42.214374999995925}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.640916999996989}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":48.351582999996026}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.6499580000236165}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":45.74866700000712}}
[capacity] run: sized at 46% of the default from 1 earlier run(s) of its class
[capacity] waiting 0s for run: not enough free memory (need 2048 MB; 1386 MB spare above the 2048 MB floor, load 5.23; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] admitted run: 1 job(s), 2048 MB reserved (6581 MB available, 2048 MB floor)
[capacity] run: work has to run, waiting 0s to grow the reservation from 2048 MB to 8190 MB: not enough free memory (1678 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind graph-plan (running, 1 job(s), 1833 MB)
[capacity] run: work has to run, waiting 6s to grow the reservation from 2048 MB to 8190 MB: not enough free memory (2886 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind graph-plan (running, 1 job(s), 3952 MB)
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 703 and delivered 0 within 0 s, 703 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

### Try 2

Peak run-owned RSS 1042768 KiB; peak machine-wide sampled RSS 1793360 KiB. Samples: 2. All pressure observations normal (1): True. See `round-4-try-2-ps.json` for full ancestry and `round-4-try-2-pressure.json`.

Observed phase starts (approximately five-second log-poll resolution):
```text
2026-11-14T17:46:39Z elapsed=0.012s [semio] Building the repo launcher (it provisions the pinned JavaScript runtime itself)…
2026-11-14T17:46:45Z elapsed=5.118s [semio] Installing the native repository command…
2026-11-14T17:46:55Z elapsed=15.265s [semio] Installing the dependency scopes (the planner of every later run needs them)…
2026-11-14T17:47:05Z elapsed=25.359s [semio] workspace:setup…
```

Observed wall per phase:
- [semio] Building the repo launcher (it provisions the pinned JavaScript runtime itself)…: 5.106 seconds until next observed phase/end; exact internal wall not measured.
- [semio] Installing the native repository command…: 10.147 seconds until next observed phase/end; exact internal wall not measured.
- [semio] Installing the dependency scopes (the planner of every later run needs them)…: 10.094 seconds until next observed phase/end; exact internal wall not measured.
- [semio] workspace:setup…: 20.286 seconds until next observed phase/end; exact internal wall not measured.

Failed bootstrap phase: `workspace:setup`; no downstream Nx task list was emitted. First emitted refusal and 15 preceding lines, or interruption context if no error was emitted:
```text
Planning 1 targets.
Reading the project graph slice (2 s).
Reading the project graph slice (4 s): files 416 of 416.
[repo] still working after 5 s; last phase started: door.script
Reading the project graph slice (6 s): directories 19 of 19.
Reading the project graph slice (8 s): facts 16384 of 184133.
[repo] still working after 10 s; last phase started: door.script
Reading the project graph slice (10 s): facts 40960 of 184133.
Reading the project graph slice (12 s): verification 1 of 2277.
Reading the project graph slice (14 s): verification 75 of 2277.
Reading the project graph slice (16 s): flush 1 of 1.
Reading the project graph slice (18 s): flush 1 of 1.
[repo] still working after 20 s; last phase started: door.script
Reading the project graph slice (20 s): rederivation 256 of 256.
Reading the project graph slice (22 s): flush 1 of 1.
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 703 and delivered 0 within 0 s, 703 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

Required capacity/graph/fleet/wanted/arrived/serving/restored/restarting/adopted lines, verbatim:
```text
[capacity] run: sized at 8% of the default from 4 earlier run(s) of its class
[capacity] admitted run: 8 job(s), 1709 MB reserved (7932 MB available, 2048 MB floor)
[capacity] run: sized at 8% of the default from 4 earlier run(s) of its class
[capacity] run: sized at 46% of the default from 1 earlier run(s) of its class
[capacity] run: sized at 46% of the default from 1 earlier run(s) of its class
  fleet status                                    Show one picture of the shared workspace: who runs what (phase, progress, age, priority, code freshness), the queues with waiting-time estimates, memory admitted against really used, the Bazel servers, storage against its budgets, the sealed implementation and the last graph generation.
  --json                                          Print lists as JSON instead of one entry per line. (used by: graph, show, incremental, concurrency, bench, capacity, fleet, scheduler, leases, cache, remote-cache, doctor, tools, conformance, units, service, session)
  --origin <id>                                   remote-execution workers: origin id of this machine in the fleet (default: the origin of the replica, else the host name); the scheduler never gives a machine the units it requested itself. (used by: remote-execution)
  --limit=<n>                                     fleet status: how many runs the text output lists (default: 20, the JSON output always lists all). (used by: fleet)
  repo fleet status
      Run the service with the shared store and one local worker: gRPC for build tools on 127.0.0.1:8980, HTTP for workers, status and the fleet load table on 127.0.0.1:8981; hermetic Bazel actions can then run remotely.
  fleet status                                    Show one picture of the shared workspace: who runs what (phase, progress, age, priority, code freshness), the queues with waiting-time estimates, memory admitted against really used, the Bazel servers, storage against its budgets, the sealed implementation and the last graph generation.
  --json                                          Print lists as JSON instead of one entry per line. (used by: graph, show, incremental, concurrency, bench, capacity, fleet, scheduler, leases, cache, remote-cache, doctor, tools, conformance, units, service, session)
  --origin <id>                                   remote-execution workers: origin id of this machine in the fleet (default: the origin of the replica, else the host name); the scheduler never gives a machine the units it requested itself. (used by: remote-execution)
  --limit=<n>                                     fleet status: how many runs the text output lists (default: 20, the JSON output always lists all). (used by: fleet)
  repo fleet status
      Run the service with the shared store and one local worker: gRPC for build tools on 127.0.0.1:8980, HTTP for workers, status and the fleet load table on 127.0.0.1:8981; hermetic Bazel actions can then run remotely.
[capacity] dependencies: sized at 28% of the default from 4 earlier run(s) of its class
[capacity] admitted dependencies: 1 job(s), 1733 MB reserved (7930 MB available, 2048 MB floor)
[capacity] dependencies: sized at 28% of the default from 4 earlier run(s) of its class
[capacity] run: sized at 46% of the default from 1 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7784 MB available, 2048 MB floor)
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 703 and delivered 0 within 0 s, 703 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

### Try 3

Peak run-owned RSS 1124672 KiB; peak machine-wide sampled RSS 3559952 KiB. Samples: 11. All pressure observations normal (1): True. See `round-4-try-3-ps.json` for full ancestry and `round-4-try-3-pressure.json`.

Observed phase starts (approximately five-second log-poll resolution):
```text
2026-11-14T17:50:25Z elapsed=0.004s [semio] Building the repo launcher (it provisions the pinned JavaScript runtime itself)…
2026-11-14T17:50:30Z elapsed=5.113s [semio] Installing the native repository command…
2026-11-14T17:50:40Z elapsed=15.305s [semio] Installing the dependency scopes (the planner of every later run needs them)…
2026-11-14T17:51:01Z elapsed=35.588s [semio] workspace:setup…
```

Observed wall per phase:
- [semio] Building the repo launcher (it provisions the pinned JavaScript runtime itself)…: 5.109 seconds until next observed phase/end; exact internal wall not measured.
- [semio] Installing the native repository command…: 10.192 seconds until next observed phase/end; exact internal wall not measured.
- [semio] Installing the dependency scopes (the planner of every later run needs them)…: 20.283 seconds until next observed phase/end; exact internal wall not measured.
- [semio] workspace:setup…: 274.022 seconds until next observed phase/end; exact internal wall not measured.

Failed bootstrap phase: `workspace:setup`; no downstream Nx task list was emitted. First emitted refusal and 15 preceding lines, or interruption context if no error was emitted:
```text
Reading the project graph slice (244 s): flush 1 of 1.
Reading the project graph slice (246 s): flush 1 of 1.
Reading the project graph slice (248 s): flush 1 of 1.
Reading the project graph slice (250 s): flush 1 of 1.
Reading the project graph slice (252 s): flush 1 of 1.
Reading the project graph slice (254 s): flush 1 of 1.
Reading the project graph slice (256 s): flush 1 of 1.
Reading the project graph slice (258 s): flush 1 of 1.
[repo] still working after 260 s; last phase started: door.script
Reading the project graph slice (260 s): flush 1 of 1.
Reading the project graph slice (262 s): flush 1 of 1.
Reading the project graph slice (264 s): flush 1 of 1.
Reading the project graph slice (266 s): flush 1 of 1.
Reading the project graph slice (268 s): flush 1 of 1.
Reading the project graph slice (270 s): flush 1 of 1.
Reading the project graph slice (272 s): flush 1 of 1.
```

Required capacity/graph/fleet/wanted/arrived/serving/restored/restarting/adopted lines, verbatim:
```text
[capacity] run: sized at 8% of the default from 5 earlier run(s) of its class
[capacity] admitted run: 8 job(s), 1709 MB reserved (7390 MB available, 2048 MB floor)
[capacity] run: sized at 8% of the default from 5 earlier run(s) of its class
[capacity] run: sized at 46% of the default from 1 earlier run(s) of its class
[capacity] run: sized at 46% of the default from 1 earlier run(s) of its class
  fleet status                                    Show one picture of the shared workspace: who runs what (phase, progress, age, priority, code freshness), the queues with waiting-time estimates, memory admitted against really used, the Bazel servers, storage against its budgets, the sealed implementation and the last graph generation.
  --json                                          Print lists as JSON instead of one entry per line. (used by: graph, show, incremental, concurrency, bench, capacity, fleet, scheduler, leases, cache, remote-cache, doctor, tools, conformance, units, service, session)
  --origin <id>                                   remote-execution workers: origin id of this machine in the fleet (default: the origin of the replica, else the host name); the scheduler never gives a machine the units it requested itself. (used by: remote-execution)
  --limit=<n>                                     fleet status: how many runs the text output lists (default: 20, the JSON output always lists all). (used by: fleet)
  repo fleet status
      Run the service with the shared store and one local worker: gRPC for build tools on 127.0.0.1:8980, HTTP for workers, status and the fleet load table on 127.0.0.1:8981; hermetic Bazel actions can then run remotely.
  fleet status                                    Show one picture of the shared workspace: who runs what (phase, progress, age, priority, code freshness), the queues with waiting-time estimates, memory admitted against really used, the Bazel servers, storage against its budgets, the sealed implementation and the last graph generation.
  --json                                          Print lists as JSON instead of one entry per line. (used by: graph, show, incremental, concurrency, bench, capacity, fleet, scheduler, leases, cache, remote-cache, doctor, tools, conformance, units, service, session)
  --origin <id>                                   remote-execution workers: origin id of this machine in the fleet (default: the origin of the replica, else the host name); the scheduler never gives a machine the units it requested itself. (used by: remote-execution)
  --limit=<n>                                     fleet status: how many runs the text output lists (default: 20, the JSON output always lists all). (used by: fleet)
  repo fleet status
      Run the service with the shared store and one local worker: gRPC for build tools on 127.0.0.1:8980, HTTP for workers, status and the fleet load table on 127.0.0.1:8981; hermetic Bazel actions can then run remotely.
[capacity] dependencies: sized at 28% of the default from 5 earlier run(s) of its class
[capacity] admitted dependencies: 1 job(s), 1733 MB reserved (7249 MB available, 2048 MB floor)
[capacity] dependencies: sized at 28% of the default from 5 earlier run(s) of its class
[capacity] run: sized at 46% of the default from 1 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7401 MB available, 2048 MB floor)
```

## Resident service observations

The same resident service was reused across retries. Full matching lines observed at report time follow; per-request timestamps are not present on every line, so exact per-try attribution of these service lines is not measured. First and last arrived lines in this snapshot:

```text
graph: waiting for the fleet: 0 of 703 projects arrived
graph: waiting for the fleet: 0 of 703 projects arrived
```

```text
[capacity] admitted graph-plan: 1 job(s), 2048 MB reserved (6269 MB available, 2048 MB floor)
[capacity] admitted graph-plan: 1 job(s), 2048 MB reserved (6362 MB available, 2048 MB floor)
graph: No verified change journal cursor is recorded for generation 0; verifying the tree once.
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:0:126773","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 0 of 126773 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:77340:126773","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 77340 of 126773 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:121877:126773","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 121877 of 126773 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:126773:126773","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 126773 of 126773 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:0:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 0 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:4211","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 4211 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:4211:4211","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 4211 of 4211 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1604","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1604 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1604:1604","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1604 of 1604 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:417","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 417 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:417:417","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 417 of 417 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:718","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 718 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:718:718","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 718 of 718 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:817","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 817 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:817:817","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 817 of 817 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:675","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 675 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:675:675","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 675 of 675 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:454","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 454 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:454:454","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 454 of 454 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:32768:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 32768 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:896","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 896 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:896:896","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 896 of 896 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:505","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 505 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:505:505","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 505 of 505 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:930","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 930 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:930:930","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 930 of 930 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:536","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 536 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:536:536","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 536 of 536 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1197","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1197 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1197:1197","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1197 of 1197 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:364","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 364 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:364:364","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 364 of 364 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1298","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1298 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1298:1298","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1298 of 1298 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:416","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 416 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:416:416","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 416 of 416 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1170","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1170 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1170:1170","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1170 of 1170 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:375","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 375 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:375:375","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 375 of 375 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:849","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 849 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:849:849","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 849 of 849 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:595","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 595 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:595:595","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 595 of 595 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2310","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2310 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2310:2310","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2310 of 2310 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:230","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 230 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:230:230","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 230 of 230 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:5360","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 5360 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:5360:5360","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 5360 of 5360 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:6152","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 6152 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:6152:6152","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 6152 of 6152 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:106496:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 106496 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:5987","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 5987 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:5987:5987","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 5987 of 5987 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:5715","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 5715 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:5715:5715","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 5715 of 5715 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2850","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2850 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2850:2850","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2850 of 2850 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8115","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8115 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8115:8115","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8115 of 8115 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8075","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8075 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8075:8075","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8075 of 8075 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:147456:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 147456 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8086","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8086 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8086:8086","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8086 of 8086 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8140","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8140 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8140:8140","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8140 of 8140 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8074","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8074 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8074:8074","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8074 of 8074 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:7895","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 7895 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:7895:7895","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 7895 of 7895 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:7923","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 7923 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:7923:7923","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 7923 of 7923 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:188416:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 188416 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8000","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8000 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8000:8000","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8000 of 8000 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:7819","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 7819 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:7819:7819","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 7819 of 7819 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8154","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8154 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8154:8154","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8154 of 8154 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:212992:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 212992 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8150","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8150 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8150:8150","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8150 of 8150 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8030","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8030 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8030:8030","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8030 of 8030 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:5072","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 5072 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:5072:5072","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 5072 of 5072 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:146","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 146 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:146:146","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 146 of 146 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:237568:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 237568 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:82","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 82 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:82:82","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 82 of 82 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:260","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 260 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:260:260","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 260 of 260 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:26","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 26 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:26:26","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 26 of 26 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:137","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 137 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:137:137","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 137 of 137 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:311:311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 311 of 311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:19","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 19 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:19:19","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 19 of 19 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:3885","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 3885 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:3885:3885","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 3885 of 3885 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:159","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 159 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:159:159","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 159 of 159 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:2311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 2311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:2311:2311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 2311 of 2311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:101","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 101 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:101:101","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 101 of 101 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1131","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1131 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1131:1131","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1131 of 1131 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:127","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 127 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:127:127","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 127 of 127 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1635","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1635 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1635:1635","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1635 of 1635 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:140","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 140 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:140:140","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 140 of 140 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:2141","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 2141 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:2141:2141","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 2141 of 2141 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:119","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 119 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:119:119","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 119 of 119 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:2459","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 2459 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:2459:2459","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 2459 of 2459 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:319488:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 319488 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:138","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 138 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:138:138","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 138 of 138 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1417","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1417 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1417:1417","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1417 of 1417 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:349","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 349 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:349:349","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 349 of 349 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1163","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1163 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1163:1163","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1163 of 1163 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 277 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:277:277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 277 of 277 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1738","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1738 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1738:1738","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1738 of 1738 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:45","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 45 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:45:45","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 45 of 45 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:2132","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 2132 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:2132:2132","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 2132 of 2132 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:125","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 125 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:125:125","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 125 of 125 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1286","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1286 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1286:1286","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1286 of 1286 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1029","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1029 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1029:1029","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1029 of 1029 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1002","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1002 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1002:1002","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1002 of 1002 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1018","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1018 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1018:1018","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1018 of 1018 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:53","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 53 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:53:53","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 53 of 53 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1686","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1686 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1686:1686","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1686 of 1686 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:409600:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 409600 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:26","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 26 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:26:26","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 26 of 26 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:369","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 369 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:369:369","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 369 of 369 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:81","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 81 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:81:81","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 81 of 81 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:2611","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 2611 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:2611:2611","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 2611 of 2611 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:27","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 27 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:27:27","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 27 of 27 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:619","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 619 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:619:619","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 619 of 619 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:875","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 875 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:875:875","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 875 of 875 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:273","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 273 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:273:273","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 273 of 273 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:441651:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 441651 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:0:184336","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 0 of 184336 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:4096:184336","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 4096 of 184336 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:12288:184336","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 12288 of 184336 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:24576:184336","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 24576 of 184336 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:36864:184336","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 36864 of 184336 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:45056:184336","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 45056 of 184336 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:90112:184336","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 90112 of 184336 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:102400:184336","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 102400 of 184336 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:126976:184336","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 126976 of 184336 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:176144:184336","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 176144 of 184336 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:184336:184336","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 184336 of 184336 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:0:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 0 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:10:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 10 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:12:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 12 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:14:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 14 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:15:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 15 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:20:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 20 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:26:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 26 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:28:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 28 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:29:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 29 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:31:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 31 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:46:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 46 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:65:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 65 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:71:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 71 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:74:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 74 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:86:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 86 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:102:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 102 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:107:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 107 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:111:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 111 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:112:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 112 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:129:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 129 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:141:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 141 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:152:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 152 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:174:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 174 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:207:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 207 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:234:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 234 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:2254","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:2254:2254","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:127","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:127:127","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:257:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 257 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:284:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 284 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:302:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 302 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:314:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 314 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:333:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 333 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:373:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 373 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:398:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 398 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:418:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 418 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:421:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 421 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:427:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 427 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:452:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 452 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:474:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 474 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:508:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 508 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:1976","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:1976:1976","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:134","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:134:134","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:513:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 513 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:545:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 545 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:572:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 572 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:575:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 575 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:598:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 598 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:601:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 601 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:603:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 603 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:620:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 620 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:621:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 621 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:622:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 622 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:623:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 623 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:646:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 646 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:674:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 674 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:697:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 697 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:703:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 703 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:709:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 709 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:741:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 741 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:768:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 768 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:2613","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:2613:2613","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:146","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:146:146","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:769:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 769 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:781:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 781 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:787:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 787 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:810:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 810 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:829:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 829 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:862:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 862 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:887:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 887 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:934:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 934 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:958:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 958 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:985:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 985 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1024:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1024 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:2742","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:2742:2742","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:140","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:140:140","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1025:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1025 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1052:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1052 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1095:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1095 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1117:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1117 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1151:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1151 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1168:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1168 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1198:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1198 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1231:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1231 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1258:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1258 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1264:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1264 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1267:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1267 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:3013","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:3013:3013","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:150","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:150:150","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1281:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1281 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1310:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1310 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1322:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1322 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1335:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1335 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1345:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1345 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1363:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1363 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1382:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1382 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1423:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1423 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1447:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1447 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1455:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1455 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1464:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1464 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1471:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1471 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1498:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1498 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1517:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1517 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1524:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1524 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1530:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1530 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1531:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1531 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:2676","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:2676:2676","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:132","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:132:132","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1537:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1537 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1589:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1589 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1633:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1633 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1658:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1658 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1669:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1669 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1671:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1671 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1674:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1674 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1694:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1694 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1730:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1730 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1746:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1746 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1763:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1763 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1785:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1785 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1792:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1792 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:2854","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:2854:2854","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:140","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:140:140","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1793:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1793 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1819:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1819 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1872:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1872 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1899:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1899 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1930:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1930 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1940:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1940 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1952:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1952 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1964:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1964 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1978:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1978 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1997:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1997 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2030:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2030 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2048:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2048 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:2415","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:2415:2415","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:143","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:143:143","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2049:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2049 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2061:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2061 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2078:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2078 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2103:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2103 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2107:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2107 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2110:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2110 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2113:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2113 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2117:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2117 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2141:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2141 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2180:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2180 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2208:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2208 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2236:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2236 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2271:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2271 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2277:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2277 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:2339","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:2339:2339","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:124","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:124:124","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
graph: 703 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 703
graph: the fleet cache cannot be reached, so 703 stale projects cannot be requested from it; this machine can infer 192
graph: nothing to wait for any longer: asked the fleet for 703 stale projects and received 0 within 0 s, 703 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T17:43:34.229Z graph-wanted-expired {"wanted":"703","arrived":"0","seconds":"0","missing":"703","local":"192"}
[capacity] admitted graph-plan: 1 job(s), 2048 MB reserved (7784 MB available, 2048 MB floor)
graph: No verified change journal cursor is recorded for generation 0; verifying the tree once.
graph: the replica manifest cannot vouch for the tree (the replica daemon is not healthy); verifying the tree once.
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:0:126773","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 0 of 126773 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:97884:126773","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 97884 of 126773 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:126773:126773","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 126773 of 126773 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:0:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 0 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:4211","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 4211 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:4211:4211","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 4211 of 4211 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1604","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1604 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1604:1604","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1604 of 1604 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:417","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 417 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:417:417","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 417 of 417 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:718","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 718 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:718:718","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 718 of 718 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:817","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 817 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:817:817","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 817 of 817 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:675","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 675 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:675:675","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 675 of 675 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:454","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 454 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:454:454","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 454 of 454 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:896","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 896 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:896:896","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 896 of 896 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:505","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 505 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:505:505","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 505 of 505 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:930","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 930 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:930:930","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 930 of 930 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:536","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 536 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:536:536","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 536 of 536 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1197","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1197 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1197:1197","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1197 of 1197 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:364","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 364 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:364:364","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 364 of 364 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:57344:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 57344 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1298","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1298 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1298:1298","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1298 of 1298 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:416","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 416 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:416:416","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 416 of 416 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1170","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1170 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1170:1170","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1170 of 1170 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:375","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 375 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:375:375","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 375 of 375 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:849","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 849 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:849:849","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 849 of 849 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:595","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 595 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:595:595","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 595 of 595 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2310","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2310 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2310:2310","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2310 of 2310 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:230","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 230 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:230:230","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 230 of 230 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:5360","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 5360 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:5360:5360","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 5360 of 5360 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:6152","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 6152 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:6152:6152","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 6152 of 6152 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:5987","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 5987 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:5987:5987","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 5987 of 5987 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:5715","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 5715 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:5715:5715","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 5715 of 5715 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:122880:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 122880 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2850","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2850 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2850:2850","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2850 of 2850 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8115","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8115 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8115:8115","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8115 of 8115 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8075","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8075 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8075:8075","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8075 of 8075 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8086","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8086 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8086:8086","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8086 of 8086 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8140","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8140 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8140:8140","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8140 of 8140 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:163840:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 163840 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8074","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8074 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8074:8074","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8074 of 8074 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:7895","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 7895 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:7895:7895","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 7895 of 7895 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:7923","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 7923 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:7923:7923","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 7923 of 7923 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8000","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8000 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8000:8000","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8000 of 8000 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:196608:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 196608 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:7819","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 7819 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:7819:7819","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 7819 of 7819 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8154","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8154 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8154:8154","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8154 of 8154 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:212992:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 212992 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8150","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8150 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8150:8150","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8150 of 8150 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:221184:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 221184 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8030","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8030 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8030:8030","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8030 of 8030 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:229376:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 229376 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:5072","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 5072 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:5072:5072","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 5072 of 5072 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:146","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 146 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:146:146","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 146 of 146 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:82","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 82 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:82:82","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 82 of 82 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:260","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 260 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:260:260","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 260 of 260 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:26","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 26 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:26:26","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 26 of 26 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:137","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 137 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:137:137","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 137 of 137 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:311:311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 311 of 311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:19","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 19 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:19:19","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 19 of 19 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:3885","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 3885 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:3885:3885","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 3885 of 3885 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:159","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 159 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:159:159","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 159 of 159 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:2311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 2311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:2311:2311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 2311 of 2311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:101","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 101 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:101:101","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 101 of 101 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1131","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1131 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1131:1131","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1131 of 1131 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:127","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 127 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:127:127","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 127 of 127 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1635","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1635 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1635:1635","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1635 of 1635 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:303104:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 303104 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:140","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 140 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:140:140","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 140 of 140 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:2141","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 2141 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:2141:2141","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 2141 of 2141 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:119","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 119 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:119:119","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 119 of 119 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:2459","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 2459 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:2459:2459","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 2459 of 2459 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:138","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 138 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:138:138","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 138 of 138 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1417","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1417 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1417:1417","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1417 of 1417 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:349","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 349 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:349:349","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 349 of 349 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1163","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1163 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1163:1163","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1163 of 1163 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 277 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:277:277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 277 of 277 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1738","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1738 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1738:1738","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1738 of 1738 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:45","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 45 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:45:45","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 45 of 45 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:2132","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 2132 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:2132:2132","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 2132 of 2132 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:125","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 125 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:125:125","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 125 of 125 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1286","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1286 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1286:1286","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1286 of 1286 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1029","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1029 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1029:1029","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1029 of 1029 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1002","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1002 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1002:1002","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1002 of 1002 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:376832:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 376832 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1018","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1018 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1018:1018","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1018 of 1018 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:53","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 53 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:53:53","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 53 of 53 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1686","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1686 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1686:1686","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1686 of 1686 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:26","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 26 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:26:26","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 26 of 26 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:369","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 369 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:369:369","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 369 of 369 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:81","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 81 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:81:81","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 81 of 81 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:2611","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 2611 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:2611:2611","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 2611 of 2611 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:27","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 27 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:27:27","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 27 of 27 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:619","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 619 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:619:619","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 619 of 619 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:875","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 875 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:875:875","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 875 of 875 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:273","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 273 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:273:273","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 273 of 273 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:441651:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 441651 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:0:184133","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 0 of 184133 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:4096:184133","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 4096 of 184133 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:12288:184133","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 12288 of 184133 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:20480:184133","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 20480 of 184133 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:32768:184133","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 32768 of 184133 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:40960:184133","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 40960 of 184133 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:90112:184133","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 90112 of 184133 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:102400:184133","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 102400 of 184133 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:126976:184133","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 126976 of 184133 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:167936:184133","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 167936 of 184133 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:184133:184133","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 184133 of 184133 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:0:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 0 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:3:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 3 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:22:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 22 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:29:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 29 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:122:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 122 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:250:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 250 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:9162","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:2581:9162","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:5556:9162","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:8518:9162","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:9162:9162","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:256:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:257:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 257 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:418:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 418 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:8572","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:8572:8572","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:256:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:513:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 513 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:619:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 619 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:8287","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:8287:8287","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:256:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:769:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 769 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1024:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1024 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:10980","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:10980:10980","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:256:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1025:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1025 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:11291","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:11291:11291","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:256:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1281:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1281 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:11122","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:11122:11122","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:256:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1537:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1537 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1763:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1763 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:11928","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:11928:11928","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:256:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1793:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1793 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:10141","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:10141:10141","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:256:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2049:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2049 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2277:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2277 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:10798","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:10798:10798","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:229","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:229:229","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
graph: 703 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 703
graph: the fleet cache cannot be reached, so 703 stale projects cannot be requested from it; this machine can infer 192
graph: nothing to wait for any longer: asked the fleet for 703 stale projects and received 0 within 0 s, 703 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T17:47:24.421Z graph-wanted-expired {"wanted":"703","arrived":"0","seconds":"0","missing":"703","local":"192"}
[capacity] admitted graph-plan: 1 job(s), 2048 MB reserved (7421 MB available, 2048 MB floor)
graph: No verified change journal cursor is recorded for generation 0; verifying the tree once.
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:0:126773","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 0 of 126773 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:90237:126773","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 90237 of 126773 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:126773:126773","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 126773 of 126773 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:0:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 0 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:4211","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 4211 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:4211:4211","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 4211 of 4211 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1604","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1604 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1604:1604","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1604 of 1604 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:417","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 417 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:417:417","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 417 of 417 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:718","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 718 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:718:718","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 718 of 718 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:817","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 817 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:817:817","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 817 of 817 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:675","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 675 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:675:675","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 675 of 675 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:454","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 454 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:454:454","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 454 of 454 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:896","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 896 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:896:896","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 896 of 896 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:505","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 505 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:505:505","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 505 of 505 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:930","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 930 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:930:930","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 930 of 930 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:536","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 536 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:536:536","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 536 of 536 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:49152:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 49152 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1197","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1197 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1197:1197","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1197 of 1197 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:364","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 364 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:364:364","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 364 of 364 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1298","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1298 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1298:1298","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1298 of 1298 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:416","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 416 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:416:416","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 416 of 416 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1170","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1170 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1170:1170","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1170 of 1170 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:375","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 375 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:375:375","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 375 of 375 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:849","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 849 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:849:849","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 849 of 849 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:595","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 595 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:595:595","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 595 of 595 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2310","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2310 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2310:2310","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2310 of 2310 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:230","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 230 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:230:230","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 230 of 230 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:5360","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 5360 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:5360:5360","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 5360 of 5360 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:6152","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 6152 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:6152:6152","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 6152 of 6152 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:106496:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 106496 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:5987","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 5987 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:5987:5987","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 5987 of 5987 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:5715","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 5715 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:5715:5715","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 5715 of 5715 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2850","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2850 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2850:2850","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2850 of 2850 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8115","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8115 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8115:8115","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8115 of 8115 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8075","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8075 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8075:8075","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8075 of 8075 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:147456:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 147456 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8086","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8086 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8086:8086","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8086 of 8086 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8140","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8140 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8140:8140","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8140 of 8140 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8074","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8074 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8074:8074","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8074 of 8074 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:7895","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 7895 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:7895:7895","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 7895 of 7895 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:180224:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 180224 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:7923","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 7923 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:7923:7923","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 7923 of 7923 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8000","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8000 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8000:8000","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8000 of 8000 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:7819","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 7819 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:7819:7819","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 7819 of 7819 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:204800:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 204800 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8154","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8154 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8154:8154","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8154 of 8154 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8150","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8150 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8150:8150","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8150 of 8150 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8030","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8030 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8030:8030","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8030 of 8030 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:229376:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 229376 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:5072","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 5072 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:5072:5072","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 5072 of 5072 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:146","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 146 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:146:146","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 146 of 146 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:82","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 82 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:82:82","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 82 of 82 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:260","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 260 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:260:260","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 260 of 260 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:26","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 26 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:26:26","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 26 of 26 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:137","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 137 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:137:137","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 137 of 137 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:311:311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 311 of 311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:19","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 19 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:19:19","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 19 of 19 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:3885","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 3885 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:3885:3885","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 3885 of 3885 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:159","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 159 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:159:159","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 159 of 159 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:2311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 2311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:2311:2311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 2311 of 2311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:101","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 101 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:101:101","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 101 of 101 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1131","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1131 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1131:1131","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1131 of 1131 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:127","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 127 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:127:127","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 127 of 127 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1635","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1635 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1635:1635","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1635 of 1635 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:140","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 140 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:140:140","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 140 of 140 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:2141","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 2141 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:2141:2141","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 2141 of 2141 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:311296:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 311296 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:119","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 119 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:119:119","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 119 of 119 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:2459","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 2459 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:2459:2459","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 2459 of 2459 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:138","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 138 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:138:138","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 138 of 138 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1417","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1417 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1417:1417","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1417 of 1417 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:349","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 349 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:349:349","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 349 of 349 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1163","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1163 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1163:1163","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1163 of 1163 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 277 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:277:277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 277 of 277 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1738","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1738 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1738:1738","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1738 of 1738 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:45","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 45 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:45:45","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 45 of 45 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:2132","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 2132 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:2132:2132","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 2132 of 2132 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:125","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 125 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:125:125","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 125 of 125 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1286","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1286 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1286:1286","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1286 of 1286 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1029","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1029 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1029:1029","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1029 of 1029 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1002","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1002 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1002:1002","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1002 of 1002 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1018","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1018 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1018:1018","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1018 of 1018 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:53","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 53 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:53:53","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 53 of 53 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1686","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1686 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1686:1686","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1686 of 1686 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:393216:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 393216 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:26","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 26 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:26:26","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 26 of 26 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:369","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 369 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:369:369","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 369 of 369 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:81","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 81 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:81:81","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 81 of 81 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:2611","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 2611 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:2611:2611","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 2611 of 2611 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:27","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 27 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:27:27","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 27 of 27 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:619","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 619 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:619:619","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 619 of 619 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:875","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 875 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:875:875","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 875 of 875 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:273","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 273 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:273:273","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 273 of 273 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:441651:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 441651 of 441651 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:0:184214","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 0 of 184214 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:4096:184214","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 4096 of 184214 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:12288:184214","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 12288 of 184214 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:20480:184214","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 20480 of 184214 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:24576:184214","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 24576 of 184214 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:36864:184214","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 36864 of 184214 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:40960:184214","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 40960 of 184214 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:86016:184214","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 86016 of 184214 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:94208:184214","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 94208 of 184214 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:102400:184214","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 102400 of 184214 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:110592:184214","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 110592 of 184214 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:114688:184214","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 114688 of 184214 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:131072:184214","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 131072 of 184214 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:135168:184214","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 135168 of 184214 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:151552:184214","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 151552 of 184214 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:180118:184214","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 180118 of 184214 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:184214:184214","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 184214 of 184214 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:0:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 0 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:3:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 3 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:5:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 5 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:10:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 10 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:13:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 13 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:14:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 14 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:15:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 15 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:20:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 20 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:25:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 25 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:28:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 28 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:29:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 29 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:31:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 31 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:37:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 37 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:46:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 46 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:62:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 62 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:77:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 77 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:80:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 80 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:86:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 86 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:100:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 100 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:115:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 115 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:127:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 127 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:129:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 129 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:130:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 130 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:132:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 132 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:141:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 141 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:161:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 161 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:195:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 195 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:211:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 211 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:247:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 247 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:2254","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:2254:2254","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:127","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:127:127","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:257:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 257 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:291:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 291 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:299:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 299 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:334:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 334 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:342:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 342 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:381:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 381 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:411:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 411 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:431:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 431 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:434:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 434 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:436:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 436 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:468:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 468 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:512:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 512 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:1976","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:1976:1976","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:134","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:134:134","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:513:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 513 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:541:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 541 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:580:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 580 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:603:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 603 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:605:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 605 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:616:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 616 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:621:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 621 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:622:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 622 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:623:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 623 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:642:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 642 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:663:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 663 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:690:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 690 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:715:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 715 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:726:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 726 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:743:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 743 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:744:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 744 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:768:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 768 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:2613","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:2464:2613","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:2613:2613","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:146","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:146:146","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:769:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 769 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:785:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 785 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:790:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 790 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:795:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 795 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:820:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 820 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:832:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 832 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:880:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 880 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:930:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 930 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:953:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 953 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:985:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 985 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1024:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1024 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:2742","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:2742:2742","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:140","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:140:140","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1025:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1025 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1060:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1060 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1107:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1107 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1139:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1139 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1179:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1179 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1229:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1229 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1256:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1256 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1267:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1267 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:3013","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:3013:3013","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:150","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:150:150","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1281:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1281 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1327:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1327 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1366:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1366 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1404:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1404 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1443:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1443 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1481:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1481 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1495:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1495 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1507:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1507 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1530:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1530 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:2676","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:2676:2676","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:132","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:132:132","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1537:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1537 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1591:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1591 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1635:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1635 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1638:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1638 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1645:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1645 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1648:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1648 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1664:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1664 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1685:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1685 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1689:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1689 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1690:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1690 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1691:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1691 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1693:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1693 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1696:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1696 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1709:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1709 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1716:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1716 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1720:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1720 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1746:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1746 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1762:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1762 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1765:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1765 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1776:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1776 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1792:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1792 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:2854","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:2854:2854","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:140","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:140:140","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1793:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1793 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1814:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1814 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1866:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1866 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1901:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1901 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1939:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1939 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1971:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1971 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2013:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2013 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2047:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2047 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:2415","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:2415:2415","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:143","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:143:143","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2049:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2049 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2101:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2101 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2105:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2105 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2107:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2107 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2108:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2108 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2118:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2118 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2159:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2159 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2197:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2197 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2236:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2236 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2277:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2277 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:2339","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:2339:2339","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:124","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:124:124","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
graph: 703 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 703
graph: waiting for the fleet: 0 of 703 projects arrived
graph: waiting for the fleet: 0 of 703 projects arrived
graph: waiting for the fleet: 0 of 703 projects arrived
graph: waiting for the fleet: 0 of 703 projects arrived
graph: waiting for the fleet: 0 of 703 projects arrived
graph: waiting for the fleet: 0 of 703 projects arrived
graph: waiting for the fleet: 0 of 703 projects arrived
graph: waiting for the fleet: 0 of 703 projects arrived
graph: waiting for the fleet: 0 of 703 projects arrived
graph: waiting for the fleet: 0 of 703 projects arrived
graph: waiting for the fleet: 0 of 703 projects arrived
graph: waiting for the fleet: 0 of 703 projects arrived
graph: waiting for the fleet: 0 of 703 projects arrived
graph: waiting for the fleet: 0 of 703 projects arrived
graph: waiting for the fleet: 0 of 703 projects arrived
graph: waiting for the fleet: 0 of 703 projects arrived
graph: waiting for the fleet: 0 of 703 projects arrived
graph: waiting for the fleet: 0 of 703 projects arrived
```

## Completion

Exact final-attempt last 200 native lines: `round-4.tail.txt`. Exact tails for every try and all process ancestry samples are retained. Coordinator interruption evidence: `round-4-coordinator-interruption.json`. Replica code digest: `39e3688eb315d48831772ca305d78bcb6f35975794444da47750bc5fa7459759`. No permissions or build source were changed. Proceeding to round 5 as instructed.

Report completed 2026-11-14T17:56:42Z.
