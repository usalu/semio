# Result: Plugin Wave 0 Generators (carve engine, pilot space-space)

Executor: Sonnet. Scope: make artifact authoring produce gate-compliant crate structure, then pilot on one oversized plugin crate.
Ticket folder: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/`. Scratch tools and logs: `🗑️generated/pw0-*` (not deleted).

Library root used below: `L = 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`.

## 1. Design (written before coding, amended where reality corrected it)

### 1.1 Crate boundaries
- The unit of carving is a Rust module declaration inside the artifact crate's original mount nest (`#[path="."] pub mod ...`), grouped to taxonomy-component level (`🧬️schema`, `🔺️diff`, `🧬️mutations`, `✏️editor`, `🚪️io` codecs follow the type they encode). No per-leaf crates: pilot yields 4 component crates plus the artifact facade for 75 files / 212,827 bytes.
- Path-preserving "mirror" carve: each new crate has a lib root at `<component>/📦️packages/🦀️rust/🦀️.rs` that repeats the original `mod` nest of its slices (same effective module paths) and `pub use`s whatever it needs from sibling crates. Source files do not move; only the mount hierarchy is split. Effective paths (collapsing `mod component; pub use component::*;`) are equal before and after, which is what the verifier checks.
- Aggregation without mount fan-out: the artifact facade crate `semio-s-artifact-<x>` keeps its name and public API and re-exports the carved crates, so downstream `use` paths, the plugin registry and the wasm build entry are unchanged. Aggregate leaf tests are merged into one test file per pure namespace (T3), so the number of mounts does not grow with the leaf count.
- Generators decide the crate count by budget (target 28 files / 230,000 bytes / 3,500 lines, hard 32 / 262,144 / 4,000, in both lib and test cfg closures), never by one-crate-per-leaf.

### 1.2 Correction to the audit (S1/S2): coherence, not facet, is the cut line
`AUDIT-PLUGIN-CRATE-CARVING.md` proposed slicing schema and io codecs by facet. The orphan rule forbids it: impls of framework traits and inherent impls must live in the crate that owns the type. Slices are therefore by type (snapshot plus its codecs, diff plus its codecs, mutations aggregate plus leaves plus op codecs); the planner runs a coherence merge and reports `ambiguous coherence` as a problem instead of guessing. `checkOrphanRules` verifies the result (empty in the pilot).

