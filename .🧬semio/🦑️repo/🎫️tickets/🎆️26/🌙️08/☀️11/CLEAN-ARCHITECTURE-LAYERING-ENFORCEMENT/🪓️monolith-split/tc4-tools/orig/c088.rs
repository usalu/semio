/// 🚚️ Mesh delivery: the preview mesh delivery through the browser wire, per example.

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
    host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot,
    config: &semio_s_artifact_procedural_3d_editor_config::Generation3dConfig,
    session: &semio_framework_os_flow::FlowEvalSession,
    selected: &[String],
) -> Vec<String> {
    let marks = semio_s_artifact_procedural_3d_mode_edit::windows::preview::PreviewInteractionMarks { source_revision: None, components: Default::default(), hovered: Default::default(), selected: selected.iter().cloned().collect() };
    let payload = semio_s_artifact_procedural_3d_mode_edit::windows::preview::preview_payload(eval_json, host_snapshot, config, Some(session), &marks);
    selection_json_ids(&semio_s_artifact_procedural_3d_mode_edit::windows::preview::preview_selection_json(config, "", &payload))
}

/// 🕹️ The VIEWER window's twin of [`editor_selection_ids`] — a read-only role paints the same pick.
fn viewer_selection_ids(
    eval_json: &str,
    host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot,
    config: &semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfig,
    session: &semio_framework_os_flow::FlowEvalSession,
    selected: &[String],
) -> Vec<String> {
    let marks = semio_s_artifact_procedural_3d_viewer_modes::view::windows::preview::Generation3dViewMarks { hovered: Default::default(), selected: selected.iter().cloned().collect() };
    let payload = semio_s_artifact_procedural_3d_viewer_modes::view::windows::preview::preview_payload(eval_json, host_snapshot, config, Some(session), &marks);
    selection_json_ids(&semio_s_artifact_procedural_3d_viewer_modes::view::windows::preview::preview_selection_json(config, &payload))
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

/// 🧩️ Resolves every painted geometry channel through the supplied Session's retained broker.
fn resolve_preview_geometry(eval_json: &str, graph: &semio_framework_artifact_flow_flow::FlowHostSnapshot, lod_mode: &str, session: &mut semio_framework_os_flow::FlowEvalSession) {
    use semio_s_artifact_procedural_3d_preview_eval::pending_preview_tessellate_handles;
    use semio_s_artifact_procedural_3d_preview_eval::preview_tolerance;
    use semio_s_artifact_procedural_3d_preview_eval::PREVIEW_TESSELLATE_STEP_BUDGET;
    use semio_s_artifact_procedural_3d_preview_eval::PREVIEW_TESSELLATE_STEP_WALL_MICROS;
    let eval = semio_framework_pack_json::parse(eval_json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("preview evaluation parses");
    let tolerance = preview_tolerance(lod_mode);
    for handle in pending_preview_tessellate_handles(&eval, graph, session) {
        let hash = semio_framework_os_flow::preview_tessellate_node_hash(&handle, tolerance.to_bits());
        for _ in 0..4096 {
            if !session.note_pending_tessellate(hash, handle.clone()) { break; }
            let chunk = session.next_tessellate_chunk(hash) as usize;
            let envelope = geometry_session().tessellate_step_envelope_json(&handle, tolerance, PREVIEW_TESSELLATE_STEP_BUDGET as usize, PREVIEW_TESSELLATE_STEP_WALL_MICROS, chunk);
            if !matches!(session.resolve_preview_tessellate(hash, &envelope), semio_framework_os_flow::PreviewTessellateOutcome::Working) { break; }
        }
        assert!(session.preview_mesh_pack(&handle).is_some(), "painted handle {handle} must resolve through its owning Session");
    }
}

/// 🚚️ Evaluates one example and delivers its preview handle's mesh across the extension boundary
/// exactly as the app does, counting every round trip the transfer costs.
fn run_delivery(dsl: &str, fixture: &ExampleGeometryFixture, lod_mode: &str) -> DeliveryRun {
    use semio_s_artifact_procedural_3d_preview_eval::preview_tolerance;
    use semio_s_artifact_procedural_3d_preview_eval::PREVIEW_TESSELLATE_STEP_BUDGET;
    use semio_s_artifact_procedural_3d_preview_eval::PREVIEW_TESSELLATE_STEP_WALL_MICROS;
    operators_installed();
    let Generation3dSnapshot { host_snapshot: graph, generation } = parse_dsl(dsl).expect("example dsl parses");
    generation.retire_cold();
    let mut host = FlowHost::from_host_snapshot(graph).with_history_store_factory(::semio_framework_artifact_flow_flow::FLOW_HISTORY_STORE_FACTORY).with_geometry_port(geometry_session().port());
    host.set_neuron_kind_info_map(flow_neuron_kind_info_map());
    let eval_json = host.evaluate().expect("example evaluates");
    transfer_evaluation_geometry(&host, &eval_json);
    let eval: serde_json::Value = serde_json::from_str(&eval_json).expect("evaluation json");
    let handle = output_channel(&eval, &fixture.preview.node, &fixture.preview.channel).get("handle").and_then(serde_json::Value::as_str).expect("geometry handle").to_string();
    let tolerance = preview_tolerance(lod_mode);
    // 🧼️ The lane above already tessellated this handle at a FINER tolerance, and
    // `cached_mesh_at_or_finer` would serve that cache instead of stepping — so the delivery starts
    // from a cold kernel, the way a freshly evaluated handle does in the app.
    geometry_session().evict_mesh_cache_for_handle(&handle);
    geometry_session().cancel_all_tessellations();
    let node_hash = semio_framework_os_flow::preview_tessellate_node_hash(&handle, tolerance.to_bits());
    let mut session = semio_framework_os_flow::FlowEvalSession::new().with_geometry_port(geometry_session().port());
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
        let envelope = geometry_session().tessellate_step_envelope_json(&handle, tolerance, PREVIEW_TESSELLATE_STEP_BUDGET as usize, PREVIEW_TESSELLATE_STEP_WALL_MICROS, chunk);
        step_micros.push(started.elapsed().as_micros() as u64);
        round_trips += 1;
        let parsed: serde_json::Value = serde_json::from_str(&envelope).expect("envelope json");
        phase = parsed.get("phase").and_then(serde_json::Value::as_str).unwrap_or_default().to_string();
        chunks = chunks.max(parsed.get("chunks").and_then(serde_json::Value::as_u64).unwrap_or_default() as u32);
        assert!(envelope.len() <= semio_framework_os_flow::mesh::tessellate_envelope_maximum_bytes(), "{}: a {} B envelope exceeds the declared transfer unit", fixture.example, envelope.len());
        match session.resolve_preview_tessellate(node_hash, &envelope) {
            semio_framework_os_flow::PreviewTessellateOutcome::Working => continue,
            _ => break,
        }
    }
    let pack = session.preview_mesh_pack(&handle).unwrap_or_default().to_string();
    let mesh = semio_s_artifact_procedural_3d_preview_eval::decode_preview_mesh_pack(&pack);
    let diagnostics = session.preview_diagnostics(&handle).map(str::to_string);
    if fixture.example == "sphere-box-fuse" {
        let data = mesh.as_ref().expect("closed preview mesh");
        let incidence = edge_incidence(&data.positions, &data.indices);
        assert_eq!(incidence.boundary, 0, "{}: preview boundary", fixture.example);
        assert_eq!(incidence.non_manifold, 0, "{}: preview non-manifold edge", fixture.example);
        assert_eq!(incidence.orientation_defects, 0, "{}: preview winding", fixture.example);
        let volume = signed_divergence_volume(&data.positions, &data.indices);
        let oracle = parry_oracle(&data.positions, &data.indices).0;
        assert!(volume > 0.0, "{}: preview volume is outward", fixture.example);
        assert!((volume - oracle).abs() <= volume.abs().max(1.0) * 1e-5, "{}: independent preview volume", fixture.example);
    }
    // 👁️ The publication the preview window actually paints, built from the SAME delivered session
    // — transfer and publication are two different failures and this row separates them.
    resolve_preview_geometry(&eval_json, &host.host_snapshot, lod_mode, &mut session);
    let config = semio_s_artifact_procedural_3d_editor_config::Generation3dConfig { lod_mode: lod_mode.to_string(), ..Default::default() };
    let payload = semio_s_artifact_procedural_3d_mode_edit::windows::preview::preview_payload(&eval_json, &host.host_snapshot, &config, Some(&session), &semio_s_artifact_procedural_3d_mode_edit::windows::preview::PreviewInteractionMarks::default());
    let payload_meshes: serde_json::Value = serde_json::from_str(&payload.meshes_json).expect("payload meshes json");
    let payload_instances: serde_json::Value = serde_json::from_str(&payload.instances_json).expect("payload instances json");
    let published = payload_meshes.as_array().cloned().unwrap_or_default();
    let payload_triangles = published.iter().map(|entry| entry.pointer("/data/indices").and_then(serde_json::Value::as_array).map(Vec::len).unwrap_or_default() / 3).sum::<usize>();
    let payload_edge_segments = published.iter().map(|entry| entry.pointer("/data/edgePositions").and_then(serde_json::Value::as_array).map(Vec::len).unwrap_or_default() / 6).sum::<usize>();
    let mut payload_mesh_roles: BTreeMap<String, usize> = BTreeMap::new();
    for entry in &published {
        *payload_mesh_roles.entry(entry.get("role").and_then(serde_json::Value::as_str).unwrap_or("(unstamped)").to_string()).or_default() += 1;
    }
    let view_config = semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfig { lod_mode: lod_mode.to_string(), ..Default::default() };
    let view_payload = semio_s_artifact_procedural_3d_viewer_modes::view::windows::preview::preview_payload(&eval_json, &host.host_snapshot, &view_config, Some(&session), &Default::default());
    let topology_ids: Vec<String> = payload_instances.as_array().cloned().unwrap_or_default().iter().filter_map(|entry| entry.get("interactionId").and_then(serde_json::Value::as_str).map(str::to_string)).collect();
    let picked_id = format!("{}@{}", fixture.preview.node, fixture.preview.channel);
    let selection_picked_ids = editor_selection_ids(&eval_json, &host.host_snapshot, &config, &session, std::slice::from_ref(&picked_id));
    let selection_added_ids = editor_selection_ids(&eval_json, &host.host_snapshot, &config, &session, &topology_ids);
    let selection_cleared_ids = editor_selection_ids(&eval_json, &host.host_snapshot, &config, &session, &[]);
    let view_selection_picked_ids = viewer_selection_ids(&eval_json, &host.host_snapshot, &view_config, &session, std::slice::from_ref(&picked_id));
    let run = DeliveryRun {
        payload_bounds: semio_s_artifact_procedural_3d_preview_eval::preview_payload_bounds(&payload.meshes_json),
        view_payload_bounds: semio_s_artifact_procedural_3d_preview_eval::preview_payload_bounds(&view_payload.meshes_json),
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
fn assert_delivery(dsl: &str, snapshot_json: &str) {
    let _guard = exclusive();
    let fixture: ExampleGeometryFixture = serde_json::from_str(snapshot_json).expect("expected-stats fixture parses");
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
        assert!(semio_s_artifact_procedural_3d_preview_eval::PREVIEW_MESH_ROLES.contains(&role.as_str()), "{}: {role:?} is not a declared preview mesh role", fixture.example);
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
    const VIEWPORT_HEIGHT_PX: f32 = 900.0;
    for aspect in [1.7f32, 1.0, 0.7] {
        let seed = OrbitController::default();
        let framed = frame_orbit_to_bounds(&seed, minimum, maximum, VIEWPORT_HEIGHT_PX * aspect, VIEWPORT_HEIGHT_PX, WORLD_FRAME_BOUNDS_MARGIN);
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
    assert_delivery(include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🪢️rectangle-wire-preview/🖼️assets/🪢️rectangle-wire-preview/🗣️.dsl.semio"), include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🪢️rectangle-wire-preview/🧪️tests/🧫️fixtures/🧩️example/🔣️.json"));
}

#[test]
fn delivery_rectangle_extrude_volume() {
    assert_delivery(include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📦️rectangle-extrude-volume/🖼️assets/📦️rectangle-extrude-volume/🗣️.dsl.semio"), include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📦️rectangle-extrude-volume/🧪️tests/🧫️fixtures/🧩️example/🔣️.json"));
}

#[test]
fn delivery_face_sweep_extrude() {
    assert_delivery(include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧹️face-sweep-extrude/🖼️assets/🧹️face-sweep-extrude/🗣️.dsl.semio"), include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧹️face-sweep-extrude/🧪️tests/🧫️fixtures/🧩️example/🔣️.json"));
}

#[test]
fn delivery_hexagonal_mushroom_column() {
    assert_delivery(include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🍄️hexagonal-mushroom-column/🖼️assets/🍄️hexagonal-mushroom/🗣️.dsl.semio"), include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🍄️hexagonal-mushroom-column/🧪️tests/🧫️fixtures/🧩️example/🔣️.json"));
}

