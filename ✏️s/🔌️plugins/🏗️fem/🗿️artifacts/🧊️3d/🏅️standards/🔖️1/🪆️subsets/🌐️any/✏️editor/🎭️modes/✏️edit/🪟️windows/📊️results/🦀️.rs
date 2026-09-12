//! 📊️ Static, modal and buckling analysis views read their concrete Results window config.
//! [`config::Fem3dResultsWindowConfig`] owns the camera and result display selection.

#[path = "🎚️config/🦀️.rs"]
pub mod config;

use crate::app_surface::{DisplayMode, ResultDisplay};
use self::config::Fem3dResultsWindowConfig;
#[cfg(test)]
use crate::Fem3dSnapshot;
use crate::Viewport3dOrbit;
use semio_framework_plugin::BuiltNode;
#[cfg(test)]
use semio_framework_plugin::Label;
#[cfg(test)]
use semio_framework_ui_contract::{Buildable, HasChildren};

/// 🪟️ The manifest's Results window kind id.
pub const FEM3D_WINDOW_RESULTS: &str = "fem3d-results";
/// 📄️ The Results window's sole render body key.
pub const FEM3D_BODY_RESULTS: &str = "fem3d.play.results";

// #region 🔖️ConfigHelpers
/// 👁️ Project persisted configuration from the addressed results window for rendering.
fn config_result_display(cfg: &Fem3dResultsWindowConfig) -> ResultDisplay {
    let mode = cfg.result_mode.display(cfg.result_mode_index);
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

/// 📝️ Admits a result label into the fixture's semantic tree.
#[cfg(test)]
fn placeholder(label: Label) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    semio_framework_plugin::built_text_node(label).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "FEM result text admission failed"))
}

/// 🏷️ Places a data caption above the fixture's world scene.
#[cfg(test)]
fn with_caption(scene: BuiltNode, caption: String) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let caption = placeholder(Label::data(caption))?;
    let builder = semio_framework_ui_contract::column().try_children([caption, scene]).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "FEM caption children admission failed"))?;
    builder.try_build().map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "FEM caption node admission failed"))
}

/// 📊️ Results window dispatcher — picks the static/modal/buckling render based on `display`.
#[cfg(test)]
pub fn render(doc: &Fem3dSnapshot, cfg: &Fem3dResultsWindowConfig) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let display = config_result_display(cfg);
    let camera = &cfg.camera;
    match display.mode {
        DisplayMode::Static => render_static(doc, display.source_id.as_deref(), camera),
        DisplayMode::Modal(mode_index) => render_modal(doc, mode_index, camera),
        DisplayMode::Buckling(mode_index) => render_buckling(doc, display.source_id.as_deref(), mode_index, camera),
    }
}

/// 👁️ Adopts the immutable mounted result packet without solving, meshing, sorting, or encoding during render.
pub fn render_with_progress(camera: &Viewport3dOrbit, visual: Option<&crate::live_visual::Fem3dPageVisualLease>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let mut scene =
        semio_framework_plugin::world3d_scene(crate::viewport::scene_camera_json(camera), "[]".into(), "[]".into(), semio_framework_plugin::world3d_selection_json("rectangle", &[], None), &semio_framework_plugin::WorldSunConfig::default());
    scene.snapshot = visual.map(crate::live_visual::Fem3dPageVisualLease::snapshot);
    eprintln!("[DEBUG] fem3d results window render: liveVisualLease={} sceneSnapshot={}", visual.is_some(), scene.snapshot.is_some());
    crate::app_surface::world_3d_surface(FEM3D_BODY_RESULTS, &scene)
}

