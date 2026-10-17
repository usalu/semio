# Result: CFG-TEST-CROSS-CRATE (prefix `ctc`)

Executor: Sonnet 5.5, foreground, no sub-agents, no ticket tool, no modifying git command. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/ctc-tools/`. Logs, scan outputs, triage list: `F/🗑️generated/ctc-*`. Cut once by the usage limit (12:12 resume); state was rebuilt from `ctc-tools` and the scan outputs.

## 0. Verdict

**DONE (static evidence only, UNCOMPILED).** `📓️FLEET.md` has no `COMPILE ADMITTED`, so no `bun repo cargo` ran. The final scan (`ctc-scan11`) reports **0 untriaged hits in every finished crate** (2,922 crates scanned), after the coordinator declared R5A, R9B, R2, R8C and fb done. 9 hit groups remain, all in crates a running rollout slice still owns (r1, r6, r7, r8b); they are the hand-offs in section 5. 36 further groups in finished crates are triaged as name collisions, each with its reason in `F/🗑️generated/ctc-triage.tsv` (section 4).

What is not proved: nothing was compiled. The closure check (section 3.3) type-checks only without the dependency crates, so it catches missing local items and dev-only crate references, not type errors inside re-gated bodies. The first real `cargo test -p` of the touched crates is the proof. The commands are in section 7.

## 1. The defect class

An upstream crate's `#[cfg(test)]` items exist only while that crate is itself the test target. The union typeck oracle resolves them across the new crate boundaries; a real multi-crate build does not. Every `#[cfg(test)] pub fn/struct/enum/const/mod/macro_rules` (and every item inside a `#[cfg(test)] mod`) named from another crate, or from an integration target of the same crate, is a latent E0425/E0432/E0433. Feature-gated twins (`#[cfg(any(test, feature = "x"))]`) fail the same way when the consumer does not enable `x`.

## 2. Scanner (`F/ctc-tools/ctc-scan.ts`, lexer `ctc-lex.ts`)

