//! 🗂️ 🗂️ Procedural3d play app commands command — `world-select`.

use crate::apps::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use crate::artifacts::procedural3d::schema::widget_id_from_instance_id;
use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::FlowEvalSession;
use semio_framework_plugin::{merge_world_selection_ids, ConfigView, ArtifactView, Emit, Fault, SelectionSet};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "world-select")]
pub struct WorldSelect {
    pub ids: Vec<String>,
    pub merge: String}

pub fn handle(payload: &WorldSelect, _doc: &ArtifactView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    let mapped: Vec<String> = payload.ids.iter().map(|id| widget_id_from_instance_id(id).to_string()).collect();
    let merged = merge_world_selection_ids(&SelectionSet::from_ids(cfg.snapshot.selected_node_ids.clone()), &mapped, &payload.merge).to_vec();
    Ok(Emit::config(vec![Procedural3dConfigMutation::SetSelection { node_ids: merged }]))
}
