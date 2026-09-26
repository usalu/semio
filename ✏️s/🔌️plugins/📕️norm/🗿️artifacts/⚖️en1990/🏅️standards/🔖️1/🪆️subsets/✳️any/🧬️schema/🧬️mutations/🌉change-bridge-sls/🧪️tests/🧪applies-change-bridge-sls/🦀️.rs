#[semio_framework_async_macros::async_test]
async fn applies_change_bridge_sls() {
    let base = crate::En1990Snapshot::default();
    let item = vec![crate::BridgeSls {
        id: "sls-1".into(),
        member_id: "beam-B1".into(),
        deck_acceleration: 0.1,
        deck_acceleration_limit: 0.5,
        deck_twist: 0.0,
        deck_twist_limit: 0.001,
        bridge_deflection: 0.01,
        bridge_deflection_limit: 0.05,
    }];
    let mutation = crate::En1990Mutation::ChangeBridgeSls(crate::standards::v1::subsets::any::schema::mutations::change_bridge_sls::ChangeBridgeSls {
        new_bridge_sls: item.clone(),
    });
    let (after, _) = vcs::apply_mutation(&base, &mutation).expect("apply");
    assert_eq!(after.bridge_sls, item);
}
