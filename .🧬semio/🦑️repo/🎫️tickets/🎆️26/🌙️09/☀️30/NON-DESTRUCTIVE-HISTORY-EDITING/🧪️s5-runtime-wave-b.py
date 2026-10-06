"""🚫️ Wave B (design §22.1 and §22.6, pure session): `beginWithdrawn` — a history row's Withdraw opens (or retargets) the
session on a `Withdrawn` draft; `restore` — a row's Restore takes one accepted draft back (coordinator decision 00:5x);
plus the refusal vocabulary the runtime answers instead of panicking (`timeTravel.not-withdrawable`,
`timeTravel.editor-closed`) and the viewer reason `refusalReadOnly`.

Schema first: the lifecycle-law fixture is the generator's (`🧪️w1-b-generate-lifecycle-law.py`), the schema of record
counts it, then the Rust reducer, the TS twin and both law drivers. Loaded by `🧪️s5-runtime-land.py`.
"""

import importlib.util
import json
import pathlib

FWT = "🧰️framework/🔨️modules/⏪️time-travel"
HERE = pathlib.Path(__file__).resolve().parent

RUST = [
    (
        """//! - Every event except `Begin`, `BaseMoved` and `Exit` carries the session `generation` the host
//!   last observed; any other generation is `timeTravel.stale` (a silent no-op).
""",
        """//! - [`TimeTravelEvent::BeginWithdrawn`] is a history row's Withdraw: `Begin` whose pending draft starts as
//!   `Withdrawn`, legal exactly where `Begin` is; the host sends [`TimeTravelEvent::Withdraw`] instead for the
//!   mutation already being edited.
//! - [`TimeTravelEvent::Restore`] is a history row's Restore: it takes the accepted draft of one mutation back while
//!   `Reviewing` and replays the rest; taking the only one back leaves time travel with zero trace.
//! - Every event except `Begin`, `BeginWithdrawn`, `BaseMoved` and `Exit` carries the session `generation` the
//!   host last observed; any other generation is `timeTravel.stale` (a silent no-op).
""",
    ),
    (
        """/// 🧩️ Driver fault: the composed member store the session edits was closed mid-session.
pub const TIME_TRAVEL_MEMBER_GONE_CODE: &str = "timeTravel.member-gone";
""",
        """/// 🧩️ Driver fault: the composed member store the session edits was closed mid-session.
pub const TIME_TRAVEL_MEMBER_GONE_CODE: &str = "timeTravel.member-gone";

/// 🪨️ Host refusal: the store's supersede law does not admit withdrawing the mutation.
pub const TIME_TRAVEL_NOT_WITHDRAWABLE_CODE: &str = "timeTravel.not-withdrawable";

/// 🥅️ Host refusal: a draft verb arrived while no draft editor is open on the session's mutation.
pub const TIME_TRAVEL_EDITOR_CLOSED_CODE: &str = "timeTravel.editor-closed";
""",
    ),
    (
        """    Begin { target: TimeTravelTarget, original: InputReplacement },
    Draft { generation: u32, replacement: InputReplacement },
""",
        """    Begin { target: TimeTravelTarget, original: InputReplacement },
    BeginWithdrawn { target: TimeTravelTarget, original: InputReplacement },
    Draft { generation: u32, replacement: InputReplacement },
""",
    ),
    (
        """    Discard { generation: u32 },
    ReplayProgressed { generation: u32, done: u32, total: u32 },
""",
        """    Discard { generation: u32 },
    Restore { generation: u32, target: MutationId },
    ReplayProgressed { generation: u32, done: u32, total: u32 },
""",
    ),
    (
        """            Self::Discard { .. } => K::Discard,
""",
        """            Self::Discard { .. } => K::Discard,
            Self::Restore { .. } => K::Restore,
""",
    ),
    (
        """            | Self::Discard { generation }
""",
        """            | Self::Discard { generation }
            | Self::Restore { generation, .. }
""",
    ),
    (
        """    Discard,
    ReplayProgressed,
    ReplayCompleted,
    ReplayCancelled,
    ReplayFaulted,
    Rerun,
    RequestFinalize,
    ChooseOverwrite,
""",
        """    Discard,
    Restore,
    ReplayProgressed,
    ReplayCompleted,
    ReplayCancelled,
    ReplayFaulted,
    Rerun,
    RequestFinalize,
    ChooseOverwrite,
""",
    ),
    (
        """        Self::Discard,
        Self::ReplayProgressed,
""",
        """        Self::Discard,
        Self::Restore,
        Self::ReplayProgressed,
""",
    ),
    (
        """            Self::Discard => "discard",
""",
        """            Self::Discard => "discard",
            Self::Restore => "restore",
""",
    ),
    (
        """            (S::Editing, E::Discard { .. }) => {
                let pending = self.pending.take().ok_or(TimeTravelRefusal::Illegal)?;
                Ok(self.resume(pending.return_stage))
            }
""",
        """            (S::Editing, E::Discard { .. }) => {
                let pending = self.pending.take().ok_or(TimeTravelRefusal::Illegal)?;
                Ok(self.resume(pending.return_stage))
            }
            (_, E::Restore { target, .. }) => match self.restore_refusal(&target) {
                Some(refusal) => Err(refusal),
                None => {
                    self.accepted.retain(|draft| draft.target.mutation != target);
                    Ok(if self.accepted.is_empty() { self.close(Vec::new()) } else { self.settle() })
                }
            },
""",
    ),
    (
        """    /// 🔁️ Why `Rerun` would be refused, `None` when it would start a replay: it needs `Reviewing`,
""",
        """    /// 🔙️ Why `Restore` of `mutation` would be refused, `None` when it would take the mutation's accepted draft back: it
    /// needs `Reviewing` and an accepted draft of that mutation (`Illegal` otherwise) — what a host disables a row's
    /// Restore control by.
    pub fn restore_refusal(&self, mutation: &MutationId) -> Option<TimeTravelRefusal> {
        (self.stage != TimeTravelStage::Reviewing || self.accepted_draft(mutation).is_none()).then_some(TimeTravelRefusal::Illegal)
    }

    /// 🔁️ Why `Rerun` would be refused, `None` when it would start a replay: it needs `Reviewing`,
""",
    ),
    (
        """            Self::Begin { .. } => K::Begin,
            Self::Draft { .. } => K::Draft,
""",
        """            Self::Begin { .. } => K::Begin,
            Self::BeginWithdrawn { .. } => K::BeginWithdrawn,
            Self::Draft { .. } => K::Draft,
""",
    ),
    (
        """    /// 🧿️ The session generation the event is addressed to; `None` for `Begin`, `BaseMoved`, `Exit`.
    pub fn generation(&self) -> Option<u32> {
        match self {
            Self::Begin { .. } | Self::BaseMoved { .. } | Self::Exit => None,
""",
        """    /// 🧿️ The session generation the event is addressed to; `None` for `Begin`, `BeginWithdrawn`, `BaseMoved`, `Exit`.
    pub fn generation(&self) -> Option<u32> {
        match self {
            Self::Begin { .. } | Self::BeginWithdrawn { .. } | Self::BaseMoved { .. } | Self::Exit => None,
""",
    ),
    (
        """pub enum TimeTravelEventKey {
    Begin,
    Draft,
""",
        """pub enum TimeTravelEventKey {
    Begin,
    BeginWithdrawn,
    Draft,
""",
    ),
    (
        """    pub const ALL: [Self; 18] = [
        Self::Begin,
        Self::Draft,
""",
        """    pub const ALL: [Self; 20] = [
        Self::Begin,
        Self::BeginWithdrawn,
        Self::Draft,
""",
    ),
    (
        """            Self::Begin => "begin",
            Self::Draft => "draft",
""",
        """            Self::Begin => "begin",
            Self::BeginWithdrawn => "beginWithdrawn",
            Self::Draft => "draft",
""",
    ),
    (
        """            (S::Inactive, E::Begin { target, original }) => {
                self.id = self.id.wrapping_add(1);
                Ok(self.begin(target, original, S::Inactive))
            }
            (S::Reviewing, E::Begin { target, original }) => Ok(self.begin(target, original, S::Reviewing)),
            (S::Editing, E::Begin { target, original }) => {
                let pending = self.pending.as_ref().ok_or(TimeTravelRefusal::Illegal)?;
                if !self.unchanged(pending) {
                    return Err(TimeTravelRefusal::Blocked);
                }
                let return_stage = pending.return_stage;
                Ok(self.begin(target, original, return_stage))
            }
""",
        """            (_, E::Begin { target, original }) => self.open(target, original, None),
            (_, E::BeginWithdrawn { target, original }) => self.open(target, original, Some(InputReplacement::Withdrawn)),
""",
    ),
    (
        """    /// ✏️ Why `Begin` would be refused, `None` when it would open (or switch) the draft editor: it is legal from `Inactive`
    /// and `Reviewing`, from `Editing` only while the pending draft still equals its start (`Blocked` otherwise), and
    /// `Illegal` while replaying, choosing or finalizing — what a host disables an Edit control by.
""",
        """    /// ✏️ Why `Begin` (and `BeginWithdrawn`) would be refused, `None` when it would open (or switch) the draft editor: it is
    /// legal from `Inactive` and `Reviewing`, from `Editing` only while the pending draft still equals its start (`Blocked`
    /// otherwise), and `Illegal` while replaying, choosing or finalizing — what a host disables a row's Edit and Withdraw
    /// controls by.
""",
    ),
    (
        """    fn begin(&mut self, target: TimeTravelTarget, original: InputReplacement, return_stage: TimeTravelStage) -> Vec<TimeTravelEffect> {
        let replacement = self.accepted_draft(&target.mutation).map_or_else(|| original.clone(), |draft| draft.replacement.clone());
        let effect = TimeTravelEffect::ShowPreview { target: target.mutation.clone(), replacement: replacement.clone() };
        self.stage = TimeTravelStage::Editing;
        self.generation = self.generation.wrapping_add(1);
        self.pending = Some(TimeTravelPending { target, original, replacement, return_stage });
        vec![effect]
    }
""",
        """    /// 🔦️ Opens (or retargets) the draft editor on `target` where [`Self::begin_refusal`] admits it: the pending draft starts
    /// as `draft`, else as the target's accepted draft, else as `original`, and returns to the stage the session was opened
    /// from; a session opened from `Inactive` is the next one.
    fn open(&mut self, target: TimeTravelTarget, original: InputReplacement, draft: Option<InputReplacement>) -> Result<Vec<TimeTravelEffect>, TimeTravelRefusal> {
        if let Some(refusal) = self.begin_refusal() {
            return Err(refusal);
        }
        let return_stage = self.pending.as_ref().map_or(self.stage, |pending| pending.return_stage);
        if self.stage == TimeTravelStage::Inactive {
            self.id = self.id.wrapping_add(1);
        }
        let replacement = draft.or_else(|| self.accepted_draft(&target.mutation).map(|accepted| accepted.replacement.clone())).unwrap_or_else(|| original.clone());
        let effect = TimeTravelEffect::ShowPreview { target: target.mutation.clone(), replacement: replacement.clone() };
        self.stage = TimeTravelStage::Editing;
        self.generation = self.generation.wrapping_add(1);
        self.pending = Some(TimeTravelPending { target, original, replacement, return_stage });
        Ok(vec![effect])
    }
""",
    ),
    (
        """pub const TIME_TRAVEL_CODE_LABELS: [(&str, TimeTravelLabel); 18] = [""",
        """pub const TIME_TRAVEL_CODE_LABELS: [(&str, TimeTravelLabel); 20] = [""",
    ),
    (
        """    (TIME_TRAVEL_MEMBER_GONE_CODE, TimeTravelLabel::RefusalMemberGone),
];
""",
        """    (TIME_TRAVEL_MEMBER_GONE_CODE, TimeTravelLabel::RefusalMemberGone),
    (TIME_TRAVEL_NOT_WITHDRAWABLE_CODE, TimeTravelLabel::RefusalNotWithdrawable),
    (TIME_TRAVEL_EDITOR_CLOSED_CODE, TimeTravelLabel::RefusalEditorClosed),
];
""",
    ),
    (
        """    RefusalMemberGone,
    MemberEdited,
}
""",
        """    RefusalMemberGone,
    MemberEdited,
    RefusalNotWithdrawable,
    RefusalEditorClosed,
    RefusalReadOnly,
}
""",
    ),
    (
        """    pub const ALL: [Self; 37] = [""",
        """    pub const ALL: [Self; 40] = [""",
    ),
    (
        """        Self::RefusalMemberGone,
        Self::MemberEdited,
    ];
""",
        """        Self::RefusalMemberGone,
        Self::MemberEdited,
        Self::RefusalNotWithdrawable,
        Self::RefusalEditorClosed,
        Self::RefusalReadOnly,
    ];
""",
    ),
    (
        """            Self::MemberEdited => ("memberEdited", "History of a composed part edited", "Verlauf eines eingebetteten Teils bearbeitet"),
""",
        """            Self::MemberEdited => ("memberEdited", "History of a composed part edited", "Verlauf eines eingebetteten Teils bearbeitet"),
            Self::RefusalNotWithdrawable => ("refusalNotWithdrawable", "This mutation cannot be withdrawn here", "Diese Mutation kann hier nicht zurückgezogen werden"),
            Self::RefusalEditorClosed => ("refusalEditorClosed", "The draft editor is closed: open the mutation again", "Der Entwurfseditor ist geschlossen: die Mutation erneut öffnen"),
            Self::RefusalReadOnly => ("refusalReadOnly", "History cannot be edited in a read-only view", "Der Verlauf kann in einer schreibgeschützten Ansicht nicht bearbeitet werden"),
""",
    ),
]

