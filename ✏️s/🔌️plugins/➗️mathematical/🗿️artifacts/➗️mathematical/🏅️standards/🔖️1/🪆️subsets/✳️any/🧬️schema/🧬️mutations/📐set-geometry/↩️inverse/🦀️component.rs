//! 🪦️ Orphaned by 26/08/12/SEMANTIC-MUTATIONS-OVERHAUL — the generic `MathematicalMutation::SetGeometry`
//! was replaced by the semantic vocabulary in `../../🌀️replace-points`, `../../➕️insert-point`,
//! `../../➖️remove-point`, `../../🎯️move-point`. This file stays present only because
//! `📦️glue.rs` (plugin-shared, outside this facet's boundary) still `#[path]`-wires it; see this
//! ticket's wave2 report `sharedFileRequests` for the glue.rs cleanup this orphaning needs (delete
//! the `set_geometry` module block entirely).

use crate::artifacts::mathematical::{mutations::MathematicalMutation, MathematicalSnapshot};

pub fn inverse(_snapshot: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    Vec::new()
}
