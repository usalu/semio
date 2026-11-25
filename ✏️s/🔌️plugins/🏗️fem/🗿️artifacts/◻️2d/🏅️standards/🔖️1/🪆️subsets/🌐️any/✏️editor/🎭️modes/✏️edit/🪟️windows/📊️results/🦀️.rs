//! 🏋️ von-Mises stress contours, reaction labels and moment diagrams.
//! 📊️ Fem2d play app — the results window: static/modal/buckling analysis views, nodal-averaged

pub use semio_s_artifact_fem_2d_results_state::config;
pub use semio_s_artifact_fem_2d_results_state::transient;

/// 🌈️ Triangle coordinates and scalar stress values at its vertices.
type StressContourTriangle = ([(f64, f64); 3], [f64; 3]);

use semio_s_fem_app_surface::hex_to_rgb01;

use semio_s_fem_app_surface::normalize_mode_shape;

use semio_s_fem_app_surface::DisplayMode;

use semio_s_fem_app_surface::ResultDisplay;

use semio_s_fem_app_surface::MODE_SHAPE_AMPLITUDE_RATIO;

use semio_s_fem_app_surface::VON_MISES_BANDS;
use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::fem2d_canvas_text_layer;
use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::fem2d_deformed_edge_layer;
use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::fem2d_deformed_shape_layers;
use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::fem2d_model_extent;
use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::fem2d_region_mesh_triangles;
use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::fem2d_structure_layers_with;
use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::MOMENT_SCALE_2D;
use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::NodeLiveField;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::fem2d_element_endpoints;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::find_node_2d;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::screen_2d;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::ORIGIN_2D;
use semio_s_artifact_fem_2d_editor_session::editor::fem2d::session::MountedFieldsView;
use semio_s_fem_model::ElementResult;
use semio_s_fem_model::Dof;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::element_id;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use semio_os_kernel::Viewport2d;
use semio_framework_pack_json::Value;
use semio_plugin::built_text_node;
use semio_plugin::BuiltNode;
use semio_plugin::Canvas2dScene;
use semio_ui_locale::Label;
use std::collections::{HashMap, HashSet};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "fem2d-results";
pub const BODY_KEY: &str = "fem2d.play.results";
const CAPTION_LINE: f64 = 16.0;
const LEGEND_GAP: f64 = 24.0;
//#endregion 🔖️Constants

/// 🧭️ Scene-space `(left, top, right)` of the model's nodes, so captions and legends sit beside the
/// structure instead of on top of it or its reaction read-outs.
pub fn model_scene_bounds(doc: &Fem2dSnapshot) -> (f64, f64, f64) {
    doc.nodes.iter().map(|node| screen_2d(node.x, node.y)).fold(None, |bounds: Option<(f64, f64, f64)>, (x, y)| Some(bounds.map_or((x, y, x), |(left, top, right)| (left.min(x), top.min(y), right.max(x))))).unwrap_or((ORIGIN_2D, ORIGIN_2D, ORIGIN_2D))
}

/// 🏷️ Scene position of caption `row`, counted upward from the line just above the model.
pub fn caption_position(doc: &Fem2dSnapshot, row: usize) -> (f64, f64) {
    let (left, top, _) = model_scene_bounds(doc);
    (left, top - CAPTION_LINE * (row as f64 + 2.0))
}

fn caption_layer(doc: &Fem2dSnapshot, id: &str, row: usize, content: &str, size: f64) -> Value {
    let (x, y) = caption_position(doc, row);
    fem2d_canvas_text_layer(id, x, y, content, size)
}

