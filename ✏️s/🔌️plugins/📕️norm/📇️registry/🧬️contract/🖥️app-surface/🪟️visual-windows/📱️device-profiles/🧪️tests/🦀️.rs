//! 🧪️ Device profile tests; scenarios in `🥒️.feature`. The layout shape is read back from the framework layout of [`visual_layout`].

use super::*;
use gestures::*;
use semio_plugin::{MergeMode, SelectionMethod};
use semio_plugin::{WindowLayoutChild, WindowLayoutRoot};
use semio_ui_locale::LocalizedLabel;
use semio_framework_value::{FromValue, ToValue};
use semio_s_plugin_norm_structural_scene::EntityKind;

fn windows(root: &WindowLayoutRoot) -> usize {
    fn count(child: &WindowLayoutChild) -> usize {
        match child {
            WindowLayoutChild::Stack(s) => s.children.len(),
            WindowLayoutChild::Axis(a) => a.children.iter().map(count).sum(),
        }
    }
    match root {
        WindowLayoutRoot::Stack(s) => s.children.len(),
        WindowLayoutRoot::Axis(a) => a.children.iter().map(count).sum(),
    }
}

#[test]
fn priority_is_desktop_then_mobile_then_tablet() {
    assert_eq!(PRIORITY, [DeviceClass::Desktop, DeviceClass::Mobile, DeviceClass::Tablet]);
    let budgets: Vec<usize> = PRIORITY.iter().map(|c| device_profile(*c).viewport.instance_budget).collect();
    assert!(budgets[0] > budgets[2] && budgets[2] > budgets[1]);
}

#[test]
fn touch_devices_get_large_targets_and_no_hover() {
    for class in PRIORITY {
        let p = device_profile(class);
        assert_eq!(p.min_target_px, if class == DeviceClass::Desktop { 24 } else { 44 });
        assert_eq!(p.hover, class == DeviceClass::Desktop);
        assert_eq!(p.handle_hit_px(10), p.min_target_px);
        assert_eq!(p.handle_hit_px(60), 60);
        assert_eq!(p.class, class);
    }
    assert_eq!(device_profile(DeviceClass::Mobile).handle_mode, HandleMode::TapNudge);
    assert!(device_profile(DeviceClass::Mobile).nudge_buttons && device_profile(DeviceClass::Tablet).nudge_buttons);
    assert!(!device_profile(DeviceClass::Desktop).nudge_buttons);
}

#[test]
fn profile_agrees_with_the_window_layout() {
    let desktop = crate::visual_layout("en1993", DeviceClass::Desktop).root;
    assert_eq!(windows(&desktop), device_profile(DeviceClass::Desktop).visible_windows as usize);
    assert!(!device_profile(DeviceClass::Desktop).tabbed);
    let mobile = crate::visual_layout("en1993", DeviceClass::Mobile).root;
    assert!(matches!(mobile, WindowLayoutRoot::Stack(_)) && device_profile(DeviceClass::Mobile).tabbed);
    assert_eq!((windows(&mobile), device_profile(DeviceClass::Mobile).visible_windows), (4, 1));
    let tablet = crate::visual_layout("en1993", DeviceClass::Tablet).root;
    let WindowLayoutRoot::Axis(axis) = &tablet else { panic!("row") };
    assert_eq!(axis.children.len(), device_profile(DeviceClass::Tablet).visible_windows as usize);
}

#[test]
fn budgets_shrink_on_smaller_devices_and_debounce_grows() {
    let (d, t, m) = (device_profile(DeviceClass::Desktop), device_profile(DeviceClass::Tablet), device_profile(DeviceClass::Mobile));
    assert!(d.diagram_point_budget > t.diagram_point_budget && t.diagram_point_budget > m.diagram_point_budget);
    assert!(d.live_debounce_ms < t.live_debounce_ms && t.live_debounce_ms < m.live_debounce_ms);
    assert!(d.heatmap_stations > t.heatmap_stations && t.heatmap_stations > m.heatmap_stations);
    assert_eq!(m.thin(5_000), 2_000);
    assert_eq!(m.thin(100), 100);
}

#[test]
fn classification_boundaries() {
    assert_eq!(classify(599, true), DeviceClass::Mobile);
    assert_eq!(classify(600, true), DeviceClass::Tablet);
    assert_eq!(classify(600, false), DeviceClass::Desktop);
    assert_eq!(classify(1023, true), DeviceClass::Tablet);
    assert_eq!(classify(1024, true), DeviceClass::Desktop);
    assert_eq!(classify(1920, false), DeviceClass::Desktop);
}

