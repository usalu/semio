"""📡️ Wave D (live fault F4, session half; design §22.1 unit refusal): a remote edit ingested while a draft is edited.

- Pure session (`FW/⏪️time-travel`, coordinator decision 05:3x): the base a session watches is the CONTENT REVISION of
  its store alone — `TimeTravelBase { content_revision }`. The store's local generation also moves when a document port
  attaches or detaches (no event changes), which was a `BaseMoved` and answered the user's next draft input `stale`.
  Schema, generator + lifecycle-law fixture, Rust, TS twin, both law drivers; the plugin runtime's base query; law: a
  backbone attach + detach while a draft is edited is no base move.

- `TT`: a base move re-resolves the open editor's applied position with the session's (a remote edit that sorts before the
  edited mutation moves it; the pending rows are counted from the position the editor holds); a mutation of a
  cross-document unit (`AppliedMutation.unit`, the store's own law) is neither editable nor withdrawable on its row, and
  opening it is refused: Withdraw answers the store's code `history.unit-spans-documents`, Edit `timeTravel.not-editable`.
- `PLG` `🔖️CommandLog`: the code constant beside the other history notice codes.
- Law (plugin, no folder): a remote edit relayed into a replica whose session edits a draft moves the base and nothing
  else — the draft, the editor and the generation the editor's controls carry survive (the user's next input is not
  stale), the remote edit is listed downstream as not applied, nothing waits for a replay, Accept replays it too, and the
  overwrite converges on the other replica.
- Law harness repair: `apply_other_edits` (600 edits seeded straight on the store) saturated the store's displaced-owner
  retirement authority (1024 owners) — it now drains them under pressure like the runtime's maintenance; the two N17 laws
  that seed through it failed there (65 passed / 2 failed at 05:16).

Loaded by `🧪️s5-runtime-land.py`. Non-test, non-Rust framework files (the TS twin, schema, fixture) need the `serve` lock too.
"""

import importlib.util
import json
import pathlib

FW = "🧰️framework/🔨️modules"
FWT = f"{FW}/⏪️time-travel"
HERE = pathlib.Path(__file__).resolve().parent

OSM = "🧰️framework/🛍️products/💻️os/🔨️modules"
PLUGIN = f"{OSM}/🔌️plugin"
TT = f"{PLUGIN}/⏪️time-travel/🦀️.rs"
PLG = f"{PLUGIN}/🦀️.rs"
LAW = f"{PLUGIN}/🧪️tests/🧪️time-travel/🦀️.rs"

FWT_RS = [
    (
        """//! - Every store generation change is forwarded as [`TimeTravelEvent::BaseMoved`], in every stage
//!   (also `Inactive`), with the re-resolved positions of every session target; an unchanged base is
//!   `timeTravel.stale`.
""",
        """//! - Every change of the store's content revision is forwarded as [`TimeTravelEvent::BaseMoved`], in every
//!   stage (also `Inactive`), with the re-resolved positions of every session target; an unchanged base is
//!   `timeTravel.stale`. The store's local generation is no part of the base.
""",
    ),
    (
        """/// 🧭️ The store state a session last observed: store generation and 32-byte content revision.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TimeTravelBase {
    pub store_generation: u64,
    pub content_revision: [u8; 32],
}
""",
        """/// 🧭️ The store state a session last observed: the 32-byte content revision of its history. The store's local
/// generation is no part of it — a document port attaching or detaching moves that generation without changing an
/// event, and never moves the base.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TimeTravelBase {
    pub content_revision: [u8; 32],
}
""",
    ),
]

FWT_UNIT = [
    (
        """    TimeTravelBase { store_generation: value["storeGeneration"].as_u64().expect("u64"), content_revision: bytes(&value["contentRevision"]).try_into().expect("32 bytes") }
""",
        """    TimeTravelBase { content_revision: bytes(&value["contentRevision"]).try_into().expect("32 bytes") }
""",
    ),
    (
        """    let base = TimeTravelBase { store_generation: 7, content_revision: [3; 32] };
""",
        """    let base = TimeTravelBase { content_revision: [3; 32] };
""",
    ),
]

