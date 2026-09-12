//! ☑️ Changes the selected compliance result without replacing unrelated config.

use crate::results_window_config::{NormResultsWindowConfig, NormResultsWindowConfigMutation};

#[derive(Clone, Debug, PartialEq, dsl::DslRecord, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[dsl(keyword = "change-selected-check-index")]
#[mutation_leaf(contract = ::protocol)]
#[value(deny_unknown_fields)]
pub struct ChangeSelectedCheckIndex {
    pub index: Option<u32>,
}

impl protocol::MutationKind<NormResultsWindowConfig, NormResultsWindowConfigMutation> for ChangeSelectedCheckIndex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "selected-check-index", kind: "change-selected-check-index", record: "ChangedSelectedCheckIndex" };

    fn diff(&self, base: &NormResultsWindowConfig) -> protocol::MutationOutcome<NormResultsWindowConfig> {
        if base.selected_check_index == self.index {
            return protocol::MutationOutcome::new(base.clone()).warn("mutation.no-op", "Selected check index is already this value.");
        }
        protocol::MutationOutcome::new(NormResultsWindowConfig { selected_check_index: self.index })
    }

    fn inverse(&self, base: &NormResultsWindowConfig) -> Vec<NormResultsWindowConfigMutation> {
        vec![Self { index: base.selected_check_index }.into()]
    }

    fn label(&self) -> String {
        match self.index {
            Some(index) => format!("Select compliance check {index}"),
            None => "Clear selected compliance check".into(),
        }
    }
}
