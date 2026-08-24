//! 📊️ FEM 3D app — the `edit` mode's Results window: static/modal/buckling analysis views over the
//! same node/member/solid scene the Model window renders. fem3d's manifest declares this window with
//! the scalar `.window_kind(..)` builder call directly (see `crate::editor::fem3d::create_fem3d_app`) — no
//! `WindowKindDefinition`/`window_kind_def` object is built anywhere in the pre-migration
//! `create_fem3d_app`, so this node exports just its id/body-key constants and `render()`.
//!
//! `config_result_display` (was the old ui crate's `🔖️Fem3dConfigHelpers` region) and the
//! captioning/model-extent helpers below are results-rendering-only — their sole consumers are this
//! file's own static/modal/buckling render functions — so they live here rather than in
//! `crate::editor::fem3d`'s shared `🎬️SceneRender` region (which only hosts helpers with 2+ consumer
//! FILES, per the migration recipe's `DocumentHelpers` placement rule).

use crate::app_surface::{DisplayMode, ResultDisplay};
use crate::artifacts::fem3d::{Fem3dSnapshot, FemCamera};
use crate::editor::fem3d::config::Fem3dConfig;
use semio_framework_plugin::{built_text_node, BuiltNode, Label};
use semio_framework_ui_contract::{Buildable, HasChildren};

/// 🪟️ The manifest's Results window kind id.
pub const FEM3D_WINDOW_RESULTS: &str = "fem3d-results";
/// 📄️ The Results window's sole render body key.
pub const FEM3D_BODY_RESULTS: &str = "fem3d.play.results";

// #region 🔖️ConfigHelpers
/// 👁️ B1: `cfg`-driven counterpart of the deleted `ResultDisplay` `RefCell` — converts the flat
/// `Fem3dConfig` result-display fields back into `crate::app_surface::ResultDisplay`/`DisplayMode` so
/// the render pipeline below (built around those shared types) needs no changes.
pub fn config_result_display(cfg: &Fem3dConfig) -> ResultDisplay {
    let mode = match cfg.result_mode.as_str() {
        "modal" => DisplayMode::Modal(cfg.result_mode_index as usize),
        "buckling" => DisplayMode::Buckling(cfg.result_mode_index as usize),
        _ => DisplayMode::Static,
    };
    ResultDisplay { source_id: cfg.result_source_id.clone(), mode }
}
// #endregion 🔖️ConfigHelpers

// #region 🔖️Render
/// 📐️ Bounding-box diagonal (in model meters) over every node plus every solid's footprint/height —
/// drives mode-shape amplitude (see `crate::app_surface::MODE_SHAPE_AMPLITUDE_RATIO`'s doc). Falls
/// back to `1.0` for a degenerate model.
#[cfg(test)]
fn fem3d_model_extent(doc: &Fem3dSnapshot) -> f64 {
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    let mut expand = |x: f64, y: f64, z: f64| {
        min[0] = min[0].min(x);
        min[1] = min[1].min(y);
        min[2] = min[2].min(z);
        max[0] = max[0].max(x);
        max[1] = max[1].max(y);
        max[2] = max[2].max(z);
    };
    for node in &doc.nodes {
        expand(node.x, node.y, node.z);
    }
    for solid in &doc.solids {
        for p in &solid.outline {
            expand(p[0], p[1], solid.base_z);
            expand(p[0], p[1], solid.base_z + solid.height);
        }
    }
    if min[0] > max[0] {
        return 1.0;
    }
    let d = [max[0] - min[0], max[1] - min[1], max[2] - min[2]];
    (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt().max(1.0)
}

/// 🏷️ Wraps a `World3d` scene node with a text caption above it — `World3dScene` itself has no text
/// field, so a vertical `UiNode` stack (already how the shell composes surfaces) is the idiomatic way to
/// show a frequency/load-factor/case caption in-scene. `caption` is genuine runtime data (a case id,
/// mode index, frequency, …), so it is wrapped via `Label::data` rather than any `LocalizedLabel`.
#[cfg(test)]
fn with_caption(scene: BuiltNode, caption: String) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    semio_framework_ui_contract::column().children([built_text_node(Label::data(caption)), scene]).build()
}

