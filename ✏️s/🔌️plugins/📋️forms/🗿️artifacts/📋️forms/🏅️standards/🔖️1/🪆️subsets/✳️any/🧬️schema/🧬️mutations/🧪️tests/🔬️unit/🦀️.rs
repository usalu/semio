use super::*;
use crate::mutations::{change_form_title, change_step_description, create_block, create_step, delete_block, delete_step, move_block_to_step, rename_step, reorder_step, replace_block};
use crate::{FormQuestion, FormStep, FORMS_DOCUMENT_SCHEMA};
use protocol::os_spr::testkit::{assert_fatal_never_applies, assert_missing_target_is_error};
use protocol::{MutationDiff, SemanticMutation};

fn sample_step(id: &str) -> FormStep {
    FormStep { id: id.into(), title: format!("Step {id}"), description: None, blocks: Vec::new() }
}

fn sample_block(id: &str) -> FormQuestion {
    FormQuestion {
        id: id.into(),
        label: format!("Block {id}"),
        kind: "text".into(),
        description: None,
        required: None,
        placeholder: None,
        default: None,
        min: None,
        max: None,
        step: None,
        unit: None,
        text: None,
        options: None,
        fields: None,
        schema: None,
        src: None,
        accept: None,
        fixture_slug: None,
        params: None,
        condition: None,
    }
}

fn base_snapshot() -> FormsSnapshot {
    base_snapshot_with_steps(vec![sample_step("s1"), sample_step("s2")])
}

fn base_snapshot_with_steps(steps: Vec<FormStep>) -> FormsSnapshot {
    crate::forms_snapshot_with_state(FORMS_DOCUMENT_SCHEMA.into(), "forms".into(), "1".into(), None, &steps)
}

fn steps_of(snapshot: &FormsSnapshot) -> Vec<FormStep> {
    crate::forms_steps(snapshot)
}

#[semio_framework_async_macros::async_test]
async fn create_then_delete_step_round_trips() {
    let base = base_snapshot();
    let create = FormMutation::CreateStep(create_step::mutation::CreateStep { step: sample_step("s3"), index: None });
    let after_create = create.diff(&base).diff().apply(&base).expect("valid mutation diff");
    assert!(steps_of(&after_create).iter().any(|step| step.id == "s3"));

    let undo = create.inverse(&base);
    assert_eq!(undo, vec![FormMutation::DeleteStep(delete_step::mutation::DeleteStep { id: "s3".into() })]);
    let mut state = after_create;
    for step in &undo {
        state = step.diff(&state).diff().apply(&state).expect("valid mutation diff");
    }
    assert_eq!(steps_of(&state), steps_of(&base));
}

#[semio_framework_async_macros::async_test]
async fn delete_step_inverse_recreates_step_with_its_blocks_at_original_index() {
    let mut step1 = sample_step("s1");
    step1.blocks.push(sample_block("b1"));
    let base = base_snapshot_with_steps(vec![step1, sample_step("s2")]);
    let delete = FormMutation::DeleteStep(delete_step::mutation::DeleteStep { id: "s1".into() });
    let after_delete = delete.diff(&base).diff().apply(&base).expect("valid mutation diff");
    assert!(!steps_of(&after_delete).iter().any(|step| step.id == "s1"));

    let undo = delete.inverse(&base);
    match undo.first() {
        Some(FormMutation::CreateStep(payload)) => {
            assert_eq!(payload.index, Some(0));
            assert_eq!(payload.step.blocks.len(), 1);
        }
        other => panic!("expected CreateStep as delete-step's inverse, got {other:?}"),
    }
    let mut state = after_delete;
    for step in &undo {
        state = step.diff(&state).diff().apply(&state).expect("valid mutation diff");
    }
    assert_eq!(steps_of(&state), steps_of(&base), "delete-step's inverse must restore the step and its blocks");
}

