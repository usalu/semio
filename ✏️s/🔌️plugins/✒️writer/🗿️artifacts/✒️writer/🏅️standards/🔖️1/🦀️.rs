//! 🏅️ Writer standard `1` root — mounts subset `any` and exports `standard() -> StandardDeclaration`
//! (design.md §1). `mimes`/`extensions`: no real MIME registration exists anywhere for this
//! artifact (unlike stdio's `📜️artifact-definition.json`) — the old `ArtifactCapability` channel
//! only ever claimed a codec id (`writer.document`) and an extension (`writer`), never a mime type
//! (`🗿️artifacts/✒️writer/🦀️.rs`'s `definition()`, capability `s.writer.codec.document`).
//! `mimes` below is therefore a documented synthesis (see `## openQuestions`), `extensions` is the
//! real, carried-over value — same documented shape `📓️w4-sequence-report.md` used for `🎬️sequence`.

use crate::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use {semio_framework_artifact_reference::StandardId};

//#region 🔖️Standard
pub fn standard<A: crate::WriterApplication>() -> StandardDeclaration<A> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.writer+json"], extensions: &["writer"] }, subsets: vec![subsets::any::subset()] }
}
//#endregion 🔖️Standard

//#region 🧩️ArtifactItems6
pub mod artifact_items_6 {
use crate::{WRITER_DIALECT};

//#region 🔖️Declaration
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    ArtifactDefinition::new(ArtifactIdentity::parse("s.writer.writer")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.writer.writer.schema.artifact")?, ArtifactCapabilityKind::schema())
                .descriptor(b"s.writer.writer")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.writer.writer")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.writer.writer.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.writer.writer.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.writer.writer.inference")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.writer.writer.composer.native")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.writer.writer@1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.writer.writer@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.writer.writer.composer.txt")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.txt@utf-8/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.txt@utf-8/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.writer.writer.composer.md")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.md@commonmark/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.md@commonmark/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.writer.writer.composer.json")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.json@rfc8259/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.json@rfc8259/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.writer.writer.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"writer.document:writer")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "writer.document")?)?
                .claim(ArtifactIdentityClaim::codec_extension("writer.document", "writer")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.writer.writer.localization.en")?, ArtifactCapabilityKind::localization()).descriptor(b"Writer")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "Writer")?)?,
        )?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.writer.writer.localization.de")?, ArtifactCapabilityKind::localization()).descriptor(b"Writer")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "Writer")?)?)
}

/// 🗿️ Declaration-tree root (design.md §1/§2) — ONE standard (`1`), atomic cutover: the old
/// `.artifact(declaration())` + `.editor::<>()`/`.viewer::<>()` channel is deleted in the SAME pass
/// (plugin root `🦀️.rs`), never coexisting with this. `kind` uses `WRITER_DIALECT`'s own
/// `artifact_kind` ("s.writer.writer") — the documented canonical coordinate, not guessed.
pub fn artifact<A: WriterApplication>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<A> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use {semio_framework_artifact_reference::ArtifactKindId};
    ArtifactDeclaration { kind: ArtifactKindId::parse(WRITER_DIALECT.artifact_kind).expect("canonical writer kind"), localization: &[], standards: vec![crate::standards::v1::standard()] }
}

/// ✒️ Application variants required to assemble the Writer artifact.
pub trait WriterApplication:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<crate::editor::writer::WriterPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<crate::viewer::writer::WriterViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
{
}

impl<A> WriterApplication for A where
    A: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<crate::editor::writer::WriterPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<crate::viewer::writer::WriterViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
{
}
//#endregion 🔖️Declaration
}
//#endregion 🧩️ArtifactItems6
