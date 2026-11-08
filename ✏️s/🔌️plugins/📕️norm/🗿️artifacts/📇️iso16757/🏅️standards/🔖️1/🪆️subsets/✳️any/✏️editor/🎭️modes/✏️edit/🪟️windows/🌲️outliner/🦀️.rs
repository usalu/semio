//! 🌲️ ISO 16757 play app — the outliner window: every entity collection and supporting table with status glyph and a select action that the checks, properties, model and diagrams windows follow.

use semio_plugin::{BuiltNode, TreeWindows, UiAssemblyResult, WindowKindDefinition};
use semio_ui_locale::{Locale, LocalizedLabel};
use semio_framework_value::{DslValue, ToValue};
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;
use semio_s_artifact_norm_iso16757_scene::ENTITY_KINDS;
use semio_s_plugin_norm_app_surface::{render_outliner_document, NormSelection, OutlinedCollection, OutlinerContext};
use semio_s_plugin_norm_compliance::CheckReport;

//#region 🔖️Constants
pub const WINDOW_OUTLINER: &str = "norm-iso16757-outliner";
pub const BODY_OUTLINER: &str = "norm.iso16757.play.outliner";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::create_iso16757_app`.
pub fn definition() -> WindowKindDefinition {
    semio_s_plugin_norm_app_surface::window_definition(WINDOW_OUTLINER, LocalizedLabel::native("Outliner", "Gliederung"), BODY_OUTLINER, "list-tree")
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🌳️ The outline tree of a catalogue: the catalogue lists lifted to the top level beside the geometry objects (the object map flattened to rows) and the dictionary lists, so every registry collection is a top-level list of rows with an `id`.
pub fn tree_of(document: &Iso16757Snapshot) -> DslValue {
    let tree = document.to_value();
    let lifted = |path: &[&str]| path.iter().try_fold(&tree, |value, key| value.get(key)).cloned().unwrap_or(DslValue::Array(Vec::new()));
    let objects = match lifted(&["geometry", "objects"]) {
        DslValue::Object(entries) => DslValue::Array(entries.into_iter().map(|(_, row)| row).collect()),
        other => other,
    };
    DslValue::object([
        ("productGroups".to_string(), lifted(&["catalogue", "productGroups"])),
        ("productClasses".to_string(), lifted(&["catalogue", "productClasses"])),
        ("productSeries".to_string(), lifted(&["catalogue", "productSeries"])),
        ("products".to_string(), lifted(&["catalogue", "products"])),
        ("productIndexes".to_string(), lifted(&["catalogue", "productIndexes"])),
        ("propertyDefinitions".to_string(), lifted(&["catalogue", "propertyDefinitions"])),
        ("geometryObjects".to_string(), objects),
        ("subjects".to_string(), lifted(&["dictionary", "subjects"])),
    ])
}

/// 🌲️ The catalogue tree: groups, classes, series, products, indexes, property definitions and geometry objects in registry order, then the dictionary subjects, each row with status glyph and a select action that the checks, properties, model and diagrams windows follow.
pub fn render(host: &crate::Host, selection: &NormSelection, locale: Locale, controller_id: &'static str, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let collections: Vec<OutlinedCollection> = ENTITY_KINDS.iter().map(|k| OutlinedCollection { path: k.collection, label_en: k.label_en, label_de: k.label_de }).collect();
    let state = selection.outliner_state_in(super::checks::INTERACTION_DOMAIN);
    let report: &CheckReport = host.report();
    render_outliner_document(&tree_of(host.document()), &collections, Some(crate::field_meta::iso16757_field_meta), &OutlinerContext { report, locale, controller_id, windows, state: &state })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
