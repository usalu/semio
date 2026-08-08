//! 📚️ FEM 3D app commands — loading a bundled example, replacing the whole document and resetting
//! view-state config back to its default.

use crate::apps::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use crate::artifacts::fem3d::op::Fem3dMutation;
use crate::artifacts::fem3d::Fem3dSnapshot;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetActiveExample
pub mod set_active_example {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[serde(rename_all = "camelCase")]
    #[dsl(keyword = "active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    /// 📚️ `"default"` loads the bundled `.fem3d` fixture; any other id resets to an empty document —
    /// fem3d only ships the one example (mirrors the pre-migration `handle_action` behavior). Also
    /// resets the whole config back to its default (camera, result display) via a `Snapshot`.
    pub fn handle(payload: &SetActiveExample, _doc: &DocumentView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation>, Fault> {
        let document = if payload.example_id == "default" {
            <Fem3dSnapshot as store::DocumentDsl>::parse_dsl(crate::artifacts::fem3d::dsl::FEM3D_EXAMPLE_TEXT).unwrap_or_default()
        } else {
            Fem3dSnapshot::default()
        };
        Ok(Emit { document_mutations: vec![Fem3dMutation::SetSnapshot { snapshot: document }], config_mutations: vec![Fem3dConfigMutation::Snapshot { config: Fem3dConfig::default() }], ..Default::default() })
    }
}
//#endregion 🔖️SetActiveExample

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::fem3d::testkit::{dispatch, fem3d_app, fem3d_app_with_registry};
    use crate::apps::fem3d::Fem3dCommand;
    use semio_framework_plugin::ActionKind;

    #[test]
    fn set_active_example_loads_default_fixture_3d() {
        let mut app = fem3d_app();
        dispatch(&mut app, Fem3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "default".into() }));
        assert!(!app.snapshot().expect("snapshot").nodes.is_empty(), "expected the default fixture's nodes");
    }

    /// 🧬️ `setActiveExample` replaces document content via `SetSnapshot` operations, so it MUST be
    /// declared as a Mutation, not a View/Shell action — the framework's "View/Shell actions must not
    /// emit operations" guard would otherwise reject it.
    #[test]
    fn set_active_example_is_declared_as_operation_3d() {
        let definition = crate::apps::fem3d::create_fem3d_app().definition;
        let action = definition.actions.iter().find(|action| action.id == "setActiveExample").expect("setActiveExample declared");
        assert!(matches!(action.kind, ActionKind::Mutation), "loading an example emits SetSnapshot operations, so it is a Mutation");
        assert!(!action.args.is_empty(), "the palette stages the example choice via a declared select arg");
    }

    #[test]
    fn set_active_example_unknown_id_resets_to_empty_document() {
        let mut app = fem3d_app_with_registry();
        dispatch(&mut app, Fem3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "default".into() }));
        dispatch(&mut app, Fem3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "nonsense".into() }));
        assert!(app.snapshot().expect("snapshot").nodes.is_empty());
    }
}
// #endregion 🧪️Tests
