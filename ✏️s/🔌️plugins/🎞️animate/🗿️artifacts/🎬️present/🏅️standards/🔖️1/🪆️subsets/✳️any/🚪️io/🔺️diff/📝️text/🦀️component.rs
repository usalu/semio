//! 🔺️ Animate present artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::present::schema::PresentArtifact;
use crate::artifacts::present::PresentSnapshot;
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::present::schema::diff::*;

//#region 🔖️Apply
impl PresentDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub async fn apply_to_artifact(&self, artifact: &PresentArtifact) -> protocol::MutationApplyResult<PresentArtifact> {
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

impl MutationDiff<PresentSnapshot> for PresentDiff {
    async fn apply(&self, snapshot: &PresentSnapshot) -> protocol::MutationApplyResult<PresentSnapshot> {
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
    async fn absorb(&mut self, other: Self) {
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
pub async fn diff_set_presentation(source: &crate::artifacts::present::FigureTileSource, tiles: &[crate::artifacts::present::FigureTileDraft]) -> PresentDiff {
    PresentDiff { presentation: Some(crate::artifacts::present::presentation_child_handle_and_cache(source, tiles)), ..Default::default() }
}
//#endregion 🔖️Helpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::present::default_present_snapshot;
    use crate::artifacts::present::op::PresentMutation;
    use crate::artifacts::present::schema::mutations::replace_source;
    use protocol::Mutation;

    #[semio_framework_async_macros::async_test]
    async fn replace_source_diff_applies_onto_the_base_snapshot() {
        let base = default_present_snapshot();
        let (source, _tiles) = crate::artifacts::present::present_working_scene(&base);
        let mut next_source = source.clone();
        next_source.kind = "video".into();
        let operation = PresentMutation::ReplaceSource(replace_source::mutation::ReplaceSource { new_source: next_source.clone() });
        let diff: PresentDiff = operation.diff(&base).into_parts().0;
        assert!(diff.presentation.is_some());
        assert!(diff.artifact.is_none());
        let (applied_source, _) = crate::artifacts::present::present_working_scene(&diff.apply(&base).expect("valid mutation diff"));
        assert_eq!(applied_source.kind, "video");
    }
}
//#endregion 🧪️Tests
