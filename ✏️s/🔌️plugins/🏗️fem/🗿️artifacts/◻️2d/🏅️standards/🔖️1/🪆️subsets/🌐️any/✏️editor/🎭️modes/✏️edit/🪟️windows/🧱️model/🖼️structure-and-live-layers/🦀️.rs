//! 🖼️ Structure and live layers: the vector layers of the structure, the emphasis rings, the region triangles, the deformed shape and the render entries. See [`super`].

use super::*;

//#region 👁️LiveVisualLanguage
pub(super) fn vector_layer(id: &str, origin: (f64, f64), vector: [f64; 2], color: &str) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::json!({
        "kind": "polyline",
        "id": id,
        "points": [[origin.0, origin.1], [origin.0 + vector[0], origin.1 - vector[1]]],
        "color": color,
    })
}

/// 👁️ Deterministic live overlays for mesh, assembly and iterative solve progress.
pub fn fem2d_live_visual_layers(doc: &Fem2dSnapshot, visual: &Fem2dLiveVisual) -> Vec<semio_framework_pack_json::Value> {
    let mut layers = Vec::new();
    let mut regions: Vec<_> = doc.regions.iter().collect();
    regions.sort_by(|a, b| a.id.cmp(&b.id));
    for region in regions {
        let quality = visual.region_quality.get(&region.id).copied().unwrap_or_default();
        let mut points: Vec<[f64; 2]> = region
            .outline
            .iter()
            .map(|point| {
                let (x, y) = screen_2d(point[0], point[1]);
                [x, y]
            })
            .collect();
        if let Some(first) = points.first().copied() {
            points.push(first);
        }
        layers.push(semio_framework_pack_json::json!({ "kind": "polyline", "id": format!("region-quality-{}-{}", quality.id(), region.id), "points": points, "color": quality.color() }));
    }
    let mut assembling = visual.assembling_element_ids.clone();
    assembling.sort();
    for id in assembling {
        let Some(element) = doc.elements.iter().find(|element| element_id(element) == id) else { continue };
        let (start, end) = fem2d_element_endpoints(element);
        let (Some(a), Some(b)) = (find_node_2d(&doc.nodes, start), find_node_2d(&doc.nodes, end)) else { continue };
        let (x0, y0) = screen_2d(a.x, a.y);
        let (x1, y1) = screen_2d(b.x, b.y);
        layers.push(semio_framework_pack_json::json!({ "kind": "line", "id": format!("assembling-{id}"), "x0": x0, "y0": y0, "x1": x1, "y1": y1, "color": "#a855f7" }));
    }
    let mut fields = visual.fields.clone();
    fields.sort_by(|a, b| a.node_id.cmp(&b.node_id));
    for field in fields {
        let Some(node) = find_node_2d(&doc.nodes, &field.node_id) else { continue };
        let origin = screen_2d(node.x, node.y);
        layers.push(vector_layer(&format!("displacement-field-{}", field.node_id), origin, [field.displacement[0] * SCALE_2D, field.displacement[1] * SCALE_2D], "#f472b6"));
        layers.push(vector_layer(&format!("residual-field-{}", field.node_id), origin, field.residual, "#eab308"));
    }
    let status = if visual.validated_final {
        "validated-final"
    } else if visual.converged {
        "converged"
    } else {
        "unconverged"
    };
    layers.push(semio_framework_pack_json::json!({ "id": format!("solve-status-{status}"), "transform": [1.0, 0.0, 0.0, 1.0, 10.0, 18.0], "text": { "content": status, "size": 11.0 } }));
    layers
}
//#endregion 👁️LiveVisualLanguage