/// 📊️ Static results: solved fresh on every render (no cache, mirrors `Fem3dPlayApp`'s v0 design) —
/// same node/member/solid instances as the model window, offset by the solved displacements, solids
/// additionally colored by nodal-averaged von Mises stress. `source_id` selects a `fem3d_solve_all`
/// case/combination id, falling back to the first load case when `None`/unknown. Caption names the
/// active case.
#[cfg(test)]
fn render_static(doc: &Fem3dSnapshot, source_id: Option<&str>, camera: &Viewport3dOrbit) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    use crate::editor::fem3d::fem3d_scene_parts;
    use crate::fem3d_engine::fem3d_solve_all;

    let results = match fem3d_solve_all(doc) {
        Ok(results) => results,
        Err(e) => return placeholder(Label::data(format!("Analysis error: {e}"))),
    };
    let case_id = source_id.filter(|id| results.contains_key(*id)).map(str::to_string).or_else(|| doc.load_cases.first().map(|c| c.id.clone()));
    let Some(case_id) = case_id else {
        return placeholder(Label::data("No load case defined"));
    };
    let Some(result) = results.get(&case_id) else {
        return placeholder(Label::data(format!("Result not found: {case_id}")));
    };
    let mut disp_map: std::collections::HashMap<String, [f64; 6]> = std::collections::HashMap::new();
    for d in &result.displacements {
        disp_map.insert(d.node_id.clone(), d.values);
    }
    let nodal_stress = crate::fem3d_engine::mesh_preview::fem3d_nodal_von_mises(doc, &case_id).ok();
    let (meshes_json, instances_json) = fem3d_scene_parts(doc, Some(&disp_map), doc.analysis.deformation_scale, nodal_stress.as_ref());
    let scene = crate::app_surface::world_3d_surface(
        FEM3D_BODY_RESULTS,
        &semio_framework_plugin::world3d_scene(crate::viewport::scene_camera_json(camera), meshes_json, instances_json, semio_framework_plugin::world3d_selection_json("rectangle", &[], None), &semio_framework_plugin::WorldSunConfig::default()),
    );
    with_caption(scene?, format!("Case: {case_id}"))
}

/// 📊️ Modal mode-shape overlay: instances offset by the selected mode's shape, normalized to unit peak
/// then scaled to `MODE_SHAPE_AMPLITUDE_RATIO` of the model's own extent, with a frequency caption.
#[cfg(test)]
fn render_modal(doc: &Fem3dSnapshot, mode_index: usize, camera: &Viewport3dOrbit) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    use crate::app_surface::{normalize_mode_shape, MODE_SHAPE_AMPLITUDE_RATIO};
    use crate::editor::fem3d::fem3d_scene_parts;
    use crate::fem3d_engine::modal_buckling::fem3d_modal_mode_values;

    let (freq_hz, mut disp_map) = match fem3d_modal_mode_values(doc, mode_index) {
        Ok(values) => values,
        Err(e) => return placeholder(Label::data(format!("Modal analysis error: {e}"))),
    };
    normalize_mode_shape(&mut disp_map);
    let (meshes_json, instances_json) = fem3d_scene_parts(doc, Some(&disp_map), fem3d_model_extent(doc) * MODE_SHAPE_AMPLITUDE_RATIO, None);
    let scene = crate::app_surface::world_3d_surface(
        FEM3D_BODY_RESULTS,
        &semio_framework_plugin::world3d_scene(crate::viewport::scene_camera_json(camera), meshes_json, instances_json, semio_framework_plugin::world3d_selection_json("rectangle", &[], None), &semio_framework_plugin::WorldSunConfig::default()),
    );
    with_caption(scene?, format!("Mode {}: {freq_hz:.3} Hz", mode_index + 1))
}

/// 📊️ Buckling mode-shape overlay: instances offset by the selected mode's shape, normalized to unit
/// peak then scaled to `MODE_SHAPE_AMPLITUDE_RATIO` of the model's own extent. `source_id` selects the
/// reference load case, falling back to the first load case when `None`. Caption names the mode and its
/// load factor.
#[cfg(test)]
fn render_buckling(doc: &Fem3dSnapshot, source_id: Option<&str>, mode_index: usize, camera: &Viewport3dOrbit) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    use crate::app_surface::{normalize_mode_shape, MODE_SHAPE_AMPLITUDE_RATIO};
    use crate::editor::fem3d::fem3d_scene_parts;
    use crate::fem3d_engine::modal_buckling::fem3d_buckling_mode_values;

    let Some(case_id) = source_id.map(str::to_string).or_else(|| doc.load_cases.first().map(|c| c.id.clone())) else {
        return placeholder(Label::data("No load case defined"));
    };
    let (factor, mut disp_map) = match fem3d_buckling_mode_values(doc, &case_id, mode_index) {
        Ok(values) => values,
        Err(e) => return placeholder(Label::data(format!("Buckling analysis error: {e}"))),
    };
    normalize_mode_shape(&mut disp_map);
    let (meshes_json, instances_json) = fem3d_scene_parts(doc, Some(&disp_map), fem3d_model_extent(doc) * MODE_SHAPE_AMPLITUDE_RATIO, None);
    let scene = crate::app_surface::world_3d_surface(
        FEM3D_BODY_RESULTS,
        &semio_framework_plugin::world3d_scene(crate::viewport::scene_camera_json(camera), meshes_json, instances_json, semio_framework_plugin::world3d_selection_json("rectangle", &[], None), &semio_framework_plugin::WorldSunConfig::default()),
    );
    with_caption(scene?, format!("Buckling mode {}: factor {factor:.3}", mode_index + 1))
}
// #endregion 🔖️Render

// #region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🧪️Tests
