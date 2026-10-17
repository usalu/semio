//! 🏅️ Standard root for `s.sequence.sequence` standard `1` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM). Exports `standard() -> StandardDeclaration`,
//! mounting subset `any` (the only subset this standard has). No real mime/extension registration
//! exists for this artifact anywhere in the codebase (unlike stdio's `📜️artifact-definition.json`) —
//! the old `ArtifactCapability` channel only ever claimed a codec id (`sequence.sequence`) and an
//! extension (`sequence`), never a mime type, so `mimes` here is a documented synthesis, not a
//! literal carry-over (see `📓️w4-sequence-report.md` `## openQuestions`).

use crate::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use {semio_framework_artifact_reference::StandardId};

pub fn standard<A: crate::SequenceApplication>() -> StandardDeclaration<A> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.sequence+json"], extensions: &["sequence"] }, subsets: vec![subsets::any::subset()] }
}

//#region 🧩️ArtifactItems4
pub mod artifact_items_4 {
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::sequence::app::create_sequence_app`'s `🔖️Manifest` region. Lifted verbatim out of the
/// old `.artifact_kind(...)` builder call.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "computation.sequence".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Sequence", "Sequenz"),
        source_format: "sequence.sequence".into(),
        component_kind: "sequence".into(),
        dimension: "graph".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Sequence },
        schema: "sequence.sequence".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.csv".into(), "stdio.json".into(), "stdio.md".into()],
        import_stdio_kinds: vec!["stdio.csv".into(), "stdio.json".into(), "stdio.md".into()],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems4

//#region 🧩️ArtifactItems6
pub mod artifact_items_6 {
//#region 🔖️ArtifactDeclaration
/// 🌳️ New tree (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM): the whole
/// `s.sequence.sequence` artifact through the declaration tree — one standard (`1`), one subset
/// (`any`). Replaces the OLD `declaration()`/`ArtifactDeclaration::builder(...)` channel outright
/// (atomic cutover; both channels never coexist). `localization: &[]` is a documented shortfall,
/// not an oversight — the real en/de localized descriptors still live on `definition()`'s
/// `ArtifactCapability` rows above (kept per debt D1, deleted repo-wide only in W6); wiring them
/// into this field too is real follow-up work, not required for the tree to register or for any
/// law to hold (mirrors the stdio pilot's own documented deviation, `📓️w2-p-report.md`).
pub fn artifact<A: SequenceApplication>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<A> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use {semio_framework_artifact_reference::ArtifactKindId};
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.sequence.sequence").expect("canonical sequence.sequence kind"), localization: &[], standards: vec![crate::standards::v1::standard()] }
}

/// 🧩️ App fleet capable of hosting this artifact's editor and viewer.
pub trait SequenceApplication:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::sequence::app::SequencePlayApp>, semio_s_artifact_stdio_semio::SemioMembers>>
    + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::sequence::SequenceViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
{
}

impl<A> SequenceApplication for A where
    A: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::sequence::app::SequencePlayApp>, semio_s_artifact_stdio_semio::SemioMembers>>
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::sequence::SequenceViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
{
}

//#endregion 🔖️ArtifactDeclaration
}
//#endregion 🧩️ArtifactItems6