//#region 🔖️SharedDrawHelpers
/// 📐️ Bounding-box diagonal (in model meters) over every node plus every region outline vertex — the
/// reference length `MODE_SHAPE_AMPLITUDE_RATIO` scales a normalized mode shape against. Falls back to
/// `1.0` for a degenerate (empty or point-like) model so mode-shape rendering never divides by zero.
pub fn fem2d_model_extent(doc: &Fem2dSnapshot) -> f64 {
    let mut min = [f64::INFINITY; 2];
    let mut max = [f64::NEG_INFINITY; 2];
    let mut expand = |x: f64, y: f64| {
        min[0] = min[0].min(x);
        min[1] = min[1].min(y);
        max[0] = max[0].max(x);
        max[1] = max[1].max(y);
    };
    for node in &doc.nodes {
        expand(node.x, node.y);
    }
    for region in &doc.regions {
        for p in &region.outline {
            expand(p[0], p[1]);
        }
    }
    if min[0] > max[0] {
        return 1.0;
    }
    let d = [max[0] - min[0], max[1] - min[1]];
    (d[0] * d[0] + d[1] * d[1]).sqrt().max(1.0)
}

/// 🖼️ Nodes/members/supports as Canvas2d layers — shared by this window (bright colors) and the results
/// window's faint undeformed backdrop (a single muted color for every layer kind).
pub(crate) fn fem2d_structure_layers(doc: &Fem2dSnapshot, node_color: &str, line_color: &str, support_color: &str) -> Vec<semio_framework_pack_json::Value> {
    let mut layers = Vec::new();
    for node in &doc.nodes {
        let (sx, sy) = screen_2d(node.x, node.y);
        layers.push(semio_framework_pack_json::json!({ "kind": "circle", "id": format!("node-{}", node.id), "x": sx - 4.0, "y": sy - 4.0, "width": 8.0, "height": 8.0, "color": node_color }));
    }
    for element in &doc.elements {
        let (start, end) = fem2d_element_endpoints(element);
        if let (Some(n1), Some(n2)) = (find_node_2d(&doc.nodes, start), find_node_2d(&doc.nodes, end)) {
            let (x0, y0) = screen_2d(n1.x, n1.y);
            let (x1, y1) = screen_2d(n2.x, n2.y);
            layers.push(semio_framework_pack_json::json!({ "kind": "line", "id": format!("el-{}", element_id(element)), "x0": x0, "y0": y0, "x1": x1, "y1": y1, "color": line_color }));
        }
    }
    for support in &doc.supports {
        if let Some(node) = find_node_2d(&doc.nodes, &support.node_id) {
            let (sx, sy) = screen_2d(node.x, node.y);
            layers.push(semio_framework_pack_json::json!({ "kind": "circle", "id": format!("support-{}", support.id), "x": sx - 5.0, "y": sy - 5.0, "width": 10.0, "height": 10.0, "color": support_color }));
        }
    }
    for case in &doc.load_cases {
        for load in &case.loads {
            match load {
                FemLoad::Nodal { id, node_id, dof, value } => {
                    let Some(node) = find_node_2d(&doc.nodes, node_id) else { continue };
                    let vector = match dof {
                        FemDof::Tx => [value.signum() * 18.0, 0.0],
                        FemDof::Ty => [0.0, value.signum() * 18.0],
                        _ => [0.0, -12.0],
                    };
                    layers.push(vector_layer(&format!("load-{id}"), screen_2d(node.x, node.y), vector, "#ef4444"));
                }
                FemLoad::MemberUdl { id, element_id: target, wx, wy } => {
                    let Some(element) = doc.elements.iter().find(|element| element_id(element) == target) else { continue };
                    let (start, end) = fem2d_element_endpoints(element);
                    let (Some(a), Some(b)) = (find_node_2d(&doc.nodes, start), find_node_2d(&doc.nodes, end)) else { continue };
                    layers.push(vector_layer(&format!("load-{id}"), screen_2d((a.x + b.x) * 0.5, (a.y + b.y) * 0.5), [wx.signum() * 18.0, wy.signum() * 18.0], "#ef4444"));
                }
                FemLoad::Area { id, region_id, pressure } => {
                    let Some(region) = doc.regions.iter().find(|region| region.id == *region_id) else { continue };
                    if region.outline.is_empty() {
                        continue;
                    }
                    let center = region.outline.iter().fold([0.0, 0.0], |sum, point| [sum[0] + point[0], sum[1] + point[1]]);
                    let count = region.outline.len() as f64;
                    layers.push(vector_layer(&format!("load-{id}"), screen_2d(center[0] / count, center[1] / count), [0.0, -pressure.signum() * 18.0], "#ef4444"));
                }
            }
        }
    }
    layers
}

