# Round 2 — g1-m08 — Complete

**Not ready.** Native `workspace:setup` stopped at its graph read. The client reported an unexpected resident-service failure, reference `20faee18`; the resident log identifies `timeout graph-wanted.expired` (703 wanted, 0 arrived, 703 missing, 902 seconds waited, local capacity 192). No resource hard stop fired.

## Required pre-run observations

Observed 2026-11-14T16:06:39Z; written before bootstrap launch. Replica digest from `.cache/f/health.json`: `39e3688eb315d48831772ca305d78bcb6f35975794444da47750bc5fa7459759`. No `repo service host run` process was found, so none older than one hour existed and no host-stop command was used.

Last five build-agent lines, verbatim:
```text
2026-11-14T15:49:38.803Z worker registered: 055c7530-d460-45bc-b264-50a3e2a29421 cache http://192.168.178.70:6094
2026-11-14T15:50:10.202Z worker stopped: ueli-workstation-mac-mini-m4-16gb-256gb-05-wlan.fritz.box-75290
2026-11-14T15:50:46.077Z joining http://192.168.178.70:8981 offering rustc for 3 target(s)
2026-11-14T15:50:46.130Z worker sandbox: darwin-sandbox confined network denied
2026-11-14T15:50:46.151Z worker registered: 56f28eba-ace2-4efd-a724-5fb9acd7acee cache http://192.168.178.70:6094
```

## Machine and result

| Measurement | Value |
|---|---|
| Origin | g1-m08 |
| Memory | 17179869184 bytes (16 GiB) |
| OS | macOS 27.2, build 26B123; arm64 Apple M4 |
| Free disk before | 117676144 KiB |
| Free disk after | See `round-2-machine-after.json`, measured after completion |
| Start UTC | 2026-11-14T16:10:01Z |
| Native logged end UTC | 2026-11-14T16:30:05Z |
| Controller observed end UTC | 2026-11-14T16:30:09Z |
| Controller wall seconds | 1208.666 |
| native_rc | 1 (anchored log line; wrapper exit was 0) |
| Completion marker | Absent |
| ready | false |
| Peak run-owned process RSS | 3678016 KiB (3.508 GiB), checked about every 5 s |
| Peak machine-wide RSS in required top-three samples | 3678016 KiB (3.508 GiB) |
| Required 30-second samples | 40, starting at launch; exact command output in `round-2-try-1-ps.json` |
| Memory pressure | Every observed sample normal (1): True; 240 checks |
| Signals to native run | None |

Run-owned processes were identified by the launched process group and descendants. RSS top-three samples used `ps -axo rss,command | sort -nr | head -3` every approximately 30 seconds. Critical pressure was checked about every 5 seconds; consecutive warning counts used the 30-second samples. No warning streak, oversized run process, or unserved inference hard-stop condition was observed.

## Attempt and retry decision

| Try | Wall seconds | Fleet wait seconds | Wanted | Arrived | Missing | Retry sleep seconds |
|---|---:|---:|---:|---:|---:|---:|
| 1 | 1208.666 | 902 (resident log) | 703 | 0 | 703 | 0 |

No retry: the bootstrap log contains neither `graph-wanted`, the missing-project fleet-refusal text, nor an exit-75 line. Instead it masks the underlying fleet refusal as an unexpected resident-service failure and exits with native_rc=1. The resident log contains the expected wanted/missing refusal but no emitted exit status 75. The source declaration describes the refusal as exit 75; this is not an observed exit code. This distinction is reported for the coordinator, without changing code or converting an unrelated client failure into an eligible retry.

## Phase timing

Times below are observations from log polling at approximately 5-second intervals, not exact internal instrumentation; multiple transitions between polls have no individually measured duration.

| Phase | Observed elapsed interval (seconds) | Observed wall (seconds) |
|---|---|---:|
| Initial bootstrap / launcher start | 0.001–5.079 | 5.078 |
| Launcher installation / Rust toolchain / native command installation | 5.079–15.215 | 10.136 |
| Dependency scopes | 15.215–222.978 | 207.763 |
| workspace:setup through observed controller completion | 222.978–1208.666 | 985.688 |
| Graph-slice read | Internal heartbeat reached 981 s before client failure | Exact full phase wall not measured |
| Fleet wait (within graph phase) | Resident refusal fields | 902 |

## Failed phase and first error with 15 preceding lines

