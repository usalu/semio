# Freeze Watch 2035

Read-only samples of the worker fleet readiness stages, every ~4 min, from 20:36 UTC until 21:55 UTC (landing freeze runs until 22:00 UTC).

## Sample 20:36:27 UTC
### 1. ready-table
```
origin     state     ready stage                    try  exit peakMB refusal          age    pool/attest
g1-m01     running   False workspace 5/5            35   75   9392   -                4s     pool 3s attest 1s
g1-m02     running   False workspace 5/5            20   75   2825   -                5s     pool 2s attest 1s
g1-m03     running   False workspace 5/5            17   1    10473  -                0s     pool 5s attest 4s
g1-m04     running   False tools 4/5                38   1    1866   -                1s     pool 0s attest 1s
g1-m05     running   False tools 4/5                8    None None   -                1s     pool 17661s attest 386s
g1-m06     running   False workspace 5/5            2    75   7539   -                39282s pool 39281s attest 39279s
g1-m07     running   False workspace 5/5            61   75   10662  -                5s     pool 1s attest 0s
g1-m08     running   False workspace 5/5            45   75   10208  -                1s     pool 4s attest 3s
g1-m09     -         -     -                        -    -    -      -                -      no-pool attest ?
g1-m10     backoff   False workspace 5/5            21   1    4566   stage-failed     20s    pool 6s attest 1s
g1-server  server    -     -                        -    -    -      -                -      pool 1s attest 2s
```
### 2. last log lines
```
g1-m01: Reading the project graph slice (319 s): taken 2 of 4774.
g1-m02: Reading the project graph slice (245 s): taken 0 of 4774.
g1-m03: Reading the project graph slice (1517 s): waiting for the load of this checkout that is running ahead of this one (1 load(s) of 2 workers busy); both share one fact store, so this one starts when that one has ended.
g1-m07: Reading the project graph slice (292 s): taken 0 of 4774.
g1-m08: [repo] still working after 770 s; last phase started: door.script
g1-m10: The resident service met an unexpected failure (reference 79312331): Error: Monotonic graph phase progress required; try again, the details under that reference are in the service log /var/folders/n2/f4tcy7ks04sb8njd18x7l1lh0000gn/T/semio-repo/501/host-af031b965cb4.log.
g1-m04: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"native-artifact-input-read:4791","mode":"exclusive","elapsedMs":0}}
g1-m05: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:maintenance-parent-observation","mode":"shared","elapsedMs":1.5049999999973807}}
```
### 3. slice-memory
```
[g1-m01 g1-m02 g1-m03 g1-m07 g1-m08 g1-m10]
(script bug in this sample: the six-machine loop received one joined string; step 3 values for 20:36 are in the 20:38 re-read below, from the same file)
```
### 4. new failure/stalled/containment files since previous sample
```
```
### 5. owners in graph-server read.log
```
[owners] owners 2 of 48 done: 2 present, 0 fetched from the fleet, 0 inferred here (0 running), 46 wanted from the fleet, 45 stale in the fleet
```
### ready.json check
```
none ready:true
```

## Sample 20:40:08 UTC
### 1. ready-table
```
origin     state     ready stage                    try  exit peakMB refusal          age    pool/attest
g1-m01     running   False workspace 5/5            35   75   9392   -                5s     pool 0s attest 0s
g1-m02     running   False workspace 5/5            20   75   2825   -                5s     pool 3s attest 2s
g1-m03     running   False workspace 5/5            17   1    10473  -                3s     pool 6s attest 5s
g1-m04     running   False tools 4/5                38   1    1866   -                2s     pool 2s attest 3s
g1-m05     running   False tools 4/5                8    None None   -                9s     pool 17882s attest 607s
g1-m06     running   False workspace 5/5            2    75   7539   -                39504s pool 39502s attest 39500s
g1-m07     running   False workspace 5/5            61   75   10662  -                0s     pool 6s attest 3s
g1-m08     running   False workspace 5/5            45   75   10208  -                3s     pool 0s attest 1s
g1-m09     -         -     -                        -    -    -      -                -      no-pool attest ?
g1-m10     running   False workspace 5/5            21   1    4566   -                4s     pool 1s attest 0s
g1-server  server    -     -                        -    -    -      -                -      pool 0s attest 4s
```
### 2. last log lines
```
g1-m01: Reading the project graph slice (543 s): taken 15 of 4774.
g1-m02: Reading the project graph slice (471 s): taken 11 of 4774.
g1-m03: Reading the project graph slice (1740 s): waiting for the load of this checkout that is running ahead of this one (1 load(s) of 2 workers busy); both share one fact store, so this one starts when that one has ended.
g1-m07: Reading the project graph slice (506 s): taken 3 of 4774.
g1-m08: Reading the project graph slice (562 s): taken 0 of 4774.
g1-m10: Reading the project graph slice (44 s): facts 282624 of 332825.
g1-m04: [repository:command] still running after 1520 s
g1-m05: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:maintenance-parent-observation","mode":"shared","elapsedMs":1.5049999999973807}}
```
### 3. slice-memory
```
[g1-m01]
  uptimeSeconds = 543.6
  memory.rss = 2495.9
  guard.megabytes = 756.2
  treeFootprintMegabytes = 1102
  sources.owners.ledger = {'total': 4774, 'present': 15, 'fetched': 0, 'here': 0, 'stale': 197, 'running': 0, 'wanted': 0, 'done': 15, 'missing': 4759}
  sources.owners.warm = {'total': 4773, 'done': 38, 'settled': 12, 'durable': 26, 'failed': 0, 'lanes': 1, 'peak': 1834, 'why': ''}
[g1-m02]
  uptimeSeconds = 464
  memory.rss = 1113.6
  guard.megabytes = 593.9
  treeFootprintMegabytes = 2838
  sources.owners.ledger = {'total': 4774, 'present': 11, 'fetched': 0, 'here': 0, 'stale': 121, 'running': 0, 'wanted': 0, 'done': 11, 'missing': 4763}
  sources.owners.warm = {'total': 4773, 'done': 51, 'settled': 32, 'durable': 19, 'failed': 0, 'lanes': 2, 'peak': 2389, 'why': ''}
[g1-m03]
  uptimeSeconds = 1384.7
  memory.rss = 1284.1
  guard.megabytes = 1213.7
  treeFootprintMegabytes = 2708
  sources.owners.ledger = {'total': 4774, 'present': 564, 'fetched': 0, 'here': 0, 'stale': 0, 'running': 0, 'wanted': 4210, 'done': 564, 'missing': 4210}
  sources.owners.warm = {'total': 881, 'done': 172, 'settled': 172, 'durable': 0, 'failed': 0, 'lanes': 1, 'peak': 4644, 'why': ''}
[g1-m07]
  uptimeSeconds = 487.7
  memory.rss = 2901.5
  guard.megabytes = 1399.1
  treeFootprintMegabytes = 3041
  sources.owners.ledger = {'total': 4774, 'present': 2, 'fetched': 1, 'here': 0, 'stale': 177, 'running': 0, 'wanted': 0, 'done': 3, 'missing': 4771}
  sources.owners.warm = {'total': 4520, 'done': 119, 'settled': 38, 'durable': 81, 'failed': 0, 'lanes': 2, 'peak': 2524, 'why': ''}
[g1-m08]
  uptimeSeconds = 550.9
  memory.rss = 4476.6
  guard.megabytes = 989.4
  treeFootprintMegabytes = 1844
  sources.owners.ledger = {'total': 4774, 'present': 0, 'fetched': 0, 'here': 0, 'stale': 128, 'running': 0, 'wanted': 0, 'done': 0, 'missing': 4774}
  sources.owners.warm = {'total': 4537, 'done': 37, 'settled': 26, 'durable': 10, 'failed': 1, 'lanes': 1, 'peak': 3424, 'why': 'failure: graph: project @semio-tech/repo-identity-rs needed 4426 MB in a worker 1'}
[g1-m10]
  uptimeSeconds = 30.5
  memory.rss = 1762.9
  guard.megabytes = 910.3
  treeFootprintMegabytes = None
  sources.owners.ledger = None
  sources.owners.warm = None
```
### 4. new failed/stalled/service-failure/containment files since previous sample
```
```
### 5. owners in graph-server read.log
```
[owners] owners 2 of 48 done: 2 present, 0 fetched from the fleet, 0 inferred here (0 running), 46 wanted from the fleet, 45 stale in the fleet
```
### ready.json check
```
none ready:true
```

## Sample 20:44:10 UTC
### 1. ready-table
```
origin     state     ready stage                    try  exit peakMB refusal          age    pool/attest
g1-m01     running   False workspace 5/5            35   75   9392   -                1s     pool 0s attest 2s
g1-m02     running   False workspace 5/5            20   75   2825   -                5s     pool 4s attest 2s
g1-m03     running   False workspace 5/5            17   1    10473  -                4s     pool 2s attest 1s
g1-m04     running   False tools 4/5                38   1    1866   -                0s     pool 3s attest 2s
g1-m05     running   False tools 4/5                8    None None   -                251s   pool 18124s attest 849s
g1-m06     running   False workspace 5/5            2    75   7539   -                39745s pool 39744s attest 39742s
g1-m07     running   False workspace 5/5            61   75   10662  -                3s     pool 3s attest 2s
g1-m08     running   False workspace 5/5            45   75   10208  -                4s     pool 3s attest 2s
g1-m09     -         -     -                        -    -    -      -                -      no-pool attest ?
g1-m10     running   False workspace 5/5            21   1    4566   -                6s     pool 1s attest 1s
g1-server  server    -     -                        -    -    -      -                -      pool 2s attest 0s
```
### 2. last log lines
```
g1-m01: Reading the project graph slice (64 s): taken 0 of 4774.
g1-m02: Reading the project graph slice (92 s): taken 0 of 4774.
g1-m03: [capacity] waiting 5s for run: not enough free memory (need 512 MB; -144 MB spare above the 2048 MB floor, load 8.29; 6 running, 1 ahead); behind @semio-tech/framework-os-dev:activate-shooting-react-prototype (running, 3 job(s), 5917 MB), command 4zNUNQ (running, 1 job(s), 1991 MB), graph-plan (runn
g1-m07: Reading the project graph slice (753 s): taken 13 of 4774.
g1-m08: Reading the project graph slice (803 s): taken 28 of 4774.
g1-m10: Reading the project graph slice (293 s): decisions 19406 of 19406.
g1-m04: [repository:command] still running after 1760 s
g1-m05: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:maintenance-parent-observation","mode":"shared","elapsedMs":1.5049999999973807}}
```
### 3. slice-memory
```
[g1-m01]
  uptimeSeconds = 60.6
  memory.rss = 1300.7
  guard.megabytes = 543.8
  treeFootprintMegabytes = 230
  sources.owners.ledger = {'total': 4774, 'present': 0, 'fetched': 0, 'here': 0, 'stale': 73, 'running': 0, 'wanted': 0, 'done': 0, 'missing': 4774}
  sources.owners.warm = {'total': 4773, 'done': 7, 'settled': 7, 'durable': 0, 'failed': 0, 'lanes': 1, 'peak': 194, 'why': ''}
[g1-m02]
  uptimeSeconds = 67.4
  memory.rss = 1217.9
  guard.megabytes = 522.3
  treeFootprintMegabytes = 122
  sources.owners.ledger = {'total': 4774, 'present': 0, 'fetched': 0, 'here': 0, 'stale': 0, 'running': 0, 'wanted': 0, 'done': 0, 'missing': 4774}
  sources.owners.warm = {'total': 4773, 'done': 1, 'settled': 0, 'durable': 1, 'failed': 0, 'lanes': 1, 'peak': 0, 'why': ''}
[g1-m03]
  uptimeSeconds = 1624.7
  memory.rss = 1350.1
  guard.megabytes = 1221.1
  treeFootprintMegabytes = 1983
  sources.owners.ledger = {'total': 4774, 'present': 564, 'fetched': 0, 'here': 0, 'stale': 0, 'running': 0, 'wanted': 4210, 'done': 564, 'missing': 4210}
  sources.owners.warm = {'total': 881, 'done': 177, 'settled': 177, 'durable': 0, 'failed': 0, 'lanes': 1, 'peak': 4644, 'why': ''}
[g1-m07]
  uptimeSeconds = 728.4
  memory.rss = 2338.6
  guard.megabytes = 1127.6
  treeFootprintMegabytes = 3261
  sources.owners.ledger = {'total': 4774, 'present': 11, 'fetched': 1, 'here': 0, 'stale': 364, 'running': 0, 'wanted': 0, 'done': 12, 'missing': 4762}
  sources.owners.warm = {'total': 4520, 'done': 125, 'settled': 44, 'durable': 81, 'failed': 0, 'lanes': 2, 'peak': 2524, 'why': ''}
[g1-m08]
  uptimeSeconds = 793.2
  memory.rss = 2723.5
  guard.megabytes = 874.7
  treeFootprintMegabytes = 1366
  sources.owners.ledger = {'total': 4774, 'present': 28, 'fetched': 0, 'here': 0, 'stale': 185, 'running': 0, 'wanted': 0, 'done': 28, 'missing': 4746}
  sources.owners.warm = {'total': 4537, 'done': 43, 'settled': 32, 'durable': 10, 'failed': 1, 'lanes': 1, 'peak': 3424, 'why': 'failure: graph: project @semio-tech/repo-identity-rs needed 4426 MB in a worker 1'}
[g1-m10]
  uptimeSeconds = 273
  memory.rss = 2939.8
  guard.megabytes = 1499.6
  treeFootprintMegabytes = None
  sources.owners.ledger = None
  sources.owners.warm = None
```
### 4. new failed/stalled/service-failure/containment files since previous sample
```
```
### 5. owners in graph-server read.log
```
[owners] owners 2 of 48 done: 2 present, 0 fetched from the fleet, 0 inferred here (0 running), 46 wanted from the fleet, 45 stale in the fleet
```
### ready.json check
```
none ready:true
```

