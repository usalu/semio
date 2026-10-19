use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::{plan, section, world};
use semio_s_artifact_bim_model_tests::editor::bim::terminology::BimLabels;
use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::SeverityCounts;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::WindowMeasure;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_authored::editor::bim::entities::field_codecs::ordered_storeys;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_entities::editor::bim::entities::{storey_of, ENTITIES};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_kit::editor::bim::kit::bim_window_action;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::{schedule, sheet};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_tests::editor::bim::terminology::bim_labels;
#[allow(unused_imports)]
use semio_framework_plugin::DslValue;
#[allow(unused_imports)]
use semio_framework_plugin::MeasureSelectItem;
#[allow(unused_imports)]
use semio_framework_plugin::ViewModel;
#[allow(unused_imports)]
use semio_framework_plugin::WindowEngagement;
#[allow(unused_imports)]
use semio_framework_plugin::WindowEngagementInput;
#[allow(unused_imports)]
use semio_framework_plugin::WindowEngagementStatus;
#[allow(unused_imports)]
use std::collections::HashMap;
use semio_s_artifact_bim_model_dispatch_context::editor::bim::chrome::{measures, status, engagements, addressed_kind, problems};
use semio_s_artifact_bim_model_bim::editor::bim::fixture::context::view;
use semio_framework_plugin::WindowConfigOwner;
use semio_framework_ui_locale::Locale;

fn demo() -> ModelSnapshot {
    semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::io::text::snapshot::default_snapshot()
}

fn measures_of(kind: &str, locale: Locale) -> Vec<WindowMeasure> {
    let view = view(locale, &[("window", kind)], Some("window"));
    let cfg = ConfigView { snapshot: &semio_framework_plugin::NoConfig::default(), window: None };
    measures(&demo(), &cfg, &view).remove("window").expect("the addressed window has measures")
}

fn ids(measures: &[WindowMeasure]) -> Vec<String> {
    measures
        .iter()
        .map(|measure| match measure {
            WindowMeasure::Select { id, .. } | WindowMeasure::Number { id, .. } | WindowMeasure::Toggle { id, .. } | WindowMeasure::Slider { id, .. } | WindowMeasure::Group { id, .. } => id.clone(),
        })
        .collect()
}

