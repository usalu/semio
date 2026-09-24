use super::*;
use semio_framework_plugin::ViewModel;

#[test]
fn transform_options_group_is_tagged_for_transform_utility() {
    let labels = crate::editor::fem3d::terminology::fem3d_labels(&ViewModel::default());
    let group = measure(&Fem3dGumballConfig::default(), labels);
    let WindowMeasure::Group { id, active_utility_id, children, .. } = group else { panic!("group") };
    assert_eq!(id, format!("{FEM3D_PLAY_CONTROLLER_ID}-utility-options-transform"));
    assert_eq!(active_utility_id.as_deref(), Some(UTILITY_ID));
    assert_eq!(children.len(), 5);
}

#[test]
fn gumball_flags_toggle_and_refuse_unknown_names() {
    let mut config = Fem3dGumballConfig::default();
    config.set_flag("rotate", Some(false)).expect("set");
    assert!(!config.rotate);
    config.set_flag("rotate", None).expect("toggle");
    assert!(config.rotate);
    assert!(config.set_flag("mirror", None).is_err());
    let off = Fem3dGumballConfig { move_axes: false, move_planes: false, rotate: false, scale_axes: false, scale_uniform: false };
    assert!(!off.any());
}
