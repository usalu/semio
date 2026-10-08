//! 🔺️ Sparse diff construction for `set-route-property`.
use super::SetRouteProperty;
use crate::diff::{GisMapDiff, GisMapFeaturesDelta};
use crate::{GisMapSnapshot, MapFeaturePatch};
use crate::schema::feature::{MapFeaturePropertyEdit, MapFeaturePropertyValue};

//#region 🔹Diff
/// 🔺️ Builds the sparse `routes` delta directly from the payload — one `modified` entry carrying exactly one property
/// edit, never the feature's whole payload. Error `target-missing` when `feature` names no route; Fatal `invariant` when
/// its payload is not an object; Warning `no-op` when the property already holds the value.
pub fn diff(payload: &SetRouteProperty, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
    let Some(existing) = base.routes.iter().find(|feature| feature.id == payload.feature) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Route \"{}\" does not exist.", payload.feature), [payload.feature.clone()]);
    };
    let Some(entries) = existing.data.as_object() else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Route \"{}\" payload is not an object.", payload.feature), [payload.feature.clone()]);
    };
    if entries.iter().any(|(key, value)| *key == payload.key && *value == payload.value) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Route \"{}\" property \"{}\" already holds the requested value.", payload.feature, payload.key));
    }
    let edit = MapFeaturePropertyEdit { key: payload.key.clone(), before: payload.before.clone(), set: Some(MapFeaturePropertyValue { value: payload.value.clone() }) };
    protocol::MutationOutcome::new(GisMapDiff {
        routes: Some(GisMapFeaturesDelta::modification(payload.feature.clone(), MapFeaturePatch { properties: vec![edit], ..Default::default() })),
        ..Default::default()
    })
}
//#endregion 🔹Diff
