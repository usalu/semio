//! ⏱️ The BUDGET law of the `evaluate` capability, replayed from
//! `🧫️fixtures/⏱️evaluate-budget.json` against the real `FlowEvalSession`, the real fold
//! (`preview_eval::resolve_eval`) and the real status projection.
//!
//! The companion `🔬️unit` module pins what ONE finished answer does; this pins what an UNFINISHED
//! one does, which is the whole reason a sixteen-second boolean no longer kills its worker
//! (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️extension-evaluate-budget-2026-09-12.md`).

use super::*;
use crate::editor::generation3d::unit_tests::context::{empty_history_view, retire_flow_eval_session};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect};

const EVALUATE_BUDGET_FIXTURE_JSON: &str = include_str!("../../../../../🧫️fixtures/⏱️evaluate-budget.json");
const GEOMETRY_EXTENSION_PLUGIN_ID: &str = "flow-extension-brep";
const BUDGET_WINDOW_ID: &str = "procedural-preview-test";
const BUDGET_NODE_HASH: u64 = 0x_b0_07_ca_11_u64;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct EvaluateBudgetFixture {
    format: String,
    version: u8,
    capability: String,
    cancel_capability: String,
    response_action: String,
    phase_labels: std::collections::BTreeMap<String, PhaseLabel>,
    job_phase_tags: std::collections::BTreeMap<String, String>,
    rows: Vec<EvaluateBudgetRow>,
    stepped_operator: SteppedOperator,
}

