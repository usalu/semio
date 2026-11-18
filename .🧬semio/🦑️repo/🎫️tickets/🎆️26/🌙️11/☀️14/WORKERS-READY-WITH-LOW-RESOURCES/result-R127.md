# R127: the walls after the owners phase of the project-graph read

Scripts: `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R127/`: `postowners.ts` (driver), `stage.mjs`/`stage2.mjs` (staged copies in scratch, no tree copy), `patches.mjs`, `land.mjs`, `restore.mjs`, `battery.sh`, `window.sh`.

## 1. The sequence from "all owners done" to `ready: true` (read from the code)

Limits that apply everywhere: stage `workspace` = `repo run workspace:setup`, timeout 4 h (`READY_STAGES_V1`); the stage is stopped when its tree shows no new normalised output line for 10 min AND no CPU (`stallStopMilliseconds` 600000, CPU liveness 500 ms), so a silent CPU-bound step is not killed, a silent idle one is. Plan limit (`planStageLimitBytesV1`): managed bytes a plan may add = 0.3 x host memory (4915 MB on 16 GB, 2457 MB on 8 GB, minimum 2048), stage `environment` 1024 MB; tree share 0.4 (6553 MB), footprint soft 60 / hard 85 percent.

A. Tail of the part (`discoverBuildGraphPartV1`, `🧭️discovery/🟦️.ts`), still inside the plan guard, in the PARENT, nothing of it is farmed: stages `environment` (`deriveEnvironmentV1`: Cargo manifests, taxonomy, lock), `tests` (feature files, `discoverTestOwnersV1`), `policies` (`createNodesV2` over every `📜️script.ts`), `dangling`, `files` (file-to-owner index over all member files), `test-edges`, `edges` (`discoverBuildDependenciesV1` over all owners: module graph, source analyses), `metadata`, `result` (`neutralBuildTargetV1` of every target). Then `rows: [...captured.observations]`: the row log (lazy, on disk) is spread into ONE in-memory array of every observation row. No progress line is printed by these stages except the plan-stage lines (`SEMIO_GRAPH_PHASES`/limit machine).
B. `shapeComputeResultV1`: sorts all rows by path, claims rows per owner, remainder to the environment; holds nodes + rows of all owners.
C. `host.release` (collect), then `buildOnce` in the store (`🕰️generations/🟨️.mjs`): `fragments` (one brotli document per owner, 32 in parallel), owner-table consistency loops, derived upserts, `patch` (merge into the fact index), orphan/support checks, `pack` (brotli of the whole index, limit 512 MiB expanded), `keep` (write), global fragment, frontier, manifest (limit 128 MiB), `publish` (pointer swap under the `facts` lease, wait 30 s, merge into the peer head when the swap is lost), `markIndexed`, stamps, `retire` (reads every manifest) when alone.
D. `build` loop: a delta build runs in waves (48, then grown by the elapsed time up to 960): every wave repeats C and the `published` hook (`announceGraphWaveV1`, `settleWaveMemoryV1`). A cold build is one wave.
E. After the last wave: `engine.dropIndexes`, `sweepSupersededBlocksV1`, `publishRemoteGenerationV1` (only if the machine may upload), `assembleStream` (fragments, 32 at a time), `expandStreamV1` (`host.expand` per chunk of at most 32 projects / 16 MiB; each chunk opens a graph-store consumer lease and reads the closure leaves), `finish`, `readBuildGraphV1` (JSON-schema validation of the WHOLE graph), `host.sealed` (pin record).
F. `repo run workspace:setup` then plans and runs Nx tasks: the root target `setup` depends on `deps-build-tools` (= `repo tools setup all`), `deps-js-all`, `deps-python`, `deps-cargo`, `deps-go`, `deps-dotnet`, `deps-cpp`, `deps-browsers`, `deps-wasm`, `deps-tools`, `setup-git`, `prepare` (5 generators), `repo-mcp:install`, and a cargo build of `@semio-tech/framework-os-mcp-rs`. All are `side-effecting`, uncached, so they run on every worker. None of this ever ran on a worker.
G. `ready`: when the stage exits 0 the sidecar writes the stage digest into `state.json` and the marker `native-bootstrap.done`; the next turn `readyDecideV1` answers `idle` iff every stage digest equals the recorded one, the marker exists and the `repo` binary exists; then `ready.json` gets `"ready": true` (validated against its schema, atomic replace). It re-verifies only the five stage digests (hashes of `bun.lock`, `package.json` fields, the tool table, the bootstrap schema, `global.json`); `workspace` has `inputs: []`, so a landing never makes it stale again, a changed `bun.lock` re-runs only `dependencies`. Nothing in it depends on server-only state; it needs the local `.cache/f/ready/` files and a writable `<store>/v1/origins/<origin>/`. Proven by the existing sidecar test "a machine that is not ready runs every stage in order ... ready.json validates" (not re-run for real here).