## Sample 20:48:50 UTC
### 1. ready-table
```
origin     state     ready stage                    try  exit peakMB refusal          age    pool/attest
g1-m01     running   False workspace 5/5            35   75   9392   -                4s     pool 4s attest 0s
g1-m02     running   False workspace 5/5            20   75   2825   -                3s     pool 2s attest 2s
g1-m03     running   False workspace 5/5            17   1    10473  -                6s     pool 4s attest 4s
g1-m04     running   False tools 4/5                38   1    1866   -                0s     pool 0s attest 2s
g1-m05     running   False tools 4/5                8    None None   -                530s   pool 18403s attest 1128s
g1-m06     running   False workspace 5/5            2    75   7539   -                40025s pool 40023s attest 40022s
g1-m07     running   False workspace 5/5            61   75   10662  -                3s     pool 3s attest 2s
g1-m08     running   False workspace 5/5            45   75   10208  -                4s     pool 6s attest 0s
g1-m09     -         -     -                        -    -    -      -                -      no-pool attest ?
g1-m10     running   False workspace 5/5            21   1    4566   -                1s     pool 6s attest 0s
g1-server  server    -     -                        -    -    -      -                -      pool 1s attest 4s
```
### 2. last log lines
```
g1-m01: Reading the project graph slice (351 s): taken 17 of 4774.
g1-m02: Reading the project graph slice (379 s): taken 14 of 4774.
g1-m03: Reading the project graph slice (120 s): reservation 0 of 6553; running (name, kind, seconds, rows): ; waiting for memory: 6553 MB are asked for (2048 MB are enough to start), 2392 MB are free above the 2048 MB floor of this machine; the request is refused after 120 s if nothing of ours runs and not
g1-m07: Reading the project graph slice (102 s): taken 4 of 4774.
g1-m08: [repo] still working after 1520 s; last phase started: door.script
g1-m10: Reading the project graph slice (76 s): decisions 19702 of 19703.
g1-m04: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"native-artifact-input-read:10341","mode":"exclusive","elapsedMs":0}}
g1-m05: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:maintenance-parent-observation","mode":"shared","elapsedMs":1.5049999999973807}}
```
### 3. slice-memory
```
[g1-m01]
  uptimeSeconds = 334.5
  memory.rss = 2611.4
  guard.megabytes = 513.6
  treeFootprintMegabytes = 1965
  sources.owners.ledger = {'total': 4774, 'present': 17, 'fetched': 0, 'here': 0, 'stale': 274, 'running': 0, 'wanted': 0, 'done': 17, 'missing': 4757}
  sources.owners.warm = {'total': 4773, 'done': 59, 'settled': 29, 'durable': 30, 'failed': 0, 'lanes': 1, 'peak': 973, 'why': ''}
[g1-m02]
  uptimeSeconds = 369.8
  memory.rss = 4758.3
  guard.megabytes = 965.7
  treeFootprintMegabytes = 1533
  sources.owners.ledger = {'total': 4774, 'present': 14, 'fetched': 0, 'here': 0, 'stale': 241, 'running': 0, 'wanted': 0, 'done': 14, 'missing': 4760}
  sources.owners.warm = {'total': 4773, 'done': 4773, 'settled': 16, 'durable': 39, 'failed': 4718, 'lanes': 1, 'peak': 1200, 'why': 'lost: closed 1, farm-off 4717'}
[g1-m03]
  uptimeSeconds = 1924.8
  memory.rss = 1236
  guard.megabytes = 1195.3
  treeFootprintMegabytes = 2683
  sources.owners.ledger = {'total': 4774, 'present': 564, 'fetched': 0, 'here': 0, 'stale': 0, 'running': 0, 'wanted': 4210, 'done': 564, 'missing': 4210}
  sources.owners.warm = {'total': 881, 'done': 196, 'settled': 196, 'durable': 0, 'failed': 0, 'lanes': 1, 'peak': 4644, 'why': ''}
[g1-m07]
  uptimeSeconds = 95.5
  memory.rss = 2898.1
  guard.megabytes = 654.5
  treeFootprintMegabytes = 654
  sources.owners.ledger = {'total': 4774, 'present': 4, 'fetched': 0, 'here': 0, 'stale': 127, 'running': 0, 'wanted': 0, 'done': 4, 'missing': 4770}
  sources.owners.warm = {'total': 4773, 'done': 43, 'settled': 17, 'durable': 26, 'failed': 0, 'lanes': 2, 'peak': 509, 'why': ''}
[g1-m08]
  uptimeSeconds = 1064.4
  memory.rss = 1845.5
  guard.megabytes = 729
  treeFootprintMegabytes = 2381
  sources.owners.ledger = {'total': 4774, 'present': 28, 'fetched': 0, 'here': 0, 'stale': 241, 'running': 0, 'wanted': 4746, 'done': 28, 'missing': 4746}
  sources.owners.warm = {'total': 4537, 'done': 43, 'settled': 32, 'durable': 10, 'failed': 1, 'lanes': 1, 'peak': 3424, 'why': 'failure: graph: project @semio-tech/repo-identity-rs needed 4426 MB in a worker 1'}
[g1-m10]
  uptimeSeconds = 60.7
  memory.rss = 2158.5
  guard.megabytes = 1095.8
  treeFootprintMegabytes = None
  sources.owners.ledger = None
  sources.owners.warm = None
```
### 4. new failed/stalled/service-failure/containment files since previous sample
```
```
### 5. owners in graph-server read.log
```
[owners] owners 2 of 48 done: 2 present, 0 fetched from the fleet, 0 inferred here (0 running), 46 wanted from the fleet, 45 stale in the fleet
```
### ready.json check
```
none ready:true
```

## Sample 20:52:51 UTC
### 1. ready-table
```
origin     state     ready stage                    try  exit peakMB refusal          age    pool/attest
g1-m01     running   False workspace 5/5            35   75   9392   -                1s     pool 1s attest 1s
g1-m02     running   False workspace 5/5            20   75   2825   -                2s     pool 4s attest 0s
g1-m03     running   False workspace 5/5            17   1    10473  -                0s     pool 5s attest 4s
g1-m04     running   False tools 4/5                38   1    1866   -                3s     pool 1s attest 3s
g1-m05     running   False tools 4/5                8    None None   -                772s   pool 18645s attest 1370s
g1-m06     running   False workspace 5/5            2    75   7539   -                40266s pool 40265s attest 40263s
g1-m07     running   False workspace 5/5            61   75   10662  -                1s     pool 1s attest 4s
g1-m08     running   False workspace 5/5            45   75   10208  -                5s     pool 6s attest 1s
g1-m09     -         -     -                        -    -    -      -                -      no-pool attest ?
g1-m10     running   False workspace 5/5            21   1    4566   -                5s     pool 6s attest 2s
g1-server  server    -     -                        -    -    -      -                -      pool 5s attest 2s
```
### 2. last log lines
```
g1-m01: Reading the project graph slice (585 s): taken 60 of 4774.
g1-m02: Reading the project graph slice (616 s): taken 16 of 4774.
g1-m03: Reading the project graph slice (364 s): reservation 0 of 6553; running (name, kind, seconds, rows): ; waiting for memory: 6553 MB are asked for (2048 MB are enough to start), 6087 MB are free above the 2048 MB floor of this machine; the request is refused after 120 s if nothing of ours runs and not
g1-m07: Reading the project graph slice (146 s): taken 15 of 4774.
g1-m08: [repo] still working after 1760 s; last phase started: door.script
g1-m10: Reading the project graph slice (2 s): reservation 0 of 2048; running (name, kind, seconds, rows): ; waiting for memory: 2048 MB are asked for (2048 MB are enough to start), 8809 MB are free above the 2048 MB floor of this machine; the request is refused after 120 s if nothing of ours runs and not e
g1-m04: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"native-artifact-input-read:11888","mode":"exclusive","elapsedMs":0}}
g1-m05: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:maintenance-parent-observation","mode":"shared","elapsedMs":1.5049999999973807}}
```
### 3. slice-memory
```
[g1-m01]
  uptimeSeconds = 575.3
  memory.rss = 2927.4
  guard.megabytes = 660.4
  treeFootprintMegabytes = None
  sources.owners.ledger = {'total': 4774, 'present': 60, 'fetched': 0, 'here': 0, 'stale': 339, 'running': 0, 'wanted': 0, 'done': 60, 'missing': 4714}
  sources.owners.warm = {'total': 4773, 'done': 4773, 'settled': 37, 'durable': 30, 'failed': 4706, 'lanes': 1, 'peak': 1286, 'why': 'lost: closed 1, farm-off 4705'}
[g1-m02]
  uptimeSeconds = 610.4
  memory.rss = 2827
  guard.megabytes = 946.7
  treeFootprintMegabytes = None
  sources.owners.ledger = {'total': 4774, 'present': 16, 'fetched': 0, 'here': 0, 'stale': 282, 'running': 0, 'wanted': 0, 'done': 16, 'missing': 4758}
  sources.owners.warm = {'total': 4773, 'done': 4773, 'settled': 16, 'durable': 39, 'failed': 4718, 'lanes': 1, 'peak': 1200, 'why': 'lost: closed 1, farm-off 4717'}
[g1-m03]
  uptimeSeconds = 2076
  memory.rss = 2028.3
  guard.megabytes = 1647.1
  treeFootprintMegabytes = 2677
  sources.owners.ledger = {'total': 4774, 'present': 564, 'fetched': 0, 'here': 0, 'stale': 0, 'running': 1, 'wanted': 4210, 'done': 564, 'missing': 4210}
  sources.owners.warm = {'total': 881, 'done': 881, 'settled': 206, 'durable': 0, 'failed': 675, 'lanes': 1, 'peak': 4644, 'why': 'lost: closed 1, farm-off 674'}
[g1-m07]
  uptimeSeconds = 120.3
  memory.rss = 1253.3
  guard.megabytes = 599.1
  treeFootprintMegabytes = 861
  sources.owners.ledger = {'total': 4774, 'present': 13, 'fetched': 0, 'here': 0, 'stale': 167, 'running': 0, 'wanted': 0, 'done': 13, 'missing': 4761}
  sources.owners.warm = {'total': 4773, 'done': 54, 'settled': 7, 'durable': 46, 'failed': 1, 'lanes': 1, 'peak': 240, 'why': 'failure: graph: project @semio-tech/procedural-generation3d-rs needed 5306 MB i 1'}
[g1-m08]
  uptimeSeconds = 30.1
  memory.rss = 991.1
  guard.megabytes = 145.6
  treeFootprintMegabytes = None
  sources.owners.ledger = None
  sources.owners.warm = None
[g1-m10]
  uptimeSeconds = 100.1
  memory.rss = 2583.3
  guard.megabytes = 1073.1
  treeFootprintMegabytes = 303
  sources.owners.ledger = None
  sources.owners.warm = None
```
### 4. new failed/stalled/service-failure/containment files since previous sample
```
```
### 5. owners in graph-server read.log
```
[owners] owners 2 of 48 done: 2 present, 0 fetched from the fleet, 0 inferred here (0 running), 46 wanted from the fleet, 45 stale in the fleet
```
### ready.json check
```
none ready:true
```

