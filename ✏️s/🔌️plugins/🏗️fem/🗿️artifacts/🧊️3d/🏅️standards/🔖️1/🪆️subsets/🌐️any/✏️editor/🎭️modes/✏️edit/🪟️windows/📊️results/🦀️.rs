//! 👪️ [`config::Fem3dResultsWindowConfig`] owns the camera, the result display selection and the
//! 📊️ Static, modal and buckling analysis views read their concrete Results window config.
//! deformation transport; [`transient::FemResultsWindowTransient`] carries the running playback
//! clock while the window plays. Mounted views borrow bounded job fields and retain the last
//! completed source while its replacement runs. Fixture-only views use revision-scoped solves.

#[path = "🎚️config/🦀️.rs"]
pub mod config;
#[path = "🫧️transient/🦀️.rs"]
pub mod transient;

use self::config::Fem3dResultsWindowConfig;
use semio_s_fem_app_surface::normalize_mode_shape;
use semio_s_fem_app_surface::DisplayMode;
use semio_s_fem_app_surface::ResultDisplay;
use semio_s_fem_app_surface::MODE_SHAPE_AMPLITUDE_RATIO;
use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::gumball::fem3d_selection_json;
use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::Fem3dInteractionSnapshot;
use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::FEM3D_GRANULARITY_NODE;
use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::FEM3D_INTERACTION_DOMAIN;
use semio_s_artifact_fem_3d_scene::standards::v1::subsets::any::scene::fem3d_scene_parts;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_framework_plugin::BuiltNode;
use semio_framework_ui_contract::Buildable;
use semio_framework_ui_contract::HasChildren;
use semio_framework_ui_contract::HasStackLayout;
use std::collections::HashMap;

pub use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::{Fem3dResultRequest, Fem3dResultSource};

/// 🎯️ Resolves the displayed source and mode against the admitted document.
pub fn result_request(doc: &Fem3dSnapshot, source: Option<&str>, mode: Option<usize>) -> Result<Fem3dResultRequest, &'static str> {
    if mode.is_some_and(|index| index >= doc.analysis.modal_count) { return Err("fem3d.results-mode-missing"); }
    let source = match source {
        None => Fem3dResultSource::FirstCase,
        Some(identity) => {
            let source = semio_s_fem_analyses_model::borrow_result_source_identity(identity).map_err(|_| "fem3d.results-source-identity")?;
            match source.kind {
                "loadCase" => Fem3dResultSource::Case(doc.load_cases.iter().position(|case| case.id == source.id).ok_or("fem3d.results-source-missing")?),
                "combination" => Fem3dResultSource::Combination(doc.combinations.iter().position(|combination| combination.id == source.id).ok_or("fem3d.results-source-missing")?),
                _ => return Err("fem3d.results-source-kind"),
            }
        }
    };
    Ok(Fem3dResultRequest { source, mode, buckling: None })
}

/// 📊️ Validated result fields lent by the mounted session for one render.
pub trait Fem3dResultFields {
    fn snapshot(&self) -> &Fem3dSnapshot;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool { self.len() == 0 }
    fn node_id(&self, index: usize) -> Option<&str>;
    fn current_revision(&self) -> bool;
    fn static_values(&self, index: usize) -> Option<[f64; 6]>;
    fn mode_values(&self, index: usize) -> Option<([f64; 6], f64)>;
    fn stress(&self, index: usize) -> Option<f64>;
}

