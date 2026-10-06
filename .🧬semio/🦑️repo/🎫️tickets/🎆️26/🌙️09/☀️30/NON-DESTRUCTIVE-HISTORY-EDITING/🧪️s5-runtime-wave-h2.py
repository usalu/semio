"""🙋️ Wave H2 — fix-forward of wave H after the shared plugin law run of 16:24 (`📓️s5-plugin-law-run.md`).

Wave H gave EVERY constructed document store the actor `local`. The store retires a replaced actor string as a displaced
owner (`replace_local_actor_retained` → `take_string_replacement`), so the first verb of another actor — and the bind at
open — now left one displaced owner on a store that used to hold none: a side effect on every directly constructed app
(every law fixture), not only on opened instances. The identity belongs to the OPEN, not to construction:

- `PLG`: the constructor binds no actor; the genesis edits alone are authored as `LOCAL_ACTOR_ID`, explicitly (the state
  a genesis `Apply` left before H). `plugin_open_actor_instance` still binds the admitted actor (nobody → actor: nothing
  displaced). Everything else of H stands (one actor source, revert route, pure hydrate, const).
- Law split so the runtime half is proven without the store half:
  `an_opened_instance_acts_as_its_admitted_actor_across_reload_and_on_the_revert_route` (edits authored through the batched
  publication, which stamps its actor today) and `every_route_authors_as_its_acting_actor` (text ingest + plain emit:
  red until S5-STORE's §22.34 makes a plain `Apply` author as the store's actor). `publish_as` is the one helper that
  authors an edit as an actor; `bury_edit_of` uses it.

Loaded by `🧪️s5-runtime-land.py`; applies on top of wave H.
"""

OSM = "🧰️framework/🛍️products/💻️os/🔨️modules"
PLUGIN = f"{OSM}/🔌️plugin"
PLG = f"{PLUGIN}/🦀️.rs"
LAW = f"{PLUGIN}/🧪️tests/🧪️time-travel/🦀️.rs"

PLG_RS = [
    (
        """            store.set_local_actor_id(Some(LOCAL_ACTOR_ID.to_string())).expect("a fresh document store holds no durable group");
""",
        "",
    ),
    (
        """            if !genesis_mutations.is_empty() {
                store.dispatch(ArtifactCommand::Apply { mutations: genesis_mutations, transaction: None }).await.expect("ArtifactApp::genesis mutations must apply cleanly onto a freshly constructed store");
""",
        """            if !genesis_mutations.is_empty() {
                store.set_local_actor_id(Some(LOCAL_ACTOR_ID.to_string())).expect("a fresh document store holds no durable group");
                store.dispatch(ArtifactCommand::Apply { mutations: genesis_mutations, transaction: None }).await.expect("ArtifactApp::genesis mutations must apply cleanly onto a freshly constructed store");
""",
    ),
    (
        """    /// 🙋️ The actor an instance acts as until one is admitted for it ([`PluginApp::bind_actor`]): the author of an app's
    /// genesis edits and of everything an instance opened without an actor does. An instance's document store is never
    /// without an actor, so no route authors, undoes or reads `revertible` as a stranger (design §22.6).
""",
        """    /// 🙋️ The actor of whatever no admitted actor made: an app's genesis edits (identical on every replica) and the verbs
    /// of an instance that was opened without one (`instance_actor`). An opened instance acts as its admitted actor from
    /// its open on ([`PluginApp::bind_actor`]), so no route authors, undoes or reads `revertible` as a stranger (design
    /// §22.6).
""",
    ),
]

