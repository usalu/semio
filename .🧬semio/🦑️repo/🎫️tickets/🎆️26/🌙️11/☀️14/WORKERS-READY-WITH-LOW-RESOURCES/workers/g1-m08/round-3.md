# Round 3 — g1-m08 — Complete

**Not ready.** The bootstrap reached graph loading and `Planning 1 targets`, then failed during `workspace:setup`: the resident service reported a macOS Documents access denial, reference `e07192d7`. Its service log records `Relative filesystem fstatat failed (EPERM)`. Native exit was 1. No fleet refusal or resource hard stop occurred, so no retry was eligible. No service restart, permission change, source edit, sudo or mutating git command was attempted.

## Pre-run observations

Observed and written before launch at 2026-11-14T16:52:06.040803+00:00. Replica code digest from health: `39e3688eb315d48831772ca305d78bcb6f35975794444da47750bc5fa7459759` (PID 75485). No matching `repo show projects` or `repo service host` process was found in the process inventory. No sidecar-parented project-show process required a three-minute wait. See `round-3-preflight-observation.json`.

Last eight build-agent lines, verbatim:
```text
2026-11-14T16:47:09.314Z agent code changed, restarting
2026-11-14T16:47:09.318Z worker stopped: ueli-workstation-mac-mini-m4-16gb-256gb-05-wlan.fritz.box-2422
2026-11-14T16:47:10.286Z joining http://192.168.178.70:8981 offering rustc for 3 target(s)
2026-11-14T16:47:10.328Z graph-server: serving 703 stale projects for g1-m03
2026-11-14T16:47:10.328Z graph-server: serving 703 stale projects for g1-m07
2026-11-14T16:47:10.346Z worker sandbox: darwin-sandbox confined network denied
2026-11-14T16:47:10.353Z worker registered: c80fc980-f7b5-4a71-a32f-0640c8d1cdf0 cache http://192.168.178.70:6094
2026-11-14T16:49:03.366Z graph-server: serving 703 stale projects for g1-m06
```

## Machine and measurements

| Measurement | Value |
|---|---|
| Origin | g1-m08 |
| OS | macOS 27.2, build 26B123, arm64 Apple M4 |
| Memory | 17179869184 bytes (16 GiB) |
| Free disk before | 117479992 KiB |
| Free disk after | 117200000 KiB |
| Start UTC | 2026-11-14T16:53:18Z |
| Native logged end UTC | 2026-11-14T16:57:05Z |
| Controller observed end UTC | 2026-11-14T16:57:07Z |
| Controller wall | 228.688 seconds |
| native_rc | 1; wrapper exit 0 |
| Completion marker | Absent |
| ready | false |
| Peak run-owned process RSS | 4568432 KiB (4.357 GiB), checked approximately every 5 seconds |
| Peak machine-wide RSS in 30-second samples | 4568432 KiB (4.357 GiB) |
| 30-second top-three samples | 8, from launch, with PID, PPID, full command and ancestor chains |
| Memory pressure | 46 observations, all normal (1): True |
| Signals to run | None |

Samples used both protocol commands: `ps -axo rss,command | sort -nr | head -3` and `ps -axo rss,pid,ppid,args | sort -nr | head -3`. Full results and ancestry to launchd are in `round-3-try-1-ps.json`. No git executable appeared among the three largest sampled processes. The peak sampled process was Bun PID 5857; its final sampled ancestry was 5857 → 5840 → 5403 → 5397 → 5391 → 6956 → 6930 → 1. No credentials were intentionally collected; credential-shaped values are redacted by the observer.

## Attempt and retry decision

| Try | Wall seconds | Wanted | Arrived | Missing | Fleet wait seconds | Retry sleep seconds |
|---|---:|---|---|---|---|---:|
| 1 | 228.688 | Not measured | Not measured | Not measured | Not measured; no fleet-wait line emitted | 0 |

First arrived line: not emitted. Last arrived line: not emitted. No second try: native_rc was 1 and no fleet refusal text was emitted; this was an access-denial failure. The sidecar pre-run lines about serving other workers are observations, not evidence that this run received projects.

## Phase timing

Observed by log polling approximately every 5 seconds; these are observed transition intervals, not exact internal timings. Multiple transitions within a poll do not have separately measured durations.

| Phase | Observed elapsed interval (s) | Observed wall (s) |
|---|---|---:|
| Bootstrap and launcher installation | 0.001–5.176 | 5.175 |
| Toolchain and native command installation | 5.176–20.383 | 15.207 |
| Dependency scopes | 20.383–223.610 | 203.227 |
| workspace:setup through controller completion | 223.610–228.688 | 5.078 |

## Failed phase and first error with preceding context

Failed phase: `workspace:setup`. No downstream Nx task failure list was emitted; downstream task errors are not measured. Native log, first access-denial line with the 15 lines before it, verbatim:

```text
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"dependencies:preservation-ownership:865280","mode":"exclusive","elapsedMs":0}}
[repository:command] still running after 200 s
[script] {"owner":"repository:build","command":"repo","stage":"complete"}
[semio] workspace:setup…
[capacity] run: sized at 46% of the default from 1 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7461 MB available, 2048 MB floor)
[script] {"owner":"repository:build","command":"repo","stage":"loading"}
[script] {"owner":"repository:build","command":"repo","stage":"running"}
Starting the repo graph service for this host.
Waiting for the repo graph service (1 ms).
Waiting for the repo graph service (2025 ms).
Waiting for the repo graph service (4055 ms).
[repo] still working after 5 s; last phase started: door.script
Loading the project graph of 440cffa5563d.
Planning 1 targets.
macOS denied the resident service access to your Documents folder (reference e07192d7). The service runs without the file access consent of this session. Allow `repo` under System Settings, Privacy & Security, Files and Folders (or Full Disk Access), run `repo service restart`, or keep the checkout outside Documents, Desktop and Downloads. Details: /var/folders/t7/ckfnmc712gsdkt0tc8042br00000gn/T/semio-repo/501/host-f74179f31e9f.log.
```

Resident service first error and 15 preceding lines, verbatim:

```text
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:implementation-source","stage":"running"}
[script] build:implementation-source · Reading the sources of the build implementation
[script] {"owner":"repository:build","command":"build:implementation-source","stage":"running"}
[script] {"owner":"repository:build","command":"build:implementation-source","stage":"running"}
[script] {"owner":"repository:build","command":"build:implementation-source","stage":"running"}
[script] {"owner":"repository:build","command":"build:implementation-source","stage":"running"}
[script] {"owner":"repository:build","command":"build:implementation-source","stage":"running"}
[capacity] admitted graph-plan: 1 job(s), 2048 MB reserved (6269 MB available, 2048 MB floor)
[repo] still working after 5 s; last phase started: source.served
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] build:source-admission · Admitting the sources of the build
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
2026-11-14T16:57:05.035Z request-failed e07192d7 Error: Relative filesystem fstatat failed (EPERM)
```

The first stack frames are preserved in `round-3-service-failure-context.txt`. The client text diagnoses macOS Documents consent; the underlying measured service error is EPERM from relative fstatat. The exact denied filesystem path and macOS consent identity were not measured. No proposed repair from the error text was executed.

## Required capacity / graph / fleet / wanted / arrived / serving / restored / restarting / adopted lines

Verbatim native and corresponding resident-service lines follow. Help text that contains these words is retained.

```text
NATIVE LOG
[capacity] run: sized at 30% of the default from 2 earlier run(s) of its class
[capacity] admitted run with 7 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 30% of the default from 2 earlier run(s) of its class
[capacity] run: sized at 52% of the default from 1 earlier run(s) of its class
[capacity] run: sized at 52% of the default from 1 earlier run(s) of its class
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
[capacity] dependencies: sized at 37% of the default from 2 earlier run(s) of its class
[capacity] admitted dependencies: 1 job(s), 2282 MB reserved (6906 MB available, 2048 MB floor)
[capacity] dependencies: sized at 37% of the default from 2 earlier run(s) of its class
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":84.921292}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.6463329999996859}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":57.40154199999961}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.9676670000008016}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":66.44925000000148}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.5909580000006827}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":60.44008300000132}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.9522079999987909}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":52.838166999998066}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":1.1948749999974098}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":53.2979579999992}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.9469580000004498}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":61.27520799999911}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":1.2107499999983702}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":52.763207999996666}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":7.324291000011726}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":59.00383300000976}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":1.3155830000032438}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":67.90783300000476}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.665500000002794}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":58.524042000004556}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":2.259749999997439}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":63.41187500000524}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":4.1038750000006985}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":57.72387499999604}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.6078330000018468}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":46.03137500000594}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":1.1553750000020955}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":60.05387499999779}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.5944580000068527}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":43.787374999999884}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.6943330000212882}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":45.56287500000326}}
[capacity] run: sized at 46% of the default from 1 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7461 MB available, 2048 MB floor)

RESIDENT SERVICE LOG (through failure e07192d7)
[capacity] admitted graph-plan: 1 job(s), 2048 MB reserved (6269 MB available, 2048 MB floor)
```

## Replica status after the run

Supported replica-module status command exited 0. Full output:

```text
Replica g1-m08 of group g1, store /Volumes/semio-store: reachable
Synchronization: on
Daemon: running (pid 75485)
281144 files tracked; queued: 10 incoming, ? outgoing; 26 overwritten files kept in .cache/f/lost
Recurring failures of kind refused: 123 times, last 3059 s ago.
Recurring failures of kind unstable: 1 times, last 3924 s ago.
Sidecar build-agent: running (process 75650, 0 restart(s), 4009 s in this state, last exit -).
Sidecar build-scheduler: off on this machine (process -, 0 restart(s), 4040 s in this state, last exit -).
Sidecar build-cache: off on this machine (process -, 0 restart(s), 4040 s in this state, last exit -).
Journal: 8.48 MiB in 12 segments of 9 origins.
Newest snapshot by g1-server: 18322s old, 89.18 MiB.
Reclaimable now: 2 journal segments (2.29 MiB).
Last compaction 27785s ago: 861 blobs (27.00 GiB) removed; 88 blobs (129.23 MiB) were still too young.
Git state: shared (stage and commits replicate); stage tree 5c75c44d73137807925a4f37dfbb0a95529a546e.
Background tiers, last pass (cost, duty cycle of one core): hot 2 ms 0.7%, directory sweep 513 ms 10%, file sweep 5561 ms 4.9%, tree walk 23765 ms 1.6%, git view 26706 ms 5%.
Latency over the last 100 publishes and 100 applies: publish p50 281931 ms, p95 281950 ms; apply p50 323 ms, p95 1071 ms.
g1-m01: 0 events (0 B) behind, last event 20s ago
g1-m02: 0 events (0 B) behind, last event 13s ago
g1-m03: 0 events (0 B) behind, last event 5s ago
g1-m04: 0 events (0 B) behind, last event 16s ago
g1-m05: 0 events (0 B) behind, last event 20865s ago
g1-m06: 0 events (0 B) behind, last event 2241s ago
g1-m07: 0 events (0 B) behind, last event 30s ago
g1-m08: 0 events (0 B) behind, last event 23s ago
g1-server: 0 events (0 B) behind, last event 382s ago
Fleet: origin platform age lag (origins behind/queued) basis (cp:s = compared at its checkpoint taken s seconds ago, now = compared as it is now) sync code view (modified/deleted/untracked) latency (publish p50/p95 ms | apply p50/p95 ms) verdict
  g1-m01 macos 1s 0/0 cp:1s on 39e3688eb315 64/12/93 184/583|253/648 lagging (not settled: no common checkpoint within 60 s yet, still moving)
  g1-m02 macos 1s 0/0 cp:1s on 39e3688eb315 63/12/83 257/729|241/824 lagging (not settled: no common checkpoint within 60 s yet, still moving)
  g1-m03 macos 0s 0/0 cp:0s on 39e3688eb315 65/12/107 174/412|242/780 ok
  g1-m04 linux 0s 0/0 cp:0s on 39e3688eb315 65/11/107 180/20721|337/19678 diverged
  g1-m05 windows 42s 6/4 cp:- on 73b7f3d88f52 66/12/81 -/-|11676/173707 lagging (not settled: no common checkpoint within 60 s yet, still moving)
  g1-m06 macos 1s 0/0 cp:1s on 39e3688eb315 65/12/107 -/-|177/812 ok
  g1-m07 macos 1s 0/0 cp:1s on 39e3688eb315 64/12/103 189/599|195/1035 ok
  g1-m08 macos 1s 0/0 cp:1s on 39e3688eb315 65/12/107 281931/281950|323/1071 diverged
  g1-server macos 1s 0/0 cp:1s on 39e3688eb315 65/12/107 516/6667|1002/7362 ok
  warning: g1-m04 runs git 2.55.0, other machines run another version
  warning: the volume of g1-m04 behaves differently from most (case=sensitive normalisation=sensitive); the view is pinned, so this is a capability difference, not a divergence
  warning: g1-m06 runs git 2.50.1, other machines run another version
  g1-m04 differs in view.digest at a common checkpoint: #16=56e3,#52=473b,#86=1047,#114=1e75,#121=b976,#147=9f30,#163=6ac3,#188=a20d,#21 instead of #16=7a72,#52=6c0c,#86=bb46,#114=dbe6,#121=09af,#147=4d74,#163=bba9,#188=2e42,#21 (#n = bucket n of 256 of the paths, value = its digest)
  g1-m08 differs in manifest at a common checkpoint: #114=8514 instead of #114=3fc3 (#n = bucket n of 256 of the paths, value = its digest)
The fleet is not in agreement (9 origins; fresh and checkpoint age mean within 60 s; lagging is not wrong, only not yet comparable).
```

## Artifacts and next step

`round-3.tail.txt` contains the exact last 200 native log lines. `round-3-try-1-result.json`, `round-3-try-1-ps.json`, `round-3-try-1-pressure.json` and `round-3-try-1-events.json` contain the observations. The run used the coordinator-provided native wrapper once. Waiting for a higher round every 60 seconds, for at most six hours from completion.

Report completed 2026-11-14T16:58:36Z.
