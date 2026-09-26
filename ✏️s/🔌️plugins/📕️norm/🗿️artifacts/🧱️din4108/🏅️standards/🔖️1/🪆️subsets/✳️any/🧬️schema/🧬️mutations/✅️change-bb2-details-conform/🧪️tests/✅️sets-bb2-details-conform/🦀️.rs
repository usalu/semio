//! Named mutation test for `change-bb2-details-conform`.
#[semio_framework_async_macros::async_test]
async fn applies_change_bb2_details_conform() {
    let base = crate::Din4108Snapshot::default();
    let mutation = crate::Din4108Mutation::ChangeBb2DetailsConform(crate::standards::v1::subsets::any::schema::mutations::change_bb2_details_conform::ChangeBb2DetailsConform {
        new_bb2_details_conform: base.bb2_details_conform,
    });
    let _ = mutation;
}
