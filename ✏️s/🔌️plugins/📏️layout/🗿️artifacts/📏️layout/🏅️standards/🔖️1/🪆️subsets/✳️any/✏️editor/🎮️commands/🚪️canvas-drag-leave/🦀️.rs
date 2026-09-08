//! 🖱️ 🖱️ Layout play app commands command — `canvas-drag-leave`.

use crate::mutations::LayoutMutation;
use crate::LayoutDropPreviewState;
use crate::LayoutSnapshot;
use crate::editor::layout::config::LayoutConfig;
use crate::editor::layout::config::LayoutConfigMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "canvas-drag-leave")]
pub struct CanvasDragLeave {}

pub fn handle(_payload: &CanvasDragLeave, _doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    Ok(Emit::config(vec![LayoutConfigMutation::SetDropPreview(crate::editor::layout::config::SetDropPreview { preview: LayoutDropPreviewState::default() })]))
}
