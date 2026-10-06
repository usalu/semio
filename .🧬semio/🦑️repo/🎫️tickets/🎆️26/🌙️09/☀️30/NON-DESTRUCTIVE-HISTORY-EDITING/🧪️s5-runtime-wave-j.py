"""🪹️ Wave J (design §22.33, coordinator 10:2x: names approved): Accept needs a change.

- Session law (pure reducer, Rust + TS twin + schema + lifecycle corpus): `Accept` of a pending draft that still equals what
  it was opened with — the applied input, or the draft accepted for that mutation before — is refused `timeTravel.unchanged`
  (new refusal `Unchanged`, label `refusalUnchanged`, en + de); it used to resume like a discard. `accept_refusal()` /
  `timeTravelAcceptRefusal` is the query hosts disable Accept by. "Unchanged" is the reducer's own start-based notion
  (`unchanged`, the one `begin_refusal` already reads): re-opening an accepted draft and setting it back to the original
  input IS a change (it takes the accepted draft back), exactly as the corpus case "accepting the original of one of two
  drafts replays from the remaining one" says.
- Wire: `HistoryTimeTravel.draftChanged: bool` (omitted when false) — true iff `Accept` would be admitted now; kernel field,
  `history-patch` schema, TS twin, fixture (one valid, one invalid case).
- Guest: the status carries it; the draft editor's Accept row is disabled with the localized reason while it is false; the
  verb answers the session's refusal code.
- Laws: reducer unit law + TS conformance (`accept` is refused exactly where the reducer refuses it); plugin law
  `accept_needs_a_change_and_says_so_on_the_wire_the_row_and_the_verb`.

`files` = Rust (landing lock); `served` = TS, JSON schema / fixtures (serve lock, wave `j-serve`). Loaded by
`🧪️s5-runtime-land.py`. The lifecycle corpus is produced by `🧪️w1-b-generate-lifecycle-law.py` (edited with this wave).
"""

import importlib.util
import pathlib

HERE = pathlib.Path(__file__).resolve().parent
FW = "🧰️framework/🔨️modules"
FWT = f"{FW}/⏪️time-travel"
KERNEL = f"{FW}/🎠️kernel"
OSM = "🧰️framework/🛍️products/💻️os/🔨️modules"
PLUGIN = f"{OSM}/🔌️plugin"
TT = f"{PLUGIN}/⏪️time-travel/🦀️.rs"
LAW = f"{PLUGIN}/🧪️tests/🧪️time-travel/🦀️.rs"