PUBLISH_OLD_HEAD = """/// ⚰️ One edit of `author` under `count` one-operation edits of the fixture actor ([`apply_other_edits`]): the history an
/// interior undo of `author` replays through. The author's edit takes the store's batched publication — the route a
/// tool's edit takes and the one that stamps the acting actor on the edit it mints (a plain `Apply`, the runtime's emit
/// route included, authors as `local`). Proves what it built: exactly one history row is `author`'s and it is not the
/// newest.
async fn bury_edit_of(app: &mut ToyApp, author: &str, count: i32) {
    let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4_096 };
"""
PUBLISH_OLD_TAIL = """    assert!(closed && publication.terminal_is_empty(), "the publication retires every owner");
    drop(publication);
    apply_other_edits(app, count).await;
"""
PUBLISH_NEW_HEAD = """/// 🪶️ One edit of `actor` holding `mutation`, through the store's batched publication — the route a tool's edit takes and
/// the one that stamps the acting actor on the edit it mints (a plain `Apply`, the runtime's emit route included, authors
/// as `local` until design §22.34); the store acts as `actor` afterwards.
fn publish_as(app: &mut ToyApp, actor: &str, mutation: TestMutation) {
    let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4_096 };
"""
PUBLISH_NEW_TAIL = """    assert!(closed && publication.terminal_is_empty(), "the publication retires every owner");
}

/// ⚰️ One edit of `author` ([`publish_as`]) under `count` one-operation edits of the fixture actor
/// ([`apply_other_edits`]): the history an interior undo of `author` replays through. Proves what it built: exactly one
/// history row is `author`'s and it is not the newest.
async fn bury_edit_of(app: &mut ToyApp, author: &str, count: i32) {
    publish_as(app, author, SetLabel { value: "buried".into() }.into());
    apply_other_edits(app, count).await;
"""

REGION = """//#region 🪪️ActorIdentity
/// 🫵️ The author of every edit of `app`'s document, oldest first.
fn edit_authors(app: &ToyApp) -> Vec<Option<String>> {
    app.store.envelope().vcs.edits.iter().map(|edit| edit.actor.clone()).collect()
}

/// ⚖️ LAW (design §22.6, runtime half of §22.34): an opened instance acts as its admitted actor — at its open, across a
/// reload and on the revert route.
/// - construction binds nobody (a store that never held an actor retires none); [`PluginApp::bind_actor`], the open, binds
///   the admitted one;
/// - a whole-document reload keeps the instance's actor although the loaded history ends in another author's edit, so an
///   undo there takes nothing back; a head-only pure hydrate keeps it too;
/// - the revert route acts as the reverting actor: the store acts as that actor afterwards and another actor's edit stays
///   applied; an undo takes back the acting actor's own edit.
#[semio_framework_async_macros::async_test]
async fn an_opened_instance_acts_as_its_admitted_actor_across_reload_and_on_the_revert_route() {
    let mut app = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest()).await;
    assert_eq!(app.store.local_actor_id(), None, "construction binds nobody");
    PluginApp::bind_actor(&mut app, "ada").await;
    assert_eq!(app.store.local_actor_id(), Some("ada"), "the admitted actor is the instance's");
    publish_as(&mut app, "ada", SetCount { value: 1 }.into());
    app.refresh_cache().await.expect("the log backfills the instance's own edit");
    publish_as(&mut app, "grace", SetCount { value: 2 }.into());
    app.refresh_cache().await.expect("the log backfills the other actor's edit");
    assert_eq!(edit_authors(&app), [Some("ada".to_string()), Some("grace".to_string())], "each edit names the actor that published it");
    let other = app.store.envelope().vcs.edits.last().map(|edit| edit.id.clone()).expect("the other actor's edit");

    let mut reader = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest()).await;
    PluginApp::bind_actor(&mut reader, "reader").await;
    artifact_app_laws::load_document(&mut reader, &app.document_pack().await.expect("document pack")).await.expect("pack reload");
    reader.refresh_cache().await.expect("the reloaded log backfills");
    assert_eq!((reader.store.local_actor_id(), head(&reader).0), (Some("reader"), 2), "a reload keeps the instance's actor although the history ends in another author's edit");
    history_verb(&mut reader, "reader", "undo", None).await;
    assert_eq!((head(&reader).0, reader.store.applied_edit_ids().len()), (2, 2), "an undo never takes back another author's edit");
    let pure_head = reader.store.snapshot().expect("the reloaded head").encode_pack();
    PluginApp::hydrate_pure_head(&mut reader, &pure_head).await.expect("the head-only lane hydrates");
    assert_eq!(reader.store.local_actor_id(), Some("reader"), "a head-only hydrate keeps the instance's actor");

    let own = app.history_patch(true).await.expect("history patch").upserts.into_iter().find(|row| row.author.as_deref() == Some("ada")).expect("the instance's own row").seq;
    history_verb(&mut app, "ada", "revertToCommand", Some(DslValue::object([("entrySeq".to_string(), DslValue::uint(own))]))).await;
    assert_eq!(app.store.local_actor_id(), Some("ada"), "the revert route acts as the reverting actor");
    assert!(app.store.applied_edit_ids().iter().any(|id| *id == other), "a revert never takes back another actor's edit");
    history_verb(&mut app, "grace", "undo", None).await;
    pump_until(&mut app, "the undo lands", |app| !app.store.local_step_pending()).await;
    assert!(!app.store.applied_edit_ids().iter().any(|id| *id == other), "an undo takes back the acting actor's own edit");
    close(&mut reader);
    close(&mut app);
}

/// ⚖️ LAW (design §22.34, the store half — red until a plain `Apply` authors as the store's actor): every route authors
/// as its acting actor. A route without an actor of its own (the text ingest) authors as the instance's admitted actor; a
/// route with one (the plain emit route) authors as it whatever the instance acted as before.
#[semio_framework_async_macros::async_test]
async fn every_route_authors_as_its_acting_actor() {
    let acting = |actor: &str| ActionMeta { view_state: Some(ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), ..artifact_app_laws::meta(actor) };
    let mut app = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest()).await;
    PluginApp::bind_actor(&mut app, "ada").await;
    let first: TestMutation = SetCount { value: 1 }.into();
    PluginApp::ingest_operations_text(&mut app, &::protocol::OpText::print_op(&first)).await.expect("the text ingest applies");
    let ingested = edit_authors(&app);
    app.dispatch_emit("setCount", Emit::<TestMutation, TestConfigMutation, NoDraftMutation>::mutations(vec![SetCount { value: 2 }.into()]), &acting("grace")).await.expect("the emit publishes");
    let emitted = edit_authors(&app);
    close(&mut app);
    assert_eq!(ingested, [Some("ada".to_string())], "a route without an actor of its own authors as the instance's");
    assert_eq!(emitted, [Some("ada".to_string()), Some("grace".to_string())], "the plain emit route authors as its acting actor");
}
//#endregion 🪪️ActorIdentity
"""


