//! 🗺️ GIS 2D play app commands — the document-mutating feature verbs. `patchPositions`/`patchRoutes`/
//! `patchRoute` are the bulk import/patch surface; `addFeature`/`moveFeature`/`renameFeature`/
//! `deleteFeature` are the per-feature editing vocabulary the Actions rail can stage.

use crate::mutations::{create_position, create_region, create_route, delete_position, delete_region, delete_route, remove_position_property, remove_region_property, remove_route_property, replace_position_data, set_position_property, set_region_property, set_route_property};
use crate::standards::v1::subsets::any::schema::mutations::GisMapMutation;
use crate::standards::v1::subsets::any::io::text::snapshot::{gis_map_document_from_descriptor_json};
use crate::{GisMapSnapshot, MapFeature};
use semio_framework_value::DslValue;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};
use serde_json::{json, Value};

//#region 🔖️FeatureCollections
/// 🗂️ The three addressable document feature collections: `(id, native English name, German name,
/// minted-id prefix)`. The Actions rail's `collection` argument options and every per-feature editing
/// verb below enumerate exactly this table, so the palette vocabulary and the dispatch arms can never
/// drift apart.
pub const GIS_MAP_FEATURE_COLLECTIONS: &[(&str, &str, &str, &str)] =
    &[("positions", "Positions", "Positionen", "position"), ("routes", "Routes", "Routen", "route"), ("regions", "Regions", "Regionen", "region")];

/// 🔎️ Resolves a `collection` argument onto the document slice it addresses; an unknown id addresses
/// nothing, so the verb becomes a no-op instead of a fault.
fn collection_features<'a>(document: &'a GisMapSnapshot, collection: &str) -> Option<&'a [MapFeature]> {
    match collection {
        "positions" => Some(document.positions.as_slice()),
        "routes" => Some(document.routes.as_slice()),
        "regions" => Some(document.regions.as_slice()),
        _ => None,
    }
}

/// 🆕️ The collection's own `create-<noun>` kind inserting `item` at `index`; an unknown collection addresses nothing.
fn create_operation(collection: &str, index: usize, item: MapFeature) -> Option<GisMapMutation> {
    match collection {
        "positions" => Some(GisMapMutation::CreatePosition(create_position::CreatePosition { index, item })),
        "routes" => Some(GisMapMutation::CreateRoute(create_route::CreateRoute { index, item })),
        "regions" => Some(GisMapMutation::CreateRegion(create_region::CreateRegion { index, item })),
        _ => None,
    }
}

/// ✏️ The collection's own `set-<noun>-property` kind setting one payload property; a new key is inserted before
/// `before` (appended without one).
fn set_property_operation(collection: &str, feature: &str, key: &str, value: DslValue, before: Option<String>) -> Option<GisMapMutation> {
    let (feature, key) = (feature.to_string(), key.to_string());
    match collection {
        "positions" => Some(GisMapMutation::SetPositionProperty(set_position_property::SetPositionProperty { feature, key, value, before })),
        "routes" => Some(GisMapMutation::SetRouteProperty(set_route_property::SetRouteProperty { feature, key, value, before })),
        "regions" => Some(GisMapMutation::SetRegionProperty(set_region_property::SetRegionProperty { feature, key, value, before })),
        _ => None,
    }
}

/// 🧽 The collection's own `remove-<noun>-property` kind removing one payload property.
fn remove_property_operation(collection: &str, feature: &str, key: &str) -> Option<GisMapMutation> {
    let (feature, key) = (feature.to_string(), key.to_string());
    match collection {
        "positions" => Some(GisMapMutation::RemovePositionProperty(remove_position_property::RemovePositionProperty { feature, key })),
        "routes" => Some(GisMapMutation::RemoveRouteProperty(remove_route_property::RemoveRouteProperty { feature, key })),
        "regions" => Some(GisMapMutation::RemoveRegionProperty(remove_region_property::RemoveRegionProperty { feature, key })),
        _ => None,
    }
}