fn close(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    assert!(actual.iter().zip(expected).all(|(a, e)| (a - e).abs() < 1e-9), "{actual:?} != {expected:?}");
}

fn size(child: &WindowLayoutChild) -> f64 {
    match child {
        WindowLayoutChild::Stack(s) => s.size.expect("size"),
        WindowLayoutChild::Axis(a) => a.size.expect("size"),
    }
}

fn roles(child: &WindowLayoutChild) -> Vec<String> {
    match child {
        WindowLayoutChild::Stack(s) => s.children.iter().map(|w| w.window_kind_id.rsplit('-').next().expect("role").to_owned()).collect(),
        WindowLayoutChild::Axis(a) => a.children.iter().flat_map(roles).collect(),
    }
}

fn root_children(root: &WindowLayoutRoot) -> &[WindowLayoutChild] {
    match root {
        WindowLayoutRoot::Axis(a) => &a.children,
        WindowLayoutRoot::Stack(_) => panic!("axis"),
    }
}

fn layout(device: DeviceClass, prefs: &LayoutPrefs) -> WindowLayoutRoot {
    crate::visual_layout_with("en1993", device, prefs).root
}

#[test]
fn default_preferences_reproduce_the_documented_layouts() {
    let prefs = LayoutPrefs::default();
    assert_eq!(validate_layout(&prefs), Vec::<String>::new());
    let desktop = layout(DeviceClass::Desktop, &prefs);
    close(&root_children(&desktop).iter().map(size).collect::<Vec<_>>(), &[0.22, 0.52, 0.26]);
    assert_eq!(windows(&desktop), windows(&crate::visual_layout("en1993", DeviceClass::Desktop).root));
    assert_eq!(root_children(&desktop).iter().flat_map(roles).collect::<Vec<_>>(), ["outliner", "properties", "viewport", "checks", "diagrams"]);
}

#[test]
fn the_dock_is_resizable_hideable_and_always_keeps_the_viewport() {
    let prefs = apply_layout(&LayoutPrefs::default(), &LayoutCommand::ResizeColumns { left: 0.3, centre: 0.4, right: 0.3 });
    let prefs = apply_layout(&prefs, &LayoutCommand::Split { left_column: true, share: 0.7 });
    let desktop = layout(DeviceClass::Desktop, &prefs);
    let columns = root_children(&desktop);
    close(&columns.iter().map(size).collect::<Vec<_>>(), &[0.3, 0.4, 0.3]);
    let WindowLayoutChild::Axis(left) = &columns[0] else { panic!("column") };
    close(&left.children.iter().map(size).collect::<Vec<_>>(), &[0.7, 0.3]);
    let prefs = apply_layout(&prefs, &LayoutCommand::SetHidden { role: "checks".into(), hidden: true });
    let prefs = apply_layout(&prefs, &LayoutCommand::SetHidden { role: "diagrams".into(), hidden: true });
    let prefs = apply_layout(&prefs, &LayoutCommand::SetHidden { role: "viewport".into(), hidden: true });
    assert!(prefs.dock.hidden == ["checks", "diagrams"]);
    let desktop = layout(DeviceClass::Desktop, &prefs);
    let columns = root_children(&desktop);
    assert_eq!(columns.len(), 2);
    assert!((columns.iter().map(size).sum::<f64>() - 1.0).abs() < 1e-12);
    assert_eq!(columns.iter().flat_map(roles).collect::<Vec<_>>(), ["outliner", "properties", "viewport"]);
    let prefs = apply_layout(&prefs, &LayoutCommand::SetHidden { role: "checks".into(), hidden: false });
    assert_eq!(root_children(&layout(DeviceClass::Desktop, &prefs)).iter().flat_map(roles).collect::<Vec<_>>(), ["outliner", "properties", "viewport", "checks"]);
    assert_eq!(apply_layout(&prefs, &LayoutCommand::Reset), LayoutPrefs::default());
}

