# g1-m02 Round 9

round: 9

Result: not ready. Three tries, no nx task ran in any of them (succeeded: none, failed: none, not run: all). Tries 1 and 2 ended with the fleet refusal `native_rc=75`; try 3 was ended by me with one SIGINT when round 10 was published (`native_rc=7`), while it waited for the fleet. No `native_rc=4`: the graph service started and read the worktree in all three tries. `.cache/f/native-bootstrap.done` does not exist.

## Machine

Mac mini `Mac16,10`, Apple M4, 10 cores, 16 GB (17179869184 bytes), macOS 27.2.1 (26B131). Free disk after the round: 105 GiB of 228 GiB.

## Tries

| Try | Log | Start (UTC) | End (UTC) | `native_rc` | Largest process on the machine | Largest in the run's group | Fleet |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `~/native-bootstrap-round-9.log` | 22:02:21 | 22:08:37 | 75 | 3308944 KiB | 471216 KiB | asked for 658, delivered 0 within 0 s (`the fleet cache cannot be reached`) |
| 2 | `~/native-bootstrap-round-9-try-2.log` | 22:10:52 | 22:39:50 | 75 | 3181088 KiB | 371568 KiB | asked for 658, delivered 242 within 1492 s, 416 missing |
| 3 | `~/native-bootstrap-round-9-try-3.log` | 22:41:56 | 23:10:59 | 7 (my SIGINT, round 10 published) | 3510000 KiB | 480128 KiB | 206, then 336 of 658 taken from the fleet; then `0 of 322 projects arrived` until the end |

- Pressure level (`kern.memorystatus_vm_pressure_level`) was 1 in all 130 samples (30 s apart); no hard stop.
- The machine can infer 192 of the stale projects itself (48 per wave) and can give about 3512 MB per project.
- Try 3: the slice worker was restarted once because the code changed under it (`it ended with exit code 75 … attempt 1 of 3`), at about 801 s.

## Projects Named as Too Heavy

- Try 1 (refusal text): `@semio-tech/s-dev-composition` (about 27952 MB), `@semio-tech/repo-lib` (about 9652 MB), `@semio-tech/framework-os-dev` (about 7676 MB), `@semio-tech/framework-os-kernel` (about 7676 MB), `@semio-tech/framework-rs` (about 6518 MB), `workspace` (about 6200 MB).
- Try 2 (service log only; the refusal text of the bootstrap log names none): `@semio-tech/s-dev-composition` (about 27952 MB), `@semio-tech/repo-lib` (about 9652 MB), `@semio-tech/framework-os-dev` (about 8452 MB), `workspace` (about 6200 MB), `@semio-tech/framework-plugin` (about 4018 MB).
- Try 3 (service log): `@semio-tech/repo-lib` (about 27770 MB), `workspace` (about 13780 MB), `@semio-tech/framework-os-dev` (about 8452 MB), `@semio-tech/framework-os` (about 8368 MB), `@semio-tech/framework-os-kernel` (about 6984 MB), `@semio-tech/framework-plugin` (about 5392 MB).

The names and the sizes differ from try to try (for example `@semio-tech/repo-lib` 9652 MB, then 27770 MB).

## Access

- No line with "runs under the app's access" (searched for `access`, `EPERM`, `where: root`) in the three bootstrap logs or in the service log parts of the tries. I cannot say from the logs whether the service ran under the launcher's own consent or as a direct child; whether the owner clicked "Allow" was not measured.
- No `fstatat failed (EPERM)` in this round.

## Launcher

Unchanged before and after every try and between the tries:

```
readlink launcher: versions/cf25f2b751f83336/repo
launcher target inode 3679486 mtime 2026-11-14T22:03:22 (local time) size 6523360
Identifier=tech.semio.repo
CDHash=6de7547a8b8ff384bbfbfc45e962462063ff0357
```

Note: the mtime is 2026-11-14T21:03:22Z, the install of round 8 try 1.

## Before the Run (2026-11-14T22:02:17Z)

- Replica agent:  state = running runs = 20 state = active state = active ; started by me 0 times in this round. `.cache/f/health.json` age 1 s.
- Replica code digest: `590fdef8b75853d124d6f615cd2612d607146e27aaf2b94715a5d7b7606ad2f1`
- `bootVersion`: 6
- `repo service host` processes: 

## Try 1

- Log `/Users/ueli/native-bootstrap-round-9.log`: `start 2026-11-14T22:02:21Z`, `end 2026-11-14T22:08:37Z`, `native_rc=75`
- Runner: `native_rc=75 peak_rss_kib=3308944 run_peak_rss_kib=471216 stop=None fleet_refusal=True nx_task_lines=0 error_lines=0 moving=False`
- nx task lines: 0

`[capacity]` lines of the bootstrap log:
```
[capacity] run: sized at 4% of the default from 16 earlier run(s) of its class, at least 2362 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 2978 MB reserved (6721 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 6 earlier run(s) of its class
[capacity] dependencies: sized at 31% of the default from 16 earlier run(s) of its class, at least 2600 MB because of its assembly phase
[capacity] admitted dependencies: 1 job(s), 2600 MB reserved (6723 MB available, 2048 MB floor)
```