#[semio_framework_async_macros::async_test]
async fn reorder_step_round_trips() {
    let base = base_snapshot();
    let mutation = FormMutation::ReorderStep(reorder_step::mutation::ReorderStep { id: "s2".into(), to_index: 0 });
    let after = mutation.diff(&base).diff().apply(&base).expect("valid mutation diff");
    assert_eq!(steps_of(&after)[0].id, "s2");

    let undo = mutation.inverse(&base);
    let mut state = after;
    for step in &undo {
        state = step.diff(&state).diff().apply(&state).expect("valid mutation diff");
    }
    assert_eq!(steps_of(&state), steps_of(&base));
}

#[semio_framework_async_macros::async_test]
async fn rename_step_and_change_description_round_trip() {
    let base = base_snapshot();
    let rename = FormMutation::RenameStep(rename_step::mutation::RenameStep { id: "s1".into(), new_title: "Renamed".into() });
    let after_rename = rename.diff(&base).diff().apply(&base).expect("valid mutation diff");
    assert_eq!(steps_of(&after_rename)[0].title, "Renamed");
    let undo = rename.inverse(&base);
    let mut state = after_rename;
    for step in &undo {
        state = step.diff(&state).diff().apply(&state).expect("valid mutation diff");
    }
    assert_eq!(steps_of(&state), steps_of(&base));

    let change = FormMutation::ChangeStepDescription(change_step_description::mutation::ChangeStepDescription { id: "s1".into(), new_description: Some("desc".into()) });
    let after_change = change.diff(&base).diff().apply(&base).expect("valid mutation diff");
    assert_eq!(steps_of(&after_change)[0].description.as_deref(), Some("desc"));
    let undo = change.inverse(&base);
    let mut state = after_change;
    for step in &undo {
        state = step.diff(&state).diff().apply(&state).expect("valid mutation diff");
    }
    assert_eq!(steps_of(&state), steps_of(&base));
}

#[semio_framework_async_macros::async_test]
async fn create_then_delete_block_round_trips() {
    let base = base_snapshot();
    let create = FormMutation::CreateBlock(create_block::mutation::CreateBlock { step_id: "s1".into(), block: sample_block("b1"), index: None });
    let after_create = create.diff(&base).diff().apply(&base).expect("valid mutation diff");
    assert!(steps_of(&after_create)[0].blocks.iter().any(|block| block.id == "b1"));

    let undo = create.inverse(&base);
    assert_eq!(undo, vec![FormMutation::DeleteBlock(delete_block::mutation::DeleteBlock { step_id: "s1".into(), id: "b1".into() })]);
    let mut state = after_create;
    for step in &undo {
        state = step.diff(&state).diff().apply(&state).expect("valid mutation diff");
    }
    assert_eq!(steps_of(&state), steps_of(&base));
}

#[semio_framework_async_macros::async_test]
async fn delete_block_inverse_recreates_block_at_original_index() {
    let mut step1 = sample_step("s1");
    step1.blocks = vec![sample_block("b1"), sample_block("b2")];
    let base = base_snapshot_with_steps(vec![step1, sample_step("s2")]);
    let delete = FormMutation::DeleteBlock(delete_block::mutation::DeleteBlock { step_id: "s1".into(), id: "b1".into() });
    let after_delete = delete.diff(&base).diff().apply(&base).expect("valid mutation diff");
    assert_eq!(steps_of(&after_delete)[0].blocks.len(), 1);

    let undo = delete.inverse(&base);
    match undo.first() {
        Some(FormMutation::CreateBlock(payload)) => assert_eq!(payload.index, Some(0)),
        other => panic!("expected CreateBlock as delete-block's inverse, got {other:?}"),
    }
    let mut state = after_delete;
    for step in &undo {
        state = step.diff(&state).diff().apply(&state).expect("valid mutation diff");
    }
    assert_eq!(steps_of(&state), steps_of(&base));
}

