# R110: local rehearsal of the fleet (`🏟️fleet-rehearsal`)

Scripts: `T/🧰️scripts/R110/` (T = `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION`): `rehearse.sh` (the rerun command), `window.sh` (the landing of the new folder; it copied the sources from a `kit/` folder that was removed afterwards, the sources live in the library), `seam/` + `stage.py` (the staged state-directory seam, NOT landed), `tests.sh` (baseline runner of the members the seam touches).

## Rerun

```
cd /Users/ueli/Documents/semio && ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R110/rehearse.sh" --machines=3 --memory=16384 --cores=8 --owners=470 --owner-seconds=3 --owner-megabytes=128 --out=<dir>
```

`--memory=8192 --cores=4` (an 8 GB worker), `--scenario=landing --landing-after=30` (second scenario; `--landing-keeps-revision` for a landing that does not move the record revision), `--timeout=<s>`, `--keep` (keeps the work directory), and the subcommand `audit`, which names the state of the checkout a simulated machine cannot isolate: `/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🏟️fleet-rehearsal/📜️script.ts" audit --root=/Users/ueli/Documents/semio`. Exit status 1 when the verdict fails. The command lives in `🧰️framework/.../📚️library/🧪️testing/🏟️fleet-rehearsal/` (`🟦️.ts` library, `📜️script.ts` command, `👷️machine` one simulated machine, `🐜️owner` one owner, `🧬️schema`, `🧪️tests`), next to `🌱️genuine-birth`. macOS and Linux (it lists processes with `ps`); on Windows it refuses with a bilingual text.

## Read this first: what is real and what is a model

The task asked to run the real `workspace` stage (`repo run workspace:setup`) of N machines on the real checkout. **That cannot be isolated today** (next section), and running it anyway would have written into the live group server's graph partition. So the kit runs the part of the readiness path that can be isolated with the real code, and models the rest:

Real code (not copies): the shared cache server with its claim table (`startRemoteCacheServerV1`, temporary directories, loopback), the fleet client (`graphFleetClientV1`), the entry protocol (`tryGraphEntryV1`, `publishGraphEntryOutcomeV1`, `fetchGraphEntryV1`: claim, heartbeat, publish, release), the wave publisher and pointer (`publishOwnerPoolV1`, `readOwnerPoolPointerV1`, `readOwnerPoolV1`, wave ids), the puller (`pullOwnerPoolV1`: rendezvous start, claim windows, gallop, lease `claimTtlMilliseconds` 60 s), the revision rule (`ownerPoolAcceptsRevisionV1`), the capacity target (`ownerPoolTargetV1`), and the declared worker: every machine gets `SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES` and `SEMIO_HOST_CAPACITY_CORES`, and the driver reads its machine through `hostProfileV1` (the seam every limit of the build derives from), the footprint limit is `PLAN_STAGE_POLICY_V1.treeShare` of that machine (6553 MB of 16 GB, 3276 MB of 8 GB; the figure in the refusals of the workers), the child limits `ownerPoolChildLimitsV1`. Each machine is a separate process tree (driver plus one process per owner) with its own origin name, state, temporary, runtime, capacity and lease-registry directories and its own TMPDIR; the cache server and the store directory are one temporary directory; the checkout is only read (the library is imported from it).

Model: an owner is a process that holds `--owner-megabytes` and burns the processor for half of `--owner-seconds`; the take pass of a reader is one `fetchGraphEntryV1` per owner; there is no inference, no record, no closure entry, no plan stage, no memory refusal, no fleet identity, no resident service, no replica. The numbers (owners per minute, footprints) are therefore those of the protocol under load, not of the real inference. Faults that depend on the cost of an owner (13 s per lookup, 10 GB plan cuts) cannot show here; faults of the exchange (idle pools after a revision move, duplicate work, held claims, orphans) can.

## The first wall: the real stage on one checkout