def law(text):
    for marker in (PUBLISH_OLD_HEAD, PUBLISH_OLD_TAIL, "//#region 🪪️ActorIdentity\n", "//#endregion 🪪️ActorIdentity\n"):
        if text.count(marker) != 1:
            raise SystemExit(f"wave h2: marker occurs {text.count(marker)} times, expected 1:\n{marker[:160]}")
    text = text.replace(PUBLISH_OLD_HEAD, PUBLISH_NEW_HEAD).replace(PUBLISH_OLD_TAIL, PUBLISH_NEW_TAIL)
    for old, new in [("author.into(), vec![SetLabel { value: \"buried\".into() }.into()], store::HistoryLane::Document", "actor.into(), vec![mutation], store::HistoryLane::Document"), ("the author's edit is admitted: {}", "the edit is admitted: {}"), ("\"the author's edit publishes\"", "\"the edit publishes\""), ("\"the author's edit is published and acknowledged\"", "\"the edit is published and acknowledged\"")]:
        if text.count(old) != 1:
            raise SystemExit(f"wave h2: publication anchor occurs {text.count(old)} times, expected 1:\n{old}")
        text = text.replace(old, new)
    start, end = text.index("//#region 🪪️ActorIdentity\n"), text.index("//#endregion 🪪️ActorIdentity\n") + len("//#endregion 🪪️ActorIdentity\n")
    return text[:start] + REGION + text[end:]


def files(_root):
    return {PLG: PLG_RS, LAW: law}