## Sample 20:56:52 UTC
### 1. ready-table
```
origin     state     ready stage                    try  exit peakMB refusal          age    pool/attest
g1-m01     running   False workspace 5/5            35   75   9392   -                3s     pool 3s attest 1s
g1-m02     running   False workspace 5/5            20   75   2825   -                3s     pool 0s attest 1s
g1-m03     running   False workspace 5/5            17   1    10473  -                6s     pool 1s attest 0s
g1-m04     running   False tools 4/5                38   1    1866   -                4s     pool 4s attest 4s
g1-m05     running   False tools 4/5                8    None None   -                1014s  pool 18887s attest 1612s
g1-m06     running   False workspace 5/5            2    75   7539   -                40508s pool 40507s attest 40505s
g1-m07     running   False workspace 5/5            61   75   10662  -                4s     pool 5s attest 4s
g1-m08     running   False workspace 5/5            45   75   10208  -                0s     pool 2s attest 1s
g1-m09     -         -     -                        -    -    -      -                -      no-pool attest ?
g1-m10     running   False workspace 5/5            21   1    4566   -                6s     pool 1s attest 0s
g1-server  server    -     -                        -    -    -      -                -      pool 0s attest 5s
```
### 2. last log lines
```
g1-m01: Reading the project graph slice (830 s): taken 212 of 4774.
g1-m02: Reading the project graph slice (852 s): taken 16 of 4774.
g1-m03: Reading the project graph slice (158 s): taken 2 of 4774.
g1-m07: Reading the project graph slice (390 s): taken 18 of 4774.
g1-m08: Reading the project graph slice (279 s): taken 0 of 4774.
g1-m10: Reading the project graph slice (104 s): stage inference 0 of 0.
g1-m04: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"native-artifact-input-read:13299","mode":"exclusive","elapsedMs":0}}
g1-m05: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:maintenance-parent-observation","mode":"shared","elapsedMs":1.5049999999973807}}
```
### 3. slice-memory
```
[g1-m01]
  uptimeSeconds = 815.8
  memory.rss = 1955.6
  guard.megabytes = 891.4
  treeFootprintMegabytes = None
  sources.owners.ledger = {'total': 4774, 'present': 212, 'fetched': 0, 'here': 0, 'stale': 407, 'running': 0, 'wanted': 0, 'done': 212, 'missing': 4562}
  sources.owners.warm = {'total': 4773, 'done': 4773, 'settled': 37, 'durable': 30, 'failed': 4706, 'lanes': 1, 'peak': 1286, 'why': 'lost: closed 1, farm-off 4705'}
[g1-m02]
  uptimeSeconds = 852.5
  memory.rss = 2643.9
  guard.megabytes = 1098.4
  treeFootprintMegabytes = None
  sources.owners.ledger = {'total': 4774, 'present': 16, 'fetched': 0, 'here': 0, 'stale': 334, 'running': 0, 'wanted': 0, 'done': 16, 'missing': 4758}
  sources.owners.warm = {'total': 4773, 'done': 4773, 'settled': 16, 'durable': 39, 'failed': 4718, 'lanes': 1, 'peak': 1200, 'why': 'lost: closed 1, farm-off 4717'}
[g1-m03]
  uptimeSeconds = 151.1
  memory.rss = 4759
  guard.megabytes = 1064.5
  treeFootprintMegabytes = None
  sources.owners.ledger = {'total': 4774, 'present': 2, 'fetched': 0, 'here': 0, 'stale': 17, 'running': 0, 'wanted': 0, 'done': 2, 'missing': 4772}
  sources.owners.warm = None
[g1-m07]
  uptimeSeconds = 391.9
  memory.rss = 2007
  guard.megabytes = 576.9
  treeFootprintMegabytes = 1094
  sources.owners.ledger = {'total': 4774, 'present': 18, 'fetched': 0, 'here': 0, 'stale': 350, 'running': 0, 'wanted': 0, 'done': 18, 'missing': 4756}
  sources.owners.warm = {'total': 4773, 'done': 74, 'settled': 26, 'durable': 47, 'failed': 1, 'lanes': 1, 'peak': 1362, 'why': 'failure: graph: project @semio-tech/procedural-generation3d-rs needed 5306 MB i 1'}
[g1-m08]
  uptimeSeconds = 270.9
  memory.rss = 1620.3
  guard.megabytes = 597.8
  treeFootprintMegabytes = 1125
  sources.owners.ledger = {'total': 4774, 'present': 0, 'fetched': 0, 'here': 0, 'stale': 105, 'running': 0, 'wanted': 0, 'done': 0, 'missing': 4774}
  sources.owners.warm = {'total': 4639, 'done': 57, 'settled': 21, 'durable': 36, 'failed': 0, 'lanes': 1, 'peak': 1386, 'why': ''}
[g1-m10]
  uptimeSeconds = 90.6
  memory.rss = 2384.8
  guard.megabytes = 887.1
  treeFootprintMegabytes = None
  sources.owners.ledger = None
  sources.owners.warm = None
```
### 4. new failed/stalled/service-failure/containment files since previous sample
```
```
### 5. owners in graph-server read.log
```
[owners] owners 2 of 48 done: 2 present, 0 fetched from the fleet, 0 inferred here (0 running), 46 wanted from the fleet, 45 stale in the fleet
```
### ready.json check
```
none ready:true
```

## Sample 21:00:55 UTC
### 1. ready-table
```
origin     state     ready stage                    try  exit peakMB refusal          age    pool/attest
g1-m01     running   False workspace 5/5            35   75   9392   -                1s     pool 2s attest 1s
g1-m02     running   False workspace 5/5            20   75   2825   -                3s     pool 1s attest 2s
g1-m03     running   False workspace 5/5            17   1    10473  -                4s     pool 0s attest 1s
g1-m04     running   False tools 4/5                38   1    1866   -                2s     pool 0s attest 1s
g1-m05     running   False tools 4/5                8    None None   -                1256s  pool 19129s attest 1854s
g1-m06     running   False workspace 5/5            2    75   7539   -                40750s pool 40749s attest 40747s
g1-m07     running   False workspace 5/5            61   75   10662  -                1s     pool 2s attest 0s
g1-m08     running   False workspace 5/5            45   75   10208  -                4s     pool 3s attest 2s
g1-m09     -         -     -                        -    -    -      -                -      no-pool attest ?
g1-m10     running   False workspace 5/5            22   75   4566   -                3s     pool 2s attest 1s
g1-server  server    -     -                        -    -    -      -                -      pool 5s attest 3s
```
### 2. last log lines
```
g1-m01: [repo] still working after 2122 s; last phase started: door.script
g1-m02: Reading the project graph slice (1101 s): taken 17 of 4774.
g1-m03: Reading the project graph slice (390 s): taken 2 of 4774.
g1-m07: Reading the project graph slice (639 s): taken 134 of 4774.
g1-m08: Reading the project graph slice (528 s): taken 2 of 4774.
g1-m10: Reading the project graph slice (8 s).
g1-m04: [repo] still working after 2780 s; last phase started: source.served
g1-m05: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:maintenance-parent-observation","mode":"shared","elapsedMs":1.5049999999973807}}
```
### 3. slice-memory
```
[g1-m01]
  uptimeSeconds = 91.1
  memory.rss = 1166.4
  guard.megabytes = 594.3
  treeFootprintMegabytes = 346
  sources.owners.ledger = {'total': 4774, 'present': 2, 'fetched': 0, 'here': 0, 'stale': 79, 'running': 0, 'wanted': 0, 'done': 2, 'missing': 4772}
  sources.owners.warm = {'total': 4773, 'done': 24, 'settled': 8, 'durable': 16, 'failed': 0, 'lanes': 1, 'peak': 228, 'why': ''}
[g1-m02]
  uptimeSeconds = 1054.4
  memory.rss = 2990.8
  guard.megabytes = 1236.2
  treeFootprintMegabytes = None
  sources.owners.ledger = {'total': 4774, 'present': 16, 'fetched': 0, 'here': 1, 'stale': 335, 'running': 0, 'wanted': 4757, 'done': 17, 'missing': 4757}
  sources.owners.warm = {'total': 4773, 'done': 4773, 'settled': 16, 'durable': 39, 'failed': 4718, 'lanes': 1, 'peak': 1200, 'why': 'lost: closed 1, farm-off 4717'}
[g1-m03]
  uptimeSeconds = 90.2
  memory.rss = 106.1
  guard.megabytes = 73.3
  treeFootprintMegabytes = None
  sources.owners.ledger = None
  sources.owners.warm = None
[g1-m07]
  uptimeSeconds = 633.6
  memory.rss = 1819.6
  guard.megabytes = 874.9
  treeFootprintMegabytes = 1337
  sources.owners.ledger = {'total': 4774, 'present': 134, 'fetched': 0, 'here': 0, 'stale': 476, 'running': 0, 'wanted': 0, 'done': 134, 'missing': 4640}
  sources.owners.warm = {'total': 4773, 'done': 95, 'settled': 46, 'durable': 48, 'failed': 1, 'lanes': 1, 'peak': 2198, 'why': 'failure: graph: project @semio-tech/procedural-generation3d-rs needed 5306 MB i 1'}
[g1-m08]
  uptimeSeconds = 511.6
  memory.rss = 3307
  guard.megabytes = 674.4
  treeFootprintMegabytes = 1234
  sources.owners.ledger = {'total': 4774, 'present': 0, 'fetched': 2, 'here': 0, 'stale': 205, 'running': 0, 'wanted': 0, 'done': 2, 'missing': 4772}
  sources.owners.warm = {'total': 4639, 'done': 71, 'settled': 35, 'durable': 36, 'failed': 0, 'lanes': 1, 'peak': 1795, 'why': ''}
[g1-m10]
  uptimeSeconds = 0.1
  memory.rss = 101.1
  guard.megabytes = 17.8
  treeFootprintMegabytes = None
  sources.owners.ledger = None
  sources.owners.warm = None
```
### 4. new failed/stalled/service-failure/containment files since previous sample
```
```
### 5. owners in graph-server read.log
```
[owners] owners 2 of 48 done: 2 present, 0 fetched from the fleet, 0 inferred here (0 running), 46 wanted from the fleet, 45 stale in the fleet
```
### ready.json check
```
none ready:true
```

## Sample 21:04:55 UTC
### 1. ready-table
```
origin     state     ready stage                    try  exit peakMB refusal          age    pool/attest
g1-m01     running   False workspace 5/5            35   75   9392   -                2s     pool 6s attest 2s
g1-m02     running   False workspace 5/5            20   75   2825   -                2s     pool 3s attest 0s
g1-m03     running   False workspace 5/5            17   1    10473  -                2s     pool 0s attest 2s
g1-m04     running   False tools 4/5                38   1    1866   -                1s     pool 5s attest 0s
g1-m05     running   False tools 4/5                8    None None   -                1496s  pool 19369s attest 2094s
g1-m06     running   False workspace 5/5            2    75   7539   -                40990s pool 40989s attest 40987s
g1-m07     running   False workspace 5/5            61   75   10662  -                2s     pool 0s attest 5s
g1-m08     running   False workspace 5/5            45   75   10208  -                5s     pool 4s attest 1s
g1-m09     -         -     -                        -    -    -      -                -      no-pool attest ?
g1-m10     running   False workspace 5/5            22   75   4566   -                2s     pool 4s attest 3s
g1-server  server    -     -                        -    -    -      -                -      pool 0s attest 3s
```
### 2. last log lines
```
g1-m01: Reading the project graph slice (353 s): taken 17 of 4774.
g1-m02: Reading the project graph slice (229 s): taken 3 of 4774.
g1-m03: Reading the project graph slice (633 s): taken 31 of 4774.
g1-m07: Reading the project graph slice (873 s): taken 277 of 4774.
g1-m08: Reading the project graph slice (761 s): taken 3 of 4774.
g1-m10: Reading the project graph slice (66 s): verification 207 of 2430.
g1-m04: [repo] still working after 3020 s; last phase started: source.served
g1-m05: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:maintenance-parent-observation","mode":"shared","elapsedMs":1.5049999999973807}}
```
### 3. slice-memory
```
[g1-m01]
  uptimeSeconds = 332.3
  memory.rss = 1195.5
  guard.megabytes = 668.1
  treeFootprintMegabytes = 1043
  sources.owners.ledger = {'total': 4774, 'present': 17, 'fetched': 0, 'here': 0, 'stale': 250, 'running': 0, 'wanted': 0, 'done': 17, 'missing': 4757}
  sources.owners.warm = {'total': 4773, 'done': 74, 'settled': 27, 'durable': 47, 'failed': 0, 'lanes': 1, 'peak': 2174, 'why': ''}
[g1-m02]
  uptimeSeconds = 213.5
  memory.rss = 1132.3
  guard.megabytes = 632.9
  treeFootprintMegabytes = 753
  sources.owners.ledger = {'total': 4774, 'present': 3, 'fetched': 0, 'here': 0, 'stale': 112, 'running': 0, 'wanted': 0, 'done': 3, 'missing': 4771}
  sources.owners.warm = {'total': 4773, 'done': 28, 'settled': 13, 'durable': 15, 'failed': 0, 'lanes': 1, 'peak': 310, 'why': ''}
[g1-m03]
  uptimeSeconds = 330.2
  memory.rss = 148.8
  guard.megabytes = 65.8
  treeFootprintMegabytes = 173
  sources.owners.ledger = None
  sources.owners.warm = None
[g1-m07]
  uptimeSeconds = 844.2
  memory.rss = 1492.1
  guard.megabytes = 943.2
  treeFootprintMegabytes = 3905
  sources.owners.ledger = {'total': 4774, 'present': 251, 'fetched': 0, 'here': 0, 'stale': 631, 'running': 0, 'wanted': 0, 'done': 251, 'missing': 4523}
  sources.owners.warm = {'total': 4773, 'done': 108, 'settled': 59, 'durable': 48, 'failed': 1, 'lanes': 1, 'peak': 2385, 'why': 'failure: graph: project @semio-tech/procedural-generation3d-rs needed 5306 MB i 1'}
[g1-m08]
  uptimeSeconds = 752.6
  memory.rss = 4599.6
  guard.megabytes = 1893.9
  treeFootprintMegabytes = 1552
  sources.owners.ledger = {'total': 4774, 'present': 1, 'fetched': 2, 'here': 0, 'stale': 270, 'running': 0, 'wanted': 0, 'done': 3, 'missing': 4771}
  sources.owners.warm = {'total': 4639, 'done': 78, 'settled': 42, 'durable': 36, 'failed': 0, 'lanes': 1, 'peak': 2689, 'why': ''}
[g1-m10]
  uptimeSeconds = 60.2
  memory.rss = 2206.5
  guard.megabytes = 1084.3
  treeFootprintMegabytes = None
  sources.owners.ledger = None
  sources.owners.warm = None
```
### 4. new failed/stalled/service-failure/containment files since previous sample
```
```
### 5. owners in graph-server read.log
```
```
### ready.json check
```
none ready:true
```

