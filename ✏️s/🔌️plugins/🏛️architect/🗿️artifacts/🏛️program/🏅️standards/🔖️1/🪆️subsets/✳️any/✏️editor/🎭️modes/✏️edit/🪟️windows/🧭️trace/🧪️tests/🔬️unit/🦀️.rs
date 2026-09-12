use super::*;
use crate::kernel::{EntityHeader, TextField};
use crate::registers::{AuditAction, AuditEvent};
use crate::sample_plugin;
use crate::EntityId;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_text_editor_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, ARCHITECT_BODY_TRACE);
    assert!(matches!(definition.surface_kind, SurfaceKind::TextEditor));
}

#[semio_framework_async_macros::async_test]
async fn no_events_renders_the_empty_placeholder_row() {
    let json = crate::editor::architect::unit_tests::context::project_render(render(&sample_plugin()));
    assert!(json.contains("architect-trace.audit"));
    assert!(json.contains("architect-trace.audit.empty"));
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the audit feed is document-wide
/// now (no `InteractionView` in `render` to scope it to a selected entity) — every event shows,
/// not just ones touching one subject.
#[semio_framework_async_macros::async_test]
async fn renders_every_document_wide_audit_event() {
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
    let json = crate::editor::architect::unit_tests::context::project_render(render(&program));
    assert!(json.contains("architect-trace.audit.0"));
    assert!(!json.contains("architect-trace.audit.empty"));
}