FWT_RS = [
    (
        """    /// 🫙️ `RequestFinalize` or `Rerun` with no accepted draft.
    Empty,
}
""",
        """    /// 🫙️ `RequestFinalize` or `Rerun` with no accepted draft.
    Empty,
    /// 🪹️ `Accept` of a pending draft that still equals what it was opened with.
    Unchanged,
}
""",
    ),
    (
        """    pub const ALL: [Self; 4] = [Self::Illegal, Self::Stale, Self::Blocked, Self::Empty];
""",
        """    pub const ALL: [Self; 5] = [Self::Illegal, Self::Stale, Self::Blocked, Self::Empty, Self::Unchanged];
""",
    ),
    (
        """            Self::Empty => "timeTravel.empty",
""",
        """            Self::Empty => "timeTravel.empty",
            Self::Unchanged => "timeTravel.unchanged",
""",
    ),
    (
        """            Self::Empty => TimeTravelLabel::RefusalEmpty,
""",
        """            Self::Empty => TimeTravelLabel::RefusalEmpty,
            Self::Unchanged => TimeTravelLabel::RefusalUnchanged,
""",
    ),
    (
        """            (S::Editing, E::Accept { .. }) => {
                let pending = self.pending.take().ok_or(TimeTravelRefusal::Illegal)?;
                Ok(self.accept(pending))
            }
""",
        """            (S::Editing, E::Accept { .. }) => match self.accept_refusal() {
                Some(refusal) => Err(refusal),
                None => {
                    let pending = self.pending.take().ok_or(TimeTravelRefusal::Illegal)?;
                    Ok(self.accept(pending))
                }
            },
""",
    ),
    (
        """    /// 🔙️ Why `Restore` of `mutation` would be refused, `None` when it would take the mutation's accepted draft back: it
""",
        """    /// ☑️ Why `Accept` would be refused, `None` when it would accept the pending draft: it needs `Editing` (`Illegal`
    /// otherwise) and a draft that differs from what it was opened with — the applied input, or the draft accepted for
    /// that mutation before (`Unchanged` otherwise) — what a host disables Accept by.
    pub fn accept_refusal(&self) -> Option<TimeTravelRefusal> {
        match (self.stage, self.pending.as_ref()) {
            (TimeTravelStage::Editing, Some(pending)) if self.unchanged(pending) => Some(TimeTravelRefusal::Unchanged),
            (TimeTravelStage::Editing, Some(_)) => None,
            _ => Some(TimeTravelRefusal::Illegal),
        }
    }

    /// 🔙️ Why `Restore` of `mutation` would be refused, `None` when it would take the mutation's accepted draft back: it
""",
    ),
    (
        """    fn accept(&mut self, pending: TimeTravelPending) -> Vec<TimeTravelEffect> {
        if self.unchanged(&pending) {
            return self.resume(pending.return_stage);
        }
""",
        """    fn accept(&mut self, pending: TimeTravelPending) -> Vec<TimeTravelEffect> {
""",
    ),
    (
        """pub const TIME_TRAVEL_CODE_LABELS: [(&str, TimeTravelLabel); 20] = [
""",
        """pub const TIME_TRAVEL_CODE_LABELS: [(&str, TimeTravelLabel); 21] = [
""",
    ),
    (
        """    ("timeTravel.empty", TimeTravelLabel::RefusalEmpty),
""",
        """    ("timeTravel.empty", TimeTravelLabel::RefusalEmpty),
    ("timeTravel.unchanged", TimeTravelLabel::RefusalUnchanged),
""",
    ),
    (
        """    RefusalEmpty,
    Frozen,
""",
        """    RefusalEmpty,
    RefusalUnchanged,
    Frozen,
""",
    ),
    (
        """    pub const ALL: [Self; 40] = [
""",
        """    pub const ALL: [Self; 41] = [
""",
    ),
    (
        """        Self::RefusalEmpty,
        Self::Frozen,
""",
        """        Self::RefusalEmpty,
        Self::RefusalUnchanged,
        Self::Frozen,
""",
    ),
    (
        """            Self::RefusalEmpty => ("refusalEmpty", "Nothing to finalize: no accepted changes", "Nichts abzuschließen: keine übernommenen Änderungen"),
""",
        """            Self::RefusalEmpty => ("refusalEmpty", "Nothing to finalize: no accepted changes", "Nichts abzuschließen: keine übernommenen Änderungen"),
            Self::RefusalUnchanged => ("refusalUnchanged", "Nothing to accept: the draft is unchanged", "Nichts zu übernehmen: der Entwurf ist unverändert"),
""",
    ),
]

FWT_UNIT = [
    (
        """#[test]
fn text_limits_hold_at_their_edges() {
""",
        """/// ☑️ `accept_refusal` answers, in every fixture context, exactly what applying `Accept` answers — the query a host
/// disables Accept by never disagrees with the reducer — and it is `Unchanged` exactly while a draft is open that still
/// equals what it was opened with.
#[test]
fn accept_is_refused_exactly_where_the_reducer_refuses_it() {
    let law = law();
    for (name, context) in law["contexts"].as_object().expect("contexts") {
        let before = session(context);
        let mut applied = before.clone();
        let refusal = before.accept_refusal();
        assert_eq!(refusal, applied.apply(TimeTravelEvent::Accept { generation: before.generation }).err(), "{name}");
        assert_eq!(refusal == Some(TimeTravelRefusal::Unchanged), before.pending.as_ref().is_some_and(|pending| before.unchanged(pending)), "{name}: unchanged is the open draft's own state");
        if refusal.is_some() {
            assert_eq!(applied, before, "{name}: a refused accept leaves the session untouched");
        }
    }
}

#[test]
fn text_limits_hold_at_their_edges() {
""",
    ),
]

