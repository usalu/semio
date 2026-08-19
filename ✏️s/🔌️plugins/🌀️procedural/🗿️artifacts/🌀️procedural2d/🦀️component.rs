//! 📏️ Procedural2d artifact — snapshot re-exports, widget id helper, and artifact kind.


use flow::Widget;
use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};

pub const PROCEDURAL_2D_SCHEMA: &str = "procedural.2d";

/// 🪪️ This artifact's canonical `s.procedural.procedural2d@1/*` dialect — lives at the ARTIFACT level
/// (not under `editor`/`viewer`) so a viewer file can read it without ever importing through the
/// sibling editor module (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1).
/// `artifact_kind` matches `definition()`'s own `s.procedural2d.schema.artifact` capability descriptor
/// (`s.procedural.procedural2d`); `standard`/`subset` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location.
pub const PROCEDURAL2D_DIALECT: Dialect = Dialect { artifact_kind: "s.procedural.procedural2d", standard: StandardId("1"), subset: SubsetId::ANY };

//#region 🔖️Helpers
/// 🌡️ A flow widget's stable id, across every widget variant (mirrors flow's private accessor).
pub async fn widget_id(widget: &Widget) -> &str {
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
/// `crate::editor::procedural2d::create_procedural2d_app`'s `🔖️Manifest` region.
pub async fn artifact_kind() -> ArtifactKindSpec {
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
        // 🖊️ "stdio.dwg" stays out of exports (procedural3d owns that EXPORT claim, D3) but stays in
        // imports below — see `🚪️io/🦀️component.rs`'s `🚪️IoRegistry` region for the ownership rule.
        export_stdio_kinds: vec!["stdio.dxf", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg"],
        import_stdio_kinds: vec!["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg"],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Declaration
/// 🧾️ Defines s.procedural2d's immutable runtime capability leaves.
pub async fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    ArtifactDefinition::new(ArtifactIdentity::parse("s.procedural2d")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural2d.schema.artifact")?, ArtifactCapabilityKind::schema())
                .descriptor(b"s.procedural.procedural2d")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.procedural.procedural2d")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural2d.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.procedural.procedural2d.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.procedural.procedural2d.inference")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural2d.composer.native")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.procedural2d@1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.procedural2d@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural2d.composer.svg")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.svg@1.1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.svg@1.1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural2d.composer.pdf")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.pdf@1.4/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.pdf@1.4/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural2d.composer.png")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.png@1.2/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.png@1.2/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural2d.composer.json")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.json@rfc8259/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.json@rfc8259/*")?)?,
        )?
        // 🖊️ No `composer.dwg` here: procedural3d owns the `s.stdio.dwg@ac1018/*` EXPORT claim
        // (26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME D3 — `ArtifactDefinitionRegistry` rejects
        // two artifacts in the same plugin exporting the identical literal dialect coordinate; procedural3d
        // has a real host-media DWG↔mesh bridge, `HostMediaHandlerDeclaration::mesh_dwg_bridge` in
        // `../../🦀️component.rs`, procedural2d has none). Import still works: `derived_composition`'s
        // `Procedural2dComposerComposition::reads()` still lists `DEP_DWG`, unaffected by this removal.
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural2d.composer.dxf")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.dxf@r12/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.dxf@r12/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural2d.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"procedural.2d:procedural2d")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "procedural.2d")?)?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::extension(), "procedural2d")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural2d.localization.en")?, ArtifactCapabilityKind::localization())
                .descriptor(b"2D Procedural")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "2D Procedural")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural2d.localization.de")?, ArtifactCapabilityKind::localization())
                .descriptor(b"2D Prozedural")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "2D Prozedural")?)?,
        )
}

/// 🔖️ Assembles s.procedural2d's typed runtime declaration.
pub async fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(crate::artifacts::procedural2d::schema::procedural2d_artifact_schema_descriptor())
        .inferences([crate::artifacts::procedural2d::standards::v1::subsets::any::schema::inferences::procedural2d_artifact_inference_descriptor()])
        .composers(crate::artifacts::procedural2d::standards::v1::subsets::any::io::io_registry::entries())
        .document_codec::<semio_framework_plugin::EditorApp<crate::editor::procedural2d::Procedural2dPlayApp>>()
        .try_build()
}
//#endregion 🔖️Declaration

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn artifact_kind_schema_matches_the_document_schema() {
        assert_eq!(artifact_kind().schema, PROCEDURAL_2D_SCHEMA);
    }

    #[test]
    async fn widget_id_covers_every_widget_kind() {
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
