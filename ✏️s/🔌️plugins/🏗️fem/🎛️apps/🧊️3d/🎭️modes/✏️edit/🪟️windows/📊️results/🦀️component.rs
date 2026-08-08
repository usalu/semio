//! 📊️ FEM 3D app — the `edit` mode's Results window: static/modal/buckling analysis views over the
//! same node/member/solid scene the Model window renders. fem3d's manifest declares this window with
//! the scalar `.window_kind(..)` builder call directly (see `crate::apps::fem3d::create_fem3d_app`) — no
//! `WindowKindDefinition`/`window_kind_def` object is built anywhere in the pre-migration
//! `create_fem3d_app`, so this node exports just its id/body-key constants and `render()`.
//!
//! `config_result_display` (was the old ui crate's `🔖️Fem3dConfigHelpers` region) and the
//! captioning/model-extent helpers below are results-rendering-only — their sole consumers are this
//! file's own static/modal/buckling render functions — so they live here rather than in the artifact's
//! `⚙️engine` (which only hosts helpers with 2+ consumer FILES, per the migration recipe's
//! `DocumentHelpers` placement rule; see `crate::artifacts::fem3d::engine`'s `🔖️SceneRender` region doc).

use crate::apps::fem3d::config::Fem3dConfig;
use crate::artifacts::fem3d::{Fem3dSnapshot, FemCamera};
use crate::app_surface::{DisplayMode, ResultDisplay};
use semio_framework_plugin::{ui_stack_vertical, ui_text, Label, UiNode};

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
fn with_caption(scene: UiNode, caption: String) -> UiNode {
    ui_stack_vertical(vec![ui_text(Label::data(caption)), scene])
}

/// 📊️ Results window dispatcher — picks the static/modal/buckling render based on `display`.
pub fn render(doc: &Fem3dSnapshot, cfg: &Fem3dConfig) -> UiNode {
    let display = config_result_display(cfg);
    let camera = &cfg.camera;
    match display.mode {
        DisplayMode::Static => render_static(doc, display.source_id.as_deref(), camera),
        DisplayMode::Modal(mode_index) => render_modal(doc, mode_index, camera),
        DisplayMode::Buckling(mode_index) => render_buckling(doc, display.source_id.as_deref(), mode_index, camera),
    }
}

/// 📊️ Static results: solved fresh on every render (no cache, mirrors `Fem3dPlayApp`'s v0 design) —
/// same node/member/solid instances as the model window, offset by the solved displacements, solids
/// additionally colored by nodal-averaged von Mises stress. `source_id` selects a `fem3d_solve_all`
/// case/combination id, falling back to the first load case when `None`/unknown. Caption names the
/// active case.
fn render_static(doc: &Fem3dSnapshot, source_id: Option<&str>, camera: &FemCamera) -> UiNode {
    use crate::artifacts::fem3d::engine::{fem3d_camera_json, fem3d_scene_parts, fem3d_solve_all};

    let results = match fem3d_solve_all(doc) {
        Ok(results) => results,
        Err(e) => return ui_text(Label::data(format!("Analysis error: {e}"))),
    };
    let case_id = source_id.filter(|id| results.contains_key(*id)).map(str::to_string).or_else(|| doc.load_cases.first().map(|c| c.id.clone()));
    let Some(case_id) = case_id else {
        return ui_text(Label::data("No load case defined"));
    };
    let Some(result) = results.get(&case_id) else {
        return ui_text(Label::data(format!("Result not found: {case_id}")));
    };
    let mut disp_map: std::collections::HashMap<String, [f64; 6]> = std::collections::HashMap::new();
    for d in &result.displacements {
        disp_map.insert(d.node_id.clone(), d.values);
    }
    let nodal_stress = crate::artifacts::fem3d::engine::mesh_preview::fem3d_nodal_von_mises(doc, &case_id).ok();
    let (meshes_json, instances_json) = fem3d_scene_parts(doc, Some(&disp_map), doc.analysis.deformation_scale, nodal_stress.as_ref());
    let scene = semio_framework_plugin::build_world_3d_scene(
        FEM3D_BODY_RESULTS,
        crate::apps::fem3d::FEM3D_APP_ID,
        semio_framework_plugin::world3d_scene(fem3d_camera_json(camera), meshes_json, instances_json, semio_framework_plugin::world3d_default_selection_json(), &semio_framework_plugin::WorldSunConfig::default()),
    );
    with_caption(scene, format!("Case: {case_id}"))
}

