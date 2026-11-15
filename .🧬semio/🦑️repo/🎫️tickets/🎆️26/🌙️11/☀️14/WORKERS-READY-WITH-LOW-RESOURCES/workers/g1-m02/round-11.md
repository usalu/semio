# g1-m02 Round 11

round: 11

Result: not ready. Eleven bootstrap runs, no nx task ran in any (succeeded: none, failed: none, not run: all). `.cache/f/native-bootstrap.done` does not exist. The runner ended when round 12 (pause) was published; no try was running then. Reference `3420bc16` did not appear. No stall of 15 minutes, no `native_rc=4`, no `fstatat failed (EPERM)`, no fallback line.

New in this round: from try 6 on the graph was restored from the fleet (`generation 74 (5100 projects) restored from the remote cache`), only 5 and then 1 project were refreshed here, generation 75 was published, and then the service failed in six tries in a row at a missing file under `.cache/b/g/m/build-input-closures/` (stack below).

## Machine

Mac mini `Mac16,10`, Apple M4, 10 cores, 16 GB (17179869184 bytes), macOS 27.2.1 (26B131). Free disk after the round: 100 GiB of 228 GiB.

## Tries

| Try | Log | Start (UTC) | End (UTC) | `native_rc` | Largest process on the machine | Largest in the run's group | How it ended |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `~/native-bootstrap-round-11.log` | 23:53:5x | 23:54:11 | 1 | not measured | not measured | my SIGINT after about 20 s, in the tool phase (runner fix, see below) |
| 2 | `…-try-2.log` | 23:54:41 | 00:12:16 | 75 | 3610304 KiB | 131968 KiB | fleet refusal: 659 stale, 356 taken, asked for 303, delivered 0 within 901 s |
| 3 | `…-try-3.log` | 00:17:18 | 00:29:54 | 7 | 6128128 KiB | 131088 KiB | hard stop: pressure level 4 in two samples in a row; local refresh at `owners 47 of 659` |
| 4 | `…-try-4.log` | 00:33:24 | 00:40:28 | 7 | 6036816 KiB | 134032 KiB | hard stop: level 4 in two samples in a row; `owners 71 of 659` |
| 5 | `…-try-5.log` | 00:43:58 | 00:56:35 | 7 | 5170176 KiB | 131296 KiB | hard stop: level 2 in six samples in a row; `owners 5 of 5` (generation 69) |
| 6 | `…-try-6.log` | 01:00:06 | 01:03:52 | 1 | 5018432 KiB | 130896 KiB | `unexpected failure (reference 21c18fad)` at `assembly 0 of 703` |
| 7 | `…-try-7.log` | 01:06:08 | 01:08:50 | 1 | 3336320 KiB | 371248 KiB | reference `21c18fad` |
| 8 | `…-try-8.log` | 01:11:10 | 01:13:42 | 1 | 3411040 KiB | 368496 KiB | reference `21c18fad` |
| 9 | `…-try-9.log` | 01:16:12 | 01:18:34 | 1 | 2870992 KiB | 124416 KiB | reference `21c18fad` |
| 10 | `…-try-10.log` | 01:20:44 | 01:22:53 | 1 | 3061888 KiB | 358240 KiB | reference `29bbd28b`, same stack, no message |
| 11 | `…-try-11.log` | 01:25:15 | 01:26:55 | 1 | 2700304 KiB | 134528 KiB | reference `21c18fad` |

- Pressure samples (30 s apart) of tries 2 to 11: level 1 in 125, level 2 in 8, level 4 in 4. The largest process in tries 3 and 4 (about 6 GB) was not in the run's process group; it is in `round-11.samples.txt`.
- The local refresh of 659 projects (tries 3 and 4) drove the machine to level 4 within 7 to 12 minutes.

## The Failure of Tries 6 to 11, Verbatim From the Service Log

```
2026-11-15T01:26:55.553Z request-failed 21c18fad Error: ENOENT: no such file or directory, lstat '/Users/ueli/Documents/semio/.cache/b/g/m/build-input-closures/0f4fa61aa5e5ca61388e839d9a7ffcc74aadca621f8734b6dc8542eb06aec6e3.json'
    at lstatSync (unknown)
    at readInventoryRecordLeanV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🪶️lean-inventory/🟨️.mjs:12:48)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📁️workspace/🟦️.ts:86:20)
    at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs:146:491)
    at processTicksAndRejections (native:7:39)
```

The same line and file name at 01:03:52, 01:08:50, 01:13:42, 01:18:34 and 01:26:55 UTC. At 01:22:53 (try 10) the reference was `29bbd28b` with `Error` and no message, same stack. The line before it in the service log: `[script] graph:slice-worker · Expanding input closures: 0 of 341 closures`. Service log: `$TMPDIR/semio-repo/501/host-ac6bfe8f337c.log`.

## What I Did Differently From ROUND.md

- Try 1: I ended my own run after about 20 s with one SIGINT, because my runner's rule for "no new progress line for 15 minutes" counted repeated lines as progress. The corrected runner started try 2 at 23:54:41Z.
- Eleven runs instead of ten: try 1 above, and try 2 was not counted (its refusal said "delivered 0 within", which my runner took for the group server move window; whether the server was being moved then was not measured).
- Tries 6 to 11 repeated the same service failure. ROUND.md's stop rule names "the same task errors"; no task had run, so my runner went on every 2 minutes. Six identical failures are five more than were needed.

## Launcher

Unchanged before and after every try and between tries: target `versions/cf25f2b751f83336/repo`, inode 3679486, mtime 2026-11-14T22:03:22 local time, size 6523360, `Identifier=tech.semio.repo`, `CDHash=6de7547a8b8ff384bbfbfc45e962462063ff0357`.

## Before the Run (2026-11-14T23:53Z)

- Replica agent: `state = running`, `runs = 22`; started by me 0 times in this round.
- Replica code digest (`.cache/f/health.json`): `eabbc92267329dbd39d8d05e0c8eceb1bcad083c0eea58c44805408a7d547bcc` (my clock said 23:53:43Z; ROUND.md names 23:58 UTC for the landing of the round 11 fix, so this may be the digest before it).
- `bootVersion`: 6
- Try 1 (`~/native-bootstrap-round-11.log`, start 23:53:5x Z, end 23:54:11Z, `native_rc=1`) was ended by me with one SIGINT after about 20 s: my runner's stall rule was wrong (it counted repeated lines as progress) and I restarted the runner with the corrected rule. It was still in the tool phase. The run goes on as try 2.

## Try 1

- Log `/Users/ueli/native-bootstrap-round-11.log`: `start 2026-11-14T23:53:48Z`, `end 2026-11-14T23:54:11Z`, `native_rc=1`
- Runner: `not recorded`
- nx task lines: 0

