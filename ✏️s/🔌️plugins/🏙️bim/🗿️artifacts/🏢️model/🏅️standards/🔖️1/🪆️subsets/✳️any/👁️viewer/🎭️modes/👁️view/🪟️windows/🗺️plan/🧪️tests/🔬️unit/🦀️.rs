use super::*;
use crate::standards::v1::subsets::any::io::text::snapshot::{parse_dsl, BIM_EXAMPLE_TEXT};
use crate::viewer::bim::terminology::bim_viewer_labels;
use crate::viewer::bim::viewer_action;
use semio_framework_plugin::ViewModel;
use semio_framework_ui_locale::{Locale, Terminology};

fn demo() -> ModelSnapshot {
    parse_dsl(BIM_EXAMPLE_TEXT).expect("the committed demo parses")
}

fn inferred(snapshot: &ModelSnapshot) -> ModelInference {
    crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::with_inference(None, snapshot, Clone::clone)
}

fn picker(config: &config::BimViewerPlanWindowConfig, locale: Locale) -> serde_json::Value {
    let labels = bim_viewer_labels(&ViewModel::new(locale, Terminology::Native));
    let rows = serde_json::to_value(measures(&demo(), config, labels, viewer_action)).expect("measures serialise");
    assert_eq!(rows.as_array().expect("measures").len(), 1);
    rows[0].clone()
}

#[test]
fn the_window_is_a_canvas2d_surface_with_stable_ids() {
    let definition = definition();
    assert_eq!(definition.id, WINDOW_KIND_ID);
    assert_eq!(definition.body_key, BODY_KEY);
    assert!(matches!(definition.surface_kind, SurfaceKind::Canvas2d));
}

#[test]
fn an_empty_model_and_every_storey_of_the_demo_render() {
    let empty = ModelSnapshot::default();
    assert!(render(&empty, &inferred(&empty), &config::BimViewerPlanWindowConfig::default(), &[]).is_ok());
    let model = demo();
    let inference = inferred(&model);
    for storey in ["", "st-ground", "st-first", "gone"] {
        let config = config::BimViewerPlanWindowConfig { storey: storey.into(), ..config::BimViewerPlanWindowConfig::default() };
        assert!(render(&model, &inference, &config, &[]).is_ok(), "storey {storey:?}");
    }
}

#[test]
fn the_picker_lists_the_storeys_and_selects_the_configured_one() {
    let unset = picker(&config::BimViewerPlanWindowConfig::default(), Locale::En);
    assert_eq!(unset["kind"], "select");
    assert_eq!(unset["label"], "Storey");
    assert_eq!(unset["value"], "st-ground", "an unset window shows the lowest storey");
    assert_eq!(unset["items"].as_array().expect("items").iter().map(|item| item["value"].as_str().expect("value")).collect::<Vec<_>>(), vec!["st-ground", "st-first"]);
    assert_eq!(unset["onChange"]["action"], "setPlanStorey");
    let chosen = picker(&config::BimViewerPlanWindowConfig { storey: "st-first".into(), ..config::BimViewerPlanWindowConfig::default() }, Locale::En);
    assert_eq!(chosen["value"], "st-first");
}

#[test]
fn the_picker_is_localised() {
    assert_eq!(picker(&config::BimViewerPlanWindowConfig::default(), Locale::De)["label"], "Geschoss");
}