Failed bootstrap phase: `workspace:setup`. No downstream Nx task failure list was emitted; downstream task names and errors are not measured because graph loading did not finish.

Client log, verbatim:
```text
Reading the project graph slice (955 s): flush 1 of 1.
Reading the project graph slice (957 s): flush 1 of 1.
Reading the project graph slice (959 s): flush 1 of 1.
Reading the project graph slice (961 s): flush 1 of 1.
Reading the project graph slice (963 s): flush 1 of 1.
Reading the project graph slice (965 s): flush 1 of 1.
Reading the project graph slice (967 s): flush 1 of 1.
Reading the project graph slice (969 s): flush 1 of 1.
Reading the project graph slice (971 s): flush 1 of 1.
Reading the project graph slice (973 s): flush 1 of 1.
[repo] still working after 980 s; last phase started: door.script
Reading the project graph slice (975 s): flush 1 of 1.
Reading the project graph slice (977 s): flush 1 of 1.
Reading the project graph slice (979 s): flush 1 of 1.
Reading the project graph slice (981 s): flush 1 of 1.
The resident service met an unexpected failure (reference 20faee18); try again, the details under that reference are in the service log /var/folders/t7/ckfnmc712gsdkt0tc8042br00000gn/T/semio-repo/501/host-38cdf415c7fb.log.
```

Resident service first error and 15 preceding lines, verbatim:
```text
[repository:command] still running after 930 s
graph: waiting for the fleet: 0 of 703 projects arrived
[repository:command] still running after 940 s
graph: waiting for the fleet: 0 of 703 projects arrived
[repo] still working after 950 s; last phase started: source.served
[repository:command] still running after 950 s
graph: waiting for the fleet: 0 of 703 projects arrived
[repository:command] still running after 960 s
graph: waiting for the fleet: 0 of 703 projects arrived
[repository:command] still running after 970 s
graph: waiting for the fleet: 0 of 703 projects arrived
[repo] still working after 980 s; last phase started: source.served
[repository:command] still running after 980 s
graph: waiting for the fleet: 0 of 703 projects arrived
graph: nothing to wait for any longer: asked the fleet for 703 stale projects and received 0 within 902 s, 703 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T16:30:05.934Z request-failed 20faee18 RepoError: timeout graph-wanted.expired {"wanted":703,"arrived":0,"seconds":902,"missing":703,"local":192}
```

## Capacity, graph, fleet, wanted, restored and restart lines