`[capacity]` lines of the bootstrap log:
```
[capacity] run: sized at 4% of the default from 16 earlier run(s) of its class, at least 2384 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 3000 MB reserved (6486 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 6 earlier run(s) of its class
[capacity] dependencies: sized at 33% of the default from 16 earlier run(s) of its class, at least 2663 MB because of its assembly phase
[capacity] admitted dependencies: 1 job(s), 2663 MB reserved (6741 MB available, 2048 MB floor)
```

Last distinct lines of the bootstrap log:
```
  --limit=<n>                                     fleet status: how many runs the text output lists (default: 20, the JSON output always lists all). (used by: fleet)
  --key=<key>                                     scheduler run: the dedupe key; agents that run the same key at the same time share one run (default: the command line). (used by: scheduler)
  --priority=<band>                               scheduler run: priority band background, agent or interactive (default: agent). (used by: scheduler)
[repo] Installed native repository command: /Users/ueli/.local/bin/repo
[semio] Installing the dependency scopes (the planner of every later run needs them)…
[capacity] dependencies: sized at 33% of the default from 16 earlier run(s) of its class, at least 2663 MB because of its assembly phase
[capacity] admitted dependencies: 1 job(s), 2663 MB reserved (6741 MB available, 2048 MB floor)
[script] build:source-admission · Admitting the sources of the build
[script] build:implementation-source · Reading the sources of the build implementation
[repo] still working after 5 s; last phase started: source.served
end 2026-11-14T23:54:11Z
native_rc=1
```

## Try 2

- Log `/Users/ueli/native-bootstrap-round-11-try-2.log`: `start 2026-11-14T23:54:41Z`, `end 2026-11-15T00:12:16Z`, `native_rc=75`
- Runner: `native_rc=75 peak_rss_kib=3610304 run_peak_rss_kib=131968 stop=None fleet_refusal=True window=True nx_task_lines=0 error_lines=0 moving=False`
- nx task lines: 0

`[capacity]` lines of the bootstrap log:
```
[capacity] run: sized at 4% of the default from 16 earlier run(s) of its class, at least 2384 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 3000 MB reserved (6403 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 6 earlier run(s) of its class
[capacity] dependencies: sized at 33% of the default from 16 earlier run(s) of its class, at least 2664 MB because of its assembly phase
[capacity] admitted dependencies: 1 job(s), 2664 MB reserved (6994 MB available, 2048 MB floor)
```

Last distinct lines of the bootstrap log:
```
Reading the project graph slice (280 s): flush 1 of 1.
[repo] still working after 290 s; last phase started: door.script
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 303 and delivered 0 within 901 s, 303 are still missing, among them the too heavy project(s) workspace (about 13896 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB) (this machine can give about 3512 MB per project). Nothing is wrong with your tree;
end 2026-11-15T00:12:16Z
native_rc=75
```

Service log, wanted lines (90 `arrived` lines; first `graph: waiting for the fleet: 0 of 303 projects arrived`, last `graph: waiting for the fleet: 0 of 303 projects arrived`):
```
graph: 659 stale projects, 356 taken from the fleet, 3 too heavy for this machine (workspace (about 13896 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 303
graph: waiting for the fleet: 0 of 303 projects arrived
graph: nothing to wait for any longer: the fleet was asked for 303 stale projects and delivered 0 within 901 s; 303 remain, among them the too heavy project(s) workspace (about 13896 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB), which this machine must never infer itself (it can give about 3512 MB per project); try again when a machine with free memory serves the 
2026-11-15T00:12:16.749Z graph-wanted-expired {"wanted":"303","arrived":"0","seconds":"901","missing":"303","local":"192","heavy":", among them the too heavy project(s) workspace (about 13896 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB) (this machine can give about 3512 MB per project)"}
```

Service log, last 30 lines of the try (cut at 300 characters):
```
[repository:command] still running after 3211 s
graph: waiting for the fleet: 0 of 303 projects arrived
[repository:command] still running after 3221 s
graph: waiting for the fleet: 0 of 303 projects arrived
[repo] still working after 3230 s; last phase started: source.served
[repository:command] still running after 3231 s
graph: waiting for the fleet: 0 of 303 projects arrived
[repository:command] still running after 3241 s
graph: waiting for the fleet: 0 of 303 projects arrived
[repository:command] still running after 3251 s
graph: waiting for the fleet: 0 of 303 projects arrived
[repo] still working after 3260 s; last phase started: source.served
[repository:command] still running after 3261 s
graph: waiting for the fleet: 0 of 303 projects arrived
[repository:command] still running after 3271 s
graph: waiting for the fleet: 0 of 303 projects arrived
[repository:command] still running after 3281 s
graph: waiting for the fleet: 0 of 303 projects arrived
[repo] still working after 3290 s; last phase started: source.served
[repository:command] still running after 3291 s
graph: waiting for the fleet: 0 of 303 projects arrived
[repository:command] still running after 3301 s
graph: waiting for the fleet: 0 of 303 projects arrived
[repository:command] still running after 3311 s
graph: waiting for the fleet: 0 of 303 projects arrived
[repo] still working after 3320 s; last phase started: source.served
[repository:command] still running after 3321 s
graph: waiting for the fleet: 0 of 303 projects arrived
graph: nothing to wait for any longer: the fleet was asked for 303 stale projects and delivered 0 within 901 s; 303 remain, among them the too heavy project(s) workspace (about 13896 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB), which this machin
2026-11-15T00:12:16.749Z graph-wanted-expired {"wanted":"303","arrived":"0","seconds":"901","missing":"303","local":"192","heavy":", among them the too heavy project(s) workspace (about 13896 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB) (this mac
```

## Try 3

- Log `/Users/ueli/native-bootstrap-round-11-try-3.log`: `start 2026-11-15T00:17:18Z`, `end 2026-11-15T00:29:54Z`, `native_rc=7`
- Runner: `native_rc=7 peak_rss_kib=6128128 run_peak_rss_kib=131088 stop=memory pressure: level 4 in 2, level 2 in 0 samples in a row fleet_refusal=False window=False nx_task_lines=0 error_lines=0 moving=False`
- nx task lines: 0

`[capacity]` lines of the bootstrap log:
```
[capacity] run: sized at 4% of the default from 16 earlier run(s) of its class, at least 2388 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 3004 MB reserved (7515 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 6 earlier run(s) of its class
[capacity] dependencies: sized at 33% of the default from 16 earlier run(s) of its class, at least 2676 MB because of its assembly phase
[capacity] admitted dependencies: 1 job(s), 2676 MB reserved (6277 MB available, 2048 MB floor)
```

