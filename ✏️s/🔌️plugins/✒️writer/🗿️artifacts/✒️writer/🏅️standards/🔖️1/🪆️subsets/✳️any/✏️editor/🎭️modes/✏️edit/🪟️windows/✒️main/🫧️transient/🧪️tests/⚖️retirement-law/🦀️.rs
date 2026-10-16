//! ⚖️ Writer window transient owners retire under quoted grants: every transient mutation passes the typed-command input lane, and a transient edit prepares, inverts and re-prepares through the owner's own bundle.

#[global_allocator]
static HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

use semio_framework_test_typed_input_law as typed_input_law;
use semio_framework_test_transient_preparation_law as transient_preparation_law;

use semio_s_artifact_writer_writer::editor::writer::modes::edit::windows::main::transient::{
    SetEditorSelection, SetEngagementInput, SetLintGeneration, WriterEditorSelection, WriterMainWindowTransient, WriterMainWindowTransientMutation, WriterMainWindowTransientOwner,
};
use transient_preparation_law::transient_preparation_law;
use typed_input_law::{typed_command_retirement_law, typed_input_retirement_law};

fn selection() -> WriterEditorSelection {
    WriterEditorSelection { start: 3, end: 9, splice: 7 }
}

fn mutations() -> Vec<(&'static str, WriterMainWindowTransientMutation, WriterMainWindowTransient)> {
    vec![
        ("set-engagement-input", SetEngagementInput { value: "typing in the writer".into() }.into(), WriterMainWindowTransient { engagement_input: "typing in the writer".into(), ..Default::default() }),
        ("set-editor-selection", SetEditorSelection { selection: Some(selection()) }.into(), WriterMainWindowTransient { editor_selection: Some(selection()), ..Default::default() }),
        ("set-lint-generation", SetLintGeneration { value: 4 }.into(), WriterMainWindowTransient { lint_generation: 4, ..Default::default() }),
    ]
}

/// 🎟️ Every transient mutation and the transient state itself retire through the typed-command input lane.
#[test]
fn writer_transients_retire_through_the_typed_command_lane() {
    for (label, mutation, expected) in mutations() {
        let law = typed_command_retirement_law(label, mutation);
        let state = typed_input_retirement_law(label, expected);
        assert!(law.turns > 0 && state.turns > 0, "{label}: retiring costs paid turns");
    }
}

/// 🔁️ Each transient edit prepares its post state, prepares its inverse back to the base, prepares again, and retires through the owner's own bundle.
#[test]
fn writer_transient_edits_prepare_invert_and_retire_through_their_owner() {
    for (label, mutation, expected) in mutations() {
        transient_preparation_law::<WriterMainWindowTransientOwner>(label, WriterMainWindowTransient::default(), mutation, expected);
    }
}
