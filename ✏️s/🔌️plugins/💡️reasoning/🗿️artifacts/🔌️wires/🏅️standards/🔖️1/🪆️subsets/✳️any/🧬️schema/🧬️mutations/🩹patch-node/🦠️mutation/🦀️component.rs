//! 🪦️ Orphaned by ticket `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL` — `PatchNode`'s
//! `BTreeMap<String, DslValue>` field was exactly the forbidden option-bag `Patch` mutation payload
//! shape (`📓️taxonomy.md`'s "Forbidden vocabulary") and no longer exists on `WiresMutation`;
//! replaced by six explicit semantic mutations (`move-node`, `resize-node`, `change-node-kind`,
//! `change-node-shape`, `edit-node-text`, `set-node-root`). Kept physically present, empty, only
//! because the plugin-shared `📦️glue.rs` (outside this facet's edit boundary) still `#[path]`-wires
//! this exact file as `mutations::patch_node::mutation` — deleting it would break that `#[path]`
//! reference. Once `glue.rs` drops its `pub mod patch_node { ... }` block (tracked as a
//! `sharedFileRequests` entry in this ticket's wave2 report), this whole `🩹patch-node/` directory
//! can be deleted outright.
