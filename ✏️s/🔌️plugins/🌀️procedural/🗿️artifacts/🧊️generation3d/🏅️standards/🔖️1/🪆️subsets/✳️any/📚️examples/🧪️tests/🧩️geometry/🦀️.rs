//! 🔬️ Headless example-geometry lane — every bundled generation3d example's real
//! `🖼️assets/<name>/🗣️.dsl.semio` is parsed into its `FlowHostDocument`, evaluated through the same
//! `FlowHost` the editor's `flow-eval-tick` drives with the packaged `brep`/`math` operator sets
//! installed, and the resulting preview handle is tessellated through the same
//! `tessellate_geometry` bridge the preview path calls. Every number the run produces is held to
//! the committed expected-stats fixture beside the example (`🧫️fixtures/🧩️example/🔣️.json`), which
//! the TypeScript twin reads too.
//!
//! ⚖️ `parry3d` recomputes volume, centre of mass and bounding box from the same triangle soup, so
//! no committed expectation rests on our own arithmetic alone.
//!
//! @see ../../../../../../📓️kernel-and-preview-audit-2026-09-09.md — the op chain per example.
//! @see ../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs — `FlowHost::evaluate`.

use std::collections::BTreeMap;
use std::sync::{Mutex, MutexGuard, OnceLock};

use semio_framework_os_flow::neural::Registry;
use semio_framework_job::{allocate_operation_id, CancelToken, Generation, StepBudget, StepContext};
use semio_framework_os_flow::{flow_neuron_kind_info_map, install_flow_extension, tessellate_geometry, FlowExtensionSpec, FlowHost, FlowHostRetirement};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::snapshot::text::parse_dsl;
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
use serde::Deserialize;

//#region 🔖️Fixture
/// 📇️ One example's committed expected-geometry statement — the single source both this lane and
/// its TypeScript twin (`🧩️example/🟦️.ts`) read.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExampleGeometryFixture {
    schema: String,
    example: String,
    op_chain: Vec<String>,
    preview: PreviewTarget,
    tessellation_tolerance: f64,
    expect: Expectation,
    delivery: DeliveryExpectation,
    budget: BudgetExpectation,
    kernel_status: String,
}

/// ⏱️ What this example's chain is allowed to COST, in wall microseconds measured on this lane's own
/// native `test` (unoptimized) profile — the third half of "the example works": geometry that is
/// right and delivered but takes 78 s to appear is an example the user never sees converge
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️kernel-performance-2026-09-13.md`).
///
/// ⚖️ These are CEILINGS with deliberate headroom over the measured value, not targets: the point is
/// to convict an algorithmic regression of the class this ticket removed (the boolean kernel's
/// face-stitch spent 99% of `🍩️sphere-cut-with-torus`'s 61 s in arbitrary-precision rational
/// predicates — a 14-35x penalty), never to police a few percent of machine-to-machine variance.
/// A number here is only ever lowered after a measured improvement, never raised to admit a
/// regression.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BudgetExpectation {
    max_evaluate_micros: u64,
    max_tessellate_micros: u64,
    max_preview_tessellate_micros: u64,
}

/// 🚚️ What this example's preview must DELIVER across the extension boundary, at the LOD the live
/// surface asks for — the second half of "the geometry is right": a correct mesh nobody receives is
/// a blank viewport. `max_round_trips` is the load-bearing number, because one round trip is one
/// whole `flowEvalTick` (evaluate → invoke → answer → refresh), seconds apiece in a served build
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️preview-mesh-delivery-2026-09-12.md`).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeliveryExpectation {
    lod_mode: String,
    /// 🏷️ How many of those meshes carry each ROLE, hand-written per example. `meshes` alone cannot
    /// separate "the solid is on screen" from "the solid vanished and its two companion channels
    /// remain": every published mesh stamps a `role` (`preview_eval::preview_mesh_role`) and a
    /// preview that loses its only solid still publishes the wire it was extruded from and the
    /// vector that drove it. The runtime oracles compare against THIS row, per role, which is what
    /// makes a browser mesh census comparable to the native one at all
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-mesh-oracle-2026-09-14.md`).
    mesh_roles: BTreeMap<String, usize>,
    /// 🕸️ The EXACT number of meshes this example's preview publishes. A floor is not enough: a
    /// preview that loses one of three meshes still reports every node `ok`, and the surface simply
    /// paints less — which is how a renamed operator port that stopped feeding `extrude` its axis
    /// reached a browser with a green suite behind it (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    meshes: usize,
    min_triangles: usize,
    min_edge_segments: usize,
    /// 📦️ The extent the DELIVERED payload occupies, hand-written per example from `run_delivery`'s own
    /// `preview_payload`. Deliberately its own row beside `expect.boundingBox*`: `expect` measures the
    /// FINE tessellation this lane computes, the delivery paints the preview LOD, and an inscribed
    /// coarse mesh is legitimately smaller. Without both committed, a browser oracle can only grade the
    /// delivered extent against a stated envelope rather than a number — which is exactly what
    /// `📓️react-oracle-hardening-2026-09-14.md` §1.1 had to do (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// 🎯️ It is also what the preview FRAMES: the render hosts read these bounds off the `fit` lane, so a
    /// wrong number here is a camera that cuts the example off, not merely a weak assertion.
    bounding_box_min: [f64; 3],
    bounding_box_max: [f64; 3],
    bounding_box_tolerance: f64,
    max_round_trips: usize,
    max_chunks: u32,
}

/// 🎯️ Which evaluated node channel carries the geometry the preview renders.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PreviewTarget {
    node: String,
    channel: String,
    kind: String,
}

/// 📐️ The measured quantities a run must reproduce, each with its own stated tolerance.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Expectation {
    min_triangles: usize,
    closed: bool,
    volume: Option<f64>,
    volume_tolerance: f64,
    volume_source: String,
    edge_perimeter: Option<f64>,
    edge_perimeter_tolerance: f64,
    bounding_box_min: [f64; 3],
    bounding_box_max: [f64; 3],
    bounding_box_tolerance: f64,
    kernel_volume_node: Option<String>,
    kernel_volume_channel: Option<String>,
    kernel_volume_tolerance: Option<f64>,
}

const FIXTURE_SCHEMA: &str = "s.procedural.generation3d.example-geometry/v1";

/// ⏱️ The hard bound the two USER-FACING phases sit under — mirrors the TypeScript twin's
/// `EXAMPLE_BUDGET_INTERACTIVE_CEILING_MICROS`. Evaluating the op chain and tessellating at the LOD
/// the live preview asks for are what the playground waits on, and neither may claim more than two
/// seconds of this lane's native `test`-profile wall time: the wasm guest runs the identical code at
/// `opt-level = 2` (root `Cargo.toml`'s `[profile.wasm-dev.package]` overrides) inside a budgeted
/// resumable job, so a phase over this bound cannot converge in the playground within the
/// user-facing target ticket 26/09/09/PROCEDURAL-3D-END-TO-END set.
const BUDGET_INTERACTIVE_CEILING_MICROS: u64 = 2_000_000;

/// ⏱️ The bound on the FIDELITY phase — `tessellate_geometry` at the fixture's own
/// `tessellationTolerance`, which is far finer than any LOD the live preview requests and exists so
/// the committed geometry statement is measured on a converged mesh. Nobody waits on this number in
/// the app, so its ceiling is a regression guard rather than a user-facing promise and is set
/// looser; it still convicts the class of defect this ticket removed, which cost 14-35x.
const BUDGET_FIDELITY_CEILING_MICROS: u64 = 8_000_000;

/// ✅️ Every invariant the `budget` row states about ITSELF, checked before any timing is compared
/// against it — a ceiling nobody bounded is the absence of a budget, and the preview LOD is by
/// construction coarser than the fixture's own tessellation tolerance.
fn assert_budget_contract(fixture: &ExampleGeometryFixture) {
    let budget = &fixture.budget;
    for micros in [budget.max_evaluate_micros, budget.max_preview_tessellate_micros] {
        assert!(micros > 0, "{}: a budget of zero is not a budget", fixture.example);
        assert!(micros <= BUDGET_INTERACTIVE_CEILING_MICROS, "{}: a {micros} us ceiling exceeds the {BUDGET_INTERACTIVE_CEILING_MICROS} us bound every user-facing phase sits under", fixture.example);
    }
    assert!(budget.max_tessellate_micros > 0, "{}: a budget of zero is not a budget", fixture.example);
    assert!(budget.max_tessellate_micros <= BUDGET_FIDELITY_CEILING_MICROS, "{}: a {} us ceiling exceeds the {BUDGET_FIDELITY_CEILING_MICROS} us bound the fidelity phase sits under", fixture.example, budget.max_tessellate_micros);
    assert!(budget.max_preview_tessellate_micros <= budget.max_tessellate_micros, "{}: the preview LOD is coarser than the fixture tolerance, so its ceiling may never exceed the full-tolerance one", fixture.example);
    assert_calibration_contract();
}

/// 🚧️ The machine-readable kernel standings a fixture may declare. Each `blocked-*` value names one
/// SPECIFIC located kernel defect that this lane's run reproduced, never a licence to relax an
/// expectation: the numbers stay exactly what the geometry must be, and the run keeps failing until
/// the named defect is fixed. A defect that gets fixed takes its value out of this list with it —
/// a standing nothing declares is dead vocabulary.
///
/// - `green` — the example's whole chain is exact today.
///
/// `blocked-on-fillet-kernel` was the last such standing and is gone with the defect it named:
/// `diff::blend`'s analytic rewrite (ticket 26/09/09/PROCEDURAL-3D-END-TO-END) mints the corner
/// patches `📐️box-fillet-preview` was missing, so every bundled example is `green`.
const KERNEL_STATUSES: [&str; 1] = ["green"];
//#endregion 🔖️Fixture

//#region ⏱️Calibration
const BUDGET_CALIBRATION_FIXTURE_JSON: &str = include_str!("../../🧫️fixtures/⏱️budget-calibration/🔣️.json");
const CALIBRATION_SCHEMA: &str = "s.procedural.generation3d.example-budget-calibration/v1";

/// ⚖️ The committed statement of how fast the machine that SET this lane's ceilings executes a
/// fixed, kernel-independent workload.
///
/// Every ceiling in every `budget` row, and `maxRoundTrips` too, is a wall-clock quantity measured
/// on one machine under one load. A wall clock on a shared build host measures the code plus
/// whatever else was resident — this repository's fleet routinely holds the load average above 50
/// while peers compile — so the same correct code reads two to five times over its ceiling and the
/// law convicts the machine instead of the algorithm. That is not a strict law, it is a noisy one,
/// and a noisy law gets ignored.
///
/// 🧭 The honest correction is a SCALE, not a looser ceiling: run a workload whose cost this
/// repository never changes, and divide out how much slower it is right now than when the ceilings
/// were set. A genuine algorithmic regression still fails, because it inflates the phase without
/// inflating the calibration; contention inflates both, and cancels
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️kernel-performance-2026-09-13.md` §6.1).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BudgetCalibration {
    schema: String,
    rounds: u32,
    checksum: String,
    reference_micros: u64,
    maximum_load_factor: f64,
}

