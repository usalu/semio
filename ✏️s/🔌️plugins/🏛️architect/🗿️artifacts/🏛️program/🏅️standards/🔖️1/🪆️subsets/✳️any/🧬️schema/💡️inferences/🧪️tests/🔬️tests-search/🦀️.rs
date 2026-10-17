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
async fn search_finds_reception_element() {
    let hits = search_plugin(&sample_plugin(), &SearchQuery { keywords: vec!["Reception".into()], ..Default::default() }, None, None);
    assert!(hits.iter().any(|h| h.name == "Reception"));
}

#[semio_framework_async_macros::async_test]
async fn search_history_records_query() {
    let mut history = Vec::new();
    search_plugin(&sample_plugin(), &SearchQuery { keywords: vec!["Waiting".into()], ..Default::default() }, None, Some(&mut history));
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].keywords, vec!["Waiting".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn entity_kind_filter_limits_registers() {
    let hits = search_plugin(&sample_plugin(), &SearchQuery { entity_kinds: vec!["elements".into()], ..Default::default() }, None, None);
    assert!(hits.iter().all(|h| h.register == "elements"));
}
