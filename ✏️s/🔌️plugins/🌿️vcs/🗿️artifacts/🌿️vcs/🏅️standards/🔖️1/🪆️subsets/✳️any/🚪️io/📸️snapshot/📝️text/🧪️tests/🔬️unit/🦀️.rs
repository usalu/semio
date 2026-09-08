
use super::*;

#[semio_framework_async_macros::async_test]
async fn vcs_demo_projection_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&crate::standards::v1::subsets::any::schema::empty_vcs_snapshot());
}

#[semio_framework_async_macros::async_test]
async fn default_example_dsl_round_trips() {
    let document = parse_dsl(VCS_DEMO_DEFAULT_EXAMPLE_TEXT).expect("parse default .vcsdemo example");
    store::os_store::test_support::assert_dsl_round_trip(&document);
}
