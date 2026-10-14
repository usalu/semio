extern crate semio_framework_os_kernel as protocol;
#[path = "../../../../🧪️tests/🧰️support/🦀️.rs"]
mod context;
use crate::context::{app_with_registry, dispatch, select_face};
use semio_s_artifact_lowpoly_lowpoly_editor::LowpolyCommand;

#[semio_framework_async_macros::async_test]
async fn engagement_submit_resolves_a_typed_token_into_a_real_command() {
    let mut a = app_with_registry().await;
    let object_id = a.snapshot().expect("projection").objects[0].id.clone();
    select_face(&mut a, &object_id, 0).await;
    let before = a.snapshot().expect("projection").objects[0].mesh.clone();
    dispatch(&mut a, LowpolyCommand::EngagementSubmit(semio_s_artifact_lowpoly_lowpoly_commands::engagement::engagement_submit::EngagementSubmit { value: Some("extrude".into()) })).await;
    assert_ne!(a.snapshot().expect("projection").objects[0].mesh, before, "typed 'extrude' must run the extrude command");
}

#[semio_framework_async_macros::async_test]
async fn engagement_submit_ignores_unresolvable_input() {
    let mut a = app_with_registry().await;
    let result = dispatch(&mut a, LowpolyCommand::EngagementSubmit(semio_s_artifact_lowpoly_lowpoly_commands::engagement::engagement_submit::EngagementSubmit { value: Some("bogus".into()) })).await;
    assert!(result.mutations.is_empty());
}
