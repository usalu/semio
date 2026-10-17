//! 🧪️ host tests (🖱️pointer-gestures): compiled against the `testkit` surface of the world crates.

use semio_framework_os_infinite_world_model::{WORLD_INTERACTION_ID_BYTE_CAPACITY, WORLD_INTERACTION_OBJECT_CAPACITY, WORLD_INTERACTION_TOPOLOGY_BYTE_CAPACITY, world_mesh_ownership_grant, WorldCatalogueDropPreviewRecord, WorldInteractionObjectKind, WorldInteractionStep};
use semio_framework_os_infinite_world_components::World3dRelocateSession;
use semio_framework_os_infinite_world_cursors::{render_pick_viewport, resolved_domain_id, selection_method_wire_str, WorldMarqueeGesture, WorldMarqueePublishJob, WorldMarqueeResult, WorldMarqueeResultPages, WorldRayPickCursor, WorldRayPickPurpose};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_infinite_world_scene::TerrainMeshOracleData;
use semio_framework_os_infinite_world_bridge::{world3d_scene_instance_interactions_are_bounded, World3dDynamicRetirement, World3dSceneInstanceEntry};
use semio_framework_os_infinite_world_interaction::{INTERACTION_TARGETS_EMPTY, plan_world3d_camera_settle, plan_world3d_wheel, publish_world3d_plan_step, store_mesh, world3d_begin_relocate_drag, world3d_cancel_relocate_drag, world3d_end_relocate_drag, world3d_relocate_active, world3d_relocate_mode, world_merge_code};
use semio_framework_os_infinite_world_host::World3dState;
use ui_wgpu::wgpu::{ActionDescriptor, Instance3d, Mat4, mesh3d_allocate_step, mesh3d_begin, mesh3d_seal, mesh3d_write_edge, mesh3d_write_u32, mesh3d_write_vec3, mesh3d_write_vec4, Mesh3dField, Mesh3dLease, Mesh3dSchema, Mesh3dWriteToken, SceneDraw3d, Vec3};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework::SelectionMethod;

/// 📖️ Reads one test input at run time (relative to the package manifest) so it does not count as an embedded compile input.
fn read_static(path_from_manifest: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path_from_manifest);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("test input {}: {error}", path.display())).into_boxed_str())
}

/// 📖️ Reads several test inputs at run time and joins them into the one text the unsplit sources formed.
fn read_joined(paths: &[&str]) -> &'static str {
    let mut joined = String::new();
    for path in paths {
        joined.push_str(read_static(path));
    }
    Box::leak(joined.into_boxed_str())
}

