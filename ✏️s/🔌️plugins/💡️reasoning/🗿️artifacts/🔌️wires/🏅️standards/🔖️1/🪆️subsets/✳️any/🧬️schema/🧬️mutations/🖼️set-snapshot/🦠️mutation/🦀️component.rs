//! 🪦️ Orphaned by ticket `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL` — `SetSnapshot` is BANNED vocabulary
//! (`📓️taxonomy.md`: whole-document replace has no in-history mutation replacement; it goes through
//! `store::ArtifactStore::reset`, entirely outside the `Mutation` enum) and no longer exists on
//! `WiresMutation`. Kept physically present, empty, only because the plugin-shared `📦️glue.rs`
//! (outside this facet's edit boundary) still `#[path]`-wires this exact file as
//! `mutations::set_snapshot::mutation` — deleting it would break that `#[path]` reference. The app's
//! `🎛️apps/🔌️wires/🎮️commands/🧬️example/🦀️component.rs` (off-limits, see this ticket's wave2 report's
//! `sharedFileRequests`) still constructs `WiresMutation::SetSnapshot` and needs rewiring onto the
//! non-history reset path once glue.rs is updated. Once `glue.rs` drops its
//! `pub mod set_snapshot { ... }` block, this whole `🖼️set-snapshot/` directory can be deleted
//! outright.
