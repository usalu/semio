# g1-m06 Round 2

Result: not ready. 5 runs, none reached a setup task. Runs 1 to 4 ended with the fleet refusal (0 of 703 projects arrived each time). Run 5 began inferring projects itself and I stopped it with one SIGINT under the hard stop rule "pressure level 2 in three samples in a row". Run 6 was not started because of that hard stop.

## Machine

| Item | Value |
| --- | --- |
| Model | Mac mini, Apple M4, 10 cores |
| Memory | 16 GB |
| Free disk | 110 GiB of 228 GiB |
| OS | macOS 26.6.2 (25G83) |
| Replica code digest (`.cache/f/health.json`) | `39e3688eb315` before and after the round |

## Before the Run (2026-11-14T15:59:55Z)

- Replica code digest (`.cache/f/health.json`, `code`): `39e3688eb315d48831772ca305d78bcb6f35975794444da47750bc5fa7459759`.
- `repo service host run` older than one hour: none. `ps -axo pid,etime,rss,args | grep "service host"` printed nothing; the process from round 1 (pid 67415) was already gone. `repo service host stop` answered `No repo graph service is running on this host.`
- Last 5 lines of `.cache/f/sidecars/build-agent.log`:

```
2026-11-14T12:41:37.045Z worker finished: f63dfcc1-c0c8-40e9-8cab-d102a38a39b3.1 completed 0
2026-11-14T12:41:41.162Z worker started: df11f957-6f15-4d69-872d-6711d489216b.1 9d08204a148d
2026-11-14T12:41:41.517Z worker finished: df11f957-6f15-4d69-872d-6711d489216b.1 completed 0
2026-11-14T12:42:18.074Z worker started: aad92535-ecc1-4951-8960-dc3f8bdb8ab8.1 c28a46f03ad7
2026-11-14T12:42:18.448Z worker finished: aad92535-ecc1-4951-8960-dc3f8bdb8ab8.1 completed 0
```

- While waiting for this round (11:11Z to 15:59Z) launchd left the replica agent down 7 times; my wait loop restarted it each time (11:13, 11:28, 11:32, 11:36, 12:05, 12:08, 12:13Z).

## Runs

| Try | Start (UTC) | End (UTC) | `native_rc` | Seconds waited for the fleet | Wanted | Arrived | Missing | Can infer here | How it ended |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | 16:00:29 | 16:19:29 | 1 | 901 | 703 | 0 | 703 | 192 | fleet refusal, shown to the caller as "unexpected failure" |
| 2 | 16:20:44 | 16:39:43 | 1 | 901 | 703 | 0 | 703 | 192 | same as try 1 |
| 3 | 16:45:08 | 17:03:58 | 75 | 900 | 703 | 0 | 703 | 192 | fleet refusal with the designed text and status 75 |
| 4 | 17:09:13 | 17:15:17 | 75 | 302 | 703 | 0 | 703 | 192 | same as try 3, shorter wait |
| 5 | 17:20:45 | 17:42:51 | 1 | about 1200 in `flush 1 of 1` (fleet lines of this try not separated in the service log) | 703 | not measured | not measured | 192 | inferring locally, `owners 12 of 703` after 90 s, hard stop |

- Try 2 started only 75 s after try 1 instead of 5 minutes: my wrapper did not recognise the refusal of try 1 (see below) and I restarted the tries by hand. Tries 3 to 5 each started 5 minutes after the run before.
- Failed nx tasks: none in any try. `workspace:setup` never got past the graph step.
- No `Inferring projects: … of M projects` line appeared in any log of a run.

## Lines About the Fleet, Verbatim

Tries 1 and 2, log of the run (the only line; no `graph-wanted`, no "fleet", no status 75):

```
The resident service met an unexpected failure (reference 80ba70d1); try again, the details under that reference are in the service log /var/folders/_d/gdnvzj856_zbxn3kl48wfkfr0000gn/T/semio-repo/501/host-38cdf415c7fb.log.
```