/// 🗑️ The collection's own `delete-<noun>` kind removing one feature.
fn delete_operation(collection: &str, id: String) -> Option<GisMapMutation> {
    match collection {
        "positions" => Some(GisMapMutation::DeletePosition(delete_position::DeletePosition { id })),
        "routes" => Some(GisMapMutation::DeleteRoute(delete_route::DeleteRoute { id })),
        "regions" => Some(GisMapMutation::DeleteRegion(delete_region::DeleteRegion { id })),
        _ => None,
    }
}

/// 🆔️ Mints the lowest unused `{prefix}-{n}` id for a collection, so `addFeature` needs no id argument
/// and stays one-click reachable from the rail.
fn minted_feature_id(features: &[MapFeature], prefix: &str) -> String {
    (1..=features.len() + 1).map(|ordinal| format!("{prefix}-{ordinal}")).find(|candidate| !features.iter().any(|feature| &feature.id == candidate)).unwrap_or_else(|| format!("{prefix}-{}", features.len() + 1))
}

/// 🎯️ Resolves the feature a per-feature verb addresses: the staged id when one is staged, otherwise
/// the collection's NEWEST entry. The rule is deterministic and reads only the payload plus the
/// document, which a retained, journaled command must be — a selection-dependent target would not
/// replay.
fn addressed_feature<'a>(features: &'a [MapFeature], feature_id: &str) -> Option<&'a MapFeature> {
    if feature_id.is_empty() {
        features.last()
    } else {
        features.iter().find(|feature| feature.id == feature_id)
    }
}

/// 🧱️ The `data` payload entries of a feature, empty when the payload is not an object.
fn data_entries(feature: &MapFeature) -> Vec<(String, DslValue)> {
    feature.data.as_object().map(<[(String, DslValue)]>::to_vec).unwrap_or_default()
}

/// 🧭️ The map anchor of a feature payload: an explicit `lon`/`lat` pair, else the first vertex of a
/// `points` polyline/ring.
fn feature_anchor(entries: &[(String, DslValue)]) -> Option<(f64, f64)> {
    let read = |key: &str| entries.iter().find(|(entry_key, _)| entry_key == key).and_then(|(_, value)| value.as_f64());
    if let (Some(lon), Some(lat)) = (read("lon"), read("lat")) {
        return Some((lon, lat));
    }
    let points = entries.iter().find(|(key, _)| key == "points").and_then(|(_, value)| value.as_array())?;
    let first = points.first().and_then(semio_framework_value::DslValue::as_array)?;
    Some((first.first()?.as_f64()?, first.get(1)?.as_f64()?))
}

/// 🚚️ Translates every `points` vertex by `(delta_lon, delta_lat)`, leaving non-numeric vertices alone.
fn translate_points(value: &DslValue, delta_lon: f64, delta_lat: f64) -> DslValue {
    let Some(points) = value.as_array() else { return value.clone() };
    semio_framework_value::DslValue::Array(
        points
            .iter()
            .map(|point| match (point.as_array().and_then(|pair| pair.first()).and_then(semio_framework_value::DslValue::as_f64), point.as_array().and_then(|pair| pair.get(1)).and_then(semio_framework_value::DslValue::as_f64)) {
                (Some(lon), Some(lat)) => semio_framework_value::DslValue::Array(vec![DslValue::float(lon + delta_lon), DslValue::float(lat + delta_lat)]),
                _ => point.clone(),
            })
            .collect(),
    )
}

