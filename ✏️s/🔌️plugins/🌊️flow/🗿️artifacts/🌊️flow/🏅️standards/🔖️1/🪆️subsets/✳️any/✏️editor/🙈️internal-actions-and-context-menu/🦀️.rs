//! 🙈️ Internal actions and context menu: the palette-hidden internal action, the context menu items and the content child. See [`super`].

use super::*;

//#region 🔖️Constants
/// 🙈️ An action that exists for dispatch but never appears in the command palette.
pub(super) fn flow_internal_action(id: &str, label: LocalizedLabel, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(id, label, kind) }
}
//#endregion 🔖️Constants

//#region 🔖️ContextMenu
/// 🖱️ On-demand flow node-graph context menu from surface hit-test and selection snapshot.
pub(super) fn flow_context_menu_items(registry: &AppActionRegistry, snapshot: &FlowSnapshot, config: &FlowMainWindowConfig, labels: &FlowPlayLabels, view_state: &semio_framework_plugin::ViewModel, surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>) -> Vec<ContextMenuItemSpec> {
    use semio_framework_plugin::{node_graph_delete_selection_spec, Menu, NodeGraphDeleteDispatch};

    let hits = surface.map_or(&[][..], |target| target.hits.as_slice());
    let groups = surface.map_or(&[][..], |target| target.selection.as_slice());
    let nodes: Vec<String> = groups.iter().filter(|group| group.domain == "node").flat_map(|group| group.ids.iter().cloned()).collect();
    let edges: Vec<String> = groups.iter().filter(|group| group.domain == "edge").flat_map(|group| group.ids.iter().cloned()).collect();
    let has_selection = !nodes.is_empty() || !edges.is_empty();
    let all_preview_off = !nodes.is_empty() && nodes.iter().all(|id| config.preview_off_node_ids.contains(id));
    let live = snapshot.to_host_snapshot();
    let is_image = nodes.len() == 1
        && live.widgets.iter().any(|widget| match widget {
            Widget::InputImage { id, .. } => id == &nodes[0],
            _ => false,
        });
    live.retire_cold();
    let primary = hits.first();
    let hit_node = primary.filter(|hit| hit.domain == "node").map(|hit| hit.id.as_str());

    {
        let mut menu = Menu::of(registry, view_state);
        if hits.is_empty() {
            menu = menu
                .item(ContextMenuItemSpec { id: "add-node".into(), label: Some(labels.add_node.into()), icon: Some("plus".into()), action: Some("openSpotlight".into()), ..Default::default() })
                .action("selectAll")
                .group("transform", |m| m.action("reorganize"));
        }
        if let Some(node_id) = hit_node {
            menu = menu.group("actions", |m| {
                m.item(ContextMenuItemSpec {
                    id: "duplicate-widget".into(),
                    label: Some(labels.duplicate_widget.into()),
                    icon: Some("copy".into()),
                    action: Some("duplicateWidget".into()),
                    args: Some(semio_framework_plugin::dsl_value!({ "widgetId": node_id })),
                    ..Default::default()
                })
            });
            if is_image {
                menu = menu.group("actions", |m| {
                    m.item(ContextMenuItemSpec {
                        id: "replace-image".into(),
                        label: Some(labels.replace_image.into()),
                        icon: Some("image".into()),
                        action: Some("replaceImage".into()),
                        args: Some(semio_framework_plugin::dsl_value!({ "id": node_id })),
                        ..Default::default()
                    })
                });
            }
        }
        if has_selection {
            menu = menu.action("focusSelection").action("clearSelection").group("view", |m| {
                m.item(ContextMenuItemSpec {
                    id: "toggle-preview".into(),
                    label: Some(if all_preview_off { labels.show_preview.into() } else { labels.hide_preview.into() }),
                    icon: Some(if all_preview_off { "eye".into() } else { "eye-off".into() }),
                    checked: Some(!all_preview_off),
                    action: Some("setPreviewOff".into()),
                    args: Some(semio_framework_plugin::dsl_value!({ "ids": nodes, "value": !all_preview_off })),
                    ..Default::default()
                })
            });
        }
        menu.item(node_graph_delete_selection_spec(labels.delete_selection.as_str(), view_state, &nodes, &edges, NodeGraphDeleteDispatch::Direct)).build()
    }
}
//#endregion 🔖️ContextMenu

/// 🧮️ The composed content child this document names, read in place for a bounded scan — `None` unless it is
/// composed as `s.stdio.semio@v1/flow`. Every retained scan reads it, never the parent's working-scene owner
/// (see [`crate::flow_composed_snapshot`]).
pub(super) fn flow_content_child<'a>(snapshot: &FlowSnapshot, children: &'a semio_framework_plugin::ChildContentView) -> Option<store::os_store::snapshot_read::SnapshotReadRef<'a, SemioFlowSnapshot>> {
    let child_id = &snapshot.content.child_id;
    let dialect = children.dialect("content", child_id)?;
    if dialect.artifact_kind != "s.stdio.semio" || dialect.standard != "v1" || dialect.subset != "flow" {
        return None;
    }
    children.typed_read::<SemioFlowSnapshot>("content", child_id).ok()
}