#[test]
fn mobile_is_one_pane_with_a_reorderable_tab_bar_and_a_bottom_sheet() {
    let prefs = apply_layout(&LayoutPrefs::default(), &LayoutCommand::MoveTab { mobile: true, role: "checks".into(), to: 1 });
    let prefs = apply_layout(&prefs, &LayoutCommand::Activate { mobile: true, role: "checks".into() });
    let WindowLayoutRoot::Stack(tabs) = layout(DeviceClass::Mobile, &prefs) else { panic!("stack") };
    assert_eq!(tabs.children.iter().map(|w| w.window_kind_id.rsplit('-').next().expect("role").to_owned()).collect::<Vec<_>>(), ["viewport", "checks", "outliner", "properties", "diagrams"]);
    assert_eq!(tabs.active_window_kind_id, Some(crate::visual_window_id("en1993", "checks")));
    assert_eq!(bottom_sheet(DeviceClass::Mobile, &prefs, true), None, "the sheet sits over the model tab only");
    let prefs = apply_layout(&LayoutPrefs::default(), &LayoutCommand::Sheet(SheetState::Half));
    let sheet = bottom_sheet(DeviceClass::Mobile, &prefs, true).expect("sheet");
    assert_eq!((sheet.role, sheet.state, sheet.height, sheet.min_handle_px), ("properties", SheetState::Half, 0.5, 44));
    assert_eq!(bottom_sheet(DeviceClass::Mobile, &LayoutPrefs::default(), true).expect("peek").height, SHEET_PEEK);
    assert_eq!(bottom_sheet(DeviceClass::Mobile, &apply_layout(&prefs, &LayoutCommand::Sheet(SheetState::Full)), true).expect("full").height, SHEET_FULL);
    assert_eq!(bottom_sheet(DeviceClass::Mobile, &prefs, false), None);
    assert_eq!(bottom_sheet(DeviceClass::Tablet, &prefs, true), None);
    assert_eq!(bottom_sheet(DeviceClass::Desktop, &prefs, true), None);
}

#[test]
fn tablet_is_two_panes_with_a_tabbed_side_pane() {
    let prefs = apply_layout(&LayoutPrefs::default(), &LayoutCommand::ViewportShare(0.5));
    let prefs = apply_layout(&prefs, &LayoutCommand::MoveTab { mobile: false, role: "diagrams".into(), to: 0 });
    let prefs = apply_layout(&prefs, &LayoutCommand::Activate { mobile: false, role: "diagrams".into() });
    let tablet = layout(DeviceClass::Tablet, &prefs);
    let panes = root_children(&tablet);
    assert_eq!(panes.len(), 2);
    close(&panes.iter().map(size).collect::<Vec<_>>(), &[0.5, 0.5]);
    assert_eq!(roles(&panes[1]), ["diagrams", "outliner", "properties", "checks"]);
    let WindowLayoutChild::Stack(side) = &panes[1] else { panic!("stack") };
    assert_eq!(side.active_window_kind_id, Some(crate::visual_window_id("en1993", "diagrams")));
}

#[test]
fn any_stored_preference_is_sanitised_into_a_valid_one() {
    let mut bad = LayoutPrefs::default();
    bad.dock = DockPrefs { left: f64::NAN, centre: 9.0, right: -1.0, left_split: 0.0, right_split: f64::INFINITY, hidden: vec!["viewport".into(), "checks".into(), "checks".into(), "nope".into()] };
    bad.tablet = TabletPrefs { viewport: 0.0, side_tabs: vec!["checks".into(), "checks".into(), "viewport".into()], side_active: "viewport".into() };
    bad.mobile = MobilePrefs { tabs: vec!["diagrams".into()], active: "x".into(), sheet: SheetState::Half, sheet_height: f64::NAN };
    assert!(validate_layout(&bad).len() >= 8);
    let fixed = sanitised(&bad);
    assert_eq!(validate_layout(&fixed), Vec::<String>::new());
    assert_eq!(sanitised(&fixed), fixed);
    assert_eq!(fixed.dock.hidden, ["checks"]);
    assert_eq!((fixed.dock.left, fixed.dock.centre, fixed.dock.right), (0.22, 0.75, 0.08));
    assert_eq!(fixed.tablet.side_tabs, ["checks", "outliner", "properties", "diagrams"]);
    assert_eq!((fixed.tablet.side_active.as_str(), fixed.mobile.active.as_str()), ("checks", "diagrams"));
    for device in PRIORITY {
        assert!(windows(&layout(device, &bad)) >= 2);
    }
}

