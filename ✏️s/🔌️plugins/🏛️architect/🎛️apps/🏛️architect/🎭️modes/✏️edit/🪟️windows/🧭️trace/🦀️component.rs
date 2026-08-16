//! 🧭️ Architect trace window — the document-wide audit trail.

use crate::apps::architect::chrome::{tree_item, tree_node, tree_section};
use crate::artifacts::program::standards::v1::subsets::any::schema::inferences::audit_trail;
use crate::artifacts::program::ProgramSnapshot;
use semio_framework_plugin::{LocalizedLabel, SurfaceKind, UiNode, UiTreeItemNode, WindowKindDefinition, WindowOptions};

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
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: this window degrades to a
        // document-wide audit feed (see `render`'s doc comment) — its rows are informational, not
        // selectable.
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `ArtifactApp::render` carries no
/// `InteractionView` (matches `note`'s/`gis2d`'s inspection panel precedent), so this window can no
/// longer scope trace chain/impact to a selected entity — both sections needed a root id and are
/// gone with it; the audit trail degrades to the document-wide feed (`audit_trail(program, None)`)
/// instead of one scoped to a selection.
pub fn render(program: &ProgramSnapshot) -> UiNode {
    let trail = audit_trail(program, None);
    let audit_items: Vec<UiTreeItemNode> = trail.events.iter().take(12).enumerate().map(|(index, event)| tree_item(format!("architect-trace.audit.{index}"), format!("{:?} @ {} — {}", event.action, event.timestamp, event.header.name))).collect();
    tree_node(vec![tree_section("architect-trace.audit", Some(format!("Audit Trail ({})", trail.events.len())), if audit_items.is_empty() { vec![tree_item("architect-trace.audit.empty", "(no events)")] } else { audit_items })])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::kernel::{EntityHeader, TextField};
    use crate::artifacts::program::registers::{AuditAction, AuditEvent};
    use crate::artifacts::program::sample_plugin;
    use crate::artifacts::program::EntityId;

    #[test]
    fn definition_declares_the_text_editor_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, ARCHITECT_BODY_TRACE);
        assert!(matches!(definition.surface_kind, SurfaceKind::TextEditor));
    }

    #[test]
    fn no_events_renders_the_empty_placeholder_row() {
        let json = serde_json::to_string(&render(&sample_plugin())).expect("json");
        assert!(json.contains("architect-trace.audit"));
        assert!(json.contains("architect-trace.audit.empty"));
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the audit feed is document-wide
    /// now (no `InteractionView` in `render` to scope it to a selected entity) — every event shows,
    /// not just ones touching one subject.
    #[test]
    fn renders_every_document_wide_audit_event() {
        let mut program = sample_plugin();
        program.audit_events.push(AuditEvent {
            header: EntityHeader::new(EntityId::new_serial("audit", "created"), "created"),
            action: AuditAction::Created,
            actor_id: None,
            subject_id: program.elements[0].header.id.clone(),
            subject_kind: "element".into(),
            timestamp: "2026-08-14T00:00:00Z".into(),
            details: TextField::plain("created"),
            before_state: None,
            after_state: None,
            ip_address: None,
            client: None,
            session_id: None,
            change_record_id: None,
            trace_link: None,
            success: true,
            error_message: None,
            correlation_id: None,
            compliance_tags: Vec::new(),
            retention_until: None,
        });
        let json = serde_json::to_string(&render(&program)).expect("json");
        assert!(json.contains("architect-trace.audit.0"));
        assert!(!json.contains("architect-trace.audit.empty"));
    }
}
//#endregion 🧪️Tests
