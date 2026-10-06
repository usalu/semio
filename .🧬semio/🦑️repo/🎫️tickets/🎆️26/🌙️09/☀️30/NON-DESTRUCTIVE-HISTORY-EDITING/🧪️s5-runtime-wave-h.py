"""🙋️ Wave H (design §22.6, runtime half of S5-STORE's widened P3; coordinator 07:27 + 09:36): an instance always acts as
someone — on boot, across a reload and on every route — so no edit is authored by a stranger.

What was missing (measured 05:56: an emit as actor `author` produced a row authored `local`):
- nothing gave an instance its actor at open: the document store acted as nobody until the first verb, and after a load as
  the author of the loaded history's newest edit (`deliver_base_moved` answered nothing, `revertible` read another author's
  rows, the routes without an `ActionMeta` — text ingest, media import, transaction undo/redo, conflict resolution —
  authored as whoever wrote last);
- the genesis edits of an app were applied on a store without an actor;
- the revert route read and undid as whoever wrote last, not as the reverting actor;
- a head-only pure hydrate replaced the store and lost the actor.

`PLG`:
- `LOCAL_ACTOR_ID` — the actor an instance acts as until one is admitted (the value `instance_actor` already fell back to).
- the constructor gives the fresh document store that actor before genesis;
- `PluginApp::bind_actor` (default: nothing) — `plugin_open_actor_instance` binds the admitted actor right after the
  instance id; the whole-document load already carries the replaced store's actor into the adopted one;
- `begin_framework_revert_route` acts as `meta.actor`; `hydrate_pure_head` carries the actor across its reset;
- ONE actor source (coordinator decision 10:0x): `plugin_handle_action` and `plugin_handle_command` act as the actor admitted
  for the instance (`instance_actor`) like every other entry point; the `context.actor` reading with its literal `local`
  fallback is gone.

Law `an_instance_always_acts_as_someone_and_every_edit_names_its_actor` — its emit and ingest clauses are green only with
the store half (a plain `Apply` authors as the store's actor instead of `local`): LAND TOGETHER with S5-STORE's P3.

Loaded by `🧪️s5-runtime-land.py`.
"""

OSM = "🧰️framework/🛍️products/💻️os/🔨️modules"
PLUGIN = f"{OSM}/🔌️plugin"
PLG = f"{PLUGIN}/🦀️.rs"
LAW = f"{PLUGIN}/🧪️tests/🧪️time-travel/🦀️.rs"

PLG_RS = [
    (
        """    pub const HISTORY_UNIT_SPANS_DOCUMENTS_CODE: &str = "history.unit-spans-documents";
""",
        """    pub const HISTORY_UNIT_SPANS_DOCUMENTS_CODE: &str = "history.unit-spans-documents";

    /// 🙋️ The actor an instance acts as until one is admitted for it ([`PluginApp::bind_actor`]): the author of an app's
    /// genesis edits and of everything an instance opened without an actor does. An instance's document store is never
    /// without an actor, so no route authors, undoes or reads `revertible` as a stranger (design §22.6).
    pub const LOCAL_ACTOR_ID: &str = "local";
""",
    ),
    (
        """        /// 🪪️ Binds the host's live instance identity before any operation-scoped work starts.
        async fn bind_instance_id(&mut self, _instance_id: u32) {}
""",
        """        /// 🪪️ Binds the host's live instance identity before any operation-scoped work starts.
        async fn bind_instance_id(&mut self, _instance_id: u32) {}
        /// 🧍️ Binds the actor admitted for this instance at its open: who it acts as from then on — across a reload too —
        /// on every route that carries no actor of its own.
        async fn bind_actor(&mut self, _actor: &str) {}
""",
    ),
    (
        """        async fn bind_instance_id(&mut self, instance_id: u32) {
            self.live_runtime_instance_id = Some(instance_id);
        }
""",
        """        async fn bind_instance_id(&mut self, instance_id: u32) {
            self.live_runtime_instance_id = Some(instance_id);
        }

        async fn bind_actor(&mut self, actor: &str) {
            self.store.set_local_actor_id(Some(actor.to_string())).expect("an opening instance holds no durable group");
        }
""",
    ),
    (
        """            if let Some(owners) = A::build_document_store_owners() {
                store.install_document_store_owners_exact(owners);
            }
""",
        """            if let Some(owners) = A::build_document_store_owners() {
                store.install_document_store_owners_exact(owners);
            }
            store.set_local_actor_id(Some(LOCAL_ACTOR_ID.to_string())).expect("a fresh document store holds no durable group");
""",
    ),
    (
        """        let mut app = program.create_app(app_id).ok_or_else(|| plugin_internal_fault("unknown app"))?;
        ::semio_framework_async::poll::resolve_ready(app.bind_instance_id(request.instance_id));
""",
        """        let mut app = program.create_app(app_id).ok_or_else(|| plugin_internal_fault("unknown app"))?;
        ::semio_framework_async::poll::resolve_ready(app.bind_instance_id(request.instance_id));
        ::semio_framework_async::poll::resolve_ready(app.bind_actor(&actor.to_string()));
""",
    ),
    (
        """.unwrap_or_else(|| "local".to_string())""",
        """.unwrap_or_else(|| crate::app::LOCAL_ACTOR_ID.to_string())""",
    ),
    (
        """        let actor = context.get("actor").and_then(|value| value.as_str()).unwrap_or("local").to_string();
        let owner_matches = """,
        """        let actor = instance_actor(runtime, instance_id).await;
        let owner_matches = """,
    ),
    (
        """        let actor = context.get("actor").and_then(|value| value.as_str()).unwrap_or("local").to_string();
        let view_state = context.get("viewState")""",
        """        let actor = instance_actor(runtime, instance_id).await;
        let view_state = context.get("viewState")""",
    ),
    (
        """            let action = REVERT_TO_COMMAND_ACTION_ID;
            self.refresh_cache().await?;
""",
        """            let action = REVERT_TO_COMMAND_ACTION_ID;
            self.store.set_local_actor_id(Some(meta.actor.clone())).map_err(|error| error.into_fault())?;
            self.refresh_cache().await?;
""",
    ),
    (
        """            self.store.reset(loaded_with_app_dialect::<A>(envelope)).await.map_err(|error| error.into_fault())?;
            self.commit_document_window_reset(window_reset);
""",
        """            let actor = self.store.local_actor_id().map(str::to_string);
            self.store.reset(loaded_with_app_dialect::<A>(envelope)).await.map_err(|error| error.into_fault())?;
            self.commit_document_window_reset(window_reset);
            self.store.set_local_actor_id(actor).map_err(|error| error.into_fault())?;
""",
    ),
]

