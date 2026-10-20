# Result rs-sweep2: structural resolution of the rs-sweep leftovers

Executor `rs-sweep2` (foreground, no sub-agents, no git writes, no cargo: the dependency sync held the registry lease). `C` = this ticket folder. Tools: `C/rs-sweep-tools/` (extended: `macros.ts` statechart model, `dyn_enum` model, bare-type check `RS_BARE=1`, `sum.ts`, `revertwiden.ts`). Run output: `C/🗑️generated/rs-sweep2-run/`. Baseline (same scope, 830 packages): 1,621 findings in 168 packages (`findings-base.jsonl`). With the new bare-type check (`RS_BARE=1`, single-segment type names): 2,854 (`findings-b1.jsonl`), of which 1,556 are the new `unresolved-type` kind.

## 0. Corrections to rs-sweep edits (aud-verify7, `audit-verify-1019g.md`) - done first

1. `_cold` renames: both definitions are `inverse_fem2d_mutation_cold` / `inverse_fem3d_mutation_cold` again (value-based variant, as at HEAD bd9e4e02bd0). 257 importer files re-pointed to the `_cold` names (all call sites pass `&base`/`&before()` of the value snapshot type; spot-checked by file: only 4 files pass the persisted `Fem2dSnapshot`/`Fem3dSnapshot` - the two `patch-material/tests/unit` and the two `move-selection/tests/laws` files - and those keep the plain name `inverse_fem*_mutation`, which now resolves nowhere: there is no plain function with the persisted signature; they are stale against the persistent-list snapshot migration (the base is built from `PersistentList` fields) and need their fixtures rewritten by the snapshot migration owner).
2. Visibility: the 62 pets `pub` widenings and four others (`path_codec`, `reserve_exact_owner_page`, `cold_paged_owner`, the `BoardFillNodeCapture` re-export) were reverted with `revertwiden.ts` (LCS line alignment against the first-touch backups), then the checker was re-run: it re-derived exactly the same widenings, because each one has a real cross-crate `use` (path-resolved, not a word search; the audit's word search missed `use` lists and test targets). Evidence from the pre-fix findings: `semio_framework_pets_staging_draft::attention::{wink,tossed,...}` are named by `pets-staging` (`🕰️clock/🦀️.rs:9`, `🎪️stage/🦀️.rs:19`, and 67 more private-item errors in `findings-beforefinal.jsonl`), `cold_paged_owner`/`reserve_exact_owner_page` by `semio-s-fem-analyses-assembly` (`🧩️assembly/🦀️.rs:34,40`, dependency crate), `path_codec` by the integration test target `codec_laws` of `stdio-semio-value-mutations` (a separate crate), `BoardFillNodeCapture` by the integration test `board_normal_fill_board_host`, `ErasedWindowConfigPackLoad` by `plugin-tests-19`. The same applies to the cad `world_*` helpers (integration test targets), `window_tab_rect` (ui-tui-engine), `ConfigurationWorker` (plugin-instance), `snapshot_wire_codec` (raster tests). Restored state = checker-minimal: no widening exists without a resolved cross-crate user. `PageAllocation` (value family) is left to value-fix. The checker guard was tightened: `tryWiden` refuses cross-package widening unless the using package declares the dependency.
3. Import dedup: checked all 6 files. The removed imports are provided by sibling files that are `include!`d into the same module (`native_actor` by `hub-hello-bootstrap-and-receive-laws`, `Seen` by `assembly-csr-element-build-and-job-laws`, the plugin and db leaves likewise). Restoring them would create E0252 duplicates; the checker (which splices `include!`) reports no unresolved leaf in any of the six. No restore needed; evidence: `findings-b1.jsonl` has no finding for those names in those files.
4. fem3d `guards`: the file held two concatenated copies; the kept copy is the second, the superset with the `entity_identity_breach` checks on value types (the removed first copy used persisted types and lacked those checks). Deliberate choice, recorded here.
5. Checker guards: `tryWiden` refuses cross-package widening unless the using package declares a dependency on the owning package (`widen-no-cross-crate-user` skip); bare-type imports are only planned from the crate's own dependency domain (never from the global index); duplicate merging stays limited to byte-identical items / same-entity imports; renames of definitions are never automated.

## 1. Structural changes (by method)

Cycles broken by moving shared items down (no new upward dependencies, `reach.ts` verified, 0 cycles refused in the last `fixdeps` run):
- fem3d results-window now owns `Fem3dResultRequest`, `Fem3dResultSource`, `result_request` and the traits `Fem3dResultFields`/`Fem3dLiveSession`; session-api implements `Fem3dMountedLive`; play-app and viewer pass `&Fem3dMountedLive`.
- puzzle-5d: `Puzzle5dKindWeights` moved into document-records `config`; mut-any crate lost its duplicate `play_snapshot` mount (mut-any-5 canonical); transient-diff bogus `ColdDiffAlgebra` impl removed.
- replication history: `HistoryFoldSet` text/copy operations moved from the index crate into the history crate as `HistoryFoldSetText`/`HistoryFoldSetCopy`.
- retained-command frontier test helper takes policy and close function as parameters (no testing-laws dependency).
- os-store: `ArtifactStorePreparedBinding` mounted in one-item-publication; `ReplayProjection` renamed `ColdReplayProjection`; `DurablePlanningOutputV1` exported from durable-group-wire.
- plugin facade: `bounded_first_step_tool_proofs` and (test/feature gated) `composed_*`/`history_edit_acceptance_law` re-exported at the root; testing-acceptance macros no longer reach `$crate::app::...`.
- actor: diagnostic file mounted as `typed_operation_diagnostics`; `semio-framework-pack-json` moved from the wasm-target table into `[dependencies]`.

Companion test crates given their own fixtures: `job_close_fixture` (tests-05, 14-2, 16), `reactor_driver` (tests-11-2, 12-2, 14-2), `app_fixture` (procedural-2d editor-routes-2), `fixture_original_turn` (wgpu host-winit), hub-stdio `component_assembly` test moved next to the catalog (feature `component-app-assembly`).

Macro-defined names taught to the checker instead of skipped: energy `patch!`/`row_patch!`/`record_patch!` made `#[macro_export]`; 277 energy leaves and 34 puzzle-5d leaves rewritten to the generic `Op` / `MutationKind::SEMANTICS` form the macros expect.

Mirror copies: canonical copy chosen, duplicate deleted, users re-pointed (no re-exports): puzzle-5d mut-any, fem3d `guards`, retained-command full-operation duplicate `include!`, plugin tests-01 duplicate `use` lines (this round), actor diagnostic duplicate imports (this round).

Last round: `poll_ready_now` users in generation3d re-pointed to the existing `semio_framework_async::poll::resolve_ready`; `RetainedIntrinsicDocument` import re-pointed to `semio_framework_pack::RetainedIntrinsicDocument`; 13 further checker-planned edits applied (`--repoint --apply`, ap3) and 4 manifest dependencies added by `fixdeps.ts --apply`.

## 2. Checker work (language-agnostic test + rustc cross-check)
- `RS_BARE=1 bun selftest.ts` in `C/rs-sweep-tools/`: 18 fixture cases, each compared with rustc nightly-2026-07-20 (new: `e0412_bare_type`, `ok_macro_export_path`). Cases are language-agnostic text fixtures under `fx/cases/`.
- New: `macros.ts` (`statechart!`), `dyn_enum` model, bare single-segment type check, widening guard (`widen-no-cross-crate-user`), per-module import key, registry-name blocklist for bare names, `foreign-root` skip, `suggest-<tag>.jsonl`.

## 3. Scope handover
After the coordinator split, edits in `✏️s/**` stopped. `✏️s` crates already touched by this executor (≈298 files): fem 2d/3d (editor, editor-session, results, session-api, mutations, standards), puzzle 2d/3d/5d (editor, mutations, document-records, store-preparation, tests mirrors), energy model (mutations, editor, inspector) and energy simulation engine (a few files), docx tests, cad, generation2d/3d (incl. the `resolve_ready` re-point), draw drawing, fem engine analyses (assembly, model, static, pcg), stdio semio/dwg/deflate, architect program, flow, raster, norm en1996, layout, sequence, reasoning wires, sourcing curation. Remaining `✏️s` findings (329 in r6) belong to `rs-sweep3`.

## 4. Verification
- Checker run `findings-r6.jsonl` (`RS_BARE=1`, 830 packages): 788 findings (non-bare comparable subset: 528 vs 1621 base, crates 168 -> 97). By file tree: framework 440, hub 19, products 329 (counted before dedupe by cause).
- Not run: cargo build/check/test (registry lease held by `bun repo dependencies sync --all`), no unit tests executed. Everything above is statically verified only.

## 5. Open items (framework/hub), with cause
- Definition absent at HEAD or in-flight migration: `ToolRunMemberFoldRefusal` and `ToolRunMemberFold.grant/receiving`, `FixtureStoreReceiving`, `DurableOwnedGroupTestMember`, `SHIPPED_FLEET_SQLITE_OWNERS`, `drawing_document_store_owners` (7 sites re-pointed already), `ReferenceSpaceMember` (member-facets directory unmounted).
- Cold/value migration owned by others: `semio_framework_os_spr::apply_diff` (replaced by `apply_diff_cold` needing `ColdMutation` bounds in store cursor operations and paged-one-item `mutation-apply`), `MutationAdmission`/`MutationMessageLedgerRetirement` moved to the replication-mutation crates, the 4 persisted-snapshot fem tests, `Puzzle5dPart*` in the 5d snapshot unit tests.
- Value-family scope (`value-fix`/`value-repoint`): `FactorySharedRetirement`, `SealedShared`, `RetainedWireByteSource`/`OriginalWireSourceIdentity` paths in tool-run.
- Residual duplicates: replication `retained` module, durable-group `durable_group_test_outcome`, space-schema `CommitSpaceCheckpoint`/`RemoveSpaceCheckpoint` impls, `TestRuntimeApps` (macro-defined by `dyn_enum_close!`).
- Wasm-only names in the wgpu program bridge (`Array`, `JsValue`, `js_sys`, `browser_ephemeral::*`): not resolvable by a native checker.

## 6. Per-crate before -> after (non-bare kinds, base = rs-sweep result, after = r6)
| crate | before | after |
| --- | ---: | ---: |
| `semio-s-artifact-puzzle-3d-editor` | 214 | 0 |
| `semio-s-artifact-puzzle-5d-tests` | 67 | 0 |
| `semio-framework-plugin-tests-14-2` | 52 | 0 |
| `semio-s-artifact-puzzle-2d-mut-any-5` | 42 | 0 |
| `semio-s-artifact-energy-model-mut-window-layer-optics` | 40 | 0 |
| `semio-s-artifact-procedural-2d-editor` | 37 | 0 |
| `semio-framework-os-store-engine-stage` | 36 | 0 |
| `semio-framework-plugin-tests-16` | 35 | 1 |
| `semio-framework-plugin-retained-command` | 34 | 0 |
| `semio-framework-plugin-tests-05` | 34 | 0 |
| `semio-s-artifact-puzzle-2d-v1` | 33 | 24 |
| `semio-framework-plugin-tests-11-2` | 31 | 3 |
| `semio-framework-plugin-tests-12-2` | 31 | 3 |
| `semio-framework-plugin-tool-run` | 26 | 24 |
| `semio-s-artifact-norm-din18599-checks` | 25 | 25 |
| `semio-s-fem-analyses` | 25 | 25 |
| `semio-s-artifact-energy-model-editor` | 25 | 12 |
| `semio-framework-os-renderer-wgpu-program-bridge` | 24 | 24 |
| `semio-framework-server` | 24 | 0 |
| `semio-s-artifact-fem-3d-editor-results-window` | 22 | 4 |
| `semio-s-artifact-fem-2d-editor` | 21 | 8 |
| `semio-s-artifact-sequence-sequence-tests` | 19 | 1 |
| `semio-s-artifact-puzzle-5d` | 18 | 18 |
| `semio-s-artifact-fem-3d-editor` | 18 | 8 |
| `semio-s-artifact-cad-cad` | 18 | 3 |
| `semio-s-artifact-puzzle-3d-editor-snapshot` | 16 | 0 |
| `semio-hub-inference-contract-laws` | 14 | 14 |
| `semio-s-artifact-puzzle-3d-mutations` | 14 | 10 |
| `semio-framework-os-renderer-wgpu-host-winit` | 14 | 2 |
| `semio-s-artifact-puzzle-5d-catalog` | 14 | 0 |
| `semio-framework-os-store-durable-group-tests` | 12 | 12 |
| `semio-framework-pets-staging` | 12 | 12 |
| `semio-s-artifact-flow-flow-editor` | 12 | 2 |
| `semio-framework-replication-history-index` | 12 | 0 |
| `semio-framework-ui-tui-laws` | 12 | 0 |
| `semio-s-artifact-puzzle-5d-puzzle5d` | 12 | 0 |
| `semio-s-artifact-stdio-docx` | 11 | 11 |
| `semio-framework-pack-record-encode` | 10 | 10 |
| `semio-framework-plugin-instance-children` | 10 | 10 |
| `semio-framework-repo-dashboard-terminal` | 10 | 10 |
| `semio-s-fem-analyses-assembly` | 10 | 10 |
| `semio-framework-value-tests` | 10 | 0 |
| `semio-s-artifact-draw-drawing-editor` | 10 | 0 |
| `semio-s-artifact-draw-drawing-scene-tests` | 10 | 0 |
| `semio-s-fem-analyses-model` | 9 | 9 |
| `semio-framework-server-tests` | 9 | 0 |
| `semio-s-artifact-puzzle-2d-mutations` | 9 | 0 |
| `semio-s-artifact-fem-3d-editor-commands` | 8 | 8 |
| `semio-s-artifact-fem-3d-editor-session` | 8 | 8 |
| `semio-s-artifact-flow-flow-editor-leaves` | 8 | 8 |
| `semio-s-artifact-procedural-generation3d` | 8 | 7 |
| `semio-s-artifact-puzzle-5d-interaction` | 8 | 5 |
| `semio-framework-plugin-tests-01` | 8 | 0 |
| `semio-framework-ui-tui-engine` | 8 | 0 |
| `semio-hub-stdio-catalog` | 8 | 0 |
| `semio-s-artifact-draw-drawing-editor-interaction` | 8 | 0 |
| `semio-s-artifact-procedural-3d-editor` | 8 | 0 |
| `semio-s-artifact-sequence-sequence-v1` | 8 | 0 |
| `semio-framework-os-store-one-item-batch` | 7 | 12 |
| `semio-s-artifact-stdio-semio-value-mutations` | 7 | 7 |
| `semio-hub-stdio` | 7 | 5 |
| `semio-framework-os-infinite-board-schema` | 6 | 6 |
| `semio-framework-os-kernel-db-sync-laws` | 6 | 6 |
| `semio-framework-os-renderer-wgpu-kernel-protocol` | 6 | 6 |
| `semio-framework-os-store-engine-core` | 6 | 6 |
| `semio-framework-plugin-reactor-turn` | 6 | 6 |
| `semio-framework-plugin-tests-09-2` | 6 | 6 |
| `semio-framework-plugin-window-config` | 6 | 6 |
| `semio-framework-replication-mutation-retained` | 6 | 6 |
| `semio-s-artifact-puzzle-2d-text-inverse` | 6 | 6 |
| `semio-s-artifact-stdio-dwg-snapshot` | 6 | 6 |
| `semio-framework-os-store-ledger` | 6 | 4 |
| `semio-framework-plugin` | 6 | 4 |
| `semio-s-artifact-raster-raster-editor` | 6 | 2 |
| `semio-s-artifact-energy-model-diff` | 6 | 0 |
| `semio-s-artifact-flow-flow-unit-tests` | 6 | 0 |
| `semio-s-artifact-layout-layout-editor` | 6 | 0 |
| `semio-s-artifact-puzzle-5d-mut-any` | 6 | 0 |
| `semio-s-artifact-architect-program-architect` | 5 | 0 |
| `semio-framework-os-mcp` | 4 | 4 |
| `semio-framework-os-store-space-schema` | 4 | 4 |
| `semio-framework-pets-staging-draft` | 4 | 4 |
| `semio-framework-plugin-reactor-lifecycle` | 4 | 4 |
| `semio-framework-replication` | 4 | 4 |
| `semio-s-artifact-stdio-docx-base-mutations` | 4 | 2 |
| `semio-s-artifact-norm-iso16757-editor` | 4 | 1 |
| `semio-framework-plugin-tests-06` | 4 | 0 |
| `semio-framework-plugin-tests-19` | 4 | 0 |
| `semio-s-artifact-fem-3d-viewer` | 4 | 0 |
| `semio-s-artifact-procedural-3d-geometry` | 4 | 0 |
| `semio-s-artifact-sequence-sequence` | 4 | 0 |
| `semio-framework-os-infinite` | 3 | 3 |
| `semio-framework-os-kernel` | 3 | 3 |
| `semio-framework-os-renderer-wgpu-native-probe` | 3 | 3 |
| `semio-framework-os-renderer-wgpu-runtime-mailbox` | 3 | 3 |
| `semio-framework-os-store-codec-registry` | 3 | 3 |
| `semio-framework-os-store-composition` | 3 | 3 |
| `semio-framework-plugin-child-group-wire-laws` | 3 | 3 |
| `semio-s-artifact-fem-2d-editor-interaction` | 3 | 3 |
| `semio-s-artifact-fem-2d-editor-session` | 3 | 3 |
| `semio-s-artifact-puzzle-5d-retained` | 3 | 3 |
| `semio-s-artifact-puzzle-5d-store-preparation` | 3 | 3 |
| `semio-framework-ui-host-tests` | 3 | 2 |
| `semio-s-artifact-puzzle-2d-unit-tests` | 3 | 1 |
| `semio-s-artifact-writer-writer` | 3 | 1 |
| `semio-framework-os-store-document` | 3 | 0 |
| `semio-s-artifact-animate-presentation-animate` | 3 | 0 |
| `semio-s-artifact-puzzle-2d-engine` | 3 | 0 |
| `semio-s-artifact-puzzle-2d-interaction` | 3 | 0 |
| `semio-s-artifact-puzzle-2d-snapshot` | 3 | 0 |
| `semio-s-artifact-puzzle-5d-document-records` | 3 | 0 |
| `semio-s-artifact-puzzle-5d-mut-any-5` | 3 | 0 |
| `semio-framework-async` | 2 | 2 |
| `semio-framework-job` | 2 | 2 |
| `semio-framework-os-config` | 2 | 2 |
| `semio-framework-os-kernel-db-artifact-contract` | 2 | 2 |
| `semio-framework-os-store-config` | 2 | 2 |
| `semio-framework-os-store-durable-group` | 2 | 2 |
| `semio-framework-os-store-durable-group-wire` | 2 | 2 |
| `semio-framework-os-store-engine-live` | 2 | 2 |
| `semio-framework-os-store-operation-wire` | 2 | 2 |
| `semio-framework-os-store-testing-prelude` | 2 | 2 |
| `semio-framework-pack-format` | 2 | 2 |
| `semio-framework-plugin-tests-03` | 2 | 2 |
| `semio-s-artifact-energy-model-mutation-fixtures` | 2 | 2 |
| `semio-s-artifact-fem-3d` | 2 | 1 |
| `semio-s-artifact-wfc-2d` | 2 | 1 |
| `semio-framework-actor` | 2 | 0 |
| `semio-framework-os-store-space-member` | 2 | 0 |
| `semio-framework-plugin-host-test-component` | 2 | 0 |
| `semio-framework-plugin-tests-17` | 2 | 0 |
| `semio-framework-value-factory` | 2 | 0 |
| `semio-hub-raster` | 2 | 0 |
| `semio-hub-space-app` | 2 | 0 |
| `semio-hub-stdio-office` | 2 | 0 |
| `semio-s-artifact-cad-cad-editor` | 2 | 0 |
| `semio-s-artifact-energy-model-viewer` | 2 | 0 |
| `semio-s-artifact-gis-gismap-gis2d` | 2 | 0 |
| `semio-s-artifact-procedural-3d-geometry-compute` | 2 | 0 |
| `semio-s-artifact-raster-raster-host-retirement-bounds` | 2 | 0 |
| `semio-s-artifact-sourcing-curation` | 2 | 0 |
| `semio-s-artifact-sourcing-curation-tests` | 2 | 0 |
| `semio-s-artifact-stdio-tiff-mutation-regressions` | 2 | 0 |
| `semio-s-artifact-writer-writer-inferences` | 2 | 0 |
| `semio-s-artifact-writer-writer-tests-2` | 2 | 0 |
| `semio-s-fem-static` | 2 | 0 |
| `teaching-proctor-domain` | 2 | 0 |
| `semio-framework-os-renderer-wgpu` | 1 | 1 |
| `semio-framework-os-renderer-wgpu-runtime-mailbox-tests` | 1 | 1 |
| `semio-framework-os-store-tests-04` | 1 | 1 |
| `semio-framework-plugin-instance-dispatch` | 1 | 1 |
| `semio-framework-plugin-interpreter` | 1 | 1 |
| `semio-framework-plugin-tests-07-3` | 1 | 1 |
| `semio-s-artifact-energy-model-editor-results` | 1 | 1 |
| `semio-s-fem-sparse-subspace` | 1 | 1 |
| `semio-s-plugin-energy-engine-sim-unit-tests` | 1 | 1 |
| `semio-s-plugin-energy-epw` | 1 | 1 |
| `semio-framework-value-edit` | 1 | 0 |
| `semio-s-artifact-fem-2d` | 1 | 0 |
| `semio-s-artifact-flow-flow-viewer` | 1 | 0 |
| `semio-s-artifact-gis-gismap-gis2d-unit-tests` | 1 | 0 |
| `semio-s-artifact-puzzle-2d` | 1 | 0 |
| `semio-s-artifact-puzzle-3d` | 1 | 0 |
| `semio-s-artifact-puzzle-5d-v1-unit-tests` | 1 | 0 |
| `semio-s-artifact-stdio-docx-base` | 1 | 0 |
| `semio-s-artifact-stdio-docx-strict` | 1 | 0 |
| `semio-s-artifact-stdio-docx-transitional` | 1 | 0 |
| `semio-s-artifact-stdio-png-tests` | 1 | 0 |
| `semio-s-artifact-puzzle-2d-tests` | 0 | 3 |
| `semio-s-artifact-fem-2d-mutations` | 0 | 1 |
| `semio-s-artifact-fem-3d-mutations` | 0 | 1 |

total before 1621 after 528; crates before 168 after 97