| state | where | can a machine move it? |
| --- | --- | --- |
| native storage images and the graph facts of a read | `<checkout>/.cache/b/<partition>/m` (11 mounts now); `repositoryBuildGraphDataDirectoryV1(workspace)` joins `.cache/b/g/m/<namespace>` below the checkout, one writer custody per checkout | **no variable**. Machines on one checkout would share every record (so nothing is ever fetched from "another machine") and would queue behind, and write into, the partition of the real group server |
| `.nx` workspace data | keyed by the checkout root | no variable for our uses |
| `.cache/f` (identity `machine.json`, cache facts, ready, owner pool, graph state, slice report, claims directory) | 38 literal sites in 21 files on the readiness path (`join(root,".cache","f",...)`), more in the replica daemon | **no variable**. The seam is written and tested and NOT landed: `T/🧰️scripts/R110/seam/` (`SEMIO_MACHINE_STATE_DIRECTORY`, `machineStateDirectoryV1`, `machineStatePathV1`, a law test that is red while any of the 21 files still spells the directory out) and `stage.py` (anchored rewrite of the 21 files from their live text, verified counts). It alone does not unblock the stage (row 1) and it touches hot files of the fleet, so it waits for a decision |
| resident host service (the graph read runs in it) | macOS: for a checkout below Documents it is started as a launchd job with an allow-listed environment (`ALLOWED_NAMES_V1`: HOME, PATH, TMPDIR, XDG_*, ...), so `SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES`, `SEMIO_FLEET_*` never reach the service that derives the plan limit | `SEMIO_SERVICE_LAUNCHD=off` (set by the kit) makes it a direct child that inherits the environment; the runtime directory (socket, key, log) follows TMPDIR, so the services of the machines would be apart |
| fleet cache endpoint | the readiness stage drops `SEMIO_REMOTE_CACHE_*` (`droppedEnvironment`), so the endpoint comes from `.cache/f/cache-facts.json` | needs the `.cache/f` seam; an unsigned facts file and an open loopback server are accepted by the resolver (read from the code, not run) |
| side effects of the stage | `repo run workspace:setup` runs `deps-js-all`, cargo builds and writes `node_modules` of the checkout | not read-only: the stage would have to run as `repo run workspace:setup --dry-run` (the plan is printed, nothing runs; the graph read is the same) |
| slice size | "the slice" is the owners that are missing locally (g1-m10: 473 of 4774); an empty simulated machine would want about 4774 | small only by seeding local records or per-machine trees |

What a real-stage mode needs, one of: (a) one checkout root per machine (an APFS clone `cp -c` costs no disk, but the rules forbid copies of the tree), or (b) a storage-root seam for `.cache/b` and `.nx` (images per machine) on top of the `.cache/f` seam. `bun 📜️script.ts audit` prints this table for the live tree and says "redirected" for `.cache/f` as soon as the seam exists. This is the decision the coordinator has to take.

## First timeline and verdict (steady: 3 machines of 16 GB, 470 owners of 3 s and 128 MB, run from the landed command)

- verdict: PASS
- every machine ready: true (rehearsal-m1 71.8 s, rehearsal-m2 76.9 s, rehearsal-m3 87.1 s)
- owners inferred per machine: {"rehearsal-m1":161,"rehearsal-m2":153,"rehearsal-m3":156}, fetched: {"rehearsal-m1":309,"rehearsal-m2":317,"rehearsal-m3":314}, inferred in total: 470
- owners stored more than once: 0
- peak footprint (limit 6553 MB): rehearsal-m1 1305 MB within, rehearsal-m2 1303 MB within, rehearsal-m3 1303 MB within
- owners per minute: 316.5, time to the end: 89.1 s
- orphan processes after the end: 0, work directory removed: true