Last distinct lines of the bootstrap log:
```
Planning 1 targets.
Reading the project graph slice (2 s).
Reading the project graph slice (22 s): directories 1083 of 1083.
Reading the project graph slice (24 s): files 0 of 0.
Reading the project graph slice (28 s): doubt 467772 of 467772.
Reading the project graph slice (30 s): facts 53248 of 238288.
Reading the project graph slice (56 s): verification 1 of 1081.
Reading the project graph slice (124 s): rederivation 7 of 7.
Reading the project graph slice (126 s): flush 1 of 1.
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 658 and delivered 0 within 0 s, 658 are still missing, among them the too heavy project(s) @semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (
end 2026-11-14T22:08:37Z
native_rc=75
```

Service log, wanted lines (0 `arrived` lines; first `-`, last `-`):
```
graph: 658 stale projects, 0 taken from the fleet, 6 too heavy for this machine (@semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); ask
graph: nothing to wait for any longer: the fleet was asked for 658 stale projects and delivered 0 within 0 s; 658 remain, among them the too heavy project(s) @semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB), which this m
2026-11-14T22:08:37.150Z graph-wanted-expired {"wanted":"658","arrived":"0","seconds":"0","missing":"658","local":"192","heavy":", among them the too heavy project(s) @semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB) (thi
```

Service log, last 30 lines of the try (cut at 300 characters):
```
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:1927","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:1927:1927","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:12","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:12:12","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1025:1081","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1025 of 1081 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1048:1081","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1048 of 1081 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1060:1081","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1060 of 1081 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1081:1081","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1081 of 1081 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:1865","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:1865:1865","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:4","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:4:4","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
graph: 658 stale projects, 0 taken from the fleet, 6 too heavy for this machine (@semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 M
graph: the fleet cache cannot be reached, so 658 stale projects cannot be requested from it; this machine can infer 192
graph: nothing to wait for any longer: the fleet was asked for 658 stale projects and delivered 0 within 0 s; 658 remain, among them the too heavy project(s) @semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tec
2026-11-14T22:08:37.150Z graph-wanted-expired {"wanted":"658","arrived":"0","seconds":"0","missing":"658","local":"192","heavy":", among them the too heavy project(s) @semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @
[repository:command] still running after 180 s
[repository:command] still running after 190 s
```

## Try 2

- Log `/Users/ueli/native-bootstrap-round-9-try-2.log`: `start 2026-11-14T22:10:52Z`, `end 2026-11-14T22:39:50Z`, `native_rc=75`
- Runner: `native_rc=75 peak_rss_kib=3181088 run_peak_rss_kib=371568 stop=None fleet_refusal=True nx_task_lines=0 error_lines=0 moving=False`
- nx task lines: 0

`[capacity]` lines of the bootstrap log:
```
[capacity] run: sized at 4% of the default from 16 earlier run(s) of its class, at least 2364 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 2980 MB reserved (7677 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 6 earlier run(s) of its class
[capacity] dependencies: sized at 33% of the default from 16 earlier run(s) of its class, at least 2605 MB because of its assembly phase
[capacity] admitted dependencies: 1 job(s), 2605 MB reserved (6162 MB available, 2048 MB floor)
```

Last distinct lines of the bootstrap log:
```
Reading the project graph slice (807 s): flush 1 of 1.
[repo] still working after 830 s; last phase started: door.script
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 658 and delivered 242 within 1492 s, 416 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
end 2026-11-14T22:39:50Z
native_rc=75
```

Service log, wanted lines (148 `arrived` lines; first `graph: waiting for the fleet: 141 of 658 projects arrived`, last `graph: waiting for the fleet: 242 of 658 projects arrived`):
```
graph: 658 stale projects, 0 taken from the fleet, 5 too heavy for this machine (@semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 8452 MB), workspace (about 6200 MB), @semio-tech/framework-plugin (about 4018 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 658
graph: waiting for the fleet: 141 of 658 projects arrived
graph: nothing to wait for any longer: asked the fleet for 658 stale projects and received 242 within 1492 s, 416 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T22:39:50.587Z graph-wanted-expired {"wanted":"658","arrived":"242","seconds":"1492","missing":"416","local":"192","heavy":""}
```

