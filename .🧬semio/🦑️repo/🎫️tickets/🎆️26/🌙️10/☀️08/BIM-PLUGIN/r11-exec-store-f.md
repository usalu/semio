# r11-store-f execution report (os-flow host / vcs / mesh / bridge)

Gate label `r11-store-f`, logs `🗑️generated/r11-store-f/*.txt`. Scope after the r11-store split: `🖥️host/🦀️.rs`, `🌿️vcs/🦀️.rs`, `🎒️mesh/🦀️.rs`, `🌉️bridge/🦀️.rs`, the flow host tests, plus the Flow authoring factory in the artifacts/flow crate (r11-store asked for it). `🕸️wasm` and `🧩️extensions/🕸️wasm` were done by r11-store.

## Gate results (all run through `🚦️gate.sh`)
- `cargo check -p semio-framework-os-flow --lib`: 0 errors (c29/c30).
- `cargo check -p semio-framework-os-flow --lib --tests`: 0 errors (t9/t10).
- `cargo test -p semio-framework-os-flow --lib` (test30, before the last cosmetic edits): 259 passed, 7 failed. A re-run after the cosmetic edits was blocked by a peer break in `semio-framework` test cfg (`ColdPairIngressStatus: serde::Serialize` ...), so that run is not counted.
- Failures left (none is a retirement-API failure):
  - `host::tests::a_cyclic_edge_dropped_by_the_dag_is_removed_by_its_own_leaf`: leaves are `[addSynapse, removeSynapse x3]` instead of `[addSynapse, removeSynapse]`, so `build_dag_host_snapshot_v1` drops fixture edges besides `cyc` (dag-side behaviour, 677 test ahead of code).
  - `host::tests::cold_preview_phases_preserve_original_status_and_cancellation`: fixture (677) names phases `validating`/`collectingTopology`, `PreviewTessellatePhase::from_tag` knows neither (owner feature not implemented).
  - `host::tests::snapshot_io_prerequisite_flow_gesture_journal_admits_or_restores_all_moves`: expected 255, got 0 (dag journal, infinite crate).
  - `vcs::flow_vcs_tests::retained_vcs_all_thirteen_fixture_operations_...` and `...language_neutral_vector_signatures...`: `FLOW_VCS_FEATURES` has 13 entries (with `replaceDocument`), the 677 `semantic-history` fixture has 12 (no replace mutation). The lifecycle fixtures still carry 13.
  - `wasm_session::domain_laws::{malformed_omitted_and_unknown_selection_data_remain_owned, production_reachability_fixture_and_hostile_source_census_reject_the_old_route}`: r11-store's file (selection payload decoding, legacy-route census).
- Not run: integration target `flow_mesh_pack_wire` and `--tests` for the artifacts/flow crate (blocked by the same peer break in `semio-framework`, and by artifacts/flow test modules still on the 676 API: registry tests, `io/sqlite/snapshot/tests`).

## Migrated
- `🎒️mesh`: `FromValue::from_value` target types follow `MeshData` (`HistoryFoldIndex` x3, `ComponentReferenceTable`).
- `🌉️bridge`: `outputs/inputs_from_channel_eval_json` and `infer_port_schema` use `HistoryFoldIndex`; `build_channel_eval_json` retires the supplied-input clone and the cloned `OperatorInfo` (the value layer now refuses any bare drop of those).
- `🌿️vcs`: `flow_retirement_turn` replaces `next_close_byte_demand` + `close_page`; multi-owner handoffs use `FlowRetirement::push_cold`; test imports and history calls moved to funded catalogs and identity authority.
- `🖥️host`:
  - `FlowHost.outputs/export_payloads` are `HistoryFoldIndex<String, Dictionary>`; `displaced` is a `ValueSink`.
  - `ValueSink` (pub): wrapper over `neural::ValueRetirement`; infallible, self-funded admission for displaced owners (reserve a slot until the queue accepts, then admit with the exact frame birth); grant-driven `close_step`, `retirement_demands`, `drain_cold`.
  - History store: `install_funded_member_store_owners` (the store's `reset` keeps its catalog, so no reinstall); `dispatch(.., &mut identity)` through the local `history_identity!` macro (plugin observer + ceiling).
  - `set_canvas_theme_from_json` decodes the palette overlay and calls `DagHost::set_canvas_palette`.
  - `FlowHostRetirement`: `retirement_demands(body)`, `retire_step(grant)`, `close_quoted_step()`; `close_page` and `FlowHostRetirementFault::NoCredit` removed (a grant below any quoted axis yields `Progress(default)`, depth below demand is an error). Every rung hands ONE owner inline into `moved: ControlledRetirement<HostOwner>` (`#[derive(RetireOwned)]` enum), then that frontier drains under its own quotes (`FlowHostClosePhase::Moved`, a backing phase). Receipts are checked with `fits(grant)`; `Complete` only after the terminal rung.
  - `FlowEvalSession`: `close_step(grant) -> InteractiveJobCloseStep`, `close_demands(body)`, `next_close_{copy,capacity,release,depth}_byte_demand`, `close_quoted_step()`; same inline `moved` frontier (`SessionOwner`), source leases admitted through `admit_shared_retirement` (capacity = frame birth), parked empty collections are dropped in one item.
  - `widget_blocked_ports` and `export_payload_json` retire their clones.
- artifacts/flow (asked by r11-store): `FlowAuthoringFactory` (an `ArtifactStoreOneItemPreparationFactory<FlowHostSnapshot, FlowMutation>` that prepares no retained gesture) installed by `FlowHostSnapshot::member_store_owners` through `admit_source_constructor_with_one_item_preparation`; `FlowMutation: ArtifactCanonicalJson` with ONE indexed navigator (`🌿️vcs/🧬️schema/🧬️mutations/🔏️canonical`) serving node/key lookups and the borrowed root; `prepared_operation_wire_source` = `CanonicalJson { header: b"flow", body: mutation }`. The Pack route cannot carry widgets (the borrowed Pack cursor has no `Statements` support), so canonical JSON is the borrowed route.
- Tests: `🧹️retirement` tests rewritten to quote-driven grants (starved-axis yield law, depth refusal, release exactness across copy pages, fixture `releasedBytes` kept as the independent oracle's semantic lower bound); host unit tests moved to `HistoryFoldIndex`, typed selection API, `close_quoted_step`; new `🌿️vcs/🧪️tests/🔬️flow-canonical` (3 tests, serde_json oracle, indexed vs borrowed-root equality) all pass.

## Edits outside my files
- `🔌️plugin/🦀️.rs`: `+ 'static` bounds on `artifact_pair_snapshot`, `ArtifactDocumentPayload::{snapshot, settled_snapshot}` (the store's `parse_document_pack` now requires them; this was blocking every dependent crate).

## Open items
- DAG rung of the host ladder still drives `DagHostRetirement::close_step(items, bytes)` / `DagRetirementStep` (infinite crate, old credit currency); quoted as a fixed 64 KiB copy+release page (`FLOW_HOST_DAG_CLOSE_PAGE_BYTES`) and mapped into receipts. Needs the infinite owner to migrate it to grants and honest per-axis quotes.
- `ValueSink` belongs in the neural crate (`ValueRetirement::push_*_cold`).
- The root `📜️script.ts` source-contract (`requireAll(flow, [... "store.close_owned_step(1, 4_096)" ...])`) names the 676 close text and will fail.
- artifacts/flow test modules still on the 676 API (registry tests, `io/sqlite/snapshot/tests`) - r11-store.
