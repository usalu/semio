//! 📊️ Fem2d play app — the results window: static/modal/buckling analysis views, nodal-averaged
//! von-Mises stress contours, reaction labels and moment diagrams.

#[path = "🎚️config/🦀️.rs"]
pub mod config;

/// 🌈️ Triangle coordinates and scalar stress values at its vertices.
type StressContourTriangle = ([(f64, f64); 3], [f64; 3]);

use crate::app_surface::{hex_to_rgb01, normalize_mode_shape, DisplayMode, ResultDisplay, MODE_SHAPE_AMPLITUDE_RATIO, VON_MISES_BANDS};
use crate::editor::fem2d::modes::edit::windows::model::{fem2d_deformed_shape_layers, fem2d_element_endpoints, fem2d_model_extent, fem2d_region_mesh_triangles, fem2d_structure_layers_with, find_node_2d, screen_2d, MOMENT_SCALE_2D};
use crate::model::ElementResult;
use crate::{element_id, Fem2dSnapshot, Viewport2d};
use dsl::json::Value;
use semio_framework_plugin::{built_text_node, BuiltNode, Canvas2dScene, Label};
use std::collections::HashMap;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "fem2d-results";
pub const BODY_KEY: &str = "fem2d.play.results";
//#endregion 🔖️Constants

