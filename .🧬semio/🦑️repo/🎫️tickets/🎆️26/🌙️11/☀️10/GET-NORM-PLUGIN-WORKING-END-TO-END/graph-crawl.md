# Graph crawl diagnosis (11-17)

- Leader: slice worker of this host's graph-plan (pid 81451 / worker 26294), 2048 MB booked, 4.3 GB real. It was NOT started by me; it serves every waiting `repo run`.
- Final ledger of the 87 min load: 326 of 4774 done = 136 present, 147 fetched, 43 inferred here, 4448 "wanted from the fleet". Fleet: 7 machines report but "Graph: no builder", remote cache :6094 refuses connections. The read published 4448 owners and waits; local inference = leader farm (planFarmWorkersV1 = 2 children on 16 GB, treeShare 0.4) + sidecar owner-pool children (4-7, drained on swap-out>64 MB/s or load, recycled at 1200 MB after 1 owner).
- Load died with build.graph.no-progress: owner s-norm-composition-tests ran 974 s with 0 rows (normally 48 s); the 5-min watch counts only completed events/rows, so one slow CPU-bound owner aborts the whole load and the next load starts again.
- Owner costs: median 46-80 s CPU per cargo/rust owner (recorded), host load 22-27 on 10 cores, swap 1-1.4 GB; only 25 owners truly inferred in the last hour.
- Graph partition 107 % of quota (inference/ce closure entries 4.7 GB vs 1.25 GiB closures budget, image 12 GiB vs 6 declared, recreate pending).
- Tooling code is edited every ~30 min (sidecar/pool/graph restarts at 13:00, 13:32, 14:00, 14:30, 14:51) -> pool waves reborn (10 waves in 2 h), leaders restart from 0.
- Crash reported by coordinator (`undefined is not an object (evaluating 's.busy')`): 🏊️project-farm describe() read pool.farm.snapshot of a pool whose farm had none -> fixed with a guard + test 🩹️describe-guard.
- NOT fixed (needs design): fleet-first wait when fleet has no builder; no-progress watch should count CPU of running owners; hit/closure-entry loading cost (queued row-reference layer format).
