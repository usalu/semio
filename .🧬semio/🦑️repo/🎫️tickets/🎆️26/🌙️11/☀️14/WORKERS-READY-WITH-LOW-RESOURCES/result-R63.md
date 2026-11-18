# R63: the `workspace` stage graph load measured on a worker-like run, and one memory plan for the whole process tree

T = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/`, LIB = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`, B = `LIB/🏗️build/`. Scripts: `T/🧰️scripts/R63/` (`land.py`, `edits.py`, `treeplan.test.txt` = copy of the new law test). Scratch (samples, logs): `.../scratchpad/R63/` (`ps-*.txt` 2 s process-table samples, `summ.py`, `comp.py`, `tree.py`, `run.sh`, `sample.sh`).

## 1. How it was reproduced (no setup run, no `repo tools setup all`)

`repo run workspace:setup --dry-run` makes exactly the slice request a worker makes (`loadBuildGraphSliceV1` -> `loadGenerationGraphV1` with `slice.roots`, R48 chain) and executes nothing. A private graph service per run: `TMPDIR=/private/tmp/r63/<label>/tmp` (the socket, the launchd label, the service log and the `semio-footprint` traces all derive from it), `SEMIO_HOST_CAPACITY_DIRECTORY` private (own capacity gate, no queue behind the host), and the declared machine `SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES` / `SEMIO_HOST_CAPACITY_CORES` (16384/4 and 8192/2). The graph store (`.cache/b`) is shared with the live server: a private store cannot be had without a second checkout, so "cold" here means "every closure stale" (all 2918 closures of the slice are stale after every landing in the build module, which is an input of all of them), not an empty store. The server's own read (`read-record.json`, runner 57701) was not touched; its builder custody, however, is the reason the inference phase waits (section 3).

The slice of today is not 78 projects: **703 stale owners (2918 input closures to verify), 86 to 128 of them "too heavy for this machine"** (workspace itself about 19.9 GB, s-dev-composition 42 GB, framework-rs 17.6 GB by the recorded costs). The 78 of the earlier rounds were the stale owners before the constant landings in the build module made every owner stale again.

## 2. What the guard measures

`observeHostTreeMegabytesV1(workerPid)` (`LIB/…/sampling`): the **footprint** (macOS `footprint` phys_footprint; Linux `smaps_rollup` PSS plus swap; Windows private working set) of the **slice worker's subtree only** (the worker and its farm children; NOT the service and NOT its eval/plan child), every second. Limits (`HOST_POLICY_V1`): plan limit `min(8192 MB, 40 % of the host)` = 6553 MB on 16 GB, 3276 MB on 8 GB; ceiling 60 % of the host (9830 / 4915 MB); over the limit it settles once, then refuses only if available memory < 2048 MB, pressure is critical, or the tree is over the ceiling. A refusal is cut-and-continue (R48): restart at the next cut level (projects at once 8/2^k, wave 10000/2^k basis points, farm cap `SEMIO_GRAPH_CUT_CONCURRENCY`).
`peakMegabytes` in a worker's `ready.json` is a different number: the sum of RSS over the whole stage tree (R61 finding 6).
On this host (swap 3 GB used, compressed memory) footprint and RSS differ by a factor of 5 to 20 (worker at 6.9 GB RSS held a phys_footprint of 367 MB, "reclaimable" 6352 MB, i.e. freed pages the allocator keeps resident), so the guard never tripped in my runs; the guard readings of 9.9 to 10.3 GB on g1-m07/m08/m10 are real footprint on machines that are not compressing. The ratio is machine state, not code: the sizing must not depend on it.

## 3. Measured tree (old code, declared 16 GB / 4 cores, `f16`; load 26 to 44; 2 s samples of `ps`)

