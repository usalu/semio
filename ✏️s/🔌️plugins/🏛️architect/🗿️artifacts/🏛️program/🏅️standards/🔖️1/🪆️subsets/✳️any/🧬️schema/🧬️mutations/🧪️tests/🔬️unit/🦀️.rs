use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::diff::ProgramDiff;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::standards::v1::subsets::any::schema::kernel::*;
use crate::standards::v1::subsets::any::schema::registers::*;
use crate::standards::v1::subsets::any::schema::registers::catalog::{RegisterKey, RegisterRow};
use crate::standards::v1::subsets::any::schema::snapshot::{empty_plugin, sample_plugin};
use protocol::{Mutation, SemanticMutation};

fn round_trip(snapshot: &ProgramSnapshot, operation: &ProgramMutation) -> ProgramSnapshot {
    let forward = protocol::apply_diff(operation.diff(snapshot).diff(), snapshot).expect("valid mutation diff");
    let mut backward = operation.inverse(snapshot).expect("valid retained mutation inverse fixture");
    backward.reverse();
    let mut restored = forward.clone();
    for undo in &backward {
        restored = protocol::apply_diff(undo.diff(&restored).diff(), &restored).expect("valid mutation diff");
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

    let create = ProgramMutation::CreateRegisterRow(crate::standards::v1::subsets::any::schema::mutations::create_register_row::CreateRegisterRow { row: RegisterRow::Stakeholder(new_stakeholder), index: None });
    let with_new = round_trip(&snapshot, &create);
    assert_eq!(with_new.stakeholders.len(), snapshot.stakeholders.len() + 1);

    let rename = ProgramMutation::RenameRegisterRow(crate::standards::v1::subsets::any::schema::mutations::rename_register_row::RenameRegisterRow { register: RegisterKey::Stakeholder, id: new_id.clone(), new_name: "Renamed".into() });
    let renamed = round_trip(&with_new, &rename);
    assert_eq!(renamed.stakeholders.iter().find(|s| s.header.id == new_id).unwrap().header.name, "Renamed");

    let mut replacement = renamed.stakeholders.iter().find(|s| s.header.id == new_id).unwrap().clone();
    replacement.role = "Sponsor".into();
    let replace = ProgramMutation::ReplaceRegisterRow(crate::standards::v1::subsets::any::schema::mutations::replace_register_row::ReplaceRegisterRow { row: RegisterRow::Stakeholder(replacement) });
    let replaced = round_trip(&renamed, &replace);
    assert_eq!(replaced.stakeholders.iter().find(|s| s.header.id == new_id).unwrap().role, "Sponsor");

    let delete = ProgramMutation::DeleteRegisterRow(crate::standards::v1::subsets::any::schema::mutations::delete_register_row::DeleteRegisterRow { register: RegisterKey::Stakeholder, id: new_id });
    let deleted = round_trip(&replaced, &delete);
    assert_eq!(deleted.stakeholders.len(), snapshot.stakeholders.len());
}

#[semio_framework_async_macros::async_test]
async fn delete_stakeholder_of_a_missing_id_has_an_empty_inverse() {
    let snapshot = sample_plugin();
    let delete = ProgramMutation::DeleteRegisterRow(crate::standards::v1::subsets::any::schema::mutations::delete_register_row::DeleteRegisterRow { register: RegisterKey::Stakeholder, id: EntityId("nope".into()) });
    assert!(delete.inverse(&snapshot).expect("valid retained mutation inverse fixture").is_empty(), "deleting an absent id has nothing to undo");
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

    let create = ProgramMutation::CreateRegisterRow(crate::standards::v1::subsets::any::schema::mutations::create_register_row::CreateRegisterRow { row: RegisterRow::ProgramElement(new_element), index: None });
    let with_new = round_trip(&snapshot, &create);
    assert_eq!(with_new.elements.len(), snapshot.elements.len() + 1);

    let rename = ProgramMutation::RenameRegisterRow(crate::standards::v1::subsets::any::schema::mutations::rename_register_row::RenameRegisterRow { register: RegisterKey::ProgramElement, id: new_id.clone(), new_name: "Storage Room".into() });
    let renamed = round_trip(&with_new, &rename);
    assert_eq!(renamed.elements.iter().find(|e| e.header.id == new_id).unwrap().header.name, "Storage Room");

    let mut replacement = renamed.elements.iter().find(|e| e.header.id == new_id).unwrap().clone();
    replacement.code = "STO".into();
    let replace = ProgramMutation::ReplaceRegisterRow(crate::standards::v1::subsets::any::schema::mutations::replace_register_row::ReplaceRegisterRow { row: RegisterRow::ProgramElement(replacement) });
    let replaced = round_trip(&renamed, &replace);
    assert_eq!(replaced.elements.iter().find(|e| e.header.id == new_id).unwrap().code, "STO");

    let delete = ProgramMutation::DeleteRegisterRow(crate::standards::v1::subsets::any::schema::mutations::delete_register_row::DeleteRegisterRow { register: RegisterKey::ProgramElement, id: new_id });
    let deleted = round_trip(&replaced, &delete);
    assert_eq!(deleted.elements.len(), snapshot.elements.len());
}
//#endregion 🧱elements

//#region 🏷️📁🏛️meta-project-governance
#[semio_framework_async_macros::async_test]
async fn update_meta_rename_and_replace_round_trip() {
    let snapshot = empty_plugin();
    let rename = ProgramMutation::RenameMeta(crate::standards::v1::subsets::any::schema::mutations::rename_meta::RenameMeta { new_title: "Clinic".into() });
    let renamed = round_trip(&snapshot, &rename);
    assert_eq!(renamed.meta.title, "Clinic");

    let mut new_meta = renamed.meta.clone();
    new_meta.industry_sector = "healthcare".into();
    let replace = ProgramMutation::ReplaceMeta(crate::standards::v1::subsets::any::schema::mutations::replace_meta::ReplaceMeta { new_meta });
    let replaced = round_trip(&renamed, &replace);
    assert_eq!(replaced.meta.industry_sector, "healthcare");
}

#[semio_framework_async_macros::async_test]
async fn update_project_rename_and_replace_round_trip() {
    let snapshot = empty_plugin();
    let rename = ProgramMutation::RenameProject(crate::standards::v1::subsets::any::schema::mutations::rename_project::RenameProject { new_code: "CLN-001".into() });
    let renamed = round_trip(&snapshot, &rename);
    assert_eq!(renamed.project.code, "CLN-001");

    let mut new_project = renamed.project.clone();
    new_project.client_name = "Sample Health".into();
    let replace = ProgramMutation::ReplaceProject(crate::standards::v1::subsets::any::schema::mutations::replace_project::ReplaceProject { new_project });
    let replaced = round_trip(&renamed, &replace);
    assert_eq!(replaced.project.client_name, "Sample Health");
}

#[semio_framework_async_macros::async_test]
async fn update_governance_rename_and_replace_round_trip() {
    let snapshot = empty_plugin();
    let rename = ProgramMutation::RenameGovernance(crate::standards::v1::subsets::any::schema::mutations::rename_governance::RenameGovernance { new_framework: "ISO 41001".into() });
    let renamed = round_trip(&snapshot, &rename);
    assert_eq!(renamed.governance.framework, "ISO 41001");

    let mut new_governance = renamed.governance.clone();
    new_governance.risk_appetite = Some("Low".into());
    let replace = ProgramMutation::ReplaceGovernance(crate::standards::v1::subsets::any::schema::mutations::replace_governance::ReplaceGovernance { new_governance });
    let replaced = round_trip(&renamed, &replace);
    assert_eq!(replaced.governance.risk_appetite, Some("Low".into()));
}
//#endregion 🏷️📁🏛️meta-project-governance

//#region 🗺️🧹connect-disconnect-adjacency
#[semio_framework_async_macros::async_test]
async fn connect_and_disconnect_adjacency_round_trip() {
    let mut snapshot = sample_plugin();
    snapshot.adjacencies.clear();
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
    let connect = ProgramMutation::ConnectAdjacency(crate::standards::v1::subsets::any::schema::mutations::connect_adjacency::ConnectAdjacency { adjacency: new_adjacency, index: None });
    let connected = round_trip(&snapshot, &connect);
    assert_eq!(connected.adjacencies.len(), snapshot.adjacencies.len() + 1);

    let disconnect = ProgramMutation::DisconnectAdjacency(crate::standards::v1::subsets::any::schema::mutations::disconnect_adjacency::DisconnectAdjacency { id: new_id });
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
    let connect = ProgramMutation::ConnectAdjacency(crate::standards::v1::subsets::any::schema::mutations::connect_adjacency::ConnectAdjacency { adjacency: updated, index: None });
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
    let connect = ProgramMutation::ConnectTrace(crate::standards::v1::subsets::any::schema::mutations::connect_trace::ConnectTrace { trace, index: None });
    let connected = round_trip(&snapshot, &connect);
    assert_eq!(connected.traces.len(), 1);

    let disconnect = ProgramMutation::DisconnectTrace(crate::standards::v1::subsets::any::schema::mutations::disconnect_trace::DisconnectTrace { id });
    let disconnected = round_trip(&connected, &disconnect);
    assert!(disconnected.traces.is_empty());
}
//#endregion 🧵connect-disconnect-trace

//#region 🗣️OpText
#[semio_framework_async_macros::async_test]
async fn program_mutation_op_text_round_trips_a_sample_of_variants() {
    let stakeholder = sample_plugin().stakeholders[0].clone();
    store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::CreateRegisterRow(crate::standards::v1::subsets::any::schema::mutations::create_register_row::CreateRegisterRow { row: RegisterRow::Stakeholder(stakeholder.clone()), index: None }));
    store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::DeleteRegisterRow(crate::standards::v1::subsets::any::schema::mutations::delete_register_row::DeleteRegisterRow { register: RegisterKey::Stakeholder, id: stakeholder.header.id.clone() }));
    store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::RenameRegisterRow(crate::standards::v1::subsets::any::schema::mutations::rename_register_row::RenameRegisterRow { register: RegisterKey::Stakeholder, id: stakeholder.header.id.clone(), new_name: "X".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::ReplaceRegisterRow(crate::standards::v1::subsets::any::schema::mutations::replace_register_row::ReplaceRegisterRow { row: RegisterRow::Stakeholder(stakeholder) }));
    store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::RenameMeta(crate::standards::v1::subsets::any::schema::mutations::rename_meta::RenameMeta { new_title: "Clinic".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&ProgramMutation::DisconnectAdjacency(crate::standards::v1::subsets::any::schema::mutations::disconnect_adjacency::DisconnectAdjacency { id: EntityId("a1".into()) }));
}
//#endregion 🗣️OpText

//#region ⚖️SemanticLaws
/// ⚖️ `assert_mutation_inverse_law`/`assert_mutation_diff_absorb_law`
/// (`protocol::os_spr::protocol_laws`, added by the Wave 0 mechanism pass) against the three most
/// structurally distinct new kinds: an id-keyed collection create/delete pair, a document-level
/// scalar facet rename, and an edge upsert.
#[semio_framework_async_macros::async_test]
async fn create_stakeholder_obeys_the_inverse_and_absorb_laws() {
    let base = sample_plugin();
    let mut new_stakeholder = base.stakeholders[0].clone();
    new_stakeholder.header.id = EntityId::new_serial("stakeholder", "stakeholder");
    let create = ProgramMutation::CreateRegisterRow(crate::standards::v1::subsets::any::schema::mutations::create_register_row::CreateRegisterRow { row: RegisterRow::Stakeholder(new_stakeholder.clone()), index: None });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &create).await;
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&create, &base).await;
    let d1 = create.diff(&base).into_parts().0;
    let after = protocol::apply_diff(&d1, &base).expect("valid mutation diff");
    let d2 = ProgramMutation::RenameRegisterRow(crate::standards::v1::subsets::any::schema::mutations::rename_register_row::RenameRegisterRow { register: RegisterKey::Stakeholder, id: new_stakeholder.header.id, new_name: "Renamed".into() }).diff(&after).into_parts().0;
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn rename_meta_obeys_the_inverse_law() {
    let base = sample_plugin();
    let rename = ProgramMutation::RenameMeta(crate::standards::v1::subsets::any::schema::mutations::rename_meta::RenameMeta { new_title: "Renamed Program".into() });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &rename).await;
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&rename, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn connect_adjacency_obeys_the_inverse_law() {
    let base = sample_plugin();
    let mut updated = base.adjacencies[0].clone();
    updated.weight = 9.0;
    let connect = ProgramMutation::ConnectAdjacency(crate::standards::v1::subsets::any::schema::mutations::connect_adjacency::ConnectAdjacency { adjacency: updated, index: None });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &connect).await;
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&connect, &base).await;
}
/// ➕️ L3 for the id-keyed kinds on a program with several rows: replacing a middle row keeps its position, so the inverse diffs sum to
/// the negative of the forward diff row order included, and a create on top of existing rows inverts to a plain removal.
#[semio_framework_async_macros::async_test]
async fn replace_of_a_middle_row_keeps_its_position_and_obeys_the_sum_law() {
    let mut base = sample_plugin();
    for suffix in ["two", "three"] {
        let mut row = base.stakeholders[0].clone();
        row.header.id = EntityId(format!("stakeholder-{suffix}"));
        row.header.name = format!("Stakeholder {suffix}");
        base.stakeholders.push(row);
    }
    let mut replacement = base.stakeholders[1].clone();
    replacement.role = "Sponsor".into();
    replacement.department = Some("Operations".into());
    let order: Vec<EntityId> = base.stakeholders.iter().map(|row| row.header.id.clone()).collect();
    let replace = ProgramMutation::ReplaceRegisterRow(crate::standards::v1::subsets::any::schema::mutations::replace_register_row::ReplaceRegisterRow { row: RegisterRow::Stakeholder(replacement) });
    let after = protocol::apply_diff(replace.diff(&base).diff(), &base).expect("valid mutation diff");
    assert_eq!(after.stakeholders.iter().map(|row| row.header.id.clone()).collect::<Vec<_>>(), order, "the replaced row keeps its position");
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&replace, &base).await;
    let mut created = base.stakeholders[0].clone();
    created.header.id = EntityId("stakeholder-new".into());
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&ProgramMutation::CreateRegisterRow(crate::standards::v1::subsets::any::schema::mutations::create_register_row::CreateRegisterRow { row: RegisterRow::Stakeholder(created), index: None }), &base).await;
    let rename = ProgramMutation::RenameRegisterRow(crate::standards::v1::subsets::any::schema::mutations::rename_register_row::RenameRegisterRow { register: RegisterKey::Stakeholder, id: order[1].clone(), new_name: "Renamed".into() });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&rename, &base).await;
    let delete_last = ProgramMutation::DeleteRegisterRow(crate::standards::v1::subsets::any::schema::mutations::delete_register_row::DeleteRegisterRow { register: RegisterKey::Stakeholder, id: order[2].clone() });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&delete_last, &base).await;
}

