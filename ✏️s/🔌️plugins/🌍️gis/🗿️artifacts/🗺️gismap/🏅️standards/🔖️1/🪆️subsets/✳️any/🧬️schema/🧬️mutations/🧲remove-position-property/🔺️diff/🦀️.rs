//! 🔺️ Sparse diff construction for `remove-position-property`.
use super::RemovePositionProperty;
use crate::diff::{GisMapDiff, GisMapFeaturesDelta};
use crate::{GisMapSnapshot, MapFeaturePatch};
use crate::schema::feature::MapFeaturePropertyEdit;

//#region 🔹Diff
/// 🔺️ Builds the sparse `positions` delta directly from the payload — one `modified` entry carrying exactly one property
/// removal. Error `target-missing` when `feature` names no position; Fatal `invariant` when its payload is not an object;
/// Warning `no-op` when the property is already absent.
pub fn diff(payload: &RemovePositionProperty, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
    let Some(existing) = base.positions.iter().find(|feature| feature.id == payload.feature) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Position \"{}\" does not exist.", payload.feature), [payload.feature.clone()]);
    };
    let Some(entries) = existing.data.as_object() else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Position \"{}\" payload is not an object.", payload.feature), [payload.feature.clone()]);
    };
    if !entries.iter().any(|(key, _)| *key == payload.key) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Position \"{}\" has no property \"{}\".", payload.feature, payload.key));
    }
    let edit = MapFeaturePropertyEdit { key: payload.key.clone(), before: None, set: None };
    protocol::MutationOutcome::new(GisMapDiff {
        positions: Some(GisMapFeaturesDelta::modification(payload.feature.clone(), MapFeaturePatch { properties: vec![edit], ..Default::default() })),
        ..Default::default()
    })
}
//#endregion 🔹Diff
