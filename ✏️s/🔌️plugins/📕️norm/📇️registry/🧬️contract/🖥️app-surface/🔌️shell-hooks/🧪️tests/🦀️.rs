use super::*;
use crate::accessibility::focus_order::Region;
use crate::viewport3d::config::DeviceClass;
use semio_framework::{ShellDeviceSignal, ShellHookCause, ShellHookDriver, ShellHookRegistry};
use std::sync::Arc;

fn request(app_id: &str, width_px: u32, touch: bool, has_selection: bool, prefs: Option<String>) -> ShellLayoutRequest {
    ShellLayoutRequest { app_id: app_id.to_owned(), signal: ShellDeviceSignal { width_px, touch }, cause: ShellHookCause::Resize, prefs, has_selection, lang: "en".to_owned() }
}

fn key(scope: &str, key: &str, ctrl: bool, alt: bool, shift: bool) -> ShellKeyEvent {
    ShellKeyEvent { scope: scope.to_owned(), key: key.to_owned(), ctrl, alt, shift, meta: false }
}

const EDITOR: &str = "s.norm.en1993#editor";
const VIEWER: &str = "s.norm.en1993#viewer";

#[test]
fn the_hook_chooses_the_named_layout_of_the_device_class() {
    for (width, touch, class) in [(1440, false, DeviceClass::Desktop), (390, true, DeviceClass::Mobile), (900, true, DeviceClass::Tablet)] {
        let choice = NormShellHooks.choose_layout(&request(EDITOR, width, touch, false, None)).expect("norm app");
        assert_eq!(choice.layout_id, visual_layout_id(class), "{width}");
        assert_eq!(choice.layout, visual_layout_with("en1993", class, &LayoutPrefs::default()), "{width}");
    }
}

#[test]
fn the_viewer_gets_its_own_windows_and_no_sheet() {
    let choice = NormShellHooks.choose_layout(&request(VIEWER, 390, true, true, None)).expect("norm viewer");
    assert_eq!(choice.layout, view_layout_with("en1993", DeviceClass::Mobile, &LayoutPrefs::default()));
    assert!(choice.sheet.is_none());
}

#[test]
fn another_plugin_app_is_left_alone() {
    assert!(NormShellHooks.choose_layout(&request("s.cad.cad#editor", 390, true, true, None)).is_none());
}

#[test]
fn stored_preferences_shape_the_layout_and_survive_their_text_form() {
    let mut prefs = LayoutPrefs::default();
    prefs.dock.left = 0.30;
    prefs.dock.centre = 0.45;
    prefs.dock.right = 0.25;
    let text = encode_layout_prefs(&prefs);
    let decoded = decode_layout_prefs(&text).expect("round trip");
    assert_eq!(crate::device_profiles::sanitised(&decoded), crate::device_profiles::sanitised(&prefs));
    let choice = NormShellHooks.choose_layout(&request(EDITOR, 1440, false, false, Some(text))).expect("norm app");
    assert_eq!(choice.layout, visual_layout_with("en1993", DeviceClass::Desktop, &prefs));
    assert_ne!(choice.layout, visual_layout_with("en1993", DeviceClass::Desktop, &LayoutPrefs::default()));
    assert_eq!(decode_layout_prefs("not json"), None);
    store_layout_prefs(&LayoutPrefs::default());
    assert_eq!(stored_layout_prefs(), LayoutPrefs::default());
}

#[test]
fn a_phone_with_a_selection_asks_for_the_properties_sheet() {
    let choice = NormShellHooks.choose_layout(&request(EDITOR, 390, true, true, None)).expect("norm app");
    let sheet = choice.sheet.expect("sheet");
    assert_eq!((sheet.label.as_str(), sheet.min_handle_px >= 44), ("Properties", true));
    assert_eq!(current_sheet(), Some(sheet));
    let german = ShellLayoutRequest { lang: "de".to_owned(), ..request(EDITOR, 390, true, true, None) };
    assert_eq!(NormShellHooks.choose_layout(&german).and_then(|choice| choice.sheet).map(|sheet| sheet.label), Some("Eigenschaften".to_owned()));
    assert!(NormShellHooks.choose_layout(&request(EDITOR, 1440, false, true, None)).expect("desktop").sheet.is_none());
}

