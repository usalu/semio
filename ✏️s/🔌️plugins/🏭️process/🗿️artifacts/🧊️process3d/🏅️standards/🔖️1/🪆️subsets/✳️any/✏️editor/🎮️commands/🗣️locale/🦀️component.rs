//! 🗣️ Process 3d play app commands — the UI locale (config-only, host-pushed, ephemeral view state).

use crate::editor::process3d::config::{Process3dConfig, Process3dConfigMutation};
use crate::artifacts::process3d::{op::Process3dMutation, Process3dSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetLocale
pub mod set_locale {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "locale")]
    pub struct SetLocale {
        pub value: String,
    }

    pub async fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>, _ctx: &mut crate::editor::process3d::Process3dDispatchCtx) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Process3dConfigMutation::SetLocale { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLocale
