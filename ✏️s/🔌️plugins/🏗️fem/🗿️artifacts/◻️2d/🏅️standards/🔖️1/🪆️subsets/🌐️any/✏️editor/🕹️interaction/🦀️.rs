//! 🕹️ Fem2d play app — the framework-owned interaction domain: its definition, the selection/hover
//! snapshot every render reads, screen-space hit-testing of document entities and the
//! `interactionSelect`/`interactionHover` request effects a pick emits. Slice A fills this in.

use crate::editor::fem2d::modes::edit::windows::model as model_window;
use crate::editor::fem2d::modes::edit::windows::results as results_window;
use crate::{element_id, Fem2dSnapshot, FemDof, FemLoad, Viewport2d};
use model_window::{fem2d_element_endpoints, find_node_2d, screen_2d, ORIGIN_2D, SCALE_2D};
use semio_framework::kernel::Effect;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{ConfigView, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, LocalizedLabel, MergeMode, NoConfig, SelectionMethod, SelectionMode, SelectionSpec, ViewModel};

//#region 🔖️Constants
pub const FEM2D_INTERACTION_DOMAIN: &str = "fem2d";
pub const FEM2D_GRANULARITY_NODE: &str = "node";
pub const FEM2D_GRANULARITY_ELEMENT: &str = "element";
pub const FEM2D_GRANULARITY_REGION: &str = "region";
pub const FEM2D_GRANULARITY_SUPPORT: &str = "support";
pub const FEM2D_GRANULARITY_LOAD: &str = "load";
pub const FEM2D_GRANULARITY_MATERIAL: &str = "material";
pub const FEM2D_GRANULARITY_SECTION: &str = "section";
pub const FEM2D_GRANULARITY_LOAD_CASE: &str = "loadCase";
pub const FEM2D_GRANULARITY_COMBINATION: &str = "combination";
pub const FEM2D_POINTER_CHANNEL: &str = "pointer";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🕹️ The one fem2d interaction domain: every document entity kind is a granularity and a target id
/// is the raw entity id, so a tree row, a viewport pick and the inspector all agree on one vocabulary.
pub fn fem2d_interaction_definition() -> InteractionDefinition {
    let granularity = |id: &str, en: &str, de: &str, icon: &str| GranularityDefinition { id: id.into(), label: LocalizedLabel::native(en, de), icon_id: icon.into() };
    InteractionDefinition {
        id: FEM2D_INTERACTION_DOMAIN.into(),
        label: LocalizedLabel::native("Structure", "Tragwerk"),
        granularities: vec![
            granularity(FEM2D_GRANULARITY_NODE, "Node", "Knoten", "circle-dot"),
            granularity(FEM2D_GRANULARITY_ELEMENT, "Element", "Element", "minus"),
            granularity(FEM2D_GRANULARITY_REGION, "Region", "Bereich", "square"),
            granularity(FEM2D_GRANULARITY_SUPPORT, "Support", "Lager", "anchor"),
            granularity(FEM2D_GRANULARITY_LOAD, "Load", "Last", "arrow-down"),
            granularity(FEM2D_GRANULARITY_MATERIAL, "Material", "Material", "layers"),
            granularity(FEM2D_GRANULARITY_SECTION, "Section", "Querschnitt", "ruler"),
            granularity(FEM2D_GRANULARITY_LOAD_CASE, "Load Case", "Lastfall", "list"),
            granularity(FEM2D_GRANULARITY_COMBINATION, "Combination", "Kombination", "link"),
        ],
        hierarchy: HierarchyProvider::Flat,
        hover: HoverSpec::default(),
        selection: SelectionSpec {
            modes: vec![SelectionMode::Multiple, SelectionMode::Single],
            methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle],
            merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
            transitive: false,
            broadcast: true,
        },
    }
}
//#endregion 🔖️Definition

//#region 🔖️Snapshot
/// 🕹️ The immutable interaction snapshot one render reads: selected and pointer-hovered entity ids.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Fem2dInteractionSnapshot {
    pub selected_ids: Vec<String>,
    pub hovered_ids: Vec<String>,
}

