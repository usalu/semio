//! 📊️ Static, modal and buckling analysis views read their concrete Results window config.
//! [`config::Fem3dResultsWindowConfig`] owns the camera, the result display selection and the
//! deformation playback state. The solved fields are cached per document revision, so playback
//! frames and selection repaints never re-run the solver, while every document edit — including
//! each coalesced transform step of a gumball drag — is a new revision and re-solves.

#[path = "🎚️config/🦀️.rs"]
pub mod config;

use self::config::Fem3dResultsWindowConfig;
use crate::app_surface::{normalize_mode_shape, DisplayMode, ResultDisplay, MODE_SHAPE_AMPLITUDE_RATIO};
use crate::editor::fem3d::interaction::gumball::fem3d_selection_json;
use crate::editor::fem3d::interaction::{Fem3dInteractionSnapshot, FEM3D_GRANULARITY_NODE, FEM3D_INTERACTION_DOMAIN};
use crate::standards::v1::subsets::any::scene::fem3d_scene_parts;
use crate::Fem3dSnapshot;
use semio_framework_plugin::BuiltNode;
use semio_framework_ui_contract::{Buildable, HasChildren, HasStackLayout};
use std::collections::HashMap;

/// 🪟️ The manifest's Results window kind id.
pub const FEM3D_WINDOW_RESULTS: &str = "fem3d-results";
/// 📄️ The Results window's sole render body key.
pub const FEM3D_BODY_RESULTS: &str = "fem3d.play.results";

// #region 🔖️ConfigHelpers
/// 👁️ Project persisted configuration from the addressed results window for rendering.
pub fn config_result_display(cfg: &Fem3dResultsWindowConfig) -> ResultDisplay {
    let mode = cfg.result_mode.display(cfg.result_mode_index);
    ResultDisplay { source_id: cfg.result_source_id.clone(), mode }
}
// #endregion 🔖️ConfigHelpers

//#region 🔖️ResultsCache
/// 🧠️ The solved fields of ONE document revision, held across the ~30 renders a second that
/// playback drives. Playback moves the PHASE, never the model, so re-solving per frame would burn a
/// full FEM assembly+factorisation on a scalar that already had every answer it needed.
///
/// Keyed by `(app_instance_id, canonical_base_revision)` off the render operation: a different key
/// is a different document (or a different app instance), and the whole entry is then DROPPED rather
/// than grown — exactly one revision is ever resident, so a long editing session cannot leak the
/// guest's heap one revision at a time. A render with no operation (every fixture render) bypasses
/// the cache completely and solves into the caller's own frame.
struct Fem3dResultsCache {
    key: ResultsCacheKey,
    statics: Option<Statics>,
    modes: Vec<(ModeKey, ModeValues)>,
}

/// 🪪️ The app instance and the canonical revision its document stands at.
pub type ResultsCacheKey = (u32, [u8; 32]);

/// 📊️ Every case's static result plus the nodal-averaged von Mises field per solved case.
type Statics = (HashMap<String, crate::model::StaticResult>, HashMap<String, HashMap<String, f64>>);

/// 🎵️ One normalized mode shape: its eigenvalue (frequency in Hz, or buckling factor) and values.
type ModeValues = (f64, HashMap<String, [f64; 6]>);

#[derive(Clone, PartialEq, Eq)]
pub enum ModeKey {
    Modal(usize),
    Buckling(String, usize),
}

/// 🧠️ How many distinct mode shapes ride along with one cached revision before the oldest is evicted.
const RESULTS_CACHE_MODES: usize = 8;

thread_local! {
    static RESULTS_CACHE: std::cell::RefCell<Option<Fem3dResultsCache>> = const { std::cell::RefCell::new(None) };
    /// 🧪️ How many solver runs the cache actually let through, so a law can prove a frame was cheap.
    static RESULTS_SOLVES: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}

/// 🪪️ The cache key of one render, or `None` for a render outside any admitted operation.
pub fn results_cache_key(operation: Option<semio_framework_plugin::AppRenderOperationContext>) -> Option<ResultsCacheKey> {
    operation.map(|operation| (operation.app_instance_id, operation.canonical_base_revision))
}

/// 🧪️ Solver runs since [`reset_results_cache`].
pub fn results_solve_count() -> u32 {
    RESULTS_SOLVES.with(std::cell::Cell::get)
}