All matching lines from both the native log and the referenced resident log are included verbatim below. Native help text containing fleet is retained as emitted.
```text
NATIVE LOG
[capacity] run: sized at 36% of the default from 1 earlier run(s) of its class
[capacity] admitted run with 6 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 36% of the default from 1 earlier run(s) of its class
[capacity] run: sized at 59% of the default from 1 earlier run(s) of its class
[capacity] run: sized at 59% of the default from 1 earlier run(s) of its class
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
[capacity] dependencies: sized at 59% of the default from 1 earlier run(s) of its class
[capacity] admitted dependencies: 1 job(s), 3644 MB reserved (6630 MB available, 2048 MB floor)
[capacity] dependencies: sized at 59% of the default from 1 earlier run(s) of its class
[capacity] run: sized at 46% of the default from 1 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7329 MB available, 2048 MB floor)

RESIDENT SERVICE LOG (through failure 20faee18)
[capacity] admitted graph-plan: 1 job(s), 2048 MB reserved (6449 MB available, 2048 MB floor)
graph: No verified change journal cursor is recorded for generation 0; verifying the tree once.
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:0:126773","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 0 of 126773 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:39565:126773","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 39565 of 126773 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:95930:126773","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 95930 of 126773 entries
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
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:40960:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 40960 of 441651 facts
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
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:5987","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 5987 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:5987:5987","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 5987 of 5987 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:114688:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 114688 of 441651 facts
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
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:245760:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 245760 of 441651 facts
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
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:335872:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 335872 of 441651 facts
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
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:26","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 26 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:26:26","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 26 of 26 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:369","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 369 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:369:369","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 369 of 369 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:417792:441651","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 417792 of 441651 facts
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
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:0:43855","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 0 of 43855 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:4096:43855","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 4096 of 43855 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:8192:43855","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 8192 of 43855 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:24576:43855","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 24576 of 43855 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:28672:43855","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 28672 of 43855 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:32768:43855","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 32768 of 43855 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:43855:43855","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 43855 of 43855 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:0:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 0 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:4:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 4 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:6:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 6 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:7:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 7 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:8:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 8 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:10:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 10 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:11:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 11 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:14:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 14 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:15:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 15 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:17:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 17 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:19:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 19 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:20:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 20 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:22:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 22 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:23:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 23 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:25:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 25 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:38:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 38 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:47:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 47 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:65:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 65 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:72:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 72 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:76:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 76 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:78:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 78 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:79:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 79 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:80:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 80 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:82:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 82 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:102:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 102 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:119:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 119 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:124:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 124 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:130:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 130 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:135:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 135 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:138:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 138 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:146:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 146 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:167:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 167 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:192:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 192 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:218:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 218 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:244:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 244 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:257:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 257 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:292:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 292 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:297:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 297 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:303:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 303 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:312:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 312 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:339:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 339 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:373:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 373 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:403:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 403 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:408:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 408 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:421:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 421 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:444:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 444 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:475:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 475 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:488:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 488 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:497:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 497 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:504:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 504 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:513:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 513 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:537:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 537 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:549:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 549 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:553:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 553 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:556:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 556 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:559:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 559 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:563:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 563 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:569:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 569 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:581:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 581 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:605:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 605 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:636:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 636 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:661:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 661 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:694:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 694 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:729:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 729 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:768:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 768 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:769:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 769 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:775:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 775 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:790:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 790 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:820:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 820 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:866:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 866 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:889:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 889 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:893:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 893 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:894:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 894 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:895:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 895 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:896:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 896 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:897:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 897 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:926:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 926 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:966:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 966 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1014:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1014 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1025:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1025 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1078:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1078 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1134:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1134 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1177:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1177 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1212:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1212 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1246:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1246 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1269:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1269 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1278:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1278 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1281:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1281 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1324:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1324 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1362:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1362 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1402:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1402 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1414:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1414 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1418:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1418 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1422:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1422 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1427:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1427 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1446:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1446 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1488:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1488 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1526:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1526 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1537:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1537 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1590:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1590 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1619:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1619 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1626:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1626 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1630:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1630 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1661:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1661 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1690:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1690 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1694:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1694 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1696:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1696 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1699:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1699 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1702:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1702 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1728:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1728 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1764:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1764 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1788:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1788 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1793:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1793 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1863:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1863 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1918:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1918 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1966:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1966 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2017:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2017 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2049:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2049 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2113:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2113 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2160:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2160 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2215:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2215 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2269:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2269 of 2277 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2277:2277","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2277 of 2277 closures
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
graph: nothing to wait for any longer: asked the fleet for 703 stale projects and received 0 within 902 s, 703 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T16:30:05.934Z request-failed 20faee18 RepoError: timeout graph-wanted.expired {"wanted":703,"arrived":0,"seconds":902,"missing":703,"local":192}
    at refuse (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🙋️wanted/📖️read/🟦️.ts:20:232)
    at fleetReadV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🙋️wanted/📖️read/🟦️.ts:29:39)
    at async fleetReadStaleGenerationV1 (unknown)
```

## Replica status after run

