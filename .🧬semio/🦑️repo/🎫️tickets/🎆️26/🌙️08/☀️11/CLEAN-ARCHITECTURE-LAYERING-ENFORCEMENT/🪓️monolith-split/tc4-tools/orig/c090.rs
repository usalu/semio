/// 🎟️ Unbounded retained grant: the pre-migration step budget carried no retained limit.
const UNBOUNDED_STEP_GRANT: semio_framework_job::RetainedCloneGrant = semio_framework_job::RetainedCloneGrant { maximum_items: usize::MAX, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX };
/// 🧰️ Harness: the serial lock, the evaluation run, the parry oracle and the timing helpers.

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
    semio_s_plugin_flow_brep::register(registry, geometry_session());
    semio_s_plugin_flow_math::register(registry);
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

/// 🔗️ Transfers produced geometry to the evaluated host before releasing temporary producer claims.
fn transfer_evaluation_geometry(host: &FlowHost, eval_json: &str) {
    fn collect(value: &serde_json::Value, handles: &mut Vec<String>) {
        if let Some(handle) = value.get("handle").and_then(serde_json::Value::as_str) { handles.push(handle.to_string()); }
        match value {
            serde_json::Value::Object(entries) => for entry in entries.values() { collect(entry, handles); },
            serde_json::Value::Array(entries) => for entry in entries { collect(entry, handles); },
            _ => {},
        }
    }
    let mut handles = Vec::new();
    collect(&serde_json::from_str(eval_json).expect("evaluation json"), &mut handles);
    host.geometry_port().expect("supplied geometry authority").retain(&handles);
    geometry_session().retain_geometry_handles(&[]);
}

/// 🚀️ Parses, evaluates and tessellates one example exactly as the editor's preview path does.
fn run_example(dsl: &str, fixture: &ExampleGeometryFixture) -> ExampleRun {
    operators_installed();
    let Generation3dSnapshot { host_snapshot: graph, generation } = parse_dsl(dsl).expect("example dsl parses");
    generation.retire_cold();
    let mut host = FlowHost::from_host_snapshot(graph).with_history_store_factory(::semio_framework_artifact_flow_flow::FLOW_HISTORY_STORE_FACTORY).with_geometry_port(geometry_session().port());
    host.set_neuron_kind_info_map(flow_neuron_kind_info_map());
    let started = std::time::Instant::now();
    let eval_json = host.evaluate().expect("example evaluates");
    transfer_evaluation_geometry(&host, &eval_json);
    let evaluate_micros = started.elapsed().as_micros() as u64;
    let eval: serde_json::Value = serde_json::from_str(&eval_json).expect("evaluation json");
    let geometry = output_channel(&eval, &fixture.preview.node, &fixture.preview.channel);
    assert_eq!(geometry.get("$schema").and_then(serde_json::Value::as_str), Some("geometry"), "{} preview channel is not a geometry handle: {geometry}", fixture.example);
    assert_eq!(geometry.get("kind").and_then(serde_json::Value::as_str), Some(fixture.preview.kind.as_str()), "{} preview geometry kind", fixture.example);
    let handle = geometry.get("handle").and_then(serde_json::Value::as_str).expect("geometry handle").to_string();
    geometry_session().evict_mesh_cache_for_handle(&handle);
    let started = std::time::Instant::now();
    let mesh = geometry_session().tessellate_geometry(&handle, fixture.tessellation_tolerance).expect("preview tessellates");
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

/// 🧹️ Drains a `FlowHost` through its explicit retirement ladder. `FlowHostSnapshot`'s ordered maps
/// panic on a bare drop ("ordered-map root must be explicitly retired before drop"), so a host is
/// closed, never dropped.
fn retire_host(host: FlowHost) {
    let mut sequence = 0;
    let mut retirement = FlowHostRetirement::new(host);
    for _ in 0..1_000_000 {
        let mut context_retained = semio_framework_job::RetainedCloneProgress::default();
        let mut context = StepContext::new(allocate_operation_id(), Generation(0), StepBudget::new(u64::MAX, u64::MAX, UNBOUNDED_STEP_GRANT), CancelToken::root_now(), frozen_now, &mut sequence, &mut context_retained);
        if retirement.close_step(&mut context) {
            return;
        }
    }
    panic!("flow host retirement did not reach terminal-empty");
}

/// ✅️ Holds one run to its committed expectations, then to the `parry3d` oracle.
fn assert_example(dsl: &str, snapshot_json: &str) {
    let _guard = exclusive();
    let fixture: ExampleGeometryFixture = serde_json::from_str(snapshot_json).expect("expected-stats fixture parses");
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
