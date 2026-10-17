//! 🏅️ Standard root for `s.vcs.vcs` standard `1` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM). Exports `standard() -> StandardDeclaration`,
//! mounting subset `any` (the only subset this standard has). No real mime registration exists for
//! this artifact anywhere in the codebase (unlike stdio's `📜️artifact-definition.json`) — the old
//! `ArtifactCapability` channel only ever claimed a codec id (`vcs.vcs`) and an extension (`vcs`),
//! never a mime type, so `mimes` here is a documented synthesis, not a literal carry-over (see
//! `📓️w4-vcs-report.md` `## openQuestions`, mirrors the sequence fan-out's identical deviation).

use crate::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use {semio_framework_artifact_reference::StandardId};

pub fn standard<A: crate::VcsApplication>() -> StandardDeclaration<A> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.vcs+json"], extensions: &["vcs"] }, subsets: vec![subsets::any::subset()] }
}

//#region 🧩️ArtifactItems6
pub mod artifact_items_6 {
use crate::{VcsApplication};

//#region 🔖️ArtifactDeclaration
/// 🌳️ New tree (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM): the whole `s.vcs.vcs`
/// artifact through the declaration tree — one standard (`1`), one subset (`any`). Replaces the OLD
/// `declaration()`/`ArtifactDeclaration::builder(...)` channel outright (atomic cutover; both
/// channels never coexist — a second parallel registration channel is the compatibility layer
/// CLAUDE.md forbids). `localization: &[]` is a documented shortfall, not an oversight — the real
/// en/de localized descriptors still live on `definition()`'s `ArtifactCapability` rows above (kept
/// per debt D1, deleted repo-wide only in W6); wiring them into this field too is real follow-up
/// work, not required for the tree to register or for any law to hold (mirrors the stdio pilot's
/// own documented deviation, `📓️w2-p-report.md`).
pub fn artifact<A: VcsApplication>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<A> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use {semio_framework_artifact_reference::ArtifactKindId};
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.vcs.vcs").expect("canonical vcs.vcs kind"), localization: &[], standards: vec![crate::standards::v1::standard()] }
}
//#endregion 🔖️ArtifactDeclaration
}
//#endregion 🧩️ArtifactItems6
