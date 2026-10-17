//! 🧩️ Set Contributions in the Forms configuration channel.

use crate::editor::forms::config::component::mutations::*;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[dsl(keyword = "set-contributions")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetContributions {
    pub json: String,
}

impl<Op> protocol::MutationKind<FormsConfig, Op> for SetContributions
where
    Op: protocol::Mutation<FormsConfig, Diff = FormsConfigDiff> + From<SetContributions>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "contributions", kind: "set-contributions", record: "SetContributions" };
    fn diff(&self, _base: &FormsConfig) -> protocol::MutationOutcome<FormsConfigDiff> {
        protocol::MutationOutcome::new(FormsConfigDiff { contributions_json: Some(self.json.clone()) })
    }
    fn inverse(&self, base: &FormsConfig) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(vec![Op::from(SetContributions { json: base.contributions_json.clone() })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Contributions", "Beiträge setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["contributions".into()]
    }
}
