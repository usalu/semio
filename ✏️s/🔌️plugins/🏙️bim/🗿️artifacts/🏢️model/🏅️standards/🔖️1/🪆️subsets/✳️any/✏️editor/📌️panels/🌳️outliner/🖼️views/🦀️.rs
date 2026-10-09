//! 🖼️ The view browser of the outliner: below each building one `Views` node whose children are the views of the building, grouped by kind (plans, ceiling plans, sections, elevations, 3D views) and ordered
//! by the level of their storey and their name. A row is the id of its view, so a click selects the view in the same `elements` domain as every other element, and the properties panel edits it.

use super::{entity_item, item, leaf};
use crate::editor::bim::entities::kind_of;
use crate::editor::bim::terminology::BimLabels;
use crate::{ModelSnapshot, ViewKind};
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::UiAssemblyResult;

/// 🗂️ The browser group of a kind: plans, ceiling plans, sections, elevations, cameras.
fn group_of(kind: ViewKind) -> usize {
    match kind {
        ViewKind::Plan => 0,
        ViewKind::CeilingPlan => 1,
        ViewKind::Section => 2,
        ViewKind::Elevation => 3,
        ViewKind::Orthographic | ViewKind::Perspective => 4,
    }
}

fn group_label(labels: &BimLabels, group: usize) -> &str {
    match group {
        0 => labels.group_view_plans.as_str(),
        1 => labels.group_view_ceiling_plans.as_str(),
        2 => labels.group_view_sections.as_str(),
        3 => labels.group_view_elevations.as_str(),
        _ => labels.group_view_cameras.as_str(),
    }
}

/// 🖼️ The views of a building in browser order, as `(group, ids)` for the groups that are not empty.
pub fn groups(snapshot: &ModelSnapshot, building: &str) -> Vec<(usize, Vec<String>)> {
    let mut rows: Vec<(usize, i32, &String, &String)> = snapshot.views.iter().filter(|(_, view)| view.building == building).map(|(id, view)| (group_of(view.kind), view.storey.as_ref().and_then(|storey| snapshot.storeys.get(storey)).map_or(0, |storey| storey.level), &view.name, id)).collect();
    rows.sort();
    (0..5).filter_map(|group| Some((group, rows.iter().filter(|row| row.0 == group).map(|row| row.3.clone()).collect::<Vec<_>>())).filter(|(_, ids)| !ids.is_empty())).collect()
}

fn scale_text(snapshot: &ModelSnapshot, id: &str) -> Option<String> {
    snapshot.views.get(id).map(|view| format!("1:{}", view.scale))
}

/// 🖼️ The `Views` node of one building, none when it has no view.
pub fn views_row(windows: &TreeWindows<'_>, snapshot: &ModelSnapshot, labels: &BimLabels, building: &str) -> Option<UiAssemblyResult<BuiltNode>> {
    let groups = groups(snapshot, building);
    if groups.is_empty() {
        return None;
    }
    let row = kind_of("view")?;
    let node_id = format!("{building}::views");
    let count = groups.iter().map(|(_, ids)| ids.len()).sum::<usize>().to_string();
    Some(item(&node_id, labels.group_views.as_str(), row.icon, None, Some(&count)).and_then(|builder| {
        semio_framework_plugin::tree_window_indexed_item(windows, builder, &node_id, true, groups.len(), |index| {
            let (group, ids) = &groups[index];
            let group_id = format!("{building}::views::{group}");
            let builder = item(&group_id, group_label(labels, *group), row.icon, None, Some(&ids.len().to_string()))?;
            semio_framework_plugin::tree_window_indexed_item(windows, builder, &group_id, true, ids.len(), |position| leaf(entity_item(snapshot, row, &ids[position], scale_text(snapshot, &ids[position]).as_deref())?))
        })
    }))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
