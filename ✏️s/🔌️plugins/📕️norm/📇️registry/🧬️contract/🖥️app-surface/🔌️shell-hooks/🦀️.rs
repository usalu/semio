//! 🔌️ Norm shell hooks: the norm side of the framework's shell hook seam, so the shell chooses the device layout from the user's stored layout preferences, the keyboard shortcut map runs before the shell defaults, the properties inspector sits in a bottom sheet on a phone and the scene reads aloud as a list.
//!
//! 🔗️ Seam: `semio_framework::PluginShellHooks`; layouts: [`visual_layout_with`]; map: [`dispatch_shortcut`]; routes: [`shortcut_route`] (undo and redo stay with the shell's reserved edit chords, which run the store's history; select all raises the framework `selectAll` verb through [`select_all_outcome`]; the stepping keys and the entity chords become app actions); names: [`entity_node`](crate::accessibility::screen_reader::entity_node); see `visual-editor-architecture.md` §6.3, §8.2 and §8.4.

use crate::*;
use crate::accessibility::focus_order::{kebab, shortcut_action_id, shortcuts, Chord, Region, Scope, ShortcutAction};
use crate::accessibility::published_nodes::{publish, published, viewport_nodes, window_scope, PublishedNode};
use crate::accessibility::screen_reader::AriaNode;
use crate::accessibility::A11yPrefs;
use crate::device_profiles::{classify, device_profile, device_view, DeviceSignal, HandleMode, LayoutPrefs, SheetState};
use crate::direct_manipulation::{Key, Modifiers};
use crate::viewport3d::config::{DeviceClass, Lang};
use semio_framework::{PluginShellHooks, ShellAccessibleNode, ShellBottomSheet, ShellDeviceSignal, ShellKeyEvent, ShellKeyOutcome, ShellLayoutChoice, ShellLayoutRequest, ShellPlatformPrefs, ShellSheetState, ShellTouchEvent, ShellTouchGesture, ShellTouchProfile};
use semio_framework_pack_json::{from_dsl_value, parse, to_dsl_value, to_string, JsonMemberPolicy};
use semio_framework_value::{FromValue, ToValue};
use semio_s_plugin_norm_structural_scene::{Handle, StructuralScene};
use std::sync::{Mutex, PoisonError};

#[path = "🎯️grip-focus/🦀️.rs"]
mod grip_focus;
pub use grip_focus::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

static STORED_PREFS: Mutex<Option<LayoutPrefs>> = Mutex::new(None);
static CURRENT_SHEET: Mutex<Option<ShellBottomSheet>> = Mutex::new(None);
static CURRENT_VARIANT: Mutex<Option<&'static str>> = Mutex::new(None);
static PLATFORM_PREFS: Mutex<A11yPrefs> = Mutex::new(A11yPrefs { forced_colors: false, prefers_contrast_more: false, prefers_dark: false, prefers_reduced_motion: false });

const VARIANTS: [&str; 10] = ["en1990", "en1991", "en1992", "en1993", "en1994", "en1995", "en1996", "en1997", "en1998", "en1999"];

/// 🗄️ The user's layout preferences the hook falls back to when the shell passes none; local-only, persisted by the host under [`STORAGE_KEY`](crate::device_profiles::STORAGE_KEY).
pub fn stored_layout_prefs() -> LayoutPrefs {
    STORED_PREFS.lock().unwrap_or_else(PoisonError::into_inner).clone().unwrap_or_default()
}

/// 💾 Remembers the user's layout preferences for the next layout choice.
pub fn store_layout_prefs(prefs: &LayoutPrefs) {
    *STORED_PREFS.lock().unwrap_or_else(PoisonError::into_inner) = Some(prefs.clone());
}

/// 📤 The preferences as the JSON text the host stores them in.
pub fn encode_layout_prefs(prefs: &LayoutPrefs) -> String {
    to_string(&from_dsl_value(&prefs.to_value()))
}

/// 📥 The preferences from their stored JSON text; `None` when the text is not a preference record.
pub fn decode_layout_prefs(text: &str) -> Option<LayoutPrefs> {
    LayoutPrefs::from_value(to_dsl_value(&parse(text, JsonMemberPolicy::Reject).ok()?)).ok()
}

/// 🖐️ The touch behaviour of a device class, what the shell's touch lane asks of the plugin: tap-to-nudge devices send a one-finger drag to the camera (a drag never edits a handle there, the nudge buttons do), every class turns a held finger into an additive press that asks for the Edit menu, and the hit edge is the profile minimum.
pub fn touch_profile_of(class: DeviceClass) -> ShellTouchProfile {
    let profile = device_profile(class);
    ShellTouchProfile { one_finger_orbit: profile.handle_mode == HandleMode::TapNudge, long_press: true, hit_px: profile.min_target_px }
}