/// 🧪️ Drops the resident revision and the solve counter — the starting state of every cache law.
pub fn reset_results_cache() {
    RESULTS_CACHE.with(|cache| *cache.borrow_mut() = None);
    RESULTS_SOLVES.with(|count| count.set(0));
}

fn note_solve() {
    RESULTS_SOLVES.with(|count| count.set(count.get().saturating_add(1)));
    eprintln!("[DEBUG] fem3d results solve #{}", results_solve_count());
}

fn entry_for(cache: &mut Option<Fem3dResultsCache>, key: ResultsCacheKey) -> &mut Fem3dResultsCache {
    if cache.as_ref().map(|entry| entry.key) != Some(key) {
        *cache = Some(Fem3dResultsCache { key, statics: None, modes: Vec::new() });
    }
    cache.as_mut().expect("the cache entry was just admitted for this key")
}

/// 📊️ Solves every case and combination, then averages the von Mises stress of each solved case
/// once — the stress field rides the same cache entry as the displacements it was derived from.
fn solve_statics(doc: &Fem3dSnapshot) -> Result<Statics, String> {
    note_solve();
    let results = crate::fem3d_engine::fem3d_solve_all(doc).map_err(|error| error.to_string())?;
    let mut stresses = HashMap::with_capacity(results.len());
    for (id, result) in &results {
        if let Ok(field) = crate::fem3d_engine::mesh_preview::fem3d_nodal_von_mises_of(doc, result) {
            stresses.insert(id.clone(), field);
        }
    }
    Ok((results, stresses))
}

fn solve_mode(doc: &Fem3dSnapshot, key: &ModeKey) -> Result<ModeValues, String> {
    note_solve();
    let values = match key {
        ModeKey::Modal(index) => crate::fem3d_engine::modal_buckling::fem3d_modal_mode_values(doc, *index),
        ModeKey::Buckling(case_id, index) => crate::fem3d_engine::modal_buckling::fem3d_buckling_mode_values(doc, case_id, *index),
    };
    let (eigenvalue, mut shape) = values.map_err(|error| error.to_string())?;
    normalize_mode_shape(&mut shape);
    Ok((eigenvalue, shape))
}

/// 🧠️ Runs `read` over the document's solved static fields, solving at most once per revision.
pub fn with_static_results<T>(doc: &Fem3dSnapshot, key: Option<ResultsCacheKey>, read: impl FnOnce(&Statics) -> T) -> Result<T, String> {
    let Some(key) = key else {
        return Ok(read(&solve_statics(doc)?));
    };
    RESULTS_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        let entry = entry_for(&mut cache, key);
        if entry.statics.is_none() {
            entry.statics = Some(solve_statics(doc)?);
        }
        Ok(read(entry.statics.as_ref().expect("the static results were just admitted")))
    })
}

/// 🧠️ Runs `read` over one normalized mode shape, solving at most once per (revision, mode).
pub fn with_mode_values<T>(doc: &Fem3dSnapshot, key: Option<ResultsCacheKey>, mode: ModeKey, read: impl FnOnce(&ModeValues) -> T) -> Result<T, String> {
    let Some(key) = key else {
        return Ok(read(&solve_mode(doc, &mode)?));
    };
    RESULTS_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        let entry = entry_for(&mut cache, key);
        if !entry.modes.iter().any(|(candidate, _)| candidate == &mode) {
            let values = solve_mode(doc, &mode)?;
            if entry.modes.len() >= RESULTS_CACHE_MODES {
                entry.modes.remove(0);
            }
            entry.modes.push((mode.clone(), values));
        }
        let (_, values) = entry.modes.iter().find(|(candidate, _)| candidate == &mode).expect("the mode values were just admitted");
        Ok(read(values))
    })
}
//#endregion 🔖️ResultsCache