//#region 🔖️StressContourHelpers
/// 🌡️ A filled-triangle Canvas2d path layer (`segments` + `fill`, evenodd) for a contour cell —
/// see `framework/renderer/react/components/canvas-2d-host.tsx`'s `buildScenePath`/`drawSceneNode`
/// for the exact JSON shape this mirrors.
fn filled_triangle_layer(id: &str, p0: (f64, f64), p1: (f64, f64), p2: (f64, f64), color: &str, alpha: f64) -> Value {
    let (r, g, b) = hex_to_rgb01(color);
    semio_framework_pack_json::json!({
        "id": id,
        "transform": [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        "segments": [
            { "kind": "move", "to": [p0.0, p0.1] },
            { "kind": "line", "to": [p1.0, p1.1] },
            { "kind": "line", "to": [p2.0, p2.1] },
            { "kind": "close" },
        ],
        "fill": { "color": [r, g, b, alpha] },
    })
}

/// 🌡️ A filled polygon Canvas2d path layer (arbitrary vertex count) — the marching-triangle contour
/// bands need this (a clipped triangle can come out as a quad), unlike `filled_triangle_layer`'s
/// fixed 3-point shape.
fn filled_polygon_layer(id: &str, points: &[(f64, f64)], color: &str, alpha: f64) -> Value {
    let (r, g, b) = hex_to_rgb01(color);
    let mut segments = Vec::with_capacity(points.len() + 1);
    for (i, &(x, y)) in points.iter().enumerate() {
        segments.push(if i == 0 { semio_framework_pack_json::json!({ "kind": "move", "to": [x, y] }) } else { semio_framework_pack_json::json!({ "kind": "line", "to": [x, y] }) });
    }
    segments.push(semio_framework_pack_json::json!({ "kind": "close" }));
    semio_framework_pack_json::json!({
        "id": id,
        "transform": [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        "segments": segments,
        "fill": { "color": [r, g, b, alpha] },
    })
}

/// ✂️ One point of a polygon being clipped for contour banding: screen position plus the (linearly
/// interpolated across the source triangle) scalar value driving the clip.
type ValuedPoint = ((f64, f64), f64);

/// ✂️ Interpolates the crossing point where the segment `a->b`'s value equals `threshold`.
fn interpolate_at_value(a: ValuedPoint, b: ValuedPoint, threshold: f64) -> ValuedPoint {
    let t = if (b.1 - a.1).abs() < 1e-12 { 0.5 } else { (threshold - a.1) / (b.1 - a.1) };
    ((a.0 .0 + (b.0 .0 - a.0 .0) * t, a.0 .1 + (b.0 .1 - a.0 .1) * t), threshold)
}

/// ✂️ Sutherland-Hodgman clip of a (convex, value-carrying) polygon against a scalar half-plane —
/// keeps the portion where `value >= threshold` (`keep_above`) or `value <= threshold` (else),
/// inserting an interpolated vertex at every edge crossing. The core of marching-triangle contour
/// banding: clipping a triangle's linear value field against 2 thresholds bands it into one polygon.
fn clip_by_value(poly: &[ValuedPoint], threshold: f64, keep_above: bool) -> Vec<ValuedPoint> {
    if poly.is_empty() {
        return Vec::new();
    }
    let n = poly.len();
    let mut out = Vec::with_capacity(n + 1);
    for i in 0..n {
        let cur = poly[i];
        let prev = poly[(i + n - 1) % n];
        let cur_in = if keep_above { cur.1 >= threshold } else { cur.1 <= threshold };
        let prev_in = if keep_above { prev.1 >= threshold } else { prev.1 <= threshold };
        if cur_in {
            if !prev_in {
                out.push(interpolate_at_value(prev, cur, threshold));
            }
            out.push(cur);
        } else if prev_in {
            out.push(interpolate_at_value(prev, cur, threshold));
        }
    }
    out
}

/// 🌡️ A stress-contour legend: a small vertical stack of `VON_MISES_BANDS` swatches plus min/max text
/// labels, anchored to the right of the model's scene bounds.
fn von_mises_legend_layers(doc: &Fem2dSnapshot, min: f64, max: f64) -> Vec<Value> {
    let (_, top, right) = model_scene_bounds(doc);
    let left = right + LEGEND_GAP;
    let mut layers = Vec::with_capacity(VON_MISES_BANDS.len() + 2);
    for (i, color) in VON_MISES_BANDS.iter().enumerate() {
        let y = top + i as f64 * 14.0;
        layers.push(filled_triangle_layer(&format!("legend-swatch-{i}-a"), (left, y), (left + 16.0, y), (left + 16.0, y + 14.0), color, 1.0));
        layers.push(filled_triangle_layer(&format!("legend-swatch-{i}-b"), (left, y), (left + 16.0, y + 14.0), (left, y + 14.0), color, 1.0));
    }
    layers.push(fem2d_canvas_text_layer("legend-label-min", left + 20.0, top + VON_MISES_BANDS.len() as f64 * 14.0, &format!("{min:.1} Pa"), 11.0));
    layers.push(fem2d_canvas_text_layer("legend-label-max", left + 20.0, top + 8.0, &format!("{max:.1} Pa"), 11.0));
    layers
}
//#endregion 🔖️StressContourHelpers

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
struct Fem2dResultsCache {
    key: ResultsCacheKey,
    statics: Option<HashMap<String, semio_s_fem_model::StaticResult>>,
    modes: Vec<(ModeKey, ModeValues)>,
}

/// 🪪️ The app instance and the canonical revision its document stands at.
type ResultsCacheKey = (u32, [u8; 32]);

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
    static RESULTS_CACHE: std::cell::RefCell<Option<Fem2dResultsCache>> = const { std::cell::RefCell::new(None) };
    /// 🧪️ How many solver runs the cache actually let through, so a law can prove a frame was cheap.
    static RESULTS_SOLVES: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}

/// 🪪️ The cache key of one render, or `None` for a render outside any admitted operation.
pub fn results_cache_key(operation: Option<semio_plugin::AppRenderOperationContext>) -> Option<ResultsCacheKey> {
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

fn entry_for(cache: &mut Option<Fem2dResultsCache>, key: ResultsCacheKey) -> &mut Fem2dResultsCache {
    if cache.as_ref().map(|entry| entry.key) != Some(key) {
        *cache = Some(Fem2dResultsCache { key, statics: None, modes: Vec::new() });
    }
    cache.as_mut().expect("the cache entry was just admitted for this key")
}

fn solve_statics(doc: &Fem2dSnapshot) -> Result<HashMap<String, semio_s_fem_model::StaticResult>, String> {
    note_solve();
    semio_s_artifact_fem_2d_engine::fem2d_engine::fem2d_solve_all(doc).map_err(|error| error.to_string())
}

fn solve_mode(doc: &Fem2dSnapshot, key: &ModeKey) -> Result<ModeValues, String> {
    note_solve();
    let values = match key {
        ModeKey::Modal(index) => semio_s_artifact_fem_2d_engine::fem2d_engine::modal_buckling::fem2d_modal_mode_values(doc, *index),
        ModeKey::Buckling(case_id, index) => semio_s_artifact_fem_2d_engine::fem2d_engine::modal_buckling::fem2d_buckling_mode_values(doc, case_id, *index),
    };
    let (eigenvalue, mut shape) = values.map_err(|error| error.to_string())?;
    normalize_mode_shape(&mut shape);
    Ok((eigenvalue, shape))
}

/// 🧠️ Runs `read` over the document's solved static fields, solving at most once per revision.
pub fn with_static_results<T>(doc: &Fem2dSnapshot, key: Option<ResultsCacheKey>, read: impl FnOnce(&HashMap<String, semio_s_fem_model::StaticResult>) -> T) -> Result<T, String> {
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
pub fn with_mode_values<T>(doc: &Fem2dSnapshot, key: Option<ResultsCacheKey>, mode: ModeKey, read: impl FnOnce(&ModeValues) -> T) -> Result<T, String> {
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

//#region 🔖️Render
/// 📊️ Results window dispatcher — picks the static/modal/buckling render based on `display`.
pub fn render(
    doc: &Fem2dSnapshot,
    display: &ResultDisplay,
    camera: &Viewport2d,
    window: &config::Fem2dResultsWindowConfig,
    interaction: &semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::Fem2dInteractionSnapshot,
    operation: Option<semio_plugin::AppRenderOperationContext>,
    window_instance_id: Option<&str>,
    authority: Option<semio_plugin::WindowConfigAuthorityIdentity>,
    active_utility: &str,
    labels: &semio_s_artifact_fem_2d_editor_terminology::editor::fem2d::terminology::Fem2dLabels,
    locale: semio_plugin::Locale,
) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    let animation = &window.animation;
    if let Some(operation) = operation {
        return render_mounted(doc, display, camera, interaction, animation, operation, window_instance_id, authority, active_utility, labels, locale);
    }
    let key = results_cache_key(operation);
    match display.mode {
        DisplayMode::Static => render_static(doc, display.source_id.as_deref(), camera, interaction, animation, key, window_instance_id, active_utility, labels, locale),
        DisplayMode::Modal(mode_index) => render_modal(doc, mode_index, camera, interaction, animation, key, window_instance_id, active_utility, labels, locale),
        DisplayMode::Buckling(mode_index) => render_buckling(doc, display.source_id.as_deref(), mode_index, camera, interaction, animation, key, window_instance_id, active_utility, labels, locale),
    }
}

fn mounted_region_deformation_layers<'a>(
    triangles: impl Iterator<Item = [&'a str; 3]>,
    points: &HashMap<&str, ((f64, f64), bool)>,
) -> Vec<Value> {
    let mut edges = HashSet::new();
    let mut layers = Vec::new();
    for [a, b, c] in triangles {
        for (start, end) in [(a, b), (b, c), (c, a)] {
            let (start, end) = (start.min(end), start.max(end));
            let (Some(&(start_point, true)), Some(&(end_point, true))) = (points.get(start), points.get(end)) else { continue; };
            if edges.insert((start, end)) {
                layers.push(fem2d_deformed_edge_layer(format!("region-deformed-{}-{start}-{end}", start.len()), start_point, end_point));
            }
        }
    }
    layers
}

fn mounted_deformation_layers(doc: &Fem2dSnapshot, view: &MountedFieldsView<'_>, scale: f64, select: impl Fn(&NodeLiveField) -> [f64; 2]) -> Vec<Value> {
    let points: HashMap<_, _> = view.model_nodes().map(|(id, [x, y], field)| {
        let [dx, dy] = field.map(&select).unwrap_or([0.0; 2]);
        (id, (screen_2d(x + scale * dx, y + scale * dy), field.is_some()))
    }).collect();
    let mut layers: Vec<_> = doc.elements.iter().filter_map(|element| {
        let (start, end) = fem2d_element_endpoints(element);
        let (&(start, _), &(end, _)) = (points.get(start)?, points.get(end)?);
        Some(fem2d_deformed_edge_layer(format!("deformed-{}", element_id(element)), start, end))
    }).collect();
    layers.extend(mounted_region_deformation_layers(view.model_triangles(), &points));
    layers
}

fn render_mounted(
    doc: &Fem2dSnapshot,
    display: &ResultDisplay,
    camera: &Viewport2d,
    interaction: &semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::Fem2dInteractionSnapshot,
    animation: &semio_s_fem_app_surface::FemResultsAnimation,
    operation: semio_plugin::AppRenderOperationContext,
    window_instance_id: Option<&str>,
    authority: Option<semio_plugin::WindowConfigAuthorityIdentity>,
    active_utility: &str,
    labels: &semio_s_artifact_fem_2d_editor_terminology::editor::fem2d::terminology::Fem2dLabels,
    locale: semio_plugin::Locale,
) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    let layers = semio_s_artifact_fem_2d_editor_session::editor::fem2d::session::with_live_fields(operation, authority, |visual| {
        let mut layers = fem2d_structure_layers_with(doc, "#334155", "#334155", "#334155", interaction);
        use semio_s_fem_app_surface::ResultMode;
        let (family, index) = match display.mode {
            DisplayMode::Static => (ResultMode::Static, 0),
            DisplayMode::Modal(index) => (ResultMode::Modal, index),
            DisplayMode::Buckling(index) => (ResultMode::Buckling, index),
        };
        if let Some(view) = visual.filter(|view| view.matches_selection(doc, family, index, display.source_id.as_deref())) {
            let visual = view.progress;
            let amplitude = animation.amplitude();
            if family == ResultMode::Static {
                layers.extend(mounted_deformation_layers(doc, &view, doc.analysis.deformation_scale * amplitude, |field| field.displacement));
                if visual.validated_final || view.retained {
                    layers.extend(mounted_reaction_labels(doc, view.fields.iter(), amplitude));
                }
            } else if let Some(eigenvalue) = view.eigenvalue {
                if view.mode_shape_peak > 0.0 {
                    layers.extend(mounted_deformation_layers(doc, &view, mode_shape_scale(doc, amplitude) / view.mode_shape_peak, |field| field.mode_shape));
                }
                let number = index + 1;
                let caption = if family == ResultMode::Modal {
                    let hz = eigenvalue.sqrt() / std::f64::consts::TAU;
                    format!("{} {number}: {hz:.3} Hz", labels.mode_shape.as_str())
                } else {
                    format!("{} {number}: {eigenvalue:.3}", labels.buckling_mode.as_str())
                };
                layers.push(caption_layer(doc, "eigen-result", 2, &caption, 12.0));
            }
            let status = if matches!(visual.state, semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::FemVisualState::FaultedCancelled) { labels.failed.as_str() } else if visual.validated_final { labels.converged.as_str() } else { labels.pending.as_str() };
            let retained = if view.retained { labels.previous_results.as_str() } else { "" };
            let caption = if family != ResultMode::Static && visual.validated_final && view.eigenvalue.is_none() {
                labels.mode_unavailable.as_str().to_owned()
            } else {
                format!("{status} · {} / {} · {} {:.3e} {retained}", visual.progress_completed, visual.progress_total, labels.residual.as_str(), visual.residual_norm)
            };
            layers.push(caption_layer(doc, "analysis-progress", 0, caption.trim_end(), 11.0));
        } else {
            layers.push(caption_layer(doc, "analysis-progress", 0, labels.pending.as_str(), 11.0));
        }
        layers.extend(playback_caption_layer(doc, animation, labels));
        layers
    });
    let layers_json = finish_results_layers(doc, interaction, layers, window_instance_id, active_utility);
    semio_s_fem_app_surface::canvas_2d_surface(BODY_KEY, &Canvas2dScene { framing: None, camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json, snapshot: None, tool_run_trace: None, lanes: Vec::new() }, semio_plugin::surface_name(locale, "Results view", "Ergebnisansicht"))
}

/// 🎵️ The scale one mode-shape frame is drawn at: the unit-normalized shape lifted to
/// `MODE_SHAPE_AMPLITUDE_RATIO` of the model's own extent, then read through the playback waveform.
/// The SAME factor the static view applies to its solved displacements, so both views animate alike.
pub fn mode_shape_scale(doc: &Fem2dSnapshot, amplitude: f64) -> f64 {
    fem2d_model_extent(doc) * MODE_SHAPE_AMPLITUDE_RATIO * amplitude
}

/// ⏯️ The transport read-out a running window carries in its own corner — silent while stopped, so a
/// still results window looks exactly as it always did.
fn playback_caption_layer(doc: &Fem2dSnapshot, animation: &semio_s_fem_app_surface::FemResultsAnimation, labels: &semio_s_artifact_fem_2d_editor_terminology::editor::fem2d::terminology::Fem2dLabels) -> Option<Value> {
    animation.playing.then(|| {
        caption_layer(doc, "playback-caption", 1, &format!("{} {:.2} · \u{25b6} {} Hz", labels.phase.as_str(), animation.phase, animation.speed), 11.0)
    })
}

fn placeholder(label: Label) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    built_text_node(label).map_err(|_| semio_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fem2d results placeholder admission failed"))
}

const REACTION_LABEL_SIZE: f64 = 10.0;
const REACTION_LABEL_LINE: f64 = 13.0;
const REACTION_LABEL_PAD: f64 = 2.0;

/// 📐️ Screen-space bounds of one reaction read-out, used to keep labels from stacking on top of each other.
struct ReactionLabelBounds {
    left: f64,
    top: f64,
    right: f64,
    bottom: f64,
}

fn reaction_label_bounds(x: f64, y: f64, content: &str) -> ReactionLabelBounds {
    let width = content.len() as f64 * REACTION_LABEL_SIZE * 0.55;
    let height = REACTION_LABEL_SIZE * 1.25;
    ReactionLabelBounds { left: x - REACTION_LABEL_PAD, top: y - REACTION_LABEL_PAD, right: x + width + REACTION_LABEL_PAD, bottom: y + height + REACTION_LABEL_PAD }
}

fn reaction_label_boxes_overlap(a: &ReactionLabelBounds, b: &ReactionLabelBounds) -> bool {
    a.left < b.right && a.right > b.left && a.top < b.bottom && a.bottom > b.top
}

/// 📍️ Picks a screen position for one reaction label: stack DOFs at the same node, then nudge until no
/// earlier label's bounding box intersects this one.
fn place_reaction_label(anchor_x: f64, anchor_y: f64, stack_index: usize, content: &str, placed: &mut Vec<ReactionLabelBounds>) -> (f64, f64) {
    let base_x = anchor_x + 8.0;
    let base_y = anchor_y + 14.0 + stack_index as f64 * REACTION_LABEL_LINE;
    for attempt in 0..48 {
        let row = attempt / 3;
        let col = attempt % 3;
        let x = base_x + (col as f64 - 1.0) * 12.0;
        let y = base_y + row as f64 * REACTION_LABEL_LINE;
        let bounds = reaction_label_bounds(x, y, content);
        if !placed.iter().any(|existing| reaction_label_boxes_overlap(&bounds, existing)) {
            placed.push(bounds);
            return (x, y);
        }
    }
    let bounds = reaction_label_bounds(base_x, base_y, content);
    placed.push(bounds);
    (base_x, base_y)
}

fn mounted_reaction_labels<'a>(doc: &Fem2dSnapshot, fields: impl Iterator<Item = &'a NodeLiveField>, amplitude: f64) -> Vec<Value> {
    let values: HashMap<_, _> = fields.map(|field| (field.node_id.as_str(), field.reaction)).collect();
    let reactions = doc.supports.iter().flat_map(|support| {
        let node_id = support.node_id.as_str();
        let values = values.get(node_id);
        support.fixed.iter().filter_map(move |dof| {
            let dof = Dof::from(*dof);
            values?.get(dof.index()).map(|value| (node_id, dof, *value))
        })
    });
    reaction_labels(doc, reactions, amplitude)
}

fn reaction_labels<'a>(doc: &Fem2dSnapshot, reactions: impl Iterator<Item = (&'a str, Dof, f64)>, amplitude: f64) -> Vec<Value> {
    let mut entries: Vec<_> = reactions.collect();
    entries.sort_unstable_by_key(|(node_id, dof, _)| (*node_id, dof.index()));
    entries.dedup_by_key(|(node_id, dof, _)| (*node_id, dof.index()));
    let mut placed = Vec::new();
    let mut layers = Vec::with_capacity(entries.len());
    let mut previous_node = None;
    let mut stack_index = 0;
    for (node_id, dof, value) in entries {
        let Some(node) = find_node_2d(&doc.nodes, node_id) else { continue };
        if previous_node != Some(node_id) {
            previous_node = Some(node_id);
            stack_index = 0;
        }
        let (x, y) = screen_2d(node.x, node.y);
        let unit = match dof {
            Dof::Tx | Dof::Ty | Dof::Tz => "N",
            Dof::Rx | Dof::Ry | Dof::Rz => "N·m",
        };
        let content = format!("{dof:?}: {:.0} {unit}", value * amplitude);
        let (label_x, label_y) = place_reaction_label(x, y, stack_index, &content, &mut placed);
        layers.push(fem2d_canvas_text_layer(&format!("reaction-{node_id}-{dof:?}"), label_x, label_y, &content, REACTION_LABEL_SIZE));
        stack_index += 1;
    }
    layers
}