Last distinct lines of the bootstrap log:
```
Reading the project graph slice (50 s): flush 1 of 1.
Reading the project graph slice (525 s): rederivation 256 of 256.
Reading the project graph slice (547 s): owners 0 of 659; running (name, kind, seconds, rows): @semio-tech/energy-oracle-py (project, 0, 0).
Reading the project graph slice (551 s): owners 0 of 659; running (name, kind, seconds, rows): @semio-tech/energy-oracle-py (project, 5, 0); waiting for memory headroom (17182 of 1024 MB reserved).
Reading the project graph slice (559 s): owners 1 of 659.
Reading the project graph slice (567 s): owners 21 of 659; running (name, kind, seconds, rows): @semio-tech/font-noto-emoji-nature-regular-rs (language:cargo, 1, 0).
Reading the project graph slice (569 s): owners 31 of 659; running (name, kind, seconds, rows): @semio-tech/font-noto-emoji-supplement-regular-rs (language:cargo, 0, 0), @semio-tech/font-anta-latin-regular-rs (language:cargo, 0, 0), @semio-tech/font-kelly-slab-latin-regular-rs (language:cargo, 0, 0).
Reading the project graph slice (571 s): owners 33 of 659; running (name, kind, seconds, rows): @semio-tech/font-kelly-slab-latin-regular-rs (language:cargo, 1, 0), @semio-tech/dsl-syntax-rs (language:rust, 1, 0), @semio-tech/dsl-lexical-rs (language:rust, 1, 0).
Reading the project graph slice (589 s): owners 47 of 659; running (name, kind, seconds, rows): @semio-tech/cargo-workspaces (language:typescript, 2, 0).
Reading the project graph slice (593 s): owners 47 of 659; running (name, kind, seconds, rows): .
end 2026-11-15T00:29:54Z
native_rc=7
```

Service log, wanted lines (37 `arrived` lines; first `graph: waiting for the fleet: 0 of 356 projects arrived`, last `graph: waiting for the fleet: 356 of 356 projects arrived`):
```
graph: 659 stale projects, 303 taken from the fleet, 10 too heavy for this machine (@semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), repo (about 14264 MB), @semio-tech/framework-os (about 8368 MB), @semio-tech/framework-renderer-wgpu (about 6858 MB), @semio-tech/plugin-publication-mounted-rs (about 6484 MB), and 4 more; it can give about 3512 MB per project), it can infer 192 of t
graph: waiting for the fleet: 0 of 356 projects arrived
graph: refreshing 659 projects (generation 62)
graph: 195 stale facts (analysis 133, file 34, readdir 28) leave 659 projects stale
graph: asking the host for up to 8 worker processes for 48 projects; the projects wait for them
[capacity] waiting 81s for exec: memory pressure is critical (need 1536 MB; -8037 MB spare above the 2048 MB floor, load 11.59; 3 running, 0 ahead); behind the host itself (nothing of ours)
```

Service log, last 30 lines of the try (cut at 300 characters):
```
[capacity] waiting 44s for exec: not enough free memory (need 1536 MB; -905 MB spare above the 2048 MB floor, load 11.57; 3 running, 0 ahead); behind graph-plan (running, 1 job(s), 1781 MB), graph-farm (running, 1 job(s), 1536 MB), workspace:setup (quiet, 1 job(s), 42 MB)
[repository:command] still running after 4271 s
[capacity] waiting 50s for exec: not enough free memory (need 1536 MB; -5757 MB spare above the 2048 MB floor, load 12.56; 3 running, 0 ahead); behind graph-plan (running, 1 job(s), 18511 MB), graph-farm (running, 1 job(s), 1536 MB), workspace:setup (quiet, 1 job(s), 46 MB)
[repo] still working after 4280 s; last phase started: source.served
[capacity] waiting 56s for exec: the host is swapping out too fast (need 1536 MB; -5757 MB spare above the 2048 MB floor, load 11.88; 3 running, 0 ahead); behind the host itself (nothing of ours)
[repository:command] still running after 4281 s
[capacity] waiting 61s for exec: the host is swapping out too fast (need 1536 MB; -4221 MB spare above the 2048 MB floor, load 11.56; 3 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 68s for exec: the host is swapping out too fast (need 1536 MB; -8036 MB spare above the 2048 MB floor, load 11.36; 3 running, 0 ahead); behind the host itself (nothing of ours)
[repository:command] still running after 4291 s
[capacity] waiting 74s for exec: the host is swapping out too fast (need 1536 MB; -8036 MB spare above the 2048 MB floor, load 11.81; 3 running, 0 ahead); behind the host itself (nothing of ours)
[repository:command] still running after 4304 s
[capacity] waiting 81s for exec: memory pressure is critical (need 1536 MB; -8037 MB spare above the 2048 MB floor, load 11.59; 3 running, 0 ahead); behind the host itself (nothing of ours)
[repo] still working after 4311 s; last phase started: source.served
[capacity] waiting 87s for exec: memory pressure is critical (need 1536 MB; -14513 MB spare above the 2048 MB floor, load 10.82; 3 running, 0 ahead); behind the host itself (nothing of ours)
[repository:command] still running after 4314 s
[capacity] waiting 94s for exec: memory pressure is critical (need 1536 MB; -14513 MB spare above the 2048 MB floor, load 11.75; 3 running, 0 ahead); behind the host itself (nothing of ours)
[repository:command] still running after 4324 s
[repository:command] still running after 4334 s
[repo] still working after 4341 s; last phase started: source.served
[capacity] waiting 117s for exec: memory pressure is critical (need 1536 MB; -12293 MB spare above the 2048 MB floor, load 11.53; 3 running, 0 ahead); behind the host itself (nothing of ours)
[repository:command] still running after 4344 s
[repository:command] still running after 4354 s
[capacity] waiting 129s for exec: memory pressure is critical (need 1536 MB; -12331 MB spare above the 2048 MB floor, load 11.62; 3 running, 0 ahead); behind the host itself (nothing of ours)
[repository:command] still running after 4364 s
[repo] still working after 4371 s; last phase started: source.served
[repository:command] still running after 4374 s
[repository:command] still running after 4384 s
[repository:command] still running after 4394 s
[repo] still working after 4401 s; last phase started: source.served
[repository:command] still running after 4404 s
```

## Try 4

- Log `/Users/ueli/native-bootstrap-round-11-try-4.log`: `start 2026-11-15T00:33:24Z`, `end 2026-11-15T00:40:28Z`, `native_rc=7`
- Runner: `native_rc=7 peak_rss_kib=6036816 run_peak_rss_kib=134032 stop=memory pressure: level 4 in 2, level 2 in 0 samples in a row fleet_refusal=False window=False nx_task_lines=0 error_lines=0 moving=False`
- nx task lines: 0

