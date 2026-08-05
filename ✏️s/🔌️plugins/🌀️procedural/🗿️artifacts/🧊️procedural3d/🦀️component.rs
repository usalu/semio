//! 📐️ Procedural3d artifact — the document entity the 🧊️3d app edits (constitutional: general).

use flow_core::{FlowFixture, Widget};
use playbook::GenerationPlayState;
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability, OsMediaFormat};
use serde::{Deserialize, Serialize};

pub const PROCEDURAL_3D_SCHEMA: &str = "procedural.3d";

//#region 🔖️Document
/// 🧾️ Persistent procedural-3d document — the flow fixture plus the generation vocabulary state.
/// Ephemeral view state (selection, sun, LOD, preview caches) lives in the app's config
/// (`crate::apps::procedural3d::config::Procedural3dConfig`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural3dDocument {
    pub fixture: FlowFixture,
    #[serde(default)]
    pub generation: GenerationPlayState,
}

/// 🪪️ A flow widget's stable id, across every widget variant (mirrors flow_core's private accessor).
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
        | Widget::Cluster { id, .. } => id,
    }
}
//#endregion 🔖️Document

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::procedural3d::create_procedural3d_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "3d.procedural".into(),
        name: "3D Procedural".into(),
        source_format: "procedural.3d".into(),
        component_kind: "procedural3d".into(),
        dimension: "3d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Flow },
        schema: "procedural.3d".into(),
        export_formats: vec![OsMediaFormat::Obj, OsMediaFormat::Glb, OsMediaFormat::Stl],
        import_formats: vec![OsMediaFormat::Obj, OsMediaFormat::Glb, OsMediaFormat::Stl],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_kind_schema_matches_the_document_schema() {
        assert_eq!(artifact_kind().schema, PROCEDURAL_3D_SCHEMA);
    }

    #[test]
    fn widget_id_covers_all_widget_kinds() {
        let widgets: Vec<Widget> = vec![
            Widget::Neuron { id: "neuron-1".into(), neuron_kind: "math.add".into(), params: Default::default(), input_ports: vec![], output_ports: vec![], preview: true },
            Widget::InputSlider { id: "slider-1".into(), value: 0.0, min: 0.0, max: 1.0, step: 0.1 },
            Widget::InputNote { id: "note-1".into(), text: String::new() },
            Widget::InputImage { id: "image-1".into(), src: String::new() },
            Widget::Variable { id: "variable-1".into(), name: "x".into(), schema: "number".into() },
            Widget::OutputPreview { id: "preview-1".into(), preview: Default::default(), expanded: Default::default() },
            Widget::OutputAction { id: "action-1".into(), action: "run".into() },
            Widget::OutputExport { id: "export-1".into(), format: "gltf".into() },
            Widget::Cluster { id: "cluster-1".into(), name: "c".into(), tree: Default::default(), flow: Default::default() },
        ];
        let ids: Vec<&str> = widgets.iter().map(widget_id).collect();
        assert_eq!(ids, vec!["neuron-1", "slider-1", "note-1", "image-1", "variable-1", "preview-1", "action-1", "export-1", "cluster-1"]);
    }
}
//#endregion 🧪️Tests
