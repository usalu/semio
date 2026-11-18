# g1-m04 Round 30 (Linux, 16 GB, SteamOS)

One try, 18:46 to 23:57 UTC on 2026-11-15 (5 h 11 min), `native_rc=75`. Marker absent, `ready: false`. No SIGINT. Round 31 (20:55 UTC: stop running setup yourself) was written while this try ran; it was allowed to finish, and no further try or setup command was started. From 23:57 UTC this session only observes (`round-31.md`).

The try ran beside the fleet agent's own `ready` child, which round 31 names as a conflict: `.cache/f/ready/` holds `dependencies.log` (19:36 UTC), `tools.log` (20:22), `native.log` (21:46) and three `repo-mcp` attempts that each ended with `stage-timeout` after 3600 s (20:46 to 23:48 UTC, `ready-timings.json`).

Files beside this report: `round-30.tries.txt`, `round-30.service.txt`, `round-30.bootstrap-last40.txt`, `round-30.p12.txt`, `round-30.samples.txt`, `round-30.monitor.txt`, `round-30.tail.txt`. Log: `~/native-bootstrap-round-30.log`.

## What Round 30 Asked For

| Asked | Measured |
|---|---|
| highest step reached | 6 of 6 |
| peak resident size of the graph child | 8937 MB (5 s sampler); largest process in the 30 s samples 8147 MB |
| `native_rc` 0 | no, 75 |

All `cut smaller` lines (each `Reading the project graph slice (0 s): the slice worker grew to N MB, more than the 6339 MB a plan may take on this machine, so the work is cut smaller (step k of 6: fewer projects at once and smaller waves) and continues; projects already inferred are kept and nothing is repeated.`):

| Step | Grew to |
|---|---|
| 2 of 6 | 7992 MB |
| 3 of 6 | 8410 MB |
| 4 of 6 | 8604 MB |
| 5 of 6 | 8281 MB |
| 6 of 6 | 8733 MB |

Refusal after step 6:

```
The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to 6344 MB, more than the 6339 MB this machine gives a plan, and only 1615 MB of memory were available (at least 2048 MB are kept in reserve) after it gave back what it could. It was stopped before the host ran short of memory. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

## Course

- 18:46 to about 20:07 UTC: dependency storage before the graph read (`bun:dependency-storage remains active on native-fixed-image:/home/deck/semio/.cache/b/d`, 1458 s at 19:17 UTC).
- From about 20:07 UTC: graph read. Largest counts: `53 of 703 projects arrived from the fleet, 82 inferred here, 568 left; the server last progressed 76 s ago, estimated time left: about 5 h 6 min` at 23:42 UTC. This is the first round in which the wait line shows arrivals from the fleet on this machine since round 14.
- Service line: `graph: 33 projects taken from the fleet with 0 closure entries, 23 of them did not hold here and were inferred again, 0 not shared, 17 inferred here`.
- In round 29 try 7 the same machine inferred 645 projects after one step (step 2); here five steps were used up with at most 82 inferred. The difference I can name: the `ready` child ran its own stages at the same time (see above), and the lowest `MemAvailable` in the 30 s samples was 1623 MB.
- The slice worker was at 5990 MB in the last sample before the refusal.
- Closure entries: 16457 before, 16406 after.
- The error of round 29 try 7 is still logged once: `graph: preparing the stale projects failed (invalid build.closure.provider-generated-import {"port":"typescript-package-index","targets":"@semio-tech/ui-rs:generate"}); the refresh computes them itself`.

## Setup Tasks

Not run: succeeded 0, failed 0, not run all.