#[test]
fn delivery_box_shell_preview() {
    assert_delivery(include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🐚️box-shell-preview/🖼️assets/🐚️box-shell-preview/🗣️.dsl.semio"), include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🐚️box-shell-preview/🧪️tests/🧫️fixtures/🧩️example/🔣️.json"));
}

#[test]
fn delivery_box_fillet_preview() {
    assert_delivery(include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📐️box-fillet-preview/🖼️assets/📐️box-fillet-preview/🗣️.dsl.semio"), include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📐️box-fillet-preview/🧪️tests/🧫️fixtures/🧩️example/🔣️.json"));
}

#[test]
fn delivery_sphere_box_fuse() {
    assert_delivery(include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧲️sphere-box-fuse/🖼️assets/🧲️sphere-box-fuse/🗣️.dsl.semio"), include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧲️sphere-box-fuse/🧪️tests/🧫️fixtures/🧩️example/🔣️.json"));
}

#[test]
fn delivery_sphere_cut_with_torus() {
    assert_delivery(include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🍩️sphere-cut-with-torus/🖼️assets/🍩️sphere-cut-with-torus/🗣️.dsl.semio"), include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🍩️sphere-cut-with-torus/🧪️tests/🧫️fixtures/🧩️example/🔣️.json"));
}
//#endregion 🔖️MeshDelivery