LAW_RS = [
    (
        """//#region 🎚️HistoryFilter
""",
        """//#region 🪪️ActorIdentity
/// 🫵️ The author of every edit of `app`'s document, oldest first.
fn edit_authors(app: &ToyApp) -> Vec<Option<String>> {
    app.store.envelope().vcs.edits.iter().map(|edit| edit.actor.clone()).collect()
}

/// ⚖️ LAW (design §22.6, P3 widened): an instance always acts as someone and every edit names the actor that made it.
/// - boot: a constructed instance acts as [`LOCAL_ACTOR_ID`] until an actor is admitted; [`PluginApp::bind_actor`] is that
///   admission;
/// - a route without an actor of its own (the text ingest) authors as the instance's actor;
/// - a route with one authors as it whatever the instance acted as before (the plain emit route);
/// - a whole-document reload keeps the instance's actor although the loaded history ends in another author's edit, so an
///   undo there takes nothing back; a head-only pure hydrate keeps it too;
/// - the revert route acts as the reverting actor: it never takes back another actor's edit, and the store acts as that
///   actor afterwards.
#[semio_framework_async_macros::async_test]
async fn an_instance_always_acts_as_someone_and_every_edit_names_its_actor() {
    let acting = |actor: &str| ActionMeta { view_state: Some(ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), ..artifact_app_laws::meta(actor) };
    let mut app = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest()).await;
    assert_eq!(app.store.local_actor_id(), Some(LOCAL_ACTOR_ID), "a constructed instance acts as the local actor");
    PluginApp::bind_actor(&mut app, "ada").await;
    assert_eq!(app.store.local_actor_id(), Some("ada"), "the admitted actor is the instance's");
    let first: TestMutation = SetCount { value: 1 }.into();
    PluginApp::ingest_operations_text(&mut app, &::protocol::OpText::print_op(&first)).await.expect("the text ingest applies");
    assert_eq!(edit_authors(&app), [Some("ada".to_string())], "a route without an actor of its own authors as the instance's");
    app.refresh_cache().await.expect("the log backfills the ingest");
    app.dispatch_emit("setCount", Emit::<TestMutation, TestConfigMutation, NoDraftMutation>::mutations(vec![SetCount { value: 2 }.into()]), &acting("grace")).await.expect("the emit publishes");
    assert_eq!(edit_authors(&app), [Some("ada".to_string()), Some("grace".to_string())], "the plain emit route authors as its acting actor");
    app.refresh_cache().await.expect("the log backfills the emit");
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
//#endregion 🪪️ActorIdentity

//#region 🎚️HistoryFilter
""",
    ),
]


def files(_root):
    return {PLG: PLG_RS, LAW: LAW_RS}