#[test]
fn the_shell_driver_switches_layout_on_resize_and_modality_through_the_norm_hooks() {
    let mut driver = ShellHookDriver::default();
    let mut registry = ShellHookRegistry::default();
    registry.install("norm", Arc::new(NormShellHooks));
    driver.registry = registry;
    assert_eq!(driver.observe("norm", EDITOR, 1440, None, "en").map(|c| c.layout_id), Some(visual_layout_id(DeviceClass::Desktop)));
    assert!(driver.observe("norm", EDITOR, 1440, None, "en").is_none());
    assert_eq!(driver.observe("norm", EDITOR, 390, None, "en").map(|c| c.layout_id), Some(visual_layout_id(DeviceClass::Mobile)));
    driver.note_touch(true);
    assert!(driver.observe("norm", EDITOR, 390, None, "en").is_some(), "the input modality changed");
}

#[test]
fn the_shortcut_map_runs_through_the_hook_in_its_scope() {
    let hooks = NormShellHooks;
    assert_eq!(hooks.handle_key(&key("norm-viewport", "ArrowLeft", false, false, false)), ShellKeyOutcome::Action { action: "norm.shortcut.orbit-left".to_owned() }, "arrows orbit in the viewport scope");
    assert_eq!(hooks.handle_key(&key("norm-global", "z", true, false, false)), ShellKeyOutcome::Ignored, "undo belongs to the store history");
    assert_eq!(hooks.handle_key(&key("norm-global", "y", true, false, false)), ShellKeyOutcome::Ignored, "redo belongs to the store history");
    assert_eq!(hooks.handle_key(&key("norm-global", "c", true, false, false)), ShellKeyOutcome::Action { action: "norm.shortcut.copy".to_owned() });
    assert_eq!(hooks.handle_key(&key("norm-global", "v", true, false, false)), ShellKeyOutcome::Action { action: "norm.shortcut.paste".to_owned() });
    assert_eq!(hooks.handle_key(&key("norm-global", "d", true, false, false)), ShellKeyOutcome::Action { action: "norm.shortcut.duplicate".to_owned() });
    assert_eq!(hooks.handle_key(&key("norm-global", "Delete", false, false, false)), ShellKeyOutcome::Action { action: "norm.shortcut.delete".to_owned() });
    assert_eq!(hooks.handle_key(&key("norm-viewport", "a", true, false, false)), ShellKeyOutcome::Ignored, "a window without an interaction domain has nothing to select");
    assert_eq!(hooks.handle_key(&key("norm-handle", "ArrowLeft", false, false, false)), ShellKeyOutcome::Action { action: "norm.handle.arrow-left.normal".to_owned() });
    assert_eq!(hooks.handle_key(&key("norm-handle", "ArrowUp", false, true, false)), ShellKeyOutcome::Action { action: "norm.handle.arrow-up.fine".to_owned() });
    assert_eq!(hooks.handle_key(&key("norm-handle", "ArrowDown", false, false, true)), ShellKeyOutcome::Action { action: "norm.handle.arrow-down.large".to_owned() });
    assert_eq!(hooks.handle_key(&key("norm-handle", "Enter", false, false, false)), ShellKeyOutcome::Action { action: "norm.handle.enter.normal".to_owned() });
    assert_eq!(hooks.handle_key(&key("norm-handle", "Escape", false, false, false)), ShellKeyOutcome::Action { action: "norm.handle.escape.normal".to_owned() });
    assert_eq!(hooks.handle_key(&key("norm-viewport", "q", false, false, false)), ShellKeyOutcome::Ignored);
}

