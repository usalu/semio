//! 👆️ Touch and pencil gestures mapped onto the interaction domain: tap, long-press and rectangle select through `interactionSelect` methods and merge modes, camera gestures for orbit, pan and zoom, and handle input per [`HandleMode`].
//!
//! 🔗️ Every selection intent is a method and merge the generated [`viewport_interaction_definition`](crate::viewport_interaction_definition) declares; see `visual-editor-architecture.md` §8.4.

use super::*;
use semio_plugin::{MergeMode, SelectionMethod};

pub const LONG_PRESS_MS: u32 = 450;
pub const DOUBLE_TAP_MS: u32 = 300;
pub const ZOOM_RANGE: (f64, f64) = (0.25, 4.0);

/// 👆️ One recognised gesture; `on_handle` is true when the touch started inside a handle's hit area.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Gesture {
    Tap { on_handle: bool },
    DoubleTap,
    LongPress,
    LongPressDrag,
    Drag { fingers: u8, on_handle: bool },
    Pinch { scale: f64 },
    NudgeButton { direction: i8 },
    PencilHover { on_handle: bool },
}

/// 🧭️ What the host does with a gesture.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Intent {
    Select { method: SelectionMethod, merge: MergeMode },
    Frame,
    Orbit,
    Pan,
    Zoom { factor: f64 },
    FocusHandle,
    HandleDrag,
    HandleNudge { direction: f64 },
    Hover { on_handle: bool },
    Ignore,
}

/// 🔭️ Zoom factor of a pinch, kept within [`ZOOM_RANGE`]; a degenerate scale does not zoom.
pub fn zoom_factor(scale: f64) -> f64 {
    if scale.is_finite() && scale > 0.0 { scale.clamp(ZOOM_RANGE.0, ZOOM_RANGE.1) } else { 1.0 }
}

/// 🧭️ The intent of a gesture on a device class: one finger selects and orbits, two pan, a pinch zooms, long-press adds to the selection; handles drag freely where [`HandleMode::FreeDrag`] and are focused for the nudge buttons where [`HandleMode::TapNudge`], where a drag on a handle neither edits nor orbits. Pencil hover acts as hover on the tablet only.
pub fn map_gesture(class: DeviceClass, gesture: Gesture) -> Intent {
    let profile = device_profile(class);
    let select = |method, merge| Intent::Select { method, merge };
    match gesture {
        Gesture::Tap { on_handle: true } => Intent::FocusHandle,
        Gesture::Tap { on_handle: false } => select(SelectionMethod::Pick, MergeMode::Replace),
        Gesture::DoubleTap => Intent::Frame,
        Gesture::LongPress => select(SelectionMethod::Pick, MergeMode::Additive),
        Gesture::LongPressDrag => select(SelectionMethod::Rectangle, MergeMode::Additive),
        Gesture::Drag { fingers: 1, on_handle: true } if profile.handle_mode == HandleMode::FreeDrag => Intent::HandleDrag,
        Gesture::Drag { fingers: 1, on_handle: true } => Intent::FocusHandle,
        Gesture::Drag { fingers: 1, .. } => Intent::Orbit,
        Gesture::Drag { fingers: 2, .. } => Intent::Pan,
        Gesture::Drag { .. } => Intent::Ignore,
        Gesture::Pinch { scale } => Intent::Zoom { factor: zoom_factor(scale) },
        Gesture::NudgeButton { direction } if profile.nudge_buttons && direction != 0 => Intent::HandleNudge { direction: f64::from(direction.signum()) },
        Gesture::NudgeButton { .. } => Intent::Ignore,
        Gesture::PencilHover { on_handle } if class == DeviceClass::Tablet => Intent::Hover { on_handle },
        Gesture::PencilHover { .. } => Intent::Ignore,
    }
}
