# g1-m10 Fleet Join — Current Outcome

Recorded: 2026-11-15T03:38:03+01:00. Account ueli remains logged in.

The Mac is joined as g1-m10 of g1 at /Users/ueli/Documents/semio. Synchronization is on and the replica daemon is running. The supported provisioner resume exited 3 (fleet ready; native build pending). Full acceptance has not passed: verify reports three Git acceleration-path differences, and the dedicated fleet comparison exceeded its wait budget.

## Machine And Repository

macOS 27.2.1 (26B131); 17179869184 bytes / 16 GiB RAM. Computer: Ueli’s Mac mini; LAN address 192.168.178.139. Login/console owner: ueli (UID 501). Step 1 had 159 GiB free; after setup 139 GiB remains, above the new 60 GiB floor.

Initial and final HEAD: 5ececaa78b9265fd72b4f7e8f411afac3c9e1df1. Commit date at step 1: Thu Nov 12 22:14:18 2026 +0100. Step 1 status paths: 1; stashes: 0. Final rev-parse exited 0.

~~~text
Repository: /Users/ueli/Documents/semio
Qualifying repositories in accessible locations: 1
Protected-directory errors:      158
/Users/Shared/semio does not exist
~~~

The full depth-4 search was attempted. Protected-directory permission errors remain explicitly unverified; only one qualifying repository was found in accessible locations. /Users/Shared/semio does not exist.

## Authorized Fleet Module Copy

Official kit receipt publishedAt: 2026-11-14T23:08:05Z (later than the required 22:02:14Z); all ten SHA256SUMS entries passed.

Source selection: attested, without the newest-folder fallback. Source: /Volumes/semio-kit/v1/bootstrap/code/eabbc92267329dbd39d8d05e0c8eceb1bcad083c0eea58c44805408a7d547bcc/files.

Copied as UID 501: replaced 16, added 36, identical 10 (62 files total). Originals are retained at /Users/ueli/Documents/semio/.cache/f/lost/provision-code-20261115-005015. No file was deleted by this code-copy operation.

The facts check exited 0 and printed 25 FLEET_ lines. The stock provisioner subsequently verified the same sealed code and reported 0 replaced, 0 added, 62 identical.

## Adoption And Provisioning

Adoption plan (exit 0): {"counts": {"same": 268590, "materialise": 5426, "publish": 1, "stale": 2699, "conflict": 0, "edit": 0}, "newLocalWork": 1, "gitRelation": "equal", "approved": true}. Stale paths were not misclassified as conflicts or new local work.

First actual adoption: 268590 same, 5429 materialised, 1 published, 0 edits, 2699 stale, 0 conflicts, 35 queued. Its exit 1 was reported and the provisioner’s exact supported resume instruction was followed. The remaining items were ignored .git metadata; none was manually repaired. Adoption report: .cache/f/lost/adopt-2026-11-14T23-52-54-418Z.md.

The resume recognized the existing g1-m10 identity, skipped join, started the services and exited 3. Native bootstrap setup exited 1 with 'Internal error: Exact privately produced preservation capsule required'. Its source data/capsule was retained; the completion marker was not written. No native retry loop or manual capsule recovery was performed.

Native log: /Users/ueli/Documents/semio/.cache/f/native-bootstrap.log.

## Acceptance Results

- preflight: exit 0.
- replica status: exit 0; store reachable; synchronization on; daemon running (reported PID 79508).
- verify: exit 1; 278762 shared paths checked; three drifts listed below; 87 paths refused by policy and not shared.
- fleet --wait 600: did not complete its inner comparison after the 600-second budget; only that CLI process was ended, exit 137. The replica service was not signalled.
- One fresh snapshot without waiting was also bounded to 90 seconds; it timed out without output, return code -9. It is not reported as a successful fleet check.
- Last completed status report's own fleet row:

~~~text
  g1-m10 macos 2s 4/0 cp:2s on d399114215f5 160/12/1276 101497/118805|3281/45153 ok
~~~

This observed status row was ok at its checkpoint; the dedicated fleet command did not produce a final row. Other origins were also reported as diverged/lagging. No claim is made that the entire fleet passed.

Git acceleration drift (not manually changed):

- .git/objects/info/commit-graphs/graph-cbd41fbd1eac8086cbd74379df5340da16b80dec.graph — extra locally
- .git/objects/info/packs — content differs
- .git/objects/pack/multi-pack-index — content differs

## Installed GUI Services

~~~text
		       0      0 	tech.semio.fleet.compact
		   76707      0 	tech.semio.fleet.replica
		       0      0 	tech.semio.repo.probe.3f08bbea4d32
~~~

The replica LaunchAgent was running; the conditional kickstart was not needed. The system store-mount daemon was installed by the official provisioner. No synchronization switch was changed manually.

## Check Output

### status