/// 🎯️ The `focusWindow` verb the chord of `region` raises in `variant`: the window kind of the region, which every device layout holds.
fn expected_focus_verb(variant: &str, region: Region) -> ShellKeyOutcome {
    ShellKeyOutcome::verb(semio_framework::FOCUS_WINDOW_ACTION_ID, [(semio_framework::FOCUS_WINDOW_ARG_KIND.to_owned(), visual_window_id(variant, region.role()))])
}

#[test]
fn every_documented_shortcut_reaches_its_shell_or_grip_walk_outcome() {
    for shortcut in shortcuts() {
        let scope = match shortcut.scope {
            Scope::Global => "norm-en1993-global",
            Scope::Viewport => "norm-en1993-viewport",
            Scope::Handle => "norm-en1993-handle",
        };
        let outcome = NormShellHooks.handle_key(&ShellKeyEvent { scope: scope.to_owned(), key: shortcut.chord.key.to_owned(), ctrl: shortcut.chord.ctrl, alt: shortcut.chord.alt, shift: shortcut.chord.shift, meta: false });
        match shortcut_route(&shortcut_action_id(&shortcut.action)) {
            HostRoute::History(_) => assert_eq!(outcome, ShellKeyOutcome::Ignored, "{shortcut:?}"),
            HostRoute::SelectAll => assert_eq!(outcome, ShellKeyOutcome::verb(semio_framework::SELECT_ALL_ACTION_ID, [("granularities".to_owned(), "member,plate,shell,solid,fastener,support,load,surface,group".to_owned())]), "{shortcut:?}"),
            HostRoute::FocusWindow(region) => assert_eq!(outcome, expected_focus_verb("en1993", region), "{shortcut:?}"),
            HostRoute::Local if matches!(&shortcut.action, ShortcutAction::CycleHandle { .. } | ShortcutAction::CycleGripInEntity { .. }) => assert_eq!(outcome, ShellKeyOutcome::Ignored, "{shortcut:?}: no grip walk is active"),
            HostRoute::ViewAct(_) | HostRoute::Entity(_) | HostRoute::Local => assert!(matches!(&outcome, ShellKeyOutcome::Action { action } if action.starts_with("norm.")), "{shortcut:?} -> {outcome:?}"),
        }
    }
}

#[test]
fn the_alt_number_chords_focus_the_window_of_their_region_through_the_framework_verb_in_every_artifact() {
    let chords = [("1", Region::Outliner), ("2", Region::Viewport), ("3", Region::Properties), ("4", Region::Checks), ("5", Region::Diagrams)];
    for variant in VARIANTS {
        for (key, region) in chords {
            for scope in [format!("norm-{variant}-properties"), format!("norm-{variant}-viewport-2"), format!("norm-{variant}-checks")] {
                let outcome = NormShellHooks.handle_key(&ShellKeyEvent { scope, key: key.to_owned(), alt: true, ..ShellKeyEvent::default() });
                let kind = visual_window_id(variant, region.role());
                assert_eq!(outcome, expected_focus_verb(variant, region), "Alt+{key} in {variant}");
                let layout = to_string(&from_dsl_value(&visual_layout(variant, DeviceClass::Desktop).to_value()));
                assert!(layout.contains(&kind), "{kind} is a window kind of the {variant} desktop layout");
            }
        }
    }
    for variant in VARIANTS {
        for device in [DeviceClass::Mobile, DeviceClass::Tablet] {
            let layout = to_string(&from_dsl_value(&visual_layout(variant, device).to_value()));
            for region in [Region::Outliner, Region::Viewport, Region::Properties, Region::Checks, Region::Diagrams] {
                assert!(layout.contains(&visual_window_id(variant, region.role())), "{variant}: a {device:?} layout holds the {region:?} window, the outliner as a tab");
            }
        }
    }
    assert_eq!(variant_of_window("norm-en1993-properties"), Some("en1993"));
    assert_eq!(variant_of_window("norm-global"), None);
    assert_eq!(variant_of_window("other-en1993-properties"), None);
}