/// ⏱️ How many timed passes a calibration reading takes, and which of them is believed.
///
/// 🐛️ The MINIMUM is the wrong statistic here, and measuring it proved so: with a pass costing
/// 1.8 ms the min of five passes read 1811 us under 48 spinners on a 10-core machine — byte for byte
/// its idle reading — while `FlowHost::evaluate` on the same machine at the same moment took an
/// order of magnitude longer than its ceiling. A workload short enough to fit inside one scheduling
/// quantum is never preempted, so its minimum measures a machine that is not the one the phase ran
/// on. The pass is therefore sized to ~100 ms — long enough to be preempted the way a real phase is
/// — and the MEDIAN of three is believed, which discards one lucky and one unlucky reading without
/// believing either.
const CALIBRATION_PASSES: usize = 3;

fn budget_calibration() -> BudgetCalibration {
    let calibration: BudgetCalibration = serde_json::from_str(BUDGET_CALIBRATION_FIXTURE_JSON).expect("budget calibration fixture parses");
    assert_eq!(calibration.schema, CALIBRATION_SCHEMA);
    calibration
}

/// ✅️ Every invariant the calibration row states about ITSELF.
fn assert_calibration_contract() {
    let calibration = budget_calibration();
    assert!(calibration.rounds > 0, "a calibration of zero rounds measures nothing");
    assert!(calibration.reference_micros > 0, "a reference of zero microseconds would scale every ceiling to infinity");
    assert!(calibration.maximum_load_factor >= 1.0, "the load factor may only ever widen a ceiling, never narrow one");
    assert_eq!(calibration.checksum.len(), 18, "the checksum is a 64-bit hex literal");
    assert!(calibration.checksum.starts_with("0x"));
}

/// 🎲 ONE deterministic calibration pass: a fixed LCG feeding a `orient2d`-shaped cross-product sum
/// and one owned `Vec` per round, so the pass exercises the same mix the geometry kernel's own cost
/// is made of — floating-point multiply/add chains plus allocator traffic — without touching a line
/// of kernel code that any lane in this repository may legitimately make faster.
///
/// 🪪️ Only the integer state is a checksum. The floating-point accumulator is returned so the work
/// cannot be elided, but never asserted: `a * b + c` may be contracted into a single fused
/// multiply-add on one target and not on another, and a calibration must not fail on that.
fn calibration_pass(rounds: u32) -> (u64, f64) {
    let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
    let mut acc = 0.0f64;
    let mut samples: Vec<f64> = Vec::with_capacity(1024);
    for _ in 0..rounds {
        samples.clear();
        for _ in 0..1024 {
            state = state.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
            samples.push(((state >> 11) as f64) * (1.0 / 9_007_199_254_740_992.0));
        }
        for window in samples.windows(4) {
            acc += window[0] * window[3] - window[1] * window[2];
        }
        let owned: Vec<f64> = samples.iter().map(|sample| sample * 1.000_000_001).collect();
        acc += owned.iter().sum::<f64>();
        state ^= owned.len() as u64;
    }
    (state, acc)
}

/// ⏱️ The machine's CURRENT cost for that workload, in microseconds — the median of
/// [`CALIBRATION_PASSES`] timed passes, each one checked to have produced the committed result.
fn calibration_micros(calibration: &BudgetCalibration) -> u64 {
    let mut readings = Vec::with_capacity(CALIBRATION_PASSES);
    for _ in 0..CALIBRATION_PASSES {
        let started = std::time::Instant::now();
        let (state, acc) = calibration_pass(calibration.rounds);
        let elapsed = started.elapsed().as_micros() as u64;
        std::hint::black_box(acc);
        assert_eq!(format!("{state:#018x}"), calibration.checksum, "the budget calibration workload must be bit-identical wherever it is measured, or its reading is not comparable to the committed reference");
        readings.push(elapsed);
    }
    readings.sort_unstable();
    readings[readings.len() / 2].max(1)
}

/// ⚖️ How much slower this machine is RIGHT NOW than the one that set the ceilings. Never below
/// `1.0` — a faster machine tightens nothing, because the ceilings are regression guards with
/// deliberate headroom, not targets — and never above the row's own `maximumLoadFactor`, past which
/// the reading is contention noise rather than a scale anyone should trust.
fn machine_load_factor() -> f64 {
    let calibration = budget_calibration();
    let measured = calibration_micros(&calibration);
    let raw = measured as f64 / calibration.reference_micros as f64;
    let factor = raw.clamp(1.0, calibration.maximum_load_factor);
    println!("[CALIBRATION] micros={measured} reference={} raw={raw:.2} factor={factor:.2}", calibration.reference_micros);
    factor
}

/// ✅️ Holds one measured phase to its committed ceiling — and, only when the raw reading overruns,
/// to that ceiling scaled by the machine's calibrated speed at that moment. An idle lane pays for no
/// calibration at all; a contended one pays five short passes and is judged on what the code cost
/// rather than on what the machine was doing.
fn assert_phase_budget(example: &str, phase: &str, measured: u64, ceiling: u64, regression: &str) {
    if measured <= ceiling {
        return;
    }
    let factor = machine_load_factor();
    let scaled = (ceiling as f64 * factor) as u64;
    assert!(
        measured <= scaled,
        "{example}: {phase} took {measured} us (best of {TIMING_ATTEMPTS}) against a {ceiling} us ceiling — {scaled} us after this machine's measured {factor:.2}x load calibration — {regression}, see 📓️kernel-performance-2026-09-13.md"
    );
    println!("[BUDGET] {example}: {phase} {measured} us is over its {ceiling} us ceiling but inside the calibrated {scaled} us — this machine is {factor:.2}x slower right now than the one that set it");
}

/// ⚖️ LAW: the calibration itself. Its workload is bit-identical everywhere — otherwise its reading
/// is not comparable to the committed reference and every scaled ceiling is a guess — and the factor
/// it derives never NARROWS a ceiling, so a fast machine can only ever judge the committed numbers
/// as they stand.
///
/// 🪪️ This law is what makes the scaled ceilings auditable: it prints the machine's raw reading, so
/// any lane that reports a budget failure can say in one line whether the machine was the cause.
#[test]
fn the_budget_calibration_is_deterministic_and_never_narrows_a_ceiling() {
    assert_calibration_contract();
    let calibration = budget_calibration();
    let (first, _) = calibration_pass(calibration.rounds);
    let (second, _) = calibration_pass(calibration.rounds);
    assert_eq!(first, second, "the calibration workload must be deterministic within one process");
    assert_eq!(format!("{first:#018x}"), calibration.checksum, "the calibration workload no longer produces the committed result — its reading cannot be compared to the committed reference");
    let factor = machine_load_factor();
    assert!(factor >= 1.0 && factor <= calibration.maximum_load_factor, "the load factor {factor} left its declared range");
}
//#endregion ⏱️Calibration

//#region 🔖️Harness
/// 🔒️ The brep kernel, its mesh cache and the flow extension registry are all process-global, so
/// every example run in this binary takes the same guard rather than racing its siblings.
fn exclusive() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(())).lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// ⏳️ Drives a registration future that is required not to suspend — the packaged extensions'
/// `register` entry points are `async` by repository convention but do no external work.
fn resolve_ready<T>(future: impl std::future::Future<Output = T>) -> T {
    match std::pin::pin!(future).as_mut().poll(&mut std::task::Context::from_waker(std::task::Waker::noop())) {
        std::task::Poll::Ready(value) => value,
        std::task::Poll::Pending => panic!("extension registration must not depend on external work"),
    }
}

/// 🧩️ Installs the two packaged flow extensions every bundled example's `neuron-kind` chain
/// resolves to. `math.vector`/`math.point` are not written out by the math extension — they are
/// auto-derived schema components produced by `Registry::finalize`.
fn install_example_operators(registry: &mut Registry) {
    resolve_ready(semio_s_plugin_flow_extension_brep::register(registry));
    semio_s_plugin_flow_extension_math::register(registry);
}

