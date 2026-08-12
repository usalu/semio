//! 🪦 ProgramSnapshot mutation — `adjacencies` leaf, orphaned.
//! The generic `Adjacencies(CollectionMutation<EntityId, Adjacency, AdjacencyPatch>)` vocabulary
//! this leaf used to back is superseded by the semantic `connect-adjacency`/`disconnect-adjacency`
//! pair (`🗺️set-adjacency`/`🧹clear-adjacency` leaves) per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 4
//! (relationship/edge collections use `connect`/`disconnect`, never a bare id-keyed CRUD quad).
//! Kept as a physically-present empty stub only because `📦️glue.rs` (outside this facet's package
//! boundary) still `#[path]`-wires this exact directory — mirrors the same pattern the
//! demonstrator/playground facet's orphaned `🖼️set-snapshot`/`🫙no-mutation` leaves already
//! document. Directory removal is a `sharedFileRequests` item for the `glue.rs` reconciliation pass.
