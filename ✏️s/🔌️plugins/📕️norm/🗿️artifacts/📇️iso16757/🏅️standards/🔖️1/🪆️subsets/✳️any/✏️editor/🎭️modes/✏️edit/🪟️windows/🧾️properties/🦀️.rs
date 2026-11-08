//! 🧾️ ISO 16757 play app — the properties window: the typed unit-aware inspector with the entity outliner, focused on the selected check's entity.

use semio_plugin::{BuiltNode, TreeWindows, UiAssemblyResult, WindowKindDefinition};
use semio_ui_locale::{Locale, LocalizedLabel};
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;
use semio_s_plugin_norm_app_surface::{render_inspector, NormSelection};

//#region 🔖️Constants
pub const WINDOW_PROPERTIES: &str = "norm-iso16757-properties";
pub const BODY_PROPERTIES: &str = "norm.iso16757.play.properties";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::create_iso16757_app`.
pub fn definition() -> WindowKindDefinition {
    semio_s_plugin_norm_app_surface::window_definition(WINDOW_PROPERTIES, LocalizedLabel::native("Properties", "Eigenschaften"), BODY_PROPERTIES, "sliders")
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🧾️ Typed properties and outliner; the selection opens its rows and marks the field paths of the selected check.
pub fn render(host: &crate::Host, selection: &NormSelection, locale: Locale, controller_id: &'static str, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let options = selection.inspector_options();
    render_inspector::<crate::Iso16757Family>(host, locale, controller_id, Some(crate::field_meta::iso16757_field_meta), windows, &options)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
