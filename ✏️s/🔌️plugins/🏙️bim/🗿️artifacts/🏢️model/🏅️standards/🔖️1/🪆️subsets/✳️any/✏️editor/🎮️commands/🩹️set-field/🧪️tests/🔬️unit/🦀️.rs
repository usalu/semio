use semio_s_artifact_bim_model_kit::editor::bim::kit::fault;
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
use semio_framework_plugin::{Emit, Fault, NoConfigMutation};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_entities::editor::bim::entities::{fields_of, kind_holding, PROJECT_FIELDS, PROJECT_ID};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_viewer::editor::bim::entities::row_types::FieldRow;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, NoConfig};
#[allow(unused_imports)]
use value_derive::{FromValue, ToValue};
use semio_s_artifact_bim_model_coordination::editor::bim::commands::set_field::{handle, SetField};
use semio_s_artifact_bim_model_bim::editor::bim::fixture::support::{applied, ctx, demo, run};

fn set(ids: &[&str], field: &str, value: &str, selected: &[&str]) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let snapshot = demo();
    let mut ctx = ctx(selected);
    run(&snapshot, |doc, cfg| handle(&SetField { ids: ids.iter().map(|id| id.to_string()).collect(), field: field.into(), value: value.into() }, doc, cfg, &mut ctx))
}

fn code(result: Result<Emit<ModelMutation, NoConfigMutation>, Fault>) -> Option<String> {
    result.err().map(|fault| fault.code.0)
}

#[semio_framework_async_macros::async_test]
async fn a_storey_height_edit_becomes_its_set_mutation() {
    let emit = set(&["st-ground"], "height", "3.5", &[]).expect("sets");
    assert!(matches!(emit.artifact_mutations.as_slice(), [ModelMutation::SetStoreyHeight(_)]));
    assert!((applied(&demo(), &emit).storeys["st-ground"].height - 3.5).abs() < 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn several_entities_are_set_at_once_and_the_selection_is_the_default_target() {
    let emit = set(&["st-ground", "st-first"], "height", "4", &[]).expect("sets");
    assert_eq!(emit.artifact_mutations.len(), 2);
    let selected = set(&[], "name", "Renamed", &["st-first"]).expect("sets the selection");
    assert_eq!(applied(&demo(), &selected).storeys["st-first"].name, "Renamed");
}

#[semio_framework_async_macros::async_test]
async fn an_invalid_value_unknown_field_or_read_only_parameter_is_refused_with_its_own_code() {
    assert_eq!(code(set(&["st-ground"], "height", "tall", &[])), Some("bim.set.value-invalid".to_string()));
    assert_eq!(code(set(&["st-ground"], "colour", "red", &[])), Some("bim.set.field-unknown".to_string()));
    assert_eq!(code(set(&["w-south"], "phase", "Old", &[])), Some("bim.set.read-only".to_string()));
    assert_eq!(code(set(&["w-south"], "axis", "0,0", &[])), Some("bim.set.value-invalid".to_string()));
    assert_eq!(code(set(&["nowhere"], "name", "X", &[])), Some("bim.set.target-missing".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn nothing_to_set_is_a_no_op() {
    assert!(set(&[], "name", "X", &[]).expect("a no-op").artifact_mutations.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn a_wall_axis_edit_becomes_its_set_mutation() {
    let emit = set(&["w-south"], "axis", "0, 0 → 5, 0", &[]).expect("sets");
    assert!(matches!(emit.artifact_mutations.as_slice(), [ModelMutation::SetWallAxis(_)]));
}
