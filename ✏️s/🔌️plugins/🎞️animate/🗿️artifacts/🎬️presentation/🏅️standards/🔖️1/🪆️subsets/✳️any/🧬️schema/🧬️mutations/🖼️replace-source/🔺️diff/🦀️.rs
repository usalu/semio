//! 🔺️ Sparse diff construction for `replace-source`.
use super::ReplaceSource;
use crate::diff::{PresentationDiff, PresentationOptionalAspect, PresentationOptionalPage, PresentationSourcePatch};
use crate::PresentationSnapshot;

//#region 🔹Diff
/// 🔺️ The source fields that differ from `payload.new_source`, read off `base.source`; `apply` re-derives the `presentation` handle.
pub fn diff(payload: &ReplaceSource, base: &PresentationSnapshot) -> protocol::MutationOutcome<PresentationDiff> {
    let (held, next) = (&base.source, &payload.new_source);
    let patch = PresentationSourcePatch {
        src: (held.src != next.src).then(|| next.src.clone()),
        kind: (held.kind != next.kind).then(|| next.kind.clone()),
        frame: (held.frame != next.frame).then(|| next.frame.clone()),
        source_aspect: (held.source_aspect != next.source_aspect).then(|| PresentationOptionalAspect { value: next.source_aspect }),
        pdf_page: (held.pdf_page != next.pdf_page).then(|| PresentationOptionalPage { value: next.pdf_page }),
    };
    if patch == PresentationSourcePatch::default() {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Source is already unchanged.".to_string());
    }
    protocol::MutationOutcome::new(PresentationDiff { source: Some(patch), ..Default::default() })
}
//#endregion 🔹Diff
