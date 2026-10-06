"""🧨️ Wave I (tests only; coordinator 09:36 item 4): the remaining §22 runtime items verified by law.

- §22.5 + §22.17 `an_inverse_refusal_is_one_mutations_fatal_that_its_row_names_and_resolves`: an upstream edit under which a
  downstream operation has no inverse leaves the Report replay complete — one `Fatal` `mutation.inverse-refused` on that
  mutation, the operation after it still replayed, the report blocking — its row reads "Fatal: Cannot be reversed" /
  "Kritisch: Nicht umkehrbar" (words, never the code), and withdrawing it finalizes. `InertLabelOp`'s children kind refuses
  its inverse under a count below -1 (the existing law edits to -1 and is untouched by it).
- §22.6 `hostile_history_edit_input_is_answered_never_panicked_on`: every history-edit verb with missing, unknown and
  wrongly typed arguments, without a session and inside one — an answer every time, the session and the document untouched.
- The two `InertLabelOp` laws share their seed and their teardown (`inert_document`, `close_inert`).

Loaded by `🧪️s5-runtime-land.py`; independent of wave H.
"""

OSM = "🧰️framework/🛍️products/💻️os/🔨️modules"
LAW = f"{OSM}/🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs"

DOC_OLD = """/// 🪵️ A document operation over the toy kinds with two kinds an editor cannot repair — the label kind, whose payload
/// schema hides its only input and which refuses to apply on a negative count, and the children kind, which declares no
/// input schema: the smallest operations an upstream edit breaks without inputs to edit.
"""
DOC_NEW = """/// 🪵️ A document operation over the toy kinds with two kinds an editor cannot repair — the label kind, whose payload
/// schema hides its only input and which refuses to apply on a negative count, and the children kind, which declares no
/// input schema and has no inverse under a count below -1: the smallest operations an upstream edit breaks without inputs
/// to edit.
"""

INVERSE_OLD = """    fn inverse(&self, base: &TestSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(store::Mutation::<TestSnapshot>::inverse(&self.0, base)?.into_iter().map(Self).collect())
    }

    fn conflict_target(&self) -> Vec<String> {
        store::Mutation::<TestSnapshot>::conflict_target(&self.0)
    }

    fn may_emit_foreign_steps(&self) -> bool {
        false
    }
"""
INVERSE_NEW = """    fn inverse(&self, base: &TestSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        if matches!(self.0, TestMutation::SetSlotChildren(_)) && base.count < -1 {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "children have no inverse under a count below -1"));
        }
        Ok(store::Mutation::<TestSnapshot>::inverse(&self.0, base)?.into_iter().map(Self).collect())
    }

    fn conflict_target(&self) -> Vec<String> {
        store::Mutation::<TestSnapshot>::conflict_target(&self.0)
    }

    fn may_emit_foreign_steps(&self) -> bool {
        false
    }
"""

USE_OLD = """    use time_travel::{TimeTravelActionRefusal, TimeTravelCommit, TimeTravelOwners, TimeTravelStoreCommand, TimeTravelStoreOutput, TimeTravelStoreState};
"""
USE_NEW = """    use time_travel::{TimeTravelActionRefusal, TimeTravelCommit, TimeTravelStoreCommand, TimeTravelStoreOutput, TimeTravelStoreState};
"""

SETUP_START = """    let schema = "semio.test.inert-label/v1";
    let genesis = store::create_document_envelope::<TestSnapshot, InertLabelOp>(schema, "inert-label", TestSnapshot::default(), None);
"""
SETUP_END = """    let (count, label, children) = (ids[0].clone(), ids[1].clone(), ids[2].clone());
"""
HIDDEN_ASSERT_HEAD = "    assert!(semio_framework::mutation_input_defs(HIDDEN_ONLY_SCHEMA, "
TAIL_START = """    owners.settle(&document, TimeTravelStage::Inactive).expect("the owners settle");
"""
TAIL_END = """    panic!("the document store did not close");
}
//#endregion 🚫️RowWithdraw
"""
LAW_HEAD = """/// ⚖️ LAW (design §22.1, §22.20): a blocking mutation without editable inputs is always resolvable. A mutation whose
"""

HELPERS = """/// 🌰️ The schema id of the [`InertLabelOp`] documents.
const INERT_SCHEMA: &str = "semio.test.inert-label/v1";

/// 🥊️ A document store over [`InertLabelOp`] holding count 1, label a, no children and count 5 — one edit each, authored
/// by `local` — with the mutation ids of those four operations.
async fn inert_document() -> (ArtifactStore<TestSnapshot, InertLabelOp>, Vec<MutationId>) {
    let genesis = store::create_document_envelope::<TestSnapshot, InertLabelOp>(INERT_SCHEMA, "inert-label", TestSnapshot::default(), None);
    let mut document = Box::pin(ArtifactStore::new(genesis)).await.expect("the document store");
    document.install_document_store_owners_exact(bounded_document_store_owners::<TestSnapshot, InertLabelOp>());
    document.set_local_actor_id(Some("local".to_string())).expect("local actor");
    for operation in [TestMutation::SetCount(SetCount { value: 1 }), TestMutation::SetLabel(SetLabel { value: "a".into() }), TestMutation::SetSlotChildren(SetSlotChildren { children: Vec::new() }), TestMutation::SetCount(SetCount { value: 5 })] {
        Box::pin(document.dispatch(ArtifactCommand::Apply { mutations: vec![InertLabelOp(operation)], transaction: None })).await.expect("a seed edit applies");
    }
    let ids = document.mutation_ops().expect("applied operations").iter().map(|op| op.mutation_id.clone()).collect();
    (document, ids)
}

/// 🪤️ Settles and retires the history-edit owners of an [`inert_document`], then closes it under its exact grant.
fn close_inert(mut owners: time_travel::TimeTravelStoreState<TestSnapshot, InertLabelOp>, mut document: ArtifactStore<TestSnapshot, InertLabelOp>) {
    use time_travel::TimeTravelOwners;
"""

NEW_LAWS = """
/// ⚖️ LAW (design §22.5, §22.17): one inverse failure is one mutation's fatal, never a faulted session. An upstream edit
/// under which a downstream operation has no inverse leaves the Report replay complete: that operation's outcome is one
/// `Fatal` `mutation.inverse-refused`, the operation after it is still replayed and applies, and the report blocks
/// finalizing. Its row says so in words — "Fatal: Cannot be reversed" / "Kritisch: Nicht umkehrbar", never the code — and
/// is the way out: withdrawing it, with the other operation the same edit broke, leaves a report that finalizes as one
/// overwrite.
#[semio_framework_async_macros::async_test]
async fn an_inverse_refusal_is_one_mutations_fatal_that_its_row_names_and_resolves() {
    use time_travel::{TimeTravelCommit, TimeTravelStoreCommand, TimeTravelStoreOutput, TimeTravelStoreState};
    let (mut document, ids) = inert_document().await;
    let (count, label, children, tail) = (ids[0].clone(), ids[1].clone(), ids[2].clone(), ids[3].clone());
    let mut owners = TimeTravelStoreState::<TestSnapshot, InertLabelOp>::new(|_| LocalizedLabel::data("operation"));
    let lower = protocol::InputReplacement::Input { schema: INERT_SCHEMA.to_string(), payload: ::protocol::OpBinary::encode_op(&InertLabelOp(TestMutation::SetCount(SetCount { value: -2 }))).expect("the edited count encodes") };
    let mut drafts = BTreeMap::from([(count.clone(), lower)]);
    let outcome_of = |report: &protocol::ReplayReport, id: &MutationId| report.outcomes.iter().find(|outcome| outcome.mutation_id == *id).cloned().expect("an outcome per replayed operation");
    let blocked = replayed_report(&mut owners, &mut document, &drafts, &count).await;
    let refused = outcome_of(&blocked, &children);
    assert_eq!((refused.worst, refused.messages.iter().map(|message| message.code.0.as_str()).collect::<Vec<_>>()), (Some(semio_framework_diagnostic::Severity::Fatal), vec!["mutation.inverse-refused"]), "the refused inverse is one fatal on its mutation: {blocked:?}");
    assert!(blocked.blocks_finalize() && outcome_of(&blocked, &tail).worst.is_none(), "the replay went on past the refusal and the report blocks: {blocked:?}");
    {
        let ops = document.mutation_ops().expect("applied operations");
        let unlabelled = HashMap::new();
        let view = time_travel::history_mutation_view_of::<TestSnapshot, InertLabelOp>(&ops[2], Some(&refused), &unlabelled, &|_: &InertLabelOp| LocalizedLabel::data("operation"), false, None);
        let row = time_travel::history_mutation_entry(&view, None);
        assert_eq!((time_travel::history_mutation_description(&row, Locale::En), time_travel::history_mutation_description(&row, Locale::De)), ("Fatal: Cannot be reversed".to_string(), "Kritisch: Nicht umkehrbar".to_string()), "the row names the refusal in words, never the code");
        assert!(view.withdrawable && !view.editable, "the row offers the way out");
    }

    let TimeTravelStoreOutput::Opened(Ok(_)) = owners.run(&mut document, TimeTravelStoreCommand::Open { target: children.clone(), current: Some(protocol::InputReplacement::Withdrawn), withdraw: true }).await.expect("the open runs") else {
        panic!("the store's supersede law admits withdrawing the operation without an inverse");
    };
    owners.run(&mut document, TimeTravelStoreCommand::Adopt(true)).await.expect("the session adopts the kind");
    drafts.insert(children.clone(), protocol::InputReplacement::Withdrawn);
    let half = replayed_report(&mut owners, &mut document, &drafts, &count).await;
    assert!(half.blocks_finalize() && outcome_of(&half, &children).withdrawn && outcome_of(&half, &children).worst.is_none(), "withdrawn, the refusal is gone and the other broken operation still blocks: {half:?}");
    drafts.insert(label.clone(), protocol::InputReplacement::Withdrawn);
    let ready = replayed_report(&mut owners, &mut document, &drafts, &count).await;
    assert!(!ready.blocks_finalize(), "every broken operation withdrawn, the report finalizes: {ready:?}");
    let committed = owners.run(&mut document, TimeTravelStoreCommand::Commit { drafts: drafts.clone(), finalization: store::HistoryFinalization::Overwrite, actor: None }).await.expect("the commit runs");
    assert!(matches!(committed, TimeTravelStoreOutput::Committed(TimeTravelCommit::Finalized { .. })), "the finished replay commits as one overwrite");
    let head = document.snapshot().expect("head");
    assert_eq!((head.count, head.label.as_str()), (5, ""), "the head holds the edit and skips the withdrawn operations");
    close_inert(owners, document);
}

/// ⚖️ LAW (design §22.6): the user-input path answers, it never panics. Every history-edit verb with missing, unknown
/// and wrongly typed arguments is answered — a result or a fault with a code — without a session, where the stage, the
/// document and its revision stay what they were, and inside one that edits a draft, where the session stays what it was.
#[semio_framework_async_macros::async_test]
async fn hostile_history_edit_input_is_answered_never_panicked_on() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    let text_arg = |key: &str, value: &str| (key.to_string(), DslValue::String(value.to_string()));
    let garbage = || vec![("mutationId".to_string(), DslValue::uint(7)), ("path".to_string(), DslValue::uint(3)), ("value".to_string(), DslValue::Null), ("generation".to_string(), DslValue::String("x".into())), text_arg("choice", "sideways"), text_arg("name", ""), text_arg("store", "no-such-store")];
    let answered = |action: &str, answer: Result<InvocationResult, Fault>| {
        if let Err(fault) = answer {
            assert!(!fault.code.0.is_empty(), "{action}: a refusal carries its code: {fault:?}");
        }
    };

    let before = (head(&app), app.store.content_revision_now());
    for action in semio_framework::HISTORY_EDIT_ACTION_IDS {
        for args in [Vec::new(), vec![text_arg("mutationId", "no-such-mutation")], vec![("mutationId".to_string(), DslValue::uint(7))], vec![text_arg("mutationId", "no-such-mutation"), text_arg("store", "no-such-store")], garbage()] {
            let answer = app.handle_action(action, Some(&DslValue::Object(args.clone())), &meta(&fixture)).await;
            answered(action, answer);
            assert_eq!((app.time_travel.session().stage, head(&app), app.store.content_revision_now()), (TimeTravelStage::Inactive, before.0.clone(), before.1), "{action} {args:?}: nothing opened and nothing changed");
        }
    }

    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } })] {
        let result = run_step(&mut app, &fixture, &step).await.expect("a verb result");
        assert_eq!(rejected(&result), None, "{step}: {:?}", result.output);
    }
    let session = app.time_travel.session().clone();
    let far = u64::from(session.generation) + 4_000_000;
    let hostile: Vec<(&str, Vec<(String, DslValue)>)> = vec![
        ("historyEditBegin", vec![text_arg("mutationId", "no-such-mutation")]),
        ("historyEditBegin", vec![("mutationId".to_string(), DslValue::uint(7))]),
        ("historyEditBegin", garbage()),
        ("historyEditWithdraw", vec![text_arg("mutationId", "no-such-mutation")]),
        ("historyEditRestore", Vec::new()),
        ("historyEditRestore", vec![text_arg("mutationId", "no-such-mutation")]),
        ("historyEditInput", Vec::new()),
        ("historyEditInput", vec![text_arg("path", "/no/such/input"), ("value".to_string(), DslValue::Null)]),
        ("historyEditInput", vec![("path".to_string(), DslValue::uint(3)), ("value".to_string(), DslValue::uint(3))]),
        ("historyEditInput", vec![text_arg("path", "/value"), text_arg("value", "c"), ("generation".to_string(), DslValue::uint(far))]),
        ("historyEditInput", garbage()),
        ("historyEditCommit", vec![text_arg("choice", "sideways")]),
        ("historyEditFinalize", Vec::new()),
        ("historyEditRerun", Vec::new()),
        ("historyEditCancelReplay", Vec::new()),
    ];
    for (action, args) in hostile {
        let answer = app.handle_action(action, Some(&DslValue::Object(args.clone())), &meta(&fixture)).await;
        answered(action, answer);
        assert_eq!(app.time_travel.session(), &session, "{action} {args:?}: the session is what it was");
    }
    assert!(render_body(&mut app).await.contains("count=1 label=b"), "the draft preview stands");
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
}
"""


def change(text):
    for old, new in [(DOC_OLD, DOC_NEW), (INVERSE_OLD, INVERSE_NEW), (USE_OLD, USE_NEW)]:
        if text.count(old) != 1:
            raise SystemExit(f"wave i: anchor occurs {text.count(old)} times, expected 1:\n{old[:160]}")
        text = text.replace(old, new)
    for marker in (SETUP_START, SETUP_END, TAIL_START, TAIL_END, LAW_HEAD):
        if text.count(marker) != 1:
            raise SystemExit(f"wave i: marker occurs {text.count(marker)} times, expected 1:\n{marker[:160]}")
    start, end = text.index(SETUP_START), text.index(SETUP_END) + len(SETUP_END)
    hidden = [line for line in text[start:end].splitlines(keepends=True) if line.startswith(HIDDEN_ASSERT_HEAD)]
    if len(hidden) != 1:
        raise SystemExit(f"wave i: the hidden-schema assertion occurs {len(hidden)} times in the setup, expected 1")
    setup = '    let schema = INERT_SCHEMA;\n    let (mut document, ids) = inert_document().await;\n' + hidden[0] + SETUP_END
    text = text[:start] + setup + text[end:]
    start, end = text.index(TAIL_START), text.index(TAIL_END) + len(TAIL_END)
    teardown = text[start:end][: -len("//#endregion 🚫️RowWithdraw\n")]
    text = text[:start] + "    close_inert(owners, document);\n}\n" + NEW_LAWS + "//#endregion 🚫️RowWithdraw\n" + text[end:]
    head = text.index(LAW_HEAD)
    return text[:head] + HELPERS + teardown + "\n" + text[head:]


def files(_root):
    return {LAW: change}