KERNEL_RS = [
    (
        """/// many drafts are accepted, what a review shows, whether a replay can be run again and the first mutation that blocks
/// finalizing.
""",
        """/// many drafts are accepted, what a review shows, whether a replay can be run again, the first mutation that blocks
/// finalizing and whether the open draft changed.
""",
    ),
    (
        """    pub next_problem: Option<HistoryTimeTravelProblem>,
}
""",
        """    pub next_problem: Option<HistoryTimeTravelProblem>,
    /// 🪹️ Whether the open draft differs from what it was opened with — the applied input, or the draft accepted for that
    /// mutation before: `historyEditAccept` is admitted exactly while it is true and answers `timeTravel.unchanged`
    /// while it is false.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    #[value(default, skip_serializing_if = "std::ops::Not::not")]
    pub draft_changed: bool,
}
""",
    ),
]

TT_RS = [
    (
        """            next_problem: self.next_problem().map(|mutation_id| HistoryTimeTravelProblem { mutation_id, store: self.member_store() }),
        })
""",
        """            next_problem: self.next_problem().map(|mutation_id| HistoryTimeTravelProblem { mutation_id, store: self.member_store() }),
            draft_changed: session.accept_refusal().is_none(),
        })
""",
    ),
    (
        """/// schema that describes none its reason), then Accept, Discard and Withdraw — and `framework.history.editor.inputs`,
""",
        """/// schema that describes none its reason), then Accept (disabled, naming why, while the draft is unchanged: design
/// §22.33), Discard and Withdraw — and `framework.history.editor.inputs`,
""",
    ),
    (
        """    rows.try_push(time_travel_button_row(controller_id, &format!("{scope}.accept"), HistoryPanelText::Accept.text(locale), "check", HISTORY_EDIT_ACCEPT_ACTION_ID, Some(time_travel_generation_args(generation)?), true)?)
        .map_err(|_| error("time-travel-panel.editor-rows"))?;
""",
        """    let accept = format!("{scope}.accept");
    let unchanged = (!editor.changed).then(|| TimeTravelRefusal::Unchanged.label().localized(LocalizedLabel::native).resolve(Terminology::Native, locale).to_string());
    let accept_button = time_travel_button(controller_id, &accept, HistoryPanelText::Accept.text(locale), "check", HISTORY_EDIT_ACCEPT_ACTION_ID, Some(time_travel_generation_args(generation)?), editor.changed)?;
    rows.try_push(time_travel_control_row(&format!("{accept}.row"), HistoryPanelText::Accept.text(locale), "check", accept_button, editor.changed, None, unchanged.as_deref())?)
        .map_err(|_| error("time-travel-panel.editor-rows"))?;
""",
    ),
]

