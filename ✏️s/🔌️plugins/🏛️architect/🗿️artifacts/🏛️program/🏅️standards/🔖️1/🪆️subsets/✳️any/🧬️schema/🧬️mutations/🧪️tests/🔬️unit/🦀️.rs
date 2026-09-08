
use super::*;
use crate::kernel::*;
use crate::registers::*;
use crate::{empty_plugin, sample_plugin};
use protocol::{Mutation, MutationDiff, SemanticMutation};

fn round_trip(snapshot: &ProgramSnapshot, operation: &ProgramMutation) -> ProgramSnapshot {
    let forward = operation.diff(snapshot).diff().apply(snapshot).expect("valid mutation diff");
    let mut backward = operation.inverse(snapshot);
    backward.reverse();
    let mut restored = forward.clone();
    for undo in &backward {
        restored = undo.diff(&restored).diff().apply(&restored).expect("valid mutation diff");
    }
    assert_eq!(&restored, snapshot, "inverse (reversed) must exactly restore the pre-operation fixture");
    forward
}

//#region 👥stakeholders
#[semio_framework_async_macros::async_test]
async fn stakeholders_create_rename_replace_delete_round_trip() {
    let snapshot = sample_plugin();
    let new_id = EntityId::new_serial("stakeholder", "stakeholder");
    let mut new_stakeholder = snapshot.stakeholders[0].clone();
    new_stakeholder.header.id = new_id.clone();
    new_stakeholder.header.name = "New Stakeholder".into();

    let create = ProgramMutation::CreateStakeholder(super::super::create_stakeholder::CreateStakeholder { stakeholder: new_stakeholder });
    let with_new = round_trip(&snapshot, &create);
    assert_eq!(with_new.stakeholders.len(), snapshot.stakeholders.len() + 1);

    let rename = ProgramMutation::RenameStakeholder(super::super::rename_stakeholder::RenameStakeholder { id: new_id.clone(), new_name: "Renamed".into() });
    let renamed = round_trip(&with_new, &rename);
    assert_eq!(renamed.stakeholders.iter().find(|s| s.header.id == new_id).unwrap().header.name, "Renamed");

    let mut replacement = renamed.stakeholders.iter().find(|s| s.header.id == new_id).unwrap().clone();
    replacement.role = "Sponsor".into();
    let replace = ProgramMutation::ReplaceStakeholder(super::super::replace_stakeholder::ReplaceStakeholder { stakeholder: replacement });
    let replaced = round_trip(&renamed, &replace);
    assert_eq!(replaced.stakeholders.iter().find(|s| s.header.id == new_id).unwrap().role, "Sponsor");

    let delete = ProgramMutation::DeleteStakeholder(super::super::delete_stakeholder::DeleteStakeholder { id: new_id });
    let deleted = round_trip(&replaced, &delete);
    assert_eq!(deleted.stakeholders.len(), snapshot.stakeholders.len());
}

#[semio_framework_async_macros::async_test]
async fn delete_stakeholder_of_a_missing_id_has_an_empty_inverse() {
    let snapshot = sample_plugin();
    let delete = ProgramMutation::DeleteStakeholder(super::super::delete_stakeholder::DeleteStakeholder { id: EntityId("nope".into()) });
    assert!(delete.inverse(&snapshot).is_empty(), "deleting an absent id has nothing to undo");
}
//#endregion 👥stakeholders

//#region 🧱elements
#[semio_framework_async_macros::async_test]
async fn elements_create_rename_replace_delete_round_trip() {
    let snapshot = sample_plugin();
    let new_id = EntityId::new_serial("element", "element");
    let mut new_element = snapshot.elements[0].clone();
    new_element.header.id = new_id.clone();
    new_element.header.name = "Storage".into();

    let create = ProgramMutation::CreateProgramElement(super::super::create_program_element::CreateProgramElement { program_element: new_element });
    let with_new = round_trip(&snapshot, &create);
    assert_eq!(with_new.elements.len(), snapshot.elements.len() + 1);

    let rename = ProgramMutation::RenameProgramElement(super::super::rename_program_element::RenameProgramElement { id: new_id.clone(), new_name: "Storage Room".into() });
    let renamed = round_trip(&with_new, &rename);
    assert_eq!(renamed.elements.iter().find(|e| e.header.id == new_id).unwrap().header.name, "Storage Room");

    let mut replacement = renamed.elements.iter().find(|e| e.header.id == new_id).unwrap().clone();
    replacement.code = "STO".into();
    let replace = ProgramMutation::ReplaceProgramElement(super::super::replace_program_element::ReplaceProgramElement { program_element: replacement });
    let replaced = round_trip(&renamed, &replace);
    assert_eq!(replaced.elements.iter().find(|e| e.header.id == new_id).unwrap().code, "STO");

    let delete = ProgramMutation::DeleteProgramElement(super::super::delete_program_element::DeleteProgramElement { id: new_id });
    let deleted = round_trip(&replaced, &delete);
    assert_eq!(deleted.elements.len(), snapshot.elements.len());
}
//#endregion 🧱elements