impl Fem2dInteractionSnapshot {
    pub fn from_interaction(interaction: &InteractionView<'_>) -> Self {
        Self { selected_ids: interaction.selection(FEM2D_INTERACTION_DOMAIN).ids.clone(), hovered_ids: interaction.hover(FEM2D_INTERACTION_DOMAIN, FEM2D_POINTER_CHANNEL).ids.clone() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️ScreenSpace
/// 🎯️ Canvas-pixel pick radius for a node glyph (`fem2d_structure_layers` draws it 8 layer-units wide).
pub const FEM2D_NODE_PICK_RADIUS_PX: f64 = 8.0;
/// 🎯️ Canvas-pixel pick radius for a support glyph — wider than the node it sits on, so the ring
/// around an exactly-hit node still resolves to the support.
pub const FEM2D_SUPPORT_PICK_RADIUS_PX: f64 = 11.0;
/// 🎯️ Canvas-pixel pick tolerance against a load's arrow glyph.
pub const FEM2D_LOAD_PICK_RADIUS_PX: f64 = 6.0;
/// 🎯️ Canvas-pixel pick tolerance against a member's axis.
pub const FEM2D_ELEMENT_PICK_RADIUS_PX: f64 = 6.0;

/// 🎯️ One pick: the granularity the hit entity belongs to plus its raw document id.
pub type Fem2dPick = (&'static str, String);

fn effective_zoom(camera: &Viewport2d) -> f64 {
    if camera.zoom.abs() < 1e-9 {
        1.0
    } else {
        camera.zoom
    }
}

/// 📐️ Layer-space (the coordinates `screen_2d` emits into `layers_json`) to canvas pixels — the exact
/// inverse-free twin of `📐️Canvas2dHost/🟦️.tsx`'s `worldToScreenLogical`, which is what actually puts
/// a layer on screen.
pub fn fem2d_layer_to_canvas(camera: &Viewport2d, point: (f64, f64), width: f64, height: f64) -> (f64, f64) {
    let zoom = effective_zoom(camera);
    ((point.0 - camera.x) * zoom + width * 0.5, (point.1 - camera.y) * zoom + height * 0.5)
}

/// 📐️ Canvas pixels back to model meters — `screenToWorldLogical` composed with the inverse of
/// `screen_2d` (`x * SCALE_2D + ORIGIN_2D`, `-y * SCALE_2D + ORIGIN_2D`).
pub fn fem2d_canvas_to_model(camera: &Viewport2d, x: f64, y: f64, width: f64, height: f64) -> (f64, f64) {
    let zoom = effective_zoom(camera);
    let layer_x = (x - width * 0.5) / zoom + camera.x;
    let layer_y = (y - height * 0.5) / zoom + camera.y;
    ((layer_x - ORIGIN_2D) / SCALE_2D, -(layer_y - ORIGIN_2D) / SCALE_2D)
}

fn distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

fn distance_to_segment(point: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let length_squared = dx * dx + dy * dy;
    if length_squared < 1e-12 {
        return distance(point, a);
    }
    let t = (((point.0 - a.0) * dx + (point.1 - a.1) * dy) / length_squared).clamp(0.0, 1.0);
    distance(point, (a.0 + dx * t, a.1 + dy * t))
}

fn point_in_polygon(point: (f64, f64), polygon: &[[f64; 2]]) -> bool {
    if polygon.len() < 3 {
        return false;
    }
    let mut inside = false;
    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        let (xi, yi) = (polygon[i][0], polygon[i][1]);
        let (xj, yj) = (polygon[j][0], polygon[j][1]);
        if (yi > point.1) != (yj > point.1) && point.0 < (xj - xi) * (point.1 - yi) / (yj - yi) + xi {
            inside = !inside;
        }
        j = i;
    }
    inside
}
//#endregion 🔖️ScreenSpace

//#region 🔖️Geometry
/// 🏋️ The layer-space arrow glyph one load is drawn as — start point and end point, matching
/// `fem2d_structure_layers`' `vector_layer` calls exactly (its `y` component is negated on the way
/// into layer space).
pub(crate) fn fem2d_load_glyph(doc: &Fem2dSnapshot, load: &FemLoad) -> Option<((f64, f64), (f64, f64))> {
    let (origin, vector) = match load {
        FemLoad::Nodal { node_id, dof, value, .. } => {
            let node = find_node_2d(&doc.nodes, node_id)?;
            let vector = match dof {
                FemDof::Tx => [value.signum() * 18.0, 0.0],
                FemDof::Ty => [0.0, value.signum() * 18.0],
                _ => [0.0, -12.0],
            };
            (screen_2d(node.x, node.y), vector)
        }
        FemLoad::MemberUdl { element_id: target, wx, wy, .. } => {
            let element = doc.elements.iter().find(|element| element_id(element) == target)?;
            let (start, end) = fem2d_element_endpoints(element);
            let (a, b) = (find_node_2d(&doc.nodes, start)?, find_node_2d(&doc.nodes, end)?);
            (screen_2d((a.x + b.x) * 0.5, (a.y + b.y) * 0.5), [wx.signum() * 18.0, wy.signum() * 18.0])
        }
        FemLoad::Area { region_id, pressure, .. } => {
            let region = doc.regions.iter().find(|region| region.id == *region_id)?;
            let center = fem2d_region_centroid(region)?;
            (screen_2d(center.0, center.1), [0.0, -pressure.signum() * 18.0])
        }
    };
    Some((origin, (origin.0 + vector[0], origin.1 - vector[1])))
}

/// 🗺️ A region's outline centroid in model meters — the point a load glyph anchors to and the point
/// `focusEntity` frames.
pub(crate) fn fem2d_region_centroid(region: &crate::FemRegion) -> Option<(f64, f64)> {
    if region.outline.is_empty() {
        return None;
    }
    let sum = region.outline.iter().fold([0.0, 0.0], |sum, point| [sum[0] + point[0], sum[1] + point[1]]);
    let count = region.outline.len() as f64;
    Some((sum[0] / count, sum[1] / count))
}

/// 🪪️ The granularity one raw document id belongs to, searched in the same precedence the viewport
/// picks with — the artifact tree and the inspector resolve a selected id through this one lookup.
pub fn fem2d_entity_kind(doc: &Fem2dSnapshot, id: &str) -> Option<&'static str> {
    if doc.nodes.iter().any(|node| node.id == id) {
        return Some(FEM2D_GRANULARITY_NODE);
    }
    if doc.elements.iter().any(|element| element_id(element) == id) {
        return Some(FEM2D_GRANULARITY_ELEMENT);
    }
    if doc.supports.iter().any(|support| support.id == id) {
        return Some(FEM2D_GRANULARITY_SUPPORT);
    }
    if fem2d_load_owner(doc, id).is_some() {
        return Some(FEM2D_GRANULARITY_LOAD);
    }
    if doc.regions.iter().any(|region| region.id == id) {
        return Some(FEM2D_GRANULARITY_REGION);
    }
    if doc.materials.iter().any(|material| material.id == id) {
        return Some(FEM2D_GRANULARITY_MATERIAL);
    }
    if doc.sections.iter().any(|section| section.id == id) {
        return Some(FEM2D_GRANULARITY_SECTION);
    }
    if doc.load_cases.iter().any(|case| case.id == id) {
        return Some(FEM2D_GRANULARITY_LOAD_CASE);
    }
    if doc.combinations.iter().any(|combination| combination.id == id) {
        return Some(FEM2D_GRANULARITY_COMBINATION);
    }
    None
}

/// 🏋️ The load case owning one load id, plus the load itself — a load target carries only the raw
/// load id, so every consumer (inspector patch, focus, highlight) resolves its case through here.
pub fn fem2d_load_owner<'a>(doc: &'a Fem2dSnapshot, load: &str) -> Option<(&'a str, &'a FemLoad)> {
    doc.load_cases.iter().find_map(|case| case.loads.iter().find(|candidate| crate::load_id(candidate) == load).map(|found| (case.id.as_str(), found)))
}

/// 🎯️ The model-space point one entity is framed on — a node's position, a member's midpoint, a
/// region's centroid, a support's node, a load's anchor.
pub fn fem2d_entity_model_point(doc: &Fem2dSnapshot, id: &str) -> Option<(f64, f64)> {
    if let Some(node) = find_node_2d(&doc.nodes, id) {
        return Some((node.x, node.y));
    }
    if let Some(element) = doc.elements.iter().find(|element| element_id(element) == id) {
        let (start, end) = fem2d_element_endpoints(element);
        let (a, b) = (find_node_2d(&doc.nodes, start)?, find_node_2d(&doc.nodes, end)?);
        return Some(((a.x + b.x) * 0.5, (a.y + b.y) * 0.5));
    }
    if let Some(support) = doc.supports.iter().find(|support| support.id == id) {
        let node = find_node_2d(&doc.nodes, &support.node_id)?;
        return Some((node.x, node.y));
    }
    if let Some(region) = doc.regions.iter().find(|region| region.id == id) {
        return fem2d_region_centroid(region);
    }
    if let Some((_, load)) = fem2d_load_owner(doc, id) {
        return match load {
            FemLoad::Nodal { node_id, .. } => find_node_2d(&doc.nodes, node_id).map(|node| (node.x, node.y)),
            FemLoad::MemberUdl { element_id: target, .. } => fem2d_entity_model_point(doc, target),
            FemLoad::Area { region_id, .. } => doc.regions.iter().find(|region| region.id == *region_id).and_then(fem2d_region_centroid),
        };
    }
    None
}
//#endregion 🔖️Geometry

//#region 🔖️HitTest
/// 🎯️ Screen-space viewport pick against the document, in the exact projection the model window
/// draws with (`screen_2d` into layer space, then the Canvas2d camera into canvas pixels).
///
/// Precedence — nodes, supports, loads, members, regions: the small point-like glyphs win over the
/// large area-like ones, and a node beats the support drawn on top of it so a click exactly on a
/// joint always selects the joint.
pub fn fem2d_hit_test(doc: &Fem2dSnapshot, camera: &Viewport2d, x: f64, y: f64, width: f64, height: f64) -> Option<Fem2dPick> {
    let pointer = (x, y);
    let at = |model: (f64, f64)| fem2d_layer_to_canvas(camera, screen_2d(model.0, model.1), width, height);
    let layer_at = |layer: (f64, f64)| fem2d_layer_to_canvas(camera, layer, width, height);
    let mut best: Option<(f64, Fem2dPick)> = None;
    let keep = |score: f64, pick: Fem2dPick, best: &mut Option<(f64, Fem2dPick)>| {
        if best.as_ref().is_none_or(|(current, _)| score < *current) {
            *best = Some((score, pick));
        }
    };

    for node in &doc.nodes {
        let score = distance(pointer, at((node.x, node.y)));
        if score <= FEM2D_NODE_PICK_RADIUS_PX {
            keep(score, (FEM2D_GRANULARITY_NODE, node.id.clone()), &mut best);
        }
    }
    if let Some((_, pick)) = best.take() {
        return Some(pick);
    }

    for support in &doc.supports {
        let Some(node) = find_node_2d(&doc.nodes, &support.node_id) else { continue };
        let score = distance(pointer, at((node.x, node.y)));
        if score <= FEM2D_SUPPORT_PICK_RADIUS_PX {
            keep(score, (FEM2D_GRANULARITY_SUPPORT, support.id.clone()), &mut best);
        }
    }
    if let Some((_, pick)) = best.take() {
        return Some(pick);
    }

    for case in &doc.load_cases {
        for load in &case.loads {
            let Some((start, end)) = fem2d_load_glyph(doc, load) else { continue };
            let score = distance_to_segment(pointer, layer_at(start), layer_at(end));
            if score <= FEM2D_LOAD_PICK_RADIUS_PX {
                keep(score, (FEM2D_GRANULARITY_LOAD, crate::load_id(load).to_string()), &mut best);
            }
        }
    }
    if let Some((_, pick)) = best.take() {
        return Some(pick);
    }

    for element in &doc.elements {
        let (start, end) = fem2d_element_endpoints(element);
        let (Some(a), Some(b)) = (find_node_2d(&doc.nodes, start), find_node_2d(&doc.nodes, end)) else { continue };
        let score = distance_to_segment(pointer, at((a.x, a.y)), at((b.x, b.y)));
        if score <= FEM2D_ELEMENT_PICK_RADIUS_PX {
            keep(score, (FEM2D_GRANULARITY_ELEMENT, element_id(element).to_string()), &mut best);
        }
    }
    if let Some((_, pick)) = best.take() {
        return Some(pick);
    }

    let model = fem2d_canvas_to_model(camera, x, y, width, height);
    doc.regions
        .iter()
        .find(|region| point_in_polygon(model, &region.outline) && !region.holes.iter().any(|hole| point_in_polygon(model, hole)))
        .map(|region| (FEM2D_GRANULARITY_REGION, region.id.clone()))
}
//#endregion 🔖️HitTest

//#region 🔖️Effects
/// 🎯️ Maps shift/ctrl/meta to a framework `MergeMode` wire label — the same policy every other
/// viewport in this repo picks with (`🖍️draw/…/🖱️canvas-pointer-down/🦀️.rs`'s `selection_merge_mode`).
pub fn selection_merge_mode(shift: bool, ctrl: bool, meta: bool) -> &'static str {
    let ctrl_or_meta = ctrl || meta;
    if shift && ctrl_or_meta {
        "invertive"
    } else if shift {
        "additive"
    } else if ctrl_or_meta {
        "subtractive"
    } else {
        "replace"
    }
}

