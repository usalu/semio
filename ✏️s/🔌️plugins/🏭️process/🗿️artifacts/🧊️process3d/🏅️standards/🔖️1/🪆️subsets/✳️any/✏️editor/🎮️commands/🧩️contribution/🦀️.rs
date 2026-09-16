//! 🧩️ Process 3d play app commands — host-pushed plugin contributions (machine catalogs).

use crate::editor::process3d::config::{Process3dConfig, Process3dConfigMutation};
use crate::{op::Process3dMutation, Process3dSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️SetContributions
pub mod set_contributions {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "contributions")]
    pub struct SetContributions {
        pub json: String,
    }

    pub fn handle(
        payload: &SetContributions,
        _doc: &ArtifactView<'_, Process3dSnapshot>,
        _cfg: &ConfigView<'_, Process3dConfig>,
        _ctx: &mut crate::editor::process3d::Process3dDispatchCtx,
    ) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let installable = crate::editor::process3d::installable_contributions(&payload.json, crate::editor::process3d::PROCESS3D_CONFIG_CONTRIBUTIONS_BYTES);
        Ok(Emit::config(vec![Process3dConfigMutation::SetContributions { json: installable }]))
    }
}
//#endregion 🔖️SetContributions
