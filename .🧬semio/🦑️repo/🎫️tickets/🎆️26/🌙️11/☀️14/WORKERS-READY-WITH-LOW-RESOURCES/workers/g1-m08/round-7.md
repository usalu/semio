# Round 7 — g1-m08 — Complete

**Not ready.** Try 1 ended with fleet refusal native_rc=75 (658 wanted, 199 arrived, 459 missing after 1482 seconds). After a measured 120-second wait, try 2 reached 144 of 658 arrivals; coordinator round 8 superseded it, so the observer sent one SIGINT at 20:54:47 UTC and waited a minute. Try 2 logged native_rc=7. No resource stop, consent error or launchd timeout occurred. No kickstart, graph gc or permission repair was performed.

## Machine and pre-run observations

Memory 17179869184 bytes (16 GiB), macOS 27.2 build 26B123, Apple M4 arm64. Replica digest before round: `28b4f47dc4d69fae3a080d0c58d3520741c02c444760d2a3cec506db943ae8ff`. Required note checks followed the first repo preflight command, which exited 0. The graph job used the stable launcher and had an ok service access receipt. Entry bootVersion was 5 before this round, then measured 6 during try 2; replica digest during try 2 was `d72ff30ebb826bfabac6ee5f126041ee5f833672897443b9666c557ce85335c1`. Full note checks, process parents, job arguments and last eight sidecar lines: `round-7-note-before-bootstrap.json`, `round-7-note-during-try2.json`, and `round-7-preflight-observation.json`. Environment blocks were excluded. No denial or exit 4 occurred, so note section 3 was not run.

Free disk before:
```text
Filesystem   1024-blocks      Used Available Capacity iused      ifree %iused  Mounted on
/dev/disk3s5   239362496  84609452 117016516    42%  831977 1170165160    0%   /System/Volumes/Data
```

Free disk after:
```text
Filesystem   1024-blocks      Used Available Capacity iused      ifree %iused  Mounted on
/dev/disk3s5   239362496  84887708 116738260    43%  832291 1167382600    0%   /System/Volumes/Data
```

## Attempts

| Try | Start UTC | End UTC observed | Wall s | native_rc | Wanted / arrived / missing / fleet wait s | Retry sleep s |
|---|---|---|---:|---:|---|---:|
| 1 | 2026-11-14T19:59:40Z | 2026-11-14T20:27:57Z | 1697.394 | 75 | 658 / 199 / 459 / 1482 | 120.01 |
| 2 | 2026-11-14T20:29:57Z | 2026-11-14T20:55:48Z | 1550.752 | 7 | 658 wanted, 144 arrived; missing and total wait not emitted before interruption | 0 |

Try 2 observed wall includes the final minute after SIGINT; native logged end was 20:54:47 UTC. No try 3 was started after supersession. Native completion marker absent; ready=false.

## Try 1 observations

Peak run-owned RSS 3730800 KiB (3.558 GiB); machine-wide sampled peak 3730752 KiB. Required top-three samples: 56; full PID/PPID/commands and ancestor chains retained. Pressure checks: 336, all normal (1): True. Round-6 hard stops were used: RSS 8 GiB, critical 4 twice at 30 seconds, warning 2 six times at 30 seconds. No hard stop fired.

First / last arrived lines, verbatim:
```text
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
```

Succeeded setup tasks: none emitted. Failed Nx tasks: none emitted. Not-run task names: not measured; graph loading never emitted a downstream task result list. Bootstrap phase workspace:setup was incomplete. First native refusal with 15 preceding lines, or final interruption context if no error was emitted:
```text
Reading the project graph slice (1636 s): flush 1 of 1.
Reading the project graph slice (1638 s): flush 1 of 1.
[repo] still working after 1640 s; last phase started: door.script
Reading the project graph slice (1640 s): flush 1 of 1.
Reading the project graph slice (1642 s): flush 1 of 1.
Reading the project graph slice (1644 s): flush 1 of 1.
Reading the project graph slice (1646 s): flush 1 of 1.
Reading the project graph slice (1648 s): flush 1 of 1.
Reading the project graph slice (1650 s): flush 1 of 1.
Reading the project graph slice (1652 s): flush 1 of 1.
Reading the project graph slice (1654 s): flush 1 of 1.
Reading the project graph slice (1656 s): flush 1 of 1.
Reading the project graph slice (1658 s): flush 1 of 1.
Reading the project graph slice (1660 s): flush 1 of 1.
Reading the project graph slice (1662 s): flush 1 of 1.
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 658 and delivered 199 within 1482 s, 459 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

Observed phase wall (approximately five-second log-poll resolution):

| Phase | Start elapsed s | Wall until next observed phase/end s |
|---|---:|---:|
| [semio] Building the repo launcher (it provisions the pinned JavaScript runtime itself)… | 0.002 | 5.101 |
| [semio] Installing the native repository command… | 5.103 | 10.109 |
| [semio] Installing the dependency scopes (the planner of every later run needs them)… | 15.212 | 20.356 |
| [semio] workspace:setup… | 35.568 | 1661.826 |

Exact internal phase wall is not measured; the final try-2 interval includes the signal grace minute. Required graph/capacity/fleet/restored/closureRepaired lines, verbatim:
```text
NATIVE
[capacity] run: sized at 8% of the default from 8 earlier run(s) of its class
[capacity] admitted run: 8 job(s), 1709 MB reserved (6938 MB available, 2048 MB floor)
[capacity] run: sized at 8% of the default from 8 earlier run(s) of its class
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
[capacity] dependencies: sized at 28% of the default from 8 earlier run(s) of its class
[capacity] admitted dependencies: 1 job(s), 1733 MB reserved (6854 MB available, 2048 MB floor)
[capacity] dependencies: sized at 28% of the default from 8 earlier run(s) of its class
[capacity] run: sized at 46% of the default from 1 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (6792 MB available, 2048 MB floor)
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 658 and delivered 199 within 1482 s, 459 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.