`[capacity]` lines of the bootstrap log:
```
[capacity] run: sized at 4% of the default from 16 earlier run(s) of its class, at least 2392 MB because of its assembly phase
[capacity] admitted run with 6 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 59% of the default from 6 earlier run(s) of its class
[capacity] dependencies: sized at 33% of the default from 16 earlier run(s) of its class, at least 2685 MB because of its assembly phase
[capacity] admitted dependencies: 1 job(s), 2685 MB reserved (8935 MB available, 2048 MB floor)
[capacity] admitted run: 1 job(s), 2048 MB reserved (9048 MB available, 2048 MB floor)
```

Last distinct lines of the bootstrap log:
```
Reading the project graph slice (324 s): owners 62 of 659; running (name, kind, seconds, rows): @semio-tech/framework-pack-json-rs (language:cargo, 12, 0), @semio-tech/framework-pack-json-source-sink-rs (language:rust, 3, 0).
Reading the project graph slice (326 s): owners 63 of 659; running (name, kind, seconds, rows): @semio-tech/framework-pack-json-rs (language:cargo, 15, 0), @semio-tech/onboarding-rs (language:rust, 2, 0), @semio-tech/kernel-media-jobs-rs (language:rust, 0, 0).
Reading the project graph slice (334 s): owners 64 of 659; running (name, kind, seconds, rows): @semio-tech/onboarding-rs (language:rust, 9, 0), @semio-tech/kernel-history-rs (language:rust, 5, 0).
Reading the project graph slice (344 s): owners 69 of 659; running (name, kind, seconds, rows): @semio-tech/framework-actor-retained-context-rs (language:rust, 2, 0), @semio-tech/framework-actor-retained-turn-rs (language:rust, 0, 0).
Reading the project graph slice (352 s): owners 70 of 659; running (name, kind, seconds, rows): @semio-tech/framework-actor-retained-context-rs (language:rust, 7, 0).
Reading the project graph slice (358 s): owners 71 of 659; running (name, kind, seconds, rows): @semio-tech/framework-actor-retained-context-rs (language:rust, 16, 0), @semio-tech/framework-actor-rs (language:cargo, 3, 0).
Reading the project graph slice (376 s): owners 71 of 659; running (name, kind, seconds, rows): @semio-tech/framework-actor-retained-context-rs (language:rust, 35, 0), @semio-tech/framework-actor-rs (language:cargo, 22, 0); waiting for memory headroom (2087 of -10395 MB reserved).
Reading the project graph slice (380 s): owners 71 of 659; running (name, kind, seconds, rows): @semio-tech/framework-actor-rs (language:cargo, 24, 0).
Reading the project graph slice (382 s): owners 71 of 659; running (name, kind, seconds, rows): @semio-tech/framework-actor-rs (language:cargo, 27, 0); waiting for memory headroom (4485 of -8634 MB reserved).
Reading the project graph slice (393 s): owners 71 of 659; running (name, kind, seconds, rows): @semio-tech/framework-actor-rs (language:cargo, 38, 0), @semio-tech/framework-actor-pack-rs (language:rust, 6, 0); waiting for memory headroom (5759 of -7481 MB reserved).
end 2026-11-15T00:40:28Z
native_rc=7
```

Service log, wanted lines (2 `arrived` lines; first `graph: waiting for the fleet: 0 of 303 projects arrived`, last `graph: waiting for the fleet: 303 of 303 projects arrived`):
```
graph: 659 stale projects, 356 taken from the fleet, 3 too heavy for this machine (workspace (about 13896 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 303
graph: waiting for the fleet: 0 of 303 projects arrived
graph: refreshing 659 projects (generation 62)
graph: 195 stale facts (analysis 133, file 34, readdir 28) leave 659 projects stale
graph: asking the host for up to 8 worker processes for 48 projects; the projects wait for them
[capacity] waiting 181s for exec: memory pressure is critical (need 1536 MB; -9365 MB spare above the 2048 MB floor, load 13.89; 3 running, 0 ahead); behind the host itself (nothing of ours)
```

Service log, last 30 lines of the try (cut at 300 characters):
```
[script] build:projects · Inferring projects: 26 of 192 projects · 1 running, longest 25 s: @semio-tech/framework-actor-rs (language:cargo, 25 s, 0 rows) · waiting for memory headroom (4485 of 0 MB reserved)
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:71:659:1:25:%40semio-tech%2Fframework-actor-rs|language%3Acargo|25|0:waiting|4485|0|","mode":"shared","elapsedMs":0}}
[repository:command] still running after 5004 s
[capacity] waiting 207s for exec: memory pressure is critical (need 1536 MB; -11905 MB spare above the 2048 MB floor, load 10.1; 3 running, 0 ahead); behind the host itself (nothing of ours)
graph: waiting for memory headroom (5251 of -8635 MB reserved, 1 projects running)
[script] {"owner":"repository:build","command":"build:projects","stage":"running","activity":{"resource":"build:graph-phase:inference:26:192:1:30:%40semio-tech%2Fframework-actor-rs|language%3Acargo|30|0:waiting|5251|0|","mode":"shared","elapsedMs":0}}
[script] build:projects · Inferring projects: 26 of 192 projects · 1 running, longest 30 s: @semio-tech/framework-actor-rs (language:cargo, 30 s, 0 rows) · waiting for memory headroom (5251 of 0 MB reserved)
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:71:659:1:30:%40semio-tech%2Fframework-actor-rs|language%3Acargo|30|0:waiting|5251|0|","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"build:projects","stage":"running","activity":{"resource":"build:graph-phase:inference:26:192:2:30:%40semio-tech%2Fframework-actor-rs|language%3Acargo|30|0,%40semio-tech%2Fframework-actor-pack-rs|language%3Arust|0|0","mode":"shared","elapsedMs":0}}
[script] build:projects · Inferring projects: 26 of 192 projects · 2 running, longest 30 s: @semio-tech/framework-actor-rs (language:cargo, 30 s, 0 rows), @semio-tech/framework-actor-pack-rs (language:rust, 0 s, 0 rows)
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:71:659:2:30:%40semio-tech%2Fframework-actor-rs|language%3Acargo|30|0,%40semio-tech%2Fframework-actor-pack-rs|language%3Arust|0|0","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"build:projects","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[capacity] waiting 212s for exec: memory pressure is critical (need 1536 MB; -11905 MB spare above the 2048 MB floor, load 9.53; 3 running, 0 ahead); behind the host itself (nothing of ours)
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
graph: waiting for memory headroom (5759 of -7481 MB reserved, 1 projects running)
[script] {"owner":"repository:build","command":"build:projects","stage":"running","activity":{"resource":"build:graph-phase:inference:26:192:2:35:%40semio-tech%2Fframework-actor-rs|language%3Acargo|35|0,%40semio-tech%2Fframework-actor-pack-rs|language%3Arust|5|0:waiting|5759|0|","mode":"shared","ela
[script] build:projects · Inferring projects: 26 of 192 projects · 2 running, longest 35 s: @semio-tech/framework-actor-rs (language:cargo, 35 s, 0 rows), @semio-tech/framework-actor-pack-rs (language:rust, 5 s, 0 rows) · waiting for memory headroom (5759 of 0 MB reserved)
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:71:659:2:35:%40semio-tech%2Fframework-actor-rs|language%3Acargo|35|0,%40semio-tech%2Fframework-actor-pack-rs|language%3Arust|5|0:waiting|5759|0|","mode":"shared","el
[repository:command] still running after 5015 s
[capacity] waiting 218s for exec: memory pressure is critical (need 1536 MB; -14520 MB spare above the 2048 MB floor, load 8.85; 3 running, 0 ahead); behind the host itself (nothing of ours)
[repository:command] still running after 5025 s
[repo] still working after 5032 s; last phase started: source.served
[repository:command] still running after 5035 s
[repository:command] still running after 5045 s
```

