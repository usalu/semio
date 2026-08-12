//! 🪦 ProgramSnapshot mutation — `set_snapshot` leaf, orphaned.
//! `SetSnapshot { snapshot: Box<ProgramSnapshot> }` is BANNED outright per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️taxonomy.md`'s closed verb
//! set: "Only `set-snapshot` (whole-document replacement) is banned — and it has NO replacement
//! mutation... whole-document replace is not expressible as an in-history mutation at all; it goes
//! through `ArtifactStore::reset`". Kept as a physically-present empty stub only because
//! `📦️glue.rs` (outside this facet's package boundary) still `#[path]`-wires this exact directory
//! — mirrors the demonstrator/playground facet's own orphaned `🖼️set-snapshot`/`🫙no-mutation`
//! leaves. Directory removal is a `sharedFileRequests` item for the `glue.rs` reconciliation pass.
