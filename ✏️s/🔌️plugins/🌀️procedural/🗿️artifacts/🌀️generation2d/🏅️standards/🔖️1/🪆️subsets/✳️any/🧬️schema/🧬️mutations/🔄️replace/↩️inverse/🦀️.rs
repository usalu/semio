//! ↩️ Inverse for `ReplaceSynapse`, reconstructed from BASE.
use super::ReplaceSynapse;
use crate::replace_synapse;
use semio_s_artifact_procedural_2d_model::synapse_index;

use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ReplaceSynapse>>(payload: &ReplaceSynapse, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    match synapse_index(&base.host_snapshot, &payload.synapse.id) {
        Some(index) => vec![replace_synapse(base.host_snapshot.synapses[index].clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
