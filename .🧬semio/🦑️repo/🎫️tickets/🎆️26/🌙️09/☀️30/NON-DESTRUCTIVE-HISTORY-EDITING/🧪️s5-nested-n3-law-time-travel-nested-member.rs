//! 🪆️ LAW (design §12 + §21.9): a mutation of a member OF A MEMBER is edited in history exactly like a document's own —
//! begin under the member's owner path, draft, accept, Report replay, finalize, overwrite — and the session names the
//! store by that owner path, never by a bare `(slot, child_id)` step.

use super::*;
use store::ArtifactPack as _;

/// 🌱️ The fixture roster's derivation: a member derives the `derived-leaf` child it declares, from its own content.
fn nested_toy_derivation(snapshot: &TestSnapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>, semio_framework_value::ValueError> {
    Ok((slot == "slot" && child_id == "derived-leaf").then(|| TestSnapshot { count: snapshot.count, label: "leaf".into(), slot: Vec::new() }.encode_pack()))
}

store::space_members! {
    pub enum NestedToyMembers, NestedToyMembersOpen {
        Child("s.test.child", "native", "*", "semio.test/v1") => (TestSnapshot, TestMutation, nested_toy_derivation),
    }
}

type NestedToyApp = VcsArtifactApp<ToyHistoryApp, NestedToyMembers>;

fn nested_dialect() -> ArtifactDialect {
    ArtifactDialect { artifact_kind: "s.test.child".into(), standard: "native".into(), subset: "*".into() }
}

fn nested_uri(id: &str) -> String {
    ArtifactRef { artifact_id: id.into(), dialect: nested_dialect() }.to_uri()
}

/// ✍️ One edit on the member at `key`, authored by the fixture actor and published the way a child-lane leaf is, with
/// the turn whose follow pass answers it; answers the edit's mutation id.
async fn nested_edit(app: &mut NestedToyApp, fixture: &Value, key: &MemberKey, mutation: TestMutation) -> String {
    let id = {
        let NestedToyMembers::Child(member) = &mut app.children.member_mut(key.borrowed()).expect("the member is live").member;
        member.set_local_actor_id(Some(text(&fixture["actor"]).to_string())).expect("the member takes the local actor");
        member.dispatch(ArtifactCommand::Apply { mutations: vec![mutation], transaction: None }).await.expect("the member edit applies");
        member.mutation_ops().expect("member mutation ops").last().expect("the edit holds one op").mutation_id.0.clone()
    };
    let generation = app.admit_child_content_publication().expect("admit the member's publication");
    app.publish_member_content(generation, key.borrowed()).await.expect("publish the member's content");
    app.advance_typed_operation_publication().await.expect("the turn follows the member's declaration");
    id
}

async fn nested_verb(app: &mut NestedToyApp, fixture: &Value, action: &str, args: Vec<(&str, DslValue)>) {
    let args = DslValue::Object(args.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
    let result = app.handle_action(action, Some(&args), &meta(fixture)).await.unwrap_or_else(|fault| panic!("{action}: {fault:?}"));
    assert_eq!(rejected(&result), None, "{action} is accepted on the nested member");
}

async fn nested_pump(app: &mut NestedToyApp, what: &str, done: impl Fn(&NestedToyApp) -> bool) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    while std::time::Instant::now() < deadline {
        if done(app) {
            return;
        }
        app.advance_typed_operation_publication().await.unwrap_or_else(|fault| panic!("{what}: driver turn faulted: {fault:?}"));
        while app.take_typed_operation_ui_progress().is_some() {}
    }
    panic!("{what} never settled; stage {:?}", app.time_travel.session().stage);
}

fn nested_head(app: &NestedToyApp, key: &MemberKey) -> (i32, String) {
    let NestedToyMembers::Child(member) = &app.children.member(key.borrowed()).expect("the member is live").member;
    let head = member.snapshot().expect("the member head folds");
    (head.count, head.label.clone())
}

/// ⚖️ LAW: the leaf a member derives is live under that member's owner edge; an edit of the leaf is opened in history by
/// the leaf's owner path, replays on the leaf's own store, and an overwrite leaves the leaf at the fresh fold of the edited
/// log while the document and the member that owns the leaf stay as they were.
#[semio_framework_async_macros::async_test]
async fn a_mutation_of_a_member_of_a_member_is_edited_in_history_under_its_owner_path() {
    let fixture = fixture();
    let mut app = artifact_app_laws::new_registered_app_with_members::<ToyHistoryApp, NestedToyMembers, _>(toy_manifest()).await;
    app.store.set_local_actor_id(Some(text(&fixture["actor"]).to_string())).expect("local actor");
    app.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetSlotChildren { children: vec![nested_uri("branch")] }.into()], transaction: None }).await.expect("the document declares its member");
    let member = NestedToyMembers::create("branch", &nested_dialect(), &TestSnapshot::default().encode_pack()).await.expect("the member store opens");
    app.register_child("slot", "branch", nested_dialect(), member).await.expect("the declared member registers");

    let branch = MemberKey::root("slot", "branch");
    nested_edit(&mut app, &fixture, &branch, SetSlotChildren { children: vec![nested_uri("derived-leaf")] }.into()).await;
    let leaf = MemberKey { owner: "branch".into(), slot: "slot".into(), child_id: "derived-leaf".into() };
    let path = "slot/branch/slot/derived-leaf";
    assert_eq!(app.children.path_of(leaf.borrowed()).map(|path| path.to_string()).as_deref(), Some(path), "the member-lane declaration opens the leaf that member derives");

    let target = nested_edit(&mut app, &fixture, &leaf, SetCount { value: 3 }.into()).await;
    nested_edit(&mut app, &fixture, &leaf, SetLabel { value: "downstream".into() }.into()).await;
    app.refresh_cache().await.expect("the history backfills the member edits");
    let (document, owner) = (app.store.snapshot().expect("document head"), nested_head(&app, &branch));

    nested_verb(&mut app, &fixture, "historyEditBegin", vec![("mutationId", DslValue::String(target)), ("store", DslValue::String(path.to_string()))]).await;
    assert_eq!(app.time_travel.member_store().as_deref(), Some(path), "the session names the store by the leaf's owner path");
    nested_verb(&mut app, &fixture, "historyEditInput", vec![("path", DslValue::String("/value".to_string())), ("value", dsl(&serde_json::json!(9)))]).await;
    nested_verb(&mut app, &fixture, "historyEditAccept", Vec::new()).await;
    nested_pump(&mut app, "the leaf replay completes", |app| app.time_travel.session().stage != TimeTravelStage::Replaying).await;
    nested_verb(&mut app, &fixture, "historyEditFinalize", Vec::new()).await;
    nested_verb(&mut app, &fixture, "historyEditCommit", vec![("choice", DslValue::String(semio_framework::HISTORY_EDIT_CHOICE_OVERWRITE.to_string()))]).await;
    nested_pump(&mut app, "the overwrite settles", |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work()).await;

    assert_eq!(nested_head(&app, &leaf), (9, "downstream".to_string()), "the leaf head is the fresh fold of its edited log");
    assert_eq!(nested_head(&app, &branch), owner, "the member that owns the leaf is untouched");
    assert_eq!(app.store.snapshot().expect("document head"), document, "the document is untouched");
    assert_eq!(app.child_content_root.typed_read_at::<TestSnapshot>(leaf.borrowed()).expect("the published leaf").count, 9, "the composed read follows the finalized leaf");

    artifact_app_laws::close_registered_fixture_app(&mut app);
    assert!(app.time_travel.terminal_is_empty(), "the history-edit ledger retires every owner on close");
}
