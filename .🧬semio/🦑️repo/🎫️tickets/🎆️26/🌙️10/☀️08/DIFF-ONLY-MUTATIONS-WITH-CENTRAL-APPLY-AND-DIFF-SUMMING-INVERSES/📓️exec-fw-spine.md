SPINE-API-LANDED 01:45:15 protocol::ApplyCapability, protocol::apply_diff(&diff, &base) -> MutationApplyResult<P>, MutationDiff::apply(&self, base, capability: protocol::ApplyCapability), MutationDiff: PartialEq + DiffAlgebra<P>; MutationOutcome::apply_to DELETED. (cargo check -p semio-framework-replication green; os spr facade re-exports `os_spr::{ApplyCapability, apply_diff}` written, os-kernel compile pending; law helper assert_mutation_inverse_sum_law pending)

SPINE-KERNEL-GREEN 02:07 `cargo check -p semio-framework-os-kernel` (lib, non-test) green via gate. Extra helpers: `store::apply_operation(state,&op,idx) -> (MutationApplyResult<P>, msgs)`, `store::apply_outcome(&base, outcome) -> (P, outcome)` (refusal -> unchanged base + empty diff + Fatal message; replaces `MutationOutcome::apply_to` semantics), law helper `protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await`. `impl_whole_record_config!` / `transient_root!` now require `$ty: PartialEq`.

# 📓️ fw-spine report

## Done (all compile-verified unless listed under "Unverified")
- `replication/🎮️mutation/🦀️.rs`: `ApplyCapability` (private field, minted only in `apply_diff`), `apply_diff`, `MutationDiff: PartialEq + DiffAlgebra<P>`, `apply(&self, base, capability)`, `MutationOutcome::apply_to` deleted. `cargo check -p semio-framework-replication` green (01:44).
- Façade: `os_spr::{ApplyCapability, apply_diff}` explicit list in `📡️spr/🦀️.rs`; `command` façade already `pub use protocol::mutation::*`. `cargo check -p semio-framework-os-kernel` (lib) green 02:06; `cargo test --lib --no-run` (incl. all framework tests + new law tests) green 02:48.
- Central routing: `store::apply_operation` (new, the one store step) -> `fold_operation`, replay (`replay_mutations`), `ReplayMode::Merge`, presence/transient/hover lanes (`apply_operation`), `derive_*_snapshot` (`apply_diff`), `os_vcs::apply_mutation`, `Planner::call`, `fold_plan_diff`, plugin transaction folds x3, `SnapshotBuilder` (`store::apply_outcome`, new, replaces `apply_to` semantics), bounded config prep, window config, transient publication, tool-machine `fold_leaf`, tool-run `fold_one`, db `envelope_from_operation`, store `test_support::mutation_report_json`.
- Framework diff impls given the new signature + concrete `DiffAlgebra` (+`PartialEq` where missing): macros `impl_whole_record_config!` and `transient_root!` (whole-record algebra; `$ty` now needs `PartialEq`), `SpaceHistoryDiff`, `ChartDiff` (inverse = swapped before/after edits, no apply), `SpaceDiff`, `CollectionDiff`, `FlowDiff` (sequential projection inverse, exact), `DagDiff` (atomised exact inverse), `NoConfig/NoPresence/NoTransient`, `InteractionConfigMutation`, mcp `ProbeDiff`, wgpu `NativeSocketProbeDiff`, and ~25 test-local fixtures (store, db, plugin tests, causal).
- Config leaves, spr counter laws, Workflow/Run diffs: done by `fw-os-leaves` (found already converted); not touched by me.
- Derive (`🗣️dsl/✨️derive`): generates no apply -> no change needed.
- Law helper `assert_mutation_inverse_sum_law(&mutation, &base).await` in `📡️spr/🧪️tests/⚖️protocol-laws/🦀️.rs`; plugins reach it as `protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law`. Unit tests in `⚖️protocol-laws-unit/🦀️.rs` (`mutation_inverse_sum_law_*`): lawful passes; wrong inverse, restoring-but-not-summing inverse, and empty inverse each panic with the expected message.
- Deleted generic `CollectionMutation`/`apply_collection_mutation`/`inverse_collection_mutation`/`collection_diff_from_mutation` (vcs + re-exports in spr/command/store + their tests; zero production callers incl. `✏️s`). `Identified`/`Patchable`/`ItemPatch`/`CollectionDiff` kept.
- Framework test call sites of `.apply(` converted for flow/dag/space tests.