## Sample 21:08:56 UTC
### 1. ready-table
```
origin     state     ready stage                    try  exit peakMB refusal          age    pool/attest
g1-m01     running   False workspace 5/5            35   75   9392   -                3s     pool 4s attest 0s
g1-m02     running   False workspace 5/5            20   75   2825   -                2s     pool 5s attest 0s
g1-m03     running   False workspace 5/5            17   1    10473  -                5s     pool 5s attest 0s
g1-m04     running   False tools 4/5                38   1    1866   -                3s     pool 0s attest 0s
g1-m05     running   False tools 4/5                8    None None   -                2s     pool 19610s attest 178s
g1-m06     running   False workspace 5/5            2    75   7539   -                41231s pool 41229s attest 41228s
g1-m07     running   False workspace 5/5            61   75   10662  -                4s     pool 1s attest 0s
g1-m08     running   False workspace 5/5            45   75   10208  -                0s     pool 2s attest 1s
g1-m09     -         -     -                        -    -    -      -                -      no-pool attest ?
g1-m10     running   False workspace 5/5            22   75   4566   -                2s     pool 1s attest 0s
g1-server  server    -     -                        -    -    -      -                -      pool 3s attest 0s
```
### 2. last log lines
```
g1-m01: Reading the project graph slice (599 s): taken 349 of 4774.
g1-m02: Reading the project graph slice (474 s): taken 17 of 4774.
g1-m03: Reading the project graph slice (877 s): taken 65 of 4774.
g1-m07: Reading the project graph slice (1117 s): taken 357 of 4774.
g1-m08: Reading the project graph slice (1004 s): taken 32 of 4774.
g1-m10: Reading the project graph slice (140 s): taken 9 of 478.
g1-m04: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"native-artifact-input-read:16138","mode":"exclusive","elapsedMs":0}}
g1-m05: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:maintenance-parent-observation","mode":"shared","elapsedMs":1.8941999999806285}}
```
### 3. slice-memory
```
[g1-m01]
  uptimeSeconds = 572.7
  memory.rss = 1379.8
  guard.megabytes = 829.4
  treeFootprintMegabytes = 1258
  sources.owners.ledger = {'total': 4774, 'present': 299, 'fetched': 0, 'here': 0, 'stale': 395, 'running': 0, 'wanted': 0, 'done': 299, 'missing': 4475}
  sources.owners.warm = {'total': 4773, 'done': 79, 'settled': 32, 'durable': 47, 'failed': 0, 'lanes': 1, 'peak': 2174, 'why': ''}
[g1-m02]
  uptimeSeconds = 454.1
  memory.rss = 1278.6
  guard.megabytes = 707.2
  treeFootprintMegabytes = 962
  sources.owners.ledger = {'total': 4774, 'present': 17, 'fetched': 0, 'here': 0, 'stale': 278, 'running': 0, 'wanted': 0, 'done': 17, 'missing': 4757}
  sources.owners.warm = {'total': 4773, 'done': 54, 'settled': 38, 'durable': 16, 'failed': 0, 'lanes': 1, 'peak': 2620, 'why': ''}
[g1-m03]
  uptimeSeconds = 877
  memory.rss = 2398.4
  guard.megabytes = 981.2
  treeFootprintMegabytes = 2587
  sources.owners.ledger = {'total': 4774, 'present': 13, 'fetched': 52, 'here': 0, 'stale': 187, 'running': 0, 'wanted': 0, 'done': 65, 'missing': 4709}
  sources.owners.warm = None
[g1-m07]
  uptimeSeconds = 1114.7
  memory.rss = 1506.5
  guard.megabytes = 889.8
  treeFootprintMegabytes = 1391
  sources.owners.ledger = {'total': 4774, 'present': 357, 'fetched': 0, 'here': 0, 'stale': 691, 'running': 0, 'wanted': 4417, 'done': 357, 'missing': 4417}
  sources.owners.warm = {'total': 4773, 'done': 127, 'settled': 77, 'durable': 48, 'failed': 2, 'lanes': 1, 'peak': 2864, 'why': 'failure: graph: project @semio-tech/procedural-generation3d-rs needed 5306 MB i 1, failure: graph: project @semio-tech/stdio-ifc-rs needed 4568 MB in a worker pro 1'}
[g1-m08]
  uptimeSeconds = 994.4
  memory.rss = 2316.8
  guard.megabytes = 619.1
  treeFootprintMegabytes = 2229
  sources.owners.ledger = {'total': 4774, 'present': 29, 'fetched': 3, 'here': 0, 'stale': 324, 'running': 0, 'wanted': 0, 'done': 32, 'missing': 4742}
  sources.owners.warm = {'total': 4639, 'done': 84, 'settled': 48, 'durable': 36, 'failed': 0, 'lanes': 1, 'peak': 2689, 'why': ''}
[g1-m10]
  uptimeSeconds = 129.5
  memory.rss = 2287.1
  guard.megabytes = 1072.3
  treeFootprintMegabytes = 962
  sources.owners.ledger = {'total': 478, 'present': 9, 'fetched': 0, 'here': 0, 'stale': 39, 'running': 0, 'wanted': 0, 'done': 9, 'missing': 469}
  sources.owners.warm = {'total': 477, 'done': 16, 'settled': 0, 'durable': 16, 'failed': 0, 'lanes': 1, 'peak': 0, 'why': ''}
```
### 4. new failed/stalled/service-failure/containment files since previous sample
```
```
### 5. owners in graph-server read.log
```
```
### ready.json check
```
none ready:true
```

## Sample 21:12:57 UTC
### 1. ready-table
```
origin     state     ready stage                    try  exit peakMB refusal          age    pool/attest
g1-m01     running   False workspace 5/5            35   75   9392   -                5s     pool 2s attest 1s
g1-m02     running   False workspace 5/5            20   75   2825   -                4s     pool 3s attest 0s
g1-m03     running   False workspace 5/5            17   1    10473  -                4s     pool 4s attest 3s
g1-m04     running   False tools 4/5                38   1    1866   -                4s     pool 2s attest 0s
g1-m05     running   False tools 4/5                8    None None   -                1s     pool 19851s attest 419s
g1-m06     running   False workspace 5/5            2    75   7539   -                41472s pool 41471s attest 41469s
g1-m07     running   False workspace 5/5            61   75   10662  -                0s     pool 6s attest 2s
g1-m08     running   False workspace 5/5            45   75   10208  -                2s     pool 4s attest 2s
g1-m09     -         -     -                        -    -    -      -                -      no-pool attest ?
g1-m10     running   False workspace 5/5            22   75   4566   -                3s     pool 2s attest 2s
g1-server  server    -     -                        -    -    -      -                -      pool 3s attest 3s
```
### 2. last log lines
```
g1-m01: Reading the project graph slice (833 s): taken 555 of 4774.
g1-m02: Reading the project graph slice (709 s): taken 17 of 4774.
g1-m03: Reading the project graph slice (1120 s): taken 76 of 4774.
g1-m07: Reading the project graph slice (1351 s): taken 357 of 4774.
g1-m08: Reading the project graph slice (1241 s): taken 32 of 4774.
g1-m10: [repo] still working after 740 s; last phase started: door.script
g1-m04: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"native-artifact-input-read:17839","mode":"exclusive","elapsedMs":0}}
g1-m05: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-sweep:nested","mode":"exclusive","elapsedMs":190}}
```
### 3. slice-memory
```
[g1-m01]
  uptimeSeconds = 814.6
  memory.rss = 2290.7
  guard.megabytes = 913.8
  treeFootprintMegabytes = 1513
  sources.owners.ledger = {'total': 4774, 'present': 494, 'fetched': 0, 'here': 0, 'stale': 443, 'running': 0, 'wanted': 0, 'done': 494, 'missing': 4280}
  sources.owners.warm = {'total': 4773, 'done': 90, 'settled': 43, 'durable': 47, 'failed': 0, 'lanes': 1, 'peak': 2174, 'why': ''}
[g1-m02]
  uptimeSeconds = 695.2
  memory.rss = 2002.7
  guard.megabytes = 776.9
  treeFootprintMegabytes = 1772
  sources.owners.ledger = {'total': 4774, 'present': 17, 'fetched': 0, 'here': 0, 'stale': 360, 'running': 0, 'wanted': 0, 'done': 17, 'missing': 4757}
  sources.owners.warm = {'total': 4773, 'done': 60, 'settled': 44, 'durable': 16, 'failed': 0, 'lanes': 1, 'peak': 2620, 'why': ''}
[g1-m03]
  uptimeSeconds = 1115.2
  memory.rss = 3706.9
  guard.megabytes = 1657.8
  treeFootprintMegabytes = 1540
  sources.owners.ledger = {'total': 4774, 'present': 13, 'fetched': 62, 'here': 1, 'stale': 235, 'running': 4, 'wanted': 4698, 'done': 76, 'missing': 4698}
  sources.owners.warm = None
[g1-m07]
  uptimeSeconds = 1325.2
  memory.rss = 1531
  guard.megabytes = 891.8
  treeFootprintMegabytes = 1509
  sources.owners.ledger = {'total': 4774, 'present': 357, 'fetched': 0, 'here': 0, 'stale': 691, 'running': 0, 'wanted': 4417, 'done': 357, 'missing': 4417}
  sources.owners.warm = {'total': 4773, 'done': 181, 'settled': 128, 'durable': 51, 'failed': 2, 'lanes': 1, 'peak': 2864, 'why': 'failure: graph: project @semio-tech/procedural-generation3d-rs needed 5306 MB i 1, failure: graph: project @semio-tech/stdio-ifc-rs needed 4568 MB in a worker pro 1'}
[g1-m08]
  uptimeSeconds = 1235.4
  memory.rss = 1399.6
  guard.megabytes = 738.3
  treeFootprintMegabytes = 1928
  sources.owners.ledger = {'total': 4774, 'present': 29, 'fetched': 3, 'here': 0, 'stale': 325, 'running': 0, 'wanted': 4742, 'done': 32, 'missing': 4742}
  sources.owners.warm = {'total': 4639, 'done': 89, 'settled': 53, 'durable': 36, 'failed': 0, 'lanes': 1, 'peak': 2689, 'why': ''}
[g1-m10]
  uptimeSeconds = 62
  memory.rss = 1780.3
  guard.megabytes = 647.7
  treeFootprintMegabytes = None
  sources.owners.ledger = None
  sources.owners.warm = None
```
### 4. new failed/stalled/service-failure/containment files since previous sample
```
```
### 5. owners in graph-server read.log
```
```
### ready.json check
```
none ready:true
```

## Sample 21:16:58 UTC
### 1. ready-table
```
origin     state     ready stage                    try  exit peakMB refusal          age    pool/attest
g1-m01     running   False workspace 5/5            35   75   9392   -                2s     pool 5s attest 0s
g1-m02     running   False workspace 5/5            20   75   2825   -                1s     pool 4s attest 1s
g1-m03     running   False workspace 5/5            17   1    10473  -                0s     pool 2s attest 0s
g1-m04     running   False tools 4/5                38   1    1866   -                4s     pool 1s attest 0s
g1-m05     running   False tools 4/5                8    None None   -                62s    pool 20091s attest 660s
g1-m06     running   False workspace 5/5            2    75   7539   -                41713s pool 41711s attest 41710s
g1-m07     running   False workspace 5/5            61   75   10662  -                2s     pool 6s attest 0s
g1-m08     running   False workspace 5/5            45   75   10208  -                4s     pool 1s attest 1s
g1-m09     -         -     -                        -    -    -      -                -      no-pool attest ?
g1-m10     running   False workspace 5/5            22   75   4566   -                1s     pool 5s attest 2s
g1-server  server    -     -                        -    -    -      -                -      pool 3s attest 0s
```
### 2. last log lines
```
g1-m01: Reading the project graph slice (1080 s): taken 569 of 4774.
g1-m02: Reading the project graph slice (72 s).
g1-m03: Reading the project graph slice (219 s).
g1-m07: Reading the project graph slice (1598 s): taken 358 of 4774.
g1-m08: Reading the project graph slice (1486 s): taken 32 of 4774.
g1-m10: Reading the project graph slice (48 s): facts 266240 of 332735.
g1-m04: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"native-artifact-input-read:20500","mode":"exclusive","elapsedMs":0}}
g1-m05: [repository:command] still running after 2430 s
```
### 3. slice-memory
```
[g1-m01]
  uptimeSeconds = 1055.1
  memory.rss = 1477.7
  guard.megabytes = 759.6
  treeFootprintMegabytes = 2041
  sources.owners.ledger = {'total': 4774, 'present': 569, 'fetched': 0, 'here': 0, 'stale': 545, 'running': 0, 'wanted': 4205, 'done': 569, 'missing': 4205}
  sources.owners.warm = {'total': 4773, 'done': 98, 'settled': 51, 'durable': 47, 'failed': 0, 'lanes': 1, 'peak': 2174, 'why': ''}
[g1-m02]
  uptimeSeconds = 65.3
  memory.rss = 431.9
  guard.megabytes = 144.3
  treeFootprintMegabytes = 104
  sources.owners.ledger = None
  sources.owners.warm = None
[g1-m03]
  uptimeSeconds = 1050.7
  memory.rss = 1495.3
  guard.megabytes = 941.6
  treeFootprintMegabytes = 1048
  sources.owners.ledger = {'total': 4774, 'present': 326, 'fetched': 0, 'here': 0, 'stale': 0, 'running': 0, 'wanted': 0, 'done': 326, 'missing': 4448}
  sources.owners.warm = None
[g1-m07]
  uptimeSeconds = 1596.4
  memory.rss = 1520.4
  guard.megabytes = 633.3
  treeFootprintMegabytes = 1263
  sources.owners.ledger = {'total': 4774, 'present': 357, 'fetched': 1, 'here': 0, 'stale': 690, 'running': 0, 'wanted': 4416, 'done': 358, 'missing': 4416}
  sources.owners.warm = {'total': 4773, 'done': 185, 'settled': 131, 'durable': 52, 'failed': 2, 'lanes': 1, 'peak': 2864, 'why': 'failure: graph: project @semio-tech/procedural-generation3d-rs needed 5306 MB i 1, failure: graph: project @semio-tech/stdio-ifc-rs needed 4568 MB in a worker pro 1'}
[g1-m08]
  uptimeSeconds = 1476.3
  memory.rss = 1261
  guard.megabytes = 734.4
  treeFootprintMegabytes = 4179
  sources.owners.ledger = {'total': 4774, 'present': 29, 'fetched': 3, 'here': 0, 'stale': 325, 'running': 0, 'wanted': 4742, 'done': 32, 'missing': 4742}
  sources.owners.warm = {'total': 4639, 'done': 103, 'settled': 66, 'durable': 36, 'failed': 1, 'lanes': 1, 'peak': 2689, 'why': 'failure: graph: project @semio-tech/repo-search-rs needed 5487 MB in a worker p 1'}
[g1-m10]
  uptimeSeconds = 30.2
  memory.rss = 1701
  guard.megabytes = 982.1
  treeFootprintMegabytes = None
  sources.owners.ledger = None
  sources.owners.warm = None
```
### 4. new failed/stalled/service-failure/containment files since previous sample
```
```
### 5. owners in graph-server read.log (tail -1 of ^[owners] grep; empty means no such line)
```
```
### ready.json check
```
none ready:true
```

