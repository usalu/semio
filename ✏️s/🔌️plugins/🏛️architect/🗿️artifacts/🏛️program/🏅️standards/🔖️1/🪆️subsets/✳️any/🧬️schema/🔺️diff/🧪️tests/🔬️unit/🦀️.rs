use super::*;

use crate::sample_plugin;
use protocol::{apply_diff, DiffAlgebra, MutationDiff};

fn rename_stakeholder_diff(id: &EntityId, name: &str) -> ProgramDiff {
    ProgramDiff { stakeholders: Some(ProgramStakeholdersDelta { modified: vec![ProgramStakeholdersPatchEntry { id: id.0.clone(), patch: StakeholderPatch { name: Some(name.into()), ..Default::default() } }], ..Default::default() }), ..Default::default() }
}

#[semio_framework_async_macros::async_test]
async fn apply_writes_a_scalar_section_patch_without_touching_the_rest() {
    let base = sample_plugin();
    let diff = ProgramDiff { meta: Some(ProgramMetaEdit::patching(ProgramMetaPatch { title: Some("Renamed".into()), ..Default::default() })), ..Default::default() };
    let next = apply_diff(&diff, &base).expect("valid diff");
    let mut expected = base.clone();
    expected.meta.title = "Renamed".into();
    assert_eq!(next, expected);
}

#[semio_framework_async_macros::async_test]
async fn a_section_replacement_is_applied_before_a_later_patch_of_the_same_section() {
    let base = sample_plugin();
    let mut project = base.project.clone();
    project.client_name = "Other Client".into();
    let replace = ProgramDiff { project: Some(ProjectDefinitionEdit::replacing(project.clone())), ..Default::default() };
    let patch = ProgramDiff { project: Some(ProjectDefinitionEdit::patching(ProjectDefinitionPatch { code: Some("NEW-001".into()), ..Default::default() })), ..Default::default() };
    let mut merged = replace.clone();
    merged.absorb(patch.clone());
    let sequential = apply_diff(&patch, &apply_diff(&replace, &base).expect("replace applies")).expect("patch applies");
    assert_eq!(apply_diff(&merged, &base).expect("merged applies"), sequential);
    let edit = merged.project.as_ref().expect("project edit");
    assert!(edit.set.is_some() && edit.patch.is_some(), "the replacement stays and the later patch is written after it");
    assert_eq!(sequential.project.code, "NEW-001");
}

#[semio_framework_async_macros::async_test]
async fn knowledge_and_benchmark_deltas_rederive_their_composed_child_handles() {
    let base = sample_plugin();
    let diff = ProgramDiff { knowledge: Some(ProgramKnowledgeDelta::removal(&base.knowledge_payload, 0)), ..Default::default() };
    let next = apply_diff(&diff, &base).expect("valid diff");
    assert!(next.knowledge_payload.is_empty());
    assert_eq!(next.knowledge, crate::knowledge_child_from_records(&[]), "the child handle is derived from the remaining rows, never carried in the diff");
    assert_eq!(next.benchmarks, base.benchmarks, "an untouched table keeps its handle");
}

#[semio_framework_async_macros::async_test]
async fn absorb_coalesces_same_id_entries_of_the_program_diff() {
    let base = sample_plugin();
    let id = base.stakeholders[0].header.id.clone();
    let mut created = base.stakeholders[0].clone();
    created.header.id = EntityId("stakeholder-new".into());
    let create = ProgramDiff { stakeholders: Some(ProgramStakeholdersDelta::insertion(base.stakeholders.len(), created.clone())), ..Default::default() };
    let delete_new = ProgramDiff { stakeholders: Some(ProgramStakeholdersDelta { removed: vec![ProgramStakeholdersRemoval { id: created.header.id.0.clone(), index: base.stakeholders.len() }], ..Default::default() }), ..Default::default() };
    let delete_old = ProgramDiff { stakeholders: Some(ProgramStakeholdersDelta::removal(&base.stakeholders, 0)), ..Default::default() };
    let recreate_old = ProgramDiff { stakeholders: Some(ProgramStakeholdersDelta::insertion(0, base.stakeholders[0].clone())), ..Default::default() };

    let mut patch_patch = rename_stakeholder_diff(&id, "One");
    patch_patch.absorb(rename_stakeholder_diff(&id, "Two"));
    assert_eq!(patch_patch.stakeholders.as_ref().expect("delta").modified.len(), 1, "patch∘patch is one patch");

    let mut create_delete = create.clone();
    create_delete.absorb(delete_new);
    assert!(create_delete.is_empty() && create_delete.stakeholders.is_none(), "create∘delete leaves nothing");

    let mut delete_create = delete_old.clone();
    delete_create.absorb(recreate_old);
    let delta = delete_create.stakeholders.as_ref().expect("delta");
    assert_eq!((delta.removed.len(), delta.inserted.len()), (1, 1), "delete∘create is a replacement");
    assert_eq!(apply_diff(&delete_create, &base).expect("applies").stakeholders.len(), base.stakeholders.len());

    let mut patch_delete = rename_stakeholder_diff(&id, "Doomed");
    patch_delete.absorb(delete_old.clone());
    let delta = patch_delete.stakeholders.as_ref().expect("delta");
    assert_eq!((delta.removed.clone(), delta.modified.len(), delta.inserted.len()), (vec![ProgramStakeholdersRemoval { id: id.0.clone(), index: 0 }], 0, 0), "patch∘delete is a delete");
    assert_eq!(apply_diff(&patch_delete, &base).expect("applies"), apply_diff(&delete_old, &base).expect("applies"));
}

