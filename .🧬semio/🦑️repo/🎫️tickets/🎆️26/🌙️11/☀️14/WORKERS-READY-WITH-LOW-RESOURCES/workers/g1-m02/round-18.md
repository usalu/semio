# g1-m02 Round 18

round: 18

Result: not ready. Try 1 ended with the fleet refusal (`native_rc=75`) after 11 min 41 s; no `native_rc=4` this time. Try 2 (started 04:05:36Z) was cut off at about 04:13Z when my session was restarted: the bootstrap and its runner died with it, so the log has a `start` line and no `end` line and there is no `native_rc` (not measured). It was in "Reading the project graph slice (448 s): flush 1 of 1" and the service log said `waiting for the fleet: 0 of 657 projects arrived`. No nx task ran in either try (succeeded: none, failed: none, not run: all). `.cache/f/native-bootstrap.done` does not exist. Round 19 was published before try 2 ended; I did not start a third try of round 18.

The graph service of try 2 (pid 37286, started about 04:09:38Z) was still running at 04:18Z; I did not stop it.

## Before the Run (2026-11-15T03:50:52Z)

- Replica agent: state = running runs = 26 ; started by me 0 times in this round.
- Replica code digest (`.cache/f/health.json`): `97445096cc0a4394ffbc45b5ae588ae86aea987d1329f6f4aa4f7779616437ea`
- `bootVersion`: 6
- Step 0 (`repo tools setup all`) was not run again, as round 18 says for this machine.

## Try 1

- Log `/Users/ueli/native-bootstrap-round-18.log`: `start 2026-11-15T03:50:55Z`, `end 2026-11-15T04:02:28Z`, `native_rc=75`
- Runner: `native_rc=75 peak_rss_kib=4075680 run_peak_rss_kib=370192 stop=None fleet_refusal=True closure_missing=False nx_task_lines=0 error_lines=0 moving=False`
- P12 sampler peak: `2026-11-15T03:59:24Z pid=25203 rss=4211MB footprint=[bun [25203]: 64-bit    Footprint: 3054 MB (16384 bytes per page)] last=[graph: plan answered, managed 105 MB, resident 378 MB (released 0 MB)]`

Key lines of the graph service log, verbatim, in order:

```
graph: generation 221 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 9 s, from a state 9 min old with 0 projects stale
graph: refreshing 270 projects (generation 222)
graph: plan stage inference starts (managed 1487 MB, resident 3631 MB)
graph: plan stage inference took 258.9 s (managed 2749 MB, 1262 MB more than at its start, resident 3513 MB)
graph: plan stage environment starts (managed 2749 MB, resident 3513 MB)
graph: plan stage environment took 7.8 s (managed 2967 MB, 218 MB more than at its start, resident 3401 MB)
graph: plan stage tests starts (managed 2967 MB, resident 3401 MB)
graph: plan stage tests took 0 s (managed 2967 MB, 0 MB more than at its start, resident 3401 MB)
graph: plan stage policies starts (managed 2967 MB, resident 3401 MB)
graph: plan stage policies took 0 s (managed 2967 MB, 0 MB more than at its start, resident 3402 MB)
graph: plan stage dangling starts (managed 2967 MB, resident 3402 MB)
graph: plan stage dangling took 0 s (managed 2967 MB, 0 MB more than at its start, resident 3402 MB)
graph: plan stage files starts (managed 2967 MB, resident 3402 MB)
graph: plan stage files took 0 s (managed 2967 MB, 0 MB more than at its start, resident 3403 MB)
graph: plan stage test-edges starts (managed 2967 MB, resident 3403 MB)
graph: plan stage test-edges took 0 s (managed 2967 MB, 0 MB more than at its start, resident 3403 MB)
graph: plan stage edges starts (managed 2967 MB, resident 3403 MB)
graph: plan stage edges took 0.9 s (managed 2812 MB, -155 MB more than at its start, resident 3412 MB)
graph: plan stage metadata starts (managed 2812 MB, resident 3412 MB)
graph: plan stage metadata took 0 s (managed 2812 MB, 0 MB more than at its start, resident 3413 MB)
graph: plan stage result starts (managed 2812 MB, resident 3413 MB)
graph: plan stage result took 0 s (managed 2812 MB, 0 MB more than at its start, resident 3413 MB)
graph: plan stage inference starts (managed 2963 MB, resident 4212 MB)
graph: plan stage inference took 158.2 s (managed 3518 MB, 555 MB more than at its start, resident 3603 MB)
graph: plan stage environment starts (managed 3518 MB, resident 3603 MB)
graph: plan stage environment took 14.7 s (managed 3243 MB, -275 MB more than at its start, resident 3668 MB)
graph: plan stage tests starts (managed 3243 MB, resident 3668 MB)
graph: plan stage tests took 0 s (managed 3249 MB, 6 MB more than at its start, resident 3668 MB)
graph: plan stage policies starts (managed 3249 MB, resident 3668 MB)
graph: plan stage policies took 1 s (managed 3539 MB, 290 MB more than at its start, resident 3667 MB)
graph: plan stage dangling starts (managed 3539 MB, resident 3667 MB)
graph: plan stage dangling took 0 s (managed 3539 MB, 0 MB more than at its start, resident 3668 MB)
graph: plan stage files starts (managed 3539 MB, resident 3668 MB)
graph: plan stage files took 0 s (managed 3539 MB, 0 MB more than at its start, resident 3671 MB)
graph: plan stage test-edges starts (managed 3539 MB, resident 3671 MB)
graph: plan stage test-edges took 0 s (managed 3539 MB, 0 MB more than at its start, resident 3671 MB)
graph: plan stage edges starts (managed 3539 MB, resident 3671 MB)
graph: plan stage edges took 1.3 s (managed 3569 MB, 30 MB more than at its start, resident 3697 MB)
graph: plan stage metadata starts (managed 3569 MB, resident 3697 MB)
graph: plan stage metadata took 0 s (managed 3569 MB, 0 MB more than at its start, resident 3700 MB)
graph: plan stage result starts (managed 3569 MB, resident 3700 MB)
graph: plan stage result took 0 s (managed 3569 MB, 0 MB more than at its start, resident 3703 MB)
2026-11-15T04:02:28.104Z graph-plan-too-heavy {"stage":"result","operation":"result","megabytes":6581,"limit":6553,"worker":{"heapMb":2090.3,"extraMb":1236,"objectCount":11238674,"objectTypes":[{"type":"string","count":6833841},{"type":"Object","count":1334751},{"type":"Array","count":1181024},{"type":"WeakRef","count":201788},{"type":"Set","count":169625},{"type":"Structure","count":46232},{"type":"Cell Butterfly","count":34140},{"type":"Function","count":26067}],"at":1794715298141}}
```

`graph-plan-too-heavy` with the 20 lines before it:

```
graph: plan stage dangling starts (managed 3539 MB, resident 3667 MB)
graph: plan stage dangling took 0 s (managed 3539 MB, 0 MB more than at its start, resident 3668 MB)
graph: plan stage files starts (managed 3539 MB, resident 3668 MB)
graph: plan stage files took 0 s (managed 3539 MB, 0 MB more than at its start, resident 3671 MB)
graph: plan stage test-edges starts (managed 3539 MB, resident 3671 MB)
graph: plan stage test-edges took 0 s (managed 3539 MB, 0 MB more than at its start, resident 3671 MB)
graph: plan stage edges starts (managed 3539 MB, resident 3671 MB)
[script] {"owner":"repository:build","command":"build:generator-producers","stage":"running"}
[script] build:generator-producers · Collecting generator producers
[repo] still working after 650 s; last phase started: source.served
[repository:command] still running after 650 s
[script] {"owner":"repository:build","command":"build:modules","stage":"running"}
graph: plan stage edges took 1.3 s (managed 3569 MB, 30 MB more than at its start, resident 3697 MB)
graph: plan stage metadata starts (managed 3569 MB, resident 3697 MB)
[script] {"owner":"repository:build","command":"build:graph","stage":"complete"}
graph: plan stage metadata took 0 s (managed 3569 MB, 0 MB more than at its start, resident 3700 MB)
graph: plan stage result starts (managed 3569 MB, resident 3700 MB)
graph: plan stage result took 0 s (managed 3569 MB, 0 MB more than at its start, resident 3703 MB)
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:96:270","mode":"shared","elapsedMs":0}}
graph: after the wave managed 3450 MB, resident 3707 MB; largest bounded caches: inference:closure-entries 661 entries 16 MB, inference:closure-recipes 1726 entries 1 MB, inference:closure-values 64 entries 1 MB, inference:closure-inputs 51 entries 0 MB, inference:closure-walk-members 1308 entries 0 MB
2026-11-15T04:02:28.104Z graph-plan-too-heavy {"stage":"result","operation":"result","megabytes":6581,"limit":6553,"worker":{"heapMb":2090.3,"extraMb":1236,"objectCount":11238674,"objectTypes":[{"type":"string","count":6833841},{"type":"Object","count":1334751},{"type":"Array","count":1181024},{"type":"WeakRef","count":201788},{"type":"Set","count":169625},{"type":"Structure","count":46232},{"type":"Cell Butterfly","count":34140},{"type":"Function","count":26067}],"at":1794715298141}}
```

