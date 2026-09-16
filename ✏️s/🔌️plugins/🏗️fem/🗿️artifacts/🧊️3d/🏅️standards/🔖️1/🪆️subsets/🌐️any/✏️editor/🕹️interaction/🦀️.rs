//! 🕹️ Fem3d play app — the framework-owned interaction domain: its definition, the selection/hover
//! snapshot every render reads, the entity-kind and model-point lookups the panels and the camera
//! share, the `World3d` selection record the host paints and arms its gumball from, and the
//! `interactionSelect` request a panel row emits. The viewport itself never hit-tests here: the
//! `World3dHost` raycasts the scene's instances and dispatches `interactionSelect`/`interactionHover`
//! with the instance's own id and `interactionGranularityId`, which is why every scene instance is
//! keyed by the RAW entity id (`🎬️scene`).

use crate::{element_id, load_id, Fem3dSnapshot, FemLoad};
use semio_framework::kernel::Effect;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, LocalizedLabel, MergeMode, SelectionMethod, SelectionMode, SelectionSpec, ViewModel};

//#region 🔖️Constants
pub use crate::standards::v1::subsets::any::scene::{
    fem3d_element_endpoints, fem3d_node_point, fem3d_solid_centroid, fem3d_solid_top_point, FEM3D_GRANULARITY_COMBINATION, FEM3D_GRANULARITY_ELEMENT, FEM3D_GRANULARITY_LOAD, FEM3D_GRANULARITY_LOAD_CASE, FEM3D_GRANULARITY_MATERIAL, FEM3D_GRANULARITY_NODE,
    FEM3D_GRANULARITY_SECTION, FEM3D_GRANULARITY_SOLID, FEM3D_GRANULARITY_SUPPORT, FEM3D_INTERACTION_DOMAIN,
};
pub const FEM3D_POINTER_CHANNEL: &str = "pointer";
/// 🧰️ The one window utility fem3d declares: the host's transform gumball over the live selection.
pub const FEM3D_UTILITY_TRANSFORM: &str = "transform";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🕹️ The one fem3d interaction domain: every document entity kind is a granularity and a target id
/// is the raw entity id, so a tree row, a viewport pick and the inspector all agree on one vocabulary.
pub fn fem3d_interaction_definition() -> InteractionDefinition {
    let granularity = |id: &str, en: &str, de: &str, icon: &str| GranularityDefinition { id: id.into(), label: LocalizedLabel::native(en, de), icon_id: icon.into() };
    InteractionDefinition {
        id: FEM3D_INTERACTION_DOMAIN.into(),
        label: LocalizedLabel::native("Structure", "Tragwerk"),
        granularities: vec![
            granularity(FEM3D_GRANULARITY_NODE, "Node", "Knoten", "circle-dot"),
            granularity(FEM3D_GRANULARITY_ELEMENT, "Element", "Element", "minus"),
            granularity(FEM3D_GRANULARITY_SOLID, "Solid", "Volumenkörper", "box"),
            granularity(FEM3D_GRANULARITY_SUPPORT, "Support", "Lager", "anchor"),
            granularity(FEM3D_GRANULARITY_LOAD, "Load", "Last", "arrow-down"),
            granularity(FEM3D_GRANULARITY_MATERIAL, "Material", "Material", "layers"),
            granularity(FEM3D_GRANULARITY_SECTION, "Section", "Querschnitt", "ruler"),
            granularity(FEM3D_GRANULARITY_LOAD_CASE, "Load Case", "Lastfall", "list"),
            granularity(FEM3D_GRANULARITY_COMBINATION, "Combination", "Kombination", "link"),
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
pub struct Fem3dInteractionSnapshot {
    pub selected_ids: Vec<String>,
    pub hovered_ids: Vec<String>,
}

impl Fem3dInteractionSnapshot {
    pub fn from_interaction(interaction: &InteractionView<'_>) -> Self {
        Self { selected_ids: interaction.selection(FEM3D_INTERACTION_DOMAIN).ids.clone(), hovered_ids: interaction.hover(FEM3D_INTERACTION_DOMAIN, FEM3D_POINTER_CHANNEL).ids.clone() }
    }

    /// 🧪️ A snapshot selecting exactly `ids`, for tests and for a render outside any request context.
    pub fn selecting(ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self { selected_ids: ids.into_iter().map(Into::into).collect(), hovered_ids: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️Utility
/// 🧰️ The utility the addressed window has armed — the flat `active_utility_id` first, then the
/// per-window map the shell keeps for split layouts; nothing armed means the host's own select lane.
pub fn fem3d_active_utility(view: &ViewModel) -> &str {
    view.active_utility_id.as_deref().or_else(|| view.window_id.as_ref().and_then(|id| view.active_utility_by_window_id.get(id).map(String::as_str))).unwrap_or("")
}

/// 🧭️ Whether the addressed window has the transform gumball armed.
pub fn fem3d_transform_armed(view: &ViewModel) -> bool {
    fem3d_active_utility(view) == FEM3D_UTILITY_TRANSFORM
}
//#endregion 🔖️Utility

//#region 🔖️Geometry
/// 🪪️ The granularity one raw document id belongs to, searched in the same precedence the viewport
/// picks with — the artifact tree and the inspector resolve a selected id through this one lookup.
pub fn fem3d_entity_kind(doc: &Fem3dSnapshot, id: &str) -> Option<&'static str> {
    if doc.nodes.iter().any(|node| node.id == id) {
        return Some(FEM3D_GRANULARITY_NODE);
    }
    if doc.elements.iter().any(|element| element_id(element) == id) {
        return Some(FEM3D_GRANULARITY_ELEMENT);
    }
    if doc.solids.iter().any(|solid| solid.id == id) {
        return Some(FEM3D_GRANULARITY_SOLID);
    }
    if doc.supports.iter().any(|support| support.id == id) {
        return Some(FEM3D_GRANULARITY_SUPPORT);
    }
    if fem3d_load_owner(doc, id).is_some() {
        return Some(FEM3D_GRANULARITY_LOAD);
    }
    if doc.materials.iter().any(|material| material.id == id) {
        return Some(FEM3D_GRANULARITY_MATERIAL);
    }
    if doc.sections.iter().any(|section| section.id == id) {
        return Some(FEM3D_GRANULARITY_SECTION);
    }
    if doc.load_cases.iter().any(|case| case.id == id) {
        return Some(FEM3D_GRANULARITY_LOAD_CASE);
    }
    if doc.combinations.iter().any(|combination| combination.id == id) {
        return Some(FEM3D_GRANULARITY_COMBINATION);
    }
    None
}

/// 🏋️ The load case owning one load id, plus the load itself — a load target carries only the raw
/// load id, so every consumer (inspector patch, focus, highlight) resolves its case through here.
pub fn fem3d_load_owner<'a>(doc: &'a Fem3dSnapshot, load: &str) -> Option<(&'a str, &'a FemLoad)> {
    doc.load_cases.iter().find_map(|case| case.loads.iter().find(|candidate| load_id(candidate) == load).map(|found| (case.id.as_str(), found)))
}

/// 🎯️ The world point one entity is framed on — a node's position, a member's midpoint, a solid's
/// centroid, the carrying node for a support or a nodal load, the member midpoint for a member
/// load, the top of the solid for a surface load.
pub fn fem3d_entity_point(doc: &Fem3dSnapshot, id: &str) -> Option<[f64; 3]> {
    if let Some(point) = fem3d_node_point(doc, id) {
        return Some(point);
    }
    if let Some(element) = doc.elements.iter().find(|element| element_id(element) == id) {
        let (start, end) = fem3d_element_endpoints(element);
        let (a, b) = (fem3d_node_point(doc, start)?, fem3d_node_point(doc, end)?);
        return Some([(a[0] + b[0]) * 0.5, (a[1] + b[1]) * 0.5, (a[2] + b[2]) * 0.5]);
    }
    if let Some(solid) = doc.solids.iter().find(|solid| solid.id == id) {
        return fem3d_solid_centroid(solid);
    }
    if let Some(support) = doc.supports.iter().find(|support| support.id == id) {
        return fem3d_node_point(doc, &support.node_id);
    }
    if let Some((_, load)) = fem3d_load_owner(doc, id) {
        return match load {
            FemLoad::Nodal { node_id, .. } => fem3d_node_point(doc, node_id),
            FemLoad::MemberUdl { element_id: target, .. } => fem3d_entity_point(doc, target),
            FemLoad::Area { solid_id, .. } => doc.solids.iter().find(|solid| &solid.id == solid_id).and_then(fem3d_solid_top_point),
        };
    }
    None
}
//#endregion 🔖️Geometry

//#region 🔖️Effects
fn targets_json<G: AsRef<str>, I: AsRef<str>>(targets: &[(G, I)]) -> String {
    let items: Vec<dsl::json::Value> = targets.iter().map(|(granularity, id)| dsl::json!({ "granularity": granularity.as_ref(), "id": id.as_ref() })).collect();
    dsl::json::to_string(&dsl::json::Value::Array(items))
}

/// 🕹️ Asks the shell to redispatch `interactionSelect` for these targets — selection is
/// framework-owned state, so an app never writes it, it only reports WHICH ids to select.
pub fn interaction_select_effect<G: AsRef<str>, I: AsRef<str>>(targets: &[(G, I)], merge: &str, method: &str) -> Effect {
    Effect::ReplayShellCommand {
        action_id: semio_framework::INTERACTION_SELECT_ACTION_ID.into(),
        args: Some(dsl::DslValue::object([
            ("domainId".to_string(), dsl::DslValue::String(FEM3D_INTERACTION_DOMAIN.to_string())),
            ("targets".to_string(), dsl::DslValue::String(targets_json(targets))),
            ("merge".to_string(), dsl::DslValue::String(merge.to_string())),
            ("method".to_string(), dsl::DslValue::String(method.to_string())),
        ])),
    }
}
//#endregion 🔖️Effects

#[path = "🧭️gumball/🦀️.rs"]
pub mod gumball;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
