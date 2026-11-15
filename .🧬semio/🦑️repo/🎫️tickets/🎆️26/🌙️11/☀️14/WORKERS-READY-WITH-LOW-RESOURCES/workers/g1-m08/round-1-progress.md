# Progress Archive — Round 1 Finished

Final report: [round-1.md](round-1.md). Logged native_rc=1; ready=false.

# Round 1 In Progress — g1-m08

Updated: 2026-11-14T11:31:05Z. The single foreground bootstrap is still running; `native_rc` and completion-marker readiness are not yet measured. No hard-stop trigger has been observed.

- Bootstrap start: 2026-11-14T11:03:41Z.
- Latest graph-slice heartbeat: 709 seconds.
- Largest RSS observed: 5235856 KiB (4.993 GiB).
- Latest system memory-pressure level: 1 (1 = normal).
- RSS samples collected: 52, approximately 30 seconds apart.
- Sampling began at 2026-11-14T11:04:54Z, 73.0 seconds after the bootstrap log start. Earlier RSS peaks and exact early-phase timings are not measured.

The monitor checks the native log continuously for the round's project-count limits, samples RSS every 30 seconds, and checks kernel memory pressure every five seconds. It will send only one SIGINT to this attempt's own process group and wait one minute if a stated hard-stop condition occurs.

`round-1.md`, its last-200-line tail, and `state.json` will be written when the attempt ends. Raw measurements are in `round-1-ps.jsonl` and observed phase changes in `round-1-phases.jsonl`.

## Replica Status Observation

Observed at 2026-11-14T11:41:21Z. Native bootstrap is still running; native_rc remains not measured. The status command reports daemon not running, so these worker reports may remain local until the replica resumes. No daemon repair, restart, code change or sudo was attempted under this protocol. The currently running daemon code digest is not measured because no daemon is reported running; the last attested digest below is stale.

```
Replica g1-m08 of group g1, store /Volumes/semio-store: reachable
Synchronization: on
Daemon: not running
278647 files tracked; queued: 10 incoming, ? outgoing; 26 overwritten files kept in .cache/f/lost
Journal: 5.31 MiB in 11 segments of 9 origins.
Newest snapshot by g1-m01: 668s old, 89.17 MiB.
Reclaimable now: 0 journal segments (0 B).
Last compaction 8810s ago: 861 blobs (27.00 GiB) removed; 88 blobs (129.23 MiB) were still too young.
Git state: shared (stage and commits replicate); stage tree 59f8d6e4d9b1935a6afc28cfa10608ad9c946325.
Background tiers, last pass (cost, duty cycle of one core): hot 22 ms 6.7%, directory sweep 910 ms 10%, file sweep 3083 ms 2.9%, tree walk 27521 ms 1.9%, git view 30640 ms 5%.
  g1-m08 macos 2661s 8/0 cp:2661s on 1c863927612f 33/12/36 -/-|129949/574785 stale (attestation too old)
```