RUST_UNIT = [
    (
        """        "begin" => TimeTravelEvent::Begin { target: target(&value["target"]), original: replacement(&value["original"]) },
""",
        """        "begin" => TimeTravelEvent::Begin { target: target(&value["target"]), original: replacement(&value["original"]) },
        "beginWithdrawn" => TimeTravelEvent::BeginWithdrawn { target: target(&value["target"]), original: replacement(&value["original"]) },
""",
    ),
    (
        """        "discard" => TimeTravelEvent::Discard { generation: generation() },
""",
        """        "discard" => TimeTravelEvent::Discard { generation: generation() },
        "restore" => TimeTravelEvent::Restore { generation: generation(), target: mutation(&value["target"]) },
""",
    ),
    (
        """    assert_eq!(rows.len(), 130);
""",
        """    assert_eq!(rows.len(), 145);
""",
    ),
    (
        """/// ✏️ `begin_refusal` answers, in every fixture context, exactly what applying the canonical `Begin` answers — the query a
/// host disables its Edit control by never disagrees with the reducer.
#[test]
fn begin_is_refused_exactly_where_the_reducer_refuses_it() {
    let law = law();
    for (name, context) in law["contexts"].as_object().expect("contexts") {
        let mut applied = session(context);
        let refusal = applied.begin_refusal();
        assert_eq!(refusal, applied.apply(event(&law["events"]["begin"])).err(), "{name}");
    }
}
""",
        """/// ✏️ `begin_refusal` answers, in every fixture context, exactly what applying the canonical `Begin` and the canonical
/// `BeginWithdrawn` answer — the query a host disables a row's Edit and Withdraw controls by never disagrees with the
/// reducer.
#[test]
fn begin_is_refused_exactly_where_the_reducer_refuses_it() {
    let law = law();
    for (name, context) in law["contexts"].as_object().expect("contexts") {
        for key in ["begin", "beginWithdrawn"] {
            let mut applied = session(context);
            let refusal = applied.begin_refusal();
            assert_eq!(refusal, applied.apply(event(&law["events"][key])).err(), "{name} × {key}");
        }
    }
}

/// 🚫️ A row's Withdraw opens the session exactly as `Begin` does, with the pending draft withdrawn: same stage, generation,
/// session id, return stage, target and original in every context that admits it.
#[test]
fn begin_withdrawn_is_begin_with_a_withdrawn_draft() {
    let law = law();
    for (name, context) in law["contexts"].as_object().expect("contexts") {
        let (mut begun, mut withdrawn) = (session(context), session(context));
        if begun.apply(event(&law["events"]["begin"])).is_err() {
            continue;
        }
        let effects = withdrawn.apply(event(&law["events"]["beginWithdrawn"])).unwrap_or_else(|refusal| panic!("{name}: refused {refusal}"));
        let target = withdrawn.pending.as_ref().expect("a pending draft").target.mutation.clone();
        assert_eq!(effects, vec![TimeTravelEffect::ShowPreview { target, replacement: InputReplacement::Withdrawn }], "{name}: the preview is the withdrawal");
        begun.pending.as_mut().expect("a pending draft").replacement = InputReplacement::Withdrawn;
        assert_eq!(withdrawn, begun, "{name}");
    }
}

/// 🔙️ `restore_refusal` answers, in every fixture context and for every mutation, exactly what applying `Restore` of that
/// mutation answers — the query a host disables a row's Restore control by never disagrees with the reducer; an admitted
/// restore leaves exactly the other accepted drafts.
#[test]
fn restore_is_refused_exactly_where_the_reducer_refuses_it() {
    let law = law();
    for (name, context) in law["contexts"].as_object().expect("contexts") {
        for id in ["a", "b", "c"] {
            let before = session(context);
            let mut applied = before.clone();
            let target = MutationId(id.to_string());
            let refusal = before.restore_refusal(&target);
            assert_eq!(refusal, applied.apply(TimeTravelEvent::Restore { generation: before.generation, target: target.clone() }).err(), "{name} × {id}");
            if refusal.is_none() {
                let kept: Vec<&MutationId> = before.accepted.iter().map(|draft| &draft.target.mutation).filter(|mutation| **mutation != target).collect();
                assert_eq!(applied.accepted.iter().map(|draft| &draft.target.mutation).collect::<Vec<_>>(), kept, "{name} × {id}: the other drafts stay");
                assert_eq!(applied.stage, if kept.is_empty() { TimeTravelStage::Inactive } else { TimeTravelStage::Replaying }, "{name} × {id}");
            }
        }
    }
}
""",
    ),
]