fn viewport_domain(scope: &str) -> Option<String> {
    variant_of_window(scope).filter(|_| scope_of(scope) == Scope::Viewport).map(|variant| format!("norm.{variant}"))
}

/// 🖐️ What a recognised touch gesture in the window `scope` does: a long press opens the Edit menu of the viewport while the finger is still down (an app action, so it routes like a chord); a tap on the surface closes an open menu (an outside tap); a double tap toggles the camera between the whole scene and the selected entities; everything else is left to the shell.
pub fn touch_gesture_outcome(scope: &str, gesture: ShellTouchGesture) -> ShellKeyOutcome {
    let Some(domain) = viewport_domain(scope) else { return ShellKeyOutcome::Ignored };
    match gesture {
        ShellTouchGesture::LongPress => ShellKeyOutcome::Action { action: TOUCH_MENU_OPEN.to_owned() },
        ShellTouchGesture::Tap if touch_menu_open(&domain) => ShellKeyOutcome::Action { action: TOUCH_MENU_CLOSE.to_owned() },
        ShellTouchGesture::DoubleTap => ShellKeyOutcome::Action { action: TOUCH_FRAME_SELECTION.to_owned() },
        ShellTouchGesture::Tap => ShellKeyOutcome::Ignored,
    }
}

/// ⎋ Whether `event` is the bare Escape (or the back key the shell maps to it) of an open Edit menu in a norm viewport window: it closes the menu before any other Escape meaning.
pub fn escape_closes_menu(event: &ShellKeyEvent) -> bool {
    event.key.eq_ignore_ascii_case("escape") && !(event.ctrl || event.alt || event.meta || event.shift) && viewport_domain(&event.scope).is_some_and(|domain| touch_menu_open(&domain))
}

/// 🪟️ The bottom sheet of the last layout choice, `None` unless the device is a phone with something selected; the inspector wraps itself in it.
pub fn current_sheet() -> Option<ShellBottomSheet> {
    CURRENT_SHEET.lock().unwrap_or_else(PoisonError::into_inner).clone()
}

/// 🪟️ Wraps a window body in the bottom sheet of the last layout choice, or returns it unchanged without one.
pub fn in_bottom_sheet(body: BuiltNode) -> UiAssemblyResult<BuiltNode> {
    match current_sheet() {
        None => Ok(body),
        Some(sheet) => semio_framework::bottom_sheet_node(&sheet, "norm-properties-sheet", vec![body]).ok_or_else(|| ui_error("ui.node.build")),
    }
}

/// 🗣️ The scene as the screen reader reads it: the scene list with one option per entity, in scene order, named by the same text as the viewport instances.
pub fn scene_accessible_nodes(scene: &StructuralScene, selected: &[String], lang: Lang) -> Vec<ShellAccessibleNode> {
    shell_nodes(viewport_nodes(scene, selected, &[], lang))
}

/// 🎛️ The accessibility preferences of the platform as the shell last reported them (all `false` before the first report); the viewport paint reads them for forced colours, more contrast and reduced motion.
pub fn platform_a11y_prefs() -> A11yPrefs {
    *PLATFORM_PREFS.lock().unwrap_or_else(PoisonError::into_inner)
}

/// 🎛️ The norm preferences a shell report stands for: the four platform answers one to one.
pub fn a11y_prefs_of(prefs: &ShellPlatformPrefs) -> A11yPrefs {
    A11yPrefs { forced_colors: prefs.forced_colors, prefers_contrast_more: prefs.prefers_contrast_more, prefers_dark: prefs.prefers_dark, prefers_reduced_motion: prefs.prefers_reduced_motion }
}

/// 📣 Publishes the scene nodes the viewport window just lowered, for [`PluginShellHooks::accessible_nodes`].
pub fn publish_scene_access(scene: &StructuralScene, selected: &[String], lang: Lang) {
    publish_viewport_access("", scene, selected, &[], lang);
}

/// 📣 Publishes the viewport window of the artifact `domain`: the scene list, an option per entity and a slider per handle of the selection (value, limits and key help in the shell language).
pub fn publish_viewport_access(domain: &str, scene: &StructuralScene, selected: &[String], handles: &[Handle], lang: Lang) {
    note_grips(domain, handles.iter().map(|handle| GripRef { id: handle.id.clone(), entity: handle.entity.id.clone() }).collect());
    publish("viewport", domain, viewport_nodes(scene, selected, handles, lang));
}

