use super::*;
use crate::editor::bim::unit_tests::support::{ctx, demo, run};

#[semio_framework_async_macros::async_test]
async fn a_move_without_a_ground_point_does_nothing() {
    let emit = run(&demo(), |doc, cfg| handle(&WorldPointerMove { pane: String::new(), position: Vec::new() }, doc, cfg, &mut ctx(&[]))).expect("nothing to do");
    assert!(emit.artifact_mutations.is_empty() && emit.effects.is_empty());
}
