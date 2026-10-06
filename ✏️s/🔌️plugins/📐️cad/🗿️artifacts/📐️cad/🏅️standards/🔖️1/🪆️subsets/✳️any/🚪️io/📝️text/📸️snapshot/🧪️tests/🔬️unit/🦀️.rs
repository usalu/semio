use crate::standards::v1::subsets::any::io::text::snapshot::*;
use crate::sample_scene_fixture::sample_scene;

/// 🧪️ The bundled composed document preserves its literal child fields through native Text.
#[semio_framework_async_macros::async_test]
async fn default_example_dsl_round_trips() {
    let document = parse_dsl(CAD_DEFAULT_EXAMPLE_TEXT).expect("parse default .cad example");
    store::os_store::test_support::assert_dsl_round_trip(&document);
}

#[semio_framework_async_macros::async_test]
async fn cad_scene_round_trips_through_dsl_document() {
    store::os_store::test_support::assert_dsl_round_trip(&sample_scene());
}