/// 📊️ Modal mode-shape overlay: instances offset by the selected mode's shape, normalized to unit peak
/// then scaled to `MODE_SHAPE_AMPLITUDE_RATIO` of the model's own extent, with a frequency caption.
fn render_modal(doc: &Fem3dSnapshot, mode_index: usize, camera: &FemCamera) -> UiNode {
    use crate::artifacts::fem3d::engine::{fem3d_camera_json, fem3d_scene_parts, modal_buckling::fem3d_modal_mode_values};
    use crate::app_surface::{normalize_mode_shape, MODE_SHAPE_AMPLITUDE_RATIO};

    let (freq_hz, mut disp_map) = match fem3d_modal_mode_values(doc, mode_index) {
        Ok(values) => values,
        Err(e) => return ui_text(Label::data(format!("Modal analysis error: {e}"))),
    };
    normalize_mode_shape(&mut disp_map);
    let (meshes_json, instances_json) = fem3d_scene_parts(doc, Some(&disp_map), fem3d_model_extent(doc) * MODE_SHAPE_AMPLITUDE_RATIO, None);
    let scene = semio_framework_plugin::build_world_3d_scene(
        FEM3D_BODY_RESULTS,
        crate::apps::fem3d::FEM3D_APP_ID,
        semio_framework_plugin::world3d_scene(fem3d_camera_json(camera), meshes_json, instances_json, semio_framework_plugin::world3d_default_selection_json(), &semio_framework_plugin::WorldSunConfig::default()),
    );
    with_caption(scene, format!("Mode {}: {freq_hz:.3} Hz", mode_index + 1))
}

