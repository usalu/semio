# Round 14 — g1-m08 — superseded while waiting for replica agreement

No native round-14 try started. The explicit prerequisite, replica agreement, was not met before round 15 arrived. There was no running native try to interrupt. Last actual native result remains round 13 try 2: native_rc=75, ready=false; no round-14 native exit code, task result, phase time or native RSS peak exists. Round 15 retains the agreement gate, so it is still pending and no round-15 native try has started. No replica or build source was repaired or modified.

Observed machine now:

~~~json
{
  "os": "ProductName:\t\tmacOS\nProductVersion:\t\t27.2\nBuildVersion:\t\t26B123",
  "memory_bytes": "",
  "free_disk": "Filesystem   1024-blocks      Used Available Capacity iused      ifree %iused  Mounted on\n/dev/disk3s5   239362496  90000000 109200000    46%  836240 1092073280    0%   /System/Volumes/Data"
}
~~~

Replica code digest observed in health: d399114215f574cd93e191090e959fc4f901cfe0f2cd44963ab8d2b8d4046284; process 30350.

The first supported replica status command started about 02:08:22 UTC, produced ordinary status but no fleet verdict for 19 minutes, and was ended as documented in round-15-status-retry-reason.json. Only that own read-only diagnostic PID was targeted. The next check completed and reported no agreement. Exact periodic outputs and verdicts are in round-15-agreement-check-*.txt and round-15-agreement-checks.json. Latest completed check below. No native tail exists for round 14 because the bootstrap was never started.

~~~text
Replica g1-m08 of group g1, store /Volumes/semio-store: reachable
Synchronization: on
Daemon: not running
284137 files tracked; queued: 10 incoming, ? outgoing; 27 overwritten files kept in .cache/f/lost
Journal: 12.61 MiB in 14 segments of 10 origins.
Newest snapshot by g1-m08: 719s old, 90.49 MiB.
Reclaimable now: 4 journal segments (4.89 MiB).
Last compaction 62879s ago: 861 blobs (27.00 GiB) removed; 88 blobs (129.23 MiB) were still too young.
Git state: shared (stage and commits replicate); stage tree 5c75c44d73137807925a4f37dfbb0a95529a546e.
Background tiers, last pass (cost, duty cycle of one core): hot 5 ms 1.5%, directory sweep 930 ms 10%, file sweep 1776 ms 1.8%, tree walk -, git view 31770 ms 5%.
Latency over the last 7 publishes and 100 applies: publish p50 2801 ms, p95 130347 ms; apply p50 341 ms, p95 2740 ms.
g1-m01: 0 events (0 B) behind, last event 141s ago
g1-m02: 0 events (0 B) behind, last event 200s ago
g1-m03: 0 events (0 B) behind, last event 136s ago
g1-m04: 0 events (0 B) behind, last event 149s ago
g1-m05: 0 events (0 B) behind, last event 3726s ago
g1-m06: 0 events (0 B) behind, last event 7919s ago
g1-m07: 0 events (0 B) behind, last event 137s ago
g1-m08: 0 events (0 B) behind, last event 184s ago
g1-m10: 0 events (0 B) behind, last event 261s ago
g1-server: 0 events (0 B) behind, last event 571s ago
Fleet: origin platform age lag (origins behind/queued) basis (cp:s = compared at its checkpoint taken s seconds ago, now = compared as it is now) sync code view (modified/deleted/untracked) latency (publish p50/p95 ms | apply p50/p95 ms) verdict
  g1-m01 macos 143s 2/0 cp:143s on d399114215f5 163/12/1343 193/5790|261/4566 stale (attestation too old)
  g1-m02 macos 144s 2/0 cp:144s on d399114215f5 163/12/1342 270/2724|228/4031 stale (attestation too old)
  g1-m03 macos 139s 0/0 cp:139s on d399114215f5 163/12/1343 188/3273|307/4018 stale (attestation too old)
  g1-m04 linux 150s 3/0 cp:150s on d399114215f5 163/11/1341 176/2002|347/4713 stale (attestation too old)
  g1-m05 windows 8s 0/0 cp:8s on d399114215f5 - -/-|-/- lagging (not settled: no common checkpoint within 60 s yet, still moving)
  g1-m06 macos 141s 1/0 cp:141s on d399114215f5 163/12/1341 -/-|168/1224 stale (attestation too old)
  g1-m07 macos 140s 1/0 cp:140s on d399114215f5 163/12/1343 209/5134|423/6542 stale (attestation too old)
  g1-m08 macos 134s 0/0 cp:134s on d399114215f5 163/12/1342 2801/130347|341/2740 stale (attestation too old)
  g1-m10 macos 4s 0/0 cp:4s on d399114215f5 163/12/1428 -/-|-/- lagging (not settled: no common checkpoint within 60 s yet, still moving)
  g1-server macos 0s 0/0 cp:0s on d399114215f5 163/12/1341 83181/151831|196/465 lagging (not settled: no common checkpoint within 60 s yet, still moving)
The fleet is not in agreement (10 origins; fresh and checkpoint age mean within 60 s; lagging is not wrong, only not yet comparable).

~~~
