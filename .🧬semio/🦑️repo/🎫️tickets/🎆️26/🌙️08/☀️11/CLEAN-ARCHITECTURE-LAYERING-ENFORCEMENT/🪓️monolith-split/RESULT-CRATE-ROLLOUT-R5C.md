# Result: CRATE-ROLLOUT-R5C (prefix `r5c`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/r5c-tools/` (`step.sh <tag> <cmd...>`, `r5c-census.ts`). Logs: `F/🗑️generated/r5c-*`. Sandboxes: `/Users/ueli/.cache/r5c/`. Started 2026-10-17 09:00.

## 0. Verdict

**DONE (static; no real compile: no `COMPILE ADMITTED` line in `F/📓️FLEET.md`).** The three oversized stdio glTF crates are carved: every one of the 52 crates under the gltf artifact is inside the compiler source-boundary gate for lib and test closure (`r5c-census-after1.log`: `crates 52 over-or-error 0`). `bun ./📜️script.ts new crates ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/📦️packages/🦀️rust --check-applied --samples=60` (final run `r5c-ca2.log`, base HEAD): exit 0, native 5049/5263 and test 5906/6100, `onlyCombined` 0 / 0, `admission` [], `shared` [], `sharedForeign` [].

| Package | Verdict | Crates (facade included) | Verify (`--verify --oracle`) |
| --- | --- | --- | --- |
| gltf-model (`🧬️schema`) | DONE | 8 | exit 0: module tree 0, visibility 0, orphan 0, glue 0, oracle 0/0 (native, test), layout 0/0 |
| gltf-mutations (`🧬️schema/🧬️mutations`) | DONE | 13 | exit 0, same zeros |
| gltf-inferences (`🧬️schema/💡️inferences`) | DONE | 22 | exit 0, same zeros |

Other gates: owners scan (`owners-tools/scan.ts`): no missing row under `🧊️gltf`; `lib-wiring-budget`: no gltf root listed (the three failures are foreign: svg, equation, brep, graph); taxonomy (`pw0-taxcheck.ts ✏️s/🔌️plugins/🗄️stdio`): 0 mount findings, 1 pre-existing tree finding (gif); both launch files parse (no launch route was needed: no new command); crate-tree, mutation-leaf-generics and lift-kids tests: 94 pass, 0 fail; cross-crate `cfg(test)` scan (section 3): 0 real pairs in the three packages.

Open items for the compile queue (nothing compiled): `bun repo cargo --store=shared --configuration=prototype -- check -p <crate> --tests --message-format=short` for the 43 new crates (names in `📋️build-owners.json` rows `semio-s-artifact-stdio-gltf-{model,mutations,inferences}-*`), plus a `--features testing` check of `semio-s-artifact-stdio-gltf-model`; risks: (1) the facade of the model was edited by hand (section 2.4), (2) the corpus crate `-mutations-mut-any-4` got `serde_json` as a normal dependency, (3) the `|indicators| leaf::encode_result(&indicators.family.field)` closures in the inference service table are const-context closure-to-fn coercions (valid Rust, not compiled here), (4) Oracle blind spot (brief 12:10): the packages keep no root file module except the `cfg(test)` `facet_conformance_laws` in the mutations facade; its `crate::` paths resolve through facade re-exports and `--check-applied` reported nothing.

## 1. Baseline census (cgc7 reader, host cfg, `F/🗑️generated/r5c-census-base0.tsv`, scope = the gltf artifact)

| crate | lib files/bytes/lines | test closure files/bytes/lines |
| --- | --- | --- |
| semio-s-artifact-stdio-gltf-model | 69 / 612,387 / 8,194 | 82 / 736,130 / 9,671 |
| semio-s-artifact-stdio-gltf-mutations | 131 / 732,261 / 12,910 | 303 / 1,128,307 / 15,391 |
| semio-s-artifact-stdio-gltf-inferences | 365 / 523,640 / 6,655 | 447 / 564,930 / 7,611 |
| (inside the gate already) editor 16/76,793/1,331 (test 23), facade 4/108,041/1,154 (test 11/128,899/1,515), exports-* 5..9 files, test-oracle 7/144,946/2,570 (test 8/167,218/2,982) | | |

Precondition: the artifact has staged (index) changes by CENSUS4-PKG-22 and the generator split (oracles, binary/text wire, geometry-kernel, diff, editor, generator recipes) and one unstaged edit (exports-rust `DIFF`), no untracked files. The three target packages are hand-built crates of an already multi-crate artifact (workspace members), so the planner takes the package dir of each (artifact dir from `[package] workspace`); `--check-applied` takes the facade package dir.

## 2. Per package

### 2.0 Common findings (all three crates)

