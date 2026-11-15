# g1-m02 Round 13

round: 13

Result: not ready. Two tries, no nx task ran (succeeded: none, failed: none, not run: all). Try 1 ended with the typed refusal `graph-plan-too-heavy` in plan stage `environment` (`native_rc=75`): the service stopped the plan itself and the machine stayed at pressure level 1. Try 2 waited for the fleet with no new progress line for 905 s and got one SIGINT from me (`native_rc=7`). Round 14 was published before try 3. `.cache/f/native-bootstrap.done` does not exist.

## Machine

Mac mini `Mac16,10`, Apple M4, 10 cores, 16 GB (17179869184 bytes), macOS 27.2.1 (26B131). Free disk: not measured in this round.

## Tries

| Try | Log | Start (UTC) | End (UTC) | `native_rc` | Largest process on the machine (30 s samples) | P12 peak of the slice worker | How it ended |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `~/native-bootstrap-round-13.log` | 01:40:24 | 01:46:04 | 75 | 4908464 KiB | rss 6734 MB, footprint 6591 MB at 01:46:04Z | `graph-plan-too-heavy`, stage `environment` |
| 2 | `~/native-bootstrap-round-13-try-2.log` | 01:49:37 | 02:06:43 | 7 | 3100912 KiB | rss 3028 MB, footprint 483 MB at 01:51:43Z | stall: `graph: waiting for the fleet: 46 of 703 projects arrived` unchanged for 905 s; my SIGINT |

- Pressure level 1 in all 47 samples (30 s apart); no hard stop.
- Try 1 restore line: A = 26 min, S = `?` (verbatim: `from a state 26 min old with ? projects stale`), then `refreshing 703 projects (generation 212)`. Fleet: `0 taken`; `0 of 48 projects inferred by 1 worker processes in 56 s`.
- Try 1 plan stages: `inference` took 64.6 s (565 MB more than at its start); `environment` started and has no `took` line. In the `worker` sample `heapMb` (1444.4) is above `extraMb` (964.2); the object types are led by `string` (3352368), `Array` (1179397), `Object` (848746).
- Try 1 sampler: the peak's `last=[…]` shows the `fact index of generation 75` line, which is an old line: the sampler's `tail -n 600` of the service log did not reach a newer `graph:` line because `[script]` rows fill the log. By the service log the stage at the peak was `environment`.
- Try 2: a new service log (`host-007c265f8fa9.log`); no restore line and no plan stage line in it. `703 stale projects, 0 taken from the fleet, 19 too heavy`; the fleet delivered 46 of 703 and then nothing for 15 minutes.
- Closure entries (`ce/e` files): 42537 before and after both tries.
- Launcher unchanged in both tries (inode 3679486, `CDHash=6de7547a8b8ff384bbfbfc45e962462063ff0357`).

## Before the Run (2026-11-15T01:40Z)

- Replica agent: `state = running`, `runs = 23` (22 in round 11); started by me 0 times in this round.
- Replica code digest (`.cache/f/health.json`): `52e46eee5c577cf8425b43f243681f24326d6645807f938982368d8c64cbfc7f` (ROUND.md says the digest is unchanged; in round 12 this machine had `eabbc9226732…`).
- `bootVersion`: 6

## Try 1

- Log `/Users/ueli/native-bootstrap-round-13.log`: `start 2026-11-15T01:40:24Z`, `end 2026-11-15T01:46:04Z`, `native_rc=75`
- Runner: `native_rc=75 peak_rss_kib=4908464 run_peak_rss_kib=454928 stop=None fleet_refusal=True closure_missing=False nx_task_lines=0 error_lines=0 moving=False`
- P12 sampler peak: `2026-11-15T01:46:04Z pid=45946 rss=6734MB footprint=[bun [45946]: 64-bit    Footprint: 6591 MB (16384 bytes per page)] last=[graph: fact index of generation 75: 758657 facts of 5101 projects, 95.2 MB expanded (18.6 % of the 512 MB limit), 13.8 MB packed]`

