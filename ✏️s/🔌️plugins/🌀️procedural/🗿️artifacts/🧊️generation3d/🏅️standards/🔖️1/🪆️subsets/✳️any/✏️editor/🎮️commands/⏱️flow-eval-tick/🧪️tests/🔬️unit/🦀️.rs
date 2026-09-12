use super::*;
use crate::editor::generation3d::testkit::{app, dispatch_with_view, preview_views};
use crate::editor::generation3d::Generation3dCommand;
use semio_framework_artifact_flow_flow::{FlowFixture, Widget};
use semio_framework_os_flow::neural::{Atom, ColdRetire, Dictionary, Value};

/// ⏱️ `flowEvalTick` is a WINDOW-owned retained job: its `extent` resolves only against the
/// addressed preview window's transient owner, so it must be dispatched through that window's own
/// `ViewModel` — an unaddressed dispatch is refused with `retained command exceeds semantic work
/// capacity`, which is the contract, not a defect (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn flow_eval_tick_does_not_panic_with_nothing_pending() {
    let _serial = crate::editor::generation3d::test_support::lock();
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
    armed_ticks: usize,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResumeCase {
    command: String,
    rearms_per_attached_preview: usize,
    attached_preview_windows: usize,
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

/// ⚖️ LAW: who may arm the next `flowEvalTick` while the live flow extension registry cannot serve
/// the graph.
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
/// nothing serves (0 armed ticks) and once with a kind the live registry does serve (exactly 1), so
/// the gate is proven to block the uncontributed chain WITHOUT blocking a legitimate one.
#[test]
fn an_uncontributed_graph_arms_no_tick_while_a_served_one_keeps_its_chain() {
    let _serial = crate::editor::generation3d::test_support::lock();
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
        let armed = outcome.effects.iter().filter(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == "flowEvalTick")).count();
        assert_eq!(armed, case.armed_ticks, "{}: armed {armed} ticks, fixture says {}", case.id, case.armed_ticks);
        println!("[STATS] {}: kind={kind} unserved={unserved:?} pending={} armed={armed}", case.id, session.pending());
        crate::editor::generation3d::testkit::retire_flow_eval_session(session);
        graph.retire_cold();
    }
    assert_eq!(fixture.resume.command, "setContributions", "the ONE route that may resume a gated chain");
}

/// ⚖️ LAW: the resume. A gated chain is not abandoned — the host's `setContributions` push is what
/// restarts it, through the same addressed self-dispatch every other hop uses, once per attached
/// preview window. This is the other half of the gate above: without it the app would simply stop
/// evaluating forever. The end-to-end recovery (faulted preview → painted mesh, no user action) is
/// `a_late_contributions_install_re_arms_the_evaluation_the_empty_registry_faulted`
/// (`✏️editor/🧪️tests/🔬️unit`); this law states the re-arm COUNT the gate depends on.
#[semio_framework_async_macros::async_test]
async fn a_later_set_contributions_re_arms_the_chain_the_gate_stopped() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let fixture = contribution_gated_arming_fixture();
    let mut app = crate::editor::generation3d::testkit::app_with_registry().await;
    let (flow_view, _preview_view) = crate::editor::generation3d::testkit::shell_views("procedural-gate-main", "procedural-gate-preview");
    // 🪪️ A closure the registry does NOT already hold, so the push really moves the registry
    // generation — an unchanged closure is a no-op by design (`re_pushing_an_unchanged_closure_…`).
    let contributions = crate::editor::generation3d::testkit::staged_flow_extension_contributions_json(&[(
        crate::editor::generation3d::testkit::CONTRIBUTIONS_WITNESS_PLUGIN_ID,
        crate::editor::generation3d::testkit::contributions_witness_manifest_json(),
    )]);
    let pages = semio_framework::public_invocation_string_pages(&contributions);
    let mut rearms = 0;
    for (index, page) in pages.iter().enumerate() {
        let receipt = dispatch_with_view(
            &mut app,
            Generation3dCommand::SetContributions(crate::editor::generation3d::commands::set_contributions::SetContributions { json: page.clone(), page: index as u64, page_count: pages.len() as u64 }),
            flow_view.clone(),
        )
        .await
        .expect("contributions page");
        rearms += receipt.effects.iter().filter(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == "flowEvalTick")).count();
    }
    assert_eq!(rearms, fixture.resume.rearms_per_attached_preview * fixture.resume.attached_preview_windows, "the install owes one re-arm per attached preview window");
    println!("[STATS] setContributions resumed the chain with {rearms} re-arm(s) across {} page(s)", pages.len());
    semio_framework_os_flow::uninstall_flow_extension(crate::editor::generation3d::testkit::CONTRIBUTIONS_WITNESS_EXTENSION_ID).expect("the witness leaves the process-wide registry as it found it");
}
//#endregion 🚧️ContributionGatedArming

