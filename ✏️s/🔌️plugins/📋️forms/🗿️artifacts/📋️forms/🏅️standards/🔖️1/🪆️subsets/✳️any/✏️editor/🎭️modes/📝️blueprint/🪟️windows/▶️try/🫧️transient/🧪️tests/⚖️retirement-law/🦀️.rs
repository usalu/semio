//! ⚖️ Forms Try window owners retire under quoted grants: the shared answer tree retires as a lease whose last alias pays the release, every Try mutation passes the typed-command input lane, and a Try edit prepares, inverts and re-prepares through the owner's own bundle.

#[global_allocator]
static HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

use semio_framework_test_typed_input_law as typed_input_law;
use semio_framework_test_transient_preparation_law as transient_preparation_law;

use semio_framework_value::{DslValue, FromValue};
use semio_s_artifact_forms_forms::editor::forms::modes::blueprint::windows::try_wizard::transient::{FormsTryValues, FormsTryWindowTransient, FormsTryWindowTransientMutation, FormsTryWindowTransientOwner};
use transient_preparation_law::transient_preparation_law;
use typed_input_law::{typed_command_retirement_law, typed_input_retirement_law};

fn values(entries: usize) -> FormsTryValues {
    let object = (0..entries).map(|index| (format!("question-{index}"), DslValue::Array(vec![DslValue::String(format!("answer {index} part one")), DslValue::String(format!("answer {index} part two"))]))).collect();
    FormsTryValues::from_value(DslValue::Object(object)).expect("the Forms Try values admit their own wire")
}

fn transient(entries: usize) -> FormsTryWindowTransient {
    FormsTryWindowTransient { try_values: values(entries) }
}

/// 🌳️ The answer tree retires exactly, and as a lease: an alias that is not the last one releases only its count.
#[test]
fn forms_try_values_retire_as_a_lease_of_their_shared_tree() {
    let empty = typed_input_retirement_law("forms try values, empty", FormsTryValues::default());
    let tree = values(3);
    let alias = tree.clone();
    let aliased = typed_input_retirement_law("forms try values with a live alias", alias);
    let sole = typed_input_retirement_law("forms try values, last alias", tree);
    assert!(sole.declared_releases > aliased.declared_releases, "only the last alias pays the tree release");
    assert!(aliased.declared_births < sole.declared_births, "an alias does not birth the deep tree cursors");
    assert!(empty.turns > 0, "even an empty tree costs a paid turn");
}

/// 🎟️ The Try transient and its replacement mutation retire through the typed-command input lane.
#[test]
fn forms_try_transients_retire_through_the_typed_command_lane() {
    let state = typed_input_retirement_law("forms try transient", transient(2));
    let command = typed_command_retirement_law("forms try snapshot", FormsTryWindowTransientMutation::Snapshot { transient: transient(2) });
    assert!(state.turns > 0 && command.turns > 0, "retiring costs paid turns");
}

/// 🔁️ A Try answer edit prepares its post state, prepares its inverse back to the base, prepares again, and retires through the owner's own bundle.
#[test]
fn a_forms_try_edit_prepares_inverts_and_retires_through_its_owner() {
    transient_preparation_law::<FormsTryWindowTransientOwner>("forms try answers", transient(1), FormsTryWindowTransientMutation::Snapshot { transient: transient(3) }, transient(3));
}
