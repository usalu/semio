# g1-m05 (Windows): Replica Daemon Stuck for 33 Minutes With a Live Heartbeat; Restarted by Hand

Written 2026-11-16T19:25Z by the agent session on the machine. This is the one action of this session that is not observation: I ended two processes of the replica service. It is not a setup command and nothing under `.cache/f/ready` was touched.

## What Was Seen (UTC)

- 18:48:26: `S:\v1\origins\g1-m05\head.json` replaced for the last time (3 253 events).
- 18:48:40: last line of `%LOCALAPPDATA%\semio\fleet\entry\run.log` (`published count 1`, then `applied count 1`); `state.wal` last written.
- 18:48:41: `health.json` last written (`store ok`, `pending null`, sidecar `build-agent` running, pid 16604).
- 18:58:42: `sidecars\build-agent.log`: `agent lease not renewed for 600 s, leaving`. The build agent (16604), the `ready` child (39696) and the graph read were gone afterwards. Nothing started them again.
- 19:15 to 19:19: daemon pid 93524 (`🛟️supervisor … --boot-child run`, started 18:22:10) alive, 27 threads, 147 handles, 335 MB. `boot\beat.json` and `daemon.lock` were rewritten every few seconds with `phase run, lagMs 52, activity file, longestMs 12248`. In 10 s it used 0.8 s CPU and did 17 447 other file operations and 10 writes.
- Effect: nothing published and nothing applied for 33 minutes (my file `round-31.tools-stage.md` of 19:17 did not leave the machine, `result-R89.md` and `result-R90.md` did not arrive); the `tools` run of 18:16:32 (wrapper 81548, worker 7508) ran past its 3 600 s cap with no `ready` child to watch it. The store itself was reachable (`S:\v1\origins` readable, the heads of g1-m04, g1-m08 and g1-server moved at 19:18).

Reading: the event loop was not blocked (the beat had a lag of 52 ms), so the `loop-blocked-restart` watchdog had no reason to act. Something the main round awaits never came back. I could not see what; a `bun` process gives no stack from outside. The supervisor's heartbeat proves a live loop, it does not prove that rounds complete.

## What I Did

1. 19:19:32: `Stop-ScheduledTask semio-fleet-replica`, then `Start-ScheduledTask`. This did NOT end the old processes: the new instance wrote `Another replica daemon (pid 93524) already runs in this worktree.` and left with exit 75 (`entry\state.json` `attempts: exit 75`).
2. 19:20:14: `Stop-Process -Force` on 93524 and on its parent 51764 (`boot.mjs run`). Both ended.
3. The task started the daemon again by itself within a minute (boot pid 64936, daemon pid 70692).

## Result

- 19:21:42: head replaced (3 256 events). `run.log`: `applied 485`, `published 3`. `health.json` is written again, `store ok`.
- `build-agent` runs again (pid 92580, since 19:21:45); the new `ready` child (pid 53104) adopted the running `tools` stage (`reason adopted`); the wrapper 81548 and the worker 7508 were never touched and are alive.
- `health.json` failure counter `blocked` started again from 0 (52 at 19:21:50), most lines are `sharing-violation` on `.git/objects/..` and `pack-incomplete`.

## Proposals (Not Done by Me, Build Code Is the Coordinator's)

- A watchdog on round completion, not only on loop lag: when `health.json` (or the round counter behind it) has not advanced for some minutes while the beat is alive, the supervisor restarts the daemon. Today this state lasts until a person looks.
- `Stop-ScheduledTask` does not end the daemon on Windows. If the runbook or a script relies on it, it needs the process ids of `entry\state.json` and `boot\beat.json`.
- The stage cap is only enforced while a `ready` child lives. That is harmless after the adoption, but it explains a `tools` row longer than 3 600 s for the run of 18:16:32.

## Not Known

- The cause of the stuck round. The last activity in the beat was `file`; the last log lines are an ordinary publish and apply.
- Whether it happened before on this machine: the head had also not moved from 2026-11-15T15:41Z to 2026-11-16T00:50Z (reported in `round-31.pre.md`), but then nothing was written here either, so that proves nothing.

## Addendum 19:30Z: Slow After the Restart, Not Stuck

- This file was written at 19:22:20Z and left the machine at 19:28:25Z (head 3 256 to 3 258 events): six minutes, against a few seconds before the incident. In between the daemon did 1 500 to 3 300 other file operations per second and wrote `health.json` about every 100 s (19:21:50, 19:23:54, 19:25:35), so it was working, with the beat on `file`, `git` and `follow`.
- So the two states differ in `health.json`: slow = written every 100 s or so; stuck (18:48Z to 19:20Z) = not written for 30 minutes. A watchdog can use that file's age.
