//! 🏅️ Standard root for `s.sequence.sequence` standard `1` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM). Exports `standard() -> StandardDeclaration`,
//! mounting subset `any` (the only subset this standard has). No real mime/extension registration
//! exists for this artifact anywhere in the codebase (unlike stdio's `📜️artifact-definition.json`) —
//! the old `ArtifactCapability` channel only ever claimed a codec id (`sequence.sequence`) and an
//! extension (`sequence`), never a mime type, so `mimes` here is a documented synthesis, not a
//! literal carry-over (see `📓️w4-sequence-report.md` `## openQuestions`).

use crate::artifacts::sequence::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

pub async fn standard() -> StandardDeclaration {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.sequence+json"], extensions: &["sequence"] }, subsets: vec![subsets::any::subset()] }
}
