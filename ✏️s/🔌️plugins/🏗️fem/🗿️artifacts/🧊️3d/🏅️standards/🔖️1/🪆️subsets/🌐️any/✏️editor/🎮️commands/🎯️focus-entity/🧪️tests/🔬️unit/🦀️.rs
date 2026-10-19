use semio_s_artifact_fem_3d_editor::editor::fem3d::commands::focus_entity::*;
use semio_s_artifact_fem_3d_editor::editor::fem3d::commands::set_camera::self;
use semio_s_artifact_fem_3d_editor::editor::fem3d::commands::set_camera::SetCamera;
use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::fem3d_entity_point;
use semio_s_artifact_fem_3d_editor_model_window::editor::fem3d::modes::edit::windows::model;
use semio_s_artifact_fem_3d_editor_results_window::editor::fem3d::modes::edit::windows::results;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_framework_os_kernel::Viewport3dOrbit;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
use crate::context::view;
use semio_framework_plugin::HistoryView;

fn demo() -> Fem3dSnapshot {
    semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::fem3d_demo_snapshot()
}

/// 🎯️ LAW: focusing re-aims the orbit at the entity and carries the eye offset along, on the
/// window kind the command is addressed to; an unknown entity or no window is refused.
#[test]
fn focus_entity_re_aims_the_addressed_window_orbit() {
    let doc = demo();
    let history = HistoryView::empty();
    let view_doc = ArtifactView::new(&doc, &history);
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle_window(&FocusEntity { id: semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::fem3d_entity_target("node", "n20_l1").unwrap() }, &view_doc, &cfg, &view(model::FEM3D_WINDOW_MODEL)).expect("focus");
    assert_eq!(emit.window_config_mutations.len(), 1);
    assert_eq!(emit.window_config_mutations[0].window_id(), "model-left");
    let initial = semio_s_artifact_fem_3d_scene::viewport::INITIAL;
    let focused = focused_orbit(&initial, [8.0, 0.0, 2.8]);
    assert_eq!(focused.target, [8.0, 0.0, 2.8]);
    assert_eq!(focused.position, [8.0 + initial.position[0] - initial.target[0], 0.0 + initial.position[1] - initial.target[1], 2.8 + initial.position[2] - initial.target[2]], "the eye keeps its offset");
    assert_eq!(focused.zoom, initial.zoom);
    assert_eq!(focused.up, initial.up);
    assert!(handle_window(&FocusEntity { id: semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::fem3d_entity_target("node", "ghost").unwrap() }, &view_doc, &cfg, &view(model::FEM3D_WINDOW_MODEL)).is_err());
    assert!(handle_window(&FocusEntity { id: semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::fem3d_entity_target("node", "n20_l1").unwrap() }, &view_doc, &cfg, &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).is_err());
    assert!(handle(&FocusEntity { id: semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::fem3d_entity_target("node", "n20_l1").unwrap() }, &view_doc, &cfg).is_err());
    let results = handle_window(&FocusEntity { id: semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::fem3d_entity_target("solid", "sol1").unwrap() }, &view_doc, &cfg, &view(results::FEM3D_WINDOW_RESULTS)).expect("focus on results");
    assert_eq!(results.window_config_mutations[0].window_id(), "results-left");
}