Key lines of the graph service log, verbatim, in order:

```
graph: generation 211 (5101 projects) restored from the remote cache, 5105 documents (46 MB) downloaded in 14 s, from a state 26 min old with ? projects stale
graph: refreshing 703 projects (generation 212)
graph: plan stage inference starts (managed 1088 MB, resident 2960 MB)
graph: plan stage inference took 64.6 s (managed 1653 MB, 565 MB more than at its start, resident 3096 MB)
graph: plan stage environment starts (managed 1653 MB, resident 3096 MB)
graph: plan stage environment grew the plan by 4953 MB, more than the 4915 MB this machine gives a plan; it is stopped before the host runs short of memory
2026-11-15T01:46:04.487Z graph-plan-too-heavy {"stage":"environment","megabytes":6712,"limit":6553,"worker":{"heapMb":1444.4,"extraMb":964.2,"objectCount":7027604,"objectTypes":[{"type":"string","count":3352368},{"type":"Array","count":1179397},{"type":"Object","count":848746},{"type":"Set","count":163232},{"type":"WeakRef","count":104539},{"type":"Structure","count":30613},{"type":"Cell Butterfly","count":27591},{"type":"Promise","count":26832}],"at":1794707151644}}
```

`graph-plan-too-heavy` with the 20 lines before it:

```
[script] {"owner":"repository:build","command":"build:command-source-packages","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:projects","stage":"running","activity":{"resource":"build:graph-phase:inference:47:48","mode":"shared","elapsedMs":0}}
[script] build:projects · Inferring projects: 47 of 48 projects
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:47:703","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"build:projects","stage":"running","activity":{"resource":"build:graph-phase:inference:48:48","mode":"shared","elapsedMs":0}}
[script] build:projects · Inferring projects: 48 of 48 projects
graph: 0 of 48 projects inferred by 1 worker processes in 56 s (0 failed, 0 lost, 0 restarts)
[fleet] projects from the fleet: 0 taken, 0 stored, 0 offered, 0 waited for another machine, 0 computed here | Projekte aus der Flotte: 0 übernommen, 0 gespeichert, 0 angeboten, 0 auf eine andere Maschine gewartet, 0 hier berechnet
[script] {"owner":"repository:build","command":"build:infer","stage":"running"}
[script] build:infer · Inferring projects
graph: plan stage inference took 64.6 s (managed 1653 MB, 565 MB more than at its start, resident 3096 MB)
graph: plan stage environment starts (managed 1653 MB, resident 3096 MB)
[repo] still working after 290 s; last phase started: source.served
[capacity] waiting 57s for exec: not enough free memory (need 1536 MB; -776 MB spare above the 2048 MB floor, load 8.39; 3 running, 0 ahead); behind graph-plan (running, 1 job(s), 3208 MB), workspace:setup (quiet, 1 job(s), 2048 MB), graph-farm (running, 1 job(s), 1536 MB)
[repository:command] still running after 290 s
[capacity] waiting 63s for exec: not enough free memory (need 1536 MB; -717 MB spare above the 2048 MB floor, load 8.12; 3 running, 0 ahead); behind graph-plan (running, 1 job(s), 3434 MB), workspace:setup (quiet, 1 job(s), 2048 MB), graph-farm (running, 1 job(s), 0 MB)
graph: plan stage environment grew the plan by 4953 MB, more than the 4915 MB this machine gives a plan; it is stopped before the host runs short of memory
[capacity] waiting 69s for exec: not enough free memory (need 1536 MB; 748 MB spare above the 2048 MB floor, load 9.24; 3 running, 0 ahead); behind graph-plan (running, 1 job(s), 3434 MB), workspace:setup (quiet, 1 job(s), 46 MB), graph-farm (running, 1 job(s), 0 MB)
[repository:command] still running after 300 s
2026-11-15T01:46:04.487Z graph-plan-too-heavy {"stage":"environment","megabytes":6712,"limit":6553,"worker":{"heapMb":1444.4,"extraMb":964.2,"objectCount":7027604,"objectTypes":[{"type":"string","count":3352368},{"type":"Array","count":1179397},{"type":"Object","count":848746},{"type":"Set","count":163232},{"type":"WeakRef","count":104539},{"type":"Structure","count":30613},{"type":"Cell Butterfly","count":27591},{"type":"Promise","count":26832}],"at":1794707151644}}
```

