//! 📐️ Procedural3d artifact — snapshot re-exports, widget id helper, and artifact kind.


use flow::Widget;
use semio_framework_plugin::{ArtifactKindSpec, Dialect, EditorApp, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};

pub const PROCEDURAL_3D_SCHEMA: &str = "procedural.3d";

/// 🎯️ This subset's canonical dialect (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET
/// contract §2.1/§7.4) — lives at the ARTIFACT level (not under `editor`/`viewer`) specifically so a
/// viewer file can read it without ever importing through the sibling `editor` module.
/// `artifact_kind` matches this artifact's own `s.procedural3d.schema.artifact` capability descriptor
/// above (`b"s.procedural.procedural3d"`); `standard`/`subset` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location — i.e. the canonical surface id is
/// `s.procedural.procedural3d@1/*#editor` / `s.procedural.procedural3d@1/*#viewer`.
pub const PROCEDURAL3D_DIALECT: Dialect = Dialect { artifact_kind: "s.procedural.procedural3d", standard: StandardId("1"), subset: SubsetId::ANY };

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
        | Widget::Cluster { id, .. } => id,
    }
}
//#endregion 🔖️Helpers

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::procedural3d::create_procedural3d_app`'s `🔖️Manifest` region.
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
        export_formats: vec![],
        import_formats: vec![],
        // 🖼️ "stdio.json"/"stdio.png" stay out of exports (procedural2d owns those EXPORT claims, D3)
        // but stay in imports below — see `🚪️io/🦀️component.rs`'s `🚪️IoRegistry` region.
        export_stdio_kinds: vec!["stdio.dwg", "stdio.gltf", "stdio.las", "stdio.obj", "stdio.ply", "stdio.stl"],
        import_stdio_kinds: vec!["stdio.dwg", "stdio.gltf", "stdio.json", "stdio.las", "stdio.obj", "stdio.ply", "stdio.png", "stdio.stl"],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Declaration
/// 🧾️ Defines s.procedural3d's immutable runtime capability leaves.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    ArtifactDefinition::new(ArtifactIdentity::parse("s.procedural3d")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural3d.schema.artifact")?, ArtifactCapabilityKind::schema())
                .descriptor(b"s.procedural.procedural3d")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.procedural.procedural3d")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural3d.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.procedural.procedural3d.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.procedural.procedural3d.inference")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural3d.composer.native")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.procedural3d@1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.procedural3d@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural3d.composer.las")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.las@1.0/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.las@1.0/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural3d.composer.ply")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.ply@1.0/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.ply@1.0/*")?)?,
        )?
        // 🖼️ No `composer.png`/`composer.json` here: both are generic bridge dialects with no
        // real per-artifact fidelity difference (both procedural2d's and procedural3d's export stubs
        // are equally `print_dsl` placeholders), so procedural2d — the plugin's first-declared,
        // primary 2D artifact — keeps the EXPORT claim (26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME
        // D3, a documented tie-break, not evidence-backed like the DWG↔mesh-bridge split below). Import
        // still works: `reads()` on this artifact's own native composer is unaffected by this removal.
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural3d.composer.dwg")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.dwg@ac1018/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.dwg@ac1018/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural3d.composer.stl")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.stl@ascii/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.stl@ascii/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural3d.composer.gltf")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.gltf@2.0/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.gltf@2.0/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural3d.composer.obj")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.obj@3.0/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.obj@3.0/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural3d.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"procedural.3d:procedural3d")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "procedural.3d")?)?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::extension(), "procedural3d")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural3d.localization.en")?, ArtifactCapabilityKind::localization())
                .descriptor(b"3D Procedural")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "3D Procedural")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural3d.localization.de")?, ArtifactCapabilityKind::localization())
                .descriptor(b"3D Prozedural")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "3D Prozedural")?)?,
        )
}

/// 🔖️ Assembles s.procedural3d's typed runtime declaration.
pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(crate::artifacts::procedural3d::schema::procedural3d_artifact_schema_descriptor())
        .inferences([crate::artifacts::procedural3d::standards::v1::subsets::any::schema::inferences::procedural3d_artifact_inference_descriptor()])
        .composers(crate::artifacts::procedural3d::standards::v1::subsets::any::io::io_registry::entries())
        .document_codec::<EditorApp<crate::editor::procedural3d::Procedural3dPlayApp>>()
        .try_build()
}
//#endregion 🔖️Declaration

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_kind_schema_matches_the_document_schema() {
        assert_eq!(artifact_kind().schema, PROCEDURAL_3D_SCHEMA);
    }

    #[test]
    fn dialect_artifact_kind_matches_the_schema_capability_descriptor() {
        assert_eq!(PROCEDURAL3D_DIALECT.artifact_kind, "s.procedural.procedural3d");
        assert_eq!(PROCEDURAL3D_DIALECT.standard, StandardId("1"));
        assert_eq!(PROCEDURAL3D_DIALECT.subset, SubsetId::ANY);
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
