//! 🧪️ Incremental evaluation tests: unit behaviour and the law "incremental evaluation equals full evaluation on random mutation sequences".

use super::*;
use crate::{AnnexChoice, ClauseId, LocalizedCopy, Quantity, SubjectRef};
use std::sync::atomic::AtomicUsize;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub(crate) struct Section {
    pub id: String,
    pub capacity: f64,
    pub note: String,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub(crate) struct Member {
    pub id: String,
    pub section: String,
    pub load: f64,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub(crate) struct Doc {
    pub gamma: f64,
    pub label: String,
    pub sections: Vec<Section>,
    pub members: Vec<Member>,
}

pub(crate) static RUNS: AtomicUsize = AtomicUsize::new(0);

pub(crate) fn result(id: &str, demand: f64, capacity: f64) -> CheckResult {
    CheckResult::assess(id, "mock", ClauseId::new("EN 0", "1", "1"), SubjectRef::new(id, id, LocalizedCopy::new(id, id)), LocalizedCopy::new("Capacity", "Tragfähigkeit"))
        .utilization(Quantity::force_kn(demand), Quantity::force_kn(capacity))
        .annex(AnnexChoice::De)
        .build()
}

pub(crate) struct Mock;

impl UnitSource for Mock {
    type Document = Doc;

    fn units(document: &Doc) -> Vec<CheckUnit<Doc>> {
        let mut units = vec![CheckUnit::new(UnitId::new("global", "label"), vec![], ReadSet::new(["gamma"]), CostTier::T0, |doc: &Doc| vec![result("global", doc.gamma, 4.0)])];
        for (index, member) in document.members.iter().enumerate() {
            let id = member.id.clone();
            let section_index = document.sections.iter().position(|section| section.id == member.section);
            let mut reads = ReadSet::new([format!("members.{index}"), "gamma".to_owned()]);
            if let Some(section_index) = section_index {
                reads = reads.with(format!("sections.{section_index}.capacity"));
            }
            let tier = if index % 3 == 0 { CostTier::T2 } else { CostTier::T1 };
            units.push(CheckUnit::streaming(
                UnitId::new("member", &id),
                vec![EntityRef::new("members", &id)],
                reads,
                tier,
                move |doc: &Doc, progress, _| {
                    RUNS.fetch_add(1, Ordering::SeqCst);
                    progress.report(0.5);
                    let member = doc.members.iter().find(|m| m.id == id).expect("member");
                    let capacity = doc.sections.iter().find(|s| s.id == member.section).map_or(0.0, |s| s.capacity);
                    vec![result(&id, member.load * doc.gamma, capacity)]
                },
            ));
        }
        units
    }

    fn references(document: &Doc) -> Vec<EntityEdge> {
        document.members.iter().map(|m| EntityEdge { from: EntityRef::new("members", &m.id), to: EntityRef::new("sections", &m.section) }).collect()
    }
}

pub(crate) fn base() -> Doc {
    Doc {
        gamma: 1.35,
        label: "frame".into(),
        sections: (0..4).map(|i| Section { id: format!("s{i}"), capacity: 100.0 + i as f64, note: String::new() }).collect(),
        members: (0..12).map(|i| Member { id: format!("m{i}"), section: format!("s{}", i % 4), load: 10.0 + i as f64 }).collect(),
    }
}

struct Rng(u64);

impl Rng {
    fn next(&mut self, bound: usize) -> usize {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 % bound as u64) as usize
    }
}

fn mutate(doc: &mut Doc, rng: &mut Rng, serial: &mut usize) -> ChangedSet {
    match rng.next(7) {
        0 if !doc.members.is_empty() => {
            let i = rng.next(doc.members.len());
            doc.members[i].load += rng.next(20) as f64;
            ChangedSet::at([format!("members.{i}.load")], vec![])
        }
        1 => {
            let i = rng.next(doc.sections.len());
            doc.sections[i].capacity += rng.next(50) as f64 - 10.0;
            ChangedSet::at([format!("sections.{i}.capacity")], vec![EntityRef::new("sections", &doc.sections[i].id)])
        }
        2 => {
            doc.gamma += 0.05;
            ChangedSet::at(["gamma"], vec![])
        }
        3 => {
            *serial += 1;
            let section = doc.sections[rng.next(doc.sections.len())].id.clone();
            doc.members.push(Member { id: format!("n{serial}"), section, load: 5.0 });
            ChangedSet::at(["members"], vec![])
        }
        4 if doc.members.len() > 1 => {
            doc.members.remove(rng.next(doc.members.len()));
            ChangedSet::at(["members"], vec![])
        }
        5 if !doc.members.is_empty() => {
            let i = rng.next(doc.members.len());
            doc.members[i].section = doc.sections[rng.next(doc.sections.len())].id.clone();
            ChangedSet::at([format!("members.{i}.section")], vec![])
        }
        _ => {
            doc.label.push('x');
            ChangedSet::at(["label"], vec![])
        }
    }
}

fn settle(session: &mut EvaluationSession<Mock>, plan: &Plan) -> RunOutcome {
    session.run(plan, &mut |_| {})
}

#[test]
fn full_run_equals_reference_fold() {
    let mut session = EvaluationSession::<Mock>::open(base());
    let plan = session.plan(&ChangedSet::all(), &[], session.token());
    assert_eq!(plan.units.len(), 13);
    assert_eq!(settle(&mut session, &plan), RunOutcome::Completed);
    assert_eq!(session.report(), &evaluate_units::<Mock>(&base()));
    assert!(session.stale().is_empty());
}

#[test]
fn incremental_equals_full_on_random_mutation_sequences() {
    for seed in 1..=40u64 {
        let mut rng = Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
        let mut doc = base();
        let mut serial = 0;
        let mut session = EvaluationSession::<Mock>::open(doc.clone());
        let plan = session.plan(&ChangedSet::all(), &[], session.token());
        settle(&mut session, &plan);
        for step in 0..60 {
            let changed = mutate(&mut doc, &mut rng, &mut serial);
            let plan = session.update(doc.clone(), &changed, &[]);
            if step % 5 == 0 {
                session.cancel_handle().cancel();
                assert_eq!(settle(&mut session, &plan), RunOutcome::Cancelled);
                let retry = session.plan(&ChangedSet::default(), &[], session.token());
                assert_eq!(settle(&mut session, &retry), RunOutcome::Completed);
            } else {
                assert_eq!(settle(&mut session, &plan), RunOutcome::Completed);
            }
            assert_eq!(session.report(), &evaluate_units::<Mock>(&doc), "seed {seed} step {step}");
        }
    }
}

#[test]
fn single_field_edit_reruns_only_dependent_units() {
    let mut doc = base();
    let mut session = EvaluationSession::<Mock>::open(doc.clone());
    let plan = session.plan(&ChangedSet::all(), &[], session.token());
    settle(&mut session, &plan);
    doc.sections[1].capacity += 7.0;
    let plan = session.update(doc.clone(), &ChangedSet::at(["sections.1.capacity"], vec![EntityRef::new("sections", "s1")]), &[]);
    let mut reran = Vec::new();
    session.run(&plan, &mut |event| {
        if let SessionEvent::Done(done) = event {
            if done.rerun {
                reran.push(done.id.0);
            }
        }
    });
    reran.sort();
    assert_eq!(reran, vec!["member:m1", "member:m5", "member:m9"]);
    doc.label.push('!');
    let plan = session.update(doc.clone(), &ChangedSet::at(["label"], vec![]), &[]);
    assert!(plan.units.is_empty());
}

#[test]
fn unchanged_hash_skips_rerun_even_when_planned() {
    let mut session = EvaluationSession::<Mock>::open(base());
    let plan = session.plan(&ChangedSet::all(), &[], session.token());
    settle(&mut session, &plan);
    let before = session.revision();
    let plan = session.update(base(), &ChangedSet::all(), &[]);
    settle(&mut session, &plan);
    assert_eq!(session.revision(), before);
}

#[test]
fn plan_orders_cheap_tiers_first_then_focus() {
    let session = EvaluationSession::<Mock>::open(base());
    let focus = [EntityRef::new("members", "m5")];
    let plan = session.plan(&ChangedSet::all(), &focus, session.token());
    let tiers: Vec<CostTier> = plan.units.iter().map(|u| u.tier).collect();
    assert!(tiers.windows(2).all(|w| w[0] <= w[1]));
    assert_eq!(plan.units[0].tier, CostTier::T0);
    assert_eq!(plan.units[1].id.0, "member:m5");
}

#[test]
fn superseded_plan_is_cancelled_and_leaves_units_stale() {
    let mut session = EvaluationSession::<Mock>::open(base());
    let plan = session.plan(&ChangedSet::all(), &[], session.token());
    let handle = session.cancel_handle();
    let mut seen = 0;
    let outcome = session.run(&plan, &mut |event| {
        if matches!(event, SessionEvent::Done(_)) {
            seen += 1;
            if seen == 3 {
                handle.cancel();
            }
        }
    });
    assert_eq!(outcome, RunOutcome::Cancelled);
    assert!(session.progress().cancelled);
    assert_eq!(session.stale().len(), 10);
}

#[test]
fn progress_reports_every_unit_and_ticks() {
    let mut session = EvaluationSession::<Mock>::open(base());
    let plan = session.plan(&ChangedSet::all(), &[], session.token());
    let (mut ticks, mut last) = (0, EvalProgress::default());
    session.run(&plan, &mut |event| match event {
        SessionEvent::Tick { fraction, .. } => {
            assert_eq!(fraction, 0.5);
            ticks += 1;
        }
        SessionEvent::Progress(progress) => last = progress,
        SessionEvent::Done(_) => {}
    });
    assert_eq!(ticks, 12);
    assert_eq!((last.done, last.total, last.cancelled), (13, 13, false));
}

#[test]
fn entity_graph_closure_is_transitive() {
    let graph = EntityGraph::from_edges([
        EntityEdge { from: EntityRef::new("members", "a"), to: EntityRef::new("sections", "s") },
        EntityEdge { from: EntityRef::new("joints", "j"), to: EntityRef::new("members", "a") },
    ]);
    let closure = graph.closure(&[EntityRef::new("sections", "s")]);
    assert_eq!(closure.len(), 3);
    assert_eq!(graph.used_by(&EntityRef::new("sections", "s")).len(), 1);
}

#[test]
fn read_set_cover_and_overlap_with_wildcards() {
    let reads = ReadSet::new(["members.*.load", "gamma"]);
    assert!(reads.covers("members.3.load") && reads.covers("gamma") && !reads.covers("members.3.section"));
    assert!(reads.overlaps("members") && reads.overlaps("members.1.load") && !reads.overlaps("label"));
}

#[test]
fn read_audit_accepts_declared_and_flags_undeclared_reads() {
    let doc = base();
    for unit in Mock::units(&doc) {
        assert_eq!(audit_reads(&unit, &doc), Vec::<String>::new(), "{:?}", unit.id);
    }
    let leaky = CheckUnit::new(UnitId::new("leaky", "x"), vec![], ReadSet::new(["gamma"]), CostTier::T0, |doc: &Doc| vec![result("leaky", doc.gamma + doc.sections[0].capacity, 1.0)]);
    assert_eq!(audit_reads(&leaky, &doc), vec!["sections.0.capacity".to_owned()]);
}

#[test]
fn async_run_matches_sync_run() {
    use std::task::{RawWaker, RawWakerVTable, Waker};
    fn noop() -> Waker {
        fn clone(_: *const ()) -> RawWaker {
            RawWaker::new(std::ptr::null(), &VTABLE)
        }
        fn nothing(_: *const ()) {}
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, nothing, nothing, nothing);
        unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
    }
    let mut session = EvaluationSession::<Mock>::open(base());
    let plan = session.plan(&ChangedSet::all(), &[], session.token());
    let waker = noop();
    let mut cx = Context::from_waker(&waker);
    let mut sink = |_: SessionEvent| {};
    let mut future = Box::pin(session.run_async(&plan, &mut sink));
    let mut polls = 0;
    let outcome = loop {
        polls += 1;
        if let Poll::Ready(outcome) = future.as_mut().poll(&mut cx) {
            break outcome;
        }
    };
    drop(future);
    assert_eq!(outcome, RunOutcome::Completed);
    assert!(polls > 10);
    assert_eq!(session.report(), &evaluate_units::<Mock>(&base()));
}

