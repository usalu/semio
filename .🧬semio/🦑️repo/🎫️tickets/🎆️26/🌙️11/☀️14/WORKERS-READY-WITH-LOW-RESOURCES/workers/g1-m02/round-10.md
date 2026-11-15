# g1-m02 Round 10

round: 10

Result: not ready. Two tries, no nx task ran (succeeded: none, failed: none, not run: all). Try 1 ended with the fleet refusal `native_rc=75`; try 2 was ended by me with one SIGINT when round 11 was published (`native_rc=7`), while the fleet had delivered 356 of 367 wanted projects. No `native_rc=4`, no `fstatat failed (EPERM)`, no line about a fallback to the caller's access, so there is no `where: root=…` line to copy. `.cache/f/native-bootstrap.done` does not exist.

## Machine

Mac mini `Mac16,10`, Apple M4, 10 cores, 16 GB (17179869184 bytes), macOS 27.2.1 (26B131). Free disk: not measured in this round (105 GiB of 228 GiB right before it).

## Tries

| Try | Log | Start (UTC) | End (UTC) | `native_rc` | Largest process on the machine | Largest in the run's group | Fleet |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `~/native-bootstrap-round-10.log` | 23:12:14 | 23:42:25 | 75 | 3848960 KiB | 433360 KiB | 659 stale; 196, then 356 taken from the fleet; then asked for 303 and delivered 0 within 902 s |
| 2 | `~/native-bootstrap-round-10-try-2.log` | 23:47:48 | 23:52:49 | 7 (my SIGINT, round 11 published) | 3113728 KiB | 136096 KiB | 659 stale, 292 taken from the fleet; asked for 367, 356 of 367 arrived when it was ended |

- Pressure level 1 in all 72 samples (30 s apart); no hard stop.
- After try 1 my runner waited 5 minutes and did not count the try, because the refusal said "delivered 0 within": it took that for the group server move window. Whether the group server was really being moved then was not measured; count the try if it was not.
- Try 2 was close to a complete graph (356 of 367) when round 11 arrived; the SIGINT followed the rule "If an earlier round is still running: one SIGINT".

## Projects Named as Too Heavy

- Try 1, first wave (service log): `@semio-tech/repo-lib` (about 27770 MB), `@semio-tech/framework-os-dev` (about 8452 MB), `@semio-tech/framework-os` (about 8368 MB), `@semio-tech/print` (about 3628 MB).
- Try 1, refusal text: `workspace` (about 13780 MB), `@semio-tech/framework-plugin` (about 8626 MB), `@semio-tech/framework-os-kernel` (about 6984 MB).
- Try 2 (service log, 9 named, the line lists 6 and "3 more"): `@semio-tech/framework-os-dev` (about 76010 MB), `@semio-tech/repo-lib` (about 27770 MB), `repo` (about 14264 MB), `@semio-tech/framework-os` (about 8368 MB), `@semio-tech/framework-renderer-wgpu` (about 6858 MB), `semio-framework-job-native-turn` (about 6196 MB).

This machine can give about 3512 MB per project and can infer 192 of the stale projects (48 per wave).

## Launcher

Unchanged before and after both tries and between them: target `versions/cf25f2b751f83336/repo`, inode 3679486, mtime 2026-11-14T22:03:22 local time, size 6523360, `Identifier=tech.semio.repo`, `CDHash=6de7547a8b8ff384bbfbfc45e962462063ff0357`.

## Before the Run (2026-11-14T23:12Z)

- Replica agent: `launchctl print gui/501/tech.semio.fleet.replica` shows `state = running`, `runs = 22` (20 before round 9); not down, so I started it 0 times in this round. `.cache/f/health.json` was 4 s old.
- Replica code digest (`.cache/f/health.json`): `eabbc92267329dbd39d8d05e0c8eceb1bcad083c0eea58c44805408a7d547bcc`
- `bootVersion` (`~/Library/Application Support/semio/fleet/entry/entry.json`): 6
- Round 9 try 3 was ended with one SIGINT at 23:10:59Z (see `round-9.md`); round 10 try 1 started right after its report data was saved.

## Try 1

