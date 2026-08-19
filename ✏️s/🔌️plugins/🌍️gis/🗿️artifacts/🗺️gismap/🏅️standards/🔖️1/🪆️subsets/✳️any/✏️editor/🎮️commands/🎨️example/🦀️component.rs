//! 🎨️ GIS 2D play app command — loading a bundled example map.

use crate::editor::gis2d::config::{Gis2dConfig, Gis2dConfigMutation};
use crate::editor::gis2d::maphost::map_host_from;
use crate::artifacts::gismap::schema::{default_document, positions_operations, regions_operations, routes_operations};
use crate::artifacts::gismap::op::GisMapMutation;
use crate::artifacts::gismap::GisMapSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetActiveExample
/// ✏️ Replaces document content by diffing every collection (positions/routes/regions) into batched
/// create/delete/replace-data operations, so this is an Operation action (not a View one) — an empty
/// `example_id` clears the map, any other id loads the bundled reuse map and frames it. Never a
/// whole-document snapshot swap (that vocabulary is retired by the taxonomy): each batched operation
/// still has a real per-mutation inverse, so undo restores the prior document exactly.
pub mod set_active_example {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    pub async fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, GisMapSnapshot>, cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        let next = if payload.example_id.is_empty() { GisMapSnapshot::default() } else { default_document() };
        // 🕹️ The pre-migration layer/feature selection clear that used to live here (`SetSelection {
        // ids: Vec::new() }`) is gone — selection is framework-owned config now, and `Emit` has no
        // channel to touch the framework's `interaction_store` (ticket
        // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM). The `"features"` domain is
        // `HierarchyProvider::Flat`, so `validate_state` does not auto-prune it either — a stale
        // selection surviving a document swap is a known, accepted gap of this wave.
        let mut config_mutations = Vec::new();
        if !payload.example_id.is_empty() {
            let mut host = map_host_from(&next, cfg.snapshot);
            host.fit_world_camera();
            config_mutations.push(Gis2dConfigMutation::SetCamera { camera_json: host.camera_json() });
        }
        let document = doc.snapshot;
        let mut artifact_mutations = positions_operations(&document.positions, &next.positions);
        artifact_mutations.extend(routes_operations(&document.routes, &next.routes));
        artifact_mutations.extend(regions_operations(&document.regions, &next.regions));
        Ok(Emit { artifact_mutations, config_mutations, ..Default::default() })
    }
}
//#endregion 🔖️SetActiveExample

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::gis2d::testkit::{app, app_with_registry, dispatch};
    use crate::editor::gis2d::Gis2dCommand;
    use semio_framework_plugin::PluginApp;

    #[test]
    async fn set_active_example_empty_then_reuse_round_trips_document() {
        let mut app = app();
        assert!(!app.snapshot().expect("projection").positions.is_empty());
        dispatch(&mut app, Gis2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: String::new() }));
        assert!(app.snapshot().expect("projection").positions.is_empty());
        dispatch(&mut app, Gis2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "reuse-map".into() }));
        assert!(!app.snapshot().expect("projection").positions.is_empty());
        app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).expect("undo");
        assert!(app.snapshot().expect("projection").positions.is_empty(), "undo returns to the empty document");
    }

    /// 🧬️ `setActiveExample` replaces document content with batched create/delete/replace-data
    /// operations, so it MUST be declared as an Operation. Under the real registry the View/Shell →
    /// emits-operations guard rejects a mis-declaration; this proves the corrected declaration lets
    /// the document-replacing edit flow through without erroring.
    #[test]
    async fn set_active_example_is_operation_under_registry_kind_discipline() {
        let definition = crate::editor::gis2d::create_gis2d_app().definition;
        let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == "setActiveExample").expect("setActiveExample declared");
        assert!(matches!(action.kind, semio_framework_plugin::ActionKind::Mutation), "loading an example emits document-mutating operations, so it is a Mutation");
        assert!(!action.args.is_empty(), "the palette stages the example choice via a declared select arg");

        let mut app = app_with_registry();
        let result = dispatch(&mut app, Gis2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: String::new() }));
        assert!(!result.mutations.is_empty(), "clearing a non-empty example emits at least one delete operation per removed feature");
        assert!(app.snapshot().expect("projection").positions.is_empty(), "the empty example clears every position feature");
    }
}
//#endregion 🧪️Tests
