# Measurement: Where Worker Memory Goes (2026-11-24, 02:20–02:45)

Sources: `/Users/Shared/semio-store/v1/origins/<origin>/{ready.json,ready-timings.json,pool.json,slice-memory.json,service-failure-*.txt}`, `repo fleet status --json` sampled every 60 s (the series was a generated file and is deleted with the ticket; the later per-worker series comes from `sample-warden.py`), the coordinator's own process table.

## Limits of what can be seen today

- No worker publishes its available memory, swap or compressor state. The fleet row has `pressure` and `headroom` only; the group store has the stage peak, the pool's own size and the slice worker's heap report.
- The fleet load endpoint answered in 5 of 19 one-minute samples (the reader gives up after 5 s; coordinator load 50+). A worker that is out of memory is therefore invisible exactly when it matters.
- No record of an OS intervention exists anywhere in the repo's records: a paused or jetsam-killed process appears as a "vanished" or "stalled" stage.

## Pressure seen in 21 minutes (5 answered samples)

| worker | pressure / refusal reasons observed |
|---|---|
| g1-m02 | 1× `critical` (refusing: pressure-critical) |
| g1-m12 | 2× `critical`, 1× swap-activity |
| g1-m08 | 3× `warn`, 2× no headroom (memory) |
| g1-m10 | 3× `warn` (2× refusing: pressure-warn) |
| g1-m01 | 3× `warn` |
| g1-m07 | 2× no headroom (memory) |
| g1-m03, g1-m13 | 1× no headroom (memory / "needs 1.0 GiB") |
| g1-m04 (Linux) | swap, swap-activity, 1× `warn` |
| g1-m06 | 1× `warn` |

Six of eleven Macs were at `warn` or `critical` at least once within five readings; two reached `critical`, the level at which macOS starts its own measures.

## What holds the memory on a 16 GB worker

| holder | measured | admitted by | acted on while running |
|---|---|---|---|
| graph slice worker (project-graph read) | 2.2–4.8 GB resident (`slice-memory.json`, 11 workers), heap+external 2.4 GB | plan guard: limit ≈ 4.9 GB, ceiling 9.8 GB (60 % of host), no budget check on macOS | no |
| heavy-question children | cap 2560 MB each, 2 lanes | nothing (outside every grant) | self-exit 137 at the cap |
| farm workers | 1536 MB reservation each | farm reservation | recycle at 1.5× baseline |
| fleet owner-pool puller children | up to 6 × 0.7–1.3 GB (`pool.json`: g1-m13 6 children, 4.2 GB; g1-m04 6 children, 3.6 GB) | pool grant 4915 MB (30 % of host) | shed counter exists, 0 sheds recorded |
| Ready stage tree (when it executes work) | peaks 5.5–8.6 GB (`ready-timings.json`), all-time 10.7 GB (`ready.json`) | stage claim (p90 × 1.25, at most 50 % of host) | no (stall and timeout kills only) |
| lent remote unit | one at a time while the stage runs; component finals 0.3–6.5 GB | scheduler placement + worker gate | no |
| replica daemon, cache sidecar, launchers | not measured per worker | none | no |

The sum of the admitted ceilings alone (read 9.8 GB + pool 4.9 GB + stage up to 8 GB + one final up to 6.5 GB) is far above 16 GB; each gate only sees what is resident at the moment it admits, and none of them can take memory back.

## State of the Ready stage on the workers (context, not memory)

`ready-timings.json`, last 40 runs per worker: the `workspace` stage ended `stalled` 14–33 times per worker and `ok` 0–2 times; several workers show a graph read running for hours (`Reading the project graph slice (30345 s)` on g1-m13). A complete, successful stage run is rare tonight for reasons outside memory, which limits how a "previously failing run now completes" proof can be taken.

## Coordinator (32 GB) at the same time

Available 10–12 GB, swap 1.7 GB of 3 GB used, OS pressure level 1, compressor 7.2 GB (22 % of physical). Largest repo processes: a graph read child 2.6 GB, a resident service 1.4 GB, the cache sidecar 0.7 GB.
