use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
use semio_framework_plugin::Effect;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_s_artifact_bim_model_coordination::editor::bim::commands::arm_utility::{handle, ArmWall, ArmSelect, ARMED_UTILITIES};
use semio_s_artifact_bim_model_bim::editor::bim::fixture::context::view;
use semio_s_artifact_bim_model_bim::editor::bim::fixture::support::{demo, run};
use semio_framework_ui_locale::Locale;

#[semio_framework_async_macros::async_test]
async fn arming_a_utility_asks_the_host_to_arm_it_in_the_addressed_window_and_writes_nothing() {
    let snapshot = demo();
    let view = view(Locale::En, &[("bim-plan", semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan::WINDOW_KIND_ID)], Some("bim-plan"));
    let mut ctx = BimDispatchCtx::new(Vec::new(), Vec::new(), Some(&view), None, None);
    let emit = run(&snapshot, |doc, cfg| handle(&ArmWall {}, doc, cfg, &mut ctx)).expect("arms");
    assert!(emit.artifact_mutations.is_empty());
    assert!(matches!(emit.effects.as_slice(), [Effect::SetActiveUtility { window_id, utility_id }] if window_id == "bim-plan" && utility_id == "wall"));
}

#[semio_framework_async_macros::async_test]
async fn without_an_addressed_window_nothing_is_armed() {
    let snapshot = demo();
    let mut ctx = BimDispatchCtx::default();
    let emit = run(&snapshot, |doc, cfg| handle(&ArmSelect {}, doc, cfg, &mut ctx)).expect("nothing to arm");
    assert!(emit.effects.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn every_arming_command_arms_a_utility_of_the_registry() {
    assert_eq!(ARMED_UTILITIES.len(), semio_s_artifact_bim_model_kit::editor::bim::utilities::UTILITIES.iter().filter(|row| row.arm.is_some()).count(), "every utility with a hotkey has its arming command and the other way round");
    for utility in ARMED_UTILITIES {
        assert!(semio_s_artifact_bim_model_kit::editor::bim::utilities::UTILITIES.iter().any(|row| row.id == *utility), "{utility}");
    }
}
