# R200: the cost of inferring ONE Rust owner (living report)

Started 2026-11-18 about 10:00 UTC. Machine load average 35 to 58 on 10 cores during every measurement below (the refresh read, the lab and the replica run beside), so all seconds are "under the fleet's usual load", not idle-machine seconds. Scratch: `scratchpad/R200/`. Kept scripts: `FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R200/`.

## 0. Summary (read this first)

1. The premise is wrong in its main part: **the inference of a Rust owner runs no cargo, no rustc, no rustup, no build script and no unit-graph/build-plan at all.** It reads `Cargo.toml` files (TOML parse), tokenizes the crate's `.rs` files for `mod`/`include!` (`cargoSourceSetV1`: 77 ms and 47 files for `stdio-semio-rs`), and walks the TypeScript import closure of the build tooling (the cargo command script). The `cargo:artifact-plan` line of the logs is a build-time line, not an inference step. No cargo file lock can be waited on by an inference.
2. **Where the time goes** (`stdio-semio-rs`, real farm-child code path, in process, `--cpu-prof`): 54 to 75 percent is ONE computation repeated: the 18 declared tool groups and provider groups of the build tooling (each = a walk and an import analysis of up to 1800 TypeScript modules, `closurePackagesV1`). One crate with 17 targets derived them **690 times** (`toolClosureGroupV1`, 599 `closurePackagesV1` calls, 138 of them over the same 1800 files) because `expandedCommandInputsV1` produced every shared reference again at every expansion without any memo. The workspace-wide fact is "the tool groups of the providers project"; it is per script-input cache (per process), and a farm child drops its cache every few owners (`releaseOwnerStateV1`, after EVERY owner while the host is short of memory, which is the normal state of this machine).
3. **Fix (LANDED 2026-11-18 11:32:37 to 11:35:07 UTC, see sections 5 and 9)**: derive each tool/provider group once per cache and replay its observed rows into every project that expands it (`sharedGroupV1` in `🧭️script-inputs/🟨️.mjs`, one new function, three edited lines, result rows byte-identical). Measured on the real child code path, warm process: `stdio-semio-rs` 45 to 60 s / 31 to 34 s CPU before, **6.3 s / 5.8 s CPU after**; derivations 690 to 36; `closurePackagesV1` calls 599 to 32.
4. The fix does **not** reach the 512 MB law (first owner of a process grows 850 to 980 MB; late-closure mode 1.4 to 4.5 GB) and it does not remove the dominant cost of the *late-closure* mode that the running read uses (section 4): closure-store saves/loads, brotli, row hashing, serial heavy-question children. That part is measured, not fixed.
5. Result rows: byte-identical record packs before/after (same sha256 for both owners). **No semantics revision bump** (section 6).

## 1. Step table, one owner (cold = first owner of a fresh process, warm = later owner of the same process)

Harness: `🧰️scripts/R200/prof2.ts` opens the real farm-child session (`createProjectInferenceEnvironmentV1` + `service.open` + `ownerJobV1` + `service.run`, the code a farm child runs for a produce job) in one bun process; `LATE=1` sets `lateClosures` and binds the real closure store (read-only). Counters: bun plugin `inject2.ts`/`fix2.ts` count calls without editing any file.

| step (owner `stdio-semio-rs`, eager closures, cold process, load ~45) | seconds | calls |
|---|---|---|
| session open, environment, `nativeCommandInputs` (symbolic) | 0.01 | 1 |
| `ownerJobV1` = `projectAssemblyAuthorityV1` (`nativeDependencyRoots`: Cargo.toml of the path-dependency closure, 33k `stat`) | 5.1 to 9.4 | 1 |
| `cargoSourceSetV1` (tokenize 47 `.rs` files) | 0.08 | 1 |
| `closurePackagesV1` over the tool groups (cpu profile: 46 s inclusive of 85 s; counted: 64.5 s of 87 s wall with call overhead) | 46 to 64 | **599** (138 over the same 1800 modules) |
| `toolClosureGroupV1` | (inside the row above) | **690** (18 groups x 38) |
| record write, observation rows, `stringify`/`hash` | 6 to 8 | 1 |
| cargo, rustc, rustup, `cargo metadata/tree/--unit-graph/--build-plan`, build scripts | **0** | **0** |
| total `produce` | 63 to 87 wall, 42 to 47 CPU | 1 |

