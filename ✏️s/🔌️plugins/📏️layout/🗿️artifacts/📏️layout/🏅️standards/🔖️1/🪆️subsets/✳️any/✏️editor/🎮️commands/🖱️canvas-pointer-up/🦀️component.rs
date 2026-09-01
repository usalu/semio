//! 🖱️ 🖱️ Layout play app commands command — `canvas-pointer-up`.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::LayoutDropPreviewState;
use crate::artifacts::layout::{LayoutCamera, LayoutSnapshot};
use crate::editor::layout::canvas::active_page;
use crate::editor::layout::commands::{add_frame, add_page};
use crate::editor::layout::config::LayoutConfig;
use crate::editor::layout::config::LayoutConfigMutation;
use crate::editor::layout::engine::scene::{build_display_list_for_page, LayoutEngine};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "canvas-pointer-up")]
pub struct CanvasPointerUp {}

pub async fn handle(_payload: &CanvasPointerUp, _doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    Ok(Emit::default())
}