P12 sampler `round-18.p12-try-1.txt` (129 samples with a slice worker):

```
service log: /var/folders/1s/pfk8j2p90t94v1nx_gf8zq3w0000gn/T/semio-repo/501/host-79a74804a430.log
closure entries before: 42537
2026-11-15T03:51:00Z no slice worker
… (no footprint above 6000 MB)
2026-11-15T04:02:27Z pid=25203 rss=3881MB footprint=[bun [25203]: 64-bit    Footprint: 4631 MB (16384 bytes per page)] last=[graph: plan answered, managed 105 MB, resident 378 MB (released 0 MB)]
closure entries after: 40475
```

`sample` of the worker `round-18.p12-try-1.txt.sample-2026-11-15T03:59:03Z.txt` (head and `Sort by top of stack`):

```
Analysis of sampling bun (pid 25203) every 1 millisecond
Process:         bun [25203]
Path:            /Users/USER/Library/Caches/*/bun
Load Address:    0x104ae8000
Identifier:      bun
Version:         ???
Code Type:       ARM64
Platform:        macOS
Parent Process:  bun [25100]
Target Type:     live task

Date/Time:       2026-11-15 04:59:04.105 +0100
…
Sort by top of stack, same collapsed (when >= 5):
        __ulock_wait2  (in libsystem_kernel.dylib)        37021
        __psynch_cvwait  (in libsystem_kernel.dylib)        23899
        kevent64  (in libsystem_kernel.dylib)        3804
        ???  (in bun)  load address 0x104ae8000 + 0x311c08  [0x104df9c08]        594
        __read_nocancel  (in libsystem_kernel.dylib)        540
        __openat_nocancel  (in libsystem_kernel.dylib)        450
        ???  (in bun)  load address 0x104ae8000 + 0x311c10  [0x104df9c10]        344
        ???  (in bun)  load address 0x104ae8000 + 0x311c24  [0x104df9c24]        335
        ???  (in bun)  load address 0x104ae8000 + 0x315308  [0x104dfd308]        282
        ???  (in bun)  load address 0x104ae8000 + 0x300af8  [0x104de8af8]        253
        ???  (in bun)  load address 0x104ae8000 + 0x3029d4  [0x104dea9d4]        172
        ???  (in bun)  load address 0x104ae8000 + 0x2ec336c  [0x1079ab36c]        168
        ???  (in bun)  load address 0x104ae8000 + 0x3152e8  [0x104dfd2e8]        168
        ???  (in bun)  load address 0x104ae8000 + 0x3152cc  [0x104dfd2cc]        158
        ???  (in bun)  load address 0x104ae8000 + 0x311c28  [0x104df9c28]        155
        ???  (in bun)  load address 0x104ae8000 + 0x311c4c  [0x104df9c4c]        151
        ???  (in bun)  load address 0x104ae8000 + 0x302a08  [0x104deaa08]        146
        ???  (in bun)  load address 0x104ae8000 + 0x311c34  [0x104df9c34]        144
        ???  (in bun)  load address 0x104ae8000 + 0x300b34  [0x104de8b34]        141
        ???  (in bun)  load address 0x104ae8000 + 0x3127a4  [0x104dfa7a4]        124
        ???  (in bun)  load address 0x104ae8000 + 0x3154f8  [0x104dfd4f8]        120
        ???  (in bun)  load address 0x104ae8000 + 0x300b40  [0x104de8b40]        115
        ???  (in bun)  load address 0x104ae8000 + 0x3159b0  [0x104dfd9b0]        109
        ???  (in bun)  load address 0x104ae8000 + 0x3029ec  [0x104dea9ec]        101
        ???  (in bun)  load address 0x104ae8000 + 0x3127ac  [0x104dfa7ac]        101
        ???  (in bun)  load address 0x104ae8000 + 0x300b2c  [0x104de8b2c]        97
        ???  (in bun)  load address 0x104ae8000 + 0x315284  [0x104dfd284]        92
        ???  (in bun)  load address 0x104ae8000 + 0x3152f8  [0x104dfd2f8]        82
        ???  (in bun)  load address 0x104ae8000 + 0x302b80  [0x104deab80]        81
        ???  (in bun)  load address 0x104ae8000 + 0x3114f8  [0x104df94f8]        80
        ???  (in bun)  load address 0x104ae8000 + 0x3029ac  [0x104dea9ac]        78
        ???  (in bun)  load address 0x104ae8000 + 0x2ec3348  [0x1079ab348]        76
        syscall_thread_switch  (in libsystem_kernel.dylib)        75
        ???  (in bun)  load address 0x104ae8000 + 0x315950  [0x104dfd950]        74
        ???  (in bun)  load address 0x104ae8000 + 0x3026c4  [0x104dea6c4]        72
        ???  (in <unknown binary>)  [0x123d7d310]        70
        ???  (in bun)  load address 0x104ae8000 + 0x8e3c0  [0x104b763c0]        68
        ???  (in bun)  load address 0x104ae8000 + 0x2ec335c  [0x1079ab35c]        63
        ???  (in bun)  load address 0x104ae8000 + 0x31a688  [0x104e02688]        63
        ???  (in bun)  load address 0x104ae8000 + 0x31280c  [0x104dfa80c]        58
        ???  (in bun)  load address 0x104ae8000 + 0x311c2c  [0x104df9c2c]        55
        ???  (in bun)  load address 0x104ae8000 + 0x8dcc0  [0x104b75cc0]        55
        ???  (in bun)  load address 0x104ae8000 + 0x3029dc  [0x104dea9dc]        53
        ???  (in bun)  load address 0x104ae8000 + 0x31a680  [0x104e02680]        52
```

