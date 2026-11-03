//! ♿️ Accessibility of the visual windows: live-region announcements, keyboard focus order with shortcuts, and the preference-to-theme-variant rule.
//!
//! 🔗️ See `visual-editor-architecture.md` §8.2; selection announcements live in [`description`](crate::viewport3d::description).

use semio_s_plugin_norm_structural_scene::colour_maps::Variant;

#[path = "📣️live-region/🦀️.rs"]
pub mod live_region;

#[path = "⌨️focus-order/🦀️.rs"]
pub mod focus_order;

#[path = "🗣️screen-reader/🦀️.rs"]
pub mod screen_reader;

#[path = "📡️published-nodes/🦀️.rs"]
pub mod published_nodes;

#[cfg(test)]
#[path = "🌐️i18n-census/🦀️.rs"]
mod i18n_census;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

/// 🎛️ Accessibility-relevant user and system preferences.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct A11yPrefs {
    pub forced_colors: bool,
    pub prefers_contrast_more: bool,
    pub prefers_dark: bool,
    pub prefers_reduced_motion: bool,
}

/// 🌗️ Theme variant to use: forced colours and "more contrast" win over the dark preference.
pub fn resolve_variant(prefs: A11yPrefs) -> Variant {
    if prefs.forced_colors || prefs.prefers_contrast_more {
        Variant::HighContrast
    } else if prefs.prefers_dark {
        Variant::Dark
    } else {
        Variant::Light
    }
}

/// 🎞️ Whether ghost/stale pulses and camera transitions may animate.
pub fn animations_allowed(prefs: A11yPrefs) -> bool {
    !prefs.prefers_reduced_motion
}
