//! 🗣️ Process 3d play app commands — the UI locale (config-only, host-pushed, ephemeral view state).

use crate::apps::process3d::config::{Process3dConfig, Process3dConfigOperation};
use crate::artifacts::process3d::{op::Process3dOperation, Process3dDocument};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetLocale
pub mod set_locale {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "locale")]
    pub struct SetLocale {
        pub value: String,
    }

    pub fn handle(payload: &SetLocale, _doc: &DocumentView<'_, Process3dDocument>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dOperation, Process3dConfigOperation>, Fault> {
        Ok(Emit::config(vec![Process3dConfigOperation::SetLocale { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLocale
