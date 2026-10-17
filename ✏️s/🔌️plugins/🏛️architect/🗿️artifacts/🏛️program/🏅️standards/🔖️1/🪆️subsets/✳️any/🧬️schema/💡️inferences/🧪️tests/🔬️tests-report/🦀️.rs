use crate::standards::v1::subsets::any::schema::registers::ReportKind;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;
#[allow(unused_imports)]
use framework_schema::ArtifactSchema;
#[allow(unused_imports)]
use protocol::Inference;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::inferences::topology::compute_topology;
#[allow(unused_imports)]
use crate::{standards::v1::subsets::any::schema::kernel::DiagnosticSeverity, standards::v1::subsets::any::schema::kernel::EntityHeader, standards::v1::subsets::any::schema::kernel::EntityId, standards::v1::subsets::any::schema::kernel::LifecycleStatus, standards::v1::subsets::any::schema::kernel::Priority, standards::v1::subsets::any::schema::kernel::ProgramDiagnostic};
#[allow(unused_imports)]
use crate::{standards::v1::subsets::any::schema::registers::AdjacencyKind, standards::v1::subsets::any::schema::registers::AnalysisKind, standards::v1::subsets::any::schema::registers::AuditEvent, standards::v1::subsets::any::schema::registers::RelationshipKind, standards::v1::subsets::any::schema::registers::RiskLevel, standards::v1::subsets::any::schema::registers::SearchFilter, standards::v1::subsets::any::schema::registers::SeparationKind, standards::v1::subsets::any::schema::registers::ValidationStatus};
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::snapshot::ARCHITECT_PROGRAM_SCHEMA;
#[allow(unused_imports)]
use std::{collections::HashMap, collections::HashSet};
use crate::standards::v1::subsets::any::schema::inferences::*;
use crate::standards::v1::subsets::any::schema::snapshot::sample_plugin;

#[semio_framework_async_macros::async_test]
async fn executive_summary_includes_counts() {
    let report = build_report(&sample_plugin(), ReportKind::ExecutiveSummary);
    assert_eq!(report.kind, ReportKind::ExecutiveSummary);
    assert!(!report.sections.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn requirements_matrix_has_grid_rows() {
    let report = build_report(&sample_plugin(), ReportKind::RequirementsMatrix);
    assert!(!report.sections[0].bullets.is_empty());
    assert!(report.sections[0].bullets[0].contains('\t'));
}