FWT_TS = [
    (
        """ * `begin` is); `restore` is a row's Restore (it takes one accepted draft back while `reviewing`); every
 * store generation change is sent as `baseMoved` with the re-resolved positions of every session
 * target (in every stage; an unchanged base is `timeTravel.stale`); every other event except `begin`,
""",
        """ * `begin` is); `restore` is a row's Restore (it takes one accepted draft back while `reviewing`); every
 * change of the store's content revision is sent as `baseMoved` with the re-resolved positions of every
 * session target (in every stage; an unchanged base is `timeTravel.stale`; the store's local generation is
 * no part of the base); every other event except `begin`,
""",
    ),
    (
        """/** 🧭️ The store state a session last observed. */
export type TimeTravelBase = { readonly storeGeneration: bigint; readonly contentRevision: Uint8Array };
""",
        """/** 🧭️ The store state a session last observed: the content revision of its history (never the store's local generation). */
export type TimeTravelBase = { readonly contentRevision: Uint8Array };
""",
    ),
    (
        """  return left.storeGeneration === right.storeGeneration && left.contentRevision.length === right.contentRevision.length && left.contentRevision.every((byte, index) => byte === right.contentRevision[index]);
""",
        """  return left.contentRevision.length === right.contentRevision.length && left.contentRevision.every((byte, index) => byte === right.contentRevision[index]);
""",
    ),
    (
        """const baseFromJson = (json: Json): TimeTravelBase => ({ storeGeneration: BigInt(json.storeGeneration), contentRevision: timeTravelHexToBytes(json.contentRevision) });
const baseToJson = (base: TimeTravelBase): Json => ({ storeGeneration: Number(base.storeGeneration), contentRevision: timeTravelBytesToHex(base.contentRevision) });
""",
        """const baseFromJson = (json: Json): TimeTravelBase => ({ contentRevision: timeTravelHexToBytes(json.contentRevision) });
const baseToJson = (base: TimeTravelBase): Json => ({ contentRevision: timeTravelBytesToHex(base.contentRevision) });
""",
    ),
]

FWT_CONFORMANCE = [
    (
        """    base: M.timeTravelBytesToHex(session.base.contentRevision) + `@${session.base.storeGeneration}`,
""",
        """    base: M.timeTravelBytesToHex(session.base.contentRevision),
""",
    ),
    (
        """        return { type, base: M.timeTravelBytesToHex(event.base.contentRevision) + `@${event.base.storeGeneration}`, positions: event.positions.map((target) => [target.mutation, target.position]) };
""",
        """        return { type, base: M.timeTravelBytesToHex(event.base.contentRevision), positions: event.positions.map((target) => [target.mutation, target.position]) };
""",
    ),
    (
        """  const bases = [0, 1, 2].map((index) => ({ storeGeneration: BigInt(10 + index), contentRevision: new Uint8Array(32).fill(index + 1) }));
""",
        """  const bases = [0, 1, 2].map((index) => ({ contentRevision: new Uint8Array(32).fill(index + 1) }));
""",
    ),
]

PLUGIN_ORACLE = [
    (
        """  const base = { storeGeneration: 0n, contentRevision: new Uint8Array(32) };
""",
        """  const base = { contentRevision: new Uint8Array(32) };
""",
    ),
]


def fwt_schema(text):
    document = json.loads(text)
    base = document["$defs"]["TimeTravelBase"]
    assert base["required"] == ["storeGeneration", "contentRevision"], base["required"]
    base["required"] = ["contentRevision"]
    del base["properties"]["storeGeneration"]
    base["description"] = "The store state a session last observed: the content revision of its history. The store's local generation is no part of it."
    return json.dumps(document, indent=2, ensure_ascii=False) + "\n"


