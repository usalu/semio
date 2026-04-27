# Rust change tree — summary

- Added typed `Change` children (`TypeChange`, `KitDesignChange` + `KitDesignChangeBlock` / `KitDesignAtomicsBlock`, `PieceChange`) under `KitChange.children` with serde defaults for wire back-compat.
- `KitChange::lift_flat` builds the tree from flat `Vec<ChangeKitCommand>` using an isolated twin; semantic kit commands (e.g. `DragPieces`) stay in `forward` with no design child.
- `apply_forward` / `apply_backward` walk kit forward/inverse then children DFS; `flatten_forward_commands` / `flatten_inverse_commands` used for checkpoint finalize merge and `kit_event_from_kit_change`.
- Call sites: draft transactions, finalize draft, `set_field_rpc`, `add_child_rpc`/`remove_child_rpc`, GraphQL actor (`lift` before apply); `KitEvent` scalar rustdoc restores SDL parity with `semio/graphql/schema.graphql`.
- Tests extended in `tests::change_command_rt` (tree shape, drag as kit-only, round-trip).

Verified: `cargo fmt`, `cargo test` (semio/rs, 136 passed).