#[derive(serde::Deserialize)]
struct PhaseLabel {
    en: String,
    de: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct EvaluateBudgetRow {
    id: String,
    #[serde(default)]
    envelope: Option<serde_json::Value>,
    #[serde(default)]
    bare_output_json: Option<String>,
    outcome: String,
    seeds: bool,
    #[serde(default)]
    seeded_output_json: Option<String>,
    rearms_ticks: usize,
    status: ExpectedStatus,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExpectedStatus {
    phase: String,
    in_flight: u64,
    eval_units_done: u64,
    eval_units_total: u64,
    ratio: f64,
    cancellable: bool,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct SteppedOperator {
    operator_id: String,
    minimum_round_trips_under_a_tight_budget: usize,
    phase_order: Vec<String>,
}

fn fixture() -> EvaluateBudgetFixture {
    let fixture: EvaluateBudgetFixture = serde_json::from_str(EVALUATE_BUDGET_FIXTURE_JSON).expect("evaluate budget fixture");
    assert_eq!(fixture.format, "semio.generation3d.evaluate-budget");
    assert_eq!(fixture.version, 1);
    fixture
}

/// 📈️ The status object the preview window would publish right now, through the same pure
/// projection the surface uses.
fn observed_status(session: &FlowEvalSession) -> serde_json::Value {
    serde_json::from_str(&crate::editor::generation3d::preview_progress_status_json_for(Some(session), Ok(GEOMETRY_EXTENSION_PLUGIN_ID.to_string()))).expect("status json")
}

/// ⚖️ LAW: every row of the fixture, folded through the real command handler — what it seeds, what
/// it arms, and the exact status the preview window publishes afterwards.
#[test]
fn the_evaluate_budget_envelope_obeys_its_fixture_end_to_end() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let fixture = fixture();
    assert_eq!(fixture.capability, "evaluate", "the budgeted capability");
    assert_eq!(fixture.cancel_capability, "evaluateCancel", "the door a cancel reaches a parked evaluation through");
    assert_eq!(fixture.response_action, "flowEvalResolve", "the command this fold is bound to");
    for row in &fixture.rows {
        let snapshot = Generation3dSnapshot::default();
        let history = empty_history_view();
        let config = Generation3dConfig::default();
        let doc = ArtifactView::new(&snapshot, &history);
        let cfg = ConfigView { snapshot: &config, window: None };
        let mut session = FlowEvalSession::new();
        let cache = session.neural_cache();
        let output_json = match (&row.envelope, &row.bare_output_json) {
            (Some(envelope), _) => serde_json::to_string(envelope).expect("envelope json"),
            (None, Some(bare)) => bare.clone(),
            (None, None) => panic!("{}: a row declares either an envelope or a bare output body", row.id),
        };
        let emit = handle(
            &FlowEvalResolve {
                window_id: BUDGET_WINDOW_ID.into(),
                window_kind_id: crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into(),
                node_hash: BUDGET_NODE_HASH,
                output_json,
                extension_id: GEOMETRY_EXTENSION_PLUGIN_ID.into(),
                ok: true,
                fault_code: String::new(),
                fault_message: String::new(),
            },
            &doc,
            &cfg,
            &mut session,
        )
        .expect("flowEvalResolve");
        assert_eq!(cache.contains(BUDGET_NODE_HASH), row.seeds, "{}: seeds the node cache", row.id);
        if let Some(expected) = &row.seeded_output_json {
            let cached = cache.get(BUDGET_NODE_HASH).unwrap_or_else(|| panic!("{}: a seeding row must be readable", row.id));
            let expected_value: serde_json::Value = serde_json::from_str(expected).expect("expected output json");
            let observed: serde_json::Value = serde_json::from_str(&dsl::json::to_json_string(&cached)).expect("cached output json");
            assert_eq!(observed, expected_value, "{}: the seeded output is exactly the envelope's own body", row.id);
            semio_framework_os_flow::neural::ColdRetire::retire_cold(cached);
        }
        drop(cache);
        let rearmed = emit
            .effects
            .iter()
            .filter(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == "flowEvalTick"))
            .count();
        assert_eq!(rearmed, row.rearms_ticks, "{}: re-armed tick count ({} outcome)", row.id, row.outcome);
        let status = observed_status(&session);
        assert_eq!(status["phase"].as_str(), Some(row.status.phase.as_str()), "{}: published phase", row.id);
        let labels = fixture.phase_labels.get(&row.status.phase).unwrap_or_else(|| panic!("{}: the fixture declares a label for every phase it expects", row.id));
        assert_eq!(status["phaseLabel"]["en"].as_str(), Some(labels.en.as_str()), "{}: english label", row.id);
        assert_eq!(status["phaseLabel"]["de"].as_str(), Some(labels.de.as_str()), "{}: german label — the UI carries both, with no default language", row.id);
        assert_eq!(status["progress"]["inFlight"].as_u64(), Some(row.status.in_flight), "{}: inFlight counts the budgeted evaluation too", row.id);
        assert_eq!(status["progress"]["evalUnitsDone"].as_u64(), Some(row.status.eval_units_done), "{}: evalUnitsDone", row.id);
        assert_eq!(status["progress"]["evalUnitsTotal"].as_u64(), Some(row.status.eval_units_total), "{}: evalUnitsTotal", row.id);
        let ratio = status["progress"]["ratio"].as_f64().unwrap_or_else(|| panic!("{}: ratio", row.id));
        assert!((ratio - row.status.ratio).abs() < 1e-9, "{}: ratio {ratio} != {}", row.id, row.status.ratio);
        assert_eq!(status["cancellable"].as_bool(), Some(row.status.cancellable), "{}: cancellable", row.id);
        assert_eq!(status["cancelAction"].as_str(), Some("cancelPreviewEval"), "{}: the surface always names the verb that would stop it", row.id);
        retire_flow_eval_session(session);
    }
}

/// ⚖️ LAW: the fixture's declared job phase vocabulary IS the one the session projects. A tag the
/// vocabulary does not know reads as `computing`, never as `idle` — the envelope that carried it
/// said the job is still working.
#[test]
fn every_declared_job_phase_tag_projects_to_its_declared_surface_phase() {
    let fixture = fixture();
    for (job_tag, surface_tag) in &fixture.job_phase_tags {
        assert_eq!(semio_framework_os_flow::PreviewEvalPhase::from_job_tag(job_tag).tag(), surface_tag.as_str(), "job phase `{job_tag}` projects to `{surface_tag}`");
    }
    assert_eq!(semio_framework_os_flow::PreviewEvalPhase::from_job_tag("a-phase-no-surface-has-heard-of").tag(), "computing", "an unknown tag is live work, never idle");
    for (tag, labels) in &fixture.phase_labels {
        let phase = semio_framework_os_flow::PreviewEvalPhase::from_job_tag(match tag.as_str() {
            "imprinting" => "imprint",
            "splitting" => "applyA",
            "classifying" => "classifyA",
            "stitching" => "stitch",
            "validating" => "validate",
            other => other,
        });
        assert_eq!(phase.tag(), tag.as_str(), "the fixture's label table is keyed by surface phase");
        let (english, german) = phase.labels();
        assert_eq!(english, labels.en, "english label for `{tag}`");
        assert_eq!(german, labels.de, "german label for `{tag}`");
    }
}

/// ⚖️ LAW: progress is monotone across the round trips of one evaluation, and a working envelope
/// never seeds. Replayed as a real SEQUENCE (the fixture's rows are each a single fold), because
/// monotonicity is a property of the sequence, not of any one answer.
#[test]
fn progress_is_monotone_across_the_round_trips_of_one_evaluation() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let fixture = fixture();
    let snapshot = Generation3dSnapshot::default();
    let history = empty_history_view();
    let config = Generation3dConfig::default();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let mut session = FlowEvalSession::new();
    let cache = session.neural_cache();
    let mut previous_done = 0_u64;
    let total_steps = fixture.stepped_operator.phase_order.len();
    assert!(total_steps >= fixture.stepped_operator.minimum_round_trips_under_a_tight_budget, "the declared phase order must admit at least the declared minimum of round trips");
    for (index, phase) in fixture.stepped_operator.phase_order.iter().enumerate() {
        let done = index == total_steps - 1;
        let units_done = (index + 1) as u64;
        let envelope = serde_json::json!({
            "done": done,
            "cancellable": !done,
            "phase": phase,
            "unitsDone": units_done,
            "unitsTotal": total_steps as u64,
            "outputJson": if done { "{\"solid\":\"brep:solid-9\"}" } else { "" },
        });
        let emit = handle(
            &FlowEvalResolve {
                window_id: BUDGET_WINDOW_ID.into(),
                window_kind_id: crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into(),
                node_hash: BUDGET_NODE_HASH,
                output_json: serde_json::to_string(&envelope).expect("envelope json"),
                extension_id: GEOMETRY_EXTENSION_PLUGIN_ID.into(),
                ok: true,
                fault_code: String::new(),
                fault_message: String::new(),
            },
            &doc,
            &cfg,
            &mut session,
        )
        .expect("flowEvalResolve");
        let status = observed_status(&session);
        let observed_done = status["progress"]["evalUnitsDone"].as_u64().expect("evalUnitsDone");
        if done {
            assert!(cache.contains(BUDGET_NODE_HASH), "the terminal answer seeds the node cache");
            assert_eq!(observed_done, 0, "a finished evaluation leaves no in-flight row behind");
        } else {
            assert!(!cache.contains(BUDGET_NODE_HASH), "`{phase}` is still working, so nothing may be seeded");
            assert!(observed_done >= previous_done, "unitsDone never decreases ({previous_done} -> {observed_done} at `{phase}`)");
            assert_eq!(status["progress"]["inFlight"].as_u64(), Some(1), "`{phase}` is live work, and the surface must say so");
            assert_eq!(status["cancellable"].as_bool(), Some(true), "`{phase}` is stoppable");
            assert_ne!(status["phase"].as_str(), Some("idle"), "`{phase}` must never publish as idle");
            previous_done = observed_done;
        }
        assert_eq!(
            emit.effects.iter().filter(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == "flowEvalTick")).count(),
            1,
            "`{phase}` owes exactly one continuation"
        );
        // ▶️ The armed tick actually RUNS between two round trips of the real chain, which is what
        // frees the window's latch for the next one. Replaying the answers without it would assert
        // against a latch state the chain never has.
        session.begin_window_tick(BUDGET_WINDOW_ID);
    }
    if let Some(cached) = cache.get(BUDGET_NODE_HASH) {
        semio_framework_os_flow::neural::ColdRetire::retire_cold(cached);
    }
    drop(cache);
    retire_flow_eval_session(session);
}

/// ⚖️ LAW: the gesture that stops a preview reaches BOTH retained-work registries in the geometry
/// extension — the parked budgeted evaluations and the mesh jobs — because a boolean stopped
/// mid-validation lives only in the first.
#[test]
fn the_cancel_gesture_reaches_the_evaluation_registry_too() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let fixture = fixture();
    let mut session = FlowEvalSession::new();
    let payload = crate::preview_eval::CancelPreviewEval { window_id: BUDGET_WINDOW_ID.into(), window_kind_id: crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into() };
    let invocations = crate::preview_eval::cancel_preview_eval_for(&payload, &mut session, Ok(GEOMETRY_EXTENSION_PLUGIN_ID.to_string()));
    let capabilities: Vec<&str> = invocations.iter().map(|invocation| invocation.capability.as_str()).collect();
    assert!(capabilities.contains(&fixture.cancel_capability.as_str()), "the gesture emits the fixture's evaluation-cancel capability, got {capabilities:?}");
    assert!(capabilities.contains(&"tessellateCancel"), "the gesture still emits the mesh-job cancel, got {capabilities:?}");
    retire_flow_eval_session(session);
}
