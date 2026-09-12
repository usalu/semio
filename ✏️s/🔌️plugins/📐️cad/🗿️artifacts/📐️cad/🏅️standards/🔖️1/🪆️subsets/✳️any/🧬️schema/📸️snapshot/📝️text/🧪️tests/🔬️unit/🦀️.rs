use super::*;
use crate::sample_scene_fixture::sample_scene;

/// 🧪️ `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio` predates this wave's snapshot-schema
/// rewrite (`schema=`/`shapeModel=`/… lines replace the old `objects=`/`shapeGeometry=` shape)
/// and needs regenerating from a real `print_dsl` capture — flagged in the wave-3 report rather
/// than hand-transcribed (per `📌️important.md`'s "never hand-transcribe fixture bytes" rule).
#[semio_framework_async_macros::async_test]
#[ignore = "fixture predates the model/drawing composition rewrite; regenerate via print_dsl before re-enabling"]
async fn default_example_dsl_round_trips() {
    let document = parse_dsl(CAD_DEFAULT_EXAMPLE_TEXT).expect("parse default .cad example");
    store::os_store::test_support::assert_dsl_round_trip(&document);
}

#[semio_framework_async_macros::async_test]
async fn cad_scene_round_trips_through_dsl_document() {
    store::os_store::test_support::assert_dsl_round_trip(&sample_scene());
}