/// ➕️ L3 for the composed knowledge and benchmark tables: the row deltas apply to the persisted payload, the child handle is re-derived,
/// and the inverse diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn composed_table_kinds_obey_the_sum_law() {
    let base = sample_plugin();
    let knowledge = base.knowledge_payload[0].clone();
    let benchmark = base.benchmarks_payload[0].clone();
    let mut renamed_knowledge = knowledge.clone();
    renamed_knowledge.topic = "Other topic".into();
    let mut renamed_benchmark = benchmark.clone();
    renamed_benchmark.metric = "Other metric".into();
    for mutation in [
        ProgramMutation::DeleteRegisterRow(crate::standards::v1::subsets::any::schema::mutations::delete_register_row::DeleteRegisterRow { register: RegisterKey::KnowledgeRecord, id: knowledge.header.id.clone() }),
        ProgramMutation::RenameRegisterRow(crate::standards::v1::subsets::any::schema::mutations::rename_register_row::RenameRegisterRow { register: RegisterKey::KnowledgeRecord, id: knowledge.header.id.clone(), new_name: "Renamed".into() }),
        ProgramMutation::ReplaceRegisterRow(crate::standards::v1::subsets::any::schema::mutations::replace_register_row::ReplaceRegisterRow { row: RegisterRow::KnowledgeRecord(renamed_knowledge) }),
        ProgramMutation::DeleteRegisterRow(crate::standards::v1::subsets::any::schema::mutations::delete_register_row::DeleteRegisterRow { register: RegisterKey::BenchmarkRecord, id: benchmark.header.id.clone() }),
        ProgramMutation::RenameRegisterRow(crate::standards::v1::subsets::any::schema::mutations::rename_register_row::RenameRegisterRow { register: RegisterKey::BenchmarkRecord, id: benchmark.header.id.clone(), new_name: "Renamed".into() }),
        ProgramMutation::ReplaceRegisterRow(crate::standards::v1::subsets::any::schema::mutations::replace_register_row::ReplaceRegisterRow { row: RegisterRow::BenchmarkRecord(renamed_benchmark) }),
    ] {
        let after = protocol::apply_diff(mutation.diff(&base).diff(), &base).expect("valid mutation diff");
        assert_eq!(after.knowledge, crate::standards::v1::subsets::any::schema::snapshot::knowledge_child_from_records(&after.knowledge_payload), "the knowledge handle follows its rows");
        assert_eq!(after.benchmarks, crate::standards::v1::subsets::any::schema::snapshot::benchmarks_child_from_records(&after.benchmarks_payload), "the benchmarks handle follows its rows");
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
}

