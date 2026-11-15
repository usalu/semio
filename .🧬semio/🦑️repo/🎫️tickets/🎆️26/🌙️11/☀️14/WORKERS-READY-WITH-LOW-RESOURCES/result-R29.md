# Package R29: the blast radius of shared file lists in project nodes (2026-11-15, live tree, load 11 to 28). PREPARED ONLY: nothing landed, no lock taken, no graph command, no `repo tools setup`.

T = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION`, LIB = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, SI = `LIB/🏗️build/🕸️graph/📥️inference/🧭️script-inputs`. Scripts, harness, payload and the new law test: `T/🧰️scripts/R29/` (`harness/all.ts` is the R5c `laws.ts` generalised to all recipes and many worlds; `run.sh`, `runall.sh`, `runworlds.sh`, `suite.sh`, `runlocal.sh`; `tools_overlay.py` + `mkov.sh` build an overlay from a tool list; `payload.py` folds it into `edits.json`; `land.py`). Scratch (outside the worktree, 40 MB): `/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R29/`.

## Verdicts
1. **R5c covered the Rust owners and almost nothing else.** Measured on all 4 773 owners that have command recipes (41 855 recipes of the closure-entry store: 17 101 native, 3 558 generic, 21 201 target-sources; the newest generation has 5 100 owners, 327 have no command recipe), the live nodes still embedded **123 434 file names** in their command lists (mean 25.9, median 4, p90 131, max 2 058 per owner), **106 607 of them (86 percent) in 560 files that 21 or more owners embed**; 4 771 of 4 773 owners carried such a file.
2. **After R29 the nodes embed 15 992 file names (mean 3.4, median 0, p90 2): 4 123 of 4 773 owners hold no command file at all, only references. 604 slots (3.8 percent) are in files that 21 or more owners embed (three per-subtree resolution configs), 83 owners with large owner-specific walks hold 13 646 of the 15 992.** 26 new tool rows, named by reference (mean 4.2 references per owner, was 1.9, maximum 43).
3. **Never under-keyed, proven for every owner:** the expanded key set (files and external packages) of each of the 4 773 owners is **exactly equal before and after** (0 owners lose a file, 0 lose a package, 0 grow), and an added imported file re-keys the same tasks as before (4769 owners before, 4769 after: every task that executes the tool, correct).
4. **Laws on the full set (section 3):** adding one imported file to each of the 25 import-hub groups at once changes **0 nodes after (before: 580 nodes)** and moves 48 of 49 groups, all of the single providers project; one world per group before changes exactly as many nodes as embed the group (25 of 25 worlds equal the structural count; table in section 3). An unreferenced file in 11 directories changes 0 nodes, 0 task keys and 0 groups, before and after. A body edit changes 0 nodes (section 3).
5. **What cannot be referenced and why, with owner counts: section 4** (implementation data rows 5 101, digest helpers 4 774, root configuration 4 720, named input `cargoToolchain` 4 456, per-subtree resolution configs 320/298/52, artifact import closures 33 to 66).
6. **Landing costs one more full refresh** (policy rows change the implementation digest, stamp `script-inputs/5`); it is the last one this package needs. Exact command in section 6.

## 1. The measurement: blast radius by group, before and after
Method. Command lists: the real `nativeTargetCommandInputs`, `genericTargetCommandInputs` and `targetScriptSources(…,"queries")` of the live tree (post R5c) for every recipe, in a frozen read snapshot (`harness/all.ts`; 8 shards, 0 errors apart from one recipe that errors identically before and after). Non-command groups of a node (`production`, `default`, `nativeSources`, `nativeTestSources`, `cargoToolchain`, `sharedSources*`, `artifactSources`, `goToolchain`): the 5 100 fragments of generation 237 (the newest on this machine, written before the R5c landing; R5c does not touch them), closure leaves from `.cache/b/g/m/build-input-closures` (3 leaves are not on this machine: 95 `nativeSources`, 87 `nativeTestSources`, 58 `sharedSources`, 27 `production` and 26 `default` groups were skipped, so the non-command counts are lower bounds). A **group** is the closure of one import root (roots of the import graph over the files that 21 or more owners embed); its **blast radius** is the number of owners whose node embeds the root: adding an imported file anywhere below the root re-stales that many nodes. Section 3 confirms the numbers by counterfactual worlds. Candidate tools were found by an algorithm, not by hand: the direct imports of the command entry scripts that many owners share, then, iterated against the real inference until no command-embedded file remained that 21 or more owners share, the import roots of the remainder (three rounds with 18, 11 and 1 tools; 5 of the first round turned out dead or redundant and were removed again: `build-capacity`, `cargo-script-commands`, `native-orchestration`, `cargo-package-commands`, `owner-command-module`).

Top groups by blast radius (the 26 tools are all above 20 owners; the other rows are the groups that stay):

| # | group | owners embedding its files before | owners embedding after | fate |
|---|---|---|---|---|
| 1 | implementation data rows (build and caching policy, graph and execution schemas, providers designation and schema: 7 files) | 5101 | 5101 | not referenceable (below, a) |
| 2 | digest implementation helpers (closure of `📥️inputs/🧮️digest/🟨️.mjs` and `📥️inputs/🟨️.mjs`, 18 files, plus 5 schemas/policies) | 4774 | 4774 | not referenceable (b) |
| 3 | `native-toolchain` (tool) | 4728 | 5 | referenced: 4456 nodes name it by reference |
| 4 | root configuration `tsconfig.json`, `bunfig.toml` (javascript toolchain files in `sharedSources`) | 4720 | 4720 | not referenceable (c) |
| 5 | named input `cargoToolchain` (`rustfmt.toml` and the cargo execution script) | 4456 | 4456 | not referenceable (d) |
| 6 | `package-router` (tool) | 534 | 0 | referenced: 645 nodes name it by reference |
| 7 | `orphans-parent-script` (tool) | 523 | 0 | referenced: 637 nodes name it by reference |
| 8 | `runtime-identity` (tool) | 523 | 0 | referenced: 637 nodes name it by reference |
| 9 | `free-space` (tool) | 520 | 0 | referenced: 597 nodes name it by reference |
| 10 | `owned-execution` (tool) | 520 | 0 | referenced: 597 nodes name it by reference |
| 11 | `runtime-sqlite` (tool) | 520 | 0 | referenced: 597 nodes name it by reference |
| 12 | resolution configs `✏️s/tsconfig.json`, `✏️s/package.json` (+ `🌎️hub/package.json` 52) | 320 | 320 | by design (e) |
| 13 | `package-execution` (tool) | 310 | 0 | referenced: 387 nodes name it by reference |
| 14 | `runtime-parse` (tool) | 269 | 0 | referenced: 405 nodes name it by reference |
| 15 | `testing-cargo` (tool) | 258 | 0 | referenced: 400 nodes name it by reference |
| 16 | `testing-budget` (tool) | 236 | 0 | referenced: 294 nodes name it by reference |
| 17 | `owner-command` (tool) | 199 | 0 | referenced: 199 nodes name it by reference |
| 18 | `runtime-main-module` (tool) | 182 | 0 | referenced: 211 nodes name it by reference |
| 19 | `runtime-main-package` (tool) | 159 | 0 | referenced: 178 nodes name it by reference |
| 20 | `typescript-build` (tool) | 96 | 0 | referenced: 107 nodes name it by reference |
| 21 | `testing-execution` (tool) | 91 | 0 | referenced: 246 nodes name it by reference |
| 22 | `package-budget` (tool) | 82 | 0 | referenced: 226 nodes name it by reference |
| 23 | artifact import closures: `value/decode` (66), `sqlite-snapshot` artifact chain (63, 12 files), `ieee754` (38), `artifact-reference` (33) | 66 | 66 | by design (f) |
| 24 | `module-tree` (tool) | 55 | 0 | referenced: 55 nodes name it by reference |
| 25 | `testing-vitest` (tool) | 52 | 0 | referenced: 95 nodes name it by reference |
| 26 | `rust-toolchain.toml` (42), `.cargo/config.toml` (41), `go.work`, `go.work.sum` (25) | 42 | 42 | not referenceable (c) |
| 27 | `test-adapter` (tool) | 31 | 0 | referenced: 31 nodes name it by reference |
| 28 | `inventory-index` (tool) | 30 | 0 | referenced: 30 nodes name it by reference |
| 29 | `normalization-digests` (tool) | 30 | 0 | referenced: 30 nodes name it by reference |
| 30 | `extension-package` (tool) | 26 | 0 | referenced: 26 nodes name it by reference |
| 31 | `test-source-text` (tool) | 25 | 0 | referenced: 25 nodes name it by reference |
| 32 | `typescript-package-index` (tool) | 25 | 0 | referenced: 25 nodes name it by reference |
| 33 | `validator-oracles` (tool) | 22 | 0 | referenced: 22 nodes name it by reference |

## 2. What was built (data, one line of code, one test)
- **25 import-hub tool rows** in `SI/🔌️provider-closures/🔣️.json` (view `scheduling`, `selection: included`, bilingual contracts) plus **`native-toolchain`** (26 new rows, 31 in the table) (view `native`, no entries, resources: the cargo execution script, `rustfmt.toml`, `bunfig.toml`, `tsconfig.json`). The mechanism of R5c does the rest: a command whose walk reaches a declared entry module names the tool and drops the files of its group from its own list (`toolFinishV1`), a command whose entry script is a declared entry names it without walking. The `nativeCommandInputs` line `...[javascript.files…, ...cargo.files].map(file => "{workspaceRoot}/" + file)` became `...toolReferencesV1(scripts, "native-toolchain")`; the new law test fails when the tool's resources differ from the build policy's toolchain files (single source).
- Schema (`🧬️schema/🔣️.json`): `tools.closures` `maxProperties` 16 to 48 (31 rows now), `entries` `minItems` 1 to 0 (a tool of resources only). Corpus rows for the 26 new rows and the fixture files their relative paths land on; the Python oracle ignores resources outside the fixture exactly like the inferrer; the main law test expects the new reference (2 anchored one-line edits).
- Inferrer stamp `semio.graph-inferrer.script-inputs/4` to `/5` (stamps schema, output-stamps fixture): `nativeCommandInputs` changes what a node holds.
- New law test `SI/🔌️provider-closures/🧪️tests/🧮️blast-radius/🟦️.ts` (3 tests; red before, 3 of 3; green after, 3 of 3): declarations closed and unique, `native-toolchain` equals the build policy, a native command names it and holds none of the files, a Rust package test script of the live tree keeps only its own files, names `package-router` and keys on at least the files of its walk. It takes the checkout from `SEMIO_TEST_CHECKOUT` when set (the overlay rig) and from its own location otherwise.
- Landing set: 8 edited files (10 anchored edits: policy, policy schema, corpus, oracle, main law test, `script-inputs/🟨️.mjs`, stamps schema, output-stamps fixture), 1 new file. The `.mjs` and test edits are anchored one-liners, so a peer edit of the same file (a memory-governor landing changed `script-inputs/🟨️.mjs` while I worked) does not conflict; the landing command uses `--merge`.

## 3. Laws on the full owner set (4 773 owners, 41 855 recipes; before = live code, after = overlay)
Worlds are virtual (`makeWorld` overlays the live tree; no file in the worktree is touched): `base`; `unreferenced` (11 new files that nothing imports, in `🏗️build`, `🏗️build/🧪️tests`, `🏗️build/📥️inputs`, `🏗️build/🕸️graph/📥️inference`, `⚡️caching`, `⚡️caching/🦀️cargo`, the library root, `🏃️process/🧭️routing`, the repo product root, `🔨️modules/🏃️process`, `🧪️testing`); `all30` (an existing leaf file that no key set contains, found in or above the directory of each entry, is imported by the entry module of each of the 25 import-hub groups at once; `native-toolchain` has no entry module, its membership is the policy datum tested by the law test); `add-<group>` (the same for one group, restricted to the owners that embed it before). **Nodes** = the command input lists of all recipes of an owner (sha of the JSON), **keys** = sha of the contents of the expanded key set, **groups** = the 23 (before) or 49 (after) shared groups of the providers project.

| law | before (live) | after (R29) |
|---|---|---|
| L1: 11 unreferenced files: nodes / task keys / groups changed | 0 / 0 / 0 | 0 / 0 / 0 |
| L2: one imported file added to each of the 25 import-hub groups at once: nodes changed | 580 of 4 773 | **0 of 4 773** |
| L2: task keys changed (every task that executes a changed tool, correct) | 4769 | 4769 |
| L2: groups changed | 23 of 23 | 48 of 49 (all groups of the one providers project; `native-toolchain` has no entries and stays) |
| never under-key: owners that lose a file or package from the key set / grow | | 0 / 0 (4 773 of 4 773 equal) |
| embedded file names, all owners | 123 434 | 15 992 |

One world per group, before (every row: the number of nodes the structural table predicts; after: 0 by the combined world). Roots = owners that embed the group before; "nodes changed" = owners whose node differs from `base`: (`testing-cargo` 257 of 258: one recipe of an embedding owner is the recipe that errors in every world)

| group | owners embedding | nodes changed before | nodes changed after |
|---|---|---|---|
| `package-router` | 534 | 534 | 0 |
| `runtime-identity` | 523 | 523 | 0 |
| `orphans-parent-script` | 523 | 523 | 0 |
| `runtime-sqlite` | 520 | 520 | 0 |
| `owned-execution` | 520 | 520 | 0 |
| `free-space` | 520 | 520 | 0 |
| `package-execution` | 310 | 310 | 0 |
| `runtime-parse` | 269 | 269 | 0 |
| `testing-cargo` | 258 | 257 | 0 |
| `testing-budget` | 236 | 236 | 0 |
| `owner-command` | 199 | 199 | 0 |
| `runtime-main-module` | 182 | 182 | 0 |
| `runtime-main-package` | 159 | 159 | 0 |
| `typescript-build` | 96 | 96 | 0 |
| `testing-execution` | 91 | 91 | 0 |
| `package-budget` | 82 | 82 | 0 |
| `module-tree` | 55 | 55 | 0 |
| `testing-vitest` | 52 | 52 | 0 |
| `test-adapter` | 31 | 31 | 0 |
| `normalization-digests` | 30 | 30 | 0 |
| `inventory-index` | 30 | 30 | 0 |
| `extension-package` | 26 | 26 | 0 |
| `test-source-text` | 25 | 25 | 0 |
| `typescript-package-index` | 25 | 25 | 0 |
| `validator-oracles` | 22 | 22 | 0 |

Early cutoff note: a first version of the worlds imported a test file next to the entry, which pulls in its own import closure: 134 owners then changed once after (580 before) because files they embedded through another route moved into a group (a one-time subtraction, not a repeated cost). The final worlds import a leaf file (no imports) from the entry's own package; the first version is kept as a documented exception: **a file added to a group that brings along a file some node already embeds through another route changes that node once**.

**Ordinary edits (the smallest-possible claim).** Five projects of different kinds, one comment line appended to one source file of each (`body5` applies the five at once to the 4 768 owners whose key set contains one of them). **Nodes changed: 0 of 4768 before, 0 of 4768 after** (a body edit changes no command list: lists name files, not contents); owners whose command key changes: 4768 before, 4768 after (every task that executes the edited code: the routing hub is in the key set of almost every command, as it must be). Inference records (fact index of generation 237, `harness/facts.ts`; `exists`, `lstat` and `stat` rows hold the file type, not size or time, so a body edit leaves them valid): 

| project (kind) | edited file | records holding a content-keyed row on it (fact index) | records that re-read it by analysis (answer unchanged, cut off) | owners whose command key contains it | projects whose tasks re-key through the graph (dependents) |
|---|---|---|---|---|---|
| `semio-s-plugin-energy-engine-zone-air` (Rust leaf crate (plugin engine)) | `…ns/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🏠️zone_air/🦀️.rs` | 0 | 1 | 0 | 22 |
| `@semio-tech/framework-trace-rs` (Rust framework crate used everywhere) | `…F/🔨️modules/⏱️trace/📦️packages/🦀️rust/🦀️.rs` | 0 | 6 | 0 | 4143 |
| `semio-s-artifact-norm-din16798-editor` (Rust plugin artifact editor) | `…ts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | 0 | 1 | 0 | 5 |
| `@semio-framework/os-infinite-terrain-ownership` (TypeScript module package script) | `…modules/♾️infinite/🌍️world/🏔️terrain/🚦️ownership/📜️script.ts` | 1 | 2 | 1 | 0 |
| `@semio-tech/framework-process` (TypeScript process library, hub of 4700 command closures) | `…/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` | 3 | 446 | 4768 | 3942 |