RESIDENT SERVICE
[capacity] admitted graph-plan: 1 job(s), 2048 MB reserved (6792 MB available, 2048 MB floor)
graph: No verified change journal cursor is recorded for generation 60; verifying the tree once.
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:0:126988","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 0 of 126988 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:72849:126988","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 72849 of 126988 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:95552:126988","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 95552 of 126988 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:98954:126988","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 98954 of 126988 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:101976:126988","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 101976 of 126988 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:108866:126988","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 108866 of 126988 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:119780:126988","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 119780 of 126988 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:126988:126988","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 126988 of 126988 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:0:451523","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 0 of 451523 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:3295","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 3295 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:3295:3295","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 3295 of 3295 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2512","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2512 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2512:2512","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2512 of 2512 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:964","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 964 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:964:964","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 964 of 964 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1030","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1030 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1030:1030","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1030 of 1030 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:16","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 16 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:16:16","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 16 of 16 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1025","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1025 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1025:1025","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1025 of 1025 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:471","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 471 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:471:471","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 471 of 471 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:40960:451523","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 40960 of 451523 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1318","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1318 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1318:1318","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1318 of 1318 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:238","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 238 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:238:238","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 238 of 238 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1196","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1196 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1196:1196","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1196 of 1196 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:178","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 178 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:178:178","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 178 of 178 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1264","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1264 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1264:1264","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1264 of 1264 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:138","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 138 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:138:138","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 138 of 138 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1001","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1001 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1001:1001","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1001 of 1001 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:310","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 310 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:310:310","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 310 of 310 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2288","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2288 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2288:2288","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2288 of 2288 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:741","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 741 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:741:741","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 741 of 741 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8153","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8153 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8153:8153","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8153 of 8153 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8071","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8071 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8071:8071","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8071 of 8071 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:98304:451523","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 98304 of 451523 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8123","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8123 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8123:8123","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8123 of 8123 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8150","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8150 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8150:8150","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8150 of 8150 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8115","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8115 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8115:8115","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8115 of 8115 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:7880","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 7880 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:7880:7880","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 7880 of 7880 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:131072:451523","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 131072 of 451523 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:7947","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 7947 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:7947:7947","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 7947 of 7947 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8016","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8016 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8016:8016","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8016 of 8016 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:7848","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 7848 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:7848:7848","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 7848 of 7848 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:155648:451523","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 155648 of 451523 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8157","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8157 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8157:8157","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8157 of 8157 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8168","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8168 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8168:8168","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8168 of 8168 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8072","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8072 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8072:8072","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8072 of 8072 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:180224:451523","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 180224 of 451523 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:5450","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 5450 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:5450:5450","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 5450 of 5450 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:86","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 86 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:86:86","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 86 of 86 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:700","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 700 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:700:700","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 700 of 700 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:174","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 174 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:174:174","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 174 of 174 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:196608:451523","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 196608 of 451523 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:4111","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 4111 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:4111:4111","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 4111 of 4111 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:246","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 246 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:246:246","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 246 of 246 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:5258","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 5258 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:5258:5258","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 5258 of 5258 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:6149","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 6149 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1024:6149","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1024 of 6149 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:6149:6149","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 6149 of 6149 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:221184:451523","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 221184 of 451523 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:5743","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 5743 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:5743:5743","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 5743 of 5743 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:3029","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 3029 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:3029:3029","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 3029 of 3029 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:641","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 641 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:641:641","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 641 of 641 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:30","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 30 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:30:30","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 30 of 30 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:9","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 9 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:9:9","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 9 of 9 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:5108","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 5108 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:5108:5108","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 5108 of 5108 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:270336:451523","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 270336 of 451523 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:450","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 450 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:450:450","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 450 of 450 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:201","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 201 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:201:201","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 201 of 201 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:220","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 220 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:220:220","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 220 of 220 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:122","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 122 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:122:122","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 122 of 122 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:6","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 6 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:6:6","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 6 of 6 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:41","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 41 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:41:41","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 41 of 41 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:136","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 136 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:136:136","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 136 of 136 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:61","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 61 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:61:61","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 61 of 61 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:6467","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 6467 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:6467:6467","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 6467 of 6467 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:344064:451523","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 344064 of 451523 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:11","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 11 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:11:11","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 11 of 11 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:5679","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 5679 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:5679:5679","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 5679 of 5679 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:352256:451523","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 352256 of 451523 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:197","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 197 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:197:197","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 197 of 197 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1811","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1811 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1811:1811","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1811 of 1811 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:45","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 45 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:45:45","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 45 of 45 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:820","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 820 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:820:820","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 820 of 820 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 169 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:169:169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 169 of 169 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1735","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1735 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1735:1735","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1735 of 1735 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:73","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 73 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:73:73","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 73 of 73 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:758","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 758 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:758:758","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 758 of 758 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:140","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 140 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:140:140","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 140 of 140 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1395","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1395 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1395:1395","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1395 of 1395 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:5","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 5 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:5:5","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 5 of 5 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:667","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 667 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:667:667","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 667 of 667 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:82","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 82 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:82:82","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 82 of 82 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:365","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 365 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:365:365","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 365 of 365 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:5","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 5 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:5:5","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 5 of 5 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1982","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1982 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1982:1982","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1982 of 1982 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:417792:451523","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 417792 of 451523 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:6","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 6 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:6:6","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 6 of 6 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8:8","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8 of 8 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:824","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 824 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:824:824","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 824 of 824 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:33","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 33 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:33:33","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 33 of 33 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1828","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1828 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1828:1828","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1828 of 1828 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:3733","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 3733 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:3733:3733","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 3733 of 3733 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:4","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 4 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:4:4","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 4 of 4 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:451523:451523","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 451523 of 451523 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:0:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 0 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:4096:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 4096 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:12288:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 12288 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:20480:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 20480 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:28672:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 28672 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:40960:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 40960 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:45056:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 45056 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:57344:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 57344 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:73728:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 73728 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:77824:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 77824 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:86016:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 86016 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:98304:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 98304 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:102400:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 102400 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:106496:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 106496 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:110592:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 110592 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:126976:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 126976 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:151552:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 151552 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:159744:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 159744 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:163840:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 163840 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:176128:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 176128 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:180224:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 180224 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:192512:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 192512 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:196608:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 196608 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:204800:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 204800 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:212992:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 212992 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:221184:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 221184 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:229376:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 229376 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:241664:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 241664 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:245760:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 245760 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:253952:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 253952 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:258048:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 258048 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:266240:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 266240 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:272044:272044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 272044 of 272044 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:0:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 0 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:3:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 3 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:5:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 5 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:6:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 6 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:8:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 8 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:9:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 9 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:12:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 12 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:13:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 13 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:14:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 14 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:15:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 15 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:16:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 16 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:21:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 21 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:25:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 25 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:26:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 26 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:30:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 30 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:32:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 32 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:36:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 36 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:46:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 46 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:51:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 51 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:55:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 55 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:56:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 56 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:57:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 57 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:61:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 61 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:70:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 70 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:87:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 87 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:106:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 106 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:118:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 118 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:134:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 134 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:145:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 145 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:150:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 150 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:157:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 157 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:171:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 171 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:173:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 173 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:174:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 174 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:176:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 176 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:181:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 181 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:192:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 192 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:197:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 197 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:198:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 198 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:200:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 200 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:202:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 202 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:204:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 204 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:205:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 205 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:206:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 206 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:207:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 207 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:208:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 208 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:209:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 209 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:210:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 210 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:211:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 211 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:212:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 212 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:213:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 213 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:215:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 215 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:222:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 222 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:234:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 234 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:245:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 245 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:249:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 249 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:254:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 254 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:257:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 257 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:276:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 276 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:287:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 287 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:293:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 293 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:300:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 300 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:310:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 310 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:321:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 321 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:325:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 325 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:331:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 331 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:336:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 336 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:349:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 349 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:360:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 360 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:374:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 374 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:385:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 385 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:395:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 395 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:400:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 400 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:403:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 403 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:404:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 404 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:405:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 405 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:406:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 406 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:408:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 408 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:409:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 409 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:411:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 411 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:415:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 415 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:423:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 423 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:436:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 436 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:441:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 441 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:459:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 459 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:462:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 462 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:468:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 468 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:469:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 469 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:470:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 470 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:476:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 476 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:490:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 490 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:497:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 497 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:508:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 508 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:128","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:128:128","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:513:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 513 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:529:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 529 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:545:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 545 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:552:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 552 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:554:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 554 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:557:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 557 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:559:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 559 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:560:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 560 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:562:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 562 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:573:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 573 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:586:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 586 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:599:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 599 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:605:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 605 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:624:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 624 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:635:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 635 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:654:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 654 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:670:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 670 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:682:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 682 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:699:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 699 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:700:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 700 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:713:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 713 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:723:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 723 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:740:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 740 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:753:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 753 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:768:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 768 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:769:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 769 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:787:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 787 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:805:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 805 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:832:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 832 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:852:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 852 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:871:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 871 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:884:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 884 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:905:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 905 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:922:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 922 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:942:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 942 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:962:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 962 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:984:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 984 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:997:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 997 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1009:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1009 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1025:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1025 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1042:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1042 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1052:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1052 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1073:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1073 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1083:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1083 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
graph: 658 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 658
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 143 of 658 projects arrived
graph: waiting for the fleet: 151 of 658 projects arrived
graph: waiting for the fleet: 151 of 658 projects arrived
graph: waiting for the fleet: 151 of 658 projects arrived
graph: waiting for the fleet: 151 of 658 projects arrived
graph: waiting for the fleet: 151 of 658 projects arrived
graph: waiting for the fleet: 151 of 658 projects arrived
graph: waiting for the fleet: 151 of 658 projects arrived
graph: waiting for the fleet: 151 of 658 projects arrived
graph: waiting for the fleet: 151 of 658 projects arrived
graph: waiting for the fleet: 151 of 658 projects arrived
graph: waiting for the fleet: 151 of 658 projects arrived
graph: waiting for the fleet: 151 of 658 projects arrived
graph: waiting for the fleet: 151 of 658 projects arrived
graph: waiting for the fleet: 151 of 658 projects arrived
graph: waiting for the fleet: 151 of 658 projects arrived
graph: waiting for the fleet: 151 of 658 projects arrived
graph: waiting for the fleet: 151 of 658 projects arrived
graph: waiting for the fleet: 151 of 658 projects arrived
graph: waiting for the fleet: 151 of 658 projects arrived
graph: waiting for the fleet: 151 of 658 projects arrived
graph: waiting for the fleet: 151 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: waiting for the fleet: 199 of 658 projects arrived
graph: nothing to wait for any longer: asked the fleet for 658 stale projects and received 199 within 1482 s, 459 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T20:27:55.345Z graph-wanted-expired {"wanted":"658","arrived":"199","seconds":"1482","missing":"459","local":"192"}
```

Graph-service last 30 lines at try completion:

host-293b476f6b9d.log

```text
graph: waiting for the fleet: 199 of 658 projects arrived
[repo] still working after 1910 s; last phase started: source.served
[repository:command] still running after 1910 s
graph: waiting for the fleet: 199 of 658 projects arrived
[repository:command] still running after 1920 s
graph: waiting for the fleet: 199 of 658 projects arrived
[repository:command] still running after 1930 s
graph: waiting for the fleet: 199 of 658 projects arrived
[repo] still working after 1940 s; last phase started: source.served
[repository:command] still running after 1940 s
graph: waiting for the fleet: 199 of 658 projects arrived
[repository:command] still running after 1950 s
graph: waiting for the fleet: 199 of 658 projects arrived
[repository:command] still running after 1960 s
graph: waiting for the fleet: 199 of 658 projects arrived
[repo] still working after 1970 s; last phase started: source.served
[repository:command] still running after 1970 s
graph: waiting for the fleet: 199 of 658 projects arrived
[repository:command] still running after 1980 s
graph: waiting for the fleet: 199 of 658 projects arrived
[repository:command] still running after 1990 s
[repo] still working after 2000 s; last phase started: source.served
graph: waiting for the fleet: 199 of 658 projects arrived
[repository:command] still running after 2000 s
graph: waiting for the fleet: 199 of 658 projects arrived
[repository:command] still running after 2010 s
graph: waiting for the fleet: 199 of 658 projects arrived
graph: nothing to wait for any longer: asked the fleet for 658 stale projects and received 199 within 1482 s, 459 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T20:27:55.345Z graph-wanted-expired {"wanted":"658","arrived":"199","seconds":"1482","missing":"459","local":"192"}
[repository:command] still running after 2020 s
```

## Try 2 observations

Peak run-owned RSS 4026656 KiB (3.840 GiB); machine-wide sampled peak 4026560 KiB. Required top-three samples: 52; full PID/PPID/commands and ancestor chains retained. Pressure checks: 307, all normal (1): True. Round-6 hard stops were used: RSS 8 GiB, critical 4 twice at 30 seconds, warning 2 six times at 30 seconds. No hard stop fired.

First / last arrived lines, verbatim:
```text
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
```

Succeeded setup tasks: none emitted. Failed Nx tasks: none emitted. Not-run task names: not measured; graph loading never emitted a downstream task result list. Bootstrap phase workspace:setup was incomplete. First native refusal with 15 preceding lines, or final interruption context if no error was emitted:
```text
Reading the project graph slice (1278 s): flush 1 of 1.
Reading the project graph slice (1280 s): flush 1 of 1.
Reading the project graph slice (1282 s): flush 1 of 1.
Reading the project graph slice (1284 s): flush 1 of 1.
Reading the project graph slice (1286 s): flush 1 of 1.
Reading the project graph slice (1288 s): flush 1 of 1.
Reading the project graph slice (1290 s): flush 1 of 1.
Reading the project graph slice (1292 s): flush 1 of 1.
Reading the project graph slice (1294 s): flush 1 of 1.
Reading the project graph slice (1296 s): flush 1 of 1.
Reading the project graph slice (1298 s): flush 1 of 1.
Reading the project graph slice (1300 s): flush 1 of 1.
Reading the project graph slice (1302 s): flush 1 of 1.
[repo] still working after 1310 s; last phase started: door.script
Reading the project graph slice (1304 s): flush 1 of 1.
Reading the project graph slice (1306 s): flush 1 of 1.
```

Observed phase wall (approximately five-second log-poll resolution):

| Phase | Start elapsed s | Wall until next observed phase/end s |
|---|---:|---:|
| [semio] Building the repo launcher (it provisions the pinned JavaScript runtime itself)… | 0.005 | 5.103 |
| [semio] Installing the native repository command… | 5.108 | 10.099 |
| [semio] Installing the dependency scopes (the planner of every later run needs them)… | 15.207 | 162.422 |
| [semio] workspace:setup… | 177.629 | 1373.123 |

Exact internal phase wall is not measured; the final try-2 interval includes the signal grace minute. Required graph/capacity/fleet/restored/closureRepaired lines, verbatim:
```text
NATIVE
[capacity] run: sized at 5% of the default from 9 earlier run(s) of its class
[capacity] admitted run: 8 job(s), 1205 MB reserved (7548 MB available, 2048 MB floor)
[capacity] run: sized at 5% of the default from 9 earlier run(s) of its class
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
[capacity] dependencies: sized at 28% of the default from 9 earlier run(s) of its class
[capacity] admitted dependencies: 1 job(s), 1733 MB reserved (7625 MB available, 2048 MB floor)
[capacity] dependencies: sized at 28% of the default from 9 earlier run(s) of its class
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":1.7224170000008598}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":60.24054200000137}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.6877500000009604}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":50.042333000001236}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":1.0005000000019209}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":127.8291250000002}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":1.286415999999008}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":63.80437499999971}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.9871249999996508}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":48.57795900000201}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.8497500000012224}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":51.72412500000064}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.6514999999999418}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":39.62883300000249}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":1.2421659999963595}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":56.126916000001074}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.8562089999977616}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":63.17962499999703}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":1.7380000000048312}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":70.07787500000268}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":1.975749999997788}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":52.12462500001129}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.9404999999969732}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":45.59300000000803}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":1.0001669999910519}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":54.18083299999125}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.9340420000080485}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":52.674582999999984}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":1.1891669999895385}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":75.79695799999172}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.5951250000071013}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":46.774417000007816}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.996832999997423}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":57.62054099999659}}
[capacity] run: sized at 46% of the default from 1 earlier run(s) of its class
[capacity] waiting 0s for run: not enough free memory (need 2048 MB; 1315 MB spare above the 2048 MB floor, load 5.14; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] admitted run: 1 job(s), 2048 MB reserved (7726 MB available, 2048 MB floor)

