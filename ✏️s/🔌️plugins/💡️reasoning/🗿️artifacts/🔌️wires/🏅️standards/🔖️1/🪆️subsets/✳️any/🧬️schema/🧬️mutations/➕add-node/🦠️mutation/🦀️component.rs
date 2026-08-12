//! 🪦️ Orphaned by ticket `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL` — the generic `AddNode` variant it
//! backed no longer exists on `WiresMutation` (replaced by the semantic `create-node` mutation at
//! `🧬️mutations/🌱create-node/`). Kept physically present, empty, only because the plugin-shared
//! `📦️glue.rs` (outside this facet's edit boundary) still `#[path]`-wires this exact file as
//! `mutations::add_node::mutation` — deleting it would break that `#[path]` reference. Once
//! `glue.rs` drops its `pub mod add_node { ... }` block (tracked as a `sharedFileRequests` entry in
//! this ticket's wave2 report), this whole `➕add-node/` directory can be deleted outright.
