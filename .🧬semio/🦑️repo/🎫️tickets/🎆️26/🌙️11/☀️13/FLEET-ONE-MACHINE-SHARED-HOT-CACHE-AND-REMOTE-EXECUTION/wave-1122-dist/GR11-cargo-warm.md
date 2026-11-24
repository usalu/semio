# GR11: warm `repo cargo` before cargo starts (2026-11-23)

Command: `repo cargo --store=shared --configuration=prototype -- check -p semio-framework-async-timer`, `SEMIO_REPO_PROFILE=1 SEMIO_CARGO_TIMING=1`, unthrottled, host load 29-51 (not controllable). LIB = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`.

## 1. Profile marks (new `[phase]` rows in the existing table)
`cargo.body.prepare|scheduler|lock-check`, `cargo.prepare.select|witness|child`, `cargo.inputs.lock-check|admit|fetch`, `cargo.registry.fetched-receipt|sources-intact`, `cargo.config.rustc-version|get`, `cargo.plan.graph|capture|compiler|recall|remember|cfg|foreign-inventory|source-admission`. Files: launcher, dependencies, ready/storage, configuration, workspaces/cargo (see landings.md). `cargo.inputs.fetch` never appeared on a warm run: `cargo fetch` is already skipped by the fetched/ready receipts (E35). `cargo config get` is 20-70 ms, `rustc -vV` 35-200 ms, plan recall 1.5-6 s: none is the 19.5 s.

## 2. Where the 19.5 s really was (before, same code, marks only; two runs)
| phase (s) | before run A (97.8 s total) | before run B (158 s total) |
|---|---|---|
| boot to workspace-body start | 5.6 | 6.9 |
| prepare.select | (inside prepare 43.9) | **67.9** |
| prepare.witness | - | 0.07 |
| prepare.child (synchronize ran, inputs changed by a peer or my edit) | - | 48.2 |
| scheduler prep | 0.8 | 0.5 |
| inputs: lock-check + config + admit | 0.8 + 0.1 | 0.9 + 0.1 |
| registry sources-intact (65k stat + 709 archives read and inflated) | 5.9 gap | 7-13 |
| plan.graph (recall 1.5-6) | 21.4 | 5.4 |
| source admission | 0.07 | 0.08 |
| cargo start (workspace-body start to child begin) | ~74 | ~140 |

Root causes:
1. `prepare.select` 44-68 s: `cargoWorkspacePreparationInvocationV1` binds `-p semio-framework-async-timer`. The crate belongs to the workspace `🧰️framework` (its manifest says `workspace = "../../../../.."`), not to the root workspace the caller's `Cargo.toml` names, so `cargoDeclaredPackageSelectionsV1` returns undefined and `cargoRepositoryPackageSelections` walks the WHOLE repository synchronously (measured alone: 34.7 s). Not a cargo cost and not memoizable by content without the same walk.
2. `sources-intact` 6-13 s: `fetchedSourcesV1` reads every locked `.crate` (709) and inflates it on every run to compare file digests, although those digests are a pure function of the archive checksum (measured sums over the 16 parallel lanes: archive read 130 s, inflate 5 s, extracted-dir digests 14 s, archive digest 5.5 s).

## 3. Changes (pure functions of unchanged inputs, each with a recompute law test)
1. `cargoDependencyPackageSelectionsV1` (LIB/🗂️workspaces/🦀️cargo/🟦️.ts): a name that is a path dependency of the root `[workspace.dependencies]` is answered from that row: its package name is read from the manifest, its owner comes from the package's own workspace ancestry, the owner must be a discovered repository workspace (root `owner-manifests` pattern, no dot/opaque/testing-collection directory, `workspace.metadata.semio.repository` present) and the manifest must be admitted by that owner's member rules exactly as the whole-owner walk admits it (`memberPlan`). Anything else returns undefined and the whole inventory runs as before. `cargoPackageSelectionsNearV1` order: declared, dependency, inventory. `declaredPlan` also tries literal `[workspace.dependencies]` path rows of the near workspace (Cargo treats them as members). Measured: select 34.7 s alone, 54-68 s in the command, now 10-47 ms.
   Tests (`🧪️tests/🪧️declared-selection`, 8/0): corpus extended (schema `dependencies`/`uses`, Cargo `metadata` oracle still equal); new tests: dependency answers equal the inventory and list no directory; glob, excluded (oracle), undeclared, missing names fall back (undefined, answers equal inventory); recompute after the owner loses `semio.repository`, the package is renamed, the owner directory vanishes, and after a path dependency package rename/removal.
2. Archive listing memo (LIB/🏃️process/🦀️cargo/📥️dependencies/🧾️fetched + 🟢️ready/💾️storage): `fetchedListingV1` = sha256 of the sorted `path NUL file-digest LF` rows of an archive, remembered under the archive checksum in the persisted digest memo (fixed stamp, content-addressed, cannot go stale). `fetchedSourcesV1` skips the archive read and inflate when the extracted directory holds exactly those rows; any changed, removed or extra file mismatches and takes the old path (extras are still tolerated there, repair unchanged). Test (17/0, +1): TS listing equals an independent Python `hashlib` computation, second pass reads 0 archives, tampering, deletion, a wrong remembered value and an extra file recompute and repair.
3. Not memoized, with reason: `cargo fetch` (already receipt-skipped), `cargo config get` (20-70 ms, key would need every config file and CARGO_* variable for no gain), plan recall (existing manifest-digest memo, 1.5-6 s of re-hashing about 900 manifests, left), source admission (0.05-0.1 s, memo exists).

## 4. After (r6-2, second run after the change, load 51; total 44.3 s)
| phase | s |
|---|---|
| boot to workspace-body start (store, rustup, capture 0.3-1.1) | 14.6 (load 51; 4.5 at load 33) |
| prepare.select | 0.05 |
| prepare.witness (receipt held, no child) | 2.8 |
| scheduler prep | 1.2 |
| inputs: lock-check 0.7, config 0.05, fetched-receipt 0.2 | 1.0 |
| registry sources-intact | 6.5 (was 6-13; 65k lstat of extracted files remain, no archive read) |
| plan.graph | 3.5 (recall 2.8) |
| source admission + lock-check | 0.3 |
| workspace-body start to cargo start | 17.2 (was ~74) |
| cargo child | 8.6 (a real check, not 1.7) |
Whole warm command: 97.8 s and 158 s before to 44.3 s after (not the same host load). First run after landing (preparation receipt invalid because the preparation program source changed, empty listing memo): 68.9 s. Run with the proof workspace: 29.7 s.

Target not met on this host: cargo start 17 s after workspace-body (< 5 s needed) and 44 s whole (< 20 s). Remaining fixed costs: boot/store 4.5-14 s (load dependent), 65k lstat in sources-intact 6.5 s (a stat-only trust that cannot be shortened without trusting directory mtimes, which would hide in-place edits; closure-restricted verification would, at the price of repair of crates outside `-p`), witness 2.8 s, plan recall 2.8 s, scheduler 1.2 s.

## 5. Cold run
No cold store was produced (clearing the shared stores is forbidden while the fleet uses them). Closest: the first run after landing, 68.9 s (prepare child ran 26.7 s because the preparation program digest changed, listing memo cold). GR4's cold number 129 s (workspace-body 101.5 s) predates these changes; the 34.7 s repository walk it contained is gone.

## 6. Proof workspace
`wave-1122-dist/proof/Cargo.toml` gained `cargo-features = ["trim-paths"]` (the repo `prototype` profile needs it). `cd proof && repo cargo --store=shared --configuration=prototype -- build -p p-leaf --offline` built (Compiling p-leaf, Finished 4.16 s), 29.7 s wall, and the profile has no graph phase (no `graph.*`, no `door` graph read): the command needs no graph state. select was 10 ms there.

## 7. Tests (repo test-file, one at a time)
declared-selection 8/0; fetched 17/0; workspaces/cargo selected-closure 1/0 (132 s); not from this change: workspaces/cargo main 19/1 (the live-repo test "current physical workspace admissions ... nextest" took 51 s against a 30 s limit under load 45; it compares the root `members` array with the physical walk), dependencies 11/1 (`wasmCompilerSelectionArgumentsV1 is not defined`, the peer failure E35 recorded), launcher 10/2 (isolated-store settlement test, E35 recorded it). Configuration and ready members have no test file of their own touched beyond marks (exercised by the dependencies and launcher files above).

## 8. Open
- Closure-restricted `sources-intact` (needs the `-p` selection in the registry check and a repair policy for crates outside it).
- Stat memo for plan recall text answers (about 2 s).
- The root `members` array lacks `semio-framework-async-timer` (a path dependency of workspace `🧰️framework`), which is why the live admissions test and the declared lookup miss it.
