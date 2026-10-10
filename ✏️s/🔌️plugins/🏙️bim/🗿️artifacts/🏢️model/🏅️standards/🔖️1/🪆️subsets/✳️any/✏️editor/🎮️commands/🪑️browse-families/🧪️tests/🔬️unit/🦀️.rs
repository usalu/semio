use super::*;
use crate::editor::bim::entities::components::tests::placed;
use crate::editor::bim::modes::edit::windows::plan;
use crate::editor::bim::unit_tests::context::view;
use crate::editor::bim::unit_tests::support::{ctx, run};
use semio_framework_ui_locale::Locale;

fn selected(emit: &Emit<ModelMutation, NoConfigMutation>) -> String {
    emit.effects.iter().map(|effect| format!("{effect:?}")).collect::<Vec<_>>().join(" ")
}

fn code(result: Result<Emit<ModelMutation, NoConfigMutation>, Fault>) -> Option<String> {
    result.err().map(|fault| fault.code.0)
}

#[semio_framework_async_macros::async_test]
async fn placing_selects_the_family_and_arms_the_component_utility_in_the_addressed_window() {
    let snapshot = placed();
    let view = view(Locale::En, &[("bim-plan", plan::WINDOW_KIND_ID)], Some("bim-plan"));
    let mut addressed = BimDispatchCtx::new(Vec::new(), Vec::new(), Some(&view), None, None);
    let emit = run(&snapshot, |doc, cfg| handle(&PlaceComponent { family: "fam-basin".into() }, doc, cfg, &mut addressed)).expect("places");
    let text = selected(&emit);
    assert!(emit.artifact_mutations.is_empty(), "selecting and arming write nothing");
    assert!(text.contains("fam-basin") && text.contains("library"), "{text}");
    assert!(emit.effects.iter().any(|effect| matches!(effect, Effect::SetActiveUtility { window_id, utility_id } if window_id == "bim-plan" && utility_id == COMPONENT_UTILITY)), "{text}");
    let mut panel = ctx(&[]);
    let from_panel = run(&snapshot, |doc, cfg| handle(&PlaceComponent { family: "fam-basin".into() }, doc, cfg, &mut panel)).expect("places");
    assert_eq!(from_panel.effects.len(), 1, "a panel addresses no window: only the selection is set");
}

#[semio_framework_async_macros::async_test]
async fn a_profile_or_a_missing_family_cannot_be_placed() {
    let snapshot = placed();
    let mut context = ctx(&[]);
    for family in ["fam-hea", "fam-ghost"] {
        let result = run(&snapshot, |doc, cfg| handle(&PlaceComponent { family: family.into() }, doc, cfg, &mut context));
        assert_eq!(code(result).as_deref(), Some("bim.place.family-unavailable"), "{family}");
    }
}

#[semio_framework_async_macros::async_test]
async fn opening_selects_the_family_and_asks_the_shell_for_the_family_editor() {
    let snapshot = placed();
    let mut context = ctx(&[]);
    let emit = run(&snapshot, |doc, cfg| handle(&OpenFamily { family: "fam-hea".into() }, doc, cfg, &mut context)).expect("opens a profile too");
    assert!(emit.effects.iter().any(|effect| matches!(effect, Effect::OpenWindow { kind, .. } if kind.0 == crate::editor::bim::modes::edit::windows::family::WINDOW_KIND_ID)));
    let ghost = run(&snapshot, |doc, cfg| handle(&OpenFamily { family: "fam-ghost".into() }, doc, cfg, &mut context));
    assert_eq!(code(ghost).as_deref(), Some("bim.family.missing"));
}

#[semio_framework_async_macros::async_test]
async fn a_search_selects_the_placeable_matches_clears_on_empty_and_refuses_no_match() {
    let snapshot = placed();
    let mut context = ctx(&[]);
    let search = |query: &str, category: &str, context: &mut BimDispatchCtx| run(&snapshot, |doc, cfg| handle(&SearchFamilies { query: query.into(), category: category.into() }, doc, cfg, context));
    let text = selected(&search("a", "", &mut context).expect("matches"));
    assert!(text.contains("fam-table") && text.contains("fam-basin") && !text.contains("fam-hea"), "{text}");
    let text = selected(&search("a", "plumbing", &mut context).expect("one category"));
    assert!(text.contains("fam-basin") && !text.contains("fam-table"), "{text}");
    let cleared = search("  ", "", &mut context).expect("clears");
    assert_eq!(cleared.effects.len(), 1);
    assert_eq!(code(search("zzz", "", &mut context)).as_deref(), Some("bim.browser.no-match"));
}
