/// 🌉️ Scene bridge: the provenance law of the framework-side scene-bridge fixture.

//#region 🌉️SceneBridgeProvenance
/// 🌉️ The framework-side World3d scene-bridge fixture this example's preview payload PRODUCED. It
/// lives beside the bridge it exercises (`♾️infinite/🌍️world/🧫️fixtures/🌉️scene-bridge/🔣️.json`, read by
/// `scene_bridge_renders_the_generation3d_preview_payload_into_a_snapshot`) so the framework never
/// depends on this plugin — and is pinned back to the live pipeline here so it cannot rot.
const SCENE_BRIDGE_FIXTURE: &str = include_str!("../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🧫️fixtures/🌉️scene-bridge/🔣️.json");

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
    let dsl = include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🍄️hexagonal-mushroom-column/🖼️assets/🍄️hexagonal-mushroom/🗣️.dsl.semio");
    let Generation3dSnapshot { host_snapshot: graph, generation } = parse_dsl(dsl).expect("example dsl parses");
    generation.retire_cold();
    let cfg = semio_s_artifact_procedural_3d_editor_config::Generation3dConfig::default();
    let mut host = FlowHost::from_host_snapshot(graph).with_history_store_factory(::semio_framework_artifact_flow_flow::FLOW_HISTORY_STORE_FACTORY).with_geometry_port(geometry_session().port());
    host.set_neuron_kind_info_map(flow_neuron_kind_info_map());
    let eval_json = host.evaluate().expect("example evaluates");
    transfer_evaluation_geometry(&host, &eval_json);
    let mut session = semio_framework_os_flow::FlowEvalSession::new().with_geometry_port(geometry_session().port());
    resolve_preview_geometry(&eval_json, &host.host_snapshot, &cfg.lod_mode, &mut session);
    let payload = semio_s_artifact_procedural_3d_mode_edit::windows::preview::preview_payload(&eval_json, &host.host_snapshot, &cfg, Some(&session), &Default::default());
    let (meshes_json, instances_json) = (payload.meshes_json, payload.instances_json);
    retire_eval_session(session);
    let camera_json = semio_s_artifact_procedural_3d_mode_edit::windows::preview::preview_camera_json(&cfg);
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