pub(crate) fn take_actions(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Vec<ActionDescriptor> {
    let mut actions = Vec::new();
    while let Some(action) = input.take_action_step().expect("action authority live") {
        actions.push(action.into_descriptor().expect("bounded action materializes"));
    }
    actions
}

pub(crate) fn mesh_oracle_from_buffers(positions: Vec<f32>, normals: Vec<f32>, indices: Vec<u32>) -> TerrainMeshOracleData {
    TerrainMeshOracleData { positions, normals, indices, face_ids: Vec::new(), vertex_ids: Vec::new(), edge_positions: Vec::new(), edge_ids: Vec::new(), uvs: Vec::new(), colors: Vec::new() }
}

pub(crate) fn publish_oracle_mesh(data: TerrainMeshOracleData) -> Mesh3dLease {
    publish_oracle_mesh_at_revision(data, 0)
}

/// 🧊️ An oracle mesh stamped with a chosen interaction revision — what
/// `publish_world3d_asset_mesh_lease` witnesses before it stores a decoded GLB under its url's id.
pub(crate) fn publish_oracle_mesh_at_revision(data: TerrainMeshOracleData, revision: u64) -> Mesh3dLease {
    assert!(data.positions.len().is_multiple_of(3));
    assert_eq!(data.normals.len(), data.positions.len());
    assert!(data.edge_positions.len().is_multiple_of(6));
    assert!(data.uvs.len().is_multiple_of(2));
    assert!(data.colors.len().is_multiple_of(4));
    static GENERATION: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(10_000);
    let schema = Mesh3dSchema {
        vertices: (data.positions.len() / 3) as u32,
        indices: data.indices.len() as u32,
        face_ids: data.face_ids.len() as u32,
        vertex_ids: data.vertex_ids.len() as u32,
        edges: (data.edge_positions.len() / 6) as u32,
        edge_ids: data.edge_ids.len() as u32,
        uvs: (data.uvs.len() / 2) as u32,
        colors: (data.colors.len() / 4) as u32,
    surface_uvs:[0;4],tangents:0,};
    let generation = GENERATION.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let grant=world_mesh_ownership_grant(0,ui_wgpu::wgpu::mesh3d_begin_capacity_byte_demand(),0,1);
    let (token,progress)=mesh3d_begin(generation,revision,schema,grant).expect("oracle mesh claim");
    assert!(progress.fits(grant));
    allocate_oracle_mesh(token);
    for value in data.positions.as_chunks::<3>().0 {
        mesh3d_write_vec3(token, Mesh3dField::Positions, *value).unwrap();
    }
    for value in data.normals.as_chunks::<3>().0 {
        mesh3d_write_vec3(token, Mesh3dField::Normals, *value).unwrap();
    }
    for value in data.indices {
        mesh3d_write_u32(token, Mesh3dField::Indices, value).unwrap();
    }
    for value in data.face_ids {
        mesh3d_write_u32(token, Mesh3dField::FaceIds, value).unwrap();
    }
    for value in data.vertex_ids {
        mesh3d_write_u32(token, Mesh3dField::VertexIds, value).unwrap();
    }
    for value in data.edge_positions.as_chunks::<6>().0 {
        mesh3d_write_edge(token, [value[..3].try_into().unwrap(), value[3..].try_into().unwrap()]).unwrap();
    }
    for value in data.edge_ids {
        mesh3d_write_u32(token, Mesh3dField::EdgeIds, value).unwrap();
    }
    for value in data.uvs.as_chunks::<2>().0 {
        ui_wgpu::wgpu::mesh3d_write_vec2(token, Mesh3dField::Uvs, *value).unwrap();
    }
    for value in data.colors.as_chunks::<4>().0 {
        ui_wgpu::wgpu::mesh3d_write_vec4(token, Mesh3dField::Colors, *value).unwrap();
    }
    mesh3d_seal(token).expect("oracle mesh publication")
}

pub(crate) fn with_world_step_context<T>(fuel: u64, step: impl FnOnce(&mut semio_framework_job::StepContext<'_>) -> T) -> T {
    let mut sequence = 0;
    let grant=world_original_fixture_grant();
    let mut receipt=semio_framework_value::RetainedCloneProgress::default();
    let mut context = semio_framework_job::StepContext::new(
        semio_framework_job::OperationId(1),
        semio_framework_job::Generation(1),
        semio_framework_job::StepBudget::new(fuel, u64::MAX,grant),
        semio_framework_job::root_cancel_token(),
        semio_framework_job::default_now_us,
        &mut sequence,
        &mut receipt,
    );
    let result=step(&mut context);
    assert!(context.retained_progress().fits(grant));
    result
}

fn allocate_oracle_mesh(token:Mesh3dWriteToken) {
    loop {
        let grant=world_mesh_ownership_grant(0,ui_wgpu::wgpu::mesh3d_allocate_capacity_byte_demand(token).unwrap(),0,1);
        let step=mesh3d_allocate_step(token,grant).unwrap();
        assert!(step.progress.fits(grant));
        if step.complete{break}
    }
}

fn world_original_fixture_grant()->semio_framework_value::RetainedCloneGrant{
    let law:serde_json::Value=serde_json::from_str(read_static("../../../🧪️tests/🔬️unit/🧫️retained-attributes.json")).unwrap();let axes=&law["grant"];
    semio_framework_value::RetainedCloneGrant{maximum_items:axes["items"].as_u64().unwrap()as usize,maximum_copy_bytes:axes["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:axes["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:axes["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:axes["depth"].as_u64().unwrap()as usize}
}

#[allow(non_snake_case)]
fn POINTER_GESTURES() -> &'static str {
    read_static("../../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/🖱️pointer-gestures.json")
}

fn fixture() -> serde_json::Value {
    serde_json::from_str(POINTER_GESTURES()).expect("pointer gesture fixture parses")
}

fn gesture(id: &str) -> serde_json::Value {
    fixture()["gestures"].as_array().expect("gesture list").iter().find(|case| case["id"] == id).unwrap_or_else(|| panic!("fixture case {id}")).clone()
}

/// 🧫️ A `World3dState` carrying the fixture's own scene: its mesh, both rendered instances, their
/// shared topology target and the bound interaction domain.
fn fixture_state() -> World3dState {
    let scene = fixture()["scene"].clone();
    let mut state = World3dState::new(scene["surfaceId"].as_str().expect("surfaceId").into(), scene["controllerId"].as_str().expect("controllerId").into());
    state.session.bound_domain_id = Some(scene["domainId"].as_str().expect("domainId").into());
    state.session.bound_domain_granularity_id = Some(scene["domainGranularityId"].as_str().expect("domainGranularityId").into());
    let numbers = |value: &serde_json::Value| value.as_array().expect("number list").iter().map(|entry| entry.as_f64().expect("number") as f32).collect::<Vec<f32>>();
    let mesh = scene["meshes"].as_array().expect("mesh list")[0].clone();
    let data = mesh_oracle_from_buffers(numbers(&mesh["data"]["positions"]), numbers(&mesh["data"]["normals"]), mesh["data"]["indices"].as_array().expect("index list").iter().map(|entry| entry.as_u64().expect("index") as u32).collect());
    let mesh_key = mesh["id"].as_str().expect("mesh id").to_string();
    store_mesh(&mut state.session, &mut state.interaction, &mut state.authority, &mut state.scene, mesh_key.clone(), publish_oracle_mesh(data));
    let mesh_version = *state.scene.mesh_versions.get(&mesh_key).expect("mesh version");
    let mut instances = Vec::new();
    for instance in scene["instances"].as_array().expect("instance list") {
        let id = instance["id"].as_str().expect("instance id").to_string();
        let target = instance["interactionId"].as_str().expect("instance interactionId").to_string();
        let position = numbers(&instance["position"]);
        let mut model = Mat4::identity();
        model.cols[3][0] = position[0];
        model.cols[3][1] = position[1];
        model.cols[3][2] = position[2];
        state.interaction.instance_interaction_ids.insert(id.clone(), target);
        if let Some(granularity) = instance["interactionGranularityId"].as_str() {
            state.interaction.instance_interaction_granularity_ids.insert(id.clone(), granularity.to_string());
        }
        instances.push(Instance3d { component_source: None, id, model, color: [1.0; 4], selected: false, hovered: false, material: Default::default() });
    }
    state.scene.draws.push(SceneDraw3d { mesh_key, mesh_version, instances, shadow_role: Default::default() });
    state
}

/// 🎯️ A ray straight down onto one rendered instance, in the mesh's own world space — the fixture's
/// quad lies in the `z = 0` plane, so `-Z` from above hits whichever instance the model sits under.
fn instance_ray(state: &World3dState, instance_id: &str) -> (Vec3, Vec3) {
    let instance = state.scene.draws.iter().flat_map(|draw| draw.instances.iter()).find(|instance| instance.id == instance_id).expect("rendered instance");
    let translation = instance.model.cols[3];
    (Vec3::new(translation[0], translation[1], translation[2] + 4.0), Vec3::new(0.0, 0.0, -1.0))
}

fn ray_cursor(state: &World3dState, purpose: WorldRayPickPurpose, origin: Vec3, direction: Vec3, merge: u8) -> WorldRayPickCursor {
    WorldRayPickCursor {
        revision: state.session.interaction_revision,
        generation: 1,
        purpose,
        origin,
        direction,
        local_x: 0.0,
        local_y: 0.0,
        viewport: render_pick_viewport(&state.session),
        view_projection: ui_wgpu::wgpu::projection_spec_view_proj(&state.camera.orbit.to_camera(), state.camera.projection_spec, render_pick_viewport(&state.session).w, render_pick_viewport(&state.session).h),
        projection_spec: state.camera.projection_spec,
        draw: 0,
        instance: 0,
        primitive: 0,
        mesh: None,
        mesh_probe: 0,
        merge,
        best: None,
        pick_target: 0,
        best_target: None,
        complete: false,
        faulted: false,
    }
}

/// 🏁️ Drives one bounded pick to completion and returns the descriptor it published, if any.
fn publish_pick(state: &mut World3dState, mut cursor: WorldRayPickCursor) -> Option<ActionDescriptor> {
    let mut turns = 0;
    while with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 1, context)) != WorldInteractionStep::Complete {
        turns += 1;
        assert!(turns < 256, "bounded pick terminates");
    }
    let mut plan = cursor.finish_plan(&state.session, &state.selection, &state.interaction, &state.scene, 1).expect("live pick")?;
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut turns = 0;
    while with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut plan, 1, &mut input, context)).expect("bounded publication") != WorldInteractionStep::Complete {
        turns += 1;
        assert!(turns < 64, "bounded publication terminates");
    }
    take_actions(&mut input).into_iter().next()
}