P12 sampler `round-13.p12-try-1.txt` (59 samples with a slice worker):

```
service log: /var/folders/1s/pfk8j2p90t94v1nx_gf8zq3w0000gn/T/semio-repo/501/host-ac6bfe8f337c.log
closure entries before: 42537
2026-11-15T01:40:29Z no slice worker
…
2026-11-15T01:45:44Z pid=45946 rss=3095MB footprint=[bun [45946]: 64-bit    Footprint: 1572 MB (16384 bytes per page)] last=[graph: fact index of generation 75: 758657 facts of 5101 projects, 95.2 MB expanded (18.6 % of the 512 MB limit), 13.8 MB packed]
2026-11-15T01:45:49Z pid=45946 rss=3095MB footprint=[bun [45946]: 64-bit    Footprint: 1570 MB (16384 bytes per page)] last=[graph: fact index of generation 75: 758657 facts of 5101 projects, 95.2 MB expanded (18.6 % of the 512 MB limit), 13.8 MB packed]
2026-11-15T01:45:54Z pid=45946 rss=3708MB footprint=[bun [45946]: 64-bit    Footprint: 1904 MB (16384 bytes per page)] last=[graph: fact index of generation 75: 758657 facts of 5101 projects, 95.2 MB expanded (18.6 % of the 512 MB limit), 13.8 MB packed]
2026-11-15T01:45:59Z pid=45946 rss=5914MB footprint=[bun [45946]: 64-bit    Footprint: 3523 MB (16384 bytes per page)] last=[graph: fact index of generation 75: 758657 facts of 5101 projects, 95.2 MB expanded (18.6 % of the 512 MB limit), 13.8 MB packed]
2026-11-15T01:46:04Z pid=45946 rss=6734MB footprint=[bun [45946]: 64-bit    Footprint: 6591 MB (16384 bytes per page)] last=[graph: fact index of generation 75: 758657 facts of 5101 projects, 95.2 MB expanded (18.6 % of the 512 MB limit), 13.8 MB packed]
2026-11-15T01:46:09Z no slice worker
2026-11-15T01:46:14Z no slice worker
2026-11-15T01:46:20Z no slice worker
2026-11-15T01:46:25Z no slice worker
closure entries after: 42537
```

`sample` of the worker `round-13.p12-try-1.txt.sample-2026-11-15T01:45:59Z.txt` (head and `Sort by top of stack`):