#[test]
fn a_focus_chord_in_a_window_of_no_artifact_is_left_to_the_shell_and_no_focus_action_is_an_app_action() {
    let outcome = NormShellHooks.handle_key(&ShellKeyEvent { scope: "unrelated".to_owned(), key: "3".to_owned(), alt: true, ..ShellKeyEvent::default() });
    assert!(matches!(outcome, ShellKeyOutcome::Ignored | ShellKeyOutcome::Verb { .. }), "never an app action: {outcome:?}");
    assert!(shortcut_action_definitions().iter().all(|definition| !definition.id.contains("focus-")), "the focus shortcuts are framework verbs, no app actions");
}

#[test]
fn every_answered_action_is_declared_by_the_viewport_window_with_both_languages() {
    let declared = shortcut_action_definitions();
    for shortcut in shortcuts() {
        let scope = match shortcut.scope {
            Scope::Global => "norm-en1993-global",
            Scope::Viewport => "norm-en1993-viewport",
            Scope::Handle => "norm-en1993-handle",
        };
        let ShellKeyOutcome::Action { action } = NormShellHooks.handle_key(&ShellKeyEvent { scope: scope.to_owned(), key: shortcut.chord.key.to_owned(), ctrl: shortcut.chord.ctrl, alt: shortcut.chord.alt, shift: shortcut.chord.shift, meta: false }) else {
            assert!(matches!(shortcut_route(&shortcut_action_id(&shortcut.action)), HostRoute::History(_) | HostRoute::SelectAll | HostRoute::FocusWindow(_)) || matches!(&shortcut.action, ShortcutAction::CycleHandle { .. } | ShortcutAction::CycleGripInEntity { .. } | ShortcutAction::EnterHandles), "{shortcut:?} has a shell verb, history or grip-walk outcome");
            continue;
        };
        assert!(declared.iter().any(|definition| definition.id == action), "{action} is not declared");
        assert!(!shortcut.label.en.is_empty() && !shortcut.label.de.is_empty());
    }
    let mut ids: Vec<&str> = declared.iter().map(|definition| definition.id.as_str()).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), declared.len(), "one action per distinct shortcut action");
    let window = viewport_window_definition("en1993", LocalizedLabel::native("Model", "Modell"), "norm.en1993.viewport", "box");
    assert!(declared.iter().all(|definition| window.actions.iter().any(|action| action.id == definition.id)));
}

#[test]
fn a_resize_from_desktop_to_mobile_switches_the_layout_and_a_selection_opens_the_sheet_through_the_registered_hooks() {
    semio_framework::register_shell_hooks("norm-integration", Arc::new(NormShellHooks));
    let mut driver = ShellHookDriver::default();
    let desktop = driver.observe("norm-integration", EDITOR, 1440, None, "en").expect("start");
    assert_eq!(desktop.layout_id, visual_layout_id(DeviceClass::Desktop));
    assert!(driver.sheet.is_none());
    driver.note_touch(true);
    let mobile = driver.observe("norm-integration", EDITOR, 390, None, "en").expect("resize");
    assert_eq!((mobile.layout_id.as_str(), mobile.layout == visual_layout_with("en1993", DeviceClass::Mobile, &LayoutPrefs::default())), (visual_layout_id(DeviceClass::Mobile).as_str(), true));
    assert!(driver.sheet.is_none(), "nothing selected yet");
    driver.note_selection(true);
    driver.observe("norm-integration", EDITOR, 390, None, "en").expect("selection");
    assert_eq!(driver.sheet.as_ref().map(|sheet| (sheet.label.as_str(), sheet.min_handle_px >= 44)), Some(("Properties", true)));
    driver.note_selection(false);
    driver.observe("norm-integration", EDITOR, 390, None, "en").expect("selection cleared");
    assert!(driver.sheet.is_none());
    assert_eq!(driver.handle_key("norm-integration", &key("norm-viewport", "ArrowLeft", false, false, false)), ShellKeyOutcome::Action { action: "norm.shortcut.orbit-left".to_owned() });
}

