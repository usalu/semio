//! 🪦️ Orphaned by ticket `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL` — the generic `RemoveEdge` variant
//! it backed no longer exists on `WiresMutation` (replaced by the semantic `disconnect-nodes`
//! mutation at `🧬️mutations/✂️disconnect-nodes/`). Kept physically present, empty, only because the
//! plugin-shared `📦️glue.rs` (outside this facet's edit boundary) still `#[path]`-wires this exact
//! file as `mutations::remove_edge::mutation` — deleting it would break that `#[path]` reference.
//! Once `glue.rs` drops its `pub mod remove_edge { ... }` block (tracked as a `sharedFileRequests`
//! entry in this ticket's wave2 report), this whole `✂️remove-edge/` directory can be deleted
//! outright.