### 1.3 Test closure
The gate measures the `test` cfg closure too, and it was the binding constraint (pilot original: 130 files / 361,297 bytes in test cfg). Three transforms, applied by `transformTestClosure`:
- T1 runtime fixtures: `include_str!` of test fixtures becomes `runtime_fixture_text(...)` (a `LazyLock` file read, with `@fixtures/` placeholders resolved to crate-relative paths), so embedded bytes leave the closure.
- T2 hoist: test files that are bound to the artifact root (use root-only items) are hoisted to the artifact root crate instead of being pulled into a component crate.
- T3 aggregate: sibling leaf test files of a pure namespace become one file (`use super::super::<leaf>::*`, carrying the leaf's private `use` imports).

### 1.4 Package glue
Package roots must classify as pure declaration (`classifyPackageSource`, taxonomy `packageGlueGrammar.rust`) and fit `libWiringLineBudget` (150 lines). Moved root items (consts, structs, impls, fns) go to `pub mod artifact_items` appended to a component file and are re-exported. Mirror roots that would exceed 150 lines or contain non-glue are planner problems (see section 7, "namespace hoisting").

### 1.5 Mutations
Concrete mutation leaves and the aggregate form a cycle across crates. Fix is the already existing energy precedent: leaves generic over `Op`, per-domain family crates, aggregate names only the family (`new mutation <root> <domain>/<op> --family-crate`). Automated for new mutations; converting existing concrete leaves (S2b) is not automated (section 7).

### 1.6 Wasm and generators
Cargo `members` are registered explicitly (explicit members override `exclude`), the artifact `📜️script.ts` router receives `packages: [...]` via `runArtifactRustPackageMain`, `Cargo.lock` is patched by hand-equivalent rows (`patchCargoLock`), and no registry or taxonomy generator input changes; the facade keeps crate name and entry.

## 2. Deliverables implemented

### 2.1 New library module `L/🏗️authoring/📦️crate-tree/`
- `🌳️module-tree`: `parseRustSourceTree` (module nodes with spans, `#[path]`, cfg(test), items), `findRustModule`.
- `🧱️decl-tree`: `collectDecls`, `buildSlices` (namespaces vs slices, coupling via `superReach`, union-find).
- `🗺️plan`: budgets, `measureRustCrate` (uses the same `readRustSourceClosureV1` as the gate), `planArtifactCrates` returning `ArtifactCratePlan` with `problems`.
- `📝️render`: mirror roots, manifests, workspace members, `Cargo.lock` patch, router packages.
- `🧪️test-closure`: T1/T2/T3.
- `🔬️verify`: effective-path module-tree equality, cross-crate visibility, orphan check, rustc unresolved-crate sets, sandbox and full-resolution differential.
- `🧾️facet-leaves`: `externalizeFacetLeaves` codemod (S3).
- `🧱️scaffold`: `scaffoldSubsetCrates` (new subset gets model/viewer/editor crates already within budget).
- `🧫️sandbox` (added last): `verifyCarveInSandbox(plan, sandboxDir, {oracle})` runs all verifications above against a sandbox copy of the plugin (copies plugin dir plus every `#[path]` target directory outside it).
- `🟦️.ts`: `carveWrites`, `applyCarveWrites(repoRoot, writes, backupDir)`.

### 2.2 Generators edited
- `L/🏗️authoring/🎮️command/🟦️.ts`: async `run`; new kinds `new crates <artifact-package-dir> [--dry-run] [--no-test-rewrites] [--backup=<dir>] [--verify [--oracle] --sandbox=<dir>]` and `new leaves`; `--family-crate` on `new mutation`; `new mutation` now assesses the crate budget first and refuses to grow an over-budget crate (S4/S5).
- `L/🏗️authoring/🗿️artifact-tree/🟦️.ts`: new subset scaffolds model/viewer/editor crates when the artifact has a Cargo workspace (S1 by construction).
- `L/🏗️authoring/🧬️mutation-tree/🟦️.ts`: `assessMutationCrateBudget`, family planning, aggregate writer (the ~181-222 region) generalised to an `updates[]` publication and to `extern crate` family variants.

### 2.3 S3: FacetLeaves `include_str!` out of the compiled closure
Registry (`🧰️framework/🔨️modules/🧬️schema/📇️registry/🦀️.rs`, region `🔖️SchemaLeafDocuments`): `SchemaLeafDocument`, `SchemaLeafReader`, `install_schema_leaf_reader`, `register_schema_leaf_documents`, `FileSystemSchemaLeaves`, and a served-leaf fallback in `SchemaExportRegistry::resolve` used when a registered `FacetLeaves` body is empty. The JSON schema leaf stays embedded. `externalizeFacetLeaves` rewrites typescript/graphql/proto `include_str!` leaves to `""` plus a `register_schema_leaf_documents(&[...])` call; every `FacetLeaves` consumer keeps working because resolution goes through the same registry API. Explicit exports added to the registry package root. New unit test `served_leaf_documents_resolve_when_the_registering_crate_embeds_no_body`.
Status: Rust port type-checked with rustc metadata emit (lib and `--test`); codemod only dry-run tested, NOT applied to any live artifact; the pilot did not need it (space-space embeds 10 small files, 17,172 bytes).

### 2.4 S6 and S7
- S6: `libWiringLineBudget` (150) is enforced by new test `L/🧪️tests/📏️lib-wiring-budget/🟦️.ts` with ratchet fixture `L/🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json` (402 pinned existing violators; new violations or growth fail). 3 tests.
- S7: `L/🧪️tests/🔬️rust-mount-emitters/🟦️.ts` allowlists the 7 emitters that write Rust `mod`/`#[path]` mounts (graph manifest projection, surface-schema, normalization, mutation-tree, test-closure, scaffold, test materialization) and fails on any new emitter. 1 test.
- Registered (scripts, `project.json`, `package.json` of `L/📦️packages/🟦️typescript`, `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`, orders 206.17001-206.17004).

## 3. Pilot: `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space` (semio-s-artifact-space-space)

Applied live by `applyCarveWrites` (regenerated, not hand-edited; re-applied twice after generator improvements via backup restore). New crates `semio-s-artifact-space-space-{schema,diff,mutations,editor}` under `🏅️standards/🔖️1/🪆️subsets/✳️any/{🧬️schema,🧬️schema/🔺️diff,🧬️schema/🧬️mutations,✏️editor}/📦️packages/🦀️rust/`, artifact root `🦀️.rs` reduced to re-exports, `Cargo.toml` members, `📜️script.ts` packages, `Cargo.lock` patched, 14 editor leaf test files merged into one aggregated test, a sqlite test hoisted, runtime-fixture tests.

Census (files / bytes / active lines, `🗑️generated/pw0-census-space-space-after.log`, live tree):

| crate | lib | test cfg |
| --- | --- | --- |
| original single crate | 75 / 212,827 / 3,369 | 130 / 361,297 / 5,464 |
| schema | 26 / 51,745 / 573 | 28 / 54,469 / 638 |
| diff | 12 / 41,569 / 821 | 16 / 51,542 / 980 |
| mutations | 21 / 33,185 / 611 | 28 / 87,114 / 1,426 |
| editor | 21 / 89,248 / 1,388 | 27 / 146,585 / 2,333 |
| artifact root | 2 / 12,200 / 209 | 4 / 31,320 / 364 |

Every row is within the target budget (28 / 230,000 / 3,500) in both closures.

## 4. Verification actually run

| check | command / driver | result |
| --- | --- | --- |
| census, lib and test, every resulting crate | `🗑️generated/pw0-closure.ts` -> `pw0-census-space-space-after.log` | all 10 rows `ok:true target:true` (table above) |
| module tree equality (effective paths) | `pw0-verify-carve.ts` -> `pw0-verify-final.log` | original 87, union 87, no problems |
| cross-crate visibility | same | `[]` |
| orphan / coherence | same | `[]` |
| unresolved crate names per carved crate (rustc, lib and --test) | same | `usedUnresolved: []` for all 4 crates |
| patched full-resolution differential (original vs test-closure rewrites, lib and `--test`) | `pw0-oracle-union.ts` | no candidate-only diagnostics (unique-diagnostic sets, file path ignored for relocated tests); details in the sandbox run during the pilot |
| Cargo.lock | parse check | parseable, no missing dependency rows |
| new tests | `bun test ./🟦️.ts` from each test directory, just now | crate-tree 6 pass; lib-wiring-budget 3 pass; rust-mount-emitters 1 pass; mutation-family 3 pass (needs `SEMIO_TEST_ARTIFACT_DIR`; 0 pass without it) |
| registry S3 type check | rustc `--emit=metadata` lib and `--test` | passed |
| sandbox verifier (added last) | `pw0-sandbox-try.ts` on trinity-jack (not applied) | runs end to end: census, tree equality `[]`, orphan `[]`, unresolved `[]`, flags editor over budget (40 files lib) and 2 glue verdicts and 1 visibility violation (`decode_camera` is `pub(crate)`), matching the planner's 4 problems |

### Blocked or not run (all infra, none caused by this change)
- Real compile: `SEMIO_SCRIPT_POLICY=... bun repo cargo --store=shared --configuration=prototype -- check --manifest-path <space schema crate Cargo.toml>` -> exit 1: `cannot update the lock file .../🪐️space/Cargo.lock because --locked was passed` (log `pw0-cargo-check-final.log`). The same error occurs for the untouched `🏠️home` plugin (`pw0-cargo-check-space-home.log`), so the stale-lock baseline predates this work. Compilation is therefore NOT confirmed; only rustc-oracle and metadata evidence exists.
- Plugin registry `generate` and `check-generated`: fail in the build-owner admission with `Canonical native dependency owner required: 🧰️framework/🔨️modules/◻️2d/⚙️engine/...` before and after (`pw0-check-generated-baseline.log`, `pw0-check-generated-after.log`). Not verified through the tool.
- `verify taxonomy report`: cannot start because root `📜️script.ts` does not export `INTERACTIVITY_AUDIT_WORLD3D_FILE` (peer rename). Substitute: direct calls to `validateRustTaxonomyMounts` / `validateTaxonomyTree` gave identical findings before and after; this is weaker evidence than the tool.
- Nx route for the new TS tests: "Active graph readers retain required Nx pool headroom" (`pw0-nx-test-crate-tree.log`); tests were run directly with `bun test`.
- `bun ./📜️script.ts new crates ... --verify` (CLI wiring of the verifier): NOT run, root script fails to load for the same export error (`pw0-cli-verify.log`). The verifier function itself was run through a driver.
- Pre-existing unrelated test failures seen but not touched: `artifact-io-ownership`, `artifact-empty-facet-authoring` (4), `root-clean-scaffold-source` (3), workspace-contract does not load.

## 5. Survey of the remaining 75 oversized packages (final planner, `pw0-survey2.log`)

62 plan, 13 error. 11 plan with zero problems. Across planned packages the problem kinds are: over-budget 48, wiring-lines 35, test-hoist 15, glue 6 (counts are packages, not occurrences). Errors: 9 `Missing Rust source input` (fixtures or schema documents that the closure reader cannot find, e.g. puzzle-5d, gif, program), 2 `Generated Rust input requires compiler ownership` (stdio-pdf), 1 `Unsupported Rust compile expression ... include_str` (bim model), 1 `Missing or ambiguous Rust module: component` (curation).
Interpretation: roughly 15% are mechanically carvable today; most others fail on a single oversized slice (mutations or schema: gltf, puzzle-2d, trinity-rewriting, shooting, note, forms), which needs the mutation-family genericization or finer slicing, not new engine work in the loop.

## 6. Exact per-plugin rollout recipe

1. Pick the package from `pw0-oversized-packages-2.txt` (or `bun ./📜️script.ts verify ...` census); open or reuse the plugin's ticket (do not use the shared ticket).
2. Dry run: `bun ./📜️script.ts new crates <artifact>/📦️packages/🦀️rust --dry-run`. Read the problems. Fix by kind: `over-budget` -> section 7 items; `test-hoist` -> allow default transforms (omit `--no-test-rewrites`); `wiring-lines` / `glue` -> mirror root needs namespace hoisting (section 7); `Missing Rust source input` -> fix the fixture path in the original first (these are original tree defects).
3. Verify before touching the tree: `... new crates <pkg> --verify --oracle --sandbox=<dir outside repo>`. Exit code is non-zero if any crate is over the hard budget, the module tree differs, visibility or orphan violations exist, a crate has an undeclared unresolved dependency, or the differential reports candidate-only diagnostics.
4. Apply: `... new crates <pkg> --backup=<dir>` (backup folder under the ticket `🗑️generated`). Never hand-edit generated crate roots; fix the generator or the original source and regenerate (restore from backup first).
5. Re-measure with the census (lib and test) for every new crate and the facade; each row must be within 28 / 230,000 / 3,500.
6. Run the plugin's existing Rust tests through `bun repo cargo --store=shared ...` once the lock baseline is repaired (currently refused), the plugin registry `generate` then `check-generated`, `verify taxonomy report` (no new findings), and the TS tests of touched generators.
7. For artifacts with FacetLeaves `include_str!` over budget: run the `externalizeFacetLeaves` codemod (typescript/graphql/proto leaves), then re-census.
8. For mutation-heavy artifacts: first convert concrete leaves to generic-`Op` family crates (`new mutation ... --family-crate`), then carve.
9. Record the result file with the census table and the exact command results (including blocked ones).

## 7. Residual risks and open work
- S2b: existing concrete mutation leaves are not auto-genericized; mutation-heavy artifacts (gltf, puzzle-2d, trinity-rewriting, shooting, note, forms) remain blocked until that is automated or done by hand per family.
- Namespace hoisting for mirror roots over 150 lines or containing non-glue is not implemented; planner reports it as a problem rather than guessing.
- Dependency pruning not done: carved crates copy all original dependencies (29-32 of 31-34 unused per crate in the pilot). Harmless to correctness, increases build graph; prune after real compile is available.
- `module_path!()` strings change after carve (953 uses repo-wide); any persisted or logged module path must be reviewed per plugin.
- Real compile and registry/taxonomy tool runs are unconfirmed (section 4). Treat the pilot as statically verified, not build-verified.
- S3 codemod is unapplied to live artifacts; S3 Rust port needs a cargo run of the registry crate tests once the compiler gate allows it.
- The root `📜️script.ts` currently fails to load (peer rename), so all CLI entry points of the new generators are untested end to end; library functions were exercised directly.
- Temporary `[DEBUG]` logs: none left in the new modules (checked at write time; no `[DEBUG] ` prefixed logging was added to library code).

---

# Follow-up (coordinator items 1 to 3): S2b automation, rollout wave A, survey v3

Library root `L = 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. Scratch tools and logs are in `🗑️generated/pw0-*` (not deleted). Load on the machine was 60 to 100 during this follow-up, so everything below ran sequentially and niced; no cargo was run.

## 8. What changed since sections 1 to 7

### 8.1 Retry of the CLI path
After the coordinator fixed the root `📜️script.ts` export, `bun ./📜️script.ts new crates <pkg> ...` loads and runs (`pw0-cli-verify.log`: plan printed, `problem:` lines printed, exit 1 for trinity-jack, as designed). The `--verify` branch needs a package whose plan has no problems; it was exercised for the first time on `stl` (section 11.1: exit 0, both differentials empty) and `new crates` without `--verify` applied it. Flags added: `--mutation-families=off|on|auto`.

### 8.2 Owner rows
`new crates`, the subset scaffolder and `new mutation --family-crate` now register every new crate in `📋️build-owners.json` (`insertBuildOwnerRows`: byte-order insertion, never twice, read-modify-write at call time because peers edit the file; the added names are written beside the backup as `<backup>-owner-rows.json`, the shared file itself is never restored from a backup). `owners-tools/scan.ts` after wave A: 2295 packages, 2433 owners; none of the 57 crates I added (4 pilot, 35 wave A, 18 wave B; scan run again after wave B: 2355 packages, 2502 owners) is listed; the listed `missing` entries are `🧰️framework/🔨️modules/🖱️ui/🧬️contract/*` crates of a peer.

## 9. S2b: mutation-family conversion (item 1)

### 9.1 Why leaves cannot be split today and what the conversion changes
A concrete mutation leaf names its aggregate in three places: the builder returns it, `impl MutationKind<Snapshot, Aggregate> for Payload`, and the inverse companion returns `Vec<Aggregate>`. The aggregate in turn mounts and re-exports every leaf. That two-way edge puts the aggregate and all its leaves into one strongly connected component, which the planner correctly refuses to cut. The conversion rewrites the leaf side into the form the energy plugin already uses (leaf generic over `Op`), so only the aggregate-to-leaf edge remains and the planner can place leaves in separate crates.

### 9.2 What `convertMutationLeaves` (`L/🏗️authoring/📦️crate-tree/🧬️mutation-families/🟦️.ts`) rewrites
Discovery: every `#[derive(.. dsl::Mutations ..)]` enum of single-field tuple variants (also reads `#[mutations(snapshot = .., diff = ..)]`); each variant payload is located by its `#[derive(MutationLeaf)]` struct, disambiguated by the aggregate's own `pub use` paths or the written variant path when several standards define the same name. Per leaf (component file plus its `diff` and `inverse` companions):
- builder `fn b(args) -> Agg { Agg::V(x) }` becomes `fn b<Op>(args) -> Op where Op: From<crate::..::Payload> { Op::from(x) }`;
- `impl MutationKind<S, Agg> for P` becomes `impl<Op> MutationKind<S, Op> for P where Op: Mutation<S, Diff = D> + From<P> + <bounds the companions need>`; `D` comes from the impl's `diff` signature, or, when that signature is written as `<Agg as Mutation<S>>::Diff`, from an absolute path resolved through the aggregate's imports;
- inverse companion gets `<Op>` plus one `From<..>` bound per variant it constructs and per other leaf builder it calls; `Agg::V(..)` becomes `Op::from(..)`; `use Agg::*;` inside a function plus bare variant calls are handled;
- the aggregate keeps a concrete builder wrapper (`pub fn move_block(..) -> NoteMutation { <leaf>::move_block::<NoteMutation>(..) }`) in a `ConcreteBuilders` region wherever it used to re-export the builder, so consumers keep their type inference; when a builder parameter type cannot be named in the aggregate the generic builder stays re-exported;
- test modules of a leaf that mention the aggregate and use `super::` get their `super::` paths made absolute (semantically identical, they sit in the same module), so they no longer need the leaf in their crate;
- anything else (inline `#[cfg(test)]` module in a leaf file that names the aggregate, inherent impls on the payload, aliased imports) leaves that leaf concrete and is reported with the reason.
Removal of the aggregate from `use` lists is done by a real use-tree parser (`pruneUseLeaves`); the first version used the shallow `prunedUse` and produced unbalanced braces on nested groups (found by the planner's own tokenizer on `png`).

### 9.3 Planner integration and deterministic family assignment
`planArtifactCrates(.., { mutationFamilies: "off" | "on" | "auto" })`; `auto` (default) plans without conversion first and only retries with conversion when the plain plan has a hard-budget problem and the converted plan has fewer problems, so artifacts that already fit are never touched. A converted plan failing to measure falls back to the plain plan.
`buildSlices` got one rule: a cfg(test) module inside an inline leaf wrapper that needs nothing from its parent (`superReach` 0) and names an aggregate becomes a slice of its own, so aggregate-bound leaf tests no longer pull the leaf back into the aggregate's crate.
Family assignment is a pure function of the mount tree and the budget, documented here and asserted by a test (two plans of the same tree give identical crate names and directories): slices are ordered by dependency layer and then by their position in the artifact root's mount nest (the taxonomy order of the mutation directories); each slice or strongly connected component goes to the first existing crate, preferring the crate that already holds its parent mount and then the crate it shares edges with, that still fits the target budget in lib and test closure and keeps the crate graph acyclic; otherwise it opens a new crate. A crate that holds only leaves is named `<artifact>-mut-<subset>`, so a family is a contiguous run of leaves of one subset where they fit. The aggregate stays a flat enum (wire tags unchanged); a nested enum-of-family-enums would change the codecs and was not done. Diff and inverse companions travel with their leaf, so "companions per family" holds by construction. New mutations keep using `new mutation <root> <domain>/<op> --family-crate` (energy layout).

### 9.4 Tests added (all pass, run directly with `bun test`)
- `L/🧪️tests/🧬️mutation-leaf-generics/🟦️.ts` (5): aggregate discovery; generic rewrite of builder, impl and inverse including cross-leaf bounds and the concrete wrapper; a leaf left concrete with its reason; `use` pruning compared with `rustfmt` as a second parser; plan places converted leaves and their detached tests in separate crates while the unconverted one stays with the aggregate, and the plan is deterministic; associated-diff-type, glob-import and relative re-export forms. Every rewritten file is also fed to `rustfmt --emit stdout` as an independent parser.
- `L/🧪️tests/📦️crate-tree/🟦️.ts` grew from 6 to 13 tests (owner rows, include rebasing, manifest-dir rebasing, hoist rules, fixture rules).
- Registered as `test-mutation-leaf-generics` in the TS package `📜️script.ts`, `📋️project.json`, `package.json`, and in both launch files (order 206.17005).
- NOT verified: that the generic forms type-check. No cargo or compiler gate was available; the forms mirror the energy plugin line by line and are checked by rustfmt (parse) and the resolution differential, not by rustc type checking. A real cargo check of the first converted artifact is the first thing to run once the lock baseline is repaired.

### 9.5 Conversion coverage over the 75 oversized packages (`pw0-famsurvey.log`)
1166 of 1185 mutation leaves convert mechanically (98.4 percent; the first run converted 697). The 19 left concrete: 16 leaves with an inline `#[cfg(test)]` module naming the aggregate (config aggregates of dag, procedure and others), 3 with an inherent impl on the payload. The pdf aggregate family (76 leaves) converts completely once hints from the aggregate's re-exports are used.

## 10. Bugs the differential found in the wave 0 transforms (all fixed, each with a test)
The sandbox differential (original versus rewritten tree, rustc name resolution, lib and `--test`) rejected 6 of the first 11 candidate packages. These were real defects of the transforms of section 1.3, not of the packages:
1. T2 hoisting left the doc comment of the removed `mod tests;` mount dangling (`expected item after doc comment`); the doc comment now goes with the mount.
2. T2 hoisted tests that use `super::`; they cannot work at the root. Not hoisted any more.
3. T2 hoisted a test module that sibling tests name through `super::super::tests::fixture()`. Kept in place.
4. T2 hoisted tests that glob-import their host module (relying on its private imports) or use its private items; kept in place, and private absolute `use` imports of the host are carried into a hoisted test.
5. T1 replaced `include_str!` inside `concat!` by a runtime read (`expected a literal`); such includes stay embedded and a file whose const feeds `concat!` is left alone.
6. T3 aggregated tests that use private items of their host; now ineligible, and two tests of one host get distinct module names.
7. Root items referenced only from tests were reported as `test hoist needed`; they now move into `artifact_items` like lib-referenced items, with `include_str!` paths rebased and a host crate that has a mounted file and creates no crate cycle.
8. Paths joined onto `env!("CARGO_MANIFEST_DIR")` break when a file lands in a crate in another directory; they are rebased at write time (`rebaseManifestDirPaths`) and the planning mirror root is now a sibling of the package directory so measuring sees the original depth.
9. `pub(crate) mod` mirror lines are not package glue; widened to `pub` in mirror roots.
10. The sandbox did not copy directories reached only by `include_str!`; now copied.
These changes mean packages the first survey called clean are not all clean: `epw` and `mp3` now need hand work (their snapshot tests cross-reference through `super::super::tests` and must stay beside the type), which is the correct answer.

## 11. Rollout wave A (item 2): eleven zero-problem candidates
Procedure per package: plan; `verifyCarveInSandbox` with the full-resolution differential (lib and `--test`); apply only if census within the hard budget, module trees equal, visibility and orphan checks empty, no undeclared crate, and zero candidate-only diagnostics; owner rows; live census. Driver: `pw0-rollout.ts`, logs `pw0-rollout-<tag>.log`, backups `pw0-backup-<tag>*`.

| artifact | result | new crates | max lib (files / bytes / lines, per metric) | max test cfg (per metric) | differential candidate-only (lib / test) |
| --- | --- | --- | --- | --- | --- |
| stdio binary | applied | 4 + facade | 27 / 78,437 / 1,085 | 28 / 111,326 / 1,319 | 0 / 0 |
| stdio csv | applied | 4 + facade | 26 / 120,750 / 1,722 | 28 / 227,762 / 2,587 | 0 / 0 |
| stdio md | applied | 4 + facade | 24 / 172,309 / 3,042 | 29 / 213,788 / 3,238 | 0 / 0 |
| stdio bcf | applied | 4 + facade | 28 / 176,767 / 2,101 | 28 / 176,767 / 2,204 | 0 / 0 |
| stdio las | applied | 5 + facade | 26 / 190,882 / 2,950 | 27 / 195,177 / 3,023 | 0 / 0 |
| stdio ply | applied | 4 + facade | 28 / 185,816 / 2,391 | 29 / 221,909 / 2,543 | 0 / 0 |
| stdio obj | applied | 4 + facade | 27 / 214,328 / 3,257 | 29 / 217,974 / 3,433 | 0 / 0 |
| imperative procedure | applied | 6 + facade | 25 / 105,864 / 1,663 | 28 / 120,739 / 1,979 | 0 / 0 |
| stdio epw | not applied | plan has an over-budget schema crate (58 files) | n/a | n/a | n/a |
| stdio mp3 | not applied | plan has an over-budget schema crate (48 files) | n/a | n/a | n/a |
| stdio deflate | not applied | `diff` crate test closure 25 / 251,083 / 4,240 lines, over the hard line budget (one coherent hand-rolled codec slice) | n/a | n/a | n/a |

Each cell is the maximum of every crate of that artifact per metric (so a row can mix crates); the per-crate rows are in `🗑️generated/pw0-census-wave-a.log`: 43 crates (35 new, 8 facades) are all within the hard budget and 40 within the 28 / 230,000 / 3,500 target; the three outside the target are `md-inferences` (test 29 files / 3,238 lines), `ply-schema` (test 29 files) and `obj-inferences` (test 29 files / 3,433 lines), all under the hard limit. The las log was overwritten by an idempotent second pass over the already carved artifact (0 new crates); its first-pass differential was 0 / 0 and its live census is in the census log. All carved crates have owner rows, `Cargo.lock` rows (no new dangling dependency; ply and obj carry the same three dangling `os-directory` rows as at HEAD), workspace members, router packages.
trinity-jack was not ready: `editor` is one 24-slice component (coherence on the window config and transient types) of 40 files / 156,973 bytes, over the hard file budget; the `pub(crate)` glue fault that also blocked it is fixed.

Before and after checks run for the touched plugins (`pw0-taxcheck.ts`, direct calls to `validateRustTaxonomyMounts` and `validateTaxonomyTree`): stdio 0 mount findings and 1 tree finding (`gif` standard `sqlite` missing `subsets/`) before and after; imperative 1 finding (`📦️packages/🦀️rust/Cargo.toml` missing, an empty directory from Oct 10) before and after. Not run as the registered tools: `verify taxonomy report` (large, load), plugin registry `generate` and `check-generated` (the coordinator did not report them fixed). Not run: rustc undeclared-crate check separately; it is part of the sandbox report and was empty for every applied artifact.

### 11.1 Wave B: packages that became mechanical after the fixes of section 10
The re-survey (section 12) found four more packages with zero problems. Same procedure; `stl` went through the CLI end to end (`new crates <pkg> --verify --oracle --sandbox=<dir>` exited 0 with all checks empty and both differentials 0, then `new crates <pkg> --backup=<dir>` wrote 21 files for 4 crates and registered 4 owner rows), the other three through `pw0-rollout.ts`.

| artifact | new crates | max lib (files / bytes / lines) | max test cfg | differential candidate-only |
| --- | --- | --- | --- | --- |
| stdio stl | 4 + facade | 28 / 121,126 / 1,541 | 29 / 175,509 / 1,885 | 0 / 0 |
| stdio tsv | 4 + facade | 23 / 81,610 / 1,483 | 29 / 136,839 / 1,957 | 0 / 0 |
| stdio html | 4 + facade | 25 / 112,641 / 1,413 | 28 / 176,591 / 1,861 | 0 / 0 |
| playbook playbook | 6 + facade | 26 / 88,254 / 1,427 | 28 / 131,039 / 2,087 | 0 / 0 |

Per-crate rows: `pw0-census-wave-b.log`. All within the hard budget; stl and tsv have a test closure of 29 files (target 28, hard 32). `Cargo.lock` of tsv, html, playbook has no dangling dependency; stl carries 3 dangling `semio-framework-os-directory` rows, the same as ply and obj and as HEAD. The playbook taxonomy finding (`📦️packages/🦀️rust/Cargo.toml` missing) is the same empty directory from Oct 10 as in imperative and is unchanged.
Totals: 13 artifacts carved (the space-space pilot plus 8 in wave A and 4 in wave B), 57 new crates plus 13 facades, 0 candidate-only diagnostics in any differential. The lib-wiring ratchet fixture lost the 6 facade roots that now comply (402 to 396 entries; the test demands that shrinkage), and its test, plus crate-tree (13), mutation-leaf-generics (5), rust-mount-emitters (1) and mutation-family (3), pass when run directly with `bun test`.

## 12. Survey v3 after S2b and the fixes (item 3)
Planner `planArtifactCrates` default (`auto`), every remaining oversized package (`pw0-remaining.txt`, 67 packages: the 75 of section 5 minus the eight wave A packages), read-only, log `pw0-survey3.log`, table `pw0-survey3-table.md`. Column "blocker kinds" comes from the new `plan.blockers`: a component the orphan rule or a dependency cycle forces into one crate although it exceeds the target; its causes are `coherence` (an impl of a framework trait for a type that lives in another slice), `cycle` (any other mutual dependency), `parent-child` (a file module and a module it mounts depend on each other) and `mutation` (aggregate and leaves, the S2b case).

Classes: A mechanical now: 4 (all four carved in wave B). B structural blocker, needs generator work: 44. C4 embedded asset of at least 262,144 bytes (S4): 12. C5 source file of at least 3,000 lines (S5): 2. E the counter refuses the closure: 5.

### 12.1 What S2b changed in the survey
Conversion is attempted when the plain plan breaks the budget and is kept when it lowers the problem count: it was kept for 2 of the 44 B packages (`grid2d`, `forms`) and did not make any package clean on its own. 17 B packages have a `mutation` cycle among their blockers, but every one of them also has `coherence` and usually `parent-child` or `cycle` edges inside the same component: the snapshot, diff and mutation types are tied together by impls that sit in one file (`canonical`, the io codecs) and by the io parent module that mounts the codecs and is used by them. Splitting the leaves frees only part of the component. So S2b is necessary but not sufficient, and the survey says what is next, in order of packages unlocked:
1. Parent-child weave (15 B packages): the mount re-export in the parent crate (`pub use child_crate::..`) is what makes the parent depend on its child while the child uses the parent. Fix: keep parents free of re-exports of children that they do not themselves use, and let the facade crate weave the module tree (`pub mod io { pub use A::io::*; pub mod binary { pub use A::io::binary::*; pub use B::io::binary::snapshot; } }`, legal because a local module shadows a glob import). This preserves every public path and removes the cycle; the facade lines go into a component file to respect the 150-line root budget.
2. Coherence splitting (all 44 B packages touch it): move impls of one trait for several types into one file per type (codemod on a single `impl` block per type, no behaviour change) so the orphan rule stops merging snapshot, diff and mutations.
3. Test-closure coupling that pins tests beside the type (epw, mp3): sibling test modules that reach each other through `super::super::tests::..` need an absolute-path rewrite before they can move.

## 13. Per-package rollout plan: mechanical versus hand
- Mechanical, done: space-space (pilot), binary, csv, md, bcf, las, ply, obj, procedure, stl, tsv, html, playbook (13).
- Mechanical after generator work (class B, 44): needs items 1 and 2 of section 12.1, then `new crates` as in section 14. Smallest first: the semio subsets cad, flow, animation, image, model, graph, drawing (3 to 4 slices each) and gltf inferences / mutations, dxf, zip, bmp.
- Hand work or asset port first:
  - S4 (12 packages, `emb` column in the table): gif (21.0 MB embedded), mp4 (2.7 MB), stdio-semio (2.8 MB), avi (0.77 MB), jpg (0.48 MB), architect program (0.38 MB), epw (0.40 MB), semio subsets mesh, brep, presentation (0.28 to 0.32 MB), gltf schema and editor (0.28 MB). The existing S3 channel (`SchemaLeafReader`) covers schema companions only; example and fixture payloads need the runtime asset port of WAVE4-BRIEF step 2, after which these packages re-enter class A or B.
  - S5 (2 packages): equation (`cas-internals` 5,092 lines), sequence (3,739 lines); split the file by the component rule, then rerun the survey. flow editor (2,950 lines) and animate presentation (2,425 lines) are class B but need the same split.
  - S8 / input defects (5): pdf and home (generated Rust input needs compiler ownership), bim (`include_str!` with a computed argument at line 13), puzzle-5d and sourcing curation (missing source input). Not carve problems; fix the closure first.
- Not ready: trinity-jack (44-slice editor component, coherence on window config types), deflate (diff codec one coherent slice, 4,240 test lines), epw, mp3.

## 14. Per-plugin rollout recipe (updated)
1. Survey: `planArtifactCrates` (or `bun ./📜️script.ts new crates <pkg> --dry-run`); an artifact with `problems: []` is class A. Read the `note:` lines: `kept ... in place` explains tests that stay with their type.
2. Verify without touching the tree: `bun ./📜️script.ts new crates <pkg> --verify --oracle --sandbox=<dir outside the repository>`; exit 0 requires every crate within the hard budget, equal effective module trees, empty visibility and orphan reports, no undeclared crate, no glue defect and zero candidate-only diagnostics in the lib and `--test` differentials. Budget about 5 minutes at normal load.
3. Apply: `bun ./📜️script.ts new crates <pkg> --backup=<ticket generated dir>/<tag>`; it writes crates, facade, members, router packages and `Cargo.lock`, and adds the owner rows (the names are saved beside the backup). Do not restore `📋️build-owners.json` from a backup; remove the listed rows instead.
4. Census (`pw0-census-live.ts` style: `measureRustCrate` lib and test for every workspace member) and `owners-tools/scan.ts`; update the lib-wiring ratchet fixture by deleting the roots that now comply.
5. Taxonomy validators before and after (`validateRustTaxonomyMounts`, `validateTaxonomyTree`); registered tools when they run again: plugin registry `generate` then `check-generated`, `verify taxonomy report`.
6. Class B: use `--mutation-families=auto` (default) so mutation leaves become generic only where needed; read the `mutation aggregate` notes for leaves left concrete.
7. First real cargo check of one carved artifact once the lock baseline is repaired (one cargo at a time); this is the only step that can confirm the generated generic leaves type-check.

## 15. Verification status and open risks
- Verified by running: all plans and differentials above; census of 57 new crates and 13 facades (all within the hard budget); `Cargo.lock` consistency; owner coverage scan; taxonomy validators; the unit tests listed in 11.1; rustfmt parse of every converted leaf in the tests.
- Not verified: any cargo build or test (compiler gate and stale lock baseline, same as for `🏠️home`); type-checking of the generic mutation leaves; plugin registry `generate` / `check-generated` and `verify taxonomy report` as tools.
- S2b was not applied to any live artifact: every carved package fit without it, so `auto` never converted. Its evidence is the unit tests, the rustfmt parse, and the conversion survey (1166 of 1185 leaves convert, 19 stay concrete); the first live application should be one of the semio subsets or gltf inferences after the facade weave exists.
- The differential checks name resolution, not types or behaviour. `module_path!()` strings change after a carve (953 uses repo-wide); dependency lists are copied in full (unused ones are a build-graph cost, not a correctness issue).
- Temporary `[DEBUG]` code added during this follow-up was removed from the planner before the last survey; no ticket close or goal change was made.

## 16. Follow-up 3: facade weaving, the multi-crate layout differential, per-unit hosting, and the permanent union gate

Coordinator decisions executed here: fix forward (no revert); per-item hosting of root items; root-bound tests hoisted to the facade level; the multi-crate union check becomes a permanent gate; re-carve the applied artifacts until the check shows 0.

### 16.1 Finding that triggered the rework

The earlier differential compiled the carved tree as one crate and therefore could not see cross-crate name resolution. A new differential mounts every carved crate as a module of one synthetic crate (`__combined.rs`, `__c_<ident>`), rewrites `crate::` and dependency crate names, and compares the rustc resolution diagnostics (code and message, external-crate noise filtered) with the pre-carve tree. Run on the twelve applied carves it showed that all of them had between 1 and 16 names that did not resolve across crates (sample: `schema`, `engine`, `standards` root alias modules, `artifact_items` paths). Pilot space-space had 0. Real `cargo check` is still blocked by the stale lock baseline, so this check is the strongest evidence available; no applied crate has been compiled by cargo.

Root cause: the planner did not model the real crate dependency graph. Root alias modules, `super::` paths in inline modules, references to root items and the transitive closure of those items were not edges, so cycles were invisible and every plan looked cleaner than it was. With the correct graph the earlier results change: binary, tsv, html, stl, md, csv, deflate, wfc and space-home first became blocked, then most became plannable again after the fixes below.

### 16.2 Generator changes (all in `L/🏗️authoring/📦️crate-tree/`, no temporary code left)

| change | where | covered by |
| --- | --- | --- |
| Facade weaving; parent keeps the mount re-export when any slice outside the child's subtree references the child path | `🗺️plan` (`weave`, `referencedFromOutside`), `🟦️.ts` | 3 weave tests |
| Root alias modules (`pub mod schema { pub use super::..::*; }`) are not slices; they are copied into each crate that goes through them, and the edge goes to the owner of the aliased path | `rootAliasModules`, `shimCopies` | 2 tests |
| `super::` paths in inline module bodies are dependency edges | `superChains` | test |
| Relative references to child modules (`import::x`) and references to an item defined in an ancestor module (module and fn of one name) are edges | `scan` | gate runs |
| Visibility coupling: a path to a private, `pub(super)` or `pub(crate)` item or module, or through a private `component` module, merges the two slices | `restrictedTarget` | gate runs |
| Moved root items are hosted per unit (strongly connected set of items/regions): constants in the lowest crate every referencing crate can depend on, assembly functions above everything they use; dependents are placed first, each placement adds only edges that keep the crate graph acyclic; units nobody references and that fit nowhere stay in the facade | `stronglyConnected`, `movedUnits`, `assign` | gate runs |
| Root module heads in moved text are qualified (`crate::schema::..`), the host is never a `cfg(test)` module, `impl` items are never exported names, `pub use` leaves of moved regions are exported by the host and imported by the facade, copied root `use` statements follow the aliases they use | `qualifyModuleHeads`, `copiedNames` | test |
| References from slices to root items add the item closure as edges (this removed the false "clean" results) | `closureOf` | gate runs |
| Local wrapper under an imported module no longer shadows it (nested glob re-exports); imports below a file module the crate mounts itself are not emitted | `nestedGlobs`, `attachPaths` | gate runs |
| Cycle merging by Tarjan SCC instead of a pairwise reachability loop (the bim hang) | `mergeCycles`, `mergeFileChains` | plans identical, seconds |
| `prunedUse` rewritten as a real use-tree pruner (it used to split nested groups on commas and dropped leaves and `self`) | `🌳️module-tree` | test through `hostPrivate` |
| Test hoisting extended: tests that call root items are hoisted to the facade including their nested test files; private host items and modules they use are widened to `pub` (also the restricted items of a widened child module) and imported by absolute path; a glob import of the host carries the host's private imports; inline `super::` that stays inside the test file is left alone, `pub(super)` becomes `pub(in crate::..)`; a hoist is refused when another file names the test module by path; carried lines go behind inner doc comments | `🧪️test-closure` | hoist test updated (it used to keep such tests in place) |
| Detach extended in the same way, and `::component::` segments of absolute test paths are dropped | `🧪️test-closure` | gate runs |
| Harness: embedded `.rs` files are no longer rewritten twice; the layout baseline is the original tree (strictest); external-crate diagnostics are filtered; `rust-mount-emitters` audit list gained the sandbox module | `🧫️sandbox`, test | tests |

### 16.3 The permanent gate

- `verifyAppliedLayout(repoRoot, artifactDir, sandboxRoot, baseRef = "HEAD")` in `L/🏗️authoring/📦️crate-tree/🧫️sandbox/🟦️.ts`: mounts the crates of the working tree as modules of one crate and compares lib and test diagnostics with `git archive <baseRef>` of the artifact. Zero `onlyCombined` means the carve introduced no unresolved name.
- CLI: `bun ./📜️script.ts new crates <artifact-package-dir> --check-applied --sandbox=<dir>` exits 1 on any extra diagnostic.
- `new crates --verify --oracle` now compares the layout against the original tree, not the transformed union tree, and fails on any extra diagnostic (lib and test).
- Test: `gates an applied carve ...` creates a throw-away git repository, commits a fixture, applies a carve, expects 0/0, then breaks a path and expects the gate to fail (22 crate-tree tests pass).

### 16.4 Re-carve results (revert to HEAD of the previously applied files, then plan, verify with all checks, apply; scripts `pw0-revert.ts`, `pw0-rollout.ts`)

Final state, each verified by `new crates <pkg> --check-applied` after applying (exit 0, `onlyCombined` 0 for lib and test):

| artifact | crates | census lib / test per crate (files / bytes / lines) |
| --- | --- | --- |
| stdio tsv | 5 + facade | snapshot 14/50,378/482 .. 15/72,181/553; inferences 25/52,942/975 .. 28/57,306/1,063; schema 26/68,517/367; mutations 15/83,228/1,429 .. 21/117,623/2,006; registry 3/16,385/320; facade 1/5,103/117 .. 6/51,966/384 |
| stdio binary | 4 + facade | snapshot 27/71,980/904; schema 28/52,513/287; mutations 18/68,571/1,098; registry 8/38,883/729; facade 1/7,690/167 |
| stdio html | 5 + facade | schema 23/92,472/146; snapshot 9/92,326/1,211; inferences 25/75,879/1,412; mutations 21/90,397/1,567 .. 29/134,617/2,293; registry 3/14,926/300; facade 1/5,751/127 |
| stdio ply | 4 + facade | inferences 26/203,492/2,887 .. 29/208,182/3,034; mutations 23/72,067/1,252; schema 25/131,032/420; registry 7/50,246/964; facade 1/5,954/142 |
| stdio csv | 5 + facade | editor 15/9,339/186; snapshot 14/54,450/605 .. 18/97,973/891; schema 27/71,810/242; mutations 22/97,775/1,829 .. 29/146,307/2,502; registry 8/87,073/1,415 .. 15/158,999/2,113; facade 1/5,270/131 |
| stdio stl | 5 + facade | inferences 22/77,570/843; examples 11/29,102/548; schema 23/73,473/181; mutations 19/109,058/1,908 .. 22/146,463/2,316; viewer 7/32,423/523; facade 1/7,001/162 |
| space-space (pilot, unchanged) | 8 + facade | passes the gate as is |

Every crate of the six is inside the hard budget (32 files / 262,144 bytes / 4,000 lines); the test closures of binary snapshot (30 files), html mutations (29), ply inferences (29) and csv mutations (29) are above the 28-file target and below the hard limit; the facade roots are pure wiring (ply 143 lines, below the 150-line wiring budget). `lib-wiring-budget` passes after removing the stale ratchet pin of tsv (its root is now 119 lines). Owners scan (`owners-tools/scan.ts` from the repo root): 2417 packages, 2564 owners, `missing: []`. Taxonomy validators (`pw0-taxcheck.ts`): stdio 0 mount findings and 1 tree finding (`gif` standard `sqlite` without subsets, existing before this work), space 1 finding about a missing manifest at the plugin root (not an artifact I touched).

Not fixed, still applied by the old generator and failing the gate (nothing reverted, as decided):

| artifact | gate result of the applied state | what blocks a clean re-carve |
| --- | --- | --- |
| stdio md | 3 native / 5 test extra names | schema component 57 files lib (limit 32): lib code under `cfg(test)` calls `sqlite::snapshot::tests::MdReconstructTestScope`, which ties the binary snapshot pack to the sqlite tests; needs a source change (move the scope guard out of the test module) |
| stdio bcf | 3 / 4 | mutation family (15 files plus coherence with diff) and its tests exceed 32 files even with test edges ignored; needs a hand split of the mutation tests |
| stdio las | 7 / 9 | schema component 65 files lib; same family of cause |
| stdio obj | 6 / 8 | schema component 68 files lib |
| imperative procedure | baseline stops at `env!`; planner with the whole plugin archived reaches 1 native / 3 test extra names | one private `mutations` module reached through `component` from another crate; not yet traced |
| playbook | 16 native | root regions merge into one unit that cannot be hosted without a cycle |

### 16.5 Candidates re-planned with the final planner

deflate: schema crate 53 files / 286,432 bytes / 4,486 lines (blocked). wfc-3d: snapshot crate 106 files (blocked). wfc-grid2d: snapshot crate 68 files (blocked, was "clean" before the missing edges were added). wfc-2d: snapshot crate 79 files (blocked). space-home: plans into 8 crates inside the budget, layout differential 0 for lib and test, but the single-crate oracle reports 1 new test diagnostic (`unresolved import ..schema::mutations::register_s_home_mutation_descriptors` in a test-only path after the test rewrites); not applied. Correction of the earlier relay: space-home was never one test slice from clean; it needed the dependency fixes above.

### 16.6 Items of the coordinator relay

1. Asset, file-split and counter packages: not applied; they fall into the same budget blockers as above.
2. `carve-measure` re-rooting of a `#[path]` inline wrapper (sourcing-curation): fixed.
3. space-home: see 16.5.
4. bim planner: the 1,500 s hang was the pairwise reachability loop of the cycle merge (1,841 slices, quadratic with a recursive search per pair); it is replaced by Tarjan SCC. The run after the fix was stopped by me after 37 minutes wall (3.6 minutes CPU, niced under machine load 25): the early phases take 60 s (transform 11 s, slices 17 s, per-slice measures 29 s, edges 60 s), the rest is the packing loop, which re-measures the merged source closure of every candidate group (the test closure has 11,106 files, 7,980 embedded fixtures). bim was not planned to the end. Recommended fix before bim is tried: cache per-slice file lists and sizes and compute the group measure by summing them instead of re-reading the closure.

### 16.6a Rollout command sequence per package (for the parallel fan-out)

Run from the repository root, one package per agent, never two agents in one artifact. `<pkg>` is `✏️s/🔌️plugins/<plugin>/🗿️artifacts/<artifact>/📦️packages/🦀️rust`, `<art>` is its parent two levels up, `<tag>` a new short name, `<sbx>` a directory outside the repository (use the session scratchpad), `<gen>` the ticket `🗑️generated` folder.

1. Precondition: the artifact has no other uncommitted change: `git -c core.fsmonitor=false status --porcelain --untracked-files=all -- <art>` prints nothing (an artifact that was already carved by the old generator needs step 1b first).
   1b. Already carved by the old generator: `bun <gen>/pw0-revert.ts <old-tag> <art-without-repo-prefix-plugins>` where `<old-tag>` has `<gen>/pw0-backup-<old-tag>-plan.json` (or no plan file when the carve came from the CLI: then every changed path of the artifact is treated as the carve). It restores HEAD for every written path, deletes created files and removes the owner rows; it aborts and prints the foreign paths when the artifact has other changes against HEAD.
2. Plan and verify without writing: `bun ./📜️script.ts new crates <pkg> --verify --oracle --sandbox=<sbx>/<tag>` (about 3 minutes). Exit 0 and no `problem:` line are required: census within the hard budget, module trees equal, visibility 0, orphan 0, glue clean, unresolved crates 0, single-crate oracle `onlyCandidate` 0 and layout differential `onlyCombined` 0 for lib and test. Any other result: stop, record the first diagnostics, do not apply.
3. Apply: `bun ./📜️script.ts new crates <pkg> --backup=<gen>/pw0-backup-<tag>` (writes the crates, the facade, workspace members, router packages and the lock, and registers the owner rows; `<gen>/pw0-backup-<tag>-owner-rows.json` lists them).
4. Gate on the applied state: `bun ./📜️script.ts new crates <pkg> --check-applied --sandbox=<sbx>/<tag>-applied` must exit 0 (`onlyCombined` 0 for lib and test).
5. Repository checks: `bun test ./🧪️tests/📏️lib-wiring-budget/🟦️.ts` from `L/` (remove a ratchet pin whose root is now at most 150 lines, never add one), `bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/owners-tools/scan.ts` (expects `missing: []`), `bun <gen>/pw0-taxcheck.ts ✏️s/🔌️plugins/<plugin>` (mount findings 0).
6. Only then record the package in the ticket; the shared ticket is closed by the coordinator.

The scripted equivalent of steps 2 to 4 that was used for the six artifacts above is `bun <gen>/pw0-rollout.ts <pkg> <tag>` (it also prints the live census); the CLI path was exercised end to end only for `--check-applied` in this phase, so the first agent of a family should run step 2 through the CLI before applying.

State of the families on the date of this report:

- Ready: none un-carved. The six re-carved artifacts and the pilot already pass step 4.
- Re-carve needed (already carved, gate fails): stdio md, bcf, las, obj; imperative procedure; playbook. Each needs a source or generator change first (16.4 table); a plain re-run of step 2 fails for all six.
- Not carved, plan blocked by the correct dependency graph: stdio deflate, wfc 2d, grid2d, 3d, and every package of the earlier survey that has an over-budget schema family (the survey tables are older than the dependency fixes and must be re-run after them: `bun <gen>/pw0-survey3.ts <pkg...>`, about one minute per package).
- Not carved, plan fits but one test diagnostic: space-home (16.5).

### 16.7 Open risks

- The six artifacts of 16.4 that still fail the gate are compile-risky until re-carved or hand-fixed. The gate command above is the check to run after each change.
- Nothing carved has been compiled by cargo; the gate resolves names with rustc but does not type-check or link.
- The planner is now stricter than some applied layouts: re-planning csv from HEAD currently puts 33 test files into its mutations crate (limit 32), although the applied csv passes the gate.
- No ticket close or goal change was made. Scratch sandboxes of this phase were removed; the scripts and logs `pw0-*` stay in `🗑️generated`.

## Appendix: survey v3 table (67 packages)

| class | package | problems | blocker kinds | largest component (slices) | largest embedded B | largest file lines |
| --- | --- | --- | --- | --- | --- | --- |
| A | 📖️playbook/📖️playbook | 0 (-) | - | - | 20,508 | 820 |
| A | 🗄️stdio/🌐️html | 0 (-) | - | - | 8,937 | 677 |
| A | 🗄️stdio/📑️tsv | 0 (-) | - | - | 6,676 | 689 |
| A | 🗄️stdio/🔺️stl | 0 (-) | - | - | 15,400 | 526 |
| B | ✒️writer/✒️writer | 3 (wiring-lines 2, over-budget 1) | other+parent-child+coherence | 55 | 38,856 | 1,282 |
| B | 🀄️wfc/◻️2d | 2 (wiring-lines 1, over-budget 1) | mutation+other+coherence | 15 | 20,242 | 1,186 |
| B | 🀄️wfc/🔲️grid2d | 3 (wiring-lines 2, glue 1) | mutation+other+coherence | 15 | 15,184 | 954 |
| B | 🀄️wfc/🖼️bitmap | 4 (glue 1, wiring-lines 1, over-budget 2) | mutation+other+coherence | 17 | 11,738 | 1,132 |
| B | 🀄️wfc/🧊️3d | 3 (glue 1, wiring-lines 1, over-budget 1) | mutation+other+coherence | 16 | 18,101 | 1,073 |
| B | 🀄️wfc/🧱️grid3d | 3 (glue 1, wiring-lines 1, over-budget 1) | mutation+other+coherence | 16 | 20,419 | 935 |
| B | 🌊️flow/🌊️flow | 3 (wiring-lines 2, over-budget 1) | parent-child+coherence | 11 | 9,445 | 2,950 |
| B | 🌍️gis/🏔️gisterrain | 2 (wiring-lines 1, over-budget 1) | parent-child+coherence | 7 | 8,537 | 709 |
| B | 🌍️gis/🗺️gismap | 4 (wiring-lines 2, over-budget 2) | parent-child+coherence+mutation+other | 8/13 | 243,464 | 1,487 |
| B | 🌿️vcs/🌿️vcs | 2 (wiring-lines 1, over-budget 1) | other+parent-child+coherence | 34 | 9,545 | 1,154 |
| B | 🎞️animate/🎬️presentation | 4 (wiring-lines 2, over-budget 2) | other+parent-child+coherence | 24/7 | 12,239 | 2,425 |
| B | 🎥️shooting/🎥️shooting | 5 (wiring-lines 3, over-budget 2) | parent-child+coherence | 11/8/6 | 36,378 | 1,370 |
| B | 🎪️demonstrator/🎪️playground | 2 (over-budget 1, wiring-lines 1) | parent-child+coherence | 9 | 6,863 | 497 |
| B | 💡️reasoning/🔌️wires | 2 (wiring-lines 1, over-budget 1) | other+coherence | 32 | 5,624 | 886 |
| B | 📋️forms/📋️forms | 3 (over-budget 2, wiring-lines 1) | other+parent-child+coherence | 10/13 | 23,181 | 1,438 |
| B | 🔱️trinity/♻️rewriting | 3 (wiring-lines 2, over-budget 1) | other+parent-child+coherence | 38 | 22,371 | 1,134 |
| B | 🔱️trinity/🔌️jack | 2 (wiring-lines 1, over-budget 1) | mutation+other+coherence | 44 | 15,472 | 1,449 |
| B | 🕸️dag/🕸️dag | 2 (wiring-lines 1, over-budget 1) | mutation+other+coherence | 38 | 6,973 | 975 |
| B | 🗄️stdio/🎒️zip | 2 (wiring-lines 1, over-budget 1) | other+coherence | 13 | 23,860 | 1,324 |
| B | 🗄️stdio/🎨️svg | 3 (over-budget 2, wiring-lines 1) | other+coherence+mutation | 14/21 | 28,799 | 857 |
| B | 🗄️stdio/🎵️mp3 | 1 (over-budget 1) | other+parent-child+coherence | 17 | 193,275 | 660 |
| B | 🗄️stdio/📰️xml | 2 (wiring-lines 1, over-budget 1) | mutation+other+parent-child+coherence | 15/15 | 76,603 | 820 |
| B | 🗄️stdio/📷️png | 2 (over-budget 1, wiring-lines 1) | mutation+other+coherence | 28 | 27,587 | 1,449 |
| B | 🗄️stdio/🔊️wav | 1 (over-budget 1) | other+parent-child+coherence | 16 | 48,539 | 790 |
| B | 🗄️stdio/🔤️txt | 2 (over-budget 2) | other+parent-child+coherence+mutation | 9/15 | 31,864 | 528 |
| B | 🗄️stdio/🖋️dxf | 1 (over-budget 1) | coherence | 3 | 14,113 | 2,192 |
| B | 🗄️stdio/🖼️tiff | 2 (over-budget 1, wiring-lines 1) | mutation+other+coherence | 30 | 56,418 | 818 |
| B | 🗄️stdio/🗜️deflate | 1 (over-budget 1) | other+coherence | 11 | 9,318 | 1,855 |
| B | 🗄️stdio/🧊️gltf subset ♾️any schema+ | 1 (over-budget 1) | coherence | 2 | 3,689 | 1,049 |
| B | 🗄️stdio/🧊️gltf subset ♾️any schema+ | 2 (wiring-lines 1, over-budget 1) | coherence | 3 | 109,502 | 605 |
| B | 🗄️stdio/🧾️json | 3 (wiring-lines 2, over-budget 1) | mutation+other+coherence | 15/13 | 59,184 | 778 |
| B | 🗄️stdio/🧿️semio subset 🌊️flow | 1 (over-budget 1) | coherence | 4 | 46,009 | 605 |
| B | 🗄️stdio/🧿️semio subset 🎞️animation | 1 (over-budget 1) | coherence | 3 | 46,215 | 491 |
| B | 🗄️stdio/🧿️semio subset 🏛️model | 1 (over-budget 1) | coherence | 4 | 40,328 | 555 |
| B | 🗄️stdio/🧿️semio subset 📐️cad | 1 (over-budget 1) | coherence | 3 | 86,241 | 358 |
| B | 🗄️stdio/🧿️semio subset 📑️document | 1 (over-budget 1) | other+coherence | 9/3 | 101,419 | 1,031 |
| B | 🗄️stdio/🧿️semio subset 📦️object | 2 (wiring-lines 1, over-budget 1) | mutation+other+coherence | 13 | 25,101 | 319 |
| B | 🗄️stdio/🧿️semio subset 🕸️graph | 2 (wiring-lines 1, over-budget 1) | coherence | 3 | 23,800 | 558 |
| B | 🗄️stdio/🧿️semio subset 🖊️drawing | 2 (wiring-lines 1, over-budget 1) | coherence | 3 | 98,349 | 864 |
| B | 🗄️stdio/🧿️semio subset 🖼️image | 1 (over-budget 1) | coherence | 3 | 17,346 | 547 |
| B | 🗄️stdio/🧿️semio subset 🧰️kit | 2 (wiring-lines 1, over-budget 1) | other+coherence | 7/3 | 20,761 | 319 |
| B | 🗄️stdio/🪟️bmp | 1 (over-budget 1) | mutation+other+coherence | 22 | 6,131 | 503 |
| B | 🗒️note/🗒️note | 5 (wiring-lines 3, over-budget 2) | mutation+other+coherence | 15 | 37,374 | 1,495 |
| B | 🧩️puzzle/◻️2d | 5 (over-budget 2, wiring-lines 3) | other+parent-child+coherence | 37 | 0 | 0 |
| C4 | 🏛️architect/🏛️program | 2 (wiring-lines 1, over-budget 1) | other+parent-child+coherence | 55 | 375,218 | 1,962 |
| C4 | 🗄️stdio/🌦️epw | 1 (over-budget 1) | other+parent-child+coherence | 16 | 397,789 | 534 |
| C4 | 🗄️stdio/🎞️gif | 22 (coherence 18, over-budget 2, wiring-lines 2) | other | 3 | 21,033,376 | 914 |
| C4 | 🗄️stdio/🎥️mp4 | 2 (over-budget 2) | other+parent-child+coherence | 17 | 2,719,031 | 1,908 |
| C4 | 🗄️stdio/📸️jpg | 2 (over-budget 1, wiring-lines 1) | mutation+other+coherence | 22 | 483,496 | 1,555 |
| C4 | 🗄️stdio/📼️avi | 1 (over-budget 1) | other+parent-child+coherence | 16 | 766,968 | 1,336 |
| C4 | 🗄️stdio/🧊️gltf subset ♾️any/✏️editor | 1 (over-budget 1) | - | - | 284,436 | 478 |
| C4 | 🗄️stdio/🧊️gltf subset ♾️any schema+ | 1 (over-budget 1) | other+parent-child+coherence | 16 | 284,436 | 2,388 |
| C4 | 🗄️stdio/🧿️semio | 2 (over-budget 2) | mutation+other+coherence | 10 | 2,828,738 | 409 |
| C4 | 🗄️stdio/🧿️semio subset 📽️presentation | 1 (over-budget 1) | coherence | 3 | 315,532 | 1,071 |
| C4 | 🗄️stdio/🧿️semio subset 🔺️mesh | 2 (wiring-lines 1, over-budget 1) | coherence | 3 | 288,536 | 814 |
| C4 | 🗄️stdio/🧿️semio subset 🧊️brep | 2 (wiring-lines 1, over-budget 1) | other+parent-child+coherence | 20 | 282,543 | 599 |
| C5 | ➗️mathematical/➗️equation | 3 (wiring-lines 2, over-budget 1) | mutation+other+coherence+parent-child | 17/11 | 43,443 | 5,092 |
| C5 | 🎬️sequence/🎬️sequence | 2 (wiring-lines 1, over-budget 1) | other+coherence | 20 | 6,902 | 3,739 |
| E | ✏️s/🔌️plugins/🏙️bim/🏢️model/📦️packages/🦀️rust | n/a | Unsupported Rust compile expression at line 13: include_str | n/a | n/a | n/a |
| E | ✏️s/🔌️plugins/🗄️stdio/📖️pdf/📦️packages/🦀️rust | n/a | Generated Rust input requires compiler ownership: ✏️s/🔌️plu | n/a | n/a | n/a |
| E | ✏️s/🔌️plugins/🧩️puzzle/🖐️5d/📦️packages/🦀️rust | n/a | Missing Rust source input: ✏️s/🔌️plugins/🧩️puzzle/🗿️artif | n/a | n/a | n/a |
| E | ✏️s/🔌️plugins/🪐️space/🏠️home/📦️packages/🦀️rust | n/a | Generated Rust input requires compiler ownership: ✏️s/🔌️plu | n/a | n/a | n/a |
| E | ✏️s/🔌️plugins/🪵️sourcing/🗂️curation/📦️packages/🦀️rust | n/a | Missing Rust source input: ✏️s/🔌️plugins/🪵️sourcing/🗿️art | n/a | n/a | n/a |
