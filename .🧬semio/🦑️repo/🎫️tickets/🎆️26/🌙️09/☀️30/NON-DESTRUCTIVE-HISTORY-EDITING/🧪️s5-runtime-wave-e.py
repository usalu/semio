"""🎞️ Wave E (live probe: the `choosing` edge took one full turn; coordinator approval 04:5x): a session edge re-publishes
every window body only when the document a window shows was swapped.

- `TT` `apply_time_travel_event`: the rendered document is dirty when the identity of the shown snapshot changed across the
  event and its effects (committed ↔ draft preview ↔ replayed head), not whenever an effect ran. RequestFinalize, Back,
  Accept-into-replay, Rerun and a Restore that keeps drafts answer the history body and the chips only.
- `TT` `TimeTravelStoreState::shown`: a replay started from a review (Replay again, a restored draft, a moved base) keeps
  showing the head reviewed last instead of falling back to the committed document until the new head arrives.
- Law: the scope of every edge of one session, and the body a window shows across a rerun.

Loaded by `🧪️s5-runtime-land.py`; applies on top of wave D.
"""

OSM = "🧰️framework/🛍️products/💻️os/🔨️modules"
PLUGIN = f"{OSM}/🔌️plugin"
TT = f"{PLUGIN}/⏪️time-travel/🦀️.rs"
LAW = f"{PLUGIN}/🧪️tests/🧪️time-travel/🦀️.rs"

TT_RS = [
    (
        """    /// 🪞️ The snapshot the session shows at `stage`: the draft preview while editing or replaying, the replayed head while
    /// reviewing.
    pub(crate) fn shown(&self, stage: TimeTravelStage) -> Option<&store::ArtifactDerivedSnapshot<P>> {
        match stage {
            TimeTravelStage::Inactive => None,
            TimeTravelStage::Editing | TimeTravelStage::Replaying => self.preview.as_ref(),
            TimeTravelStage::Reviewing | TimeTravelStage::Choosing | TimeTravelStage::Finalizing => self.head.as_ref(),
        }
    }
""",
        """    /// 🪞️ The snapshot the session shows at `stage`: the draft preview while editing; while replaying that preview — or,
    /// for a replay started from a review (Replay again, a restored draft, a moved base), the head reviewed last, so no
    /// window falls back to the committed document in between; the replayed head while reviewing.
    pub(crate) fn shown(&self, stage: TimeTravelStage) -> Option<&store::ArtifactDerivedSnapshot<P>> {
        match stage {
            TimeTravelStage::Inactive => None,
            TimeTravelStage::Editing => self.preview.as_ref(),
            TimeTravelStage::Replaying => self.preview.as_ref().or(self.head.as_ref()),
            TimeTravelStage::Reviewing | TimeTravelStage::Choosing | TimeTravelStage::Finalizing => self.head.as_ref(),
        }
    }
""",
    ),
    (
        """    /// ⚖️ Applies one event to the session and performs its effects; a refusal changes nothing.
    async fn apply_time_travel_event(&mut self, event: TimeTravelEvent, meta: Option<&ActionMeta>, effects: &mut Vec<Effect>) -> Result<TimeTravelActionOutcome, Fault> {
        let preview_changes = matches!(event, TimeTravelEvent::Accept { .. } | TimeTravelEvent::Discard { .. } | TimeTravelEvent::Restore { .. } | TimeTravelEvent::Exit | TimeTravelEvent::Back { .. });
        match self.time_travel.session.apply(event) {
            Ok(session_effects) => {
                self.note_time_travel_changed(preview_changes || !session_effects.is_empty(), true);
                self.perform_time_travel_effects(session_effects, meta, effects).await?;
                Ok(TimeTravelActionOutcome::Applied(self.time_travel.session.stage))
            }
""",
        """    /// 🎞️ The identity of the document snapshot the session shows in its stage on the document's own store (`None`: the
    /// committed document) — what tells whether a session change swapped the document a window renders. A session on a
    /// composed member marks its children view itself ([`Self::refresh_time_travel_children`]).
    fn time_travel_shown_identity(&self) -> Option<usize> {
        self.time_travel.document.shown(self.time_travel.session.stage).map(|shown| Arc::as_ptr(shown.snapshot_owner()) as usize)
    }

    /// ⚖️ Applies one event to the session and performs its effects; a refusal changes nothing. Every window body is
    /// re-published only when the event or its effects swapped the document a window shows (the committed document, the
    /// draft preview, the replayed head); any other edge — the finalize prompt opening or closing, an accepted draft
    /// starting its replay, a replay run again — re-publishes the history body and the chips alone.
    async fn apply_time_travel_event(&mut self, event: TimeTravelEvent, meta: Option<&ActionMeta>, effects: &mut Vec<Effect>) -> Result<TimeTravelActionOutcome, Fault> {
        let shown = self.time_travel_shown_identity();
        match self.time_travel.session.apply(event) {
            Ok(session_effects) => {
                self.note_time_travel_changed(false, true);
                self.perform_time_travel_effects(session_effects, meta, effects).await?;
                self.time_travel.document_dirty |= shown != self.time_travel_shown_identity();
                Ok(TimeTravelActionOutcome::Applied(self.time_travel.session.stage))
            }
""",
    ),
]