fn arg<'a>(action: &'a ActionDescriptor, key: &str) -> &'a str {
    action.args.as_ref().and_then(|args| args.get(key)).and_then(|value| value.as_str()).unwrap_or_else(|| panic!("action {} carries a text arg {key}, got {:?}", action.action, action.args))
}

/// 🎯️ The `targets` arg as the framework's own decoder reads it: `DslValue::as_str` then
/// `serde_json`. A structured array answers `as_str` with `None`, which is exactly the defect.
fn decoded_targets(action: &ActionDescriptor) -> serde_json::Value {
    serde_json::from_str(arg(action, "targets")).expect("targets is JSON text")
}

#[test]
fn the_instance_interaction_carrier_is_schema_bounded_and_retires_one_entry_per_step() {
    let scene = fixture()["scene"].clone();
    let parsed: Vec<World3dSceneInstanceEntry> = serde_json::from_value(scene["instances"].clone()).expect("shared instance records satisfy the native carrier");
    assert!(scene["instances"].to_string().len() <= WORLD_INTERACTION_TOPOLOGY_BYTE_CAPACITY, "the encoded fixture is inside the existing topology-byte authority");
    assert!(world3d_scene_instance_interactions_are_bounded(&parsed));
    assert_eq!(parsed[0].interaction_granularity_id, None, "the first instance exercises scene fallback");
    assert_eq!(parsed[1].interaction_granularity_id.as_deref(), Some("node"), "the second instance carries the schema-first override");

    let mut overlong = parsed.clone();
    overlong[0].interaction_granularity_id = Some("g".repeat(WORLD_INTERACTION_ID_BYTE_CAPACITY + 1));
    assert!(!world3d_scene_instance_interactions_are_bounded(&overlong), "an override beyond the existing interaction-id byte authority is refused");
    let mut overfull = vec![parsed[0].clone(); WORLD_INTERACTION_OBJECT_CAPACITY + 1];
    assert!(!world3d_scene_instance_interactions_are_bounded(&overfull), "an instance carrier beyond the existing interaction-object authority is refused");
    overfull.clear();

    let mut state = fixture_state();
    let initial = state.interaction.instance_interaction_ids.len() + state.interaction.instance_interaction_granularity_ids.len();
    let mut retirement = World3dDynamicRetirement { phase: 6, blocked: None };
    let mut steps = 0;
    while !with_world_step_context(1,|context|retirement.step(&mut state.selection, &mut state.interaction, &mut state.scene, &mut state.bridge,context)) {
        steps += 1;
        assert!(steps <= initial + 3, "interaction carrier retirement remains one entry or phase transition per step");
    }
    assert_eq!(steps, initial + 3);
    assert!(state.interaction.instance_interaction_ids.is_empty() && state.interaction.instance_interaction_granularity_ids.is_empty());
    assert!(retirement.terminal_is_empty());
}

