# repo graph reading: diagnosis (partial, not fully fixed)

Status: causes identified with evidence, one code defect fixed with a regression test, `repo show projects` and both `repo run` targets were NOT completed (see "Open").

## Measured chain (macOS arm64, 16 GB, load 4 to 11 from other agents)

1. Dependency gate (first 5 min of every graph command).
   - Every graph command first runs the Store dependency sync gate (`verifyJavascriptDependencySyncCurrentV1`, `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️incremental/💾️storage/🟦️.ts`).
   - Log evidence: `Store dependency generation receipt stale ... installer input changed ... w:package.json` and `Store dependency sync stale ... obsolete-generations`.
   - A stale gate re-runs the whole producer: 1.1 M `store-ownership` operations (state/list/readText/frontier) over about 142 k directories and 173 k files (`✏️s/🔌️plugins` alone has 104 k directories) plus 7 745 "declared package provider" edge checks.
   - It took 326 s and 3.5 GB max RSS with `repo dependencies sync --all` (exit 0, kept 18 pinned/recent generations). A killed or cancelled run publishes nothing, so the next command repeats it.
   - Contributors (b) and (d) are confirmed: the root `package.json`/`bun.lock` were rewritten by other agents at 20:43, 21:39 and 21:48 (assimpjs, gltf-validator, @gltf-transform, robust-predicates). Each rewrite invalidates the sync again. My second sync failed with `Bun current manifest changed during publication: package.json`.
   - (c) not observable now. (e) `📜️script.ts` diff only adds a test source to the `graph-memory` list: not a cause.
   - Walk rules are fine: `.🧬semio`, `.cache`, `node_modules`, `🗑️generated` etc. are already excluded (`🧭️roster`, `🗂️workspaces/🟦️bun/🟨️.cjs`). The 152 staffage JSONs are not read by this phase.
2. Cold graph generation.
   - `graph: no usable generation (empty), building the first one`; 4 754 projects. The graph cache stores (`.cache/b/g`, `d`, `r`) were created at 19:30 to 19:43 today, so there is no generation to refresh.
   - Inference runs in the parent process (the worker farm never engages: plan was 11 workers, but fewer than 24 misses occur because `np/` owner records, 1 204 of them, are cache hits).
   - Throughput is about 0.5 projects/s, 3 to 9 GB parent RSS (sawtooth 2 to 8.7 GB) with 8 owners in flight. At that rate a cold build is about 2.5 h, so it never publishes before being killed. This is the 4-5 GB, 50 minute symptom.
3. Admission poison (fixed).
   - `createMemoryGovernorV1` learned each owner kind's peak from the machine-wide headroom delta, split over running owners, and persisted the maximum forever (`np/owner-peaks.json`).
   - On a shared host other agents' allocations were charged to the owner: learned peaks were project 1 648 MB, rust 2 571 to 2 920 MB, cargo 1 945 MB for projects that show `0 s, 0 rows` (declared guess: 256 MB).
   - Result: `graph: headroom ... is below one project (2531 MB), inferring one project at a time` and "waiting for memory headroom" for most of the run.

## Change

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🟨️.mjs` and `🟨️.d.mts`: new optional `attributable` port on `createMemoryGovernorV1`. The headroom delta charged to owners is capped by the growth of this process's own managed memory (heapUsed + external). `createGraphParentMemoryGovernorV1` enables it by default only when the headroom source is not a cgroup (macOS, native Windows). Linux/cgroup behaviour is unchanged.
- Regression test appended to `🧠️memory/🧪️tests/🟦️.ts`: machine-wide headroom with another process growing 1000 MB per owner. Without the port the learned peak is at least 1000 MB; with it exactly 50 MB. 19/19 tests of that file pass (run with `bun test`).
- Deleted the poisoned `.cache/b/g/m/inference/selections/*/np/owner-peaks.json` (cache data). After the fix a run relearned project 960 MB, rust 488 MB, cargo 24 MB (still own-process sawtooth) and ran 7 to 8 owners at once, but throughput did not improve: the cost is per-project CPU in the single parent thread, not admission.
- All temporary `[DEBUG]` edits in `📥️inference/📦️project-nodes/` were removed (git status clean there). Helper `repo-graph-sample.sh` (RSS vs phase sampler) is in this ticket folder; logs are in `🗑️generated/repo-graph/`.

## Before / after

- `repo show projects`: before, killed after 420 s (about 2.8 GB) or 50 min / 5 GB; after the sync repair it reached inference within 20 s (no gate), then ran 100 to 230 projects per 3.5 min at 3 to 9 GB and was cancelled by me. Never completed. Peak RSS observed 9.1 GB (parent only).
- `repo run @semio-tech/framework-3d-js:test`: not run (graph read does not finish).
- staffage test target: not run.

## Open

- Cold graph build cost: about 4 s main-thread CPU per project (native sample shows 100 % busy JS main thread, no JS-level profile obtained; Bun `--cpu-prof` through `BUN_OPTIONS` untried). Needs a JS profile of `inferProject` in `📥️inference/📦️project-nodes/🟨️.mjs` and either farm engagement on a cold (empty) generation (`coldInference` is read but never set anywhere) or incremental generation publication so a killed run keeps its progress.
- Dependency gate: `verifyJavascriptDependencySyncCurrentV1` passes an empty keep-list to `selectObsoleteJavascriptDependencyGenerationsV1` while the producer keeps pinned/recent/rollback generations; check that kept generations cannot make the gate stale forever. And the gate re-runs the 1.1 M-operation ownership walk whenever any agent touches a manifest; it should be bounded by the persisted ownership index (listing stamps) and single-flight across concurrent invocations.
- Learned owner peaks are a monotonic maximum persisted forever; one noisy sample still ratchets them up. Consider a decay or percentile.
- Root `package.json`/`bun.lock` are edited by several agents; the sync cannot publish while that continues. Coordinate one `repo dependencies lock --all` then `sync --all` when edits stop.
