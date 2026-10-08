//! 🧬️ Replace Presence in the presentation.presence channel.

use super::*;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "replace-presence")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplacePresence {
    #[dsl(block)]
    pub presence: PresentationPresence,
}

impl protocol::MutationKind<PresentationPresence, PresentationPresenceMutation> for ReplacePresence {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "presence", kind: "replace-presence", record: "ReplacePresence" };
    fn diff(&self, _base: &PresentationPresence) -> protocol::MutationOutcome<PresentationPresenceDiff> {
        protocol::MutationOutcome::new(PresentationPresenceDiff {})
    }
    fn inverse(&self, base: &PresentationPresence) -> Result<Vec<PresentationPresenceMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![PresentationPresenceMutation::ReplacePresence(Self { presence: base.clone() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace Presence", "Präsenz ersetzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["presence".into()]
    }
}

#[cfg(test)]
mod law_tests {
    use super::*;

    /// ⚖️ The inverse diffs sum to the negative of the forward diff (L3).
    #[semio_framework_async_macros::async_test]
    async fn inverse_diffs_sum_to_the_negative_diff() {
        let base = PresentationPresence::default();
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&PresentationPresenceMutation::ReplacePresence(ReplacePresence { presence: PresentationPresence::default() }), &base).await;
    }
}
