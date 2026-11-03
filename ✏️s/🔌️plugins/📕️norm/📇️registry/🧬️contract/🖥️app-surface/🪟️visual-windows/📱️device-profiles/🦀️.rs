//! 📱️ Device profiles of the visual windows: level of detail, budgets, hit-target sizes, input mode and layout shape per device class, desktop first, then mobile, then tablet.
//!
//! 🔗️ Extends [`DeviceClass::profile`](crate::viewport3d::config::DeviceClass::profile) and agrees with [`visual_layout`](crate::visual_layout); see `visual-editor-architecture.md` §8.4.

use crate::viewport3d::config::{DeviceClass, DeviceProfile};
use std::sync::{Mutex, PoisonError};

static CURRENT_DEVICE: Mutex<Option<DeviceClass>> = Mutex::new(None);

#[path = "🧬️schema/🦀️.rs"]
pub mod schema;
pub use schema::*;

#[path = "📐️layout/🦀️.rs"]
pub mod layout;
pub use layout::*;

#[path = "👆️gestures/🦀️.rs"]
pub mod gestures;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

/// ✋️ How a handle is moved.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HandleMode {
    FreeDrag,
    TapNudge,
}

/// 🗂️ Where the properties inspector lives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InspectorPlacement {
    Docked,
    BottomSheet,
    SidePane,
}

/// 📐️ Everything a window needs to adapt to a device class.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DeviceProfileSpec {
    pub class: DeviceClass,
    pub viewport: DeviceProfile,
    pub min_target_px: u32,
    pub hover: bool,
    pub handle_mode: HandleMode,
    pub nudge_buttons: bool,
    pub inspector: InspectorPlacement,
    pub tabbed: bool,
    pub visible_windows: u8,
    pub diagram_point_budget: usize,
    pub live_debounce_ms: u32,
    pub frame_budget_ms: u32,
    pub heatmap_stations: usize,
}

/// 📱️ Device classes in priority order.
pub const PRIORITY: [DeviceClass; 3] = [DeviceClass::Desktop, DeviceClass::Mobile, DeviceClass::Tablet];

/// 📐️ The profile of a device class.
pub fn device_profile(class: DeviceClass) -> DeviceProfileSpec {
    let viewport = class.profile();
    match class {
        DeviceClass::Desktop => DeviceProfileSpec { class, viewport, min_target_px: 24, hover: true, handle_mode: HandleMode::FreeDrag, nudge_buttons: false, inspector: InspectorPlacement::Docked, tabbed: false, visible_windows: 5, diagram_point_budget: 20_000, live_debounce_ms: 16, frame_budget_ms: 16, heatmap_stations: 64 },
        DeviceClass::Mobile => DeviceProfileSpec { class, viewport, min_target_px: 44, hover: false, handle_mode: HandleMode::TapNudge, nudge_buttons: true, inspector: InspectorPlacement::BottomSheet, tabbed: true, visible_windows: 1, diagram_point_budget: 2_000, live_debounce_ms: 120, frame_budget_ms: 33, heatmap_stations: 16 },
        DeviceClass::Tablet => DeviceProfileSpec { class, viewport, min_target_px: 44, hover: false, handle_mode: HandleMode::FreeDrag, nudge_buttons: true, inspector: InspectorPlacement::SidePane, tabbed: false, visible_windows: 2, diagram_point_budget: 6_000, live_debounce_ms: 60, frame_budget_ms: 24, heatmap_stations: 32 },
    }
}

/// 📱️ Remembers the device class of the last layout choice, so the windows adapt their touch controls and hit areas to it.
pub fn note_device(class: DeviceClass) {
    *CURRENT_DEVICE.lock().unwrap_or_else(PoisonError::into_inner) = Some(class);
}

/// 📱️ The device class of the last layout choice, `None` before the shell asked.
pub fn current_device() -> Option<DeviceClass> {
    *CURRENT_DEVICE.lock().unwrap_or_else(PoisonError::into_inner)
}

/// 👆️ The smallest hit edge a pick target needs on the device of the last layout choice: the profile minimum on a touch class, `0` (the target's own size) on a desktop or before the shell asked.
pub fn touch_hit_px() -> u32 {
    current_device().filter(|class| *class != DeviceClass::Desktop).map_or(0, |class| device_profile(class).min_target_px)
}

/// 🧭️ Device class from the window width in CSS pixels and touch capability: below 600 px mobile, below 1024 px with touch tablet, otherwise desktop.
pub fn classify(width_px: u32, touch: bool) -> DeviceClass {
    if width_px < 600 {
        DeviceClass::Mobile
    } else if width_px < 1024 && touch {
        DeviceClass::Tablet
    } else {
        DeviceClass::Desktop
    }
}

impl DeviceProfileSpec {
    /// 🎯️ Hit-area edge a handle needs on this device, never below the profile minimum.
    pub fn handle_hit_px(&self, visual_px: u32) -> u32 {
        visual_px.max(self.min_target_px)
    }

    /// 📉️ Evenly thinned sample count for a series under the diagram point budget.
    pub fn thin(&self, points: usize) -> usize {
        points.min(self.diagram_point_budget)
    }
}
