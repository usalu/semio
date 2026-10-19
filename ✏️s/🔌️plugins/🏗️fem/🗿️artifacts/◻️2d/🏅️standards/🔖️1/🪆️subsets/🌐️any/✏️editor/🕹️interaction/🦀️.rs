//! 🌐️ snapshot every render reads, screen-space hit-testing of document entities and the
//! 🕹️ Fem2d play app — the framework-owned interaction domain: its definition, the selection/hover
//! `interactionSelect`/`interactionHover` request effects a pick emits. Slice A fills this in.

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::element_id;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemLoad;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use semio_framework_os_kernel::Viewport2d;
use semio_framework::kernel::Effect;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::Fault;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::ViewModel;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemElement;

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
/// uses the shared typed scoped identity, so tree rows, viewport picks and inspection agree.
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
        hover: HoverSpec { enabled: true, transitive: false, channels: vec![FEM2D_POINTER_CHANNEL.into()], broadcast: true },
        selection: SelectionSpec {
            modes: vec![SelectionMode::Multiple, SelectionMode::Single],
            methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle, SelectionMethod::Lasso],
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

/// 🎯️ One pick: the entity granularity and its canonical scoped identity.
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
pub fn fem2d_load_glyph(doc: &Fem2dSnapshot, load: &FemLoad) -> Option<((f64, f64), (f64, f64))> {
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
pub(crate) fn fem2d_region_centroid(region: &semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemRegion) -> Option<(f64, f64)> {
    if region.outline.is_empty() {
        return None;
    }
    let sum = region.outline.iter().fold([0.0, 0.0], |sum, point| [sum[0] + point[0], sum[1] + point[1]]);
    let count = region.outline.len() as f64;
    Some((sum[0] / count, sum[1] / count))
}

/// 🪪️ The granularity one raw document id belongs to, searched in the same precedence the viewport
/// picks with — the artifact tree and the inspector resolve a selected id through this one lookup.
pub fn fem2d_entity_kind(doc: &Fem2dSnapshot, target: &str) -> Option<&'static str> {
    let segments = protocol::wire::decode_scoped_identity(target).ok()?;
    match segments.as_slice() {
        [FEM2D_GRANULARITY_NODE, id] if doc.nodes.iter().any(|row| row.id == *id) => Some(FEM2D_GRANULARITY_NODE),
        [FEM2D_GRANULARITY_ELEMENT, id] if doc.elements.iter().any(|row| element_id(row) == *id) => Some(FEM2D_GRANULARITY_ELEMENT),
        [FEM2D_GRANULARITY_REGION, id] if doc.regions.iter().any(|row| row.id == *id) => Some(FEM2D_GRANULARITY_REGION),
        [FEM2D_GRANULARITY_SUPPORT, id] if doc.supports.iter().any(|row| row.id == *id) => Some(FEM2D_GRANULARITY_SUPPORT),
        [FEM2D_GRANULARITY_MATERIAL, id] if doc.materials.iter().any(|row| row.id == *id) => Some(FEM2D_GRANULARITY_MATERIAL),
        [FEM2D_GRANULARITY_SECTION, id] if doc.sections.iter().any(|row| row.id == *id) => Some(FEM2D_GRANULARITY_SECTION),
        [FEM2D_GRANULARITY_LOAD_CASE, id] if doc.load_cases.iter().any(|row| row.id == *id) => Some(FEM2D_GRANULARITY_LOAD_CASE),
        [FEM2D_GRANULARITY_COMBINATION, id] if doc.combinations.iter().any(|row| row.id == *id) => Some(FEM2D_GRANULARITY_COMBINATION),
        [FEM2D_GRANULARITY_LOAD, _, _] if fem2d_load_owner(doc, target).is_some() => Some(FEM2D_GRANULARITY_LOAD),
        _ => None,
    }
}

/// 🪪️ Canonical interaction identity for one typed top-level entity.
pub fn fem2d_entity_target_id(kind: &str, id: &str) -> Result<String, protocol::wire::ScopedIdentityError> {
    protocol::wire::encode_scoped_identity(&[kind, id])
}

/// 🧾️ Semantic id at the boundary of a canonical interaction address.
pub fn fem2d_entity_raw_id(target: &str) -> Option<&str> {
    let segments = protocol::wire::decode_scoped_identity(target).ok()?;
    match segments.as_slice() { [_, id] | [FEM2D_GRANULARITY_LOAD, _, id] => Some(*id), _ => None }
}