/// 📊️ Static results: undeformed structure faintly, plus a deformed-shape polyline, text labels at
/// every support reaction, (for beams) a moment-diagram polyline, and (for meshed regions) a
/// nodal-averaged, marching-triangle-banded von-Mises stress contour with a color-swatch legend.
/// `source_id` selects a `fem2d_solve_all` case/combination id, falling back to the first load case
/// when `None`/unknown (preserves v0's default behavior).
fn finish_results_layers(doc: &Fem2dSnapshot, interaction: &semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::Fem2dInteractionSnapshot, layers: Vec<Value>, window_instance_id: Option<&str>, active_utility: &str) -> String {
    let gumball_meta = semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::gumball::fem2d_gumball_meta_layer(doc, &interaction.selected_ids, active_utility, window_instance_id);
    semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::canvas_gesture::fem2d_finish_canvas_layers_json(layers, window_instance_id, active_utility, gumball_meta)
}

fn render_static(
    doc: &Fem2dSnapshot,
    source_id: Option<&str>,
    camera: &Viewport2d,
    interaction: &semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::Fem2dInteractionSnapshot,
    animation: &semio_s_fem_app_surface::FemResultsAnimation,
    key: Option<ResultsCacheKey>,
    window_instance_id: Option<&str>,
    active_utility: &str,
    labels: &semio_s_artifact_fem_2d_editor_terminology::editor::fem2d::terminology::Fem2dLabels,
    locale: semio_plugin::Locale,
) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    let case_id = source_id.map(str::to_string).or_else(|| doc.load_cases.first().and_then(|case| semio_s_fem_analyses_model::encode_result_source_parts("loadCase", &case.id).ok()));
    let Some(case_id) = case_id else {
        return placeholder(Label::data(labels.no_load_case.as_str()));
    };
    let amplitude = animation.amplitude();
    let scene = with_static_results(doc, key, |results| {
        let Some(result) = results.get(&case_id) else {
            return Err(format!("{}: {case_id}", labels.result_missing.as_str()));
        };
        Ok(static_layers(doc, result, interaction, animation, amplitude, labels))
    });
    let layers = match scene {
        Ok(Ok(layers)) => layers,
        Ok(Err(message)) => return placeholder(Label::data(message)),
        Err(error) => return placeholder(Label::data(format!("{}: {error}", labels.analysis_error.as_str()))),
    };
    let layers_json = finish_results_layers(doc, interaction, layers, window_instance_id, active_utility);
    semio_s_fem_app_surface::canvas_2d_surface(BODY_KEY, &Canvas2dScene { framing: None, camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json, snapshot: None, tool_run_trace: None, lanes: Vec::new() }, semio_plugin::surface_name(locale, "Results view", "Ergebnisansicht"))
}

