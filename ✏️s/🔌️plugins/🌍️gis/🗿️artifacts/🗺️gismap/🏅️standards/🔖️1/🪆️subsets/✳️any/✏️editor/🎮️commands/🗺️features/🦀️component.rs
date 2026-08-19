//! 🗺️ GIS 2D play app commands — the document-mutating feature patches (positions and routes).

use crate::editor::gis2d::config::{Gis2dConfig, Gis2dConfigMutation};
use crate::artifacts::gismap::schema::{gis_map_document_from_descriptor_json, positions_operations};
use crate::artifacts::gismap::mutations::replace_route_data;
use crate::artifacts::gismap::op::GisMapMutation;
use crate::artifacts::gismap::GisMapSnapshot;
use dsl::DslValue;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

//#region 🔖️RouteHelpers
/// 🌉️ Shared `patchRoutes`/`patchRoute` implementation — a single route id (`patchRoute`) is just a
/// one-element slice of the many-route form (`patchRoutes`).
pub async fn patch_routes_operations(document: &GisMapSnapshot, route_ids: &[String], field: &str, value: &str) -> Emit<GisMapMutation, Gis2dConfigMutation> {
    if route_ids.is_empty() {
        return Emit::default();
    }
    let dsl_value = dsl::to_dsl_value(&json!(value)).unwrap_or(DslValue::Null);
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
            Some(GisMapMutation::ReplaceRouteData(replace_route_data::mutation::ReplaceRouteData { id: route.id.clone(), new_data: data }))
        })
        .collect();
    Emit::mutations(operations)
}
//#endregion 🔖️RouteHelpers

//#region 🔖️PatchPositions
pub mod patch_positions {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patch-positions")]
    pub struct PatchPositions {
        pub positions_json: String,
    }

    pub async fn handle(payload: &PatchPositions, doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patch-routes")]
    pub struct PatchRoutes {
        pub route_ids: Vec<String>,
        pub field: String,
        pub value: String,
    }

    pub async fn handle(payload: &PatchRoutes, doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        Ok(patch_routes_operations(doc.snapshot, &payload.route_ids, &payload.field, &payload.value))
    }
}
//#endregion 🔖️PatchRoutes

//#region 🔖️PatchRoute
pub mod patch_route {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patch-route")]
    pub struct PatchRoute {
        pub route_id: String,
        pub field: String,
        pub value: String,
    }

    pub async fn handle(payload: &PatchRoute, doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        Ok(patch_routes_operations(doc.snapshot, std::slice::from_ref(&payload.route_id), &payload.field, &payload.value))
    }
}
//#endregion 🔖️PatchRoute

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::gis2d::testkit::{app, dispatch};
    use crate::editor::gis2d::Gis2dCommand;

    const ROUTE_A: &str = "bg_holz_fassade_botanique:bw_institut_botanique_ulg:0";
    const ROUTE_B: &str = "bg_stahl_mehrere_lycee_profiles_canopy:bw_lycee_block_3000:0";

    #[semio_framework_async_macros::async_test]
    async fn patch_routes_emits_route_patch_ops_and_updates_document() {
        let mut app = app();
        let result = dispatch(&mut app, Gis2dCommand::PatchRoute(patch_route::PatchRoute { route_id: ROUTE_A.into(), field: "label".into(), value: "Renamed Route".into() }));
        assert_eq!(result.mutations.len(), 1, "one matching route → one patch operation");
        let document = app.snapshot().expect("projection");
        let route = document.routes.iter().find(|route| route.id == ROUTE_A).expect("route");
        assert_eq!(route.data.get("label").and_then(|value| value.as_str()), Some("Renamed Route"));
    }

    #[semio_framework_async_macros::async_test]
    async fn patch_routes_with_no_ids_emits_nothing() {
        let mut app = app();
        let result = dispatch(&mut app, Gis2dCommand::PatchRoutes(patch_routes::PatchRoutes { route_ids: Vec::new(), field: "label".into(), value: "x".into() }));
        assert!(result.mutations.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn patch_positions_with_malformed_json_emits_nothing() {
        let mut app = app();
        let result = dispatch(&mut app, Gis2dCommand::PatchPositions(patch_positions::PatchPositions { positions_json: "not json".into() }));
        assert!(result.mutations.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn patch_positions_diffs_the_incoming_array_into_granular_operations() {
        let mut app = app();
        dispatch(&mut app, Gis2dCommand::PatchPositions(patch_positions::PatchPositions { positions_json: r#"[{"id":"patched-1","lon":1.0,"lat":2.0}]"#.into() }));
        let document = app.snapshot().expect("projection");
        assert!(document.positions.iter().any(|feature| feature.id == "patched-1"), "the incoming array is diffed into a granular add");
        assert_eq!(document.positions.len(), 1, "features absent from the incoming array are removed");
    }

    /// 🤝️ Definitional merge proof: two instances on one backbone patch DIFFERENT routes; after
    /// exchanging operations both converge and keep both edits — impossible under whole-map LWW snapshots.
    #[semio_framework_async_macros::async_test]
    async fn two_instances_converge_on_disjoint_route_edits() {
        let command_a = Gis2dCommand::PatchRoute(patch_route::PatchRoute { route_id: ROUTE_A.into(), field: "label".into(), value: "A".into() });
        let command_b = Gis2dCommand::PatchRoute(patch_route::PatchRoute { route_id: ROUTE_B.into(), field: "label".into(), value: "B".into() });
        let label = |document: &GisMapSnapshot, id: &str| document.routes.iter().find(|route| route.id == id).and_then(|route| route.data.get("label").and_then(|value| value.as_str().map(str::to_string)));
        semio_framework_plugin::testkit::assert_two_instances_converge::<crate::editor::gis2d::Gis2dPlayApp, _>("mem://gis2d-convergence", command_a, command_b, |app| {
            let document = app.snapshot().expect("projection");
            (label(&document, ROUTE_A), label(&document, ROUTE_B))
        });
    }
}
//#endregion 🧪️Tests