//#region 🔒️TickLatch
const TICK_LATCH_FIXTURE_JSON: &str = include_str!("../../../../../🧫️fixtures/🔒️tick-latch.json");

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct TickLatchFixture {
    format: String,
    version: u8,
    window_kinds: std::collections::BTreeMap<String, String>,
    rows: Vec<TickLatchRow>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct TickLatchRow {
    id: String,
    surface: String,
    attached: Vec<TickLatchWindow>,
    sequence: Vec<TickLatchEvent>,
    armed: Vec<Vec<String>>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct TickLatchWindow {
    id: String,
    kind: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct TickLatchEvent {
    event: String,
    #[serde(default)]
    window: Option<String>,
    #[serde(default)]
    parks: usize,
    #[serde(default)]
    more: bool,
    #[serde(default)]
    ok: bool,
}

fn tick_latch_fixture() -> TickLatchFixture {
    let fixture: TickLatchFixture = serde_json::from_str(TICK_LATCH_FIXTURE_JSON).expect("tick latch fixture");
    assert_eq!(fixture.format, "semio.generation3d.tick-latch");
    assert_eq!(fixture.version, 1);
    for (name, declared) in [
        ("preview", crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW),
        ("generatePreview", crate::editor::generation3d::modes::generate::windows::preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW),
        ("viewPreview", crate::viewer::generation3d::modes::view::windows::preview::WINDOW_KIND_ID),
    ] {
        assert_eq!(fixture.window_kinds.get(name).map(String::as_str), Some(declared), "the fixture's {name} window kind must be the id the surface actually registers");
    }
    fixture
}

/// ⚖️ LAW: **at most ONE pending `flowEvalTick` per (app instance, preview window)**, replayed row by
/// row against the RETAINED `FlowEvalSession`'s own latch — the single gate every arming source goes
/// through (the host refresh poll, `setContributions`, `setActiveExample`, every view command, and
/// the chain's own continuations).
///
/// 🪟️ The latch is per WINDOW, not per session: two preview windows on one instance each own their
/// retained evaluation publication, and generate-mode evaluates a patched fixture of its own.
///
/// 🧪️ The fixture is the LAW, in any language — `contract.ts` replays the identical rows against an
/// independent TypeScript implementation of the same state machine and must answer the same `armed`
/// lists (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn every_arming_source_passes_through_one_latch_per_preview_window() {
    let fixture = tick_latch_fixture();
    assert!(fixture.rows.iter().any(|row| row.surface == "viewer"), "the shared chain's law must cover BOTH surfaces");
    for row in &fixture.rows {
        assert_eq!(row.sequence.len(), row.armed.len(), "{}: every event owes exactly one armed answer", row.id);
        let mut session = semio_framework_os_flow::FlowEvalSession::new();
        let attached: Vec<&str> = row.attached.iter().map(|window| window.id.as_str()).collect();
        let mut observed: Vec<Vec<String>> = Vec::new();
        for event in &row.sequence {
            observed.push(replay_tick_latch_event(&mut session, &attached, event, &row.id));
        }
        assert_eq!(observed, row.armed, "tick-latch row {}", row.id);
        for window in &attached {
            assert!(session.window_extensions_in_flight(window) <= 1 || row.sequence.iter().any(|event| event.parks > 1), "{}: in-flight bookkeeping leaked on {window}", row.id);
        }
        println!("[STATS] tick-latch {}: surface={} armed={:?}", row.id, row.surface, observed);
        crate::editor::generation3d::testkit::retire_flow_eval_session(session);
    }
}

/// 🔁️ One fixture event against the real latch API — the ONE place the declared state machine is
/// bound to `FlowEvalSession`, so the law reads as data and the binding is auditable in one glance.
fn replay_tick_latch_event(session: &mut semio_framework_os_flow::FlowEvalSession, attached: &[&str], event: &TickLatchEvent, row_id: &str) -> Vec<String> {
    match event.event.as_str() {
        "refresh" => {
            session.retain_window_tick_latches(attached);
            attached.iter().filter(|window| session.arm_owed_window_tick(window)).map(|window| (*window).to_string()).collect()
        }
        "gesture" => attached.iter().filter(|window| session.arm_window_tick(window)).map(|window| (*window).to_string()).collect(),
        "tick" => {
            let window = event.window.as_deref().unwrap_or_else(|| panic!("{row_id}: a tick event names its window"));
            session.begin_window_tick(window);
            session.note_window_tick_outcome(window, event.more);
            if event.parks > 0 {
                session.note_window_extensions_in_flight(window, event.parks);
                Vec::new()
            } else if event.more && session.arm_window_tick(window) {
                vec![window.to_string()]
            } else {
                Vec::new()
            }
        }
        "resolve" => {
            let window = event.window.as_deref().unwrap_or_else(|| panic!("{row_id}: a resolve event names its window"));
            if !event.ok {
                session.abandon_window_tick(window);
            }
            let discharged = session.settle_window_extension(window);
            let armed = event.ok && event.more && session.arm_window_tick(window);
            if discharged || armed {
                vec![window.to_string()]
            } else {
                Vec::new()
            }
        }
        "invalidate" => {
            session.clear_window_tick_latches();
            Vec::new()
        }
        "detach" => {
            let window = event.window.as_deref().unwrap_or_else(|| panic!("{row_id}: a detach event names its window"));
            let roster: Vec<&str> = attached.iter().copied().filter(|candidate| *candidate != window).collect();
            session.retain_window_tick_latches(&roster);
            Vec::new()
        }
        other => panic!("{row_id}: unknown tick-latch event {other}"),
    }
}

/// ⚖️ LAW: the REAL editor, through the REAL poll. Two `refreshUi` passes in a row leave exactly ONE
/// pending `flowEvalTick` per attached preview window, and a third adds none — the defect this lane
/// closes was the poll running on a THROWAWAY `FlowEvalSession` whose latch was always clear, so
/// every host refresh stacked another chain on the running one (298 invocations / 111 settles in 80 s
/// of live console, each extension answer waiting ~16 s behind the queue).
#[semio_framework_async_macros::async_test]
async fn a_second_refresh_arms_no_second_tick_for_the_same_preview_window() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = crate::editor::generation3d::testkit::app_with_registry().await;
    let (flow_view, preview_view) = crate::editor::generation3d::testkit::shell_views("procedural-latch-main", "procedural-latch-preview");
    let armed_ticks = |effects: &[Effect]| effects.iter().filter(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == "flowEvalTick")).count();

    // 1️⃣ The gesture owes the preview window exactly one chain.
    let receipt = dispatch_with_view(&mut app, Generation3dCommand::SetActiveExample(crate::editor::generation3d::commands::set_active_example::SetActiveExample { example_id: crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_RECT_EXTRUDE.into() }), flow_view.clone()).await.expect("example");
    let armed_by_gesture = armed_ticks(&receipt.effects);
    assert_eq!(armed_by_gesture, 1, "the example switch owes one tick per attached preview window");

    // 2️⃣ Three host refreshes on top of that pending tick add NOTHING.
    let mut armed_per_refresh = Vec::new();
    for _ in 0..3 {
        armed_per_refresh.push(armed_ticks(&semio_framework_plugin::PluginApp::pending_effects(&mut *app, Some(&preview_view)).await));
    }
    assert_eq!(armed_per_refresh, vec![0, 0, 0], "the poll must never stack a chain on a window that already owes a tick, got {armed_per_refresh:?}");

    // 3️⃣ Once the chain converges the poll still arms nothing — idle turns cost no evaluation.
    let ticks = crate::editor::generation3d::testkit::drain_armed_flow_eval_ticks_from(&mut app, &flow_view, &receipt.effects).await;
    let settled = armed_ticks(&semio_framework_plugin::PluginApp::pending_effects(&mut *app, Some(&preview_view)).await);
    assert_eq!(settled, 0, "a settled evaluation owes no continuation");
    println!("[STATS] latch: gesture={armed_by_gesture} refreshes={armed_per_refresh:?} ticks={ticks} settled={settled}");
    semio_framework_plugin::testkit::close_registered_fixture_app(&mut *app);
}
//#endregion 🔒️TickLatch