/// 📊️ Every layer one static frame paints. `amplitude` is the waveform read of the playback phase:
/// the deformed shape, the reaction magnitudes and the moment diagram all ride the SAME factor, so a
/// frame is one consistent instant of the structure's motion rather than a mix of poses.
fn static_layers(
    doc: &Fem2dSnapshot,
    result: &semio_s_fem_model::StaticResult,
    interaction: &semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::Fem2dInteractionSnapshot,
    animation: &semio_s_fem_app_surface::FemResultsAnimation,
    amplitude: f64,
    labels: &semio_s_artifact_fem_2d_editor_terminology::editor::fem2d::terminology::Fem2dLabels,
) -> Vec<Value> {
    let mut layers = fem2d_structure_layers_with(doc, "#334155", "#334155", "#334155", interaction);
    let mut disp_map: HashMap<String, [f64; 6]> = HashMap::new();
    for d in &result.displacements {
        disp_map.insert(d.node_id.clone(), d.values);
    }
    layers.extend(fem2d_deformed_shape_layers(doc, &disp_map, doc.analysis.deformation_scale * amplitude));
    layers.extend(playback_caption_layer(doc, animation, labels));

    layers.extend(reaction_labels(doc, result.reactions.iter().map(|value| (value.node_id.as_str(), value.dof, value.value)), amplitude));

    for element in &doc.elements {
        let (start, end) = fem2d_element_endpoints(element);
        let (Some(n1), Some(n2)) = (find_node_2d(&doc.nodes, start), find_node_2d(&doc.nodes, end)) else { continue };
        let (x0, y0) = screen_2d(n1.x, n1.y);
        let (x1, y1) = screen_2d(n2.x, n2.y);
        if let Some((_, ElementResult::Beam { stations })) = result.elements.iter().find(|(id, _)| id.as_str() == element_id(element)) {
            let model_length = ((n2.x - n1.x).powi(2) + (n2.y - n1.y).powi(2)).sqrt().max(1e-9);
            let dx = x1 - x0;
            let dy = y1 - y0;
            let len = (dx * dx + dy * dy).sqrt().max(1e-9);
            let (px, py) = (-dy / len, dx / len);
            let points: Vec<[f64; 2]> = stations
                .iter()
                .map(|s| {
                    let t = s.x / model_length;
                    let bx = x0 + dx * t;
                    let by = y0 + dy * t;
                    [bx + px * s.m * MOMENT_SCALE_2D * amplitude, by + py * s.m * MOMENT_SCALE_2D * amplitude]
                })
                .collect();
            layers.push(semio_framework_pack_json::json!({
                "kind": "polyline",
                "id": format!("moment-{}", element_id(element)),
                "points": points,
                "color": "#fbbf24",
            }));
        }
    }

    //#region 🔖️StressContour
    let nodal_von_mises = semio_s_artifact_fem_2d_engine::fem2d_engine::mesh_preview::fem2d_nodal_von_mises(doc, result).unwrap_or_default();
    let mesh_triangles = fem2d_region_mesh_triangles(doc);
    let mut valued_triangles: Vec<StressContourTriangle> = Vec::new();
    for (_, tri_points, node_ids) in &mesh_triangles {
        if let (Some(&v0), Some(&v1), Some(&v2)) = (nodal_von_mises.get(&node_ids[0]), nodal_von_mises.get(&node_ids[1]), nodal_von_mises.get(&node_ids[2])) {
            valued_triangles.push((*tri_points, [v0, v1, v2]));
        }
    }
    if !valued_triangles.is_empty() {
        let min = valued_triangles.iter().flat_map(|(_, v)| v.iter().copied()).fold(f64::INFINITY, f64::min);
        let max = valued_triangles.iter().flat_map(|(_, v)| v.iter().copied()).fold(f64::NEG_INFINITY, f64::max);
        let n_bands = VON_MISES_BANDS.len();
        let span = (max - min).max(1e-9);
        let boundaries: Vec<f64> = (0..=n_bands).map(|k| min + span * k as f64 / n_bands as f64).collect();
        for (tri_index, (points, values)) in valued_triangles.iter().enumerate() {
            let base: Vec<ValuedPoint> = points.iter().zip(values.iter()).map(|(&p, &v)| (p, v)).collect();
            for band in 0..n_bands {
                let mut poly = base.clone();
                if band > 0 {
                    poly = clip_by_value(&poly, boundaries[band], true);
                }
                if band < n_bands - 1 {
                    poly = clip_by_value(&poly, boundaries[band + 1], false);
                }
                if poly.len() >= 3 {
                    let screen_points: Vec<(f64, f64)> = poly.iter().map(|(p, _)| *p).collect();
                    layers.push(filled_polygon_layer(&format!("contour-{tri_index}-{band}"), &screen_points, VON_MISES_BANDS[band], 0.85));
                }
            }
        }
        layers.extend(von_mises_legend_layers(doc, min, max));
    }
    //#endregion 🔖️StressContour

    layers
}