## Try 5

- Log `/Users/ueli/native-bootstrap-round-11-try-5.log`: `start 2026-11-15T00:43:58Z`, `end 2026-11-15T00:56:36Z`, `native_rc=7`
- Runner: `native_rc=7 peak_rss_kib=5170176 run_peak_rss_kib=131296 stop=memory pressure: level 4 in 0, level 2 in 6 samples in a row fleet_refusal=False window=False nx_task_lines=0 error_lines=0 moving=False`
- nx task lines: 0

`[capacity]` lines of the bootstrap log:
```
[capacity] run: sized at 4% of the default from 16 earlier run(s) of its class, at least 2394 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 3017 MB reserved (9430 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 6 earlier run(s) of its class
[capacity] dependencies: sized at 33% of the default from 16 earlier run(s) of its class, at least 2691 MB because of its assembly phase
[capacity] admitted dependencies: 1 job(s), 2691 MB reserved (9725 MB available, 2048 MB floor)
```

Last distinct lines of the bootstrap log:
```
Reading the project graph slice (44 s): parts 5908 of 14279.
Reading the project graph slice (48 s): flush 1 of 1.
Reading the project graph slice (50 s): rederivation 252 of 252.
Reading the project graph slice (404 s): directories 33 of 33.
Reading the project graph slice (438 s): owners 0 of 5.
Reading the project graph slice (448 s): owners 0 of 5; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 0, 0).
Reading the project graph slice (452 s): owners 0 of 5; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 5, 0); waiting for memory headroom (1186 of 1024 MB reserved).
Reading the project graph slice (517 s): owners 1 of 5; running (name, kind, seconds, rows): workspace (root:language:cargo, 4, 0).
Reading the project graph slice (521 s): owners 1 of 5; running (name, kind, seconds, rows): workspace (root:language:cargo, 9, 0); waiting for memory headroom (1300 of 44 MB reserved).
Reading the project graph slice (537 s): owners 2 of 5; running (name, kind, seconds, rows): @semio-tech/framework-actor-rs (language:cargo, 2, 0).
end 2026-11-15T00:56:36Z
native_rc=7
```

Service log, wanted lines (30 `arrived` lines; first `graph: waiting for the fleet: 0 of 354 projects arrived`, last `graph: waiting for the fleet: 354 of 354 projects arrived`):
```
graph: 610 stale projects, 256 taken from the fleet, 10 too heavy for this machine (@semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), repo (about 14264 MB), @semio-tech/framework-os (about 8368 MB), @semio-tech/framework-renderer-wgpu (about 6858 MB), @semio-tech/plugin-publication-mounted-rs (about 6484 MB), and 4 more; it can give about 3512 MB per project), it can infer 192 of t
graph: waiting for the fleet: 0 of 354 projects arrived
graph: generation 68 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 8 s
graph: refreshing 5 projects (generation 69)
graph: 9 stale facts (directoryExists 3, exists 3, readdir 3) leave 5 projects stale
```

Service log, last 30 lines of the try (cut at 300 characters):
```
[repository:command] still running after 5815 s
[script] {"owner":"repository:build","command":"build:modules","stage":"running"}
[repository:command] still running after 5825 s
[repository:command] still running after 5835 s
[script] {"owner":"repository:build","command":"build:modules","stage":"running"}
[repo] still working after 5842 s; last phase started: source.served
[script] {"owner":"repository:build","command":"build:graph","stage":"complete"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:5:5","mode":"shared","elapsedMs":0}}
[repository:command] still running after 5845 s
[repository:command] still running after 5855 s
[repository:command] still running after 5865 s
[repo] still working after 5872 s; last phase started: source.served
[repository:command] still running after 5875 s
[repository:command] still running after 5885 s
[repository:command] still running after 5895 s
[repo] still working after 5902 s; last phase started: source.served
[repository:command] still running after 5905 s
[repository:command] still running after 5915 s
[repository:command] still running after 5925 s
[repo] still working after 5932 s; last phase started: source.served
[repository:command] still running after 5935 s
[repository:command] still running after 5945 s
[repository:command] still running after 5955 s
[repo] still working after 5962 s; last phase started: source.served
[repository:command] still running after 5965 s
[repository:command] still running after 5975 s
[repository:command] still running after 5985 s
[repo] still working after 5992 s; last phase started: source.served
[repository:command] still running after 5995 s
[repository:command] still running after 6005 s
```

## Try 6

- Log `/Users/ueli/native-bootstrap-round-11-try-6.log`: `start 2026-11-15T01:00:06Z`, `end 2026-11-15T01:03:52Z`, `native_rc=1`
- Runner: `native_rc=1 peak_rss_kib=5018432 run_peak_rss_kib=130896 stop=None fleet_refusal=False window=False nx_task_lines=0 error_lines=0 moving=False`
- nx task lines: 0

`[capacity]` lines of the bootstrap log:
```
[capacity] run: sized at 4% of the default from 16 earlier run(s) of its class, at least 2397 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 3020 MB reserved (8807 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 6 earlier run(s) of its class
[capacity] dependencies: sized at 33% of the default from 16 earlier run(s) of its class, at least 2700 MB because of its assembly phase
[capacity] admitted dependencies: 1 job(s), 2700 MB reserved (9438 MB available, 2048 MB floor)
```