Reading: a body edit invalidates only the records that hold a content-keyed row (`file`, `typescriptText`) on the file: 0, 0, 0, 1 and 3 records; the `analysis` rows (1, 6, 1, 2 and 446 records) are re-read and compared by their answer, an unchanged import list cuts the record off. So at the inference level an edit of the body of one source file re-stales at most its own project and the few records that read its text; **no dependent is re-staled by inference**. At the task level the project and its real dependents re-key through the graph (22, 4 143, 5, 0 and 3 942 projects), which is the correct, source-keyed behaviour; whether a dependent of a Rust crate re-executes when the crate's compiled result is unchanged is decided by the cargo unit cache (result digest), which this package did not touch and did not measure. The routing hub is the one file in this list that is also in a tool group: its body edit moves the group of `package-router` (one group of the providers project) and the keys of the commands that load it, and not one node.

## 4. What remains, and why (owner counts)
- a. **Implementation data rows, 5 101 records**: the 7 files `🏗️build/🔣️policy.json`, `🏗️build/🕸️graph/🧬️schema/🔣️.json`, `🏃️process/🔌️providers/🔣️.json` and its schema, `⚡️caching/🔣️policy.json`, `🏗️build/🏃️execution/🧬️schema/💾️cache.json` and `🔣️.json` are `digest` rows of every record (generation fact index). They decide results (N9 classification "data nobody covers"): a byte edit re-stales every record on purpose. Not changed.
- b. **Digest implementation helpers, 4 774 nodes**: a hand-kept list of 17 files (plus the two literal entries) that `compactSourceInputs` (`🏗️build/📥️inputs/🟨️.mjs:223`) adds to every compacted group, among them `production` and `default`, which dependents use through `^production` and `^default`. Nx refuses a cross-project reference inside a group used with `^` (`liftNxNamedInputReferencesV1` throws `nx.dependency-input-not-projectable`), so a providers-project reference cannot replace the list there; the tool route needs Nx dependency-input projection, or the digest implementation must move into the engine input (`nxBuildEngineInputsV1` hashes the plugin closure only). Left; the list changes only when someone edits the digest implementation, and a derived tool would be more correct than the hand list (it is a hand-kept approximation of an import closure).
- c. **Single configuration files, 4 720 / 4 720 / 42 / 41 / 25**: `tsconfig.json`, `bunfig.toml` (javascript toolchain files in `sharedSources`), `rust-toolchain.toml`, `.cargo/config.toml`, `go.work`, `go.work.sum`. Not import closures: a policy datum (`toolchains.*.files`) of 2 to 4 files each; an edit of the list is an edit of the build policy, which moves the implementation digest anyway.
- d. **Named input `cargoToolchain`, 4 456 nodes** (`rustfmt.toml`, the cargo execution script): a second copy of the `native-toolchain` files, generated by `📥️inputs` (line 322) together with the owner's `Cargo.lock` input. A reference is possible in principle (the group is a plain named input of native targets) but the group is built in a module that does not know the providers project, and I could not prove from the repository that no target uses it with `^`; not done. Next step: have project assembly replace the policy files in that group by the `native-toolchain` reference after `projectInputs`, and add the `^cargoToolchain` check to the same test.
- e. **Per-subtree resolution configs, 320 (`✏️s/tsconfig.json`), 298 (`✏️s/package.json`), 52 (`🌎️hub/package.json`), 60 to 64 (`value`, `async`, `typescript` package manifests)**: the walk lists the configuration file that resolves a module. A new nested config re-stales exactly the owners below it: that is the intended precision, not a shared closure.
- f. **Artifact and library import closures, 33 to 66 owners per chain** (`value/decode` 66, `sqlite-snapshot` artifact chain 63 with 12 files, `ieee754` 38, `artifact-reference` 33, `schema/record` 21): the file-precise TypeScript source closure of workspace libraries inside `artifactSources` of artifact packages and in their command lists. A library edit that adds an import re-stales its importers by design (the Nx alternative, `^production`, is coarser). A project-level "exports" group referenced by key from the importers would remove it; that needs the same Nx projection as b.
- g. **Owner-specific walks**: 83 owners hold 13 646 of the 15 992 remaining names (playground, root, caching, course, print, storybook): each file is held by fewer than 21 owners; they re-stale only their own nodes.
- h. **Not measured**: the closure leaves missing on this machine (non-command counts are lower bounds); the real `np` record store of a refreshed generation (no graph read; the effect on late-mode owner records is by the late==direct law of R5c, which the suite re-checks).

