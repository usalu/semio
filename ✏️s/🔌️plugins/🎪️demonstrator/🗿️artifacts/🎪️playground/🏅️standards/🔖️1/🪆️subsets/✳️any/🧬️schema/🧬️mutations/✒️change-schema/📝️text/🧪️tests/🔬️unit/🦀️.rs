
use super::*;

#[test]
fn text_wire_form_round_trips() {
    let operation = PlaygroundMutation::ChangeSchema(ChangeSchema { new_schema: "playground custom".into() });
    store::os_store::test_support::assert_op_line_round_trip(&operation);
}
