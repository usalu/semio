use super::{WriterMainWindowTransient, WriterMainWindowTransientMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-engagement-input")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetEngagementInput {
    pub value: String,
}

impl protocol::MutationKind<WriterMainWindowTransient, WriterMainWindowTransientMutation> for SetEngagementInput {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "writer-window-engagement-input", kind: "set-engagement-input", record: "SetEngagementInput" };
    fn diff(&self, base: &WriterMainWindowTransient) -> protocol::MutationOutcome<WriterMainWindowTransient> {
        let mut next = base.clone();
        next.engagement_input.clone_from(&self.value);
        protocol::MutationOutcome::new(next)
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