/// 📊️ Results window dispatcher — picks the static/modal/buckling render based on `display`.
#[cfg(test)]
pub fn render(doc: &Fem3dSnapshot, cfg: &Fem3dConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let display = config_result_display(cfg);
    let camera = &cfg.camera;
    match display.mode {
        DisplayMode::Static => render_static(doc, display.source_id.as_deref(), camera),
        DisplayMode::Modal(mode_index) => render_modal(doc, mode_index, camera),
        DisplayMode::Buckling(mode_index) => render_buckling(doc, display.source_id.as_deref(), mode_index, camera),
    }
}

/// 👁️ Adopts the immutable mounted result packet without solving, meshing, sorting, or encoding during render.
pub fn render_with_progress(camera: &FemCamera, visual: Option<&crate::artifacts::fem3d::live_visual::Fem3dPageVisualLease>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut scene =
        semio_framework_plugin::world3d_scene(crate::editor::fem3d::fem3d_camera_json(camera), "[]".into(), "[]".into(), semio_framework_plugin::world3d_selection_json("rectangle", &[], None), &semio_framework_plugin::WorldSunConfig::default());
    scene.snapshot = visual.map(crate::artifacts::fem3d::live_visual::Fem3dPageVisualLease::snapshot);
    crate::app_surface::world_3d_surface(FEM3D_BODY_RESULTS, scene)
}

/// 📊️ Static results: solved fresh on every render (no cache, mirrors `Fem3dPlayApp`'s v0 design) —
/// same node/member/solid instances as the model window, offset by the solved displacements, solids
/// additionally colored by nodal-averaged von Mises stress. `source_id` selects a `fem3d_solve_all`
/// case/combination id, falling back to the first load case when `None`/unknown. Caption names the
/// active case.
#[cfg(test)]
fn render_static(doc: &Fem3dSnapshot, source_id: Option<&str>, camera: &FemCamera) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    use crate::editor::fem3d::{fem3d_camera_json, fem3d_scene_parts};
    use crate::fem3d_engine::fem3d_solve_all;

    let results = match fem3d_solve_all(doc) {
        Ok(results) => results,
        Err(e) => return built_text_node(Label::data(format!("Analysis error: {e}"))),
    };
    let case_id = source_id.filter(|id| results.contains_key(*id)).map(str::to_string).or_else(|| doc.load_cases.first().map(|c| c.id.clone()));
    let Some(case_id) = case_id else {
        return built_text_node(Label::data("No load case defined"));
    };
    let Some(result) = results.get(&case_id) else {
        return built_text_node(Label::data(format!("Result not found: {case_id}")));
    };
    let mut disp_map: std::collections::HashMap<String, [f64; 6]> = std::collections::HashMap::new();
    for d in &result.displacements {
        disp_map.insert(d.node_id.clone(), d.values);
    }
    let nodal_stress = crate::fem3d_engine::mesh_preview::fem3d_nodal_von_mises(doc, &case_id).ok();
    let (meshes_json, instances_json) = fem3d_scene_parts(doc, Some(&disp_map), doc.analysis.deformation_scale, nodal_stress.as_ref());
    let scene = crate::app_surface::world_3d_surface(
        FEM3D_BODY_RESULTS,
        semio_framework_plugin::world3d_scene(fem3d_camera_json(camera), meshes_json, instances_json, semio_framework_plugin::world3d_selection_json("rectangle", &[], None), &semio_framework_plugin::WorldSunConfig::default()),
    );
    with_caption(scene, format!("Case: {case_id}"))
}

