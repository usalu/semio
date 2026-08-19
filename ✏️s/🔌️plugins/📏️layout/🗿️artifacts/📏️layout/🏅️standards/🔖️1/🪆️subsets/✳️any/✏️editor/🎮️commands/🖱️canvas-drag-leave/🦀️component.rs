//! 🖱️ 🖱️ Layout play app commands command — `canvas-drag-leave`.

use crate::editor::layout::canvas::active_page;
use crate::editor::layout::commands::{add_frame, add_page};
use crate::editor::layout::config::LayoutConfig;
use crate::artifacts::layout::LayoutDropPreviewState;
use crate::editor::layout::config::LayoutConfigMutation;
use crate::editor::layout::engine::scene::{build_display_list_for_page, LayoutEngine};
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{LayoutCamera, LayoutSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "canvas-drag-leave")]
pub struct CanvasDragLeave {}

pub async fn handle(_payload: &CanvasDragLeave, _doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    Ok(Emit::config(vec![LayoutConfigMutation::SetDropPreview { preview: LayoutDropPreviewState::default() }]))
}
