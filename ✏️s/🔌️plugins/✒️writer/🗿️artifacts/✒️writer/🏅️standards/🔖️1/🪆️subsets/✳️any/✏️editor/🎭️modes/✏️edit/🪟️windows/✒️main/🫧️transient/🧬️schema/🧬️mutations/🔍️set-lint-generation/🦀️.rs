use super::{WriterMainWindowTransient, WriterMainWindowTransientDiff, WriterMainWindowTransientMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[dsl(keyword = "set-lint-generation")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetLintGeneration {
    pub value: u32,
}

impl protocol::MutationKind<WriterMainWindowTransient, WriterMainWindowTransientMutation> for SetLintGeneration {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "writer-window-lint-generation", kind: "set-lint-generation", record: "SetLintGeneration" };
    fn diff(&self, base: &WriterMainWindowTransient) -> protocol::MutationOutcome<WriterMainWindowTransientDiff> {
        protocol::MutationOutcome::new(WriterMainWindowTransientDiff { lint_generation: (base.lint_generation != self.value).then_some(self.value), ..Default::default() })
    }

    fn inverse(&self, base: &WriterMainWindowTransient) -> Result<Vec<WriterMainWindowTransientMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { value: base.lint_generation }.into()]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Writer Window Lint Generation", "Prüfstand des Schreibfensters setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["lint_generation".into()]
    }
}

#[cfg(test)]
mod law_tests {
    use super::*;

    /// ⚖️ The inverse diffs sum to the negative of the forward diff (L3).
    #[semio_framework_async_macros::async_test]
    async fn inverse_diffs_sum_to_the_negative_diff() {
        let base = WriterMainWindowTransient::default();
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&WriterMainWindowTransientMutation::from(SetLintGeneration { value: 7 }), &base).await;
    }
}