#[test]
fn render_right_after_edit_has_checks_without_evaluate() {
    let shared = SharedEvaluation::<Mock>::new(base());
    assert_eq!(*shared.report_for(&base()), evaluate_units::<Mock>(&base()));
    let mut edited = base();
    edited.members[2].load += 40.0;
    let editor_view = shared.commit(edited.clone(), &ChangedSet::at(["members.2.load"], vec![]), &[]);
    let viewer_view = shared.report_for(&edited);
    assert!(!editor_view.checks.is_empty());
    assert_eq!(editor_view, viewer_view);
    assert_eq!(*viewer_view, evaluate_units::<Mock>(&edited));
    let mut unseen = edited.clone();
    unseen.gamma += 0.1;
    assert_eq!(*shared.report_for(&unseen), evaluate_units::<Mock>(&unseen));
}

#[test]
fn a_settled_session_hands_every_reader_the_one_retained_report_without_copying() {
    let shared = SharedEvaluation::<Mock>::new(base());
    let first = shared.report_for(&base());
    assert!(Arc::ptr_eq(&first, &shared.report_for(&base())), "the same document answers with the same allocation");
    assert!(Arc::ptr_eq(&first, &shared.report_now()), "a paint that runs nothing reads the same allocation");
    let mut edited = base();
    edited.members[2].load += 40.0;
    let next = shared.report_for(&edited);
    assert!(!Arc::ptr_eq(&first, &next), "an edit publishes a new report");
    assert_eq!(*first, evaluate_units::<Mock>(&base()), "a handle keeps the report it was given");
    assert_eq!(*next, evaluate_units::<Mock>(&edited));
}

