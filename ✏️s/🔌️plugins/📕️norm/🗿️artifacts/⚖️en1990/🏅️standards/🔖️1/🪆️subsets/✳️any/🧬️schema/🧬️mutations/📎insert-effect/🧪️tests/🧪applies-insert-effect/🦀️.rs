#[semio_framework_async_macros::async_test]
async fn applies_insert_effect() {
    let base = crate::En1990Snapshot::default();
    let mutation = crate::En1990Mutation::InsertEffect(crate::standards::v1::subsets::any::schema::mutations::insert_effect::InsertEffect {
        index: base.effects.len(), item: crate::MemberEffect { member_id: "beam-B1".into(), action_id: "G-sup".into(), influence: 0.0 },
    });
    let (after, _) = vcs::apply_mutation(&base, &mutation).expect("apply");
    assert_eq!(after.effects.len(), base.effects.len() + 1);
}
