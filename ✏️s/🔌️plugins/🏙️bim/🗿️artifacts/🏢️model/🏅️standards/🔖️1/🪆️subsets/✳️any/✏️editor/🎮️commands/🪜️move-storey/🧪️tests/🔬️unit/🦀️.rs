use super::*;
use crate::editor::bim::unit_tests::support::{applied, ctx, demo, run};

fn up(ids: &[&str], selected: &[&str]) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let snapshot = demo();
    let mut ctx = ctx(selected);
    run(&snapshot, |doc, cfg| handle(&StoreyUp { ids: ids.iter().map(|id| id.to_string()).collect() }, doc, cfg, &mut ctx))
}

fn down(ids: &[&str], selected: &[&str]) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let snapshot = demo();
    let mut ctx = ctx(selected);
    run(&snapshot, |doc, cfg| handle(&StoreyDown { ids: ids.iter().map(|id| id.to_string()).collect() }, doc, cfg, &mut ctx))
}

#[semio_framework_async_macros::async_test]
async fn moving_up_stands_the_walls_on_the_storey_above_in_one_emit() {
    let emit = up(&["w-south", "w-east"], &[]).expect("moves");
    assert!(matches!(emit.artifact_mutations.as_slice(), [ModelMutation::SetElementStorey(first), ModelMutation::SetElementStorey(second)] if first.storey == "st-first" && second.storey == "st-first"));
    let after = applied(&demo(), &emit);
    assert_eq!((after.walls["w-south"].storey.as_str(), after.walls["w-east"].storey.as_str(), after.walls["w-north"].storey.as_str()), ("st-first", "st-first", "st-ground"));
}

#[semio_framework_async_macros::async_test]
async fn without_ids_the_selection_moves_and_what_stands_on_no_storey_is_skipped() {
    let emit = up(&[], &["w-south", "st-ground"]).expect("moves the wall");
    assert!(matches!(emit.artifact_mutations.as_slice(), [ModelMutation::SetElementStorey(step)] if step.id == "w-south" && step.storey == "st-first"));
}

#[semio_framework_async_macros::async_test]
async fn down_is_the_mirror_of_up_and_a_missing_neighbour_refuses_the_whole_command() {
    let moved = applied(&demo(), &up(&["w-south"], &[]).expect("up"));
    let ctx = &mut ctx(&[]);
    let back = run(&moved, |doc, cfg| handle(&StoreyDown { ids: vec!["w-south".into()] }, doc, cfg, ctx)).expect("down");
    assert!(matches!(back.artifact_mutations.as_slice(), [ModelMutation::SetElementStorey(step)] if step.storey == "st-ground"));
    assert_eq!(down(&["w-south"], &[]).err().map(|fault| fault.code.0), Some("bim.storey.no-neighbour".to_string()));
    let refused = run(&moved, |doc, cfg| handle(&StoreyUp { ids: vec!["w-east".into(), "w-south".into()] }, doc, cfg, ctx));
    assert_eq!(refused.err().map(|fault| fault.code.0), Some("bim.storey.no-neighbour".to_string()), "w-south has no storey above: w-east does not move either");
}

#[semio_framework_async_macros::async_test]
async fn no_element_that_stands_on_a_storey_is_refused_with_its_own_code() {
    for (ids, selected) in [(&["st-ground"][..], &[][..]), (&[][..], &[][..])] {
        assert_eq!(up(ids, selected).err().map(|fault| fault.code.0), Some("bim.storey.target-missing".to_string()));
    }
}

#[semio_framework_async_macros::async_test]
async fn the_neighbour_follows_the_level_order_of_the_building() {
    let snapshot = demo();
    assert_eq!(neighbour(&snapshot, "st-ground", 1).as_deref(), Some("st-first"));
    assert_eq!(neighbour(&snapshot, "st-first", -1).as_deref(), Some("st-ground"));
    assert_eq!(neighbour(&snapshot, "st-first", 1), None);
    assert_eq!(neighbour(&snapshot, "st-ground", -1), None);
    assert_eq!(neighbour(&snapshot, "st-missing", 1), None);
    assert_eq!(storey_of(&snapshot, "w-south").as_deref(), Some("st-ground"));
    assert_eq!(storey_of(&snapshot, "st-ground"), None);
}
