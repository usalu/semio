//! 🎬️ Structural scene: the domain-neutral, locale-free, SI/Z-up picture of one document revision.

extern crate semio_framework_value as value;
extern crate semio_framework_value_derive as value_derive;
pub extern crate semio_s_plugin_norm_compliance as norm;

pub use semio_s_plugin_norm_structural_scene_foundation::*;

pub use semio_s_plugin_norm_structural_scene_meshing::{colour_maps, tessellation};

#[path = "🧰️projection-kit/🦀️.rs"]
pub mod projection_kit;

#[path = "🎯️projection/🦀️.rs"]
pub mod projection;

#[path = "🔺️diff/🦀️.rs"]
pub mod diff;

#[cfg(test)]
#[path = "🧪️tests/🧫️fixtures/🎭️mock-projection/🦀️.rs"]
pub(crate) mod mock_projection;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit;