/// 🌿️ Publishes the operator registry once per test binary.
fn operators_installed() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        install_flow_extension(FlowExtensionSpec { id: "example-geometry".into(), name: "Example Geometry".into(), version: "1".into(), install: install_example_operators }).expect("example geometry operator admission");
    });
}

/// 🧵️ Edge bookkeeping of one tessellated surface.
#[derive(Debug)]
struct EdgeIncidence {
    boundary: usize,
    non_manifold: usize,
    orientation_defects: usize,
}

/// 🧊️ Everything one tessellated preview yields, in the shape the committed expectations speak.
#[derive(Debug)]
struct MeshStats {
    triangles: usize,
    vertices: usize,
    edge_segments: usize,
    edge_length: f64,
    closed: bool,
    boundary_edges: usize,
    non_manifold_edges: usize,
    orientation_defects: usize,
    volume: f64,
    signed_volume: f64,
    bounding_box_min: [f64; 3],
    bounding_box_max: [f64; 3],
}

/// 📊️ One example's full evaluation result.
#[derive(Debug)]
struct ExampleRun {
    eval: serde_json::Value,
    handle: String,
    stats: MeshStats,
    oracle_volume: f64,
    oracle_bounding_box_min: [f64; 3],
    oracle_bounding_box_max: [f64; 3],
    kernel_volume: Option<f64>,
    evaluate_micros: u64,
    tessellate_micros: u64,
}

/// 🚦 Reads a node's output channel out of the flow host's evaluation JSON, naming the node's own
/// error when the chain broke upstream so a kernel regression reports as itself.
fn output_channel<'a>(eval: &'a serde_json::Value, node: &str, channel: &str) -> &'a serde_json::Value {
    let entry = eval.get(node).unwrap_or_else(|| panic!("node {node:?} missing from evaluation json: {eval}"));
    if let Some(error) = entry.get("error").and_then(serde_json::Value::as_str) {
        panic!("node {node:?} failed to evaluate: {error}");
    }
    entry.get("out").and_then(|out| out.get(channel)).unwrap_or_else(|| panic!("node {node:?} has no {channel:?} output: {entry}"))
}

/// 🧮️ Signed volume of a closed triangle soup by the divergence theorem — POSITIVE exactly when the
/// soup's own winding faces outward, so its sign is the compact statement of whether every face is
/// oriented the way a renderer and a mass-property integrator both need. Reported as its own
/// `[STATS]` field beside the magnitude the expectations are stated in.
fn signed_divergence_volume(positions: &[f32], indices: &[u32]) -> f64 {
    let point = |index: u32| {
        let base = index as usize * 3;
        [positions[base] as f64, positions[base + 1] as f64, positions[base + 2] as f64]
    };
    let mut total = 0.0;
    for triangle in indices.chunks_exact(3) {
        let (a, b, c) = (point(triangle[0]), point(triangle[1]), point(triangle[2]));
        let cross = [b[1] * c[2] - b[2] * c[1], b[2] * c[0] - b[0] * c[2], b[0] * c[1] - b[1] * c[0]];
        total += (a[0] * cross[0] + a[1] * cross[1] + a[2] * cross[2]) / 6.0;
    }
    total
}

/// 🧵️ Per-edge incidence of a triangle soup: how many edges are used once (a boundary), more than
/// twice (non-manifold), and how many are traversed the SAME way by both their triangles (an
/// orientation defect — a watertight surface can still be inconsistently wound, which is exactly
/// what flips a preview's normals). Vertices are welded on their quantised position first, because
/// the tessellator emits one vertex per face corner.
fn edge_incidence(positions: &[f32], indices: &[u32]) -> EdgeIncidence {
    use std::collections::HashMap;
    let key = |index: u32| {
        let base = index as usize * 3;
        let quantise = |value: f32| (value as f64 * 1e6).round() as i64;
        (quantise(positions[base]), quantise(positions[base + 1]), quantise(positions[base + 2]))
    };
    type Vertex = (i64, i64, i64);
    let mut counts: HashMap<(Vertex, Vertex), (usize, usize)> = HashMap::new();
    for triangle in indices.chunks_exact(3) {
        for pair in [(triangle[0], triangle[1]), (triangle[1], triangle[2]), (triangle[2], triangle[0])] {
            let (first, second) = (key(pair.0), key(pair.1));
            let forward = first <= second;
            let ordered = if forward { (first, second) } else { (second, first) };
            let entry = counts.entry(ordered).or_default();
            entry.0 += 1;
            if forward {
                entry.1 += 1;
            }
        }
    }
    EdgeIncidence {
        boundary: counts.values().filter(|(total, _)| *total == 1).count(),
        non_manifold: counts.values().filter(|(total, _)| *total > 2).count(),
        orientation_defects: counts.values().filter(|(total, forward)| *total == 2 && *forward != 1).count(),
    }
}

/// 📏️ Total length of the preview's edge polyline buffer (pairs of consecutive points).
fn edge_polyline_length(edge_positions: &[f32]) -> f64 {
    let mut total = 0.0;
    for segment in edge_positions.chunks_exact(6) {
        let delta = [(segment[3] - segment[0]) as f64, (segment[4] - segment[1]) as f64, (segment[5] - segment[2]) as f64];
        total += (delta[0] * delta[0] + delta[1] * delta[1] + delta[2] * delta[2]).sqrt();
    }
    total
}

/// 📦️ Axis-aligned extent of a flat `[x, y, z, …]` buffer.
fn bounds(values: &[f32]) -> ([f64; 3], [f64; 3]) {
    let mut minimum = [f64::INFINITY; 3];
    let mut maximum = [f64::NEG_INFINITY; 3];
    for point in values.chunks_exact(3) {
        for axis in 0..3 {
            minimum[axis] = minimum[axis].min(point[axis] as f64);
            maximum[axis] = maximum[axis].max(point[axis] as f64);
        }
    }
    (minimum, maximum)
}

/// ⚖️ The third-party half: `parry3d` recomputes the same volume and extent from our triangle soup.
fn parry_oracle(positions: &[f32], indices: &[u32]) -> (f64, [f64; 3], [f64; 3]) {
    if indices.len() < 3 {
        return (0.0, [0.0; 3], [0.0; 3]);
    }
    let vertices: Vec<parry3d::na::Point3<f32>> = positions.chunks_exact(3).map(|point| parry3d::na::Point3::new(point[0], point[1], point[2])).collect();
    let triangles: Vec<[u32; 3]> = indices.chunks_exact(3).map(|triangle| [triangle[0], triangle[1], triangle[2]]).collect();
    let properties = parry3d::mass_properties::MassProperties::from_trimesh(1.0, &vertices, &triangles);
    let mesh = parry3d::shape::TriMesh::with_flags(vertices, triangles, parry3d::shape::TriMeshFlags::MERGE_DUPLICATE_VERTICES);
    let aabb = mesh.local_aabb();
    (properties.mass().abs() as f64, [aabb.mins.x as f64, aabb.mins.y as f64, aabb.mins.z as f64], [aabb.maxs.x as f64, aabb.maxs.y as f64, aabb.maxs.z as f64])
}

/// 🚀️ Parses, evaluates and tessellates one example exactly as the editor's preview path does.
fn run_example(dsl: &str, fixture: &ExampleGeometryFixture) -> ExampleRun {
    operators_installed();
    let Generation3dSnapshot { host_document: graph, generation } = parse_dsl(dsl).expect("example dsl parses");
    generation.retire_cold();
    let mut host = FlowHost::from_host_document(graph);
    host.set_neuron_kind_info_map(flow_neuron_kind_info_map());
    let started = std::time::Instant::now();
    let eval_json = host.evaluate().expect("example evaluates");
    let evaluate_micros = started.elapsed().as_micros() as u64;
    let eval: serde_json::Value = serde_json::from_str(&eval_json).expect("evaluation json");
    let geometry = output_channel(&eval, &fixture.preview.node, &fixture.preview.channel);
    assert_eq!(geometry.get("$schema").and_then(serde_json::Value::as_str), Some("geometry"), "{} preview channel is not a geometry handle: {geometry}", fixture.example);
    assert_eq!(geometry.get("kind").and_then(serde_json::Value::as_str), Some(fixture.preview.kind.as_str()), "{} preview geometry kind", fixture.example);
    let handle = geometry.get("handle").and_then(serde_json::Value::as_str).expect("geometry handle").to_string();
    semio_framework_os_flow::brep_geometry::evict_mesh_cache_for_handle(&handle);
    let started = std::time::Instant::now();
    let mesh = tessellate_geometry(&handle, fixture.tessellation_tolerance).expect("preview tessellates");
    let tessellate_micros = started.elapsed().as_micros() as u64;
    let incidence = edge_incidence(&mesh.positions, &mesh.indices);
    let (surface_min, surface_max) = bounds(if mesh.positions.is_empty() { &mesh.edge_positions } else { &mesh.positions });
    let signed_volume = signed_divergence_volume(&mesh.positions, &mesh.indices);
    let stats = MeshStats {
        triangles: mesh.indices.len() / 3,
        vertices: mesh.positions.len() / 3,
        edge_segments: mesh.edge_positions.len() / 6,
        edge_length: edge_polyline_length(&mesh.edge_positions),
        closed: incidence.boundary == 0 && incidence.non_manifold == 0 && !mesh.indices.is_empty(),
        boundary_edges: incidence.boundary,
        non_manifold_edges: incidence.non_manifold,
        orientation_defects: incidence.orientation_defects,
        volume: signed_volume.abs(),
        signed_volume,
        bounding_box_min: surface_min,
        bounding_box_max: surface_max,
    };
    let (oracle_volume, oracle_bounding_box_min, oracle_bounding_box_max) = parry_oracle(&mesh.positions, &mesh.indices);
    let kernel_volume = fixture.expect.kernel_volume_node.as_ref().zip(fixture.expect.kernel_volume_channel.as_ref()).map(|(node, channel)| output_channel(&eval, node, channel).get("value").and_then(serde_json::Value::as_f64).expect("kernel volume value"));
    retire_host(host);
    ExampleRun { eval, handle, stats, oracle_volume, oracle_bounding_box_min, oracle_bounding_box_max, kernel_volume, evaluate_micros, tessellate_micros }
}