| t (s) | machine | total | present | fetched | here | wanted | pool | reason | running | footprint MB | cpu % |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 5.1 | rehearsal-m1 | 470 | 0 | 0 | 0 | 470 | pulling | - | 8 | 1302 | 11.4 |
| 5.1 | rehearsal-m2 | 470 | 0 | 0 | 0 | 470 | pulling | - | 8 | 1295 | 23 |
| 5.1 | rehearsal-m3 | 470 | 0 | 0 | 0 | 470 | pulling | - | 8 | 1301 | 13.2 |
| 10.1 | rehearsal-m1 | 470 | 0 | 0 | 16 | 454 | pulling | - | 8 | 1293 | 281 |
| 10.1 | rehearsal-m2 | 470 | 0 | 0 | 16 | 454 | pulling | - | 8 | 1289 | 289.4 |
| 10.1 | rehearsal-m3 | 470 | 0 | 0 | 16 | 454 | pulling | - | 8 | 1290 | 250.1 |
| 15.2 | rehearsal-m1 | 470 | 0 | 0 | 31 | 439 | pulling | - | 7 | 988 | 189.3 |
| 15.2 | rehearsal-m2 | 470 | 0 | 0 | 27 | 443 | pulling | - | 7 | 992 | 2.7 |
| 15.2 | rehearsal-m3 | 470 | 0 | 0 | 30 | 440 | pulling | - | 8 | 1268 | 111.6 |
| 20.2 | rehearsal-m1 | 470 | 0 | 0 | 40 | 430 | pulling | - | 8 | 1305 | 62.1 |
| 20.2 | rehearsal-m2 | 470 | 0 | 0 | 40 | 430 | pulling | - | 8 | 1301 | 194.8 |
| 20.2 | rehearsal-m3 | 470 | 0 | 0 | 40 | 430 | pulling | - | 8 | 1303 | 91.2 |
| 25.3 | rehearsal-m1 | 470 | 0 | 0 | 56 | 414 | pulling | - | 8 | 1294 | 212.4 |
| 25.3 | rehearsal-m2 | 470 | 0 | 0 | 56 | 414 | pulling | - | 8 | 1291 | 197.2 |
| 25.3 | rehearsal-m3 | 470 | 0 | 0 | 56 | 414 | pulling | - | 8 | 1291 | 193.4 |
| 30.3 | rehearsal-m1 | 470 | 0 | 0 | 65 | 405 | pulling | - | 8 | 1046 | 3.8 |
| 30.3 | rehearsal-m2 | 470 | 0 | 0 | 64 | 406 | pulling | - | 8 | 1303 | 10.7 |
| 30.3 | rehearsal-m3 | 470 | 0 | 0 | 64 | 406 | pulling | - | 8 | 1303 | 6.6 |
| 35.4 | rehearsal-m1 | 470 | 0 | 0 | 80 | 390 | pulling | - | 8 | 1301 | 169.5 |
| 35.4 | rehearsal-m2 | 470 | 0 | 0 | 80 | 390 | pulling | - | 8 | 1292 | 272.2 |
| 35.4 | rehearsal-m3 | 470 | 0 | 0 | 80 | 390 | pulling | - | 8 | 1299 | 233.2 |
| 41.2 | rehearsal-m1 | 470 | 0 | 0 | 96 | 374 | pulling | - | 8 | 1273 | 239.1 |
| 41.2 | rehearsal-m2 | 470 | 0 | 0 | 96 | 374 | pulling | - | 8 | 1271 | 246.3 |
| 41.2 | rehearsal-m3 | 470 | 0 | 0 | 96 | 374 | pulling | - | 8 | 1273 | 281.7 |
| 46.5 | rehearsal-m1 | 470 | 0 | 0 | 109 | 361 | pulling | - | 5 | 820 | 185.1 |
| 46.5 | rehearsal-m2 | 470 | 0 | 0 | 104 | 366 | idle | - | 0 | 62 | 0.9 |
| 46.5 | rehearsal-m3 | 470 | 0 | 0 | 106 | 364 | pulling | - | 2 | 363 | 63.5 |
| 51.6 | rehearsal-m1 | 470 | 0 | 0 | 120 | 350 | pulling | - | 8 | 1285 | 68.9 |
| 51.6 | rehearsal-m2 | 470 | 0 | 0 | 120 | 350 | pulling | - | 8 | 1283 | 157.2 |
| 51.6 | rehearsal-m3 | 470 | 0 | 0 | 120 | 350 | pulling | - | 8 | 1282 | 111.9 |
| 56.7 | rehearsal-m1 | 470 | 0 | 0 | 136 | 334 | pulling | - | 8 | 1247 | 231.4 |
| 56.7 | rehearsal-m2 | 470 | 0 | 0 | 130 | 340 | pulling | - | 8 | 1274 | 214.5 |
| 56.7 | rehearsal-m3 | 470 | 0 | 0 | 136 | 334 | pulling | - | 8 | 1262 | 225.1 |
| 61.7 | rehearsal-m1 | 470 | 0 | 0 | 146 | 324 | pulling | - | 8 | 1283 | 168.8 |
| 61.7 | rehearsal-m2 | 470 | 0 | 0 | 144 | 326 | pulling | - | 5 | 823 | 111.5 |
| 61.7 | rehearsal-m3 | 470 | 0 | 0 | 146 | 324 | pulling | - | 8 | 1281 | 117.2 |
| 66.8 | rehearsal-m1 | 470 | 0 | 0 | 160 | 310 | pulling | - | 1 | 216 | 2.7 |
| 66.8 | rehearsal-m2 | 470 | 0 | 0 | 153 | 317 | idle | - | 0 | 63 | 0.2 |
| 66.8 | rehearsal-m3 | 470 | 0 | 0 | 156 | 314 | idle | - | 0 | 61 | 0.2 |
| 71.8 | rehearsal-m1 | 470 | 0 | 309 | 161 | 0 | idle | - | 0 | 75 | 1.2 |
| 71.8 | rehearsal-m2 | 470 | 0 | 222 | 153 | 95 | idle | - | 0 | 82 | 25.9 |
| 71.8 | rehearsal-m3 | 470 | 0 | 0 | 156 | 314 | idle | - | 0 | 61 | 0.2 |
| 76.9 | rehearsal-m1 | 470 | 0 | 309 | 161 | 0 | idle | - | 0 | 75 | 0.1 |
| 76.9 | rehearsal-m2 | 470 | 0 | 317 | 153 | 0 | idle | - | 0 | 82 | 0.5 |
| 76.9 | rehearsal-m3 | 470 | 0 | 0 | 156 | 314 | idle | - | 0 | 61 | 0.2 |
| 82 | rehearsal-m1 | 470 | 0 | 309 | 161 | 0 | idle | - | 0 | 75 | 0.2 |
| 82 | rehearsal-m2 | 470 | 0 | 317 | 153 | 0 | idle | - | 0 | 82 | 1.2 |
| 82 | rehearsal-m3 | 470 | 0 | 17 | 156 | 297 | idle | - | 0 | 66 | 9.8 |
| 87.1 | rehearsal-m1 | 470 | 0 | 309 | 161 | 0 | idle | - | 0 | 75 | 0.7 |
| 87.1 | rehearsal-m2 | 470 | 0 | 317 | 153 | 0 | idle | - | 0 | 82 | 0.5 |
| 87.1 | rehearsal-m3 | 470 | 0 | 314 | 156 | 0 | idle | - | 0 | 75 | 0.1 |