/// 📊️ Modal mode-shape overlay: instances offset by the selected mode's shape, normalized to unit peak
/// then scaled to `MODE_SHAPE_AMPLITUDE_RATIO` of the model's own extent, with a frequency caption.
#[cfg(test)]
fn render_modal(doc: &Fem3dSnapshot, mode_index: usize, camera: &FemCamera) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    use crate::app_surface::{normalize_mode_shape, MODE_SHAPE_AMPLITUDE_RATIO};
    use crate::editor::fem3d::{fem3d_camera_json, fem3d_scene_parts};
    use crate::fem3d_engine::modal_buckling::fem3d_modal_mode_values;

    let (freq_hz, mut disp_map) = match fem3d_modal_mode_values(doc, mode_index) {
        Ok(values) => values,
        Err(e) => return built_text_node(Label::data(format!("Modal analysis error: {e}"))),
    };
    normalize_mode_shape(&mut disp_map);
    let (meshes_json, instances_json) = fem3d_scene_parts(doc, Some(&disp_map), fem3d_model_extent(doc) * MODE_SHAPE_AMPLITUDE_RATIO, None);
    let scene = crate::app_surface::world_3d_surface(
        FEM3D_BODY_RESULTS,
        semio_framework_plugin::world3d_scene(fem3d_camera_json(camera), meshes_json, instances_json, semio_framework_plugin::world3d_selection_json("rectangle", &[], None), &semio_framework_plugin::WorldSunConfig::default()),
    );
    with_caption(scene, format!("Mode {}: {freq_hz:.3} Hz", mode_index + 1))
}