// #region 🔖️Render
/// 📐️ Bounding-box diagonal (in model meters) over every node plus every solid's footprint/height —
/// drives mode-shape amplitude (see `crate::app_surface::MODE_SHAPE_AMPLITUDE_RATIO`'s doc). Falls
/// back to `1.0` for a degenerate model.
pub fn fem3d_model_extent(doc: &Fem3dSnapshot) -> f64 {
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    let mut expand = |point: [f64; 3]| {
        for axis in 0..3 {
            min[axis] = min[axis].min(point[axis]);
            max[axis] = max[axis].max(point[axis]);
        }
    };
    for node in &doc.nodes {
        expand([node.x, node.y, node.z]);
    }
    for solid in &doc.solids {
        for p in &solid.outline {
            expand(solid.axis.to_world(p[0], p[1], solid.base_z));
            expand(solid.axis.to_world(p[0], p[1], solid.base_z + solid.height));
        }
    }
    if min[0] > max[0] {
        return 1.0;
    }
    let d = [max[0] - min[0], max[1] - min[1], max[2] - min[2]];
    (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt().max(1.0)
}

/// 🎵️ The scale one mode-shape frame is drawn at: the unit-normalized shape lifted to
/// `MODE_SHAPE_AMPLITUDE_RATIO` of the model's own extent, then read through the playback waveform.
pub fn mode_shape_scale(doc: &Fem3dSnapshot, amplitude: f64) -> f64 {
    fem3d_model_extent(doc) * MODE_SHAPE_AMPLITUDE_RATIO * amplitude
}

/// 🏷️ Places a data caption above the world scene — both siblings enter the column as BUILDERS so
/// the column keys them positionally; a pre-built keyless node already carries `"#0"`, and two of
/// those under one parent are a `duplicate-key` projection fault.
fn with_caption(scene: BuiltNode, caption: String) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let admission = |detail: &'static str| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", detail);
    let caption = semio_framework_ui_contract::text(crate::editor::fem3d::ui_label(caption)?);
    let scene_stack = semio_framework_ui_contract::column().grow(true).try_child(scene).map_err(|_| admission("FEM caption scene child admission failed"))?;
    semio_framework_ui_contract::column()
        .grow(true)
        .try_child(caption)
        .map_err(|_| admission("FEM caption text admission failed"))?
        .try_child(scene_stack)
        .map_err(|_| admission("FEM caption scene stack admission failed"))?
        .try_build()
        .map_err(|_| admission("FEM caption node admission failed"))
}

/// ⏯️ The transport read-out a running window carries in its caption — silent while stopped.
fn playback_caption(animation: &config::Fem3dResultsAnimation) -> String {
    if animation.playing {
        format!(" · phase {:.2} · \u{25b6} {} Hz", animation.phase, animation.speed)
    } else {
        String::new()
    }
}

/// 🎬️ One results frame: the scene deformed by `displacements` at `scale`, painted with the shared
/// `fem3d` selection so a pick in the model window is visible here too, and bound to the domain so
/// a pick HERE lands in the same selection.
fn results_scene(doc: &Fem3dSnapshot, window: &Fem3dResultsWindowConfig, interaction: &Fem3dInteractionSnapshot, displacements: Option<&HashMap<String, [f64; 6]>>, scale: f64, stress: Option<&HashMap<String, f64>>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let (meshes_json, instances_json) = fem3d_scene_parts(doc, displacements, scale, stress);
    let gumball = crate::editor::fem3d::modes::edit::windows::model::config::Fem3dGumballConfig { move_axes: false, move_planes: false, rotate: false, scale_axes: false, scale_uniform: false };
    let mut scene = semio_framework_plugin::world3d_scene(crate::viewport::scene_camera_json(&window.camera), meshes_json, instances_json, fem3d_selection_json(doc, interaction, false, &gumball), &semio_framework_plugin::WorldSunConfig::default());
    scene.domain_id = Some(FEM3D_INTERACTION_DOMAIN.into());
    scene.domain_granularity_id = Some(FEM3D_GRANULARITY_NODE.into());
    crate::app_surface::world_3d_surface(FEM3D_BODY_RESULTS, &scene)
}

/// ⚠️ A solve that failed still shows the structure: the undeformed scene under a caption naming
/// the fault, instead of a blank pane the user cannot read the model from.
fn failed(doc: &Fem3dSnapshot, window: &Fem3dResultsWindowConfig, interaction: &Fem3dInteractionSnapshot, message: String) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    with_caption(results_scene(doc, window, interaction, None, doc.analysis.deformation_scale, None)?, message)
}

/// 📊️ Results window dispatcher — picks the static/modal/buckling render based on the window's
/// display selection, at the playback phase the window sits at.
pub fn render(doc: &Fem3dSnapshot, cfg: &Fem3dResultsWindowConfig, interaction: &Fem3dInteractionSnapshot, operation: Option<semio_framework_plugin::AppRenderOperationContext>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let display = config_result_display(cfg);
    let key = results_cache_key(operation);
    match display.mode {
        DisplayMode::Static => render_static(doc, cfg, interaction, display.source_id.as_deref(), key),
        DisplayMode::Modal(mode_index) => render_modal(doc, cfg, interaction, mode_index, key),
        DisplayMode::Buckling(mode_index) => render_buckling(doc, cfg, interaction, display.source_id.as_deref(), mode_index, key),
    }
}