Reading: every machine reached ready (rehearsal-m1 71.8 s, rehearsal-m2 76.9 s, rehearsal-m3 87.1 s; a machine is ready when it knows every owner, the last owners are fetched at once because everything is stored); each owner was stored exactly once (0 owner keys with a second PUT at the cache server, counted per owner key at the proxy of each machine; the entries of the wave and of its pointer, which every reader publishes, are not owners and are excluded); the machines inferred 161, 153 and 156 of the 470 owners (the rendezvous start keeps them apart, nobody waited for another from the first second); the footprint of a machine peaked at 1305 MB (8 owners of 128 MB plus the driver) against 6553 MB; 0 processes and no directory were left; 316.5 owners per minute.

## Second scenario: a landing in the middle

A landing is modelled by what the real rule makes of it: at `--landing-after` seconds the kit declares a new record revision; every machine restarts its reader and pool under it (as the sidecars do within seconds of a code change), keys are made with the revision, so the stored results of the old revision are unreadable, readers announce a new wave, and the pools decline waves of another revision (`ownerPoolAcceptsRevisionV1`, reason `no-current-wave`). `--landing-keeps-revision` is the rule of R109 (a landing that keeps the schema stamp moves no revision). No file of the checkout is touched (touching one would be a real landing for the eight workers). Lost = stored results of the old revision that the readers cannot use (all of them when the revision moves); pool idle = seconds with owners open and no owner running, summed over the machines, sampled every 5 s.

Revision moves (the rule before R109):

- verdict: PASS
- every machine ready: true (rehearsal-m2 111.9 s, rehearsal-m1 117 s, rehearsal-m3 122 s)
- owners inferred per machine: {"rehearsal-m1":174,"rehearsal-m2":143,"rehearsal-m3":153}, fetched: {"rehearsal-m1":296,"rehearsal-m2":327,"rehearsal-m3":317}, inferred in total: 470
- owners stored more than once: 0
- peak footprint (limit 6553 MB): rehearsal-m1 1310 MB within, rehearsal-m2 1306 MB within, rehearsal-m3 1308 MB within
- owners per minute: 227.4, time to the end: 124 s
- orphan processes after the end: 0, work directory removed: true
- landing at 30.8 s, revision moved: true: stored results lost 218 of 218, pool idle with owners open 55 s (sum over machines)

