//! 🖼️ The viewport window body: assembles a `World3dScene` from a lowered viewport and renders it as the `world-3d` surface.

use super::lowering::LoweredViewport;
use super::*;
use semio_plugin::{scene_surface, world3d_scene, world3d_selection_json_with_granularity, World3dScene, WorldSunConfig};

pub const VIEWPORT_ACTION_SET_CAMERA: &str = "viewportSetCamera";
pub const VIEWPORT_ACTION_FRAME: &str = "viewportFrame";
pub const VIEWPORT_ACTION_TOGGLE_LAYER: &str = "viewportToggleLayer";
pub const VIEWPORT_ACTION_SET_COLOUR_MAP: &str = "viewportSetColourMap";
pub const VIEWPORT_ACTION_SET_SECTION: &str = "viewportSetSection";
pub const VIEWPORT_ACTION_SET_DEFORMATION_SCALE: &str = "viewportSetDeformationScale";
pub const VIEWPORT_ACTION_SET_LOAD_CASE: &str = "viewportSetLoadCase";

/// 🧊️ Framework scene of a lowered viewport bound to the artifact's interaction domain; the host raycasts instances and dispatches `interactionSelect` with their scoped address.
pub fn world3d_scene_of(lowered: &LoweredViewport, domain_id: &str, sun: bool) -> World3dScene {
    let selection = world3d_selection_json_with_granularity("pick", &lowered.selection_ids, lowered.hovered.as_deref(), None);
    let sun = WorldSunConfig { enabled: sun, ..WorldSunConfig::default() };
    let mut scene = world3d_scene(lowered.camera.json(), semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Array(lowered.meshes.clone())), semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Array(lowered.instances.clone())), selection, &sun);
    scene.domain_id = Some(domain_id.to_string());
    scene.scalar_field = lowered.scalar_field.clone();
    scene.annotations = lowered.annotations.clone();
    scene.modelling_options = Some(lowered.options);
    scene.pick_targets_json = lowered.pick_targets_json.clone();
    scene
}

/// 🪟️ The `world-3d` surface node of a lowered viewport; `body_key` is the window body key of the artifact shim.
pub fn render_viewport3d(body_key: &str, lowered: &LoweredViewport, domain_id: &str, sun: bool) -> UiAssemblyResult<BuiltNode> {
    scene_surface(body_key, semio_ui_contract::SurfaceKind::World3d, &world3d_scene_of(lowered, domain_id, sun))
}