/// 🕰️ A frozen clock for the retirement budget — this lane grants the whole close in one page and
/// never wants a deadline yield.
fn frozen_now() -> Option<u64> {
    Some(0)
}

/// 🧹️ Drains a `FlowHost` through its explicit retirement ladder. `FlowHostDocument`'s ordered maps
/// panic on a bare drop ("ordered-map root must be explicitly retired before drop"), so a host is
/// closed, never dropped.
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

/// ✅️ Holds one run to its committed expectations, then to the `parry3d` oracle.
fn assert_example(dsl: &str, fixture_json: &str) {
    let _guard = exclusive();
    let fixture: ExampleGeometryFixture = serde_json::from_str(fixture_json).expect("expected-stats fixture parses");
    assert_eq!(fixture.schema, FIXTURE_SCHEMA, "{} fixture schema", fixture.example);
    assert_budget_contract(&fixture);
    for kind in &fixture.op_chain {
        assert!(dsl.contains(kind.as_str()), "{}: op chain step {kind:?} absent from the dsl", fixture.example);
    }
    let run = run_example(dsl, &fixture);
    let stats = &run.stats;
    println!("[STATS] {} handle={} triangles={} vertices={} edgeSegments={} edgeLength={:.9} closed={} boundary={} nonManifold={} orientationDefects={} volume={:.9} signedVolume={:.9} parryVolume={:.9} kernelVolume={:?} bboxMin={:?} bboxMax={:?} parryBboxMin={:?} parryBboxMax={:?}",
        fixture.example, run.handle, stats.triangles, stats.vertices, stats.edge_segments, stats.edge_length, stats.closed, stats.boundary_edges, stats.non_manifold_edges, stats.orientation_defects, stats.volume, stats.signed_volume, run.oracle_volume, run.kernel_volume, stats.bounding_box_min, stats.bounding_box_max, run.oracle_bounding_box_min, run.oracle_bounding_box_max);
    assert!(stats.triangles >= fixture.expect.min_triangles, "{}: {} triangles, expected at least {}", fixture.example, stats.triangles, fixture.expect.min_triangles);
    assert_eq!(stats.closed, fixture.expect.closed, "{}: closed surface (boundary edges {}, non-manifold edges {})", fixture.example, stats.boundary_edges, stats.non_manifold_edges);
    if fixture.expect.closed {
        assert_eq!(stats.orientation_defects, 0, "{}: {} tessellated edges are traversed the same way by both their triangles — the surface is watertight but not consistently wound, so its preview normals flip", fixture.example, stats.orientation_defects);
    }
    if let Some(expected) = fixture.expect.volume {
        assert!((stats.volume - expected).abs() <= fixture.expect.volume_tolerance, "{}: tessellated volume {} vs expected {} ({}) beyond {}", fixture.example, stats.volume, expected, fixture.expect.volume_source, fixture.expect.volume_tolerance);
        assert!((run.oracle_volume - expected).abs() <= fixture.expect.volume_tolerance, "{}: parry3d volume {} vs expected {} beyond {}", fixture.example, run.oracle_volume, expected, fixture.expect.volume_tolerance);
        if let Some(kernel) = run.kernel_volume {
            let tolerance = fixture.expect.kernel_volume_tolerance.unwrap_or_else(|| panic!("{}: a fixture naming a kernelVolumeNode must state its own kernelVolumeTolerance", fixture.example));
            assert!((kernel - expected).abs() <= tolerance, "{}: kernel brep.measure.volume {} vs expected {} beyond {}", fixture.example, kernel, expected, tolerance);
        }
    }
    if let Some(expected) = fixture.expect.edge_perimeter {
        assert!((stats.edge_length - expected).abs() <= fixture.expect.edge_perimeter_tolerance, "{}: preview edge length {} vs expected {} beyond {}", fixture.example, stats.edge_length, expected, fixture.expect.edge_perimeter_tolerance);
    }
    for axis in 0..3 {
        assert!((stats.bounding_box_min[axis] - fixture.expect.bounding_box_min[axis]).abs() <= fixture.expect.bounding_box_tolerance, "{}: bbox min axis {axis} is {} vs expected {}", fixture.example, stats.bounding_box_min[axis], fixture.expect.bounding_box_min[axis]);
        assert!((stats.bounding_box_max[axis] - fixture.expect.bounding_box_max[axis]).abs() <= fixture.expect.bounding_box_tolerance, "{}: bbox max axis {axis} is {} vs expected {}", fixture.example, stats.bounding_box_max[axis], fixture.expect.bounding_box_max[axis]);
        if !fixture.expect.closed {
            continue;
        }
        assert!((run.oracle_bounding_box_min[axis] - fixture.expect.bounding_box_min[axis]).abs() <= fixture.expect.bounding_box_tolerance, "{}: parry3d bbox min axis {axis} is {}", fixture.example, run.oracle_bounding_box_min[axis]);
        assert!((run.oracle_bounding_box_max[axis] - fixture.expect.bounding_box_max[axis]).abs() <= fixture.expect.bounding_box_tolerance, "{}: parry3d bbox max axis {axis} is {}", fixture.example, run.oracle_bounding_box_max[axis]);
    }
    assert!(!run.eval.as_object().map(|entries| entries.is_empty()).unwrap_or(true), "{}: evaluation json is empty", fixture.example);
    assert!(KERNEL_STATUSES.contains(&fixture.kernel_status.as_str()), "{}: unknown kernel status {:?}", fixture.example, fixture.kernel_status);
    let (evaluate_micros, tessellate_micros) = best_timings(dsl, &fixture, run.evaluate_micros, run.tessellate_micros);
    println!("[BUDGET] {} evaluateMicros={} budget={} tessellateMicros={} budget={}", fixture.example, evaluate_micros, fixture.budget.max_evaluate_micros, tessellate_micros, fixture.budget.max_tessellate_micros);
    assert_phase_budget(&fixture.example, "FlowHost::evaluate", evaluate_micros, fixture.budget.max_evaluate_micros, "the op chain regressed algorithmically");
    assert_phase_budget(&fixture.example, "tessellate_geometry", tessellate_micros, fixture.budget.max_tessellate_micros, "the tessellator regressed algorithmically");
}

/// ⏱️ How many times a phase that overran its ceiling is re-measured before the overrun is believed.
/// The quantity a budget law is about is what the CODE costs, and a wall clock on a shared build
/// machine measures the code plus whatever else was resident — this repository's fleet routinely
/// holds the load average above 50 while peers compile, which inflates a single reading of a
/// single-threaded phase two- to fourfold. The LEAST contended observation is the honest estimate,
/// so an overrun is retried and the minimum is what the ceiling judges: contention is retried away,
/// an algorithmic regression fails every attempt.
const TIMING_ATTEMPTS: usize = 3;

/// ⏱️ The first run's timings, improved by re-running only the phases that overran — a lane where
/// nothing is over its ceiling pays for exactly one run.
fn best_timings(dsl: &str, fixture: &ExampleGeometryFixture, evaluate_micros: u64, tessellate_micros: u64) -> (u64, u64) {
    let (mut evaluate, mut tessellate) = (evaluate_micros, tessellate_micros);
    for _ in 1..TIMING_ATTEMPTS {
        if evaluate <= fixture.budget.max_evaluate_micros && tessellate <= fixture.budget.max_tessellate_micros {
            break;
        }
        let retry = run_example(dsl, fixture);
        evaluate = evaluate.min(retry.evaluate_micros);
        tessellate = tessellate.min(retry.tessellate_micros);
    }
    (evaluate, tessellate)
}
//#endregion 🔖️Harness

//#region 🔖️Examples
#[test]
fn rectangle_wire_preview_evaluates_to_an_open_wire() {
    assert_example(include_str!("../../🪢️rectangle-wire-preview/🖼️assets/🪢️rectangle-wire-preview/🗣️.dsl.semio"), include_str!("../../🪢️rectangle-wire-preview/🧫️fixtures/🧩️example/🔣️.json"));
}

#[test]
fn rectangle_extrude_volume_evaluates_to_the_analytic_box() {
    assert_example(include_str!("../../📦️rectangle-extrude-volume/🖼️assets/📦️rectangle-extrude-volume/🗣️.dsl.semio"), include_str!("../../📦️rectangle-extrude-volume/🧫️fixtures/🧩️example/🔣️.json"));
}

#[test]
fn face_sweep_extrude_evaluates_to_a_closed_solid() {
    assert_example(include_str!("../../🧹️face-sweep-extrude/🖼️assets/🧹️face-sweep-extrude/🗣️.dsl.semio"), include_str!("../../🧹️face-sweep-extrude/🧫️fixtures/🧩️example/🔣️.json"));
}

