use super::{WriterMainWindowTransient, WriterMainWindowTransientDiff, WriterMainWindowTransientMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-engagement-input")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetEngagementInput {
    pub value: String,
}

impl protocol::MutationKind<WriterMainWindowTransient, WriterMainWindowTransientMutation> for SetEngagementInput {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "writer-window-engagement-input", kind: "set-engagement-input", record: "SetEngagementInput" };
    fn diff(&self, base: &WriterMainWindowTransient) -> protocol::MutationOutcome<WriterMainWindowTransientDiff> {
        protocol::MutationOutcome::new(WriterMainWindowTransientDiff { engagement_input: (base.engagement_input != self.value).then(|| self.value.clone()), ..Default::default() })
    }

    fn inverse(&self, base: &WriterMainWindowTransient) -> Result<Vec<WriterMainWindowTransientMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { value: base.engagement_input.clone() }.into()]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Writer Window Engagement Input", "Interaktionseingabe des Schreibfensters setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["engagement_input".into()]
    }
}

#[cfg(test)]
mod law_tests {
    use super::*;

    /// ⚖️ The inverse diffs sum to the negative of the forward diff (L3).
    #[semio_framework_async_macros::async_test]
    async fn inverse_diffs_sum_to_the_negative_diff() {
        let base = WriterMainWindowTransient::default();
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&WriterMainWindowTransientMutation::from(SetEngagementInput { value: "typing".into() }), &base).await;
    }
}
