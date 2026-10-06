"""⚰️ Wave F (tests only; coordinator 05:47 "RB ON DISK" + the two N17 laws that never ran green since session 4):

- N17 harness fault: a mutation dispatched straight on the store names no author, and the store authors it as `local`
  (`replay_mutations`: `author_id().unwrap_or("local")`, and the `Apply` takes the store's local actor from it) — so
  `set_local_actor_id("other")` + `apply_other_edits` never made "600 edits of another author"; the undo of the fixture
  actor hit the applied TAIL, which the store adopts inside the dispatch (tail step, design §20.14 D22), and nothing
  waited. `bury_edit_of` authors the acting author's ONE edit through the runtime (the batched publication stamps
  `meta.actor`) and puts the 600 one-operation edits of the fixture actor downstream; both laws undo as that author.
- RB clause: a replay finished before a backbone attached and detached still commits (store wave RB: `commit_finished_replay`
  refuses `Stale` on the content revision alone).

Loaded by `🧪️s5-runtime-land.py`; applies on top of wave E.
"""

OSM = "🧰️framework/🛍️products/💻️os/🔨️modules"
LAW = f"{OSM}/🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs"

LAW_RS = [
    (
        """/// ✏️ `count` one-operation edits on `app`'s store as its current local actor — a long downstream history a replay steps through
/// edit by edit. Each edit displaces owners the runtime's maintenance retires between turns; seeding straight on the store,
/// this drains them under pressure as that maintenance does, so a long seed never saturates the store's fixed retirement
/// authority.
async fn apply_other_edits(app: &mut ToyApp, count: i32) {
""",
        """/// ✏️ `count` one-operation edits dispatched straight on `app`'s store — a long downstream history a replay steps through
/// edit by edit. Their mutations name no author, so the store authors every one as the fixture actor (`local`) and takes
/// that actor as its local one. Each edit displaces owners the runtime's maintenance retires between turns; seeding
/// straight on the store, this drains them under pressure as that maintenance does, so a long seed never saturates the
/// store's fixed retirement authority.
async fn apply_other_edits(app: &mut ToyApp, count: i32) {
""",
    ),
    (
        """/// ⚖️ LAW (gap N17): an interior undo over a long downstream history — the author's own edit under 600 edits of another
/// author — is a local history step the runtime replays over driver turns:
""",
        """/// ⚰️ One edit of `author` — authored through the runtime, whose publication stamps the acting actor — under `count`
/// one-operation edits of the fixture actor ([`apply_other_edits`]): the history an interior undo of `author` replays
/// through. Proves what it built: exactly one history row is `author`'s and it is not the newest.
async fn bury_edit_of(app: &mut ToyApp, author: &str, count: i32) {
    let meta = ActionMeta { view_state: Some(ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), ..artifact_app_laws::meta(author) };
    app.dispatch_emit("select", Emit::<TestMutation, TestConfigMutation, NoDraftMutation>::mutations(vec![SetLabel { value: "buried".into() }.into()]), &meta).await.expect("the author's edit publishes");
    apply_other_edits(app, count).await;
    app.refresh_cache().await.expect("backfill");
    let authors: Vec<Option<String>> = app.history_patch(true).await.expect("history patch").upserts.into_iter().map(|row| row.author).collect();
    let own = authors.iter().filter(|row| row.as_deref() == Some(author)).count();
    assert!(own == 1 && authors.first().and_then(|row| row.as_deref()) != Some(author), "one interior row is {author}'s: {own} of {} rows, newest {:?}", authors.len(), authors.first());
}

/// ⚖️ LAW (gap N17): an interior undo over a long downstream history — the author's own edit under 600 edits of another
/// author — is a local history step the runtime replays over driver turns:
""",
    ),
    (
        """    let actor = text(&fixture["actor"]).to_string();
    let mut undeferred = seeded_app(&fixture).await;
    undeferred.store.defer_local_replays(None);
    undeferred.store.set_local_actor_id(Some("other".into())).expect("another author");
    apply_other_edits(&mut undeferred, 600).await;
    undeferred.refresh_cache().await.expect("backfill");
    history_verb(&mut undeferred, &actor, "undo", None).await;
    let undone = undeferred.store.snapshot().expect("the undeferred undo");
    for cancel in [true, false] {
        let mut app = seeded_app(&fixture).await;
        app.store.set_local_actor_id(Some("other".into())).expect("another author");
        apply_other_edits(&mut app, 600).await;
        app.refresh_cache().await.expect("backfill");
        let before = history_trace(&mut app).await;
""",
        """    let actor = "author".to_string();
    let mut undeferred = seeded_app(&fixture).await;
    undeferred.store.defer_local_replays(None);
    bury_edit_of(&mut undeferred, &actor, 600).await;
    history_verb(&mut undeferred, &actor, "undo", None).await;
    let undone = undeferred.store.snapshot().expect("the undeferred undo");
    for cancel in [true, false] {
        let mut app = seeded_app(&fixture).await;
        bury_edit_of(&mut app, &actor, 600).await;
        let before = history_trace(&mut app).await;
""",
    ),
    (
        """    let actor = text(&fixture["actor"]).to_string();
    let mut app = seeded_app(&fixture).await;
    app.time_travel.set_turn_clock(counted_clock);
    app.store.defer_local_replays(Some(store::ReplayTurnBudget { wall_us: TIME_TRAVEL_TURN_WALL_US, operations: time_travel::TIME_TRAVEL_REPLAY_OPERATIONS, now_us: counted_clock }));
    app.store.set_local_actor_id(Some("other".into())).expect("another author");
    apply_other_edits(&mut app, 600).await;
    app.refresh_cache().await.expect("backfill");
    history_verb(&mut app, &actor, "undo", None).await;
""",
        """    let actor = "author".to_string();
    let mut app = seeded_app(&fixture).await;
    app.time_travel.set_turn_clock(counted_clock);
    app.store.defer_local_replays(Some(store::ReplayTurnBudget { wall_us: TIME_TRAVEL_TURN_WALL_US, operations: time_travel::TIME_TRAVEL_REPLAY_OPERATIONS, now_us: counted_clock }));
    bury_edit_of(&mut app, &actor, 600).await;
    history_verb(&mut app, &actor, "undo", None).await;
""",
    ),
    (
        """    assert!(render_body(&mut app).await.contains("count=1 label=c"), "the preview follows the draft");
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    drop(probe);
    close(&mut app);
}
//#endregion 📡️RemoteEditWhileEditing
""",
        """    assert!(render_body(&mut app).await.contains("count=1 label=c"), "the preview follows the draft");
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    drop(probe);
    close(&mut app);
}

/// ⚖️ LAW (design §22.24, store wave RB): a replay finished before a port change still commits. A backbone attaching and
/// detaching while the session reviews its finished replay moves the store's local generation and no event: driver turns
/// deliver no base move and start no second replay, and Finalize → Overwrite commits the head reviewed — no
/// `timeTravel.stale`, the session closes with its commit.
#[semio_framework_async_macros::async_test]
async fn a_replay_finished_before_a_backbone_attach_and_detach_still_commits() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } }), serde_json::json!({ "accept": null }), serde_json::json!({ "replay": "clean" })] {
        if let Some(result) = run_step(&mut app, &fixture, &step).await {
            assert_eq!(rejected(&result), None, "{step}: {:?}", result.output);
        }
    }
    let before = app.time_travel.session().clone();
    assert_eq!(before.stage, TimeTravelStage::Reviewing, "the replay finished");
    let (store_generation, revision) = (app.store.generation(), app.store.content_revision());
    let (backbone, probe) = MemoryBackbone::pair("attach-while-reviewing", "attach-while-reviewing").await;
    app.attach_backbone(store::Backbones::Memory(backbone)).await.expect("the backbone attaches");
    app.detach_backbone().await.expect("the backbone detaches");
    assert!(app.store.generation() > store_generation && app.store.content_revision() == revision, "the port moved the store's local generation and no event");
    for _ in 0..4 {
        app.advance_typed_operation_publication().await.expect("a driver turn");
        while app.take_typed_operation_ui_progress().is_some() {}
    }
    assert_eq!(app.time_travel.session(), &before, "no base move: the review stands and no second replay started");
    for step in [serde_json::json!({ "finalize": null }), serde_json::json!({ "commit": { "choice": "overwrite" } })] {
        let result = run_step(&mut app, &fixture, &step).await.expect("a verb result");
        assert_eq!(rejected(&result), None, "{step}: the replay finished before the port change still commits: {:?}", result.output);
    }
    pump_until(&mut app, "the finalize retires", |app| !app.time_travel.has_pending_work()).await;
    assert!(app.time_travel.status().is_none(), "the session closed with its commit: {:?}", app.time_travel.session().stage);
    assert_eq!(head(&app), (5, "b".to_string()), "the committed document is the head reviewed");
    assert!(render_body(&mut app).await.contains("count=5 label=b"), "every window shows the committed edit");
    drop(probe);
    close(&mut app);
}
//#endregion 📡️RemoteEditWhileEditing
""",
    ),
]


def files(_root):
    return {LAW: LAW_RS}
