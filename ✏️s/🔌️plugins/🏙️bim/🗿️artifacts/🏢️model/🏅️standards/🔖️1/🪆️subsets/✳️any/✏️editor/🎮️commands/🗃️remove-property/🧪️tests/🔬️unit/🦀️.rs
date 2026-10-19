use semio_s_artifact_bim_model_kit::editor::bim::kit::fault;
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_framework_plugin::{Emit, Fault, NoConfigMutation};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_move_storey::editor::bim::commands::set_property::elements_of;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, NoConfig};
#[allow(unused_imports)]
use value_derive::{FromValue, ToValue};
use semio_s_artifact_bim_model_move_storey::editor::bim::commands::remove_property::{handle, RemoveProperty};
use semio_s_artifact_bim_model_bim::editor::bim::fixture::support::{applied, ctx, demo, run};
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::PropertyValue;

fn described() -> ModelSnapshot {
    let mut snapshot = demo();
    snapshot.properties.insert("w-south".into(), std::collections::BTreeMap::from([("Pset".to_string(), std::collections::BTreeMap::from([("Load".to_string(), PropertyValue::Real { value: 2.5 }), ("Fire".to_string(), PropertyValue::Integer { value: 90 })]))]));
    snapshot
}

fn remove(snapshot: &ModelSnapshot, ids: &[&str], pset: &str, property: &str, selected: &[&str]) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let mut ctx = ctx(selected);
    run(snapshot, |doc, cfg| handle(&RemoveProperty { ids: ids.iter().map(|id| id.to_string()).collect(), pset: pset.into(), property: property.into() }, doc, cfg, &mut ctx))
}

#[semio_framework_async_macros::async_test]
async fn removing_a_property_leaves_its_siblings_and_the_last_one_drops_the_entry() {
    let snapshot = described();
    let emit = remove(&snapshot, &["w-south"], "Pset", "Load", &[]).expect("removes");
    assert!(matches!(emit.artifact_mutations.as_slice(), [ModelMutation::RemoveElementProperty(_)]));
    let after = applied(&snapshot, &emit);
    assert!(after.properties["w-south"]["Pset"].contains_key("Fire") && !after.properties["w-south"]["Pset"].contains_key("Load"));
    let last = remove(&after, &[], "Pset", "Fire", &["w-south"]).expect("removes the selected element's last property");
    assert!(!applied(&after, &last).properties.contains_key("w-south"));
}

#[semio_framework_async_macros::async_test]
async fn elements_without_the_property_are_skipped_and_none_is_refused() {
    let snapshot = described();
    let emit = remove(&snapshot, &["w-south", "w-east"], "Pset", "Load", &[]).expect("removes from the one that has it");
    assert_eq!(emit.artifact_mutations.len(), 1);
    assert_eq!(remove(&snapshot, &["w-east"], "Pset", "Load", &[]).err().map(|fault| fault.code.0), Some("bim.property.missing".to_string()));
}
