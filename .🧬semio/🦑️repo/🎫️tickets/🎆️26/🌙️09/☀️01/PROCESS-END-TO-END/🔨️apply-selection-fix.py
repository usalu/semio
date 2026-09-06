#!/usr/bin/env python3
"""🖱️ Threads the live "geometry" selection into the process3d workpiece viewport.

Splits the memoized preview payload so only the EXPENSIVE half (the CSG-replayed mesh) stays in
`Process3dPreviewCache`; the cheap instance record — which is the only thing selection affects — is
rebuilt per render. Keying the memo on selection instead would replay the whole process on every
click, on a path already suspected of crossing INTERACTIVE_STEP_CEILING_US (8ms).
"""
import io, sys, os

ROOT = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(ROOT, "..", "..", "..", "..", "..", "..", ".."))
SUB = "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any"
W = os.path.join(REPO, SUB, "✏️editor/🎭️modes/✏️edit/🪟️windows/🪚️workpiece/🦀️.rs")
E = os.path.join(REPO, SUB, "✏️editor/🦀️.rs")

def sub(path, old, new, label):
    s = io.open(path, encoding="utf-8").read()
    n = s.count(old)
    if n != 1:
        raise SystemExit(f"ABORT {label}: expected 1 occurrence, found {n} in {path}")
    io.open(path, "w", encoding="utf-8").write(s.replace(old, new))
    print(f"  ok  {label}")

print("patching", W)

# 1. split the payload builder
sub(W,
'''fn evaluated_preview_payload(fixture: &Process3dSnapshot, scene: &ProcessWorkingScene) -> (String, String) {
    let mesh = processed_mesh(scene, fixture.resolved_up_to).unwrap_or_else(|| mesh_from_kind(PROCESS3D_FALLBACK_MESH_KIND));
    let meshes = json::Value::Array(vec![json::object([("id".to_string(), json::Value::String("processed".to_string())), ("data".to_string(), json::Value::from(mesh))])]);
    let floats = |values: [f64; 3]| json::Value::Array(values.into_iter().map(json::Value::from).collect());
    let instances = json::Value::Array(vec![json::object([''',
'''fn evaluated_meshes_json(fixture: &Process3dSnapshot, scene: &ProcessWorkingScene) -> String {
    let mesh = processed_mesh(scene, fixture.resolved_up_to).unwrap_or_else(|| mesh_from_kind(PROCESS3D_FALLBACK_MESH_KIND));
    json::to_string(&json::Value::Array(vec![json::object([("id".to_string(), json::Value::String("processed".to_string())), ("data".to_string(), json::Value::from(mesh))])]))
}

/// 🕹️ The single stock instance. Selection belongs to the framework-owned `"geometry"` interaction
/// domain and changes far more often than the geometry does, so this is rebuilt every render and
/// deliberately NOT memoized: keying the CSG memo on selection would replay the entire process on
/// every click.
fn instances_json(fixture: &Process3dSnapshot, selected: bool) -> String {
    let floats = |values: [f64; 3]| json::Value::Array(values.into_iter().map(json::Value::from).collect());
    json::to_string(&json::Value::Array(vec![json::object([''', "split payload builder")

# 2. close the new fn: selected flag + return
sub(W,
'''        ("label".to_string(), json::Value::String(fixture.stock_label.clone())),
        ("selected".to_string(), json::Value::Bool(false)),
        ("hovered".to_string(), json::Value::Bool(false)),
    ])]);
    (json::to_string(&meshes), json::to_string(&instances))
}''',
'''        ("label".to_string(), json::Value::String(fixture.stock_label.clone())),
        ("selected".to_string(), json::Value::Bool(selected)),
        ("hovered".to_string(), json::Value::Bool(false)),
    ])]))
}''', "instances_json body")

# 3. cache stores the mesh only (label no longer participates: its only consumer is uncached)
sub(W, "    label: String,\n    payload: (String, String),\n    volume: f64,",
       "    meshes: String,\n    volume: f64,", "cache struct")
sub(W, "entry.scene == scene && entry.resolved_up_to == fixture.resolved_up_to && entry.label == fixture.stock_label",
       "entry.scene == scene && entry.resolved_up_to == fixture.resolved_up_to", "cache freshness key")
sub(W,
'''    let payload = evaluated_preview_payload(fixture, &scene);
    let volume = crate::artifacts::process3d::schema::inferences::processed_volume(&scene, fixture.resolved_up_to).unwrap_or(0.0);
    Process3dPreviewCache { scene, resolved_up_to: fixture.resolved_up_to, label: fixture.stock_label.clone(), payload, volume }''',
'''    let meshes = evaluated_meshes_json(fixture, &scene);
    let volume = crate::artifacts::process3d::schema::inferences::processed_volume(&scene, fixture.resolved_up_to).unwrap_or(0.0);
    Process3dPreviewCache { scene, resolved_up_to: fixture.resolved_up_to, meshes, volume }''', "build_preview_cache")
sub(W,
'''fn preview_payload_cached(fixture: &Process3dSnapshot) -> (String, String) {
    with_preview_cache(fixture, |entry| entry.payload.clone())
}''',
'''fn meshes_json_cached(fixture: &Process3dSnapshot) -> String {
    with_preview_cache(fixture, |entry| entry.meshes.clone())
}''', "cached accessor")

# 4. render takes the live selection
sub(W,
'''pub fn render(fixture: &Process3dSnapshot, config: &Process3dConfig) -> UiAssemblyResult<BuiltNode> {
    let (meshes_json, instances_json) = preview_payload_cached(fixture);
    MeshWindowKit::render(&MeshView {
        camera_json: world3d_camera_json(config.camera_position, config.camera_target, config.camera_fov),
        meshes_json,
        instances_json,''',
'''pub fn render(fixture: &Process3dSnapshot, config: &Process3dConfig, selected_ids: &[String]) -> UiAssemblyResult<BuiltNode> {
    let selected = selected_ids.iter().any(|id| id == &fixture.stock_id);
    MeshWindowKit::render(&MeshView {
        camera_json: world3d_camera_json(config.camera_position, config.camera_target, config.camera_fov),
        meshes_json: meshes_json_cached(fixture),
        instances_json: instances_json(fixture, selected),''', "render signature")

# 5. retire the stale "unreachable at this render boundary" claim
sub(W,
'''/// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): object/face selection AND hover are the
/// framework-owned `"geometry"` interaction domain now, unreachable at this `render` boundary
/// (`ArtifactEditor::render` carries no `InteractionView` — a known SDK gap, see `w3c-summary.md`) —''',
'''/// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): object/face selection AND hover are the
/// framework-owned `"geometry"` interaction domain. Object selection IS reachable here — the app
/// overrides `ArtifactEditor::render_with_request_context`, which carries an `InteractionView`, and
/// threads its ids into `render` — so the stock instance reflects it. Hover has no source at this
/// boundary and stays false; `selectionMode`/`targets`/`componentIds` are still not emitted here —''', "stale doc comment")

print("patching", E)
sub(E, "PROCESS_3D_PLAY_BODY_MAIN => workpiece::render(doc, config).map(semio_framework_plugin::built_to_component_tree),",
       "PROCESS_3D_PLAY_BODY_MAIN => workpiece::render(doc, config, selected_ids).map(semio_framework_plugin::built_to_component_tree),",
       "render body routing")
print("done")