def fwt_fixture(_text):
    spec = importlib.util.spec_from_file_location("lifecycle_law", HERE / "🧪️w1-b-generate-lifecycle-law.py")
    generator = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(generator)
    return generator.TEXT


TT_RS = [
    (
        """            TimeTravelStoreQuery::Base => TimeTravelQueryOutput::Base(TimeTravelBase { store_generation: store.generation(), content_revision: store.content_revision() }),
""",
        """            TimeTravelStoreQuery::Base => TimeTravelQueryOutput::Base(TimeTravelBase { content_revision: store.content_revision() }),
""",
    ),
    (
        """    /// 👀️ Store watch (design §4, driver contract): every generation change of the store the session edits reaches the
""",
        """    /// 👀️ Store watch (design §4, driver contract): every change of the content revision of the store the session edits
    /// (never of its local generation alone: a document port attaching or detaching is no base move) reaches the
""",
    ),
    (
        """    NotWithdrawable,
    EditorClosed,
}
""",
        """    NotWithdrawable,
    EditorClosed,
    UnitSpansDocuments,
}
""",
    ),
    (
        """            Self::EditorClosed => TIME_TRAVEL_EDITOR_CLOSED_CODE,
        }
""",
        """            Self::EditorClosed => TIME_TRAVEL_EDITOR_CLOSED_CODE,
            Self::UnitSpansDocuments => HISTORY_UNIT_SPANS_DOCUMENTS_CODE,
        }
""",
    ),
    (
        """    /// ([`time_travel_admits_withdrawal`]), and an operation without editable inputs opens on an editor without inputs.
""",
        """    /// ([`time_travel_admits_withdrawal`]), and an operation without editable inputs opens on an editor without inputs.
    /// An operation of a cross-document unit takes no supersession in this document alone (`AppliedMutation::unit`, the
    /// store's authoring law): its Withdraw answers the store's own refusal, its inputs are not editable.
""",
    ),
    (
        """        let ops = store.mutation_ops().map_err(|_| TimeTravelActionRefusal::UnknownMutation)?;
        let row = ops.iter().find(|row| row.mutation_id == *target).ok_or(TimeTravelActionRefusal::UnknownMutation)?;
""",
        """        let ops = store.mutation_ops().map_err(|_| TimeTravelActionRefusal::UnknownMutation)?;
        let row = ops.iter().find(|row| row.mutation_id == *target).ok_or(TimeTravelActionRefusal::UnknownMutation)?;
        if row.unit.is_some() {
            return Err(if withdraw { TimeTravelActionRefusal::UnitSpansDocuments } else { TimeTravelActionRefusal::NotEditable });
        }
""",
    ),
    (
        """    /// session as `BaseMoved` with the re-resolved target positions; a target that vanished (undone remotely, document
    /// replaced, the edited member gone) exits the session, since nothing it drafted is addressable anymore. Answers
    /// whether the base moved.
""",
        """    /// session as `BaseMoved` with the re-resolved target positions — the open editor's too, so the rows downstream of
    /// the edited mutation stay the pending ones when a remote edit sorts before it; a target that vanished (undone
    /// remotely, document replaced, the edited member gone) exits the session, since nothing it drafted is addressable
    /// anymore. Answers whether the base moved.
""",
    ),
    (
        """        if self.time_travel.is_active() || !effects.is_empty() {
            self.note_time_travel_changed(true, true);
        }
        let mut dialogs = Vec::new();
        self.perform_time_travel_effects(effects, None, &mut dialogs).await?;
        Ok(true)
""",
        """        if let (Some(editor), Some(pending)) = (self.time_travel.editor.as_mut(), self.time_travel.session.pending.as_ref()) {
            editor.position = pending.target.position;
        }
        if self.time_travel.is_active() || !effects.is_empty() {
            self.note_time_travel_changed(true, true);
        }
        let mut dialogs = Vec::new();
        self.perform_time_travel_effects(effects, None, &mut dialogs).await?;
        Ok(true)
""",
    ),
]