#[semio_framework_async_macros::async_test]
async fn the_program_diff_obeys_the_absorb_and_inverse_laws() {
    let base = sample_plugin();
    let id = base.stakeholders[0].header.id.clone();
    let d1 = rename_stakeholder_diff(&id, "One");
    let d2 = ProgramDiff { meta: Some(ProgramMetaEdit::patching(ProgramMetaPatch { title: Some("T".into()), ..Default::default() })), ..rename_stakeholder_diff(&id, "Two") };
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1.clone(), d2.clone()).await;
    protocol::os_spr::protocol_laws::assert_diff_algebra_inverse_law(&base, &d2).await;
}

#[semio_framework_async_macros::async_test]
async fn a_middle_row_removal_inverts_to_a_reinsertion_at_its_base_index() {
    let mut base = sample_plugin();
    let template = base.stakeholders[0].clone();
    base.stakeholders = (0..3).map(|n| Stakeholder { header: EntityHeader { id: EntityId(format!("stakeholder-{n}")), ..template.header.clone() }, ..template.clone() }).collect();
    let delete = ProgramDiff { stakeholders: Some(ProgramStakeholdersDelta::removal(&base.stakeholders, 1)), ..Default::default() };
    let after = apply_diff(&delete, &base).expect("applies");
    assert_eq!(after.stakeholders.iter().map(|row| row.header.id.0.as_str()).collect::<Vec<_>>(), ["stakeholder-0", "stakeholder-2"]);
    let undo = delete.inverse(&base);
    assert_eq!(undo.stakeholders.as_ref().expect("delta").inserted.iter().map(|entry| entry.index).collect::<Vec<_>>(), [1]);
    assert_eq!(apply_diff(&undo, &after).expect("inverse applies"), base);
    protocol::os_spr::protocol_laws::assert_diff_algebra_inverse_law(&base, &delete).await;
}

#[semio_framework_async_macros::async_test]
async fn apply_addresses_a_rejected_collection_delta_by_its_section() {
    let base = sample_plugin();
    let diff = ProgramDiff { stakeholders: Some(ProgramStakeholdersDelta { removed: vec![ProgramStakeholdersRemoval { id: "missing".into(), index: 0 }], ..Default::default() }), ..Default::default() };
    let error = apply_diff(&diff, &base).expect_err("an unknown removal is rejected");
    assert_eq!(error.code, "mutation.apply.missing-target");
    assert_eq!(error.target.first().map(String::as_str), Some("stakeholders"));
}

#[test]
fn architect_native_sparse_diff_preserves_declared_roles_and_exact_text() {
    use semio_framework_dsl_record::{parse_exact, print, JoinMode, ParseOptions};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🔗️native-roles/🧫️fixtures/🔣️.json")).unwrap();
    let delta: ProgramStakeholdersDelta = serde_json::from_value(fixture["delta"].clone()).unwrap();
    for value in [ProgramDiff::default(), ProgramDiff { stakeholders: Some(delta), ..Default::default() }] {
        let record = value.__dsl_to_record();
        let spec = ProgramDiff::__dsl_spec();
        for mode in [JoinMode::Inline, JoinMode::Document] {
            let text = print(&record, &spec, mode);
            let parsed = parse_exact(&text, &spec, &ParseOptions::default()).unwrap();
            assert_eq!(ProgramDiff::__dsl_from_record(&parsed).unwrap(), value);
            assert!(parse_exact(&format!("{text} unowned 17"), &spec, &ParseOptions::default()).is_err());
        }
    }
    eprintln!("[DEBUG] Architect native sparse delta roles preserve independent serde fixture and exact terminal grammar");
}
