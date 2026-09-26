//! Named mutation test for `change-t-int-c`.
#[semio_framework_async_macros::async_test]
async fn applies_change_t_int_c() {
    let base = crate::Din4108Snapshot::default();
    let mutation = crate::Din4108Mutation::ChangeTIntC(crate::standards::v1::subsets::any::schema::mutations::change_t_int_c::ChangeTIntC {
        new_t_int_c: base.t_int_c,
    });
    let _ = mutation;
}