Service log, last 30 lines of the try (cut at 300 characters):
```
[repository:command] still running after 1930 s
[repo] still working after 1940 s; last phase started: source.served
graph: waiting for the fleet: 242 of 658 projects arrived
[repository:command] still running after 1940 s
graph: waiting for the fleet: 242 of 658 projects arrived
[repository:command] still running after 1950 s
graph: waiting for the fleet: 242 of 658 projects arrived
[repository:command] still running after 1960 s
[repo] still working after 1970 s; last phase started: source.served
graph: waiting for the fleet: 242 of 658 projects arrived
[repository:command] still running after 1970 s
graph: waiting for the fleet: 242 of 658 projects arrived
[repository:command] still running after 1980 s
graph: waiting for the fleet: 242 of 658 projects arrived
[repository:command] still running after 1990 s
[repo] still working after 2000 s; last phase started: source.served
graph: waiting for the fleet: 242 of 658 projects arrived
[repository:command] still running after 2000 s
[repository:command] still running after 2010 s
graph: waiting for the fleet: 242 of 658 projects arrived
[repository:command] still running after 2020 s
graph: waiting for the fleet: 242 of 658 projects arrived
[repo] still working after 2030 s; last phase started: source.served
[repository:command] still running after 2030 s
graph: waiting for the fleet: 242 of 658 projects arrived
[repository:command] still running after 2040 s
graph: waiting for the fleet: 242 of 658 projects arrived
graph: nothing to wait for any longer: asked the fleet for 658 stale projects and received 242 within 1492 s, 416 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T22:39:50.587Z graph-wanted-expired {"wanted":"658","arrived":"242","seconds":"1492","missing":"416","local":"192","heavy":""}
[repository:command] still running after 2050 s
```

## Try 3

- Log `/Users/ueli/native-bootstrap-round-9-try-3.log`: `start 2026-11-14T22:41:56Z`, `end 2026-11-14T23:10:59Z`, `native_rc=7`
- Runner: `native_rc=7 peak_rss_kib=3510000 run_peak_rss_kib=480128 stop=round 10 was published and no task has started fleet_refusal=False nx_task_lines=0 error_lines=0 moving=False`
- nx task lines: 0

`[capacity]` lines of the bootstrap log:
```
[capacity] run: sized at 4% of the default from 16 earlier run(s) of its class, at least 2370 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 2986 MB reserved (7483 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 6 earlier run(s) of its class
[capacity] dependencies: sized at 33% of the default from 16 earlier run(s) of its class, at least 2623 MB because of its assembly phase
[capacity] admitted dependencies: 1 job(s), 2623 MB reserved (5902 MB available, 2048 MB floor)
```

Last distinct lines of the bootstrap log:
```
[repo] still working after 800 s; last phase started: door.script
Reading the project graph slice (801 s): flush 1 of 1.
Reading the project graph slice (0 s): the code on this machine changed under the slice worker (it ended with exit code 75), so it is started again with the current code, attempt 1 of 3.
Reading the project graph slice (2 s).
Reading the project graph slice (10 s): files 0 of 0.
Reading the project graph slice (16 s): doubt 467772 of 467772.
Reading the project graph slice (18 s): facts 32768 of 238204.
Reading the project graph slice (34 s): verification 2 of 1081.
Reading the project graph slice (40 s): parts 7305 of 14343.
Reading the project graph slice (50 s): rederivation 256 of 256.
end 2026-11-14T23:10:59Z
native_rc=7
```

Service log, wanted lines (138 `arrived` lines; first `graph: waiting for the fleet: 0 of 452 projects arrived`, last `graph: waiting for the fleet: 0 of 322 projects arrived`):
```
graph: 658 stale projects, 206 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 452
graph: waiting for the fleet: 0 of 452 projects arrived
graph: 658 stale projects, 336 taken from the fleet, 6 too heavy for this machine (@semio-tech/repo-lib (about 27770 MB), workspace (about 13780 MB), @semio-tech/framework-os-dev (about 8452 MB), @semio-tech/framework-os (about 8368 MB), @semio-tech/framework-os-kernel (about 6984 MB), @semio-tech/framework-plugin (about 5392 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); a
```

Service log, last 30 lines of the try (cut at 300 characters):
```
graph: waiting for the fleet: 0 of 322 projects arrived
[repository:command] still running after 3821 s
[repo] still working after 3830 s; last phase started: source.served
graph: waiting for the fleet: 0 of 322 projects arrived
[repository:command] still running after 3831 s
graph: waiting for the fleet: 0 of 322 projects arrived
[repository:command] still running after 3841 s
graph: waiting for the fleet: 0 of 322 projects arrived
[repository:command] still running after 3851 s
[repo] still working after 3860 s; last phase started: source.served
graph: waiting for the fleet: 0 of 322 projects arrived
[repository:command] still running after 3861 s
graph: waiting for the fleet: 0 of 322 projects arrived
[repository:command] still running after 3871 s
graph: waiting for the fleet: 0 of 322 projects arrived
[repository:command] still running after 3881 s
[repo] still working after 3890 s; last phase started: source.served
graph: waiting for the fleet: 0 of 322 projects arrived
[repository:command] still running after 3891 s
graph: waiting for the fleet: 0 of 322 projects arrived
[repository:command] still running after 3901 s
graph: waiting for the fleet: 0 of 322 projects arrived
[repository:command] still running after 3911 s
[repo] still working after 3920 s; last phase started: source.served
SIGINT received: the running work is cancelled and its leases are released; this process exits after 10 s at the latest.
Cancelled (SIGINT); leases released.
[repository:command] still running after 3921 s
[repository:command] still running after 3931 s
[repository:command] still running after 3941 s
[repo] still working after 3950 s; last phase started: source.served
```

