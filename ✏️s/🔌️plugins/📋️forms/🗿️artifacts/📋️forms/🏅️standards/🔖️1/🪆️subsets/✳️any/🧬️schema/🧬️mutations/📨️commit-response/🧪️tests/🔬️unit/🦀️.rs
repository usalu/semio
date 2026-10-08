use crate::mutations::FormMutation;
use crate::schema::response::FormsResponse;
use crate::{forms_snapshot_with_state, FormsSnapshot, FORMS_DOCUMENT_SCHEMA};

fn response(id: &str) -> FormsResponse {
    FormsResponse { id: id.into(), submitted_at: 1, definition_version: "1".into(), answers: Vec::new() }
}

fn base() -> FormsSnapshot {
    let mut base = forms_snapshot_with_state(FORMS_DOCUMENT_SCHEMA.into(), "forms".into(), "1".into(), None, &[]);
    base.responses = vec![response("r1"), response("r2"), response("r3")];
    base.results = crate::forms_results_child(&base.responses);
    base
}

/// ⚖️ Appending and inserting in the middle both invert to a discard whose diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    for index in [None, Some(0), Some(1)] {
        let mutation = FormMutation::CommitResponse(super::mutation::CommitResponse { response: response("r9"), index });
        protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base(), &mutation).await;
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base()).await;
    }
}
