//! ✏️ Generation3d play app — the `edit` mode: the default two-window authoring layout (flow graph +
//! 3D preview).

use crate::editor::generation3d::modes::edit::windows::{flow, preview};
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, ToolRef, WindowLayout};

pub const GENERATION_3D_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// ⏯️ The `previewEval` run is referenced HERE because this mode mounts the preview window that starts
/// it: a declared tool that no mode references is refused outright by the plugin builder
/// (`app-definition.invalid: … tool previewEval is not referenced by any mode`,
/// `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`), so mode ownership is where a tool's scope
/// is stated — not a comment on the `.tool()` call.
pub fn definition() -> ModeDefinition {
    ModeDefinition {
        id: GENERATION_3D_PLAY_MODE_EDIT.into(),
        label: LocalizedLabel::native("Edit", "Bearbeiten"),
        icon_id: "pencil".into(),
        tools: vec![semio_framework::io::resolve_ready(ToolRef::new(crate::preview_eval::PREVIEW_EVAL_TOOL_ID))],
        layout_id: None,
        commands: Vec::new(),
    }
}

pub fn layout() -> WindowLayout {
    create_default_layout(&[flow::GENERATION_3D_PLAY_WINDOW_MAIN.into(), preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into()], "row", Some(&[68.0, 32.0]), Some(&["Flow".into(), "Preview".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
