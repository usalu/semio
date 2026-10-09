use super::*;
use crate::editor::bim::modes::edit::windows::plan;
use crate::editor::bim::unit_tests::context::view;
use crate::editor::bim::unit_tests::support::{demo, run};
use semio_framework_ui_locale::Locale;

#[semio_framework_async_macros::async_test]
async fn arming_a_utility_asks_the_host_to_arm_it_in_the_addressed_window_and_writes_nothing() {
    let snapshot = demo();
    let view = view(Locale::En, &[("bim-plan", plan::WINDOW_KIND_ID)], Some("bim-plan"));
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
    assert_eq!(ARMED_UTILITIES.len(), crate::editor::bim::utilities::UTILITIES.iter().filter(|row| row.arm.is_some()).count(), "every utility with a hotkey has its arming command and the other way round");
    for utility in ARMED_UTILITIES {
        assert!(crate::editor::bim::utilities::UTILITIES.iter().any(|row| row.id == *utility), "{utility}");
    }
}
