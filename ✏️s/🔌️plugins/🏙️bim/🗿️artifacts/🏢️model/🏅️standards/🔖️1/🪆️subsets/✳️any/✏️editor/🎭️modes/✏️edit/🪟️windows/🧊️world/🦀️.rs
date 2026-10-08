//! 🧊️ BIM world window: the model in 3D on a `World3d` surface. Every element solid of the `element-solids` inference becomes one inline mesh keyed by its element id and one
//! instance that names the framework `elements` interaction domain with the element's kind as granularity, so the host's own raycast pick, hover and marquee select
//! the same ids the outliner and the plan select. Storey isolation and visibility filter the instances, the section plane clips them.

#[path = "🎚️config/🦀️.rs"]
pub mod config;

use self::config::BimWorldWindowConfig;
use crate::editor::bim::entities::kind_holding;
use crate::editor::bim::interaction::BIM_ELEMENT_DOMAIN;
use crate::editor::bim::terminology::BimLabels;
use crate::render::world::{mesh_data, overview_orbit, world_bounds};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::ElementSolid;
use crate::{ModelInference, ModelSnapshot};
use semio_framework_plugin::DslValue;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::SurfaceKind;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_plugin::WindowOptions;
use semio_framework_ui_locale::LocalizedLabel;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "bim-edit-world";
pub const BODY_KEY: &str = "bim.edit.world";
const SURFACE_ID: &str = "bim.edit.world3d/world";
/// 🎥️ The camera of a window that has not framed its content yet.
pub const INITIAL_CAMERA: store::Viewport3dOrbit = store::Viewport3dOrbit { position: [14.0, -14.0, 10.0], target: [4.0, 3.0, 1.5], zoom: 1.0, up: None };
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::bim::create_bim_app`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: crate::editor::bim::utilities::initial(),
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native(BimLabels::NATIVE_EN.window_world.as_str(), BimLabels::NATIVE_DE.window_world.as_str()),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "box".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: crate::editor::bim::utilities::for_window(WINDOW_KIND_ID),
        interactions: vec![InteractionRef::new(BIM_ELEMENT_DOMAIN)],
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Visibility
/// 👁️ Whether a solid is drawn: not on a hidden storey and, when one storey is isolated, on that storey.
pub fn visible(solid: &ElementSolid, config: &BimWorldWindowConfig) -> bool {
    (config.isolated_storey.is_empty() || solid.storey == config.isolated_storey) && !config.hidden_storeys.contains(&solid.storey)
}

/// 🧊️ The solids the window draws, in element id order.
pub fn visible_solids<'a>(inference: &'a ModelInference, config: &BimWorldWindowConfig) -> Vec<(&'a String, &'a ElementSolid)> {
    inference.element_solids.iter().filter(|(_, solid)| visible(solid, config)).collect()
}
//#endregion 🔖️Visibility

//#region 🔖️Scene
fn array(values: &[f64]) -> DslValue {
    DslValue::Array(values.iter().map(|value| DslValue::float(*value)).collect())
}

/// 🧱️ The inline mesh entry of one solid: the shared render module's mesh data under the element id.
fn mesh_entry(id: &str, snapshot: &ModelSnapshot, solid: &ElementSolid) -> DslValue {
    DslValue::object([("id".to_string(), DslValue::String(id.to_string())), ("data".to_string(), semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::Value::from(mesh_data(snapshot, solid))))])
}

/// 🧭️ The instance of one solid: its storey placement as translation plus a rotation about +Z, the element id both as instance and as interaction id.
fn instance_entry(id: &str, snapshot: &ModelSnapshot, solid: &ElementSolid, selected: bool, hovered: bool) -> DslValue {
    let placement = solid.placement;
    let (sin, cos) = (placement.rotation * 0.5).sin_cos();
    let kind = kind_holding(snapshot, id).map_or("wall", |row| row.kind);
    let name = kind_holding(snapshot, id).and_then(|row| (row.name)(snapshot, id)).unwrap_or_else(|| id.to_string());
    DslValue::object([
        ("id".to_string(), DslValue::String(id.to_string())),
        ("meshId".to_string(), DslValue::String(id.to_string())),
        ("position".to_string(), array(&[placement.x, placement.y, placement.z])),
        ("rotation".to_string(), array(&[0.0, 0.0, sin, cos])),
        ("scale".to_string(), array(&[1.0, 1.0, 1.0])),
        ("label".to_string(), DslValue::String(name)),
        ("selected".to_string(), DslValue::Bool(selected)),
        ("hovered".to_string(), DslValue::Bool(hovered)),
        ("interactionId".to_string(), DslValue::String(id.to_string())),
        ("interactionGranularityId".to_string(), DslValue::String(kind.to_string())),
    ])
}

/// 🎥️ The camera pose for a model: the shared overview of the visible bounds, so a window that has not been navigated frames its content.
pub fn framing_camera(solids: &[(&String, &ElementSolid)]) -> store::Viewport3dOrbit {
    if solids.is_empty() {
        return INITIAL_CAMERA;
    }
    overview_orbit(world_bounds(solids.iter().map(|(_, solid)| *solid)))
}

/// ✂️ The section plane of the window: it removes everything beyond `offset` on `axis`, capped so the cut reads solid.
pub fn section_options(config: &BimWorldWindowConfig) -> Option<semio_framework_plugin::World3dModellingOptions> {
    if !config.section_enabled {
        return None;
    }
    let (origin, normal) = match config.section_axis.as_str() {
        "x" => ([config.section_offset, 0.0, 0.0], [1.0, 0.0, 0.0]),
        "y" => ([0.0, config.section_offset, 0.0], [0.0, 1.0, 0.0]),
        _ => ([0.0, 0.0, config.section_offset], [0.0, 0.0, 1.0]),
    };
    Some(semio_framework_plugin::World3dModellingOptions { section: Some(semio_framework_plugin::World3dSection::new(origin, normal).capped(semio_framework_plugin::World3dTone::Neutral)), ..Default::default() })
}

/// 🎬️ The world scene of the model: camera, meshes, instances, selection and the interaction domain the host dispatches picks into.
pub fn scene(snapshot: &ModelSnapshot, inference: &ModelInference, config: &BimWorldWindowConfig, selection: &[String], hover: &[String], revision: u32) -> semio_framework_plugin::World3dScene {
    let solids = visible_solids(inference, config);
    let meshes: Vec<DslValue> = solids.iter().map(|(id, solid)| mesh_entry(id, snapshot, solid)).collect();
    let instances: Vec<DslValue> = solids.iter().map(|(id, solid)| instance_entry(id, snapshot, solid, selection.contains(id), hover.contains(id))).collect();
    let camera = if config.framed { config.camera } else { framing_camera(&solids) };
    let camera_json = semio_framework_plugin::world3d_camera_projection_json(camera.position, camera.target, camera.up, camera.zoom, &config.projection);
    let selected_ids: Vec<String> = selection.to_vec();
    let mut scene = semio_framework_plugin::world3d_scene(
        camera_json,
        semio_framework_pack_json::to_json_string(&meshes),
        semio_framework_pack_json::to_json_string(&instances),
        semio_framework_plugin::world3d_selection_json("rectangle", &selected_ids, hover.first().map(String::as_str)),
        &semio_framework_plugin::WorldSunConfig::default(),
    );
    scene.domain_id = Some(BIM_ELEMENT_DOMAIN.into());
    scene.domain_granularity_id = Some("wall".into());
    scene.fit_json = Some(semio_framework_pack_json::to_json_string(&DslValue::object([("enabled".to_string(), DslValue::Bool(!config.framed)), ("revision".to_string(), DslValue::float(f64::from(revision))), ("padding".to_string(), DslValue::float(1.25))])));
    scene.modelling_options = section_options(config);
    scene
}

/// 🧊️ Renders the world window.
pub fn render(snapshot: &ModelSnapshot, inference: &ModelInference, config: &BimWorldWindowConfig, selection: &[String], hover: &[String], revision: u32) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    semio_framework_plugin::scene_surface(SURFACE_ID, semio_framework_ui_contract::SurfaceKind::World3d, &scene(snapshot, inference, config, selection, hover, revision))
}
//#endregion 🔖️Scene

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