/// 🌐️ The payload a newly added feature carries in each collection: a point for `positions`, a real
/// two-vertex segment for `routes`, a closed quad for `regions` — all seeded from `(lon, lat)` and the
/// `span` extent in degrees, so every collection gets geometry its renderer can actually draw.
fn minted_feature_data(collection: &str, id: &str, label: &str, lon: f64, lat: f64, span: f64) -> Value {
    match collection {
        "routes" => json!({ "id": id, "label": label, "kind": "marker", "points": [[lon, lat], [lon + span, lat + span]] }),
        "regions" => json!({ "id": id, "label": label, "kind": "marker", "points": [[lon, lat], [lon + span, lat], [lon + span, lat + span], [lon, lat + span]] }),
        _ => json!({ "id": id, "label": label, "kind": "marker", "lon": lon, "lat": lat }),
    }
}

/// 🏷️ The current value of one payload entry, if the payload carries it.
fn entry_value<'a>(entries: &'a [(String, DslValue)], key: &str) -> Option<&'a DslValue> {
    entries.iter().find(|(entry_key, _)| entry_key == key).map(|(_, value)| value)
}

/// ✏️ The `set-<noun>-property` operation for `key` when the payload does not already hold `value` there.
fn changed_property_operation(collection: &str, target: &MapFeature, entries: &[(String, DslValue)], key: &str, value: DslValue) -> Option<GisMapMutation> {
    if entry_value(entries, key) == Some(&value) {
        return None;
    }
    set_property_operation(collection, &target.id, key, value, None)
}

/// 🧱️ Per-property edits taking `before` to `after`: removed keys become `remove-<noun>-property`, changed or new keys
/// `set-<noun>-property` (new keys are inserted before the key that followed them, appended when last). Payloads that are
/// not both objects are replaced whole.
fn property_edit_operations(collection: &str, id: &str, before: &DslValue, after: &DslValue) -> Vec<GisMapMutation> {
    let (Some(old), Some(new)) = (before.as_object(), after.as_object()) else {
        return replace_data_operation(collection, id.to_string(), after.clone()).into_iter().collect();
    };
    let mut operations = Vec::new();
    for (key, _) in old.iter().filter(|(key, _)| entry_value(new, key).is_none()) {
        operations.extend(remove_property_operation(collection, id, key));
    }
    for (at, (key, value)) in new.iter().enumerate() {
        if entry_value(old, key) == Some(value) {
            continue;
        }
        let anchor = entry_value(old, key).is_none().then(|| new[at + 1..].iter().map(|(next, _)| next.clone()).find(|next| entry_value(old, next).is_some())).flatten();
        operations.extend(set_property_operation(collection, id, key, value.clone(), anchor));
    }
    operations
}

/// ✏️ The collection's own `replace-<noun>-data` kind, used only by whole-payload replacement gestures.
fn replace_data_operation(collection: &str, id: String, new_data: DslValue) -> Option<GisMapMutation> {
    match collection {
        "positions" => Some(GisMapMutation::ReplacePositionData(replace_position_data::ReplacePositionData { id, new_data })),
        "routes" => Some(GisMapMutation::ReplaceRouteData(crate::mutations::replace_route_data::ReplaceRouteData { id, new_data })),
        "regions" => Some(GisMapMutation::ReplaceRegionData(crate::mutations::replace_region_data::ReplaceRegionData { id, new_data })),
        _ => None,
    }
}

/// 🧱️ Concrete per-feature operations taking a collection from `before` to `after` for a patch gesture: features the patch
/// omits are deleted, new ones created at their index, present ones edited property by property.
fn collection_patch_operations(collection: &str, before: &[MapFeature], after: &[MapFeature]) -> Vec<GisMapMutation> {
    let mut operations: Vec<GisMapMutation> = before.iter().filter(|feature| !after.iter().any(|next| next.id == feature.id)).filter_map(|feature| delete_operation(collection, feature.id.clone())).collect();
    for (index, feature) in after.iter().enumerate() {
        match before.iter().find(|entry| entry.id == feature.id) {
            None => operations.extend(create_operation(collection, index, feature.clone())),
            Some(previous) => operations.extend(property_edit_operations(collection, &feature.id, &previous.data, &feature.data)),
        }
    }
    operations
}

