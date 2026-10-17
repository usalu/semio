use crate::{standards::v1::subsets::any::schema::kernel::EntityHeader, standards::v1::subsets::any::schema::kernel::EntityId};
use crate::standards::v1::subsets::any::schema::registers::AuditEvent;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;
#[allow(unused_imports)]
use framework_schema::ArtifactSchema;
#[allow(unused_imports)]
use protocol::Inference;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::inferences::topology::compute_topology;
#[allow(unused_imports)]
use crate::{standards::v1::subsets::any::schema::kernel::DiagnosticSeverity, standards::v1::subsets::any::schema::kernel::LifecycleStatus, standards::v1::subsets::any::schema::kernel::Priority, standards::v1::subsets::any::schema::kernel::ProgramDiagnostic};
#[allow(unused_imports)]
use crate::{standards::v1::subsets::any::schema::registers::AdjacencyKind, standards::v1::subsets::any::schema::registers::AnalysisKind, standards::v1::subsets::any::schema::registers::RelationshipKind, standards::v1::subsets::any::schema::registers::ReportKind, standards::v1::subsets::any::schema::registers::RiskLevel, standards::v1::subsets::any::schema::registers::SearchFilter, standards::v1::subsets::any::schema::registers::SeparationKind, standards::v1::subsets::any::schema::registers::ValidationStatus};
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::snapshot::ARCHITECT_PROGRAM_SCHEMA;
#[allow(unused_imports)]
use std::{collections::HashMap, collections::HashSet};
use crate::standards::v1::subsets::any::schema::inferences::*;
use crate::standards::v1::subsets::any::schema::snapshot::sample_plugin;

#[semio_framework_async_macros::async_test]
async fn audit_trail_sorted_newest_first() {
    let mut program = sample_plugin();
    program.audit_events.push(AuditEvent {
        header: EntityHeader::new(EntityId::new_serial("audit", "older"), "older"),
        action: crate::standards::v1::subsets::any::schema::registers::AuditAction::Created,
        actor_id: None,
        subject_id: program.elements[0].header.id.clone(),
        subject_kind: "element".into(),
        timestamp: "2020-01-01T00:00:00Z".into(),
        details: crate::standards::v1::subsets::any::schema::kernel::TextField::plain("old"),
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
        action: crate::standards::v1::subsets::any::schema::registers::AuditAction::Updated,
        actor_id: None,
        subject_id: program.elements[0].header.id.clone(),
        subject_kind: "element".into(),
        timestamp: "2025-01-01T00:00:00Z".into(),
        details: crate::standards::v1::subsets::any::schema::kernel::TextField::plain("new"),
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
