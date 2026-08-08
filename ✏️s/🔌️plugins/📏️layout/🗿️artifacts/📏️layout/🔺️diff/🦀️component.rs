//! 🔺️ Layout artifact — the operation diff (constitutional: diff).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::LayoutDocument;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 📦️ Mutation-list diff: layout mutations fold sequentially over a cloned projection. `absorb`
/// concatenates — sequential edits replay forwards in order.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LayoutDiff {
    pub mutations: Vec<LayoutMutation>,
}

impl MutationDiff<LayoutDocument> for LayoutDiff {
    fn apply(&self, projection: &LayoutDocument) -> LayoutDocument {
        let mut next = projection.clone();
        for mutation in &self.mutations {
            crate::artifacts::layout::mutations::apply_layout_mutation(&mut next, mutation);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        self.mutations.extend(other.mutations);
    }
}
//#endregion 🔖️Diff

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::Mutation;

    /// ⚖️ LAW: `op.diff(base)` applied to `base` equals applying the operation directly.
    #[test]
    fn set_data_fields_diff_applies_onto_the_base_projection() {
        let base = LayoutDocument {
            schema: crate::artifacts::layout::LAYOUT_FIXTURE_SCHEMA.into(),
            name: "t".into(),
            grid: crate::artifacts::layout::GridSettings { baseline_grid: 12.0, baseline_offset: 0.0, snap_to_baseline: false },
            paragraph_styles: Vec::new(),
            character_styles: Vec::new(),
            stories: Vec::new(),
            links: Vec::new(),
            parent_pages: Vec::new(),
            spreads: Vec::new(),
            pages: Vec::new(),
            print_target: None,
            data_fields_json: None,
        };
        let operation = LayoutMutation::SetDataFields { json: Some("{}".into()) };
        let diff: LayoutDiff = operation.diff(&base);
        assert_eq!(diff.mutations.len(), 1);
        let applied = diff.apply(&base);
        assert_eq!(applied.data_fields_json.as_deref(), Some("{}"));
    }

    #[test]
    fn absorb_concatenates_operations_in_order() {
        let mut diff = LayoutDiff { mutations: vec![LayoutMutation::SetDataFields { json: Some("a".into()) }] };
        diff.absorb(LayoutDiff { mutations: vec![LayoutMutation::SetDataFields { json: Some("b".into()) }] });
        assert_eq!(diff.mutations.len(), 2);
    }
}
//#endregion 🧪️Tests