#[test]
fn a_catch_up_that_re_runs_no_unit_does_not_merge_the_report_again() {
    let mut session = EvaluationSession::<Mock>::open(base());
    let plan = session.plan(&ChangedSet::all(), &[], session.token());
    session.run(&plan, &mut |_| {});
    let settled = session.shared_report();
    let revision = session.revision();
    let plan = session.plan(&ChangedSet::all(), &[], session.token());
    session.run(&plan, &mut |_| {});
    assert_eq!(session.revision(), revision, "every read hash still matches");
    assert!(Arc::ptr_eq(&settled, &session.shared_report()), "nothing re-ran, so nothing was merged");
}

struct Slow;

impl UnitSource for Slow {
    type Document = Doc;

    fn units(document: &Doc) -> Vec<CheckUnit<Doc>> {
        let mut rows = Rows::<Doc>::new(["gamma"]).global(|doc: &Doc| vec![result("global", doc.gamma, 4.0)]);
        for (index, member) in document.members.iter().enumerate() {
            let id = member.id.clone();
            rows = if index % 4 == 0 {
                rows.slow("member", "members", index, &member.id, CostTier::T2, &[], move |doc: &Doc, progress, token| {
                    for step in 0..50 {
                        if token.is_cancelled() {
                            return Vec::new();
                        }
                        progress.report(step as f32 / 50.0);
                        std::thread::sleep(std::time::Duration::from_millis(2));
                    }
                    vec![result(&id, doc.members[index].load * doc.gamma, 100.0)]
                })
            } else {
                rows.row("member", "members", index, &member.id, CostTier::T0, &[], move |doc: &Doc| vec![result(&id, doc.members[index].load * doc.gamma, 100.0)])
            };
        }
        rows.finish()
    }
}

