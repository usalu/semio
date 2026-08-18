//! 🏅️ Wires standard `1` root — `pub fn standard() -> StandardDeclaration`, mounts subset `any`
//! (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §1/§2).

use crate::artifacts::wires::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

/// 🧬️ `mimes` is a documented synthesis, not a literal carry-over — no real MIME registration
/// exists anywhere in the pre-migration code for this artifact (the old `ArtifactCapability` channel
/// in `🗿️artifacts/🔌️wires/🦀️component.rs`'s `definition()` only ever claimed a codec id
/// (`reasoning.wires.fixture`) and an extension (`wires`), never a mime type — same documented
/// shortfall as `🎬️sequence`'s own standard root, see `📓️w4-reasoning-report.md` `## openQuestions`).
/// `extensions: ["wires"]` is the real, carried-over value.
pub fn standard() -> StandardDeclaration {
    StandardDeclaration {
        id: StandardId("1"),
        media: MediaDeclaration { mimes: &["application/vnd.semio.wires+json"], extensions: &["wires"] },
        subsets: vec![subsets::any::subset()],
    }
}