PLG_RS = [
    (
        """    pub const HISTORY_STEP_BLOCKED_CODE: &str = "history.step-blocked";
""",
        """    pub const HISTORY_STEP_BLOCKED_CODE: &str = "history.step-blocked";

    /// 🫱️ The code the document store refuses a supersession of one operation of a cross-document unit by
    /// (`VcsError::UnitSpansDocuments`), which a mutation row's Withdraw answers for such an operation; its en/de notice is
    /// the kernel's `history_notice("history.unit-spans-documents")`.
    pub const HISTORY_UNIT_SPANS_DOCUMENTS_CODE: &str = "history.unit-spans-documents";
""",
    ),
]

TT_RS += [
    (
        """/// shows an input ([`time_travel_schema_shows_inputs`], design §22.20) and no foreign steps; `withdrawable` asks the
/// store's supersede law for an operation that is not withdrawn yet ([`time_travel_admits_withdrawal`]); a `viewer`
/// edits and withdraws nothing.
""",
        """/// shows an input ([`time_travel_schema_shows_inputs`], design §22.20) and no foreign steps; `withdrawable` asks the
/// store's supersede law for an operation that is not withdrawn yet ([`time_travel_admits_withdrawal`]); an operation
/// of a cross-document unit (`AppliedMutation::unit`) is neither — it takes no supersession in one document alone; a
/// `viewer` edits and withdraws nothing.
""",
    ),
    (
        """    let editable = !viewer && !shown.may_emit_foreign_steps() && shown.input_schema().is_some_and(time_travel_schema_shows_inputs);
""",
        """    let editable = !viewer && op.unit.is_none() && !shown.may_emit_foreign_steps() && shown.input_schema().is_some_and(time_travel_schema_shows_inputs);
""",
    ),
    (
        """        withdrawable: !viewer && !withdrawn && time_travel_admits_withdrawal::<P, Mu>(op.operation),
""",
        """        withdrawable: !viewer && !withdrawn && op.unit.is_none() && time_travel_admits_withdrawal::<P, Mu>(op.operation),
""",
    ),
]