#[test]
fn layout_preferences_are_local_only_and_round_trip_through_the_value_codec() {
    assert_eq!((PERSISTENCE, STORAGE_KEY), ("persistedLocalOnly", "norm.visual.layout"));
    let mut prefs = LayoutPrefs::default();
    prefs.dock.hidden.push("diagrams".into());
    prefs.mobile.sheet = SheetState::Full;
    prefs.tablet.side_tabs.reverse();
    assert_eq!(LayoutPrefs::from_value(prefs.to_value()).expect("round trip"), prefs);
}

#[test]
fn every_layout_type_is_in_every_schema_twin() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../🪟️visual-windows/📱️device-profiles/🧬️schema");
    let rust = std::fs::read_to_string(dir.join("🦀️.rs")).expect("rust");
    let names: Vec<&str> = rust.lines().filter_map(|l| l.trim().strip_prefix("pub struct ").or_else(|| l.trim().strip_prefix("pub enum "))).map(|l| l.trim_end_matches(" {")).collect();
    assert_eq!(names, ["SheetState", "DockPrefs", "TabletPrefs", "MobilePrefs", "LayoutPrefs", "DeviceSignal"]);
    for twin in ["🛰️.proto", "🟦️.ts", "🔗️.graphql"] {
        let text = std::fs::read_to_string(dir.join(twin)).expect("twin");
        for n in &names {
            assert!(text.contains(&format!("{n} ")) || text.contains(&format!("{n}{{")), "{n} missing in {twin}");
        }
    }
}

fn domain() -> semio_plugin::InteractionDefinition {
    crate::viewport_interaction_definition("en1993", LocalizedLabel::native("Steel", "Stahl"), &[EntityKind::Member])
}

#[test]
fn touch_gestures_select_through_methods_the_interaction_domain_declares() {
    let domain = domain();
    let all = [Gesture::Tap { on_handle: false }, Gesture::LongPress, Gesture::LongPressDrag, Gesture::DoubleTap, Gesture::Pinch { scale: 2.0 }];
    for class in PRIORITY {
        for gesture in all {
            if let Intent::Select { method, merge } = map_gesture(class, gesture) {
                assert!(domain.selection.methods.contains(&method) && domain.selection.merges.contains(&merge), "{class:?} {gesture:?}");
            }
        }
    }
    assert_eq!(map_gesture(DeviceClass::Mobile, Gesture::Tap { on_handle: false }), Intent::Select { method: SelectionMethod::Pick, merge: MergeMode::Replace });
    assert_eq!(map_gesture(DeviceClass::Mobile, Gesture::LongPress), Intent::Select { method: SelectionMethod::Pick, merge: MergeMode::Additive });
    assert_eq!(map_gesture(DeviceClass::Tablet, Gesture::LongPressDrag), Intent::Select { method: SelectionMethod::Rectangle, merge: MergeMode::Additive });
    assert_eq!(map_gesture(DeviceClass::Mobile, Gesture::DoubleTap), Intent::Frame);
}

#[test]
fn camera_gestures_orbit_pan_and_zoom() {
    for class in PRIORITY {
        assert_eq!(map_gesture(class, Gesture::Drag { fingers: 1, on_handle: false }), Intent::Orbit);
        assert_eq!(map_gesture(class, Gesture::Drag { fingers: 2, on_handle: false }), Intent::Pan);
        assert_eq!(map_gesture(class, Gesture::Drag { fingers: 3, on_handle: false }), Intent::Ignore);
        assert_eq!(map_gesture(class, Gesture::Pinch { scale: 1.5 }), Intent::Zoom { factor: 1.5 });
    }
    assert_eq!((zoom_factor(100.0), zoom_factor(0.0), zoom_factor(f64::NAN), zoom_factor(0.01)), (4.0, 1.0, 1.0, 0.25));
}