//#region 🕹️SelectionHighlight
/// 🎨️ Emphasis colour for the framework-owned `"fem2d"` selection.
pub(crate) const SELECTION_COLOR_2D: &str = "#facc15";

/// 🎨️ Lighter emphasis colour for pointer hover.
pub(crate) const HOVER_COLOR_2D: &str = "#fde68a";

const SELECTION_STROKE_WIDTH_2D: f64 = 4.0;

const HOVER_STROKE_WIDTH_2D: f64 = 2.5;

/// 🎯️ A bounds-shaped emphasis ring around a point glyph — `selected` drives the Canvas2d host's own
/// selected-bounds treatment (`📐️Canvas2dHost/🟦️.tsx`'s `drawBoundsLayer`).
fn emphasis_ring_layer(id: &str, center: (f64, f64), radius: f64, color: &str, selected: bool) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::json!({ "kind": "circle", "id": id, "x": center.0 - radius, "y": center.1 - radius, "width": radius * 2.0, "height": radius * 2.0, "color": color, "selected": selected })
}

/// 🎯️ A stroked emphasis path over a member axis, a region outline or a load arrow — a `segments`
/// layer because only that shape honours an explicit `stroke.width`.
fn emphasis_path_layer(id: &str, points: &[(f64, f64)], close: bool, color: &str, width: f64) -> semio_framework_pack_json::Value {
    let (r, g, b) = semio_s_fem_app_surface::hex_to_rgb01(color);
    let mut segments = Vec::with_capacity(points.len() + 1);
    for (index, &(x, y)) in points.iter().enumerate() {
        segments.push(if index == 0 { semio_framework_pack_json::json!({ "kind": "move", "to": [x, y] }) } else { semio_framework_pack_json::json!({ "kind": "line", "to": [x, y] }) });
    }
    if close {
        segments.push(semio_framework_pack_json::json!({ "kind": "close" }));
    }
    semio_framework_pack_json::json!({ "id": id, "transform": [1.0, 0.0, 0.0, 1.0, 0.0, 0.0], "segments": segments, "stroke": { "color": [r, g, b, 1.0], "width": width } })
}

fn emphasis_layers_for(doc: &Fem2dSnapshot, ids: &[String], prefix: &str, color: &str, width: f64, selected: bool) -> Vec<semio_framework_pack_json::Value> {
    let mut layers = Vec::new();
    for target in ids {
        let Some(id) = semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::fem2d_entity_raw_id(target) else { continue };
        let kind = semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::fem2d_entity_kind(doc, target);
        if let Some(node) = find_node_2d(&doc.nodes, id).filter(|_| kind == Some("node")) {
            layers.push(emphasis_ring_layer(&format!("{prefix}-node-{id}"), screen_2d(node.x, node.y), width + 5.0, color, selected));
            continue;
        }
        if let Some(element) = doc.elements.iter().find(|element| element_id(element) == id && kind == Some("element")) {
            let (start, end) = fem2d_element_endpoints(element);
            if let (Some(a), Some(b)) = (find_node_2d(&doc.nodes, start), find_node_2d(&doc.nodes, end)) {
                layers.push(emphasis_path_layer(&format!("{prefix}-el-{id}"), &[screen_2d(a.x, a.y), screen_2d(b.x, b.y)], false, color, width));
            }
            continue;
        }
        if let Some(support) = doc.supports.iter().find(|support| support.id == id && kind == Some("support")) {
            if let Some(node) = find_node_2d(&doc.nodes, &support.node_id) {
                layers.push(emphasis_ring_layer(&format!("{prefix}-support-{id}"), screen_2d(node.x, node.y), width + 7.0, color, selected));
            }
            continue;
        }
        if let Some(region) = doc.regions.iter().find(|region| region.id == id && kind == Some("region")) {
            let points: Vec<(f64, f64)> = region.outline.iter().map(|point| screen_2d(point[0], point[1])).collect();
            if points.len() >= 3 {
                layers.push(emphasis_path_layer(&format!("{prefix}-region-{id}"), &points, true, color, width));
            }
            continue;
        }
        if let Some((_, load)) = semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::fem2d_load_owner(doc, target) {
            if let Some((from, to)) = semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::fem2d_load_glyph(doc, load) {
                layers.push(emphasis_path_layer(&format!("{prefix}-load-{target}"), &[from, to], false, color, width));
            }
        }
    }
    layers
}