## 5. Tests, proof of the edit (all on an overlay with `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`, rebuilt against the live tree after the peers' landing of 09:40)
| test directory | baseline (live) | with R29 |
|---|---|---|
| `provider-closures/🧪️tests` (main law, oracle) | 5 pass | 5 pass |
| `provider-closures/🧪️tests/🪡️tool-cover` | 6 pass | 6 pass |
| `provider-closures/🧪️tests/🧮️blast-radius` (new) | 3 fail (red before) | 3 pass |
| `script-inputs/🧪️tests`, `🔀️router`, `👓️walk-views` | 4, 25, 5 pass | 4, 25, 5 pass |
| `inference/🧪️tests/🧷️closure-facts`, `🔖️output-stamps`, `🔗️late-closures` | 22, 4, 4 pass | 22, 4, 4 pass |
| `🧱️project-assembly/🧪️tests` | 2 pass | 2 pass |
| `⚡️caching/…/🧾️command-inputs` | 2 pass | 2 pass |
| `🗓️scheduling/🧪️tests` | 18 pass | 17 pass, 1 rig artifact |
| `🔖️stamps/🧪️tests` | 22 pass | 20 pass, 2 rig artifacts |
Rig artifacts (the same as R5c reported): an overlay module loads next to the live one where a test imports by absolute path (`Field extractor already registered`), and the stamps oracle reads the live schema while the TypeScript side reads the overlay stamp `/5`. `land.py` runs all three live after the landing; both test directories are in its list with 0 expected red.
- Parse proof: every changed or new `.mjs`/`.ts` parses (`bun build --no-bundle`), the 5 changed JSON files and the Python oracle parse (`T/🧰️scripts/R29/parse.sh`).
- `python3 -I T/🧰️scripts/R29/land.py --dry-run --merge`: clean (8 files edited, 1 new, 0 refused).
- Rehearsal and rollback in a scratch root (`land.py --land --root <scratch> --authorized --no-tests`): 9 files landed, a second land was a no-op ("same"), `--rollback` restored every byte (`cmp` against the live originals: no difference).

