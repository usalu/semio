//! 🧬️ Presentation diff schema — sparse field delta over the artifact.

use crate::PresentationChild;
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the presentation artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
/// `presentation` carries a whole-handle replacement (content-addressed, so a changed handle IS the
/// change signal — matches writer's `document: Option<WriterDocumentChild>` convention: this slot is
/// never absent, only ever replaced, so a single `Option<PresentationChild>` — not the double-`Option`
/// an optional slot needs — is the sparse-vs-unchanged signal here). `animation` never changes (see
/// `crate::animation_child_handle`'s doc comment), so this diff carries no field
/// for it at all — nothing in this plugin yet produces a delta for that slot.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[artifact_schema(id = "s.animate.presentation")]
pub struct PresentationDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::standards::v1::subsets::any::schema::PresentationArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    /// 🖼️ The persisted payload the replacement `presentation` handle was minted from — it travels
    /// WITH the handle so an applied diff leaves the parent able to re-derive its child through
    /// `genesis_presentation_child_pack` (see `PresentationSnapshot::source`).
    #[state(artifact)]
    pub source: Option<crate::FigureTileSource>,
    /// 🧱 See [`PresentationDiff::source`].
    #[state(artifact)]
    pub tiles: Option<Vec<crate::FigureTileDraft>>,
    #[state(artifact)]
    pub presentation: Option<PresentationChild>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct PresentationStringList {
    pub values: Vec<String>,
}
//#endregion 🔖️DeltaHelpers
