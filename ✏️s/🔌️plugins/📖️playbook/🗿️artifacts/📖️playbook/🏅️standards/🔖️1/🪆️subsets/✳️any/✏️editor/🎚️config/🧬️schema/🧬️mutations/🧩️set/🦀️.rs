//! 🧩️ Set Contributions in the Playbook configuration channel.

use super::{PlaybookConfig, PlaybookConfigDiff, PlaybookConfigMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[dsl(keyword = "set-contributions")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetContributions {
    pub json: String,
}

impl protocol::MutationKind<PlaybookConfig, PlaybookConfigMutation> for SetContributions {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "contributions", kind: "set-contributions", record: "SetContributions" };
    fn diff(&self, base: &PlaybookConfig) -> protocol::MutationOutcome<PlaybookConfigDiff> {
        protocol::MutationOutcome::new(PlaybookConfigDiff { contributions_json: (base.contributions_json != self.json).then(|| self.json.clone()) })
    }
    fn inverse(&self, base: &PlaybookConfig) -> Result<Vec<PlaybookConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![PlaybookConfigMutation::SetContributions(Self { json: base.contributions_json.clone() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Contributions", "Beiträge setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["contributions".into()]
    }
}

#[cfg(test)]
mod law_tests {
    use super::*;

    /// ⚖️ The inverse diffs sum to the negative of the forward diff (L3).
    #[semio_framework_async_macros::async_test]
    async fn inverse_diffs_sum_to_the_negative_diff() {
        let base = PlaybookConfig::default();
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&PlaybookConfigMutation::SetContributions(SetContributions { json: "[2]".into() }), &base).await;
    }
}
