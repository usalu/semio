//! 🏅️ Standard root for `s.vcs.vcs` standard `1` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM). Exports `standard() -> StandardDeclaration`,
//! mounting subset `any` (the only subset this standard has). No real mime registration exists for
//! this artifact anywhere in the codebase (unlike stdio's `📜️artifact-definition.json`) — the old
//! `ArtifactCapability` channel only ever claimed a codec id (`vcs.vcs`) and an extension (`vcs`),
//! never a mime type, so `mimes` here is a documented synthesis, not a literal carry-over (see
//! `📓️w4-vcs-report.md` `## openQuestions`, mirrors the sequence fan-out's identical deviation).

use crate::artifacts::vcs::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

pub fn standard() -> StandardDeclaration<crate::VcsApps> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.vcs+json"], extensions: &["vcs"] }, subsets: vec![subsets::any::subset()] }
}
