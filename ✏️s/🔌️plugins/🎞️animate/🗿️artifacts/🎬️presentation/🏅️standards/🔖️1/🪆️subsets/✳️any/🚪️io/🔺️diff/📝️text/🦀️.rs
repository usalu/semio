//! 🔺️ Animate presentation artifact — sparse field-delta diff codec and apply/absorb.

use crate::standards::v1::subsets::any::schema::PresentationArtifact;
use crate::PresentationSnapshot;
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::standards::v1::subsets::any::schema::diff::*;

//#region 🔖️Apply
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
        take!(presentation);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
/// 🔺️ Mints a new content-addressed `presentation` handle for a whole `(source, tiles)`
/// replacement and seeds the working-scene cache with it (`presentation_child_handle_and_cache`) —
/// real handcrafted construction, never apply-then-capture, never a snapshot clone. The standard
/// builder every mutation triad in this facet's `🧬️mutations` uses.
pub fn diff_set_presentation(source: &crate::FigureTileSource, tiles: &[crate::FigureTileDraft]) -> PresentationDiff {
    PresentationDiff { presentation: Some(crate::presentation_child_handle_and_cache(source, tiles)), ..Default::default() }
}
//#endregion 🔖️Helpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