* `bun ./📜️script.ts new crates <pkg>` takes the hand-built sub-package dir (the planner reads the artifact workspace from `[package] workspace`); `--check-applied` must be given the facade package dir `…/🧊️gltf/📦️packages/🦀️rust`.
* New generator capability (library authoring module, `L/🏗️authoring/📦️crate-tree/🪜️lift-kids/🟦️.ts`, flag `--lift=<module-path>[::*],…`, option `PlanOptions.lift`): child modules of the named parents are lifted out of their host before slicing. Every `super::` chain that climbs above the child becomes an absolute `crate::` path (`absolutizeBeyond`), private host imports are carried and private host items widened when the host is a file. The slicer then places the child alone instead of unioning it with its host (`buildSlices` unions every kid of an inline wrapper once one kid reaches `super::`). Test: see section 6 (to be written).

### 2.3 gltf-inferences (365 files, 523,640 bytes)

Causes found (planner `--explain`, per-slice `F3_SLICES=1`):

1. The aggregate `gltf_artifact_inference_descriptors()` embedded 335 `include_str!` companion documents (67 leaves x rust/ts/graphql/json/proto = 337 files, 384 KB) in one file, so the aggregate alone broke the file limit.
2. All 67 leaves import `GltfEntityIndicators`, `GltfInferenceLeaf`, `GltfInferenceLeafDescriptor` and `GLTF_GEOMETRY_READS` from the aggregate, while the aggregate mounts the families and leaves: one strongly connected component.
3. Every leaf and family reaches `super::super::modules`/`super::geometry_core`, so the slicer unions the whole tree.
4. `pub(crate)` items used across slices force coherence merges.
5. Two assembly leaves (`repetition-ratio`, `modularity-ratio`) and the symmetry family root depend on the aggregate type `GltfPartInference`.

Source changes (all scripted by `F/r5c-tools/inf-refactor.py` for the 67 leaves; originals in `F/🗑️generated/r5c-orig/inferences/`):

* `GltfInferenceLeaf`, `GltfInferenceLeafDescriptor`, `GLTF_GEOMETRY_READS` moved to `🔨️modules/🧾️measurement-contracts` (region `🧭️LeafContract`).
* Leaf `encode_result` now takes the measure (`&GltfMeasure<T>`); the aggregate service table projects it (`encode: |indicators| leaf::encode_result(&indicators.<family>.<field>)`). Leaves no longer name `GltfEntityIndicators`.
* Each leaf owns its registry descriptor (`inference_descriptor()` in its own file with its own `include_str!` companions); the aggregate lists them in the original order.
* `pub(crate)` widened to `pub` in the inference tree and its shared modules (51 files).
* `symmetry::assembly_ratios`, `GltfSymmetryInference::infer_assembly` and the two `from_assembly` leaf functions moved into `🏗️dag-assembly` as private functions (`assembly_ratios`, `infer_symmetry_assembly`), the only user.

### 2.1 Generator changes (library authoring module `L/🏗️authoring/📦️crate-tree/`, tests in `L/🧪️tests/🪜️lift-kids/🟦️.ts`, 8 tests pass)