#[test]
fn hexagonal_mushroom_column_evaluates_to_the_analytic_prism() {
    assert_example(include_str!("../../🍄️hexagonal-mushroom-column/🖼️assets/🍄️hexagonal-mushroom-column/🗣️.dsl.semio"), include_str!("../../🍄️hexagonal-mushroom-column/🧫️fixtures/🧩️example/🔣️.json"));
}

#[test]
fn box_shell_preview_evaluates_to_a_hollow_solid() {
    assert_example(include_str!("../../🐚️box-shell-preview/🖼️assets/🐚️box-shell-preview/🗣️.dsl.semio"), include_str!("../../🐚️box-shell-preview/🧫️fixtures/🧩️example/🔣️.json"));
}

#[test]
fn box_fillet_preview_evaluates_to_a_rounded_solid() {
    assert_example(include_str!("../../📐️box-fillet-preview/🖼️assets/📐️box-fillet-preview/🗣️.dsl.semio"), include_str!("../../📐️box-fillet-preview/🧫️fixtures/🧩️example/🔣️.json"));
}

#[test]
fn sphere_box_fuse_evaluates_to_the_union_volume() {
    assert_example(include_str!("../../🧲️sphere-box-fuse/🖼️assets/🧲️sphere-box-fuse/🗣️.dsl.semio"), include_str!("../../🧲️sphere-box-fuse/🧫️fixtures/🧩️example/🔣️.json"));
}

#[test]
fn sphere_cut_with_torus_evaluates_to_the_difference_volume() {
    assert_example(include_str!("../../🍩️sphere-cut-with-torus/🖼️assets/🍩️sphere-cut-with-torus/🗣️.dsl.semio"), include_str!("../../🍩️sphere-cut-with-torus/🧫️fixtures/🧩️example/🔣️.json"));
}
//#endregion 🔖️Examples

