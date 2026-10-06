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

use crate::standards::v1::subsets::any::schema::PresentationArtifact;
use crate::PresentationSnapshot;
use protocol::MutationDiff;

impl PresentationDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &PresentationArtifact) -> protocol::MutationApplyResult<PresentationArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(source) = &self.source {
                next.source = source.clone();
            }
            if let Some(tiles) = &self.tiles {
                next.tiles = tiles.clone();
            }
            if let Some(presentation) = &self.presentation {
                next.presentation = presentation.clone();
            }
            next
        })
    }
}

impl MutationDiff<PresentationSnapshot> for PresentationDiff {
    fn apply(&self, snapshot: &PresentationSnapshot) -> protocol::MutationApplyResult<PresentationSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(source) = &self.source {
                next.source = source.clone();
            }
            if let Some(tiles) = &self.tiles {
                next.tiles = tiles.clone();
            }
            if let Some(presentation) = &self.presentation {
                next.presentation = presentation.clone();
            }
            next
        })
    }
    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(source);
        take!(tiles);
        take!(presentation);
    }
}

/// 🔺️ Mints a new content-addressed `presentation` handle for a whole `(source, tiles)`
/// replacement and carries that exact payload beside it — real handcrafted construction, never
/// apply-then-capture, never a snapshot clone. The standard builder every mutation triad in this
/// facet's `🧬️mutations` uses.
pub fn diff_set_presentation(source: &crate::FigureTileSource, tiles: &[crate::FigureTileDraft]) -> PresentationDiff {
    PresentationDiff { source: Some(source.clone()), tiles: Some(tiles.to_vec()), presentation: Some(crate::presentation_child_handle(source, tiles)), ..Default::default() }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
