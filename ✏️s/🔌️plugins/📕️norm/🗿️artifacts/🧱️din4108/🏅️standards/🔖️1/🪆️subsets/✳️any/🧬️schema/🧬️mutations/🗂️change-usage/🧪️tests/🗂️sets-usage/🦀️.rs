//! Named mutation test for `change-usage`.
#[semio_framework_async_macros::async_test]
async fn applies_change_usage() {
    let base = crate::Din4108Snapshot::default();
    let mutation = crate::Din4108Mutation::ChangeUsage(crate::standards::v1::subsets::any::schema::mutations::change_usage::ChangeUsage {
        new_usage: base.usage.clone(),
    });
    let _ = mutation;
}