```
Analysis of sampling bun (pid 45946) every 1 millisecond
Process:         bun [45946]
Path:            /Users/USER/Library/Caches/*/bun
Load Address:    0x104dcc000
Identifier:      bun
Version:         ???
Code Type:       ARM64
Platform:        macOS
Parent Process:  bun [45847]
Target Type:     live task

Date/Time:       2026-11-15 02:45:59.882 +0100
…
Sort by top of stack, same collapsed (when >= 5):
        __psynch_cvwait  (in libsystem_kernel.dylib)        23550
        __ulock_wait2  (in libsystem_kernel.dylib)        20125
        __read_nocancel  (in libsystem_kernel.dylib)        4946
        kevent64  (in libsystem_kernel.dylib)        2976
        __openat_nocancel  (in libsystem_kernel.dylib)        2791
        ???  (in bun)  load address 0x104dcc000 + 0x315950  [0x1050e1950]        945
        ???  (in bun)  load address 0x104dcc000 + 0x300b2c  [0x1050ccb2c]        206
        madvise  (in libsystem_kernel.dylib)        182
        ???  (in bun)  load address 0x104dcc000 + 0x311450  [0x1050dd450]        180
        _platform_memmove  (in libsystem_platform.dylib)        174
        ???  (in bun)  load address 0x104dcc000 + 0x311514  [0x1050dd514]        141
        ???  (in bun)  load address 0x104dcc000 + 0x302998  [0x1050ce998]        131
        ???  (in bun)  load address 0x104dcc000 + 0x3029d4  [0x1050ce9d4]        112
        ???  (in bun)  load address 0x104dcc000 + 0x302990  [0x1050ce990]        106
        ???  (in bun)  load address 0x104dcc000 + 0x340df4  [0x10510cdf4]        96
        ???  (in bun)  load address 0x104dcc000 + 0x311c68  [0x1050ddc68]        88
        ???  (in bun)  load address 0x104dcc000 + 0xa088  [0x104dd6088]        87
        ???  (in bun)  load address 0x104dcc000 + 0x2fee1c  [0x1050cae1c]        81
        ???  (in bun)  load address 0x104dcc000 + 0x315308  [0x1050e1308]        77
        ???  (in bun)  load address 0x104dcc000 + 0x2d8fcd0  [0x107b5bcd0]        73
        ???  (in bun)  load address 0x104dcc000 + 0x2ec336c  [0x107c8f36c]        73
        ???  (in bun)  load address 0x104dcc000 + 0x3154f8  [0x1050e14f8]        61
        ???  (in bun)  load address 0x104dcc000 + 0x300ac0  [0x1050ccac0]        60
        ???  (in bun)  load address 0x104dcc000 + 0x311c08  [0x1050ddc08]        60
        ???  (in bun)  load address 0x104dcc000 + 0x300af8  [0x1050ccaf8]        58
        ???  (in bun)  load address 0x104dcc000 + 0x3152c4  [0x1050e12c4]        58
        ???  (in bun)  load address 0x104dcc000 + 0x3029ac  [0x1050ce9ac]        55
        ???  (in bun)  load address 0x104dcc000 + 0x302a08  [0x1050cea08]        52
        ???  (in bun)  load address 0x104dcc000 + 0x340dac  [0x10510cdac]        48
        ???  (in bun)  load address 0x104dcc000 + 0x3127ac  [0x1050de7ac]        47
        ???  (in bun)  load address 0x104dcc000 + 0x300b24  [0x1050ccb24]        43
        ???  (in bun)  load address 0x104dcc000 + 0x3029f4  [0x1050ce9f4]        40
        ???  (in bun)  load address 0x104dcc000 + 0x2d8fcdc  [0x107b5bcdc]        39
        ???  (in bun)  load address 0x104dcc000 + 0x300ac8  [0x1050ccac8]        39
        ???  (in bun)  load address 0x104dcc000 + 0x3152b0  [0x1050e12b0]        39
        ???  (in bun)  load address 0x104dcc000 + 0x31a680  [0x1050e6680]        38
        ???  (in bun)  load address 0x104dcc000 + 0x101840c  [0x105de440c]        37
        ???  (in bun)  load address 0x104dcc000 + 0x3152e8  [0x1050e12e8]        37
        ???  (in bun)  load address 0x104dcc000 + 0x302b6c  [0x1050ceb6c]        36
        ???  (in bun)  load address 0x104dcc000 + 0x3152cc  [0x1050e12cc]        35
        ???  (in bun)  load address 0x104dcc000 + 0x31a6d4  [0x1050e66d4]        35
        ???  (in bun)  load address 0x104dcc000 + 0x1daa6c4  [0x106b766c4]        33
        ???  (in bun)  load address 0x104dcc000 + 0x315278  [0x1050e1278]        32
        ???  (in bun)  load address 0x104dcc000 + 0x31a6ac  [0x1050e66ac]        32
```

nx task lines: 0

Last 40 lines of the bootstrap log (without `[script] {` rows):