//#region 🌉️SceneBridgeProvenance
/// 🌉️ The framework-side World3d scene-bridge fixture this example's preview payload PRODUCED. It
/// lives beside the bridge it exercises (`♾️infinite/🌍️world/🧫️fixtures/🌉️scene-bridge/🔣️.json`, read by
/// `scene_bridge_renders_the_generation3d_preview_payload_into_a_snapshot`) so the framework never
/// depends on this plugin — and is pinned back to the live pipeline here so it cannot rot.
const SCENE_BRIDGE_FIXTURE: &str = include_str!("../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧫️fixtures/🌉️scene-bridge/🔣️.json");

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SceneBridgeFixture {
    meshes_json: Vec<SceneBridgeMesh>,
    instances_json: Vec<SceneBridgeInstance>,
    camera_json: serde_json::Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SceneBridgeMesh {
    id: String,
    data: SceneBridgeMeshData,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SceneBridgeMeshData {
    #[serde(default)]
    positions: Vec<f32>,
    #[serde(default)]
    indices: Vec<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SceneBridgeInstance {
    id: String,
    mesh_id: String,
}

/// 🌉️ Regenerates `hexagonal-mushroom-column`'s preview payload through the live editor pipeline and
/// holds it to the committed scene-bridge fixture, so the framework-side bridge lane is always
/// asserting against geometry this kernel actually produces today.
#[test]
fn hexagonal_mushroom_column_preview_payload_matches_the_scene_bridge_fixture() {
    let _guard = exclusive();
    operators_installed();
    let expected: SceneBridgeFixture = serde_json::from_str(SCENE_BRIDGE_FIXTURE).expect("scene bridge fixture parses");
    let dsl = include_str!("../../🍄️hexagonal-mushroom-column/🖼️assets/🍄️hexagonal-mushroom-column/🗣️.dsl.semio");
    let Generation3dSnapshot { host_document: graph, generation } = parse_dsl(dsl).expect("example dsl parses");
    generation.retire_cold();
    let cfg = semio_s_artifact_procedural_generation3d::editor::generation3d::config::Generation3dConfig::default();
    let mut host = FlowHost::from_host_document(graph);
    host.set_neuron_kind_info_map(flow_neuron_kind_info_map());
    let eval_json = host.evaluate().expect("example evaluates");
    let (meshes_json, instances_json) = semio_s_artifact_procedural_generation3d::editor::generation3d::preview_payload_from_eval(&eval_json, &host.host_document, &cfg);
    let camera_json = semio_s_artifact_procedural_generation3d::editor::generation3d::preview_camera_json(&cfg);
    retire_host(host);
    let meshes: Vec<SceneBridgeMesh> = serde_json::from_str(&meshes_json).expect("live meshes json");
    let instances: Vec<SceneBridgeInstance> = serde_json::from_str(&instances_json).expect("live instances json");
    let camera: serde_json::Value = serde_json::from_str(&camera_json).expect("live camera json");

    assert_eq!(meshes.iter().map(|mesh| mesh.id.as_str()).collect::<Vec<_>>(), expected.meshes_json.iter().map(|mesh| mesh.id.as_str()).collect::<Vec<_>>(), "preview mesh ids drifted from the committed scene-bridge fixture");
    for (live, committed) in meshes.iter().zip(&expected.meshes_json) {
        assert_eq!(live.data.indices.len(), committed.data.indices.len(), "{}: triangle count drifted", live.id);
        assert_eq!(live.data.positions.len(), committed.data.positions.len(), "{}: vertex count drifted", live.id);
    }
    assert_eq!(
        instances.iter().map(|instance| (instance.id.as_str(), instance.mesh_id.as_str())).collect::<Vec<_>>(),
        expected.instances_json.iter().map(|instance| (instance.id.as_str(), instance.mesh_id.as_str())).collect::<Vec<_>>(),
        "channel-qualified preview instance ids drifted from the committed scene-bridge fixture"
    );
    assert_eq!(camera, expected.camera_json, "preview camera measure drifted from the committed scene-bridge fixture");
}
//#endregion 🌉️SceneBridgeProvenance

//#region 🔖️MeshDelivery
/// 🚚️ One example's preview mesh delivery, driven through the REAL browser wire rather than the
/// in-process `tessellate_geometry` shortcut the lane above uses: the same budgeted
/// `tessellate_step_envelope_json` the `brep` extension answers with, folded by the same
/// `FlowEvalSession::resolve_preview_tessellate` the `flowTessellateResolve` command folds with, at
/// the same LOD tolerance [`preview_tolerance`] gives the live preview.
///
/// ⚖️ Round trips are the measured quantity because each one costs the app a WHOLE `flowEvalTick`
/// (evaluate → invoke → answer → refresh), which is seconds in a served wasm build — a delivery
/// that needs dozens of them never reaches the user's eyes, however correct the mesh is
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️preview-mesh-delivery-2026-09-12.md`).
#[derive(Debug)]
struct DeliveryRun {
    round_trips: usize,
    chunks: u32,
    pack_base64_bytes: usize,
    triangles: usize,
    edge_segments: usize,
    phase: String,
    diagnostics: Option<String>,
    payload_meshes: usize,
    payload_instances: usize,
    payload_edge_segments: usize,
    payload_triangles: usize,
    /// 🏷️ How many published meshes carry each role, keyed by the payload's own `role` stamp.
    payload_mesh_roles: BTreeMap<String, usize>,
    /// 📦️ The editor payload's own extent, and the VIEWER payload's, built from the same delivered
    /// session — one number per role, because the preview a viewer frames must be the preview an editor
    /// frames (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️boot-camera-framing-2026-09-15.md`).
    payload_bounds: Option<([f64; 3], [f64; 3])>,
    view_payload_bounds: Option<([f64; 3], [f64; 3])>,
    step_micros: Vec<u64>,
    /// 🕹️ Every `interactionId` this example's published instances declare — the exact target id
    /// shape a renderer's pick dispatches (`{node}@{channel}`), read off the publication rather than
    /// guessed.
    topology_ids: Vec<String>,
    /// 🕹️ `World3dScene.selection_json`'s `ids` for four framework `graph` selections applied to the
    /// SAME delivered session: the fixture's own preview target picked, every published target
    /// picked (a shift-click that adds), nothing picked (Escape / an empty-space click), and the
    /// viewer window's own projection of the first.
    ///
    /// ⚖️ This is the lane the wgpu renderer reported EMPTY on 7 of 8 examples in both roles while
    /// the guest's own interaction store held the pick: the host dropped the reserved verb's settled
    /// answer, so no refresh ever asked the app to republish it and these ids were never rebuilt
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-selection-roundtrip-2026-09-15.md`).
    selection_picked_ids: Vec<String>,
    selection_added_ids: Vec<String>,
    selection_cleared_ids: Vec<String>,
    view_selection_picked_ids: Vec<String>,
}

/// 🕹️ The `ids` one `World3dScene.selection_json` publishes for a given framework `graph` selection,
/// built through the EDITOR preview window's own projection (`preview_payload` →
/// `preview_selection_json`) on an already-delivered session.
fn editor_selection_ids(
    eval_json: &str,
    fixture: &semio_framework_artifact_flow_flow::FlowHostDocument,
    config: &semio_s_artifact_procedural_generation3d::editor::generation3d::config::Generation3dConfig,
    session: &semio_framework_os_flow::FlowEvalSession,
    selected: &[String],
) -> Vec<String> {
    let marks = semio_s_artifact_procedural_generation3d::editor::generation3d::PreviewInteractionMarks { hovered: Default::default(), selected: selected.iter().cloned().collect() };
    let payload = semio_s_artifact_procedural_generation3d::editor::generation3d::preview_payload(eval_json, fixture, config, Some(session), &marks);
    selection_json_ids(&semio_s_artifact_procedural_generation3d::editor::generation3d::preview_selection_json(config, "", &payload))
}

/// 🕹️ The VIEWER window's twin of [`editor_selection_ids`] — a read-only role paints the same pick.
fn viewer_selection_ids(
    eval_json: &str,
    fixture: &semio_framework_artifact_flow_flow::FlowHostDocument,
    config: &semio_s_artifact_procedural_generation3d::viewer::generation3d::config::Generation3dViewConfig,
    session: &semio_framework_os_flow::FlowEvalSession,
    selected: &[String],
) -> Vec<String> {
    let marks = semio_s_artifact_procedural_generation3d::viewer::generation3d::modes::view::windows::preview::Generation3dViewMarks { hovered: Default::default(), selected: selected.iter().cloned().collect() };
    let payload = semio_s_artifact_procedural_generation3d::viewer::generation3d::modes::view::windows::preview::preview_payload(eval_json, fixture, config, Some(session), &marks);
    selection_json_ids(&semio_s_artifact_procedural_generation3d::viewer::generation3d::modes::view::windows::preview::preview_selection_json(config, &payload))
}

/// 🕹️ The `ids` array of one published `selection_json`.
fn selection_json_ids(selection_json: &str) -> Vec<String> {
    let parsed: serde_json::Value = serde_json::from_str(selection_json).expect("selection json parses");
    parsed.get("ids").and_then(serde_json::Value::as_array).cloned().unwrap_or_default().iter().filter_map(|id| id.as_str().map(str::to_string)).collect()
}

/// 🧹️ Retires a `FlowEvalSession`, which rejects a live drop — the same `begin_close` + granted
/// `close_step` loop production's `FlowInstanceOperationOwner::maintenance_step` runs.
fn retire_eval_session(mut session: semio_framework_os_flow::FlowEvalSession) {
    session.begin_close();
    for _ in 0..1_000_000 {
        match session.close_step(1, 65_536) {
            semio_framework_job::InteractiveJobCloseStep::Pending { .. } => continue,
            semio_framework_job::InteractiveJobCloseStep::Complete => return,
            semio_framework_job::InteractiveJobCloseStep::Blocked => panic!("a positive close grant must never block the evaluation session"),
        }
    }
    panic!("the evaluation session did not reach terminal-empty under a positive close grant");
}

/// 🚚️ Evaluates one example and delivers its preview handle's mesh across the extension boundary
/// exactly as the app does, counting every round trip the transfer costs.
fn run_delivery(dsl: &str, fixture: &ExampleGeometryFixture, lod_mode: &str) -> DeliveryRun {
    use semio_s_artifact_procedural_generation3d::preview_eval::{preview_tolerance, PREVIEW_TESSELLATE_STEP_BUDGET, PREVIEW_TESSELLATE_STEP_WALL_MICROS};
    operators_installed();
    let Generation3dSnapshot { host_document: graph, generation } = parse_dsl(dsl).expect("example dsl parses");
    generation.retire_cold();
    let mut host = FlowHost::from_host_document(graph);
    host.set_neuron_kind_info_map(flow_neuron_kind_info_map());
    let eval_json = host.evaluate().expect("example evaluates");
    let eval: serde_json::Value = serde_json::from_str(&eval_json).expect("evaluation json");
    let handle = output_channel(&eval, &fixture.preview.node, &fixture.preview.channel).get("handle").and_then(serde_json::Value::as_str).expect("geometry handle").to_string();
    let tolerance = preview_tolerance(lod_mode);
    // 🧼️ The lane above already tessellated this handle at a FINER tolerance, and
    // `cached_mesh_at_or_finer` would serve that cache instead of stepping — so the delivery starts
    // from a cold kernel, the way a freshly evaluated handle does in the app.
    semio_framework_os_flow::brep_geometry::evict_mesh_cache_for_handle(&handle);
    semio_framework_os_flow::brep_geometry::cancel_all_tessellations();
    let node_hash = semio_framework_os_flow::preview_tessellate_node_hash(&handle, tolerance.to_bits());
    let mut session = semio_framework_os_flow::FlowEvalSession::new();
    let mut round_trips = 0usize;
    let mut chunks = 0u32;
    let mut phase = String::new();
    let mut step_micros: Vec<u64> = Vec::new();
    for _ in 0..4096 {
        if !session.note_pending_tessellate(node_hash, handle.clone()) {
            break;
        }
        let chunk = session.next_tessellate_chunk(node_hash) as usize;
        let started = std::time::Instant::now();
        let envelope = semio_framework_os_flow::brep_geometry::tessellate_step_envelope_json(&handle, tolerance, PREVIEW_TESSELLATE_STEP_BUDGET as usize, PREVIEW_TESSELLATE_STEP_WALL_MICROS, chunk);
        step_micros.push(started.elapsed().as_micros() as u64);
        round_trips += 1;
        let parsed: serde_json::Value = serde_json::from_str(&envelope).expect("envelope json");
        phase = parsed.get("phase").and_then(serde_json::Value::as_str).unwrap_or_default().to_string();
        chunks = chunks.max(parsed.get("chunks").and_then(serde_json::Value::as_u64).unwrap_or_default() as u32);
        assert!(envelope.len() <= semio_framework_os_flow::brep_geometry::tessellate_envelope_maximum_bytes(), "{}: a {} B envelope exceeds the declared transfer unit", fixture.example, envelope.len());
        match session.resolve_preview_tessellate(node_hash, &envelope) {
            semio_framework_os_flow::PreviewTessellateOutcome::Working => continue,
            _ => break,
        }
    }
    let pack = session.preview_mesh_pack(&handle).unwrap_or_default().to_string();
    let mesh = semio_s_artifact_procedural_generation3d::preview_eval::decode_preview_mesh_pack(&pack);
    let diagnostics = session.preview_diagnostics(&handle).map(str::to_string);
    // 👁️ The publication the preview window actually paints, built from the SAME delivered session
    // — transfer and publication are two different failures and this row separates them.
    let config = semio_s_artifact_procedural_generation3d::editor::generation3d::config::Generation3dConfig { lod_mode: lod_mode.to_string(), ..Default::default() };
    let payload = semio_s_artifact_procedural_generation3d::editor::generation3d::preview_payload(&eval_json, &host.host_document, &config, Some(&session), &semio_s_artifact_procedural_generation3d::editor::generation3d::PreviewInteractionMarks::default());
    let payload_meshes: serde_json::Value = serde_json::from_str(&payload.meshes_json).expect("payload meshes json");
    let payload_instances: serde_json::Value = serde_json::from_str(&payload.instances_json).expect("payload instances json");
    let published = payload_meshes.as_array().cloned().unwrap_or_default();
    let payload_triangles = published.iter().map(|entry| entry.pointer("/data/indices").and_then(serde_json::Value::as_array).map(Vec::len).unwrap_or_default() / 3).sum::<usize>();
    let payload_edge_segments = published.iter().map(|entry| entry.pointer("/data/edgePositions").and_then(serde_json::Value::as_array).map(Vec::len).unwrap_or_default() / 6).sum::<usize>();
    let mut payload_mesh_roles: BTreeMap<String, usize> = BTreeMap::new();
    for entry in &published {
        *payload_mesh_roles.entry(entry.get("role").and_then(serde_json::Value::as_str).unwrap_or("(unstamped)").to_string()).or_default() += 1;
    }
    let view_config = semio_s_artifact_procedural_generation3d::viewer::generation3d::config::Generation3dViewConfig { lod_mode: lod_mode.to_string(), ..Default::default() };
    let view_payload = semio_s_artifact_procedural_generation3d::viewer::generation3d::modes::view::windows::preview::preview_payload(&eval_json, &host.host_document, &view_config, Some(&session), &Default::default());
    let topology_ids: Vec<String> = payload_instances.as_array().cloned().unwrap_or_default().iter().filter_map(|entry| entry.get("interactionId").and_then(serde_json::Value::as_str).map(str::to_string)).collect();
    let picked_id = format!("{}@{}", fixture.preview.node, fixture.preview.channel);
    let selection_picked_ids = editor_selection_ids(&eval_json, &host.host_document, &config, &session, std::slice::from_ref(&picked_id));
    let selection_added_ids = editor_selection_ids(&eval_json, &host.host_document, &config, &session, &topology_ids);
    let selection_cleared_ids = editor_selection_ids(&eval_json, &host.host_document, &config, &session, &[]);
    let view_selection_picked_ids = viewer_selection_ids(&eval_json, &host.host_document, &view_config, &session, std::slice::from_ref(&picked_id));
    let run = DeliveryRun {
        payload_bounds: semio_s_artifact_procedural_generation3d::preview_eval::preview_payload_bounds(&payload.meshes_json),
        view_payload_bounds: semio_s_artifact_procedural_generation3d::preview_eval::preview_payload_bounds(&view_payload.meshes_json),
        payload_meshes: published.len(),
        payload_instances: payload_instances.as_array().map(Vec::len).unwrap_or_default(),
        payload_edge_segments,
        payload_triangles,
        payload_mesh_roles,
        step_micros,
        round_trips,
        chunks,
        pack_base64_bytes: pack.len(),
        triangles: mesh.as_ref().map(|m| m.indices.len() / 3).unwrap_or_default(),
        edge_segments: mesh.as_ref().map(|m| m.edge_positions.len() / 6).unwrap_or_default(),
        phase,
        diagnostics,
        topology_ids,
        selection_picked_ids,
        selection_added_ids,
        selection_cleared_ids,
        view_selection_picked_ids,
    };
    retire_eval_session(session);
    retire_host(host);
    run
}

/// ✅️ The SELECTION round trip's app half, one example at a time, driven by the fixture's own
/// `preview.node`/`preview.channel` — the very target id a renderer's pick dispatches.
///
/// 🕹️ Four laws, all against the same delivered session: a click on the example's own preview target
/// publishes that target in `World3dScene.selection_json` (and its viewer twin publishes the same),
/// a shift-click that names every published target publishes every published instance and never
/// fewer than the single pick did, and an empty selection publishes none — the Escape / empty-space
/// answer. The complementary framework laws live in
/// `💻️os/🔨️modules/🔌️plugin/🧪️tests/🕹️interaction-selection-laws/🦀️.rs` (an example switch prunes the
/// pick, its mirror and its leftover cover) and in
/// `🧑‍🎨engine/🧪️tests/🕹️wgpu-selection-roundtrip/🟦️.ts` (the wgpu host folds the reserved verb's
/// settled answer at all) — together they are the whole round trip
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-selection-roundtrip-2026-09-15.md`).
fn assert_selection_round_trip(fixture: &ExampleGeometryFixture, run: &DeliveryRun) {
    let picked_id = format!("{}@{}", fixture.preview.node, fixture.preview.channel);
    assert!(run.topology_ids.contains(&picked_id), "{}: the publication declares {:?}, none of which is the fixture's own preview target {picked_id}", fixture.example, run.topology_ids);
    for id in &run.topology_ids {
        assert!(id.split_once('@').is_some_and(|(node, channel)| !node.is_empty() && !channel.is_empty()), "{}: {id:?} is not a {{node}}@{{channel}} interaction id", fixture.example);
    }
    assert!(!run.selection_picked_ids.is_empty(), "{}: clicking {picked_id} published no selection id at all — the pick never reaches the scene", fixture.example);
    for id in &run.selection_picked_ids {
        assert!(id.starts_with(&picked_id), "{}: clicking {picked_id} published {id:?}, which belongs to another target", fixture.example);
    }
    assert_eq!(run.selection_added_ids.len(), run.payload_instances, "{}: selecting every published target ({:?}) painted {} of {} instances", fixture.example, run.topology_ids, run.selection_added_ids.len(), run.payload_instances);
    for id in &run.selection_picked_ids {
        assert!(run.selection_added_ids.contains(id), "{}: shift-clicking must ADD to the pick, but {id:?} is missing from {:?}", fixture.example, run.selection_added_ids);
    }
    assert!(run.selection_added_ids.len() >= run.selection_picked_ids.len(), "{}: the added selection {:?} is smaller than the single pick {:?}", fixture.example, run.selection_added_ids, run.selection_picked_ids);
    assert!(run.selection_cleared_ids.is_empty(), "{}: an empty selection still published {:?}", fixture.example, run.selection_cleared_ids);
    assert_eq!(run.view_selection_picked_ids, run.selection_picked_ids, "{}: the viewer window paints {:?} for the same pick the editor paints {:?} for", fixture.example, run.view_selection_picked_ids, run.selection_picked_ids);
}

/// ✅️ Holds one example's delivery to its committed `delivery` row.
fn assert_delivery(dsl: &str, fixture_json: &str) {
    let _guard = exclusive();
    let fixture: ExampleGeometryFixture = serde_json::from_str(fixture_json).expect("expected-stats fixture parses");
    assert_budget_contract(&fixture);
    let run = run_delivery(dsl, &fixture, &fixture.delivery.lod_mode.clone());
    println!("[DELIVERY-BOUNDS] {} editPayload={:?} viewPayload={:?}", fixture.example, run.payload_bounds, run.view_payload_bounds);
    println!("[DELIVERY] {} roundTrips={} chunks={} packBase64Bytes={} triangles={} edgeSegments={} phase={} diagnostics={:?} payloadMeshes={} payloadInstances={} payloadTriangles={} payloadEdgeSegments={} payloadMeshRoles={:?} stepMicros={:?} totalMicros={}", fixture.example, run.round_trips, run.chunks, run.pack_base64_bytes, run.triangles, run.edge_segments, run.phase, run.diagnostics, run.payload_meshes, run.payload_instances, run.payload_triangles, run.payload_edge_segments, run.payload_mesh_roles, run.step_micros, run.step_micros.iter().sum::<u64>());
    println!(
        "[SELECTION] {} picked={:?} topology={:?} pickedIds={:?} addedIds={:?} clearedIds={:?} viewerPickedIds={:?}",
        fixture.example,
        format!("{}@{}", fixture.preview.node, fixture.preview.channel),
        run.topology_ids,
        run.selection_picked_ids,
        run.selection_added_ids,
        run.selection_cleared_ids,
        run.view_selection_picked_ids
    );
    assert_selection_round_trip(&fixture, &run);
    let delivery = &fixture.delivery;
    assert_eq!(run.diagnostics, None, "{}: the validate gate rejected the preview solid", fixture.example);
    assert_eq!(run.phase, "complete", "{}: the delivery ended in phase {:?}", fixture.example, run.phase);
    assert!(run.triangles >= delivery.min_triangles, "{}: {} delivered triangles, expected at least {}", fixture.example, run.triangles, delivery.min_triangles);
    assert!(run.edge_segments >= delivery.min_edge_segments, "{}: {} delivered edge segments, expected at least {}", fixture.example, run.edge_segments, delivery.min_edge_segments);
    // 👁️ A wire preview has NO triangles and must still paint: its polyline rides the same `pack`
    // body in `edgePositions`, and `mesh_has_preview_geometry` admits it on that alone.
    assert!(run.triangles > 0 || run.edge_segments > 0, "{}: the delivered mesh is empty", fixture.example);
    assert_eq!(run.payload_meshes, delivery.meshes, "{}: the preview published {} meshes, the example delivers exactly {}", fixture.example, run.payload_meshes, delivery.meshes);
    assert_eq!(run.payload_instances, delivery.meshes, "{}: every published mesh owes exactly one preview instance", fixture.example);
    assert_eq!(run.payload_mesh_roles, delivery.mesh_roles, "{}: the preview published {:?} mesh roles, the example delivers {:?}", fixture.example, run.payload_mesh_roles, delivery.mesh_roles);
    assert_eq!(delivery.mesh_roles.values().sum::<usize>(), delivery.meshes, "{}: the committed role counts {:?} do not add up to the committed mesh count {}", fixture.example, delivery.mesh_roles, delivery.meshes);
    assert!(delivery.mesh_roles.get(&fixture.preview.kind).copied().unwrap_or_default() >= 1, "{}: the fixture's own preview kind {:?} carries no published mesh", fixture.example, fixture.preview.kind);
    for role in delivery.mesh_roles.keys() {
        assert!(semio_s_artifact_procedural_generation3d::preview_eval::PREVIEW_MESH_ROLES.contains(&role.as_str()), "{}: {role:?} is not a declared preview mesh role", fixture.example);
    }
    assert!(run.payload_triangles >= delivery.min_triangles, "{}: the published payload carries {} triangles, expected at least {}", fixture.example, run.payload_triangles, delivery.min_triangles);
    assert!(run.payload_edge_segments >= delivery.min_edge_segments, "{}: the published payload carries {} edge segments, expected at least {}", fixture.example, run.payload_edge_segments, delivery.min_edge_segments);
    assert!(run.chunks <= delivery.max_chunks, "{}: the mesh body crossed in {} chunks, budget {}", fixture.example, run.chunks, delivery.max_chunks);
    assert_delivery_bounds(&fixture, &run);
    assert_delivered_bounds_frame_inside_the_viewport(&fixture);
    let (preview_micros, round_trips) = best_delivery(dsl, &fixture, &run);
    assert_phase_budget(&fixture.example, "the preview LOD tessellation", preview_micros, fixture.budget.max_preview_tessellate_micros, "a step that overruns the kernel's own budget here is seconds in a served wasm build");
    assert_round_trip_budget(&fixture.example, round_trips, delivery.max_round_trips);
}

/// 📦️ Holds the delivered extent — the number the render hosts FRAME — to its committed row, in both
/// roles. A preview whose payload is a different size than the fixture says is a preview the boot
/// camera frames wrongly, and nothing else in this lane can see it: `expect.boundingBox*` grades the
/// fine tessellation, not what crosses the extension boundary
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️boot-camera-framing-2026-09-15.md`).
fn assert_delivery_bounds(fixture: &ExampleGeometryFixture, run: &DeliveryRun) {
    let delivery = &fixture.delivery;
    for (role, measured) in [("edit", run.payload_bounds), ("view", run.view_payload_bounds)] {
        let Some((minimum, maximum)) = measured else {
            panic!("{}: the {role} preview payload published no extent at all", fixture.example);
        };
        for axis in 0..3 {
            assert!(
                (minimum[axis] - delivery.bounding_box_min[axis]).abs() <= delivery.bounding_box_tolerance,
                "{}: {role} delivered bbox min axis {axis} is {} vs committed {}",
                fixture.example,
                minimum[axis],
                delivery.bounding_box_min[axis]
            );
            assert!(
                (maximum[axis] - delivery.bounding_box_max[axis]).abs() <= delivery.bounding_box_tolerance,
                "{}: {role} delivered bbox max axis {axis} is {} vs committed {}",
                fixture.example,
                maximum[axis],
                delivery.bounding_box_max[axis]
            );
            assert!(
                delivery.bounding_box_min[axis] >= fixture.expect.bounding_box_min[axis] - fixture.expect.bounding_box_tolerance && delivery.bounding_box_max[axis] <= fixture.expect.bounding_box_max[axis] + fixture.expect.bounding_box_tolerance,
                "{}: the committed DELIVERY extent reaches past the committed EXPECT extent on axis {axis} — a coarse tessellation inscribes a fine one, it never exceeds it",
                fixture.example
            );
        }
    }
}

/// 🎯️ The boot-framing law: the camera the render hosts derive from this example's committed delivery
/// bounds puts every corner of that box inside the viewport, with margin, at every viewport shape a
/// user plausibly has — and it is the SAME rule the React host runs
/// (`world3dFrameDistanceForRadius`/`frame_distance_for_radius`). A rule that merely moves the camera
/// closer is not a framing: the defect this law convicts is a converged example clipped at the corner
/// of the preview (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️boot-camera-framing-2026-09-15.md`).
fn assert_delivered_bounds_frame_inside_the_viewport(fixture: &ExampleGeometryFixture) {
    use semio_framework_ui::wgpu::{frame_orbit_to_bounds, OrbitController, WORLD_FRAME_BOUNDS_MARGIN};
    let delivery = &fixture.delivery;
    let minimum = [delivery.bounding_box_min[0] as f32, delivery.bounding_box_min[1] as f32, delivery.bounding_box_min[2] as f32];
    let maximum = [delivery.bounding_box_max[0] as f32, delivery.bounding_box_max[1] as f32, delivery.bounding_box_max[2] as f32];
    for aspect in [1.7f32, 1.0, 0.7] {
        let seed = OrbitController::default();
        let framed = frame_orbit_to_bounds(&seed, minimum, maximum, aspect, WORLD_FRAME_BOUNDS_MARGIN);
        let camera = framed.to_camera();
        let half_vertical = (camera.fov_y * 0.5).tan();
        let forward = [camera.target.x - camera.position.x, camera.target.y - camera.position.y, camera.target.z - camera.position.z];
        let forward_length = (forward[0] * forward[0] + forward[1] * forward[1] + forward[2] * forward[2]).sqrt();
        let unit_forward = [forward[0] / forward_length, forward[1] / forward_length, forward[2] / forward_length];
        let right = [unit_forward[1] * camera.up.z - unit_forward[2] * camera.up.y, unit_forward[2] * camera.up.x - unit_forward[0] * camera.up.z, unit_forward[0] * camera.up.y - unit_forward[1] * camera.up.x];
        let right_length = (right[0] * right[0] + right[1] * right[1] + right[2] * right[2]).sqrt();
        let unit_right = [right[0] / right_length, right[1] / right_length, right[2] / right_length];
        let unit_up = [
            unit_right[1] * unit_forward[2] - unit_right[2] * unit_forward[1],
            unit_right[2] * unit_forward[0] - unit_right[0] * unit_forward[2],
            unit_right[0] * unit_forward[1] - unit_right[1] * unit_forward[0],
        ];
        for corner in 0..8 {
            let point = [
                if corner & 1 == 0 { minimum[0] } else { maximum[0] },
                if corner & 2 == 0 { minimum[1] } else { maximum[1] },
                if corner & 4 == 0 { minimum[2] } else { maximum[2] },
            ];
            let relative = [point[0] - camera.position.x, point[1] - camera.position.y, point[2] - camera.position.z];
            let depth = relative[0] * unit_forward[0] + relative[1] * unit_forward[1] + relative[2] * unit_forward[2];
            assert!(depth > 0.0, "{}: corner {corner} sits behind the framed camera at aspect {aspect}", fixture.example);
            let ndc_y = (relative[0] * unit_up[0] + relative[1] * unit_up[1] + relative[2] * unit_up[2]) / (depth * half_vertical);
            let ndc_x = (relative[0] * unit_right[0] + relative[1] * unit_right[1] + relative[2] * unit_right[2]) / (depth * half_vertical * aspect);
            assert!(ndc_x.abs() <= 1.0 && ndc_y.abs() <= 1.0, "{}: corner {corner} projects to ({ndc_x}, {ndc_y}) at aspect {aspect} — outside the viewport", fixture.example);
        }
    }
}