/// 🕹️ The selected/hovered emphasis overlay for one interaction snapshot — hovered first, selected on
/// top, every layer id stable (`sel-node-n1`, `hov-el-e3`, …) so a host can diff frames.
pub(crate) fn fem2d_highlight_layers(doc: &Fem2dSnapshot, interaction: &semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::Fem2dInteractionSnapshot) -> Vec<semio_framework_pack_json::Value> {
    let mut layers = emphasis_layers_for(doc, &interaction.hovered_ids, "hov", HOVER_COLOR_2D, HOVER_STROKE_WIDTH_2D, false);
    layers.extend(emphasis_layers_for(doc, &interaction.selected_ids, "sel", SELECTION_COLOR_2D, SELECTION_STROKE_WIDTH_2D, true));
    layers
}

/// 🖼️ [`fem2d_structure_layers`] plus the interaction emphasis overlay — the entry point both windows
/// render through; the bare twin stays for the viewer app and the story fixtures, which have no
/// interaction state at all.
pub fn fem2d_structure_layers_with(doc: &Fem2dSnapshot, node_color: &str, line_color: &str, support_color: &str, interaction: &semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::Fem2dInteractionSnapshot) -> Vec<semio_framework_pack_json::Value> {
    let mut layers = fem2d_structure_layers(doc, node_color, line_color, support_color);
    layers.extend(fem2d_highlight_layers(doc, interaction));
    layers
}
//#endregion 🕹️SelectionHighlight

/// 🗺️ Every meshed region's triangles as `(element_id, [screen_p0, screen_p1, screen_p2])` — the
/// element id matches `fem2d_solve`/`fem2d_solve_all`'s `Tri3Cst` ids (`"{region_id}_t{tri_index}"`),
/// so callers can correlate a solved `ElementResult::Plane` back to on-screen triangle geometry. A
/// mesh failure for one region silently yields fewer triangles rather than failing the whole render.
pub(crate) fn fem2d_region_triangles(doc: &Fem2dSnapshot) -> Vec<(String, [(f64, f64); 3])> {
    let mut out = Vec::new();
    let Ok(meshes) = semio_s_artifact_fem_2d_engine::fem2d_engine::mesh_preview::fem2d_mesh_preview(doc) else { return out };
    for mesh in &meshes {
        for (tri_index, tri) in mesh.tris.iter().enumerate() {
            let id = format!("{}_t{}", mesh.region_id, tri_index);
            let p0 = mesh.points[tri[0] as usize];
            let p1 = mesh.points[tri[1] as usize];
            let p2 = mesh.points[tri[2] as usize];
            out.push((id, [screen_2d(p0[0], p0[1]), screen_2d(p1[0], p1[1]), screen_2d(p2[0], p2[1])]));
        }
    }
    out
}

/// 🗺️ Every meshed region's triangles as `(element_id, screen points, node ids)` — like
/// `fem2d_region_triangles` but also carrying each vertex's mesh node id, needed to look values up in
/// `fem2d_nodal_von_mises`'s node-keyed map for banded contour rendering.
pub fn fem2d_region_mesh_triangles(doc: &Fem2dSnapshot) -> Vec<Fem2dRegionMeshTriangle> {
    let mut out = Vec::new();
    let Ok(meshes) = semio_s_artifact_fem_2d_engine::fem2d_engine::mesh_preview::fem2d_mesh_preview(doc) else { return out };
    for mesh in &meshes {
        for (tri_index, tri) in mesh.tris.iter().enumerate() {
            let id = format!("{}_t{}", mesh.region_id, tri_index);
            let p0 = mesh.points[tri[0] as usize];
            let p1 = mesh.points[tri[1] as usize];
            let p2 = mesh.points[tri[2] as usize];
            let node_ids = [mesh.node_ids[tri[0] as usize].clone(), mesh.node_ids[tri[1] as usize].clone(), mesh.node_ids[tri[2] as usize].clone()];
            out.push((id, [screen_2d(p0[0], p0[1]), screen_2d(p1[0], p1[1]), screen_2d(p2[0], p2[1])], node_ids));
        }
    }
    out
}

