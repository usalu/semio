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
use crate::{standards::v1::subsets::any::schema::registers::AdjacencyKind, standards::v1::subsets::any::schema::registers::AnalysisKind, standards::v1::subsets::any::schema::registers::AuditEvent, standards::v1::subsets::any::schema::registers::RelationshipKind, standards::v1::subsets::any::schema::registers::ReportKind, standards::v1::subsets::any::schema::registers::RiskLevel, standards::v1::subsets::any::schema::registers::SearchFilter, standards::v1::subsets::any::schema::registers::SeparationKind, standards::v1::subsets::any::schema::registers::ValidationStatus};
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::snapshot::ARCHITECT_PROGRAM_SCHEMA;
#[allow(unused_imports)]
use std::{collections::HashMap, collections::HashSet};
use crate::standards::v1::subsets::any::schema::inferences::*;
use crate::standards::v1::subsets::any::schema::snapshot::sample_plugin;

#[semio_framework_async_macros::async_test]
async fn sample_plugin_matrix_has_one_cell() {
    let program = sample_plugin();
    let matrix = adjacency_matrix(&program);
    assert_eq!(matrix.element_ids.len(), 2);
    let populated: usize = matrix.cells.iter().flat_map(|row| row.iter()).filter(|cell| cell.is_some()).count();
    assert_eq!(populated, 1);
}

#[semio_framework_async_macros::async_test]
async fn detects_distance_min_max_violation() {
    let mut program = sample_plugin();
    program.adjacencies[0].distance_min_m = Some(10.0);
    program.adjacencies[0].distance_max_m = Some(5.0);
    let conflicts = detect_adjacency_conflicts(&program);
    assert!(conflicts.iter().any(|c| c.message.contains("distance_min")));
}
