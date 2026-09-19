//! 🗺️ GIS 2D play app commands — the document-mutating feature verbs. `patchPositions`/`patchRoutes`/
//! `patchRoute` are the bulk import/patch surface; `addFeature`/`moveFeature`/`renameFeature`/
//! `deleteFeature` are the per-feature editing vocabulary the Actions rail can stage.

use crate::mutations::replace_route_data;
use crate::op::GisMapMutation;
use crate::schema::{gis_map_document_from_descriptor_json, positions_operations, regions_operations, routes_operations};
use crate::{GisMapSnapshot, MapFeature};
use dsl::DslValue;
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

/// 🧬️ Diffs one collection's before/after into that collection's own create/replace-data/delete
/// triplet — the SAME granular path `patchPositions` and `setActiveExample` take, so every editing
/// verb below inherits the leaves' authored `diff`/`inverse` and is undoable by construction.
fn collection_operations(collection: &str, before: &[MapFeature], after: &[MapFeature]) -> Vec<GisMapMutation> {
    match collection {
        "positions" => positions_operations(before, after),
        "routes" => routes_operations(before, after),
        "regions" => regions_operations(before, after),
        _ => Vec::new(),
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

/// ✏️ Writes one `data` key, appending it when the payload does not carry it yet.
fn write_entry(entries: &mut Vec<(String, DslValue)>, key: &str, value: DslValue) {
    match entries.iter_mut().find(|(entry_key, _)| entry_key == key) {
        Some((_, slot)) => *slot = value,
        None => entries.push((key.to_string(), value)),
    }
}

/// 🧭️ The map anchor of a feature payload: an explicit `lon`/`lat` pair, else the first vertex of a
/// `points` polyline/ring.
fn feature_anchor(entries: &[(String, DslValue)]) -> Option<(f64, f64)> {
    let read = |key: &str| entries.iter().find(|(entry_key, _)| entry_key == key).and_then(|(_, value)| value.as_f64());
    if let (Some(lon), Some(lat)) = (read("lon"), read("lat")) {
        return Some((lon, lat));
    }
    let points = entries.iter().find(|(key, _)| key == "points").and_then(|(_, value)| value.as_array())?;
    let first = points.first().and_then(DslValue::as_array)?;
    Some((first.first()?.as_f64()?, first.get(1)?.as_f64()?))
}

/// 🚚️ Translates every `points` vertex by `(delta_lon, delta_lat)`, leaving non-numeric vertices alone.
fn translate_points(value: &DslValue, delta_lon: f64, delta_lat: f64) -> DslValue {
    let Some(points) = value.as_array() else { return value.clone() };
    DslValue::Array(
        points
            .iter()
            .map(|point| match (point.as_array().and_then(|pair| pair.first()).and_then(DslValue::as_f64), point.as_array().and_then(|pair| pair.get(1)).and_then(DslValue::as_f64)) {
                (Some(lon), Some(lat)) => DslValue::Array(vec![DslValue::float(lon + delta_lon), DslValue::float(lat + delta_lat)]),
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

/// 🦠️ Rebuilds one collection with `edit` applied to the addressed feature and diffs it — the shared
/// body of `moveFeature`/`renameFeature`.
fn edited_collection_operations(document: &GisMapSnapshot, collection: &str, feature_id: &str, edit: impl Fn(&mut Vec<(String, DslValue)>, &MapFeature)) -> Vec<GisMapMutation> {
    let Some(before) = collection_features(document, collection) else { return Vec::new() };
    let Some(target) = addressed_feature(before, feature_id) else { return Vec::new() };
    let mut entries = data_entries(target);
    if entries.is_empty() {
        return Vec::new();
    }
    edit(&mut entries, target);
    let after: Vec<MapFeature> = before.iter().map(|feature| if feature.id == target.id { MapFeature { id: feature.id.clone(), data: DslValue::Object(entries.clone()) } } else { feature.clone() }).collect();
    collection_operations(collection, before, &after)
}
//#endregion 🔖️FeatureCollections

//#region 🔖️AddFeature
pub mod add_feature {
    use super::*;

    /// 🆕️ Appends a feature to one document collection. `collection` picks the triplet, the id is
    /// minted, and `(lon, lat, span)` seed the geometry that collection needs.
    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
        let mut after = before.to_vec();
        after.push(MapFeature { id, data: DslValue::from(&data) });
        Ok(Emit::mutations(collection_operations(&payload.collection, before, &after)))
    }
}
//#endregion 🔖️AddFeature

//#region 🔖️MoveFeature
pub mod move_feature {
    use super::*;

    /// 🚚️ Moves the addressed feature's anchor to `(lon, lat)` — a point is re-seated, a polyline or
    /// ring is translated whole so its shape survives the move.
    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "move-feature")]
    pub struct MoveFeature {
        pub collection: String,
        pub feature_id: String,
        pub lon: f64,
        pub lat: f64,
    }

    pub fn handle(payload: &MoveFeature, doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<GisMapMutation, NoConfigMutation>, Fault> {
        let operations = edited_collection_operations(doc.snapshot, &payload.collection, &payload.feature_id, |entries, _| {
            let Some((lon, lat)) = feature_anchor(entries) else { return };
            let (delta_lon, delta_lat) = (payload.lon - lon, payload.lat - lat);
            if entries.iter().any(|(key, _)| key == "lon") {
                write_entry(entries, "lon", DslValue::float(payload.lon));
                write_entry(entries, "lat", DslValue::float(payload.lat));
            }
            if let Some((_, points)) = entries.iter_mut().find(|(key, _)| key == "points") {
                *points = translate_points(points, delta_lon, delta_lat);
            }
        });
        Ok(Emit::mutations(operations))
    }
}
//#endregion 🔖️MoveFeature

//#region 🔖️RenameFeature
pub mod rename_feature {
    use super::*;

    /// 🏷️ Retitles the addressed feature — the attribute half of feature editing, kept apart from the
    /// geometry half so each verb does exactly one thing.
    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
        let operations = edited_collection_operations(doc.snapshot, &payload.collection, &payload.feature_id, |entries, _| {
            write_entry(entries, "label", DslValue::String(payload.label.clone()));
            if entries.iter().any(|(key, _)| key == "name") {
                write_entry(entries, "name", DslValue::String(payload.label.clone()));
            }
        });
        Ok(Emit::mutations(operations))
    }
}
//#endregion 🔖️RenameFeature

//#region 🔖️DeleteFeature
pub mod delete_feature {
    use super::*;

    /// 🗑️ Removes the addressed feature from its collection.
    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "delete-feature")]
    pub struct DeleteFeature {
        pub collection: String,
        pub feature_id: String,
    }

    pub fn handle(payload: &DeleteFeature, doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<GisMapMutation, NoConfigMutation>, Fault> {
        let Some(before) = collection_features(doc.snapshot, &payload.collection) else { return Ok(Emit::default()) };
        let Some(target) = addressed_feature(before, &payload.feature_id) else { return Ok(Emit::default()) };
        let after: Vec<MapFeature> = before.iter().filter(|feature| feature.id != target.id).cloned().collect();
        Ok(Emit::mutations(collection_operations(&payload.collection, before, &after)))
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
    let dsl_value = DslValue::String(value.to_string());
    let operations: Vec<GisMapMutation> = document
        .routes
        .iter()
        .filter(|route| route_ids.iter().any(|id| id == &route.id))
        .filter_map(|route| {
            let mut data = route.data.clone();
            let DslValue::Object(entries) = &mut data else {
                return None;
            };
            if let Some((_, slot)) = entries.iter_mut().find(|(key, _)| key == field) {
                *slot = dsl_value.clone();
            } else {
                entries.push((field.to_string(), dsl_value.clone()));
            }
            Some(GisMapMutation::ReplaceRouteData(replace_route_data::ReplaceRouteData { id: route.id.clone(), new_data: data }))
        })
        .collect();
    Emit::mutations(operations)
}
//#endregion 🔖️RouteHelpers

//#region 🔖️PatchPositions
pub mod patch_positions {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "patch-positions")]
    pub struct PatchPositions {
        pub positions_json: String,
    }

    pub fn handle(payload: &PatchPositions, doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<GisMapMutation, NoConfigMutation>, Fault> {
        let Ok(positions) = serde_json::from_str::<Value>(&payload.positions_json) else {
            return Ok(Emit::default());
        };
        let next = gis_map_document_from_descriptor_json(&json!({ "positions": positions }).to_string()).positions;
        Ok(Emit::mutations(positions_operations(&doc.snapshot.positions, &next)))
    }
}
//#endregion 🔖️PatchPositions

//#region 🔖️PatchRoutes
pub mod patch_routes {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
