#[allow(unused_imports)]
use semio_s_artifact_bim_model_tests::editor::bim::terminology::BimLabels;
#[allow(unused_imports)]
use semio_framework_plugin::create_default_layout;
#[allow(unused_imports)]
use semio_framework_plugin::ModeDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::WindowLayout;
use crate::editor::bim::modes::edit::*;

#[semio_framework_async_macros::async_test]
async fn the_mode_has_a_stable_id() {
    assert_eq!(definition().id, BIM_EDIT_MODE_EDIT);
}

#[semio_framework_async_macros::async_test]
async fn the_default_layout_arranges_plan_world_section_and_schedule() {
    let layout = format!("{:?}", layout());
    for window in [semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan::WINDOW_KIND_ID, semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::WINDOW_KIND_ID, semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section::WINDOW_KIND_ID, semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::schedule::WINDOW_KIND_ID] {
        assert!(layout.contains(window), "the layout must place {window}");
    }
}
