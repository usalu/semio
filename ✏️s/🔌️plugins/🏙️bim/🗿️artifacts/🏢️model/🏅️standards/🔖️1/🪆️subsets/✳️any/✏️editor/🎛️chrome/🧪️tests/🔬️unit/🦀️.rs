use super::*;
use crate::editor::bim::unit_tests::context::view;
use semio_framework_plugin::WindowConfigOwner;
use semio_framework_ui_locale::Locale;

fn demo() -> ModelSnapshot {
    crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot()
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
    assert_eq!(ids(&measures_of(plan::WINDOW_KIND_ID, Locale::En)), vec!["bim.measure.plan.storey", "bim.measure.plan.cut-height"]);
    assert_eq!(ids(&measures_of(world::WINDOW_KIND_ID, Locale::En)), vec!["bim.measure.world.projection", "bim.measure.world.isolate", "bim.measure.world.section", "bim.measure.world.section-axis", "bim.measure.world.section-offset"]);
    assert_eq!(ids(&measures_of(section::WINDOW_KIND_ID, Locale::En)), vec!["bim.measure.section.depth"]);
    assert!(measures_of("bim-edit-schedule", Locale::En).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn the_plan_storey_measure_lists_the_storeys_by_level_and_shows_the_active_one() {
    let WindowMeasure::Select { value, items, .. } = &measures_of(plan::WINDOW_KIND_ID, Locale::En)[0] else { panic!("the first plan measure is the storey select") };
    assert_eq!(value, "st-ground");
    assert_eq!(items.iter().map(|item| item.label.as_str()).collect::<Vec<_>>(), vec!["Ground", "First"]);
}

#[semio_framework_async_macros::async_test]
async fn measure_labels_follow_the_locale() {
    let WindowMeasure::Select { label, .. } = &measures_of(world::WINDOW_KIND_ID, Locale::De)[0] else { panic!("the first world measure is the projection select") };
    assert_eq!(label.as_deref(), Some("Projektion"));
}

#[semio_framework_async_macros::async_test]
async fn the_status_counts_the_storey_elements_and_the_selection() {
    let view = view(Locale::En, &[("window", plan::WINDOW_KIND_ID)], Some("window"));
    assert_eq!(status(&demo(), &view, Some("st-ground"), 2), "Storey Ground · 4 elements · 2 selected");
    assert_eq!(status(&demo(), &view, None, 0), "All storeys · 0 selected");
    let german = self::view(Locale::De, &[("window", plan::WINDOW_KIND_ID)], Some("window"));
    assert_eq!(status(&demo(), &german, Some("st-first"), 1), "Geschoss First · 0 Elemente · 1 ausgewählt");
    assert!(engagements(&demo(), &view, Some("st-ground"), 0).contains_key("window"));
}

#[semio_framework_async_macros::async_test]
async fn the_addressed_kind_comes_from_the_window_roster() {
    let view = view(Locale::En, &[("a", plan::WINDOW_KIND_ID), ("b", world::WINDOW_KIND_ID)], Some("b"));
    assert_eq!(addressed_kind(&view), Some(world::WINDOW_KIND_ID));
    assert_eq!(addressed_kind(&self::view(Locale::En, &[("a", plan::WINDOW_KIND_ID)], None)), None);
    assert_eq!(<world::config::BimWorldWindowConfigOwner as WindowConfigOwner>::WINDOW_KIND_ID, world::WINDOW_KIND_ID);
}
