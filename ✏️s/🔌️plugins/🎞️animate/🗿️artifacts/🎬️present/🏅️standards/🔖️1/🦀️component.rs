//! 🏅️ Standard root — `standard() -> StandardDeclaration` (design.md §1/§2 recipe step 6). Mounts
//! subset `any` for `s.animate.present@1`.

use crate::artifacts::present::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

/// 🪪️ `extensions: ["present"]` is the real, carried-over value (`ArtifactDsl::EXTENSION`, and the
/// old `definition()`'s `s.present.codec.document` capability's `extension` claim — both kept, per
/// debt D1). `mimes` is a documented synthesis: no real MIME registration exists anywhere in the
/// pre-migration code for this artifact (unlike stdio's `📜️artifact-definition.json`) — see
/// `## openQuestions` in the fan-out report, mirrors `🎬️sequence`'s identical documented deviation.
pub async fn standard() -> StandardDeclaration {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.animate.present"], extensions: &["present"] }, subsets: vec![subsets::any::subset()] }
}