Around the landing (machine m1, owners known = here + fetched):

| t (s) | machine | owners known (here+fetched) | wanted | pool | reason | running |
| --- | --- | --- | --- | --- | --- | --- |
| 25.7 | rehearsal-m1 | 56 | 414 | pulling | - | 8 |
| 25.7 | rehearsal-m2 | 56 | 414 | pulling | - | 8 |
| 25.7 | rehearsal-m3 | 56 | 414 | pulling | - | 8 |
| 30.8 | rehearsal-m1 | 72 | 398 | pulling | - | 8 |
| 30.8 | rehearsal-m2 | 72 | 398 | pulling | - | 8 |
| 30.8 | rehearsal-m3 | 72 | 398 | pulling | - | 8 |
| 35.8 | rehearsal-m1 | 0 | 470 | pulling | - | 8 |
| 35.8 | rehearsal-m2 | 0 | 470 | pulling | - | 8 |
| 35.8 | rehearsal-m3 | 0 | 470 | idle | no-current-wave | 0 |
| 40.9 | rehearsal-m1 | 8 | 462 | pulling | - | 8 |
| 40.9 | rehearsal-m2 | 8 | 462 | pulling | - | 8 |
| 40.9 | rehearsal-m3 | 8 | 462 | pulling | - | 8 |
| 45.9 | rehearsal-m1 | 24 | 446 | pulling | - | 8 |
| 45.9 | rehearsal-m2 | 24 | 446 | pulling | - | 8 |
| 45.9 | rehearsal-m3 | 24 | 446 | pulling | - | 8 |
| 50.9 | rehearsal-m1 | 32 | 438 | pulling | - | 8 |
| 50.9 | rehearsal-m2 | 32 | 438 | pulling | - | 8 |
| 50.9 | rehearsal-m3 | 32 | 438 | pulling | - | 8 |
| 56 | rehearsal-m1 | 48 | 422 | pulling | - | 8 |
| 56 | rehearsal-m2 | 48 | 422 | pulling | - | 8 |
| 56 | rehearsal-m3 | 48 | 422 | pulling | - | 8 |

Revision kept (the rule of R109):

- verdict: PASS
- every machine ready: true (rehearsal-m1 81.5 s, rehearsal-m2 81.5 s, rehearsal-m3 81.5 s)
- owners inferred per machine: {"rehearsal-m1":141,"rehearsal-m2":168,"rehearsal-m3":161}, fetched: {"rehearsal-m1":329,"rehearsal-m2":302,"rehearsal-m3":309}, inferred in total: 470
- owners stored more than once: 0
- peak footprint (limit 6553 MB): rehearsal-m1 1306 MB within, rehearsal-m2 1306 MB within, rehearsal-m3 1309 MB within
- owners per minute: 337.7, time to the end: 83.5 s
- orphan processes after the end: 0, work directory removed: true
- landing at 30.6 s, revision moved: false: stored results lost 0 of 218, pool idle with owners open 40 s (sum over machines)

| t (s) | machine | owners known (here+fetched) | wanted | pool | reason | running |
| --- | --- | --- | --- | --- | --- | --- |
| 25.6 | rehearsal-m1 | 56 | 414 | pulling | - | 8 |
| 25.6 | rehearsal-m2 | 56 | 414 | pulling | - | 8 |
| 25.6 | rehearsal-m3 | 56 | 414 | pulling | - | 8 |
| 30.6 | rehearsal-m1 | 72 | 398 | pulling | - | 8 |
| 30.6 | rehearsal-m2 | 72 | 398 | pulling | - | 8 |
| 30.6 | rehearsal-m3 | 72 | 398 | pulling | - | 8 |
| 35.7 | rehearsal-m1 | 88 | 382 | pulling | - | 8 |
| 35.7 | rehearsal-m2 | 81 | 389 | pulling | - | 8 |
| 35.7 | rehearsal-m3 | 81 | 389 | pulling | - | 7 |
| 40.9 | rehearsal-m1 | 96 | 374 | pulling | - | 8 |
| 40.9 | rehearsal-m2 | 96 | 374 | pulling | - | 8 |
| 40.9 | rehearsal-m3 | 96 | 374 | pulling | - | 8 |
| 45.9 | rehearsal-m1 | 112 | 358 | pulling | - | 8 |
| 45.9 | rehearsal-m2 | 112 | 358 | pulling | - | 8 |
| 45.9 | rehearsal-m3 | 112 | 358 | pulling | - | 8 |
| 50.9 | rehearsal-m1 | 121 | 349 | pulling | - | 8 |
| 50.9 | rehearsal-m2 | 120 | 350 | pulling | - | 8 |
| 50.9 | rehearsal-m3 | 120 | 350 | pulling | - | 8 |
| 56 | rehearsal-m1 | 136 | 334 | pulling | - | 2 |
| 56 | rehearsal-m2 | 132 | 338 | pulling | - | 8 |
| 56 | rehearsal-m3 | 136 | 334 | pulling | - | 8 |