/// 📊️ Buckling mode-shape overlay: instances offset by the selected mode's shape, normalized to unit
/// peak then scaled to `MODE_SHAPE_AMPLITUDE_RATIO` of the model's own extent. `source_id` selects the
/// reference load case, falling back to the first load case when `None`. Caption names the mode and its
/// load factor.
#[cfg(test)]
fn render_buckling(doc: &Fem3dSnapshot, source_id: Option<&str>, mode_index: usize, camera: &FemCamera) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    use crate::app_surface::{normalize_mode_shape, MODE_SHAPE_AMPLITUDE_RATIO};
    use crate::editor::fem3d::{fem3d_camera_json, fem3d_scene_parts};
    use crate::fem3d_engine::modal_buckling::fem3d_buckling_mode_values;

    let Some(case_id) = source_id.map(str::to_string).or_else(|| doc.load_cases.first().map(|c| c.id.clone())) else {
        return built_text_node(Label::data("No load case defined"));
    };
    let (factor, mut disp_map) = match fem3d_buckling_mode_values(doc, &case_id, mode_index) {
        Ok(values) => values,
        Err(e) => return built_text_node(Label::data(format!("Buckling analysis error: {e}"))),
    };
    normalize_mode_shape(&mut disp_map);
    let (meshes_json, instances_json) = fem3d_scene_parts(doc, Some(&disp_map), fem3d_model_extent(doc) * MODE_SHAPE_AMPLITUDE_RATIO, None);
    let scene = crate::app_surface::world_3d_surface(
        FEM3D_BODY_RESULTS,
        semio_framework_plugin::world3d_scene(fem3d_camera_json(camera), meshes_json, instances_json, semio_framework_plugin::world3d_selection_json("rectangle", &[], None), &semio_framework_plugin::WorldSunConfig::default()),
    );
    with_caption(scene, format!("Buckling mode {}: factor {factor:.3}", mode_index + 1))
}
// #endregion 🔖️Render

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::fem3d::testkit::{dispatch, fem3d_app, render as render_body, Fem3dApp};
    use crate::editor::fem3d::Fem3dCommand;

    async fn app_with_example() -> Fem3dApp {
        let mut app = fem3d_app();
        dispatch(&mut app, Fem3dCommand::SetActiveExample(crate::editor::fem3d::commands::set_active_example::SetActiveExample { example_id: "default".into() })).await;
        app
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_fem3d_results_scene() {
        let mut app = app_with_example().await;
        let json = render_body(&mut app, FEM3D_BODY_RESULTS);
        assert!(json.contains("world-3d"));
    }

    #[semio_framework_async_macros::async_test]
    async fn results_window_surfaces_solver_error_without_panicking_3d() {
        let mut app = fem3d_app();
        let _ = render_body(&mut app, FEM3D_BODY_RESULTS);
    }

    #[semio_framework_async_macros::async_test]
    async fn results_window_renders_modal_mode_shape_3d() {
        let mut app = app_with_example().await;
        dispatch(&mut app, Fem3dCommand::SetResultDisplay(crate::editor::fem3d::commands::set_result_display::SetResultDisplay { source_id: None, mode: "modal".into(), mode_index: 0 })).await;
        let json = render_body(&mut app, FEM3D_BODY_RESULTS);
        assert!(json.contains("world-3d"), "expected a valid world-3d scene, got: {json}");
        assert!(!json.contains("Modal analysis error"), "unexpected modal error: {json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn results_window_renders_buckling_mode_shape_3d() {
        let mut app = app_with_example().await;
        dispatch(&mut app, Fem3dCommand::SetResultDisplay(crate::editor::fem3d::commands::set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "buckling".into(), mode_index: 0 })).await;
        let json = render_body(&mut app, FEM3D_BODY_RESULTS);
        assert!(json.contains("world-3d"), "expected a valid world-3d scene, got: {json}");
        assert!(!json.contains("Buckling analysis error"), "unexpected buckling error: {json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn results_scene_includes_solid_vertex_colors_3d() {
        let mut app = app_with_example().await;
        let snapshot = semio_framework_plugin::resolve_ready(app.snapshot()).expect("snapshot");
        let config = Fem3dConfig { result_source_id: Some("dead".into()), result_mode: "static".into(), ..Fem3dConfig::default() };
        let node = render(&snapshot, &config);
        let props = node
            .children
            .iter()
            .find_map(|child| match &child.component {
                semio_framework_ui_contract::Component::Surface(props) => Some(props),
                _ => None,
            })
            .expect("world surface child");
        let scene: semio_framework_ui_scene::World3dScene = semio_framework_ui_scene::decode(props).expect("decode world scene");
        let json = serde_json::to_string(&node).expect("render json");
        assert!(scene.meshes_json.contains("solid-sol1"), "expected the solid mesh in the results scene: {}", scene.meshes_json);
        assert!(scene.meshes_json.contains("\"colors\""), "expected a vertex colors array on the solid mesh data: {}", scene.meshes_json);
        assert!(json.contains("Case: dead"), "expected a case-id caption: {json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn results_scene_captions_name_mode_and_factor_3d() {
        let mut app = app_with_example().await;
        dispatch(&mut app, Fem3dCommand::SetResultDisplay(crate::editor::fem3d::commands::set_result_display::SetResultDisplay { source_id: None, mode: "modal".into(), mode_index: 0 })).await;
        let json_modal = render_body(&mut app, FEM3D_BODY_RESULTS);
        assert!(json_modal.contains("Hz"), "expected a frequency caption: {json_modal}");

        dispatch(&mut app, Fem3dCommand::SetResultDisplay(crate::editor::fem3d::commands::set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "buckling".into(), mode_index: 0 })).await;
        let json_buckling = render_body(&mut app, FEM3D_BODY_RESULTS);
        assert!(json_buckling.contains("factor"), "expected a load-factor caption: {json_buckling}");
    }

    #[semio_framework_async_macros::async_test]
    async fn fem3d_model_extent_degenerate_model_returns_one() {
        assert_eq!(fem3d_model_extent(&Fem3dSnapshot::default()), 1.0);
    }
}
// #endregion 🧪️Tests
