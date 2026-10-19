use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::ModelInference;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_framework_plugin::SurfaceKind;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_plan_linework::render::plan::scene;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_tests::viewer::bim::terminology::BimViewerLabels;
#[allow(unused_imports)]
use semio_framework_plugin::{scene_surface, ActionDescriptor, BuiltNode, MeasureSelectItem, UiAssemblyResult, WindowKindDefinition, WindowMeasure, WindowOptions};
use semio_s_artifact_bim_model_family_tests::viewer::bim::modes::view::windows::plan::{config, measures, definition, WINDOW_KIND_ID, BODY_KEY, render};
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::io::text::snapshot::{parse_dsl, BIM_EXAMPLE_TEXT};
use semio_s_artifact_bim_model_tests::viewer::bim::terminology::bim_viewer_labels;
use crate::viewer::bim::viewer_action;
use semio_framework_plugin::ViewModel;
use semio_framework_ui_locale::{Locale, Terminology};

fn demo() -> ModelSnapshot {
    parse_dsl(BIM_EXAMPLE_TEXT).expect("the committed demo parses")
}

fn inferred(snapshot: &ModelSnapshot) -> ModelInference {
    semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::with_inference(None, snapshot, Clone::clone)
}

fn picker(config: &semio_s_artifact_bim_model_family_tests::viewer::bim::modes::view::windows::plan::config::BimViewerPlanWindowConfig, locale: Locale) -> serde_json::Value {
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
        let config = semio_s_artifact_bim_model_family_tests::viewer::bim::modes::view::windows::plan::config::BimViewerPlanWindowConfig { storey: storey.into(), ..semio_s_artifact_bim_model_family_tests::viewer::bim::modes::view::windows::plan::config::BimViewerPlanWindowConfig::default() };
        assert!(render(&model, &inference, &config, &[]).is_ok(), "storey {storey:?}");
    }
}

#[test]
fn the_picker_lists_the_storeys_and_selects_the_configured_one() {
    let unset = picker(&semio_s_artifact_bim_model_family_tests::viewer::bim::modes::view::windows::plan::config::BimViewerPlanWindowConfig::default(), Locale::En);
    assert_eq!(unset["kind"], "select");
    assert_eq!(unset["label"], "Storey");
    assert_eq!(unset["value"], "st-ground", "an unset window shows the lowest storey");
    assert_eq!(unset["items"].as_array().expect("items").iter().map(|item| item["value"].as_str().expect("value")).collect::<Vec<_>>(), vec!["st-ground", "st-first"]);
    assert_eq!(unset["onChange"]["action"], "setPlanStorey");
    let chosen = picker(&semio_s_artifact_bim_model_family_tests::viewer::bim::modes::view::windows::plan::config::BimViewerPlanWindowConfig { storey: "st-first".into(), ..semio_s_artifact_bim_model_family_tests::viewer::bim::modes::view::windows::plan::config::BimViewerPlanWindowConfig::default() }, Locale::En);
    assert_eq!(chosen["value"], "st-first");
}

#[test]
fn the_picker_is_localised() {
    assert_eq!(picker(&config::BimViewerPlanWindowConfig::default(), Locale::De)["label"], "Geschoss");
}
