use super::*;
use crate::editor::bim::entities::kind_of;
use crate::editor::bim::unit_tests::support::{applied, ctx, demo, run};
use crate::TopConstraint;
use semio_framework_plugin::ArtifactEditor;

fn furnished() -> ModelSnapshot {
    let mut snapshot = demo();
    for (kind, id, parent) in [("slab-type", "sl-1", ""), ("slab", "slab-1", "st-ground"), ("roof-type", "rf-1", ""), ("roof", "roof-1", "st-ground")] {
        let create = kind_of(kind).and_then(|row| row.create).expect("a creatable kind");
        let mutation = create(&snapshot, id, parent, "Sample").expect("creates");
        snapshot = crate::mutations::apply_model_mutation(&snapshot, &mutation).expect("applies");
    }
    snapshot
}

fn attach(snapshot: &ModelSnapshot, ids: &[&str], target: &str, selected: &[&str]) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let mut ctx = ctx(selected);
    run(snapshot, |doc, cfg| handle(&AttachWalls { ids: ids.iter().map(|id| id.to_string()).collect(), target: target.into() }, doc, cfg, &mut ctx))
}

#[semio_framework_async_macros::async_test]
async fn the_selected_walls_follow_the_selected_roof_in_one_emit() {
    let snapshot = furnished();
    let emit = attach(&snapshot, &[], "", &["w-south", "w-east", "roof-1"]).expect("attaches");
    assert!(matches!(emit.artifact_mutations.as_slice(), [ModelMutation::SetWallTop(_), ModelMutation::SetWallTop(_)]));
    let after = applied(&snapshot, &emit);
    for wall in ["w-south", "w-east"] {
        assert_eq!(after.walls[wall].top, TopConstraint::Roof { roof: "roof-1".into(), offset: 0.0 }, "{wall}");
    }
    assert_eq!(after.walls["w-north"].top, snapshot.walls["w-north"].top, "an unselected wall keeps its top");
}

#[semio_framework_async_macros::async_test]
async fn an_explicit_target_and_ids_win_over_the_selection_and_a_slab_or_ceiling_serves_as_well() {
    let snapshot = furnished();
    let emit = attach(&snapshot, &["w-north"], "slab-1", &["roof-1"]).expect("attaches");
    let after = applied(&snapshot, &emit);
    assert_eq!(after.walls["w-north"].top, TopConstraint::Slab { slab: "slab-1".into(), offset: 0.0 });
    assert_eq!(after.walls["w-south"].top, snapshot.walls["w-south"].top);
}

#[semio_framework_async_macros::async_test]
async fn the_ids_may_hold_the_surface_too() {
    let snapshot = furnished();
    let emit = attach(&snapshot, &["w-south", "roof-1"], "", &[]).expect("attaches");
    assert_eq!(applied(&snapshot, &emit).walls["w-south"].top, TopConstraint::Roof { roof: "roof-1".into(), offset: 0.0 });
}

#[semio_framework_async_macros::async_test]
async fn without_a_wall_or_without_a_surface_the_command_is_refused_with_its_own_code() {
    let snapshot = furnished();
    let code = |result: Result<Emit<ModelMutation, NoConfigMutation>, Fault>| result.err().map(|fault| fault.code.0);
    assert_eq!(code(attach(&snapshot, &[], "", &["roof-1"])), Some("bim.attach.wall-missing".to_string()));
    assert_eq!(code(attach(&snapshot, &[], "", &[])), Some("bim.attach.wall-missing".to_string()));
    assert_eq!(code(attach(&snapshot, &[], "", &["w-south"])), Some("bim.attach.target-missing".to_string()));
    assert_eq!(code(attach(&snapshot, &["w-south"], "w-east", &[])), Some("bim.attach.target-missing".to_string()), "a wall is no surface");
}

#[semio_framework_async_macros::async_test]
async fn the_command_is_reachable_by_its_action_id_and_the_shift_r_key() {
    let command = crate::editor::bim::BimModelApp::command_from_action("attachWalls", None).expect("decodes without arguments");
    assert!(crate::editor::bim::BIM_TOOL_IDS.contains(&command.command_id()));
    assert!(crate::editor::bim::all_keybindings().contains(&("shift+r", "attachWalls")));
}
