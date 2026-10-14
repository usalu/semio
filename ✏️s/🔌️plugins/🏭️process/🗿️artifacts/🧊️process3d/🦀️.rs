//! 🪚️ Process3d artifact aggregator — the `s.process.process3d` definition and its declaration (schema descriptor
//! assembled from the facet-owned leaves, inferences, composers and the editor document codec). Every domain facet
//! lives in its own family crate (`semio-s-artifact-process-process3d-*`).

#![allow(async_fn_in_trait)]

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#region 🔖️Declaration
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    ArtifactDefinition::new(ArtifactIdentity::parse("s.process.process3d")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.process.process3d.schema.artifact")?, ArtifactCapabilityKind::schema())
                .descriptor(b"s.process.process3d")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.process.process3d")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.process.process3d.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.process.process3d.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.process.process3d.inference")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.process.process3d.composer.native")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.process.process3d@1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.process.process3d@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.process.process3d.composer.json")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.json@rfc8259/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.json@rfc8259/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.process.process3d.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"process.3d:process3d")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "process.3d")?)?
                .claim(ArtifactIdentityClaim::codec_extension("process.3d", "process3d")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.process.process3d.localization.en")?, ArtifactCapabilityKind::localization())
                .descriptor(b"Process 3D")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "Process 3D")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.process.process3d.localization.de")?, ArtifactCapabilityKind::localization())
                .descriptor(b"Process 3D")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "Process 3D")?)?,
        )
}

pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(crate::process3d_artifact_schema_descriptor())
        .inferences([semio_s_artifact_process_process3d_inferences::process3d_artifact_inference_descriptor()])
        .composers(semio_s_artifact_process_process3d_io::io_registry::entries())
        .document_codec::<semio_framework_plugin::EditorApp<semio_s_artifact_process_process3d_editor::Process3dPlayApp>>()
        .try_build()
}
//#endregion 🔖️Declaration

/// 🧬️ Descriptor for `s.process.process3d`, assembled from the facet-owned schema leaves.
pub fn process3d_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.process.process3d",
        artifact: semio_s_artifact_process_process3d_model::ARTIFACT_LEAVES,
        snapshot: semio_s_artifact_process_process3d_model::snapshot::SNAPSHOT_LEAVES,
        diff: semio_s_artifact_process_process3d_model_diff::DIFF_LEAVES,
        mutations: semio_s_artifact_process_process3d_mutations::MUTATION_LEAVES,
    }
}