nx task lines: 0

Last 40 lines of the bootstrap log (without `[script] {` rows):

```
Reading the project graph slice (587 s): owners 95 of 270.
Reading the project graph slice (589 s): owners 95 of 270; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 106, 0).
Reading the project graph slice (591 s): owners 95 of 270; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 106, 0).
Reading the project graph slice (593 s): owners 95 of 270; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 111, 0).
Reading the project graph slice (595 s): owners 95 of 270; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 111, 0).
Reading the project graph slice (597 s): owners 95 of 270; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 111, 0).
Reading the project graph slice (599 s): owners 95 of 270; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 116, 0).
Reading the project graph slice (601 s): owners 95 of 270; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 116, 0).
Reading the project graph slice (603 s): owners 95 of 270; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 121, 0).
Reading the project graph slice (605 s): owners 95 of 270; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 121, 0).
Reading the project graph slice (607 s): owners 95 of 270; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 126, 0).
Reading the project graph slice (609 s): owners 95 of 270; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 126, 0).
Reading the project graph slice (611 s): owners 95 of 270; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 126, 0).
Reading the project graph slice (613 s): owners 95 of 270; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 126, 0).
Reading the project graph slice (615 s): owners 95 of 270; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 126, 0).
[repo] still working after 620 s; last phase started: door.script
Reading the project graph slice (617 s): owners 95 of 270; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 136, 0).
Reading the project graph slice (619 s): owners 95 of 270; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 136, 0).
Reading the project graph slice (621 s): owners 95 of 270; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 136, 0).
Reading the project graph slice (623 s): owners 95 of 270; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 141, 0).
Reading the project graph slice (625 s): owners 95 of 270; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 141, 0).
Reading the project graph slice (627 s): owners 95 of 270; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 141, 0).
Reading the project graph slice (629 s): owners 95 of 270; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 146, 0).
Reading the project graph slice (631 s): owners 95 of 270; running (name, kind, seconds, rows): .
Reading the project graph slice (633 s): owners 95 of 270; running (name, kind, seconds, rows): .
Reading the project graph slice (635 s): owners 95 of 270; running (name, kind, seconds, rows): .
Reading the project graph slice (637 s): owners 95 of 270; running (name, kind, seconds, rows): .
Reading the project graph slice (639 s): owners 95 of 270; running (name, kind, seconds, rows): .
Reading the project graph slice (641 s): owners 95 of 270; running (name, kind, seconds, rows): .
Reading the project graph slice (643 s): owners 95 of 270; running (name, kind, seconds, rows): .
Reading the project graph slice (645 s): owners 95 of 270; running (name, kind, seconds, rows): .
[repo] still working after 650 s; last phase started: door.script
Reading the project graph slice (647 s): owners 95 of 270; running (name, kind, seconds, rows): .
Reading the project graph slice (649 s): owners 96 of 270.
Reading the project graph slice (651 s): owners 96 of 270.
Reading the project graph slice (653 s): owners 96 of 270.
Reading the project graph slice (655 s): owners 96 of 270.
The project graph could not be read on this machine yet: the plan stage result (during result) grew to 6581 MB, more than the 6553 MB this machine gives a plan. It was stopped before the host ran short of memory. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
end 2026-11-15T04:02:28Z
native_rc=75
```

