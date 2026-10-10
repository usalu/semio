use super::*;
use crate::editor::bim::entities::kind_of;
use crate::editor::bim::unit_tests::support::{applied, ctx, demo, run};
use crate::SpaceConditions;

fn with_space(snapshot: ModelSnapshot, id: &str) -> ModelSnapshot {
    let create = kind_of("space").and_then(|row| row.create).expect("a creatable kind");
    let mutation = create(&snapshot, id, "st-ground", "Sample").expect("creates");
    crate::mutations::apply_model_mutation(&snapshot, &mutation).expect("applies")
}

fn stated(snapshot: &ModelSnapshot, id: &str, record: &SpaceConditions) -> ModelSnapshot {
    let mutation = ModelMutation::SetSpaceConditions(SetSpaceConditions::stating(id, record));
    crate::mutations::apply_model_mutation(snapshot, &mutation).expect("applies")
}

fn office() -> SpaceConditions {
    SpaceConditions { occupancy: Some("Office".into()), heating_setpoint: Some(21.0), cooling_setpoint: Some(26.0), lighting_power_density: Some(9.0), schedule: Some("Office 08-18".into()), ..SpaceConditions::empty() }
}

fn rooms() -> ModelSnapshot {
    let snapshot = ["sp-a", "sp-b", "sp-c"].into_iter().fold(demo(), with_space);
    stated(&snapshot, "sp-a", &office())
}

fn apply_to(snapshot: &ModelSnapshot, source: &str, ids: &[&str], selected: &[&str]) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let mut ctx = ctx(selected);
    run(snapshot, |doc, cfg| handle(&ApplyConditions { source: source.into(), ids: ids.iter().map(|id| id.to_string()).collect() }, doc, cfg, &mut ctx))
}

fn clear_of(snapshot: &ModelSnapshot, ids: &[&str], selected: &[&str]) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let mut ctx = ctx(selected);
    run(snapshot, |doc, cfg| handle(&ClearConditions { ids: ids.iter().map(|id| id.to_string()).collect() }, doc, cfg, &mut ctx))
}

fn code(result: Result<Emit<ModelMutation, NoConfigMutation>, Fault>) -> Option<String> {
    result.err().map(|fault| fault.code.0)
}

#[semio_framework_async_macros::async_test]
async fn the_first_selected_space_with_conditions_is_copied_to_the_others_in_one_gesture() {
    let snapshot = rooms();
    let emit = apply_to(&snapshot, "", &[], &["sp-b", "sp-a", "sp-c"]).expect("copies");
    assert_eq!(emit.artifact_mutations.len(), 2, "one set-space-conditions per target, the source needs none");
    assert!(emit.artifact_mutations.iter().all(|mutation| matches!(mutation, ModelMutation::SetSpaceConditions(_))));
    let after = applied(&snapshot, &emit);
    assert_eq!((&after.space_conditions["sp-b"], &after.space_conditions["sp-c"]), (&office(), &office()));
    assert_eq!(after.space_conditions["sp-a"], office());
}

#[semio_framework_async_macros::async_test]
async fn a_target_ends_up_with_exactly_the_conditions_of_the_source_unstated_fields_included() {
    let snapshot = stated(&rooms(), "sp-b", &SpaceConditions { occupancy: Some("Storage".into()), ventilation_rate: Some(0.5), heating_setpoint: Some(15.0), ..SpaceConditions::empty() });
    let after = applied(&snapshot, &apply_to(&snapshot, "sp-a", &["sp-b"], &[]).expect("copies"));
    assert_eq!(after.space_conditions["sp-b"], office(), "the occupancy and the ventilation of the target are cleared, not merged");
}

#[semio_framework_async_macros::async_test]
async fn a_named_source_and_explicit_targets_win_over_the_selection() {
    let snapshot = stated(&rooms(), "sp-b", &SpaceConditions { heating_setpoint: Some(18.0), ..SpaceConditions::empty() });
    let after = applied(&snapshot, &apply_to(&snapshot, "sp-b", &["sp-c", "sp-b", "sp-ghost"], &["sp-a"]).expect("copies"));
    assert_eq!(after.space_conditions["sp-c"], SpaceConditions { heating_setpoint: Some(18.0), ..SpaceConditions::empty() });
    assert_eq!(after.space_conditions["sp-a"], office(), "the selection is not touched when ids are given");
}

#[semio_framework_async_macros::async_test]
async fn a_target_that_already_has_the_conditions_changes_nothing_and_is_no_refusal() {
    let snapshot = stated(&rooms(), "sp-b", &office());
    let emit = apply_to(&snapshot, "", &[], &["sp-a", "sp-b"]).expect("nothing to do is fine");
    assert!(emit.artifact_mutations.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn copying_without_a_source_or_without_a_target_is_refused_whole() {
    let snapshot = rooms();
    assert_eq!(code(apply_to(&demo(), "", &[], &[])), Some("bim.conditions.source-missing".to_string()), "nothing selected");
    assert_eq!(code(apply_to(&snapshot, "", &[], &["sp-b", "sp-c"])), Some("bim.conditions.source-missing".to_string()), "no selected space states conditions");
    assert_eq!(code(apply_to(&snapshot, "sp-c", &["sp-b"], &[])), Some("bim.conditions.source-missing".to_string()), "the named source states none");
    assert_eq!(code(apply_to(&snapshot, "", &[], &["sp-a"])), Some("bim.conditions.target-missing".to_string()), "the source alone has no one to copy to");
    assert_eq!(code(apply_to(&snapshot, "", &[], &["sp-a", "w-south"])), Some("bim.conditions.target-missing".to_string()), "a wall is no target");
}

#[semio_framework_async_macros::async_test]
async fn clearing_removes_the_records_that_exist_and_skips_spaces_without() {
    let snapshot = stated(&rooms(), "sp-b", &office());
    let emit = clear_of(&snapshot, &[], &["sp-a", "sp-b", "sp-c"]).expect("clears");
    assert_eq!(emit.artifact_mutations.len(), 2);
    assert!(emit.artifact_mutations.iter().all(|mutation| matches!(mutation, ModelMutation::RemoveSpaceConditions(_))));
    assert!(applied(&snapshot, &emit).space_conditions.is_empty());
    let named = applied(&snapshot, &clear_of(&snapshot, &["sp-b"], &["sp-a"]).expect("clears the named one"));
    assert_eq!(named.space_conditions.keys().collect::<Vec<_>>(), ["sp-a"]);
}

#[semio_framework_async_macros::async_test]
async fn clearing_nothing_is_quiet_and_clearing_no_space_is_refused() {
    assert!(clear_of(&demo(), &[], &["w-south"]).err().is_some_and(|fault| fault.code.0 == "bim.conditions.target-missing"));
    let snapshot = with_space(demo(), "sp-a");
    assert!(clear_of(&snapshot, &["sp-a"], &[]).expect("quiet").artifact_mutations.is_empty());
}