- Log `/Users/ueli/native-bootstrap-round-10.log`: `start 2026-11-14T23:12:14Z`, `end 2026-11-14T23:42:25Z`, `native_rc=75`
- Runner: `native_rc=75 peak_rss_kib=3848960 run_peak_rss_kib=433360 stop=None fleet_refusal=True window=True nx_task_lines=0 error_lines=0 moving=False`
- nx task lines: 0

`[capacity]` lines of the bootstrap log:
```
[capacity] run: sized at 4% of the default from 16 earlier run(s) of its class, at least 2376 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 2992 MB reserved (7052 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 6 earlier run(s) of its class
[capacity] dependencies: sized at 33% of the default from 16 earlier run(s) of its class, at least 2640 MB because of its assembly phase
[capacity] admitted dependencies: 1 job(s), 2640 MB reserved (7085 MB available, 2048 MB floor)
```

Last distinct lines of the bootstrap log:
```
Reading the project graph slice (220 s): flush 1 of 1.
[repo] still working after 800 s; last phase started: door.script
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 303 and delivered 0 within 902 s, 303 are still missing, among them the too heavy project(s) workspace (about 13780 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB) (this machine can give about 3512 MB per project). Nothing is wrong with your tree;
end 2026-11-14T23:42:25Z
native_rc=75
```

Service log, wanted lines (126 `arrived` lines; first `graph: waiting for the fleet: 0 of 463 projects arrived`, last `graph: waiting for the fleet: 0 of 303 projects arrived`):
```
graph: 659 stale projects, 196 taken from the fleet, 4 too heavy for this machine (@semio-tech/repo-lib (about 27770 MB), @semio-tech/framework-os-dev (about 8452 MB), @semio-tech/framework-os (about 8368 MB), @semio-tech/print (about 3628 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 463
graph: waiting for the fleet: 0 of 463 projects arrived
graph: 659 stale projects, 356 taken from the fleet, 3 too heavy for this machine (workspace (about 13780 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 303
graph: nothing to wait for any longer: the fleet was asked for 303 stale projects and delivered 0 within 902 s; 303 remain, among them the too heavy project(s) workspace (about 13780 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB), which this machine must never infer itself (it can give about 3512 MB per project); try again when a machine with free memory serves the 
2026-11-14T23:42:25.976Z graph-wanted-expired {"wanted":"303","arrived":"0","seconds":"902","missing":"303","local":"192","heavy":", among them the too heavy project(s) workspace (about 13780 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB) (this machine can give about 3512 MB per project)"}
```

Service log, last 30 lines of the try (cut at 300 characters):
```
graph: waiting for the fleet: 0 of 303 projects arrived
[repository:command] still running after 1430 s
graph: waiting for the fleet: 0 of 303 projects arrived
[repository:command] still running after 1440 s
graph: waiting for the fleet: 0 of 303 projects arrived
[repository:command] still running after 1450 s
[repo] still working after 1460 s; last phase started: source.served
graph: waiting for the fleet: 0 of 303 projects arrived
[repository:command] still running after 1460 s
graph: waiting for the fleet: 0 of 303 projects arrived
[repository:command] still running after 1470 s
graph: waiting for the fleet: 0 of 303 projects arrived
[repository:command] still running after 1480 s
[repo] still working after 1490 s; last phase started: source.served
graph: waiting for the fleet: 0 of 303 projects arrived
[repository:command] still running after 1490 s
graph: waiting for the fleet: 0 of 303 projects arrived
[repository:command] still running after 1500 s
graph: waiting for the fleet: 0 of 303 projects arrived
[repository:command] still running after 1510 s
[repo] still working after 1520 s; last phase started: source.served
graph: waiting for the fleet: 0 of 303 projects arrived
[repository:command] still running after 1520 s
graph: waiting for the fleet: 0 of 303 projects arrived
[repository:command] still running after 1530 s
graph: nothing to wait for any longer: the fleet was asked for 303 stale projects and delivered 0 within 902 s; 303 remain, among them the too heavy project(s) workspace (about 13780 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB), which this machin
2026-11-14T23:42:25.976Z graph-wanted-expired {"wanted":"303","arrived":"0","seconds":"902","missing":"303","local":"192","heavy":", among them the too heavy project(s) workspace (about 13780 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB) (this mac
[repository:command] still running after 1540 s
[repo] still working after 1550 s; last phase started: source.served
[repository:command] still running after 1550 s
```

