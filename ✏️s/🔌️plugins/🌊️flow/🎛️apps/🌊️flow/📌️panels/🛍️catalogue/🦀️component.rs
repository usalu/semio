//! 🛍️ Flow play app panel — the catalogue: draggable widget/operator palette plus the extension sections.

use crate::apps::flow::commands::extension::FLOW_AUTOMATIONS;
use crate::apps::flow::config::FlowConfig;
use crate::apps::flow::flow_action;
use crate::apps::flow::terminology::{flow_extension_action_title_label, flow_extension_label, FlowPlayLabels};
use crate::artifacts::flow::engine::{flow_widget_descriptor, flow_widget_drag_json, host_from_fixture};
use crate::artifacts::flow::FlowFixture;
use flow_core::FlowEvalSession;
use semio_framework_plugin::{
    tree_item_with_action, tree_item_with_action_draggable, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiPresence, UiTreeItemNode, UiTreeSectionNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const FLOW_PLAY_BODY_CATALOGUE: &str = "flow.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(FLOW_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(fixture: &FlowFixture, config: &FlowConfig, session: &FlowEvalSession, labels: &FlowPlayLabels) -> UiNode {
    let host = host_from_fixture(fixture, config, session);
    let sections: Vec<Value> = host.catalogue_json().ok().and_then(|raw| serde_json::from_str(&raw).ok()).unwrap_or_default();
    let tree_sections: Vec<UiTreeSectionNode> = sections
        .iter()
        .filter_map(|section| {
            let id = section.get("id")?.as_str()?.to_string();
            let title = section.get("title").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
            let items: Vec<UiTreeItemNode> = section
                .get("items")
                .and_then(|value| value.as_array())
                .map(|entries| {
                    entries
                        .iter()
                        .filter_map(|entry| {
                            let kind = entry.get("kind")?.as_str()?;
                            let label = entry.get("name").or_else(|| entry.get("abbreviation")).and_then(|value| value.as_str()).unwrap_or(kind);
                            let descriptor = if kind == "neuron" { flow_widget_descriptor("neuron", entry.get("neuronKind").and_then(|value| value.as_str())) } else { flow_widget_descriptor(kind, None) };
                            let action = flow_action("addWidget", Some(descriptor.clone()));
                            Some(tree_item_with_action_draggable(format!("flow-play-catalogue.{id}.{kind}.{label}"), Label::data(label), Some(kind.to_string()), action, &flow_widget_drag_json(&descriptor)))
                        })
                        .collect()
                })
                .unwrap_or_default();
            Some(UiTreeSectionNode { presence: UiPresence::default(), id: format!("flow-play-catalogue.{id}"), label: Some(Label::data(title)), default_open: Some(true), items })
        })
        .collect();
    let tree_sections = if tree_sections.is_empty() { catalogue_tree_sections_fallback(labels) } else { tree_sections };
    let mut builder = PanelTreeBuilder::new("flow-play-catalogue");
    for section in tree_sections.into_iter().chain(flow_extensions_tree_sections(config, labels)) {
        builder = builder.section(section.id, section.label, section.default_open.unwrap_or(false), section.items);
    }
    builder.selected(vec![]).build()
}

/// 🧩️ Installed/enabled extension palette plus actions surfaced by active extensions.
fn flow_extensions_tree_sections(config: &FlowConfig, labels: &FlowPlayLabels) -> Vec<UiTreeSectionNode> {
    let extension_enabled = config.automation_enabled();
    let installed: Vec<UiTreeItemNode> = FLOW_AUTOMATIONS
        .iter()
        .map(|(id, name, _, _, _)| {
            let enabled = extension_enabled.get(*id).copied().unwrap_or(false);
            tree_item_with_action(
                format!("flow-play-extensions.{id}"),
                flow_extension_label(id, name, labels),
                Some(if enabled { "enabled".into() } else { "disabled".into() }),
                flow_action("toggleExtension", Some(json!({ "id": id, "enabled": !enabled }))),
            )
        })
        .collect();
    let actions: Vec<UiTreeItemNode> = FLOW_AUTOMATIONS
        .iter()
        .filter(|(id, ..)| extension_enabled.get(*id).copied().unwrap_or(false))
        .map(|(_, _, action_id, title, _)| {
            tree_item_with_action(format!("flow-play-extensions.action.{action_id}"), flow_extension_action_title_label(action_id, title, labels), Some((*action_id).into()), flow_action("runExtensionAction", Some(json!({ "actionId": action_id }))))
        })
        .collect();
    let mut sections = vec![UiTreeSectionNode { presence: UiPresence::default(), id: "flow-play-extensions.installed".into(), label: Some(labels.extensions.into()), default_open: Some(false), items: installed }];
    if !actions.is_empty() {
        sections.push(UiTreeSectionNode { presence: UiPresence::default(), id: "flow-play-extensions.actions".into(), label: Some(labels.extension_actions.into()), default_open: Some(false), items: actions });
    }
    sections
}