LAW_RS = [
    (
        """//#region 🚫️RowWithdraw
""",
        """//#region 🎞️SessionEdgeScope
/// ⚖️ LAW (live probe: the `choosing` edge; design §20.14, per-render work is O(change)): a history-edit verb re-publishes
/// every window body only when the document a window shows was swapped — opening the session (committed → preview), a
/// draft (a new preview), a finished replay (preview → head), leaving (→ committed). The edges that swap nothing —
/// an accepted draft starting its replay, the finalize prompt opening and closing, a replay run again — re-publish the
/// history body alone; and a replay run again from a review keeps showing the head reviewed last, never the committed
/// document.
#[semio_framework_async_macros::async_test]
async fn a_session_edge_republishes_every_window_only_when_the_shown_document_swaps() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    let full = |result: &InvocationResult| matches!(result.ui_scope, UiDirtyScope::Full);
    let history_only = |result: &InvocationResult| matches!(&result.ui_scope, UiDirtyScope::Partial { window_bodies, panel_bodies, .. } if window_bodies.is_empty() && panel_bodies.len() == 1 && panel_bodies[0] == FRAMEWORK_HISTORY_BODY_KEY);
    let mutation = seeded_mutation(&app, 1);
    let begun = verb(&mut app, &fixture, "historyEditBegin", vec![("mutationId".into(), DslValue::String(mutation))]).await;
    assert!(rejected(&begun).is_none() && full(&begun), "opening shows the preview in every window: {:?}", begun.ui_scope);
    let drafted = verb(&mut app, &fixture, "historyEditInput", vec![("path".into(), DslValue::String("/value".into())), ("value".into(), DslValue::String("b".into()))]).await;
    assert!(rejected(&drafted).is_none() && full(&drafted), "a draft is a new preview: {:?}", drafted.ui_scope);
    let accepted = verb(&mut app, &fixture, "historyEditAccept", Vec::new()).await;
    assert!(rejected(&accepted).is_none() && history_only(&accepted), "an accepted draft keeps its preview while it replays: {:?}", accepted.ui_scope);
    assert!(render_body(&mut app).await.contains("count=1 label=b"), "the preview stays while the replay runs");
    pump_until(&mut app, "the replay completes", |app| app.time_travel.session().stage != TimeTravelStage::Replaying).await;
    assert!(render_body(&mut app).await.contains("count=5 label=b"), "the review shows the replayed head");

    let prompted = verb(&mut app, &fixture, "historyEditFinalize", Vec::new()).await;
    assert!(rejected(&prompted).is_none() && history_only(&prompted), "the finalize prompt swaps no document: {:?}", prompted.ui_scope);
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Choosing);
    let back = verb(&mut app, &fixture, "historyEditBack", Vec::new()).await;
    assert!(rejected(&back).is_none() && history_only(&back), "leaving the prompt swaps no document: {:?}", back.ui_scope);

    let first = seeded_mutation(&app, 0);
    for (action, args) in [("historyEditBegin", vec![("mutationId".to_string(), DslValue::String(first))]), ("historyEditInput", vec![("path".to_string(), DslValue::String("/value".into())), ("value".to_string(), DslValue::uint(7))]), ("historyEditAccept", Vec::new())] {
        let result = verb(&mut app, &fixture, action, args).await;
        assert_eq!(rejected(&result), None, "{action}: {:?}", result.output);
    }
    let cancelled = verb(&mut app, &fixture, "historyEditCancelReplay", Vec::new()).await;
    assert!(rejected(&cancelled).is_none() && full(&cancelled), "a cancelled replay leaves its preview for the head reviewed last: {:?}", cancelled.ui_scope);
    assert!(render_body(&mut app).await.contains("count=5 label=b"), "the cancelled review shows the head reviewed last");
    let rerun = verb(&mut app, &fixture, "historyEditRerun", Vec::new()).await;
    assert!(rejected(&rerun).is_none() && history_only(&rerun), "a replay run again keeps the head reviewed last: {:?}", rerun.ui_scope);
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Replaying);
    let replaying = render_body(&mut app).await;
    assert!(replaying.contains("count=5 label=b") && !replaying.contains(text(&fixture["committedBody"])), "no window falls back to the committed document while it replays again: {replaying}");
    pump_until(&mut app, "the replay completes", |app| app.time_travel.session().stage != TimeTravelStage::Replaying).await;
    assert!(render_body(&mut app).await.contains("count=5 label=b"), "the review shows the replayed head");

    let exited = verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    assert!(rejected(&exited).is_none() && full(&exited), "leaving shows the committed document in every window: {:?}", exited.ui_scope);
    assert!(render_body(&mut app).await.contains(text(&fixture["committedBody"])));
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
}
//#endregion 🎞️SessionEdgeScope

//#region 🚫️RowWithdraw
""",
    ),
]


def files(_root):
    return {TT: TT_RS, LAW: LAW_RS}
