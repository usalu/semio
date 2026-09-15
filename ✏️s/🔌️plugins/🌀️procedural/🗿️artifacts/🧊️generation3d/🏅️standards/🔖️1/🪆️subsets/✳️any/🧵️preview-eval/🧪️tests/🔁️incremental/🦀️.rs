//! 🔁️ Headless incremental re-evaluation lane — what ONE slider edit may cost the chain, measured
//! on the real hex-column graph with the packaged `brep`/`math` operators installed, so the answer
//! is the kernel's own and not a model of it.
//!
//! ⚖️ Three statements, over the committed fixture `🧫️fixtures/🔁️incremental-eval.json`:
//!   1. DIRTY-SET PRECISION — moving `height` re-evaluates the extrusion branch and leaves the
//!      profile branch to free-ride on the previous channels; moving `radius` does the mirror image.
//!      A burst of N values costs N such branches, never a growing one.
//!   2. TESSELLATION CACHE — a node whose geometry HANDLE did not change keeps its mesh, so exactly
//!      one handle is owed a `tessellate` round trip after a `height` edit, and the untouched
//!      branch's tessellated geometry is byte-for-byte the mesh that was already on screen.
//!   3. PAINTED EVALUATION — a node whose request is still crossing keeps its last converged answer,
//!      so the preview never blanks mid-gesture, while an answered node (error included) and a node
//!      the document no longer declares are never dressed up.
//!
//! @see ../../../../../../../📓️slider-latency-incremental-eval-2026-09-15.md
//! @see ../../../📚️examples/🧪️tests/🧩️geometry/🦀️.rs — the same install/parse/retire recipe.

use std::sync::{Mutex, MutexGuard, OnceLock};

use semio_framework_job::{allocate_operation_id, CancelToken, Generation, StepBudget, StepContext};
use semio_framework_os_flow::neural::{ColdRetire as _, Registry};
use semio_framework_os_flow::{flow_neuron_kind_info_map, install_flow_extension, merge_unanswered_eval_entries, tessellate_geometry, FlowExtensionSpec, FlowHost, FlowHostRetirement};
use semio_s_artifact_procedural_generation3d::preview_eval;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::snapshot::text::parse_dsl;
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
use serde::Deserialize;