#[test]
fn rows_scope_addresses_by_position_and_duplicate_ids_get_distinct_units() {
    assert!(Scope::All.admits("members", 3) && Scope::All.globals());
    assert!(!Scope::Globals.admits("members", 0) && Scope::Globals.globals());
    assert!(Scope::Row("members", 1).admits("members", 1) && !Scope::Row("members", 1).admits("members", 2) && !Scope::Row("members", 1).globals());
    let mut document = base();
    document.members[1].id = document.members[0].id.clone();
    let ids: HashSet<_> = Slow::units(&document).into_iter().map(|unit| unit.id).collect();
    assert_eq!(ids.len(), Slow::units(&document).len());
}

#[test]
fn global_and_part_units_with_declared_reads_depend_on_those_reads_only() {
    let units = Rows::<Doc>::new(["gamma"]).global_reading(&["gamma"], |doc: &Doc| vec![result("global", doc.gamma, 4.0)]).part_reading("sections-part", &["sections"], |doc: &Doc| vec![result("sections-part", doc.sections.len() as f64, 4.0)]).finish();
    assert_eq!(units.len(), 2);
    assert!(units[0].reads.covers("gamma") && !units[0].reads.overlaps("label") && !units[0].reads.overlaps("members.0.load"));
    assert!(units[1].reads.covers("sections.1.capacity") && !units[1].reads.overlaps("gamma"));
    let whole = Rows::<Doc>::new(["gamma"]).global(|doc: &Doc| vec![result("global", doc.gamma, 4.0)]).finish();
    assert!(whole[0].reads.overlaps("label"), "the plain global unit still reads the whole document");
}

#[test]
fn background_worker_settles_slow_units_and_equals_the_full_evaluation() {
    let shared = Arc::new(SharedEvaluation::<Slow>::new(Doc::default()));
    let worker = shared.commit_background(base(), &ChangedSet::all(), &[]);
    assert_eq!(worker.join(), RunOutcome::Completed);
    assert!(shared.settled());
    assert_eq!(*shared.report_now(), evaluate_units::<Slow>(&base()));
    assert!(shared.background(&base()).is_none());
}