LAW_RS = [
    (
        """//#region 🎚️HistoryFilter
""",
        """//#region 🪹️AcceptNeedsAChange
/// ⚖️ LAW (design §22.33): Accept needs a change. While the open draft equals what it was opened with, the wire status
/// omits `draftChanged`, the editor's Accept row is disabled and names why in the shell's language, and the verb answers
/// `timeTravel.unchanged` leaving the session untouched; one changed input turns all three, setting it back turns them
/// again, and the accepted draft replays.
#[semio_framework_async_macros::async_test]
async fn accept_needs_a_change_and_says_so_on_the_wire_the_row_and_the_verb() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    let wire = |app: &ToyApp| serde_json::to_value(app.time_travel.status().expect("an open session")).expect("the status serializes");
    let input = |value: &str| vec![("path".to_string(), DslValue::String("/value".into())), ("value".to_string(), DslValue::String(value.to_string()))];
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 1 })).await;
    for (value, changed) in [(None, false), (Some("b"), true), (Some("a"), false)] {
        if let Some(value) = value {
            assert_eq!(rejected(&verb(&mut app, &fixture, "historyEditInput", input(value)).await), None, "the draft takes {value}");
        }
        assert_eq!((app.time_travel.status().expect("an open session").draft_changed, wire(&app).get("draftChanged").cloned()), (changed, changed.then_some(Value::Bool(true))), "{value:?}: the wire names a changed draft and omits an unchanged one");
        for (locale, reason) in [(Locale::En, "Nothing to accept: the draft is unchanged"), (Locale::De, "Nichts zu übernehmen: der Entwurf ist unverändert")] {
            let history = render_history(&mut app, locale).await;
            let (button, row) = (find_node(&history, "framework.history.editor.accept").expect("the Accept button"), find_node(&history, "framework.history.editor.accept.row").expect("the Accept row"));
            assert_eq!((button["disabled"] == Value::Bool(true), row.to_string().contains(reason)), (!changed, !changed), "{value:?} {locale:?}: Accept is disabled with its reason exactly while nothing changed: {row}");
        }
    }
    let before = app.time_travel.session().clone();
    let refused = verb(&mut app, &fixture, "historyEditAccept", Vec::new()).await;
    assert_eq!((rejected(&refused), app.time_travel.session()), (Some("timeTravel.unchanged"), &before), "the verb answers what the row says and leaves the session untouched");
    assert_eq!(rejected(&verb(&mut app, &fixture, "historyEditInput", input("b")).await), None);
    let accepted = verb(&mut app, &fixture, "historyEditAccept", Vec::new()).await;
    assert_eq!((rejected(&accepted), app.time_travel.session().stage), (None, TimeTravelStage::Replaying), "a changed draft is accepted and replays");
    assert_eq!(wire(&app).get("draftChanged"), None, "outside the editor no draft is open");
    pump_until(&mut app, "the replay completes", |app| app.time_travel.session().stage != TimeTravelStage::Replaying).await;
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
}
//#endregion 🪹️AcceptNeedsAChange

//#region 🎚️HistoryFilter
""",
    ),
]

FWT_TS = [
    (
        """export const TIME_TRAVEL_REFUSALS = ["timeTravel.illegal", "timeTravel.stale", "timeTravel.blocked", "timeTravel.empty"] as const;
""",
        """export const TIME_TRAVEL_REFUSALS = ["timeTravel.illegal", "timeTravel.stale", "timeTravel.blocked", "timeTravel.empty", "timeTravel.unchanged"] as const;
""",
    ),
    (
        """/** 🔙️ Why `restore` of `mutation` would be refused, `null` when it would take the mutation's accepted draft back: it needs
""",
        """/** ☑️ Why `accept` would be refused, `null` when it would accept the pending draft: it needs `editing`
 * (`timeTravel.illegal` otherwise) and a draft that differs from what it was opened with — the applied input, or the draft
 * accepted for that mutation before (`timeTravel.unchanged` otherwise) — what a host disables Accept by. Mirrors Rust
 * `TimeTravelSession::accept_refusal`. */
export function timeTravelAcceptRefusal(session: TimeTravelSession): TimeTravelRefusal | null {
  if (session.stage !== "editing" || session.pending === null) return "timeTravel.illegal";
  return timeTravelUnchanged(session, session.pending) ? "timeTravel.unchanged" : null;
}

/** 🔙️ Why `restore` of `mutation` would be refused, `null` when it would take the mutation's accepted draft back: it needs
""",
    ),
    (
        """function accept(session: TimeTravelSession, pending: TimeTravelPending): Step {
  if (timeTravelUnchanged(session, pending)) return resume(session, pending.returnStage);
""",
        """function accept(session: TimeTravelSession, pending: TimeTravelPending): Step {
""",
    ),
    (
        """    case "accept":
      return stage === "editing" && pending !== null ? admit(accept({ ...session, pending: null }, pending)) : refuse("timeTravel.illegal");
""",
        """    case "accept": {
      const refusal = timeTravelAcceptRefusal(session);
      return refusal === null && pending !== null ? admit(accept({ ...session, pending: null }, pending)) : refuse(refusal ?? "timeTravel.illegal");
    }
""",
    ),
    (
        """  refusalEmpty: { en: "Nothing to finalize: no accepted changes", de: "Nichts abzuschließen: keine übernommenen Änderungen" },
""",
        """  refusalEmpty: { en: "Nothing to finalize: no accepted changes", de: "Nichts abzuschließen: keine übernommenen Änderungen" },
  refusalUnchanged: { en: "Nothing to accept: the draft is unchanged", de: "Nichts zu übernehmen: der Entwurf ist unverändert" },
""",
    ),
    (
        """  ["timeTravel.empty", "refusalEmpty"],
""",
        """  ["timeTravel.empty", "refusalEmpty"],
  ["timeTravel.unchanged", "refusalUnchanged"],
""",
    ),
]