#[test]
fn handles_drag_freely_on_desktop_and_tablet_and_nudge_by_buttons_on_mobile() {
    let drag = Gesture::Drag { fingers: 1, on_handle: true };
    assert_eq!(map_gesture(DeviceClass::Desktop, drag), Intent::HandleDrag);
    assert_eq!(map_gesture(DeviceClass::Tablet, drag), Intent::HandleDrag);
    assert_eq!(map_gesture(DeviceClass::Mobile, drag), Intent::Orbit, "no accidental edit while orbiting");
    for class in PRIORITY {
        assert_eq!(map_gesture(class, Gesture::Tap { on_handle: true }), Intent::FocusHandle);
    }
    assert_eq!(map_gesture(DeviceClass::Mobile, Gesture::NudgeButton { direction: -3 }), Intent::HandleNudge { direction: -1.0 });
    assert_eq!(map_gesture(DeviceClass::Tablet, Gesture::NudgeButton { direction: 1 }), Intent::HandleNudge { direction: 1.0 });
    assert_eq!(map_gesture(DeviceClass::Desktop, Gesture::NudgeButton { direction: 1 }), Intent::Ignore);
    assert_eq!(map_gesture(DeviceClass::Mobile, Gesture::NudgeButton { direction: 0 }), Intent::Ignore);
    assert_eq!(map_gesture(DeviceClass::Tablet, Gesture::PencilHover { on_handle: true }), Intent::Hover { on_handle: true });
    assert_eq!(map_gesture(DeviceClass::Mobile, Gesture::PencilHover { on_handle: true }), Intent::Ignore);
}

#[test]
fn touch_hit_areas_and_the_sheet_handle_are_at_least_44_px() {
    for class in [DeviceClass::Mobile, DeviceClass::Tablet] {
        let p = device_profile(class);
        for visual in [8, 16, 30, 44, 60] {
            assert!(p.handle_hit_px(visual) >= 44, "{class:?} {visual}");
        }
    }
    assert!(LONG_PRESS_MS > DOUBLE_TAP_MS);
}

#[test]
fn a_host_signal_selects_class_layout_id_profile_and_sheet() {
    let prefs = LayoutPrefs::default();
    let phone = device_view(&DeviceSignal { width_px: 390, touch: true }, &prefs, true);
    assert_eq!((phone.class, phone.layout_id.as_str(), phone.profile.min_target_px), (DeviceClass::Mobile, "norm-visual-mobile", 44));
    assert_eq!(phone.sheet.expect("sheet").role, "properties");
    let tablet = device_view(&DeviceSignal { width_px: 820, touch: true }, &prefs, true);
    assert_eq!((tablet.class, tablet.layout_id.as_str(), tablet.sheet), (DeviceClass::Tablet, "norm-visual-tablet", None));
    let desktop = device_view(&DeviceSignal { width_px: 1920, touch: false }, &prefs, false);
    assert_eq!((desktop.class, desktop.layout_id.as_str()), (DeviceClass::Desktop, "norm-visual-desktop"));
    assert_eq!(DeviceSignal::from_value(DeviceSignal { width_px: 1, touch: true }.to_value()).expect("round trip"), DeviceSignal { width_px: 1, touch: true });
}

#[test]
fn viewers_keep_only_model_checks_and_diagrams_and_every_device_has_a_named_layout() {
    for device in PRIORITY {
        let layout = crate::view_layout_with("en1993", device, &LayoutPrefs::default()).root;
        assert_eq!(windows(&layout), 3, "{device:?}");
        let ids = match &layout {
            WindowLayoutRoot::Axis(a) => a.children.iter().flat_map(roles).collect::<Vec<_>>(),
            WindowLayoutRoot::Stack(s) => s.children.iter().map(|w| w.window_kind_id.rsplit('-').next().expect("role").to_owned()).collect(),
        };
        assert!(ids.iter().all(|r| ["viewport", "checks", "diagrams"].contains(&r.as_str())), "{ids:?}");
    }
    let WindowLayoutRoot::Axis(tablet) = crate::view_layout_with("en1993", DeviceClass::Tablet, &LayoutPrefs::default()).root else { panic!("row") };
    assert_eq!(roles(&tablet.children[1]), ["checks", "diagrams"]);
    for (name, class) in [("desktop", DeviceClass::Desktop), ("mobile", DeviceClass::Mobile), ("tablet", DeviceClass::Tablet)] {
        let edit = crate::edit_named_layout("en1993", name);
        assert_eq!((edit.id.as_str(), edit.origin.as_str()), (crate::visual_layout_id(class).as_str(), "builtin"));
        assert_eq!(windows(&edit.layout.root), device_profile_windows(class));
        assert_eq!(crate::view_named_layout("en1993", name).id, edit.id);
    }
}

fn device_profile_windows(class: DeviceClass) -> usize {
    if class == DeviceClass::Desktop { 5 } else if class == DeviceClass::Mobile { 4 } else { 4 }
}