/// ➕️ L3 for the three singleton sections: a rename patches one field, a replace sets the whole section, and both invert exactly.
#[semio_framework_async_macros::async_test]
async fn singleton_section_kinds_obey_the_sum_law() {
    let base = sample_plugin();
    let mut project = base.project.clone();
    project.client_name = "Other client".into();
    let mut governance = base.governance.clone();
    governance.risk_appetite = Some("low".into());
    let mut meta = base.meta.clone();
    meta.subtitle = Some("Subtitle".into());
    for mutation in [
        ProgramMutation::RenameProject(crate::standards::v1::subsets::any::schema::mutations::rename_project::RenameProject { new_code: "NEW".into() }),
        ProgramMutation::ReplaceProject(crate::standards::v1::subsets::any::schema::mutations::replace_project::ReplaceProject { new_project: project }),
        ProgramMutation::RenameGovernance(crate::standards::v1::subsets::any::schema::mutations::rename_governance::RenameGovernance { new_framework: "Framework".into() }),
        ProgramMutation::ReplaceGovernance(crate::standards::v1::subsets::any::schema::mutations::replace_governance::ReplaceGovernance { new_governance: governance }),
        ProgramMutation::RenameMeta(crate::standards::v1::subsets::any::schema::mutations::rename_meta::RenameMeta { new_title: "Title".into() }),
        ProgramMutation::ReplaceMeta(crate::standards::v1::subsets::any::schema::mutations::replace_meta::ReplaceMeta { new_meta: meta }),
    ] {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
}
/// 📍️ A create with an `index` inserts at that position and a delete of a non-last row is undone by a create at the original index, so
/// the summed inverse diffs carry the row position (`inserted` at its original index) as well as the row.
#[semio_framework_async_macros::async_test]
async fn create_at_an_index_and_delete_of_a_middle_row_obey_the_sum_law() {
    let mut base = sample_plugin();
    for suffix in ["two", "three"] {
        let mut row = base.stakeholders[0].clone();
        row.header.id = EntityId(format!("stakeholder-{suffix}"));
        base.stakeholders.push(row);
    }
    let order: Vec<EntityId> = base.stakeholders.iter().map(|row| row.header.id.clone()).collect();
    let mut inserted = base.stakeholders[0].clone();
    inserted.header.id = EntityId("stakeholder-inserted".into());
    let create_at = |index: Option<usize>| ProgramMutation::CreateRegisterRow(crate::standards::v1::subsets::any::schema::mutations::create_register_row::CreateRegisterRow { row: RegisterRow::Stakeholder(inserted.clone()), index: index });
    let after = protocol::apply_diff(create_at(Some(1)).diff(&base).diff(), &base).expect("valid mutation diff");
    let ids: Vec<&str> = after.stakeholders.iter().map(|row| row.header.id.0.as_str()).collect();
    assert_eq!(ids, vec![order[0].0.as_str(), "stakeholder-inserted", order[1].0.as_str(), order[2].0.as_str()], "the row lands at the requested index");
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&create_at(Some(1)), &base).await;
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&create_at(Some(order.len())), &base).await;
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&create_at(None), &base).await;
    let beyond = create_at(Some(order.len() + 1)).diff(&base);
    assert!(beyond.messages().iter().any(|message| message.code.0 == "mutation.target-missing"), "an index past the end is rejected");
    for id in &order {
        let delete = ProgramMutation::DeleteRegisterRow(crate::standards::v1::subsets::any::schema::mutations::delete_register_row::DeleteRegisterRow { register: RegisterKey::Stakeholder, id: id.clone() });
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&delete, &base).await;
    }
}

