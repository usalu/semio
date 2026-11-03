use crate::insert_characteristic_basis::InsertCharacteristicBasis;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997CharacteristicBasesRows};

pub fn diff(payload: &InsertCharacteristicBasis, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.characteristic_bases.iter().any(|existing| existing.id == payload.characteristic_basis.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("CharacteristicBasis id {} already exists.", payload.characteristic_basis.id), [payload.characteristic_basis.id.clone()]);
    }
    if payload.index > base.characteristic_bases.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.characteristic_bases.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1997Diff { characteristic_bases: Some(En1997CharacteristicBasesRows::insertion(payload.index, payload.characteristic_basis.clone())), ..Default::default() })
}
