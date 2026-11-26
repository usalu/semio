# History trunk accessor and stale test crates

## 1. Stale test crates
### `semio-os-store-tests-08`
- `LawOp` (supersede-law): declared its receiving like production vocabularies, `semio_os_dsl::original_decoded_operation_receiving!(LawOp)` (its only wire decoder is its `OpBinary`) plus `semio_os_store_one_item_publication::artifact_persisted_binary_operation!(LawOp)`. Cargo gained `semio-os-dsl` and `semio-os-store-one-item-publication`.
- `rejected-page-close`: imports `semio_framework_value::RetainedAllocationEffect`; `ErasedSnapshotRetirement::close_step` now takes `(self, body, effect)` and the owners are reached through `&mut **rejected`; the body hint is the effect's granted copy bytes.
- Result: the crate compiles.
- Runtime: every law that drives `dispatch_test_command` (viewer-head, supersede-law) fails at its first `Apply` with `original work requires its original caller loan`. That is the known gap of the standalone store harness (a bare `ArtifactStore::dispatch` has no lent Work authority; the same fault is recorded in `SHOOTING-PLUGIN-END-TO-END/native-fixtures-green.md`, item D). Not caused by this change; fixing it means moving `store/test-support::dispatch_test_command` into an original turn driver, which is a separate task. The TS twins of both laws pass (viewer-head 5/5, supersede-law 4/4).

### `semio-plugin-tests-07-2`
- Added the missing dev-dependencies (`semio-framework-async`, `semio-framework-actor`, `semio-framework-pack-json`, `semio-plugin-store-lifecycle`, `semio-plugin-testing-instance-laws`), the part-1 imports (`serde_json::Value`, `HashMap`, `HashSet`, `bounded_document_store_owners`), `InertLabelOp` receiving declarations, the instance-laws path of `project_and_retire_fixture_tree`, and threaded `cx` through `history_verb` / `history_verb_refusal`.
- The name resolution errors are gone. What remains is 177 type errors in the nine `time-travel-part-1` files, all of one cause: the plugin runtime moved to the original-turn API (`render`, `handle_action`, `settle_framework_reserved_admission` and replay `run` take a `StepContext` / return `OriginalTurnFailure` / `Option<..>`; `dispatch_emit` is gone; `StartReplay` takes an effect, not a grant). Porting them is a separate, large migration of the time-travel part-1 laws; not done here.

## 2. Trunk chain accessor
- `HistoryFold` (replication `transition`): new field `trunk_checkpoint_ids` (the trunk's checkpoint chain) and `HistoryFold::lines()` (the trunk first once it has a chain, then each named alternative, as `(id, chain)`); `alternatives` lists named alternatives only; the fold no longer pushes the trunk row (`fold.rs`, `artifact_retire_struct!` includes the new field).
- Consumers: `spr` frontier (`content_frontier`/`alternative_head`) and the CLI `--alternative` filter use `lines()`.
- Store boundary filters removed: persisted hydration (plain and original steps), text parse, and the batch fold (`store-owners-batch-fold-and-history`); `branched_alternatives` no longer filters (the envelope never holds the trunk).
- Laws: replication `the_trunk_is_a_first_class_alternative` (now the implicit trunk line), `branch_commit_and_repin_track_alternative_chains`, spr `fold_falls_back_to_positional_mutation_ids_and_zero_clock_without_meta`. TS twins: none modelled the fold's alternatives list (the TS fold twins cover supersessions and the trunk id only); unchanged.
