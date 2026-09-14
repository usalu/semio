use super::*;
use crate::editor::generation3d::unit_tests::context::{app, dispatch_with_view, preview_views};
use crate::editor::generation3d::Generation3dCommand;
use semio_framework_artifact_flow_flow::{FlowFixture, Widget};
use semio_framework_os_flow::neural::{Atom, ColdRetire, Dictionary, Value};

/// ⏱️ `flowEvalTick` is a WINDOW-owned retained job: its `extent` resolves only against the
/// addressed preview window's transient owner, so it must be dispatched through that window's own
/// `ViewModel` — an unaddressed dispatch is refused with `retained command exceeds semantic work
/// capacity`, which is the contract, not a defect (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn flow_eval_tick_does_not_panic_with_nothing_pending() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    let (view, _) = preview_views("procedural-preview-test", "procedural-preview-test-other");
    dispatch_with_view(&mut app, Generation3dCommand::FlowEvalTick(FlowEvalTick { window_id: view.window_id.clone().expect("preview window"), window_kind_id: view.active_window_kind_id.clone().unwrap_or_else(|| crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into()) }), view).await.expect("flowEvalTick");
}

//#region 🚧️ContributionGatedArming
const CONTRIBUTION_GATED_ARMING_FIXTURE_JSON: &str = include_str!("../../../../🧫️fixtures/🚧️contribution-gated-arming.json");

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContributionGatedArmingFixture {
    format: String,
    version: u8,
    unserved_kind: String,
    served_kind_candidates: Vec<String>,
    tick_step_budget: usize,
    cases: Vec<ArmingCase>,
    resume: ResumeCase,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ArmingCase {
    id: String,
    kind_source: String,
    widgets: usize,
    unfinished_tick: bool,
    may_rearm: bool,
    owed_hops: usize,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResumeCase {
    command: String,
    carried_run_action: String,
}

fn contribution_gated_arming_fixture() -> ContributionGatedArmingFixture {
    let fixture: ContributionGatedArmingFixture = serde_json::from_str(CONTRIBUTION_GATED_ARMING_FIXTURE_JSON).expect("contribution gated arming fixture");
    assert_eq!(fixture.format, "semio.generation3d.contribution-gated-arming");
    assert_eq!(fixture.version, 1);
    assert_eq!(fixture.tick_step_budget, semio_framework_os_flow::FLOW_EVAL_TICK_STEP_BUDGET, "the fixture's widget counts only force a second tick while they exceed the REAL step budget");
    fixture
}

/// 🕸️ A graph of `widgets` independent neurons of one kind — more than the tick step budget, so one
/// tick can never finish it and the chain's continuation decision is the thing under test.
///
/// 🔢 Every neuron carries its OWN `a`/`b` params: the node cache is keyed by
/// `node_hash(kind, merged inputs)`, so identical neurons would be one miss and 599 hits, the whole
/// graph would land inside one tick's budget, and neither arm would ever reach a re-arm decision.
fn single_kind_graph(kind: &str, widgets: usize) -> FlowFixture {
    FlowFixture {
        schema: "flow.fixture".into(),
        widgets: (0..widgets)
            .map(|index| Widget::Neuron {
                id: format!("n{index}"),
                neuron_kind: kind.to_string(),
                params: Dictionary::new().insert("a", Value::Atom(Atom::Integer(index as i64))).insert("b", Value::Atom(Atom::Integer(index as i64 + 1))),
                input_ports: Vec::new(),
                output_ports: Vec::new(),
                preview: false,
            })
            .collect(),
        synapses: Vec::new(),
        ..Default::default()
    }
}

/// ⚖️ LAW: whether a window still owes the `previewEval` run another hop while the live flow extension
/// registry cannot serve the graph.
///
/// An uncontributed operator kind is not slow work. `Evaluator::dispatch` answers
/// `EvalError::UnknownKind`, the node publishes an error dictionary, the error is never cached, and
/// the next tick recomputes the identical miss — so a chain armed here runs forever. It did: the
/// served playground spent a whole 45 s boot re-arming `flowEvalTick` every 3–14 s with the preview
/// faulted at `flow.extension-not-contributed`, and because every settle drove a host `refreshUi`,
/// the spin starved the very `setContributions` push that would have lifted it
/// (`🗑️generated/console-dump/console.txt`, ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// The law is measured on BOTH arms off one fixture: the same oversized graph, once with a kind
/// nothing serves (0 owed hops) and once with a kind the live registry does serve (exactly 1), so
/// the gate is proven to give the uncontributed window up WITHOUT giving up a legitimate one.
#[test]
fn an_uncontributed_graph_arms_no_tick_while_a_served_one_keeps_its_chain() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let fixture = contribution_gated_arming_fixture();
    let served_kind = fixture
        .served_kind_candidates
        .iter()
        .find(|kind| {
            // 🧹️ The probe graph owns `Dictionary` params like every other one — retire it, or the
            // neural engine's ownership gate fails the run on a leak this test invented itself.
            let probe = single_kind_graph(kind, 1);
            let served = semio_framework_os_flow::unserved_flow_operator_kinds(&probe).is_empty();
            probe.retire_cold();
            served
        })
        .unwrap_or_else(|| panic!("this binary's registry must serve at least one of {:?}, or the served arm proves nothing", fixture.served_kind_candidates))
        .clone();
    for case in &fixture.cases {
        let kind = if case.kind_source == "unservedKind" { fixture.unserved_kind.clone() } else { served_kind.clone() };
        let graph = single_kind_graph(&kind, case.widgets);
        let unserved = semio_framework_os_flow::unserved_flow_operator_kinds(&graph);
        assert_eq!(unserved.is_empty(), case.may_rearm, "{}: the registry's answer for {kind} disagrees with the fixture ({unserved:?})", case.id);
        assert_eq!(crate::preview_eval::may_rearm(&graph), case.may_rearm, "{}: may_rearm disagrees with the fixture", case.id);

        let mut session = semio_framework_os_flow::FlowEvalSession::new();
        let outcome = crate::preview_eval::evaluate_tick("procedural-preview-gate", crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW, &graph, 0.1, &mut session, None);
        assert_eq!(session.pending(), case.unfinished_tick, "{}: the graph must be big enough that one tick cannot finish it", case.id);
        assert!(outcome.extension_invocations.is_empty(), "{}: an in-guest graph parks no extension work", case.id);
        let owed = usize::from(session.window_tick_owed("procedural-preview-gate"));
        assert_eq!(owed, case.owed_hops, "{}: owes {owed} hops, fixture says {}", case.id, case.owed_hops);
        println!("[STATS] {}: kind={kind} unserved={unserved:?} pending={} owed={owed}", case.id, session.pending());
        crate::editor::generation3d::unit_tests::context::retire_flow_eval_session(session);
        graph.retire_cold();
    }
    assert_eq!(fixture.resume.command, "setContributions", "the ONE route that may resume a gated chain");
}

/// ⚖️ LAW: the resume. A given-up window is not abandoned forever — the host's `setContributions` push
/// owes the attached previews an evaluation again, and a settled run is finalized so the next refresh
/// starts a fresh one. This is the other half of the gate above: without it the app would simply stop
/// evaluating forever. The end-to-end recovery (faulted preview → painted mesh, no user action) is
/// `a_late_contributions_install_re_arms_the_evaluation_the_empty_registry_faulted`
/// (`✏️editor/🧪️tests/🔬️unit`).
#[semio_framework_async_macros::async_test]
async fn a_later_set_contributions_owes_the_settled_run_a_restart() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let fixture = contribution_gated_arming_fixture();
    let mut app = crate::editor::generation3d::unit_tests::context::app_with_registry().await;
    let (flow_view, _preview_view) = crate::editor::generation3d::unit_tests::context::shell_views("procedural-gate-main", "procedural-gate-preview");
    let settled = crate::editor::generation3d::unit_tests::context::drive_preview_run(&mut app, &flow_view, &[]).await;
    assert_eq!(settled.state.as_deref(), Some("finalized"), "the preview settles before the push: {settled:?}");
    // 🪪️ A closure the registry does NOT already hold, so the push really moves the registry
    // generation — an unchanged closure is a no-op by design (`re_pushing_an_unchanged_closure_…`).
    let contributions = crate::editor::generation3d::unit_tests::context::staged_flow_extension_contributions_json(&[(
        crate::editor::generation3d::unit_tests::context::CONTRIBUTIONS_WITNESS_PLUGIN_ID,
        crate::editor::generation3d::unit_tests::context::contributions_witness_manifest_json(),
    )]);
    let pages = semio_framework::public_invocation_string_pages(&contributions);
    let mut carried = Vec::new();
    for (index, page) in pages.iter().enumerate() {
        let receipt = dispatch_with_view(
            &mut app,
            Generation3dCommand::SetContributions(crate::editor::generation3d::commands::set_contributions::SetContributions { json: page.clone(), page: index as u64, page_count: pages.len() as u64 }),
            flow_view.clone(),
        )
        .await
        .expect("contributions page");
        carried.extend(receipt.effects);
    }
    let owed = crate::editor::generation3d::unit_tests::context::run_actions(&carried);
    assert_eq!(owed, vec![fixture.resume.carried_run_action.clone()], "the install carries the settled preview exactly one restart on its own emit");
    let restarted = crate::editor::generation3d::unit_tests::context::drive_preview_run(&mut app, &flow_view, &carried).await;
    assert!(restarted.hops > 0 && restarted.state.as_deref() == Some("finalized"), "the restarted run evaluates again: {restarted:?}");
    println!("[STATS] setContributions carried {owed:?} across {} page(s); restart ran {} hops", pages.len(), restarted.hops);
    semio_framework_os_flow::uninstall_flow_extension(crate::editor::generation3d::unit_tests::context::CONTRIBUTIONS_WITNESS_EXTENSION_ID).expect("the witness leaves the process-wide registry as it found it");
}
//#endregion 🚧️ContributionGatedArming

