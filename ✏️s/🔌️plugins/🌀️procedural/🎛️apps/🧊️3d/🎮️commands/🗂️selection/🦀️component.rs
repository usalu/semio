//! 🗂️ Procedural3d play app commands — ephemeral selection/hover across the flow-graph and 3D world
//! views (config-only, never document operations).

use crate::apps::procedural3d::config::{Procedural3dConfig, Procedural3dConfigOperation};
use crate::artifacts::procedural3d::engine::widget_id_from_instance_id;
use crate::artifacts::procedural3d::op::Procedural3dOperation;
use crate::artifacts::procedural3d::Procedural3dDocument;
use flow::FlowEvalSession;
use semio_framework_plugin::{merge_world_selection_ids, ConfigView, DocumentView, Emit, Fault, SelectionSet};
use serde::{Deserialize, Serialize};

//#region 🔖️SetSelection
pub mod set_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-selection")]
    pub struct SetSelection {
        pub node_ids: Vec<String>,
    }

    pub fn handle(payload: &SetSelection, _doc: &DocumentView<'_, Procedural3dDocument>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        Ok(Emit::config(vec![Procedural3dConfigOperation::SetSelection { node_ids: payload.node_ids.clone() }]))
    }
}
//#endregion 🔖️SetSelection

//#region 🔖️SelectNode
pub mod select_node {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "select-node")]
    pub struct SelectNode {
        pub node_ids: Vec<String>,
    }

    pub fn handle(payload: &SelectNode, _doc: &DocumentView<'_, Procedural3dDocument>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        Ok(Emit::config(vec![Procedural3dConfigOperation::SetSelection { node_ids: payload.node_ids.clone() }]))
    }
}
//#endregion 🔖️SelectNode

//#region 🔖️SetHover
pub mod set_hover {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-hover")]
    pub struct SetHover {
        pub object_id: Option<String>,
    }

    pub fn handle(payload: &SetHover, _doc: &DocumentView<'_, Procedural3dDocument>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        Ok(Emit::config(vec![Procedural3dConfigOperation::SetHover { node_id: payload.object_id.clone() }]))
    }
}
//#endregion 🔖️SetHover

//#region 🔖️WorldPointerDown
pub mod world_pointer_down {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "world-pointer-down")]
    pub struct WorldPointerDown {}

    pub fn handle(_payload: &WorldPointerDown, _doc: &DocumentView<'_, Procedural3dDocument>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️WorldPointerDown

//#region 🔖️WorldSelect
pub mod world_select {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "world-select")]
    pub struct WorldSelect {
        pub ids: Vec<String>,
        pub merge: String,
    }

    pub fn handle(payload: &WorldSelect, _doc: &DocumentView<'_, Procedural3dDocument>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        let mapped: Vec<String> = payload.ids.iter().map(|id| widget_id_from_instance_id(id).to_string()).collect();
        let merged = merge_world_selection_ids(&SelectionSet::from_ids(cfg.projection.selected_node_ids.clone()), &mapped, &payload.merge).to_vec();
        Ok(Emit::config(vec![Procedural3dConfigOperation::SetSelection { node_ids: merged }]))
    }
}
//#endregion 🔖️WorldSelect

//#region 🔖️WorldHover
pub mod world_hover {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "world-hover")]
    pub struct WorldHover {
        pub id: Option<String>,
    }

    pub fn handle(payload: &WorldHover, _doc: &DocumentView<'_, Procedural3dDocument>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        let resolved = payload.id.as_deref().map(|id| widget_id_from_instance_id(id).to_string());
        Ok(Emit::config(vec![Procedural3dConfigOperation::SetHover { node_id: resolved }]))
    }
}
//#endregion 🔖️WorldHover

//#region 🔖️SetSelectionMethod
pub mod set_selection_method {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "selection-method")]
    pub struct SetSelectionMethod {
        pub method: String,
    }

    pub fn handle(payload: &SetSelectionMethod, _doc: &DocumentView<'_, Procedural3dDocument>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        Ok(Emit::config(vec![Procedural3dConfigOperation::SetSelectionMethod { method: payload.method.clone() }]))
    }
}
//#endregion 🔖️SetSelectionMethod

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural3d::testkit::{app_with_registry, dispatch};
    use crate::apps::procedural3d::Procedural3dCommand;

    #[test]
    fn set_active_utility_switch_clears_scratch_and_emits_no_operations() {
        let _serial = crate::artifacts::procedural3d::engine::test_support::lock();
        let mut app = app_with_registry();
        dispatch(&mut app, Procedural3dCommand::WorldHover(world_hover::WorldHover { id: Some("extrude".into()) }));
        let before = app.projection().expect("projection");
        dispatch(&mut app, Procedural3dCommand::SetSelection(set_selection::SetSelection { node_ids: vec!["extrude".into()] }));
        assert_eq!(app.projection().expect("projection"), before, "selection changes never touch the document");
    }
}
//#endregion 🧪️Tests