#[test]
fn names_are_kebab_cased() {
    assert_eq!(kebab("CycleRegion { forward: true }"), "cycle-region-forward-true");
    assert_eq!(kebab("ArrowLeft"), "arrow-left");
}

fn grip(id: &str, entity: &str) -> GripRef {
    GripRef { id: id.to_owned(), entity: entity.to_owned() }
}

#[test]
fn enter_starts_the_grip_walk_tab_and_ctrl_arrow_move_it_and_escape_ends_it() {
    let hooks = NormShellHooks;
    let (window, domain) = ("norm-en1998-viewport", "norm.en1998");
    note_grips(domain, vec![grip("a.length", "a"), grip("a.height", "a"), grip("b.length", "b")]);
    set_focused_grip(domain, None);
    let press = |name: &str, ctrl: bool, shift: bool| hooks.handle_key(&ShellKeyEvent { scope: window.to_owned(), key: name.to_owned(), ctrl, alt: false, shift, meta: false });
    assert_eq!(press("Tab", false, false), ShellKeyOutcome::Ignored, "outside the grip walk Tab belongs to the shell");
    assert_eq!(press("Enter", false, false), ShellKeyOutcome::Consumed);
    assert_eq!(focused_grip(domain).as_deref(), Some("a.length"));
    assert_eq!(press("Tab", false, false), ShellKeyOutcome::Consumed);
    assert_eq!(focused_grip(domain).as_deref(), Some("a.height"));
    assert_eq!(press("Tab", false, true), ShellKeyOutcome::Consumed);
    assert_eq!(focused_grip(domain).as_deref(), Some("a.length"));
    assert_eq!(press("ArrowRight", true, false), ShellKeyOutcome::Consumed);
    assert_eq!(focused_grip(domain).as_deref(), Some("a.height"));
    assert_eq!(press("ArrowDown", true, false), ShellKeyOutcome::Consumed);
    assert_eq!(focused_grip(domain).as_deref(), Some("a.length"), "Ctrl+Arrow wraps inside the entity and never reaches b");
    assert_eq!(press("ArrowLeft", true, false), ShellKeyOutcome::Consumed);
    assert_eq!(focused_grip(domain).as_deref(), Some("a.height"));
    assert_eq!(hooks.handle_key(&ShellKeyEvent { scope: window.to_owned(), key: "ArrowUp".to_owned(), ctrl: false, alt: false, shift: false, meta: true }), ShellKeyOutcome::Consumed, "Command acts as Ctrl");
    assert_eq!(focused_grip(domain).as_deref(), Some("a.length"));
    assert_eq!(press("ArrowRight", false, false), ShellKeyOutcome::Action { action: "norm.handle.arrow-right.normal".to_owned() }, "plain arrows stay nudges");
    assert_eq!(focused_grip(domain).as_deref(), Some("a.length"));
    for _ in 0..2 {
        assert_eq!(press("Tab", false, false), ShellKeyOutcome::Consumed);
    }
    assert_eq!(focused_grip(domain).as_deref(), Some("b.length"));
    assert_eq!(press("Tab", false, false), ShellKeyOutcome::Ignored, "past the last grip the walk ends");
    assert_eq!(focused_grip(domain), None);
    assert_eq!(press("Enter", false, false), ShellKeyOutcome::Consumed);
    assert_eq!(press("Escape", false, false), ShellKeyOutcome::Action { action: "norm.handle.escape.normal".to_owned() }, "Escape still cancels the open change");
    assert_eq!(focused_grip(domain), None, "and returns the keyboard to the viewport");
}

