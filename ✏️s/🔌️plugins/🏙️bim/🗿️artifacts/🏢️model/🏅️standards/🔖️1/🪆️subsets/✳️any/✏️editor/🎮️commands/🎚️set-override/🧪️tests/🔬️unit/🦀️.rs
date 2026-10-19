use semio_s_artifact_bim_model_kit::editor::bim::kit::fault;
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_framework_plugin::{Emit, Fault, NoConfigMutation};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_mut_any_156::standards::v1::subsets::any::schema::mutations::remove_component_override::RemoveComponentOverride;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_mut_any_156::standards::v1::subsets::any::schema::mutations::set_component_override::SetComponentOverride;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::formula;
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, NoConfig};
#[allow(unused_imports)]
use value_derive::{FromValue, ToValue};
use semio_s_artifact_bim_model_move_storey::editor::bim::commands::set_override::{handle, SetOverride};
use semio_s_artifact_bim_model_bim::editor::bim::fixture::placed;
use semio_s_artifact_bim_model_bim::editor::bim::fixture::support::{applied, ctx, run};
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ComponentOverride;

fn call(snapshot: &ModelSnapshot, name: &str, value: &str) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let mut ctx = ctx(&[]);
    run(snapshot, |doc, cfg| handle(&SetOverride { component: "c-table".into(), name: name.into(), value: value.into() }, doc, cfg, &mut ctx))
}

fn overridden() -> ModelSnapshot {
    let mut snapshot = placed();
    snapshot.component_overrides.insert("c-table.width".into(), ComponentOverride { component: "c-table".into(), name: "width".into(), value: "2 m".into() });
    snapshot
}

fn code(result: Result<Emit<ModelMutation, NoConfigMutation>, Fault>) -> Option<String> {
    result.err().map(|fault| fault.code.0)
}

#[semio_framework_async_macros::async_test]
async fn a_formula_becomes_one_canonical_set_component_override_that_stands() {
    let snapshot = placed();
    let emit = call(&snapshot, "width", "depth*2").expect("sets");
    assert!(matches!(emit.artifact_mutations.as_slice(), [ModelMutation::SetComponentOverride(leaf)] if leaf.component == "c-table" && leaf.name == "width" && leaf.value == "depth * 2"));
    assert_eq!(applied(&snapshot, &emit).component_overrides["c-table.width"].value, "depth * 2");
}

#[semio_framework_async_macros::async_test]
async fn an_empty_formula_or_the_formula_of_the_family_takes_the_override_away() {
    let snapshot = overridden();
    for text in ["", "  ", "1.6 m", "1.6m"] {
        let emit = call(&snapshot, "width", text).expect("resets");
        assert!(matches!(emit.artifact_mutations.as_slice(), [ModelMutation::RemoveComponentOverride(leaf)] if leaf.component == "c-table" && leaf.name == "width"), "{text:?}");
        assert!(!applied(&snapshot, &emit).component_overrides.contains_key("c-table.width"));
    }
}

#[semio_framework_async_macros::async_test]
async fn what_changes_nothing_writes_nothing() {
    let snapshot = overridden();
    assert!(call(&snapshot, "width", "2m").expect("same").artifact_mutations.is_empty(), "the override already holds this formula");
    assert!(call(&placed(), "width", "").expect("nothing to reset").artifact_mutations.is_empty());
    assert!(call(&placed(), "width", "1.6 m").expect("the family's own").artifact_mutations.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn a_missing_component_a_missing_parameter_and_text_that_does_not_parse_are_refused_with_their_codes() {
    let snapshot = placed();
    let mut ctx = ctx(&[]);
    let ghost = run(&snapshot, |doc, cfg| handle(&SetOverride { component: "c-ghost".into(), name: "width".into(), value: "1 m".into() }, doc, cfg, &mut ctx));
    assert_eq!(code(ghost).as_deref(), Some("bim.override.component-missing"));
    assert_eq!(code(call(&snapshot, "weight", "1 m")).as_deref(), Some("bim.override.parameter-missing"));
    assert_eq!(code(call(&snapshot, "width", "1 m +")).as_deref(), Some("bim.family.formula-invalid"));
}

#[semio_framework_async_macros::async_test]
async fn a_formula_naming_an_unknown_parameter_parses_here_and_is_refused_by_the_mutation() {
    let snapshot = placed();
    let emit = call(&snapshot, "width", "nothing * 2").expect("it parses");
    let [mutation] = emit.artifact_mutations.as_slice() else { panic!("one mutation") };
    assert!(semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::apply_model_mutation(&snapshot, mutation).is_err(), "the mutation knows the family and refuses the unknown name");
}