## 2. What I ran for real (this Mac, load 14 to 40, bun 1.4.2)

`postowners.ts` drives the REAL `loadGenerationGraphV1`, store, shape, fragment/index/pack/publish/retire/assemble/expand code and the real `readBuildGraphV1` over the repository's own mock host, with a synthetic part of 4774 owners (22 KB node, 72 percent target inputs, 8 edges, 150 rows each, which matches the 700 to 760 thousand facts of 5000 projects in the logs of the real server generations) and a mock expansion of 1500 files per target. Isolated store in scratch, no live store touched. Peak = resident set sampled every 50 ms, killed at the cap.

| run | peak RSS | time | result |
| --- | --- | --- | --- |
| cold, 4774 owners, 150 rows, full path incl. expand and real validation | 4084 MB | 16 s (idle) / 36 s (loaded) | ok under 6553 MB cap |
| same at 300 rows per owner | 5347 to 5766 MB | 34 to 41 s | ok under 6553 |
| same at 1000 rows per owner | more than 12 GB (killed) | - | FAILS |
| half cap 3276 MB (8 GB worker) | killed at 4357 MB, 11 s, inside the shaping | - | FAILS |
| delta of 473 owners on a 4774 generation (the worker case) | 4723 MB | 91 s | 10 generations (waves) published for 473 owners, verification of 716100 facts 87 s |
| scaling 1194 / 2387 / 4774 owners | 1659 / 3241 / 5681 MB | 7.5 / 20 / 34 s | memory linear, `patch` 2.2 / 5.9 / 15 s (power 1.4) |

Per phase peak (150 rows): compute 1.3 GB (the spread rows alone cost 0.5 GB), fragments 2.3 GB, patch 2.9 GB, keep 3.1 GB, assembly + expand + validation 4.1 GB. Releasing nodes after the fragments or forcing collections between phases did NOT lower the peak (staged variants `release`, `gc`: 4162 / 4467 MB against 4176 MB): the resident set is a high-water mark and the assembled graph (about 1 GB) is added on top.

## 3. What would fail

1. The whole step is bounded by rows per owner, not owners: 300 rows reach 5.3 to 5.8 GB, 1000 rows pass any limit. Everything is materialised in the parent (all rows spread into an array, `ordered` path sort, claimed key set, all nodes, then the whole graph). On an 8 GB worker (plan limit 2457 MB managed, 3276 MB tree) the step cannot pass at 4774 owners even at the real 150 rows.
2. Nothing between "owners N of N" and the end prints progress: tail stages A, fragments, patch, pack and publish are silent (here 10 to 40 s, on a loaded worker minutes). The sidecar does not kill it while it computes, but every human and monitor reads it as the same wall.
3. A kill (landing restart, stall stop, memory stop) after the owners phase loses everything: a cold build publishes once at the end, a delta build publishes per wave but redoes plan and take for the rest; owner records persist, the assembled part does not.
4. A delta build publishes a full generation per wave (10 for 473 owners): each wave patches and packs the whole 58 MB index.
5. Stage F (30 side-effecting tasks, a cargo build) and stage A (tests/policies/edges for all owners in the parent) are unmeasured walls: see 5.

## 4. Landed

2026-11-17 12:03:40 to 12:06:40 UTC (first attempt 12:00 aborted, restored): progress for the silent steps. A build of more than 48 owners says `storing`, `indexing`, `sealing` (EN and DE) and counts the fragment step (`fragments done/total`). New test `⏳️sealing-progress` (2, red before, green after); subset battery of 15 generation tests not worse than baseline; both standing preflights green. No memory fix landed: every cheap variant measured showed no gain (section 2); the real fixes (streaming rows, waves for a cold build) are design changes listed in section 5.

## 5. Not exercised

Stage A tail with real inference (tests, policies, dependency edges, metadata), real `holds` cost, the real `expand` with closure leaves under the graph-store lease, `restoreRemoteGenerationV1`/upload, `sweepSupersededBlocksV1`, the Nx task phase F and a real `ready` write on a worker. The row count per owner is the biggest uncertainty (150 from the server logs, 300 and 1000 as sensitivity). Not changed (design decisions for the coordinator): stream the rows (keep the row-log list, no spread, path index by ancestor lookup), publish a cold build in waves so a retry resumes, one generation per read instead of per wave.