## Sample 21:20:58 UTC
### 1. ready-table
```
origin     state     ready stage                    try  exit peakMB refusal          age    pool/attest
g1-m01     running   False workspace 5/5            35   75   9392   -                1s     pool 4s attest 0s
g1-m02     running   False workspace 5/5            20   75   2825   -                1s     pool 0s attest 1s
g1-m03     running   False workspace 5/5            17   1    10473  -                4s     pool 5s attest 0s
g1-m04     running   False tools 4/5                38   1    1866   -                4s     pool 5s attest 0s
g1-m05     running   False tools 4/5                8    None None   -                5s     pool 20332s attest 233s
g1-m06     running   False workspace 5/5            2    75   7539   -                41953s pool 41951s attest 41950s
g1-m07     running   False workspace 5/5            61   75   10662  -                3s     pool 1s attest 0s
g1-m08     running   False workspace 5/5            45   75   10208  -                6s     pool 1s attest 1s
g1-m09     -         -     -                        -    -    -      -                -      no-pool attest ?
g1-m10     running   False workspace 5/5            23   75   4566   -                2s     pool 4s attest 1s
g1-server  server    -     -                        -    -    -      -                -      pool 3s attest 1s
```
### 2. last log lines
```
g1-m01: Reading the project graph slice (1316 s): taken 569 of 4774.
g1-m02: Reading the project graph slice (309 s): taken 14 of 4774.
g1-m03: Reading the project graph slice (461 s): taken 28 of 4774.
g1-m07: Reading the project graph slice (1832 s): taken 360 of 4774.
g1-m08: Reading the project graph slice (213 s): taken 0 of 4774.
g1-m10: Reading the project graph slice (72 s): verification 2426 of 2430.
g1-m04: [repository:command] still running after 3970 s
g1-m05: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-sweep:nested","mode":"exclusive","elapsedMs":50}}
```
### 3. slice-memory
```
[g1-m01]
  uptimeSeconds = 1295.5
  memory.rss = 1290.3
  guard.megabytes = 908.4
  treeFootprintMegabytes = 1707
  sources.owners.ledger = {'total': 4774, 'present': 569, 'fetched': 0, 'here': 0, 'stale': 545, 'running': 0, 'wanted': 4205, 'done': 569, 'missing': 4205}
  sources.owners.warm = {'total': 4773, 'done': 124, 'settled': 76, 'durable': 47, 'failed': 1, 'lanes': 1, 'peak': 2174, 'why': 'failure: graph: project @semio-tech/repo-search-rs needed 4338 MB in a worker p 1'}
[g1-m02]
  uptimeSeconds = 307
  memory.rss = 1016.8
  guard.megabytes = 634.2
  treeFootprintMegabytes = 1958
  sources.owners.ledger = {'total': 4774, 'present': 14, 'fetched': 0, 'here': 0, 'stale': 177, 'running': 0, 'wanted': 0, 'done': 14, 'missing': 4760}
  sources.owners.warm = {'total': 4773, 'done': 73, 'settled': 33, 'durable': 40, 'failed': 0, 'lanes': 2, 'peak': 307, 'why': ''}
[g1-m03]
  uptimeSeconds = 120.5
  memory.rss = 348.4
  guard.megabytes = 68.3
  treeFootprintMegabytes = None
  sources.owners.ledger = None
  sources.owners.warm = None
[g1-m07]
  uptimeSeconds = 1807
  memory.rss = 1847.9
  guard.megabytes = 922.1
  treeFootprintMegabytes = 2306
  sources.owners.ledger = {'total': 4774, 'present': 357, 'fetched': 3, 'here': 0, 'stale': 688, 'running': 0, 'wanted': 4414, 'done': 360, 'missing': 4414}
  sources.owners.warm = {'total': 4773, 'done': 185, 'settled': 131, 'durable': 52, 'failed': 2, 'lanes': 1, 'peak': 2864, 'why': 'failure: graph: project @semio-tech/procedural-generation3d-rs needed 5306 MB i 1, failure: graph: project @semio-tech/stdio-ifc-rs needed 4568 MB in a worker pro 1'}
[g1-m08]
  uptimeSeconds = 211.2
  memory.rss = 1188.7
  guard.megabytes = 595.6
  treeFootprintMegabytes = 1548
  sources.owners.ledger = {'total': 4774, 'present': 0, 'fetched': 0, 'here': 0, 'stale': 150, 'running': 0, 'wanted': 0, 'done': 0, 'missing': 4774}
  sources.owners.warm = {'total': 4773, 'done': 30, 'settled': 30, 'durable': 0, 'failed': 0, 'lanes': 2, 'peak': 1590, 'why': ''}
[g1-m10]
  uptimeSeconds = 61.5
  memory.rss = 1676.5
  guard.megabytes = 844.8
  treeFootprintMegabytes = None
  sources.owners.ledger = None
  sources.owners.warm = None
```
### 4. new failed/stalled/service-failure/containment files since previous sample
```
```
### 5. owners in graph-server read.log (tail -1 of ^[owners] grep; empty means no such line)
```
[owners] owners 8 of 48 done: 8 present, 0 fetched from the fleet, 0 inferred here (0 running), 0 wanted from the fleet, 6 stale in the fleet
```
### ready.json check
```
none ready:true
```

## Sample 21:24:58 UTC
### 1. ready-table
```
origin     state     ready stage                    try  exit peakMB refusal          age    pool/attest
g1-m01     running   False workspace 5/5            35   75   9392   -                3s     pool 0s attest 2s
g1-m02     running   False workspace 5/5            20   75   2825   -                5s     pool 2s attest 1s
g1-m03     running   False workspace 5/5            17   1    10473  -                3s     pool 2s attest 3s
g1-m04     running   False tools 4/5                38   1    1866   -                3s     pool 4s attest 1s
g1-m05     running   False tools 4/5                8    None None   -                6s     pool 20572s attest 473s
g1-m06     running   False workspace 5/5            2    75   7539   -                42193s pool 42192s attest 42190s
g1-m07     running   False workspace 5/5            61   75   10662  -                5s     pool 0s attest 5s
g1-m08     running   False workspace 5/5            45   75   10208  -                5s     pool 4s attest 0s
g1-m09     -         -     -                        -    -    -      -                -      no-pool attest ?
g1-m10     running   False workspace 5/5            23   75   4566   -                3s     pool 6s attest 4s
g1-server  server    -     -                        -    -    -      -                -      pool 4s attest 2s
```
### 2. last log lines
```
g1-m01: Reading the project graph slice (1561 s): taken 569 of 4774.
g1-m02: Reading the project graph slice (556 s): taken 18 of 4774.
g1-m03: Reading the project graph slice (704 s): taken 51 of 4774.
g1-m07: Reading the project graph slice (2076 s): taken 362 of 4774.
g1-m08: Reading the project graph slice (452 s): taken 6 of 4774.
g1-m10: Reading the project graph slice (4 s).
g1-m04: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:workspace-edges:1983","mode":"exclusive","elapsedMs":0}}
g1-m05: [repository:command] still running after 2961 s
```
### 3. slice-memory
```
[g1-m01]
  uptimeSeconds = 1537.3
  memory.rss = 1384.1
  guard.megabytes = 928.3
  treeFootprintMegabytes = 1989
  sources.owners.ledger = {'total': 4774, 'present': 569, 'fetched': 0, 'here': 0, 'stale': 545, 'running': 0, 'wanted': 4205, 'done': 569, 'missing': 4205}
  sources.owners.warm = {'total': 4773, 'done': 148, 'settled': 100, 'durable': 47, 'failed': 1, 'lanes': 1, 'peak': 2174, 'why': 'failure: graph: project @semio-tech/repo-search-rs needed 4338 MB in a worker p 1'}
[g1-m02]
  uptimeSeconds = 548.3
  memory.rss = 1198.6
  guard.megabytes = 835.4
  treeFootprintMegabytes = 2307
  sources.owners.ledger = {'total': 4774, 'present': 18, 'fetched': 0, 'here': 0, 'stale': 306, 'running': 0, 'wanted': 0, 'done': 18, 'missing': 4756}
  sources.owners.warm = {'total': 4773, 'done': 84, 'settled': 44, 'durable': 40, 'failed': 0, 'lanes': 2, 'peak': 2302, 'why': ''}
[g1-m03]
  uptimeSeconds = 360.5
  memory.rss = 284.6
  guard.megabytes = 68
  treeFootprintMegabytes = None
  sources.owners.ledger = None
  sources.owners.warm = None
[g1-m07]
  uptimeSeconds = 2048.7
  memory.rss = 1577.4
  guard.megabytes = 941.8
  treeFootprintMegabytes = 1222
  sources.owners.ledger = {'total': 4774, 'present': 357, 'fetched': 5, 'here': 0, 'stale': 686, 'running': 0, 'wanted': 4412, 'done': 362, 'missing': 4412}
  sources.owners.warm = {'total': 4773, 'done': 197, 'settled': 141, 'durable': 54, 'failed': 2, 'lanes': 1, 'peak': 3691, 'why': 'failure: graph: project @semio-tech/procedural-generation3d-rs needed 5306 MB i 1, failure: graph: project @semio-tech/stdio-ifc-rs needed 4568 MB in a worker pro 1'}
[g1-m08]
  uptimeSeconds = 452.2
  memory.rss = 1361.5
  guard.megabytes = 764.3
  treeFootprintMegabytes = 3582
  sources.owners.ledger = {'total': 4774, 'present': 6, 'fetched': 0, 'here': 0, 'stale': 307, 'running': 0, 'wanted': 0, 'done': 6, 'missing': 4768}
  sources.owners.warm = {'total': 4773, 'done': 88, 'settled': 70, 'durable': 18, 'failed': 0, 'lanes': 2, 'peak': 4560, 'why': ''}
[g1-m10]
  uptimeSeconds = 0.2
  memory.rss = 103.5
  guard.megabytes = 17.8
  treeFootprintMegabytes = None
  sources.owners.ledger = None
  sources.owners.warm = None
```
### 4. new failed/stalled/service-failure/containment files since previous sample
```
```
### 5. owners in graph-server read.log (tail -1 of ^[owners] grep; empty means no such line)
```
[owners] owners 8 of 48 done: 8 present, 0 fetched from the fleet, 0 inferred here (0 running), 0 wanted from the fleet, 21 stale in the fleet
```
### ready.json check
```
none ready:true
```

## Sample 21:28:58 UTC
### 1. ready-table
```
origin     state     ready stage                    try  exit peakMB refusal          age    pool/attest
g1-m01     running   False workspace 5/5            35   75   9392   -                1s     pool 1s attest 3s
g1-m02     running   False workspace 5/5            20   75   2825   -                3s     pool 4s attest 1s
g1-m03     running   False workspace 5/5            17   1    10473  -                5s     pool 6s attest 4s
g1-m04     running   False tools 4/5                38   1    1866   -                3s     pool 0s attest 4s
g1-m05     running   False tools 4/5                8    None None   -                117s   pool 20812s attest 713s
g1-m06     running   False workspace 5/5            2    75   7539   -                42433s pool 42432s attest 42430s
g1-m07     running   False workspace 5/5            61   75   10662  -                1s     pool 2s attest 1s
g1-m08     running   False workspace 5/5            45   75   10208  -                3s     pool 3s attest 1s
g1-m09     -         -     -                        -    -    -      -                -      no-pool attest ?
g1-m10     running   False workspace 5/5            23   75   4566   -                5s     pool 5s attest 2s
g1-server  server    -     -                        -    -    -      -                -      pool 4s attest 4s
```
### 2. last log lines
```
g1-m01: [repo] still working after 10 s; last phase started: door.script
g1-m02: Reading the project graph slice (794 s): taken 18 of 4774.
g1-m03: Reading the project graph slice (938 s): taken 54 of 4774.
g1-m07: Reading the project graph slice (2321 s): taken 362 of 4774.
g1-m08: Reading the project graph slice (685 s): taken 34 of 4774.
g1-m10: Reading the project graph slice (80 s): verification 2430 of 2430.
g1-m04: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:workspace-edges:19327","mode":"exclusive","elapsedMs":0}}
g1-m05: [repository:command] still running after 3092 s
```
### 3. slice-memory
```
[g1-m01]
  uptimeSeconds = 0.1
  memory.rss = 109.9
  guard.megabytes = 17.9
  treeFootprintMegabytes = None
  sources.owners.ledger = None
  sources.owners.warm = None
[g1-m02]
  uptimeSeconds = 790.8
  memory.rss = 1478.4
  guard.megabytes = 849
  treeFootprintMegabytes = 3354
  sources.owners.ledger = {'total': 4774, 'present': 18, 'fetched': 0, 'here': 0, 'stale': 328, 'running': 0, 'wanted': 0, 'done': 18, 'missing': 4756}
  sources.owners.warm = {'total': 4773, 'done': 117, 'settled': 77, 'durable': 40, 'failed': 0, 'lanes': 2, 'peak': 2302, 'why': ''}
[g1-m03]
  uptimeSeconds = 933.1
  memory.rss = 4601.1
  guard.megabytes = 2449.6
  treeFootprintMegabytes = None
  sources.owners.ledger = {'total': 4774, 'present': 29, 'fetched': 25, 'here': 0, 'stale': 470, 'running': 0, 'wanted': 0, 'done': 54, 'missing': 4720}
  sources.owners.warm = None
[g1-m07]
  uptimeSeconds = 2320.7
  memory.rss = 1493.2
  guard.megabytes = 987
  treeFootprintMegabytes = 3238
  sources.owners.ledger = {'total': 4774, 'present': 357, 'fetched': 5, 'here': 0, 'stale': 686, 'running': 0, 'wanted': 4412, 'done': 362, 'missing': 4412}
  sources.owners.warm = {'total': 4773, 'done': 197, 'settled': 141, 'durable': 54, 'failed': 2, 'lanes': 1, 'peak': 3691, 'why': 'failure: graph: project @semio-tech/procedural-generation3d-rs needed 5306 MB i 1, failure: graph: project @semio-tech/stdio-ifc-rs needed 4568 MB in a worker pro 1'}
[g1-m08]
  uptimeSeconds = 664.3
  memory.rss = 1635.2
  guard.megabytes = 593.9
  treeFootprintMegabytes = 2104
  sources.owners.ledger = {'total': 4774, 'present': 34, 'fetched': 0, 'here': 0, 'stale': 360, 'running': 0, 'wanted': 0, 'done': 34, 'missing': 4740}
  sources.owners.warm = {'total': 4773, 'done': 144, 'settled': 109, 'durable': 35, 'failed': 0, 'lanes': 2, 'peak': 4560, 'why': ''}
[g1-m10]
  uptimeSeconds = 61.7
  memory.rss = 1341.3
  guard.megabytes = 837.9
  treeFootprintMegabytes = None
  sources.owners.ledger = None
  sources.owners.warm = None
```
### 4. new failed/stalled/service-failure/containment files since previous sample
```
```
### 5. owners in graph-server read.log (tail -1 of ^[owners] grep; empty means no such line)
```
[owners] owners 8 of 48 done: 8 present, 0 fetched from the fleet, 0 inferred here (0 running), 0 wanted from the fleet, 33 stale in the fleet
```
### ready.json check
```
none ready:true
```

