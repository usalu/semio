//! 🎨️ GIS 2D play app command — loading a bundled example map.

use crate::apps::gis2d::config::{Gis2dConfig, Gis2dConfigMutation};
use crate::apps::gis2d::maphost::map_host_from;
use crate::artifacts::gismap::engine::default_document;
use crate::artifacts::gismap::op::GisMapMutation;
use crate::artifacts::gismap::GisMapSnapshot;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetActiveExample
/// ✏️ Replaces document content via a `SetSnapshot` operation, so this is an Operation action (not a
/// View one) — an empty `example_id` clears the map, any other id loads the bundled reuse map and
/// frames it.
pub mod set_active_example {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    pub fn handle(payload: &SetActiveExample, _doc: &DocumentView<'_, GisMapSnapshot>, cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        let next = if payload.example_id.is_empty() { GisMapSnapshot::default() } else { default_document() };
        let mut config_mutations = vec![Gis2dConfigMutation::SetSelection { ids: Vec::new() }];
        if !payload.example_id.is_empty() {
            let mut host = map_host_from(&next, cfg.snapshot);
            host.fit_world_camera();
            config_mutations.push(Gis2dConfigMutation::SetCamera { camera_json: host.camera_json() });
        }
        Ok(Emit { document_mutations: vec![GisMapMutation::SetSnapshot { snapshot: next }], config_mutations, ..Default::default() })
    }
}
//#endregion 🔖️SetActiveExample

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::gis2d::testkit::{app, app_with_registry, dispatch};
    use crate::apps::gis2d::Gis2dCommand;
    use semio_framework_plugin::PluginApp;

    #[test]
    fn set_active_example_empty_then_reuse_round_trips_document() {
        let mut app = app();
        assert!(!app.snapshot().expect("projection").positions.is_empty());
        dispatch(&mut app, Gis2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: String::new() }));
        assert!(app.snapshot().expect("projection").positions.is_empty());
        dispatch(&mut app, Gis2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "reuse-map".into() }));
        assert!(!app.snapshot().expect("projection").positions.is_empty());
        app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).expect("undo");
        assert!(app.snapshot().expect("projection").positions.is_empty(), "undo returns to the empty document");
    }

    /// 🧬️ `setActiveExample` replaces document content with `SetSnapshot` operations, so it MUST be declared as
    /// an Operation. Under the real registry the View/Shell → emits-operations guard rejects a mis-declaration;
    /// this proves the corrected declaration lets the document-replacing edit flow through without erroring.
    #[test]
    fn set_active_example_is_operation_under_registry_kind_discipline() {
        let definition = crate::apps::gis2d::create_gis2d_app().definition;
        let action = definition.actions.iter().find(|action| action.id == "setActiveExample").expect("setActiveExample declared");
        assert!(matches!(action.kind, semio_framework_plugin::ActionKind::Mutation), "loading an example emits SetSnapshot operations, so it is a Mutation");
        assert!(!action.args.is_empty(), "the palette stages the example choice via a declared select arg");

        let mut app = app_with_registry();
        let result = dispatch(&mut app, Gis2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: String::new() }));
        assert_eq!(result.mutations.len(), 1, "loading an example is one document-replacing edit");
        assert!(app.snapshot().expect("projection").positions.is_empty(), "the empty example clears every position feature");
    }
}
//#endregion 🧪️Tests