/// 📊️ Static results: the same node/member/solid instances as the model window, offset by the
/// solved displacements read through the playback amplitude, solids additionally colored by
/// nodal-averaged von Mises stress. `source_id` selects a `fem3d_solve_all` case/combination id,
/// falling back to the first load case when `None`/unknown. Caption names the active case.
fn render_static(doc: &Fem3dSnapshot, cfg: &Fem3dResultsWindowConfig, interaction: &Fem3dInteractionSnapshot, source_id: Option<&str>, key: Option<ResultsCacheKey>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    if doc.load_cases.is_empty() {
        return failed(doc, cfg, interaction, "No load case defined".into());
    }
    let amplitude = cfg.animation.amplitude();
    let frame = with_static_results(doc, key, |(results, stresses)| {
        let case_id = source_id.filter(|id| results.contains_key(*id)).map(str::to_string).or_else(|| doc.load_cases.first().map(|c| c.id.clone()));
        let Some(case_id) = case_id else {
            return Err("No load case defined".to_string());
        };
        let Some(result) = results.get(&case_id) else {
            return Err(format!("Result not found: {case_id}"));
        };
        let disp_map: HashMap<String, [f64; 6]> = result.displacements.iter().map(|d| (d.node_id.clone(), d.values)).collect();
        Ok((case_id.clone(), results_scene(doc, cfg, interaction, Some(&disp_map), doc.analysis.deformation_scale * amplitude, stresses.get(&case_id))))
    });
    match frame {
        Ok(Ok((case_id, scene))) => with_caption(scene?, format!("Case: {case_id}{}", playback_caption(&cfg.animation))),
        Ok(Err(message)) => failed(doc, cfg, interaction, message),
        Err(error) => failed(doc, cfg, interaction, format!("Analysis error: {error}")),
    }
}

/// 📊️ Modal mode-shape overlay: instances offset by the selected mode's shape, normalized to unit peak
/// then scaled to `MODE_SHAPE_AMPLITUDE_RATIO` of the model's own extent, with a frequency caption.
fn render_modal(doc: &Fem3dSnapshot, cfg: &Fem3dResultsWindowConfig, interaction: &Fem3dInteractionSnapshot, mode_index: usize, key: Option<ResultsCacheKey>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let amplitude = cfg.animation.amplitude();
    match with_mode_values(doc, key, ModeKey::Modal(mode_index), |(freq_hz, shape)| (*freq_hz, results_scene(doc, cfg, interaction, Some(shape), mode_shape_scale(doc, amplitude), None))) {
        Ok((freq_hz, scene)) => with_caption(scene?, format!("Mode {}: {freq_hz:.3} Hz{}", mode_index + 1, playback_caption(&cfg.animation))),
        Err(error) => failed(doc, cfg, interaction, format!("Modal analysis error: {error}")),
    }
}

/// 📊️ Buckling mode-shape overlay: instances offset by the selected mode's shape, normalized to unit
/// peak then scaled to `MODE_SHAPE_AMPLITUDE_RATIO` of the model's own extent. `source_id` selects the
/// reference load case, falling back to the first load case when `None`. Caption names the mode and its
/// load factor.
fn render_buckling(doc: &Fem3dSnapshot, cfg: &Fem3dResultsWindowConfig, interaction: &Fem3dInteractionSnapshot, source_id: Option<&str>, mode_index: usize, key: Option<ResultsCacheKey>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let Some(case_id) = source_id.map(str::to_string).or_else(|| doc.load_cases.first().map(|c| c.id.clone())) else {
        return failed(doc, cfg, interaction, "No load case defined".into());
    };
    let amplitude = cfg.animation.amplitude();
    match with_mode_values(doc, key, ModeKey::Buckling(case_id, mode_index), |(factor, shape)| (*factor, results_scene(doc, cfg, interaction, Some(shape), mode_shape_scale(doc, amplitude), None))) {
        Ok((factor, scene)) => with_caption(scene?, format!("Buckling mode {}: factor {factor:.3}{}", mode_index + 1, playback_caption(&cfg.animation))),
        Err(error) => failed(doc, cfg, interaction, format!("Buckling analysis error: {error}")),
    }
}
// #endregion 🔖️Render

// #region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🧪️Tests
