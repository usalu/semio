//! ↩️ `update-synapse` inverse — captures the pre-state edge from `base` and re-`update-synapse`s
//! back to it (self-inverse); missing target ⇒ nothing to undo.

use crate::standards::v1::subsets::any::schema::mutations::update_synapse::UpdateSynapse;
use crate::standards::v1::subsets::any::schema::mutations::{synapse_index,Generation3dMutation};

use crate::Generation3dSnapshot;

/// ↩️ Missing id in `base` ⇒ `Vec::new()`.
pub fn inverse(payload: &UpdateSynapse, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match synapse_index(&base.host_snapshot, &payload.synapse.id) {
        Some(index) => vec![Generation3dMutation::UpdateSynapse(UpdateSynapse { synapse: base.host_snapshot.synapses[index].clone() })],
        None => Vec::new(),
    }

    })())
}
