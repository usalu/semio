//! 🪦️ Orphaned by 26/08/12/SEMANTIC-MUTATIONS-OVERHAUL — `PlaygroundMutation::SetSnapshot` is
//! banned outright (whole-document replace has NO mutation-enum replacement, see
//! `📓️taxonomy.md`; `store::ArtifactStore::reset` is the sanctioned non-history path). This file
//! stays present only because `📦️glue.rs` (plugin-shared, outside this facet's boundary) still
//! `#[path]`-wires it; see this ticket's wave2 report `sharedFileRequests` for the glue.rs cleanup
//! this orphaning needs (delete the `set_snapshot` module block entirely).

use crate::artifacts::playground::PlaygroundSnapshot;

pub fn apply(snapshot: &mut PlaygroundSnapshot, replacement: &PlaygroundSnapshot) {
    *snapshot = replacement.clone();
}
