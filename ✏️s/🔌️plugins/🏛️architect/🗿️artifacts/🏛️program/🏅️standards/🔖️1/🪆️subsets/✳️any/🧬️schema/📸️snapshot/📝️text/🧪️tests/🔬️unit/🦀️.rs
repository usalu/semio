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
