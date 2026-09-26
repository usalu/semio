#[semio_framework_async_macros::async_test]
async fn applies_insert_variable() {
    let base = crate::En1990Snapshot::default();
    let mutation = crate::En1990Mutation::InsertVariable(crate::standards::v1::subsets::any::schema::mutations::insert_variable::InsertVariable {
        index: base.variables.len(), item: crate::VariableAction { id: "Q-new".into(), category: "office".into(), qk: 0.0},
    });
    let (after, _) = vcs::apply_mutation(&base, &mutation).expect("apply");
    assert_eq!(after.variables.len(), base.variables.len() + 1);
}
