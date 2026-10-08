//! 🔺️ Diff constructor for `SetColumn`: a sparse column patch naming only the fields that really change. The column type must
//! exist, a storey constraint must name a storey of the column's building, and the resulting authored top must still lie above
//! the base. The resolved height is inferred, never written.

use super::SetColumn;
use super::super::placement::rise;
use crate::{ColumnPatch, Entry, ModelDiff, ModelSnapshot, TopConstraint};
use protocol::{MutationOutcome, OutcomeCode};

fn changed<T: Clone + PartialEq>(value: &Option<T>, current: &T) -> Option<T> {
    value.as_ref().filter(|next| *next != current).cloned()
}

pub fn diff(payload: &SetColumn, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(column) = base.columns.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Column \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(kind) = &payload.column_type {
        if !base.column_types.contains_key(kind) {
            return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Column type \"{kind}\" does not exist."), ["column_type"]);
        }
    }
    let storey = base.storeys.get(&column.storey);
    if let Some(TopConstraint::Storey { storey: target, .. }) = &payload.top {
        match base.storeys.get(target) {
            None => return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{target}\" does not exist."), ["top", "storey"]),
            Some(row) if Some(&row.building) != storey.map(|own| &own.building) => return MutationOutcome::refuse(OutcomeCode::Invariant, format!("Storey \"{target}\" belongs to another building."), ["top", "storey"]),
            Some(_) => {}
        }
    }
    if payload.position.is_some_and(|point| !(point.x.is_finite() && point.y.is_finite())) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A column position must be finite.", ["position"]);
    }
    if payload.rotation.is_some_and(|rotation| !rotation.is_finite()) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A column rotation must be finite.", ["rotation"]);
    }
    if payload.base_offset.is_some_and(|offset| !offset.is_finite()) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A column base offset must be finite.", ["base_offset"]);
    }
    let patch = ColumnPatch {
        column_type: changed(&payload.column_type, &column.column_type),
        position: changed(&payload.position, &column.position),
        rotation: changed(&payload.rotation, &column.rotation),
        base_offset: changed(&payload.base_offset, &column.base_offset),
        top: changed(&payload.top, &column.top),
        name: changed(&payload.name, &column.name),
        ..Default::default()
    };
    if patch == ColumnPatch::default() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Column \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    if patch.base_offset.is_some() || patch.top.is_some() {
        let (offset, top) = (patch.base_offset.unwrap_or(column.base_offset), patch.top.as_ref().unwrap_or(&column.top));
        if !rise(base, &column.storey, offset, top).is_some_and(|rise| rise.is_finite() && rise > 0.0) {
            return MutationOutcome::refuse(OutcomeCode::Invariant, "A column top must lie above its base.", [if patch.top.is_some() { "top" } else { "base_offset" }]);
        }
    }
    MutationOutcome::new(ModelDiff::columns(payload.id.clone(), Entry::Patched(patch)))
}
