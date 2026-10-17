use super::*;
use crate::standards::v1_7::subsets::base::io::mutation_bridge::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn inserts_the_script_action() {
    let base = PdfSnapshot::default();
    let mutation = InsertJavascriptAction { script: "app.alert('audit');".to_string(), placements: Vec::new() };
    let next = applied(&base, &PdfHMutation::InsertJavascriptAction(mutation.clone()));
    assert!(support::action_with(&next, "JavaScript", "JS", &mutation.script).is_some());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = support::document();
    assert_mutation_inverse_sum_law(&PdfHMutation::InsertJavascriptAction(InsertJavascriptAction { script: "app.alert('audit');".to_string(), placements: Vec::new() }), &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserts_at_the_placements_it_is_given() {
    let base = support::with_tail(&support::document());
    let mutation = PdfHMutation::InsertJavascriptAction(InsertJavascriptAction { script: "app.alert('audit');".to_string(), placements: vec![support::placed(98, 1), support::placed(99, 2)] });
    let next = applied(&base, &mutation);
    assert_eq!(next.objects[1].id.num, 98);
    assert_mutation_inverse_sum_law(&mutation, &base).await;
}
