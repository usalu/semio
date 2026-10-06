
#[semio_framework_async_macros::async_test]
async fn dispatch_group_borrowed_child_keeps_exact_sources_on_policy_refusal() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🫳️child-dispatch/🔣️.json")).expect("neutral borrowed child declaration");
    let parent_ref=crate::os_io::ArtifactRef{artifact_id:fixture["parent"].as_str().unwrap().into(),dialect:crate::os_io::ArtifactDialect{artifact_kind:"s.stdio.demoparent".into(),standard:"1".into(),subset:"*".into()}};
    let child_ref=crate::os_io::ArtifactRef{artifact_id:fixture["child"].as_str().unwrap().into(),dialect:crate::os_io::ArtifactDialect{artifact_kind:"s.stdio.demochild".into(),standard:"1".into(),subset:"*".into()}};
    let mut parent=ArtifactStore::new(create_document_envelope::<DemoSnapshot,ValidatedMutation>("demo/v1",&parent_ref.artifact_id,DemoSnapshot{n:Some(0)},None)).await;
    let mut child=ArtifactStore::new(create_document_envelope::<DemoSnapshot,ValidatedMutation>("demo/v1",&child_ref.artifact_id,DemoSnapshot{n:Some(0)},None)).await;
    let mut coordinator=CompositionCoordinator::new().await;
    coordinator.graph_mut().await.insert_owns(&parent_ref.artifact_id,fixture["slot"].as_str().unwrap(),&child_ref.artifact_id).await.unwrap();
    let source=vec![ValidatedMutation::SetN(ValidatedSetN{n:fixture["invalidN"].as_i64().unwrap() as i32}).encode_op().unwrap()];
    let expected=source[0].clone();
    let bytes=source[0].as_ptr();
    let cells=source.as_ptr();
    let schema=SchemaId(fixture["opSchema"].as_str().unwrap().into());
    let labels=[crate::LocalizedLabel::data(fixture["label"].as_str().unwrap().to_owned())];
    let dispatch=ChildDispatch::borrowed(child_ref,&source,&schema,&labels);
    assert_eq!(dispatch.ops.as_ptr(),cells);
    assert_eq!(dispatch.ops[0].as_ptr(),bytes);
    assert!(std::ptr::eq(dispatch.op_schema,&schema));
    assert_eq!(dispatch.labels.as_ptr(),labels.as_ptr());
    let mut children=[(&mut child,dispatch)];
    let result=coordinator.dispatch_group(&parent_ref,&mut parent,&mut children,Vec::new(),Vec::new(),GroupMeta::default()).await;
    assert!(matches!(result,Err(VcsError::Rejected{policy:crate::os_spr::MergePolicy::Normal,..})));
    assert_eq!(children[0].1.ops.as_ptr(),cells);
    assert_eq!(children[0].1.ops[0].as_ptr(),bytes);
    assert_eq!(children[0].1.ops[0],expected);
    drop(children);
    assert!(parent.envelope().vcs.edits.is_empty()&&child.envelope().vcs.edits.is_empty());
    assert_eq!(source[0].as_ptr(),bytes);
    assert_eq!(source[0],expected);
    println!("[DEBUG] exact child source, label and schema borrows survive genuine group policy refusal");
}
