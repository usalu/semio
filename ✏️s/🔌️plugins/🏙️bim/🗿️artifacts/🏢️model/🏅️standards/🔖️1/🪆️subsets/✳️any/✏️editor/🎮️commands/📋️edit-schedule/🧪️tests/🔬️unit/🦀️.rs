use semio_s_artifact_bim_model_kit::editor::bim::kit::fault;
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_framework_plugin::{Emit, Fault, NoConfigMutation};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_ramp_runs::editor::bim::modes::edit::windows::schedule::edit::{apply, Edit};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, NoConfig};
#[allow(unused_imports)]
use value_derive::{FromValue, ToValue};
use semio_s_artifact_bim_model_coordination::editor::bim::commands::edit_schedule::{handle, EditSchedule};
use semio_s_artifact_bim_model_bim::editor::bim::fixture::support::{applied, ctx, demo, run};
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::schedule_kit::preset;

fn with_schedule() -> ModelSnapshot {
    let mut snapshot = demo();
    snapshot.schedules.insert("sch-walls".into(), preset("wall", "Walls").expect("the wall preset"));
    snapshot
}

fn edit(part: &str, op: &str, key: &str, value: &str) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let snapshot = with_schedule();
    let mut ctx = ctx(&[]);
    run(&snapshot, |doc, cfg| handle(&EditSchedule { id: "sch-walls".into(), part: part.into(), op: op.into(), key: key.into(), value: value.into() }, doc, cfg, &mut ctx))
}

#[semio_framework_async_macros::async_test]
async fn a_column_is_added_through_one_set_schedule_mutation() {
    let emit = edit("column", "add", "mass", "").expect("adds");
    assert!(matches!(emit.artifact_mutations.as_slice(), [ModelMutation::SetSchedule(_)]));
    let after = applied(&with_schedule(), &emit);
    assert_eq!(after.schedules["sch-walls"].columns.last().map(|column| column.key.token()), Some("mass".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn the_name_and_the_scope_are_set() {
    let renamed = applied(&with_schedule(), &edit("name", "set", "", "Wall list").expect("renames"));
    assert_eq!(renamed.schedules["sch-walls"].name, "Wall list");
    let storey = with_schedule().storeys.keys().next().cloned().expect("a storey");
    let scoped = applied(&with_schedule(), &edit("storey", "toggle", &storey, "").expect("scopes"));
    assert_eq!(scoped.schedules["sch-walls"].storeys, vec![storey]);
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_part_or_key_is_refused() {
    assert_eq!(edit("shape", "set", "", "").err().map(|fault| fault.code.0), Some("bim.schedule.part-unknown".to_string()));
    assert_eq!(edit("column", "add", "no_such_field", "").err().map(|fault| fault.code.0), Some("bim.schedule.key-unknown".to_string()));
}
