//! ↩️ `update-synapse` inverse — captures the pre-state edge from `base` and re-`update-synapse`s
//! back to it (self-inverse); missing target ⇒ nothing to undo.

use crate::update_synapse::UpdateSynapse;
use semio_s_artifact_procedural_3d_model::synapse_index;

use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

/// ↩️ Missing id in `base` ⇒ `Vec::new()`.
pub fn inverse<Op: From<UpdateSynapse>>(payload: &UpdateSynapse, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    match synapse_index(&base.host_snapshot, &payload.synapse.id) {
        Some(index) => vec![Op::from(UpdateSynapse { synapse: base.host_snapshot.synapses[index].clone() })],
        None => Vec::new(),
    }

    })())
}