//#region 🔖️StressContourHelpers
/// 🌡️ A filled-triangle Canvas2d path layer (`segments` + `fill`, evenodd) for a contour cell —
/// see `framework/renderer/react/components/canvas-2d-host.tsx`'s `buildScenePath`/`drawSceneNode`
/// for the exact JSON shape this mirrors.
fn filled_triangle_layer(id: &str, p0: (f64, f64), p1: (f64, f64), p2: (f64, f64), color: &str, alpha: f64) -> Value {
    let (r, g, b) = hex_to_rgb01(color);
    dsl::json!({
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
        segments.push(if i == 0 { dsl::json!({ "kind": "move", "to": [x, y] }) } else { dsl::json!({ "kind": "line", "to": [x, y] }) });
    }
    segments.push(dsl::json!({ "kind": "close" }));
    dsl::json!({
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
/// labels, anchored near the canvas origin.
fn von_mises_legend_layers(min: f64, max: f64) -> Vec<Value> {
    let mut layers = Vec::with_capacity(VON_MISES_BANDS.len() + 2);
    for (i, color) in VON_MISES_BANDS.iter().enumerate() {
        let y = 20.0 + i as f64 * 14.0;
        layers.push(filled_triangle_layer(&format!("legend-swatch-{i}-a"), (10.0, y), (26.0, y), (26.0, y + 14.0), color, 1.0));
        layers.push(filled_triangle_layer(&format!("legend-swatch-{i}-b"), (10.0, y), (26.0, y + 14.0), (10.0, y + 14.0), color, 1.0));
    }
    layers.push(dsl::json!({
        "id": "legend-label-min",
        "transform": [1.0, 0.0, 0.0, 1.0, 30.0, 20.0 + VON_MISES_BANDS.len() as f64 * 14.0],
        "text": { "content": format!("{min:.1} Pa"), "size": 11.0 },
    }));
    layers.push(dsl::json!({
        "id": "legend-label-max",
        "transform": [1.0, 0.0, 0.0, 1.0, 30.0, 28.0],
        "text": { "content": format!("{max:.1} Pa"), "size": 11.0 },
    }));
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
    statics: Option<HashMap<String, crate::model::StaticResult>>,
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

fn entry_for(cache: &mut Option<Fem2dResultsCache>, key: ResultsCacheKey) -> &mut Fem2dResultsCache {
    if cache.as_ref().map(|entry| entry.key) != Some(key) {
        *cache = Some(Fem2dResultsCache { key, statics: None, modes: Vec::new() });
    }
    cache.as_mut().expect("the cache entry was just admitted for this key")
}

fn solve_statics(doc: &Fem2dSnapshot) -> Result<HashMap<String, crate::model::StaticResult>, String> {
    note_solve();
    crate::fem2d_engine::fem2d_solve_all(doc).map_err(|error| error.to_string())
}

fn solve_mode(doc: &Fem2dSnapshot, key: &ModeKey) -> Result<ModeValues, String> {
    note_solve();
    let values = match key {
        ModeKey::Modal(index) => crate::fem2d_engine::modal_buckling::fem2d_modal_mode_values(doc, *index),
        ModeKey::Buckling(case_id, index) => crate::fem2d_engine::modal_buckling::fem2d_buckling_mode_values(doc, case_id, *index),
    };
    let (eigenvalue, mut shape) = values.map_err(|error| error.to_string())?;
    normalize_mode_shape(&mut shape);
    Ok((eigenvalue, shape))
}

/// 🧠️ Runs `read` over the document's solved static fields, solving at most once per revision.
pub fn with_static_results<T>(doc: &Fem2dSnapshot, key: Option<ResultsCacheKey>, read: impl FnOnce(&HashMap<String, crate::model::StaticResult>) -> T) -> Result<T, String> {
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
    interaction: &crate::editor::fem2d::interaction::Fem2dInteractionSnapshot,
    operation: Option<semio_framework_plugin::AppRenderOperationContext>,
    window_instance_id: Option<&str>,
    active_utility: &str,
) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let animation = &window.animation;
    let key = results_cache_key(operation);
    match display.mode {
        DisplayMode::Static => render_static(doc, display.source_id.as_deref(), camera, interaction, animation, key, window_instance_id, active_utility),
        DisplayMode::Modal(mode_index) => render_modal(doc, mode_index, camera, interaction, animation, key, window_instance_id, active_utility),
        DisplayMode::Buckling(mode_index) => render_buckling(doc, display.source_id.as_deref(), mode_index, camera, interaction, animation, key, window_instance_id, active_utility),
    }
}

/// 🎵️ The scale one mode-shape frame is drawn at: the unit-normalized shape lifted to
/// `MODE_SHAPE_AMPLITUDE_RATIO` of the model's own extent, then read through the playback waveform.
/// The SAME factor the static view applies to its solved displacements, so both views animate alike.
pub fn mode_shape_scale(doc: &Fem2dSnapshot, amplitude: f64) -> f64 {
    fem2d_model_extent(doc) * MODE_SHAPE_AMPLITUDE_RATIO * amplitude
}

/// ⏯️ The transport read-out a running window carries in its own corner — silent while stopped, so a
/// still results window looks exactly as it always did.
fn playback_caption_layer(animation: &config::Fem2dResultsAnimation) -> Option<Value> {
    animation.playing.then(|| {
        dsl::json!({
            "id": "playback-caption",
            "transform": [1.0, 0.0, 0.0, 1.0, 10.0, 36.0],
            "text": { "content": format!("phase {:.2} · \u{25b6} {} Hz", animation.phase, animation.speed), "size": 11.0 },
        })
    })
}

fn placeholder(label: Label) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    built_text_node(label).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fem2d results placeholder admission failed"))
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

/// 📊️ Static results: undeformed structure faintly, plus a deformed-shape polyline, text labels at
/// every support reaction, (for beams) a moment-diagram polyline, and (for meshed regions) a
/// nodal-averaged, marching-triangle-banded von-Mises stress contour with a color-swatch legend.
/// `source_id` selects a `fem2d_solve_all` case/combination id, falling back to the first load case
/// when `None`/unknown (preserves v0's default behavior).
fn finish_results_layers(doc: &Fem2dSnapshot, interaction: &crate::editor::fem2d::interaction::Fem2dInteractionSnapshot, camera: &Viewport2d, layers: Vec<Value>, window_instance_id: Option<&str>, active_utility: &str) -> String {
    let gumball_meta = crate::editor::fem2d::interaction::gumball::fem2d_gumball_meta_layer(doc, &interaction.selected_ids, camera, active_utility, window_instance_id);
    crate::editor::fem2d::interaction::canvas_gesture::fem2d_finish_canvas_layers_json(layers, window_instance_id, active_utility, gumball_meta)
}

fn render_static(
    doc: &Fem2dSnapshot,
    source_id: Option<&str>,
    camera: &Viewport2d,
    interaction: &crate::editor::fem2d::interaction::Fem2dInteractionSnapshot,
    animation: &config::Fem2dResultsAnimation,
    key: Option<ResultsCacheKey>,
    window_instance_id: Option<&str>,
    active_utility: &str,
) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let amplitude = animation.amplitude();
    let scene = with_static_results(doc, key, |results| {
        let case_id = source_id.filter(|id| results.contains_key(*id)).map(str::to_string).or_else(|| doc.load_cases.first().map(|c| c.id.clone()));
        let Some(case_id) = case_id else {
            return Err("No load case defined".to_string());
        };
        let Some(result) = results.get(&case_id) else {
            return Err(format!("Result not found: {case_id}"));
        };
        Ok(static_layers(doc, &case_id, result, interaction, animation, amplitude))
    });
    let layers = match scene {
        Ok(Ok(layers)) => layers,
        Ok(Err(message)) => return placeholder(Label::data(message)),
        Err(error) => return placeholder(Label::data(format!("Analysis error: {error}"))),
    };
    let layers_json = finish_results_layers(doc, interaction, camera, layers, window_instance_id, active_utility);
    crate::app_surface::canvas_2d_surface(BODY_KEY, &Canvas2dScene { camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json, snapshot: None, tool_run_trace: None, lanes: Vec::new() })
}

/// 📊️ Every layer one static frame paints. `amplitude` is the waveform read of the playback phase:
/// the deformed shape, the reaction magnitudes and the moment diagram all ride the SAME factor, so a
/// frame is one consistent instant of the structure's motion rather than a mix of poses.
fn static_layers(
    doc: &Fem2dSnapshot,
    case_id: &str,
    result: &crate::model::StaticResult,
    interaction: &crate::editor::fem2d::interaction::Fem2dInteractionSnapshot,
    animation: &config::Fem2dResultsAnimation,
    amplitude: f64,
) -> Vec<Value> {
    let mut layers = fem2d_structure_layers_with(doc, "#334155", "#334155", "#334155", interaction);
    let mut disp_map: HashMap<String, [f64; 6]> = HashMap::new();
    for d in &result.displacements {
        disp_map.insert(d.node_id.clone(), d.values);
    }
    layers.extend(fem2d_deformed_shape_layers(doc, &disp_map, doc.analysis.deformation_scale * amplitude));
    layers.extend(playback_caption_layer(animation));

    //#region 🔖️ReactionLabels
    let mut reaction_entries: Vec<(&crate::model::NodeReaction, f64, f64)> = Vec::new();
    for reaction in &result.reactions {
        let Some(node) = find_node_2d(&doc.nodes, &reaction.node_id) else { continue };
        let (sx, sy) = screen_2d(node.x, node.y);
        reaction_entries.push((reaction, sx, sy));
    }
    reaction_entries.sort_by(|(left, ..), (right, ..)| left.node_id.cmp(&right.node_id).then(left.dof.index().cmp(&right.dof.index())));
    let mut stack_by_node: HashMap<String, usize> = HashMap::new();
    let mut placed_reaction_labels: Vec<ReactionLabelBounds> = Vec::new();
    for (reaction, sx, sy) in reaction_entries {
        let stack_index = {
            let entry = stack_by_node.entry(reaction.node_id.clone()).or_insert(0);
            let index = *entry;
            *entry += 1;
            index
        };
        let content = format!("{:?}: {:.0} N", reaction.dof, reaction.value * amplitude);
        let (label_x, label_y) = place_reaction_label(sx, sy, stack_index, &content, &mut placed_reaction_labels);
        layers.push(dsl::json!({
            "id": format!("reaction-{}-{:?}", reaction.node_id, reaction.dof),
            "transform": [1.0, 0.0, 0.0, 1.0, label_x, label_y],
            "text": { "content": content, "size": REACTION_LABEL_SIZE },
        }));
    }
    //#endregion 🔖️ReactionLabels

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
            layers.push(dsl::json!({
                "kind": "polyline",
                "id": format!("moment-{}", element_id(element)),
                "points": points,
                "color": "#fbbf24",
            }));
        }
    }

    //#region 🔖️StressContour
    let nodal_von_mises = crate::fem2d_engine::mesh_preview::fem2d_nodal_von_mises(doc, case_id).unwrap_or_default();
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
        layers.extend(von_mises_legend_layers(min, max));
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
    interaction: &crate::editor::fem2d::interaction::Fem2dInteractionSnapshot,
    animation: &config::Fem2dResultsAnimation,
    key: Option<ResultsCacheKey>,
    window_instance_id: Option<&str>,
    active_utility: &str,
) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let scale = mode_shape_scale(doc, animation.amplitude());
    let mode = with_mode_values(doc, key, ModeKey::Modal(mode_index), |(freq_hz, disp_map)| {
        let mut layers = fem2d_structure_layers_with(doc, "#334155", "#334155", "#334155", interaction);
        layers.extend(fem2d_deformed_shape_layers(doc, disp_map, scale));
        layers.push(dsl::json!({
            "id": "modal-caption",
            "transform": [1.0, 0.0, 0.0, 1.0, 10.0, 20.0],
            "text": { "content": format!("Mode {}: {freq_hz:.3} Hz", mode_index + 1), "size": 12.0 },
        }));
        layers.extend(playback_caption_layer(animation));
        layers
    });
    let layers = match mode {
        Ok(layers) => layers,
        Err(error) => return placeholder(Label::data(format!("Modal analysis error: {error}"))),
    };
    let layers_json = finish_results_layers(doc, interaction, camera, layers, window_instance_id, active_utility);
    crate::app_surface::canvas_2d_surface(BODY_KEY, &Canvas2dScene { camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json, snapshot: None, tool_run_trace: None, lanes: Vec::new() })
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
    interaction: &crate::editor::fem2d::interaction::Fem2dInteractionSnapshot,
    animation: &config::Fem2dResultsAnimation,
    key: Option<ResultsCacheKey>,
    window_instance_id: Option<&str>,
    active_utility: &str,
) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let Some(case_id) = source_id.map(str::to_string).or_else(|| doc.load_cases.first().map(|c| c.id.clone())) else {
        return placeholder(Label::data("No load case defined"));
    };
    let scale = mode_shape_scale(doc, animation.amplitude());
    let mode = with_mode_values(doc, key, ModeKey::Buckling(case_id, mode_index), |(factor, disp_map)| {
        let mut layers = fem2d_structure_layers_with(doc, "#334155", "#334155", "#334155", interaction);
        layers.extend(fem2d_deformed_shape_layers(doc, disp_map, scale));
        layers.push(dsl::json!({
            "id": "buckling-caption",
            "transform": [1.0, 0.0, 0.0, 1.0, 10.0, 20.0],
            "text": { "content": format!("Buckling mode {}: factor {factor:.3}", mode_index + 1), "size": 12.0 },
        }));
        layers.extend(playback_caption_layer(animation));
        layers
    });
    let layers = match mode {
        Ok(layers) => layers,
        Err(error) => return placeholder(Label::data(format!("Buckling analysis error: {error}"))),
    };
    let layers_json = finish_results_layers(doc, interaction, camera, layers, window_instance_id, active_utility);
    crate::app_surface::canvas_2d_surface(BODY_KEY, &Canvas2dScene { camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json, snapshot: None, tool_run_trace: None, lanes: Vec::new() })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