fn targets_json<G: AsRef<str>, I: AsRef<str>>(targets: &[(G, I)]) -> String {
    let items: Vec<dsl::json::Value> = targets.iter().map(|(granularity, id)| dsl::json!({ "granularity": granularity.as_ref(), "id": id.as_ref() })).collect();
    dsl::json::to_string(&dsl::json::Value::Array(items))
}

fn request_interaction_action(action_id: &str, args: dsl::DslValue) -> Effect {
    Effect::ReplayShellCommand { action_id: action_id.into(), args: Some(args) }
}

/// 🕹️ Asks the shell to redispatch `interactionSelect` for this pick — selection is framework-owned
/// state, so an app never writes it, it only reports WHICH ids the pointer hit.
pub fn interaction_select_effect<G: AsRef<str>, I: AsRef<str>>(targets: &[(G, I)], merge: &str) -> Effect {
    request_interaction_action(
        semio_framework::INTERACTION_SELECT_ACTION_ID,
        dsl::DslValue::object([
            ("domainId".to_string(), dsl::DslValue::String(FEM2D_INTERACTION_DOMAIN.to_string())),
            ("targets".to_string(), dsl::DslValue::String(targets_json(targets))),
            ("merge".to_string(), dsl::DslValue::String(merge.to_string())),
            ("method".to_string(), dsl::DslValue::String("pick".to_string())),
        ]),
    )
}