/// 🔍️ The pre-fix shape used the RENDER id and hardcoded `object`; both must differ
/// from the fixture's declared topology target on every case.
#[test]
fn an_instance_pick_selects_the_topology_target_at_its_declared_granularity() {
    for id in ["instance-pick-replaces", "instance-pick-additive", "instance-pick-subtractive", "instance-pick-subtractive-on-command", "instance-pick-invertive", "second-instance-of-one-topology-id-pick"] {
        let case = gesture(id);
        let expect = case["expect"].clone();
        let mut state = fixture_state();
        let (origin, direction) = instance_ray(&state, case["instanceId"].as_str().expect("instanceId"));
        let merge = world_merge_code(case["modifiers"]["shiftKey"].as_bool().unwrap_or(false), case["modifiers"]["ctrlKey"].as_bool().unwrap_or(false), case["modifiers"]["metaKey"].as_bool().unwrap_or(false));
        let cursor = ray_cursor(&state, WorldRayPickPurpose::Instance, origin, direction, merge);
        let action = publish_pick(&mut state, cursor).unwrap_or_else(|| panic!("{id} publishes an action"));
        assert_eq!(action.action, expect["action"].as_str().expect("action id"), "{id}");
        assert_eq!(arg(&action, "domainId"), expect["domainId"].as_str().expect("domainId"), "{id}");
        assert_eq!(arg(&action, "merge"), expect["merge"].as_str().expect("merge"), "{id}");
        assert_eq!(arg(&action, "method"), expect["method"].as_str().expect("method"), "{id}");
        assert_eq!(decoded_targets(&action), expect["targets"], "{id}");

        let render_id = case["instanceId"].as_str().expect("instanceId");
        let topology_id = expect["targets"][0]["id"].as_str().expect("expected target id");
        assert_ne!(render_id, topology_id, "{id}: the fixture's own scene renders the target under a different id");
        assert_ne!(expect["targets"][0]["granularity"].as_str().expect("granularity"), "object", "{id}: the old hardcoded pick granularity cannot satisfy the fixture");
    }
}

