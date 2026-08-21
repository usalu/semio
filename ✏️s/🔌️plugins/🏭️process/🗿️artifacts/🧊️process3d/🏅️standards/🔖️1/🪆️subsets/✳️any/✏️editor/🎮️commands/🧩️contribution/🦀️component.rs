//! 🧩️ Process 3d play app commands — host-pushed plugin contributions (machine catalogs).

use crate::artifacts::process3d::{op::Process3dMutation, Process3dSnapshot};
use crate::editor::process3d::config::{Process3dConfig, Process3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetContributions
pub mod set_contributions {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "contributions")]
    pub struct SetContributions {
        pub json: String,
    }

    pub async fn handle(
        payload: &SetContributions,
        _doc: &ArtifactView<'_, Process3dSnapshot>,
        _cfg: &ConfigView<'_, Process3dConfig>,
        _ctx: &mut crate::editor::process3d::Process3dDispatchCtx,
    ) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Process3dConfigMutation::SetContributions { json: payload.json.clone() }]))
    }
}
//#endregion 🔖️SetContributions