/// 🪪️ A load's interaction identity includes its owning case; byte lengths preserve arbitrary Unicode and delimiters.
pub fn fem2d_load_target_id(case: &str, load: &str) -> Result<String, protocol::wire::ScopedIdentityError> {
    protocol::wire::encode_scoped_identity(&[FEM2D_GRANULARITY_LOAD, case, load])
}

/// 🏋️ Resolves only an exact case-owned load interaction identity.
pub fn fem2d_load_owner<'a>(doc: &'a Fem2dSnapshot, target: &str) -> Option<(&'a str, &'a FemLoad)> {
    let segments = protocol::wire::decode_scoped_identity(target).ok()?;
    let ["load", case_id, load_id] = segments.as_slice() else { return None };
    let case = doc.load_cases.iter().find(|case| case.id == *case_id)?;
    case.loads.iter().find(|load| semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::load_id(load) == *load_id).map(|load| (case.id.as_str(), load))
}

/// 🎯️ The model-space point one entity is framed on — a node's position, a member's midpoint, a
/// region's centroid, a support's node, a load's anchor.
pub fn fem2d_entity_model_point(doc: &Fem2dSnapshot, target: &str) -> Option<(f64, f64)> {
    let kind = fem2d_entity_kind(doc, target)?;
    let id = fem2d_entity_raw_id(target)?;
    if kind == FEM2D_GRANULARITY_NODE {
        return find_node_2d(&doc.nodes, id).map(|node| (node.x, node.y));
    }
    if kind == FEM2D_GRANULARITY_ELEMENT {
        let element = doc.elements.iter().find(|element| element_id(element) == id)?;
        let (start, end) = fem2d_element_endpoints(element);
        let (a, b) = (find_node_2d(&doc.nodes, start)?, find_node_2d(&doc.nodes, end)?);
        return Some(((a.x + b.x) * 0.5, (a.y + b.y) * 0.5));
    }
    if kind == FEM2D_GRANULARITY_SUPPORT {
        let support = doc.supports.iter().find(|support| support.id == id)?;
        let node = find_node_2d(&doc.nodes, &support.node_id)?;
        return Some((node.x, node.y));
    }
    if kind == FEM2D_GRANULARITY_REGION {
        let region = doc.regions.iter().find(|region| region.id == id)?;
        return fem2d_region_centroid(region);
    }
    if let Some((_, load)) = fem2d_load_owner(doc, target) {
        return match load {
            FemLoad::Nodal { node_id, .. } => find_node_2d(&doc.nodes, node_id).map(|node| (node.x, node.y)),
            FemLoad::MemberUdl { element_id: target, .. } => fem2d_entity_model_point(doc, &fem2d_entity_target_id(FEM2D_GRANULARITY_ELEMENT, target).ok()?),
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
pub fn fem2d_hit_test(doc: &Fem2dSnapshot, camera: &Viewport2d, x: f64, y: f64, width: f64, height: f64) -> Result<Option<Fem2dPick>, protocol::wire::ScopedIdentityError> {
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
            keep(score, (FEM2D_GRANULARITY_NODE, fem2d_entity_target_id(FEM2D_GRANULARITY_NODE, &node.id)?), &mut best);
        }
    }
    if let Some((_, pick)) = best.take() {
        return Ok(Some(pick));
    }

    for support in &doc.supports {
        let Some(node) = find_node_2d(&doc.nodes, &support.node_id) else { continue };
        let score = distance(pointer, at((node.x, node.y)));
        if score <= FEM2D_SUPPORT_PICK_RADIUS_PX {
            keep(score, (FEM2D_GRANULARITY_SUPPORT, fem2d_entity_target_id(FEM2D_GRANULARITY_SUPPORT, &support.id)?), &mut best);
        }
    }
    if let Some((_, pick)) = best.take() {
        return Ok(Some(pick));
    }

    for case in &doc.load_cases {
        for load in &case.loads {
            let Some((start, end)) = fem2d_load_glyph(doc, load) else { continue };
            let score = distance_to_segment(pointer, layer_at(start), layer_at(end));
            if score <= FEM2D_LOAD_PICK_RADIUS_PX {
                keep(score, (FEM2D_GRANULARITY_LOAD, fem2d_load_target_id(&case.id, semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::load_id(load))?), &mut best);
            }
        }
    }
    if let Some((_, pick)) = best.take() {
        return Ok(Some(pick));
    }

    for element in &doc.elements {
        let (start, end) = fem2d_element_endpoints(element);
        let (Some(a), Some(b)) = (find_node_2d(&doc.nodes, start), find_node_2d(&doc.nodes, end)) else { continue };
        let score = distance_to_segment(pointer, at((a.x, a.y)), at((b.x, b.y)));
        if score <= FEM2D_ELEMENT_PICK_RADIUS_PX {
            keep(score, (FEM2D_GRANULARITY_ELEMENT, fem2d_entity_target_id(FEM2D_GRANULARITY_ELEMENT, element_id(element))?), &mut best);
        }
    }
    if let Some((_, pick)) = best.take() {
        return Ok(Some(pick));
    }

    let model = fem2d_canvas_to_model(camera, x, y, width, height);
    doc.regions
        .iter()
        .find(|region| point_in_polygon(model, &region.outline) && !region.holes.iter().any(|hole| point_in_polygon(model, hole)))
        .map(|region| fem2d_entity_target_id(FEM2D_GRANULARITY_REGION, &region.id).map(|id| (FEM2D_GRANULARITY_REGION, id))).transpose()
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
    let items: Vec<semio_framework_pack_json::Value> = targets.iter().map(|(granularity, id)| semio_framework_pack_json::json!({ "granularity": granularity.as_ref(), "id": id.as_ref() })).collect();
    semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Array(items))
}

