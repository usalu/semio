use super::*;
use semio_plugin::ViewModel;

fn labels() -> &'static Fem2dLabels {
    semio_s_artifact_fem_2d_editor_terminology::editor::fem2d::terminology::fem2d_labels(&ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native))
}

#[test]
fn transform_options_group_is_tagged_for_transform_utility() {
    let group = measure(&Fem2dGumballConfig::default(), labels());
    let WindowMeasure::Group { id, active_utility_id, children, .. } = group else { panic!("group") };
    assert_eq!(id, format!("{FEM2D_PLAY_CONTROLLER_ID}-utility-options-transform"));
    assert_eq!(active_utility_id.as_deref(), Some(UTILITY_ID));
    assert_eq!(children.len(), 4);
}

/// ⚖️ LAW: the toggles read their pressed state from the model window's gumball config, so a flag written to the window config is what the options rail shows.
#[test]
fn gumball_flag_toggle_follows_the_config_it_is_read_from() {
    let mut config = Fem2dGumballConfig::default();
    config.set_flag("rotate", Some(false)).expect("rotate is a flag");
    let WindowMeasure::Group { children, .. } = measure(&config, labels()) else { panic!("group") };
    let pressed: Vec<(String, bool)> = children.into_iter().map(|child| match child {
        WindowMeasure::Toggle { id, pressed, .. } => (id, pressed),
        other => panic!("toggle expected, got {other:?}"),
    }).collect();
    assert_eq!(pressed.iter().find(|(id, _)| id == "fem2d-transform-rotate").map(|(_, on)| *on), Some(false));
    assert_eq!(pressed.iter().filter(|(_, on)| *on).count(), 3);
    config.set_flag("rotate", None).expect("rotate is a flag");
    let WindowMeasure::Group { children, .. } = measure(&config, labels()) else { panic!("group") };
    assert!(children.iter().all(|child| matches!(child, WindowMeasure::Toggle { pressed: true, .. })));
    assert!(config.set_flag("mirror", None).is_err());
}

/// 🖼️ LAW: every transform toggle names a registered icon id; an unregistered one silently falls back to the generic circle.
#[test]
fn every_transform_toggle_icon_is_registered_2d() {
    let WindowMeasure::Group { children, .. } = measure(&Fem2dGumballConfig::default(), labels()) else { panic!("group") };
    for child in &children {
        let WindowMeasure::Toggle { id, icon_id, .. } = child else { panic!("toggle") };
        assert_ne!(*icon_id, semio_plugin::IconName::Circle, "{id} falls back to the generic circle");
    }
}