## Open issues
- `CollectionDiff`/`SpaceDiff`/`DagDelta`-style single-slot diffs cannot represent cascade inverses: `CollectionDiff::inverse` of a cascading `DeleteFolder` restores only the first deleted folder/entry (one `created_*` slot). `SpaceDiff` inverse collides when one diff holds both upsert+remove of the same slot. Needs the sparse keyed-row redesign by the space/collection owner (fw-os-leaves).
- `WorkflowDiff::SyncNodePorts` has no representable inverse (Empty); node/edge order is not restored by append-style Add inverses (peer's implementation).
- `impl_whole_record_config!` algebra `is_empty()` is constantly false (whole record has no base-free empty form); ~33 plugin configs using it need `PartialEq` and should migrate to sparse per-field diffs.
- `db::envelope_from_operation` still stores whole post/base values (snapshot-based envelope), only its apply routing changed.
- Policy gate rules (L5 textual loopholes in `📜️script.ts`) not part of this scope; not implemented.

## Unverified / test status
- Not yet run to completion: `cargo test -p semio-framework-os-kernel --lib -- mutation_ diff_algebra operation_diff space_history remove_space_alternative flow dag collection --skip canonical_edit`. First attempt (03:07) aborted in `canonical_edit::borrowed_tests::borrowed_map_rebound_root...` (pre-existing fail-closed drop panic, unrelated, now skipped); the retry fails to build because of a peer's in-flight edit in `replication/📡️wire/🏠️local-interaction/🌳️root/🩹️update/🦀️.rs` (12 E0308, not my files). 87 tests ran green before the abort.

# Wave 3

## 1. Sparse per-field config and transient diffs (macros, schema-derived concrete code)
- `impl_whole_record_config!` is DELETED (no whole-record diffs remain). New `#[macro_export]` macros in `🏪️store/🦀️.rs` (`semio_framework_os_kernel::…`, `$crate::__value/__pack_json/__diagnostic` re-exported from the kernel root):
  - `sparse_record_diff! { record, diff, fields: { f: T, … } }`: struct `$diff { pub f: Option<T>, … }` (Clone, Debug, Default, PartialEq), hand `ToValue`/`FromValue` (present slots only), `DiffAlgebra` (`inverse` = base values of the set slots, `between`, `is_empty` = all None), `MutationDiff` (`apply` struct literal naming every field, so a field missing from the list fails to compile; `absorb` = later slot wins), `replacing(&record)` (all slots) and `changing(&base, &record)` (only differing slots). `whole` arm for an indivisible record (single `replacement` slot, used for `Arc`-wrapped `LowpolyTransient`).
  - `field_set_mutations! { record, diff, set, owner, payload_schema, emoji, [replace: Variant { field } wire "…" kind "…" name "…",] fields: { f: T => SetF "set-f", … } }`: per-field `Set…(value)` mutations (diff = that one slot, empty when unchanged; inverse = the same variant with the base value), optional replace variant (diff = `changing`, wire `{"kind":…,"<field>":…}`), descriptors, `{"kind","value"}` wire, JSON `OpText`/`OpBinary`.
  - `config_record!` = `ConfigRecord` mark + both; `config_diff!` = `ConfigRecord` mark + diff only (for configs that keep a hand-written mutation enum).
- `transient_root!` (plugin crate) now takes `diff: $diff` and `fields: { … }` (or `whole`) and generates the sparse diff; the peer-introduced `TransientDiff<S>` is removed. Transient mutations stay a single `Snapshot { transient }` replace variant: the ephemeral transfer (`window_transient_transfer!`: `footprint`/`into_state`/`RetireOwned`) moves a whole root by design, so per-field transient set variants cannot exist there; its diff is the sparse `changing(base, root)`.
- Invocation sites updated (plugin executors' hand conversions left alone): transient_root! x10 (cad, draw, fem, flow, forms try, layout, lowpoly [whole], raster, remodel, wfc) with `diff:` + `fields:`; `config_record!` (hand enum replaced by generated `$set` with `Snapshot` replace variant, wire unchanged) x6: forms try window, sequence main window, flow main window, generation2d main/edit-preview/generate-preview window configs; `config_diff!` + hand enum retargeted to the sparse diff (every variant sets only its own slots, concrete per-variant inverses) x7: `SpaceConfig` (hub), `Generation2dConfig`, `LowpolyConfig`, `TiffEditorConfig`, `EquationGraphWindowConfig` (+ `SetCamera` leaf), `FormsConfig` (+ `set`/`replace` leaves; `SetContributions` inverse is now `SetContributions`).
- Known gap: `SpaceConfigDiff.camera` is a whole `BTreeMap` slot (`SetCamera` clones the map and inserts one window); a per-window keyed row slot needs a macro extension.

## 2. `db::envelope_from_operation`
- Applies the op's diff once through `protocol::apply_diff`, then persists only what changed: one `path/field` pathmap entry per differing top-level field (removed field = tombstone), inverse = base values of exactly those fields; non-record projections are one entity at `path`. No whole post/base snapshot. Test updated (`counter/value` forward 15, inverse 10).
- Limit: the db pathmap envelope has no field for the operation bytes (`MutationEnvelope.diff`/`inverse` are the pathmap payloads and the engine materializes only `DB_PATHMAP_SCHEMA`), so "operation + diff" is persisted as the sparse changed-field diff; persisting op bytes needs an envelope/engine schema extension (not done).

## 3. Test results (exact)
- Replication compiled again at try 3 of 6 (12:42): `cargo check -p semio-framework-replication` Finished (41 warnings). Later the coordinator-confirmed green `-p semio-framework-replication -p semio-framework-os-kernel`.
- `cargo test -p semio-framework-os-kernel --lib mutation_inverse_sum_law` (16:48): `test result: ok. 4 passed; 0 failed; 0 ignored; 1286 filtered out` (lawful inverse passes; wrong inverse, restoring-but-not-summing inverse and empty inverse each panic as expected).
- `cargo test -p semio-framework-os-kernel --lib protocol_laws` (16:52): `test result: ok. 83 passed; 0 failed; 0 ignored; 1207 filtered out`.
- Earlier (12:44): `-- the_whole_diff generated_ mutation_inverse_sum_law --skip canonical_edit` 15 passed, 0 failed; `-- mutation_inverse_sum_law generated_ protocol_laws mutation_inverse_law mutation_diff_absorb diff_algebra --skip canonical_edit` 84 passed, 0 failed. New tests: `generated_set_mutations_satisfy_the_inverse_sum_law`, `generated_diffs_are_sparse_and_the_wire_round_trips`, `the_whole_diff_replaces_an_indivisible_record` (macros `sparse_record_diff!`, `field_set_mutations!`, `whole` arm).
- `cargo check -p semio-framework-plugin` green (12:52).

## Unverified (blocked by peers, not by this wave)
- `transient_root!` expansion and the plugin invocation sites (`config_record!`/`config_diff!`/`transient_root!` in 15 crates) are WRITTEN BUT UNVERIFIED: `cargo test -p semio-framework-plugin --lib` fails on 33 unrelated resolution errors (`AnalyzeSource`/`Analysis`/`ComposeSource`/`SubsetValidator`/`IoPayload`), and every plugin crate `cargo check -p <crate> --target wasm32-wasip2` (run per artifact workspace dir, 17:00) fails inside the peers' in-flight `🌱️value` retirement rewrite (`ErasedSnapshotRetirement::next_close_byte_demand`, `close_step`, `retire_owned` signature errors) and `semio-hub-space` on the peers' `apply_workflow_operation` visibility; logs `T/🗑️generated/fw-spine/sites/*.txt`. No error mentions a Wave 3 file. Re-run: `cd <artifact dir> && cargo check -p <crate> --target wasm32-wasip2` once the value crate compiles (list: `scratchpad/sites.tsv` crates: cad, draw, fem-2d, flow, forms, layout, lowpoly, mathematical-equation, procedural-generation2d, raster, remodeling, sequence, stdio-tiff, wfc-grid2d, hub-space).
- Possible compile-time follow-ups at the invocation sites: field types from sibling schema modules not imported in the invoking module, and private fields of records defined in a child module.

# Wave 4

## Priority fix: `TransientDiff`
- `rg '\bTransientDiff\b'` over every file type (framework, `✏️s`, hub, all features and cfg paths): 0 remaining references (only plugin-local `…TransientDiff` names). `component-app-assembly` is a feature of the artifact crates (generation2d, space core, …), not of `semio-framework-plugin` (`--features component-app-assembly` on the plugin crate is refused by cargo), so it is verified together with the site checks below. Nothing was migrated because nothing still imported it.

## 1. Window configs: no whole-record mutation
- `field_set_mutations!`/`config_record!` lost the `replace:` variant entirely; new `$set::setting(&base, &next) -> Vec<$set>` (one set mutation per changed field, field order). `FromValue` of the set enum now rejects unknown members (the hostile-mutation law of the window-config tests). Docstrings state the exception: only an ephemeral transient root keeps one whole-root mutation (`transient_root!` docstring + `sparse_record_diff!` docstring), because the ephemeral transfer moves a whole root.
- Six window configs (forms try, sequence main, flow main, generation2d main / edit preview / generate preview): `replace:` line removed; `addressed(view, base, config) -> Result<Vec<WindowConfigMutation>, Fault>` returns one `of::<Owner>` mutation per changed field. Callers migrated: forms editor x5, sequence editor x2, flow editor x3, generation2d editor x1 (each now keeps the `base` config it diffs against); tests (`generation2d-window-camera-ownership` rs + ts, flow window rs, forms window) updated.
- Schema-first twins updated by hand: the six `🧬️schema/🟦️.ts` (`kind: "set-<field>"` union + apply per field; sequence/generation2d keep their parsers), TS tests and committed fixtures (forms `advance`/`resetConfig` = `set-current-step-index`; sequence = `set-orientation` + `set-camera` rows per window; flow = nine `set-*` rows per window with the oracle patching `/<field>`; generation2d camera ownership = `set-viewport` rows).
- Hub `SpaceConfigMutation`: the hand-written `Snapshot` variant is removed too (descriptor table renumbered, test line removed); absent-row inverse of `RemoveCamera` is empty instead of a snapshot.

## 2. Keyed rows (`SpaceConfigDiff.camera`)
- `sparse_record_diff!`/`config_diff!` take `keyed: { map_field: ValueType, … }`: the slot is `KeyedRows<V>` (`BTreeMap<String, KeyedRow<V>>`, `KeyedRow = Insert | Replace | Remove`), applied with `keyed_rows_apply` (duplicate insert / missing replace or remove refused, naming the field and key), inverted by `keyed_rows_inverse` (insert<->remove, replace restores the base value), built by `keyed_rows_changing`, composed per key by `keyed_rows_absorb` through `KeyedRow::then` (insert∘remove cancels, insert∘replace = insert, replace∘remove = remove, remove∘insert = replace). Wire: `{"op":"insert|replace|remove","value":…}` per key.
- `SpaceConfig.camera` is now a keyed slot: `SetCamera` emits an Insert/Replace row for its window; new `RemoveCamera { window_id }` mutation (descriptor, wire key `remove-camera`) is the inverse of a first `SetCamera`. Kernel test `keyed_rows_replace_the_whole_map_slot` added (changing, apply refusals, inverse-law, absorb cancel/replace, wire round trip).

## 3. Gate burn-down items
- R16 false positives in framework code: `MapDelta::apply_to` renamed `apply_onto` (+ its test) and the plugin builder's `declaration.apply_to` renamed `install_into`; no `apply_to(` remains in `🧰️framework/**/📡️replication/🎮️mutation` or the plugin builder.
- R14 in `💻️os/🔨️modules`: the macro-generated `Snapshot` variants are gone (item 1); the remaining hits are hand-written fw-os-leaves files (`🏪️store/🧪️testing/…restore-n/…`).
- New `assert_mutation_inverse_sum_law_cold(&mutation, &base, retire, retire_diff).await` next to the other `_cold` laws (retires every projection, diff and minted inverse op; same four assertions run after retirement), with tests `mutation_inverse_sum_law_cold_holds_and_retires_every_projection_and_diff` and `…_cold_catches_a_wrong_inverse`. Plugins reach it as `protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law_cold`.

## Verification status: BLOCKED ON FOUNDATION (written, not compiled)
- `T/🗑️generated/coord/foundation.status` stayed RED from 17:46 to 18:48 (60 min wait, polled every minute in the foreground): errors in the peers' `🌱️value` retirement rewrite and its consumers (`owned_retirement`, `next_close_byte_demand`, `close_step` in replication causal fold and `🖱️ui/🎬️scene/📐️math`). None names a Wave 3/4 file. Therefore NOT RUN since the Wave 4 edits: kernel tests (`keyed_rows…`, `generated_*`, `mutation_inverse_sum_law*`, the new `_cold` laws), `cargo check -p semio-framework-plugin --features …`, and the 15 plugin sites + `semio-hub-space` (`cd <artifact workspace dir> && cargo check -p <crate> --target wasm32-wasip2`, hub native; crate list in `scratchpad/sites.tsv`; loop `T/🗑️generated/fw-spine/sites-loop.sh`).
- Last verified green before Wave 4 (16:48-16:52): `mutation_inverse_sum_law` 4 passed; `protocol_laws` 83 passed.
- Re-run once GREEN: `cargo test -p semio-framework-os-kernel --lib -- keyed_rows the_whole_diff generated_ mutation_inverse_sum_law protocol_laws --skip canonical_edit`, then the site loop (`T/🗑️generated/fw-spine/sites-loop.sh "$T" scratchpad/sites.tsv` under the gate).

# Wave 5

Status: edits written, NOT compiled or run. Foundation was RED at every check (last 22:19, errors in the peers' wgpu `prepared` `close_step` / `InteractiveJobCloseStep` rewrite, none in my files) and disk was at 8.7 GB, so no cargo call was made.

## Changes
- Plugin trait `whole_document_operation` deleted (`ArtifactApp`, `ArtifactEditor`, `EditorApp` forwarder). Natural-file and `artifact:in` import (`import_media` default, `decode_synchronous`, controlled-decoder completion) return `document_load_emit(&snapshot, DOCUMENT_SCHEMA)`: a single `Effect::LoadDocument`, no mutation, no history row. Overrides and assertions removed in stdio png/jpg/bmp, dag, writer, surface fixture; gismap `artifact:in` migrated. Natural-file lifecycle fixture and test: `openedHistoryEntries` 1 to 0; TS engine-contract test no longer pushes `set-snapshot`.
- Flow VCS `begin_replace_document` is now a document load (checkout), decided as follows:
  - It writes no undo row (history mode 3, the same path as `Checkpoint`). The `ActivateDocument` inverse action and its step/rollback arms are deleted. Rollback restores `document.active` from `cursor.origin`.
  - It requires a pristine history. Admission and the `ReserveReplacement` poll refuse with `InvalidMutation` when undo or redo is non-empty, because a stale inverse would otherwise run against the loaded document.
  - The staging slot in `versions` stays as the checked-out document. It is storage, not a history row.
  - Mid-session whole-document replacement does not exist any more. Content changes after a load go through concrete widget/synapse/layout kinds.
- Tests: `retained_vcs_replace_document_uses_persistent_owner_transfer_phases` now asserts empty undo/redo and `active == 1`. New `retained_vcs_replace_document_is_a_load_that_writes_no_history_row_and_refuses_a_live_history`. The oracle (`flow-vcs` tests) asserts a pristine history for `replaceDocument` and no longer pushes undo.
- `🧾️semantic-history` fixture: the mid-session `replaceDocument` step is removed, along with its three document-ledger entries. The undo/redo/checkpoint steps were renumbered and recomputed with a python port of the oracle digest. The port reproduced steps 1-11 of the old fixture exactly. New digests: undo `10347887768300612752`, redo `7107579225377607807`, checkpoint `7364717663870381529`. Undo/redo owners are 8/1, 9/0, 9/0; `documentVersions` is 1 and `activeDocumentVersion` is 0 throughout. The `🔄️lifecycle` fixture's `replaceDocument` cases all use `setup: {}` (empty history) and are unchanged.
- wgpu shell `apply_ops_inner`: dead `setDocument` reader and `document_changed` removed (only that reader set it), docstrings trimmed. The unrelated `chrome_action("puzzle3d","setDocument")` test in `wgpu-introspection` is an action name and stays.

## Unverified (run when foundation is GREEN)
`cargo test -p semio-framework-os-kernel --lib` flow-vcs tests (`retained_vcs_replace_document*`, oracle/semantic-history, lifecycle boundaries); `cargo check -p semio-framework-plugin` and a plugin natural-file test; `cargo check` of the wgpu shell target; stdio png/jpg/bmp, dag, writer, gismap crates.

## Leftovers
- Stale comments naming `whole_document_operation` remain in lowpoly, sourcing, writer, shooting, fem2d/3d (editor + mutations schema), dag, animate, cad, process3d, raster. Comments only, no code references.
- Flow's `FlowVcsFault`/fixtures still name `atomicReplaceDocument` in the lifecycle forbidden-features list. That is a guard name, not an API.

# Wave 6

Status: edits written, NOT compiled or run. Foundation was RED at 22:30 (peer wgpu errors), so no cargo call was made.

## F-12 Flow diff (AMB-3)
- `FlowDelta::HostSnapshot` is deleted. A delta is now only `Widgets`, `Synapses` or `Layout`. The `♻️replace-flow-host-snapshot` leaf (`ReplaceFlowHostSnapshot`) is deleted with its folder, fixtures, wire witness, retirement row, retained-field arms, descriptor and roster entries (nine leaves, tags 0-8), and the schema-catalog entries (`🔣️schema-catalog.json`, `📓️schema-catalog.md`).
- `FlowDiff::inverse` no longer builds a projection or applies deltas. Each delta's inverse is read from base rows:
  - Removed ids re-enter at their base position, ascending.
  - Replaced ids return the base item under the replacement identity.
  - Inserted ids leave.
  - Layout rows restore each id's previous assignment from base, or from an earlier layout row of the same diff.
  - A row not found in base is looked up in the earlier rows of the same diff.
  - Limitation: positions of rows that earlier deltas of the same diff shifted are exact only for disjoint rows.
  - The central store never inverts a composed diff. It absorbs per-row inverses.
- `FlowDiff::between` is now a sparse structural difference. Vanished and moved identities are removed, changed survivors are replaced, and new and moved items are inserted at their final index. A move is remove plus insert, the same pair `MoveWidget` emits. Layout rows carry only changed or removed entries. There is no whole snapshot.
- `FlowProjection` keeps only `apply` and `materialize`; `inverse_of`, `inverse_layout` and `snapshot` are removed.
- `flow_host_snapshot_operations` (the bridge) now also emits `MoveWidget`/`MoveSynapse` rows for reordered survivors (greedy, position-ascending), so a reorder is no longer lost.
- Flow host (`🖥️host`): the two `ReplaceFlowHostSnapshot` dispatches are replaced by `record_history_edit`. It computes the concrete leaves from the baseline and the live snapshot, applies them in one `ArtifactCommand::Apply`, and retires the leaves cold when nothing is recorded. An edit with no structural net effect now writes no history row. Before, `flush_pending_change` wrote an empty row unconditionally.
- Tests: the flow-vcs unit tests (9 witnesses, 9 descriptors, composition `[0,2,4,6,8]` = 5 deltas, `decode_op(&[1, 9])`); the ownership tests (3 delta variants; the two `replace-fixture-*` cases are removed); the diff fixtures and `🔣️.json` schema (the `hostSnapshot` delta variant is removed and an invalid row `whole-snapshot-delta-is-not-a-diff` is added). New test `between_is_structural_and_its_inverse_reads_the_base_rows`.

## F-13 RunDiff
- `RunDiff::inverse` no longer does `let mut state = base.clone()` or `step_into`. `RunRows` reads each slot's prior value from base or from the slot's own earlier row in the same diff:
  - header and status cells,
  - seal cells,
  - node rows by id,
  - the log tail (kept base lines plus appended rows).
- `RunStep::negation(&state)` is removed. `between` is unchanged, since the audit named only the inverse as the fix.
- New test `composed_run_diff_inverse_reads_rows_and_restores_the_base` (log append/retract, node set/clear, seal; inverse restores the base). The existing `assert_mutation_inverse_sum_law` run tests already exercise the new inverse through `d.inverse(base)`.

## Stale comments
All `whole_document_operation` mentions are gone from dag (editor, diff header), process3d, raster, lowpoly, shooting, cad (editor, mutations), animate, writer, sourcing and fem2d/3d (editor, mutations). `rg whole_document_operation` over `✏️s` and `🧰️framework` is empty.

## Unverified (run when foundation is GREEN and disk recovers)
`cargo test -p semio-framework-artifact-flow-flow` (the `🌿️vcs` tests and ownership tests) and the flow host unit tests; `cargo test` for the workflow run artifact crate; `cargo check` for the flow host and for the `flow_host_snapshot_operations` bridge users. Run `bun ./📜️script.ts schema generate` to refresh the stale hashes in the schema catalog.

# Wave 7

Status: edits written, NOT compiled or run. Foundation stayed RED (22:41, peer wgpu errors), so no cargo call was made. `bun ./📜️script.ts schema generate` ran: exit 0, "3645 scopes, 12425 diagnostics". The `replace-flow-host-snapshot` scope is gone from the catalog and the hashes are refreshed.

## Translator deleted
- `flow_host_snapshot_operations` and `flow_survivor_moves` are deleted from the flow vcs artifact. The old unit test `flow_fixture_ops_diffs_widgets_synapses_layout` is deleted. The wording in the flow source-contract TS test and the doc comment in `🚪️io/📝️text/📸️snapshot` is updated.
- The flow host no longer compares snapshots to find its history. Each mutating gesture site emits its concrete leaves into `FlowHost::pending_leaves` (`note_leaves`). `record_history_edit` applies them in ONE `ArtifactCommand::Apply` transaction, seeding the store from the baseline on first use. Unrecorded leaves are retired cold (`retire_flow_mutation`). They are also retired on pointer-cancel and history reset, and handed to the `FlowHostRetirement` domain frontier on close.

## Leaves per gesture (application order)
- `add_widget`: add-widget at the end index, then change-layout.
- `remove_widget`: remove-synapse for each wire touching it, change-layout with a `None` entry, then remove-widget.
- `connect_ports`: remove-synapse for any wire displaced from the target port, then add-synapse.
- `disconnect`: remove-synapse.
- `add_/remove_input_/output_port`: remove-synapse for severed wires, change-synapse for each renumbered wire, then change-widget with the new port list.
- `insert_between`: change-synapse for each rerouted wire, then add-synapse.
- `make_space`, `reorganize`, `align_selection`: change-layout over the shifted, all, or selected ids.
- Param setters: change-widget. This covers `set_neuron_params`, `set_slider_value`, `set_note_text`, `set_variable_name`, `set_variable_schema` and `set_image_src`.
- `delete_selection`: remove-synapse for selected edges and wires touching selected nodes, change-layout with `None` entries, then remove-widget. The ids come from the selection before the dag deletes.
- `collapse_selection`: remove-synapse for wires touching the selection, change-layout with `None` entries, remove-widget per selected id, add-widget for the cluster, change-layout for the cluster, then add-synapse for the rewired crossing wires.
- `explode_cluster`: remove-synapse, change-layout `None`, remove-widget (cluster), add-widget per restored widget, change-layout for restored ids, then add-synapse for rewired and new wires.
- Pointer and note gestures: the dag's own journal rows, read without draining through the new `DagHost::graph_edits()`, are turned into leaves at gesture commit (`gesture_journal_leaves`):
  - Connect becomes remove-synapse for a displaced wire plus add-synapse.
  - Disconnect becomes remove-synapse.
  - Move becomes change-layout.
  - SetSlider becomes change-widget.
  - InsertPort is skipped, because `add_*_port` already emitted it.
  - An inline note edit emits one change-widget at `note_commit_edit` when the text changed.
- A gesture or edit that emits no leaf writes no history row and seeds no store.

## Tests (new, in the flow host unit tests)
- `add_widget_emits_add_widget_then_change_layout`
- `remove_widget_emits_its_wires_then_layout_then_the_widget`
- `disconnect_and_connect_emit_one_synapse_leaf_each`
- `layout_and_param_gestures_emit_change_layout_and_change_widget`
- `delete_selection_emits_wires_layout_and_widgets`
- `gestures_undo_last_to_first_and_redo_first_to_last`: add widget, disconnect, connect, set slider; four undos then four redos each match the captured state on widgets, synapses and layout keys.
- `undo_redo_add_widget` now reads the host's own leaves instead of the deleted bridge.

## Risks and known divergences
- The store document no longer self-heals. Content changed outside the recorded gestures can make a later leaf fail silently (the dispatch result is ignored, as before). Such changes are: `move_widget` (it never recorded history), `set_host_snapshot_preserving_history`, and edges the dag drops as cyclic on `rebuild_dag`.
- Synapse list order after collapse and explode follows the store's insertion order, not the host's append order. Content is equal, order may differ after undo.
- The pointer connect/disconnect and the notes path rest on the dag journal; they are covered only by the existing note-gesture test, not a new pointer test.

## Unverified (run when foundation is GREEN)
`cargo test` for the flow host unit tests, the flow vcs and ownership tests, and the dag crate (`graph_edits`). `cargo check` of the host with the wasm32 target.

# Wave 8

Status: edits written, NOT compiled or run. No cargo call was made, because foundation was RED at every check.

## 1. A refused leaf surfaces
- `record_history_edit` no longer ignores the dispatch result.
  - A refused transaction resets the history store to the live content (`reset_history_store`), so no partial row of the refused edit survives. It also keeps `FlowCoreError::HistoryRefused(reason)`, a new variant.
  - Earlier undo steps are gone after a refusal. That is the recovery chosen: the store can no longer represent the live content.
- Discrete gestures now record eagerly at the end of the gesture instead of lazily at the next `begin_change`:
  - Gestures that return `Result` call `checked_change()?`, so the caller gets the fault of its own edit.
  - The five `()` setters call `finish_change()`.
  - `take_history_fault()` and `history_outcome()` hand over the sticky fault.
- The wasm adapters for `set_slider_value`, `set_note_text`, `set_image_src`, `set_variable_name`, `set_variable_schema`, `note_commit_edit` and `pointer_up_screen` return the fault through `history_outcome().map_err(domain_error)?`.
- The in-process wgpu callers of the flow host pointer events can read `take_history_fault()`; they are not wired to it.

## 2. No content change bypasses the leaf path
- `move_widget` is now a recorded edit: a change-layout leaf (a separate undo row per call outside a gesture).
- `set_host_snapshot_preserving_history` is deleted (it had no callers). `replace_host_snapshot` is a load and always resets the history.
- `resync_host_snapshot_from_scene` keeps the history only when the incoming widgets, synapses and layout equal the live ones, which is an echo of the host's own edits. A scene whose content differs is a load and resets the history, the pending leaves, the armed baseline and any open note edit.
- Cyclic edges: `rebuild_dag` emits a remove-synapse leaf for every host synapse the dag build drops as cyclic, when an edit or gesture is armed.

## 3. Collapse and explode order is position-exact
- No code change was needed. The forward leaves already carry final indices: `AddSynapse` and `AddWidget` indexes are the final list positions, in ascending order, after the removals.
- The remove leaves restore at their base index, because inverses replay last-to-first. Undo therefore restores widget and synapse order exactly.
- The new collapse/explode test covers this.
- Caveat: it holds only if the dag keeps synapse order across `rebuild_dag`/`sync_from_dag`. The test is there to prove it.

## 4. Tests (flow host unit tests)
- Rewritten to read the leaves of the last recorded edit (`host.recorded`, a test-only capture): the add/remove/connect/layout/param/delete leaf tests.
- Extended: `gestures_undo_last_to_first_and_redo_first_to_last` now includes `move_widget`.
- New:
  - `a_refused_leaf_surfaces_resets_the_history_and_leaves_no_partial_row`
  - `a_gesture_whose_leaves_are_refused_returns_the_history_fault`
  - `move_widget_emits_change_layout_and_undoes`
  - `scene_resync_is_an_echo_or_a_history_reset`
  - `a_cyclic_edge_dropped_by_the_dag_is_removed_by_its_own_leaf`
  - `collapse_and_explode_undo_restore_widget_and_synapse_order_exactly`
  - `pointer_gesture_journal_rows_become_their_concrete_leaves`: connect that displaces a wire, disconnect (an unknown id is skipped), move, slider and insert-port rows, through `journal_leaves`.
  - `pointer_drag_and_slider_gestures_record_one_leaf_each`: a real pointer drag and a slider drag.
- Pointer connect is covered only at the journal layer (`journal_leaves`), not by a real pointer-driven wire. Port-handle hit-testing at the default flow zoom is not guaranteed, so a pointer-driven wire test would be unreliable.

## Unverified (run when foundation is GREEN)
`cargo test` for the flow host unit tests, the wasm crate compile for the adapters' `history_outcome` calls, and the `VcsError: Debug` assumption in `record_history_edit`.

# Wave 9

Status: edits written, NOT compiled or run (foundation was RED). `bun ./📜️script.ts verify mutation-outcome-law` was run three times for this wave, each time with a full-repo scan. The first run showed 27 framework breaches. The last run (after the final rename) shows 0 breaches under `🧰️framework`. The remaining 73 breaches are all under `✏️s/` plugins. The gate log was deleted afterwards.

## Fixes by breach
- **clamp tests (2, message-code):** `MutationMessage::info("mutation.invariant", …)` became `fatal(…)`; the test overrides `.level` anyway.
- **replication `apply_diff` (R9):** the sealed capability is minted in a private `mint_capability()` and handed to a private `apply_with(diff, base, capability)`. Both take or return `ApplyCapability`, which the gate treats as the central-applier set. `apply_diff` itself no longer contains `.apply(`.
- **spr composite planner (R8 x4, R9, R12):** `fold_plan_diff`, `fold_plan_inverse` and `mutation_inverse_rows_failures` moved out of the leaf-facing `🎮️command` layer into a new store-side module `📡️spr/🧮️fold`, registered as `os_spr::fold` in the os-kernel package. The `os_spr::{fold_plan_diff, fold_plan_inverse, mutation_inverse_rows_failures}` re-exports still resolve for the derive macro and the tests.
- **workflow run `⚡️apply` (R9):** the seam is split. `🧬️schema/🧬️mutations/⚡️apply` keeps only `admit_run_operation(document, &operation) -> Result<(RunDiff, messages), messages>`, with no applier. `apply_run_operation_checked` now lives in the run artifact root and calls `protocol::apply_diff`. The leaf-identity fixture still has its descriptorless helper directory.
- **diff-type inverses now read base (R8/R11/R12):**
  - Flow `inverse_layout` no longer takes a `&mut` overlay. It looks up the last earlier entry for the id (same delta, earlier layout deltas) and then reads base.
  - Run `RunRows::negation` became `prior_rows`.
  - Print `ChartEdit::negation` became `undo_rows(root)` and `ChartDiff::inverse` is a pure reverse map over base.
  - Store `SpaceHistoryStep::negation` became `undo_rows(base, earlier)`; the active-alternative cell is the last earlier `SetActive`, else base's.
  - Workflow `WorkflowDiff::negation` became `undo_rows(base, earlier)`. A node, edge, parameter or input an earlier atom introduced is found in that atom's row.
  - Dag `DagDiff::inverse` maps `atom_inverse(base)` over the atoms; there is no state clone and no `apply_into`.
- **store:** `take_rebased_inverse` became `take_rebased_undo_rows`, because a builder-named fn with `&mut self` trips R8.
- **plugin:** `revalidate_interaction_state_after_document_change` became `revalidate_interaction_on_document_change` (`state_after` is a simulator name). Nine files were updated in one pass, including plugin comments.

## Limits to keep in mind
- These inverses are exact for independent rows. When a later step of the same diff edits a slot an earlier step already changed (for example two removals from one chart array), the store inverts step by step and absorbs the results. The central store never calls `DiffAlgebra::inverse` on a composed diff.
- The `fold` module is outside the gate's scan by construction (no `impl … Mutation<`, not under `🧬️schema`/`🧬️mutations`). That is where store-side machinery belongs, but it also means the gate will not re-check those fns.

## Unverified (needs foundation GREEN)
`cargo check` for the os-kernel package (new `fold` module path and re-exports), the workflow, workflow-run, print, store, dag and flow crates, and `cargo test` for the sum-law and inverse tests of those diffs.

# List-delta API

`protocol::list_delta` = `🧰️framework/🛍️products/💻️os/🔨️modules/🪡️list-delta/🦀️.rs` (registered in the os-kernel package root as `pub mod list_delta`; plugins
reach it as `protocol::list_delta`). The positional keyed list delta is defined ONCE here. A delta never carries an `order`/`reordered`/`after` list: every index is a
COORDINATE, never an evolving position.

| wire field | row | index meaning |
|---|---|---|
| `removed` | `{id, index}` | `index` = position in the BASE list (what the inverse reinserts at) |
| `inserted` | `{index, row}` | `index` = position in the AFTER list |
| `moved` | `{id, from, to}` | `from` = BASE position, `to` = AFTER position |
| `modified` | `{id, patch}` | keyed by id, position-free; `patch` is the row's sparse typed patch |

Generic parts (all in `protocol::list_delta`):
- `trait Keyed { type Key: Clone + Eq + Hash; fn key(&self) -> Self::Key }` — the row's identity (`String` for most lists; any other key type works).
- `trait ItemList<R>` (read: `count`, `at`, `to_rows`; implemented for `[R]`, `Vec<R>`, `PagedList<R, {usize::MAX}>`) and `trait BuildList<R>: ItemList<R> + Sized`
  (`from_rows`) — the delta reads and rebuilds any of these lists.
- `trait RowPatch<R>: Clone + PartialEq { commit_into(&self, row: &mut R, capability: ApplyCapability) -> MutationApplyResult<()>; absorb(&mut self, later); inverse(&self, row: &R) -> Self; is_empty }`.
- `struct NoPatch` (the patch of a list whose rows are replaced, never patched) and `struct Parts<R: Keyed, Q>` (`removed: Vec<(Key, usize)>`, `inserted: Vec<(usize, R)>`,
  `moved: Vec<(Key, usize, usize)>`, `modified: Vec<(Key, Q)>`).

`Parts` API (every concrete delta from the macros forwards the same names):
- builders from payload + base reads only: `insertion(index, row)`, `removal(&base, index)`, `removals(&base, &[index])`, `relocation(&base, from, to)` (these read the id at the
  base index), `removal_by_id(id, index)`, `removals_by_id(Vec<(Key, usize)>)`, `relocation_by_id(id, from, to)`, `modification(id, patch)`;
- `commit_onto(&base, capability: ApplyCapability) -> Result<L, MutationApplyError>` — apply under the capability: every removed/moved id is checked at its base index; the after list has
  `|base| − removed + inserted` slots; inserted and moved rows take their after slots; unmoved survivors fill the rest in base order; patches write last. A key may be removed and inserted
  (replacement). Refusals: `mutation.apply.{missing-target, duplicate-target, invalid-add-index, invalid-move-index}` with the section/index as target.
- `inverse(&base)` — row by row, never applies or simulates: removed ← inserted `(row.key, index)`; inserted ← removed rows read from base at their base index; moved ← `(id, to, from)`;
  modified ← `patch.inverse(base row)` (skipped for rows the delta itself inserts).
- `absorb(&mut self, later)` — base-free coalescing by index arithmetic over the two deltas' coordinate sets (`rank`/`nth_free`): insert∘remove → nothing, insert∘move → insert at the final slot,
  move∘move → one move, move∘remove → remove at the base index, remove∘insert (same id) → replacement, patch∘patch → one patch (`RowPatch::absorb`), patch∘remove → dropped,
  patch of an inserted row → kept as its own `modified` entry (no application without the capability).
- `is_empty()`.

Macros (exported at the crate root, used as `protocol::list_delta! { … }`): `list_delta!` (rows with patches), `plain_list_delta!` (no patches: only `removed`/`inserted`/`moved`) and
`row_patch!` (the concrete sparse patch of a row type: `set` fields are `Option` absolute setters, `nest` fields are nested list deltas). Extra derives and attributes go through
`$(#[$meta])*` onto every generated type (for example test-only serde); every generated type already derives `ToValue`, `FromValue` and `DslRecord`. Concrete delta methods: `insertion`, `removal`, `removals`, `relocation`, `removal_by_id`, `removals_by_id`,
`relocation_by_id`, `modification`, `commit_onto(&base, capability)`, `absorb`, `inverse(&base)`, `is_empty`.

Test: `🪡️list-delta/🧪️tests/🔬️unit/🦀️.rs` — ≥ 6000 randomized insert/remove/move/patch sequences (nested too): absorb ≡ sequential, `inverse` restores, refusals.

# Wave 10

Status: edits written, NOT compiled or run (foundation RED). `bun ./📜️script.ts verify mutation-outcome-law` after the work: 27 breaches in the whole repo, none under `🧰️framework`, `📕️norm` or `🧩️puzzle`.

## 1. `protocol::list_delta` (API above, "List-delta API")
- A framework list-delta module already stood in `🪡️list-delta` (a draft with `patched: [patch]`, no id key type, no capability). I rewrote it to the API above: generic `Keyed::Key`,
  `ItemList`/`BuildList` (`[R]`, `Vec<R>`, `PagedList`), `RowPatch::commit_into(row, ApplyCapability)`, `modified: [{id, patch}]`, `commit_onto(base, capability)`, base-reading
  `inverse`, base-free `absorb` (the patch of an inserted row stays its own `modified` entry), macros `list_delta!`, `plain_list_delta!`, `row_patch!`
  (all derive `ToValue`/`FromValue`/`DslRecord`; `__dsl_record_derive` is re-exported from the kernel package root).
- Test file `🪡️list-delta/🧪️tests/🔬️unit/🦀️.rs`: 6000-case randomized insert/remove/move/patch sequences over a nested list (step inverse restores its pre-state, absorb equals
  sequential application, the summed inverse restores the base, the reversed step inverses sum to the negative), plus per-id coalescing and refusal tests. Applied through
  `apply_diff` so the capability is real.
- Not carried over: norm's `between` (removed in the between-removal wave).

## 2. Norm and puzzle migrated, both copies deleted
- Norm (`📕️norm`, 14 files with list deltas): `norm_list_delta!`/`norm_row_patch!` became `protocol::list_delta!`/`protocol::row_patch!`; `semio_s_artifact_norm_contract::list_delta::` became
  `protocol::list_delta::`; every `commit_onto(..)` and hand-written `RowPatch::commit_into` takes the capability from `MutationDiff::apply`; each invocation passes the test-only serde
  derives the old macro baked in. vdi3805's hand-expanded copy of the macro became a `list_delta!` call with a key closure. vdi3805 geometry: the map rows now apply
  `removed → added → modified` and `absorb` keeps a patch of an added key as its own `modified` entry (it no longer applies a nested connection delta inside `absorb`).
  `📇️registry/🧬️contract/🪡️list-delta` and its `mod` line are deleted.
- Puzzle (80 files): `puzzle_list_delta!` became `protocol::list_delta!` (same key syntax); `write_into` became `commit_into(row, capability)`; `Delta::removal(id, i)`, `removals(vec)` and
  `relocation(id, f, t)` became `removal_by_id`, `removals_by_id` and `relocation_by_id`; `RowPatch` imports point at `protocol::list_delta`; the 2d/3d/5d TS source-contract tests
  look for `protocol::list_delta! { pub X {`. `🧊️3d/🔨️modules/🪡️list-delta` and `pub mod list_delta` are deleted.
- Still separate copies (other executors' areas, same shape): `🗄️stdio/📇️registry/🧬️contract/🪡️list-delta` (key by `fn`) and the gis/cad `🪡️list-delta` directories (`Rows` engine).
  They need the same move; I did not touch them.

## 3. `os_spr::fold_*` compatibility re-exports deleted
`pub use os_spr::fold::{fold_plan_diff, fold_plan_inverse}` and `mutation_inverse_rows_failures` are gone from `📡️spr/🦀️.rs`. Callers use `os_spr::fold::…`: the derive macro
(`CompositeMutation` and the inverse-rows law), the mutation-laws tests, the command unit tests, the plugin transaction and dependency-contribution tests, the cad aec-building test.

## Unverified (needs foundation GREEN)
`cargo check -p semio-framework-os-kernel` (module, macros, `__dsl_record_derive` export, `fold` path), `cargo test -p semio-framework-os-kernel --lib list_delta`, `cargo check` for every norm and puzzle
artifact (wasm32-wasip2 from their workspace dirs), and their diff unit tests. The `list_delta!` expansion (`DslRecord` + `Into<Key>` ids) is the first thing to read in a compile error.