#[semio_framework_async_macros::async_test]
async fn move_block_to_step_round_trips_across_steps() {
    let mut step1 = sample_step("s1");
    step1.blocks = vec![sample_block("b1")];
    let base = base_snapshot_with_steps(vec![step1, sample_step("s2")]);
    let mutation = FormMutation::MoveBlockToStep(move_block_to_step::mutation::MoveBlockToStep { step_id: "s1".into(), block_id: "b1".into(), to_step_id: "s2".into(), index: 0 });
    let after = mutation.diff(&base).diff().apply(&base).expect("valid mutation diff");
    assert!(steps_of(&after)[0].blocks.is_empty());
    assert!(steps_of(&after)[1].blocks.iter().any(|block| block.id == "b1"));

    let undo = mutation.inverse(&base);
    let mut state = after;
    for step in &undo {
        state = step.diff(&state).diff().apply(&state).expect("valid mutation diff");
    }
    assert_eq!(steps_of(&state), steps_of(&base));
}

#[semio_framework_async_macros::async_test]
async fn move_block_to_step_reorders_within_the_same_step() {
    let mut step1 = sample_step("s1");
    step1.blocks = vec![sample_block("b1"), sample_block("b2")];
    let base = base_snapshot_with_steps(vec![step1, sample_step("s2")]);
    let mutation = FormMutation::MoveBlockToStep(move_block_to_step::mutation::MoveBlockToStep { step_id: "s1".into(), block_id: "b2".into(), to_step_id: "s1".into(), index: 0 });
    let after = mutation.diff(&base).diff().apply(&base).expect("valid mutation diff");
    assert_eq!(steps_of(&after)[0].blocks.iter().map(|block| block.id.clone()).collect::<Vec<_>>(), vec!["b2".to_string(), "b1".to_string()]);

    let undo = mutation.inverse(&base);
    let mut state = after;
    for step in &undo {
        state = step.diff(&state).diff().apply(&state).expect("valid mutation diff");
    }
    assert_eq!(steps_of(&state), steps_of(&base));
}

#[semio_framework_async_macros::async_test]
async fn replace_block_round_trips() {
    let mut step1 = sample_step("s1");
    step1.blocks = vec![sample_block("b1")];
    let base = base_snapshot_with_steps(vec![step1, sample_step("s2")]);
    let mut replacement = sample_block("b1");
    replacement.label = "Renamed block".into();
    let mutation = FormMutation::ReplaceBlock(replace_block::mutation::ReplaceBlock { step_id: "s1".into(), block: replacement });
    let after = mutation.diff(&base).diff().apply(&base).expect("valid mutation diff");
    assert_eq!(steps_of(&after)[0].blocks[0].label, "Renamed block");

    let undo = mutation.inverse(&base);
    let mut state = after;
    for step in &undo {
        state = step.diff(&state).diff().apply(&state).expect("valid mutation diff");
    }
    assert_eq!(steps_of(&state), steps_of(&base));
}

#[semio_framework_async_macros::async_test]
async fn change_form_title_round_trips_including_clearing() {
    let base = base_snapshot();
    let mutation = FormMutation::ChangeFormTitle(change_form_title::mutation::ChangeFormTitle { new_title: Some("Renamed".into()) });
    let after = mutation.diff(&base).diff().apply(&base).expect("valid mutation diff");
    assert_eq!(after.title.as_deref(), Some("Renamed"));

    let undo = mutation.inverse(&base);
    let mut state = after.clone();
    for step in &undo {
        state = step.diff(&after).diff().apply(&state).expect("valid mutation diff");
    }
    assert_eq!(state, base);

    let clear = FormMutation::ChangeFormTitle(change_form_title::mutation::ChangeFormTitle { new_title: None });
    assert_eq!(clear.diff(&base).diff().apply(&base).expect("valid mutation diff").title, None);
}

