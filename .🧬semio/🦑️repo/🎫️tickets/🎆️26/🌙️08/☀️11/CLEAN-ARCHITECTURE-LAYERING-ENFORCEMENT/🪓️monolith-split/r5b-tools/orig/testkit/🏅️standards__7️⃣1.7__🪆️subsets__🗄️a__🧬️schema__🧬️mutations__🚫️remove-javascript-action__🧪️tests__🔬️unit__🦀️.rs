use super::*;
use crate::standards::v1_7::subsets::base::io::mutation_bridge::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn removes_the_matching_script_action() {
    let base = support::document_of(vec![support::action_object("JavaScript", "JS", "audit")]);
    let mutation = RemoveJavascriptAction { script: "audit".to_string() };
    let next = applied(&base, &PdfAMutation::RemoveJavascriptAction(mutation.clone()));
    assert!(support::action_with(&next, "JavaScript", "JS", "audit").is_none());
    assert_eq!(<RemoveJavascriptAction as MutationKind<PdfSnapshot, PdfAMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture").len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = applied(&support::document(), &PdfAMutation::InsertJavascriptAction(InsertJavascriptAction { script: "audit".to_string(), placements: Vec::new() }));
    assert_mutation_inverse_sum_law(&PdfAMutation::RemoveJavascriptAction(RemoveJavascriptAction { script: "audit".to_string() }), &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let base = support::with_tail(&applied(&support::document(), &PdfAMutation::InsertJavascriptAction(InsertJavascriptAction { script: "audit".to_string(), placements: Vec::new() })));
    assert_mutation_inverse_sum_law(&PdfAMutation::RemoveJavascriptAction(RemoveJavascriptAction { script: "audit".to_string() }), &base).await;
}