## Sample 21:33:00 UTC
### 1. ready-table
```
origin     state     ready stage                    try  exit peakMB refusal          age    pool/attest
g1-m01     running   False workspace 5/5            35   75   9392   -                5s     pool 3s attest 1s
g1-m02     running   False workspace 5/5            20   75   2825   -                4s     pool 1s attest 0s
g1-m03     running   False workspace 5/5            17   1    10473  -                1s     pool 3s attest 2s
g1-m04     running   False tools 4/5                38   1    1866   -                3s     pool 3s attest 0s
g1-m05     running   False tools 4/5                8    None None   -                359s   pool 21054s attest 955s
g1-m06     running   False workspace 5/5            2    75   7539   -                42676s pool 42674s attest 42672s
g1-m07     running   False workspace 5/5            61   75   10662  -                5s     pool 6s attest 3s
g1-m08     running   False workspace 5/5            45   75   10208  -                4s     pool 0s attest 0s
g1-m09     -         -     -                        -    -    -      -                -      no-pool attest ?
g1-m10     running   False workspace 5/5            23   75   4566   -                2s     pool 0s attest 3s
g1-server  server    -     -                        -    -    -      -                -      pool 3s attest 1s
```
### 2. last log lines
```
g1-m01: Reading the project graph slice (247 s): taken 18 of 4774.
g1-m02: Reading the project graph slice (1031 s): taken 25 of 4774.
g1-m03: Reading the project graph slice (1184 s): taken 57 of 4774.
g1-m07: Reading the project graph slice (2555 s): taken 362 of 4774.
g1-m08: Reading the project graph slice (931 s): taken 75 of 4774.
g1-m10: Reading the project graph slice (24 s): facts 155648 of 332735.
g1-m04: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:workspace-edges:55615","mode":"exclusive","elapsedMs":0}}
g1-m05: [repository:command] still running after 3092 s
```
### 3. slice-memory
```
[g1-m01]
  uptimeSeconds = 241.3
  memory.rss = 1118.8
  guard.megabytes = 679.8
  treeFootprintMegabytes = 969
  sources.owners.ledger = {'total': 4774, 'present': 17, 'fetched': 0, 'here': 0, 'stale': 243, 'running': 0, 'wanted': 0, 'done': 17, 'missing': 4757}
  sources.owners.warm = {'total': 4773, 'done': 53, 'settled': 53, 'durable': 0, 'failed': 0, 'lanes': 1, 'peak': 209, 'why': ''}
[g1-m02]
  uptimeSeconds = 1003
  memory.rss = 2695.3
  guard.megabytes = 832.2
  treeFootprintMegabytes = 3708
  sources.owners.ledger = {'total': 4774, 'present': 20, 'fetched': 0, 'here': 0, 'stale': 359, 'running': 0, 'wanted': 0, 'done': 20, 'missing': 4754}
  sources.owners.warm = {'total': 4773, 'done': 139, 'settled': 99, 'durable': 40, 'failed': 0, 'lanes': 2, 'peak': 2302, 'why': ''}
[g1-m03]
  uptimeSeconds = 841
  memory.rss = 128.6
  guard.megabytes = 72.3
  treeFootprintMegabytes = None
  sources.owners.ledger = None
  sources.owners.warm = None
[g1-m07]
  uptimeSeconds = 2531.6
  memory.rss = 1450.8
  guard.megabytes = 954.9
  treeFootprintMegabytes = 1504
  sources.owners.ledger = {'total': 4774, 'present': 357, 'fetched': 5, 'here': 0, 'stale': 686, 'running': 0, 'wanted': 4412, 'done': 362, 'missing': 4412}
  sources.owners.warm = {'total': 4773, 'done': 211, 'settled': 151, 'durable': 58, 'failed': 2, 'lanes': 1, 'peak': 3691, 'why': 'failure: graph: project @semio-tech/procedural-generation3d-rs needed 5306 MB i 1, failure: graph: project @semio-tech/stdio-ifc-rs needed 4568 MB in a worker pro 1'}
[g1-m08]
  uptimeSeconds = 907.1
  memory.rss = 1706.9
  guard.megabytes = 823.6
  treeFootprintMegabytes = 3101
  sources.owners.ledger = {'total': 4774, 'present': 52, 'fetched': 0, 'here': 0, 'stale': 429, 'running': 0, 'wanted': 0, 'done': 52, 'missing': 4722}
  sources.owners.warm = {'total': 4773, 'done': 173, 'settled': 138, 'durable': 35, 'failed': 0, 'lanes': 2, 'peak': 4560, 'why': ''}
[g1-m10]
  uptimeSeconds = 0.1
  memory.rss = 101.5
  guard.megabytes = 17.9
  treeFootprintMegabytes = None
  sources.owners.ledger = None
  sources.owners.warm = None
```
### 4. new failed/stalled/service-failure/containment files since previous sample
```
```
### 5. owners in graph-server read.log (tail -1 of ^[owners] grep; empty means no such line)
```
[owners] owners 8 of 48 done: 8 present, 0 fetched from the fleet, 0 inferred here (0 running), 40 wanted from the fleet, 39 stale in the fleet
```
### ready.json check
```
none ready:true
```

## Sample 21:37:02 UTC
### 1. ready-table
```
origin     state     ready stage                    try  exit peakMB refusal          age    pool/attest
g1-m01     running   False workspace 5/5            35   75   9392   -                2s     pool 5s attest 2s
g1-m02     running   False workspace 5/5            20   75   2825   -                5s     pool 5s attest 0s
g1-m03     running   False workspace 5/5            17   1    10473  -                3s     pool 5s attest 4s
g1-m04     running   False tools 4/5                38   1    1866   -                0s     pool 1s attest 0s
g1-m05     running   False tools 4/5                8    None None   -                2s     pool 21296s attest 71s
g1-m06     running   False workspace 5/5            2    75   7539   -                42917s pool 42916s attest 42914s
g1-m07     running   False workspace 5/5            61   75   10662  -                2s     pool 1s attest 4s
g1-m08     running   False workspace 5/5            45   75   10208  -                6s     pool 3s attest 3s
g1-m09     -         -     -                        -    -    -      -                -      no-pool attest ?
g1-m10     running   False workspace 5/5            24   75   4566   -                3s     pool 5s attest 2s
g1-server  server    -     -                        -    -    -      -                -      pool 5s attest 3s
```
### 2. last log lines
```
g1-m01: Reading the project graph slice (491 s): taken 354 of 4774.
g1-m02: Reading the project graph slice (1278 s): taken 26 of 4774.
g1-m03: Reading the project graph slice (14 s).
g1-m07: Reading the project graph slice (2799 s): taken 364 of 4774.
g1-m08: Reading the project graph slice (1179 s): taken 109 of 4774.
g1-m10: Reading the project graph slice (58 s): rederivation 255 of 255.
g1-m04: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"native-artifact-input-read:23709","mode":"exclusive","elapsedMs":0}}
g1-m05: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:maintenance-parent-observation","mode":"shared","elapsedMs":1.9303000001236796}}
```
### 3. slice-memory
```
[g1-m01]
  uptimeSeconds = 481.7
  memory.rss = 1269.8
  guard.megabytes = 846.4
  treeFootprintMegabytes = 1387
  sources.owners.ledger = {'total': 4774, 'present': 341, 'fetched': 0, 'here': 0, 'stale': 444, 'running': 0, 'wanted': 0, 'done': 341, 'missing': 4433}
  sources.owners.warm = {'total': 4773, 'done': 148, 'settled': 83, 'durable': 65, 'failed': 0, 'lanes': 1, 'peak': 817, 'why': ''}
[g1-m02]
  uptimeSeconds = 1274
  memory.rss = 1522.8
  guard.megabytes = 855.9
  treeFootprintMegabytes = 2992
  sources.owners.ledger = {'total': 4774, 'present': 25, 'fetched': 1, 'here': 0, 'stale': 365, 'running': 0, 'wanted': 4748, 'done': 26, 'missing': 4748}
  sources.owners.warm = {'total': 4773, 'done': 145, 'settled': 105, 'durable': 40, 'failed': 0, 'lanes': 2, 'peak': 2302, 'why': ''}
[g1-m03]
  uptimeSeconds = 1081.2
  memory.rss = 922.3
  guard.megabytes = 420.4
  treeFootprintMegabytes = None
  sources.owners.ledger = None
  sources.owners.warm = None
[g1-m07]
  uptimeSeconds = 2772.4
  memory.rss = 1505.8
  guard.megabytes = 993.6
  treeFootprintMegabytes = 3284
  sources.owners.ledger = {'total': 4774, 'present': 357, 'fetched': 7, 'here': 0, 'stale': 684, 'running': 0, 'wanted': 4410, 'done': 364, 'missing': 4410}
  sources.owners.warm = {'total': 4773, 'done': 221, 'settled': 156, 'durable': 63, 'failed': 2, 'lanes': 1, 'peak': 3691, 'why': 'failure: graph: project @semio-tech/procedural-generation3d-rs needed 5306 MB i 1, failure: graph: project @semio-tech/stdio-ifc-rs needed 4568 MB in a worker pro 1'}
[g1-m08]
  uptimeSeconds = 1149.4
  memory.rss = 1456.4
  guard.megabytes = 907.9
  treeFootprintMegabytes = 2863
  sources.owners.ledger = {'total': 4774, 'present': 109, 'fetched': 0, 'here': 0, 'stale': 453, 'running': 0, 'wanted': 4665, 'done': 109, 'missing': 4665}
  sources.owners.warm = {'total': 4773, 'done': 196, 'settled': 161, 'durable': 35, 'failed': 0, 'lanes': 2, 'peak': 4560, 'why': ''}
[g1-m10]
  uptimeSeconds = 31.1
  memory.rss = 1701
  guard.megabytes = 873
  treeFootprintMegabytes = None
  sources.owners.ledger = None
  sources.owners.warm = None
```
### 4. new failed/stalled/service-failure/containment files since previous sample
```
```
### 5. owners in graph-server read.log (tail -1 of ^[owners] grep; empty means no such line)
```
[owners] owners 8 of 48 done: 8 present, 0 fetched from the fleet, 0 inferred here (0 running), 40 wanted from the fleet, 39 stale in the fleet
```
### ready.json check
```
none ready:true
```