#[test]
fn an_instance_hover_reports_the_resolved_granularity_on_the_pointer_channel() {
    for id in ["instance-hover", "instance-hover-override"] {
        let case = gesture(id);
        let expect = case["expect"].clone();
        let mut state = fixture_state();
        let (origin, direction) = instance_ray(&state, case["instanceId"].as_str().expect("instanceId"));
        let cursor = ray_cursor(&state, WorldRayPickPurpose::Hover, origin, direction, 0);
        let action = publish_pick(&mut state, cursor).expect("hover publishes an action");
        assert_eq!(action.action, expect["action"].as_str().expect("action id"), "{id}");
        assert_eq!(arg(&action, "domainId"), expect["domainId"].as_str().expect("domainId"), "{id}");
        assert_eq!(arg(&action, "channel"), expect["channel"].as_str().expect("channel"), "{id}");
        assert_eq!(decoded_targets(&action), expect["targets"], "{id}");
        assert_eq!(state.selection.local_hover_id.as_deref(), expect["targets"][0]["id"].as_str(), "{id}: the local marker is the topology target");
        assert_eq!(state.selection.local_hover_granularity_id.as_deref(), expect["targets"][0]["granularity"].as_str(), "{id}: local hover retains the exact target pair");
    }
}

#[test]
fn a_background_click_clears_with_an_empty_target_list_rather_than_a_silence() {
    let case = gesture("background-click-clears");
    let expect = case["expect"].clone();
    let mut state = fixture_state();
    let cursor = ray_cursor(&state, WorldRayPickPurpose::Instance, Vec3::new(500.0, 500.0, 4.0), Vec3::new(0.0, 0.0, -1.0), 0);
    let action = publish_pick(&mut state, cursor).expect("a background click still dispatches");
    assert_eq!(action.action, expect["action"].as_str().expect("action id"));
    assert_eq!(arg(&action, "merge"), expect["merge"].as_str().expect("merge"));
    assert_eq!(arg(&action, "method"), expect["method"].as_str().expect("method"));
    assert_eq!(decoded_targets(&action), expect["targets"]);
}