LAW_RS = [
    (
        """app.time_travel.session().report.is_some() && app.time_travel.session().base.store_generation == app.store.generation())
""",
        """app.time_travel.session().report.is_some() && app.time_travel.session().base.content_revision == app.store.content_revision())
""",
    ),
    (
        """/// ✏️ `count` one-operation edits on `app`'s store as its current local actor — a long downstream history a replay steps through
/// edit by edit.
async fn apply_other_edits(app: &mut ToyApp, count: i32) {
    for value in 0..count {
        app.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value }.into()], description: None, transaction: None }).await.expect("another author's edit");
    }
}
""",
        """/// ✏️ `count` one-operation edits on `app`'s store as its current local actor — a long downstream history a replay steps through
/// edit by edit. Each edit displaces owners the runtime's maintenance retires between turns; seeding straight on the store,
/// this drains them under pressure as that maintenance does, so a long seed never saturates the store's fixed retirement
/// authority.
async fn apply_other_edits(app: &mut ToyApp, count: i32) {
    for value in 0..count {
        app.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value }.into()], description: None, transaction: None }).await.expect("another author's edit");
        for _ in 0..4_096 {
            if !app.store.maintenance_retirements_under_pressure() || !matches!(app.store.maintenance_retirements_step(64, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).expect("the store retires what the edits displaced"), store::SnapshotRetirementStep::Pending { .. }) {
                break;
            }
        }
    }
}
""",
    ),
    (
        """//#region 🚫️RowWithdraw
""",
        """//#region 📡️RemoteEditWhileEditing
/// ⚖️ LAW (live fault F4, session half; design §7: remote ingests keep arriving → `BaseMoved`): a remote edit ingested
/// while a draft is being edited moves the base and nothing else. The session stays `Editing` with its draft, its editor
/// and its generation, so an input stamped with the generation the editor's controls carried before the base move is
/// not stale; the remote edit is listed downstream of the edited mutation as not applied while editing, and nothing
/// waits for a replay (a tail edit is adopted at ingest); Accept replays the remote edit too — the reviewed head holds
/// the draft and the remote edit — and the overwrite converges on the other replica.
#[semio_framework_async_macros::async_test]
async fn a_remote_edit_while_editing_keeps_the_draft_and_accept_replays_it_too() {
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let (mut local, mut local_probe) = replica(&fixture, "remote-while-editing-local", &actor, true).await;
    let (mut remote, mut remote_probe) = replica(&fixture, "remote-while-editing-remote", "remote", false).await;
    relay(&mut local_probe, &mut remote_probe, &mut remote).await;
    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } })] {
        let result = run_step(&mut local, &fixture, &step).await.expect("a verb result");
        assert_eq!(rejected(&result), None, "{step}: {:?}", result.output);
    }
    let (target, generation) = (seeded_mutation(&local, 1), local.time_travel.session().generation);
    let drafted = local.time_travel.session().pending.clone().expect("a pending draft");

    remote.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value: 9 }.into()], description: None, transaction: None }).await.expect("the remote edit");
    relay_adopted(&mut remote_probe, &mut local_probe, &mut local).await;
    pump_until(&mut local, "the base move reaches the session", |app| app.time_travel.session().base.content_revision == app.store.content_revision()).await;
    let arrived = local.store.mutation_ops().expect("applied operations").last().map(|op| op.mutation_id.0.clone()).expect("the remote operation");
    assert!(arrived != seeded_mutation(&local, 3), "the remote edit is applied in this replica's store");

    let session = local.time_travel.session();
    assert_eq!((session.stage, session.generation, session.pending.as_ref()), (TimeTravelStage::Editing, generation, Some(&drafted)), "the base move keeps the stage, the generation and the draft");
    assert_eq!(local.time_travel.editor().map(|editor| (editor.target.0.clone(), editor.value.get("value").and_then(DslValue::as_str).map(str::to_string))), Some((target.clone(), Some("b".to_string()))), "the editor keeps its drafted value");
    assert!(local.reprojection_status().is_none(), "a tail edit is adopted at ingest: nothing waits for a replay");
    let wire = local.history_patch(true).await.expect("history patch").upserts.into_iter().flat_map(|row| row.mutations).find(|mutation| mutation.mutation_id == arrived).expect("the remote edit is a history row");
    assert!(wire.pending && wire.label.resolve(Terminology::Native, Locale::En) == "Set count to 9", "the remote edit is downstream and not applied in the preview: {wire:?}");
    let seq = local.history_patch(true).await.expect("history patch").upserts.iter().find(|entry| entry.mutations.iter().any(|mutation| mutation.mutation_id == arrived)).map(|entry| entry.seq).expect("the history row of the remote edit");
    let body = render_history_window(&mut local, seq, 0, 8).await;
    let row = find_node(&body, &format!("framework.history.mutation.{arrived}")).unwrap_or_else(|| panic!("the mutation row of the remote edit: {body}"));
    assert!(row.to_string().contains("Not applied while editing"), "the row says so in words: {row}");
    assert!(render_body(&mut local).await.contains("count=1 label=b"), "the preview stays the state before the edited mutation with the draft");

    let typed = verb(&mut local, &fixture, "historyEditInput", vec![("path".into(), DslValue::String("/value".into())), ("value".into(), DslValue::String("c".into())), ("generation".into(), DslValue::uint(u64::from(generation)))]).await;
    assert_eq!(rejected(&typed), None, "the user's next input, stamped with the generation from before the base move, is not stale");

    let accepted = verb(&mut local, &fixture, "historyEditAccept", Vec::new()).await;
    assert_eq!(rejected(&accepted), None, "{:?}", accepted.output);
    pump_until(&mut local, "the replay completes", |app| app.time_travel.session().stage != TimeTravelStage::Replaying).await;
    let status = local.time_travel.status().expect("a reviewing session");
    assert_eq!((status.review, status.blocking, status.accepted_count), (Some(semio_framework::kernel::HistoryTimeTravelReview::Ready), false, 1), "the replay over the remote edit is clean");
    assert!(render_body(&mut local).await.contains("count=9 label=c"), "the reviewed head holds the draft and the remote edit");

    for (action, args) in [("historyEditFinalize", Vec::new()), ("historyEditCommit", vec![("choice".to_string(), DslValue::String("overwrite".into()))])] {
        let result = verb(&mut local, &fixture, action, args).await;
        assert_eq!(rejected(&result), None, "{action}: {:?}", result.output);
    }
    assert_eq!(head(&local), (9, "c".to_string()), "the overwrite lands on the remote edit");
    relay_adopted(&mut local_probe, &mut remote_probe, &mut remote).await;
    assert_eq!(head(&remote), head(&local), "the other replica adopts the history edit");
    pump_until(&mut local, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    drop((local_probe, remote_probe));
    close(&mut local);
    close(&mut remote);
}

/// ⚖️ LAW (design §22.24, live fault F4): the base a session watches is the content revision of its store, never the
/// store's local generation. A backbone attaching and detaching while a draft is edited moves that generation and no
/// event: driver turns deliver no base move — the stage, the session generation, the base and the draft are what they
/// were — and an input stamped with the generation from before is accepted, not stale.
#[semio_framework_async_macros::async_test]
async fn a_backbone_attach_and_detach_while_editing_is_no_base_move() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } })] {
        let result = run_step(&mut app, &fixture, &step).await.expect("a verb result");
        assert_eq!(rejected(&result), None, "{step}: {:?}", result.output);
    }
    let before = app.time_travel.session().clone();
    let (store_generation, revision) = (app.store.generation(), app.store.content_revision());
    let (backbone, probe) = MemoryBackbone::pair("attach-while-editing", "attach-while-editing").await;
    app.attach_backbone(store::Backbones::Memory(backbone)).await.expect("the backbone attaches");
    app.detach_backbone().await.expect("the backbone detaches");
    assert!(app.store.generation() > store_generation && app.store.content_revision() == revision, "the port moved the store's local generation and no event");
    for _ in 0..4 {
        app.advance_typed_operation_publication().await.expect("a driver turn");
        while app.take_typed_operation_ui_progress().is_some() {}
    }
    assert_eq!(app.time_travel.session(), &before, "no base move: the stage, the generation, the base and the draft are what they were");
    let typed = verb(&mut app, &fixture, "historyEditInput", vec![("path".into(), DslValue::String("/value".into())), ("value".into(), DslValue::String("c".into())), ("generation".into(), DslValue::uint(u64::from(before.generation)))]).await;
    assert_eq!(rejected(&typed), None, "an input stamped before the port change is accepted");
    assert!(render_body(&mut app).await.contains("count=1 label=c"), "the preview follows the draft");
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    drop(probe);
    close(&mut app);
}
//#endregion 📡️RemoteEditWhileEditing

//#region 🚫️RowWithdraw
""",
    ),
]


def served(_root):
    """The schema, the fixture and the TS halves: saved under the `serve` lock as wave `d-serve` (rules 57 and 61)."""
    return {
        f"{FWT}/🧫️fixtures/🧫️lifecycle-law/🔣️.json": fwt_fixture,
        f"{FWT}/🧬️schema/🔣️.json": fwt_schema,
        f"{FWT}/🟦️.ts": FWT_TS,
        f"{FWT}/🧪️tests/🧪️conformance/🟦️.ts": FWT_CONFORMANCE,
        f"{PLUGIN}/🧪️tests/🧪️time-travel/🟦️.ts": PLUGIN_ORACLE,
    }


def files(_root):
    return {f"{FWT}/🦀️.rs": FWT_RS, f"{FWT}/🧪️tests/🔬️unit/🦀️.rs": FWT_UNIT, TT: TT_RS, PLG: PLG_RS, LAW: LAW_RS}
