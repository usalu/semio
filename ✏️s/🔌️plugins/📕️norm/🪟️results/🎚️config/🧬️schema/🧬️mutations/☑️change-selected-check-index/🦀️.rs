//! ☑️ Changes the selected compliance result without replacing unrelated config.

use crate::results_window_config::diff::NormResultsWindowConfigDiff;
use crate::results_window_config::{NormResultsWindowConfig, NormResultsWindowConfigMutation};

#[path = "🔺️diff/🦀️.rs"]
mod diff;
#[path = "↩️inverse/🦀️.rs"]
mod inverse;

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[dsl(keyword = "change-selected-check-index")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChangeSelectedCheckIndex {
    pub index: Option<u32>,
}

impl protocol::MutationKind<NormResultsWindowConfig, NormResultsWindowConfigMutation> for ChangeSelectedCheckIndex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "selected-check-index", kind: "change-selected-check-index", record: "ChangedSelectedCheckIndex" };

    fn diff(&self, base: &NormResultsWindowConfig) -> protocol::MutationOutcome<NormResultsWindowConfigDiff> {
        diff::diff(self, base)
    }

    fn inverse(&self, base: &NormResultsWindowConfig) -> Result<Vec<NormResultsWindowConfigMutation>, semio_framework_value::ValueError> {
        inverse::inverse(self, base)
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&{
        match self.index {
            Some(index) => format!("Select compliance check {index}"),
            None => "Clear selected compliance check".into(),
        }
        }, &{
        match self.index {
            Some(index) => format!("Konformitätsprüfung {index} auswählen"),
            None => "Auswahl der Konformitätsprüfung aufheben".into(),
        }
        })
    }
}

#[cfg(test)]
#[path = "🧪️tests/✅apply/🦀️.rs"]
mod named_test;