CPU profile self time of the same run (85.6 s): `readFileSync` 23.3 s, `statSync` 12.2 s, `existsSync` 9.1 s (syscalls 52 percent, all inside the repeated walk: 19.3 s read + 6.3 s stat + 6.3 s exists come from `closurePackagesV1 > observedSourceAnalysisV1`), `hash` 4.0 s, `stringify` 3.6 s, TypeScript `createSourceFile` 2.7 s, `parse` 1.5 s. The machine is at 41 to 54 percent system time (R194 section 10), which is why a syscall-heavy repeat costs 5 to 10 times the idle price.

Cold vs warm, other owners (eager mode, same harness, three runs each):

| owner | position in process | before: wall / CPU / growth | after fix: wall / CPU / growth |
|---|---|---|---|
| `@semio-tech/norm-din4108-rs` | 1st (cold) | 17.0 to 27.6 s / 16.3 to 18.3 s / 902 to 977 MB | 19.8 to 23.2 s / 14.3 to 17.3 s / 849 to 916 MB |
| `@semio-tech/stdio-semio-rs` | 2nd (warm process) | 45.0 to 60.0 s / 30.8 to 33.6 s / 177 to 197 MB | **6.3 s / 5.8 s / 230 MB** |
| `@semio-tech/stdio-semio-rs` | 1st (cold) | 63 to 87 s / 42 to 47 s / 936 MB | (not run alone; the 2nd-owner figure is the amortized one) |
| `@semio-tech/norm-din4108-rs` again | 3rd, same owner | 0.4 s (owner memo) | n/a |

Late-closure mode (what the running read uses: `lateClosures` true, real closure store, heavy-question children), same harness:

| owner | position | wall / main-process CPU / growth |
|---|---|---|
| `norm-din4108-rs` | 1st | 40.2 s / 27.5 s / 1.74 GB |
| `stdio-semio-rs` | 1st alone | 105 to 264 s / 85 to 100 s / 2.3 to 4.5 GB |
| `stdio-semio-rs` | 2nd | **843 s** / 220 s / +1.4 GB (RSS 2.9 GB) |

So the "25 to 45 minutes" of the log lines are one owner in late mode on a loaded machine plus queueing: for `stdio-semio-rs` the footprint line says `wall=4025814ms queue=286791ms farm=908174ms observe=2704200ms` (11-18 10:01 UTC) against `wall=125363ms farm=12783ms` on 11-17 23:49 for the same owner: 70 times slower for the same work, with 1796 s of the observe time waiting for a farm child. Fleet distribution of all 1693 Rust owner lines in the footprint logs (`semio-footprint/*.log`): `farm` median 29.6 s, p90 233 s, max 3643 s; `wall` median 189 s, p90 1010 s; `queue` median 38 s, p90 508 s; `authority` median 2 ms (so the assembly authority is not the cost in a long-lived child, only in a cold one).

Late-mode profile of one owner (main process, 112 s; heavy children not covered): closure-store `save` 13.6 s (`listDigest` 8.8 s hashing the same row lists again, brotli 10 s), `layerBatchesSync` 11.3 s (read + decompress layers), `packBundleV1` 8.3 s, forced garbage collection 7.5 s (`releaseRuntimeMemoryV1`), `closurePackagesV1` 9.9 s, `observedSourceAnalysisV1` 13.1 s. The 843 s wall with under 1 percent main CPU while the heavy children ran shows the serial lane (`inLane("questions")` in `🪆️heavy-questions/🟨️.mjs`: ONE heavy child at a time per process, one fresh bun per question). Not proven which of the two dominates; not profiled inside the children.

## 2. Answers to the questions of the task

- Does it run `cargo metadata` / `cargo tree` / unit graph / `rustc --print` / build script / `rustup which`? No. `grep` of `child_process|spawn|exec` in the inference tree finds only the pool processes, the reaper, the take poll (`ps`) and the heavy-question child (a bun, not cargo); `ps` during every run showed bun processes only.
- Does it parse the 3200-member workspace manifest and lock per owner? Not the lock. It parses the `Cargo.toml` of the crate's own path-dependency closure (`nativeDependencyRoots`, memoized per process by `readToml` content memo) and of its workspace root (the root `Cargo.toml`, 141 KB, one read per process). `cargo metadata --locked` appears only as a *text pattern* (`nativeLockInputs`: targets whose command contains `native cargo metadata` get the inputs `{workspaceRoot}/**/Cargo.toml` and `**/Cargo.lock`); that is an input glob of the task hash, not a cargo run.
- Does it wait on a cargo lock? It cannot: nothing in the inference opens the cargo package cache or the build directory. The 2.5 GB children of the refresh read are TypeScript-closure and record memory.
- Where do the 2.5 GB go? Growth of a first owner is 850 to 980 MB in eager mode (TypeScript compiler programs, import walks of 1800 modules, observation rows); in late mode 1.4 to 4.5 GB (closure entries, layers, pack bundles). Neither is cargo.