TS = [
    (
        """ * Driver contract: `begin` carries the target position resolved at the session base; every store
 * generation change is sent as `baseMoved` with the re-resolved positions of every session target
 * (in every stage; an unchanged base is `timeTravel.stale`); every other event except `begin` and
 * `exit` carries the session generation.""",
        """ * Driver contract: `begin` carries the target position resolved at the session base; `beginWithdrawn`
 * is a history row's Withdraw (`begin` whose pending draft starts withdrawn, legal exactly where
 * `begin` is); `restore` is a row's Restore (it takes one accepted draft back while `reviewing`); every
 * store generation change is sent as `baseMoved` with the re-resolved positions of every session
 * target (in every stage; an unchanged base is `timeTravel.stale`); every other event except `begin`,
 * `beginWithdrawn` and `exit` carries the session generation.""",
    ),
    (
        """/** 🧩️ Driver fault: the composed member store the session edits was closed mid-session. */
export const TIME_TRAVEL_MEMBER_GONE_CODE = "timeTravel.member-gone";
""",
        """/** 🧩️ Driver fault: the composed member store the session edits was closed mid-session. */
export const TIME_TRAVEL_MEMBER_GONE_CODE = "timeTravel.member-gone";

/** 🪨️ Host refusal: the store's supersede law does not admit withdrawing the mutation. */
export const TIME_TRAVEL_NOT_WITHDRAWABLE_CODE = "timeTravel.not-withdrawable";

/** 🥅️ Host refusal: a draft verb arrived while no draft editor is open on the session's mutation. */
export const TIME_TRAVEL_EDITOR_CLOSED_CODE = "timeTravel.editor-closed";
""",
    ),
    (
        """  | { readonly type: "begin"; readonly target: TimeTravelTarget; readonly original: InputReplacement }
""",
        """  | { readonly type: "begin" | "beginWithdrawn"; readonly target: TimeTravelTarget; readonly original: InputReplacement }
""",
    ),
    (
        """  | { readonly type: "replayProgressed"; readonly generation: number; readonly done: number; readonly total: number }
""",
        """  | { readonly type: "restore"; readonly generation: number; readonly target: string }
  | { readonly type: "replayProgressed"; readonly generation: number; readonly done: number; readonly total: number }
""",
    ),
    (
        """  "discard",
  "replayProgressed",
""",
        """  "discard",
  "restore",
  "replayProgressed",
""",
    ),
    (
        """/** 🔁️ Why `rerun` would be refused, `null` when it would start a replay: it needs `reviewing`, accepted drafts, and either no report or a fault. */
""",
        """/** 🔙️ Why `restore` of `mutation` would be refused, `null` when it would take the mutation's accepted draft back: it needs
 * `reviewing` and an accepted draft of that mutation — what a host disables a row's Restore control by. Mirrors Rust
 * `TimeTravelSession::restore_refusal`. */
export function timeTravelRestoreRefusal(session: TimeTravelSession, mutation: string): TimeTravelRefusal | null {
  return session.stage === "reviewing" && timeTravelAcceptedDraft(session, mutation) !== undefined ? null : "timeTravel.illegal";
}

/** 🔁️ Why `rerun` would be refused, `null` when it would start a replay: it needs `reviewing`, accepted drafts, and either no report or a fault. */
""",
    ),
    (
        """    case "discard":
      return stage === "editing" && pending !== null ? admit(resume({ ...session, pending: null }, pending.returnStage)) : refuse("timeTravel.illegal");
""",
        """    case "discard":
      return stage === "editing" && pending !== null ? admit(resume({ ...session, pending: null }, pending.returnStage)) : refuse("timeTravel.illegal");
    case "restore": {
      const refusal = timeTravelRestoreRefusal(session, event.target);
      if (refusal !== null) return refuse(refusal);
      const accepted = session.accepted.filter((draft) => draft.target.mutation !== event.target);
      return admit(accepted.length === 0 ? close(session, []) : settle({ ...session, accepted }));
    }
""",
    ),
    (
        """export const TIME_TRAVEL_EVENT_KEYS = [
  "begin",
  "draft",
""",
        """export const TIME_TRAVEL_EVENT_KEYS = [
  "begin",
  "beginWithdrawn",
  "draft",
""",
    ),
    (
        """/** 🧿️ The generation an event is addressed to; `null` for `begin`, `baseMoved` and `exit`. */""",
        """/** 🧿️ The generation an event is addressed to; `null` for `begin`, `beginWithdrawn`, `baseMoved` and `exit`. */""",
    ),
    (
        """/** ✏️ Why `begin` would be refused, `null` when it would open (or switch) the draft editor: legal from `inactive` and
 * `reviewing`, from `editing` only while the pending draft still equals its start (`timeTravel.blocked` otherwise), and
 * `timeTravel.illegal` while replaying, choosing or finalizing. Mirrors Rust `TimeTravelSession::begin_refusal`. */""",
        """/** ✏️ Why `begin` (and `beginWithdrawn`) would be refused, `null` when it would open (or switch) the draft editor: legal
 * from `inactive` and `reviewing`, from `editing` only while the pending draft still equals its start
 * (`timeTravel.blocked` otherwise), and `timeTravel.illegal` while replaying, choosing or finalizing — what a host
 * disables a row's Edit and Withdraw controls by. Mirrors Rust `TimeTravelSession::begin_refusal`. */""",
    ),
    (
        """function begin(session: TimeTravelSession, target: TimeTravelTarget, original: InputReplacement, returnStage: TimeTravelStage, id: bigint): Step {
  const replacement = timeTravelAcceptedDraft(session, target.mutation)?.replacement ?? original;
  return { session: { ...session, id, stage: "editing", generation: bump(session.generation), pending: { target, original, replacement, returnStage } }, effects: [{ type: "showPreview", target: target.mutation, replacement }] };
}

""",
        "",
    ),
    (
        """const refuse = (rejection: TimeTravelRefusal): TimeTravelApplyResult => ({ ok: false, rejection });
const admit = (step: Step): TimeTravelApplyResult => ({ ok: true, session: step.session, effects: step.effects });
""",
        """const refuse = (rejection: TimeTravelRefusal): TimeTravelApplyResult => ({ ok: false, rejection });
const admit = (step: Step): TimeTravelApplyResult => ({ ok: true, session: step.session, effects: step.effects });

/** 🔦️ Opens (or retargets) the draft editor on `target` where {@link timeTravelBeginRefusal} admits it: the pending draft
 * starts as `draft`, else as the target's accepted draft, else as `original`, and returns to the stage the session was
 * opened from; a session opened from `inactive` is the next one. */
function open(session: TimeTravelSession, target: TimeTravelTarget, original: InputReplacement, draft: InputReplacement | null): TimeTravelApplyResult {
  const refusal = timeTravelBeginRefusal(session);
  if (refusal !== null) return refuse(refusal);
  const returnStage = session.pending?.returnStage ?? session.stage;
  const id = session.stage === "inactive" ? (session.id + 1n) & 0xffff_ffff_ffff_ffffn : session.id;
  const replacement = draft ?? timeTravelAcceptedDraft(session, target.mutation)?.replacement ?? original;
  return admit({ session: { ...session, id, stage: "editing", generation: bump(session.generation), pending: { target, original, replacement, returnStage } }, effects: [{ type: "showPreview", target: target.mutation, replacement }] });
}
""",
    ),
    (
        """    case "begin":
      if (stage === "inactive") return admit(begin(session, event.target, event.original, "inactive", (session.id + 1n) & 0xffff_ffff_ffff_ffffn));
      if (stage === "reviewing") return admit(begin(session, event.target, event.original, "reviewing", session.id));
      if (stage === "editing" && pending !== null) return timeTravelUnchanged(session, pending) ? admit(begin(session, event.target, event.original, pending.returnStage, session.id)) : refuse("timeTravel.blocked");
      return refuse("timeTravel.illegal");
""",
        """    case "begin":
      return open(session, event.target, event.original, null);
    case "beginWithdrawn":
      return open(session, event.target, event.original, { kind: "withdrawn" });
""",
    ),
    (
        """    case "begin":
      return { type: "begin", target: { ...json.target }, original: json.original };
""",
        """    case "begin":
    case "beginWithdrawn":
      return { type: json.type, target: { ...json.target }, original: json.original };
""",
    ),
    (
        """  memberEdited: { en: "History of a composed part edited", de: "Verlauf eines eingebetteten Teils bearbeitet" },
""",
        """  memberEdited: { en: "History of a composed part edited", de: "Verlauf eines eingebetteten Teils bearbeitet" },
  refusalNotWithdrawable: { en: "This mutation cannot be withdrawn here", de: "Diese Mutation kann hier nicht zurückgezogen werden" },
  refusalEditorClosed: { en: "The draft editor is closed: open the mutation again", de: "Der Entwurfseditor ist geschlossen: die Mutation erneut öffnen" },
  refusalReadOnly: { en: "History cannot be edited in a read-only view", de: "Der Verlauf kann in einer schreibgeschützten Ansicht nicht bearbeitet werden" },
""",
    ),
    (
        """  [TIME_TRAVEL_MEMBER_GONE_CODE, "refusalMemberGone"],
""",
        """  [TIME_TRAVEL_MEMBER_GONE_CODE, "refusalMemberGone"],
  [TIME_TRAVEL_NOT_WITHDRAWABLE_CODE, "refusalNotWithdrawable"],
  [TIME_TRAVEL_EDITOR_CLOSED_CODE, "refusalEditorClosed"],
""",
    ),
]