Last distinct lines of the bootstrap log:
```
Reading the project graph slice (44 s): facts 49152 of 276157.
Reading the project graph slice (62 s): verification 1 of 7.
Reading the project graph slice (98 s): flush 1 of 1.
Reading the project graph slice (106 s): owners 0 of 5; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 0, 0).
Reading the project graph slice (110 s): owners 0 of 5; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 5, 0); waiting for memory headroom (1756 of 1024 MB reserved).
Reading the project graph slice (140 s): owners 1 of 5.
Reading the project graph slice (142 s): owners 1 of 5; running (name, kind, seconds, rows): workspace (root:language:cargo, 1, 0).
Reading the project graph slice (146 s): owners 1 of 5; running (name, kind, seconds, rows): workspace (root:language:cargo, 6, 0); waiting for memory headroom (1529 of 663 MB reserved).
Reading the project graph slice (199 s): assembly 0 of 703.
The resident service met an unexpected failure (reference 21c18fad); try again, the details under that reference are in the service log /var/folders/1s/pfk8j2p90t94v1nx_gf8zq3w0000gn/T/semio-repo/501/host-ac6bfe8f337c.log.
end 2026-11-15T01:03:52Z
native_rc=1
```

Service log, wanted lines (0 `arrived` lines; first `-`, last `-`):
```
graph: generation 74 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 8 s
graph: refreshing 5 projects (generation 75)
graph: 14 stale facts (readdir 8, directoryExists 3, exists 3) leave 5 projects stale
graph: 7 derived results asked again after 14 stale facts: 5 unchanged, 0 changed
2026-11-15T01:03:52.870Z request-failed 21c18fad Error: ENOENT: no such file or directory, lstat '/Users/ueli/Documents/semio/.cache/b/g/m/build-input-closures/0f4fa61aa5e5ca61388e839d9a7ffcc74aadca621f8734b6dc8542eb06aec6e3.json'
```

Service log, last 30 lines of the try (cut at 300 characters):
```
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:0:170","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 0 of 170 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:128:170","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 128 of 170 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:170:170","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 170 of 170 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"complete"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:assembly:33:703","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Assembling the graph: 33 of 703 projects
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:0:167","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 0 of 167 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:167:167","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 167 of 167 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"complete"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:assembly:96:703","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Assembling the graph: 96 of 703 projects
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:0:341","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 0 of 341 closures
2026-11-15T01:03:52.870Z request-failed 21c18fad Error: ENOENT: no such file or directory, lstat '/Users/ueli/Documents/semio/.cache/b/g/m/build-input-closures/0f4fa61aa5e5ca61388e839d9a7ffcc74aadca621f8734b6dc8542eb06aec6e3.json'
    at lstatSync (unknown)
    at readInventoryRecordLeanV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🪶️lean-inventory/🟨️.mjs:12:48)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📁️workspace/🟦️.ts:86:20)
    at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs:146:491)
    at processTicksAndRejections (native:7:39)
[repository:command] still running after 6426 s
[repository:command] still running after 6436 s
```

## Try 7

- Log `/Users/ueli/native-bootstrap-round-11-try-7.log`: `start 2026-11-15T01:06:08Z`, `end 2026-11-15T01:08:50Z`, `native_rc=1`
- Runner: `native_rc=1 peak_rss_kib=3336320 run_peak_rss_kib=371248 stop=None fleet_refusal=False window=False nx_task_lines=0 error_lines=0 moving=False`
- nx task lines: 0

`[capacity]` lines of the bootstrap log:
```
[capacity] run: sized at 4% of the default from 16 earlier run(s) of its class, at least 2398 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 3021 MB reserved (8025 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 6 earlier run(s) of its class
[capacity] dependencies: sized at 33% of the default from 16 earlier run(s) of its class, at least 2703 MB because of its assembly phase
[capacity] admitted dependencies: 1 job(s), 2703 MB reserved (7036 MB available, 2048 MB floor)
```

Last distinct lines of the bootstrap log:
```
Reading the project graph slice (2 s).
[repo] still working after 5 s; last phase started: door.script
Reading the project graph slice (48 s): files 0 of 0.
Reading the project graph slice (52 s): doubt 455914 of 455914.
Reading the project graph slice (54 s): facts 4096 of 111842.
Reading the project graph slice (76 s): owners 0 of 1.
Reading the project graph slice (86 s): owners 0 of 1; running (name, kind, seconds, rows): workspace (root:language:cargo, 0, 0).
Reading the project graph slice (104 s): owners 0 of 1; running (name, kind, seconds, rows): .
Reading the project graph slice (128 s): assembly 0 of 703.
The resident service met an unexpected failure (reference 21c18fad); try again, the details under that reference are in the service log /var/folders/1s/pfk8j2p90t94v1nx_gf8zq3w0000gn/T/semio-repo/501/host-ac6bfe8f337c.log.
end 2026-11-15T01:08:50Z
native_rc=1
```

Service log, wanted lines (0 `arrived` lines; first `-`, last `-`):
```
graph: generation 74 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 7 s
graph: refreshing 1 projects (generation 75)
2026-11-15T01:08:50.675Z request-failed 21c18fad Error: ENOENT: no such file or directory, lstat '/Users/ueli/Documents/semio/.cache/b/g/m/build-input-closures/0f4fa61aa5e5ca61388e839d9a7ffcc74aadca621f8734b6dc8542eb06aec6e3.json'
```

Service log, last 30 lines of the try (cut at 300 characters):
```
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:0:170","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 0 of 170 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:170:170","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 170 of 170 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"complete"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:assembly:33:703","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Assembling the graph: 33 of 703 projects
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:0:167","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 0 of 167 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:128:167","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 128 of 167 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:167:167","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 167 of 167 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"complete"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:assembly:65:703","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Assembling the graph: 65 of 703 projects
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:0:341","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 0 of 341 closures
2026-11-15T01:08:50.675Z request-failed 21c18fad Error: ENOENT: no such file or directory, lstat '/Users/ueli/Documents/semio/.cache/b/g/m/build-input-closures/0f4fa61aa5e5ca61388e839d9a7ffcc74aadca621f8734b6dc8542eb06aec6e3.json'
    at lstatSync (unknown)
    at readInventoryRecordLeanV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🪶️lean-inventory/🟨️.mjs:12:48)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📁️workspace/🟦️.ts:86:20)
    at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs:146:491)
    at processTicksAndRejections (native:7:39)
[repository:command] still running after 6726 s
[repository:command] still running after 6736 s
```

## Try 8