| component | peak (RSS) | phase / when | data structure |
|---|---|---|---|
| graph service | 365 to 375 MB | whole run | resident protocol, indexes dropped |
| eval/plan child of the service | 701 to 734 MB | whole run | warm module closure of the plan |
| slice worker (the parent of the load) | 2.3 GB at +31 s, 2.4 GB at the start of verification, 3.1 GB at the end of verification (+480 s), 3.5 GB at prefetch, 3.8 to 4.2 GB at inference start | fact load, verification, inference | see the phase table below |
| walk farm children | 4 children, 150 to 716 MB each, 1.1 to 1.7 GB together; second farm of 4 at +535 s | settle, +126 s to +170 s | closure rows, part metas of manifest v4, TypeScript programs of the walks |
| project farm workers | 3 workers (3 x 1536 MB granted): 120 MB idle for 6 minutes, 780 to 860 MB each (2.4 GB together) at +1127 s and growing | inference start | TypeScript programs of the owners |
| **tree (all processes)** | **5.5 GB at +170 s (slice worker 2.75 + 4 walk children 1.7 + service + eval); 7.4 GB at +1127 s (worker 3.9 + 3 project workers 2.4 + eval 0.7 + service 0.4), still growing** | | |

The ~10 GB of the workers is the same composition: slice worker 3.9 + project farm 3 x ~2.1 GB (measured on m07/m08/m10: 9 915 to 10 302 MB around a 3.9 GB parent) + walk farm. Phase table of the slice worker itself (`semio-footprint/<pid>.log`, declared 8 GB / 2 cores, `f8`; MB):

| phase | resident | managed heap | external | footprint | objects |
|---|---|---|---|---|---|
| start | 327 | 46 | 29 | 104 | 0.3 M |
| membership | 1040 | 313 | 278 | 243 | 1.5 M |
| directories | 1464 | 544 | 379 | 156 | 3.0 M |
| facts (31 413 rows) | 2208 | 842 | 596 | 242 | 4.0 M |
| verification (2918 closures) | 2990 | 1287 | 869 | 377 | 6.3 M |
| flush / prefetch | 3003 | 928 | 584 | 349 | 5.0 M |
| inference (in process, 571 light projects) | **6749** | 1713 | 1249 | 1609 | 7.9 M |

Reading: resident minus managed minus external is 0.8 GB at the facts, 0.8 GB at verification and **3.8 GB during inference** (memory the heap freed and the process keeps: the allocator never returns it), so the parent alone is 3 GB before any child exists and 6.7 GB after a few hundred projects were inferred in it. The settle itself took 430 s for 2918 closures (ask 345 s, support 66 s, seal 3.6 s, save 0.6 s; slow=2918 vouched=0); the first 48 derivations ran at one closure per 8 s under load 38, then the walk farm took over.

What the fleet wait looks like (both sizes): `703 stale projects, 0 taken from the fleet, 86/128 too heavy`, `inferring 617 light projects here while the fleet works`, then `0 of 703 projects arrived from the fleet ... the server last progressed N s ago, estimated time left about 33 min`: the loads sit in this state because the server's own read (builder custody) has not finished once today.

## 4. Root causes found (by the numbers above)

