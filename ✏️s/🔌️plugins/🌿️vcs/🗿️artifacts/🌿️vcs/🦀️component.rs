//! 🌿️ VCS artifact — the document entity the `vcs-play` app edits.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub const VCS_DOCUMENT_SCHEMA: &str = "vcs.vcs";
pub use crate::artifacts::vcs::diff::schema::VcsDiff;
pub use crate::artifacts::vcs::mutations::VcsDemoMutation;
pub use crate::artifacts::vcs::snapshot::schema::VcsSnapshot;

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::vcs::create_vcs_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "vcs.document".into(),
        name: "VCS Document".into(),
        source_format: VCS_DOCUMENT_SCHEMA.into(),
        component_kind: "vcs".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: VCS_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🔖️Declaration
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    ArtifactDefinition::new(ArtifactIdentity::parse("s.vcs")?)
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.vcs.schema.artifact")?, ArtifactCapabilityKind::schema()).descriptor(b"s.vcs.vcs")?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.vcs.vcs")?)?)?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.vcs.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.vcs.vcs.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.vcs.vcs.inference")?)?,
        )?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.vcs.composer.native")?, ArtifactCapabilityKind::composer()).descriptor(b"s.vcs@1/*")?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.vcs@1/*")?)?)?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.vcs.composer.json")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.json@rfc8259/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.json@rfc8259/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.vcs.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"vcs.vcs:vcs")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "vcs.vcs")?)?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::extension(), "vcs")?)?,
        )?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.vcs.localization.en")?, ArtifactCapabilityKind::localization()).descriptor(b"VCS")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "VCS")?)?)?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.vcs.localization.de")?, ArtifactCapabilityKind::localization()).descriptor(b"VCS")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "VCS")?)?)
}

pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(crate::artifacts::vcs::schema::vcs_artifact_schema_descriptor())
        .inferences([crate::artifacts::vcs::standards::v1::subsets::any::schema::inferences::vcs_artifact_inference_descriptor()])
        .composers(crate::artifacts::vcs::standards::v1::subsets::any::io::io_registry::entries())
        .document_codec::<crate::apps::vcs::VcsPlayApp>()
        .try_build()
}
//#endregion 🔖️Declaration