//#region 🏷️📁🏛️meta-project-governance
#[semio_framework_async_macros::async_test]
async fn update_meta_rename_and_replace_round_trip() {
    let snapshot = empty_plugin();
    let rename = ProgramMutation::RenameMeta(super::super::rename_meta::RenameMeta { new_title: "Clinic".into() });
    let renamed = round_trip(&snapshot, &rename);
    assert_eq!(renamed.meta.title, "Clinic");

    let mut new_meta = renamed.meta.clone();
    new_meta.industry_sector = "healthcare".into();
    let replace = ProgramMutation::ReplaceMeta(super::super::replace_meta::ReplaceMeta { new_meta });
    let replaced = round_trip(&renamed, &replace);
    assert_eq!(replaced.meta.industry_sector, "healthcare");
}

#[semio_framework_async_macros::async_test]
async fn update_project_rename_and_replace_round_trip() {
    let snapshot = empty_plugin();
    let rename = ProgramMutation::RenameProject(super::super::rename_project::RenameProject { new_code: "CLN-001".into() });
    let renamed = round_trip(&snapshot, &rename);
    assert_eq!(renamed.project.code, "CLN-001");

    let mut new_project = renamed.project.clone();
    new_project.client_name = "Sample Health".into();
    let replace = ProgramMutation::ReplaceProject(super::super::replace_project::ReplaceProject { new_project });
    let replaced = round_trip(&renamed, &replace);
    assert_eq!(replaced.project.client_name, "Sample Health");
}

#[semio_framework_async_macros::async_test]
async fn update_governance_rename_and_replace_round_trip() {
    let snapshot = empty_plugin();
    let rename = ProgramMutation::RenameGovernance(super::super::rename_governance::RenameGovernance { new_framework: "ISO 41001".into() });
    let renamed = round_trip(&snapshot, &rename);
    assert_eq!(renamed.governance.framework, "ISO 41001");

    let mut new_governance = renamed.governance.clone();
    new_governance.risk_appetite = Some("Low".into());
    let replace = ProgramMutation::ReplaceGovernance(super::super::replace_governance::ReplaceGovernance { new_governance });
    let replaced = round_trip(&renamed, &replace);
    assert_eq!(replaced.governance.risk_appetite, Some("Low".into()));
}
//#endregion 🏷️📁🏛️meta-project-governance

//#region 🗺️🧹connect-disconnect-adjacency
#[semio_framework_async_macros::async_test]
async fn connect_and_disconnect_adjacency_round_trip() {
    let snapshot = sample_plugin();
    let a = snapshot.elements[0].header.id.clone();
    let b = snapshot.elements[1].header.id.clone();
    let new_adjacency = Adjacency {
        header: EntityHeader::new(EntityId::new_serial("adjacency", "adjacency"), "New Adjacency"),
        element_a_id: a,
        element_b_id: b,
        kind: AdjacencyKind::Preferred,
        connection: ConnectionKind::Direct,
        separations: Vec::new(),
        weight: 1.0,
        rationale: None,
        distance_max_m: None,
        distance_min_m: None,
        level_constraint: None,
        access_path: None,
        shared_wall: false,
        shared_entry: false,
        traffic_isolation: false,
        circulation_overlap: false,
        conflict_ids: Vec::new(),
        normalized: false,
        verification_status: ValidationStatus::Pending,
        source_relationship_id: None,
        internal_external_access: None,
    };
    let new_id = new_adjacency.header.id.clone();
    let connect = ProgramMutation::ConnectAdjacency(super::super::connect_adjacency::ConnectAdjacency { adjacency: new_adjacency });
    let connected = round_trip(&snapshot, &connect);
    assert_eq!(connected.adjacencies.len(), snapshot.adjacencies.len() + 1);

    let disconnect = ProgramMutation::DisconnectAdjacency(super::super::disconnect_adjacency::DisconnectAdjacency { id: new_id });
    let disconnected = round_trip(&connected, &disconnect);
    assert_eq!(disconnected.adjacencies.len(), snapshot.adjacencies.len());
}

#[semio_framework_async_macros::async_test]
async fn connect_adjacency_upserts_an_existing_pair_by_endpoint_identity() {
    let snapshot = sample_plugin();
    let existing = &snapshot.adjacencies[0];
    let mut updated = existing.clone();
    updated.header.id = EntityId::new_serial("adjacency", "adjacency");
    updated.weight = 5.0;
    let connect = ProgramMutation::ConnectAdjacency(super::super::connect_adjacency::ConnectAdjacency { adjacency: updated });
    let connected = round_trip(&snapshot, &connect);
    assert_eq!(connected.adjacencies.len(), snapshot.adjacencies.len(), "same-pair connect patches in place, it does not add a row");
    assert_eq!(connected.adjacencies[0].weight, 5.0);
    assert_eq!(connected.adjacencies[0].header.id, existing.header.id, "the pre-existing edge keeps its own id");
}
//#endregion 🗺️🧹connect-disconnect-adjacency

