//! 🔧 Forms artifact — Op facet re-exports `FormMutation`.
pub use crate::artifacts::forms::mutations::{apply_form_edit_mutation, inverse_form_mutation, FormMutation};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::forms::engine::empty_forms_snapshot;

    #[test]
    fn update_form_mutation_sets_title() {
        let spec = empty_forms_snapshot();
        let mutation = FormMutation::UpdatePlaybook { title: Some("Renamed".into()) };
        let next = apply_form_edit_mutation(&spec, &mutation);
        assert_eq!(next.title.as_deref(), Some("Renamed"));
    }
}
//#endregion 🧪️Tests