RESIDENT SERVICE
[capacity] admitted graph-plan: 1 job(s), 2048 MB reserved (6956 MB available, 2048 MB floor)
graph: No verified change journal cursor is recorded for generation 60; verifying the tree once.
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:0:126988","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 0 of 126988 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:92577:126988","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 92577 of 126988 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:95552:126988","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 95552 of 126988 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:100664:126988","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 100664 of 126988 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:107008:126988","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 107008 of 126988 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:117974:126988","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 117974 of 126988 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:126988:126988","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 126988 of 126988 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:0:451523","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 0 of 451523 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:3295","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 3295 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:3295:3295","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 3295 of 3295 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2512","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2512 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2512:2512","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2512 of 2512 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:964","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 964 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:964:964","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 964 of 964 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:24576:451523","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 24576 of 451523 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1030","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1030 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1030:1030","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1030 of 1030 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:16","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 16 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:16:16","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 16 of 16 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1025","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1025 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1025:1025","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1025 of 1025 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:471","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 471 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:471:471","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 471 of 471 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1318","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1318 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1318:1318","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1318 of 1318 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:238","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 238 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:238:238","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 238 of 238 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1196","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1196 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1196:1196","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1196 of 1196 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:178","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 178 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:178:178","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 178 of 178 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1264","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1264 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1264:1264","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1264 of 1264 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:138","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 138 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:138:138","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 138 of 138 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1001","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1001 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1001:1001","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1001 of 1001 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:310","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 310 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:310:310","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 310 of 310 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2288","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2288 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2288:2288","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2288 of 2288 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:741","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 741 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:741:741","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 741 of 741 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8153","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8153 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8153:8153","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8153 of 8153 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:90112:451523","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 90112 of 451523 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8071","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8071 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8071:8071","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8071 of 8071 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8123","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8123 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8123:8123","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8123 of 8123 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8150","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8150 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8150:8150","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8150 of 8150 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8115","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8115 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8115:8115","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8115 of 8115 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:122880:451523","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 122880 of 451523 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:7880","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 7880 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:7880:7880","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 7880 of 7880 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:7947","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 7947 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:7947:7947","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 7947 of 7947 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8016","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8016 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8016:8016","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8016 of 8016 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:147456:451523","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 147456 of 451523 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:7848","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 7848 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:7848:7848","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 7848 of 7848 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8157","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8157 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8157:8157","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8157 of 8157 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:163840:451523","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 163840 of 451523 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8168","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8168 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8168:8168","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8168 of 8168 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8072","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8072 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8072:8072","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8072 of 8072 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:5450","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 5450 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:5450:5450","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 5450 of 5450 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:86","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 86 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:86:86","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 86 of 86 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:188416:451523","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 188416 of 451523 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:700","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 700 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:700:700","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 700 of 700 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:174","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 174 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:174:174","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 174 of 174 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:4111","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 4111 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:4111:4111","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 4111 of 4111 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:246","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 246 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:246:246","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 246 of 246 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:5258","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 5258 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:5258:5258","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 5258 of 5258 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:6149","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 6149 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:6149:6149","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 6149 of 6149 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:5743","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 5743 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:5743:5743","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 5743 of 5743 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:229376:451523","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 229376 of 451523 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:3029","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 3029 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:3029:3029","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 3029 of 3029 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:641","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 641 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:641:641","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 641 of 641 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:30","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 30 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:30:30","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 30 of 30 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:9","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 9 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:9:9","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 9 of 9 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:5108","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 5108 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:5108:5108","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 5108 of 5108 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:450","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 450 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:450:450","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 450 of 450 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:201","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 201 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:201:201","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 201 of 201 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:220","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 220 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:220:220","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 220 of 220 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:122","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 122 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:122:122","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 122 of 122 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:6","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 6 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:6:6","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 6 of 6 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:41","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 41 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:41:41","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 41 of 41 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:136","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 136 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:136:136","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 136 of 136 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:335872:451523","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 335872 of 451523 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:61","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 61 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:61:61","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 61 of 61 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:6467","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 6467 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:6467:6467","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 6467 of 6467 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:11","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 11 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:11:11","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 11 of 11 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:5679","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 5679 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:5679:5679","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 5679 of 5679 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:197","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 197 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:197:197","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 197 of 197 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1811","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1811 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1811:1811","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1811 of 1811 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:45","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 45 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:45:45","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 45 of 45 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:820","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 820 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:820:820","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 820 of 820 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 169 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:169:169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 169 of 169 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1735","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1735 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1735:1735","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1735 of 1735 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:73","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 73 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:73:73","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 73 of 73 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:758","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 758 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:758:758","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 758 of 758 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:140","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 140 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:140:140","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 140 of 140 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1395","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1395 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1395:1395","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1395 of 1395 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:5","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 5 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:5:5","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 5 of 5 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:667","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 667 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:667:667","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 667 of 667 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:401408:451523","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 401408 of 451523 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:82","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 82 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:82:82","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 82 of 82 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:365","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 365 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:365:365","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 365 of 365 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:5","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 5 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:5:5","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 5 of 5 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1982","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1982 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1982:1982","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1982 of 1982 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:6","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 6 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:6:6","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 6 of 6 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:8","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 8 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:8:8","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 8 of 8 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:824","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 824 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:824:824","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 824 of 824 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:33","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 33 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:33:33","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 33 of 33 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1828","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1828 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1828:1828","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1828 of 1828 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:3733","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 3733 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:3733:3733","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 3733 of 3733 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:4","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 4 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:4:4","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 4 of 4 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:451523:451523","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 451523 of 451523 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:0:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 0 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:12288:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 12288 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:24576:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 24576 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:32768:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 32768 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:36864:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 36864 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:45056:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 45056 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:53248:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 53248 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:65536:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 65536 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:77824:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 77824 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:81920:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 81920 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:86016:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 86016 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:90112:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 90112 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:94208:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 94208 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:102400:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 102400 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:106496:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 106496 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:114688:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 114688 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:122880:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 122880 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:135168:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 135168 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:147456:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 147456 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:159744:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 159744 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:172032:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 172032 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:180224:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 180224 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:184320:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 184320 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:196608:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 196608 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:200704:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 200704 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:204800:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 204800 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:212992:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 212992 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:217088:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 217088 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:225280:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 225280 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:229376:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 229376 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:245760:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 245760 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:253952:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 253952 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:258048:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 258048 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:266240:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 266240 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:272169:272169","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 272169 of 272169 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:0:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 0 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:3:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 3 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:4:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 4 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:5:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 5 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:6:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 6 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:8:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 8 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:9:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 9 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:18:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 18 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:25:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 25 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:27:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 27 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:29:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 29 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:37:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 37 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:43:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 43 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:49:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 49 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:62:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 62 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:72:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 72 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:83:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 83 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:85:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 85 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:86:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 86 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:97:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 97 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:105:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 105 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:110:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 110 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:113:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 113 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:114:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 114 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:120:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 120 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:129:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 129 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:131:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 131 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:135:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 135 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:138:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 138 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:148:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 148 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:150:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 150 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:151:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 151 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:152:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 152 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:153:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 153 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:155:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 155 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:159:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 159 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:162:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 162 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:163:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 163 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:165:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 165 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:166:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 166 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:171:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 171 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:176:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 176 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:178:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 178 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:179:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 179 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:185:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 185 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:192:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 192 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:194:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 194 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:198:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 198 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:199:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 199 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:200:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 200 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:202:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 202 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:205:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 205 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:208:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 208 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:210:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 210 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:214:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 214 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:237:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 237 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:249:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 249 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:1613","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:1613:1613","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:7","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:7:7","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:257:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 257 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:275:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 275 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:294:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 294 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:311:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 311 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:330:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 330 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:352:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 352 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:369:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 369 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:371:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 371 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:376:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 376 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:377:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 377 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:379:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 379 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:382:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 382 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:384:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 384 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:388:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 388 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:400:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 400 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:415:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 415 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:417:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 417 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:419:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 419 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:434:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 434 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:455:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 455 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:464:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 464 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:472:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 472 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:488:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 488 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:508:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 508 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:950","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:950:950","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:7","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:7:7","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:513:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 513 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:528:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 528 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:552:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 552 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:553:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 553 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:554:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 554 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:557:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 557 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:560:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 560 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:573:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 573 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:592:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 592 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:603:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 603 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:611:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 611 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:629:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 629 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:638:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 638 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:651:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 651 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:664:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 664 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:679:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 679 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:698:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 698 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:699:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 699 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:700:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 700 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:707:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 707 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:714:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 714 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:719:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 719 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:731:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 731 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:748:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 748 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:764:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 764 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:768:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 768 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:1871","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:1867:1871","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:1871:1871","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:11","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:11:11","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:769:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 769 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:791:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 791 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:806:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 806 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:834:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 834 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:853:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 853 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:860:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 860 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:865:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 865 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:874:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 874 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:894:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 894 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:914:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 914 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:934:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 934 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:951:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 951 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:953:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 953 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:957:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 957 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:979:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 979 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:993:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 993 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:999:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 999 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1020:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1020 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:1924","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:1924:1924","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:11","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:11:11","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1025:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1025 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1048:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1048 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1061:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1061 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1080:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1080 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1083:1083","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1083 of 1083 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:1865","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:1865:1865","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:5","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:5:5","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
graph: 658 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 658
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 96 of 658 projects arrived
graph: waiting for the fleet: 96 of 658 projects arrived
graph: waiting for the fleet: 96 of 658 projects arrived
graph: waiting for the fleet: 96 of 658 projects arrived
graph: waiting for the fleet: 96 of 658 projects arrived
graph: waiting for the fleet: 96 of 658 projects arrived
graph: waiting for the fleet: 96 of 658 projects arrived
graph: waiting for the fleet: 96 of 658 projects arrived
graph: waiting for the fleet: 96 of 658 projects arrived
graph: waiting for the fleet: 96 of 658 projects arrived
graph: waiting for the fleet: 96 of 658 projects arrived
graph: waiting for the fleet: 96 of 658 projects arrived
graph: waiting for the fleet: 96 of 658 projects arrived
graph: waiting for the fleet: 96 of 658 projects arrived
graph: waiting for the fleet: 96 of 658 projects arrived
graph: waiting for the fleet: 96 of 658 projects arrived
graph: waiting for the fleet: 96 of 658 projects arrived
graph: waiting for the fleet: 96 of 658 projects arrived
graph: waiting for the fleet: 96 of 658 projects arrived
graph: waiting for the fleet: 96 of 658 projects arrived
graph: waiting for the fleet: 96 of 658 projects arrived
graph: waiting for the fleet: 96 of 658 projects arrived
graph: waiting for the fleet: 96 of 658 projects arrived
graph: waiting for the fleet: 96 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
graph: waiting for the fleet: 144 of 658 projects arrived
```

Graph-service last 30 lines at try completion:

host-293b476f6b9d.log

```text
[repo] still working after 2180 s; last phase started: source.served
[repository:command] still running after 2180 s
[repository:command] still running after 2190 s
[repository:command] still running after 2200 s
[repo] still working after 2210 s; last phase started: source.served
[repository:command] still running after 2211 s
[repository:command] still running after 2221 s
[repository:command] still running after 2231 s
[repo] still working after 2240 s; last phase started: source.served
[repository:command] still running after 2241 s
[repository:command] still running after 2251 s
[repository:command] still running after 2261 s
[repo] still working after 2270 s; last phase started: source.served
[repository:command] still running after 2271 s
[repository:command] still running after 2281 s
[repository:command] still running after 2291 s
[repo] still working after 2300 s; last phase started: source.served
[repository:command] still running after 2301 s
[repository:command] still running after 2311 s
[repository:command] still running after 2321 s
[repo] still working after 2330 s; last phase started: source.served
[repository:command] still running after 2331 s
[repository:command] still running after 2341 s
[repository:command] still running after 2351 s
[repo] still working after 2360 s; last phase started: source.served
[repository:command] still running after 2361 s
[repository:command] still running after 2371 s
[repository:command] still running after 2381 s
2026-11-14T20:34:01.200Z shutdown superseded
[script] {"owner":"repository:build","command":"repo","stage":"complete"}
```

host-6f286e53e68a.log

```text
graph: waiting for the fleet: 144 of 658 projects arrived
[repository:command] still running after 1230 s
graph: waiting for the fleet: 144 of 658 projects arrived
[repository:command] still running after 1240 s
[repo] still working after 1250 s; last phase started: source.served
graph: waiting for the fleet: 144 of 658 projects arrived
[repository:command] still running after 1250 s
graph: waiting for the fleet: 144 of 658 projects arrived
[repository:command] still running after 1260 s
graph: waiting for the fleet: 144 of 658 projects arrived
[repository:command] still running after 1270 s
[repo] still working after 1280 s; last phase started: source.served
graph: waiting for the fleet: 144 of 658 projects arrived
[repository:command] still running after 1280 s
graph: waiting for the fleet: 144 of 658 projects arrived
[repository:command] still running after 1290 s
graph: waiting for the fleet: 144 of 658 projects arrived
[repository:command] still running after 1300 s
[repo] still working after 1310 s; last phase started: source.served
SIGINT received: the running work is cancelled and its leases are released; this process exits after 10 s at the latest.
[repository:command] still running after 1310 s
Cancelled (SIGINT); leases released.
[repository:command] still running after 1320 s
[repository:command] still running after 1330 s
[repo] still working after 1340 s; last phase started: source.served
[repository:command] still running after 1340 s
[repository:command] still running after 1350 s
[repository:command] still running after 1360 s
[repo] still working after 1370 s; last phase started: source.served
[repository:command] still running after 1370 s
```

## Replica status after the round

Supported command exited 0:
```text
Replica g1-m08 of group g1, store /Volumes/semio-store: reachable
Synchronization: on
Daemon: running (pid 29075)
281571 files tracked; queued: 10 incoming, ? outgoing; 26 overwritten files kept in .cache/f/lost
Sidecar build-agent: running (process 29400, 0 restart(s), 189 s in this state, last exit -).
Sidecar build-scheduler: off on this machine (process -, 0 restart(s), 222 s in this state, last exit -).
Sidecar build-cache: off on this machine (process -, 0 restart(s), 222 s in this state, last exit -).
Journal: 9.97 MiB in 13 segments of 9 origins.
Newest snapshot by g1-server: 32685s old, 89.18 MiB.
Reclaimable now: 2 journal segments (2.29 MiB).
Last compaction 42148s ago: 861 blobs (27.00 GiB) removed; 88 blobs (129.23 MiB) were still too young.
Git state: shared (stage and commits replicate); stage tree -.
Background tiers, last pass (cost, duty cycle of one core): hot 32 ms 8%, directory sweep 439 ms 10%, file sweep 1029 ms 1%, tree walk -, git view 38851 ms 5%.
Latency over the last 13 publishes and 67 applies: publish p50 1509 ms, p95 1569 ms; apply p50 313 ms, p95 1830 ms.
g1-m01: 1 events (395 B) behind, last event 1s ago
g1-m02: 0 events (0 B) behind, last event 8s ago
g1-m03: 0 events (0 B) behind, last event 15s ago
g1-m04: 0 events (0 B) behind, last event 84s ago
g1-m05: 0 events (0 B) behind, last event 8273s ago
g1-m06: 0 events (0 B) behind, last event 3610s ago
g1-m07: 0 events (0 B) behind, last event 2832s ago
g1-m08: 0 events (0 B) behind, last event 63s ago
g1-server: 0 events (0 B) behind, last event 136s ago
Fleet: origin platform age lag (origins behind/queued) basis (cp:s = compared at its checkpoint taken s seconds ago, now = compared as it is now) sync code view (modified/deleted/untracked) latency (publish p50/p95 ms | apply p50/p95 ms) verdict
  g1-m01 macos 1s 0/0 cp:1s on 590fdef8b758 134/12/534 230/2152|149/1940 ok
  g1-m02 macos 3s 0/0 cp:3s on 590fdef8b758 134/12/534 253/5249|105/970 lagging (not settled: no common checkpoint within 60 s yet, still moving)
  g1-m03 macos 1s 0/0 cp:1s on 590fdef8b758 134/12/534 212/534|192/740 diverged
  g1-m04 linux 1s 0/0 cp:1s on 590fdef8b758 134/11/534 225/14454|323/1066 diverged
  g1-m05 windows 2s 0/4 cp:- on 6996465a9065 - -/-|2978/12807 lagging (not settled: no common checkpoint within 60 s yet, still moving)
  g1-m06 macos 2831s 6/0 cp:2831s on 28b4f47dc4d6 110/10/413 341/363|230/2162 stale (attestation too old)
  g1-m07 macos 2829s 6/0 cp:2829s on 28b4f47dc4d6 109/10/409 231/1343|278/2833 stale (attestation too old)
  g1-m08 macos 1s 0/0 cp:1s on 590fdef8b758 134/12/534 1509/1569|313/1830 ok
  g1-server macos 0s 0/0 cp:0s on 590fdef8b758 134/12/534 197/274|239/1612 ok
  warning: g1-m04 runs git 2.55.0, other machines run another version
  warning: the volume of g1-m04 behaves differently from most (case=sensitive normalisation=sensitive); the view is pinned, so this is a capability difference, not a divergence
  g1-m03 differs in indexTree at a common checkpoint: - instead of 5c75c44d73137807925a4f37dfbb0a95529a546e (#n = bucket n of 256 of the paths, value = its digest)
  g1-m04 differs in view.digest at a common checkpoint: #16=1e8a,#52=6fde,#86=0997,#114=864b,#121=b976,#147=9704,#163=3887,#188=c73f,#21 instead of #16=321b,#52=44e9,#86=a296,#114=43d8,#121=09af,#147=4540,#163=e9ed,#188=4b70,#21 (#n = bucket n of 256 of the paths, value = its digest)
The fleet is not in agreement (9 origins; fresh and checkpoint age mean within 60 s; lagging is not wrong, only not yet comparable).
```

Replica login job was observed running before and during both tries; observer kickstart count is 0. Its launchd runs count increased from 4 to 5 independently of this observer. Old graph job exited 0 and the new implementation job was observed running via the stable launcher; no timeout recovery was triggered.

Exact final 200 native lines: `round-7.tail.txt`; each try also has its exact tail. Per-try RSS/pressure/events/service logs and first/last arrivals are retained. No source edits, sudo, mutating git, broad process termination or permissions changes were performed. Proceeding to round 8 as instructed.

Report completed 2026-11-14T20:56:58Z.
