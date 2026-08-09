//! 🧬️ Wires play app commands — loading a named example fixture (currently just "metabolism").

use crate::apps::wires::config::{WiresConfig, WiresConfigMutation};
use crate::artifacts::wires::empty_wires_snapshot;
use crate::artifacts::wires::engine::metabolism_wires_example_snapshot;
use crate::artifacts::wires::op::WiresMutation;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

/// 🧬️ Manifest `.example` id for the metabolism fixture — shared by `SetActiveExample`'s payload check
/// and `crate::apps::wires::create_wires_app`'s `.example(...)` registration.
pub const WIRES_PLAY_EXAMPLE_METABOLISM_ID: &str = "metabolism";

//#region 🔖️SetActiveExample
pub mod set_active_example {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    pub fn handle(payload: &SetActiveExample, _doc: &DocumentView<'_, crate::artifacts::wires::WiresSnapshot>, _cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<WiresMutation, WiresConfigMutation>, Fault> {
        let next = if payload.example_id.as_str() == WIRES_PLAY_EXAMPLE_METABOLISM_ID { metabolism_wires_example_snapshot() } else { empty_wires_snapshot() };
        Ok(Emit {
            document_mutations: vec![WiresMutation::SetSnapshot { snapshot: next }],
            config_mutations: vec![WiresConfigMutation::SetSelection { ids: Vec::new() }, WiresConfigMutation::SetDrag { node_id: None, last_x: 0.0, last_y: 0.0 }],
            ..Default::default()
        })
    }
}
//#endregion 🔖️SetActiveExample

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::wires::testkit::{dispatch, metabolism_app, new_app};
    use crate::apps::wires::WiresCommand;
    use crate::artifacts::wires::engine::fixture_nodes;

    #[test]
    fn set_active_example_metabolism_loads_seven_nodes() {
        let mut app = new_app();
        dispatch(&mut app, WiresCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: WIRES_PLAY_EXAMPLE_METABOLISM_ID.into() }));
        assert_eq!(fixture_nodes(&app.snapshot().expect("snapshot").board_fixture).len(), 7);
    }

    #[test]
    fn set_active_example_unknown_id_loads_empty_document() {
        let mut app = metabolism_app();
        dispatch(&mut app, WiresCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "nope".into() }));
        assert!(fixture_nodes(&app.snapshot().expect("snapshot").board_fixture).is_empty());
    }
}
//#endregion 🧪️Tests