1. `🪜️lift-kids` (new) and flag `--lift=<module path>[::*],…` / `PlanOptions.lift`: see section 2.0.
2. `🗺️plan`: a carved crate was placed in `<lca>/📦️packages/🦀️rust`, which is the directory of the hand-built package being carved whenever a slice's anchor lives next to it (all three gltf packages); the crate overwrote the facade. `usedDirs` now starts with the package's own directory and a collision falls back to `<dir>/🧩️<label>`. Test: `never places a carved crate in the directory of the package it carves`.
3. `🧬️mutation-families`: a `MutationLeaf` payload may be an `enum` (the glTF leaves are `enum XMutation { Apply(Payload) }`); `payloadDefinition` only matched `pub struct`, so 0 of 120 leaves converted. Now 120/120. Test: `converts a mutation leaf whose payload is an enum of phases`.
4. `🧱️decl-tree` `buildSlices`: a test module that names the aggregate was detached only when its parent is an inline wrapper, so the leaf tests of every file-hosted leaf stayed in the leaf slice and pulled the leaf back into the aggregate crate (ring of all leaves). It now falls through to the common independence check. Test: `detaches a test that names the aggregate and needs nothing from its host`.
5. `🧪️test-closure` `absolutizeSuperPaths`: drops the private `::component` segment from the paths it writes (a detached test's `super::super::component::x` kept the segment and was refused as `private component module`). Test: `drops the private component segment`.

Open generator limitation (not fixed): `hostPrivate` does not see private glob imports of the host (`use super::super::modules::{measurement_contracts::*}`), so a test that reaches those names through `use super::*` is detached without them. Worked around in the one affected test (`🔨️geometry-kernel/🧪️tests/🔬️unit`) by naming the imports explicitly.

### 2.2 gltf-mutations (131 files, 732,261 bytes)

Causes: the converter (`--mutation-families`) refused all 120 leaves because the glTF leaves are `enum XMutation { Apply(Payload) }` (generator fix 3 above); the leaf tests stayed with their file-hosted leaves (fix 4); every leaf and the aggregate were united by `super::` (lift); `pub(crate)` `rejection_outcome`, `rejection_outcome_code` and the corpus helper `assert_case` (widened to `pub`).
Source changes: `🔨️modules/🧬️mutation-support/📚️top-level` (`rejection_outcome*` pub), and the corpus module `🧪️tests/🧪️fixture-corpus` (see section 3). Applied with `--lift=schema::mutations` and mutation families chosen by `auto`: 12 crates (aggregate, three leaf-family crates of 27 slices each, the fixture-corpus crate `-mut-any-4`, six case crates, the create-scene support crate).

### 3. Cross-crate `cfg(test)` items (coordinator rule 11:30)

* Scanner `F/r5c-tools/cfg-test-cross.ts` (planned crates) and `F/r5c-tools/cfg-test-cross-applied.ts` (applied crates): for every crate, every `cfg(test)` item or `pub` item of a test-only file, and the identifiers of every other crate's test closure. Results: inferences 19 applied crates, 0 pairs; mutations plan before the fix 12 pairs (all `fixture_corpus_tests`/`assert_case`), after 0; model plan 0.
* mutations: `assert_case` lived in `#[cfg(test)] pub mod fixture_corpus_tests` of the aggregate and was called by 120 case crates. The module is now a plain mount in the package nest (`schema::mutations::fixture_corpus_tests`, no cfg), hosted by its own crate `semio-s-artifact-stdio-gltf-mutations-mut-any-4`, a normal dependency of the case crates; its `#[test]` laws still run in that crate. `serde_json` moved from `[dev-dependencies]` to `[dependencies]` of that crate (hand edit of the generated manifest, description says test-support).
* model: `ordered_attr_map` (serde oracle, `cfg(any(test, feature = "testing"))`) was first extracted into its own file and the planner put it in its own crate; the module is inline in the snapshot file again so that it stays with the `serde(with = ...)` derives that name it.
* model feature `testing`: the package declared `testing = []` and the generator dropped it (`unsupported` list, never reported), so a carved model would have lost every `cfg(feature = "testing")` serde oracle for the dependents' tests. Generator fix: `forwardableFeatures`/`forwardFeatures` in `L/🏗️authoring/📦️crate-tree/🟦️.ts`: every carved crate declares each empty feature of the package and forwards it to the carved crates it depends on, each crate lists its siblings as `[dev-dependencies]` with `features = ["testing"]` (a case crate's tests need the oracles of the crates under test), and the package manifest forwards the feature to all carved crates. Test: `declares an empty feature of the carved package in every carved crate and forwards it from the package`.

### 2.4 gltf-model (69 files, 612,387 bytes; binding limit bytes, then files)

Findings (planner `--explain`): (1) the snapshot cluster (type + sqlite + pack + text codecs, 38 files / 272,872 bytes) is one coherence group because `GltfSnapshot` carries the foreign-trait impls of three codecs; (2) `schema::diff` and the two wire crates form a ring through `GltfDiff` (the type, its two codec roots and the wire that the roots call); (3) `schema::snapshot` named `io::text::snapshot::ordered_attr_map` from a serde attribute, tying schema to io::text.

Source changes (originals of every edited file in `F/🗑️generated/r5c-orig/model*`):