/// ⏱️ The delivery's two WALL-DERIVED readings, improved by re-running only when one of them
/// overran — a lane where nothing is over budget pays for exactly one delivery.
///
/// 🪪️ `round_trips` belongs here beside the microseconds because it IS a wall-clock quantity:
/// `tessellate_step_envelope_json` keeps calling `tessellate_step` "while the job is still working
/// AND the deadline has not passed" (`🧰️framework/…/🌊️flow/📐️brep-geometry/🦀️.rs:721`), so a machine
/// that runs the kernel half as fast fits half as many steps into one round trip's
/// `TESSELLATE_STEP_WALL_MICROS` and the same unchanged geometry costs more round trips. Counting
/// them is right — one round trip is one whole `flowEvalTick` — but judging a single contended
/// count is the same mistake as judging a single contended microsecond reading.
fn best_delivery(dsl: &str, fixture: &ExampleGeometryFixture, first: &DeliveryRun) -> (u64, usize) {
    let (mut micros, mut round_trips) = (first.step_micros.iter().sum::<u64>(), first.round_trips);
    for _ in 1..TIMING_ATTEMPTS {
        if micros <= fixture.budget.max_preview_tessellate_micros && round_trips <= fixture.delivery.max_round_trips {
            break;
        }
        let retry = run_delivery(dsl, fixture, &fixture.delivery.lod_mode.clone());
        micros = micros.min(retry.step_micros.iter().sum::<u64>());
        round_trips = round_trips.min(retry.round_trips);
    }
    (micros, round_trips)
}