/// 📍️ The same positional law for the edge-shaped registers and the composed tables: a middle delete or disconnect is undone at its index.
#[semio_framework_async_macros::async_test]
async fn middle_deletes_of_edges_and_composed_rows_restore_their_position() {
    let mut base = sample_plugin();
    let mut knowledge = base.knowledge_payload[0].clone();
    knowledge.header.id = EntityId("knowledge-two".into());
    base.knowledge_payload.push(knowledge.clone());
    knowledge.header.id = EntityId("knowledge-three".into());
    base.knowledge_payload.push(knowledge);
    base.knowledge = crate::standards::v1::subsets::any::schema::snapshot::knowledge_child_from_records(&base.knowledge_payload);
    let delete = ProgramMutation::DeleteRegisterRow(crate::standards::v1::subsets::any::schema::mutations::delete_register_row::DeleteRegisterRow { register: RegisterKey::KnowledgeRecord, id: base.knowledge_payload[1].header.id.clone() });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&delete, &base).await;
    let mut trace = TraceLink::new(base.elements[0].header.id.clone(), base.elements[1].header.id.clone(), TraceKind::FunctionToProgramElement);
    for suffix in ["a", "b", "c"] {
        trace.id = EntityId(format!("trace-{suffix}"));
        base.traces.push(trace.clone());
    }
    let disconnect = ProgramMutation::DisconnectTrace(crate::standards::v1::subsets::any::schema::mutations::disconnect_trace::DisconnectTrace { id: EntityId("trace-b".into()) });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&disconnect, &base).await;
}
//#endregion ⚖️SemanticLaws

//#region 📋️DescriptorLaws
#[semio_framework_async_macros::async_test]
async fn semantic_kinds_cover_every_variant() {
    assert_eq!(ProgramMutation::kinds().len(), 14);
    let stakeholder = sample_plugin().stakeholders[0].clone();
    let mutation = ProgramMutation::RenameRegisterRow(crate::standards::v1::subsets::any::schema::mutations::rename_register_row::RenameRegisterRow { register: RegisterKey::Stakeholder, id: stakeholder.header.id, new_name: "X".into() });
    assert_eq!(mutation.semantics().kind, "rename-register-row");
    assert_eq!(mutation.semantics().record, "RenamedRegisterRow");
}
//#endregion 📋️DescriptorLaws
