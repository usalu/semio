//! 🧬️ Present diff schema — sparse field delta over the artifact.

use crate::artifacts::present::PresentationChild;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the present artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
/// `presentation` carries a whole-handle replacement (content-addressed, so a changed handle IS the
/// change signal — matches writer's `document: Option<WriterDocumentChild>` convention: this slot is
/// never absent, only ever replaced, so a single `Option<PresentationChild>` — not the double-`Option`
/// an optional slot needs — is the sparse-vs-unchanged signal here). `animation` never changes (see
/// `crate::artifacts::present::animation_child_handle`'s doc comment), so this diff carries no field
/// for it at all — nothing in this plugin yet produces a delta for that slot.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.animate.present")]
pub struct PresentDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::present::schema::PresentArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub presentation: Option<PresentationChild>,
    #[state(presence)]
    pub selected_ids: Option<PresentStringList>,
    #[state(config)]
    pub engagement_input: Option<String>,
    #[state(config)]
    pub locale: Option<String>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PresentStringList {
    pub values: Vec<String>,
}
//#endregion 🔖️DeltaHelpers
