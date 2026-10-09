use super::*;
use crate::editor::bim::unit_tests::context::view;
use crate::editor::bim::unit_tests::support::{demo, run};
use semio_framework_ui_locale::Locale;

fn window(kind: &str) -> BimDispatchCtx {
    let view = view(Locale::En, &[("window", kind)], Some("window"));
    BimDispatchCtx::new(Vec::new(), Vec::new(), Some(&view), None, None)
}

fn set(kind: &str, field: &str, value: &str, pressed: Option<bool>) -> (Result<Emit<ModelMutation, NoConfigMutation>, Fault>, BimDispatchCtx) {
    let snapshot = demo();
    let mut ctx = window(kind);
    let result = run(&snapshot, |doc, cfg| handle(&SetView { field: field.into(), value: value.into(), pressed }, doc, cfg, &mut ctx));
    (result, ctx)
}

fn code(result: Result<Emit<ModelMutation, NoConfigMutation>, Fault>) -> Option<String> {
    result.err().map(|fault| fault.code.0)
}

#[semio_framework_async_macros::async_test]
async fn choosing_the_plan_view_writes_the_window_config_and_shares_its_storey_as_presence() {
    let (result, ctx) = set(plan::WINDOW_KIND_ID, "view", "v-plan-st-first", None);
    let emit = result.expect("sets");
    assert_eq!(emit.window_config_mutations.len(), 1);
    assert!(emit.artifact_mutations.is_empty(), "a view parameter never edits the document");
    assert_eq!(ctx.presence_out, vec![ctx.presence.on_storey("st-first")]);
}

#[semio_framework_async_macros::async_test]
async fn the_world_parameters_and_the_views_of_the_plan_and_section_windows_each_write_their_window() {
    for (kind, field, value, pressed) in [
        (plan::WINDOW_KIND_ID, "view", "v-plan-st-ground", None),
        (world::WINDOW_KIND_ID, "projection", "orthographic", None),
        (world::WINDOW_KIND_ID, "isolated_storey", "st-ground", None),
        (world::WINDOW_KIND_ID, "hidden_storey", "st-first", Some(true)),
        (world::WINDOW_KIND_ID, "view_phase", "demolished", None),
        (world::WINDOW_KIND_ID, "view_phase", "", None),
        (world::WINDOW_KIND_ID, "section_enabled", "", Some(true)),
        (world::WINDOW_KIND_ID, "section_axis", "x", None),
        (world::WINDOW_KIND_ID, "section_offset", "2.5", None),
        (section::WINDOW_KIND_ID, "view", "v-section-a", None),
    ] {
        let (result, _) = set(kind, field, value, pressed);
        assert_eq!(result.unwrap_or_else(|fault| panic!("{kind} {field}: {}", fault.message)).window_config_mutations.len(), 1, "{kind} {field}");
    }
}

#[semio_framework_async_macros::async_test]
async fn bad_values_unknown_fields_and_missing_windows_are_refused_with_their_own_code() {
    assert_eq!(code(set(world::WINDOW_KIND_ID, "view_phase", "planned", None).0), Some("bim.view.value-invalid".to_string()));
    assert_eq!(code(set(plan::WINDOW_KIND_ID, "view", "nowhere", None).0), Some("bim.view.view-missing".to_string()));
    assert_eq!(code(set(plan::WINDOW_KIND_ID, "view", "v-section-a", None).0), Some("bim.view.view-missing".to_string()), "a section is no plan view");
    assert_eq!(code(set(world::WINDOW_KIND_ID, "section_axis", "w", None).0), Some("bim.view.value-invalid".to_string()));
    assert_eq!(code(set(section::WINDOW_KIND_ID, "view", "v-plan-st-ground", None).0), Some("bim.view.view-missing".to_string()), "a plan is no section");
    assert_eq!(code(set(plan::WINDOW_KIND_ID, "projection", "x", None).0), Some("bim.view.field-unknown".to_string()));
    assert_eq!(code(set("bim-edit-schedule", "view", "v-plan-st-first", None).0), Some("bim.view.window-unsupported".to_string()));
    let snapshot = demo();
    let mut windowless = BimDispatchCtx::default();
    let refused = run(&snapshot, |doc, cfg| handle(&SetView { field: "view".into(), value: "v-plan-st-first".into(), pressed: None }, doc, cfg, &mut windowless));
    assert_eq!(code(refused), Some("bim.view.window-required".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn hiding_a_storey_twice_toggles_it_back() {
    let (result, _) = set(world::WINDOW_KIND_ID, "hidden_storey", "st-first", None);
    assert!(result.is_ok());
}