#[test]
fn the_platform_preferences_the_shell_reports_become_the_norm_accessibility_preferences() {
    let hooks = NormShellHooks;
    let reported = ShellPlatformPrefs { forced_colors: true, prefers_contrast_more: false, prefers_dark: true, prefers_reduced_motion: true };
    hooks.platform_prefs_changed(&reported);
    assert_eq!(platform_a11y_prefs(), crate::accessibility::A11yPrefs { forced_colors: true, prefers_contrast_more: false, prefers_dark: true, prefers_reduced_motion: true });
    assert_eq!(a11y_prefs_of(&ShellPlatformPrefs::default()), crate::accessibility::A11yPrefs::default());
    hooks.platform_prefs_changed(&ShellPlatformPrefs::default());
    assert_eq!(platform_a11y_prefs(), crate::accessibility::A11yPrefs::default());
}

#[test]
fn a_long_press_in_the_viewport_opens_the_edit_menu_through_an_action_the_window_declares_and_the_view_lane_folds() {
    for variant in VARIANTS {
        let scope = format!("norm-{variant}-viewport");
        assert_eq!(touch_gesture_outcome(&scope, ShellTouchGesture::LongPress), ShellKeyOutcome::Action { action: TOUCH_MENU_OPEN.to_owned() }, "{variant}");
        for other in ["properties", "checks", "outliner", "diagrams"] {
            assert_eq!(touch_gesture_outcome(&format!("norm-{variant}-{other}"), ShellTouchGesture::LongPress), ShellKeyOutcome::Ignored, "{variant} {other}: a window without the viewport has no Edit menu");
        }
    }
    assert_eq!(touch_gesture_outcome("unrelated-viewport", ShellTouchGesture::LongPress), ShellKeyOutcome::Ignored, "another app's viewport is not ours");
    assert_eq!(touch_gesture_outcome("norm-en1993-viewport", ShellTouchGesture::DoubleTap), ShellKeyOutcome::Action { action: TOUCH_FRAME_SELECTION.to_owned() }, "a double tap frames the selection");
    assert_eq!(touch_gesture_outcome("norm-en1993-checks", ShellTouchGesture::DoubleTap), ShellKeyOutcome::Ignored, "only the viewport frames");
    assert_eq!(touch_gesture_outcome("unrelated-viewport", ShellTouchGesture::DoubleTap), ShellKeyOutcome::Ignored);
    let event = ShellTouchEvent { scope: "norm-en1993-viewport-2".to_owned(), gesture: ShellTouchGesture::LongPress, x: 40.0, y: 60.0, width_px: 390 };
    assert_eq!(NormShellHooks.handle_touch(&event), ShellKeyOutcome::Action { action: TOUCH_MENU_OPEN.to_owned() });
    let window = viewport_window_definition("en1993", LocalizedLabel::native("Model", "Modell"), "norm.en1993.viewport", "box");
    for id in [TOUCH_MENU_OPEN, TOUCH_MENU_CLOSE, TOUCH_FRAME_SELECTION] {
        assert!(window.actions.iter().any(|action| action.id == id), "{id} is not declared by the viewport window");
    }
    let opened = view_event(view_act_of(TOUCH_MENU_OPEN).expect("open").0, None, view_act_of(TOUCH_MENU_OPEN).map(|(_, number)| number)).expect("an event");
    let lane = NormTransient::default().apply(opened);
    assert!(lane.view.menu);
    let closed = view_event(view_act_of(TOUCH_MENU_CLOSE).expect("close").0, None, view_act_of(TOUCH_MENU_CLOSE).map(|(_, number)| number)).expect("an event");
    assert!(!lane.apply(closed).view.menu);
    let framed = view_event(view_act_of(TOUCH_FRAME_SELECTION).expect("frame").0, None, view_act_of(TOUCH_FRAME_SELECTION).map(|(_, number)| number)).expect("an event");
    assert!(NormTransient::default().apply(framed).view.frame, "the double tap frames the selection in the view lane");
    assert_eq!(view_act_of("norm.shortcut.next-check").map(|(act, _)| act), Some(VIEW_ACTION_STEP_CHECK), "the stepping shortcuts keep their route");
    assert_eq!(view_act_of("norm.shortcut.copy"), None);
}

