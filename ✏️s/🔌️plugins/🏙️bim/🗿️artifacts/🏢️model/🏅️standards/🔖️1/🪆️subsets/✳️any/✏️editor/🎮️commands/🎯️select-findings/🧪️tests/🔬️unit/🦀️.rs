use semio_s_artifact_bim_model_kit::editor::bim::kit::fault;
use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
use semio_framework_plugin::{Emit, Fault, NoConfigMutation};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_entities::editor::bim::entities::kind_holding;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_interaction::editor::bim::interaction::BIM_ELEMENT_DOMAIN;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_kit::editor::bim::kit::select_effect;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, NoConfig};
#[allow(unused_imports)]
use value_derive::{FromValue, ToValue};
use semio_s_artifact_bim_model_move_storey::editor::bim::commands::select_findings::{handle, SelectFindings};
use semio_s_artifact_bim_model_bim::editor::bim::fixture::support::{demo, run};
use semio_framework_plugin::Effect;

fn select(ids: &[&str]) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let snapshot = demo();
    let mut ctx = BimDispatchCtx::default();
    run(&snapshot, |doc, cfg| handle(&SelectFindings { ids: ids.iter().map(|id| id.to_string()).collect() }, doc, cfg, &mut ctx))
}

#[semio_framework_async_macros::async_test]
async fn a_finding_selects_its_elements_with_their_kinds_and_writes_nothing() {
    let emit = select(&["w-south", "st-ground", "w-south"]).expect("selects");
    assert!(emit.artifact_mutations.is_empty());
    let [Effect::ReplayShellCommand { args: Some(args), .. }] = emit.effects.as_slice() else { panic!("one selection effect: {:?}", emit.effects) };
    let targets = args.get("targets").and_then(|value| value.as_str()).expect("targets");
    assert!(targets.contains("w-south") && targets.contains("\"wall\"") && targets.contains("st-ground") && targets.contains("\"storey\""), "{targets}");
    assert_eq!(targets.matches("w-south").count(), 1, "an element named twice is selected once");
    assert_eq!(args.get("domainId").and_then(|value| value.as_str()), Some("elements"));
    assert_eq!(args.get("merge").and_then(|value| value.as_str()), Some("replace"));
}

#[semio_framework_async_macros::async_test]
async fn ids_no_collection_holds_are_skipped_and_none_left_is_refused() {
    let emit = select(&["w-south", "gone"]).expect("the existing one is selected");
    assert!(matches!(emit.effects.as_slice(), [Effect::ReplayShellCommand { args: Some(args), .. }] if !args.get("targets").and_then(|value| value.as_str()).unwrap_or_default().contains("gone")));
    assert_eq!(select(&["gone"]).err().map(|fault| fault.code.0), Some("bim.diagnostic.target-missing".to_string()));
    assert_eq!(select(&[]).err().map(|fault| fault.code.0), Some("bim.diagnostic.target-missing".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn library_entries_are_not_selectable_as_findings_of_the_elements_domain() {
    assert!(select(&["wt-300"]).is_err());
}