/// 🕹️ Asks the shell to redispatch `interactionHover` on the `pointer` channel — the framework dedupes
/// an unchanged batch, so a pointer-move handler stays stateless.
pub fn interaction_hover_effect<G: AsRef<str>, I: AsRef<str>>(targets: &[(G, I)]) -> Effect {
    request_interaction_action(
        semio_framework::INTERACTION_HOVER_ACTION_ID,
        dsl::DslValue::object([
            ("domainId".to_string(), dsl::DslValue::String(FEM2D_INTERACTION_DOMAIN.to_string())),
            ("channel".to_string(), dsl::DslValue::String(FEM2D_POINTER_CHANNEL.to_string())),
            ("targets".to_string(), dsl::DslValue::String(targets_json(targets))),
        ]),
    )
}
//#endregion 🔖️Effects

//#region 🔖️WindowCamera
/// 🪟️ The kind id of the window one command is addressed to.
pub fn fem2d_addressed_window_kind<'a>(view: &'a ViewModel, fault: &str) -> Result<&'a str, Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| Fault::from(format!("{fault}.window-required")))?;
    view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| Fault::from(format!("{fault}.window-stale")))
}

/// 🎥️ The camera the addressed Canvas2d window is currently drawn with — both window kinds keep it
/// in their own persisted window config, so a pick must read the one it was produced under.
pub fn fem2d_addressed_camera(cfg: &ConfigView<'_, NoConfig>, view: &ViewModel, fault: &str) -> Result<Viewport2d, Fault> {
    match fem2d_addressed_window_kind(view, fault)? {
        model_window::WINDOW_KIND_ID => Ok(model_window::config::current(cfg).camera),
        results_window::WINDOW_KIND_ID => Ok(results_window::config::current(cfg).camera),
        _ => Err(Fault::from(format!("{fault}.window-kind"))),
    }
}
//#endregion 🔖️WindowCamera

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
