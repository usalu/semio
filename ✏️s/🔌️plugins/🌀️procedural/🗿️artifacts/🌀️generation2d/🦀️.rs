//! 📏️ Generation2d artifact — its runtime definition and declaration over the layer crates.

#![allow(clippy::result_large_err)]

//#region 🔖️Declaration
/// 🧾️ Defines s.generation2d's immutable runtime capability leaves.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    ArtifactDefinition::new(ArtifactIdentity::parse("s.procedural.generation2d")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation2d.schema.artifact")?, ArtifactCapabilityKind::schema())
                .descriptor(b"s.procedural.generation2d")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.procedural.generation2d")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation2d.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.procedural.generation2d.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.procedural.generation2d.inference")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation2d.composer.native")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.procedural.generation2d@1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.procedural.generation2d@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation2d.composer.svg")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.svg@1.1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.svg@1.1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation2d.composer.pdf")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.pdf@1.4/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.pdf@1.4/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation2d.composer.png")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.png@1.2/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.png@1.2/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation2d.composer.json")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.json@rfc8259/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.json@rfc8259/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation2d.composer.dxf")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.dxf@r12/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.dxf@r12/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation2d.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"generation.2d:generation2d")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "generation.2d")?)?
                .claim(ArtifactIdentityClaim::codec_extension("generation.2d", "generation2d")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation2d.localization.en")?, ArtifactCapabilityKind::localization())
                .descriptor(b"2D Generation")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "2D Generation")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation2d.localization.de")?, ArtifactCapabilityKind::localization())
                .descriptor(b"2D Generierung")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "2D Generierung")?)?,
        )
}

/// 🔖️ Assembles s.generation2d's typed runtime declaration.
pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(semio_s_artifact_procedural_2d_model::generation2d_artifact_schema_descriptor())
        .inferences([semio_s_artifact_procedural_2d_model::inferences::generation2d_artifact_inference_descriptor()])
        .composers(semio_s_artifact_procedural_2d_io::io_registry::entries())
        .document_codec::<semio_framework_plugin::EditorApp<semio_s_artifact_procedural_2d_editor::Generation2dPlayApp>>()
        .try_build()
}
//#endregion 🔖️Declaration