TS_CONFORMANCE = [
    (
        """  test("begin is refused exactly where the reducer refuses it, in every context", () => {
    for (const [name, json] of Object.entries(law.contexts)) {
      const session = M.timeTravelSessionFromJson(json);
      const result = M.applyTimeTravel(session, M.timeTravelEventFromJson(law.events.begin));
      expect(M.timeTravelBeginRefusal(session), name).toBe(result.ok ? null : result.rejection);
    }
  });
""",
        """  test("begin and beginWithdrawn are refused exactly where the reducer refuses them, in every context", () => {
    for (const [name, json] of Object.entries(law.contexts)) {
      const session = M.timeTravelSessionFromJson(json);
      for (const key of ["begin", "beginWithdrawn"]) {
        const result = M.applyTimeTravel(session, M.timeTravelEventFromJson(law.events[key]));
        expect(M.timeTravelBeginRefusal(session), `${name} × ${key}`).toBe(result.ok ? null : result.rejection);
      }
    }
  });

  test("beginWithdrawn is begin with a withdrawn draft, in every context that admits it", () => {
    for (const [name, json] of Object.entries(law.contexts)) {
      const session = M.timeTravelSessionFromJson(json);
      const begun = M.applyTimeTravel(session, M.timeTravelEventFromJson(law.events.begin));
      if (!begun.ok) continue;
      const withdrawn = M.applyTimeTravel(session, M.timeTravelEventFromJson(law.events.beginWithdrawn));
      expect(withdrawn.ok, name).toBe(true);
      if (!withdrawn.ok) continue;
      const expected = M.timeTravelSessionToJson(begun.session);
      expected.pending.replacement = { kind: "withdrawn" };
      expect(M.timeTravelSessionToJson(withdrawn.session), name).toEqual(expected);
      expect(withdrawn.effects.map(M.timeTravelEffectToJson), name).toEqual([{ type: "showPreview", target: expected.pending.target.mutation, replacement: { kind: "withdrawn" } }]);
    }
  });
""",
    ),
    (
        """  const begun = (model: Model, event: OracleEvent, returnStage: string, id: bigint): Model => ({
    ...model,
    id,
    generation: model.generation + 1,
    pending: { mutation: event.mutation!, position: event.position!, original: event.original!, value: model.accepted.find((draft) => draft.mutation === event.mutation)?.value ?? event.original!, returnStage },
  });
""",
        """  const begun = (model: Model, event: OracleEvent, row: any, draft: Key | null): Model => ({
    ...model,
    id: row.from === "inactive" ? model.id + 1n : model.id,
    generation: model.generation + 1,
    pending: {
      mutation: event.mutation!,
      position: event.position!,
      original: event.original!,
      value: draft ?? model.accepted.find((accepted) => accepted.mutation === event.mutation)?.value ?? event.original!,
      returnStage: row.from === "editing" ? model.pending!.returnStage : row.from,
    },
  });
""",
    ),
    (
        """    begin: (model, event, row) => begun(model, event, row.from === "editing" ? model.pending!.returnStage : row.from, row.from === "inactive" ? model.id + 1n : model.id),
""",
        """    begin: (model, event, row) => begun(model, event, row, null),
    beginWithdrawn: (model, event, row) => begun(model, event, row, "withdrawn"),
""",
    ),
    (
        """    discard: (model) => resumed(model, model.pending!.returnStage),
""",
        """    discard: (model) => resumed(model, model.pending!.returnStage),
    restore: (model, event) => {
      const accepted = model.accepted.filter((draft) => draft.mutation !== event.mutation);
      return accepted.length === 0 ? cleared(model) : replayed({ ...model, accepted });
    },
""",
    ),
    (
        """    validName: (_, event) => fitsBytes(event.name!) && codePoints(event.name!).some((point) => !WHITE_SPACE.has(point)),
""",
        """    validName: (_, event) => fitsBytes(event.name!) && codePoints(event.name!).some((point) => !WHITE_SPACE.has(point)),
    restoreKeepsDrafts: (model, event) => model.accepted.length > 1 && model.accepted.some((draft) => draft.mutation === event.mutation),
    restoreLastDraft: (model, event) => model.accepted.length === 1 && model.accepted[0]!.mutation === event.mutation,
""",
    ),
    (
        """      case "replayProgressed":
        return { type, generation: event.generation, value: `${event.done}/${event.total}` };
""",
        """      case "restore":
        return { type, generation: event.generation, mutation: event.target };
      case "replayProgressed":
        return { type, generation: event.generation, value: `${event.done}/${event.total}` };
""",
    ),
    (
        """            case "replayProgressed":
              return { type: "replayProgressed", generation, done: choice.done, total: 4 };
""",
        """            case "restore":
              return { type: "restore", generation, target: choice.mutation === "accepted" ? (session.accepted[0]?.target.mutation ?? "a") : choice.mutation };
            case "replayProgressed":
              return { type: "replayProgressed", generation, done: choice.done, total: 4 };
""",
    ),
    (
        """  test("stale generations are silent no-ops in every context", () => {
""",
        """  test("restore is refused exactly where the reducer refuses it, for every mutation in every context", () => {
    for (const [name, json] of Object.entries(law.contexts)) {
      const session = M.timeTravelSessionFromJson(json);
      for (const target of ["a", "b", "c"]) {
        const result = M.applyTimeTravel(session, { type: "restore", generation: session.generation, target });
        expect(M.timeTravelRestoreRefusal(session, target), `${name} × ${target}`).toBe(result.ok ? null : result.rejection);
        if (!result.ok) continue;
        const kept = session.accepted.map((draft) => draft.target.mutation).filter((mutation) => mutation !== target);
        expect([result.session.accepted.map((draft) => draft.target.mutation), result.session.stage], `${name} × ${target}`).toEqual([kept, kept.length === 0 ? "inactive" : "replaying"]);
      }
    }
  });

  test("stale generations are silent no-ops in every context", () => {
""",
    ),
    (
        """      case "begin":
        return { type, mutation: event.target.mutation, position: event.target.position, original: keyOf(event.original) };
""",
        """      case "begin":
      case "beginWithdrawn":
        return { type, mutation: event.target.mutation, position: event.target.position, original: keyOf(event.original) };
""",
    ),
    (
        """            case "begin": {
              const mutation = choice.mutation === "accepted" ? (session.accepted[0]?.target.mutation ?? "a") : choice.mutation;
              return { type: "begin", target: { mutation, position: positions[mutation]! }, original: originals[mutation]! };
            }
""",
        """            case "begin":
            case "beginWithdrawn": {
              const mutation = choice.mutation === "accepted" ? (session.accepted[0]?.target.mutation ?? "a") : choice.mutation;
              return { type: choice.key, target: { mutation, position: positions[mutation]! }, original: originals[mutation]! };
            }
""",
    ),
]


