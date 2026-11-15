# g1-m02 Round 1

round: 1

Result: `native_rc=1`, not ready. No task failed: `workspace:setup` never started a task. Its graph slice waited 2702 s for memory headroom and I ended the run with one SIGINT.

## Machine

| Item | Value |
| --- | --- |
| Model | Mac mini `Mac16,10`, Apple M4, 10 cores |
| Memory | 16 GB (17179869184 bytes) |
| Free disk | 100 GiB of 228 GiB on the data volume (after the run) |
| OS | macOS 27.2.1 (26B131) |
| Replica code digest (`status`, 10:48Z) | `3d9ead73396c`, daemon running, 0 events behind |
| Fleet row of g1-m02 (`status`, 10:48Z) | `diverged`, view 36/12/51; warning `g1-m02 cannot resolve HEAD of its repository while the fleet has one (config-unreadable)` |

## Run

| Item | Value |
| --- | --- |
| Command | `W/native-bootstrap.sh "$HOME/native-bootstrap-round-1.log"`, once, started by `run-round.py` |
| Start | 2026-11-14T10:49:19Z |
| End | 2026-11-14T11:38:20Z (49 min 1 s) |
| `native_rc` | 1 (after my SIGINT to the process group at 11:38:20Z) |
| Marker `.cache/f/native-bootstrap.done` | absent |
| Failed tasks | none reported; no task ran. No line starting with `error` in the log |
| Hard stop conditions of the round | none met: no `graph: refreshing N projects` and no `Inferring projects` line in the log; largest process 3455216 KiB resident; `kern.memorystatus_vm_pressure_level` 1 in all 98 samples |
| Peak resident size seen | 3455216 KiB (30 s samples in `round-1.samples.txt`) |

The stop is my decision and outside the round's hard stop list. The same wait ran 27172 s on this machine last night (04:14 to 11:47 local) without ever starting, so I ended this one after 45 minutes.

## Wall Time per Phase

| Phase | From | Seconds |
| --- | --- | --- |
| Tools, runtimes, dependency storage (everything before `[semio] workspace:setup…`) | 10:49:19Z | about 235 (derived: 2941 s total minus 2704 s of the wait; not measured separately) |
| `workspace:setup`, reading the project graph slice, waiting for memory | about 10:53:16Z | 2702 (last counter in the log) |
| Any nx task | never started | 0 |

## `[capacity]` and `graph:` Lines, Verbatim

There is no `graph:` line in the log.

```
[capacity] run: sized at 9% of the default from 6 earlier run(s) of its class, at least 2237 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 3476 MB reserved (6126 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 6 earlier run(s) of its class
[capacity] dependencies: sized at 87% of the default from 5 earlier run(s) of its class
[capacity] admitted dependencies with 3146 of 5370 MB: only 3146 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7860 MB available, 2048 MB floor)
```

## The Blocking Wait

The lines before it (the `[script] {…}` progress lines above them left out), and its first and last line, verbatim. 1345 such lines, one every 2 s:

```
[DATA] Store dependency concurrency {"version":1,"purpose":"dependency-concurrency-event","event":"kept","count":6}
Kept 6 superseded dependency generation(s) that are pinned, recent or the rollback target.
[script] {"owner":"repository:build","command":"repo","stage":"complete"}
[semio] workspace:setup…
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7860 MB available, 2048 MB floor)
[script] {"owner":"repository:build","command":"repo","stage":"loading"}
[script] {"owner":"repository:build","command":"repo","stage":"running"}
Loading the project graph of 440cffa5563d.
Planning 1 targets.
Reading the project graph slice (2 s): reservation 0 of 8192; running (name, kind, seconds, rows): ; waiting for memory headroom (8192 of 6785 MB reserved).
…
Reading the project graph slice (2702 s): reservation 0 of 8192; running (name, kind, seconds, rows): ; waiting for memory headroom (8192 of 8273 MB reserved).
end 2026-11-14T11:38:20Z
native_rc=1
```

Measured on the second figure of that line (`of N MB`): minimum 5378, maximum 8710. It was at or above 8200 in 746 of the 1345 lines, first at 1210 s, and stayed above 8192 from then to the end (last values 8224, 8209, 8234, 8273). The reservation stayed `0 of 8192` all the time. So on this machine the waiter is not admitted even while the offered figure exceeds the demand. Whether admission needs the demand plus the 2048 MB floor or plus the 2048 MB the run itself already holds, or is not re-evaluated, is not measured.

## State of the Machine Before the Round (things the coordinator should know)

- Two graph commands from last night were still queued when I started, both since about 04:12 local behind a `graph-plan` waiter of the resident host (pid 33841, `wants 1 job(s), 8192 MB`, capacity then `6161 MB available of 16384 MB (floor 2048 MB, 4113 MB spare)`):
  - mine, `repo run workspace:deps-js-all`: stopped by me at 11:47 local, after 27172 s in `door.script`.
  - another Claude session's on this machine, `repo run @teaching/proctor:test --skip-remote-cache` (pid 79795, 2048 MB grant): stopped by that session at my request before the round started. It holds all graph commands until this round is reported.
- The resident graph host (`repo service host run`, pid 33840, started 02:49 local) was running during the round and still is. I did not stop it.
- A code change of mine from last night is still in the shared tree, made before this protocol existed: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟨️.cjs` line 254, in `observe`, `error.code === "ENOENT"` became `error.code === "ENOENT" || error.code === "ENOTDIR"` (the native bootstrap had failed on `ENOTDIR` for `CLAUDE.md/package.json`). No test was added. It is the coordinator's to keep or revert; I do not touch it again.
- Earlier failures of the bootstrap on this machine, with the errors of eight tasks, are in `🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/g1-m02-relocate.md` (last section).

## Files

- `round-1.tail.txt`: last 200 lines of the log (colour codes removed, lines cut at 600 characters).
- `round-1.samples.txt`: `ps -axo rss,command | sort -nr | head -3` and the pressure level every 30 s.
- `run-round.py`: the runner.
- Full log on the machine: `~/native-bootstrap-round-1.log` (17656 lines).
