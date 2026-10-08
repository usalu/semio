use super::*;
use crate::editor::bim::unit_tests::context::view;
use crate::editor::bim::unit_tests::support::{demo, run};
use crate::editor::bim::modes::edit::windows::section;
use semio_framework_ui_locale::Locale;

/// 🖼️ A plan window 800 by 600 pixels framing the origin at 40 pixels per metre: the press at the pixel of model point (x, y) is `(400 + 40 x, 300 - 40 y)`.
fn press_at(model: (f64, f64), shift: bool) -> CanvasPointerDown {
    CanvasPointerDown { x: 400.0 + 40.0 * model.0, y: 300.0 - 40.0 * model.1, width: 800.0, height: 600.0, shift, ctrl: false, meta: false }
}

fn window(kind: &str, utility: &str) -> BimDispatchCtx {
    let view = view(Locale::En, &[("window", kind)], Some("window"));
    let mut ctx = BimDispatchCtx::new(Vec::new(), Vec::new(), Some(&view), None, None);
    ctx.plan.storey = "st-ground".into();
    ctx.utility = utility.into();
    ctx
}

fn press(kind: &str, utility: &str, payload: &CanvasPointerDown) -> Emit<ModelMutation, NoConfigMutation> {
    let snapshot = demo();
    let mut ctx = window(kind, utility);
    run(&snapshot, |doc, cfg| handle(payload, doc, cfg, &mut ctx)).expect("a press never faults")
}

#[semio_framework_async_macros::async_test]
async fn the_modifiers_map_to_the_framework_merge_modes() {
    assert_eq!([merge_mode(false, false, false), merge_mode(true, false, false), merge_mode(false, true, false), merge_mode(false, false, true), merge_mode(true, true, false)], ["replace", "additive", "subtractive", "subtractive", "invertive"]);
}

#[semio_framework_async_macros::async_test]
async fn a_press_on_a_wall_selects_it_and_a_press_on_nothing_clears_the_selection() {
    let hit = press(plan::WINDOW_KIND_ID, DEFAULT_UTILITY, &press_at((4.0, 0.0), false));
    assert_eq!(hit.effects.len(), 1, "one select request");
    assert!(hit.artifact_mutations.is_empty(), "selection is never a mutation");
    let miss = press(plan::WINDOW_KIND_ID, DEFAULT_UTILITY, &press_at((4.0, 3.0), false));
    assert_eq!(miss.effects.len(), 1, "a replacing press on nothing clears the selection");
    let shifted_miss = press(plan::WINDOW_KIND_ID, DEFAULT_UTILITY, &press_at((4.0, 3.0), true));
    assert!(shifted_miss.effects.is_empty(), "an additive press on nothing changes nothing");
}

#[semio_framework_async_macros::async_test]
async fn only_the_plan_window_with_the_select_utility_owns_the_press() {
    assert!(press(plan::WINDOW_KIND_ID, "wall", &press_at((4.0, 0.0), false)).effects.is_empty(), "another armed utility owns the press");
    assert!(press(section::WINDOW_KIND_ID, DEFAULT_UTILITY, &press_at((4.0, 0.0), false)).effects.is_empty(), "the section window picks nothing");
}
