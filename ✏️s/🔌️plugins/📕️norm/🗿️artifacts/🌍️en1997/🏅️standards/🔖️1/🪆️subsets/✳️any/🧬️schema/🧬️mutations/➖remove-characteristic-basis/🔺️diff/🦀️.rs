use crate::remove_characteristic_basis::RemoveCharacteristicBasis;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997CharacteristicBasesRows};

pub fn diff(payload: &RemoveCharacteristicBasis, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if payload.index >= base.characteristic_bases.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("characteristic-basis #{} missing", payload.index), vec![payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1997Diff { characteristic_bases: Some(En1997CharacteristicBasesRows::removal(&base.characteristic_bases, payload.index)), ..Default::default() })
}
