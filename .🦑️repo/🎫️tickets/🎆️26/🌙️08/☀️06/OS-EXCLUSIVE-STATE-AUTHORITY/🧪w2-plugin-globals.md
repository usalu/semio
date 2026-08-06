# Wave 2 — plugin global removals

Repo-relative paths. Ephemeral session state moved onto app instances unless noted.

## 1. Flow — `FLOW_PLAY_NEURAL_CACHE` + eval session

| Item | Action |
|------|--------|
| `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/⚙️engine/🦀️component.rs` | **Removed** `static FLOW_PLAY_NEURAL_CACHE: OnceLock<Arc<NeuralCache>>` and `flow_play_neural_cache()`. `flow_host_with_session` already uses `FlowEvalSession::neural_cache()` (per-instance). |
| `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🦀️component.rs` | **Unchanged** `FlowPlayApp::eval_session: Mutex<FlowEvalSession>` — instance-scoped driver, not a process registry; neural memoization lives on the session. |
| Tests | Replaced process-wide cache test with `flow_eval_session_neural_cache_is_per_instance_not_process_wide`. |

## 2. Space — `PRESENCE_PEERS`

| Item | Action |
|------|--------|
| `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🦀️component.rs` | **Removed** `static PRESENCE_PEERS: LazyLock<Mutex<…>>`. Peers map is `SpaceApp::presence_peers`. |
| Dispatch | `SpaceApp::handle` publishes presence after config ops that touch client/selection; command modules no longer call a global. |
| Render | `workflow::render` takes `&SpaceApp` for `presence_peers_json`. |
| Tests | `testkit` uses thread-local `SpaceApp` + `studio_presence_peers_json` so heartbeat tests share one instance. |

## 3. Home — `STUDIO_PORTS`

| Item | Action |
|------|--------|
| `✏️s/🔌️plugins/🪐️space/🎛️apps/🏠️home/🦀️component.rs` | **Removed** `static STUDIO_PORTS: LazyLock<Mutex<…>>`. Ports map is `HomeApp::studio_ports: Arc<Mutex<…>>` (shared handle via `shared_studio_ports()` so Home/Space free helpers still resolve folder-backed studios). |
| API | `register_studio_port` / `resolve_studio_document` wrappers keep stable signatures; `_for` variants take `&HomeApp`. |
| Glue | `HomeApp::default()` factory for document app registration. |

## 4. Puzzle 3D — `PUZZLE3D_MESH_REGISTRY`

| Item | Action |
|------|--------|
| `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d/🦀️component.rs` | **Removed** `pub static PUZZLE3D_MESH_REGISTRY`. Registry is `Puzzle3dPlayApp::mesh_registry`. |
| `🎮️commands/🖌️brush` | `register_brush_mesh` writes through `ctx.app.mesh_registry`. |
| OS mesh exporters | `puzzle3d_mesh_from_document` uses empty registry (box fallback); `puzzle3d_mesh_from_document_with_registry` for in-app use when needed. |

## 5. Layout — `ENGINE` global `Mutex<LayoutEngine>`

| Item | Action |
|------|--------|
| `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/⚙️engine/🎬️scene/🦀️component.rs` | **Removed** `static ENGINE: OnceLock<Mutex<LayoutEngine>>`. `build_display_list_for_page`, `layout_story_in_frame`, `build_scene_from_document_json`, `hit_test_document_json` take `&mut LayoutEngine`. |
| `✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/🦀️component.rs` | `LayoutPlayApp::layout_engine: Mutex<LayoutEngine>`; render path locks and passes into blueprint/preview/canvas. |
| WASM | `LayoutSessionInner::layout_engine` for GPU session. |
| Pointer hit-test | Local `LayoutEngine::new()` per hit (no global). |

## Verify

- Full `cargo check -p semio-s-plugin-{flow,space,puzzle,layout}` blocked by pre-existing workspace errors (`plugin_bundle_installer_shim.rs`, missing flow `list` module) — same as ticket gate baseline, not introduced by this wave.
- Extended/updated tests: flow engine, space presence selection, layout scene export tests (engine parameter).