//#region 🔖️Fixture
const FIXTURE_JSON: &str = include_str!("../../../🧫️fixtures/🔁️incremental-eval.json");
const HEX_COLUMN_DSL: &str = include_str!("../../../📚️examples/🍄️hexagonal-mushroom-column/🖼️assets/🍄️hexagonal-mushroom-column/🗣️.dsl.semio");

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct IncrementalEvalFixture {
    format: String,
    version: u32,
    example: String,
    edits: Vec<EditRow>,
    burst: BurstRow,
    painted_eval: PaintedEvalRow,
    chain_scope: ChainScopeRow,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EditRow {
    widget: String,
    from: f64,
    to: f64,
    dirty: Vec<String>,
    clean: Vec<String>,
    handles_unchanged: Vec<String>,
    handles_changed: Vec<String>,
    max_retessellations: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BurstRow {
    widget: String,
    values: Vec<f64>,
    dirty_per_value: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PaintedEvalRow {
    outstanding_node: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChainScopeRow {
    always: Vec<String>,
    only_when_census_moved: Vec<String>,
    panels: Vec<String>,
    never_full: bool,
    never_none: bool,
    chrome: ChromeRow,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChromeRow {
    same_marks: Vec<MarksPair>,
    different_marks: Vec<MarksPair>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MarksPair {
    left: serde_json::Value,
    right: serde_json::Value,
}

fn fixture() -> IncrementalEvalFixture {
    let fixture: IncrementalEvalFixture = serde_json::from_str(FIXTURE_JSON).expect("the incremental evaluation fixture parses");
    assert_eq!(fixture.format, "semio.generation3d.incremental-eval");
    assert_eq!(fixture.version, 1);
    assert_eq!(fixture.example, "hexagonal-mushroom-column");
    fixture
}
//#endregion 🔖️Fixture

//#region 🧩️Harness
fn exclusive() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(())).lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn resolve_ready<T>(future: impl std::future::Future<Output = T>) -> T {
    match std::pin::pin!(future).as_mut().poll(&mut std::task::Context::from_waker(std::task::Waker::noop())) {
        std::task::Poll::Ready(value) => value,
        std::task::Poll::Pending => panic!("extension registration must not depend on external work"),
    }
}

fn install_example_operators(registry: &mut Registry) {
    resolve_ready(semio_s_plugin_flow_extension_brep::register(registry));
    semio_s_plugin_flow_extension_math::register(registry);
}

fn operators_installed() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        install_flow_extension(FlowExtensionSpec { id: "incremental-eval".into(), name: "Incremental Eval".into(), version: "1".into(), install: install_example_operators }).expect("incremental eval operator admission");
    });
}

fn frozen_now() -> Option<u64> {
    Some(0)
}

/// 🧹️ `FlowHostSnapshot`'s ordered maps panic on a bare drop, so a host is closed, never dropped.
fn retire_host(host: FlowHost) {
    let mut sequence = 0;
    let mut retirement = FlowHostRetirement::new(host);
    for _ in 0..1_000_000 {
        let mut context = StepContext::new(allocate_operation_id(), Generation(0), StepBudget::new(u64::MAX, u64::MAX), CancelToken::root_now(), frozen_now, &mut sequence);
        if retirement.close_step(&mut context) {
            return;
        }
    }
    panic!("flow host retirement did not reach terminal-empty");
}

/// 🏠️ The hex column as the editor's chain sees it, with the neuron catalogue installed.
fn hex_column_host() -> FlowHost {
    operators_installed();
    let Generation3dSnapshot { host_snapshot, generation } = parse_dsl(HEX_COLUMN_DSL).expect("the hex column dsl parses");
    generation.retire_cold();
    let mut host = FlowHost::from_host_snapshot(host_snapshot);
    host.set_neuron_kind_info_map(flow_neuron_kind_info_map());
    host
}

/// 🧊️ The geometry handle one evaluated node channel carries, or `None` for a channel that carries
/// none — the content address the tessellation cache is keyed by.
fn handle_of(eval_json: &str, node: &str, channel: &str) -> Option<String> {
    let eval: serde_json::Value = serde_json::from_str(eval_json).ok()?;
    eval.get(node)?.get("out")?.get(channel)?.get("handle")?.as_str().map(str::to_string)
}

fn node_channel(reference: &str) -> (&str, &str) {
    reference.split_once('@').expect("a handle reference is node@channel")
}

//#endregion 🧩️Harness

//#region ⚖️DirtySet
/// ⚖️ LAW: an edit re-evaluates the nodes DOWNSTREAM of the edited widget and nothing else — the
/// untouched branch free-rides on the converged baseline, and its evaluated output is byte-identical.
#[test]
fn an_edit_dirties_its_own_branch_and_leaves_every_other_node_to_free_ride() {
    let _guard = exclusive();
    let fixture = fixture();
    for row in &fixture.edits {
        let mut before = hex_column_host();
        let baseline_eval = before.evaluate().expect("the hex column evaluates");
        let (snapshot, channels) = before.eval_baseline();
        let generation = before.eval_baseline_registry_generation();

        let mut after = hex_column_host();
        after.install_eval_baseline(snapshot, channels, generation);
        after.set_slider_value(&row.widget, row.to);
        let dirty = after.dirty_widget_ids_since(&before);
        println!("[STATS] edit {} {} -> {} dirty={dirty:?}", row.widget, row.from, row.to);
        assert_eq!(dirty, row.dirty, "moving {} must dirty exactly its own branch", row.widget);
        for clean in &row.clean {
            assert!(!dirty.contains(clean), "{clean} is upstream of or unrelated to {} and may not be re-evaluated", row.widget);
        }

        let edited_eval = after.evaluate().expect("the edited hex column evaluates");
        for reference in &row.handles_unchanged {
            let (node, channel) = node_channel(reference);
            assert_eq!(handle_of(&edited_eval, node, channel), handle_of(&baseline_eval, node, channel), "{reference} is not downstream of {} and must keep its content address", row.widget);
        }
        for reference in &row.handles_changed {
            let (node, channel) = node_channel(reference);
            let (was, now) = (handle_of(&baseline_eval, node, channel), handle_of(&edited_eval, node, channel));
            assert!(was.is_some() && now.is_some(), "{reference} carries a geometry handle before and after the edit");
            assert_ne!(was, now, "{reference} is downstream of {} and must mint a new content address", row.widget);
        }
        retire_host(before);
        retire_host(after);
    }
}

/// ⚖️ LAW: a BURST of N values costs N dirty branches, never a growing one — the baseline advances
/// with every converged walk, so value k+1 is diffed against value k and not against the document
/// the gesture started on.
#[test]
fn a_burst_of_values_costs_one_dirty_branch_per_value_and_never_a_growing_one() {
    let _guard = exclusive();
    let fixture = fixture();
    let burst = &fixture.burst;
    let mut previous = hex_column_host();
    previous.evaluate().expect("the hex column evaluates");
    for value in &burst.values {
        let (snapshot, channels) = previous.eval_baseline();
        let generation = previous.eval_baseline_registry_generation();
        let mut next = hex_column_host();
        next.install_eval_baseline(snapshot, channels, generation);
        next.set_slider_value(&burst.widget, *value);
        let dirty = next.dirty_widget_ids_since(&previous);
        println!("[STATS] burst {} = {value} dirty={dirty:?}", burst.widget);
        assert_eq!(dirty, burst.dirty_per_value, "every value of a burst dirties the same branch");
        next.evaluate().expect("the edited hex column evaluates");
        retire_host(previous);
        previous = next;
    }
    retire_host(previous);
}
//#endregion ⚖️DirtySet

//#region ⚖️TessellationCache
/// ⚖️ LAW: a node whose geometry handle did not change is never re-tessellated, and the mesh it
/// keeps is byte-for-byte the one already on screen. Exactly the changed branch's handle is owed a
/// round trip.
#[test]
fn only_a_changed_handle_is_owed_a_tessellate_round_trip() {
    let _guard = exclusive();
    let fixture = fixture();
    let row = fixture.edits.iter().find(|row| row.widget == "height").expect("the fixture states the height edit");
    let mut before = hex_column_host();
    let baseline_eval = before.evaluate().expect("the hex column evaluates");
    let (snapshot, channels) = before.eval_baseline();
    let generation = before.eval_baseline_registry_generation();

    let mut after = hex_column_host();
    after.install_eval_baseline(snapshot, channels, generation);
    after.set_slider_value(&row.widget, row.to);
    let edited_eval = after.evaluate().expect("the edited hex column evaluates");

    let tolerance = preview_eval::preview_tolerance("coarse");
    let mut retessellated = 0usize;
    for reference in row.handles_unchanged.iter().chain(row.handles_changed.iter()) {
        let (node, channel) = node_channel(reference);
        let (was, now) = (handle_of(&baseline_eval, node, channel).expect("a handle before"), handle_of(&edited_eval, node, channel).expect("a handle after"));
        if was == now {
            let (old_mesh, new_mesh) = (tessellate_geometry(&was, tolerance).expect("tessellates"), tessellate_geometry(&now, tolerance).expect("tessellates"));
            assert_eq!(old_mesh.positions, new_mesh.positions, "{reference} kept its content address, so its mesh may not move");
            assert_eq!(old_mesh.indices, new_mesh.indices, "{reference} kept its content address, so its topology may not move");
            continue;
        }
        retessellated += 1;
    }
    println!("[STATS] height edit retessellations={retessellated} budget={}", row.max_retessellations);
    assert!(retessellated <= row.max_retessellations, "a {} edit may re-tessellate at most {} mesh", row.widget, row.max_retessellations);
    assert_eq!(retessellated, row.handles_changed.len(), "exactly the changed handles are re-tessellated");
    retire_host(before);
    retire_host(after);
}
//#endregion ⚖️TessellationCache

//#region ⚖️PaintedEvaluation
/// ⚖️ LAW: a node the live walk has not answered YET keeps its last converged answer, so a preview
/// never blanks while its branch recomputes; a node the walk DID answer keeps the live answer even
/// when that answer is an error; a node the document no longer declares is never filled in.
#[test]
fn an_unanswered_node_keeps_its_converged_answer_and_an_answered_one_never_does() {
    let _guard = exclusive();
    let fixture = fixture();
    let outstanding = &fixture.painted_eval.outstanding_node;
    let mut host = hex_column_host();
    let converged = host.evaluate().expect("the hex column evaluates");

    let mut live_value: serde_json::Value = serde_json::from_str(&converged).expect("evaluation json");
    let live_map = live_value.as_object_mut().expect("an evaluation is an object");
    let parked = live_map.get_mut(outstanding.as_str()).expect("the outstanding node was answered by the converged walk");
    assert!(parked.get("out").and_then(serde_json::Value::as_object).is_some_and(|out| !out.is_empty()), "the converged walk answered {outstanding}");
    parked.as_object_mut().expect("an evaluation row is an object").insert("out".into(), serde_json::json!({}));
    live_map.insert("ghost-node".into(), serde_json::json!({ "out": { "value": 1 } }));
    let live = serde_json::to_string(&live_value).expect("live evaluation json");

    let painted = merge_unanswered_eval_entries(&live, &converged, &host.host_snapshot);
    let painted_value: serde_json::Value = serde_json::from_str(&painted).expect("painted evaluation json");
    let converged_value: serde_json::Value = serde_json::from_str(&converged).expect("converged evaluation json");
    assert_eq!(painted_value.get(outstanding.as_str()), converged_value.get(outstanding.as_str()), "an unanswered node keeps its converged answer");
    assert!(painted_value.get("ghost-node").is_some(), "the live answer is never dropped by the merge");

    let mut answered_value: serde_json::Value = serde_json::from_str(&live).expect("live evaluation json");
    answered_value.as_object_mut().expect("object").insert(outstanding.clone(), serde_json::json!({ "in": {}, "out": {}, "error": "kernel refused" }));
    let answered = serde_json::to_string(&answered_value).expect("answered evaluation json");
    let repainted = merge_unanswered_eval_entries(&answered, &converged, &host.host_snapshot);
    assert_eq!(repainted, answered, "a node the walk ANSWERED keeps its live answer, error included");

    let foreign = serde_json::to_string(&serde_json::json!({ "some-other-document-node": { "out": { "solid": { "handle": "solid-1" } } } })).expect("foreign json");
    let unfilled = merge_unanswered_eval_entries(&live, &foreign, &host.host_snapshot);
    assert_eq!(unfilled, live, "a node this document does not declare is never filled in");
    retire_host(host);
}
//#endregion ⚖️PaintedEvaluation

//#region ⚖️ChainScope
/// ⚖️ LAW: one evaluation hop invalidates the addressed preview, the panels that read the chain's
/// own progress back, and the graph body only when the per-node census moved. Never the whole shell,
/// and never nothing at all.
#[test]
fn a_chain_hop_names_the_preview_and_names_the_graph_only_when_the_census_moved() {
    use semio_framework::kernel::UiDirtyScope;
    let fixture = fixture();
    let scope = &fixture.chain_scope;
    let panels: Vec<&str> = scope.panels.iter().map(String::as_str).collect();
    let graph = scope.only_when_census_moved.first().expect("the fixture names the graph body");
    for census_moved in [false, true] {
        let built = preview_eval::chain_ui_scope(scope.always.first().expect("the fixture names the preview body"), Some(graph.as_str()), &panels, census_moved);
        let UiDirtyScope::Partial { window_bodies, panel_bodies, utilities, tools, engagements, measures, labels } = &built else {
            panic!("a chain hop is always partial, never {built:?}");
        };
        assert!(scope.never_full && scope.never_none, "the fixture states both bounds");
        for body in &scope.always {
            assert!(window_bodies.contains(body), "every hop names {body}");
        }
        assert_eq!(window_bodies.contains(graph), census_moved, "the graph body is named exactly when the census moved");
        assert_eq!(panel_bodies, &scope.panels, "a hop names the panels that read its own progress back");
        assert!(!utilities && !tools && !engagements && !measures && !labels, "no rail of the shell depends on an evaluation hop");
        println!("[STATS] chain scope censusMoved={census_moved} windows={window_bodies:?} panels={panel_bodies:?}");
    }
}
/// ⚖️ LAW: the graph body is re-rendered for the census only where the census changes what it PAINTS
/// — the chrome appearing, the chrome clearing, a fault landing or changing. A reshuffle of which
/// busy node is the active one leaves the marks identical, because that reshuffle is not worth a
/// patch install of the biggest body in the app.
#[test]
fn the_chrome_digest_moves_on_a_fault_or_a_chrome_edge_and_never_on_a_reshuffle() {
    let fixture = fixture();
    let marks = |value: &serde_json::Value| preview_eval::census_chrome_marks(&serde_json::to_string(value).expect("census json"));
    for pair in &fixture.chain_scope.chrome.same_marks {
        assert_eq!(marks(&pair.left), marks(&pair.right), "these two censuses paint the same chrome: {} vs {}", pair.left, pair.right);
    }
    for pair in &fixture.chain_scope.chrome.different_marks {
        assert_ne!(marks(&pair.left), marks(&pair.right), "these two censuses paint different chrome: {} vs {}", pair.left, pair.right);
    }
    println!("[STATS] chrome marks same={} different={}", fixture.chain_scope.chrome.same_marks.len(), fixture.chain_scope.chrome.different_marks.len());
}
//#endregion ⚖️ChainScope