1. **Two farms, no common plan.** The project farm sized itself with `planFarmWorkersV1` (tree share minus the worker's resident set at that moment, 1536 MB a worker) and the walk farm (R53/R62) with the same function: each took the whole share, and neither knew the other (7 children at once in `f16`). Together with the shared recipe farm there were three independent sizers.
2. **The share was 50 % of the host while the guard's plan limit is 40 %** (and ceiling 60 %): a farm that fit its share was planned to overshoot the point where the guard settles.
3. **The parent was counted at the size it had when the farm was sized (3133 MB), not at the size it reaches (3.9 GB)**, and a project worker was costed at its grant (1536 MB) although three of them took 6.3 GB (2.1 GB each).
4. Cut-and-continue reduced only the wave size and the count cap (8/4/2/1); a cap of 4 at level 1 does not reduce a farm of 3, so levels 1 and 2 changed nothing on a 16 GB machine (that is why the peak was ~10 GB "whatever the cut level").

## 5. The fix (one landing)

Landed under `landing.lock` by `T/🧰️scripts/R63/land.py` (entry "Package R63 landing" in `T/landings.md`; four earlier attempts of the same script applied the files, hit a red and restored the base files at once: two red laws of my own new test and the missing `SEMIO_TEST_ARTIFACT_DIR` of the project-farm tests; the live files were therefore changed and restored four times within about 25 minutes, which restarted every slice worker once more with exit 75, see 7).

- `B/🧠️memory/🪜️plan-stage/🟨️.mjs`: **one ledger for the process tree**. `reserveFarmV1({kind,workerMegabytes,wanted,yield})` takes workers from `treeShare` of the host after the parent (`max(resident, parentMegabytes)`) and after the reservations of the same or a higher priority; a farm that does not fit makes the lower priority farms give their children back (`yield`) before it takes less; `release()` is idempotent; nothing is recorded where the plan has no limit (a serving machine). `planFarmWorkersV1` got `kind`/`counted`; `farmReservationsV1()` reports the ledger. Schema (`🧬️schema/🔣️.json`): `treeShare` 0.5 -> **0.4 = the plan limit share of the slice guard** (a law ties it to `HOST_POLICY_V1.sliceHostElasticity.hostCapBasisPoints`), `farmPriority` (project-farm 2, walk-farm 1), `parentMegabytes` 3600, `farmWorkerPeakMegabytes` project-farm 2200. Types in the `.d.mts`.
- Walk farm (`B/🕸️graph/🧮️derived-nodes/🧩️closure-kinds/🏭️walk-farm/🟨️.mjs`) and the settler (`closure-kinds/🟨️.mjs`): both the settler farm and the shared recipe farm reserve through `reserveWalkFarmV1`, hand the lease to `createWalkFarmV1`, which releases it at `close()`; the yield closes the farm (the settle then walks in process).
- Project farm (`project-nodes/🏊️project-farm/🟨️.mjs`): reserves at hub creation, below two workers it stays serial (`say("serial", memory)`), releases at `retireHubV1`; `project-nodes/🟨️.mjs` plans with `kind:"project-farm"`.
- Cut-and-continue reaches every component now because every farm draws from `planFarmWorkersV1`, which still honours `SEMIO_GRAPH_CUT_CONCURRENCY`.

Laws (new `B/🧠️memory/🪜️plan-stage/🧪️tests/🧮️tree-plan`, 7 tests, green; red before by construction: the old tree has no `reserveFarmV1`/`farmReservationsV1`, and the control assertions compute the old independent sizing): share equals the guard's plan limit; at 16 GB with a 3133 MB parent the old sizing plans 2 + 4 children (9.3 GB nominal, over the 6553 MB share) and the ledger grants 1 project worker and no walk child (parent + children 5.8 GB); 8 GB: no farm gets a child at any parent size; 12 GB: one walk child; priority yield (walk farm is asked to give back, never the other way); cut caps every farm; a seeded random law over 400 reserve/release steps on 8/12/16/20 GB hosts (parent plus all children of the same or higher priority stay within the share at every grant); **real-process law**: a declared small machine, real `bun` children holding their budget (random-filled ballast so the compressor cannot hide it), the summed tree measured with the guard's own `observeHostTreeMegabytesV1` stays under the plan, the farms sized alone are the larger tree (the nominal budget of the old sizing is over the plan). Updated: plan-stage schema law (two new keys), cut law (3 workers at 16 GB, was 4), walk-farm law (16 GB: floor((0.4 x 16384 - 3600)/768); 8 GB: 0 children).
Targeted tests run by the landing, all green: plan-stage, tree-plan, cut, walk-farm, `closure-kinds/⏱️throughput`, project-farm, `resident/✂️cut-load`, `resident/🛡️plan-guard`, and the two standing preflight tests.

## 6. After the fix (new code, declared 16 GB / 4 cores, `g16`, same shared store)

The farms no longer overshoot: at inference start the plan said `resident 3304 MB`, the ledger left room for 1 project worker (< 2), so `inferring 628 projects in this process (memory)`; no child at all besides the walk children of the settle (3, 1.6 GB, peak tree 5.8 GB at +138 s during the settle, before the project phase). Slice worker 3.84 GB (footprint 1.45 GB) at inference start, tree 4.4 GB (RSS) at +750 s with 3 projects in flight in process, 814 s in all until I stopped it (SIGINT, still in the fleet wait). **Peak tree RSS 5 815 MB (during the settle) against 7 426 MB and growing before, and below the 6 553 MB plan limit**; the old code's inference phase added 3 project workers on top of the 3.9 GB parent, the new code none. Not the same store state as `f16` (the verification took 403 s instead of 480 s because part of the closures was saved), so the figures are comparable in composition, not to the megabyte.

## 7. Proven / not proven

Proven: the tree composition and per-phase table above (measured, old code); the guard's definition; the independent sizing overshoots on paper and with real processes, the ledger does not (laws); the farms obey the cut; one landing with all targeted laws and the standing preflight green.
NOT proven:
- That the real slice **finishes** under a declared 8 GB or 16 GB machine. Neither load got past the fleet wait / inference of the 700 stale owners in 15 to 25 minutes, because the server's builder custody (its read has not finished once today) is shared with my private service; I stopped the runs. The 8 GB run showed what is still missing: **the parent alone reaches 3.0 GB at verification and 6.7 GB RSS (footprint 1.6 GB) after a few hundred projects were inferred in it**, above the 3276 MB plan limit of an 8 GB machine; a farm cannot help there. What an 8 GB machine needs is not more farm logic but a smaller parent: stream the 31 413 fact rows and the verification support instead of holding them (the facts index is 0.8 GB managed + 0.6 external at the end of the fact load, the verification adds 0.45 + 0.27 GB, inference leaves 3.8 GB of freed-but-resident heap), and run the inference of the stale owners in short-lived children whose exit returns the memory (the project farm worker is exactly that, but is off below 2 workers). Not done in this landing.
- The streaming consumer side of R59's packed closure records (not touched), the settle walking only the slice's closures (it already does: 2918 of 7392).
- Windows and Linux (the ledger and the farms are platform-neutral JavaScript; only macOS was run).
- The per-worker 2200 MB and the parent 3600 MB are measured on one run each (f16 and the logged refusals of m07/m08/m10), not learned per machine.

## 8. Other finding: landings in the build module restart every worker

`g1-m03` workspace try 3 failed with `the slice worker ended 4 times in a row with exit code 75 because the code on this machine kept changing under it`; every landing (mine included) makes the workers restart their slice worker and makes every owner stale again (78 -> 703 stale owners). Until the build module stops changing, no worker can finish `workspace`: land nothing in `B/` (production) until a worker has passed.

## 9. Readiness

Watched until about 21:50Z (about 20 minutes after the landing): **no machine had `ready: true`**. g1-m01, m02, m03, m07, m08, m10 in `workspace 5/5`; g1-m02 is the furthest: `Reading the project graph slice (1952 s): owners 24 of 703`, i.e. real local inference at about one owner per 80 s while it works through `shared closures 1/49`; g1-m08 restarted its slice worker with the new code (`verification 0 of 3622`); g1-m04 `repo-mcp`, g1-m05 `dependencies` (try 6, exit 1), g1-m06 `native` (try 5, exit 1), g1-m09 never attested. Nothing established readiness, so no ready message was sent. At 703 stale owners and the rate of m02 the stage needs hours even if memory is no longer the wall: the stale count (every landing in the build module restarts it) is the next wall, see 8.
