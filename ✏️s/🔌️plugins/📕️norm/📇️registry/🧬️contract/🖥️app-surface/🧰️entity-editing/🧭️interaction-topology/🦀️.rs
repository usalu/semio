//! 🗺️ Interaction topology of the norm app surface: every id the interaction domain of an artifact can hold, so the framework's `selectAll` needs no payload (any scene size) and its stale-selection pruning keeps exactly what still exists. Mounted in the inspection crate next to the host routes.
//!
//! 🔗️ Seam: `ArtifactEditor::interaction_topology`; ids: [`entity_address`](crate::viewport3d::picking::entity_address), [`row_address`](crate::row_address) and the `check` granularity of [`GRANULARITY_CHECK`](crate::viewport3d::picking::GRANULARITY_CHECK).

use crate::*;
use crate::viewport3d::picking::{entity_address, granularity, GRANULARITY_CHECK};
use semio_framework_plugin::{DomainTopology, InteractionTopology, TopologyNode};
use semio_framework_value::DslValue;
use semio_s_plugin_norm_structural_scene::StructuralScene;
use std::collections::{BTreeMap, HashSet};

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

/// 🧾️ The interaction address of check `id` (granularity `check`).
pub fn check_address(id: &str) -> Option<String> {
    crate::protocol::wire::encode_scoped_identity(&[GRANULARITY_CHECK, id]).ok()
}

/// 🗺️ The nodes of one domain in selection order (entities and rows narrowed to the ids `visible` admits when the outliner filter is active; checks always stay): the scene entities of the viewport (granularity of their kind, the nodes Select all picks), the rows of the outliner collections (granularity of the collection) and the check rows (`check`); all roots, each id once.
pub fn domain_topology(tree: &DslValue, scene: &StructuralScene, report: &CheckReport, visible: Option<&[String]>) -> DomainTopology {
    let admitted = |id: &str| visible.is_none_or(|ids| ids.iter().any(|visible| visible == id));
    let mut seen: HashSet<String> = HashSet::new();
    let entities = scene.entities.iter().filter(|scene_entity| admitted(&scene_entity.entity.id)).filter_map(|scene_entity| Some((entity_address(scene_entity.entity.kind, &scene_entity.entity.id)?, granularity(scene_entity.entity.kind).to_owned())));
    let rows = match tree {
        DslValue::Object(fields) => fields.iter().filter_map(|(name, value)| value.as_array().map(|items| (name, items))).filter(|(_, items)| is_entity_collection(items)).flat_map(|(name, items)| collection_rows(name, items, "")).filter(|row| admitted(&row.id)).filter_map(|row| Some((row_address(&row)?, row.path.split('[').next().unwrap_or_default().to_owned()))).collect::<Vec<_>>(),
        _ => Vec::new(),
    };
    let checks = report.checks.iter().filter_map(|check| Some((check_address(&check.id)?, GRANULARITY_CHECK.to_owned())));
    let ordered = entities.chain(rows).chain(checks).filter(|(id, _)| seen.insert(id.clone())).map(|(id, granularity)| TopologyNode { id, granularity, parent: None }).collect();
    DomainTopology { ordered }
}

/// 🗺️ The topology of an artifact: its one interaction `domain` over the document `tree`, the lowered `scene` and the settled `report`.
pub fn norm_interaction_topology(domain: &str, tree: &DslValue, scene: &StructuralScene, report: &CheckReport) -> InteractionTopology {
    InteractionTopology { domains: BTreeMap::from([(domain.to_owned(), domain_topology(tree, scene, report, visible_rows(domain).as_deref()))]) }
}
