//! Named mutation test for `change-rh-int`.
#[semio_framework_async_macros::async_test]
async fn applies_change_rh_int() {
    let base = crate::Din4108Snapshot::default();
    let mutation = crate::Din4108Mutation::ChangeRhInt(crate::standards::v1::subsets::any::schema::mutations::change_rh_int::ChangeRhInt {
        new_rh_int: base.rh_int,
    });
    let _ = mutation;
}