//#region 🧵connect-disconnect-trace
#[semio_framework_async_macros::async_test]
async fn connect_and_disconnect_trace_round_trip() {
    let snapshot = sample_plugin();
    let trace = TraceLink::new(snapshot.elements[0].header.id.clone(), snapshot.elements[1].header.id.clone(), TraceKind::FullAuditTrail);
    let id = trace.id.clone();
    let connect = ProgramMutation::ConnectTrace(super::super::connect_trace::ConnectTrace { trace });
    let connected = round_trip(&snapshot, &connect);
    assert_eq!(connected.traces.len(), 1);

    let disconnect = ProgramMutation::DisconnectTrace(super::super::disconnect_trace::DisconnectTrace { id });
    let disconnected = round_trip(&connected, &disconnect);
    assert!(disconnected.traces.is_empty());
}
//#endregion 🧵connect-disconnect-trace

//#region 🗣️OpText
#[semio_framework_async_macros::async_test]
async fn program_mutation_op_text_round_trips_a_sample_of_variants() {
    let stakeholder = sample_plugin().stakeholders[0].clone();
    store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::CreateStakeholder(super::super::create_stakeholder::CreateStakeholder { stakeholder: stakeholder.clone() }));
    store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::DeleteStakeholder(super::super::delete_stakeholder::DeleteStakeholder { id: stakeholder.header.id.clone() }));
    store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::RenameStakeholder(super::super::rename_stakeholder::RenameStakeholder { id: stakeholder.header.id.clone(), new_name: "X".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::ReplaceStakeholder(super::super::replace_stakeholder::ReplaceStakeholder { stakeholder }));
    store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::RenameMeta(super::super::rename_meta::RenameMeta { new_title: "Clinic".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::DisconnectAdjacency(super::super::disconnect_adjacency::DisconnectAdjacency { id: EntityId("a1".into()) }));
}
//#endregion 🗣️OpText

//#region ⚖️SemanticLaws
/// ⚖️ `assert_mutation_inverse_law`/`assert_mutation_diff_absorb_law`
/// (`protocol::os_spr::testkit`, added by the Wave 0 mechanism pass) against the three most
/// structurally distinct new kinds: an id-keyed collection create/delete pair, a document-level
/// scalar facet rename, and an edge upsert.
#[semio_framework_async_macros::async_test]
async fn create_stakeholder_obeys_the_inverse_and_absorb_laws() {
    let base = sample_plugin();
    let mut new_stakeholder = base.stakeholders[0].clone();
    new_stakeholder.header.id = EntityId::new_serial("stakeholder", "stakeholder");
    let create = ProgramMutation::CreateStakeholder(super::super::create_stakeholder::CreateStakeholder { stakeholder: new_stakeholder.clone() });
    protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &create).await;
    let d1 = create.diff(&base).into_parts().0;
    let after = d1.apply(&base).expect("valid mutation diff");
    let d2 = ProgramMutation::RenameStakeholder(super::super::rename_stakeholder::RenameStakeholder { id: new_stakeholder.header.id, new_name: "Renamed".into() }).diff(&after).into_parts().0;
    protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn rename_meta_obeys_the_inverse_law() {
    let base = sample_plugin();
    let rename = ProgramMutation::RenameMeta(super::super::rename_meta::RenameMeta { new_title: "Renamed Program".into() });
    protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &rename).await;
}

#[semio_framework_async_macros::async_test]
async fn connect_adjacency_obeys_the_inverse_law() {
    let base = sample_plugin();
    let mut updated = base.adjacencies[0].clone();
    updated.weight = 9.0;
    let connect = ProgramMutation::ConnectAdjacency(super::super::connect_adjacency::ConnectAdjacency { adjacency: updated });
    protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &connect).await;
}
//#endregion ⚖️SemanticLaws

//#region 📋️DescriptorLaws
#[semio_framework_async_macros::async_test]
async fn semantic_kinds_cover_every_variant() {
    assert_eq!(ProgramMutation::kinds().len(), 266);
    let stakeholder = sample_plugin().stakeholders[0].clone();
    let mutation = ProgramMutation::RenameStakeholder(super::super::rename_stakeholder::RenameStakeholder { id: stakeholder.header.id, new_name: "X".into() });
    assert_eq!(mutation.semantics().kind, "rename-stakeholder");
    assert_eq!(mutation.semantics().record, "RenamedStakeholder");
}
//#endregion 📋️DescriptorLaws
