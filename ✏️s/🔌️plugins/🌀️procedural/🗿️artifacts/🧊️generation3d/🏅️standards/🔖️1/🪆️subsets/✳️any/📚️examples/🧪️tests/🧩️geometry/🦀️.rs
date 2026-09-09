//! 🔬️ Headless example-geometry lane — every bundled generation3d example's real
//! `🖼️assets/<name>/🗣️.dsl.semio` is parsed into its `FlowFixture`, evaluated through the same
//! `FlowHost` the editor's `flow-eval-tick` drives with the packaged `brep`/`math` operator sets
//! installed, and the resulting preview handle is tessellated through the same
//! `tessellate_geometry` bridge the preview path calls. Every number the run produces is held to
//! the committed expected-stats fixture beside the example (`🧪️tests/🧩️example/🔣️.json`), which
//! the TypeScript twin reads too.
//!
//! ⚖️ `parry3d` recomputes volume, centre of mass and bounding box from the same triangle soup, so
//! no committed expectation rests on our own arithmetic alone.
//!
//! @see ../../../../../../📓️kernel-and-preview-audit-2026-09-09.md — the op chain per example.
//! @see ../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs — `FlowHost::evaluate`.

use std::sync::{Mutex, MutexGuard, OnceLock};

use semio_framework_os_flow::neural::Registry;
use semio_framework_job::{allocate_operation_id, CancelToken, Generation, StepBudget, StepContext};
use semio_framework_os_flow::{flow_neuron_kind_infos_json, install_flow_extension, tessellate_geometry, FlowExtensionSpec, FlowHost, FlowHostRetirement};
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
    kernel_status: String,
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
    let Generation3dSnapshot { fixture: graph, generation } = parse_dsl(dsl).expect("example dsl parses");
    generation.retire_cold();
    let mut host = FlowHost::from_fixture(graph);
    host.set_neuron_kind_infos_json(&flow_neuron_kind_infos_json());
    let eval_json = host.evaluate().expect("example evaluates");
    let eval: serde_json::Value = serde_json::from_str(&eval_json).expect("evaluation json");
    let geometry = output_channel(&eval, &fixture.preview.node, &fixture.preview.channel);
    assert_eq!(geometry.get("$schema").and_then(serde_json::Value::as_str), Some("geometry"), "{} preview channel is not a geometry handle: {geometry}", fixture.example);
    assert_eq!(geometry.get("kind").and_then(serde_json::Value::as_str), Some(fixture.preview.kind.as_str()), "{} preview geometry kind", fixture.example);
    let handle = geometry.get("handle").and_then(serde_json::Value::as_str).expect("geometry handle").to_string();
    let mesh = tessellate_geometry(&handle, fixture.tessellation_tolerance).expect("preview tessellates");
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
    ExampleRun { eval, handle, stats, oracle_volume, oracle_bounding_box_min, oracle_bounding_box_max, kernel_volume }
}

/// 🕰️ A frozen clock for the retirement budget — this lane grants the whole close in one page and
/// never wants a deadline yield.
fn frozen_now() -> Option<u64> {
    Some(0)
}

/// 🧹️ Drains a `FlowHost` through its explicit retirement ladder. `FlowFixture`'s ordered maps
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
}
//#endregion 🔖️Harness

//#region 🔖️Examples
#[test]
fn rectangle_wire_preview_evaluates_to_an_open_wire() {
    assert_example(include_str!("../../🪢️rectangle-wire-preview/🖼️assets/🪢️rectangle-wire-preview/🗣️.dsl.semio"), include_str!("../../🪢️rectangle-wire-preview/🧪️tests/🧩️example/🔣️.json"));
}

#[test]
fn rectangle_extrude_volume_evaluates_to_the_analytic_box() {
    assert_example(include_str!("../../📦️rectangle-extrude-volume/🖼️assets/📦️rectangle-extrude-volume/🗣️.dsl.semio"), include_str!("../../📦️rectangle-extrude-volume/🧪️tests/🧩️example/🔣️.json"));
}

#[test]
fn face_sweep_extrude_evaluates_to_a_closed_solid() {
    assert_example(include_str!("../../🧹️face-sweep-extrude/🖼️assets/🧹️face-sweep-extrude/🗣️.dsl.semio"), include_str!("../../🧹️face-sweep-extrude/🧪️tests/🧩️example/🔣️.json"));
}

#[test]
fn hexagonal_mushroom_column_evaluates_to_the_analytic_prism() {
    assert_example(include_str!("../../🍄️hexagonal-mushroom-column/🖼️assets/🍄️hexagonal-mushroom-column/🗣️.dsl.semio"), include_str!("../../🍄️hexagonal-mushroom-column/🧪️tests/🧩️example/🔣️.json"));
}

#[test]
fn box_shell_preview_evaluates_to_a_hollow_solid() {
    assert_example(include_str!("../../🐚️box-shell-preview/🖼️assets/🐚️box-shell-preview/🗣️.dsl.semio"), include_str!("../../🐚️box-shell-preview/🧪️tests/🧩️example/🔣️.json"));
}

#[test]
fn box_fillet_preview_evaluates_to_a_rounded_solid() {
    assert_example(include_str!("../../📐️box-fillet-preview/🖼️assets/📐️box-fillet-preview/🗣️.dsl.semio"), include_str!("../../📐️box-fillet-preview/🧪️tests/🧩️example/🔣️.json"));
}

#[test]
fn sphere_box_fuse_evaluates_to_the_union_volume() {
    assert_example(include_str!("../../🧲️sphere-box-fuse/🖼️assets/🧲️sphere-box-fuse/🗣️.dsl.semio"), include_str!("../../🧲️sphere-box-fuse/🧪️tests/🧩️example/🔣️.json"));
}

#[test]
fn sphere_cut_with_torus_evaluates_to_the_difference_volume() {
    assert_example(include_str!("../../🍩️sphere-cut-with-torus/🖼️assets/🍩️sphere-cut-with-torus/🗣️.dsl.semio"), include_str!("../../🍩️sphere-cut-with-torus/🧪️tests/🧩️example/🔣️.json"));
}
//#endregion 🔖️Examples
