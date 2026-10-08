//! 🧬️ Set Engagement Input in the presentation.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-engagement-input")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetEngagementInput {
    pub value: String,
}

impl protocol::MutationKind<PresentationConfig, PresentationConfigMutation> for SetEngagementInput {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "engagement-input", kind: "set-engagement-input", record: "SetEngagementInput" };
    fn diff(&self, base: &PresentationConfig) -> protocol::MutationOutcome<PresentationConfigDiff> {
        protocol::MutationOutcome::new(PresentationConfigDiff { engagement_input: (base.engagement_input != self.value).then(|| self.value.clone()) })
    }
    fn inverse(&self, base: &PresentationConfig) -> Result<Vec<PresentationConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![PresentationConfigMutation::SetEngagementInput(Self { value: base.engagement_input.clone() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Engagement Input", "Interaktionseingabe setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["engagementInput".into()]
    }
}

#[cfg(test)]
mod law_tests {
    use super::*;

    /// ⚖️ The inverse diffs sum to the negative of the forward diff (L3).
    #[semio_framework_async_macros::async_test]
    async fn inverse_diffs_sum_to_the_negative_diff() {
        let base = PresentationConfig::default();
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&PresentationConfigMutation::SetEngagementInput(SetEngagementInput { value: "draft".into() }), &base).await;
    }
}
