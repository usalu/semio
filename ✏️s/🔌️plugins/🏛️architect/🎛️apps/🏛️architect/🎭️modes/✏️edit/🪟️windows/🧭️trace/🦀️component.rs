//! 🧭️ Architect trace window — trace chain, impact and audit trail for the selected entity.

use crate::apps::architect::chrome::{tree_item, tree_node, tree_section};
use crate::apps::architect::config::ArchitectConfig;
use crate::artifacts::program::engine::trace::{audit_trail, trace_chain, trace_impact, TraceChain};
use crate::artifacts::program::{EntityId, Program};
use semio_framework_plugin::{ui_text, Label, LocalizedLabel, SurfaceKind, UiNode, UiTreeItemNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const ARCHITECT_WINDOW_TRACE: &str = "architect-trace";
pub const ARCHITECT_BODY_TRACE: &str = "architect.trace";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🏛️ Stitched into the app manifest by `crate::apps::architect::create_architect_app`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: ARCHITECT_WINDOW_TRACE.into(),
        label: LocalizedLabel::native("Trace", "Nachverfolgung"),
        body_key: ARCHITECT_BODY_TRACE.into(),
        surface_kind: SurfaceKind::TextEditor,
        icon_id: "file-code".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        document_projection_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(program: &Program, cfg: &ArchitectConfig) -> UiNode {
    if cfg.selected_ids.is_empty() {
        return ui_text(Label::data("Select an entity to inspect trace chains and impact."));
    }
    let root = EntityId(cfg.selected_ids[0].clone());
    let mut scratch = program.clone();
    let chain: TraceChain = trace_chain(&mut scratch, &root);
    let impact = trace_impact(&mut scratch, &root);
    let trail = audit_trail(program, Some(&root));
    let chain_items: Vec<UiTreeItemNode> = chain.links.iter().enumerate().map(|(index, link)| tree_item(format!("architect-trace.chain.{index}"), format!("{:?}: {} → {}", link.kind, link.from_id, link.to_id))).collect();
    let impact_items: Vec<UiTreeItemNode> = impact.upstream_ids.iter().enumerate().map(|(index, id)| tree_item(format!("architect-trace.impact.{index}"), id.to_string())).collect();
    let audit_items: Vec<UiTreeItemNode> = trail.events.iter().take(12).enumerate().map(|(index, event)| tree_item(format!("architect-trace.audit.{index}"), format!("{:?} @ {} — {}", event.action, event.timestamp, event.header.name))).collect();
    tree_node(
        vec![
            tree_section("architect-trace.chain", Some(format!("Trace Chain ({})", chain.links.len())), if chain_items.is_empty() { vec![tree_item("architect-trace.chain.empty", "(no links)")] } else { chain_items }),
            tree_section("architect-trace.impact", Some(format!("Impact ({})", impact.upstream_ids.len())), if impact_items.is_empty() { vec![tree_item("architect-trace.impact.empty", "(no upstream)")] } else { impact_items }),
            tree_section("architect-trace.audit", Some(format!("Audit Trail ({})", trail.events.len())), if audit_items.is_empty() { vec![tree_item("architect-trace.audit.empty", "(no events)")] } else { audit_items }),
        ],
        None,
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::sample_plugin;

    #[test]
    fn definition_declares_the_text_editor_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, ARCHITECT_BODY_TRACE);
        assert!(matches!(definition.surface_kind, SurfaceKind::TextEditor));
    }

    #[test]
    fn no_selection_renders_the_placeholder() {
        let json = serde_json::to_string(&render(&sample_plugin(), &ArchitectConfig::default())).expect("json");
        assert!(json.contains("Select an entity"));
    }

    #[test]
    fn a_selected_entity_renders_the_three_trace_sections() {
        let program = sample_plugin();
        let cfg = ArchitectConfig { selected_ids: vec![program.elements[0].header.id.to_string()], ..ArchitectConfig::default() };
        let json = serde_json::to_string(&render(&program, &cfg)).expect("json");
        assert!(json.contains("architect-trace.chain"));
        assert!(json.contains("architect-trace.impact"));
        assert!(json.contains("architect-trace.audit"));
    }
}
//#endregion 🧪️Tests