#[test]
fn the_report_is_readable_while_the_worker_runs_and_the_edit_cancels_it() {
    let shared = Arc::new(SharedEvaluation::<Slow>::new(Doc::default()));
    let worker = shared.commit_background(base(), &ChangedSet::all(), &[]);
    let partial = shared.report_now();
    assert!(partial.checks.len() <= evaluate_units::<Slow>(&base()).checks.len());
    let mut edited = base();
    edited.members[1].load += 5.0;
    let latest = shared.commit(edited.clone(), &ChangedSet::at(["members.1.load"], vec![]), &[]);
    worker.join();
    assert_eq!(*latest, evaluate_units::<Slow>(&edited));
    assert_eq!(*shared.report_for(&edited), evaluate_units::<Slow>(&edited));
    assert!(shared.settled());
}

#[test]
fn cancelling_a_background_run_keeps_finished_units_and_leaves_the_rest_stale() {
    let shared = Arc::new(SharedEvaluation::<Slow>::new(Doc::default()));
    let worker = shared.commit_background(base(), &ChangedSet::all(), &[]);
    shared.cancel_handle().cancel();
    assert_eq!(worker.join(), RunOutcome::Cancelled);
    assert!(!shared.settled());
    assert!(shared.progress().cancelled);
    assert_eq!(*shared.report_for(&base()), evaluate_units::<Slow>(&base()));
}

#[test]
fn a_summary_aggregates_the_merged_unit_results_after_the_fold() {
    struct Summed;

    impl UnitSource for Summed {
        type Document = Doc;

        fn units(document: &Doc) -> Vec<CheckUnit<Doc>> {
            Mock::units(document)
        }

        fn references(document: &Doc) -> Vec<EntityEdge> {
            Mock::references(document)
        }

        fn summarise(_document: &Doc, report: &CheckReport) -> Vec<CheckResult> {
            vec![result("summary", report.checks.len() as f64, 100.0)]
        }
    }

    let full = evaluate_units::<Summed>(&base());
    assert_eq!(full.checks.last().map(|c| c.id.as_str()), Some("summary"));
    let mut session = EvaluationSession::<Summed>::open(base());
    let plan = session.plan(&ChangedSet::all(), &[], session.token());
    session.run(&plan, &mut |_| {});
    assert_eq!(session.report(), &full);
}

#[test]
fn repeated_catch_ups_never_restart_a_running_worker_and_settle_to_the_full_evaluation() {
    let shared = Arc::new(SharedEvaluation::<Slow>::new(Doc::default()));
    let first = shared.background(&base()).expect("first catch-up starts a worker");
    assert!(shared.background(&base()).is_none());
    assert_eq!(first.join(), RunOutcome::Completed);
    assert!(shared.settled() && shared.background(&base()).is_none());
    assert_eq!(*shared.report_now(), evaluate_units::<Slow>(&base()));
}

crate::live_report!(mod live_slow(Doc) = Slow);

#[test]
fn a_responsive_paint_returns_within_its_budget_and_a_later_one_is_the_full_evaluation() {
    let budget = std::time::Duration::from_millis(5);
    let started = std::time::Instant::now();
    let (partial, settled) = live_slow::responsive(&base(), budget);
    assert!(!settled, "three slow units of 100 ms cannot finish inside 5 ms");
    assert!(started.elapsed() < std::time::Duration::from_millis(90), "a paint never waits for the slow units");
    assert!(partial.checks.len() < evaluate_units::<Slow>(&base()).checks.len());
    assert!(live_slow::progress().total > 0);
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
    while !live_slow::settled() && std::time::Instant::now() < deadline {
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    let (full, settled) = live_slow::responsive(&base(), budget);
    assert!(settled);
    assert_eq!(*full, evaluate_units::<Slow>(&base()));
    assert_eq!(full, live_slow::report(&base()));
}

static WHOLE_CONVERSIONS: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone)]
struct Counting(Doc);

