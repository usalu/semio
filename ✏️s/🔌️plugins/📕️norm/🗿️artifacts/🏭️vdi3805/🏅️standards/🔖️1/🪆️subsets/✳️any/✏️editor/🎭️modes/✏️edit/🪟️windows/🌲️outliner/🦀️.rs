//! 🌲️ VDI 3805 play app — the outliner window: every entity collection and supporting table with status glyph and a select action that the checks, properties, model and diagrams windows follow.

use semio_plugin::{BuiltNode, TreeWindows, UiAssemblyResult, WindowKindDefinition};
use semio_ui_locale::{Locale, LocalizedLabel};
use semio_s_artifact_norm_vdi3805_scene::ENTITY_KINDS;
use semio_s_plugin_norm_app_surface::{render_outliner_document, NormSelection, OutlinedCollection, OutlinerContext};
use semio_s_plugin_norm_compliance::CheckReport;

//#region 🔖️Constants
pub const WINDOW_OUTLINER: &str = "norm-vdi3805-outliner";
pub const BODY_OUTLINER: &str = "norm.vdi3805.play.outliner";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::create_vdi3805_app`.
pub fn definition() -> WindowKindDefinition {
    semio_s_plugin_norm_app_surface::window_definition(WINDOW_OUTLINER, LocalizedLabel::native("Outliner", "Gliederung"), BODY_OUTLINER, "list-tree")
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🌲️ The value tree the outliner reads: the nested product rows under their dotted document path and the id-keyed geometries and curves lifted to row lists, so every entity group is a list of rows with an `id`.
fn outline_tree(document: &semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot) -> semio_framework_value::DslValue {
    use semio_framework_value::{DslValue, ToValue};
    DslValue::Object(vec![
        (semio_s_artifact_norm_vdi3805_scene::PRODUCTS.to_owned(), DslValue::Array(document.catalog.products.iter().map(ToValue::to_value).collect())),
        (semio_s_artifact_norm_vdi3805_scene::GEOMETRY.to_owned(), DslValue::Array(document.geometry.values().map(ToValue::to_value).collect())),
        (semio_s_artifact_norm_vdi3805_scene::CURVES.to_owned(), DslValue::Array(document.curves.values().map(ToValue::to_value).collect())),
    ])
}

/// 🌲️ Entity kinds first (registry order), then every other list of entities of the document, marking the selection the mirror resolved.
pub fn render(host: &crate::Host, selection: &NormSelection, locale: Locale, controller_id: &'static str, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let collections: Vec<OutlinedCollection> = ENTITY_KINDS.iter().map(|k| OutlinedCollection { path: k.collection, label_en: k.label_en, label_de: k.label_de }).collect();
    let state = selection.outliner_state_in(super::checks::INTERACTION_DOMAIN);
    let report: &CheckReport = host.report();
    render_outliner_document(&outline_tree(host.document()), &collections, Some(crate::field_meta::vdi3805_field_meta), &OutlinerContext { report, locale, controller_id, windows, state: &state })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
