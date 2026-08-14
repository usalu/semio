//! 🗂️ Process 3d play app commands — object selection/hover (config-only, ephemeral view state).

use crate::apps::process3d::config::{Process3dConfig, Process3dConfigMutation};
use crate::artifacts::process3d::{op::Process3dMutation, Process3dSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetSelection
pub mod set_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-selection")]
    pub struct SetSelection {
        pub id: Option<String>,
    }

    pub fn handle(payload: &SetSelection, _doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Process3dConfigMutation::SetSelectedId { value: payload.id.clone() }]))
    }
}
//#endregion 🔖️SetSelection

//#region 🔖️SetHover
pub mod set_hover {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-hover")]
    pub struct SetHover {
        pub id: Option<String>,
    }

    pub fn handle(payload: &SetHover, _doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Process3dConfigMutation::SetHoveredId { value: payload.id.clone() }]))
    }
}
//#endregion 🔖️SetHover

//#region 🔖️ContextMenuAt
pub mod context_menu_at {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "context-menu-at")]
    pub struct ContextMenuAt {
        pub kind: String,
        pub id: String,
    }

    pub fn handle(payload: &ContextMenuAt, _doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let mutations = match payload.kind.as_str() {
            "face" => payload
                .id
                .parse()
                .ok()
                .map(|value| vec![Process3dConfigMutation::SetSelectedId { value: Some("processed".into()) }, Process3dConfigMutation::SetSelectedFaceId { value: Some(value) }])
                .unwrap_or_default(),
            "mesh" | "object" => vec![Process3dConfigMutation::SetSelectedId { value: Some(payload.id.clone()) }, Process3dConfigMutation::SetSelectedFaceId { value: None }],
            _ => Vec::new(),
        };
        Ok(Emit::config(mutations))
    }
}
//#endregion 🔖️ContextMenuAt