## Sample 21:41:02 UTC
### 1. ready-table
```
origin     state     ready stage                    try  exit peakMB refusal          age    pool/attest
g1-m01     running   False workspace 5/5            35   75   9392   -                4s     pool 2s attest 1s
g1-m02     running   False workspace 5/5            20   75   2825   -                3s     pool 7s attest 0s
g1-m03     running   False workspace 5/5            17   1    10473  -                6s     pool 4s attest 3s
g1-m04     running   False tools 4/5                38   1    1866   -                2s     pool 1s attest 4s
g1-m05     running   False tools 4/5                8    None None   -                0s     pool 21536s attest 312s
g1-m06     running   False workspace 5/5            2    75   7539   -                43158s pool 43156s attest 43155s
g1-m07     running   False workspace 5/5            61   75   10662  -                3s     pool 5s attest 3s
g1-m08     running   False workspace 5/5            45   75   10208  -                1s     pool 2s attest 3s
g1-m09     -         -     -                        -    -    -      -                -      no-pool attest ?
g1-m10     running   False workspace 5/5            24   75   4566   -                4s     pool 3s attest 1s
g1-server  server    -     -                        -    -    -      -                -      pool 3s attest 1s
```
### 2. last log lines
```
g1-m01: Reading the project graph slice (724 s): taken 574 of 4774.
g1-m02: Reading the project graph slice (1517 s): taken 27 of 4774.
g1-m03: Reading the project graph slice (256 s).
g1-m07: Reading the project graph slice (3045 s): taken 439 of 4774.
g1-m08: Reading the project graph slice (1416 s): taken 110 of 4774.
g1-m10: Reading the project graph slice (16 s): facts 24576 of 332752.
g1-m04: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"native-artifact-input-read:25611","mode":"exclusive","elapsedMs":0}}
g1-m05: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-sweep:nested","mode":"exclusive","elapsedMs":48}}
```
### 3. slice-memory
```
[g1-m01]
  uptimeSeconds = 722.7
  memory.rss = 1754.6
  guard.megabytes = 958.9
  treeFootprintMegabytes = 1710
  sources.owners.ledger = {'total': 4774, 'present': 572, 'fetched': 2, 'here': 0, 'stale': 654, 'running': 0, 'wanted': 0, 'done': 574, 'missing': 4200}
  sources.owners.warm = {'total': 4773, 'done': 162, 'settled': 97, 'durable': 65, 'failed': 0, 'lanes': 1, 'peak': 1630, 'why': ''}
[g1-m02]
  uptimeSeconds = 1515.7
  memory.rss = 1570.5
  guard.megabytes = 932.3
  treeFootprintMegabytes = 2412
  sources.owners.ledger = {'total': 4774, 'present': 25, 'fetched': 2, 'here': 0, 'stale': 364, 'running': 0, 'wanted': 4747, 'done': 27, 'missing': 4747}
  sources.owners.warm = {'total': 4773, 'done': 158, 'settled': 117, 'durable': 40, 'failed': 1, 'lanes': 2, 'peak': 3128, 'why': 'failure: graph: project @semio-tech/stdio-ifc-rs needed 4477 MB in a worker pro 1'}
[g1-m03]
  uptimeSeconds = 254.3
  memory.rss = 156.8
  guard.megabytes = 69.1
  treeFootprintMegabytes = None
  sources.owners.ledger = None
  sources.owners.warm = None
[g1-m07]
  uptimeSeconds = 3043.3
  memory.rss = 1698.7
  guard.megabytes = 1067.7
  treeFootprintMegabytes = 1410
  sources.owners.ledger = {'total': 4774, 'present': 357, 'fetched': 82, 'here': 0, 'stale': 684, 'running': 0, 'wanted': 4335, 'done': 439, 'missing': 4335}
  sources.owners.warm = {'total': 4773, 'done': 229, 'settled': 162, 'durable': 64, 'failed': 3, 'lanes': 1, 'peak': 3969, 'why': 'failure: graph: project @semio-tech/procedural-generation3d-rs needed 5306 MB i 1, failure: graph: project @semio-tech/stdio-ifc-rs needed 4568 MB in a worker pro 1, failure: graph: project @semio-tech/repo-mcp-rs needed 4453 MB in a worker proc 1'}
[g1-m08]
  uptimeSeconds = 1392.4
  memory.rss = 1456.7
  guard.megabytes = 703.2
  treeFootprintMegabytes = 2803
  sources.owners.ledger = {'total': 4774, 'present': 109, 'fetched': 1, 'here': 0, 'stale': 452, 'running': 0, 'wanted': 4664, 'done': 110, 'missing': 4664}
  sources.owners.warm = {'total': 4773, 'done': 208, 'settled': 173, 'durable': 35, 'failed': 0, 'lanes': 2, 'peak': 4560, 'why': ''}
[g1-m10]
  uptimeSeconds = 0.1
  memory.rss = 106.2
  guard.megabytes = 17.9
  treeFootprintMegabytes = None
  sources.owners.ledger = None
  sources.owners.warm = None
```
### 4. new failed/stalled/service-failure/containment files since previous sample
```
```
### 5. owners in graph-server read.log (tail -1 of ^[owners] grep; empty means no such line)
```
[owners] owners 8 of 48 done: 8 present, 0 fetched from the fleet, 0 inferred here (0 running), 40 wanted from the fleet, 39 stale in the fleet
```
### ready.json check
```
none ready:true
```

## Sample 21:45:03 UTC
### 1. ready-table
```
origin     state     ready stage                    try  exit peakMB refusal          age    pool/attest
g1-m01     running   False workspace 5/5            35   75   9392   -                6s     pool 0s attest 0s
g1-m02     running   False workspace 5/5            20   75   2825   -                2s     pool 2s attest 0s
g1-m03     running   False workspace 5/5            17   1    10473  -                3s     pool 4s attest 0s
g1-m04     running   False tools 4/5                38   1    1866   -                2s     pool 0s attest 0s
g1-m05     running   False tools 4/5                8    None None   -                3s     pool 21777s attest 167s
g1-m06     running   False workspace 5/5            2    75   7539   -                43398s pool 43396s attest 43395s
g1-m07     running   False workspace 5/5            61   75   10662  -                3s     pool 1s attest 0s
g1-m08     running   False workspace 5/5            45   75   10208  -                2s     pool 3s attest 1s
g1-m09     -         -     -                        -    -    -      -                -      no-pool attest ?
g1-m10     running   False workspace 5/5            24   75   4566   -                3s     pool 4s attest 5s
g1-server  server    -     -                        -    -    -      -                -      pool 1s attest 0s
```
### 2. last log lines
```
g1-m01: Reading the project graph slice (972 s): taken 589 of 4774.
g1-m02: Reading the project graph slice (1754 s): taken 29 of 4774.
g1-m03: Reading the project graph slice (498 s).
g1-m07: Reading the project graph slice (140 s): stage inference 0 of 0.
g1-m08: Reading the project graph slice (1657 s): taken 110 of 4774.
g1-m10: Reading the project graph slice (160 s): facts 69632 of 332791.
g1-m04: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"native-artifact-input-read:28362","mode":"exclusive","elapsedMs":0}}
g1-m05: [repository:command] still running after 4172 s
```
### 3. slice-memory
```
[g1-m01]
  uptimeSeconds = 963.3
  memory.rss = 2264.4
  guard.megabytes = 1050.1
  treeFootprintMegabytes = 1969
  sources.owners.ledger = {'total': 4774, 'present': 572, 'fetched': 17, 'here': 0, 'stale': 824, 'running': 0, 'wanted': 0, 'done': 589, 'missing': 4185}
  sources.owners.warm = {'total': 4773, 'done': 175, 'settled': 110, 'durable': 65, 'failed': 0, 'lanes': 1, 'peak': 1792, 'why': ''}
[g1-m02]
  uptimeSeconds = 1728.6
  memory.rss = 1601.7
  guard.megabytes = 809.7
  treeFootprintMegabytes = 2384
  sources.owners.ledger = {'total': 4774, 'present': 25, 'fetched': 4, 'here': 0, 'stale': 362, 'running': 0, 'wanted': 4745, 'done': 29, 'missing': 4745}
  sources.owners.warm = {'total': 4773, 'done': 166, 'settled': 125, 'durable': 40, 'failed': 1, 'lanes': 2, 'peak': 3128, 'why': 'failure: graph: project @semio-tech/stdio-ifc-rs needed 4477 MB in a worker pro 1'}
[g1-m03]
  uptimeSeconds = 495
  memory.rss = 134.3
  guard.megabytes = 70.7
  treeFootprintMegabytes = None
  sources.owners.ledger = None
  sources.owners.warm = None
[g1-m07]
  uptimeSeconds = 121.1
  memory.rss = 2500.6
  guard.megabytes = 1195.7
  treeFootprintMegabytes = 351
  sources.owners.ledger = None
  sources.owners.warm = None
[g1-m08]
  uptimeSeconds = 1634.4
  memory.rss = 1544.9
  guard.megabytes = 986.4
  treeFootprintMegabytes = 3669
  sources.owners.ledger = {'total': 4774, 'present': 109, 'fetched': 1, 'here': 0, 'stale': 452, 'running': 0, 'wanted': 4664, 'done': 110, 'missing': 4664}
  sources.owners.warm = {'total': 4773, 'done': 223, 'settled': 188, 'durable': 35, 'failed': 0, 'lanes': 2, 'peak': 4560, 'why': ''}
[g1-m10]
  uptimeSeconds = 159.3
  memory.rss = 2486.4
  guard.megabytes = 1150.7
  treeFootprintMegabytes = 1019
  sources.owners.ledger = None
  sources.owners.warm = None
```
### 4. new failed/stalled/service-failure/containment files since previous sample
```
```
### 5. owners in graph-server read.log (tail -1 of ^[owners] grep; empty means no such line)
```
[owners] owners 8 of 48 done: 8 present, 0 fetched from the fleet, 0 inferred here (0 running), 40 wanted from the fleet, 39 stale in the fleet
```
### ready.json check
```
none ready:true
```

## Sample 21:48:54 UTC
### 1. ready-table
```
origin     state     ready stage                    try  exit peakMB refusal          age    pool/attest
g1-m01     running   False workspace 5/5            35   75   9392   -                3s     pool 4s attest 1s
g1-m02     running   False workspace 5/5            20   75   2825   -                5s     pool 5s attest 1s
g1-m03     running   False workspace 5/5            17   1    10473  -                5s     pool 3s attest 1s
g1-m04     running   False tools 4/5                38   1    1866   -                1s     pool 3s attest 0s
g1-m05     backoff   False tools 4/5                9    None None   stage-timeout    14s    pool 22008s attest 144s
g1-m06     running   False workspace 5/5            2    75   7539   -                43629s pool 43628s attest 43626s
g1-m07     running   False workspace 5/5            61   75   10662  -                2s     pool 4s attest 2s
g1-m08     running   False workspace 5/5            45   75   10208  -                8s     pool 4s attest 1s
g1-m09     -         -     -                        -    -    -      -                -      no-pool attest ?
g1-m10     running   False workspace 5/5            24   75   4566   -                3s     pool 3s attest 1s
g1-server  server    -     -                        -    -    -      -                -      pool 1s attest 2s
```
### 2. last log lines
```
g1-m01: Reading the project graph slice (1197 s): taken 590 of 4774.
g1-m02: Reading the project graph slice (1991 s): taken 29 of 4774.
g1-m03: Reading the project graph slice (734 s): taken 4 of 4774.
g1-m07: Reading the project graph slice (374 s): taken 17 of 4774.
g1-m08: Reading the project graph slice (1890 s): taken 111 of 4774.
g1-m10: Reading the project graph slice (396 s): owners 1 of 482; running (name, kind, seconds, rows): @semio-tech/assets (project, 23, 0).
g1-m04: [repository:command] still running after 5651 s
g1-m05: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:maintenance-parent-observation","mode":"shared","elapsedMs":3.716000000014901}}
```
### 3. slice-memory
```
[g1-m01]
  uptimeSeconds = 1173.9
  memory.rss = 1597.2
  guard.megabytes = 891.9
  treeFootprintMegabytes = 2316
  sources.owners.ledger = {'total': 4774, 'present': 572, 'fetched': 18, 'here': 0, 'stale': 826, 'running': 0, 'wanted': 4184, 'done': 590, 'missing': 4184}
  sources.owners.warm = {'total': 4773, 'done': 199, 'settled': 134, 'durable': 65, 'failed': 0, 'lanes': 1, 'peak': 1792, 'why': ''}
[g1-m02]
  uptimeSeconds = 1969.6
  memory.rss = 1523.4
  guard.megabytes = 819.2
  treeFootprintMegabytes = 1249
  sources.owners.ledger = {'total': 4774, 'present': 25, 'fetched': 4, 'here': 0, 'stale': 362, 'running': 0, 'wanted': 4745, 'done': 29, 'missing': 4745}
  sources.owners.warm = {'total': 4773, 'done': 183, 'settled': 142, 'durable': 40, 'failed': 1, 'lanes': 2, 'peak': 3128, 'why': 'failure: graph: project @semio-tech/stdio-ifc-rs needed 4477 MB in a worker pro 1'}
[g1-m03]
  uptimeSeconds = 735.2
  memory.rss = 3139.2
  guard.megabytes = 689.6
  treeFootprintMegabytes = 1175
  sources.owners.ledger = {'total': 4774, 'present': 1, 'fetched': 3, 'here': 0, 'stale': 51, 'running': 0, 'wanted': 0, 'done': 4, 'missing': 4770}
  sources.owners.warm = {'total': 1420, 'done': 15, 'settled': 4, 'durable': 11, 'failed': 0, 'lanes': 1, 'peak': 2293, 'why': ''}
[g1-m07]
  uptimeSeconds = 361.2
  memory.rss = 1463.7
  guard.megabytes = 807.1
  treeFootprintMegabytes = 2969
  sources.owners.ledger = {'total': 4774, 'present': 16, 'fetched': 0, 'here': 0, 'stale': 251, 'running': 0, 'wanted': 0, 'done': 16, 'missing': 4758}
  sources.owners.warm = {'total': 4773, 'done': 17, 'settled': 17, 'durable': 0, 'failed': 0, 'lanes': 1, 'peak': 1548, 'why': ''}
[g1-m08]
  uptimeSeconds = 1876.6
  memory.rss = 1504.1
  guard.megabytes = 1024.2
  treeFootprintMegabytes = 4155
  sources.owners.ledger = {'total': 4774, 'present': 109, 'fetched': 2, 'here': 0, 'stale': 451, 'running': 0, 'wanted': 4663, 'done': 111, 'missing': 4663}
  sources.owners.warm = {'total': 4773, 'done': 239, 'settled': 204, 'durable': 35, 'failed': 0, 'lanes': 2, 'peak': 4560, 'why': ''}
[g1-m10]
  uptimeSeconds = 380.3
  memory.rss = 4570.4
  guard.megabytes = 2762.7
  treeFootprintMegabytes = None
  sources.owners.ledger = {'total': 1, 'present': 0, 'fetched': 0, 'here': 0, 'stale': 1, 'running': 1, 'wanted': 1, 'done': 0, 'missing': 1}
  sources.owners.warm = None
```
### 4. new failed/stalled/service-failure/containment files since previous sample
```
```
### 5. owners in graph-server read.log (tail -1 of ^[owners] grep; empty means no such line)
```
[owners] owners 8 of 48 done: 8 present, 0 fetched from the fleet, 0 inferred here (0 running), 40 wanted from the fleet, 39 stale in the fleet
```
### ready.json check
```
none ready:true
```

