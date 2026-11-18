# R140: the readiness probe read must take seconds (living report)

Scripts: `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R140/` (`probe5.ts` in-process admission closure with per-owner rows, `profsum.py` cpu profile summary, `stage.py` + `patch-*.py` the staged patches, `window.sh`/`land.py`/`restore.py`/`battery.py` the landing, `new/` the new tests and module).
Lab rerun: see `result-R133.md`. In-process closure probe (no lab, no service): `bun probe5.ts` from the checkout root with `SEMIO_WORKSPACE_ROOT` set (the wrapper `prof.sh` adds `--cpu-prof`); it runs `closeBuildOwnerAdmissionV1` exactly as `loadBuildGraphAttemptV1.inventory()` does (capture + settle round) for `repo-mcp,norm-din4108,norm-din4108-rs` and writes `probe.json`.

## 1. What the selected-* activities iterate over (counts, measured 2026-11-17 18:00 to 19:10 UTC)

The `selected-*` and `command-source-imports` activities are the stages of ONE function, `discoverBuildOwnerAdmissionFactsV1` (owner-discovery), called by `closeBuildOwnerAdmissionV1` once per owner of the closure (`observe(owner,[],[])` for the base facts, then once more with the selected targets). The lab's "seconds per activity" are sampler counts of the last log line, so they are summed over all those calls.

- Registry: 4772 owners. **Closure of the three probes: 4407 owners (92 percent of the workspace), 97300 files, 444100 observation rows (116 MB as JSON), 4452 owner observations (4449 base observations with no target, 3 with the `build` target).**
- Why the closure is the whole workspace: `norm-din4108` and `norm-din4108-rs` lie below `✏️s/🔌️plugins`, a consumer root of the plugin catalog provider, so their observation adds the Cargo manifest of every catalog owner (3226 owners) to its files (`catalogNativeManifestCandidatesV1`), and every file of an observed owner admits the owner that holds it. The TypeScript probe alone touches 3279 owners (`repo-mcp` alone: 53, no catalog). A selection under the plugin tree is therefore never a small read; a selection of `repo-mcp` alone would be.
- The three targeted observations (CPU seconds on a loaded machine, load 40 to 56): `repo-mcp` 16.2 s (2747 files, 38 dependencies), `norm-din4108` 13.1 s (5869 files, 10 dependencies), `norm-din4108-rs` 6.6 s then refused with "Original bounded graph writer custody required" in the in-process probe (it needs the real graph writer; the probe falls back to its base facts). The 4449 base observations cost 60.7 s CPU (13.6 ms each, of which the rebuild of the owner lookups was a quarter).
- Command-source imports / compile imports / source inputs: per targeted owner, the import walk of its `build` command (the repo script closure, 2747 to 5869 files) and the walk of its input globs (`readBuildSourceInputMatchesV1`: readdir of the matched roots, shared across targets), repeated in the second inventory of the same read.

## 2. Profile (bun --cpu-prof of the in-process closure, 241 s wall at load 50; the first lab run spent 1215 s because the closure is derived at least twice per read, once in the initial and once in the final `inventory()`, and again after every worker restart)

| function | self or inclusive seconds | what it is |
|---|---|---|
| `sourceOwner` / `contains` in `closeBuildOwnerAdmissionV1` (admission `🟦️.ts:95`) | 105 inclusive, 109 self | finds the owner of every file by scanning all 4772 roots: 97300 files, one owner (`s-dev-composition`) has 81256 files, so 388 million `startsWith` |
| `discoverBuildOwnerAdmissionFactsV1` | 58 inclusive | per owner: the manifest parse, `cargoTargets`, the lookups `byPackage/byRoot/owners` rebuilt from the 4772 owners each time |
| `readdirSync`, `readFileSync`, `existsSync`, `lstatSync`, `statSync` | 15.6, 15.4, 7.6, 6.6, 6.5 self | the observed file system calls of the walks |
| `releaseRuntimeMemoryV1` (forced gc) | 9 | memory governor between owners |
| (an earlier probe without the settle round) `memoRowsHoldV1` | 117 of 122 | the validation of the shared Wasm index memo per import when no validation round is open: not a production cost, the production read has the round |

