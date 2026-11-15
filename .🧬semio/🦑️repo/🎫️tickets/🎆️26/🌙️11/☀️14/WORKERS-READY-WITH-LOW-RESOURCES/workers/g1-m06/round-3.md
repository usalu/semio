# g1-m06 Round 3

Result: not ready, `native_rc=1`, 1 run. The run did not ask the fleet; it inferred projects on this machine and reached `owners 192 of 703` after 560 s in the graph step. Memory pressure was level 2 in three samples in a row, so I sent one SIGINT under the hard stop rule. No further try was started because of the hard stop. No setup task ran.

## Machine

| Item | Value |
| --- | --- |
| Model | Mac mini, Apple M4, 10 cores |
| Memory | 16 GB |
| Free disk | 110 GiB of 228 GiB |
| OS | macOS 26.6.2 (25G83) |
| Replica code digest (`.cache/f/health.json`) | `39e3688eb315d48831772ca305d78bcb6f35975794444da47750bc5fa7459759` |

## Before the Run

- Replica code digest (`.cache/f/health.json`): `39e3688eb315d48831772ca305d78bcb6f35975794444da47750bc5fa7459759`.
- `ps` lines of `repo show projects` and `repo service host` processes (pid, ppid, age, resident KB, command; runtime path shortened):

```
97767     1       58:17  64528 bun …/door/script.ts repo service host run
97781 97767       58:16  99104 bun --eval \012const fs=await import('node:fs'),path=await import('node:path'),url=await import('node:url'),crypto=await import('node:crypto'),awaitModule=await import('
```

- Last 8 lines of `.cache/f/sidecars/build-agent.log`:

```
2026-11-14T17:23:47.430Z worker stopped: ueli-workstation-mac-mini-m4-16gb-256gb-04-lan.fritz.box-4268
2026-11-14T17:23:48.397Z joining http://192.168.178.70:8981 offering rustc for 3 target(s)
2026-11-14T17:23:48.427Z worker sandbox: darwin-sandbox confined network denied
2026-11-14T17:23:48.430Z worker registered: d0616e5c-5228-4aec-816c-afd728be135b cache http://192.168.178.70:6094
2026-11-14T17:28:44.314Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T17:33:45.312Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T17:38:46.139Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T17:42:52.188Z worker stopped: ueli-workstation-mac-mini-m4-16gb-256gb-04-lan.fritz.box-13636
```

- No `repo show projects` process existed. The `repo service host run` (pid 97767, parent launchd, 58 minutes old, 64 MB) was started by try 3 of round 2; the round did not ask me to stop it and I left it. The run of this round used it.
- While waiting for this round, launchd left the replica agent down 0 times.

## Run

| Item | Value |
| --- | --- |
| Start | 2026-11-14T17:46:01Z |
| End | 2026-11-14T17:55:43Z |
| `native_rc` | 1 (after my SIGINT at 17:55:43Z) |
| Wanted from the fleet, arrived, missing, seconds | none: no request to the fleet was made in this run |
| First and last "arrived" line | none in the log of the run and none in the service log for this run |
| Failed nx tasks | none; `workspace:setup` did not get past the graph step |
| Hard stop | `kern.memorystatus_vm_pressure_level` 2 at 17:54:43Z, 17:55:13Z and 17:55:43Z; 1 in the 16 samples before and 1 again after the stop |

## Lines About the Fleet, `wanted`, `arrived`, `serving`, `restored`, `Restarting`, `adopted`

The log of the run contains none of these words outside the help text. The service log (`host-f74179f31e9f.log`), section of this run, verbatim and in order (stale and withheld lines left out: 203 `is withheld` lines, the last ones name projects that "this graph snapshot does not contain (semio-framework-replication-retained-field-prep)"):

```
[capacity] admitted graph-plan: 1 job(s), 2048 MB reserved (9997 MB available, 2048 MB floor)
graph: No verified change journal cursor is recorded for generation 15; verifying the tree once.
graph: refreshing 703 projects (generation 16)
graph: 544 stale facts (file 221, analysis 116, readdir 93, field 77, digest 20, exists 10, directoryExists 4, readdirNames 3) leave 703 projects stale
graph: 544 more stale inputs are not listed
graph: asking the host for up to 8 worker processes for 48 projects; projects are inferred in this process meanwhile
graph: inferring projects with 1 worker processes (1536 MB each)
graph: 14 of 48 projects inferred by 1 worker processes in 41 s (0 failed, 0 lost, 0 restarts)
[fleet] projects from the fleet: 14 taken, 14 stored, 0 offered, 0 waited for another machine, 14 computed here | Projekte aus der Flotte: 14 übernommen, 14 gespeichert, 0 angeboten, 0 auf eine andere Maschine gewartet, 14 hier berechnet
graph: inferring projects with 1 worker processes (1536 MB each)
graph: 0 of 48 projects inferred by 1 worker processes in 32 s (0 failed, 0 lost, 0 restarts)
[fleet] projects from the fleet: 0 taken, 47 stored, 0 offered, 0 waited for another machine, 48 computed here | Projekte aus der Flotte: 0 übernommen, 47 gespeichert, 0 angeboten, 0 auf eine andere Maschine gewartet, 48 hier berechnet
graph: inferring projects with 1 worker processes (1536 MB each)
graph: worker processes were not used because they differ from this process (identity differs: records (file:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs)); projects are inferred in this process
graph: inferring projects with 2 worker processes (1536 MB each)
graph: 0 of 48 projects inferred by 8 worker processes in 75 s (0 failed, 0 lost, 0 restarts)
[fleet] projects from the fleet: 0 taken, 48 stored, 0 offered, 0 waited for another machine, 48 computed here | Projekte aus der Flotte: 0 übernommen, 48 gespeichert, 0 angeboten, 0 auf eine andere Maschine gewartet, 48 hier berechnet
graph: 0 of 48 projects inferred by 8 worker processes in 73 s (0 failed, 0 lost, 0 restarts)
[fleet] projects from the fleet: 0 taken, 48 stored, 0 offered, 0 waited for another machine, 48 computed here | Projekte aus der Flotte: 0 übernommen, 48 gespeichert, 0 angeboten, 0 auf eine andere Maschine gewartet, 48 hier berechnet
SIGINT received: the running work is cancelled and its leases are released; this process exits after 10 s at the latest.
```

