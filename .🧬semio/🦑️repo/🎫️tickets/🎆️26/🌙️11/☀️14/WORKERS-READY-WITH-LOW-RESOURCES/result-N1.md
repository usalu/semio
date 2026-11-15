# Package N1: toolchain provisioning chain (2026-11-14 11:40 to 14:10, live tree g1-server)

## Verdicts

- `repo tools setup all` standalone passes: **not verified.** First run (after roles landing): 193 s in `Checking declared package providers`, 6 586 edges at 48 edges/s, then my SIGINT (rc 1) to land the throttle; no run since, because the host stayed in memory/swap wait (see below).
- `repo run workspace:deps-build-tools` passes: **not verified** (never run: the graph image `.cache/b/g/m` was full, 31 MiB of 12 GiB, ENOSPC at the first `repo show project workspace`; N10 owns it).
- `repo run @semio-tech/repo-lib:cpp-setup` passes: **not verified** (depends on the two above). Reading: `program("c++")` realpaths `/usr/bin/c++` (78 links) on macOS; nothing in `tools setup` sets `CXX` there, so this probably still refuses after 1 and 2; unproven.
- `repo dependencies sync --all` second run: before my change 82 s and 186 s (both `current`, then a 1.18 M-operation preservation pass); with the seed 503 s under capacity waits; after landing 3 the run `sync4` waited for capacity more than 25 minutes: **not verified**.
- Unit/law tests all pass: operation 2, roles 2, relocated 1, seeded 2, bazel delegation 1, pressure 2.

## 1. `deps-build-tools`: a declared Repo operation (decision)

A script entry cannot work: twelve purpose checks in `📤️publication/🟦️.ts` require the Original's arguments to be exactly `tools setup all`, and the task envelope says `nx run workspace:deps-build-tools`. So the task must be born as the Original of that operation. Schema first: `metadata.semio.execution` of kind `orchestration` takes `operation: ["tools","setup","all"]` (definition `Operation` in `🏗️build/📥️caller/📤️orchestration/🧬️schema/🔣️.json`, also in the execution contract). `projectBuildOrchestrationCapabilitiesV1` then projects `arguments`, `selection` and `launches` = that request, so `readOwnedBuildInvocationV1` admits the nested `repo ./📜️script.ts repo tools setup all` and the setup-all plans see the exact arguments. One predicate `buildOrchestrationDelegatesV1` replaces the three copies of "has children" (nx plugin, semio backend, bazel). Root `📋️project.json` declares it; README EN+DE. Tests: `📥️caller/🧪️tests/📍️operation` (Ajv oracle, 8 refused forms, projection for nx and bazel), bazel delegation corpus +3 cases. Not tested live: `readOwnedBuildInvocationV1` with the projected envelope and the nx plugin issuing path (needs a graph run). The graph does not dirty on a content edit of the root project file (known, N4): `chmod 755`/`644` plus a graph command is required before a run sees the declaration.

## 2. Dependency partition roles and the 28 minutes

- Roles: `SelectedPlanProtocol.partitionRoles` (schema `PartitionRole`, writer EN+DE, retention `retained|self-managed|service`) declares `generation`, `pointer`, `python`, `dotnet`, `nuget`, `.metadata_never_index`; `maximumRoles` 16 -> 24 (declared total 17). `requireRepositoryDependencyImageRolesV1` checks names and physical kinds, is called before any provider is observed (fail-fast instead of after 28 minutes) and by the plan capture. Test `📤️publication/🧪️tests/📋️roles` with the real listing of this Mac (14 entries), the worker listing (17), a foreign name, a mistyped role, a missing `j` and 40 entries; independent Ajv validator agrees. Retirement: only `j` generations are retired; every other role is untouched. `dotnet`/`nuget`/`python` are declared as the worker report observed them (N3 owns their writers; if they rename, change `partitionRoles` only). The tools partition has the same disagreement (`go`, `native`, `repo-watch`, `.metadata_never_index` against `SelectedPlanProtocol`), but `planSelectedRepositoryToolingGenerationsV1` is not on the `tools setup all` path; not changed.
- The 28 minutes: CPU-bound, 48 edges/s against 1 360 edges/s in `pressure.test.ts` (same code, trivial context); the difference is `check()` (maintenance-port checks with `lstat`/`statfs`, 21 % of samples are `lstat`) charged at every one of about 100 charge calls per edge. Landing 2 checks custody once per `progressWork` (64) units. Also the walk is performed twice per current-bindings read (`verifyExecution`) and again per retirement plan; the replay primitive exists (`replayJavascriptDependencyExecutionV1`) but is not applied there yet. **Effect of the throttle: not measured** (no real run after landing).

## Added by the coordinator

(a) Second `dependencies sync --all`: the receipts were right (`Store dependency sync current` after seconds) but the caller then ran the preservation pass (1.18 M ownership operations, 18 459 lease lines), the exclusive bindings gate and the collection regardless. Fix (landing 2 and 3): the pass seeds the recorded ownership index; and `runJavascriptDependencyStoreGenerationsV1` returns right after a `current` build phase (the proof already covers generations, ledgers and no obsolete generation). Risk stated: the sync receipt is sealed in the publish phase before the preservation pass, so a crash between the two is no longer completed by the next no-op sync, only by the next non-current one. Test `💾️storage/🧪️tests/🌱️seeded` (2, source-order laws, not a timing test). The `stale installed:.` of the 299 s run was not reproduced (ctime bump suspected; a fixed-ctime stamp landed in 5ececaa).
(b) `Unowned artifact directory` after relocation: owners embed the absolute root. `artifactOwnerRelocatedV1` (`🏃️process/📦️artifacts/📤️publication/🟦️.ts`) accepts a marker whose owner equals the current owner below another absolute root (shared relative tail of at least 4 segments, both absolute); the directory is then republished and re-owned. Judged by the tail, not by `.cache/f/machine.json`, which holds no former roots. Test `📍️relocated` with the two real markers of this Mac. Not run against the real directory.

## Landings

`FLEET-ONE-MACHINE…/landings.md`, "Package N1 landing 1, 2, 3"; scripts `n1-scripts/`.

## Not verified / others must know

- Everything live. Host state at the end: `dependencies sync --all` waited in `door.capacity` for more than 25 minutes (projects jobs of other agents held 15 + 12 GB, swap above ceiling); the process may still be running as `sync4`.
- Pre-existing failures I did not cause: managers digest/oracle/dotnet/npm (4) and one neutrality hit in `💾️storage/🟦️.ts`, birth (8), rust intent (3).
- Path length: `requireMaterializedPathV1` refuses 256 characters; deep `node_modules` below `/Users/ueli/Documents/semio` is N2's.