FWT_CONFORMANCE = [
    (
        """    pendingUnchanged: (model) => unchanged(model),
    unchangedReturnInactive: (model) => unchanged(model) && model.pending!.returnStage === "inactive",
    unchangedReturnReviewing: (model) => unchanged(model) && model.pending!.returnStage === "reviewing" && !reportLost(model),
    unchangedReturnReplaying: (model) => unchanged(model) && model.pending!.returnStage === "reviewing" && reportLost(model),
""",
        """    pendingUnchanged: (model) => unchanged(model),
""",
    ),
    (
        """    accept: (model) => (unchanged(model) ? resumed(model, model.pending!.returnStage) : replayed({ ...model, accepted: acceptedAfter(model), pending: null })),
""",
        """    accept: (model) => replayed({ ...model, accepted: acceptedAfter(model), pending: null }),
""",
    ),
    (
        """  test("stale generations are silent no-ops in every context", () => {
""",
        """  test("accept is refused exactly where the reducer refuses it, and unchanged is the open draft's own state", () => {
    for (const [name, json] of Object.entries(law.contexts)) {
      const session = M.timeTravelSessionFromJson(json);
      const result = M.applyTimeTravel(session, { type: "accept", generation: session.generation });
      const refusal = M.timeTravelAcceptRefusal(session);
      expect(refusal, name).toBe(result.ok ? null : result.rejection);
      expect(refusal === "timeTravel.unchanged", name).toBe(session.pending !== null && M.timeTravelUnchanged(session, session.pending));
    }
  });

  test("stale generations are silent no-ops in every context", () => {
""",
    ),
]

KERNEL_TS = [
    (
        """  /** 👉️ The first mutation whose replay outcome blocks finalizing; present exactly while `blocking`. */
  readonly nextProblem?: HistoryTimeTravelProblem;
};
""",
        """  /** 👉️ The first mutation whose replay outcome blocks finalizing; present exactly while `blocking`. */
  readonly nextProblem?: HistoryTimeTravelProblem;
  /** 🪹️ Whether the open draft differs from what it was opened with (absent: unchanged): `historyEditAccept` is admitted
   * exactly while it is true and answers `timeTravel.unchanged` otherwise. */
  readonly draftChanged?: boolean;
};
""",
    ),
]

KERNEL_SCHEMA = [
    (
        """        "nextProblem": { "$ref": "#/definitions/HistoryTimeTravelProblem" }
""",
        """        "nextProblem": { "$ref": "#/definitions/HistoryTimeTravelProblem" },
        "draftChanged": { "type": "boolean" }
""",
    ),
]

KERNEL_FIXTURE = [
    (
        """ "blocking": false, "acceptedCount": 0 }
""",
        """ "blocking": false, "acceptedCount": 0, "draftChanged": true }
""",
    ),
    (
        """ "nextProblem": "e-3#0" } }, "reason": "a next problem is its mutation id and store, the arguments of historyEditBegin" }
""",
        """ "nextProblem": "e-3#0" } }, "reason": "a next problem is its mutation id and store, the arguments of historyEditBegin" },
    { "id": "draft-changed-not-a-boolean", "patch": { "cursor": 1, "timeTravel": { "sessionId": "1", "generation": 0, "stage": "editing", "draftChanged": "yes" } }, "reason": "whether the open draft changed is a boolean" }
""",
    ),
]


def fwt_schema(text):
    pairs = [
        (
            """        "timeTravel.blocked",
        "timeTravel.empty"
      ]
""",
            """        "timeTravel.blocked",
        "timeTravel.empty",
        "timeTravel.unchanged"
      ]
""",
        ),
        (
            """        "pendingUnchanged",
        "pendingChanged",
        "unchangedReturnInactive",
        "unchangedReturnReviewing",
        "unchangedReturnReplaying",
""",
            """        "pendingUnchanged",
        "pendingChanged",
""",
        ),
        (
            """        "refusalEmpty",
""",
            """        "refusalEmpty",
        "refusalUnchanged",
""",
        ),
        (
            """        "refusals": {
          "type": "array",
          "minItems": 4,
          "maxItems": 4,
""",
            """        "refusals": {
          "type": "array",
          "minItems": 5,
          "maxItems": 5,
""",
        ),
        (
            """        "guards": {
          "type": "array",
          "minItems": 27,
          "maxItems": 27,
""",
            """        "guards": {
          "type": "array",
          "minItems": 24,
          "maxItems": 24,
""",
        ),
        (
            """split into guard branches: 6 × 20 pairs, 145 rows.""",
            """split into guard branches: 6 × 20 pairs, 143 rows.""",
        ),
        (
            """          "minItems": 145,
          "maxItems": 145,
""",
            """          "minItems": 143,
          "maxItems": 143,
""",
        ),
        (
            """        "labels": {
          "type": "array",
          "minItems": 40,
          "maxItems": 40,
""",
            """        "labels": {
          "type": "array",
          "minItems": 41,
          "maxItems": 41,
""",
        ),
        (
            """        "codeLabels": {
          "type": "array",
          "minItems": 20,
          "maxItems": 20,
""",
            """        "codeLabels": {
          "type": "array",
          "minItems": 21,
          "maxItems": 21,
""",
        ),
    ]
    for old, new in pairs:
        if text.count(old) != 1:
            raise SystemExit(f"wave j: schema anchor occurs {text.count(old)} times, expected 1:\n{old}")
        text = text.replace(old, new)
    return text


def fwt_fixture(_text):
    spec = importlib.util.spec_from_file_location("lifecycle_law", HERE / "🧪️w1-b-generate-lifecycle-law.py")
    generator = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(generator)
    return generator.TEXT


def served(_root):
    """The schema, the fixtures and the TS halves: saved under the `serve` lock as wave `j-serve` (rule 61)."""
    return {
        f"{FWT}/🧫️fixtures/🧫️lifecycle-law/🔣️.json": fwt_fixture,
        f"{FWT}/🧬️schema/🔣️.json": fwt_schema,
        f"{FWT}/🟦️.ts": FWT_TS,
        f"{FWT}/🧪️tests/🧪️conformance/🟦️.ts": FWT_CONFORMANCE,
        f"{KERNEL}/🟦️.ts": KERNEL_TS,
        f"{KERNEL}/🧬️schema/🔣️history-patch/🔣️.json": KERNEL_SCHEMA,
        f"{KERNEL}/🧫️fixtures/🧫️history-patch/🔣️.json": KERNEL_FIXTURE,
    }


def files(_root):
    return {f"{FWT}/🦀️.rs": FWT_RS, f"{FWT}/🧪️tests/🔬️unit/🦀️.rs": FWT_UNIT, f"{KERNEL}/🦀️.rs": KERNEL_RS, TT: TT_RS, LAW: LAW_RS}