What these lines show, as observed:

- There is no `703 stale projects, K taken from the fleet … asking the fleet for 703` line in this run, unlike every try of round 2. The run went straight to waves of 48 projects on this machine.
- From the third wave on, the `worker processes were not used because they differ from this process (identity differs: records (…/👁️observation/🟨️.mjs))` line appeared 12 times, and the wave summaries say `0 of 48 projects inferred by 8 worker processes`: all 48 were computed in the service process itself.
- 4 waves finished (14 + 48 + 48 + 48 computed here by the `[fleet]` lines; the run log counts `owners 192 of 703`). 192 is also the number round 2 named as what this machine can infer. The stop came at that point; what the run would have done next was not observed.

## Correction to My Round 2 Report, Try 5

I wrote "arrived: not measured" for try 5. The service log shows that the fleet did deliver in that try: `graph: waiting for the fleet: K of 703 projects arrived` went 0, 263, 311, 359, 407, 455, 503, 551 (steps of 48), and local inference of the rest began after 551. First and last line:

```
graph: waiting for the fleet: 0 of 703 projects arrived
graph: waiting for the fleet: 551 of 703 projects arrived
```

So try 5 of round 2 had 551 of 703 from the fleet before pressure stopped it; this run of round 3 took 14.

## Last Lines of the Run

```
Reading the project graph slice (560 s): owners 192 of 703.
SIGINT received: the running work is cancelled and its leases are released; this process exits after 10 s at the latest.
Cancelled (SIGINT); leases released.
Internal error: Original Script process interrupted
end 2026-11-14T17:55:43Z
native_rc=1
```

## `[capacity]` Lines of the Run

```
[capacity] admitted run: 8 job(s), 1303 MB reserved (10709 MB available, 2048 MB floor)
[capacity] dependencies: sized at 48% of the default from 14 earlier run(s) of its class
[capacity] admitted dependencies: 1 job(s), 2955 MB reserved (10775 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 2 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (9997 MB available, 2048 MB floor)
```

The dependency step printed `[DATA] Store dependency sync current {"version":1,"purpose":"current-store-dependency-sync-current","scopes":6}`; the stale receipt line of round 2 is gone.

## Largest Processes (19 samples, every 30 s, with parent pid)

| Process | Parent | Peak resident |
| --- | --- | --- |
| pid 24177, `bun -- …` slice process | 97781, the child of the graph service (97767, parent launchd) | 6,396,048 KB; 4,247,200, 4,372,032 and 5,129,648 KB in the three level-2 samples |
| pid 24163, `bun …` of the run before `workspace:setup` | 24162 | 1,297,328 KB |
| pid 25103, `bun …` worker | 24177 | 1,287,728 KB |
| Claude desktop renderer, pid 16829 | 16801 | 1,257,408 KB |

- The process that grew is not a descendant of the bootstrap; it hangs under the graph service. My runner's 8 GB rule only looks at descendants of the run, so it would not have caught it; it stayed below 8 GB.
- No `git` process above 1 GB appeared. The largest `git` seen was pid 28606 with 138,880 KB, parent 26556, which is the replica daemon (`…/shared-worktree/…`); its command starts `/Library/Developer/CommandLineTools/usr/bin/git -c core.excludesFile= -c core.attributesFile= -c core.quotepath=false …`.

## Wall Time per Phase

| Phase | Wall time |
| --- | --- |
| Launcher, toolchain, native command, dependencies | about 22 s (total minus graph step; not measured separately) |
| Graph step until `flush 1 of 1` begins | 32 s |
| `flush 1 of 1` | 84 s (32 s to 116 s) |
| `owners` 0 to 192 of 703 | 442 s (118 s to 560 s) |
| Total | 582 s |

## Other Observations

- launchd left the replica agent down twice during the run (17:46:01Z and 17:50:52Z); my runner restarted it.
- The log starts again with `[semio] Homebrew not available; install pkg-config, Cairo, Pango and DejaVu fonts before running native C++ builds`.
- I replaced `round-2-tries.sh` by `round-tries.sh <round> <tries> <pause>`; `round-run.sh` now samples with parent pid and full command and records the parent chain of a `git` process above 1 GB.