/// 📊️ Buckling mode-shape overlay: instances offset by the selected mode's shape, normalized to unit
/// peak then scaled to `MODE_SHAPE_AMPLITUDE_RATIO` of the model's own extent. `source_id` selects the
/// reference load case, falling back to the first load case when `None`. Caption names the mode and its
/// load factor.
fn render_buckling(doc: &Fem3dSnapshot, source_id: Option<&str>, mode_index: usize, camera: &FemCamera) -> UiNode {
    use crate::artifacts::fem3d::engine::{fem3d_camera_json, fem3d_scene_parts, modal_buckling::fem3d_buckling_mode_values};
    use crate::app_surface::{normalize_mode_shape, MODE_SHAPE_AMPLITUDE_RATIO};

    let Some(case_id) = source_id.map(str::to_string).or_else(|| doc.load_cases.first().map(|c| c.id.clone())) else {
        return ui_text(Label::data("No load case defined"));
    };
    let (factor, mut disp_map) = match fem3d_buckling_mode_values(doc, &case_id, mode_index) {
        Ok(values) => values,
        Err(e) => return ui_text(Label::data(format!("Buckling analysis error: {e}"))),
    };
    normalize_mode_shape(&mut disp_map);
    let (meshes_json, instances_json) = fem3d_scene_parts(doc, Some(&disp_map), fem3d_model_extent(doc) * MODE_SHAPE_AMPLITUDE_RATIO, None);
    let scene = semio_framework_plugin::build_world_3d_scene(
        FEM3D_BODY_RESULTS,
        crate::apps::fem3d::FEM3D_APP_ID,
        semio_framework_plugin::world3d_scene(fem3d_camera_json(camera), meshes_json, instances_json, semio_framework_plugin::world3d_default_selection_json(), &semio_framework_plugin::WorldSunConfig::default()),
    );
    with_caption(scene, format!("Buckling mode {}: factor {factor:.3}", mode_index + 1))
}
// #endregion 🔖️Render

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::fem3d::testkit::{dispatch, fem3d_app, render as render_body, Fem3dApp};
    use crate::apps::fem3d::Fem3dCommand;

    fn app_with_example() -> Fem3dApp {
        let mut app = fem3d_app();
        dispatch(&mut app, Fem3dCommand::SetActiveExample(crate::apps::fem3d::commands::example::set_active_example::SetActiveExample { example_id: "default".into() }));
        app
    }

    #[test]
    fn renders_fem3d_results_scene() {
        let mut app = app_with_example();
        let json = render_body(&mut app, FEM3D_BODY_RESULTS);
        assert!(json.contains("world-3d"));
    }

    #[test]
    fn results_window_surfaces_solver_error_without_panicking_3d() {
        let mut app = fem3d_app();
        let _ = render_body(&mut app, FEM3D_BODY_RESULTS);
    }

    #[test]
    fn results_window_renders_modal_mode_shape_3d() {
        let mut app = app_with_example();
        dispatch(&mut app, Fem3dCommand::SetResultDisplay(crate::apps::fem3d::commands::results::set_result_display::SetResultDisplay { source_id: None, mode: "modal".into(), mode_index: 0 }));
        let json = render_body(&mut app, FEM3D_BODY_RESULTS);
        assert!(json.contains("world-3d"), "expected a valid world-3d scene, got: {json}");
        assert!(!json.contains("Modal analysis error"), "unexpected modal error: {json}");
    }

    #[test]
    fn results_window_renders_buckling_mode_shape_3d() {
        let mut app = app_with_example();
        dispatch(&mut app, Fem3dCommand::SetResultDisplay(crate::apps::fem3d::commands::results::set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "buckling".into(), mode_index: 0 }));
        let json = render_body(&mut app, FEM3D_BODY_RESULTS);
        assert!(json.contains("world-3d"), "expected a valid world-3d scene, got: {json}");
        assert!(!json.contains("Buckling analysis error"), "unexpected buckling error: {json}");
    }

    #[test]
    fn results_scene_includes_solid_vertex_colors_3d() {
        let mut app = app_with_example();
        dispatch(&mut app, Fem3dCommand::SetResultDisplay(crate::apps::fem3d::commands::results::set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "static".into(), mode_index: 0 }));
        let json = render_body(&mut app, FEM3D_BODY_RESULTS);
        assert!(json.contains("solid-sol1"), "expected the solid mesh in the results scene: {json}");
        assert!(json.contains("\\\"colors\\\""), "expected a vertex colors array on the solid mesh data: {json}");
        assert!(json.contains("Case: dead"), "expected a case-id caption: {json}");
    }

    #[test]
    fn results_scene_captions_name_mode_and_factor_3d() {
        let mut app = app_with_example();
        dispatch(&mut app, Fem3dCommand::SetResultDisplay(crate::apps::fem3d::commands::results::set_result_display::SetResultDisplay { source_id: None, mode: "modal".into(), mode_index: 0 }));
        let json_modal = render_body(&mut app, FEM3D_BODY_RESULTS);
        assert!(json_modal.contains("Hz"), "expected a frequency caption: {json_modal}");

        dispatch(&mut app, Fem3dCommand::SetResultDisplay(crate::apps::fem3d::commands::results::set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "buckling".into(), mode_index: 0 }));
        let json_buckling = render_body(&mut app, FEM3D_BODY_RESULTS);
        assert!(json_buckling.contains("factor"), "expected a load-factor caption: {json_buckling}");
    }

    #[test]
    fn fem3d_model_extent_degenerate_model_returns_one() {
        assert_eq!(fem3d_model_extent(&Fem3dSnapshot::default()), 1.0);
    }
}
// #endregion 🧪️Tests
