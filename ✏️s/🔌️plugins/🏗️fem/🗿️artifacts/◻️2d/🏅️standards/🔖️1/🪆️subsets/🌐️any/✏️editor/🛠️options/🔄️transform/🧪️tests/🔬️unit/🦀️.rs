use super::*;
use semio_framework_plugin::ViewModel;

#[test]
fn transform_options_group_is_tagged_for_transform_utility() {
    let labels = crate::editor::fem2d::terminology::fem2d_labels(&ViewModel::default());
    let group = measure(&ViewModel::default(), labels);
    let WindowMeasure::Group { id, active_utility_id, children, .. } = group else { panic!("group") };
    assert_eq!(id, format!("{FEM2D_PLAY_CONTROLLER_ID}-utility-options-transform"));
    assert_eq!(active_utility_id.as_deref(), Some(UTILITY_ID));
    assert_eq!(children.len(), 4);
}

#[test]
fn gumball_flag_toggle_updates_window_config() {
    set_gumball_flag("test-window", "rotate", Some(false));
    assert!(!gumball_config_for_window("test-window").rotate);
    set_gumball_flag("test-window", "rotate", None);
    assert!(gumball_config_for_window("test-window").rotate);
}
