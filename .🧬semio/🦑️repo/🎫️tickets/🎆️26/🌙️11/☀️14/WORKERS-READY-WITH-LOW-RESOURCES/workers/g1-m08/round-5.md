# Round 5 — g1-m08 — Complete

**Not ready.** Attempt 1 ended native_rc=1 during `workspace:setup`: a newly started resident graph service again reported macOS Documents denial, reference `e07192d7`, with underlying `Relative filesystem fstatat failed (EPERM)`. No fleet refusal, native_rc=75 or resource stop occurred; no retry was eligible. Permissions and build code were not changed.

## Required pre-run observations

Observed and written before launch 2026-11-14T17:57:11.905217+00:00. Replica code digest: `39e3688eb315d48831772ca305d78bcb6f35975794444da47750bc5fa7459759`, daemon PID 75485. Pre-run process inventory with full commands is in `round-5-preflight-observation.json`; PID 27987 (PPID 1) and PID 27988 (PPID 27987) were the resident graph-host wrapper/child, both age 15:29, RSS 54096 / 282160 KiB.

Last eight sidecar lines, verbatim:
```text
2026-11-14T17:50:43.001Z joining http://192.168.178.70:8981 offering rustc for 3 target(s)
2026-11-14T17:50:43.050Z worker sandbox: darwin-sandbox confined network denied
2026-11-14T17:50:43.062Z worker registered: c3d4502b-7b22-4835-ae1c-401af5e665ef cache http://192.168.178.70:6094
2026-11-14T17:53:42.637Z agent code changed, restarting
2026-11-14T17:53:42.644Z worker stopped: ueli-workstation-mac-mini-m4-16gb-256gb-05-wlan.fritz.box-31598
2026-11-14T17:53:43.900Z joining http://192.168.178.70:8981 offering rustc for 3 target(s)
2026-11-14T17:53:43.945Z worker sandbox: darwin-sandbox confined network denied
2026-11-14T17:53:43.956Z worker registered: 7a589c1e-89ac-48ad-af66-f8c08d72952c cache http://192.168.178.70:6094
```

## Machine and result

Memory 17179869184 bytes (16 GiB), macOS 27.2 build 26B123, Apple M4 arm64.

Free disk before / after:
```text
Filesystem   1024-blocks      Used Available Capacity iused      ifree %iused  Mounted on
/dev/disk3s5   239362496  84492368 117133600    42%  831663 1171336000    0%   /System/Volumes/Data
Filesystem   1024-blocks      Used Available Capacity iused      ifree %iused  Mounted on
/dev/disk3s5   239362496  84400000 116800000    42%  831680 1171305840    0%   /System/Volumes/Data

```

| Measurement | Value |
|---|---|
| Start UTC | 2026-11-14T17:59:55Z |
| Native logged end UTC | 2026-11-14T18:00:29Z |
| Controller observed end UTC | 2026-11-14T18:00:30Z |
| Wall | 35.613 seconds |
| native_rc | 1; wrapper exit 0 |
| Completion marker | Absent |
| ready | false |
| Peak run-owned RSS | 1033472 KiB (0.986 GiB) |
| Peak machine-wide RSS sampled | 1991360 KiB (1.899 GiB) |
| 30-second top-three samples | 2, from launch, both protocol commands with full PID/PPID ancestry |
| Memory pressure | 8 checks, all normal (1): True |
| Signals | None |

RSS ownership included the known reused host, new host metadata, process group and descendants. Critical pressure and run RSS were checked about every five seconds; consecutive warning samples were 30 seconds apart, with the round-5 threshold of five warnings. No oversized-process or memory-pressure stop was observed.

## Per-try fleet measurements

| Try | Wanted | Arrived | Missing | Fleet wait seconds | Retry sleep |
|---|---|---|---|---|---|
| 1 | Not measured | Not measured | Not measured | Not measured; no fleet-wait progress for the new request | 0 seconds |

There was no eligible second try. The EPERM failure happened before an emitted fleet refusal. Pre-existing host log heartbeat lines continued during this try; these do not demonstrate completed or arrived graph records for this new request.

## EPERM process and parent chain

