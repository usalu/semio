# Round 10 on g1-m07 (round 9 was never seen here; round 8 is reported in `round-8.md`)

## Machine
- Mac mini, 16 GiB memory, macOS 27.2.1, 108 GiB free on the data volume.

## Replica agent
- At 23:47Z: `state = running`, `runs = 50`, pid 63536, `last exit code = 0`; kickstarted 0 times. Code digest `eabbc92267329dbd39d8d05e0c8eceb1bcad083c0eea58c44805408a7d547bcc` (the one the round names), now "code":"eabbc92267329dbd39d8d05e0c8eceb1bcad083c0eea58c44805408a7d547bcc". `bootVersion` 6.

## Launcher
- Unchanged before and after both tries: link -> `versions/cf25f2b751f83336/repo`, target inode 3496898, mtime 2026-11-14T22:55:52 (CET), Identifier `tech.semio.repo`, CDHash `6de7547a8b8ff384bbfbfc45e962462063ff0357`. Listings in `round-10.launcher.txt`.
- No `native_rc=4`. No line "runs under the access of the calling app" in these two logs. Launcher receipt at 23:46Z: `state: waiting` (pid 63553).

## Tries (2 of 10)
| try | log | start | end | native_rc | peak resident of the run | pressure samples (level 1 / 2 / 4) |
|---|---|---|---|---|---|---|
| 1 | `~/native-bootstrap-round-10.log` | 23:47:00Z | 00:04:35Z | 7 (hard stop) | 7043888 KiB | 30 / 6 / 0 |
| 2 | `~/native-bootstrap-round-10-try-2.log` | 00:08:35Z | 00:31:00Z | 1 | 2956816 KiB | 45 / 0 / 0 |

Both failed in `workspace:setup` before any task ran (succeeded 0, failed 0, not run: all). Earlier phases passed; wall time per phase not measured.