* `GltfSnapshot` + `Default` + `STDIO_GLTF_DOCUMENT_SCHEMA` moved to `🧬️schema/📸️snapshot/🧳️snapshot-root/🦀️.rs`, mounted as its own package-nest module `schema::snapshot_root`; the three snapshot-root codecs (`ArtifactPack`/controlled record/native encode: `📦️pack/🧳️snapshot-pack`; text DSL: `📝️text/📸️snapshot/🧳️snapshot-dsl`; sqlite) mount under `io::snapshot_codecs` / `io::sqlite` so no file of the element crate mounts a file of the snapshot-root crate. The encoding helpers (`emit`, `put`, `build`, `list`) and the pack records became `pub`. Internal importers (11 files) name `snapshot_root::GltfSnapshot`.
* `GltfDiff` + `demo_diff_cases` moved to its own nest module `schema::diff_aggregate` (the file `🧊️gltf-diff-aggregate` keeps its place); its two uses of the `absorb_slot!` macro became the local `absorb_collection!`; `print_gltf_diff`/`parse_gltf_diff` moved from the text wire into the text diff root (the only text-wire code that named `GltfDiff`); the binary wire only named it in a comment. `deserialize_double_option` and `IndexedDiffParts` are `pub`.
* serde oracles `serde_oracle`, `semantic_cache_codec` of the text snapshot moved from inline modules into files (inactive bytes leave the lib closure: 9.5 KB); `ordered_attr_map` stays inline in the schema snapshot file.
* sqlite root: `use backing::Read;` is absolute and `backing`/`Read` are `pub` (a detached test carried the relative import); sqlite tests import the snapshot names explicitly (the generator limitation of section 2.1).
* Facade `🧬️schema/📦️packages/🦀️rust/🦀️.rs` (generated, then edited by hand): `schema::snapshot` and `schema::diff` are inline modules that glob the element crate and explicitly re-export `GltfSnapshot`, `STDIO_GLTF_DOCUMENT_SCHEMA` (from the snapshot-root crate) and `GltfDiff`, `demo_diff_cases` (from the diff-aggregate crate), so the external paths of the mutations, inferences, editor and facade crates stay valid; `io::text` re-exports the diff-aggregate crate's `io::text` (the generator emitted merged modules for `binary` and `sqlite` but not for `text`, found by `--check-applied`).

## 4. Before and after (cgc7 reader, host cfg; `F/🗑️generated/r5c-census-base0.tsv` and `r5c-census-after1.tsv`)

| crate | before lib | before test | after (crates, worst lib files/bytes/lines; worst test) |
| --- | --- | --- | --- |
| gltf-model | 69 / 612,387 / 8,194 | 82 / 736,130 / 9,671 | 8 crates: v2-0 7/4,927/86, diff-aggregate 9/78,889/865, snapshot 10/146,551/2,110, snapshot-root 28/113,557/569, schema 10/116,322/2,306, wire 7/78,939/1,264, wire-2 7/84,512/1,284, facade 1/4,424/77; worst test 28/113,557 and 16/218,999/3,388 |
| gltf-mutations | 131 / 732,261 / 12,910 | 303 / 1,128,307 / 15,391 | 13 crates: aggregate 24/168,499/2,772, three leaf-family crates 28/168,268, 28/156,816, 28/142,470 (+28/149,614), corpus crate -mut-any-4 5/240,904/281, six case crates 1..2 files, facade 1/21,069/172; worst test 29/51,385/749 (case crates), 28/168,268 |
| gltf-inferences | 365 / 523,640 / 6,655 | 447 / 564,930 / 7,611 | 22 crates: leaf-family crates 22..27 files, 21,354..41,947 bytes, aggregate 10/87,828/1,187, geometry-core 5/47,520/1,113, facade 1/3,802/53; worst test 29/31,054/533 |

Gate: `r5c-census-after1.log`: `crates 52 over-or-error 0` (limits 32 files / 262,144 bytes / 4,000 lines, lib and test closure).

## 5. Commands run (results)

* Planner/explain: `new crates <pkg> --dry-run --explain` per package (logs `r5c-*-plan*.log`).
* Verify: inferences `r5c-inf-verify3.log` exit 0 (after two failed runs: carved crate overwrote the facade dir; one test lost glob names), mutations `r5c-mut-verify3.log` exit 0 (after: converter, tests detach, corpus), model `r5c-model-verify4.log` exit 0.
* Apply: `r5c-inf-apply.log` (21 crates, 205 files), `r5c-mut-apply.log` (12 crates, 308 files), `r5c-model-apply2.log` (7 crates, 48 files); backups `r5c-backup-{inferences,mutations,model}`; my pre-edit copies in `r5c-orig/`.
* Gate: `r5c-ca1.log` exit 1 (4 extras: corpus type names, facade `io::text`), fixed, `r5c-ca2.log` exit 0.

## 6. Files

Library authoring module (`L/🏗️authoring/`): new `📦️crate-tree/🪜️lift-kids/🟦️.ts`; edited `🗺️plan` (lift option, own-directory guard), `🧱️decl-tree` (aggregate-naming test detach), `🧪️test-closure` (`::component` drop), `🧬️mutation-families` (enum payload), `🧫️sandbox` (`placeless` label), `📝️render` (`featureRows`), `📦️crate-tree/🟦️.ts` (feature forwarding), `🎮️command` (`--lift`). New test file `L/🧪️tests/🪜️lift-kids/🟦️.ts` (10 tests). Tools `F/r5c-tools/` (census, files, slice-trace, plan-stack/deps/root, show-overlay/verify, cfg-test scanners, inf-refactor.py, extract-inline-mod.ts, move-tail.ts). Sources: the three gltf packages (per section 2) plus 43 new crates, `📋️build-owners.json` rows, `Cargo.toml`/`Cargo.lock` of the gltf workspace.