def schema(text):
    document = json.loads(text)
    defs = document["$defs"]
    begin = defs["TimeTravelEvent"]["oneOf"][0]["properties"]["type"]
    assert begin == {"const": "begin"}, begin
    defs["TimeTravelEvent"]["oneOf"][0]["properties"]["type"] = {"enum": ["begin", "beginWithdrawn"]}
    keys = defs["TimeTravelEventKey"]["enum"]
    assert keys[0] == "begin" and "beginWithdrawn" not in keys
    keys.insert(1, "beginWithdrawn")
    keys.insert(keys.index("discard") + 1, "restore")
    events = defs["TimeTravelEvent"]["oneOf"]
    events.insert(len(events) - 1, {"type": "object", "additionalProperties": False, "required": ["type", "generation", "target"], "properties": {"type": {"const": "restore"}, "generation": {"$ref": "#/$defs/U32"}, "target": {"$ref": "#/$defs/MutationId"}}})
    guards = defs["TimeTravelGuard"]["enum"]
    assert guards[-1] == "invalidName", guards[-1]
    guards.extend(["restoreKeepsDrafts", "restoreLastDraft", "restoreUnaccepted"])
    labels = defs["TimeTravelLabelKey"]["enum"]
    assert labels[-1] == "memberEdited"
    labels.extend(["refusalNotWithdrawable", "refusalEditorClosed", "refusalReadOnly"])
    fixture = defs["LifecycleLawFixture"]["properties"]
    for name, bounds, before, after in [
        ("eventKeys", ("minItems", "maxItems"), 18, 20),
        ("events", ("minProperties", "maxProperties"), 18, 20),
        ("guards", ("minItems", "maxItems"), 24, 27),
        ("matrix", ("minItems", "maxItems"), 130, 145),
        ("labels", ("minItems", "maxItems"), 37, 40),
        ("codeLabels", ("minItems", "maxItems"), 18, 20),
    ]:
        for bound in bounds:
            assert fixture[name][bound] == before, (name, bound, fixture[name][bound])
            fixture[name][bound] = after
    description = fixture["matrix"]["description"]
    assert "6 × 18 pairs, 130 rows" in description
    fixture["matrix"]["description"] = description.replace("6 × 18 pairs, 130 rows", "6 × 20 pairs, 145 rows")
    return json.dumps(document, indent=2, ensure_ascii=False) + "\n"


def fixture(_text):
    spec = importlib.util.spec_from_file_location("lifecycle_law", HERE / "🧪️w1-b-generate-lifecycle-law.py")
    generator = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(generator)
    return generator.TEXT


def files(_root):
    return {
        f"{FWT}/🧫️fixtures/🧫️lifecycle-law/🔣️.json": fixture,
        f"{FWT}/🧬️schema/🔣️.json": schema,
        f"{FWT}/🦀️.rs": RUST,
        f"{FWT}/🟦️.ts": TS,
        f"{FWT}/🧪️tests/🔬️unit/🦀️.rs": RUST_UNIT,
        f"{FWT}/🧪️tests/🧪️conformance/🟦️.ts": TS_CONFORMANCE,
    }