#[test]
fn a_marquee_release_replaces_with_the_deduplicated_topology_targets() {
    let case = gesture("marquee-release-replaces");
    let expect = case["expect"].clone();
    let mut state = fixture_state();
    state.session.interaction_revision = 3;
    state.interaction.interaction_objects.revision = 3;
    let mut results = WorldMarqueeResultPages::default();
    for instance in state.scene.draws[0].instances.clone() {
        let token = state.interaction.interaction_objects.admit(3, WorldInteractionObjectKind::Instance, &instance.id, None, instance.model, [0.0; 8], None).expect("marquee object token");
        assert!(results.push(WorldMarqueeResult::Object(token), instance.id.len()), "both rendered instances are admitted");
    }
    let gesture_state = WorldMarqueeGesture::new(3, 1, [0.0, 0.0]);
    let mut job = WorldMarqueePublishJob::new(2, gesture_state, results, false, false);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut turns = 0;
    while with_world_step_context(1, |context| job.step(&state.session, &state.interaction, 2, &mut input, context)).expect("bounded marquee publication") != WorldInteractionStep::Complete {
        turns += 1;
        assert!(turns < 128, "bounded marquee publication terminates");
    }
    let action = take_actions(&mut input).into_iter().next().expect("marquee publishes an action");
    assert_eq!(action.action, expect["action"].as_str().expect("action id"));
    assert_eq!(arg(&action, "domainId"), expect["domainId"].as_str().expect("domainId"));
    assert_eq!(arg(&action, "merge"), expect["merge"].as_str().expect("merge"));
    assert_eq!(decoded_targets(&action), expect["targets"], "only exact granularity/id pairs collapse; one id under two granularities remains two targets");
}

/// 🧭️ The zoom itself publishes nothing — the wire report is the SETTLE's, one per gesture, the
/// trailing debounce React's `dispatchWorldCameraDebounced` keeps (`WorldCameraSync`).
///
/// 🪟️ `windowId`, never `surfaceId`: the shell resolves `ActionAddress::window_instance_id` from
/// this argument, and a World3d surface IS keyed by its window instance id.
#[test]
fn a_camera_gesture_addresses_the_window_that_owns_the_surface() {
    let case = gesture("orbit-completes-into-one-setcamera");
    let mut state = fixture_state();
    let mut plan = plan_world3d_wheel(&state.session, &state.camera, 1, 200.0).expect("bounded wheel plan");
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut turns = 0;
    while with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut plan, 1, &mut input, context)).expect("bounded publication") != WorldInteractionStep::Complete {
        turns += 1;
        assert!(turns < 64, "bounded publication terminates");
    }
    assert!(take_actions(&mut input).is_empty(), "a navigation step never publishes per move");
    let mut settle = plan_world3d_camera_settle(&state.session, &state.camera, 2).expect("a moved camera owes one settle");
    let mut turns = 0;
    while with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut settle, 2, &mut input, context)).expect("bounded publication") != WorldInteractionStep::Complete {
        turns += 1;
        assert!(turns < 64, "bounded settle terminates");
    }
    let published = take_actions(&mut input);
    assert_eq!(published.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), vec!["noteWorldNavigation", "setCamera"], "the un-debounced navigation report lands ahead of the debounced camera report, exactly as React journals it");
    let action = published.into_iter().nth(1).expect("a camera gesture publishes its report");
    assert_eq!(action.action, case["expect"]["action"].as_str().expect("action id"));
    assert_eq!(arg(&action, "windowId"), fixture()["scene"]["surfaceId"].as_str().expect("surfaceId"));
    assert!(action.args.as_ref().and_then(|args| args.get("surfaceId")).is_none(), "the pre-fix `surfaceId` address is gone, not merely joined by `windowId`");
    let camera = action.args.as_ref().and_then(|args| args.get("camera")).expect("the pose nests under `camera`");
    assert!(
        camera.get("position").is_some() && camera.get("target").is_some() && camera.get("zoom").is_some() && camera.get("up").is_some(),
        "the pose carries React's `{{position, target, zoom, up}}` — never `fov`, which the guest camera value has no member for"
    );
    assert!(camera.get("fov").is_none(), "`fov` is gone from the wire, not merely joined by `zoom`");
    println!("pointer-gestures orbit-completes-into-one-setcamera: windowId={} camera={:?}", arg(&action, "windowId"), camera);
}