/// 📣 Publishes the check rows the checks window just drew, for [`PluginShellHooks::accessible_nodes`].
pub fn publish_checks_access(domain: &str, nodes: Vec<PublishedNode>) {
    publish("checks", domain, nodes);
}

fn shell_nodes(nodes: Vec<PublishedNode>) -> Vec<ShellAccessibleNode> {
    nodes.into_iter().map(|PublishedNode { key, node, depth }| shell_node(key, node, depth)).collect()
}

fn shell_node(key: String, node: AriaNode, depth: usize) -> ShellAccessibleNode {
    ShellAccessibleNode {
        key,
        role: node.role.to_owned(),
        name: node.name,
        description: node.description,
        role_description: node.role_description,
        value_text: node.value_text,
        position: node.position.map(|(position, _)| position),
        set_size: node.position.map(|(_, size)| size),
        selected: node.selected,
        depth,
    }
}

fn variant_of(app_id: &str) -> Option<&'static str> {
    VARIANTS.into_iter().find(|variant| app_id.contains(variant))
}

/// 🪟️ The artifact variant a window id belongs to (`norm-en1993-viewport-2` is `en1993`).
pub fn variant_of_window(window_id: &str) -> Option<&'static str> {
    let mut parts = window_id.split('-');
    (parts.next() == Some("norm")).then(|| parts.next()).flatten().and_then(|variant| VARIANTS.into_iter().find(|candidate| *candidate == variant))
}

/// 🎯️ The window kind id of `region` for the open artifact: the variant comes from the window `scope` the key or the menu happened in, else from the app of the last layout choice.
pub fn window_kind_of_region(scope: &str, region: Region) -> Option<String> {
    let variant = variant_of_window(scope).or_else(|| *CURRENT_VARIANT.lock().unwrap_or_else(PoisonError::into_inner))?;
    Some(visual_window_id(variant, region.role()))
}

/// 🎯️ Asks the shell to focus the window of `region` through the framework `focusWindow` verb (open it when closed, raise it, move the keyboard focus in, announce it); every device layout holds all five windows (the outliner is a tab on mobile and tablet), so there is no stand-in; `Ignored` while no artifact variant is known.
pub fn focus_window_outcome(scope: &str, region: Region) -> ShellKeyOutcome {
    window_kind_of_region(scope, region).map_or(ShellKeyOutcome::Ignored, |kind| ShellKeyOutcome::verb(semio_framework::FOCUS_WINDOW_ACTION_ID, [(semio_framework::FOCUS_WINDOW_ARG_KIND.to_owned(), kind)]))
}

fn scope_for(event: &ShellKeyEvent) -> Scope {
    match window_scope(&event.scope) {
        Some(("viewport", domain)) if focused_grip(&domain).is_some() => Scope::Handle,
        _ => scope_of(&event.scope),
    }
}

fn scope_of(window: &str) -> Scope {
    if window.contains("handle") {
        Scope::Handle
    } else if window.contains("viewport") {
        Scope::Viewport
    } else {
        Scope::Global
    }
}

fn sheet_state(state: SheetState) -> ShellSheetState {
    match state {
        SheetState::Collapsed => ShellSheetState::Collapsed,
        SheetState::Half => ShellSheetState::Half,
        SheetState::Full => ShellSheetState::Full,
    }
}

/// 🔌️ The norm plugin's hooks: install them under the norm plugin id with `ShellHookRegistry::install`.
#[derive(Clone, Copy, Debug, Default)]
pub struct NormShellHooks;

impl PluginShellHooks for NormShellHooks {
    fn choose_layout(&self, request: &ShellLayoutRequest) -> Option<ShellLayoutChoice> {
        let variant = variant_of(&request.app_id)?;
        *CURRENT_VARIANT.lock().unwrap_or_else(PoisonError::into_inner) = Some(variant);
        let viewer = request.app_id.ends_with("#viewer");
        let prefs = request.prefs.as_deref().and_then(decode_layout_prefs).unwrap_or_else(stored_layout_prefs);
        let view = device_view(&DeviceSignal { width_px: request.signal.width_px, touch: request.signal.touch }, &prefs, request.has_selection);
        crate::device_profiles::note_device(view.class);
        let layout = if viewer { view_layout_with(variant, view.class, &prefs) } else { visual_layout_with(variant, view.class, &prefs) };
        let german = request.lang == "de";
        let sheet = view.sheet.filter(|_| !viewer).map(|spec| ShellBottomSheet { label: (if german { "Eigenschaften" } else { "Properties" }).to_owned(), state: sheet_state(spec.state), height: spec.height, min_handle_px: spec.min_handle_px });
        *CURRENT_SHEET.lock().unwrap_or_else(PoisonError::into_inner) = sheet.clone();
        Some(ShellLayoutChoice { layout_id: view.layout_id, layout, sheet })
    }