Tries 1 and 2, service log `host-38cdf415c7fb.log`:

```
graph: 703 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 703
graph: waiting for the fleet: 0 of 703 projects arrived
graph: nothing to wait for any longer: asked the fleet for 703 stale projects and received 0 within 901 s, 703 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T16:19:29.289Z request-failed 80ba70d1 RepoError: timeout graph-wanted.expired {"wanted":703,"arrived":0,"seconds":901,"missing":703,"local":192}
2026-11-14T16:39:43.761Z request-failed 80ba70d1 RepoError: timeout graph-wanted.expired {"wanted":703,"arrived":0,"seconds":901,"missing":703,"local":192}
```

Try 3, log of the run (line cut at 300 characters in my extract; full text in the log on this machine):

```
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 703 and delivered 0 within 900 s, 703 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the grap
```

Try 4, log of the run:

```
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 703 and delivered 0 within 302 s, 703 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the grap
```

Tries 3 to 5, service log `host-f74179f31e9f.log`:

```
graph: 703 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 703
graph: nothing to wait for any longer: asked the fleet for 703 stale projects and received 0 within 900 s, 703 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
graph: 703 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 703
graph: nothing to wait for any longer: asked the fleet for 703 stale projects and received 0 within 302 s, 703 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
graph: 703 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 703
```

No line with `restored` or `Restarting with the current implementation` appeared in any log of a run.

## Try 5: Local Inference and the Hard Stop

After about 1213 s in `flush 1 of 1` the run began to infer projects on this machine. Last lines of the run:

```
Reading the project graph slice (1303 s): owners 12 of 703; running (name, kind, seconds, rows): @semio-tech/2d-compute-rs (language:rust, 42, 0), @semio-tech/compute-ownership-runner (project, 42, 0), @semio-tech/2d-regions-rs (language:rust, 42, 0).
Reading the project graph slice (1305 s): owners 12 of 703; running (name, kind, seconds, rows): @semio-tech/2d-compute-rs (language:rust, 53, 0), @semio-tech/compute-ownership-runner (project, 53, 0), @semio-tech/2d-regions-rs (language:rust, 53, 0).
SIGINT received: the running work is cancelled and its leases are released; this process exits after 10 s at the latest.
Cancelled (SIGINT); leases released.
Internal error: Original Script process interrupted
```

Last `graph:` lines of the service log for this try:

```
graph: 544 more stale inputs are not listed
graph: asking the host for up to 8 worker processes for 48 projects; projects are inferred in this process meanwhile
graph: the worker processes are starting; this process infers 2 projects meanwhile and holds the others for the workers for at most 20 s
graph: waiting for memory headroom (60 of -31 MB reserved, 5 projects running)
graph: headroom of -91 MB is below one project (12 MB), inferring one project at a time
graph: headroom of 51 MB again (12 MB taken), resuming
graph: waiting for memory headroom (40 of 51 MB reserved, 4 projects running)
```

- `kern.memorystatus_vm_pressure_level` was 2 at 17:41:49Z, 17:42:19Z and 17:42:50Z; the SIGINT went out at 17:42:50Z. It was 1 in all earlier samples of the round and is 1 again after the stop.
- The largest process at the stop was pid 12264 with 4,580,848 KB resident (3,470,704 KB 30 s earlier). It is not a child of the run; it belonged to the graph service of this host and ended with the run.
- GitKraken was open on the machine at that time and ran a `git` process of 288,080 KB; it is not mine and I did not touch it.
- 12 of 703 owners in 90 s; at that rate the 703 would take about 88 minutes if memory allowed (extrapolated, not measured).

## `[capacity]` Lines

No `graph:` line appears in the logs of the runs (they are in the service log, above). `[capacity]` lines, verbatim, per try:

