//! 🪛️ Projection kit: the domain-neutral building blocks every norm scene crate shares (vectors and frames, identity, check-report bindings, placements, handle constructors, layers and legends, diagram parts, golden-test support). Domain knowledge stays in the artifacts.
//!
//! 📦️ Solid convention: `SolidShape::Box` is anchored at the centre of its base on the frame origin; see [`vectors::solid::box_from_corner`].

pub use semio_s_plugin_norm_structural_scene_case_kit::{case_laws, design_case, diagram_parts, handle_kit, identity, placement, station_fields, utilisation, vectors};
pub use vectors::*;

pub use identity::*;

pub use utilisation::*;

#[path = "🔦️highlight/🦀️.rs"]
pub mod highlight;
pub use highlight::*;

pub use station_fields::*;

pub use placement::*;

#[path = "📍️placement/📖️read/🦀️.rs"]
pub mod placement_read;
pub use placement_read::*;

#[path = "🏷️legends/🦀️.rs"]
pub mod legends;

#[path = "🎞️tracks/🦀️.rs"]
pub mod tracks;

#[path = "🧪️test-support/🦀️.rs"]
pub mod test_support;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit;