#[semio_framework_async_macros::async_test]
async fn semantic_kinds_cover_every_variant() {
    assert_eq!(FormMutation::kinds().len(), 10);
    let mutation = FormMutation::RenameStep(rename_step::mutation::RenameStep { id: "s1".into(), new_title: "x".into() });
    assert_eq!(mutation.semantics().kind, "rename-step");
    assert_eq!(mutation.semantics().record, "RenamedStep");
}

#[semio_framework_async_macros::async_test]
async fn apply_form_edit_mutation_and_inverse_form_mutation_delegate_to_the_derive() {
    let base = base_snapshot();
    let mutation = FormMutation::ChangeFormTitle(change_form_title::mutation::ChangeFormTitle { new_title: Some("Delegated".into()) });
    assert_eq!(apply_form_edit_mutation(&base, &mutation).expect("valid mutation diff").title.as_deref(), Some("Delegated"));
    assert_eq!(inverse_form_mutation(&base, &mutation), mutation.inverse(&base));
}

//#region 🔖️OutcomeLaws
// 26/08/16 MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS — one law test per verb
// family present in this facet (`assert_missing_target_is_error`/`assert_fatal_never_applies`,
// landed in `📡️spr/🧪️testkit`). `assert_outcome_policy_matrix` is NOT landed under that name
// (only the generic closure-based `assert_policy_matrix` exists) — see this ticket's report.
#[semio_framework_async_macros::async_test]
async fn delete_family_missing_target_is_error() {
    let base = base_snapshot();
    assert_missing_target_is_error(&base, &FormMutation::DeleteStep(delete_step::mutation::DeleteStep { id: "missing".into() })).await;
    assert_missing_target_is_error(&base, &FormMutation::DeleteBlock(delete_block::mutation::DeleteBlock { step_id: "missing".into(), id: "b1".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn rename_family_missing_target_is_error() {
    let base = base_snapshot();
    assert_missing_target_is_error(&base, &FormMutation::RenameStep(rename_step::mutation::RenameStep { id: "missing".into(), new_title: "x".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn reorder_family_missing_target_is_error() {
    let base = base_snapshot();
    assert_missing_target_is_error(&base, &FormMutation::ReorderStep(reorder_step::mutation::ReorderStep { id: "missing".into(), to_index: 0 })).await;
}

#[semio_framework_async_macros::async_test]
async fn change_family_missing_target_is_error() {
    let base = base_snapshot();
    assert_missing_target_is_error(&base, &FormMutation::ChangeStepDescription(change_step_description::mutation::ChangeStepDescription { id: "missing".into(), new_description: None })).await;
}

#[semio_framework_async_macros::async_test]
async fn move_family_missing_target_is_error() {
    let base = base_snapshot();
    assert_missing_target_is_error(&base, &FormMutation::MoveBlockToStep(move_block_to_step::mutation::MoveBlockToStep { step_id: "missing".into(), block_id: "b1".into(), to_step_id: "s2".into(), index: 0 })).await;
}

#[semio_framework_async_macros::async_test]
async fn replace_family_missing_target_is_error() {
    let base = base_snapshot();
    assert_missing_target_is_error(&base, &FormMutation::ReplaceBlock(replace_block::mutation::ReplaceBlock { step_id: "missing".into(), block: sample_block("b1") })).await;
}

#[semio_framework_async_macros::async_test]
async fn create_family_fatal_never_applies() {
    let base = base_snapshot();
    let outcome = FormMutation::CreateStep(create_step::mutation::CreateStep { step: sample_step("s1"), index: None }).diff(&base);
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    assert_fatal_never_applies(&outcome).await;
    let outcome = FormMutation::CreateBlock(create_block::mutation::CreateBlock { step_id: "missing".into(), block: sample_block("b1"), index: None }).diff(&base);
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    assert_fatal_never_applies(&outcome).await;
}
//#endregion 🔖️OutcomeLaws
