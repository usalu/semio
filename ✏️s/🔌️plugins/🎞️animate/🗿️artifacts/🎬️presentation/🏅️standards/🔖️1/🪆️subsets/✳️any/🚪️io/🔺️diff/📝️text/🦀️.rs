//! 🔺️ Animate presentation artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::presentation::schema::PresentationArtifact;
use crate::artifacts::presentation::PresentationSnapshot;
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::presentation::schema::diff::*;

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
            if let Some(list) = &self.selected_ids {
                next.selected_ids = list.values.clone();
            }
            if let Some(value) = &self.engagement_input {
                next.engagement_input = value.clone();
            }
            if let Some(value) = &self.locale {
                next.locale = value.clone();
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
        take!(selected_ids);
        take!(engagement_input);
        take!(locale);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
/// 🔺️ Mints a new content-addressed `presentation` handle for a whole `(source, tiles)`
/// replacement and seeds the working-scene cache with it (`presentation_child_handle_and_cache`) —
/// real handcrafted construction, never apply-then-capture, never a snapshot clone. The standard
/// builder every mutation triad in this facet's `🧬️mutations` uses.
pub fn diff_set_presentation(source: &crate::artifacts::presentation::FigureTileSource, tiles: &[crate::artifacts::presentation::FigureTileDraft]) -> PresentationDiff {
    PresentationDiff { presentation: Some(crate::artifacts::presentation::presentation_child_handle_and_cache(source, tiles)), ..Default::default() }
}
//#endregion 🔖️Helpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::presentation::default_presentation_snapshot;
    use crate::artifacts::presentation::op::PresentationMutation;
    use crate::artifacts::presentation::schema::mutations::replace_source;
    use protocol::Mutation;

    #[test]
    fn replace_source_diff_applies_onto_the_base_snapshot() {
        let base = default_presentation_snapshot();
        let (source, _tiles) = crate::artifacts::presentation::presentation_working_scene(&base);
        let mut next_source = source;
        next_source.kind = "video".into();
        let operation = PresentationMutation::ReplaceSource(replace_source::mutation::ReplaceSource { new_source: next_source });
        let diff: PresentationDiff = operation.diff(&base).into_parts().0;
        assert!(diff.presentation.is_some());
        assert!(diff.artifact.is_none());
        let (applied_source, _) = crate::artifacts::presentation::presentation_working_scene(&diff.apply(&base).expect("valid mutation diff"));
        assert_eq!(applied_source.kind, "video");
    }
}
//#endregion 🧪️Tests
