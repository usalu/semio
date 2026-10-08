//! 🕹️ BIM interaction: two framework-owned domains. `elements` carries every placed entity (site to space) as one hierarchy, so the outliner tree, the plan, the section and
//! the 3D window select and hover the same ids; `library` carries the materials and the seven type families as a flat list.

use crate::editor::bim::entities::{ordered_storeys, ENTITIES};
use crate::editor::bim::terminology::BimLabels;
use crate::ModelSnapshot;
use semio_framework_plugin::DomainTopology;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionTopology;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::TopologyNode;
use semio_framework_ui_locale::LocalizedLabel;

//#region 🔖️Constants
pub const BIM_ELEMENT_DOMAIN: &str = "elements";
pub const BIM_LIBRARY_DOMAIN: &str = "library";
//#endregion 🔖️Constants

fn localized(label: crate::editor::bim::entities::LabelOf) -> LocalizedLabel {
    LocalizedLabel::native(label(&BimLabels::NATIVE_EN).as_str(), label(&BimLabels::NATIVE_DE).as_str())
}

fn selection() -> SelectionSpec {
    SelectionSpec {
        modes: vec![SelectionMode::Multiple, SelectionMode::Single],
        methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle],
        merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
        transitive: false,
        broadcast: true,
    }
}

//#region 🔖️Definitions
/// 🕹️ The two interaction domains of the manifest, one granularity per entity kind.
pub fn definitions() -> Vec<InteractionDefinition> {
    let granularities = |library: bool| -> Vec<GranularityDefinition> { ENTITIES.iter().filter(|row| row.library == library).map(|row| GranularityDefinition { id: row.kind.into(), label: localized(row.label), icon_id: row.icon.into() }).collect() };
    vec![
        InteractionDefinition {
            id: BIM_ELEMENT_DOMAIN.into(),
            label: LocalizedLabel::native("Elements", "Bauteile"),
            granularities: granularities(false),
            hierarchy: HierarchyProvider::Topology,
            hover: HoverSpec::default(),
            selection: selection(),
        },
        InteractionDefinition {
            id: BIM_LIBRARY_DOMAIN.into(),
            label: LocalizedLabel::native("Library", "Bibliothek"),
            granularities: granularities(true),
            hierarchy: HierarchyProvider::Flat,
            hover: HoverSpec::default(),
            selection: selection(),
        },
    ]
}
//#endregion 🔖️Definitions

//#region 🔖️Topology
/// 🌳️ The `elements` domain in tree order: every site, its buildings, their storeys by level, and each storey's elements with the openings under their host.
pub fn element_topology(snapshot: &ModelSnapshot) -> DomainTopology {
    let node = |kind: &str, id: &str, parent: Option<&str>| TopologyNode { id: id.to_string(), granularity: kind.into(), parent: parent.map(str::to_string) };
    let mut ordered = Vec::new();
    for site in snapshot.sites.keys() {
        ordered.push(node("site", site, None));
        for building in snapshot.buildings.iter().filter(|(_, row)| &row.site == site).map(|(id, _)| id) {
            ordered.push(node("building", building, Some(site)));
            for grid in snapshot.grids.iter().filter(|(_, row)| &row.building == building).map(|(id, _)| id) {
                ordered.push(node("grid", grid, Some(building)));
            }
            for storey in ordered_storeys(snapshot, building) {
                ordered.push(node("storey", &storey, Some(building)));
                for placed in ENTITIES.iter().filter(|row| !row.library && !matches!(row.kind, "site" | "building" | "storey" | "grid" | "opening")) {
                    for id in (placed.ids)(snapshot).into_iter().filter(|id| (placed.parent)(snapshot, id).as_deref() == Some(storey.as_str())) {
                        ordered.push(node(placed.kind, &id, Some(&storey)));
                        for opening in snapshot.openings.iter().filter(|(_, row)| row.host == id).map(|(opening, _)| opening) {
                            ordered.push(node("opening", opening, Some(&id)));
                        }
                    }
                }
            }
        }
    }
    DomainTopology { ordered }
}

/// 📚️ The `library` domain: materials then each type family, flat.
pub fn library_topology(snapshot: &ModelSnapshot) -> DomainTopology {
    let ordered = ENTITIES.iter().filter(|row| row.library).flat_map(|row| (row.ids)(snapshot).into_iter().map(|id| TopologyNode { id, granularity: row.kind.into(), parent: None })).collect();
    DomainTopology { ordered }
}

/// 🕹️ Both domains for the framework's pick revalidation: without them every pick is dropped.
pub fn topology(snapshot: &ModelSnapshot) -> InteractionTopology {
    InteractionTopology { domains: std::collections::BTreeMap::from([(BIM_ELEMENT_DOMAIN.into(), element_topology(snapshot)), (BIM_LIBRARY_DOMAIN.into(), library_topology(snapshot))]) }
}
//#endregion 🔖️Topology

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
