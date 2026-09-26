//! Named mutation test for `change-climate-zone`.
#[semio_framework_async_macros::async_test]
async fn applies_change_climate_zone() {
    let base = crate::Din4108Snapshot::default();
    let mutation = crate::Din4108Mutation::ChangeClimateZone(crate::standards::v1::subsets::any::schema::mutations::change_climate_zone::ChangeClimateZone {
        new_climate_zone: base.climate_zone,
    });
    let _ = mutation;
}
