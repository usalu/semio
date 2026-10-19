use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_gestures::editor::bim::gestures::run_cursor;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_s_artifact_bim_model_gestures::editor::bim::commands::cursor_keys::{handle, CursorLeft, CursorRight, CursorUp, CursorDown, CursorLeftFar, CursorRightFar, CursorUpFar, CursorDownFar, CursorPlace};
use semio_s_artifact_bim_model_bim::editor::bim::fixture::context::view;
use semio_s_artifact_bim_model_bim::editor::bim::fixture::support::{demo, run};
use semio_framework_ui_locale::Locale;

#[semio_framework_async_macros::async_test]
async fn a_cursor_key_outside_the_retained_route_is_refused_because_no_gesture_can_hear_it() {
    let view = view(Locale::En, &[("bim-plan", semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan::WINDOW_KIND_ID)], Some("bim-plan"));
    let mut ctx = BimDispatchCtx::new(Vec::new(), Vec::new(), Some(&view), None, None);
    let result = run(&demo(), |doc, cfg| handle(&CursorLeft {}, doc, cfg, &mut ctx));
    assert_eq!(result.err().map(|fault| fault.code.0), Some(semio_s_artifact_bim_model_gestures::editor::bim::gestures::RETAINED_ROUTE.to_string()));
}

#[semio_framework_async_macros::async_test]
async fn the_arrows_step_by_a_fine_or_a_coarse_distance_and_place_is_the_click() {
    assert_eq!([CursorLeft::DELTA, CursorRight::DELTA, CursorUp::DELTA, CursorDown::DELTA], [Some([-0.1, 0.0]), Some([0.1, 0.0]), Some([0.0, 0.1]), Some([0.0, -0.1])]);
    assert_eq!([CursorLeftFar::DELTA, CursorRightFar::DELTA, CursorUpFar::DELTA, CursorDownFar::DELTA], [Some([-1.0, 0.0]), Some([1.0, 0.0]), Some([0.0, 1.0]), Some([0.0, -1.0])]);
    assert_eq!(CursorPlace::DELTA, None);
}