- Log `/Users/ueli/native-bootstrap-round-11-try-8.log`: `start 2026-11-15T01:11:10Z`, `end 2026-11-15T01:13:42Z`, `native_rc=1`
- Runner: `native_rc=1 peak_rss_kib=3411040 run_peak_rss_kib=368496 stop=None fleet_refusal=False window=False nx_task_lines=0 error_lines=0 moving=False`
- nx task lines: 0

`[capacity]` lines of the bootstrap log:
```
[capacity] run: sized at 4% of the default from 16 earlier run(s) of its class, at least 2399 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 3022 MB reserved (8476 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 6 earlier run(s) of its class
[capacity] dependencies: sized at 33% of the default from 16 earlier run(s) of its class, at least 2706 MB because of its assembly phase
[capacity] admitted dependencies: 1 job(s), 2706 MB reserved (8115 MB available, 2048 MB floor)
```

Last distinct lines of the bootstrap log:
```
Planning 1 targets.
Reading the project graph slice (2 s).
[repo] still working after 5 s; last phase started: door.script
Reading the project graph slice (44 s): directories 67 of 67.
Reading the project graph slice (46 s): doubt 455914 of 455914.
Reading the project graph slice (54 s): facts 4096 of 111842.
Reading the project graph slice (72 s): owners 0 of 1.
Reading the project graph slice (78 s): owners 0 of 1; running (name, kind, seconds, rows): workspace (root:language:cargo, 0, 0).
Reading the project graph slice (100 s): owners 0 of 1; running (name, kind, seconds, rows): .
The resident service met an unexpected failure (reference 21c18fad); try again, the details under that reference are in the service log /var/folders/1s/pfk8j2p90t94v1nx_gf8zq3w0000gn/T/semio-repo/501/host-ac6bfe8f337c.log.
end 2026-11-15T01:13:42Z
native_rc=1
```

Service log, wanted lines (0 `arrived` lines; first `-`, last `-`):
```
graph: generation 74 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 9 s
graph: refreshing 1 projects (generation 75)
2026-11-15T01:13:42.058Z request-failed 21c18fad Error: ENOENT: no such file or directory, lstat '/Users/ueli/Documents/semio/.cache/b/g/m/build-input-closures/0f4fa61aa5e5ca61388e839d9a7ffcc74aadca621f8734b6dc8542eb06aec6e3.json'
```

Service log, last 30 lines of the try (cut at 300 characters):
```
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:0:170","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 0 of 170 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:170:170","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 170 of 170 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"complete"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:assembly:33:703","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Assembling the graph: 33 of 703 projects
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:0:167","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 0 of 167 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:167:167","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 167 of 167 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"complete"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:assembly:65:703","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Assembling the graph: 65 of 703 projects
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:0:341","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 0 of 341 closures
2026-11-15T01:13:42.058Z request-failed 21c18fad Error: ENOENT: no such file or directory, lstat '/Users/ueli/Documents/semio/.cache/b/g/m/build-input-closures/0f4fa61aa5e5ca61388e839d9a7ffcc74aadca621f8734b6dc8542eb06aec6e3.json'
    at lstatSync (unknown)
    at readInventoryRecordLeanV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🪶️lean-inventory/🟨️.mjs:12:48)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📁️workspace/🟦️.ts:86:20)
    at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs:146:491)
    at processTicksAndRejections (native:7:39)
[repository:command] still running after 7016 s
[repository:command] still running after 7026 s
[repository:command] still running after 7036 s
[repo] still working after 7042 s; last phase started: source.served
```

## Try 9

- Log `/Users/ueli/native-bootstrap-round-11-try-9.log`: `start 2026-11-15T01:16:12Z`, `end 2026-11-15T01:18:34Z`, `native_rc=1`
- Runner: `native_rc=1 peak_rss_kib=2870992 run_peak_rss_kib=124416 stop=None fleet_refusal=False window=False nx_task_lines=0 error_lines=0 moving=False`
- nx task lines: 0

`[capacity]` lines of the bootstrap log:
```
[capacity] run: sized at 4% of the default from 16 earlier run(s) of its class, at least 2400 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 3023 MB reserved (6929 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 6 earlier run(s) of its class
[capacity] dependencies: sized at 28% of the default from 16 earlier run(s) of its class, at least 2709 MB because of its assembly phase
[capacity] admitted dependencies: 1 job(s), 2709 MB reserved (6466 MB available, 2048 MB floor)
```

Last distinct lines of the bootstrap log:
```
Planning 1 targets.
Reading the project graph slice (2 s).
[repo] still working after 5 s; last phase started: door.script
Reading the project graph slice (38 s): files 0 of 0.
Reading the project graph slice (42 s): doubt 455914 of 455914.
Reading the project graph slice (44 s): facts 4096 of 111842.
Reading the project graph slice (60 s): owners 0 of 1.
Reading the project graph slice (70 s): owners 0 of 1; running (name, kind, seconds, rows): workspace (root:language:cargo, 0, 0).
Reading the project graph slice (92 s): owners 0 of 1; running (name, kind, seconds, rows): .
The resident service met an unexpected failure (reference 21c18fad); try again, the details under that reference are in the service log /var/folders/1s/pfk8j2p90t94v1nx_gf8zq3w0000gn/T/semio-repo/501/host-ac6bfe8f337c.log.
end 2026-11-15T01:18:34Z
native_rc=1
```

Service log, wanted lines (0 `arrived` lines; first `-`, last `-`):
```
graph: generation 74 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 8 s
graph: refreshing 1 projects (generation 75)
2026-11-15T01:18:34.198Z request-failed 21c18fad Error: ENOENT: no such file or directory, lstat '/Users/ueli/Documents/semio/.cache/b/g/m/build-input-closures/0f4fa61aa5e5ca61388e839d9a7ffcc74aadca621f8734b6dc8542eb06aec6e3.json'
```

Service log, last 30 lines of the try (cut at 300 characters):
```
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:0:170","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 0 of 170 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:128:170","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 128 of 170 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:170:170","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 170 of 170 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"complete"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:assembly:33:703","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Assembling the graph: 33 of 703 projects
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:0:167","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 0 of 167 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:167:167","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 167 of 167 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"complete"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:assembly:65:703","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Assembling the graph: 65 of 703 projects
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:0:341","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 0 of 341 closures
2026-11-15T01:18:34.198Z request-failed 21c18fad Error: ENOENT: no such file or directory, lstat '/Users/ueli/Documents/semio/.cache/b/g/m/build-input-closures/0f4fa61aa5e5ca61388e839d9a7ffcc74aadca621f8734b6dc8542eb06aec6e3.json'
    at lstatSync (unknown)
    at readInventoryRecordLeanV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🪶️lean-inventory/🟨️.mjs:12:48)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📁️workspace/🟦️.ts:86:20)
    at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs:146:491)
    at processTicksAndRejections (native:7:39)
[repository:command] still running after 7306 s
[repo] still working after 7312 s; last phase started: source.served
```

