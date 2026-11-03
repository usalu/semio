//! 🧱️ Entity renderers of the EN 1995 scene: the typed physical entities (tapered and curved members, notches and holes, fastener patterns, CLT panels, timber floors, diaphragms, trusses and bracings) are drawn as the case row they stand for, and every catalogue case is drawn by the generic forms of the projection kit; the cases whose picture needs the timber kernel (floor modes, CLT layups, fire sections, the jointed beam, the spaced column, the fatigue tower, the stress-laminated deck, the crowd, the trusses, bracings and diaphragms) hand the forms their data through [`case_extras`]. The renderers draw into a [`Sink`] and read the document through [`Input`], so they know nothing of the scene crate that owns the accumulator. Holes add a void cylinder, floors add the creep keyframes: the deformed floor at 0, 1, 2, 5, 10, 20 and 50 years on the layers `creep-*` behind the age track, and a creep time diagram. See [`semio_s_artifact_norm_en1995_scene`].

extern crate semio_s_artifact_norm_en1995_model as model;
extern crate semio_s_artifact_norm_en1995_schema as schema;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_structural_scene as scene;

pub use semio_s_artifact_norm_en1995_scene_facts as facts;
pub use semio_s_artifact_norm_en1995_scene_modes as modes;

#[path = "🧰️kit/🦀️.rs"]
pub mod kit;
pub use kit::{band, binding_of, rect, tinted, worst, Input, Sink, Span2, Spec};
pub use kit::{CREEP_LAYERS, CREEP_YEARS, FIRE_LAYERS, FIRE_MINUTES, FRONT_RGB, FRONT_THICKNESS};
pub use kit::{LAYER_ANNOTATIONS, LAYER_CONNECTIONS, LAYER_FIRE, LAYER_LOADS, LAYER_MEMBERS, LAYER_SUPPORTS, LAYER_ZONES};

#[path = "🗂️rows/🦀️.rs"]
mod rows;
pub use rows::special::jointed_parts;
pub use rows::{cached_mode, draw_row, elevation, extent, fastener_positions, geo, product_of, regular, spacing_demands, Geo, Plate, Profile};

#[path = "🧩️cases/🦀️.rs"]
mod cases;
pub use cases::case_extras;

#[path = "🕹️grips/🦀️.rs"]
mod grips;
pub use grips::row_handles;

#[path = "🏷️typed/🦀️.rs"]
mod typed;
pub use typed::{creep_diagram, finish, project, rehome};