/// 🧮️ A marquee page with NO targets still writes the two bytes of `[]`, and its reservation says so.
///
/// ⚖️ `targets` became JSON TEXT, so the array's own punctuation is part of the action's byte
/// credits. `page_credit` charged the delimiters PER TARGET, which is zero for an empty page — and
/// the two-byte shortfall surfaced on the LAST string of the action (`method`), faulting the whole
/// publish. Measured on 6118 as `world3d interaction … leave step=Fault` with
/// `active=MarqueePublish[page=0 stage=5 targets=2b]`
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-world3d-interaction-2026-09-13.md`).
#[test]
fn an_empty_marquee_page_reserves_the_bytes_of_its_own_empty_array() {
    let state = fixture_state();
    let job = WorldMarqueePublishJob::new(2, WorldMarqueeGesture::new(state.session.interaction_revision, 1, [0.0, 0.0]), WorldMarqueeResultPages::default(), false, false);
    let base = ui_wgpu::wgpu::checked_action_string_bytes(&[state.session.controller_id.as_str(), "interactionSelect", "domainId", resolved_domain_id(&state.session), "targets", "merge", job.merge, "method", selection_method_wire_str(SelectionMethod::Rectangle)])
        .expect("base action bytes");
    let credit = job.page_credit(&state.session, &state.interaction, 0).expect("empty page credit");
    assert_eq!(credit - base, INTERACTION_TARGETS_EMPTY.len(), "an empty page reserves exactly the `[]` it writes");
    println!("pointer-gestures empty-marquee-credit: base={base} credit={credit} array={}", INTERACTION_TARGETS_EMPTY.len());
}

/// ⎋️ React binds Escape as a CAPTURE-phase `keydown` while `relocateMode` is on and calls
/// `endRelocateDrag(null)`, which drops the ghost and dispatches NOTHING
/// (`🌐️World3dHost/🟦️.tsx:6873-6881`). It answers `false` when no drag was live, so the key falls
/// through to the chord table.
#[test]
fn escape_cancels_a_live_relocate_drag_and_commits_nothing() {
    let mut state = fixture_state();
    assert!(!world3d_cancel_relocate_drag(&mut state.session, &mut state.camera, &mut state.gumball, &mut state.scene), "with no live drag Escape is not consumed");
    state.gumball.relocate = Some(World3dRelocateSession { object_id: "o1".into(), origin: [0.0, 0.0, 0.0], from: [0.0, 0.0, 0.0] });
    state.gumball.catalogue_drop_preview = Some(WorldCatalogueDropPreviewRecord { object_kind: "o1".into(), mesh_url: None, origin: [1.0, 1.0, 0.0] });
    assert!(world3d_cancel_relocate_drag(&mut state.session, &mut state.camera, &mut state.gumball, &mut state.scene), "a live drag consumes the key");
    assert!(!world3d_relocate_active(&state.gumball), "the session is gone");
    assert!(state.gumball.catalogue_drop_preview.is_none(), "the ghost is gone");
    assert!(world3d_end_relocate_drag(&mut state.session, &mut state.camera, &mut state.gumball, &mut state.scene, Some((0.0, 0.0))).is_none(), "a cancelled drag has nothing left to commit");
}

/// 🚚️ `activeUtility === "worldRelocate"` is React's `relocateMode`, and it is the ONLY state in
/// which a press arms the gesture.
#[test]
fn only_the_world_relocate_utility_arms_the_gesture() {
    let mut state = fixture_state();
    state.session.active_utility = "select".into();
    assert!(!world3d_relocate_mode(&state.session));
    assert!(!world3d_begin_relocate_drag(&mut state.session, &mut state.camera, &mut state.selection, &mut state.gumball, &mut state.scene, 10.0, 10.0), "the select utility never arms a relocate");
    state.session.active_utility = "worldRelocate".into();
    assert!(world3d_relocate_mode(&state.session));
}
//#endregion WorldRelocateGestureLaws
