//! 🧊️ BIM world window: the model in 3D on a `World3d` surface. Every element solid of the `element-solids` inference becomes one inline mesh keyed by its element id and one
//! instance that names the framework `elements` interaction domain with the element's kind as granularity, so the host's own raycast pick, hover and marquee select
//! the same ids the outliner and the plan select. Storey isolation and visibility filter the instances, the section plane clips them.

#[path = "🎚️config/🦀️.rs"]
pub mod config;

#[path = "🌡️envelope/🦀️.rs"]
pub mod envelope;

use self::config::BimWorldWindowConfig;
use crate::editor::bim::entities::kind_holding;
use crate::editor::bim::gestures::session::{Preview, Shape};
use crate::editor::bim::interaction::BIM_ELEMENT_DOMAIN;
use crate::editor::bim::terminology::BimLabels;
use crate::render::world::{mesh_data, overview_orbit, world_bounds};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::ElementSolid;
use crate::standards::v1::subsets::any::schema::inferences::phase_visibility::ViewPhase;
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
/// 🎥️ The projection kinds of the 3D window, as the host names them: perspective, orthographic, axonometric, one-point and two-point.
pub const PROJECTION_KINDS: [&str; 5] = ["threePoint", "orthographic", "axonometric", "onePoint", "twoPoint"];
/// 🎥️ The camera of a window that has not framed its content yet.
pub const INITIAL_CAMERA: store::Viewport3dOrbit = store::Viewport3dOrbit { position: [14.0, -14.0, 10.0], target: [4.0, 3.0, 1.5], zoom: 1.0, up: None };
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::bim::create_bim_app`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: crate::editor::bim::utilities::initial(),
        id: WINDOW_KIND_ID.into(),
        label: BimLabels::localized(|labels| labels.window_world),
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
/// 🎭️ The phase filter of the window: its `view_phase` key (`all`, `existing`, `new`, `demolished`, `temporary`), every phase for an empty or unknown key.
pub fn view_phase(config: &BimWorldWindowConfig) -> ViewPhase {
    ViewPhase::parse(&config.view_phase).unwrap_or_default()
}

/// 🔭️ Whether the world box of the solid meets the section box of the window (six values: the corner with the smallest coordinates, then the largest; none means no box).
fn in_box(solid: &ElementSolid, config: &BimWorldWindowConfig) -> bool {
    let [low_x, low_y, low_z, high_x, high_y, high_z] = config.section_box[..] else { return true };
    world_bounds([solid]).is_none_or(|(min, max)| min[0] <= high_x && max[0] >= low_x && min[1] <= high_y && max[1] >= low_y && min[2] <= high_z && max[2] >= low_z)
}

/// 👁️ Whether the solid of element `id` is drawn: not on a hidden storey, on the isolated storey when there is one, and not hidden by the phase filter (the `phase-visibility` of its storey decides).
pub fn visible(id: &str, solid: &ElementSolid, inference: &ModelInference, config: &BimWorldWindowConfig) -> bool {
    (config.isolated_elements.is_empty() || config.isolated_elements.iter().any(|isolated| isolated == id))
        && inference.option_scope.includes(id, &config.selected_options, &config.workset_visibility)
        && in_box(solid, config)
        && (config.isolated_storey.is_empty() || solid.storey == config.isolated_storey)
        && !config.hidden_storeys.contains(&solid.storey)
        && !inference.phase_visibility.get(&solid.storey).is_some_and(|phases| phases.hides(view_phase(config), id))
}

/// 🧊️ The solids the window draws, in element id order.
pub fn visible_solids<'a>(inference: &'a ModelInference, config: &BimWorldWindowConfig) -> Vec<(&'a String, &'a ElementSolid)> {
    inference.element_solids.iter().filter(|(id, solid)| visible(id, solid, inference, config)).collect()
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
    let holder = kind_holding(snapshot, id);
    let name = holder.and_then(|row| (row.name)(snapshot, id)).unwrap_or_else(|| id.to_string());
    let granularity = holder.map(|row| ("interactionGranularityId".to_string(), DslValue::String(row.kind.to_string())));
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
    ].into_iter().chain(granularity))
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

/// 🫧️ How far above the storey floor the marks of a gesture float in the 3D window, in metres.
const PREVIEW_LIFT: f64 = 0.02;

fn segment_entry(from: [f64; 3], to: [f64; 3]) -> DslValue {
    DslValue::object([("kind".to_string(), DslValue::String("segment".into())), ("from".to_string(), array(&from)), ("to".to_string(), array(&to))])
}

fn point_entry(at: [f64; 3]) -> DslValue {
    DslValue::object([("kind".to_string(), DslValue::String("point".into())), ("position".to_string(), array(&at))])
}

/// 🫧️ The marks of the gesture in progress as the world's engagement preview: every path becomes the segments between its points (a closed one gets its closing segment), every dot and snap
/// marker a point cross, all at the floor of the storey the gesture draws on. Labels have no 3D form and stay in the plan.
pub fn preview_items(snapshot: &ModelSnapshot, inference: &ModelInference, config: &BimWorldWindowConfig, preview: &Preview) -> Vec<DslValue> {
    let storey = crate::editor::bim::gestures::world_storey(snapshot, &config.isolated_storey);
    let z = inference.storey_levels.get(&storey).map_or(0.0, |level| level.elevation) + PREVIEW_LIFT;
    let lift = |at: [f64; 2]| [at[0], at[1], z];
    let mut items = Vec::new();
    for mark in &preview.marks {
        let corners = mark.corners();
        match mark.shape {
            Shape::Path => {
                items.extend(corners.windows(2).map(|pair| segment_entry(lift(pair[0]), lift(pair[1]))));
                if let (true, Some(first), Some(last)) = (mark.closed && corners.len() > 2, corners.first(), corners.last()) {
                    items.push(segment_entry(lift(*last), lift(*first)));
                }
            }
            Shape::Dot | Shape::Snap => items.extend(corners.first().map(|at| point_entry(lift(*at)))),
            Shape::Label => {}
        }
    }
    items
}

/// 🎬️ The world scene of the model: camera, meshes, instances, selection and the interaction domain the host dispatches picks into.
pub fn scene(snapshot: &ModelSnapshot, inference: &ModelInference, config: &BimWorldWindowConfig, selection: &[String], hover: &[String], revision: u32) -> semio_framework_plugin::World3dScene {
    scene_over(snapshot, inference, config, selection, hover, revision, &Preview::default())
}

/// 🎬️ [`scene`] with the marks of the gesture in progress painted over the model as the scene's engagement preview.
pub fn scene_over(snapshot: &ModelSnapshot, inference: &ModelInference, config: &BimWorldWindowConfig, selection: &[String], hover: &[String], revision: u32, preview: &Preview) -> semio_framework_plugin::World3dScene {
    let solids = visible_solids(inference, config);
    let mut meshes: Vec<DslValue> = solids.iter().map(|(id, solid)| mesh_entry(id, snapshot, solid)).collect();
    let mut instances: Vec<DslValue> = solids.iter().map(|(id, solid)| instance_entry(id, snapshot, solid, selection.contains(id), hover.contains(id))).collect();
    if config.structural_overlay {
        let (structural_meshes,structural_instances)=crate::render::structure::overlay(inference,|id| inference.element_solids.get(id).is_some_and(|solid|visible(id,solid,inference,config)));
        meshes.extend(structural_meshes);
        instances.extend(structural_instances);
    }
    let overlay = envelope::overlay(snapshot, inference, config);
    if let Some(overlay) = &overlay {
        meshes.extend(overlay.meshes.iter().cloned());
        instances.extend(overlay.instances.iter().cloned());
    }
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
    scene.domain_granularity_id = solids.first().and_then(|(id, _)| kind_holding(snapshot, id)).map(|row| row.kind.to_string());
    scene.fit_json = Some(semio_framework_pack_json::to_json_string(&DslValue::object([("enabled".to_string(), DslValue::Bool(!config.framed)), ("revision".to_string(), DslValue::float(f64::from(revision))), ("padding".to_string(), DslValue::float(1.25))])));
    scene.modelling_options = section_options(config);
    if let Some(overlay) = overlay {
        scene.annotations = overlay.annotations;
        scene.scalar_field = overlay.scalar_field;
    }
    let marks = preview_items(snapshot, inference, config, preview);
    scene.engagement_preview_json = (!marks.is_empty()).then(|| semio_framework_pack_json::to_json_string(&marks));
    scene
}

/// 🧊️ Renders the world window.
pub fn render(snapshot: &ModelSnapshot, inference: &ModelInference, config: &BimWorldWindowConfig, selection: &[String], hover: &[String], revision: u32) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    render_over(snapshot, inference, config, selection, hover, revision, &Preview::default())
}

/// 🧊️ [`render`] with the marks of the gesture in progress painted over the model.
pub fn render_over(snapshot: &ModelSnapshot, inference: &ModelInference, config: &BimWorldWindowConfig, selection: &[String], hover: &[String], revision: u32, preview: &Preview) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    semio_framework_plugin::scene_surface(SURFACE_ID, semio_framework_ui_contract::SurfaceKind::World3d, &scene_over(snapshot, inference, config, selection, hover, revision, preview))
}
//#endregion 🔖️Scene

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
