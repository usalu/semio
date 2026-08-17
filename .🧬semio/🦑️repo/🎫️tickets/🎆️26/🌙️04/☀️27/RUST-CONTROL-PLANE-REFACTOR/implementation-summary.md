# Rust change tree — summary

- Added typed `Change` children (`TypeChange`, `KitDesignChange` + `KitDesignChangeBlock` / `KitDesignAtomicsBlock`, `PieceChange`) under `KitChange.children` with serde defaults for wire back-compat.
- `KitChange::lift_flat` builds the tree from flat `Vec<ChangeKitCommand>` using an isolated twin; semantic kit commands (e.g. `DragPieces`) stay in `forward` with no design child.
- `apply_forward` / `apply_backward` walk kit forward/inverse then children DFS; `flatten_forward_commands` / `flatten_inverse_commands` used for checkpoint finalize merge and `kit_event_from_kit_change`.
- Call sites: draft transactions, finalize draft, `set_field_rpc`, `add_child_rpc`/`remove_child_rpc`, GraphQL actor (`lift` before apply); `KitEvent` scalar rustdoc restores SDL parity with `compose/graphql/schema.graphql`.
- Tests extended in `tests::change_command_rt` (tree shape, drag as kit-only, round-trip).

Verified: `cargo fmt`, `cargo test` (compose/rs, 136 passed).

# Control-plane refactor (2026-04-27)

- `ControlPlanePreBatch` + `control_plane_pre_batch`, `control_plane_batch_apply_with_undo` (lift → `with_undo`+`apply_many` → `emit_kit_change_event_bus`).
- `rpc_undo_lifting_apply_and_emit` for RPC set-field / add-child / remove-child (lift inside undo window).
- `open_transaction_apply_one_command` for `TransactionCommand::ChangeKitCommands` loop.
- `spawn_actor` `GraphWork::ChangeKitCommands` / `ChangeKitWithInverse` use `control_plane_batch_apply_with_undo`.
- GraphQL: `empty_kit_store_batch_result` + struct update for `KitStoreBatchResult`; `ChangeKitWithInverse` batch uses `control_plane_pre_batch` on transaction `view` for inverse/semantics (aligned with lift).
- Tests: `tests::control_plane` (pre-batch vs inverse, apply DTO parity, kit event JSON); `transaction_change_kit_with_inverse_row_includes_change_kind_and_inverse`; SDL line for `KitStoreMutation` doc in `compose/graphql/schema.graphql`.

Verified: `cargo fmt`, `cargo test --lib` (compose/rs, 140 passed, 1 ignored).
