//! ⌨️ ⌨️ Animate present app commands command — `engagement-input`.

use crate::editor::animate::config::{PresentConfig, PresentConfigMutation};
use crate::editor::animate::PresentDispatchCtx;
use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::PresentSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "engagement-input")]
pub struct EngagementInput {
    pub value: String,
}

pub fn handle(payload: &EngagementInput, _doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>, _ctx: &mut PresentDispatchCtx) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
    Ok(Emit::config(vec![PresentConfigMutation::SetEngagementInput { value: payload.value.clone() }]))
}
