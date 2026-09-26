//! Named mutation test for `change-airtightness-n50`.
#[semio_framework_async_macros::async_test]
async fn applies_change_airtightness_n50() {
    let base = crate::Din4108Snapshot::default();
    let mutation = crate::Din4108Mutation::ChangeAirtightnessN50(crate::standards::v1::subsets::any::schema::mutations::change_airtightness_n50::ChangeAirtightnessN50 {
        new_airtightness_n50: base.airtightness_n50,
    });
    let _ = mutation;
}