## Try 10

- Log `/Users/ueli/native-bootstrap-round-11-try-10.log`: `start 2026-11-15T01:20:44Z`, `end 2026-11-15T01:22:53Z`, `native_rc=1`
- Runner: `native_rc=1 peak_rss_kib=3061888 run_peak_rss_kib=358240 stop=None fleet_refusal=False window=False nx_task_lines=0 error_lines=0 moving=False`
- nx task lines: 0

`[capacity]` lines of the bootstrap log:
```
[capacity] run: sized at 4% of the default from 16 earlier run(s) of its class, at least 2401 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 3024 MB reserved (7892 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 6 earlier run(s) of its class
[capacity] dependencies: sized at 28% of the default from 16 earlier run(s) of its class, at least 2711 MB because of its assembly phase
[capacity] admitted dependencies: 1 job(s), 2711 MB reserved (6940 MB available, 2048 MB floor)
```

Last distinct lines of the bootstrap log:
```
Planning 1 targets.
Reading the project graph slice (2 s).
[repo] still working after 5 s; last phase started: door.script
Reading the project graph slice (56 s): files 0 of 0.
Reading the project graph slice (58 s): doubt 455914 of 455914.
Reading the project graph slice (60 s): facts 36864 of 111842.
Reading the project graph slice (66 s): owners 0 of 1.
Reading the project graph slice (68 s): owners 0 of 1; running (name, kind, seconds, rows): workspace (root:language:cargo, 0, 0).
Reading the project graph slice (82 s): owners 0 of 1; running (name, kind, seconds, rows): .
The resident service met an unexpected failure (reference 29bbd28b); try again, the details under that reference are in the service log /var/folders/1s/pfk8j2p90t94v1nx_gf8zq3w0000gn/T/semio-repo/501/host-ac6bfe8f337c.log.
end 2026-11-15T01:22:53Z
native_rc=1
```

Service log, wanted lines (0 `arrived` lines; first `-`, last `-`):
```
graph: generation 74 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 24 s
graph: refreshing 1 projects (generation 75)
2026-11-15T01:22:53.522Z request-failed 29bbd28b Error
```

Service log, last 30 lines of the try (cut at 300 characters):
```
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:assembly:0:703","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Assembling the graph: 0 of 703 projects
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:0:170","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 0 of 170 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:170:170","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 170 of 170 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"complete"}
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:0:167","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 0 of 167 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:167:167","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 167 of 167 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"complete"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:assembly:65:703","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Assembling the graph: 65 of 703 projects
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:0:341","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 0 of 341 closures
2026-11-15T01:22:53.522Z request-failed 29bbd28b Error
    at lstatSync (unknown)
    at readInventoryRecordLeanV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🪶️lean-inventory/🟨️.mjs:12:48)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📁️workspace/🟦️.ts:86:20)
    at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs:146:491)
    at processTicksAndRejections (unknown:7:39)
[repository:command] still running after 7566 s
[repository:command] still running after 7576 s
[repo] still working after 7582 s; last phase started: source.served
```

## Try 11

- Log `/Users/ueli/native-bootstrap-round-11-try-11.log`: `start 2026-11-15T01:25:15Z`, `end 2026-11-15T01:26:55Z`, `native_rc=1`
- Runner: `native_rc=1 peak_rss_kib=2700304 run_peak_rss_kib=134528 stop=None fleet_refusal=False window=False nx_task_lines=0 error_lines=0 moving=False`
- nx task lines: 0

`[capacity]` lines of the bootstrap log:
```
[capacity] run: sized at 4% of the default from 16 earlier run(s) of its class, at least 2401 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 3024 MB reserved (8435 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 6 earlier run(s) of its class
[capacity] dependencies: sized at 28% of the default from 16 earlier run(s) of its class, at least 2714 MB because of its assembly phase
[capacity] admitted dependencies: 1 job(s), 2714 MB reserved (7127 MB available, 2048 MB floor)
```

Last distinct lines of the bootstrap log:
```
Planning 1 targets.
Reading the project graph slice (2 s).
[repo] still working after 5 s; last phase started: door.script
Reading the project graph slice (40 s): files 0 of 0.
Reading the project graph slice (42 s): doubt 455914 of 455914.
Reading the project graph slice (44 s): facts 4096 of 111842.
Reading the project graph slice (50 s): owners 0 of 1.
Reading the project graph slice (56 s): owners 0 of 1; running (name, kind, seconds, rows): workspace (root:language:cargo, 0, 0).
Reading the project graph slice (66 s): owners 0 of 1; running (name, kind, seconds, rows): .
The resident service met an unexpected failure (reference 21c18fad); try again, the details under that reference are in the service log /var/folders/1s/pfk8j2p90t94v1nx_gf8zq3w0000gn/T/semio-repo/501/host-ac6bfe8f337c.log.
end 2026-11-15T01:26:55Z
native_rc=1
```

Service log, wanted lines (0 `arrived` lines; first `-`, last `-`):
```
graph: generation 74 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 23 s
graph: refreshing 1 projects (generation 75)
2026-11-15T01:26:55.553Z request-failed 21c18fad Error: ENOENT: no such file or directory, lstat '/Users/ueli/Documents/semio/.cache/b/g/m/build-input-closures/0f4fa61aa5e5ca61388e839d9a7ffcc74aadca621f8734b6dc8542eb06aec6e3.json'
```

Service log, last 30 lines of the try (cut at 300 characters):
```
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:assembly:0:703","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Assembling the graph: 0 of 703 projects
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:0:170","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 0 of 170 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:170:170","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 170 of 170 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"complete"}
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:0:167","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 0 of 167 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:167:167","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 167 of 167 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"complete"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:assembly:65:703","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Assembling the graph: 65 of 703 projects
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:0:341","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 0 of 341 closures
2026-11-15T01:26:55.553Z request-failed 21c18fad Error: ENOENT: no such file or directory, lstat '/Users/ueli/Documents/semio/.cache/b/g/m/build-input-closures/0f4fa61aa5e5ca61388e839d9a7ffcc74aadca621f8734b6dc8542eb06aec6e3.json'
    at lstatSync (unknown)
    at readInventoryRecordLeanV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🪶️lean-inventory/🟨️.mjs:12:48)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📁️workspace/🟦️.ts:86:20)
    at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs:146:491)
    at processTicksAndRejections (native:7:39)
[repository:command] still running after 7806 s
[repository:command] still running after 7816 s
[repo] still working after 7822 s; last phase started: source.served
```