~~~text
Replica g1-m10 of group g1, store /Volumes/semio-store: reachable
Synchronization: on
Daemon: running (pid 79508)
284004 files tracked; queued: 10 incoming, ? outgoing; 2731 overwritten files kept in .cache/f/lost
The manifest is not verified against the tree yet; build tools check every file themselves until a sweep has completed.
Recurring failures of kind loop-blocked-restart: 2 times, last 133 s ago.
Sidecar build-agent: running (process 79863, 0 restart(s), 18 s in this state, last exit -).
Sidecar build-scheduler: off on this machine (process -, 0 restart(s), 122 s in this state, last exit -).
Sidecar build-cache: off on this machine (process -, 0 restart(s), 122 s in this state, last exit -).
Journal: 12.05 MiB in 14 segments of 10 origins.
Newest snapshot by g1-m04: 2132s old, 90.09 MiB.
Reclaimable now: 4 journal segments (4.89 MiB).
Last compaction 60552s ago: 861 blobs (27.00 GiB) removed; 88 blobs (129.23 MiB) were still too young.
Git state: shared (stage and commits replicate); stage tree 5c75c44d73137807925a4f37dfbb0a95529a546e.
Background tiers, last pass (cost, duty cycle of one core): hot 11 ms 3.5%, directory sweep -, file sweep -, tree walk -, git view 29247 ms 5%.
Latency over the last 100 publishes and 75 applies: publish p50 101497 ms, p95 118805 ms; apply p50 3375 ms, p95 45153 ms.
g1-m01: 0 events (0 B) behind, last event 2s ago
g1-m02: 1 events (398 B) behind, last event 1s ago
g1-m03: 1 events (406 B) behind, last event 2s ago
g1-m04: 0 events (0 B) behind, last event 1412s ago
g1-m05: 0 events (0 B) behind, last event 1398s ago
g1-m06: 0 events (0 B) behind, last event 5591s ago
g1-m07: 1 events (400 B) behind, last event 1s ago
g1-m08: 3 events (1.20 KiB) behind, last event 1s ago
g1-m10: 0 events (0 B) behind, last event 17s ago
g1-server: 0 events (0 B) behind, last event 6s ago
Fleet: origin platform age lag (origins behind/queued) basis (cp:s = compared at its checkpoint taken s seconds ago, now = compared as it is now) sync code view (modified/deleted/untracked) latency (publish p50/p95 ms | apply p50/p95 ms) verdict
  g1-m01 macos 0s 1/0 cp:0s on d399114215f5 163/12/1215 191/13291|616/1035 diverged
  g1-m02 macos 0s 0/0 cp:0s on d399114215f5 163/12/1215 298/11723|635/974 diverged
  g1-m03 macos 0s 0/0 cp:0s on d399114215f5 160/12/1214 201/5968|592/5065 diverged
  g1-m04 linux 0s 0/0 cp:0s on d399114215f5 163/11/1214 -/-|705/902 diverged
  g1-m05 windows 21s 7/0 cp:21s on d399114215f5 - -/-|9652/19652 ok
  g1-m06 macos 0s 1/0 cp:0s on d399114215f5 160/12/1212 -/-|488/860 ok
  g1-m07 macos 0s 0/0 cp:0s on d399114215f5 163/12/1215 402/25936|627/1091 ok
  g1-m08 macos 0s 1/0 cp:0s on d399114215f5 163/12/1215 879/5088|1063/2147 diverged
  g1-m10 macos 2s 4/0 cp:2s on d399114215f5 160/12/1276 101497/118805|3281/45153 ok
  g1-server macos 0s 0/0 cp:0s on d399114215f5 160/12/1212 267/2220|487/1009 ok
  warning: g1-m04 runs git 2.55.0, other machines run another version
  warning: the volume of g1-m04 behaves differently from most (case=sensitive normalisation=sensitive); the view is pinned, so this is a capability difference, not a divergence
  warning: g1-m06 runs git 2.50.1, other machines run another version
  warning: g1-m02 cannot resolve HEAD of its repository while the fleet has one (config-unreadable); its daemon repairs the shared git files from the store, a git client shows nothing until it has
  warning: g1-m03 cannot resolve HEAD of its repository while the fleet has one (config-unreadable); its daemon repairs the shared git files from the store, a git client shows nothing until it has
  warning: g1-m08 cannot resolve HEAD of its repository while the fleet has one (config-unreadable); its daemon repairs the shared git files from the store, a git client shows nothing until it has
  g1-m01 differs in view.digest at a common checkpoint: #23=b406 instead of #23=19bd (#n = bucket n of 256 of the paths, value = its digest)
  g1-m02 differs in view.digest at a common checkpoint: #153=5d46 instead of #153=9aed (#n = bucket n of 256 of the paths, value = its digest)
  g1-m03 differs in view.digest at a common checkpoint: #76=9958,#237=d8a0 instead of #76=254a,#237=b887 (#n = bucket n of 256 of the paths, value = its digest)
  g1-m04 differs in view.digest at a common checkpoint: #16=10f3,#52=859d,#86=80bb,#114=af69,#121=b75f,#147=c850,#163=2674,#188=d72b,#21 instead of #16=3c62,#52=aeaa,#86=2bba,#114=6afa,#121=0786,#147=1a14,#163=f71e,#188=5b64,#21 (#n = bucket n of 256 of the paths, value = its digest)
  g1-m02 differs in head.commit at a common checkpoint: - instead of 5ececaa78b9265fd72b4f7e8f411afac3c9e1df1 (#n = bucket n of 256 of the paths, value = its digest)
  g1-m03 differs in head.commit at a common checkpoint: - instead of 5ececaa78b9265fd72b4f7e8f411afac3c9e1df1 (#n = bucket n of 256 of the paths, value = its digest)
  g1-m08 differs in head.commit at a common checkpoint: - instead of 5ececaa78b9265fd72b4f7e8f411afac3c9e1df1 (#n = bucket n of 256 of the paths, value = its digest)
The fleet is not in agreement (10 origins; fresh and checkpoint age mean within 60 s; lagging is not wrong, only not yet comparable).
~~~

### verify

~~~text
{"version":1,"event":"progress","stage":"fold","done":1,"total":11}
{"version":1,"event":"progress","stage":"fold","done":2,"total":11}
{"version":1,"event":"progress","stage":"fold","done":3,"total":11}
{"version":1,"event":"progress","stage":"fold","done":4,"total":11}
{"version":1,"event":"progress","stage":"fold","done":5,"total":11}
{"version":1,"event":"progress","stage":"fold","done":6,"total":11}
{"version":1,"event":"progress","stage":"fold","done":7,"total":11}
{"version":1,"event":"progress","stage":"fold","done":8,"total":11}
{"version":1,"event":"progress","stage":"fold","done":10,"total":11}
{"version":1,"event":"progress","stage":"fold","done":11,"total":11}
{"version":1,"event":"progress","stage":"verify","done":500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":1000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":1500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":2000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":2500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":3000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":3500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":4000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":4500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":5000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":5500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":6000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":6500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":7000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":7500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":8000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":8500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":9000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":9500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":10000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":10500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":11000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":11500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":12000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":12500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":13000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":13500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":14000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":14500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":15000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":15500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":16000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":16500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":17000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":17500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":18000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":18500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":19000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":19500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":20000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":20500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":21000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":21500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":22000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":22500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":23000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":23500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":24000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":24500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":25000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":25500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":26000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":26500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":27000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":27500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":28000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":28500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":29000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":29500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":30000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":30500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":31000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":31500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":32000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":32500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":33000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":33500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":34000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":34500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":35000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":35500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":36000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":36500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":37000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":37500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":38000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":38500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":39000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":39500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":40000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":40500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":41000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":41500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":42000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":42500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":43000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":43500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":44000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":44500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":45000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":45500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":46000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":46500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":47000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":47500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":48000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":48500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":49000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":49500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":50000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":50500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":51000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":51500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":52000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":52500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":53000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":53500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":54000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":54500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":55000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":55500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":56000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":56500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":57000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":57500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":58000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":58500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":59000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":59500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":60000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":60500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":61000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":61500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":62000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":62500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":63000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":63500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":64000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":64500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":65000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":65500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":66000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":66500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":67000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":67500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":68000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":68500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":69000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":69500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":70000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":70500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":71000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":71500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":72000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":72500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":73000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":73500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":74000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":74500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":75000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":75500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":76000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":76500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":77000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":77500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":78000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":78500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":79000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":79500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":80000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":80500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":81000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":81500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":82000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":82500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":83000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":83500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":84000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":84500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":85000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":85500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":86000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":86500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":87000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":87500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":88000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":88500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":89000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":89500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":90000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":90500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":91000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":91500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":92000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":92500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":93000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":93500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":94000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":94500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":95000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":95500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":96000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":96500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":97000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":97500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":98000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":98500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":99000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":99500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":100000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":100500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":101000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":101500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":102000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":102500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":103000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":103500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":104000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":104500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":105000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":105500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":106000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":106500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":107000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":107500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":108000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":108500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":109000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":109500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":110000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":110500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":111000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":111500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":112000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":112500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":113000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":113500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":114000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":114500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":115000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":115500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":116000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":116500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":117000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":117500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":118000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":118500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":119000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":119500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":120000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":120500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":121000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":121500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":122000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":122500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":123000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":123500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":124000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":124500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":125000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":125500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":126000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":126500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":127000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":127500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":128000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":128500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":129000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":129500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":130000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":130500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":131000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":131500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":132000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":132500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":133000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":133500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":134000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":134500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":135000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":135500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":136000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":136500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":137000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":137500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":138000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":138500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":139000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":139500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":140000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":140500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":141000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":141500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":142000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":142500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":143000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":143500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":144000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":144500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":145000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":145500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":146000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":146500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":147000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":147500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":148000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":148500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":149000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":149500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":150000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":150500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":151000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":151500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":152000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":152500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":153000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":153500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":154000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":154500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":155000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":155500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":156000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":156500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":157000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":157500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":158000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":158500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":159000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":159500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":160000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":160500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":161000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":161500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":162000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":162500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":163000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":163500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":164000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":164500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":165000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":165500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":166000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":166500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":167000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":167500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":168000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":168500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":169000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":169500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":170000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":170500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":171000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":171500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":172000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":172500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":173000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":173500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":174000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":174500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":175000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":175500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":176000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":176500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":177000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":177500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":178000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":178500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":179000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":179500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":180000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":180500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":181000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":181500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":182000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":182500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":183000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":183500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":184000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":184500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":185000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":185500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":186000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":186500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":187000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":187500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":188000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":188500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":189000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":189500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":190000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":190500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":191000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":191500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":192000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":192500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":193000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":193500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":194000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":194500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":195000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":195500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":196000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":196500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":197000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":197500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":198000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":198500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":199000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":199500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":200000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":200500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":201000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":201500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":202000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":202500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":203000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":203500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":204000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":204500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":205000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":205500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":206000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":206500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":207000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":207500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":208000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":208500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":209000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":209500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":210000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":210500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":211000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":211500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":212000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":212500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":213000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":213500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":214000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":214500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":215000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":215500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":216000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":216500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":217000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":217500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":218000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":218500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":219000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":219500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":220000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":220500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":221000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":221500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":222000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":222500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":223000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":223500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":224000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":224500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":225000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":225500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":226000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":226500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":227000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":227500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":228000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":228500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":229000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":229500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":230000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":230500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":231000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":231500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":232000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":232500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":233000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":233500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":234000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":234500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":235000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":235500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":236000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":236500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":237000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":237500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":238000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":238500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":239000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":239500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":240000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":240500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":241000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":241500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":242000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":242500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":243000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":243500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":244000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":244500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":245000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":245500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":246000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":246500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":247000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":247500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":248000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":248500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":249000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":249500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":250000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":250500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":251000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":251500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":252000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":252500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":253000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":253500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":254000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":254500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":255000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":255500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":256000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":256500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":257000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":257500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":258000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":258500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":259000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":259500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":260000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":260500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":261000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":261500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":262000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":262500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":263000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":263500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":264000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":264500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":265000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":265500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":266000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":266500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":267000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":267500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":268000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":268500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":269000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":269500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":270000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":270500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":271000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":271500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":272000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":272500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":273000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":273500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":274000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":274500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":275000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":275500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":276000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":276500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":277000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":277500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":278000,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":278500,"total":278761}
{"version":1,"event":"progress","stage":"verify","done":278761,"total":278761}
Drift on 3 paths (278762 shared paths checked).
  extra locally: .git/objects/info/commit-graphs/graph-cbd41fbd1eac8086cbd74379df5340da16b80dec.graph
  content differs: .git/objects/info/packs
  content differs: .git/objects/pack/multi-pack-index
87 paths are refused by the path policy and not shared:
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/wbs-tools/sb-config/🎨️styling
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/basetree/🧰️framework/🔨️modules
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🔨️modules
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⏱️profile
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎯️host-platform
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎯️targets
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👷️workers
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📊️reporting
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📐️conformance
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️storybook
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️texts
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📤️arguments
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔌️backends
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔒️leases
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/host-capacity
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/⚙️operations
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🌐️platform
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🌐️remote-cache
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🎬️executor
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🎬️scopes
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🎯️affected
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🐹️go
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/👁️evidence
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/👷️workers
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/💾️storage
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📋️task-plan
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📐️conformance
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📐️path-plan
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📸️snapshot
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🔁️probe
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🔌️backend
  unsafe-link: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🔗️closure
  ... and 37 more
~~~

### fleet

~~~text
Waiting for agreement: 8 origins pending, 599 s left.
Waiting for agreement: 8 origins pending, 597 s left.
Waiting for agreement: 8 origins pending, 595 s left.
Waiting for agreement: 8 origins pending, 592 s left.
Waiting for agreement: 6 origins pending, 590 s left.
Waiting for agreement: 6 origins pending, 588 s left.
Waiting for agreement: 4 origins pending, 587 s left.
Waiting for agreement: 4 origins pending, 585 s left.
Waiting for agreement: 5 origins pending, 583 s left.
Waiting for agreement: 5 origins pending, 580 s left.
Waiting for agreement: 7 origins pending, 578 s left.
Waiting for agreement: 6 origins pending, 576 s left.
Waiting for agreement: 6 origins pending, 574 s left.
Waiting for agreement: 6 origins pending, 572 s left.
Waiting for agreement: 6 origins pending, 570 s left.
Waiting for agreement: 6 origins pending, 568 s left.
Waiting for agreement: 6 origins pending, 565 s left.
Waiting for agreement: 6 origins pending, 563 s left.
Waiting for agreement: 6 origins pending, 561 s left.
Waiting for agreement: 5 origins pending, 558 s left.
Waiting for agreement: 5 origins pending, 556 s left.
Waiting for agreement: 5 origins pending, 554 s left.
Waiting for agreement: 5 origins pending, 552 s left.
Waiting for agreement: 6 origins pending, 550 s left.
Waiting for agreement: 6 origins pending, 548 s left.
Waiting for agreement: 6 origins pending, 546 s left.
Waiting for agreement: 6 origins pending, 543 s left.
Waiting for agreement: 7 origins pending, 541 s left.
Waiting for agreement: 9 origins pending, 539 s left.
Waiting for agreement: 9 origins pending, 537 s left.
Waiting for agreement: 8 origins pending, 534 s left.
Waiting for agreement: 8 origins pending, 532 s left.
Waiting for agreement: 8 origins pending, 530 s left.
Waiting for agreement: 8 origins pending, 528 s left.
Waiting for agreement: 8 origins pending, 526 s left.
Waiting for agreement: 9 origins pending, 523 s left.
Waiting for agreement: 8 origins pending, 521 s left.
Waiting for agreement: 8 origins pending, 519 s left.
Waiting for agreement: 8 origins pending, 517 s left.
Waiting for agreement: 8 origins pending, 514 s left.
Waiting for agreement: 8 origins pending, 512 s left.
Waiting for agreement: 8 origins pending, 510 s left.
Waiting for agreement: 8 origins pending, 508 s left.
Waiting for agreement: 8 origins pending, 506 s left.
Waiting for agreement: 8 origins pending, 503 s left.
Waiting for agreement: 8 origins pending, 500 s left.
Waiting for agreement: 9 origins pending, 498 s left.
~~~

## Retained Evidence And Next Phase

Official finish logs: /Users/ueli/semio-setup/runs/20261115-004830-official-finish. Resume/check logs: /Users/ueli/semio-setup/runs/20261115-010448-official-resume. Full preflight output is retained in preflight.log (exit 0). The declined fallback was never executed and its entry scripts were deleted. No modifying Git command, pkill, killall, token generation/copy/printing, git GUI action, or manual repository-code patch was used. Code changes were the authorized published module copy, normal fleet adoption/synchronization, and the provided bootstrap's generated state.

Temporary-kit cleanup completed: ~/semio-setup/topology and the unused fallback files are removed; logs and authorized helper inputs are retained. Readiness-round outcome will be reported only inside workers/g1-m10 after that transition, as instructed.

## Earlier Attempt History

# g1-m10 Fleet Join

Current status: joined as g1-m10 in g1; synchronization is on and the replica daemon is running. The supported provisioner resume exited 3 (fleet ready, native build pending). Preflight and status exited 0. Verification exited 1 on three Git acceleration paths; the explicit 600-second fleet check is still running. The latest status attestation listed g1-m10 as ok. Readiness rounds have not yet begun.

Native bootstrap failed with `Internal error: Exact privately produced preservation capsule required`. Its source data was retained; no manual recovery or native retry loop was performed. Full logs remain in the official-resume run directory recorded below.

## Earlier Attempts And Evidence

The first attempts stopped at an incompatible adoption plan. The entries below preserve that history; the current status above supersedes their old outcome statements.

## Machine And Preconditions

- Account: `ueli` (UID 501), matching the logged-in console account; left logged in.
- Computer name: Ueli’s Mac mini; local hostname: `Uelis-Mac-mini`.
- Addresses: `en1` IPv4 `192.168.178.139`; loopback `127.0.0.1`.
- macOS: 27.2.1, build 26B131.
- Memory: 17179869184 bytes (16 GiB).
- Home volume free: 159 GiB, satisfying the 90 GiB requirement.
- `showmount -e 192.168.178.70` listed `/Users/Shared/semio-store` for `192.168.178.0`.
- Sudo was authenticated by the owner in Terminal. The provision script re-executed using `sudo -E`.

## Repository Inventory

The adoption case of step 2 applied: `/Users/ueli/Documents/semio` holds `.git`, `AGENTS.md`, and `.🧬semio`. Home and `/Users/Shared` were searched to depth 4. Exactly one qualifying repository was found in accessible locations, at the required path. `/Users/Shared/semio` does not exist.

macOS protected-directory permission errors prevent claiming complete coverage of inaccessible directories. The independent agent inventory recorded 151 such errors; full inventory and permission outputs remain with the run logs.

Before writing this report:

- HEAD: `5ececaa78b9265fd72b4f7e8f411afac3c9e1df1`.
- `git --no-optional-locks status --porcelain | wc -l`: `0`.
- `git --no-optional-locks stash list | wc -l`: `0`.

Only read-only Git commands were used. No Git GUI was opened by this agent.

## Kit And Origin

The previously selected origin `g1-m10` was retained. It was not listed in the live store, so the uniqueness check passed. The origin remains proposed; no local fleet identity was created.

The provisioning kit was refreshed from the read-only NFS mount, which was then unmounted. All eight entries in `SHA256SUMS` reported `OK`. The repository provision script differed from the kit; the verified kit was used as instructed.

Kit provision script SHA-256: `f491310e4127492951ef6c01ed978bd95d9ea1df7501e0ceab3e14656494fee3`.

The required language, group, NFS host and network were exported. Cache endpoint overrides and the remote cache token variable were unset. No token was generated, copied, printed or stored.

## Plan Failure

Adoption-plan exit code: `1`. Real provision exit code: not applicable (not started). Adoption counts and git relation were not produced.

The plan reached step 3, standalone launcher preparation, and failed with `Unknown option for facts: --home`. Per the exit-1 instruction, no repair or retry was attempted.

The following is the entire plan log (fewer than 40 lines):

```text
[fleet-provision] not root: re-running through sudo
[fleet-provision] group g1, origin g1-m10, platform macos, account ueli
[fleet-provision] step 1: canonical worktree /Users/ueli/Documents/semio
[fleet-provision] the worktree already holds a repository: it is adopted into the fleet (nothing is refused, every difference is kept in .cache/f/lost and reported), the archive is skipped
[fleet-provision] step 3: standalone repo launcher (built from the checkout; the native bootstrap comes last)
[fleet-provision] repo launcher present for ueli
[fleet-provision] linked /usr/local/bin/repo for the service definitions
Unknown option for facts: --home
```

## Acceptance And Services

Preflight, replica status, verify and fleet acceptance were not run because the adoption plan failed before join. No fleet line is available for `g1-m10`.

`launchctl print gui/501 | grep semio` returned no matching services. The plan confirmed the existing account launcher and linked `/usr/local/bin/repo`; it did not reach fleet service installation or fleet join.

Readiness rounds were not entered: step 4 required stopping on this error. No repository code, `AGENTS.md`, `fleet-setup.md`, or topology JSON was edited. The copied setup topology was removed after the attempt; logs and setup command inputs were retained.

## Retained Logs

Run directory: `/Users/ueli/semio-setup/runs/20261114-165240`

- Session: `/Users/ueli/semio-setup/runs/20261114-165240/session.log`
- Plan: `/Users/ueli/semio-setup/runs/20261114-165240/plan.log`
- Plan exit: `/Users/ueli/semio-setup/runs/20261114-165240/plan.exit`
- Checksums: `/Users/ueli/semio-setup/runs/20261114-165240/checksums.log`
- Origin listing: `/Users/ueli/semio-setup/runs/20261114-165240/existing-origins.log`
- Aggregate plan log: `~/semio-setup/plan.log`

Recorded: 2026-11-14 16:55:13 (Europe/Berlin).

## Continuing Attempt — 2026-11-14 18:55:59

At the user's request, the worker session remains active and is checking for a compatible coordinator-published kit. The owner authenticated sudo again. The fresh kit passed all checksums and still has provision-script hash `f491310e4127492951ef6c01ed978bd95d9ea1df7501e0ceab3e14656494fee3`. `g1-m10` remains absent from the live origin listing.

The fresh adoption plan again exited 1 at step 3 with `Unknown option for facts: --home`. No real provisioning or native bootstrap was started.

Read-only source checks establish a broader bootstrap-version incompatibility:

- The kit's provision script invokes topology and replica commands from the existing checkout, rather than from a self-contained current runtime.
- The checkout's topology `facts` parser does not accept `--home`.
- The checkout's committed macOS worktree is `/Users/Shared/semio`; the published kit specifies `~/Documents/semio`.
- The checkout's replica `join` parser has no `--adopt` or `--dry-run` flags; its operation set has no `fleet` command.
- The copied kit is not a standalone runtime: its TypeScript modules import repository files that are not included among the eight kit manifest entries.

Removing `--home` would therefore not solve adoption and could select the wrong worktree. No option was stripped, no topology was edited, and no repository code or Git state was changed to work around this incompatibility.

Coordinator prerequisite: publish a bootstrap/adoption runtime capable of upgrading this existing checkout, or deliver the compatible fleet modules through an authorized mechanism. The worker can then run the real dry run, enforce the 50-conflict/1000-new-path/equal-or-ancestor gates, and proceed with join and readiness rounds. Publishing the same eight-file kit again does not supply the missing adoption implementation.

The session checks for published kit updates without repeating an unchanged failing plan. This is not a retry loop for native builds. Membership and all acceptance checks remain pending.

Continuing run logs: `/Users/ueli/semio-setup/runs/20261114-170009-continued`; failed plan: `/Users/ueli/semio-setup/runs/20261114-170009-continued/plan-2.log`; refreshed kit check: `/Users/ueli/semio-setup/runs/20261114-170009-continued/fetch-3.log`.

## Prepared Alternative — Not Executed

A reviewable fallback is prepared outside the repository at `~/semio-setup/stage-published-runtime.command`, with its exact provisioner changes in `~/semio-setup/runtime-adapter-preview.diff`. The code that prepares that copy is `~/semio-setup/prepare-runtime-adapter.ts`.

The fallback copies the coordinator-published bootstrap archive over the read-only NFS mount and verifies its manifest SHA-256 and file count. It extracts only framework tool sources and root manifests into a short user tool-cache directory `~/.st-<archive-sha-prefix>`. It does not extract `.git`, root `AGENTS.md`, or `.🧬semio`, and does not create another repository. It refuses an archive lacking the adoption/dry-run implementation.

An adjacent copy of the provisioner runs topology, replica and service-entry commands from those verified tools while explicitly targeting `~/Documents/semio`. The original kit script and topology JSON remain unchanged. No business or framework source file in the repository is patched. Only the replica's normal adoption would reconcile repository contents, retaining replaced files under `.cache/f/lost/`.

The helper performs the adoption dry run first and waits for the agent's explicit marker after reviewing the original conflict/new-work/git-relation limits. The root setup process ends after real provision exit 0 or 3; native builds are not retried in a loop. Consent exit 4 remains gated on owner consent and agent preflight. Archive copying and staging have not been run, and runtime compatibility is therefore not yet proven.

Preparation validation completed: both shell scripts pass `bash -n`, and the guarded adapter generator successfully matched the exact current kit and produced the preview diff. No fallback execution or real provisioning has occurred.

This is an alternative to the exact published-kit procedure. Approval is needed to depart from the original instruction not to improvise repairs; the fallback has not been substituted silently. Official-kit monitoring remains active while awaiting that decision.

## User Decision: Await Official Fix

The user explicitly selected “Wait for the coordinator’s official fix.” The archive-runtime fallback is not approved and has not been executed. No archive was staged, no adapter was used for a plan or provisioning run, and no repository code was patched.

Continue monitoring the official kit and the checkout's topology/replica command implementations. Run the original procedure only after a compatible official update arrives; retain the original adoption safety gates. Do not switch to the prepared fallback without new explicit authorization.

## Official Finish Attempt — Step 1 Facts

~~~text
HEAD: 5ececaa78b9265fd72b4f7e8f411afac3c9e1df1
Commit date: Thu Nov 12 22:14:18 2026 +0100
Status paths:        1
Stashes:        0
Filesystem      Size    Used   Avail Capacity iused ifree %iused  Mounted on
/dev/disk3s1   228Gi    44Gi   159Gi    22%    796k  1.7G    0%   /System/Volumes/Data
ProductName:		macOS
ProductVersion:		27.2.1
BuildVersion:		26B131
17179869184
Ueli’s Mac mini
lo0: 127.0.0.1
en1: 192.168.178.139
Repository: /Users/ueli/Documents/semio
Accessible qualifying repositories found: 1
Protected-directory search errors:      159
~~~

Search errors are retained in the run logs; inaccessible directories cannot be fully certified.

Run logs: /Users/ueli/semio-setup/runs/20261114-231808-official-finish
The coordinator-authorized step 3 will be used. The archive-runtime fallback is discarded and will not run.

## Official Finish Attempt — Step 1 Facts

~~~text
HEAD: 5ececaa78b9265fd72b4f7e8f411afac3c9e1df1
Commit date: Thu Nov 12 22:14:18 2026 +0100
Status paths:        1
Stashes:        0
Filesystem      Size    Used   Avail Capacity iused ifree %iused  Mounted on
/dev/disk3s1   228Gi    44Gi   159Gi    22%    796k  1.7G    0%   /System/Volumes/Data
ProductName:		macOS
ProductVersion:		27.2.1
BuildVersion:		26B131
17179869184
Ueli’s Mac mini
lo0: 127.0.0.1
en1: 192.168.178.139
Repository: /Users/ueli/Documents/semio
Accessible qualifying repositories found: 1
Protected-directory search errors:      159
~~~

Search errors are retained in the run logs; inaccessible directories cannot be fully certified.

Run logs: /Users/ueli/semio-setup/runs/20261115-004830-official-finish
The coordinator-authorized step 3 will be used. The archive-runtime fallback is discarded and will not run.

### Publication Receipt Reader Correction

The first official-finish batch stopped before step 3 because the agent's orchestration helper incorrectly expected the published receipt to be a bare timestamp. The receipt is JSON; its publishedAt value was 2026-11-14T23:08:05Z, newer than the required 2026-11-14T22:02:14Z. All ten kit file checksums passed. This was an orchestration metadata-reader error, not a failure of the published fleet command, and no checkout code was copied during that batch.

The helper now parses publishedAt from the JSON and compares timezone-aware timestamps. It was checked against the actual receipt, and the batched official session was restarted. The extra authentication prompt resulted from the earlier batch exiting. The declined fallback remains deleted and unexecuted.

## Official Module Copy, Plan, And Partial Adoption

Kit publishedAt: 2026-11-14T23:08:05Z. All ten checksums passed.

Code source: /Volumes/semio-kit/v1/bootstrap/code/eabbc92267329dbd39d8d05e0c8eceb1bcad083c0eea58c44805408a7d547bcc/files. The attested source was used (62 files), without the newest-folder fallback.

Copied as UID 501: replaced 16, added 36, identical 10. Preserved originals: /Users/ueli/Documents/semio/.cache/f/lost/provision-code-20261115-005015.

The facts check exited 0 and produced 25 FLEET_ lines. The stock provisioner's subsequent sealed-code refresh found all 62 files identical.

Plan: {"counts": {"same": 268590, "materialise": 5426, "publish": 1, "stale": 2699, "conflict": 0, "edit": 0}, "newLocalWork": 1, "gitRelation": "equal", "approved": true}. The original gates passed; stale paths were correctly treated separately from conflicts and new local work.

The real run exited 1 at step 7 because 35 items remained queued after adoption. Actual adoption: same 268590, materialise 5429, publish 1, edit 0, stale 2699, conflict 0. Report: .cache/f/lost/adopt-2026-11-14T23-52-54-418Z.md.

The remaining paths are ignored .git metadata (including config, index, hooks, and logs). No manual Git or queue repair was performed. Local identity now reads g1-m10 / g1. The GUI replica job is not yet loaded, because the provisioner stopped before step 8. Its explicit supported instruction is to run the same provision command again to continue; that resume skips the completed join and installs/starts the services. A single supported resume is being prepared, without rerunning native builds in a loop.

Last 40 lines of the failed real run:

~~~text
      for example .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️04/SOLVE-ALL-MERGE-CONFLICTS/🎫️ticket.json
      for example .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/design-contract.md
      for example .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/🎫️ticket.json
      for example .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END/coordination-plan.md
      for example Cargo.lock
      for example README.md
      for example bun.lock
      for example nx.json
      for example puzzle-artifact-editors-windows-kit/README.txt
      for example puzzle-artifact-editors-windows-kit/agents.json
      for example puzzle-artifact-editors-windows-kit/prompt.txt
      for example puzzle-artifact-editors-windows-kit/run.ps1
      for example staffage-launch-kit/README.md
  .claude/settings.json: older versions of fleet files or files the fleet deleted since: the fleet wins, nothing is published (local bytes kept in lost/); unchanged since this repository's last commit while the fleet moved on
  .gitignore: older versions of fleet files or files the fleet deleted since: the fleet wins, nothing is published (local bytes kept in lost/); unchanged since this repository's last commit while the fleet moved on
  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1028-coordination.md: older versions of fleet files or files the fleet deleted since: the fleet wins, nothing is published (local bytes kept in lost/); unchanged since this repository's last commit while the fleet moved on
  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1105b-claims.md: older versions of fleet files or files the fleet deleted since: the fleet wins, nothing is published (local bytes kept in lost/); unchanged since this repository's last commit while the fleet moved on
  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1105b-tools/slots.k: older versions of fleet files or files the fleet deleted since: the fleet wins, nothing is published (local bytes kept in lost/); unchanged since this repository's last commit while the fleet moved on
  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1105b-tools/slots.wextra: older versions of fleet files or files the fleet deleted since: the fleet wins, nothing is published (local bytes kept in lost/); unchanged since this repository's last commit while the fleet moved on
  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1112-result-puzzle2d.md: older versions of fleet files or files the fleet deleted since: the fleet wins, nothing is published (local bytes kept in lost/); unchanged since this repository's last commit while the fleet moved on
  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️04/SOLVE-ALL-MERGE-CONFLICTS/🎫️ticket.json: older versions of fleet files or files the fleet deleted since: the fleet wins, nothing is published (local bytes kept in lost/); unchanged since this repository's last commit while the fleet moved on
  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/design-contract.md: older versions of fleet files or files the fleet deleted since: the fleet wins, nothing is published (local bytes kept in lost/); unchanged since this repository's last commit while the fleet moved on
  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/🎫️ticket.json: older versions of fleet files or files the fleet deleted since: the fleet wins, nothing is published (local bytes kept in lost/); unchanged since this repository's last commit while the fleet moved on
  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END/coordination-plan.md: older versions of fleet files or files the fleet deleted since: the fleet wins, nothing is published (local bytes kept in lost/); unchanged since this repository's last commit while the fleet moved on
  Cargo.lock: older versions of fleet files or files the fleet deleted since: the fleet wins, nothing is published (local bytes kept in lost/); unchanged since this repository's last commit while the fleet moved on
  README.md: older versions of fleet files or files the fleet deleted since: the fleet wins, nothing is published (local bytes kept in lost/); unchanged since this repository's last commit while the fleet moved on
  bun.lock: older versions of fleet files or files the fleet deleted since: the fleet wins, nothing is published (local bytes kept in lost/); unchanged since this repository's last commit while the fleet moved on
  nx.json: older versions of fleet files or files the fleet deleted since: the fleet wins, nothing is published (local bytes kept in lost/); unchanged since this repository's last commit while the fleet moved on
  puzzle-artifact-editors-windows-kit/README.txt: older versions of fleet files or files the fleet deleted since: the fleet wins, nothing is published (local bytes kept in lost/); unchanged since this repository's last commit while the fleet moved on
  puzzle-artifact-editors-windows-kit/agents.json: older versions of fleet files or files the fleet deleted since: the fleet wins, nothing is published (local bytes kept in lost/); unchanged since this repository's last commit while the fleet moved on
  puzzle-artifact-editors-windows-kit/prompt.txt: older versions of fleet files or files the fleet deleted since: the fleet wins, nothing is published (local bytes kept in lost/); unchanged since this repository's last commit while the fleet moved on
  puzzle-artifact-editors-windows-kit/run.ps1: older versions of fleet files or files the fleet deleted since: the fleet wins, nothing is published (local bytes kept in lost/); unchanged since this repository's last commit while the fleet moved on
  staffage-launch-kit/README.md: older versions of fleet files or files the fleet deleted since: the fleet wins, nothing is published (local bytes kept in lost/); unchanged since this repository's last commit while the fleet moved on
  ... and 2679 more older versions of fleet files or files the fleet deleted since: the fleet wins, nothing is published (local bytes kept in lost/)
Git: fleet tip 5ececaa78b92, local tip 5ececaa78b92, relation equal; 0 staged paths here are replaced by the stage of the fleet (the old index is kept in lost/).
Evidence: 35505 journal events read, 18 commits of the fleet's history searched.
Conflicts: 0. Stop and report if there are more than 50: the fleet's version of each stays, the local one is kept in .cache/f/lost/ and listed.
New local work: 1 paths (published as new work or as edits). Stop and report if there are more than 1000: something unexpected, such as build output that is not ignored, is probably in the repository.
Adopted: 268590 identical, 5429 taken from the fleet, 1 published as new work, 0 published as edits, 2699 stale (fleet won), 0 conflicts (fleet's version stays, yours kept), 35 still queued. Report: .cache/f/lost/adopt-2026-11-14T23-52-54-418Z.md
[fleet-provision] ERROR: the adoption did not finish: nothing was lost (see .cache/f/lost), run this script again to continue where it stopped
~~~

Logs: /Users/ueli/semio-setup/runs/20261115-004830-official-finish