//#endregion 🔖️FeatureCollections

//#region 🔖️AddFeature
pub mod add_feature {
    use super::*;

    /// 🆕️ Appends a feature to one document collection. `collection` picks the triplet, the id is
    /// minted, and `(lon, lat, span)` seed the geometry that collection needs.
    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "add-feature")]
    pub struct AddFeature {
        pub collection: String,
        pub label: String,
        pub lon: f64,
        pub lat: f64,
        pub span: f64,
    }

    pub fn handle(payload: &AddFeature, doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<GisMapMutation, NoConfigMutation>, Fault> {
        let Some((_, _, _, prefix)) = GIS_MAP_FEATURE_COLLECTIONS.iter().find(|(id, _, _, _)| *id == payload.collection) else {
            return Ok(Emit::default());
        };
        let Some(before) = collection_features(doc.snapshot, &payload.collection) else { return Ok(Emit::default()) };
        let id = minted_feature_id(before, prefix);
        let label = if payload.label.is_empty() { id.clone() } else { payload.label.clone() };
        let data = minted_feature_data(&payload.collection, &id, &label, payload.lon, payload.lat, payload.span);
        let item = MapFeature { id, data: semio_framework_value::DslValue::from(&data) };
        Ok(Emit::mutations(create_operation(&payload.collection, before.len(), item).into_iter().collect()))
    }
}
//#endregion 🔖️AddFeature

//#region 🔖️MoveFeature
pub mod move_feature {
    use super::*;

    /// 🚚️ Moves the addressed feature's anchor to `(lon, lat)` — a point is re-seated, a polyline or
    /// ring is translated whole so its shape survives the move.
    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "move-feature")]
    pub struct MoveFeature {
        pub collection: String,
        pub feature_id: String,
        pub lon: f64,
        pub lat: f64,
    }

    pub fn handle(payload: &MoveFeature, doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<GisMapMutation, NoConfigMutation>, Fault> {
        let Some(before) = collection_features(doc.snapshot, &payload.collection) else { return Ok(Emit::default()) };
        let Some(target) = addressed_feature(before, &payload.feature_id) else { return Ok(Emit::default()) };
        let entries = data_entries(target);
        let Some((lon, lat)) = feature_anchor(&entries) else { return Ok(Emit::default()) };
        let (delta_lon, delta_lat) = (payload.lon - lon, payload.lat - lat);
        let mut operations = Vec::new();
        if entries.iter().any(|(key, _)| key == "lon") {
            operations.extend(changed_property_operation(&payload.collection, target, &entries, "lon", DslValue::float(payload.lon)));
            operations.extend(changed_property_operation(&payload.collection, target, &entries, "lat", DslValue::float(payload.lat)));
        }
        if let Some(points) = entry_value(&entries, "points") {
            operations.extend(changed_property_operation(&payload.collection, target, &entries, "points", translate_points(points, delta_lon, delta_lat)));
        }
        Ok(Emit::mutations(operations))
    }
}
//#endregion 🔖️MoveFeature

//#region 🔖️RenameFeature
pub mod rename_feature {
    use super::*;

    /// 🏷️ Retitles the addressed feature — the attribute half of feature editing, kept apart from the
    /// geometry half so each verb does exactly one thing.
    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "rename-feature")]
    pub struct RenameFeature {
        pub collection: String,
        pub feature_id: String,
        pub label: String,
    }

    pub fn handle(payload: &RenameFeature, doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<GisMapMutation, NoConfigMutation>, Fault> {
        if payload.label.is_empty() {
            return Ok(Emit::default());
        }
        let Some(before) = collection_features(doc.snapshot, &payload.collection) else { return Ok(Emit::default()) };
        let Some(target) = addressed_feature(before, &payload.feature_id) else { return Ok(Emit::default()) };
        let entries = data_entries(target);
        if entries.is_empty() {
            return Ok(Emit::default());
        }
        let mut operations = Vec::new();
        operations.extend(changed_property_operation(&payload.collection, target, &entries, "label", DslValue::String(payload.label.clone())));
        if entry_value(&entries, "name").is_some() {
            operations.extend(changed_property_operation(&payload.collection, target, &entries, "name", DslValue::String(payload.label.clone())));
        }
        Ok(Emit::mutations(operations))
    }
}
//#endregion 🔖️RenameFeature