```
Reading the project graph slice (228 s): owners 0 of 703; running (name, kind, seconds, rows): @semio-tech/energy-oracle-py (project, 0, 0).
Reading the project graph slice (230 s): owners 0 of 703; running (name, kind, seconds, rows): @semio-tech/energy-oracle-py (project, 0, 0).
Reading the project graph slice (232 s): owners 0 of 703; running (name, kind, seconds, rows): @semio-tech/energy-oracle-py (project, 5, 0); waiting for memory headroom (1756 of 1024 MB reserved).
Reading the project graph slice (234 s): owners 0 of 703; running (name, kind, seconds, rows): @semio-tech/energy-oracle-py (project, 5, 0); waiting for memory headroom (1756 of 1024 MB reserved).
Reading the project graph slice (236 s): owners 0 of 703; running (name, kind, seconds, rows): @semio-tech/energy-oracle-py (project, 5, 0); waiting for memory headroom (1756 of 1024 MB reserved).
Reading the project graph slice (238 s): owners 0 of 703; running (name, kind, seconds, rows): @semio-tech/energy-oracle-py (project, 11, 0); waiting for memory headroom (1756 of 1365 MB reserved).
Reading the project graph slice (240 s): owners 0 of 703; running (name, kind, seconds, rows): @semio-tech/energy-oracle-py (project, 11, 0); waiting for memory headroom (1756 of 1365 MB reserved).
Reading the project graph slice (242 s): owners 1 of 703.
Reading the project graph slice (244 s): owners 1 of 703.
Reading the project graph slice (246 s): owners 1 of 703.
Reading the project graph slice (248 s): owners 1 of 703; running (name, kind, seconds, rows): @semio-tech/async-timer-rs (language:rust, 5, 0); waiting for memory headroom (5759 of 736 MB reserved).
Reading the project graph slice (250 s): owners 3 of 703.
Reading the project graph slice (252 s): owners 14 of 703.
[repo] still working after 260 s; last phase started: door.script
Reading the project graph slice (254 s): owners 15 of 703.
Reading the project graph slice (256 s): owners 23 of 703.
Reading the project graph slice (258 s): owners 25 of 703.
Reading the project graph slice (260 s): owners 25 of 703.
Reading the project graph slice (262 s): owners 34 of 703; running (name, kind, seconds, rows): @semio-tech/dsl-syntax-rs (language:rust, 0, 0), @semio-tech/dsl-lexical-rs (language:rust, 0, 0).
Reading the project graph slice (264 s): owners 42 of 703.
Reading the project graph slice (267 s): owners 42 of 703.
Reading the project graph slice (269 s): owners 44 of 703.
Reading the project graph slice (271 s): owners 44 of 703.
Reading the project graph slice (273 s): owners 45 of 703; running (name, kind, seconds, rows): @semio-tech/dsl-derive-rs (language:cargo, 4, 0), @semio-tech/component-deployment-contract (project, 3, 0).
Reading the project graph slice (275 s): owners 45 of 703; running (name, kind, seconds, rows): @semio-tech/dsl-derive-rs (language:cargo, 4, 0), @semio-tech/component-deployment-contract (project, 3, 0).
Reading the project graph slice (277 s): owners 45 of 703; running (name, kind, seconds, rows): @semio-tech/dsl-derive-rs (language:cargo, 4, 0), @semio-tech/component-deployment-contract (project, 3, 0).
Reading the project graph slice (279 s): owners 47 of 703.
Reading the project graph slice (281 s): owners 47 of 703.
[repo] still working after 290 s; last phase started: door.script
Reading the project graph slice (283 s): owners 47 of 703; running (name, kind, seconds, rows): .
Reading the project graph slice (285 s): owners 47 of 703; running (name, kind, seconds, rows): .
Reading the project graph slice (287 s): owners 47 of 703; running (name, kind, seconds, rows): .
Reading the project graph slice (289 s): owners 47 of 703; running (name, kind, seconds, rows): .
Reading the project graph slice (291 s): owners 47 of 703; running (name, kind, seconds, rows): .
Reading the project graph slice (293 s): owners 47 of 703; running (name, kind, seconds, rows): .
Reading the project graph slice (295 s): owners 47 of 703; running (name, kind, seconds, rows): .
Reading the project graph slice (297 s): owners 47 of 703; running (name, kind, seconds, rows): .
The project graph could not be read on this machine yet: the plan stage environment grew to 6712 MB, more than the 6553 MB this machine gives a plan. It was stopped before the host ran short of memory. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
end 2026-11-15T01:46:04Z
native_rc=75
```

