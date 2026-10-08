use super::*;
use crate::forms_steps;
use crate::mutations::create_step;
use crate::{mutations::FormMutation, FormStep, FORMS_DOCUMENT_SCHEMA};
use protocol::Mutation;

#[semio_framework_async_macros::async_test]
async fn empty_diff_is_a_no_operation() {
    let base = FormsSnapshot::default();
    let diff = FormsDiff::default();
    assert_eq!(protocol::apply_diff(&diff, &base).expect("valid mutation diff"), base);
}

#[semio_framework_async_macros::async_test]
async fn create_step_diff_applies_onto_the_base_snapshot() {
    let base = crate::forms_snapshot_with_state(FORMS_DOCUMENT_SCHEMA.into(), "forms".into(), "1".into(), None, &[]);
    let step = FormStep { id: "s".into(), title: "Inputs".into(), description: None, blocks: Vec::new() };
    let operation = FormMutation::CreateStep(create_step::mutation::CreateStep { step, index: None });
    let diff: FormsDiff = operation.diff(&base).into_parts().0;
    assert_eq!(forms_steps(&protocol::apply_diff(&diff, &base).expect("valid mutation diff")).len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn positional_step_delta_inserts_moves_and_inverts_at_middle_rows() {
    let step = |id: &str| FormStep { id: id.into(), title: id.into(), description: None, blocks: Vec::new() };
    let snapshot = |ids: &[&str]| crate::forms_snapshot_with_state(FORMS_DOCUMENT_SCHEMA.into(), "forms".into(), "1".into(), None, &ids.iter().map(|id| step(id)).collect::<Vec<_>>());
    let order = |snapshot: &FormsSnapshot| forms_steps(snapshot).iter().map(|row| row.id.clone()).collect::<Vec<_>>();
    let diff = |delta: FormsStepsDelta| FormsDiff { steps: Some(delta), ..Default::default() };
    let base = snapshot(&["a", "b", "c"]);
    let insert = diff(FormsStepsDelta::insertion(1, step("x")));
    let inserted = protocol::apply_diff(&insert, &base).expect("valid insertion");
    let relocate = diff(FormsStepsDelta::relocation(&forms_steps(&inserted), 3, 0));
    let moved = protocol::apply_diff(&relocate, &inserted).expect("valid relocation");
    assert_eq!(order(&moved), ["c", "a", "x", "b"]);
    let mut sum = insert;
    sum.absorb(relocate);
    assert_eq!(protocol::apply_diff(&sum, &base).expect("valid sum"), moved);
    assert_eq!(protocol::apply_diff(&protocol::DiffAlgebra::inverse(&sum, &base), &moved).expect("valid inverse"), base);
}
