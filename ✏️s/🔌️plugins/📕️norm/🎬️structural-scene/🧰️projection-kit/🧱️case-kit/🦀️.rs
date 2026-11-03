//! 🧱️ Case kit: the lowest layer of the projection kit (vectors, identity, utilisation, station fields, handle constructors, diagram parts) and the design-case forms drawn on it, kept in their own crate inside the source-boundary budget; [`semio_s_plugin_norm_structural_scene::projection_kit`] re-exports every module.

pub extern crate semio_s_plugin_norm_compliance as norm;

pub use semio_s_plugin_norm_structural_scene_foundation::*;
pub use {identity::*, placement::*, station_fields::*, utilisation::*, vectors::*};

#[path = "../📐️vectors/🦀️.rs"]
pub mod vectors;

#[path = "../🆔️identity/🦀️.rs"]
pub mod identity;

#[path = "../🌈️station-fields/🦀️.rs"]
pub mod station_fields;

#[path = "../📶️utilisation/🦀️.rs"]
pub mod utilisation;

#[path = "../📍️placement/🦀️.rs"]
pub mod placement;

#[path = "../🕹️handle-kit/🦀️.rs"]
pub mod handle_kit;

#[path = "../📈️diagram-parts/🦀️.rs"]
pub mod diagram_parts;

#[path = "../🗂️design-case/🦀️.rs"]
pub mod design_case;

#[path = "../🧪️case-laws/🦀️.rs"]
pub mod case_laws;
