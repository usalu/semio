//! 🏅️ Writer standard `1` root — mounts subset `any` and exports `standard() -> StandardDeclaration`
//! (design.md §1). `mimes`/`extensions`: no real MIME registration exists anywhere for this
//! artifact (unlike stdio's `📜️artifact-definition.json`) — the old `ArtifactCapability` channel
//! only ever claimed a codec id (`writer.document`) and an extension (`writer`), never a mime type
//! (`🗿️artifacts/✒️writer/🦀️component.rs`'s `definition()`, capability `s.writer.codec.document`).
//! `mimes` below is therefore a documented synthesis (see `## openQuestions`), `extensions` is the
//! real, carried-over value — same documented shape `📓️w4-sequence-report.md` used for `🎬️sequence`.

use crate::artifacts::writer::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

//#region 🔖️Standard
pub async fn standard() -> StandardDeclaration {
    StandardDeclaration {
        id: StandardId("1"),
        media: MediaDeclaration { mimes: &["application/vnd.semio.writer+json"], extensions: &["writer"] },
        subsets: vec![subsets::any::subset()],
    }
}
//#endregion 🔖️Standard

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn standard_mounts_exactly_one_subset() {
        assert_eq!(standard().subsets.len(), 1);
    }
}
