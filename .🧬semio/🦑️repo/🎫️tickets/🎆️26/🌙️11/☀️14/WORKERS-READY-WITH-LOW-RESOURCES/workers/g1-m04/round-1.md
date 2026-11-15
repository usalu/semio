# g1-m04 Round 1

round: 1

## Machine
- `ueli-workstation-lin-1`, SteamOS (kernel 7.2.7-valve2-2-neptune), account `deck`, worktree `/home/deck/semio`. 16 GB memory (MemTotal 16229024 kB), swap 25331 MB of which 9745 MB were in use before the run, 142 GiB free of 465 GiB on `/home`.
- Replica daemon code digest (`.cache/f/health.json` after the run): `39e3688eb315d48831772ca305d78bcb6f35975794444da47750bc5fa7459759`, store `ok`; `failures`: `loop-blocked-restart` count 5. The replica `status` command was not run during the round (one graph command at a time).

## Run
- Command: `W/native-bootstrap.sh "$HOME/native-bootstrap-round-1.log"`, once, in its own process group, started by `workers/g1-m04/run-round.sh 1` (Linux variant of the runner of g1-m07: `ps -eo rss,args`, memory pressure from `/proc/pressure/memory`, hard stop at PSI `full avg10` of 20 % or more).
- Start `2026-11-14T11:36:30Z`, end `2026-11-14T12:12:35Z` (36 min 5 s).
- `native_rc=1`. Marker `.cache/f/native-bootstrap.done` absent.
- Hard stop: none met. No `graph: refreshing N projects` and no `Inferring projects` line in the log; largest process 1968864 KiB resident (the graph service of an older implementation, see below); PSI memory `full avg10` at most 2.34 % over 146 samples.

## Failure
No nx task ran. The tools and the dependency step passed in about 5 minutes; `[semio] workspace:setup…` (log line 16448) then waited for the graph service from the first poll to the timeout, 937 lines `Waiting for the repo graph service (… ms)` over 1883 s. First error line verbatim with the 15 lines before it (without the JSON progress lines):

```
Waiting for the repo graph service (1856974 ms).
Waiting for the repo graph service (1858989 ms).
Waiting for the repo graph service (1861003 ms).
Waiting for the repo graph service (1863019 ms).
Waiting for the repo graph service (1865032 ms).
Waiting for the repo graph service (1867046 ms).
Waiting for the repo graph service (1869060 ms).
Waiting for the repo graph service (1871076 ms).
Waiting for the repo graph service (1873090 ms).
Waiting for the repo graph service (1875104 ms).
Waiting for the repo graph service (1877119 ms).
Waiting for the repo graph service (1879134 ms).
[repo] still working after 1880 s; last phase started: door.script
Waiting for the repo graph service (1881148 ms).
Waiting for the repo graph service (1883163 ms).
The graph service could not answer (timeout); look at `repo service host status` and try again
```

Failed tasks: none listed by nx; `workspace:setup` did not start.

## What Was Seen Around the Graph Service (`/tmp/semio-repo/1000`)
- A graph service of an older implementation is alive and does not serve: `repo service host run` pid 569455 with its child pid 569811 (1.9 GB resident), started 2026-11-14T03:35Z by this machine's provisioning run 4, `host-2afd49639079.json` implementation `58ce27e0e88…`. Its log `host-2afd49639079.log` only repeats `[repository:command] still running after 31091 s` and `[repo] still working after 31100 s; last phase started: source.served`.
- `repo service host status` (11:39 CET, before the round) answered `No repo graph service is running on this host.` while that process existed.
- The service of the current implementation never got past its start: `host-8fb107334f5d.log` holds six lines `booting` and nothing else; there is no `h-8fb107334f5d.sock` and no `host-8fb107334f5d.json`.
- `exit.log`: both services were started in the same cgroup `…/semio-service-graph-a6ffd38e4a64` with `limitMegabytes: 4096`.
- The same timeout ended four single `repo run <project>:<target>` calls between 09:32 and 11:38 CET (31 minutes each) and, earlier, the service of the first implementation ended with `This process (repository:build) made no progress for 10 min and has no live child … exit code 124` after `still running after 30561 s`.
- Not measured: whether the new service waits for the old one (lock, cgroup) or fails on its own.

