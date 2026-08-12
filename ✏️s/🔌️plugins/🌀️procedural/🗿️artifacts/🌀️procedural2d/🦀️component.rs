//! 📏️ Procedural2d artifact — snapshot re-exports, widget id helper, and artifact kind.


pub use crate::artifacts::procedural2d::schema::snapshot::Procedural2dSnapshot;
pub use crate::artifacts::procedural2d::schema::mutations::Procedural2dMutation;
pub use crate::artifacts::procedural2d::schema::diff::Procedural2dDiff;

use flow::Widget;
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};



pub const PROCEDURAL_2D_SCHEMA: &str = "procedural.2d";


//#region 🔖️Helpers
/// 🌡️ A flow widget's stable id, across every widget variant (mirrors flow's private accessor).
pub fn widget_id(widget: &Widget) -> &str {
    match widget {
        Widget::Neuron { id, .. }
        | Widget::InputSlider { id, .. }
        | Widget::InputNote { id, .. }
        | Widget::InputImage { id, .. }
        | Widget::Variable { id, .. }
        | Widget::OutputPreview { id, .. }
        | Widget::OutputAction { id, .. }
        | Widget::OutputExport { id, .. }
        | Widget::Cluster { id, .. } => id}
}
//#endregion 🔖️Helpers

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::procedural2d::create_procedural2d_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "2d.procedural".into(),
        name: "2D Procedural".into(),
        source_format: "procedural.2d".into(),
        component_kind: "procedural2d".into(),
        dimension: "2d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Flow },
        schema: "procedural.2d".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg"],
        import_stdio_kinds: vec!["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg"],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_kind_schema_matches_the_document_schema() {
        assert_eq!(artifact_kind().schema, PROCEDURAL_2D_SCHEMA);
    }

    #[test]
    fn widget_id_covers_every_widget_kind() {
        let widgets = vec![
            Widget::Neuron { id: "w-neuron".into(), neuron_kind: "math.add".into(), params: Default::default(), input_ports: vec![], output_ports: vec![], preview: true },
            Widget::InputSlider { id: "w-slider".into(), value: 1.0, min: 0.0, max: 2.0, step: 0.5 },
            Widget::InputNote { id: "w-note".into(), text: String::new() },
            Widget::InputImage { id: "w-image".into(), src: String::new() },
            Widget::Variable { id: "w-variable".into(), name: "value".into(), schema: "dictionary".into() },
            Widget::OutputPreview { id: "w-preview".into(), preview: Default::default(), expanded: Default::default() },
            Widget::OutputAction { id: "w-action".into(), action: String::new() },
            Widget::OutputExport { id: "w-export".into(), format: "svg".into() },
            Widget::Cluster { id: "w-cluster".into(), name: String::new(), tree: Default::default(), flow: Default::default() },
        ];
        for widget in &widgets {
            assert!(!widget_id(widget).is_empty());
        }
    }
}
//#endregion 🧪️Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::procedural2d::standards::v1::engine::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("Procedural2dComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