## 6. Landing (the coordinator, in the single window)
Prepared only. Preconditions: `T/landing.lock` free, the working tree of the 8 files unchanged (the dry run says so), `N/LAND-OK` written by the coordinator (or `--authorized`).
```
python3 -I "/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R29/land.py" --land --merge
```
It takes the lock, writes 9 files with temp-and-rename, runs 13 test directories (all expected green), appends to `T/landings.md`, releases the lock; originals in `T/🗑️generated/R29-landing-<time>` for `land.py --rollback <dir>`. Effects: every project re-infers once (policy rows and stamp), then a landing that adds an imported file to any of the 26 groups moves the single providers project and not the nodes.

## 7. Not verified
- No graph read: the effect on a refreshed generation (real `np` records, fleet travel of the 26 new group leaves) is by the same argument as R5c (groups are ordinary closure entries of the providers project, restored with the generation by R8) and not measured.
- The scheduling and stamps tests run live only after the landing (rig artifacts above).
- The task-level cutoff by output digest (a dependent of an edited Rust crate re-runs although the crate's compiled result is unchanged) lives in the cargo unit cache and was not touched or measured.
- Non-command counts are lower bounds (missing leaves) and come from generation 237.
- Windows/Linux: the new code adds no path separators; the tool rows use `/`.