/// ✅️ Holds the delivery to its committed round-trip budget, and — only when the best of
/// [`TIMING_ATTEMPTS`] deliveries still overruns — to that budget scaled by the machine's calibrated
/// speed, for the reason [`best_delivery`] states: the count is produced by a wall-clock deadline
/// inside the kernel, so a slower machine buys fewer kernel steps per round trip.
fn assert_round_trip_budget(example: &str, round_trips: usize, ceiling: usize) {
    if round_trips <= ceiling {
        return;
    }
    let factor = machine_load_factor();
    let scaled = (ceiling as f64 * factor).ceil() as usize;
    assert!(
        round_trips <= scaled,
        "{example}: the preview cost {round_trips} tessellate round trips (best of {TIMING_ATTEMPTS}), budget {ceiling} — {scaled} after this machine's measured {factor:.2}x load calibration — one round trip is one whole flowEvalTick"
    );
    println!("[BUDGET] {example}: {round_trips} tessellate round trips is over its budget of {ceiling} but inside the calibrated {scaled} — this machine is {factor:.2}x slower right now than the one that set it");
}

#[test]
fn delivery_rectangle_wire_preview() {
    assert_delivery(include_str!("../../🪢️rectangle-wire-preview/🖼️assets/🪢️rectangle-wire-preview/🗣️.dsl.semio"), include_str!("../../🪢️rectangle-wire-preview/🧫️fixtures/🧩️example/🔣️.json"));
}

#[test]
fn delivery_rectangle_extrude_volume() {
    assert_delivery(include_str!("../../📦️rectangle-extrude-volume/🖼️assets/📦️rectangle-extrude-volume/🗣️.dsl.semio"), include_str!("../../📦️rectangle-extrude-volume/🧫️fixtures/🧩️example/🔣️.json"));
}

#[test]
fn delivery_face_sweep_extrude() {
    assert_delivery(include_str!("../../🧹️face-sweep-extrude/🖼️assets/🧹️face-sweep-extrude/🗣️.dsl.semio"), include_str!("../../🧹️face-sweep-extrude/🧫️fixtures/🧩️example/🔣️.json"));
}

#[test]
fn delivery_hexagonal_mushroom_column() {
    assert_delivery(include_str!("../../🍄️hexagonal-mushroom-column/🖼️assets/🍄️hexagonal-mushroom-column/🗣️.dsl.semio"), include_str!("../../🍄️hexagonal-mushroom-column/🧫️fixtures/🧩️example/🔣️.json"));
}

#[test]
fn delivery_box_shell_preview() {
    assert_delivery(include_str!("../../🐚️box-shell-preview/🖼️assets/🐚️box-shell-preview/🗣️.dsl.semio"), include_str!("../../🐚️box-shell-preview/🧫️fixtures/🧩️example/🔣️.json"));
}

#[test]
fn delivery_box_fillet_preview() {
    assert_delivery(include_str!("../../📐️box-fillet-preview/🖼️assets/📐️box-fillet-preview/🗣️.dsl.semio"), include_str!("../../📐️box-fillet-preview/🧫️fixtures/🧩️example/🔣️.json"));
}

#[test]
fn delivery_sphere_box_fuse() {
    assert_delivery(include_str!("../../🧲️sphere-box-fuse/🖼️assets/🧲️sphere-box-fuse/🗣️.dsl.semio"), include_str!("../../🧲️sphere-box-fuse/🧫️fixtures/🧩️example/🔣️.json"));
}

#[test]
fn delivery_sphere_cut_with_torus() {
    assert_delivery(include_str!("../../🍩️sphere-cut-with-torus/🖼️assets/🍩️sphere-cut-with-torus/🗣️.dsl.semio"), include_str!("../../🍩️sphere-cut-with-torus/🧫️fixtures/🧩️example/🔣️.json"));
}
//#endregion 🔖️MeshDelivery