```
try 1: [capacity] admitted dependencies: 1 job(s), 2955 MB reserved (7509 MB available, 2048 MB floor)
try 1: [capacity] admitted run: 1 job(s), 2048 MB reserved (7644 MB available, 2048 MB floor)
try 2: [capacity] admitted run: 8 job(s), 1695 MB reserved (6937 MB available, 2048 MB floor)
try 2: [capacity] admitted dependencies: 1 job(s), 2955 MB reserved (6886 MB available, 2048 MB floor)
try 2: [capacity] waiting 0s for run: not enough free memory (need 2048 MB; 884 MB spare above the 2048 MB floor, load 2.68; 0 running, 0 ahead); behind the host itself (nothing of ours)
try 2: [capacity] admitted run: 1 job(s), 2048 MB reserved (6887 MB available, 2048 MB floor)
try 3: [capacity] admitted run: 8 job(s), 1695 MB reserved (8197 MB available, 2048 MB floor)
try 3: [capacity] admitted dependencies: 1 job(s), 2955 MB reserved (8106 MB available, 2048 MB floor)
try 3: [capacity] admitted run: 1 job(s), 2048 MB reserved (4709 MB available, 2048 MB floor)
try 4: [capacity] admitted run: 8 job(s), 1695 MB reserved (7625 MB available, 2048 MB floor)
try 4: [capacity] admitted dependencies: 1 job(s), 2955 MB reserved (7457 MB available, 2048 MB floor)
try 4: [capacity] admitted run: 1 job(s), 2048 MB reserved (7398 MB available, 2048 MB floor)
try 5: [capacity] admitted run: 8 job(s), 1695 MB reserved (7396 MB available, 2048 MB floor)
try 5: [capacity] admitted dependencies: 1 job(s), 2955 MB reserved (6741 MB available, 2048 MB floor)
try 5: [capacity] admitted run: 1 job(s), 2048 MB reserved (7319 MB available, 2048 MB floor)
```

Each try also printed the `sized at 8%`, `46%` and `48% of the default` lines of round 1 unchanged. The dependency step of every try printed:

```
The dependency generation receipt of scope generation:. is stale: A provider file or directory changed: w:🧰️framework/🛍️products/🦑️repo.
```

## Peak Resident Size

Sampled every 30 s (38, 38, 38, 13 and 45 samples).

| Try | Largest process | Peak resident |
| --- | --- | --- |
| 1 | pid 79424, child of the run, constant during the fleet wait | 3,991,504 KB |
| 2 | pid 88411 | 3,755,552 KB |
| 3 | pid 97857 | 3,816,528 KB |
| 4 | children of the run only | 126,400 KB; the graph service was not sampled separately |
| 5 | pid 12264, graph service side | 4,580,848 KB |

No process came near 8 GB.

## Wall Time per Phase

| Try | Before `workspace:setup` (launcher, toolchain, native command, dependencies) | Graph step | Total |
| --- | --- | --- | --- |
| 1 | about 149 s | 991 s | 1140 s |
| 2 | about 154 s | 985 s | 1139 s |
| 3 | about 143 s | 987 s | 1130 s |
| 4 | about 24 s | 340 s | 364 s |
| 5 | about 21 s | 1305 s | 1326 s |

The first column is total minus the graph step, not measured separately.

## Other Observations

- Tries 1 and 2 used the graph service started by try 1 (`host-38cdf415c7fb.log`), which answered the refusal as an internal failure with reference 80ba70d1. From try 3 on a new service (`host-f74179f31e9f.log`) gave the designed text and status 75. I did not restart the service; it changed by itself between try 2 and try 3.
- A `repo service host run` (pid 97767, started with try 3, 64 MB) and its child (pid 97781, 95 MB) are still running after the round. I left them.
- launchd left the replica agent down once during the round (16:26:15Z, restarted by my runner).
- The automatic stop "more than 300 inferred projects without a fleet line" of my runner cannot fire: the help text printed early in each log contains "the fleet". I checked that condition by hand; it did not occur.
- `round-run.sh` and `round-2-tries.sh` in this folder are the runner and the retry wrapper; `round-2.try-1.md` is the interim note written after try 1.