- Enumeration: every `Cargo.toml` (git tracked plus untracked, 2,922 crates); path dependencies resolved through `[workspace.dependencies]`, renames and `package =`; targets lib/bin/test/bench/example with `required-features`.
- Item model: a Rust lexer and an item walker follow the real module tree (`mod x;`, `#[path]`, inline `mod` with `#[path = "."]`, and `include!("….rs")`, which hid the R2 facade tests until I added it). cfg expressions are evaluated three-valued: pure `test` gate, `any(test, feature = …)` (testkit-style) or not a gate. Items under private modules that nothing re-exports are skipped (they cannot be named).
- Reference model per consumer file: `use` leaves, qualified paths, module aliases (`use … as`, facade `pub use dep::*` modules), globs (exact and loose), `Type::method` paths, `.method()` calls (confirmed only when the owner type name occurs in the file), and an indirect pass for facade crates (`crate::schema::mutations::fixture` resolved through `pub use owner::…::*`). Declared names of the consumer, ungated pub names of the owner and ungated same-path items cancel false matches.
- Feature enabling: direct `features = […]` on any dependency table, forwarded features (`dep/feature`, followed transitively through the dependency graph), default features, and `required-features` of the consuming target. Such hits are "sanctioned", not defects.
- Sensitivity: `ctc-selftest.ts` builds a synthetic two-crate workspace twice. The clean tree gives 0 hits; the seeded tree (gated fn, struct, mod, file mod, `all(test, …)`, `any(test, feature)`, gated `pub use`, `macro_export` macro, gated method by path and by call, glob import, module alias, renamed dependency, integration test of the owner's own lib) gives 21 hits, with the decoys (ungated same-name fn, `cfg(not(test))`, private test mod) silent. Result: `SELFTEST PASS`. On the real tree the first scan of the R2 crates reported exactly the helpers `RESULT-CRATE-ROLLOUT-R2.md` predicted (`fixture`, `demo_mutation_cases`, `demo_*_snapshot`, `demo_diff_cases`, `sample_transform`).
- Helper tools: `ctc-report.ts` (groups, slice labels, `--triage=`), `ctc-ctx.ts`/`ctc-decl.ts` (source context), `ctc-gatename.ts`/`ctc-lines.ts`/`ctc-fileall.ts` (re-gating), `ctc-ownerfeat.ts`/`ctc-devfeat.ts`/`ctc-fix.sh` (manifests), `ctc-oracle.ts`/`ctc-oracle-batch.sh` (closure), `ctc-undeclared.ts` (gate names a feature the owner does not declare), `ctc-slices.ts` (slice map; updated after the coordinator's note: fb, R2, R8C and the finished R7 packages count as finished).

## 3. Findings and fixes

Scale at the first scan that followed `include!`: 119 hit groups (5,078 references) among finished owners plus 2,100 feature-gated references that were already sanctioned. Counts per area are in 3.4. The established precedent is `RESULT-RENDERER-TEST-BINS.md`: a non-default `testkit` feature (`#[cfg(any(test, feature = "testkit"))]` on the owner, enabled from the consumer's `[dev-dependencies]`). I used the owner's existing feature name where one existed (`artifact-app-testing`, `testing`, `protocol-laws`, `integration-fixtures`). A fixtures crate was not needed: every helper uses the owner's private items or sits in the owner's mounted test tree.

### 3.1 R2 (the example from the brief): model, mesh, drawing, brep io facades

The `io` facade crates mount their original tests (`derived-composition-unit`, `🧬️mutations/🧪️tests/↩️inverts`, `…/🔬️unit`) and name helpers through `crate::schema::{mutations,snapshot,diff}` globs that forward to the producer crates. Re-gated `any(test, feature = "testkit")`, `testkit = []` declared, feature enabled in the facade's dev-dependencies:

| owner crate | items | consumer |
| --- | --- | --- |
| `…-semio-model-mutations` | `fixture`, `demo_mutation_cases`, `sample_transform` | `…-model-io` |
| `…-semio-model-examples` | `demo_diff_cases`, `sweep_a/b`, `moved_transform` | `…-model-io` |
| `…-semio-model-snapshot` | `demo_semio_model_snapshot` | `…-model-io`, `…-model-schema` |
| `…-semio-mesh-mutations` / `-mesh-examples` | `fixture`, `demo_mutation_cases`, `demo_diff_cases`, `demo_snapshot_a/b` and their `use` lines | `…-mesh-io` |
| `…-semio-drawing-editor` / `-drawing-inferences` / `-drawing-snapshot` | `demo_mutation_cases`, `demo_diff_cases`, `sweep_a/b`, `transform`, `demo_drawing_snapshot` | `…-drawing-io`, `…-drawing-schema` |
| `…-semio-brep-viewer` / `-brep-examples` / `-brep-snapshot` | `fixture`, `demo_mutation_cases`, `demo_diff_cases`, `demo_brep_snapshot` | `…-brep-io`, `…-brep-viewer`, `…-brep-schema` |

(`…` = `semio-s-artifact-stdio`.)

### 3.2 Other areas fixed

- **Framework OS store and kernel-db:** `spr` (`protocol_laws`, plus `semio-framework-async` made an optional dependency of `protocol-laws`), `store-ledger` (`hot_path_census`), `store-sync` (`fixtures`), `store-sync-native` (18 `ArtifactActor` test hooks and `retained_turn_fixtures`; `testing` now pulls the optional dev-only crates it uses), `store-test-support` (`SerdeValue`), `store-one-item-publication`, `store-durable-group`, `directory-client`; kernel-db `io-core` (`DbIoPageWriter::seal/finish`), `wal-log` (`WalRecord::encode` and its two helpers), `artifact-history`, `io-driver`. The `kernel-db-cli` test named `db_wal::tests::committed_fixture_storage`, a private test module; it now names `db_wal::testkit::…`.
- **Framework plugin:** `plugin-instance-app` (`drive_framework_reserved_spawn_like_host`, `test_app`, `original_store_stage_tests`), `plugin-window-kits` (`AppActionRegistry::test_*`), `plugin-reactor-patches` (`PatchTracker::begin`, `commit`; `artifact-app-testing` now implies `original-actor-context-testing`, whose `semio-framework-trace` is optional), `plugin-child`, `plugin-testing-laws`, `plugin-instance-render`, `plugin-reactor-lifecycle`, `plugin` (enabled for fem-2d/3d editor and remodel tests), `job` (enabled for os-inference, plugin-operation, plugin-ownership, plugin-surface-fixture; the job crate's own `original-turn-failure` test now has `required-features = ["testkit"]` like its siblings).
- **Renderer and ui:** `engine-registry` (`EngineSurfaceRegistry::token/remove`, `EngineCanvasBuildContext::new/try_reserve_fresh_packet/publish_reserved`), `scene-state` (`close_all`), `engine-attach`, `program-bridge`, `kernel-protocol`, `kernel-retained`, `ui` (`ui_wgpu`/`testkit` on the four renderer test crates), `ui-retained-tree` (`accepted_layout_generation`), `ui-draw-contract`.
- **Hub:** `directory-service` (the whole `DirectoryWriterTestFence` mechanism, 14 gates), `directory-sqlite`, `hub-runtime`, `trusted-catalog` (`integration-fixtures`, `load_fixture`, guest cache), `route-models`.
- **Artifacts:** `procedural-2d/3d-model` (the `…SnapshotRead`/`…DiffRead` read guards, 13 gates each, 18 consumers), `procedural-2d/3d-host` (the publication-hostile authority and the fixture helpers), `procedural-2d-mutations` (two `pub(crate)` `central_apply` helpers that the host imported from outside the crate; now `pub`), `process3d-host` and `process3d-mutations`, `puzzle-3d-precompute` and `-precompute-geometry` (work meter and reservation hooks with their `cfg(not(test))` twins converted together), `puzzle-3d-editor-session`, `remodel-remodeling-model`, `draw-drawing-io/-scene/-editor-commands`, `fem-3d-editor-model-window`, `norm-en1994-mutations` (`demo_mutation_cases` moved out of the unit-test file into a new `🧰️testkit` module) and `norm-en1998-io`, `stdio-{gif,gif-2,docx,docx-strict,xlsx,pptx,ifc-4,ifc-2x3,step}-mutations` (`apply_*_mutation`), `stdio-gif-editor` (`register_sqlite_test_declaration`), `cad-cad` (test code used a `pub(crate)` alias of a private test module; now calls `semio_framework_io_sqlite_snapshot_testing_backing::measure` with a dev-dependency), `layout-layout-editor` and the new companion crates (see section 6). The draw scene `draw_scene_physical_*` macros moved from the tests file into a new `🧰️testkit` module.
- **Defects the scan exposed that are not cfg(test) re-gates:**
  - `flow-operators` used `feature = "protocol-laws"` without declaring it (now declared).
  - `puzzle-3d-mutations` carried a stray `#[cfg(test)]` that gated `COMPONENT_GRAMMAR_SEMIO`, which production `LanguageSpec` uses (removed).
  - `os-io` and a plugin test imported `ui_contract_validation::validate_snapshot` (a test-only fn, never used); `subset-validator` called it with the wrong arity where `V::validate_snapshot` was meant (all three fixed).
  - Features that enabled code using crates the owner has only as dev-dependencies (`spr`, `store-sync-native`, `mcp-capabilities`, `reactor-patches`) now declare them optional.

### 3.2b Late additions after R5A/R9B finished

- `stdio-dxf-snapshot` `TEST_ALLOCATION_OBSERVER` (a `#[global_allocator]` static) used by the `stdio-dxf` facade: the static, its `pub(crate) use … as test_allocation` line (which was ungated and named a dev-only crate, a latent E0432 in every normal build) and the snapshot crate's `pub use` are now `any(test, feature = "testkit")`; `testkit = ["dep:semio-framework-trace-memory-testing"]` with that crate as an optional dependency; the `stdio-dxf` facade enables it in `[dev-dependencies]`. Only the dxf facade's tests enable the feature, so no second global allocator is linked elsewhere.
- `architect-program-tests-2` `editor::architect::example`: false positive (section 4).

### 3.3 Verification run

- `ctc-selftest.ts`: PASS (clean 0, seeded 21 hits, no decoy).
- Full scans 1 to 11; final `ctc-scan11` with `--triage`: 0 untriaged hits in finished crates, 9 groups in slice crates.
- `ctc-undeclared.ts`: no owner names a feature it does not declare (only the ticket crate `document-descriptor-rust-oracle`, outside the tree).
- Every `Cargo.toml` parses (2,914 manifests); every `dep:` feature entry refers to an optional dependency.
- Closure check `ctc-oracle.ts`: `rustc +nightly-2026-07-20 --emit=metadata -o /dev/null` on the lib root with and without `--cfg feature="x"`, diffing error sets and checking new crate paths against `[dependencies]`. Run on every re-gated owner and, in batch, on all 199 (owner, feature) pairs that exist (`ctc-oracle-batch.sh`). It found the six real closure gaps fixed above (`DirectoryWriterTestFence`, `commit`, and the four optional-dependency cases); the remaining differences are unresolved-extern noise.
- rustfmt parse: the two new files parse. Edits to existing files are attribute swaps.
- Not run: cgc7 census, taxonomy, real cargo. The census measures each crate with its own default features, so the extra gated lines a consumer pulls in under `testkit` are not measured. The new files sit in existing `🧰️testkit` folder names.

### 3.4 Hits found by area (finished crates)

Roughly: R2 stdio semio model/mesh/drawing/brep 15 groups; other stdio artifacts (apply_*, gif, norm, draw, cad, fem) 24; procedural/process/puzzle/remodel 26; framework OS store, kernel-db, spr, sync 27; framework plugin 22; renderer and ui 17; hub 12; layout and draw companions 3. About 190 groups fixed in all; section 4 lists what was triaged instead.

## 4. Triaged as non-defects (36 groups, `ctc-triage.tsv`)

- Derive-macro namespace only: `value-intrinsic` `FromValue`/`ToValue` (the trait of the same name is ungated through `pub use value::*`).
- Other function of the same name: `plugin-reactor-jobs` `tests::fixture_identity` (callers use `app_testing::artifact_app_laws::fixture_identity`), `ui-contract-validation` and the `validate`/`reference`/`observe` family.
- 25 method names without evidence that the receiver is the owner type (`close`, `encode`, `len`, `insert`, `is_empty`, `matches`, `push_frame`, `take_quality` and similar). If one of them is real, the scan will show it as a `method-path` hit once the type is named.
- `plugin` `time_travel_support_1::dsl`: a `pub(crate)` module.
- `architect-program-tests-2` `editor::architect::example`: the consumer path is `crate::editor::architect::commands::example`, a different ungated module.
- Benign orphan `#[cfg(test)]` attributes followed by a blank line before an unused import (`procedural-2d-host`, `procedural-2d` binary mutations). The only one that gated production code (puzzle) is fixed.

## 5. Hand-off (crates a rollout slice is carving; not edited)

- **hand-off to r1:** `stdio-deflate-examples` `Publication::applied_progress/commit_output` used by `stdio-deflate-schema` (weak, confirm the receiver type).
- **hand-off to r6:** `wfc-3d-mutations`, `wfc-grid2d-inferences`, `wfc-grid3d-mutations` `mutations` (private test module name; check that the consumers' `mutations` paths are not the gated module), and `gis-gismap-gis2d` `unit_tests::context::dispatch` (weak).
- **hand-off to r7:** `trinity-jack` `test_support` used by `trinity-jack-shell` (real if the shell tests name it); `sequence-sequence` `unit_tests::context::new_app_with_registry_wired` (weak).
- **hand-off to r8b:** `forms-forms-forms` `unit_tests::context::dispatch` used by `forms-forms-config-tests` (weak).
- r3, r4, r5a, r5b, r5c, r9b: no remaining hits at the last scan (r5a and r9b were fixed here once finished); the FLEET rule applies to their pending carves. The r1, r6 and r8b items go to those slices; the r7 items (trinity-jack, sequence) go to r7b/r7c per the coordinator.

Fix recipe for every hand-off: re-gate the item (and the helpers its body uses) as `#[cfg(any(test, feature = "testkit"))]`, declare `testkit = []` (plus `dep:` entries for dev-only crates the body uses), and enable the feature in the consumer's `[dev-dependencies]`.

## 6. Coordination notes

- `F/📓️FLEET.md` got one `- CTC:` line with the rule and the tool names.
- Two companion crates, `semio-s-artifact-layout-layout-editor-commands-tests` and `semio-s-artifact-draw-drawing-editor-commands-tests`, appeared during the run and one peer rewrote the first manifest after my edit (the `testkit` dev-dependency vanished and I re-applied it). Whoever generates them must keep `semio-s-artifact-layout-layout-editor = { …, features = ["testkit"] }` and the same line for `semio-s-artifact-draw-drawing-editor-commands`. A rescan after their regeneration is cheap (about 70 s).
- I edited only finished crates plus the two new companions. No slice-owned manifest or source was touched (checked by a diff over every slice prefix).

## 7. First real checks (when `COMPILE ADMITTED` appears)

One batched call under the mutex, from `/Users/ueli/Documents/semio`:
`bun repo cargo --store=shared --configuration=prototype -- check --tests -p semio-s-artifact-stdio-semio-model-io -p semio-s-artifact-stdio-semio-mesh-io -p semio-s-artifact-stdio-semio-drawing-io -p semio-s-artifact-stdio-semio-brep-io -p semio-framework-os-store-tests-03 -p semio-framework-os-store-sync-tests-1 -p semio-framework-plugin -p semio-framework-os-kernel-db-compact -p semio-s-artifact-procedural-generation3d -p semio-s-artifact-process-process3d-host --message-format=short`.
Expect errors only from the `…SnapshotRead`/hostile-authority bodies if one of them used a dev-only crate the closure check could not see (type errors inside re-gated bodies are not covered).
