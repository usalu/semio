//! 🖼️ 🖼️ Animate present app commands command — `set-frame`.

#![allow(clippy::result_large_err)]

use crate::artifacts::present::mutations::resize_source_frame::mutation::ResizeSourceFrame;
use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::{FigureTileFrame, PresentSnapshot};
use crate::editor::animate::config::{PresentConfig, PresentConfigMutation};
use crate::editor::animate::PresentDispatchCtx;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-frame")]
pub struct SetFrame {
    #[dsl(block)]
    pub frame: FigureTileFrame,
}

pub fn handle(payload: &SetFrame, _doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>, _ctx: &mut PresentDispatchCtx) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![PresentMutation::ResizeSourceFrame(ResizeSourceFrame { new_frame: payload.frame.clone() })]))
}