impl ToValue for Counting {
    fn to_value(&self) -> DslValue {
        WHOLE_CONVERSIONS.fetch_add(1, Ordering::SeqCst);
        self.0.to_value()
    }

    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, semio_framework_value::ValueError> {
        self.0.value_at_path(path)
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, semio_framework_value::ValueError> {
        self.0.value_key_at_path(path, index)
    }
}

#[test]
fn the_shared_reads_come_from_the_field_names_without_converting_the_document() {
    let counted = Counting(base());
    let units = Rows::<Counting>::except(&counted, &["sections", "members"]).row("members", "members", 0, "m0", CostTier::T0, &[], |_| Vec::new()).finish();
    assert_eq!(WHOLE_CONVERSIONS.load(Ordering::SeqCst), 0, "no whole value tree is built to list the field names");
    let DslValue::Object(entries) = base().to_value() else { panic!("a document is an object") };
    let by_tree: Vec<String> = entries.into_iter().map(|(key, _)| key).filter(|key| key != "sections" && key != "members").chain(["members.0".to_owned()]).collect();
    assert_eq!(units[0].reads.paths(), by_tree.as_slice(), "the same shared reads as the whole-tree reading");
}

static METERED_CONVERSIONS: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Debug, PartialEq)]
struct Metered(Doc);

impl ToValue for Metered {
    fn to_value(&self) -> DslValue {
        METERED_CONVERSIONS.fetch_add(1, Ordering::SeqCst);
        self.0.to_value()
    }

    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, semio_framework_value::ValueError> {
        self.0.value_at_path(path)
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, semio_framework_value::ValueError> {
        self.0.value_key_at_path(path, index)
    }
}

impl FromValue for Metered {
    fn from_value(value: DslValue) -> Result<Self, semio_framework_value::ValueError> {
        Doc::from_value(value).map(Metered)
    }
}

struct MeteredSource;

impl UnitSource for MeteredSource {
    type Document = Metered;

    fn units(document: &Metered) -> Vec<CheckUnit<Metered>> {
        Rows::<Metered>::except(document, &["sections", "members"]).global(|document: &Metered| vec![result("global", document.0.gamma, 4.0)]).finish()
    }
}

#[test]
fn the_session_and_every_other_reader_of_one_revision_convert_the_document_once() {
    let document = Metered(base());
    let session = EvaluationSession::<MeteredSource>::open(document.clone());
    let tree = shared_value_tree(&document);
    let again = shared_value_tree(&document.clone());
    assert_eq!(METERED_CONVERSIONS.load(Ordering::SeqCst), 1, "opening the session (units and merge included) and two further readers convert once");
    assert!(Arc::ptr_eq(&tree, &again));
    assert_eq!(*tree, base().to_value(), "the shared tree is the plain conversion");
    assert_eq!(session.document(), &document);
    let mut edited = document.clone();
    edited.0.gamma += 0.1;
    let next = shared_value_tree(&edited);
    assert!(!Arc::ptr_eq(&tree, &next) && *next == edited.0.to_value(), "an edit is a new revision with its own tree");
    assert!(Arc::ptr_eq(&tree, &shared_value_tree(&document)), "the previous revision is still served, committed and preview documents alternate");
}

#[test]
fn a_gate_is_computed_once_for_all_units_of_a_list_and_every_list_has_its_own() {
    let calls = Arc::new(AtomicUsize::new(0));
    let gate = Gate::<usize>::new();
    let other_unit = gate.clone();
    let counter = calls.clone();
    assert_eq!(*gate.get(|| { counter.fetch_add(1, Ordering::SeqCst); 7 }), 7);
    assert_eq!(*other_unit.get(|| 9), 7, "a clone of the gate shares the one verdict");
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(*Gate::<usize>::default().get(|| 9), 9, "the unit list of another revision computes its own verdict");
}

#[test]
fn global_and_part_units_read_the_shared_paths_plus_only_the_collections_they_declare() {
    let units = Rows::<Doc>::except(&base(), &["sections", "members"])
        .global_sharing(&["members.*.id"], |doc: &Doc| vec![result("global", doc.gamma, 4.0)])
        .part_sharing("verdict", &["sections"], |doc: &Doc| vec![result("verdict", doc.gamma, 4.0)])
        .finish();
    assert_eq!(units[0].reads.paths(), ["gamma", "label", "members.*.id"]);
    assert_eq!(units[1].reads.paths(), ["gamma", "label", "sections"]);
    assert!(units[0].reads.covers("members.7.id") && !units[0].reads.overlaps("members.7.load") && !units[0].reads.overlaps("sections"), "a member load drag leaves the global unit clean");
    assert!(units[1].reads.overlaps("sections.2.capacity") && !units[1].reads.overlaps("members"), "the part unit follows its collection only");
}
