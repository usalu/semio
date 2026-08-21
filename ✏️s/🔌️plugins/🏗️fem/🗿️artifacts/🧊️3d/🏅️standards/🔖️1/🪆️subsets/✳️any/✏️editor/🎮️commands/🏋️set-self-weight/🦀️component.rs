//! 🏋️ 🏋️ FEM 3D app commands command — `set-self-weight`.

use crate::artifacts::fem3d::mutations::change_load_case_self_weight;
use crate::artifacts::fem3d::op::Fem3dMutation;
use crate::artifacts::fem3d::Fem3dSnapshot;
use crate::editor::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "set-self-weight")]
pub struct SetSelfWeight {
    pub case_id: String,
    pub enabled: bool,
}

pub async fn handle(payload: &SetSelfWeight, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    match snapshot.load_cases.iter().any(|lc| lc.id == payload.case_id) {
        true => Ok(Emit::mutations(vec![Fem3dMutation::ChangeLoadCaseSelfWeight(change_load_case_self_weight::mutation::ChangeLoadCaseSelfWeight { case_id: payload.case_id.clone(), new_self_weight: payload.enabled })])),
        false => Ok(Emit::default()),
    }
}
