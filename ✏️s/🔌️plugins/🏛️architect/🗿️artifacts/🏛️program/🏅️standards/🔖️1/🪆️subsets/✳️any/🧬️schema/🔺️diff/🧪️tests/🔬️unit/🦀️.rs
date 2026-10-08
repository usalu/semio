use super::*;

use crate::{empty_plugin, sample_plugin};
use protocol::{apply_diff, DiffAlgebra, MutationDiff};

fn rename_stakeholder_diff(id: &EntityId, name: &str) -> ProgramDiff {
    ProgramDiff { stakeholders: Some(ProgramStakeholdersDelta { patched: vec![ProgramStakeholdersPatchEntry { id: id.0.clone(), patch: StakeholderPatch { name: Some(name.into()), ..Default::default() } }], ..Default::default() }), ..Default::default() }
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
async fn a_section_replacement_sets_the_whole_section_and_a_later_patch_folds_into_it() {
    let base = sample_plugin();
    let mut project = base.project.clone();
    project.client_name = "Other Client".into();
    let replace = ProgramDiff { project: Some(ProjectDefinitionEdit::replacing(project.clone())), ..Default::default() };
    let patch = ProgramDiff { project: Some(ProjectDefinitionEdit::patching(ProjectDefinitionPatch { code: Some("NEW-001".into()), ..Default::default() })), ..Default::default() };
    let mut merged = replace.clone();
    merged.absorb(patch.clone());
    let sequential = apply_diff(&patch, &apply_diff(&replace, &base).expect("replace applies")).expect("patch applies");
    assert_eq!(apply_diff(&merged, &base).expect("merged applies"), sequential);
    assert_eq!(merged.project.as_ref().expect("project edit").patch, None, "the patch is folded into the replacement");
    assert_eq!(sequential.project.code, "NEW-001");
}

#[semio_framework_async_macros::async_test]
async fn knowledge_and_benchmark_deltas_rederive_their_composed_child_handles() {
    let base = sample_plugin();
    let record = base.knowledge_payload[0].clone();
    let diff = ProgramDiff { knowledge: Some(ProgramKnowledgeDelta { removed: vec![record.header.id.0.clone()], ..Default::default() }), ..Default::default() };
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
    let create = ProgramDiff { stakeholders: Some(ProgramStakeholdersDelta { added: vec![created.clone()], ..Default::default() }), ..Default::default() };
    let delete_new = ProgramDiff { stakeholders: Some(ProgramStakeholdersDelta { removed: vec![created.header.id.0.clone()], ..Default::default() }), ..Default::default() };
    let delete_old = ProgramDiff { stakeholders: Some(ProgramStakeholdersDelta { removed: vec![id.0.clone()], ..Default::default() }), ..Default::default() };
    let recreate_old = ProgramDiff { stakeholders: Some(ProgramStakeholdersDelta { added: vec![base.stakeholders[0].clone()], ..Default::default() }), ..Default::default() };

    let mut patch_patch = rename_stakeholder_diff(&id, "One");
    patch_patch.absorb(rename_stakeholder_diff(&id, "Two"));
    assert_eq!(patch_patch.stakeholders.as_ref().expect("delta").patched.len(), 1, "patch∘patch is one patch");

    let mut create_delete = create.clone();
    create_delete.absorb(delete_new);
    assert!(create_delete.is_empty() && create_delete.stakeholders.is_none(), "create∘delete leaves nothing");

    let mut delete_create = delete_old.clone();
    delete_create.absorb(recreate_old);
    let delta = delete_create.stakeholders.as_ref().expect("delta");
    assert_eq!((delta.removed.len(), delta.added.len()), (1, 1), "delete∘create is a replacement");
    assert_eq!(apply_diff(&delete_create, &base).expect("applies").stakeholders.len(), base.stakeholders.len());

    let mut patch_delete = rename_stakeholder_diff(&id, "Doomed");
    patch_delete.absorb(delete_old.clone());
    let delta = patch_delete.stakeholders.as_ref().expect("delta");
    assert_eq!((delta.removed.clone(), delta.patched.len(), delta.added.len()), (vec![id.0.clone()], 0, 0), "patch∘delete is a delete");
    assert_eq!(apply_diff(&patch_delete, &base).expect("applies"), apply_diff(&delete_old, &base).expect("applies"));
}

#[semio_framework_async_macros::async_test]
async fn the_program_diff_obeys_the_absorb_inverse_and_between_laws() {
    let base = sample_plugin();
    let id = base.stakeholders[0].header.id.clone();
    let d1 = rename_stakeholder_diff(&id, "One");
    let d2 = ProgramDiff { meta: Some(ProgramMetaEdit::patching(ProgramMetaPatch { title: Some("T".into()), ..Default::default() })), ..rename_stakeholder_diff(&id, "Two") };
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1.clone(), d2.clone()).await;
    protocol::os_spr::protocol_laws::assert_diff_algebra_inverse_law(&base, &d2).await;
    protocol::os_spr::protocol_laws::assert_diff_algebra_between_law::<ProgramSnapshot, ProgramDiff>(&base, &apply_diff(&d2, &base).expect("applies")).await;
    protocol::os_spr::protocol_laws::assert_diff_algebra_between_law::<ProgramSnapshot, ProgramDiff>(&base, &empty_plugin()).await;
}

#[semio_framework_async_macros::async_test]
async fn apply_addresses_a_rejected_collection_delta_by_its_section() {
    let base = sample_plugin();
    let diff = ProgramDiff { stakeholders: Some(ProgramStakeholdersDelta { removed: vec!["missing".into()], ..Default::default() }), ..Default::default() };
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