## Try 2

- Log `/Users/ueli/native-bootstrap-round-18-try-2.log`: `start 2026-11-15T04:05:36Z`
- Runner: `no end line`
- P12 sampler peak: `not measured`

Key lines of the graph service log, verbatim, in order:

```
(none)
```

P12 sampler `round-18.p12-try-2.txt` (88 samples with a slice worker):

```
service log: /var/folders/1s/pfk8j2p90t94v1nx_gf8zq3w0000gn/T/semio-repo/501/host-c5f530f58f3a.log
closure entries before: 40475
2026-11-15T04:05:40Z no slice worker
… (no footprint above 6000 MB)
2026-11-15T04:17:06Z pid=37416 rss=2642MB footprint=[bun [37416]: 64-bit    Footprint: 1527 MB (16384 bytes per page)] last=[graph: a worker process was lost (exited); its project is inferred again]
2026-11-15T04:17:11Z pid=37416 rss=2642MB footprint=[bun [37416]: 64-bit    Footprint: 1527 MB (16384 bytes per page)] last=[graph: a worker process was lost (exited); its project is inferred again]
```

nx task lines: 0

Last 40 lines of the bootstrap log (without `[script] {` rows):

```
Reading the project graph slice (374 s): flush 1 of 1.
Reading the project graph slice (376 s): flush 1 of 1.
Reading the project graph slice (378 s): flush 1 of 1.
Reading the project graph slice (380 s): flush 1 of 1.
Reading the project graph slice (382 s): flush 1 of 1.
Reading the project graph slice (384 s): flush 1 of 1.
Reading the project graph slice (386 s): flush 1 of 1.
Reading the project graph slice (388 s): flush 1 of 1.
Reading the project graph slice (390 s): flush 1 of 1.
Reading the project graph slice (392 s): flush 1 of 1.
Reading the project graph slice (394 s): flush 1 of 1.
Reading the project graph slice (396 s): flush 1 of 1.
Reading the project graph slice (398 s): flush 1 of 1.
Reading the project graph slice (400 s): flush 1 of 1.
Reading the project graph slice (402 s): flush 1 of 1.
[repo] still working after 410 s; last phase started: door.script
Reading the project graph slice (404 s): flush 1 of 1.
Reading the project graph slice (406 s): flush 1 of 1.
Reading the project graph slice (408 s): flush 1 of 1.
Reading the project graph slice (410 s): flush 1 of 1.
Reading the project graph slice (412 s): flush 1 of 1.
Reading the project graph slice (414 s): flush 1 of 1.
Reading the project graph slice (416 s): flush 1 of 1.
Reading the project graph slice (418 s): flush 1 of 1.
Reading the project graph slice (420 s): flush 1 of 1.
Reading the project graph slice (422 s): flush 1 of 1.
Reading the project graph slice (424 s): flush 1 of 1.
Reading the project graph slice (426 s): flush 1 of 1.
Reading the project graph slice (428 s): flush 1 of 1.
Reading the project graph slice (430 s): flush 1 of 1.
Reading the project graph slice (432 s): flush 1 of 1.
[repo] still working after 440 s; last phase started: door.script
Reading the project graph slice (434 s): flush 1 of 1.
Reading the project graph slice (436 s): flush 1 of 1.
Reading the project graph slice (438 s): flush 1 of 1.
Reading the project graph slice (440 s): flush 1 of 1.
Reading the project graph slice (442 s): flush 1 of 1.
Reading the project graph slice (444 s): flush 1 of 1.
Reading the project graph slice (446 s): flush 1 of 1.
Reading the project graph slice (448 s): flush 1 of 1.
```