/// 🧵️ The mounted session as the results window sees it: the window depends on this contract, the session implements it.
pub trait Fem3dLiveSession {
    fn result_stage(&self, operation: Option<semio_framework_plugin::AppRenderOperationContext>, authority: Option<semio_framework_plugin::WindowConfigAuthorityIdentity>) -> Option<&'static str>;
    fn with_result_fields(&self, operation: Option<semio_framework_plugin::AppRenderOperationContext>, doc: &Fem3dSnapshot, authority: Option<semio_framework_plugin::WindowConfigAuthorityIdentity>, request: Fem3dResultRequest, read: &mut dyn FnMut(Option<&dyn Fem3dResultFields>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode>;
    fn with_live_visual(&self, operation: Option<semio_framework_plugin::AppRenderOperationContext>, build: &mut dyn FnMut(Option<&semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::Fem3dPageVisualLease>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode>;
}

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
type Statics = (HashMap<String, semio_s_fem_model::StaticResult>, HashMap<String, HashMap<String, f64>>);

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
    let results = semio_s_artifact_fem_3d_engine::fem3d_engine::fem3d_solve_all(doc).map_err(|error| error.to_string())?;
    let mut stresses = HashMap::with_capacity(results.len());
    for (id, result) in &results {
        if let Ok(field) = semio_s_artifact_fem_3d_engine::fem3d_engine::mesh_preview::fem3d_nodal_von_mises_of(doc, result) {
            stresses.insert(id.clone(), field);
        }
    }
    Ok((results, stresses))
}

fn solve_mode(doc: &Fem3dSnapshot, key: &ModeKey) -> Result<ModeValues, String> {
    note_solve();
    let values = match key {
        ModeKey::Modal(index) => semio_s_artifact_fem_3d_engine::fem3d_engine::modal_buckling::fem3d_modal_mode_values(doc, *index),
        ModeKey::Buckling(case_id, index) => semio_s_artifact_fem_3d_engine::fem3d_engine::modal_buckling::fem3d_buckling_mode_values(doc, case_id, *index),
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
/// drives mode-shape amplitude (see `semio_s_fem_app_surface::MODE_SHAPE_AMPLITUDE_RATIO`'s doc). Falls
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
    let caption = semio_framework_ui_contract::text(semio_s_artifact_fem_3d_editor_terminology::editor::fem3d::terminology::ui_label(caption)?);
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
fn playback_caption(animation: &semio_s_fem_app_surface::FemResultsAnimation) -> String {
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
    results_scene_with_visual(doc, window, interaction, displacements, scale, stress, None)
}

fn results_scene_with_visual(doc: &Fem3dSnapshot, window: &Fem3dResultsWindowConfig, interaction: &Fem3dInteractionSnapshot, displacements: Option<&HashMap<String, [f64; 6]>>, scale: f64, stress: Option<&HashMap<String, f64>>, visual: Option<&semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::Fem3dPageVisualLease>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let (meshes_json, instances_json) = fem3d_scene_parts(doc, displacements, scale, stress).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("fem3d.scene-identity", "FEM3d scene target identity refused"))?;
    let gumball = semio_s_artifact_fem_3d_editor_model_window::editor::fem3d::modes::edit::windows::model::config::Fem3dGumballConfig { move_axes: false, move_planes: false, rotate: false, scale_axes: false, scale_uniform: false };
    let mut scene = semio_framework_plugin::world3d_scene(semio_s_artifact_fem_3d_scene::viewport::scene_camera_json(&window.camera), meshes_json, instances_json, fem3d_selection_json(doc, interaction, false, &gumball), &semio_framework_plugin::WorldSunConfig::default());
    scene.domain_id = Some(FEM3D_INTERACTION_DOMAIN.into());
    scene.domain_granularity_id = Some(FEM3D_GRANULARITY_NODE.into());
    scene.snapshot = visual.map(semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::Fem3dPageVisualLease::snapshot);
    semio_s_fem_app_surface::world_3d_surface(FEM3D_BODY_RESULTS, &scene)
}

/// ⚠️ A solve that failed still shows the structure: the undeformed scene under a caption naming
/// the fault, instead of a blank pane the user cannot read the model from.
fn failed(doc: &Fem3dSnapshot, window: &Fem3dResultsWindowConfig, interaction: &Fem3dInteractionSnapshot, message: String) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    with_caption(results_scene(doc, window, interaction, None, doc.analysis.deformation_scale, None)?, message)
}

/// 📊️ Results window dispatcher — picks the static/modal/buckling render based on the window's
/// display selection, at the playback phase the window sits at.
pub fn render(doc: &Fem3dSnapshot, cfg: &Fem3dResultsWindowConfig, interaction: &Fem3dInteractionSnapshot, operation: Option<semio_framework_plugin::AppRenderOperationContext>, authority: Option<semio_framework_plugin::WindowConfigAuthorityIdentity>, live: &dyn Fem3dLiveSession) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let display = config_result_display(cfg);
    if operation.is_some() {
        let mode = match display.mode { DisplayMode::Modal(index) => Some(index), _ => None };
        let mut request = result_request(doc, display.source_id.as_deref(), mode).map_err(|message| semio_framework_plugin::PluginAssemblyError::new("fem3d.results-source", message))?;
        request.buckling = match display.mode { DisplayMode::Buckling(index) => Some(index), _ => None };
        let live_stage = live.result_stage(operation, authority);
        return live.with_result_fields(operation, doc, authority, request, &mut |fields| {
            let Some(fields) = fields else {
                return render_pending(doc, cfg, interaction, operation, authority, live);
            };
            let doc = fields.snapshot();
            let mut values = HashMap::with_capacity(fields.len());
            let mut stresses = HashMap::with_capacity(fields.len());
            let mut eigenvalue: f64 = 0.0;
            for index in 0..fields.len() {
                let Some(id) = fields.node_id(index) else { continue };
                let values_of_node = if display.mode.uses_mode_shape() {
                    let Some((components, eigen)) = fields.mode_values(index) else { continue };
                    eigenvalue = eigenvalue.max(eigen);
                    components
                } else {
                    let Some(components) = fields.static_values(index) else { continue };
                    components
                };
                values.insert(id.to_owned(), values_of_node);
                if let Some(stress) = fields.stress(index) { stresses.insert(id.to_owned(), stress); }
            }
            let scale = if display.mode.uses_mode_shape() { normalize_mode_shape(&mut values); mode_shape_scale(doc, cfg.animation.amplitude()) } else { doc.analysis.deformation_scale * cfg.animation.amplitude() };
            let mut caption = match display.mode {
                DisplayMode::Modal(index) => format!("Mode {}: {:.3} Hz{}", index + 1, display.mode.display_eigenvalue(eigenvalue).expect("modal display value"), playback_caption(&cfg.animation)),
                DisplayMode::Buckling(index) => format!("Buckling mode {}: factor {:.3}{}", index + 1, display.mode.display_eigenvalue(eigenvalue).expect("buckling display value"), playback_caption(&cfg.animation)),
                DisplayMode::Static => format!("Case: {}{}", display.source_id.as_deref().and_then(|identity| semio_s_fem_analyses_model::borrow_result_source_identity(identity).ok()).map(|source| source.id).or_else(|| doc.load_cases.first().map(|case| case.id.as_str())).unwrap_or("∅"), playback_caption(&cfg.animation)),
            };
            if !fields.current_revision() { caption.push_str(" · "); caption.push_str(progress_caption(live_stage)); }
            with_caption(results_scene(doc, cfg, interaction, Some(&values), scale, (!display.mode.uses_mode_shape()).then_some(&stresses))?, caption)
        });
    }
    let key = results_cache_key(operation);
    match display.mode {
        DisplayMode::Static => render_static(doc, cfg, interaction, display.source_id.as_deref(), key),
        DisplayMode::Modal(mode_index) => render_modal(doc, cfg, interaction, mode_index, key),
        DisplayMode::Buckling(mode_index) => render_buckling(doc, cfg, interaction, display.source_id.as_deref(), mode_index, key),
    }
}

/// ⏳️ Gestures and pending generations retain the last admitted visualization without solving.
pub fn render_pending(doc: &Fem3dSnapshot, cfg: &Fem3dResultsWindowConfig, interaction: &Fem3dInteractionSnapshot, operation: Option<semio_framework_plugin::AppRenderOperationContext>, authority: Option<semio_framework_plugin::WindowConfigAuthorityIdentity>, live: &dyn Fem3dLiveSession) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let mode = cfg.result_mode.display(cfg.result_mode_index);
    let stage = result_request(doc, cfg.result_source_id.as_deref(), match mode { DisplayMode::Modal(index) => Some(index), _ => None }).ok().and_then(|mut request| { request.buckling = match mode { DisplayMode::Buckling(index) => Some(index), _ => None }; live.result_stage(operation, authority) });
    live.with_live_visual(operation, &mut |visual| {
        with_caption(results_scene_with_visual(doc, cfg, interaction, None, doc.analysis.deformation_scale, None, visual)?, progress_caption(stage).into())
    })
}

fn progress_caption(stage: Option<&str>) -> &'static str {
    match stage {
        Some("fem3d.meshing") => "Generating mesh · Netz wird erzeugt",
        Some("fem3d.assembly") => "Assembling analysis · Berechnung wird aufgebaut",
        Some("fem3d.statics") => "Solving loads · Lasten werden berechnet",
        Some("fem3d.modes") => "Computing modes · Eigenformen werden berechnet",
        Some("fem3d.validation") => "Validating results · Ergebnisse werden geprüft",
        Some("fem3d.visual" | "fem3d.validated") => "Preparing result view · Ergebnisansicht wird vorbereitet",
        Some("fem3d.cancelled") => "Analysis cancelled · Berechnung abgebrochen",
        Some("fem3d.failed") => "Analysis failed · Berechnung fehlgeschlagen",
        Some(_) => "Preparing analysis · Berechnung wird vorbereitet",
        None => "Analysis queued · Berechnung vorgemerkt",
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
// #endregion 🧪️Tests

//#region 🪟️InstanceLookup
/// 🪟️ The first results-window instance of the layout — the one the results panel addresses when
/// the panel itself has no window context.
pub fn results_window_instance_id(view_state: &semio_framework_plugin::ViewModel) -> Option<String> {
    let is_results = |id: &str| view_state.window_instances.iter().any(|window| window.id == id && window.window_kind_id == FEM3D_WINDOW_RESULTS);
    view_state
        .window_id
        .as_deref()
        .filter(|id| is_results(id))
        .or_else(|| view_state.focused_window_id.as_deref().filter(|id| is_results(id)))
        .map(str::to_string)
        .or_else(|| view_state.window_instances.iter().find(|window| window.window_kind_id == FEM3D_WINDOW_RESULTS).map(|window| window.id.clone()))
}
//#endregion 🪟️InstanceLookup