fn request_interaction_action(action_id: &str, args: semio_framework_value::DslValue) -> Effect {
    Effect::ReplayShellCommand { action_id: action_id.into(), args: Some(args) }
}

/// 🕹️ Asks the shell to redispatch `interactionSelect` for this pick — selection is framework-owned
/// state, so an app never writes it, it only reports WHICH ids the pointer hit.
pub fn interaction_select_effect<G: AsRef<str>, I: AsRef<str>>(targets: &[(G, I)], merge: &str, method: &str) -> Effect {
    request_interaction_action(
        semio_framework::INTERACTION_SELECT_ACTION_ID,
        semio_framework_value::DslValue::object([
            ("domainId".to_string(), semio_framework_value::DslValue::String(FEM2D_INTERACTION_DOMAIN.to_string())),
            ("targets".to_string(), semio_framework_value::DslValue::String(targets_json(targets))),
            ("merge".to_string(), semio_framework_value::DslValue::String(merge.to_string())),
            ("method".to_string(), semio_framework_value::DslValue::String(method.to_string())),
        ]),
    )
}

#[path = "🖱️canvas-gesture/🦀️.rs"]
pub mod canvas_gesture;

#[path = "🧭️gumball/🦀️.rs"]
pub mod gumball;

/// 🕹️ Asks the shell to redispatch `interactionHover` on the `pointer` channel — the framework dedupes
/// an unchanged batch, so a pointer-move handler stays stateless.
pub fn interaction_hover_effect<G: AsRef<str>, I: AsRef<str>>(targets: &[(G, I)]) -> Effect {
    request_interaction_action(
        semio_framework::INTERACTION_HOVER_ACTION_ID,
        semio_framework_value::DslValue::object([
            ("domainId".to_string(), semio_framework_value::DslValue::String(FEM2D_INTERACTION_DOMAIN.to_string())),
            ("channel".to_string(), semio_framework_value::DslValue::String(FEM2D_POINTER_CHANNEL.to_string())),
            ("targets".to_string(), semio_framework_value::DslValue::String(targets_json(targets))),
        ]),
    )
}
//#endregion 🔖️Effects


//#region 🔖️CanvasGeometry
/// 📐️ Model-meters -> screen-pixels scale for the 2D canvas (a 6m span shouldn't render as 6px wide).
pub const SCALE_2D: f64 = 20.0;
/// 📐️ Screen-space origin offset so a structure anchored at (0,0) isn't drawn at the canvas corner.
pub const ORIGIN_2D: f64 = 40.0;

pub fn screen_2d(x: f64, y: f64) -> (f64, f64) {
    (x * SCALE_2D + ORIGIN_2D, -y * SCALE_2D + ORIGIN_2D)
}

pub fn find_node_2d<'a>(nodes: &'a [semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemNode], id: &str) -> Option<&'a semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemNode> {
    nodes.iter().find(|n| n.id == id)
}

pub fn fem2d_element_endpoints(element: &semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemElement) -> (&str, &str) {
    match element {
        FemElement::Bar { start, end, .. } | FemElement::Beam { start, end, .. } => (start.as_str(), end.as_str()),
    }
}
//#endregion 🔖️CanvasGeometry

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
