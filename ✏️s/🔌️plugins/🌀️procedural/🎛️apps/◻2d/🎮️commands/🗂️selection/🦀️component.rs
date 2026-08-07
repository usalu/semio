//! 🗂️ Procedural2d play app commands — ephemeral selection (config-only, never document operations).

use crate::apps::procedural2d::config::{Procedural2dConfig, Procedural2dConfigOperation};
use crate::artifacts::procedural2d::op::Procedural2dOperation;
use crate::artifacts::procedural2d::Procedural2dDocument;
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetSelection
pub mod set_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-selection")]
    pub struct SetSelection {
        pub ids: Vec<String>,
    }

    pub fn handle(payload: &SetSelection, _doc: &DocumentView<'_, Procedural2dDocument>, _cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dOperation, Procedural2dConfigOperation>, Fault> {
        Ok(Emit::config(vec![Procedural2dConfigOperation::SetSelection { ids: payload.ids.clone() }]))
    }
}
//#endregion 🔖️SetSelection

//#region 🔖️SelectNode
pub mod select_node {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "select-node")]
    pub struct SelectNode {
        pub ids: Vec<String>,
    }

    pub fn handle(payload: &SelectNode, _doc: &DocumentView<'_, Procedural2dDocument>, _cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dOperation, Procedural2dConfigOperation>, Fault> {
        Ok(Emit::config(vec![Procedural2dConfigOperation::SetSelection { ids: payload.ids.clone() }]))
    }
}
//#endregion 🔖️SelectNode

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural2d::testkit::{app, dispatch};
    use crate::apps::procedural2d::Procedural2dCommand;

    #[test]
    fn set_selection_updates_config_only() {
        let mut app = app();
        let before = app.projection().expect("projection");
        dispatch(&mut app, Procedural2dCommand::SetSelection(set_selection::SetSelection { ids: vec!["w1".into()] }));
        assert_eq!(app.projection().expect("projection"), before, "setSelection must not mutate the document");
    }
}
//#endregion 🧪️Tests