Supported command: `SEMIO_WORKSPACE_ROOT=/Users/ueli/Documents/semio repo ./📜️script.ts status --lang en`, from the replica module. Exit 0. Daemon running, synchronization on, g1-m08 code prefix `39e3688eb315`; the fleet view reports g1-m08 manifest divergence. No repair was attempted.
```text
Replica g1-m08 of group g1, store /Volumes/semio-store: reachable
Synchronization: on
Daemon: running (pid 75485)
281099 files tracked; queued: 10 incoming, ? outgoing; 26 overwritten files kept in .cache/f/lost
Recurring failures of kind refused: 123 times, last 1500 s ago.
Recurring failures of kind unstable: 1 times, last 2365 s ago.
Sidecar build-agent: running (process 75650, 0 restart(s), 2450 s in this state, last exit -).
Sidecar build-scheduler: off on this machine (process -, 0 restart(s), 2481 s in this state, last exit -).
Sidecar build-cache: off on this machine (process -, 0 restart(s), 2481 s in this state, last exit -).
Journal: 8.36 MiB in 12 segments of 9 origins.
Newest snapshot by g1-server: 16763s old, 89.18 MiB.
Reclaimable now: 2 journal segments (2.29 MiB).
Last compaction 26226s ago: 861 blobs (27.00 GiB) removed; 88 blobs (129.23 MiB) were still too young.
Git state: shared (stage and commits replicate); stage tree 5c75c44d73137807925a4f37dfbb0a95529a546e.
Background tiers, last pass (cost, duty cycle of one core): hot 9 ms 2.8%, directory sweep 1237 ms 10%, file sweep 2904 ms 2.7%, tree walk 23907 ms 1.4%, git view 25271 ms 5%.
Latency over the last 100 publishes and 100 applies: publish p50 281939 ms, p95 281961 ms; apply p50 360 ms, p95 920 ms.
g1-m01: 0 events (0 B) behind, last event 26s ago
g1-m02: 0 events (0 B) behind, last event 18s ago
g1-m03: 0 events (0 B) behind, last event 19s ago
g1-m04: 0 events (0 B) behind, last event 1277s ago
g1-m05: 0 events (0 B) behind, last event 19307s ago
g1-m06: 0 events (0 B) behind, last event 683s ago
g1-m07: 0 events (0 B) behind, last event 13s ago
g1-m08: 0 events (0 B) behind, last event 82s ago
g1-server: 0 events (0 B) behind, last event 79s ago
Fleet: origin platform age lag (origins behind/queued) basis (cp:s = compared at its checkpoint taken s seconds ago, now = compared as it is now) sync code view (modified/deleted/untracked) latency (publish p50/p95 ms | apply p50/p95 ms) verdict
  g1-m01 macos 0s 0/0 cp:0s on 39e3688eb315 40/12/62 238/28778|224/2973 ok
  g1-m02 macos 0s 0/0 cp:0s on 39e3688eb315 23/12/54 396/2207|206/1242 lagging (not settled: no common checkpoint within 60 s yet, still moving)
  g1-m03 macos 1s 0/0 cp:1s on 39e3688eb315 40/12/62 412/7261|237/826 ok
  g1-m04 linux 2s 0/0 cp:2s on 39e3688eb315 40/11/62 -/-|261/9301 diverged
  g1-m05 windows 4s 0/4 cp:- on 73b7f3d88f52 16/12/41 -/-|8421/213617 lagging (not settled: no common checkpoint within 60 s yet, still moving)
  g1-m06 macos 2s 0/0 cp:2s on 39e3688eb315 40/12/62 -/-|193/755 ok
  g1-m07 macos 4s 0/0 cp:4s on 39e3688eb315 23/12/54 178/122984|192/866 lagging (not settled: no common checkpoint within 60 s yet, still moving)
  g1-m08 macos 1s 0/0 cp:1s on 39e3688eb315 40/12/62 281939/281961|360/920 diverged
  g1-server macos 0s 0/0 cp:0s on 39e3688eb315 40/12/62 62/388|1237/14474 ok
  warning: g1-m04 runs git 2.55.0, other machines run another version
  warning: the volume of g1-m04 behaves differently from most (case=sensitive normalisation=sensitive); the view is pinned, so this is a capability difference, not a divergence
  warning: g1-m06 runs git 2.50.1, other machines run another version
  g1-m04 differs in view.digest at a common checkpoint: #16=56e3,#52=473b,#86=1047,#114=1e75,#121=b976,#147=9f30,#163=6ac3,#188=a20d,#21 instead of #16=7a72,#52=6c0c,#86=bb46,#114=dbe6,#121=09af,#147=4d74,#163=bba9,#188=2e42,#21 (#n = bucket n of 256 of the paths, value = its digest)
  g1-m08 differs in manifest at a common checkpoint: #114=5937 instead of #114=e3e0 (#n = bucket n of 256 of the paths, value = its digest)
The fleet is not in agreement (9 origins; fresh and checkpoint age mean within 60 s; lagging is not wrong, only not yet comparable).
```

## Observer deviation

An attempted status query used the installed service-entry shim `boot.sh status`, which does not support status. It returned unknown-command errors and retried; its shim wrote its own `shim.txt` outside the worker folder. I terminated only that query process (PID 95468, SIGTERM), confirmed it exited, then used the supported replica status command above. No source changes or cleanup outside this worker folder were made. This was an observer mistake, not a native-bootstrap failure.

## Artifacts and next step

Exact last 200 native lines: `round-2.tail.txt`. Per-attempt result, pressure, RSS, observed events and fleet lines: `round-2-try-1-*`. Required pre-run observations: `round-2-preflight-observation.json`. No second bootstrap was started. Waiting for a higher round under PROTOCOL step 1, up to six hours after this report.

Report completed: 2026-11-14T16:33:00Z.