## 3. Fix (landing 1, 18:11 UTC, entry in landings.md)

Tests first, red then green (each red before: the export or the module did not exist; the timing law is red with the old scan, 5.6 CPU s against a limit of 4):
- `createSourceOwnerResolverV1` (admission): ancestor lookup in an index, same ties as the scan. Tests `🧭️source-owners`: edge case, 120 seeded worlds against the scan and an independent Python model, and a scale law (6000 owners, 60000 files of one owner: 5.6 s CPU with the scan, under 4 with the index).
- `registryOwnerIndexesV1`: the per-registry owner lookups once (used by owner discovery).
- `📥️load/🧠️admission-memo` (schema: the shape of the closed admission plus the observation schema of the authority receipt): the closed admission of a selected scope and its observation rows are remembered beside the selected publication and served while every observation holds and the loaded producer is the same; corrupt, foreign or oversized memos are misses. The inventory checks the registry scope first, asks the memo, derives only on a miss and remembers what it derived (source-order law in the memo test).

Measured with the probe (`probe5.ts`, same machine, load 47 to 56): wall 226 s before, 115 s after; CPU of the process after 82 s (user 51, system 31). The remaining CPU: 4449 base observations 43 s (9.7 ms each, mostly syscalls), the three targeted observations 33 s.

## 4. Typed refusal and worker error text (R136 open list)

- `graph.unknown-source-input` (EN/DE, `build.graph.unknown-source-input`, values `owner`, `input`) replaces the raw `Error` at both throw sites (admission and owner discovery). Test `❓️unknown-input` (red: not a RepoError before), `requested-group` and the environment test now check key and values.
- Wall 13: a slice worker's stderr is piped (and still copied to the service log), `workerCauseV1` picks the cause line, the pool puts it into the restart decision (`restarted-cause`, EN/DE text `host-slice-owners-restarted-cause`) and a worker that ends early with the same text 3 times (`sliceRestartSameCauseMaximum`) stops the plan with `host-slice-worker-failing` (EN/DE) instead of restarting for ever. Test `🗣️restart-cause` (5), including a real worker process.

## Numbers: start phase CPU seconds, before and after

Machine at load 28 to 56 (10 cores, 32 GB; the figures are CPU seconds of the process, not wall seconds; wall seconds are 2 to 5 times larger at this load). The idle-machine target (cold under 60 s) could not be measured: the machine was never idle.

| step | before | after |
|---|---|---|
| closure of the three probes, cold, in process (`probe5.ts`) | about 200 CPU s (97 s in owner observations plus 105 s in the owner scan, from one 241 s profile; wall 226 s) | **82 CPU s** (user 51, system 31; wall 115 s) |
| closure, warm (memo hit: read 134 MB, parse, validate 444157 rows, `probe7.ts`) | not memoized, the same as cold (and again for every restart and for the second inventory of one read) | **3.5 s wall, 1.5 s CPU** (write of the memo 0.6 s) |
| lab start phase, 16384 MB / 8 cores (cold) | never finished (1215 s) | 246 s (run 5, load 45), 327 s (run 6, load 28), 583 s (run 7, load 48); the closure is 285 to 538 s of it, the rest source admission 20 s and implementation source 14 s; the lab's `tools` stage (74 to 90 s of install) is a lab artefact outside this phase |
| lab owners phase (take pass), 4453 owners | not reached | 840 to 1130 s at load 28 to 48, farm of 4 to 5 children, busy share 20 percent |

The cold CPU is still above the target of 60 s: 43 s are the 4449 base observations (9.7 ms each, mostly system calls: manifest read, directory listing, existence probes per owner) and 33 s the three targeted observations. What remains to get under 60 s cold: per-owner memo by content that the fleet store can serve (one record per owner instead of one record per scope), or the base observations on the farm. Not done.

## Lab runs after landing 1 (16384 MB / 8 cores, probe selection)

