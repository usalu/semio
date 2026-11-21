#[test]
fn archived_objects_restore_indirect_object_and_role_storage() {
    use standards::v1_7::subsets::base::schema::{
        object_state::{PdfObjectsDiff, PdfObjectAdded, apply_objects_diff},
        stream_roles::{PdfAdmittedStreamRole, PdfGraphIdentity, PdfStreamRoleValue},
    };
    use dsl::original_receiving::receive_to_completion;
    let (forward, inverse) = object_archive_fixtures();
    let insert = receive_to_completion::<PdfMutation>(&forward, (0, forward.len()), image_archive_receiving_stock()).unwrap();
    let remove = receive_to_completion::<PdfMutation>(&inverse, (0, inverse.len()), image_archive_receiving_stock()).unwrap();
    let (PdfMutation::InsertObject(insert), PdfMutation::RemoveObject(remove)) = (insert, remove) else { panic!("typed object pair"); };
    let owner = ObjRef { num: 2, r#gen: 0 };
    let prior_objects = vec![PdfIndirectObject { id: owner, value: PdfObject::Null }];
    let prior_roles = vec![PdfAdmittedStreamRole {
        identity: PdfGraphIdentity { owner, path: Vec::new() },
        dependencies: Vec::new(),
        value: PdfStreamRoleValue::MetadataText { text: "Neutral restored role.".into() },
    }];
    let forward_graph = PdfObjectsDiff { added: vec![PdfObjectAdded {
        index: insert.index.map_or(prior_objects.len(), |index| index.min(prior_objects.len())),
        id: insert.id, value: insert.value,
    }], ..Default::default() };
    let inverse_graph = PdfObjectsDiff { removed: vec![remove.id], ..Default::default() };
    let forward_roles = insert.admitted_stream_roles.unwrap();
    let inverse_roles = remove.admitted_stream_roles.unwrap();
    let objects = apply_objects_diff(&forward_graph, &prior_objects);
    let roles = forward_roles.apply(&prior_roles);
    assert_eq!(objects.len(), 2);
    assert!(objects.iter().any(|object| object.id == insert.id));
    assert!(roles.is_empty());
    assert_eq!(apply_objects_diff(&inverse_graph, &objects), prior_objects);
    assert_eq!(inverse_roles, forward_roles.inverse(&prior_roles));
    assert_eq!(inverse_roles.apply(&roles), prior_roles);
}
