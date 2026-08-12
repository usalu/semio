//! 🪦️ Orphaned by 26/08/12/SEMANTIC-MUTATIONS-OVERHAUL — `Iso16757Mutation::SetSnapshot` is
//! banned outright (whole-document replace has NO mutation-enum replacement, see
//! `📓️taxonomy.md`; `store::ArtifactStore::reset` is the sanctioned non-history path). This file
//! stays present only because `📦️glue.rs` (plugin-shared, outside this facet's boundary) still
//! `#[path]`-wires it; see this ticket's wave2 report `sharedFileRequests` for the glue.rs cleanup
//! this orphaning needs (delete the `set_snapshot` module block entirely).

use crate::artifacts::iso16757::Iso16757Snapshot;

pub fn apply(base: &mut Iso16757Snapshot, replacement: &Iso16757Snapshot) {
    *base = replacement.clone();
}