## 3. Why "once per workspace" is the right shape and where it already exists

The only workspace-wide fact the Rust owners share is the tool-group set of the providers project (18 groups plus the provider ports) and its package list. Across processes it already has a durable home (the content-addressed closure store `ce/`, R105/R120 pattern) used by the late mode; inside a process it had none for the eager expansion and a racy one for `toolGroupFilesV1` (17 targets asked while the first derivation ran, `groups.get(name)` was still empty, each started its own walk). The fix gives the in-process memo (section 5). A fleet-wide "one resolve per lock digest" is not needed for cargo because no cargo resolve exists to share; the per-crate Cargo facts (TOML, path dependencies) cost 5 to 9 s once per cold process and are memoized afterwards.

## 4. What is NOT fixed (the late mode that the read uses)

The fix lands in code the late mode also calls (`toolGroupFilesV1`, the race), but late mode asks the groups through closure recipes, not through `expandedCommandInputsV1`. Measured effect on the late path (same harness, `LATE=1`, plugin `fix2.ts`, one run each, load 45 to 58): `norm-din4108-rs` 1st 46.7 s / 35.0 s CPU / 1.77 GB (before 40.2 s / 27.5 s / 1.74 GB: no change, noise), `stdio-semio-rs` 2nd **66.6 s / 45.0 s CPU / no growth** (before 843 s / 220 s / +1.4 GB). `closurePackagesV1` calls 23 (10 over the big closure) instead of 599 (138), tool-group derivations 22 instead of 690. One measurement each: the 12-fold gap is larger than the load noise seen between repeated runs (1.3-fold), but it is one pair. Remaining late-mode levers, none built: named by the profile, none built:
1. `listDigest` of the same 1800-row layers on every save (8.8 s of 112): key the digest memo by content of the first/last rows plus length, or keep the digest with the list that came from the store.
2. The heavy-question lane is serial per process: a pool of 2 to 3 children for the independent tool recipes (they share nothing but the store).
3. `releaseRuntimeMemoryV1` forces a garbage collection (7.5 s per owner at the 4 GB heaps of a loaded child): collect on growth, not per owner.
4. A child that is allowed to keep its cache (the group lists above) between owners: `releaseOwnerStateV1(shrink)` clears them after every owner while the machine is short of memory, which re-derives the 18 groups per owner. With the fix this costs 6 s instead of 50 s, without it the memory pressure itself multiplies the cost.

## 5. The fix (landed)