The failing log `host-6feebb05efe3.log` maps through its host metadata to **PID 37394**, startedAt 1794679227684 (epoch milliseconds). Observed parent chain: **37394 → 37382 → 1 (launchd)**. PID 37394 was the Bun immutable-loader child ending with implementation `d19cd38cfedf86bd463f7052a3e92f505178aedfdbe41e5d865d90f7f30637a7 service host run`; parent 37382 ran Bun with the repository build-door script and `repo service host run`. This native bootstrap printed `Starting the repo graph service for this host.` before the failure. The wrapper had detached to launchd. Full commands, timestamps and RSS are captured in `round-5-try-1-result.json` under graph_hosts, plus `round-5-try-1-ps.json`.

Exact denied filesystem path: **not measured / not named in the log**. The stack names the filesystem descriptor implementation file, which is code location rather than the denied path. The client identifies Documents access consent as the cause; measured service error is EPERM from relative fstatat. No permission repair or service restart was attempted.

## Failed phase and first error with 15 preceding lines

Failed bootstrap phase `workspace:setup`. No downstream Nx task failure list was emitted; downstream task names and first errors are not measured. Native first error, verbatim:

```text
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"dependencies:sync-gate:7392","mode":"shared","elapsedMs":9068.80175}}
[DATA] Store dependency sync current {"version":1,"purpose":"current-store-dependency-sync-current","scopes":6}
[script] {"owner":"repository:build","command":"repo","stage":"complete"}
[semio] workspace:setup…
[capacity] run: sized at 46% of the default from 1 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (6787 MB available, 2048 MB floor)
[script] {"owner":"repository:build","command":"repo","stage":"loading"}
[script] {"owner":"repository:build","command":"repo","stage":"running"}
Starting the repo graph service for this host.
Waiting for the repo graph service (1 ms).
Waiting for the repo graph service (2048 ms).
Waiting for the repo graph service (4093 ms).
Loading the project graph of 440cffa5563d.
Planning 1 targets.
[repo] still working after 5 s; last phase started: door.script
macOS denied the resident service access to your Documents folder (reference e07192d7). The service runs without the file access consent of this session. Allow `repo` under System Settings, Privacy & Security, Files and Folders (or Full Disk Access), run `repo service restart`, or keep the checkout outside Documents, Desktop and Downloads. Details: /var/folders/t7/ckfnmc712gsdkt0tc8042br00000gn/T/semio-repo/501/host-6feebb05efe3.log.
```

Resident service first error, verbatim:

```text
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:implementation-source","stage":"running"}
[script] build:implementation-source · Reading the sources of the build implementation
[script] {"owner":"repository:build","command":"build:implementation-source","stage":"running"}
[script] {"owner":"repository:build","command":"build:implementation-source","stage":"running"}
[script] {"owner":"repository:build","command":"build:implementation-source","stage":"running"}
[script] {"owner":"repository:build","command":"build:implementation-source","stage":"running"}
[script] {"owner":"repository:build","command":"build:implementation-source","stage":"running"}
[capacity] admitted graph-plan: 1 job(s), 2048 MB reserved (6140 MB available, 2048 MB floor)
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] build:source-admission · Admitting the sources of the build
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[repo] still working after 5 s; last phase started: source.served
2026-11-14T18:00:29.409Z request-failed e07192d7 Error: Relative filesystem fstatat failed (EPERM)
```

## Observed wall per phase

Approximately five-second log-poll resolution; these are observed intervals, not exact internal phase instrumentation.

| Phase | Observed start elapsed s | Observed wall until next phase/end s |
|---|---:|---:|
| [semio] Building the repo launcher (it provisions the pinned JavaScript runtime itself)… | 0.003 | 5.099 |
| [semio] Installing the native repository command… | 5.102 | 15.267 |
| [semio] Installing the dependency scopes (the planner of every later run needs them)… | 20.369 | 10.086 |
| [semio] workspace:setup… | 30.455 | 5.158 |

## Required native and graph-service lines

Per-try graph-service lines were read from the service logs during the try and retained with observation times in `round-5-try-1-service-events.json`. Required capacity and graph lines, verbatim:

```text
NATIVE
[capacity] run: sized at 8% of the default from 6 earlier run(s) of its class
[capacity] admitted run: 8 job(s), 1709 MB reserved (6921 MB available, 2048 MB floor)
[capacity] run: sized at 8% of the default from 6 earlier run(s) of its class
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
[capacity] dependencies: sized at 28% of the default from 6 earlier run(s) of its class
[capacity] admitted dependencies: 1 job(s), 1733 MB reserved (6913 MB available, 2048 MB floor)
[capacity] dependencies: sized at 28% of the default from 6 earlier run(s) of its class
[capacity] run: sized at 46% of the default from 1 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (6787 MB available, 2048 MB floor)

RESIDENT SERVICE
[capacity] admitted graph-plan: 1 job(s), 2048 MB reserved (6140 MB available, 2048 MB floor)
2026-11-14T18:00:29.409Z request-failed e07192d7 Error: Relative filesystem fstatat failed (EPERM)
```

## Graph service last 30 lines at try completion

host-f74179f31e9f.log (the old host is distinguished from the failing new host):

```text
[repository:command] still running after 900 s
[repository:command] still running after 910 s
[repo] still working after 920 s; last phase started: source.served
[repository:command] still running after 920 s
[repository:command] still running after 930 s
[repository:command] still running after 940 s
[repo] still working after 950 s; last phase started: source.served
[repository:command] still running after 950 s
[repository:command] still running after 960 s
[repository:command] still running after 970 s
[repo] still working after 980 s; last phase started: source.served
[repository:command] still running after 980 s
[repository:command] still running after 990 s
[repository:command] still running after 1000 s
[repo] still working after 1010 s; last phase started: source.served
[repository:command] still running after 1010 s
[repository:command] still running after 1020 s
[repository:command] still running after 1030 s
[repo] still working after 1040 s; last phase started: source.served
[repository:command] still running after 1040 s
[repository:command] still running after 1050 s
[repository:command] still running after 1060 s
[repo] still working after 1070 s; last phase started: source.served
[repository:command] still running after 1070 s
[repository:command] still running after 1080 s
[repository:command] still running after 1090 s
[repo] still working after 1100 s; last phase started: source.served
[repository:command] still running after 1100 s
[repository:command] still running after 1110 s
[repository:command] still running after 1120 s
```

host-6feebb05efe3.log (the old host is distinguished from the failing new host):

```text
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:implementation-source","stage":"running"}
[script] build:implementation-source · Reading the sources of the build implementation
[script] {"owner":"repository:build","command":"build:implementation-source","stage":"running"}
[script] {"owner":"repository:build","command":"build:implementation-source","stage":"running"}
[script] {"owner":"repository:build","command":"build:implementation-source","stage":"running"}
[script] {"owner":"repository:build","command":"build:implementation-source","stage":"running"}
[script] {"owner":"repository:build","command":"build:implementation-source","stage":"running"}
[capacity] admitted graph-plan: 1 job(s), 2048 MB reserved (6140 MB available, 2048 MB floor)
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] build:source-admission · Admitting the sources of the build
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[repo] still working after 5 s; last phase started: source.served
2026-11-14T18:00:29.409Z request-failed e07192d7 Error: Relative filesystem fstatat failed (EPERM)
    at failure (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:148:307)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:153:768)
    at processTicksAndRejections (native:7:39)
```

## Replica status after run

Supported status command exited 0:

```text
Replica g1-m08 of group g1, store /Volumes/semio-store: reachable
Synchronization: on
Daemon: running (pid 75485)
281278 files tracked; queued: 10 incoming, ? outgoing; 26 overwritten files kept in .cache/f/lost
Recurring failures of kind unstable: 2 times, last 1050 s ago.
Recurring failures of kind refused: 123 times, last 6867 s ago.
Sidecar build-agent: running (process 75650, 0 restart(s), 7816 s in this state, last exit -).
Sidecar build-scheduler: off on this machine (process -, 0 restart(s), 7848 s in this state, last exit -).
Sidecar build-cache: off on this machine (process -, 0 restart(s), 7848 s in this state, last exit -).
Journal: 9.22 MiB in 12 segments of 9 origins.
Newest snapshot by g1-server: 22129s old, 89.18 MiB.
Reclaimable now: 2 journal segments (2.29 MiB).
Last compaction 31592s ago: 861 blobs (27.00 GiB) removed; 88 blobs (129.23 MiB) were still too young.
Git state: shared (stage and commits replicate); stage tree 5c75c44d73137807925a4f37dfbb0a95529a546e.
Background tiers, last pass (cost, duty cycle of one core): hot 2 ms 0.7%, directory sweep 569 ms 10%, file sweep 2650 ms 2.7%, tree walk 25256 ms 1.2%, git view 29023 ms 5%.
Latency over the last 100 publishes and 100 applies: publish p50 216376 ms, p95 216542 ms; apply p50 303 ms, p95 1205 ms.
g1-m01: 0 events (0 B) behind, last event 14s ago
g1-m02: 0 events (0 B) behind, last event 15s ago
g1-m03: 0 events (0 B) behind, last event 28s ago
g1-m04: 0 events (0 B) behind, last event 8s ago
g1-m05: 0 events (0 B) behind, last event 24672s ago
g1-m06: 0 events (0 B) behind, last event 141s ago
g1-m07: 0 events (0 B) behind, last event 10s ago
g1-m08: 0 events (0 B) behind, last event 25s ago
g1-server: 0 events (0 B) behind, last event 35s ago
Fleet: origin platform age lag (origins behind/queued) basis (cp:s = compared at its checkpoint taken s seconds ago, now = compared as it is now) sync code view (modified/deleted/untracked) latency (publish p50/p95 ms | apply p50/p95 ms) verdict
  g1-m01 macos 0s 0/0 cp:0s on 39e3688eb315 75/12/241 183/328|302/917 lagging (not settled: no common checkpoint within 60 s yet, still moving)
  g1-m02 macos 1s 0/0 cp:1s on 39e3688eb315 75/12/241 495/1401|321/770 ok
  g1-m03 macos 1s 0/0 cp:1s on 39e3688eb315 75/12/224 198/825|328/1137 lagging (not settled: no common checkpoint within 60 s yet, still moving)
  g1-m04 linux 0s 0/0 cp:0s on 39e3688eb315 75/11/241 176/20727|349/872 diverged
  g1-m05 windows 5s 1/4 cp:- on 73b7f3d88f52 79/12/241 -/-|109440/329736 lagging (not settled: no common checkpoint within 60 s yet, still moving)
  g1-m06 macos 1s 0/0 cp:1s on 39e3688eb315 75/12/231 134/178|277/780 lagging (not settled: no common checkpoint within 60 s yet, still moving)
  g1-m07 macos 0s 0/0 cp:0s on 39e3688eb315 75/12/241 220/8209|325/789 ok
  g1-m08 macos 0s 0/0 cp:0s on 39e3688eb315 75/12/241 216376/216542|303/1205 diverged
  g1-server macos 0s 0/0 cp:0s on 39e3688eb315 75/12/241 612/642|257/733 ok
  warning: g1-m04 runs git 2.55.0, other machines run another version
  warning: the volume of g1-m04 behaves differently from most (case=sensitive normalisation=sensitive); the view is pinned, so this is a capability difference, not a divergence
  warning: g1-m06 runs git 2.50.1, other machines run another version
  g1-m04 differs in view.digest at a common checkpoint: #16=56e3,#52=473b,#86=0997,#114=1e75,#121=b976,#147=9f30,#163=bae7,#188=c73f,#21 instead of #16=7a72,#52=6c0c,#86=a296,#114=dbe6,#121=09af,#147=4d74,#163=6b8d,#188=4b70,#21 (#n = bucket n of 256 of the paths, value = its digest)
  g1-m08 differs in manifest at a common checkpoint: #114=a04c instead of #114=1a9b (#n = bucket n of 256 of the paths, value = its digest)
The fleet is not in agreement (9 origins; fresh and checkpoint age mean within 60 s; lagging is not wrong, only not yet comparable).
```

## Artifacts and next step

`round-5.tail.txt` is the exact last 200 native log lines; all per-try observations, service tail snapshots and EPERM context are retained in this folder. Waiting every 60 seconds for a higher round, up to six hours after report completion.

Report completed 2026-11-14T18:01:57Z.
