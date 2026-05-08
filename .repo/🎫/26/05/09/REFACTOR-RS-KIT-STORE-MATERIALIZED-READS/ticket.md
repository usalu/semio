# Refactor RS Kit Store To Materialized Reads

**Status:** Closed (subagent completion).

**Goal:** Kit store materialization per `.cursor/plans/rs_kit_store_materialization_86ff846b.plan.md` (plan file not edited here).

## Summary

- Fixed `Graph::apply_create_fixed_piece` to use `record_op_in_open_transaction` + `materialized_kit_for_draft` (was mutating a discarded clone).
- Golden / fingerprint tests now use `materialized_kit_for_draft` for stable projection.
- Normalized `KitOperation` tests updated for `Scope` / `Input` / `__ops` `KitDiff`; create-tag backwards uses staged clone + `apply_diff`.
- `kit_graph_engine`: `Serialize` on `CreatedFixedPiecePayload`; `serde::{Deserialize, Serialize}` import.
- GraphQL tests: `graphql_seed_defaults_and_open_tx` so mutations target the same draft as `wip.theKit` (seed draft).
- `no_deep_clone_on_traversal`: uses `ensure_default_seed_state` + `open_transaction` for draft/tx ids.
- Bundle round-trip: materialized vs frozen checkpoint + abort revert + re-rename for serialize path.
- Guard test: `worker_child_runtime_guard_no_direct_root_or_apply_diff`.

## Files touched

- `semio/rs/lib.rs`
- `.repo/🎫/26/05/09/REFACTOR-RS-KIT-STORE-MATERIALIZED-READS/ticket.md`
