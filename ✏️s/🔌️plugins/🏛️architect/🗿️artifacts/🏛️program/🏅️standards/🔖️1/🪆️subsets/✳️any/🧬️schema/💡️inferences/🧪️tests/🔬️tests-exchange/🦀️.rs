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
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::csv::v_rfc4180::any::export_relationships_csv;
use crate::standards::v1::subsets::any::schema::inferences::*;
use crate::standards::v1::subsets::any::io::text::inferences::{export_json,import_json};
use crate::standards::v1::subsets::any::schema::snapshot::sample_plugin;

#[semio_framework_async_macros::async_test]
async fn json_round_trip() {
    let program = sample_plugin();
    let json = export_json(&program).expect("export");
    let imported = import_json(&json).expect("import");
    assert_eq!(imported.elements.len(), program.elements.len());
    assert_eq!(imported.adjacencies.len(), program.adjacencies.len());
}

#[semio_framework_async_macros::async_test]
async fn relationships_csv_round_trips_via_stdio_codec() {
    let program = sample_plugin();
    let csv = export_relationships_csv(&program).expect("relationships csv export");
    let snapshot = semio_s_artifact_stdio_csv::standards::v_rfc4180::subsets::any::io::text::snapshot::decode_csv_with(&csv, true);
    assert_eq!(snapshot.records.len(), program.relationships.len() + 1, "header + one row per relationship");
}
