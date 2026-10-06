"""⚰️ Wave F fix (tests only): `bury_edit_of` authors the acting author's edit through the store's batched publication.

The run of wave F (05:56, `test-plugin-laws-4.txt`) showed "0 of 605 rows" authored by the acting author: the runtime's
plain emit route (`dispatch_emit` → `ArtifactCommand::Apply`) authors every edit as `local` too — only the batched
publication (the retained tool route, `begin_apply_batch(.., actor, ..)`) stamps the acting actor on the edit it mints.

Loaded by `🧪️s5-runtime-land.py`; applies on top of wave F.
"""

OSM = "🧰️framework/🛍️products/💻️os/🔨️modules"
LAW = f"{OSM}/🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs"

LAW_RS = [
    (
        """/// ⚰️ One edit of `author` — authored through the runtime, whose publication stamps the acting actor — under `count`
/// one-operation edits of the fixture actor ([`apply_other_edits`]): the history an interior undo of `author` replays
/// through. Proves what it built: exactly one history row is `author`'s and it is not the newest.
async fn bury_edit_of(app: &mut ToyApp, author: &str, count: i32) {
    let meta = ActionMeta { view_state: Some(ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), ..artifact_app_laws::meta(author) };
    app.dispatch_emit("select", Emit::<TestMutation, TestConfigMutation, NoDraftMutation>::mutations(vec![SetLabel { value: "buried".into() }.into()]), &meta).await.expect("the author's edit publishes");
    apply_other_edits(app, count).await;
""",
        """/// ⚰️ One edit of `author` under `count` one-operation edits of the fixture actor ([`apply_other_edits`]): the history an
/// interior undo of `author` replays through. The author's edit takes the store's batched publication — the route a
/// tool's edit takes and the one that stamps the acting actor on the edit it mints (a plain `Apply`, the runtime's emit
/// route included, authors as `local`). Proves what it built: exactly one history row is `author`'s and it is not the
/// newest.
async fn bury_edit_of(app: &mut ToyApp, author: &str, count: i32) {
    let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4_096 };
    let mut publication = app
        .store
        .begin_apply_batch(semio_framework_job::OperationId(1), app.store.generation_now(), app.store.content_revision_now(), author.into(), vec![SetLabel { value: "buried".into() }.into()], None, store::HistoryLane::Document, app.artifact_one_item_factory.as_ref(), None)
        .unwrap_or_else(|rejected| panic!("the author's edit is admitted: {}", rejected.into_owners().0));
    let published = (0..65_536).any(|_| matches!(app.store.advance_apply_batch(&mut publication, grant).expect("the author's edit publishes"), store::ArtifactStoreOneItemAdvance::Published(_)));
    assert!(published && publication.acknowledge(), "the author's edit is published and acknowledged");
    publication.begin_close();
    let closed = (0..4_096).any(|_| publication.close_step(grant).expect("the publication closes") == store::SnapshotRetirementStep::Complete);
    assert!(closed && publication.terminal_is_empty(), "the publication retires every owner");
    drop(publication);
    apply_other_edits(app, count).await;
""",
    ),
]


def files(_root):
    return {LAW: LAW_RS}
