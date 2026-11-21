// Registered snapshot replay test.
#[test]
fn original_archive_objects_restore_actual_snapshot_and_role_lane() {
    use crate::standards::v1_7::subsets::base::schema::stream_roles::{PdfAdmittedStreamRole, PdfGraphIdentity, PdfStreamRoleValue};
    use dsl::original_receiving::receive_to_completion;
    let (forward, inverse) = object_archive_fixtures();
    let forward = receive_to_completion::<PdfMutation>(&forward, (0, forward.len()), image_archive_receiving_stock()).unwrap();
    let inverse = receive_to_completion::<PdfMutation>(&inverse, (0, inverse.len()), image_archive_receiving_stock()).unwrap();
    let id = ObjRef { num: 2, r#gen: 0 };
    let mut base = PdfSnapshot::default();
    base.objects.push(PdfIndirectObject { id, value: PdfObject::Null });
    base.admitted_stream_roles.push(PdfAdmittedStreamRole {
        identity: PdfGraphIdentity { owner: id, path: Vec::new() },
        dependencies: Vec::new(),
        value: PdfStreamRoleValue::MetadataText { text: "Neutral restored role.".into() },
    });
    assert_eq!(semio_s_artifact_stdio_contract::mutation_inverse(&forward, &base).unwrap(), vec![inverse.clone()]);
    let mut state = base.clone();
    apply_pdf_mutation(&mut state, &forward);
    assert!(state.objects.iter().any(|object| object.id.num == 5 && object.value == PdfObject::Null));
    assert!(state.admitted_stream_roles.is_empty());
    apply_pdf_mutation(&mut state, &inverse);
    assert_eq!(state.objects, base.objects);
    assert_eq!(state.admitted_stream_roles, base.admitted_stream_roles);
}