#[test]
fn an_open_edit_menu_closes_on_an_outside_tap_and_on_escape_and_a_closed_one_leaves_both_alone() {
    let scope = "norm-en1995-viewport";
    let domain = "norm.en1995";
    let escape = |scope: &str, ctrl: bool| ShellKeyEvent { scope: scope.to_owned(), key: "Escape".to_owned(), ctrl, alt: false, shift: false, meta: false };
    note_touch_menu(domain, false);
    assert_eq!(touch_gesture_outcome(scope, ShellTouchGesture::Tap), ShellKeyOutcome::Ignored, "a tap with no open menu is the shell's");
    assert!(!escape_closes_menu(&escape(scope, false)), "Escape with no open menu keeps its other meanings");
    note_touch_menu(domain, true);
    assert_eq!(touch_gesture_outcome(scope, ShellTouchGesture::Tap), ShellKeyOutcome::Action { action: TOUCH_MENU_CLOSE.to_owned() }, "an outside tap closes the menu");
    assert_eq!(touch_gesture_outcome("norm-en1995-viewport-2", ShellTouchGesture::Tap), ShellKeyOutcome::Action { action: TOUCH_MENU_CLOSE.to_owned() }, "a second viewport instance of the same artifact shares the menu");
    assert_eq!(touch_gesture_outcome("norm-en1995-checks", ShellTouchGesture::Tap), ShellKeyOutcome::Ignored, "a tap in another window is not an outside tap of this menu");
    assert_eq!(NormShellHooks.handle_key(&escape(scope, false)), ShellKeyOutcome::Action { action: TOUCH_MENU_CLOSE.to_owned() }, "Escape (and the back key mapped to it) closes the menu first");
    assert!(!escape_closes_menu(&escape(scope, true)), "Ctrl+Escape is another chord");
    assert!(!escape_closes_menu(&escape("norm-en1996-viewport", false)), "the menu of another artifact is not closed");
    note_touch_menu(domain, false);
    assert_eq!(touch_gesture_outcome(scope, ShellTouchGesture::Tap), ShellKeyOutcome::Ignored);
}

#[test]
fn the_touch_profile_follows_the_device_class_and_the_shell_asks_it_per_signal() {
    for (width, touch, class) in [(390, true, DeviceClass::Mobile), (900, true, DeviceClass::Tablet), (1440, false, DeviceClass::Desktop), (1440, true, DeviceClass::Desktop)] {
        let profile = NormShellHooks.touch_profile(&ShellDeviceSignal { width_px: width, touch });
        assert_eq!(profile, touch_profile_of(class), "{width} {touch}");
        assert_eq!((profile.one_finger_orbit, profile.long_press, profile.hit_px), (class == DeviceClass::Mobile, true, device_profile(class).min_target_px), "{class:?}");
    }
    assert!(touch_profile_of(DeviceClass::Mobile).hit_px >= 44 && touch_profile_of(DeviceClass::Tablet).hit_px >= 44);
    semio_framework::register_shell_hooks("norm-touch", Arc::new(NormShellHooks));
    let mut driver = ShellHookDriver::default();
    driver.observe("norm-touch", EDITOR, 390, None, "en");
    assert!(driver.touch_profile("norm-touch").one_finger_orbit, "a phone orbits with one finger");
    driver.observe("norm-touch", EDITOR, 900, None, "en");
    assert!(!driver.touch_profile("norm-touch").one_finger_orbit, "a tablet drags handles with one finger");
    assert_eq!(driver.handle_touch("norm-touch", "norm-en1993-viewport", ShellTouchGesture::LongPress, 1.0, 2.0), ShellKeyOutcome::Action { action: TOUCH_MENU_OPEN.to_owned() });
}

