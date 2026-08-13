//! 🗂️ 🗂️ Procedural3d play app commands command — `set-selection`.

use crate::apps::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use crate::artifacts::procedural3d::schema::widget_id_from_instance_id;
use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::FlowEvalSession;
use semio_framework_plugin::{merge_world_selection_ids, ConfigView, ArtifactView, Emit, Fault, SelectionSet};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-selection")]
pub struct SetSelection {
    pub node_ids: Vec<String>}

pub fn handle(payload: &SetSelection, _doc: &ArtifactView<'_, Procedural3dSnapshot>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Procedural3dConfigMutation::SetSelection { node_ids: payload.node_ids.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural3d::testkit::{app_with_registry, dispatch};
    use crate::apps::procedural3d::Procedural3dCommand;

    #[test]
    fn set_active_utility_switch_clears_scratch_and_emits_no_operations() {
        let _serial = crate::apps::procedural3d::test_support::lock();
        let mut app = app_with_registry();
        dispatch(&mut app, Procedural3dCommand::WorldHover(world_hover::WorldHover { id: Some("extrude".into()) }));
        let before = app.snapshot().expect("snapshot");
        dispatch(&mut app, Procedural3dCommand::SetSelection(SetSelection { node_ids: vec!["extrude".into()] }));
        assert_eq!(app.snapshot().expect("snapshot"), before, "selection changes never touch the document");
    }
}
//#endregion 🧪️Tests
