//! 🗂️ 🗂️ Flow play app commands command — `focus-selection`.

use crate::apps::flow::config::{FlowConfig, FlowConfigMutation};
use crate::apps::flow::{focus_selection_camera, host_operations, sync_host_selection_domains};
use crate::artifacts::flow::schema::widget_id;
use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct FocusSelection {}

pub fn handle(_payload: &FocusSelection, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    match focus_selection_camera(doc.snapshot, cfg.snapshot, session) {
        Some(camera) => Ok(Emit::config(vec![FlowConfigMutation::SetCamera { camera }])),
        None => Ok(Emit::default()),
    }
}
