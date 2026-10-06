//#region 🪆️NestedCompositionLaws
/// 🧬️ The reference of one fixture member — every level composes the same child dialect.
fn nested_reference(id: &str) -> ArtifactRef {
    ArtifactRef { artifact_id: id.into(), dialect: ArtifactDialect { artifact_kind: "s.test.child".into(), standard: "native".into(), subset: "*".into() } }
}

/// 🌱️ The derivation the fixture document and the fixture member roster share (design §21.9): a `derived-branch-<n>` child
/// declares and derives its own `derived-leaf-<n>` child, so one declaration on the document reaches two levels down, and
/// a member that re-points its declaration at another `derived-leaf-<m>` derives that one.
fn test_nested_derivation(_snapshot: &TestSnapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>, semio_framework_value::ValueError> {
    if slot != "slot" {
        return Ok(None);
    }
    if let Some(name) = child_id.strip_prefix("derived-branch-") {
        let leaf = nested_reference(&format!("derived-leaf-{name}"));
        return Ok(Some(TestSnapshot { count: 0, label: format!("branch {name}"), slot: vec![store::ArtifactChild::new(leaf.artifact_id.clone(), leaf)] }.encode_pack()));
    }
    Ok(child_id.strip_prefix("derived-leaf-").map(|name| TestSnapshot { count: 0, label: format!("leaf {name}"), slot: Vec::new() }.encode_pack()))
}

/// 🧭️ Every live member as `(owner path, artifact id of its owner)`, sorted.
fn nested_members(app: &VcsArtifactApp<TestApp, TestMembers>) -> Vec<(String, String)> {
    let mut members: Vec<(String, String)> = app.children.keyed_entries().map(|(key, entry)| (app.children.path_of(key).expect("a live member has an owner path").to_string(), entry.owner.parent.artifact_id.clone())).collect();
    members.sort();
    members
}

/// ✍️ One edit that re-points the children `owner` declares — the document's parent lane (`None`) or one member's lane,
/// published the way a child-lane leaf is — and the turn whose follow pass answers it.
async fn nested_declare(app: &mut VcsArtifactApp<TestApp, TestMembers>, owner: Option<&MemberKey>, children: &[&str]) {
    let mutation = TestMutation::SetSlotChildren(SetSlotChildren { children: children.iter().map(|id| nested_reference(id).to_uri()).collect() });
    match owner {
        None => {
            app.test_store_mut().await.dispatch(store::ArtifactCommand::Apply { mutations: vec![mutation], transaction: None }).await.expect("the document re-points its declaration");
        }
        Some(owner) => {
            {
                let TestMembers::Child(member) = &mut app.children.member_mut(owner.borrowed()).expect("the owning member is live").member;
                member.dispatch(store::ArtifactCommand::Apply { mutations: vec![mutation], transaction: None }).await.expect("the member re-points its declaration");
            }
            let generation = app.admit_child_content_publication().expect("admit the member's publication");
            app.publish_member_content(generation, owner.borrowed()).await.expect("publish the member's new content");
        }
    }
    PluginApp::advance_typed_operation_publication(app).await.expect("the follow pass runs with the turn");
}

/// 📥️ Loads `archive` into a fresh fixture document through the stepped archive load and answers it once it is ready.
async fn nested_loaded(operation: u64, archive: protocol::DocumentArchivePack) -> VcsArtifactApp<TestApp, TestMembers> {
    let mut app = contract_composed_app_raw().await;
    PluginApp::begin_document_archive_load(&mut app, operation, archive).expect("the runtime admits the archive");
    let status = loop {
        let status = PluginApp::poll_document_archive_load(&mut app, operation).await.expect("poll");
        if !matches!(status.state, protocol::DocumentArchiveLoadState::Pending | protocol::DocumentArchiveLoadState::Running) {
            break status;
        }
    };
    PluginApp::acknowledge_document_archive_load(&mut app, operation).expect("acknowledge");
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "{:?}", semio_framework_diagnostic::decode_fault_bytes(&status.fault));
    app
}

