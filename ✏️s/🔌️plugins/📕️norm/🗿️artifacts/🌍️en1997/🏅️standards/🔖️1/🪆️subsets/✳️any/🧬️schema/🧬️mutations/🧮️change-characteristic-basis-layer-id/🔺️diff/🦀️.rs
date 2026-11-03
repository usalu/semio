//! 🔺️ `change-characteristic-basis-layer-id` diff: patches `layer_id` of the addressed characteristic value basis row.

use crate::change_characteristic_basis_layer_id::ChangeCharacteristicBasisLayerId;
use semio_s_artifact_norm_en1997_diff::{En1997CharacteristicBasesPatch, En1997CharacteristicBasesRows, En1997Diff};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeCharacteristicBasisLayerId, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    let Some(r0) = base.characteristic_bases.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("characteristic-basis {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.layer_id == payload.new_layer_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The characteristic value basis already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { characteristic_bases: Some(En1997CharacteristicBasesRows::modification(&payload.id, En1997CharacteristicBasesPatch { layer_id: Some(payload.new_layer_id.clone()), ..Default::default() })), ..Default::default() })
}
