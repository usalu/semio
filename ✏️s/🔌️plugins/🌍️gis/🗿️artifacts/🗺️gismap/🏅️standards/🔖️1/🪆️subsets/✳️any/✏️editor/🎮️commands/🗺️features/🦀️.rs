//! 🗺️ GIS 2D play app commands — the document-mutating feature patches (positions and routes).

use crate::editor::gis2d::config::{Gis2dConfig, Gis2dConfigMutation};
use crate::mutations::replace_route_data;
use crate::op::GisMapMutation;
use crate::schema::{gis_map_document_from_descriptor_json, positions_operations};
use crate::GisMapSnapshot;
use dsl::DslValue;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};
use serde_json::{json, Value};

//#region 🔖️RouteHelpers
/// 🌉️ Shared `patchRoutes`/`patchRoute` implementation — a single route id (`patchRoute`) is just a
/// one-element slice of the many-route form (`patchRoutes`).
pub fn patch_routes_operations(document: &GisMapSnapshot, route_ids: &[String], field: &str, value: &str) -> Emit<GisMapMutation, Gis2dConfigMutation> {
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

    pub fn handle(payload: &PatchPositions, doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
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

    pub fn handle(payload: &PatchRoutes, doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
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

    pub fn handle(payload: &PatchRoute, doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        Ok(patch_routes_operations(doc.snapshot, std::slice::from_ref(&payload.route_id), &payload.field, &payload.value))
    }
}
//#endregion 🔖️PatchRoute

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
