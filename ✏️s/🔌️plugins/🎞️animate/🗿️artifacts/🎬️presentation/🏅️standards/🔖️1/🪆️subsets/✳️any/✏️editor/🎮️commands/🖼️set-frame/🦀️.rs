//! 🖼️ 🖼️ Animate presentation app commands command — `set-frame`.

#![allow(clippy::result_large_err)]

use crate::mutations::resize_source_frame::ResizeSourceFrame;
use crate::op::PresentationMutation;
use crate::{FigureTileFrame, PresentationSnapshot};
use crate::editor::animate::config::{PresentationConfig, PresentationConfigMutation};
use crate::editor::animate::PresentationDispatchCtx;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-frame")]
pub struct SetFrame {
    #[dsl(block)]
    pub frame: FigureTileFrame,
}

pub fn handle(payload: &SetFrame, _doc: &ArtifactView<'_, PresentationSnapshot>, _cfg: &ConfigView<'_, PresentationConfig>, _ctx: &mut PresentationDispatchCtx) -> Result<Emit<PresentationMutation, PresentationConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![PresentationMutation::ResizeSourceFrame(ResizeSourceFrame { new_frame: payload.frame.clone() })]))
}
