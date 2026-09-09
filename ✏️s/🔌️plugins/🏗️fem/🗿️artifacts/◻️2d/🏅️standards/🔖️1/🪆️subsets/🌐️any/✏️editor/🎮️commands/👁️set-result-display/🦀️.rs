//! 👁️ 👁️ Fem2d play app commands command — `set-result-display`.

use crate::editor::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::Fem2dSnapshot;

//#region 🔖️SetResultDisplay
//#endregion 🔖️SetResultDisplay

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "result-display")]
pub struct SetResultDisplay {
    pub source_id: Option<String>,
    pub mode: String,
    pub mode_index: u32,
}

pub fn handle(payload: &SetResultDisplay, _doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Fem2dConfigMutation::SetResultDisplay { source_id: payload.source_id.clone(), mode: payload.mode.clone(), mode_index: payload.mode_index }]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