### Try 1: hard stop, memory pressure level 2 in six samples in a row (00:04:35Z); one SIGINT, gone one minute later
Last progress line: `Reading the project graph slice (668 s): owners 48 of 659; running (name, kind, seconds, rows): @semio-tech/framework-rs (language:cargo, 155, 0); waiting for memory headroom (4161 of 3771 MB reserved).`
```
[capacity] run: sized at 7% of the default from 15 earlier run(s) of its class, at least 4093 MB because of its plan phase
[capacity] admitted run: 8 job(s), 5073 MB reserved (7370 MB available, 2048 MB floor)
[capacity] run: sized at 7% of the default from 15 earlier run(s) of its class, at least 4093 MB because of its plan phase
[capacity] run: sized at 46% of the default from 6 earlier run(s) of its class
[capacity] run: sized at 46% of the default from 6 earlier run(s) of its class
[capacity] dependencies: sized at 24% of the default from 15 earlier run(s) of its class, at least 4093 MB because of its plan phase
[capacity] admitted dependencies: 1 job(s), 4093 MB reserved (7635 MB available, 2048 MB floor)
[capacity] dependencies: sized at 24% of the default from 15 earlier run(s) of its class, at least 4093 MB because of its plan phase
[capacity] run: sized at 102% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7375 MB available, 2048 MB floor)
```
### Try 2
```
The plan cannot continue: the slice worker ended 4 times in a row with exit code 75 because the code on this machine kept changing under it; try again once the replica has settled.
```
```
[capacity] run: sized at 7% of the default from 15 earlier run(s) of its class, at least 4093 MB because of its plan phase
[capacity] admitted run with 7 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 8% of the default from 15 earlier run(s) of its class, at least 4093 MB because of its plan phase
[capacity] run: sized at 52% of the default from 6 earlier run(s) of its class
[capacity] run: sized at 52% of the default from 6 earlier run(s) of its class
[capacity] dependencies: sized at 24% of the default from 15 earlier run(s) of its class, at least 4093 MB because of its plan phase
[capacity] admitted dependencies: 1 job(s), 4093 MB reserved (7730 MB available, 2048 MB floor)
[capacity] dependencies: sized at 24% of the default from 15 earlier run(s) of its class, at least 4093 MB because of its plan phase
[capacity] run: sized at 102% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7413 MB available, 2048 MB floor)
```
### Service log lines (`graph:`, EPERM, `where:`), verbatim, cut at 500 characters
```
# host-9888631afea4.log
graph: No verified change journal cursor is recorded for generation 61; verifying the tree once.
graph: 658 stale projects, 0 taken from the fleet, 6 too heavy for this machine (@semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 8452 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB), @semio-tech/framework-plugin (about 4018 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 658
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 159 of 658 projects arrived
graph: nothing to wait for any longer: the fleet was asked for 658 stale projects and delivered 159 within 1015 s; 499 remain, among them the too heavy project(s) @semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), workspace (about 6200 MB), which this machine must never infer itself (it can give about 3512 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machine
graph: 658 stale projects, 159 taken from the fleet, 2 too heavy for this machine (@semio-tech/repo-lib (about 27770 MB), workspace (about 6200 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 499
graph: waiting for the fleet: 0 of 499 projects arrived
graph: waiting for the fleet: 47 of 499 projects arrived
graph: waiting for the fleet: 95 of 499 projects arrived
graph: nothing to wait for any longer: asked the fleet for 499 stale projects and received 95 within 1487 s, 404 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
# host-ac6bfe8f337c.log
graph: headroom of 306 MB is below one project (410 MB), inferring one project at a time
graph: headroom of 748 MB again (50 MB taken), resuming
graph: waiting for memory headroom (410 of 1043 MB reserved, 1 projects running)
graph: waiting for memory headroom (575 of 983 MB reserved, 1 projects running)
graph: headroom of 740 MB is below one project (1434 MB), inferring one project at a time
graph: waiting for memory headroom (1434 of 740 MB reserved, 1 projects running)
graph: waiting for memory headroom (1434 of 910 MB reserved, 1 projects running)
graph: waiting for memory headroom (1434 of 1049 MB reserved, 1 projects running)
graph: waiting for memory headroom (1434 of 791 MB reserved, 1 projects running)
graph: headroom of 595 MB again (50 MB taken), resuming
graph: waiting for memory headroom (834 of 1228 MB reserved, 5 projects running)
graph: waiting for memory headroom (710 of 1293 MB reserved, 2 projects running)
graph: waiting for memory headroom (866 of 574 MB reserved, 1 projects running)
graph: waiting for memory headroom (970 of 1368 MB reserved, 1 projects running)
graph: waiting for memory headroom (1805 of 746 MB reserved, 1 projects running)
graph: project @semio-tech/framework-os-kernel (language:cargo) grew by 2376 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: waiting for memory headroom (3798 of 574 MB reserved, 1 projects running)
graph: waiting for the graph build of a peer (58 s in the queue, generation 62 was the newest when this command queued)
graph: a peer builds the graph; this run waits as its front door alone, without a loader (58 s, generation 62 was the newest when it queued)
graph: waiting for the graph build of a peer (70 s in the queue, generation 62 was the newest when this command queued)
graph: a peer builds the graph; this run waits as its front door alone, without a loader (70 s, generation 62 was the newest when it queued)
graph: waiting for the graph build of a peer (79 s in the queue, generation 62 was the newest when this command queued)
graph: a peer builds the graph; this run waits as its front door alone, without a loader (79 s, generation 62 was the newest when it queued)
graph: waiting for the graph build of a peer (83 s in the queue, generation 62 was the newest when this command queued)
graph: a peer builds the graph; this run waits as its front door alone, without a loader (83 s, generation 62 was the newest when it queued)
```
If no `fstatat failed (EPERM)` or `where: root=` line is listed above, none was written on this machine in this round.

## End of the round
- Not ready, marker absent. Tries 3 to 10 not run: `ROUND.md` showed `round: 11` when try 2 ended (00:31Z).