/// 📊️ Modal mode-shape overlay: undeformed structure faintly plus the selected mode's deformed-shape
/// polyline (normalized to unit peak, then scaled to `MODE_SHAPE_AMPLITUDE_RATIO` of the model's own
/// extent — see `normalize_mode_shape`) and a frequency caption.
fn render_modal(
    doc: &Fem2dSnapshot,
    mode_index: usize,
    camera: &Viewport2d,
    interaction: &semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::Fem2dInteractionSnapshot,
    animation: &semio_s_fem_app_surface::FemResultsAnimation,
    key: Option<ResultsCacheKey>,
    window_instance_id: Option<&str>,
    active_utility: &str,
    labels: &semio_s_artifact_fem_2d_editor_terminology::editor::fem2d::terminology::Fem2dLabels,
    locale: semio_plugin::Locale,
) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    let scale = mode_shape_scale(doc, animation.amplitude());
    let mode = with_mode_values(doc, key, ModeKey::Modal(mode_index), |(freq_hz, disp_map)| {
        let mut layers = fem2d_structure_layers_with(doc, "#334155", "#334155", "#334155", interaction);
        layers.extend(fem2d_deformed_shape_layers(doc, disp_map, scale));
        layers.push(caption_layer(doc, "modal-caption", 0, &format!("{} {}: {freq_hz:.3} Hz", labels.mode_shape.as_str(), mode_index + 1), 12.0));
        layers.extend(playback_caption_layer(doc, animation, labels));
        layers
    });
    let layers = match mode {
        Ok(layers) => layers,
        Err(error) => return placeholder(Label::data(format!("{}: {error}", labels.modal_error.as_str()))),
    };
    let layers_json = finish_results_layers(doc, interaction, layers, window_instance_id, active_utility);
    semio_s_fem_app_surface::canvas_2d_surface(BODY_KEY, &Canvas2dScene { framing: None, camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json, snapshot: None, tool_run_trace: None, lanes: Vec::new() }, semio_plugin::surface_name(locale, "Results view", "Ergebnisansicht"))
}