//#region 🔒️RunStart
/// ⚖️ LAW: the REAL editor, through the REAL poll. A gesture carries exactly ONE `previewEval` run start,
/// no refresh over the owed preview asks for a second while that request stands, and a settled run owes
/// nothing — the defect the per-window latch closed was a poll stacking another evaluation chain on the
/// running one at every host refresh (298 invocations / 111 settles in 80 s of live console).
#[semio_framework_async_macros::async_test]
async fn a_second_refresh_starts_no_second_run() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = crate::editor::generation3d::unit_tests::context::app_with_registry().await;
    let (flow_view, preview_view) = crate::editor::generation3d::unit_tests::context::shell_views("procedural-latch-main", "procedural-latch-preview");
    let switched = dispatch_with_view(&mut app, Generation3dCommand::SetActiveExample(crate::editor::generation3d::commands::set_active_example::SetActiveExample { example_id: crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_RECT_EXTRUDE.into() }), flow_view.clone()).await.expect("example");
    let first = crate::editor::generation3d::unit_tests::context::run_actions(&switched.effects);
    assert_eq!(first, vec![semio_framework_plugin::TOOL_RUN_START_ACTION_ID.to_string()], "the gesture carries one run start");
    for _ in 0..3 {
        let polled = crate::editor::generation3d::unit_tests::context::owed_run_actions(&mut app, &preview_view).await;
        assert!(polled.is_empty(), "a refresh must not ask again while the gesture's start stands, got {polled:?}");
    }
    let receipt = crate::editor::generation3d::unit_tests::context::drive_preview_run(&mut app, &flow_view, &switched.effects).await;
    let mut owed_per_refresh = Vec::new();
    for _ in 0..3 {
        owed_per_refresh.push(crate::editor::generation3d::unit_tests::context::owed_run_actions(&mut app, &preview_view).await);
    }
    assert!(owed_per_refresh.iter().all(Vec::is_empty), "a settled run owes nothing to any later refresh, got {owed_per_refresh:?}");
    assert_eq!(receipt.run_actions.iter().filter(|action| *action == semio_framework_plugin::TOOL_RUN_START_ACTION_ID).count(), 1, "one run start in the whole drive: {receipt:?}");
    assert_eq!(receipt.state.as_deref(), Some("finalized"));
    println!("[STATS] run start: first={first:?} hops={} answered={} refreshes={owed_per_refresh:?}", receipt.hops, receipt.answered);
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}
//#endregion 🔒️RunStart
