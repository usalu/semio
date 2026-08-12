//! 🪦️ Orphaned by ticket `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL` — the generic `AddRelationship`
//! variant it backed no longer exists on `WiresMutation` (replaced by the semantic `connect-nodes`
//! mutation at `🧬️mutations/🔗connect-nodes/`). Kept physically present, empty, only because the
//! plugin-shared `📦️glue.rs` (outside this facet's edit boundary) still `#[path]`-wires this exact
//! file as `mutations::add_relationship::mutation` — deleting it would break that `#[path]`
//! reference. Once `glue.rs` drops its `pub mod add_relationship { ... }` block (tracked as a
//! `sharedFileRequests` entry in this ticket's wave2 report), this whole `➕add-relationship/`
//! directory can be deleted outright.