## Try 2

- Log `/Users/ueli/native-bootstrap-round-13-try-2.log`: `start 2026-11-15T01:49:37Z`, `end 2026-11-15T02:06:43Z`, `native_rc=7`
- Runner: `native_rc=7 peak_rss_kib=3100912 run_peak_rss_kib=130176 stop=no new progress line for 905 s fleet_refusal=False closure_missing=False nx_task_lines=0 error_lines=0 moving=False`
- P12 sampler peak: `2026-11-15T01:51:43Z pid=51680 rss=3028MB footprint=[bun [51680]: 64-bit    Footprint: 483 MB (16384 bytes per page)] last=[graph: waiting for the fleet: 0 of 703 projects arrived]`

Key lines of the graph service log, verbatim, in order:

```
graph: 703 stale projects, 0 taken from the fleet, 19 too heavy for this machine (@semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), @semio-tech/framework-pack-json-rs (about 15422 MB), repo (about 14264 MB), @semio-tech/framework-actor-retained-turn-rs (about 11518 MB), @semio-tech/framework-plugin (about 8626 MB), and 13 more; it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 703
```

P12 sampler `round-13.p12-try-2.txt` (197 samples with a slice worker):

```
service log: /var/folders/1s/pfk8j2p90t94v1nx_gf8zq3w0000gn/T/semio-repo/501/host-007c265f8fa9.log
closure entries before: 42537
2026-11-15T01:49:41Z no slice worker
… (no footprint above 6000 MB)
2026-11-15T02:07:12Z no slice worker
closure entries after: 42537
```

nx task lines: 0

Last 40 lines of the bootstrap log (without `[script] {` rows):

```
Reading the project graph slice (927 s): flush 1 of 1.
Reading the project graph slice (929 s): flush 1 of 1.
Reading the project graph slice (931 s): flush 1 of 1.
Reading the project graph slice (933 s): flush 1 of 1.
Reading the project graph slice (935 s): flush 1 of 1.
Reading the project graph slice (937 s): flush 1 of 1.
Reading the project graph slice (939 s): flush 1 of 1.
Reading the project graph slice (941 s): flush 1 of 1.
Reading the project graph slice (943 s): flush 1 of 1.
Reading the project graph slice (945 s): flush 1 of 1.
Reading the project graph slice (947 s): flush 1 of 1.
Reading the project graph slice (949 s): flush 1 of 1.
[repo] still working after 950 s; last phase started: door.script
Reading the project graph slice (951 s): flush 1 of 1.
Reading the project graph slice (953 s): flush 1 of 1.
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
Reading the project graph slice (975 s): flush 1 of 1.
Reading the project graph slice (977 s): flush 1 of 1.
Reading the project graph slice (979 s): flush 1 of 1.
[repo] still working after 980 s; last phase started: door.script
Reading the project graph slice (981 s): flush 1 of 1.
Reading the project graph slice (983 s): flush 1 of 1.
Reading the project graph slice (985 s): flush 1 of 1.
Reading the project graph slice (987 s): flush 1 of 1.
Reading the project graph slice (989 s): flush 1 of 1.
Reading the project graph slice (991 s): flush 1 of 1.
Reading the project graph slice (993 s): flush 1 of 1.
Reading the project graph slice (995 s): flush 1 of 1.
Reading the project graph slice (997 s): flush 1 of 1.
end 2026-11-15T02:06:43Z
native_rc=7
```

## Stall 1: Last 60 Lines of the Service Log

