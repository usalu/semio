# g1-m04 Round 29 (Linux, 16 GB, SteamOS)

Seven tries five minutes apart from 15:22 to 18:46 UTC on 2026-11-15, ended because `ROUND.md` named round 30. Marker absent, `ready: false`. Replica code at the start `32b23f76…`, nothing pending. Try 7 began at 16:54 UTC and printed the `cut smaller` line of round 30 (written 16:55 UTC), so it ran that code. Round 30 runs in `run-loop.sh 30` from 18:47 UTC (replica code `b4c6513b…`, nothing pending).

Files beside this report: `round-29.tries.txt`, `round-29[-try-k].service.txt`, `.bootstrap-last40.txt`, `.p12.txt`, `.samples.txt`, `.monitor.txt`, `round-29-try-7.request-failed.txt`, `round-29.tail.txt`. Logs: `~/native-bootstrap-round-29[-try-k].log`.

## Tries

| Try | UTC | End | Slice worker peak | Largest process (30 s) | Largest x of `waiting for the fleet` |
|---|---|---|---|---|---|
| 1 | 15:22 to 15:28 | SIGINT of my sampler above 8 GB, rc 7 | 9243 MB | 9260 MB | 0 of 703 |
| 2 | 15:34 to 15:39 | same | 9064 MB | 9071 MB | 0 of 703 |
| 3 | 15:45 to 15:50 | same | 8473 MB | 8485 MB | 0 of 703 |
| 4 | 15:56 to 16:07 | `native_rc=1`, service memory limit | 7404 MB | 6024 MB | 0 of 703 |
| 5 | 16:12 to 16:32 | same | 7815 MB | 6782 MB | 0 of 703 |
| 6 | 16:37 to 16:48 | same | 8977 MB | 6373 MB | 0 of 703 |
| 7 | 16:54 to 18:46 | `native_rc=1`, unexpected failure of the service after the graph was inferred | 9347 MB | 6715 MB | 0 of 703; one line says 12 projects taken from the fleet |

Tries 4 to 6 ended with `The plan cannot continue: the service holds 9764 MB of its 9764 MB memory limit (safety margin 256 MB), so it was cancelled to keep the service up; try again later or ask for a smaller selection.`

## Try 7: First Try That Got Through the Inference

Highest step reached: 2 of 6. The only `cut smaller` line:

```
Reading the project graph slice (0 s): the slice worker grew to 9440 MB, more than the 6339 MB a plan may take on this machine, so the work is cut smaller (step 2 of 6: fewer projects at once and smaller waves) and continues; projects already inferred are kept and nothing is repeated.
```

After it the slice worker stayed near 1.5 GB (1548 MB at 18:08 UTC) and the count rose to `0 of 703 projects arrived from the fleet, 645 inferred here, 58 left` (largest value seen). Service lines in order:

```
graph: 703 stale projects, 0 taken from the fleet, 57 too heavy (…; it can give about 3298 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 703
graph: preparing 646 of the 646 open stale projects (1 compute slots), 0 of 646 already prepared
graph: plan stage inference starts (managed 1164 MB, resident 1586 MB)
graph: plan stage inference settled from 6464 to 7400 MB
graph: 12 projects taken from the fleet with 0 closure entries, 0 of them did not hold here and were inferred again, 0 not shared, 38 inferred here
graph: plan stage inference took 18.1 s (managed 1510 MB, 356 MB more than at its start, resident 1806 MB)
graph: preparing the stale projects failed (invalid build.closure.provider-generated-import {"port":"typescript-package-index","targets":"@semio-tech/ui-rs:generate"}); the refresh computes them itself
graph: plan stage inference took 27.2 s (managed 2105 MB, 314 MB more than at its start, resident 2435 MB)
```

End of the bootstrap log after 6727 s:

```
The resident service met an unexpected failure (reference 7e3ceb77); try again, the details under that reference are in the service log /run/user/1000/semio-repo/1000/host-9a98371c2d16.log.
native_rc=1
```

`grep -a 'request-failed 7e3ceb77' -A12` (full copy in `round-29-try-7.request-failed.txt`):

```
2026-11-15T18:46:07.311Z request-failed 7e3ceb77 RepoError: invalid build.closure.provider-generated-import {"port":"typescript-package-index","targets":"@semio-tech/ui-rs:generate"}
    at repoErrorV1 (…/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts:33:14)
    at toolClosureGroupV1 (…/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs:1046:33)
```

So the memory dead end is passed on this machine with step 2 of 6; the try then failed on a closure rule of the project `@semio-tech/ui-rs` (target `generate`, port `typescript-package-index`), which is not a memory matter.

## Capacity

Try 7, last `[capacity]` lines: `run: work has to run, waiting 27s to grow the reservation from 512 MB to 11812 MB: not enough free memory (2044 MB spare above the 1981 MB floor; 1 running, 0 ahead); behind graph-plan (running, 1 job(s), 6339 MB)`, `run: reservation grown from 512 MB to 3712 MB with 1 of 8 job(s) after 31s`, `run: grown from 1 to 4 of 8 job(s) after 90s, reservation 3712 MB -> 6412 MB`.

## Setup Tasks

Not run in any try: succeeded 0, failed 0, not run all.

## Open for the Coordinator

1. `invalid build.closure.provider-generated-import {"port":"typescript-package-index","targets":"@semio-tech/ui-rs:generate"}` ends the try after a complete inference (1 h 52 min).
2. Tries 1 to 3: the 8 GB hard stop of round 23 acts before the service's own handling (see `round-28.md`); with the `cut smaller` step the worker reached 9347 MB once in try 7 between two 30 s samples and then fell to 1.5 GB.
3. The fleet wait stayed at 0 of 703.
