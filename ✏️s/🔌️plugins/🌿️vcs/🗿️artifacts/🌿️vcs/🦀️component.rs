//! 🌿️ VCS artifact — the document entity the `vcs-play` app edits.

use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};

pub const VCS_DOCUMENT_SCHEMA: &str = "vcs.vcs";

//#region 🔖️Dialect
/// 🎯️ This artifact's surface coordinate (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET
/// contract §1) — `artifact_kind` matches the `"s.vcs.schema.artifact"` capability row's own
/// descriptor below (`s.vcs.vcs`), which is also this subset's `🧬️schema` id
/// (`#[artifact_schema(id = "s.vcs.vcs")]`) and the `🏅️standards/🔖️1/🪆️subsets/🔣️component.json`
/// `"artifact"` field — not guessed. Lives at the ARTIFACT level (not under `editor`/`viewer`) so a
/// viewer file can read it without ever importing through the sibling editor module. `standard`/
/// `subset` mirror this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location, so the canonical
/// surface id is `s.vcs.vcs@1/*#editor` / `s.vcs.vcs@1/*#viewer` (contract §1 grammar).
pub const VCS_DIALECT: Dialect = Dialect { artifact_kind: "s.vcs.vcs", standard: StandardId("1"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect
pub use crate::artifacts::vcs::diff::schema::VcsDiff;
pub use crate::artifacts::vcs::mutations::VcsDemoMutation;
pub use crate::artifacts::vcs::snapshot::schema::VcsSnapshot;

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the editor manifest by
/// `crate::editor::vcs::create_vcs_app`'s `🔖️Manifest` region.
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
        .document_codec::<semio_framework_plugin::EditorApp<crate::editor::vcs::VcsPlayApp>>()
        .try_build()
}
//#endregion 🔖️Declaration
