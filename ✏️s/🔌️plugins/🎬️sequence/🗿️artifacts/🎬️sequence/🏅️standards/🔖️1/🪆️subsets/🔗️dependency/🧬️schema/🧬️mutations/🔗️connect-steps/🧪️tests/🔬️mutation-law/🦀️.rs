use super::*;
use crate::default_snapshot;
use protocol::{
    os_spr::protocol_laws::{assert_fatal_never_applies, assert_missing_target_is_error},
    Mutation,
};

#[semio_framework_async_macros::async_test]
async fn connect_family_missing_target_is_error() {
    let base = default_snapshot();
    assert_missing_target_is_error(&base, &connect_steps("edge-99".into(), "missing".into(), "step-2".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn connect_family_fatal_never_applies() {
    let base = default_snapshot();
    let outcome = connect_steps("edge-99".into(), "step-1".into(), "step-1".into()).diff(&base);
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    assert_fatal_never_applies(&outcome).await;
}
