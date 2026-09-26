#[semio_framework_async_macros::async_test]
async fn applies_insert_member() {
    let base = crate::En1990Snapshot::default();
    let mutation = crate::En1990Mutation::InsertMember(crate::standards::v1::subsets::any::schema::mutations::insert_member::InsertMember {
        index: base.members.len(), item: crate::Member {
        id: "M-new".into(), label_en: "New member".into(), label_de: "Neues Bauteil".into(),
        rd_str: 0.0, rd_geo: 0.0, rd_equ_stab: 0.0, rd_equ_destab: 0.0, rd_fat: 0.0,
        span: 1.0, deflection_w: 0.0, deflection_limit_ratio: 250.0,
        vibration_frequency: 5.0, vibration_frequency_min: 3.0,
    },
    });
    let (after, _) = vcs::apply_mutation(&base, &mutation).expect("apply");
    assert_eq!(after.members.len(), base.members.len() + 1);
}
