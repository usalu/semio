# Exec puzzle2d-editor

Status: WRITTEN BUT UNVERIFIED. `foundation.status` stayed RED (20:01 native=101 wasm=101, `semio_framework_value` retirement imports) through the whole session; no cargo call was made. Every claim below is from reading code, not from a build or test run. Crate: `semio-s-artifact-puzzle-2d` (standalone workspace; run from `◻️2d/📦️packages/🦀️rust`).

Root: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any` (below `S`).

## Result
- `puzzle2d_snapshot_mutations` is DELETED (`S/🧬️schema/🧬️mutations/🦀️.rs`, region SnapshotDelta). `puzzle2d_operations_from_host_values` / `_snapshot_change` are DELETED (`S/✏️editor/🦀️.rs`). No `puzzle2d_document_delta_operations` exists for 2d (that name is 5d's). 5d does not depend on the 2d crate (`rg` of puzzle2d symbols in `🖐️5d`: none); 5d keeps its own `puzzle5d_snapshot_mutations`, owned by `puzzle-3d5d-host`.
- New kind-emitting recorder `S/✏️editor/📼️recorder/🦀️.rs` (`Puzzle2dRecorder`, mounted as `editor::puzzle2d::recorder`). It holds the document after the rows recorded so far; `record(kind)` computes the kind's diff on it, drops an Error/Fatal outcome or an empty diff, and applies the diff through `protocol::apply_diff`. Commands read the base `scene.board_snapshot` (now read-only) and write only through `ctx.recorder`.
- `puzzle2d_dispatch_emit` builds a recorder over the document, the commands record into it, host events are replayed into it, and the emitted `artifact_mutations` are `recorder.into_rows()` followed by the select-tool yields.

## Per command: before -> after
| Command | Before | After (kinds emitted) |
|---|---|---|
| addNode (batch + retained reduce) | JSON node pushed into scene, diffed | `create-node` (retained path now numbers the label against the real document, it used an empty one) |
| deleteSelection | JSON removal, diffed | `delete-node` (cascades its edges), `remove-node-handle`, `disconnect-handles`, `delete-target-region`; the host `delete_selection()` call is dropped (explicit rows are authoritative) |
| duplicateSelection, paste | JSON clone + relabel, diffed | `create-node` per clone (next free label per kind, seen by later clones) + `connect-handles` per rewired edge (+ edge flag kinds) via `clone_fragment` |
| cut | whole board minus nodes, diffed | `delete-node` per cut node (+ `disconnect-handles` for edges on node ids) |
| setSelectionFlag/Hidden/Locked | JSON flag, diffed | `change-node-visible/locked`, `replace-node-handle` (handle flag), `change-edge-visible/locked` |
| patchInspectorNodes (absolute / non-position delta) | JSON field, diffed | field->kind table: x,y `move-node`; shape/radius/width/height `replace-node-geometry`; nodeKind, text, iconKind, scale, visible, locked, root, anchor their own kinds; handle fields `replace-node-handle`. Unknown field records nothing |
| createEdge (+ engagement `connect`) / deleteEdge | JSON edge push/retain | `connect-handles` / `disconnect-handles` |
| proximityConnect | JSON edges | `connect-handles` via `connect_handles_in_proximity` (tolerance = max(distance, radius), id from `puzzle2d_minted_edge_id`), same as the select tool |
| addTargetRegion, regionCreate | JSON region push | `create-target-region` |
| relocateTargetRegion, regionResize | JSON pose | `move-target-region` + `resize-target-region` (locked region refuses) |
| setTargetRegionFlag/Hidden/Locked | JSON flag | `change-target-region-hidden/locked` |
| deleteTargetRegion | JSON removal | `delete-target-region` |
| forceLayout / reorganize (batch path) | whole layout JSON, diffed | `move-node` per moved node (retained work already emitted `move-node`) |
| applyBoardEvents board tools, acceptSuggestion (host `brushPlace`) | `BoardToolRelease` folded rows into JSON and diffed | `Puzzle2dRecorder::fold_board_row`: `brushPlace` = `create-node` + `connect-handles` (`link`), `edgeCreate` `connect-handles`, `edgeDelete`, `nodeDelete`, `regionCreate`, `regionResize`; `BoardToolRelease::outcome` yields the recorded rows keyed `<kind>:<index>`; `apply_host_events(host, recorder, runtime)` replays host rows the same way |
| importSnapshot | replaced scene JSON, diffed | whole-document LOAD path: `Effect::LoadDocument` built by `puzzle2d_load_document_effect` (`encode_pack` + `store::empty_document_spr`), no mutation; undecodable file = `import_invalid` notice. Publication lane changed Artifact -> HostOnly in the source contract and in `🧫️fixtures/🔏️publication-authority/🔣️.json` (route moved to the host-only group) |
| translate/rotate/scale, patch delta, engagement move/rotate/scale | select tool yields | unchanged (already concrete: `drag-/rotate-/scale-selection`, `connect-handles`) |
| setActiveExample | stage machine emitting concrete kinds from the example | UNCHANGED, deliberately: it never used the translators, and the static law `cohort_hostile_static_law_...` pins its cursorised stages. It remains the candidate to move to `LoadDocument` if the coordinator rules example swaps are loads too |
| redrawHandles, Puzzle2dImportJob (import-media merge) | concrete kinds | unchanged |

Removed helpers (editor `🦀️.rs`): `add_node_to_host_snapshot`, `delete_selection_from_host_snapshot`, `apply_selection_flag`, `puzzle2d_relabel_nodes`, `duplicate_selection_in_snapshot`, `patch_inspector_nodes`, `puzzle2d_paint_target_region`, `puzzle2d_push_target_region`, `puzzle2d_relocate_target_region`, `apply_target_region_flag`, `delete_target_regions_from_snapshot`, `apply_brush_place_payload`, `puzzle2d_push_node`, `puzzle2d_fold_board_row`, `apply_board_events_from_json`. Kept: `puzzle2d_push_edge`/`puzzle2d_push_entity` (scratch bookkeeping of the select tool's proximity search, never edits the document). `puzzle2d_cut_operations_from` / `puzzle2d_paste_operations_on` keep their names (policy `toolJobPuzzle2dReservedRoutesExact` greps them) and now take `&Puzzle2dPlaySnapshot`.
Also fixed in passing: `apply-board-events` called `from_dsl_value` without the `&` it takes.

## Tests rewritten (the ~8) and added
Rewritten: mutations unit tests `puzzle2d_delta_ops_are_granular_and_round_trip` -> `concrete_kinds_round_trip_through_the_central_applier`; `sparse_node_without_anchor_still_emits_create_node` -> `a_sparse_node_record_without_anchor_creates_through_create_node`; `puzzle2d_typed_mutation_fixture_and_refusal_law` now replays committed `operations` (fixture `S/🧬️schema/🧬️mutations/🧫️fixtures/📸️typed-mutation/🔣️.json` schema v2: `before`, `operations` (camelCase tagged), `after`; ops decoded by our codec and by serde and compared); editor unit tests `adding_nodes_stamps_the_next_display_label`, `cloning_a_batch_numbers_each_new_node_in_order` (was relabel), `patching_an_addressed_node_field_writes_only_that_node`, `patching_an_addressed_handle_field_writes_the_nested_handle` (now assert recorded rows); board-tools topology test (inline JSON region/edge); clipboard foreign-media test (play snapshot); proximity-connect laws (recorder, assert the `connect-handles` tolerance). Seeding: new context helpers `board_genesis_kinds`, `seed_board` (ingested op log of concrete kinds), `import_board` (dispatch + `load_document`), `recorder_of`; `seeded_app` in select-tool-history / history-edit-runtime and the proximity drop test use `seed_board` because an import is no longer a history edit.
Added: `S/✏️editor/📼️recorder/🧪️tests/🔬️unit/🦀️.rs` (14 recorder laws: each method's rows, middle-row delete, refused/idle kind dropped, `assert_mutation_inverse_sum_law` per row, inverse rows replayed `.rev()` restore the base) and `S/✏️editor/🧪️tests/🧪️recorded-kinds/🦀️.rs` (every command above through `puzzle2d_dispatch_emit`: expected kinds, undo restores, import = one `LoadDocument` that decodes to the imported board and no mutation, malformed import = one notice, cut/paste).

## Not run (exact commands, cwd `◻️2d/📦️packages/🦀️rust`, via `"$T/🚦️gate.sh" puzzle2d-editor -- ...`)
1. `cargo check --target wasm32-wasip2 --message-format=short --features component-app-assembly` (the editor is feature-gated)
2. `cargo test --features component-app-assembly recorder`, `recorded_kind_tests`, `board_tool_tests`, `clipboard_tests`, `proximity`, `adding_nodes`, `cloning_a_batch`, `patching_an_addressed`, `concrete_kinds_round_trip`, `a_sparse_node_record`, `puzzle2d_typed_mutation_fixture`, `select_tool_history_tests`, `history_edit_runtime_tests`
3. `bun ./📜️script.ts verify` publication-authority oracle in `🧩️puzzle/📦️packages/🟦️typescript` (fixture edited by hand).

## Open issues / risks to check on first compile
- Field names assumed from builders: `ChangeNodeVisible.new_visible`, `CreateTargetRegion.target_region`, `RemoveNodeHandle.handle_id`, `ConnectHandles.tolerance`.
- `PagedUtf8::as_str/eq_str`, `PagedList::iter/len`, `Severity: Ord` are used as the existing code uses them.
- History tests that seed with `seed_board` now see N op lines ingested instead of one `importSnapshot` edit; they compute `seed = edits().len()` so offsets should hold, but the remote-replay law relies on `ingest_operations_text` relaying the seed.
- A later dispatch row reads `recorder.value()` (host view rebuilt after each recorded row): quadratic for big gestures, bounded by existing extents (<= 1024 entities).
- `Puzzle2dActiveExampleWork` still emits through staged kinds (see table); `importSnapshot` `ActionKind::Mutation` + `HostOnly` lane mirrors process3d/jack `setActiveExample`.
- No independent third-party oracle was added for the recorder (only the fixture v2 serde oracle); the round trip and sum-law checks are the cross-checks.

## Addendum (wave-4 ruling: example switch = load)
- `setActiveExample` now answers ONE `Effect::LoadDocument` of the example (`puzzle2d_active_example_emit`, unknown id loads the empty board), no mutation row, no history row. Deleted: `Puzzle2dActiveExampleWork`, `Puzzle2dExampleStage`, `PUZZLE2D_EXAMPLE_STAGE_STEPS`. The retained route is `BoundedFirstStepCommandWork::new("setActiveExample", puzzle2d_active_example_reduce, puzzle2d_active_example_extent)`; publication lane Artifact -> HostOnly (source contract and `🧫️fixtures/🔏️publication-authority/🔣️.json`, route moved to the host-only group).
- Static law `cohort_routes_are_cursorized` / `cohort_hostile_static_law_...`: setActiveExample is pinned to the bounded route plus `Effect::LoadDocument` and the absence of `Puzzle2dActiveExampleWork`; the staged markers were dropped, the force-layout cursors stay pinned.
- Fixture `🧫️fixtures/🗄️retained-jobs/🔣️.json`: setActiveExample semantic cursors and cancel/fault boundaries are now boundedDecode/loadDocument/publish/closeOwner; the three staged hostile mutations became `setActiveExampleMutationRowsRestored` and `setActiveExampleUnboundedReducer`.
- Tests: `load_example` helper dispatches and loads the requested `LoadDocument` (returns 0 edits); `set_active_example_loads_concrete_forest_through_the_load_path`, `a_newer_example_load_replaces_the_previous_document`, new `an_example_switch_emits_one_load_and_no_mutation` replace the stage/no-op-row tests. Risk: other tests that counted example-load history edits or undid across a load (fill tests only call `load_example`) are unchecked.

## Build status
BLOCKED ON FOUNDATION: `foundation.status` RED at 22:09, 22:19 and after a further 20-minute wait loop (native=101 wasm=101, errors in the framework ui/prepared and value layers, none in files I changed). No cargo call was made, so the field-name check (`new_visible`, `target_region`, `handle_id`, `tolerance`) and the remote-replay seed (`ingest_operations_text`) remain the first things to verify; the commands listed above plus `cargo test --features component-app-assembly cohort_hostile an_example_switch set_active_example a_newer_example` are pending. Everything is still WRITTEN BUT UNVERIFIED.