    fn prefs_key(&self) -> Option<String> {
        Some(crate::device_profiles::STORAGE_KEY.to_owned())
    }

    fn handle_key(&self, event: &ShellKeyEvent) -> ShellKeyOutcome {
        if escape_closes_menu(event) {
            return ShellKeyOutcome::Action { action: TOUCH_MENU_CLOSE.to_owned() };
        }
        let Some(key) = shortcuts().iter().map(|shortcut| shortcut.chord.key).find(|key| key.eq_ignore_ascii_case(&event.key)) else { return ShellKeyOutcome::Ignored };
        let chord = Chord { key, ctrl: event.ctrl || event.meta, alt: event.alt, shift: event.shift };
        let domain = window_scope(&event.scope).map(|(_, domain)| domain).unwrap_or_default();
        match crate::dispatch_shortcut(scope_for(event), chord) {
            None => ShellKeyOutcome::Ignored,
            Some(crate::KeyCommand::Host(ShortcutAction::EnterHandles)) if focused_grip(&domain).is_none() && walk_focus(&domain, GripWalk::Forward) != GripStep::Nothing => ShellKeyOutcome::Consumed,
            Some(crate::KeyCommand::Host(ShortcutAction::CycleGripInEntity { forward })) => match walk_focus(&domain, if forward { GripWalk::ForwardInEntity } else { GripWalk::BackwardInEntity }) {
                GripStep::Nothing => ShellKeyOutcome::Ignored,
                _ => ShellKeyOutcome::Consumed,
            },
            Some(crate::KeyCommand::Handle { key: Key::Tab, modifiers }) => match walk_focus(&domain, if modifiers.shift { GripWalk::Backward } else { GripWalk::Forward }) {
                GripStep::To(_) => ShellKeyOutcome::Consumed,
                GripStep::Leave | GripStep::Nothing => ShellKeyOutcome::Ignored,
            },
            Some(crate::KeyCommand::Handle { key: Key::Escape, .. }) => {
                set_focused_grip(&domain, None);
                ShellKeyOutcome::Action { action: shortcut_action_id(&ShortcutAction::CancelHandle) }
            }
            Some(crate::KeyCommand::Host(action)) => {
                let id = shortcut_action_id(&action);
                match shortcut_route(&id) {
                    HostRoute::History(_) => ShellKeyOutcome::Ignored,
                    HostRoute::SelectAll => select_all_outcome(&event.scope),
                    HostRoute::FocusWindow(region) => focus_window_outcome(&event.scope, region),
                    HostRoute::ViewAct(_) | HostRoute::Entity(_) | HostRoute::Local => ShellKeyOutcome::Action { action: id },
                }
            }
            Some(crate::KeyCommand::Handle { key: handle_key, modifiers }) => {
                let key_name = kebab(&format!("{handle_key:?}"));
                let step = match (key_name.as_str(), modifiers) {
                    ("tab", m) => if m.shift { "backward" } else { "forward" },
                    (_, m) if m == Modifiers::ALT => "fine",
                    (_, m) if m == Modifiers::SHIFT => "large",
                    _ => "normal",
                };
                ShellKeyOutcome::Action { action: format!("norm.handle.{key_name}.{step}") }
            }
        }
    }

    fn platform_prefs_changed(&self, prefs: &ShellPlatformPrefs) {
        *PLATFORM_PREFS.lock().unwrap_or_else(PoisonError::into_inner) = a11y_prefs_of(prefs);
    }

    fn touch_profile(&self, signal: &ShellDeviceSignal) -> ShellTouchProfile {
        touch_profile_of(classify(signal.width_px, signal.touch))
    }

    fn handle_touch(&self, event: &ShellTouchEvent) -> ShellKeyOutcome {
        touch_gesture_outcome(&event.scope, event.gesture)
    }

    fn accessible_nodes(&self, scope: &str, lang: &str) -> Vec<ShellAccessibleNode> {
        let _ = lang;
        window_scope(scope).map(|(role, domain)| shell_nodes(published(role, &domain))).unwrap_or_default()
    }
}
