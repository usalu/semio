//! Named mutation test for `change-has-mechanical-ventilation`.
#[semio_framework_async_macros::async_test]
async fn applies_change_has_mechanical_ventilation() {
    let base = crate::Din4108Snapshot::default();
    let mutation = crate::Din4108Mutation::ChangeHasMechanicalVentilation(crate::standards::v1::subsets::any::schema::mutations::change_has_mechanical_ventilation::ChangeHasMechanicalVentilation {
        new_has_mechanical_ventilation: base.has_mechanical_ventilation,
    });
    let _ = mutation;
}