## `[capacity]` and `graph:` lines (verbatim, repeated lines once; there is no `graph:` line in the log)

```
[capacity] run: sized at 5% of the default from 5 earlier run(s) of its class, at least 8632 MB because of its plan phase
[capacity] waiting 0s for run: not enough free memory (need 6932 MB; 5716 MB spare above the 1981 MB floor, load 2.96; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 5s for run: not enough free memory (need 6932 MB; 5359 MB spare above the 1981 MB floor, load 2.96; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 10s for run: not enough free memory (need 6932 MB; 5378 MB spare above the 1981 MB floor, load 3.12; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 16s for run: not enough free memory (need 6932 MB; 5552 MB spare above the 1981 MB floor, load 3.11; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 21s for run: not enough free memory (need 6932 MB; 5482 MB spare above the 1981 MB floor, load 3.02; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 27s for run: not enough free memory (need 6932 MB; 5526 MB spare above the 1981 MB floor, load 2.94; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] admitted run after 30s with 5611 MB of the 6932 MB it asked for: even if every run of ours ended the host would have only 5611 MB above its 1981 MB floor, so waiting can never free more
[capacity] run: sized at 25% of the default from 5 earlier run(s) of its class, at least 8632 MB because of its plan phase
[capacity] run: sized at 100% of the default from 4 earlier run(s) of its class, at least 8632 MB because of its plan phase
[capacity] dependencies: sized at 51% of the default from 3 earlier run(s) of its class, at least 10142 MB because of its plan phase
[capacity] waiting 0s for dependencies: swap use is above its ceiling (need 6933 MB; 6010 MB spare above the 1981 MB floor, load 3.06; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 6s for dependencies: swap use is above its ceiling (need 6933 MB; 5869 MB spare above the 1981 MB floor, load 3.06; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] admitted dependencies with 5927 of 6933 MB: only 5927 MB are free above the 1981 MB floor right now, and 3072 MB are enough to start
[capacity] run: sized at 46% of the default from 4 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7984 MB available, 1981 MB floor)
```

## Wall Time per Phase
- Tools, launcher and dependency step (`repo dependencies sync --all`): 11:36:30Z to about 11:41Z (not measured more exactly; the dependency scopes were already installed).
- `workspace:setup`: waiting for the graph service 1883 s, then the timeout at 12:12:35Z.

## Facts of This Machine From Earlier Runs (ticket `☀️14/FINISH-CANONICAL-LINUX-WORKER-SETUP-ON-WORKSTATION-LIN-1/report.md`)
- A cold graph load took 126 minutes (run 4, 703 owners) and was cancelled in the assembly at `the service holds 7125 MB of its 7125 MB memory limit`; run 5 got through and `workspace:setup` then ran 86 minutes with four failed tasks (`workspace:deps-build-tools`, `@semio-tech/energy-oracle-py:deps`, `workspace:deps-cargo`, `@semio-tech/framework-os-mcp-rs:build`).
- The npm mirror of the group server answered 404 for `…/mirror/npm/react/-/react-18.3.1.tgz` at 12:35 CET today; this round ran without `SEMIO_FLEET_MIRROR=off` and the dependency step passed (nothing had to be fetched).
- Standalone `repo tools setup all` (11:42 to 12:34 CET, before this protocol was read): 52 minutes of `Checking declared package providers` (7747 edges), then `Internal error: Original physical body, drain and cleanup settlement failed`.

## Disclosure: One Build File Was Changed From This Machine Before the Protocol Was Read
At 12:34 CET, before `PROTOCOL.md` was found, this worker's agent changed `🧰️framework/🔨️modules/🏃️process/🏗️build/🗣️language/📣️progress/🟦️.ts` (new `faultMessageV1`: the message of an `AggregateError` lists the messages of the faults it holds; used by `refuseInternalV1`, so `Internal error: … settlement failed` names its causes) and added one test to `…/📣️progress/🧪️tests/🟦️.ts` (22 pass). Nothing else was changed. It is left in place; the coordinator may keep or remove it. Since then this agent changes only `W/workers/g1-m04/`.
