use crate::standards::v1::subsets::any::io::text::mutations::change_schema::{apply_playground_mutation_json,undo_playground_mutation_json};
use crate::standards::v1::subsets::any::schema::mutations::{PlaygroundMutation, ChangeSchema};

#[test]
fn text_wire_form_round_trips() {
    let operation = PlaygroundMutation::ChangeSchema(ChangeSchema { new_schema: "playground custom".into() });
    store::os_store::test_support::assert_op_line_round_trip(&operation);
}
