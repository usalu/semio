//! 🏅️ Forms standard `1` root (design.md §1) — `pub fn standard() -> StandardDeclaration`, mounting
//! its one subset (`✳️any`). New file: this level did not exist before ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM.

pub async fn standard() -> semio_framework_plugin::app::declarations::StandardDeclaration {
    use crate::artifacts::forms::standards::v1::subsets;
    use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
    use semio_framework_plugin::StandardId;

    StandardDeclaration {
        id: StandardId("1"),
        // 🪪️ `extensions: ["forms"]` is real, carried over from the native text codec's own
        // extension constant (`🚪️io/📸️snapshot/📝️text/🦀️component.rs`) and the old `definition()`'s own
        // `"s.forms.codec.document.v1"` capability row (`extension` claim = `"forms"`). No real MIME
        // registration exists anywhere in the pre-migration code for this artifact (only that codec
        // id + extension claim) — `application/vnd.semio.forms+json` is a documented synthesis,
        // matching `🎬️sequence`'s identical documented deviation (`📓️w4-sequence-report.md`
        // openQuestion 1). Flag for whoever eventually wires a real media-type registry.
        media: MediaDeclaration { mimes: &["application/vnd.semio.forms+json"], extensions: &["forms"] },
        subsets: vec![subsets::any::subset()],
    }
}