Reading: when the revision moves, 218 of the 218 stored results are lost and every owner is inferred again where it was not yet stored (the run needs 124 s instead of 89.1 s without a landing, 227.4 owners per minute instead of 316.5), the pools sit in `no-current-wave` until a reader of the new revision announces its wave (one sample here, because the readers announce at once after their restart; on the real fleet that time is the crawl of one reader, R104); pool idle with owners open: 55 s. When the revision is kept, 0 of 218 are lost, the run needs 83.5 s (337.7 owners per minute) and the pool idle time (40 s) is the tail of the run (a pool with nothing left to claim), which shows that the metric also counts a tail. The two rules are the difference R109 staged.

## 8 GB worker (3 machines, `--memory=8192 --cores=4`)

- verdict: PASS
- every machine ready: true (rehearsal-m2 142 s, rehearsal-m1 147 s, rehearsal-m3 147 s)
- owners inferred per machine: {"rehearsal-m1":155,"rehearsal-m2":159,"rehearsal-m3":156}, fetched: {"rehearsal-m1":315,"rehearsal-m2":311,"rehearsal-m3":314}, inferred in total: 470
- owners stored more than once: 0
- peak footprint (limit 3276 MB): rehearsal-m1 696 MB within, rehearsal-m2 696 MB within, rehearsal-m3 694 MB within
- owners per minute: 189.1, time to the end: 149.1 s
- orphan processes after the end: 0, work directory removed: true

The limit is 3276 MB, the pool slots follow the declared machine (4 cores), peak footprint 696 MB, the run takes 149.1 s for the same owners (16 GB: 89.1 s), duplicates 0, orphans 0.

## Not simulated

- the real reader: take pass cost, plan stage, closure entries, record freshness, the memory refusal and the plan cuts, the farm, heavy owners, `repo`, the resident service, the sidecar `ready` loop and its stages, the replica daemon and its health, code changes under a running read (exit 75), the fleet identity and sealing, the real store of the workers (`/Users/Shared/semio-store/v1` is never touched; the temporary store is empty), Windows and Linux workers, network faults, the load of this Mac (the run shares it with the real group server: load average 36 to 48 at the time).
- a landing that touches a file of the library (that would restart the sidecars of the eight real workers; the kit refuses to do it).

## Proof of the kit

`🧪️tests` (10 pass): the plan of three machines has distinct origins, directories and marker; the environment of a machine names its declared memory and has no cache credential; the declared machine reaches `hostProfileV1`; the limits of 16 GB and 8 GB come from the build library (6553 MB); options are closed; the process table keeps only the rows with the marker of the run; the verdict (clean run, duplicate, missing ready, orphan, footprint over the limit, landing); the audit names `.cache/b`; and a real rehearsal of two machines and 12 owners ends with both ready, no duplicate, no orphan and no directory left. The tests ran red before the library existed. The state-directory seam test (`seam/machine-state.test.ts`) is red against the live tree by design: it lists the 21 files that still spell `.cache/f` out.

## Found while building

- Two numbers are called "the plan limit": `planStageLimitBytesV1` (hostShare 0.3: 4915 MB of 16 GB, the managed bytes a plan may add) and the footprint limit of the slice guard (`treeShare` 0.4: 6553 MB, the figure in the refusals). The kit prints the second one as the limit and keeps the first one in `limits.managedMegabytes`.
- The model itself had the bug class a landing makes real: an owner in flight when the revision moves stores its result under the old revision, and a machine that counts it as known under the new one waits for ever for an entry nobody wrote. The driver now fixes the revision at claim time. The real publish after a revision move should be looked at for the same case.