/// 📊️ Buckling mode-shape overlay: undeformed structure faintly plus the selected mode's deformed-shape
/// polyline (normalized to unit peak, then scaled to `MODE_SHAPE_AMPLITUDE_RATIO` of the model's own
/// extent — see `normalize_mode_shape`) and a load-factor caption. `source_id` selects the reference
/// load case, falling back to the first load case when `None`.
fn render_buckling(
    doc: &Fem2dSnapshot,
    source_id: Option<&str>,
    mode_index: usize,
    camera: &Viewport2d,
    interaction: &semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::Fem2dInteractionSnapshot,
    animation: &semio_s_fem_app_surface::FemResultsAnimation,
    key: Option<ResultsCacheKey>,
    window_instance_id: Option<&str>,
    active_utility: &str,
    labels: &semio_s_artifact_fem_2d_editor_terminology::editor::fem2d::terminology::Fem2dLabels,
    locale: semio_plugin::Locale,
) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    let source = source_id.map(semio_s_fem_analyses_model::decode_result_source_identity).transpose().map_err(|_| semio_plugin::PluginAssemblyError::new("fem2d.result-source", "invalid buckling result source identity"))?;
    let case_id = match source {
        Some(semio_s_fem_analyses_model::ResultSource::LoadCase(id)) => Some(id),
        Some(semio_s_fem_analyses_model::ResultSource::Combination(_)) => return Err(semio_plugin::PluginAssemblyError::new("fem2d.buckling-source", "combination buckling requires an admitted prestress source")),
        None => doc.load_cases.first().map(|case| case.id.clone()),
    };
    let Some(case_id) = case_id else {
        return placeholder(Label::data(labels.no_load_case.as_str()));
    };
    let scale = mode_shape_scale(doc, animation.amplitude());
    let mode = with_mode_values(doc, key, ModeKey::Buckling(case_id, mode_index), |(factor, disp_map)| {
        let mut layers = fem2d_structure_layers_with(doc, "#334155", "#334155", "#334155", interaction);
        layers.extend(fem2d_deformed_shape_layers(doc, disp_map, scale));
        layers.push(caption_layer(doc, "buckling-caption", 0, &format!("{} {}: {} {factor:.3}", labels.buckling_mode.as_str(), mode_index + 1, labels.factor.as_str()), 12.0));
        layers.extend(playback_caption_layer(doc, animation, labels));
        layers
    });
    let layers = match mode {
        Ok(layers) => layers,
        Err(error) => return placeholder(Label::data(format!("{}: {error}", labels.buckling_error.as_str()))),
    };
    let layers_json = finish_results_layers(doc, interaction, layers, window_instance_id, active_utility);
    semio_s_fem_app_surface::canvas_2d_surface(BODY_KEY, &Canvas2dScene { framing: None, camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json, snapshot: None, tool_run_trace: None, lanes: Vec::new() }, semio_plugin::surface_name(locale, "Results view", "Ergebnisansicht"))
}
//#endregion 🔖️Render

/// 🪟️ The first results-window instance of the layout — the one the results panel addresses when
/// the panel itself has no window context.
pub fn results_window_instance_id(view_state: &semio_plugin::ViewModel) -> Option<String> {
    let is_results = |id: &str| view_state.window_instances.iter().any(|window| window.id == id && window.window_kind_id == WINDOW_KIND_ID);
    view_state
        .window_id
        .as_deref()
        .filter(|id| is_results(id))
        .or_else(|| view_state.focused_window_id.as_deref().filter(|id| is_results(id)))
        .map(str::to_string)
        .or_else(|| view_state.window_instances.iter().find(|window| window.window_kind_id == WINDOW_KIND_ID).map(|window| window.id.clone()))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