#[semio_framework_async_macros::async_test]
async fn each_window_kind_gets_its_own_measures_with_stable_ids() {
    assert_eq!(ids(&measures_of(plan::WINDOW_KIND_ID, Locale::En)), vec!["bim.measure.plan.view"]);
    assert_eq!(ids(&measures_of(world::WINDOW_KIND_ID, Locale::En)), vec!["bim.measure.world.projection", "bim.measure.world.isolate", "bim.measure.world.phase", "bim.measure.world.storeys", "bim.measure.world.section", "bim.measure.world.section-axis", "bim.measure.world.section-offset", "bim.measure.world.analyse", "bim.measure.world.export"]);
    assert_eq!(ids(&measures_of(section::WINDOW_KIND_ID, Locale::En)), vec!["bim.measure.section.view"]);
    assert!(measures_of("bim-edit-schedule", Locale::En).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn the_plan_view_measure_lists_the_plans_by_storey_level_and_shows_the_active_one() {
    let WindowMeasure::Select { value, items, on_change, .. } = &measures_of(semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan::WINDOW_KIND_ID, Locale::En)[0] else { panic!("the first plan measure is the view select") };
    assert_eq!(value, "v-plan-st-ground");
    assert_eq!(items.iter().map(|item| item.label.as_str()).collect::<Vec<_>>(), vec!["Plan Ground", "Plan First"]);
    assert_eq!(items.iter().map(|item| item.value.as_str()).collect::<Vec<_>>(), vec!["v-plan-st-ground", "v-plan-st-first"]);
    assert_eq!(on_change.action, "setView");
}

#[semio_framework_async_macros::async_test]
async fn the_section_view_measure_lists_the_sections_before_the_elevations_and_shows_the_active_one() {
    let WindowMeasure::Select { value, items, label, .. } = &measures_of(semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section::WINDOW_KIND_ID, Locale::De)[0] else { panic!("the first section measure is the view select") };
    assert_eq!(value, "v-section-a");
    assert_eq!(items.iter().map(|item| item.label.as_str()).collect::<Vec<_>>(), vec!["Section A", "Section B", "East", "North", "South", "West"]);
    assert_eq!(label.as_deref(), Some("Ansicht"));
}

#[semio_framework_async_macros::async_test]
async fn measure_labels_follow_the_locale() {
    let WindowMeasure::Select { label, .. } = &measures_of(semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::WINDOW_KIND_ID, Locale::De)[0] else { panic!("the first world measure is the projection select") };
    assert_eq!(label.as_deref(), Some("Projektion"));
}

#[semio_framework_async_macros::async_test]
async fn the_status_counts_the_storey_elements_and_the_selection() {
    let view = view(Locale::En, &[("window", semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan::WINDOW_KIND_ID)], Some("window"));
    assert_eq!(status(&demo(), &view, Some("st-ground"), 2), "Storey Ground · 4 elements · 2 selected");
    assert_eq!(status(&demo(), &view, None, 0), "All storeys · 0 selected");
    let german = semio_s_artifact_bim_model_bim::editor::bim::fixture::context::view(Locale::De, &[("window", semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan::WINDOW_KIND_ID)], Some("window"));
    assert_eq!(status(&demo(), &german, Some("st-first"), 1), "Geschoss First · 0 Elemente · 1 ausgewählt");
    assert!(engagements(&demo(), &view, Some("st-ground"), 0, &SeverityCounts::default()).contains_key("window"));
}

#[semio_framework_async_macros::async_test]
async fn the_addressed_kind_comes_from_the_window_roster() {
    let view = view(Locale::En, &[("a", semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan::WINDOW_KIND_ID), ("b", semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::WINDOW_KIND_ID)], Some("b"));
    assert_eq!(addressed_kind(&view), Some(world::WINDOW_KIND_ID));
    assert_eq!(addressed_kind(&semio_s_artifact_bim_model_bim::editor::bim::fixture::context::view(Locale::En, &[("a", plan::WINDOW_KIND_ID)], None)), None);
    assert_eq!(<world::config::BimWorldWindowConfigOwner as WindowConfigOwner>::WINDOW_KIND_ID, world::WINDOW_KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn every_storey_of_the_world_has_a_hide_toggle_that_sets_the_hidden_storey_view_field() {
    let measures = measures_of(semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::WINDOW_KIND_ID, Locale::En);
    let WindowMeasure::Group { children, .. } = measures.iter().find(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id == "bim.measure.world.storeys")).expect("the storey group") else { panic!("a group") };
    assert_eq!(children.len(), 2, "one toggle per storey");
    let WindowMeasure::Toggle { label, pressed, on_change, .. } = &children[1] else { panic!("a toggle") };
    assert_eq!((label.as_deref(), *pressed), (Some("Hide First"), false));
    let args = on_change.args.as_ref().map(semio_framework_pack_json::to_json_string).unwrap_or_default();
    assert!(on_change.action == "setView" && args.contains("hidden_storey") && args.contains("st-first"), "{args}");
    let german = measures_of(semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::WINDOW_KIND_ID, Locale::De);
    assert!(german.iter().any(|measure| matches!(measure, WindowMeasure::Group { label, .. } if label == "Geschosse")), "the group is German");
}

#[semio_framework_async_macros::async_test]
async fn every_window_with_gestures_has_a_labelled_entry_field_wired_to_the_typed_commands_and_the_schedule_has_none() {
    for (kind, expected) in [(semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan::WINDOW_KIND_ID, true), (semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::WINDOW_KIND_ID, true), (semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section::WINDOW_KIND_ID, true), ("bim-edit-schedule", false)] {
        let view = view(Locale::De, &[("window", kind)], Some("window"));
        let engagement = engagements(&demo(), &view, None, 0, &SeverityCounts::default()).remove("window").expect("the addressed window engages");
        assert_eq!(engagement.input.is_some(), expected, "{kind}");
        if let Some(input) = engagement.input {
            assert_eq!(input.id.as_deref(), Some("window.entry"));
            assert_eq!(input.placeholder.as_deref(), Some("x, y · @dx, dy · Länge<Winkel · Länge"), "the placeholder is German");
            let actions = [&input.on_change, &input.on_submit, &input.on_abort].map(|action| action.as_ref().map(|action| action.action.clone()));
            assert_eq!(actions, [Some("engagementInput".to_string()), Some("engagementSubmit".to_string()), Some("canvasEscape".to_string())]);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn the_problems_line_counts_the_findings_by_severity_in_both_languages() {
    let counts = SeverityCounts { error: 2, warning: 0, info: 5 };
    assert_eq!(problems(&BimLabels::NATIVE_EN, &counts), "Errors: 2 · Notes: 5");
    assert_eq!(problems(&BimLabels::NATIVE_DE, &counts), "Fehler: 2 · Hinweise: 5");
    assert_eq!(problems(&BimLabels::NATIVE_EN, &SeverityCounts::default()), "No problems");
    assert_eq!(problems(&BimLabels::NATIVE_DE, &SeverityCounts::default()), "Keine Probleme");
}

#[semio_framework_async_macros::async_test]
async fn every_addressed_window_shows_the_problems_next_to_its_status() {
    let view = view(Locale::En, &[("window", semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::WINDOW_KIND_ID)], Some("window"));
    let engagement = engagements(&demo(), &view, None, 0, &SeverityCounts { error: 1, warning: 3, info: 0 }).remove("window").expect("the addressed window engages");
    let statuses: Vec<(String, String)> = engagement.status.expect("status lines").into_iter().map(|status| (status.id, status.text)).collect();
    assert_eq!(statuses.iter().map(|(id, _)| id.as_str()).collect::<Vec<_>>(), vec!["window.status", "window.problems"]);
    assert_eq!(statuses[1].1, "Errors: 1 · Warnings: 3");
}
