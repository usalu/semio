use super::*;
use crate::{empty_plugin, sample_plugin};

#[semio_framework_async_macros::async_test]
async fn parse_and_print_round_trip_the_sample_program() {
    let program = sample_plugin();
    assert_eq!(parse(&print(&program)).expect("parse"), program);
}

#[semio_framework_async_macros::async_test]
async fn the_bundled_example_text_parses() {
    let parsed = parse(ARCHITECT_EXAMPLE_TEXT).expect("parse bundled example");
    assert_eq!(parsed.meta.title, sample_plugin().meta.title);
}

#[semio_framework_async_macros::async_test]
async fn an_empty_program_prints_and_reparses() {
    let program = empty_plugin();
    assert_eq!(parse(&print(&program)).expect("parse"), program);
}


#[test]
fn zz_dump_adjacency_fixtures() {
    use crate::schema::mutations::{apply_program_mutation_outcome, ProgramMutation};
    for (name, before_json, mutation_json) in [
        ("connect",
         include_str!("../../../../../🧫️fixtures/🧬️mutations/🧲️adjacency/🧲️connect/🧲️reception-waiting/📸️snapshot/⬅️before/🔣️.json"),
         include_str!("../../../../../🧫️fixtures/🧬️mutations/🧲️adjacency/🧲️connect/🧲️reception-waiting/🦠️mutation/🔣️.json")),
        ("disconnect",
         include_str!("../../../../../🧫️fixtures/🧬️mutations/🧲️adjacency/🫷️disconnect/🫷️reception-waiting/📸️snapshot/⬅️before/🔣️.json"),
         include_str!("../../../../../🧫️fixtures/🧬️mutations/🧲️adjacency/🫷️disconnect/🫷️reception-waiting/🦠️mutation/🔣️.json")),
    ] {
        let before: crate::ProgramSnapshot = dsl::json::from_json_str(before_json).expect("before decodes");
        let mutation: ProgramMutation = dsl::json::from_json_str(mutation_json).expect("mutation decodes");
        let outcome = <ProgramMutation as protocol::Mutation<crate::ProgramSnapshot>>::diff(&mutation, &before);
        let mut after = before.clone();
        let _ = apply_program_mutation_outcome(&mut after, &mutation);
        std::fs::write(format!("/tmp/adj-{name}-before.json"), dsl::json::to_json_string(&before)).expect("dump before");
        std::fs::write(format!("/tmp/adj-{name}-after.json"), dsl::json::to_json_string(&after)).expect("dump after");
        std::fs::write(format!("/tmp/adj-{name}-diff.json"), dsl::json::to_json_string(outcome.diff())).expect("dump diff");
    }
}
