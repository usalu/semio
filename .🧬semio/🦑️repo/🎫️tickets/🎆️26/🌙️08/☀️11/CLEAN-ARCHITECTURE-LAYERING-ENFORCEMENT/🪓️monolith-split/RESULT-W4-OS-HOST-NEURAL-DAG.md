# Result: wave 4, OS-HOST-NEURAL-DAG (`semio-framework-os`, `-os-kernel-neural-engine`, `-artifact-infinite-dag`)

Executor: Sonnet 5.5, foreground, no sub-agents. Rules: `📓️EXECUTOR-RULES.md`, `📓️CARVING-RULES.md`, `📓️RESOLUTION-ORACLE.md`, `PRODUCER-CRATE-RECIPE.md`, `📓️WAVE4-BRIEF.md`. Nothing committed, staged, stashed or checked out. No ticket opened or closed (subagent rule).
`F` = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os`. Tools: `🪓️monolith-split/osnd-tools/` (`oracle.py` + `cfg-{dag,host,neural}.json`, `neural/gen.py`, `neural/tests_fix.py`, `neural/manifests.py`, `host/carve1..5.py`, `dag/carve1..3.py`, `registry_all.py`, `defscan*.py`, `depcheck.py`, `census-rows.ts`). Logs: `🗑️generated/osnd-*.log`.

## 0. Verdict

All three targets are now below the gate and inside the brief's target (3500 lines, 28 files, 230000 bytes) for the lib closure and the cfg-test closure, with ten new producer crates. The real compile did not run: `bun repo cargo ... check -p semio-framework-os-kernel-neural-engine-model` fails with `Finite original maintenance authority required` (start and end of work, `osnd-cargo-baseline-dag.log`, `osnd-cargo-check-model-end.log`), and the Nx route `bun repo run @semio-tech/neural-engine-model-rs:check ...` fails with `Active graph readers retain required Nx pool headroom` (`osnd-nx-check-model-end.log`). So no type-check or unit test of any new crate has been executed; verification is the patched full-resolution differential (union of the new crates as modules of one crate, see section 4) plus static checks.

## 1. Census (canonical `readRustSourceClosureV1`, host cfg; test = cfg `test`; limits 4000 lines / 32 files / 262144 bytes)

| Crate | Before lib: files / bytes / lines | After lib | After test closure |
| --- | --- | --- | --- |
| `semio-framework-os` | 14 / 306,233 / 5,486 (test 35 / 529,485 / 8,519) | 7 / 117,585 / 1,972 | 11 / 214,672 / 3,147 |
| `-os-host-codec-abi` (new) | | 2 / 43,081 / 1,013 | 4 / 68,438 / 1,457 |
| `-os-host-instance` (new) | | 1 / 21,832 / 453 | 2 / 25,665 / 526 |
| `-os-host-registry` (new) | | 1 / 27,464 / 531 | 3 / 32,271 / 614 |
| `-os-host-workflow` (new) | | 1 / 52,917 / 820 | 9 / 88,844 / 1,213 |
| `-os-host-media-raster` (new) | | 1 / 12,755 / 212 | 2 / 16,136 / 265 |
| `-os-host-space` (new) | | 1 / 30,970 / 476 | 4 / 83,360 / 1,295 |
| `semio-framework-os-kernel-neural-engine` | 18 / 350,084 / 4,420 (test 47 / 565,602 / 6,586) | 2 / 28,417 / 537 | 8 / 120,141 / 1,892 |
| `-neural-engine-model` (new) | | 7 / 136,603 / 2,080 | 16 / 180,177 / 2,545 |
| `-neural-engine-registry` (new) | | 3 / 68,872 / 881 | 8 / 93,295 / 1,034 |
| `-neural-engine-budgeted` (new) | | 7 / 120,035 / 948 | 18 / 177,520 / 1,162 |
| `semio-framework-artifact-infinite-dag` | 36 / 150,611 / 2,417 (test 90 / 277,904 / 3,625) | 4 / 13,815 / 164 | 17 / 130,752 / 1,349 |
| `-artifact-infinite-dag-vcs` (new) | | 28 / 136,666 / 2,251 | 28 / 136,666 / 2,251 |

wasm32-unknown-unknown and wasip2 rows are in `🗑️generated/osnd-census-after.log` (all `ok`; the dag parent grows by the 1-file browser bridge, host raster loses its native engines on wasip2). The canonical `census.ts` reports `overLimit false` for every checked row (`osnd-census-canonical-after.log`); `closureCompilable` stays false only because of peer-owned oversized dependencies (`semio-framework`, `os-kernel`, `replication`, `plugin`, `ui`).

## 2. Crate graphs (downward only)

- Host: `instance` and `codec-abi` (leaves) <- `registry` (adds the immediate kernel-future bridge `resolve_kernel_future`, moved out of the plugin host) <- `workflow`, `space` <- `media-raster` (needs workflow) <- parent `semio-framework-os` (plugin host, document backbone, simple SVG builders, persistence, activation, lib root glue, `test_identity!`).
- Neural: `model` (dictionary, schema, tree, operator vocabulary, hashing, validation laws, retirement, cold owners, lookup/projection/native/failure, `testkit` feature) <- `registry` (Registry, SchemaComponent, shared registry ownership, admission, output cursor) <- `budgeted` (budgeted evaluation state and wave, topology, input merge, tree copy, cache insert, `NeuralCache`, eval budget types) <- parent (Evaluator, facade, unit/owned-control/canonical tests, `registry_admission` integration test).
- Dag: `vcs` (snapshot, patches, delta algebra, diff, `DagMutation` with all 14 leaves, canonical wire, text/binary/SQLite codecs) <- parent (store aliases and `create_dag_store`, retained producer, demo, browser bridge, all tests).

Why dag is not split further: `impl ArtifactStoreSnapshot for DagSnapshot` names `DagMutation` (`bare_artifact_codec::<Self, DagMutation>`) while `DagMutation` leaves name `DagSnapshot`/`DagDiff`, and the orphan rule forces every foreign-trait impl (pack, dsl, SQLite) next to `DagSnapshot`. Snapshot, diff, mutations and their codecs are one cycle and one crate; it sits at exactly 28 files.

Facades (explicit re-exports, no globs): the three parents keep their names and lib names. Host: `pub use semio_framework_os_host_{codec_abi,instance,media_raster,registry,workflow,space} as {codec_abi,instance,media_export_raster,registry,workflow,space};` so `semio_framework_os::workflow::...` (17 consumers) and the 89 consumer files are unchanged. Neural: explicit `pub use` item lists per child plus the module names `retirement`, `cold`, `topology`, `input`, `cache_insert`, `tree_copy` and `registry`/`evaluation` aliases (so `neural_engine::retirement::RetainedDictionaryInput` and `evaluation::wave::PendingWaveCursor` keep resolving). Dag: one explicit `pub use semio_framework_artifact_infinite_dag_vcs::{36 names}`; the public `io` and `mutations` modules are no longer exported (no consumer used them; searched all `*.rs`).

## 3. Changes beyond wiring

Host: `resolve_kernel_future` moved to `-os-host-registry` as `pub fn` (registry/workflow/backbone/host/space import it); registry imports `S_SPACE_SCHEMA`, `S_COLLECTION_SCHEMA`, `S_WORKFLOW_SCHEMA` from the artifact crates; `instance` imports `WorkflowNode` from the artifact crate; workflow handler registries (`export_handlers`, `import_handlers`, `os_media_handler_key`, the two handler type aliases) widened to `pub`; the three registrar tests (`svg_path_extraction_preserves_transformed_geometry`, mesh exporter/importer) moved from `workflow-unit` to `media-export-raster-unit` (new module `registrar_tests`, include gate widened to all `cfg(test)`); the stale codec source-text test (`pub mod codec_abi {` no longer exists) now reads the codec file; parent manifest loses resvg/tiny-skia/usvg/pixels/png (moved with raster, **fulfilling the SVG-rasterizer item**), `artifact-workflow-workflow` and `semio-framework-plugin` (only moved tests used it). TS readers updated: hub stdio script (`pub use ... as module` assertions), hub space script (law now `registrar_tests::svg_path_...` in package `semio-framework-os-host-media-raster`), host script (exact laws split over workflow and registry packages, stale source read re-pointed to the workflow file), root `📜️script.ts` allowlist `lineHint` 1359 to 1331. Manifest invariants the stdio script asserts (no `[features]`, `semio-framework-os-kernel` with `sync` in the non-p2 target table) are kept.
Neural: `EvalChannels`, `VALUE_TYPE_*`, validation and hashing fns live in `model`; `Dictionary::pairs()` accessor added (the budgeted input merge read the private field); widened to `pub`: `contract_channel`, `derive_channel_names`, `operator_signature`, `validate_neuron_inputs`, `validate_operator_outputs`, `budgeted_remaining`, `eval_error_dictionary`, `evaluate_cached_output`, `waits_on_parked`. Retirement macros for `OperatorImpl`/`OperatorRecord`/`Registry` and `ColdRetire` impls for `OperatorRecord`/`Registry`/`BudgetedEval`/`NeuralCache` moved to the crates that own those types (orphan rule). The observing global allocator and `observe_ownership` moved from the registry tests into `model::testkit` (feature `testkit`); each test unit registers it once, the cache retirement tests moved to the budgeted crate. `wasm-prototype` hot-package profile (`opt-level = 2`) added for the three children in all 144 manifests that carry the neural-engine override, and the policy fixture `iterateHotPackages` extended (the root-manifest policy test compares them exactly). Package script `test` now runs the four neural packages bottom-up.
Dag: 14 per-leaf stub tests merged into one `🌿️vcs/🧪️tests/🔬️leaf-contracts` test mounted by the parent; the 14 witnesses and 14 descriptors are read at run time (`read_vcs_fixture`) to keep the parent test closure at 17 files; `dag_index_from_wire` widened to `pub`; `default_dag_document` moved to the parent; the four `io` assembly files were removed and the six representation files are mounted directly by the vcs root (inline `mod` blocks with `..` paths do not work, rustc needs the directories to exist; verified with a scratch crate).

## 4. Verification (actual results)

1. Patched full-resolution differential (`oracle.py`, `rustc +nightly-2026-07-20 --emit=metadata -o /dev/null`, extern crates removed and external globs neutralised identically; baseline = snapshot of the original, split = every new crate mounted as a module of its parent, `crate::` rewritten, `pub(crate)` to `pub(in crate::child)`; external-missing-crate errors compared as per-crate counts). Native, `--test`, wasm32-unknown-unknown (+test), wasm32-wasip2 (+test), per target:
   - host: 0 new errors in all 6 configurations; only difference is the removed `workflow_kernel` import (baseline-only).
   - neural: 0 new in native/wasm/p2; in the three test configurations 1 new error, "cannot define multiple global allocators", which is a union artifact (four crates, four test binaries, each registers the kit once).
   - dag: only 1 (native) / 2 (test) new errors: `unresolved import crate::ActorId` (cascade of the second unresolved `pub use semio_framework_os_kernel::{...}`) and `CARGO_MANIFEST_DIR` (cargo-only env of the new run-time fixture reader).
   Logs: `osnd-oracle-{host,neural,dag}.log`.
2. Sensitivity: seeded faults were caught: private `register_app_io` and `export_handlers` (E0603 in host), private `validate_neuron_inputs` (E0603 in neural), private `dag_index_to_wire` (E0603 in dag). The first run of each target also caught real faults that were fixed (mangled `space::S_SPACE_SCHEMA` replacement, double `pub(in ...)` rewrite in the tool, missing imports in moved tests, missing `pub` on re-exported header uses).
3. Definition scan (`defscan.py`, textual: every non-pub item/field/method of a lower crate mentioned by a higher crate): found one real violation (`Dictionary.pairs` read by the budgeted input merge), fixed with the accessor; the remaining hits are name collisions of private internals (checked by reading: `Registry` fields, `TreeSnapshot` fields, cursor helpers).
4. Dependency completeness (`depcheck.py`): no missing idents for any new crate or parent; extern-crate aliases (`dsl/protocol/store/vcs/value_derive`) added where code uses them (workflow, space, dag-vcs); unused-declared lists only reflect pre-existing declarations kept on purpose (dag-vcs derive support crates).
5. rustfmt parse of all 74 changed `.rs` files: 0 failures. Path/brace/region check (`verify_paths.py`): 96 path literals, 0 missing, no imbalance, longest package path far below 240 bytes. Losslessness (neural, line multiset vs snapshot): only intended edits.
6. Registries: 10 workspace members (byte-sorted), 10 owner rows, 60 launch routes in `.vscode/launch.json` and the same 60 in `.vscode/🧩️launch.seed.jsonc` (orders `900.2NNxx`/`922.2NNxx`/`903.2NNxx`, no clash, unique commands), all manifests parse (`toml`), workspace/lib/dependency paths resolve, project names unique. Both JSON files parse; `📋️build-owners.json` re-serialises byte-identically.
7. TS: `bun test` of the neural type test passes; the three edited TS scripts and `📜️script.ts` parse (`Bun.Transpiler`). No Rust test and no gate script ran (launcher down).

## 5. Recorded, not moved (wrong direction)

- `seed_builtin_artifact_kinds` (registry crate) still hard-wires `space`, `workflow`, `collection`, `parameter.value`, `data` kinds and depends on the artifact crates. Moving it to the owners needs a lower descriptor type than `OsArtifactDescriptor`/`OsMediaCapability` (the entries carry media class/form and `MeshOnly` capability that `ArtifactKindSpec` does not), so it was left.
- Product defaults `SPACE_FOLDER_DOCUMENT_ID = "studio"` and `OS_SPACE_BACKBONE_URI_PREFIX = "space://"` stay in host/backbone.
- `registry` (host) still depends on `ui_wgpu` for `SurfaceKind`.
- Pre-existing, untouched: `use ::graph::manifest::PropertyBag` in the dag vcs root is not resolvable by the old manifest either (no `graph` dependency key); the new tests import `semio_framework_graph_manifest` instead.

## 6. Residual risks / next steps

1. First real compile: expect at worst a missed `pub` on a field or method read across a new crate boundary (the textual scan cannot see macro token trees), an unused-import warning from token-based import lists, and `private_interfaces` warnings. Typeck-level trait scope and derive expansions (`DslArtifact`, `Mutations`, `RetireOwned`) were not exercised.
2. Lock files (`Cargo.lock` root, hub, teaching, plugin workspaces) are not refreshed; `update --workspace --offline` must run once the launcher works. Expected: 10 new packages, edges from the three parents, `semio-framework-os` loses resvg/tiny-skia/usvg/pixels/png/plugin/workflow-artifact edges.
3. `🔒️dependencies.json` (dependency ratchet baseline) lists the old host manifest as user of png/resvg/tiny-skia/usvg and has no rows for the new manifests; the coordinating session must run `verify dependencies write-baseline`.
4. `Cargo.toml` profile blocks in 144 workspaces and the policy fixture were edited mechanically; run the `WASI component profile policy` test.
5. Test layout changes that need a first run: neural cache retirement tests now in the budgeted crate (fixture path `../../🧵️retirement/🧪️tests/🧫️fixtures/...`), allocator registration once per test unit, dag leaf contract tests (14 names changed to `direct_leaf_contract_<snake>`), run-time fixture reads (dag, `CARGO_MANIFEST_DIR`).
6. Taxonomy: the new folders `🧱️model`, `🌿️vcs/📦️packages`, `🪐️space/📦️packages`, `🧱️model/🧪️tests/🧰️testkit` were not registered or checked with `verify taxonomy` (launcher down); neural children have no memberNames list, the others reuse existing package/test folder names.
7. The dag `vcs` crate is exactly at the 28-file target (new mutation leaves each add one file).

## 7. Files

Created: `{📡️codec-abi,🧱️instance,🗂️registry,🔀️workflow,🖼️media-export-raster}/📦️packages/🦀️rust/{Cargo.toml,📋️project.json}` under `F/🖥️host`; `F/🔨️modules/🪐️space/📦️packages/🦀️rust/*`; `F/🔨️modules/🧠️neural/⚙️engine/{🧱️model/🦀️.rs,🧱️model/🧪️tests/🧰️testkit/🦀️.rs,🧱️model/📦️packages/🦀️rust/*,📔️registry/📦️packages/🦀️rust/*,⏱️evaluation/📦️packages/🦀️rust/*}`; `F/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/{📦️packages/🦀️rust/*,🧪️tests/🔬️leaf-contracts/🦀️.rs}`.
Edited: host root and package glue (`🦀️.rs`, `📦️packages/🦀️rust/{🦀️.rs,Cargo.toml,📜️script.ts}`), the host modules and tests named in section 3, space module and unit test; neural root/registry/evaluation/retirement/cold/wave/input files, the nine test files, `📦️packages/🦀️rust/{Cargo.toml,📜️script.ts}`; dag roots, 4 io files, 14 leaves (stub tests removed, 14 stub test files deleted), the dag vcs/direct tests, parent and child manifests; repo-level: `Cargo.toml`, `📋️build-owners.json`, `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`, `📜️script.ts`, the wasm profile fixture, 144 plugin/hub/teaching/framework `Cargo.toml` profile blocks, the hub stdio and space scripts.
Deleted: dag `🚪️io/{🦀️,💾️binary/🦀️,📝️text/🦀️,🪶️sqlite/🦀️}.rs` and the 14 `🧬️mutations/*/🧪️tests/🔬️unit/🦀️.rs`.
