//! 🏅️ Standard root for `s.mathematical.equation` standard `1` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM). Exports `standard() -> StandardDeclaration`,
//! mounting subset `any` (the only subset this standard has). No real mime/extension registration
//! exists for this artifact anywhere in the codebase (unlike stdio's `📜️artifact-definition.json`)
//! — the old `ArtifactCapability` channel (`crate::artifacts::equation::definition`) only ever
//! claimed a codec id (`semio.equation/v1`) and an extension (`equation`), never a mime
//! type, so `mimes` here is a documented synthesis, not a literal carry-over (see the W4 report's
//! `## openQuestions`, mirrors `🎬️sequence`'s identical documented deviation).

use crate::artifacts::equation::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

pub async fn standard() -> StandardDeclaration {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.equation+json"], extensions: &["equation"] }, subsets: vec![subsets::any::subset()] }
}