File: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs` (live sha256 `6f901feb840c...` at 10:55 UTC). Staged copy `scratchpad/R200/s/si.new.mjs` (made by `🧰️scripts/R200/mk.py` from the live file, anchors asserted unique), diff 35 lines:
- `toolGroupFilesV1` stores the pending promise, so concurrent targets share one derivation (a failure leaves no entry).
- New `sharedGroupV1(recipe, ...)`: for `tool-closure` and `script-provider` recipes `memoizedObservedAsyncComputationV1` over `scripts.groupLists`, keyed by `closureRecipeKeyV1`, an in-flight table `scripts.groupFlights` (later askers wait, then replay the stored rows into their own project), counter `scripts.groupComputations`; every other recipe is produced as before. `expandedCommandInputsV1` calls it.
- `scripts.groupLists` is cleared where `commandDependencies` is cleared (`releaseOwnerStateV1`, both sites).
No new module, no schema change, no migration.

Laws (new test `🧪️tests/🗂️shared-groups/🟦️.ts`, kept as `🧰️scripts/R200/law.ts`; staged run `staged-law.test.ts` with absolute imports through the plugin `stage.ts` that serves the staged file):
- call-count law: expanding the references of the native command inputs 6 times derives at most 2 x references groups and a 7th expansion derives none; 8 concurrent expansions derive each group once. RED on the live code (2 of 2 fail: `groupComputations` does not exist; the equality assertions before it already pass), GREEN staged (2 pass, 20 expects).
- result equality: all six answers equal each other and equal the first (cold) expansion of an independent cache, which is the oracle.
- Independent counters (plugin, no file edited): derivations 690 to 36, `closurePackagesV1` 599 to 32.
- Record equality on the two named owners plus the catalog crate: the content-addressed leaves are identical (724 files for `norm-din4108-rs`, 324 for `stdio-semio-rs`; only the pid/time-named `.idx/.pack` file names differ between any two runs, base against base too) and the record packs are byte-identical: sha256 `4f41c43f0d94fe85` (2 757 309 bytes) for `norm-din4108-rs` and `bf4c051a0a14c90c` (1 449 747 bytes) for `stdio-semio-rs` in the base run, the repeated base run and the fixed run.
- Memory law (512 MB per Rust owner): NOT met and not claimed. Measured growth is in the table of section 1.
- Time law: the 6.3 s warm figure is under load 45; the R135 speed factor of this machine multiplies the reference-machine bound, so the bound for the law is `warm owner <= 8 s x speed factor`; not written as a test (a wall-clock law on a shared machine is a flake source; the call-count law is the stable form).

## 6. Record semantics (R171/R181 rule)

The rows another process accepts do not change: same record packs, same content-addressed leaves. No bump of the declared semantics revision; existing fleet records stay valid. The code changes only how often a pure derivation runs inside a process.

## 7. Projection of a whole read (~3200 Rust owners), from the measurements above

`time = owners x seconds per owner / width`. Width = farm children the governor grants (R194 section 4: up to 9 asked, 6 granted at 32 GB with 26.6 GB booked by the read itself; 16 GB machine: 3 children, 4 GB floor).

| per-owner seconds (load 45) | 32 GB, width 6 | 16 GB, width 3 |
|---|---|---|
| 30 s (fleet median now, `farm` column) | 4.4 h | 8.9 h |
| 60 s (warm stdio before fix, eager) | 8.9 h | 17.8 h |
| 6 s (warm stdio after fix, eager) | 53 min | 1.8 h |
| 840 s (late mode, heavy owner alone, measured once) | not a median; 4 children x 2.5 GB held for it | |

These are projections from single-owner measurements under load, not a measured read. They exclude the other 1500 non-Rust owners, the fleet help (the pools publish about 50 owners per minute), and the late-mode costs of section 4. They say: the repeat removed by the fix is worth about a factor 8 per warm owner in eager mode; for the late mode one pair of runs shows 843 s to 67 s for the warm heavy owner (12-fold); the median late-mode owner was not measured.

## 8. Not verified

- The effect of the fix in late mode beyond one pair of runs (section 4), and in a real farm child (the harness is the same code in one process, not a spawned child).
- Heavy-child profile (the children of `inLane("questions")` were not profiled).
- Running reads/services keep the old code until they restart; the effect on the refresh read that was started at 08:12 is therefore none.
- Windows/Linux: the harness ran on this Mac only; the change is pure JavaScript.

## 9. Landing status

Landed in the 11:30 window: lock 11:32:37 to 11:35:07 (hold 150 s), entry in `landings.md`. Script `🧰️scripts/R200/land.sh` (+ `land-tests.txt`; base sha `6f901feb840c` compared with the live file after the lock and before the first write; restore tested exact on a scratch mirror with a forced red and the base-differs path). A first try at 11:30:00 restored itself on a 120 s timeout of the walk-views test: that test passes in 3 s alone, live and staged, and the machine was running this agent's own two test batteries beside it; they were stopped, the second try was green (15 files, import smoke ok), live sha equals the staged file (`a99fb1707ce6`), law live 2/2.
Test files green under the lock: script-inputs walk-views, router, provider-closures (3), module-graph, reach-memo, tests main, layer-lost, derive-claim, take-release; project-assembly; providers and requested-group; shared-groups. Pre-existing reds, identical live and staged (so not caused): `💾️entries/🧪️tests` 1, `producer-path` 1, `🫧️cargo-yield` 1, `🚶️repository-walks` 2, `🪆️heavy-questions` 0 to 2 (load); `🎯️target-fanout` and `🔌️providers` showed 1 to 2 load flakes staged and passed on rerun.

## 10. What to do next (ranked by measured share)
1. Restart the refresh read and the pool roles on the landed code, then read `owner @... farm=` of the Rust owners: expected warm Rust owner in the farm child 6 to 15 s instead of 30 to 60 s (eager figure) and, for the heavy late-mode owners, minutes instead of 14 min (one pair).
2. Late-mode levers of section 4 (digest memo of layer lists, 2 to 3 parallel heavy children, no forced collection per owner), each needs its own law; the profile numbers are in section 1.
3. Memory: 850 MB to 4.5 GB growth per owner is TypeScript programs plus closure rows, not cargo; the 512 MB law needs the walk to stream (no module programs kept) and is a separate piece of work.