/// 🪆️ LAW (design §21.9): composition is recursive. One declaration on the document opens the member it derives AND the
/// member that member derives, each owner-stamped to the member that derives it and keyed by its owner edge; a member-lane
/// edit re-mints the child that member derives; a re-pointed member leaves with every member it owns; the archive carries
/// the whole closure and a fresh document loads it; a document loaded WITHOUT its members derives them again at every depth.
#[semio_framework_async_macros::async_test]
async fn a_member_composes_and_derives_children_like_a_document_at_every_depth() {
    let mut app = contract_composed_app_raw().await;
    let document = app.store.envelope().id.clone();

    nested_declare(&mut app, None, &["derived-branch-1"]).await;
    assert_eq!(
        nested_members(&app),
        vec![("slot/derived-branch-1".to_string(), document.clone()), ("slot/derived-branch-1/slot/derived-leaf-1".to_string(), "derived-branch-1".to_string())],
        "one declaration on the document opens the member it derives and the member that member derives"
    );
    let leaf = MemberKey { owner: "derived-branch-1".into(), slot: "slot".into(), child_id: "derived-leaf-1".into() };
    assert_eq!(app.child_content_root.typed_read_at::<TestSnapshot>(leaf.borrowed()).expect("the leaf reads under its owner").label, "leaf 1");
    assert!(app.child_content_root.typed_read::<TestSnapshot>("slot", "derived-leaf-1").is_err(), "a member of a member is no member of the document");
    assert_eq!(app.children.member(leaf.borrowed()).expect("the leaf is live").member.owner_ref().map(|owner| owner.parent), Some(nested_reference("derived-branch-1")), "the member is the leaf's `owner.parent`");
    assert_eq!(app.composition.graph_mut().await.owner_of("derived-leaf-1").await, Some("derived-branch-1"));
    assert_eq!(app.children.resolve(&MemberPath::root("slot", "derived-branch-1").join("slot", "derived-leaf-1")).map(|(key, _)| key), Some(leaf));

    let branch = MemberKey::root("slot", "derived-branch-1");
    nested_declare(&mut app, Some(&branch), &["derived-leaf-2"]).await;
    assert_eq!(
        nested_members(&app),
        vec![("slot/derived-branch-1".to_string(), document.clone()), ("slot/derived-branch-1/slot/derived-leaf-2".to_string(), "derived-branch-1".to_string())],
        "a member-lane edit re-mints the child that member derives and retires the one it no longer names"
    );

    nested_declare(&mut app, None, &["derived-branch-3"]).await;
    let closure = vec![("slot/derived-branch-3".to_string(), document.clone()), ("slot/derived-branch-3/slot/derived-leaf-3".to_string(), "derived-branch-3".to_string())];
    assert_eq!(nested_members(&app), closure, "a re-pointed member leaves with every member it owns, and its successor arrives with its own");
    assert!(app.composition.graph_mut().await.owner_of("derived-leaf-2").await.is_none(), "a retired member leaves the ownership graph");

    let archive = PluginApp::document_archive(&app).await.expect("the composed document archives its whole closure");
    let mut archived: Vec<(String, String)> = archive.members.iter().map(|entry| (entry.owner.parent.artifact_id.clone(), entry.reference.artifact_id.clone())).collect();
    archived.sort();
    let mut expected = vec![(document.clone(), "derived-branch-3".to_string()), ("derived-branch-3".to_string(), "derived-leaf-3".to_string())];
    expected.sort();
    assert_eq!(archived, expected, "every archived member names the member that owns it");

    let mut reloaded = nested_loaded(11, archive.clone()).await;
    assert_eq!(nested_members(&reloaded), closure, "save → fresh load restores the closure at every depth");
    assert_eq!(PluginApp::document_archive(&reloaded).await.expect("the reloaded document archives again").members.len(), 2);

    let mut bare = archive;
    bare.members.clear();
    let mut derived = nested_loaded(12, bare).await;
    assert_eq!(nested_members(&derived), closure, "a document loaded without its members derives them again at every depth");

    drain_and_close_composed_fixture(&mut derived);
    drain_and_close_composed_fixture(&mut reloaded);
    drain_and_close_composed_fixture(&mut app);
}
//#endregion 🪆️NestedCompositionLaws
