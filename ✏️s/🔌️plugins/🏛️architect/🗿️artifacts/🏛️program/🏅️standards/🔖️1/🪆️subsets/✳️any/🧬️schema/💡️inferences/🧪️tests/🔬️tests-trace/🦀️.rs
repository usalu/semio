use super::*;
use crate::sample_plugin;

#[semio_framework_async_macros::async_test]
async fn audit_trail_sorted_newest_first() {
    let mut program = sample_plugin();
    program.audit_events.push(AuditEvent {
        header: EntityHeader::new(EntityId::new_serial("audit", "older"), "older"),
        action: crate::registers::AuditAction::Created,
        actor_id: None,
        subject_id: program.elements[0].header.id.clone(),
        subject_kind: "element".into(),
        timestamp: "2020-01-01T00:00:00Z".into(),
        details: crate::kernel::TextField::plain("old"),
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
    program.audit_events.push(AuditEvent {
        header: EntityHeader::new(EntityId::new_serial("audit", "newer"), "newer"),
        action: crate::registers::AuditAction::Updated,
        actor_id: None,
        subject_id: program.elements[0].header.id.clone(),
        subject_kind: "element".into(),
        timestamp: "2025-01-01T00:00:00Z".into(),
        details: crate::kernel::TextField::plain("new"),
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
    let trail = audit_trail(&program, None);
    assert!(trail.events[0].timestamp > trail.events[1].timestamp);
}