//#region 🔖️DeleteFeature
pub mod delete_feature {
    use super::*;

    /// 🗑️ Removes the addressed feature from its collection.
    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "delete-feature")]
    pub struct DeleteFeature {
        pub collection: String,
        pub feature_id: String,
    }

    pub fn handle(payload: &DeleteFeature, doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<GisMapMutation, NoConfigMutation>, Fault> {
        let Some(before) = collection_features(doc.snapshot, &payload.collection) else { return Ok(Emit::default()) };
        let Some(target) = addressed_feature(before, &payload.feature_id) else { return Ok(Emit::default()) };
        Ok(Emit::mutations(delete_operation(&payload.collection, target.id.clone()).into_iter().collect()))
    }
}
//#endregion 🔖️DeleteFeature

//#region 🔖️RouteHelpers
/// 🌉️ Shared `patchRoutes`/`patchRoute` implementation — a single route id (`patchRoute`) is just a
/// one-element slice of the many-route form (`patchRoutes`).
pub fn patch_routes_operations(document: &GisMapSnapshot, route_ids: &[String], field: &str, value: &str) -> Emit<GisMapMutation, NoConfigMutation> {
    if route_ids.is_empty() {
        return Emit::default();
    }
    let dsl_value = semio_framework_value::DslValue::String(value.to_string());
    let operations: Vec<GisMapMutation> = document
        .routes
        .iter()
        .filter(|route| route_ids.iter().any(|id| id == &route.id))
        .filter(|route| route.data.as_object().is_some_and(|entries| entry_value(entries, field) != Some(&dsl_value)))
        .filter_map(|route| set_property_operation("routes", &route.id, field, dsl_value.clone(), None))
        .collect();
    Emit::mutations(operations)
}
//#endregion 🔖️RouteHelpers

//#region 🔖️PatchPositions
pub mod patch_positions {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "patch-positions")]
    pub struct PatchPositions {
        pub positions_json: String,
    }

    pub fn handle(payload: &PatchPositions, doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<GisMapMutation, NoConfigMutation>, Fault> {
        let Ok(positions) = serde_json::from_str::<Value>(&payload.positions_json) else {
            return Ok(Emit::default());
        };
        let next = gis_map_document_from_descriptor_json(&json!({ "positions": positions }).to_string()).positions;
        Ok(Emit::mutations(collection_patch_operations("positions", &doc.snapshot.positions, &next)))
    }
}
//#endregion 🔖️PatchPositions

//#region 🔖️PatchRoutes
pub mod patch_routes {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "patch-routes")]
    pub struct PatchRoutes {
        pub route_ids: Vec<String>,
        pub field: String,
        pub value: String,
    }

    pub fn handle(payload: &PatchRoutes, doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<GisMapMutation, NoConfigMutation>, Fault> {
        Ok(patch_routes_operations(doc.snapshot, &payload.route_ids, &payload.field, &payload.value))
    }
}
//#endregion 🔖️PatchRoutes

//#region 🔖️PatchRoute
pub mod patch_route {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "patch-route")]
    pub struct PatchRoute {
        pub route_id: String,
        pub field: String,
        pub value: String,
    }

    pub fn handle(payload: &PatchRoute, doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<GisMapMutation, NoConfigMutation>, Fault> {
        Ok(patch_routes_operations(doc.snapshot, std::slice::from_ref(&payload.route_id), &payload.field, &payload.value))
    }
}
//#endregion 🔖️PatchRoute

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
