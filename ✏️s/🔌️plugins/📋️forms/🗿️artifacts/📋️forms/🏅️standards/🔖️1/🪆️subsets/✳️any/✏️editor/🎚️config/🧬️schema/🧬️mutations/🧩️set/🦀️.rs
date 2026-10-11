//! 🧩️ Set Contributions in the Forms configuration channel.

use super::*;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[dsl(keyword = "set-contributions")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetContributions {
    pub json: String,
}

impl protocol::MutationKind<FormsConfig, FormsConfigMutation> for SetContributions {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "contributions", kind: "set-contributions", record: "SetContributions" };
    fn diff(&self, _base: &FormsConfig) -> protocol::MutationOutcome<FormsConfigDiff> {
        protocol::MutationOutcome::new(FormsConfigDiff { contributions_json: Some(self.json.clone()) })
    }
    fn inverse(&self, base: &FormsConfig) -> Result<Vec<FormsConfigMutation>, semio_framework_value::ValueError> {
        Ok(vec![FormsConfigMutation::SetContributions(SetContributions { json: base.contributions_json.clone() })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Contributions", "Beiträge setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["contributions".into()]
    }
}
