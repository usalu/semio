#[semio_framework_async_macros::async_test]
async fn applies_change_altitude_m() {
    let base = crate::En1990Snapshot::default();
    let mutation = crate::En1990Mutation::ChangeAltitudeM(crate::standards::v1::subsets::any::schema::mutations::change_altitude_m::ChangeAltitudeM {
        new_altitude_m: 1200.0,
    });
    let (after, _) = vcs::apply_mutation(&base, &mutation).expect("apply");
    assert!((after.altitude_m - 1200.0).abs() < 1e-12);
}