## Try 2

- Log `/Users/ueli/native-bootstrap-round-10-try-2.log`: `start 2026-11-14T23:47:48Z`, `end 2026-11-14T23:52:49Z`, `native_rc=7`
- Runner: `native_rc=7 peak_rss_kib=3113728 run_peak_rss_kib=136096 stop=round 11 was published and no task has started fleet_refusal=False window=False nx_task_lines=0 error_lines=0 moving=False`
- nx task lines: 0

`[capacity]` lines of the bootstrap log:
```
[capacity] run: sized at 4% of the default from 15 earlier run(s) of its class, at least 2383 MB because of its assembly phase
[capacity] admitted run with 6 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 59% of the default from 6 earlier run(s) of its class
[capacity] dependencies: sized at 33% of the default from 15 earlier run(s) of its class, at least 2660 MB because of its assembly phase
[capacity] admitted dependencies: 1 job(s), 2660 MB reserved (6192 MB available, 2048 MB floor)
[capacity] admitted run: 1 job(s), 2048 MB reserved (6043 MB available, 2048 MB floor)
```

Last distinct lines of the bootstrap log:
```
[capacity] admitted run: 1 job(s), 2048 MB reserved (6043 MB available, 2048 MB floor)
Loading the project graph of 440cffa5563d.
Planning 1 targets.
Reading the project graph slice (2 s).
[repo] still working after 5 s; last phase started: door.script
Reading the project graph slice (10 s): files 1 of 1.
Reading the project graph slice (16 s): facts 24576 of 238213.
Reading the project graph slice (34 s): verification 2 of 1081.
Reading the project graph slice (40 s): parts 3898 of 14344.
Reading the project graph slice (44 s): flush 1 of 1.
end 2026-11-14T23:52:49Z
native_rc=7
```

Service log, wanted lines (19 `arrived` lines; first `graph: waiting for the fleet: 0 of 367 projects arrived`, last `graph: waiting for the fleet: 356 of 367 projects arrived`):
```
graph: 659 stale projects, 292 taken from the fleet, 9 too heavy for this machine (@semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 27770 MB), repo (about 14264 MB), @semio-tech/framework-os (about 8368 MB), @semio-tech/framework-renderer-wgpu (about 6858 MB), semio-framework-job-native-turn (about 6196 MB), and 3 more; it can give about 3512 MB per project), it can infer 192 of the others (
graph: waiting for the fleet: 0 of 367 projects arrived
```

Service log, last 30 lines of the try (cut at 300 characters):
```
graph: waiting for the fleet: 0 of 367 projects arrived
[repository:command] still running after 2040 s
graph: waiting for the fleet: 0 of 367 projects arrived
[repository:command] still running after 2050 s
graph: waiting for the fleet: 0 of 367 projects arrived
[repo] still working after 2060 s; last phase started: source.served
[repository:command] still running after 2060 s
graph: waiting for the fleet: 0 of 367 projects arrived
[repository:command] still running after 2070 s
graph: waiting for the fleet: 0 of 367 projects arrived
[repository:command] still running after 2080 s
graph: waiting for the fleet: 0 of 367 projects arrived
[repo] still working after 2090 s; last phase started: source.served
[repository:command] still running after 2090 s
graph: waiting for the fleet: 0 of 367 projects arrived
[repository:command] still running after 2100 s
graph: waiting for the fleet: 0 of 367 projects arrived
[repository:command] still running after 2110 s
graph: waiting for the fleet: 0 of 367 projects arrived
[repo] still working after 2120 s; last phase started: source.served
[repository:command] still running after 2120 s
graph: waiting for the fleet: 356 of 367 projects arrived
[repository:command] still running after 2130 s
[repository:command] still running after 2140 s
[repo] still working after 2150 s; last phase started: source.served
[repository:command] still running after 2150 s
[repository:command] still running after 2160 s
[repository:command] still running after 2170 s
[repo] still working after 2180 s; last phase started: source.served
[repository:command] still running after 2180 s
```

