//! 🔺️ Diff constructor for `CreateColumn`: one created column entry. The storey and the column type must exist, a storey constraint
//! must name a storey of the same building, and the authored top must lie above the base. No height is stored: it is inferred.

use super::super::elements;
use super::CreateColumn;
use super::super::placement::rise;
use crate::{Entry, ModelDiff, ModelSnapshot, TopConstraint};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateColumn, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let column = &payload.column;
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    let Some(storey) = base.storeys.get(&column.storey) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{}\" does not exist.", column.storey), ["column", "storey"]);
    };
    if !base.column_types.contains_key(&column.column_type) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Column type \"{}\" does not exist.", column.column_type), ["column", "column_type"]);
    }
    if let TopConstraint::Storey { storey: target, .. } = &column.top {
        match base.storeys.get(target) {
            None => return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{target}\" does not exist."), ["column", "top", "storey"]),
            Some(row) if row.building != storey.building => return MutationOutcome::refuse(OutcomeCode::Invariant, format!("Storey \"{target}\" belongs to another building."), ["column", "top", "storey"]),
            Some(_) => {}
        }
    }
    if !(column.position.x.is_finite() && column.position.y.is_finite()) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A column position must be finite.", ["column", "position"]);
    }
    if !column.rotation.is_finite() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A column rotation must be finite.", ["column", "rotation"]);
    }
    if !column.base_offset.is_finite() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A column base offset must be finite.", ["column", "base_offset"]);
    }
    if !rise(base, &column.storey, column.base_offset, &column.top).is_some_and(|rise| rise.is_finite() && rise > 0.0) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A column top must lie above its base.", ["column", "top"]);
    }
    MutationOutcome::new(ModelDiff::columns(payload.id.clone(), Entry::Created(column.clone())))
}