Runs 5, 6 and 7 (timeouts 1500, 1700 and 3000 s). All three: start phase passed, take pass 4453 of 4455 owners done (`present 0 fetched 0 here 4453`, peak footprint 3276 to 3853 MB of 6553 MB, no loop-lag limit breach, nice 0 everywhere), then the walls below. Further phases (look queue, assembly, validation, sealed selected publication, the `build` targets of the three probes, `ready: true`) were NOT reached.

## Walls (numbered from 18)

18. **Settle round 0 of the owners phase ends with `build.graph.owner-too-heavy`: `@semio-tech/s-dev-composition (about 27952 MB)` and `@semio-tech/repo-lib (about 9652 MB)` against the limit of 8612 MB of a 16 GB host.** Evidence: run 7 service log (`[graph-settle] {"round":0,"failure":"unserved build.graph.owner-too-heavy ...`). 4453 of 4455 owners are done at that moment; only these two are not. `s-dev-composition` has 81256 files in its admission facts (it is the owner of the plugin catalog provider and carries the whole catalogue's inputs). Status: OPEN, large. A 16 GB host can never infer them (28 GB recorded); options: lean (streaming) inference of such owners, not admitting the provider owner for a consumer (its published source file is the only input a consumer needs), or a bigger fleet machine that serves the two records (the pool's heavy hand-off exists for that, but the lab has one machine).
19. **The hand-off round re-infers everything.** After wall 18 the pool starts the load again (hand-off, up to 3 times). Round 1 logs `owner records present: 4453 of 4455, 2 to infer or fetch` but then walks all owners again and ends with `4453 inferred here, 0 present` after about 15 minutes (3 to 4 owners per second at load 40, single thread). The 4453 records of round 0 are announced as present and not reused. Status: OPEN. This alone doubles or triples the wall of a doomed read; a doomed hand-off (recorded size above the host's memory) should refuse at once.
20. **The closure of a selection below the plugin tree is the whole workspace.** `norm-din4108` and `norm-din4108-rs` are catalog consumers; their facts list the Cargo manifest of 3226 catalog owners, each of which admits its owner (4407 of 4772 owners, 97300 files). So "three probes" is a full-workspace read in every respect except the targets; the probe selection itself is not a small slice. Status: documented; the catalog edge is intended hashing; not changed.
21. **Closure derivation is 285 to 538 s wall at load 28 to 48** (82 CPU s), twice per read before landing 1 (initial and final inventory), now once per scope and machine. Status: partly fixed (memo, resolver); cold CPU still 82 s (target 60 s).
22. **The memo and the sealed selected publication are stale whenever any observed file changes** (one stale row out of 444157 invalidated the memo of `probe7`: a peer had edited `owner-discovery/🟨️.mjs` in the meantime). On a replicated shared worktree with many editors a read that lasts 25 minutes is invalidated by a large share of edits. Status: documented; correct behaviour, but it makes the warm path depend on a quiet tree.
23. **Wall 13 (restart text) and the unknown input refusal: fixed in landing 1** (tests `🗣️restart-cause`, `❓️unknown-input`). The real cause of the exit 75 of a missing module is printed by the nested command (door README, "nested command prints its cause and yields exit 75"), the pool did not read it; now it does.
24. **The lab removes the service log with its work directory** (`/tmp/sl-<id>`): every diagnosis needs a copy loop. Status: OPEN, small (the lab should copy the host log into `--out`); also the lab's leftover disk images (wall 17) remain.

## Not exercised

`--memory=8192 --cores=4` (the 16 GB run does not pass the owners phase: walls 18 and 19), `--machines=2` (second machine fetching instead of recomputing: needs a first machine that publishes; the take pass published nothing before wall 18), look queue, assembly, validation and seal of the selected publication, the real `build` targets of the three probes (the Rust probe needs a cargo build; `norm-din4108-rs` also failed the in-process closure probe with "Original bounded graph writer custody required", which the service provides), the real `tools` stage (the lab skips it), an idle-machine measurement, the live use of the admission memo inside the service (its source-order law passes; the timing lines `admission-memo` and `admission-memo-write` appear in the service log: miss at 77 ms, write included in the 285 to 538 s of the first inventory; a memo HIT inside the service was not observed because the final inventory of a read is only reached after the owners phase).
