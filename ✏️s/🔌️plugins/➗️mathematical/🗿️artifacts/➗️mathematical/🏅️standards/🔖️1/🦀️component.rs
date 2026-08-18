//! 🏅️ Standard root for `s.mathematical.mathematical` standard `1` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM). Exports `standard() -> StandardDeclaration`,
//! mounting subset `any` (the only subset this standard has). No real mime/extension registration
//! exists for this artifact anywhere in the codebase (unlike stdio's `📜️artifact-definition.json`)
//! — the old `ArtifactCapability` channel (`crate::artifacts::mathematical::definition`) only ever
//! claimed a codec id (`semio.mathematical/v1`) and an extension (`mathematical`), never a mime
//! type, so `mimes` here is a documented synthesis, not a literal carry-over (see the W4 report's
//! `## openQuestions`, mirrors `🎬️sequence`'s identical documented deviation).

use crate::artifacts::mathematical::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

pub fn standard() -> StandardDeclaration {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.mathematical+json"], extensions: &["mathematical"] }, subsets: vec![subsets::any::subset()] }
}
