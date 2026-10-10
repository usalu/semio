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
/// 🗂️ The granularity of a row of the entry table of a classification system in the `library` domain; its id is `system:code`.
pub const CLASSIFICATION_ENTRY: &str = "classification-entry";
//#endregion 🔖️Constants

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
    let granularities = |library: bool| -> Vec<GranularityDefinition> {
        let mut found: Vec<GranularityDefinition> = ENTITIES.iter().filter(|row| row.library == library).map(|row| GranularityDefinition { id: row.kind.into(), label: BimLabels::localized(row.label), icon_id: row.icon.into() }).collect();
        if library {
            found.push(GranularityDefinition { id: CLASSIFICATION_ENTRY.into(), label: BimLabels::localized(|labels| labels.kind_classification_entry), icon_id: "list-tree".into() });
        }
        found
    };
    vec![
        InteractionDefinition {
            id: BIM_ELEMENT_DOMAIN.into(),
            label: BimLabels::localized(|labels| labels.domain_elements),
            granularities: granularities(false),
            hierarchy: HierarchyProvider::Topology,
            hover: HoverSpec::default(),
            selection: selection(),
        },
        InteractionDefinition {
            id: BIM_LIBRARY_DOMAIN.into(),
            label: BimLabels::localized(|labels| labels.domain_library),
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
                for placed in ENTITIES.iter().filter(|row| !row.library && !matches!(row.kind, "site" | "building" | "storey" | "grid" | "opening" | "curtain-panel-override" | "component-override")) {
                    for id in (placed.ids)(snapshot).into_iter().filter(|id| (placed.parent)(snapshot, id).as_deref() == Some(storey.as_str())) {
                        ordered.push(node(placed.kind, &id, Some(&storey)));
                        for opening in snapshot.openings.iter().filter(|(_, row)| row.host == id).map(|(opening, _)| opening) {
                            ordered.push(node("opening", opening, Some(&id)));
                        }
                        for sweep in snapshot.wall_sweeps.iter().filter(|(_, row)| row.host == id).map(|(sweep, _)| sweep) {
                            ordered.push(node("wall-sweep", sweep, Some(&id)));
                        }
                        for panel in snapshot.curtain_panel_overrides.iter().filter(|(_, row)| row.curtain == id).map(|(panel, _)| panel) {
                            ordered.push(node("curtain-panel-override", panel, Some(&id)));
                        }
                        for overridden in snapshot.component_overrides.iter().filter(|(_, row)| row.component == id).map(|(key, _)| key) {
                            ordered.push(node("component-override", overridden, Some(&id)));
                        }
                    }
                }
            }
        }
    }
    DomainTopology { ordered }
}

/// 📚️ The `library` domain: materials then each type family, flat, then the rows of the entry table of each classification system (the nodes a search of the classification browser selects).
pub fn library_topology(snapshot: &ModelSnapshot) -> DomainTopology {
    let mut ordered: Vec<TopologyNode> = ENTITIES.iter().filter(|row| row.library).flat_map(|row| (row.ids)(snapshot).into_iter().map(|id| TopologyNode { id, granularity: row.kind.into(), parent: None })).collect();
    ordered.extend(snapshot.classification_systems.iter().flat_map(|(system, row)| row.entries.iter().map(move |entry| TopologyNode { id: format!("{system}:{}", entry.code), granularity: CLASSIFICATION_ENTRY.into(), parent: None })));
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