## Sample 21:52:46 UTC
### 1. ready-table
```
origin     state     ready stage                    try  exit peakMB refusal          age    pool/attest
g1-m01     running   False workspace 5/5            35   75   9392   -                3s     pool 1s attest 0s
g1-m02     running   False workspace 5/5            20   75   2825   -                3s     pool 1s attest 0s
g1-m03     running   False workspace 5/5            17   1    10473  -                3s     pool 1s attest 0s
g1-m04     running   False tools 4/5                38   1    1866   -                8s     pool 2s attest 0s
g1-m05     backoff   False tools 4/5                9    None None   stage-timeout    5s     pool 22240s attest 96s
g1-m06     running   False workspace 5/5            2    75   7539   -                43862s pool 43860s attest 43858s
g1-m07     running   False workspace 5/5            61   75   10662  -                2s     pool 3s attest 1s
g1-m08     running   False workspace 5/5            45   75   10208  -                3s     pool 2s attest 1s
g1-m09     -         -     -                        -    -    -      -                -      no-pool attest ?
g1-m10     running   False workspace 5/5            24   75   4566   -                2s     pool 5s attest 2s
g1-server  server    -     -                        -    -    -      -                -      pool 0s attest 2s
```
### 2. last log lines
```
g1-m01: Reading the project graph slice (1431 s): taken 590 of 4774.
g1-m02: Reading the project graph slice (106 s).
g1-m03: Reading the project graph slice (0 s): the slice worker grew to 10216 MB, more than the 6553 MB a plan may take on this machine, so the work is cut smaller (step 5 of 6: fewer projects at once and smaller waves) and continues; projects already inferred are kept and nothing is repeated.
g1-m07: Reading the project graph slice (599 s): taken 17 of 4774.
g1-m08: Reading the project graph slice (2123 s): taken 113 of 4774.
g1-m10: Reading the project graph slice (625 s): owners 2 of 482; running (name, kind, seconds, rows): @semio-tech/browser-actor-import-test (project, 161, 49).
g1-m04: [repository:command] still running after 5881 s
g1-m05: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:maintenance-parent-observation","mode":"shared","elapsedMs":3.716000000014901}}
```
### 3. slice-memory
```
[g1-m01]
  uptimeSeconds = 1416
  memory.rss = 1595.9
  guard.megabytes = 948.7
  treeFootprintMegabytes = 1954
  sources.owners.ledger = {'total': 4774, 'present': 572, 'fetched': 18, 'here': 0, 'stale': 826, 'running': 0, 'wanted': 4184, 'done': 590, 'missing': 4184}
  sources.owners.warm = {'total': 4773, 'done': 217, 'settled': 152, 'durable': 65, 'failed': 0, 'lanes': 1, 'peak': 1792, 'why': ''}
[g1-m02]
  uptimeSeconds = 100.8
  memory.rss = 1028.5
  guard.megabytes = 210.8
  treeFootprintMegabytes = 126
  sources.owners.ledger = None
  sources.owners.warm = None
[g1-m03]
  uptimeSeconds = 31
  memory.rss = 643.4
  guard.megabytes = 534.2
  treeFootprintMegabytes = None
  sources.owners.ledger = None
  sources.owners.warm = None
[g1-m07]
  uptimeSeconds = 601
  memory.rss = 1622
  guard.megabytes = 972.3
  treeFootprintMegabytes = 2795
  sources.owners.ledger = None
  sources.owners.warm = None
[g1-m08]
  uptimeSeconds = 2119.6
  memory.rss = 1203.4
  guard.megabytes = 1005
  treeFootprintMegabytes = 3996
  sources.owners.ledger = {'total': 4774, 'present': 109, 'fetched': 4, 'here': 0, 'stale': 449, 'running': 0, 'wanted': 4661, 'done': 113, 'missing': 4661}
  sources.owners.warm = {'total': 4773, 'done': 245, 'settled': 209, 'durable': 35, 'failed': 1, 'lanes': 2, 'peak': 4560, 'why': 'failure: graph: project @semio-tech/draw-plugin needed 4706 MB in a worker proc 1'}
[g1-m10]
  uptimeSeconds = 597
  memory.rss = 6609.9
  guard.megabytes = 3172.2
  treeFootprintMegabytes = 2741
  sources.owners.ledger = {'total': 1, 'present': 0, 'fetched': 0, 'here': 0, 'stale': 1, 'running': 1, 'wanted': 1, 'done': 0, 'missing': 1}
  sources.owners.warm = None
```
### 4. new failed/stalled/service-failure/containment files since previous sample
```
```
### 5. owners in graph-server read.log (tail -1 of ^[owners] grep; empty means no such line)
```
[owners] owners 8 of 48 done: 8 present, 0 fetched from the fleet, 0 inferred here (0 running), 40 wanted from the fleet, 39 stale in the fleet
```
### ready.json check
```
none ready:true
```

## Sample 21:52:54 UTC
### 1. ready-table
```
origin     state     ready stage                    try  exit peakMB refusal          age    pool/attest
g1-m01     running   False workspace 5/5            35   75   9392   -                5s     pool 2s attest 3s
g1-m02     running   False workspace 5/5            20   75   2825   -                5s     pool 3s attest 0s
g1-m03     running   False workspace 5/5            17   1    10473  -                5s     pool 4s attest 0s
g1-m04     running   False tools 4/5                38   1    1866   -                0s     pool 2s attest 3s
g1-m05     backoff   False tools 4/5                9    None None   stage-timeout    13s    pool 22248s attest 104s
g1-m06     running   False workspace 5/5            2    75   7539   -                43869s pool 43868s attest 43866s
g1-m07     running   False workspace 5/5            61   75   10662  -                3s     pool 3s attest 4s
g1-m08     running   False workspace 5/5            45   75   10208  -                3s     pool 2s attest 1s
g1-m09     -         -     -                        -    -    -      -                -      no-pool attest ?
g1-m10     running   False workspace 5/5            24   75   4566   -                4s     pool 6s attest 2s
g1-server  server    -     -                        -    -    -      -                -      pool 2s attest 1s
```
### 2. last log lines
```
g1-m01: Reading the project graph slice (1441 s): taken 590 of 4774.
g1-m02: Reading the project graph slice (116 s): stage inference 0 of 0.
g1-m03: Reading the project graph slice (10 s).
g1-m07: Reading the project graph slice (8 s).
g1-m08: Reading the project graph slice (2123 s): taken 113 of 4774.
g1-m10: Reading the project graph slice (637 s): stage environment 0 of 0.
g1-m04: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"native-artifact-input-read:30134","mode":"exclusive","elapsedMs":0}}
g1-m05: [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:maintenance-parent-observation","mode":"shared","elapsedMs":3.716000000014901}}
```
### 3. slice-memory
```
[g1-m01]
  uptimeSeconds = 1416
  memory.rss = 1595.9
  guard.megabytes = 948.7
  treeFootprintMegabytes = 1954
  sources.owners.ledger = {'total': 4774, 'present': 572, 'fetched': 18, 'here': 0, 'stale': 826, 'running': 0, 'wanted': 4184, 'done': 590, 'missing': 4184}
  sources.owners.warm = {'total': 4773, 'done': 217, 'settled': 152, 'durable': 65, 'failed': 0, 'lanes': 1, 'peak': 1792, 'why': ''}
[g1-m02]
  uptimeSeconds = 100.8
  memory.rss = 1028.5
  guard.megabytes = 210.8
  treeFootprintMegabytes = 126
  sources.owners.ledger = None
  sources.owners.warm = None
[g1-m03]
  uptimeSeconds = 0.3
  memory.rss = 104.3
  guard.megabytes = 17.9
  treeFootprintMegabytes = None
  sources.owners.ledger = None
  sources.owners.warm = None
[g1-m07]
  uptimeSeconds = 0.2
  memory.rss = 105.1
  guard.megabytes = 17.8
  treeFootprintMegabytes = None
  sources.owners.ledger = None
  sources.owners.warm = None
[g1-m08]
  uptimeSeconds = 2119.6
  memory.rss = 1203.4
  guard.megabytes = 1005
  treeFootprintMegabytes = 3996
  sources.owners.ledger = {'total': 4774, 'present': 109, 'fetched': 4, 'here': 0, 'stale': 449, 'running': 0, 'wanted': 4661, 'done': 113, 'missing': 4661}
  sources.owners.warm = {'total': 4773, 'done': 245, 'settled': 209, 'durable': 35, 'failed': 1, 'lanes': 2, 'peak': 4560, 'why': 'failure: graph: project @semio-tech/draw-plugin needed 4706 MB in a worker proc 1'}
[g1-m10]
  uptimeSeconds = 633.7
  memory.rss = 5963.9
  guard.megabytes = 3676.6
  treeFootprintMegabytes = 2869
  sources.owners.ledger = {'total': 1, 'present': 0, 'fetched': 0, 'here': 0, 'stale': 1, 'running': 1, 'wanted': 1, 'done': 0, 'missing': 1}
  sources.owners.warm = None
```
### 4. new failed/stalled/service-failure/containment files since previous sample
```
```
### 5. owners in graph-server read.log (tail -1 of ^[owners] grep; empty means no such line)
```
[owners] owners 8 of 48 done: 8 present, 0 fetched from the fleet, 0 inferred here (0 running), 40 wanted from the fleet, 39 stale in the fleet
```
### ready.json check
```
none ready:true
```

## What happened

Samples: 21 between 20:36:27 and 21:52:55 UTC (about 4 min apart, the last one at 21:52:55, the next would fall after 21:55). The first sample's step 3 was mis-scripted (joined loop string); the six slice-memory values for that time were re-read at 20:38:14 and are in the file as a separate block. Step 5 (`[owners]` in graph-server read.log) disappeared from the log tail between about 21:04 and 21:12 and returned as `[owners] owners 8 of 48 done` from 21:16 on.

Ready state: ready-table stage stayed `workspace 5/5` for g1-m01, m02, m03, m06, m07, m08, m10 and `tools 4/5` for g1-m04 and g1-m05 for the whole window. No `ready.json` contained `"ready": true` at any sample. No file named failed, stalled, service-failure or containment appeared in any origin folder.

Furthest phase per machine:
- g1-m10: furthest. Decisions 19406 of 19406 (~20:44), verification 2430 of 2430 (21:28), rederivation 255 of 255 (21:37), then owners 1 then 2 of 482 (21:48 to 21:52). Owner stage was still running at the last sample.
- g1-m01: taken 590 of 4774 by 21:48 (2 at 20:36; 354 at 21:37; 590 at 21:48). Overall about 0.14 owners/s; about 0.36/s between 21:37 and 21:48 (partly restart-limited).
- g1-m07: taken 362 of 4774 by 21:28, 439 at 21:41, then restarted at about 21:43 and fell back to 17.
- g1-m08: taken 113 of 4774 by 21:52. g1-m03: 76 at 21:12, then repeated restarts.
- g1-m02: 29 of 4774 by 21:45.
- g1-m04 ("tools" stage 4/5): repository:command still running after 5881 s at 21:52. g1-m05: stuck in tools 4/5 with pool/attest ages growing; backoff from 21:48.
- graph-server read.log owners: 2 of 48 at 20:40, 8 of 48 from 21:16 to 21:52 (no change).

Failures seen, verbatim:
- 20:36:27 g1-m10: "The resident service met an unexpected failure (reference 79312331): Error: Monotonic graph phase progress required; try again, ..."
- 20:44:10 g1-m03: "waiting 5s for run: not enough free memory (need 512 MB; ..."; 20:48 to 21:12 g1-m03 and 20:52 g1-m10: "waiting for memory: 6553 MB are asked for (2048 MB are enough to start) ... the request is refused after 120 s if nothing of ours runs"
- Warm-stage failures (sources.owners.warm why), each with a time: 20:40 "failure: graph: project @semio-tech/repo-identity-rs needed 4426 MB in a worker 1" (g1-m08); 20:52 to 21:24 "failure: graph: project @semio-tech/procedural-generation3d-rs needed 5306 MB i 1" (machine not stated on the line); 21:08 "...stdio-ifc-rs needed 4568 MB in a worker pro 1"; 21:16 "...repo-search-rs needed 5487 MB in a worker p 1"; 21:41 "...repo-mcp-rs needed 4453 MB in a worker proc 1"; 21:52:46 "failure: graph: project @semio-tech/draw-plugin needed 4706 MB in a worker proc 1" (machine not stated).
- 21:52:46 g1-m03: "the slice worker grew to 10216 MB, more than the 6553 MB a plan may take on this machine, so the work is cut smaller (step 5 of 6 ...)".
- Recurring "lost: closed 1, farm-off 4717" in the warm-stage why field, 20:48 to 21:00 (machine not stated).

Restarts (uptime drops): very frequent. g1-m10 reset about every 4 to 10 min from 20:40 to 21:52 (uptimes 30, 0.1, 0.2, 0.1 at 20:40, 21:00, 21:24, 21:32, 21:40). g1-m03 reset about 20:55, 21:00, 21:15, 21:18, 21:32, 21:36, 21:51 and 21:52:50. g1-m07 about 20:46, 21:43, 21:52:53. g1-m01 about 20:43, 20:59, 21:28. g1-m02 about 20:42, 21:01, 21:15, 21:51. g1-m08 about 20:51, 21:17. The log lines just before the resets were the normal "taken N" lines; the causes were not logged by the sampled files.

Memory peaks (RSS, MB, from slice-memory.json): g1-m10 6610 (21:52:46); g1-m02 4758 (20:48); g1-m03 4759 (20:56); g1-m08 4600 (21:04); g1-m01 2927 (20:52); g1-m07 2901 (20:40).
