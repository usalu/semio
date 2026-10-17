//! 🏅️ Wires standard `1` root — `pub fn standard() -> StandardDeclaration`, mounts subset `any`
//! (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §1/§2).

use crate::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use {semio_framework_artifact_reference::StandardId};

/// 🧬️ `mimes` is a documented synthesis, not a literal carry-over — no real MIME registration
/// exists anywhere in the pre-migration code for this artifact (the old `ArtifactCapability` channel
/// in `🗿️artifacts/🔌️wires/🦀️.rs`'s `definition()` only ever claimed a codec id
/// (`reasoning.wires.identity.snapshot`) and an extension (`wires`), never a mime type — same documented
/// shortfall as `🎬️sequence`'s own standard root, see `📓️w4-reasoning-report.md` `## openQuestions`).
/// `extensions: ["wires"]` is the real, carried-over value.
pub fn standard<A: crate::WiresApplication>() -> StandardDeclaration<A> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.wires+json"], extensions: &["wires"] }, subsets: vec![subsets::any::subset()] }
}

//#region 🧩️ArtifactItems5
pub mod artifact_items_5 {
//#region 🔖️Declaration
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    ArtifactDefinition::new(ArtifactIdentity::parse("s.reasoning.wires")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.reasoning.wires.schema.artifact")?, ArtifactCapabilityKind::schema())
                .descriptor(b"s.reasoning.wires")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.reasoning.wires")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.reasoning.wires.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.reasoning.wires.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.reasoning.wires.inference")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.reasoning.wires.composer.native")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.reasoning.wires@1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.reasoning.wires@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.reasoning.wires.composer.json")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.json@rfc8259/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.json@rfc8259/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.reasoning.wires.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"reasoning.wires.identity.snapshot:wires")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "reasoning.wires.identity.snapshot")?)?
                .claim(ArtifactIdentityClaim::codec_extension("reasoning.wires.identity.snapshot", "wires")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.reasoning.wires.localization.en")?, ArtifactCapabilityKind::localization())
                .descriptor(b"Mindmap Wires")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "Mindmap Wires")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.reasoning.wires.localization.de")?, ArtifactCapabilityKind::localization())
                .descriptor(b"Mindmap-Wires")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "Mindmap-Wires")?)?,
        )
}

/// 🗿️ New declaration-tree registration channel (ticket
/// `26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM` design.md §1/§2) — the ONLY channel: the old
/// `declaration()` (`ArtifactDeclaration::builder(...).schema(...).inferences(...).composers(...)
/// .document_codec(...)` chain) is deleted outright, not kept alongside this, per the ticket's own
/// "Rejected approaches" ruling against dual registration. `localization: &[]` is a documented
/// shortfall: the real en/de localized names (`"Mindmap Wires"`/`"Mindmap-Wires"`) still live on
/// `definition()`'s `ArtifactCapability` rows above (kept, per debt D1) — wiring them into this
/// field is real follow-up work, not required for this pass (mirrors `📓️w4-sequence-report.md`
/// `## openQuestions` #2).
pub fn artifact<A: WiresApplication>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<A> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use {semio_framework_artifact_reference::ArtifactKindId};
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.reasoning.wires").expect("canonical reasoning.wires kind"), localization: &[], standards: vec![crate::standards::v1::standard()] }
}

/// 🧩️ App fleet capable of hosting this artifact's editor and viewer.
pub trait WiresApplication:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::wires::ReasoningWiresPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>>
    + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::wires::WiresViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
{
}

impl<A> WiresApplication for A where
    A: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::wires::ReasoningWiresPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>>
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::wires::WiresViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
{
}

//#endregion 🔖️Declaration
}
//#endregion 🧩️ArtifactItems5
