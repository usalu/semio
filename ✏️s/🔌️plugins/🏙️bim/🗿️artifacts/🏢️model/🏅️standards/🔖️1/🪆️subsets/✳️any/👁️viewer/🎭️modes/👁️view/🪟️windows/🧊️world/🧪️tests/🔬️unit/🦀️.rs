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
    crate::standards::v1::subsets::any::schema::inferences::model_graph::instance::with_inference(None, snapshot, Clone::clone)
}

fn measures_of(config: &config::BimViewerWorldWindowConfig, locale: Locale) -> serde_json::Value {
    let labels = bim_viewer_labels(&ViewModel::new(locale, Terminology::Native));
    serde_json::to_value(measures(&demo(), config, labels, viewer_action)).expect("measures serialise")
}

#[test]
fn the_window_is_a_world3d_surface_with_stable_ids() {
    let definition = definition();
    assert_eq!(definition.id, WINDOW_KIND_ID);
    assert_eq!(definition.body_key, BODY_KEY);
    assert!(matches!(definition.surface_kind, SurfaceKind::World3d));
}

#[test]
fn an_empty_and_a_demo_model_render() {
    let empty = ModelSnapshot::default();
    assert!(render(&empty, &inferred(&empty), &config::BimViewerWorldWindowConfig::default(), &Marks::default()).is_ok());
    let model = demo();
    assert!(render(&model, &inferred(&model), &config::BimViewerWorldWindowConfig::default(), &Marks::default()).is_ok());
}

#[test]
fn the_chrome_offers_the_projection_tree_and_one_visibility_toggle_per_storey() {
    let rows = measures_of(&config::BimViewerWorldWindowConfig::default(), Locale::En);
    let rows = rows.as_array().expect("measures");
    assert_eq!(rows.len(), 2);
    let storeys = &rows[1];
    assert_eq!(storeys["kind"], "group");
    assert_eq!(storeys["label"], "Storeys");
    let toggles = storeys["children"].as_array().expect("storey toggles");
    assert_eq!(toggles.iter().map(|toggle| toggle["label"].as_str().expect("label")).collect::<Vec<_>>(), vec!["Ground", "First"]);
    assert!(toggles.iter().all(|toggle| toggle["pressed"] == true));
    assert_eq!(toggles[0]["onChange"]["action"], "setStoreyVisible");
    assert_eq!(toggles[0]["onChange"]["args"]["storey"], "st-ground");
    assert_eq!(toggles[0]["onChange"]["controllerId"], "s.bim.model@1/*#viewer");
}

#[test]
fn a_hidden_storey_is_an_unpressed_toggle() {
    let rows = measures_of(&config::BimViewerWorldWindowConfig::default().with_storey_visible("st-first", false), Locale::En);
    let toggles = rows[1]["children"].as_array().expect("storey toggles").clone();
    assert_eq!(toggles.iter().map(|toggle| toggle["pressed"].as_bool().expect("pressed")).collect::<Vec<_>>(), vec![true, false]);
}

#[test]
fn the_chrome_is_localised() {
    assert_eq!(measures_of(&config::BimViewerWorldWindowConfig::default(), Locale::De)[1]["label"], "Geschosse");
}
