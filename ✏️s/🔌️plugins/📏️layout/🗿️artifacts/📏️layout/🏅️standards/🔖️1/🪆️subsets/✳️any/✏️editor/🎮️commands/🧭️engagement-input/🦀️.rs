//! 👁️ 👁️ Layout play app commands command — `engagement-input`.

use crate::editor::layout::config::{LayoutConfig, LayoutConfigMutation};
use crate::{op::LayoutMutation, LayoutSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "engagement-input")]
pub struct EngagementInput {
    pub value: String,
}

pub fn handle(payload: &EngagementInput, _doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    Ok(Emit::config(vec![LayoutConfigMutation::SetEngagementInput(crate::editor::layout::config::SetEngagementInput { value: payload.value.clone() })]))
}
