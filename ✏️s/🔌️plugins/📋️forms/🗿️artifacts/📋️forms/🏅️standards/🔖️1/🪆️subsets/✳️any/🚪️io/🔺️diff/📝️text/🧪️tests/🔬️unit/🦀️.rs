
use super::*;
use crate::mutations::create_step;
use crate::{FORMS_DOCUMENT_SCHEMA, FormStep, mutations::FormMutation};
use protocol::Mutation;

#[semio_framework_async_macros::async_test]
async fn empty_diff_is_a_no_operation() {
    let base = FormsSnapshot::default();
    let diff = FormsDiff::default();
    assert_eq!(diff.apply(&base).expect("valid mutation diff"), base);
}

#[semio_framework_async_macros::async_test]
async fn create_step_diff_applies_onto_the_base_snapshot() {
    let base = crate::forms_snapshot_with_state(FORMS_DOCUMENT_SCHEMA.into(), "forms".into(), "1".into(), None, Vec::new());
    let step = FormStep { id: "s".into(), title: "Inputs".into(), description: None, blocks: Vec::new() };
    let operation = FormMutation::CreateStep(create_step::mutation::CreateStep { step, index: None });
    let diff: FormsDiff = operation.diff(&base).into_parts().0;
    assert_eq!(forms_steps(&diff.apply(&base).expect("valid mutation diff")).len(), 1);
}
