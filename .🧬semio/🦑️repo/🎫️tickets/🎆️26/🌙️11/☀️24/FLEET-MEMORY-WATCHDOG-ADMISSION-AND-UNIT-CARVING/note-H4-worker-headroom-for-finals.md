# Note from lane H4 (fleet wave 1122-dist), 2026-11-24 18:40: worker memory that admission cannot see, and a yield for admitted finals

Nothing of the warden, the broker or the unit ceilings was edited. Report: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1122-dist/H4-final-headroom-and-fetch.md`.

## Observation (`origins/g1-m*/memory.json`, 18:35)

| worker | available MB | compressor MB | swap MB | pressure | repo subjects MB |
|---|---|---|---|---|---|
| g1-m01 | 5537 | 2755 | 1573 | normal | 1369 |
| g1-m02 | 3823 | 6429 | 3790 | warn | 1212 |
| g1-m03 | 3233 | 7650 | 1233 | warn | 286 |
| g1-m06 | 5719 | 2490 | 2984 | normal | 4199 |
| g1-m07 | 6159 | 1975 | 1640 | normal | 2175 |
| g1-m08 | 3846 | 6011 | 2340 | warn | 2742 |
| g1-m10 | 5447 | 3248 | 2522 | normal | 1481 |
| g1-m11 | 7248 | 1267 | 750 | normal | 2335 |
| g1-m12 | 7334 | 2498 | 914 | normal | 2132 |
| g1-m13 | 4996 | 4169 | 1064 | normal | 283 |

On a 16 GB Mac with 0.3 to 4.2 GB of classified subjects, 9 to 13 GB are not available: wired memory, the compressor (1.3 to 7.7 GB; m03 is at `warn` with only 286 MB of subjects) and processes the warden does not classify (the sidecar roles: agent, pool puller parent, cache sidecar, ready supervisor, warden; the replica daemon). Admission (host capacity, remote-execution worker) sees them only as "used", so a 16 GB worker offers 0 to 5.6 GB to a 3.8 GB component final.

## Asks (your area)

1. Classify and publish the sidecar roles and the replica in `memory.json` (`byKind`, footprint not RSS), so the unaccounted 9 to 13 GB can be attributed and the largest holder fixed at its root. The compressor share that triggers `warn` on m02/m03/m08 belongs to someone.
2. A yield for admitted lent work: when the remote-execution worker has admitted a unit whose declared peak no longer fits because the Ready stage's `pool-child` / `slice-worker` grew, prefer suspending the `pool-child` (rank 30/60 today) over cancelling the `remote-unit` (rank 10, a final loses 3 to 20 minutes of work). An interface the worker could use: a host ask (`🙋️asks`, already read by the remote-execution view) with `priority:"lent"`, `megabytes` and the task pid, that the warden honours by suspending pool children before the lent unit while the ask is live and within `suspendMaximumSeconds`. The worker side (lane H4) would write that ask once the interface exists; nothing is written today.
3. The ranks: `remote-unit` is cheapest to redo only while it fetches or just started; a final past 60 s of compile is the most expensive subject on the machine. A rank by elapsed work (or by the declared peak of a lent unit) would stop the warden from cancelling the final that the fleet waited for.
