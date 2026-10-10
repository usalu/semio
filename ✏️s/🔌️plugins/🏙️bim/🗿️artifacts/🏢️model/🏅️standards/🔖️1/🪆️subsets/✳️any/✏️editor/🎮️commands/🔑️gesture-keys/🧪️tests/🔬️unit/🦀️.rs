use super::*;
use crate::editor::bim::modes::edit::windows::plan;
use crate::editor::bim::unit_tests::context::view;
use crate::editor::bim::unit_tests::support::{demo, run};
use semio_framework_plugin::ArtifactEditor;
use semio_framework_ui_locale::Locale;

#[semio_framework_async_macros::async_test]
async fn a_gesture_key_outside_the_retained_route_is_refused_because_no_gesture_can_hear_it() {
    let view = view(Locale::En, &[("bim-plan", plan::WINDOW_KIND_ID)], Some("bim-plan"));
    let mut ctx = BimDispatchCtx::new(Vec::new(), Vec::new(), Some(&view), None, None);
    let result = run(&demo(), |doc, cfg| handle(&GestureTurn {}, doc, cfg, &mut ctx));
    assert_eq!(result.err().map(|fault| fault.code.0), Some(crate::editor::bim::gestures::RETAINED_ROUTE.to_string()));
}

#[semio_framework_async_macros::async_test]
async fn each_command_is_one_key_of_the_gesture() {
    use std::f64::consts::{FRAC_PI_2, PI};
    assert_eq!([GestureTurn::KEY, GestureTurnBack::KEY, GestureQuarter::KEY], [GestureKey::Turn(PI / 12.0), GestureKey::Turn(-PI / 12.0), GestureKey::Turn(FRAC_PI_2)]);
    assert_eq!([GestureMirror::KEY, GestureNext::KEY, GesturePrevious::KEY, GestureRaise::KEY, GestureLower::KEY, GestureSystem::KEY], [GestureKey::Mirror, GestureKey::Next, GestureKey::Previous, GestureKey::Raise, GestureKey::Lower, GestureKey::System]);
}

#[semio_framework_async_macros::async_test]
async fn every_key_binds_an_existing_command_and_no_key_is_bound_twice() {
    for (keys, action) in KEYBINDINGS {
        let command = crate::editor::bim::BimModelApp::command_from_action(action, None).unwrap_or_else(|error| panic!("{keys}: {}", error.message));
        assert!(crate::editor::bim::BIM_TOOL_IDS.contains(&command.command_id()), "{action} is a registered tool");
    }
    let all = crate::editor::bim::all_keybindings();
    let mut keys: Vec<&str> = all.iter().map(|(keys, _)| *keys).collect();
    let count = keys.len();
    keys.sort_unstable();
    keys.dedup();
    assert_eq!(keys.len(), count, "no key of the editor is bound twice");
    for (keys, action) in KEYBINDINGS {
        assert!(all.contains(&(*keys, *action)), "{keys} is part of the editor keybindings");
    }
}
