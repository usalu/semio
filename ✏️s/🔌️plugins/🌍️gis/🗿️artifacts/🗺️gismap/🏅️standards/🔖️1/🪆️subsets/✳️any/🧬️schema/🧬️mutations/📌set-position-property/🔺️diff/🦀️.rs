//! 🔺️ Sparse diff construction for `set-position-property`.
use super::SetPositionProperty;
use crate::diff::{GisMapDiff, GisMapFeaturesDelta};
use crate::{GisMapSnapshot, MapFeaturePatch};
use crate::schema::feature::{MapFeaturePropertyEdit, MapFeaturePropertyValue};

//#region 🔹Diff
/// 🔺️ Builds the sparse `positions` delta directly from the payload — one `modified` entry carrying exactly one property
/// edit, never the feature's whole payload. Error `target-missing` when `feature` names no position; Fatal `invariant` when
/// its payload is not an object; Warning `no-op` when the property already holds the value.
pub fn diff(payload: &SetPositionProperty, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
    let Some(existing) = base.positions.iter().find(|feature| feature.id == payload.feature) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Position \"{}\" does not exist.", payload.feature), [payload.feature.clone()]);
    };
    let Some(entries) = existing.data.as_object() else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Position \"{}\" payload is not an object.", payload.feature), [payload.feature.clone()]);
    };
    if entries.iter().any(|(key, value)| *key == payload.key && *value == payload.value) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Position \"{}\" property \"{}\" already holds the requested value.", payload.feature, payload.key));
    }
    let edit = MapFeaturePropertyEdit { key: payload.key.clone(), before: payload.before.clone(), set: Some(MapFeaturePropertyValue { value: payload.value.clone() }) };
    protocol::MutationOutcome::new(GisMapDiff {
        positions: Some(GisMapFeaturesDelta::modification(payload.feature.clone(), MapFeaturePatch { properties: vec![edit], ..Default::default() })),
        ..Default::default()
    })
}
//#endregion 🔹Diff
