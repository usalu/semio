# g1-m04 Round 24 (Linux, 16 GB, SteamOS)

Step 0 once (06:38 to 07:20 UTC), then twelve tries from 07:21 to 10:04 UTC on 2026-11-15: try 1 `native_rc=75`, tries 2 to 12 `native_rc=1`. Marker absent, `ready: false`. No hard stop, no SIGINT, no setup task ran. Rounds 25 to 27 arrived during the series; the code of round 25 was active from try 2 on.

**Error of my runner:** the series should have ended after try 3 (two tries in a row with the same other error). The refusal text contains "try again later", which my script took for retry wording, so it ran nine tries too many (4 to 12). They add no information beyond the table below. Corrected in `run-tries-27.sh`.

Files beside this report: `round-24.step0.txt`, `round-24.tries.txt`, `round-24[-try-k].service.txt`, `.p12.txt`, `.samples.txt`, `.monitor.txt`, `.bootstrap-last40.txt` (tries 2 to 12), `round-24.tail.txt`. Logs: `~/tools-setup-round-24.log`, `~/native-bootstrap-round-24[-try-k].log`. No file name contains a colon and none is a symlink (round 27).

## Step 0: `repo tools setup all`

Exit 1, wall time 2542 s. No `restored` / `installed` line per scope was printed (the lines my filter caught are `javascript-installed-paths` activity lines). Last lines:

```
[script] repository:command · Checking declared package providers: 7748 edges
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:selected-plan:5","mode":"exclusive","elapsedMs":0}}
Internal error: Exact selected dependency fields required
repository:command failed with exit code 1
```

This is not the error round 25 names as fixed (`retirement plan … 16 store roles`). Step 0 runs once more at the start of round 27.

## Access Proof

Every service start logged it, route direct, for example `2026-11-15T08:49:47.871Z access-proof version=3 root=/home/deck/semio protected=no sample=ok reproof=off route=direct pid=19057 ppid=18676`. No `fstatat failed (EPERM)` line, no `where:` line, no launchd line (Linux).

## Try 1 (old code, 07:21 to 08:32 UTC)

```
graph: 703 stale projects, 0 taken from the fleet, 6 too heavy (…; it can give about 0 MB per project), it can infer 0 of the others (0 per wave); asking the fleet for 703
graph: this machine is linux-x64 (class posix-shared); machines of every class serve its request except the projects bound to its host, which only a machine of that class or this machine infers, and their graph server logs 'serving' for it
graph: no machine of this class gave a sign of life for 5 min, so no one holds the request
```

Last waiting line: `waiting for the fleet: 0 of 703 projects arrived; the server last progressed 855 s ago, estimated time left: about 11 min`. End: `the fleet was asked for 703 and delivered 0 within 3941 s, 703 are still missing`. Peak of the largest process 1752 MB, slice worker 1906 MB.

## Tries 2 to 12 (code of round 25)

The grant is fixed as round 25 says. First plan lines, verbatim (heavy list shortened in the first):

```
graph: memory readings: machine 15848 MB (physical 15848 MB, cgroup limit none), available 8227 MB
graph: 703 stale projects, 0 taken from the fleet, 7 too heavy (…; it can give about 3298 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 703
graph: plan stage inference starts (managed 1250 MB, resident 1630 MB)
```

From try 3 on the too heavy count is 5: `@semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB)`.

Every one of these tries ended after 4.5 to 8 minutes with the same error (last line of the bootstrap log before `native_rc=1`):

```
The plan cannot continue: the service holds 6381 MB of its 6381 MB memory limit (safety margin 256 MB), so it was cancelled to keep the service up; try again later or ask for a smaller selection.
```

Try 2 additionally logged in the service: `graph: plan stage inference (during inference) grew the plan by 4117 MB, more than the 2048 MB this machine allows for it; it is stopped before the host runs short of memory`.

| Try | UTC | Service limit reached | Slice worker peak | Largest process | Progress line at the peak |
|---|---|---|---|---|---|
| 2 | 08:35 to 08:43 | 5644 MB | 5267 MB | 5268 MB | `0 of 703 projects arrived from the fleet, 0 inferred here, 703 left` |
| 3 | 08:46 to 08:52 | 5340 MB | 5147 MB | 1768 MB (30 s sample missed the peak) | same |
| 4 | 08:55 to 09:00 | 6172 MB | 6035 MB | 5856 MB | same |
| 5 | 09:03 to 09:07 | 5838 MB | 5609 MB | 1470 MB (sample missed the peak) | same |
| 6 | 09:10 to 09:15 | 6339 MB | 6247 MB | 5885 MB | `waiting for memory headroom (741 of 914 MB reserved, 1 projects running): semio-plugin-typing-edit needs 741 MB …` |
| 7 | 09:18 to 09:23 | 6364 MB | 6377 MB | 5808 MB | same kind |
| 8 | 09:26 to 09:32 | 6339 MB | 6262 MB | 3925 MB | same kind |
| 9 | 09:35 to 09:40 | 6339 MB | 6258 MB | 6260 MB | `0 of 703 … 0 inferred here, 703 left` |
| 10 | 09:43 to 09:48 | 6384 MB | 6253 MB | 5063 MB | headroom wait |
| 11 | 09:51 to 09:56 | 6339 MB | 6268 MB | 6078 MB | `0 of 703 projects arrived from the fleet, 1 inferred here, 702 left` |
| 12 | 09:59 to 10:04 | 6381 MB | 6351 MB | 6022 MB | `waiting for memory headroom (1222 of 1304 MB reserved, 2 projects running): @semio-tech/2d-text-rs needs 290 MB …` |

What the lines show together:

- The slice worker grows to the service's memory limit (5.3 to 6.4 GB) within a few minutes while at most 1 of 703 projects is inferred; the fleet delivered 0 of 703 in every try.
- The headroom lines still name `the 1024 MB one project is granted`, while the plan line says `about 3298 MB per project`. The most frequent wait is for `semio-plugin-typing-edit needs 741 MB`.
- Closure entries: 17377 before try 2, 17380 after it, unchanged afterwards.
- `MemAvailable` in the 30 s samples stayed above 1 GB (no NOTE in any monitor file). `[capacity]` lines of try 12: `run: sized at 5% of the default from 16 earlier run(s) of its class, at least 12858 MB because of its plan phase`, `admitted run: 8 job(s), 7303 MB reserved (10251 MB available, 1981 MB floor)`.
- No restore line (`restored … from a state`), no `closure-missing`, no `settled`, no `plan stage … took` line in any try.

## Setup Tasks

Not run in any try: succeeded 0, failed 0, not run all.

## Open for the Coordinator

1. `The plan cannot continue: the service holds X MB of its X MB memory limit` in eleven tries in a row, during plan stage `inference`, with 8 to 9 GB available on the machine.
2. The fleet delivered 0 of 703 in all twelve tries (in round 14 it delivered 517 in tries 4 and 5).
3. Step 0 exit 1 with `Internal error: Exact selected dependency fields required`.
4. The 1024 MB per-project grant in the headroom lines against 3298 MB in the plan line.