/// 🛟️ Used when the host catalogue is empty (a fresh/offline session) — a minimal hand-authored palette.
fn catalogue_tree_sections_fallback(labels: &FlowPlayLabels) -> Vec<UiTreeSectionNode> {
    let sources = [("inputSlider", labels.catalogue_slider), ("inputNote", labels.catalogue_note)];
    let components = [("math.add", labels.catalogue_add), ("logic.and", labels.catalogue_and), ("text.concat", labels.catalogue_concat)];
    let sinks = [("outputPreview", labels.catalogue_preview), ("outputExport", labels.catalogue_export)];
    vec![
        UiTreeSectionNode {
            presence: UiPresence::default(),
            id: "flow-play-catalogue.sources".into(),
            label: Some(labels.sources.into()),
            default_open: Some(true),
            items: sources
                .iter()
                .map(|(kind, label)| {
                    let descriptor = flow_widget_descriptor(kind, None);
                    tree_item_with_action_draggable(format!("flow-play-catalogue.source.{kind}"), *label, Some((*kind).into()), flow_action("addWidget", Some(descriptor.clone())), &flow_widget_drag_json(&descriptor))
                })
                .collect(),
        },
        UiTreeSectionNode {
            presence: UiPresence::default(),
            id: "flow-play-catalogue.components".into(),
            label: Some(labels.components.into()),
            default_open: Some(true),
            items: components
                .iter()
                .map(|(kind, label)| {
                    let descriptor = flow_widget_descriptor("neuron", Some(kind));
                    tree_item_with_action_draggable(format!("flow-play-catalogue.component.{kind}"), *label, Some((*kind).into()), flow_action("addWidget", Some(descriptor.clone())), &flow_widget_drag_json(&descriptor))
                })
                .collect(),
        },
        UiTreeSectionNode {
            presence: UiPresence::default(),
            id: "flow-play-catalogue.sinks".into(),
            label: Some(labels.sinks.into()),
            default_open: Some(false),
            items: sinks
                .iter()
                .map(|(kind, label)| {
                    let descriptor = flow_widget_descriptor(kind, None);
                    tree_item_with_action_draggable(format!("flow-play-catalogue.sink.{kind}"), *label, Some((*kind).into()), flow_action("addWidget", Some(descriptor.clone())), &flow_widget_drag_json(&descriptor))
                })
                .collect(),
        },
    ]
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::flow::testkit::{flow_app, render as render_body};
    use crate::artifacts::flow::engine::FLOW_WIDGET_DRAG_MIME;

    #[test]
    fn catalogue_lists_module_operators() {
        let mut app = flow_app();
        let json = render_body(&mut app, FLOW_PLAY_BODY_CATALOGUE);
        assert!(json.contains("flow-play-catalogue.math"), "expected math module section: {json}");
        assert!(json.contains("math.add"), "expected math.add operator: {json}");
    }

    #[test]
    fn catalogue_items_export_flow_widget_drag_payload() {
        let mut app = flow_app();
        let json = render_body(&mut app, FLOW_PLAY_BODY_CATALOGUE);
        assert!(json.contains(FLOW_WIDGET_DRAG_MIME), "missing drag mime: {json}");
        assert!(json.contains(r#""draggable":true"#) || json.contains(r#""draggable": true"#));
    }

    #[test]
    fn every_built_in_extension_is_listed_in_the_installed_section() {
        let mut app = flow_app();
        let json = render_body(&mut app, FLOW_PLAY_BODY_CATALOGUE);
        for (id, ..) in FLOW_AUTOMATIONS {
            assert!(json.contains(&format!("flow-play-extensions.{id}")), "extension {id} missing: {json}");
        }
    }
}
//#endregion 🧪️Tests