/// 🖼️ Every element's deformed-shape polyline (pink), given a node-id-keyed displacement map and a
/// display scale — shared by the static, modal, and buckling results renders.
pub fn fem2d_deformed_shape_layers(doc: &Fem2dSnapshot, disp_map: &HashMap<String, [f64; 6]>, deform_scale: f64) -> Vec<semio_framework_pack_json::Value> {
    let mut layers = Vec::new();
    for element in &doc.elements {
        let (start, end) = fem2d_element_endpoints(element);
        let (Some(n1), Some(n2)) = (find_node_2d(&doc.nodes, start), find_node_2d(&doc.nodes, end)) else { continue };
        let (x0, y0) = screen_2d(n1.x, n1.y);
        let (x1, y1) = screen_2d(n2.x, n2.y);
        let d1 = disp_map.get(&n1.id).copied().unwrap_or([0.0; 6]);
        let d2 = disp_map.get(&n2.id).copied().unwrap_or([0.0; 6]);
        let dx0 = d1[Dof::Tx.index()] * deform_scale * SCALE_2D;
        let dy0 = -d1[Dof::Ty.index()] * deform_scale * SCALE_2D;
        let dx1 = d2[Dof::Tx.index()] * deform_scale * SCALE_2D;
        let dy1 = -d2[Dof::Ty.index()] * deform_scale * SCALE_2D;
        layers.push(semio_framework_pack_json::json!({
            "kind": "polyline",
            "id": format!("deformed-{}", element_id(element)),
            "points": [[x0 + dx0, y0 + dy0], [x1 + dx1, y1 + dy1]],
            "color": "#f472b6",
        }));
    }
    layers
}
//#endregion 🔖️SharedDrawHelpers

//#region 🔖️Render
pub fn render(doc: &Fem2dSnapshot, camera: &Viewport2d) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let mut layers = fem2d_structure_layers(doc, "#38bdf8", "#94a3b8", "#f97316");
    for (tri_index, (_, tri)) in fem2d_region_triangles(doc).iter().enumerate() {
        let [(x0, y0), (x1, y1), (x2, y2)] = *tri;
        layers.push(semio_framework_pack_json::json!({
            "kind": "polyline",
            "id": format!("mesh-edge-{tri_index}"),
            "points": [[x0, y0], [x1, y1], [x1, y1], [x2, y2], [x2, y2], [x0, y0]],
            "color": MESH_EDGE_COLOR,
        }));
    }
    let layers_json = semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Array(layers));
    semio_s_fem_app_surface::canvas_2d_surface(BODY_KEY, &Canvas2dScene { framing: None, camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json, snapshot: None, tool_run_trace: None, lanes: Vec::new() })
}

/// 👁️ Renders the document's own structure (nodes, members, supports, loads — a projection of the
/// snapshot, no meshing or solving) plus the optional replaceable worker-job visual lease: the mesh and
/// field pages ride the lease for a host that pages them, while every host that only draws
/// `layers_json` (the React canvas today) still shows the model as it is edited.
pub fn render_with_progress(
    doc: &Fem2dSnapshot,
    camera: &Viewport2d,
    progress: Option<&Fem2dMountedVisualLease>,
    interaction: &semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::Fem2dInteractionSnapshot,
    window_instance_id: Option<&str>,
    active_utility: &str,
) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let layers = fem2d_structure_layers_with(doc, "#38bdf8", "#94a3b8", "#f97316", interaction);
    let gumball_meta = semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::gumball::fem2d_gumball_meta_layer(doc, &interaction.selected_ids, active_utility, window_instance_id);
    let layers_json = semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::canvas_gesture::fem2d_finish_canvas_layers_json(layers, window_instance_id, active_utility, gumball_meta);
    semio_s_fem_app_surface::canvas_2d_surface(BODY_KEY, &Canvas2dScene { framing: None, camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json, snapshot: progress.map(Fem2dMountedVisualLease::snapshot), tool_run_trace: None, lanes: Vec::new() })
}
//#endregion 🔖️Render
