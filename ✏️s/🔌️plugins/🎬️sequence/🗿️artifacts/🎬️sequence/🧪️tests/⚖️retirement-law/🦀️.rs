//! ⚖️ Sequence step parameters retire under quoted grants: a dictionary-valued step parameter set passes the typed-command input lane and retires its keys and values exactly.

#[global_allocator]
static HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

use semio_framework_test_typed_input_law as typed_input_law;

use neural_engine::{Atom, Dictionary, Value};
use semio_s_artifact_sequence_sequence::StepParams;
use typed_input_law::typed_input_retirement_law;

fn params() -> StepParams {
    let nested = Dictionary::new().insert("inner", Value::Atom(Atom::String("nested value".into())));
    StepParams(Dictionary::new().insert("name", Value::Atom(Atom::String("a step name".into()))).insert("count", Value::Atom(Atom::Integer(3))).insert("nested", Value::Dictionary(nested)))
}

/// 🧊️ Step parameters retire exactly: the dictionary owner is controlled, so the typed lane admits it instead of refusing it.
#[test]
fn step_params_retire_through_the_typed_command_lane() {
    let empty = typed_input_retirement_law("sequence step params, empty", StepParams::default());
    let populated = typed_input_retirement_law("sequence step params, populated", params());
    assert!(populated.declared_births > empty.declared_births, "keys and nested values cost paid births");
}