#[test]
fn the_framework_touch_lane_realises_the_gesture_table_of_every_device_class() {
    use crate::device_profiles::gestures::{map_gesture, Gesture, Intent};
    use semio_framework::interaction::gesture::{TouchSession, TouchStep, LONG_PRESS_MS};
    use semio_plugin::{MergeMode, SelectionMethod};
    let lane = |class: DeviceClass| {
        let profile = touch_profile_of(class);
        let mut session = TouchSession::default();
        session.one_finger_orbit = profile.one_finger_orbit;
        session.long_press = profile.long_press;
        session
    };
    for class in [DeviceClass::Mobile, DeviceClass::Tablet, DeviceClass::Desktop] {
        let mut session = lane(class);
        session.down(0.0, 1, 100.0, 100.0);
        assert!(matches!(session.tick(LONG_PRESS_MS).as_slice(), [TouchStep::Press { additive: true, .. }, TouchStep::LongPress { .. }]), "{class:?}: a held finger selects additively and opens the menu");
        let released = session.up(LONG_PRESS_MS + 10.0, 1);
        assert!(matches!(released.as_slice(), [TouchStep::Release { additive: true, .. }]), "{class:?}: release ends the already announced hold: {released:?}");
        assert_eq!(map_gesture(class, Gesture::LongPress), Intent::Select { method: SelectionMethod::Pick, merge: MergeMode::Additive }, "{class:?}: the additive press is the table's long press");
        let mut rectangle = lane(class);
        rectangle.down(0.0, 1, 100.0, 100.0);
        rectangle.tick(LONG_PRESS_MS);
        assert!(matches!(rectangle.moved(LONG_PRESS_MS + 5.0, 1, 160.0, 140.0).as_slice(), [TouchStep::Drag { additive: true, .. }]), "{class:?}: a long press that moves drags a rectangle");
        assert!(matches!(rectangle.up(LONG_PRESS_MS + 20.0, 1).as_slice(), [TouchStep::Release { additive: true, .. }]), "{class:?}: and asks for no menu");
        assert_eq!(map_gesture(class, Gesture::LongPressDrag), Intent::Select { method: SelectionMethod::Rectangle, merge: MergeMode::Additive });
        let mut drag = lane(class);
        drag.down(0.0, 1, 100.0, 100.0);
        let steps = drag.moved(10.0, 1, 100.0, 160.0);
        let orbits = drag.moved(20.0, 1, 100.0, 180.0).iter().any(|step| matches!(step, TouchStep::Orbit { .. }));
        assert_eq!(orbits, map_gesture(class, Gesture::Drag { fingers: 1, on_handle: false }) == Intent::Orbit && class == DeviceClass::Mobile, "{class:?}: only tap-to-nudge devices orbit with one finger");
        assert_eq!(steps.iter().any(|step| matches!(step, TouchStep::Press { additive: false, .. })), class != DeviceClass::Mobile, "{class:?}: the other devices press where the drag began, so a handle or a marquee takes it");
        assert_eq!(map_gesture(class, Gesture::Drag { fingers: 1, on_handle: true }), if class == DeviceClass::Mobile { Intent::FocusHandle } else { Intent::HandleDrag }, "{class:?}: a drag on a handle never orbits");
        let mut pinch = lane(class);
        pinch.down(0.0, 1, 100.0, 100.0);
        pinch.down(5.0, 2, 200.0, 100.0);
        assert!(matches!(pinch.moved(10.0, 2, 300.0, 100.0).as_slice(), [TouchStep::Camera(step)] if (step.scale - 2.0).abs() < 1.0e-12), "{class:?}: two fingers are the camera");
        assert_eq!(map_gesture(class, Gesture::Drag { fingers: 2, on_handle: false }), Intent::Pan);
        assert_eq!(map_gesture(class, Gesture::Pinch { scale: 1.5 }), Intent::Zoom { factor: 1.5 });
    }
}