```
[repository:command] still running after 1280 s
graph: waiting for the fleet: 46 of 703 projects arrived
[repository:command] still running after 1290 s
graph: waiting for the fleet: 46 of 703 projects arrived
[repository:command] still running after 1300 s
graph: waiting for the fleet: 46 of 703 projects arrived
[repo] still working after 1310 s; last phase started: source.served
[repository:command] still running after 1310 s
graph: waiting for the fleet: 46 of 703 projects arrived
[repository:command] still running after 1320 s
graph: waiting for the fleet: 46 of 703 projects arrived
[repository:command] still running after 1330 s
[repo] still working after 1340 s; last phase started: source.served
graph: waiting for the fleet: 46 of 703 projects arrived
[repository:command] still running after 1340 s
graph: waiting for the fleet: 46 of 703 projects arrived
[repository:command] still running after 1350 s
graph: waiting for the fleet: 46 of 703 projects arrived
[repository:command] still running after 1360 s
[repo] still working after 1370 s; last phase started: source.served
graph: waiting for the fleet: 46 of 703 projects arrived
[repository:command] still running after 1370 s
graph: waiting for the fleet: 46 of 703 projects arrived
[repository:command] still running after 1380 s
graph: waiting for the fleet: 46 of 703 projects arrived
[repository:command] still running after 1390 s
[repo] still working after 1400 s; last phase started: source.served
graph: waiting for the fleet: 46 of 703 projects arrived
[repository:command] still running after 1400 s
graph: waiting for the fleet: 46 of 703 projects arrived
[repository:command] still running after 1410 s
graph: waiting for the fleet: 46 of 703 projects arrived
[repository:command] still running after 1420 s
[repo] still working after 1430 s; last phase started: source.served
graph: waiting for the fleet: 46 of 703 projects arrived
[repository:command] still running after 1430 s
graph: waiting for the fleet: 46 of 703 projects arrived
[repository:command] still running after 1440 s
graph: waiting for the fleet: 46 of 703 projects arrived
[repository:command] still running after 1450 s
[repo] still working after 1460 s; last phase started: source.served
graph: waiting for the fleet: 46 of 703 projects arrived
[repository:command] still running after 1460 s
graph: waiting for the fleet: 46 of 703 projects arrived
[repository:command] still running after 1470 s
graph: waiting for the fleet: 46 of 703 projects arrived
[repository:command] still running after 1480 s
[repo] still working after 1490 s; last phase started: source.served
graph: waiting for the fleet: 46 of 703 projects arrived
[repository:command] still running after 1490 s
graph: waiting for the fleet: 46 of 703 projects arrived
[repository:command] still running after 1500 s
graph: waiting for the fleet: 46 of 703 projects arrived
[repository:command] still running after 1510 s
[repo] still working after 1520 s; last phase started: source.served
graph: waiting for the fleet: 46 of 703 projects arrived
[repository:command] still running after 1520 s
graph: waiting for the fleet: 46 of 703 projects arrived
[repository:command] still running after 1530 s
graph: waiting for the fleet: 46 of 703 projects arrived
```

## Stall 1: `ps` of the Service and the Run

```
35202 35172 35172 02-01:33:26 669328   1.5 /Applications/Claude.app/Contents/Frameworks/Claude Helper (Renderer).app/Contents/MacOS/Claude Helper (Renderer) --type=renderer --user-data-dir=/Users/ueli/Library/Application Support/Claude --standard-schemes=cowor
45829     1 45829       25:43  65776   0.0 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/📜️script.
45847 45829 45847       25:40 300048   1.2 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun --eval \012const fs=await import('node:fs'),path=await import('node:path'),url=await import('node:url'
51199 45255 51199       17:06   1152   0.0 bash /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES/native-bootstrap.sh /Users/ueli/native-bootstrap-round-13-try-2.log
51212 51199 51199       17:06   1216   0.0 bash /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh setup
51213 51199 51199       17:06    224   0.0 tee /Users/ueli/Documents/semio/.cache/f/native-bootstrap.log
```
