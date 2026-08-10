//! 📚️ Fem2d play app commands — loading a bundled example (or resetting to an empty document).

use crate::apps::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
use crate::artifacts::fem2d::op::Fem2dMutation;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use store::ArtifactDsl;

type Fem2dSnapshot = crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️SetActiveExample
pub mod set_active_example {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    /// 🌍️ Replaces the whole document (and resets config to its default — the pre-migration
    /// `Fem2dPlayApp::camera`/`result_display` reset) via `SetSnapshot` — the example choice never
    /// merges into the CURRENT document.
    pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
        let document = if payload.example_id == "default" {
            Fem2dSnapshot::parse_dsl(crate::apps::fem2d::FEM2D_EXAMPLE_DSL).unwrap_or_else(|_| crate::artifacts::fem2d::engine::empty_fem2d_snapshot())
        } else {
            crate::artifacts::fem2d::engine::empty_fem2d_snapshot()
        };
        Ok(Emit { artifact_mutations: vec![Fem2dMutation::SetSnapshot { snapshot: document }], config_mutations: vec![Fem2dConfigMutation::Snapshot { config: Fem2dConfig::default() }], ..Default::default() })
    }
}
//#endregion 🔖️SetActiveExample

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::fem2d::testkit::{dispatch, fem2d_app, fem2d_app_with_registry};
    use crate::apps::fem2d::Fem2dCommand;

    #[test]
    fn set_active_example_loads_default_fixture_2d() {
        let mut app = fem2d_app();
        let result = dispatch(&mut app, Fem2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "default".into() }));
        assert_eq!(result.mutations.len(), 1);
        assert!(!app.snapshot().expect("snapshot").nodes.is_empty(), "expected the default fixture's nodes");
    }

    /// 📚️ Driven through the manifest-registry-wired app: `setActiveExample` is declared as an
    /// Mutation, so the registry's View/Shell kind discipline must let a whole-document reset through.
    #[test]
    fn set_active_example_unknown_id_yields_empty_document_2d() {
        let mut app = fem2d_app_with_registry();
        dispatch(&mut app, Fem2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "default".into() }));
        dispatch(&mut app, Fem2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "".into() }));
        assert_eq!(app.snapshot().expect("snapshot"), crate::artifacts::fem2d::engine::empty_fem2d_snapshot());
    }

    /// 🧬️ `setActiveExample` replaces document content via `SetSnapshot` operations, so it MUST be
    /// declared as a Mutation, not a View/Shell action — the framework's "View/Shell actions must not
    /// emit operations" guard would otherwise reject it.
    #[test]
    fn set_active_example_is_declared_as_operation_2d() {
        let definition = crate::apps::fem2d::create_fem2d_app().definition;
        let action = definition.actions.iter().find(|action| action.id == "setActiveExample").expect("setActiveExample declared");
        assert!(matches!(action.kind, semio_framework_plugin::ActionKind::Mutation), "loading an example emits SetSnapshot operations, so it is a Mutation");
        assert!(!action.args.is_empty(), "the palette stages the example choice via a declared select arg");
    }
}
//#endregion 🧪️Tests
