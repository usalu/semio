//! 🖼️ 🖼️ Animate present app commands command — `set-frame`.

use crate::editor::animate::config::{PresentConfig, PresentConfigMutation};
use crate::editor::animate::PresentDispatchCtx;
use crate::artifacts::present::mutations::resize_source_frame::mutation::ResizeSourceFrame;
use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::{FigureTileFrame, PresentSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-frame")]
pub struct SetFrame {
    #[dsl(block)]
    pub frame: FigureTileFrame,
}

pub fn handle(payload: &SetFrame, _doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>, _ctx: &mut PresentDispatchCtx) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![PresentMutation::ResizeSourceFrame(ResizeSourceFrame { new_frame: payload.frame.clone() })]))
}
