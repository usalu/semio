//! 🧊️ The 3D viewport window of every Eurocode artifact: lowers the locale-free structural scene to the framework's `World3dScene`
//! (instanced meshes, utilisation colours, section, highlights, annotations, deformation), with camera presets, measure tool,
//! picking to selection events, accessible descriptions and device profiles. The toolbar and lane overlays live in the panels crate. See the facade crate `semio-s-plugin-norm-app-surface`.
//!
//! 🔬️ Mesh format (day-one spike against `fem3d_solid_mesh_entries`): custom meshes are `{ id, data: { positions, normals, colors, indices } }`
//! with flat `f64` triples (colours linear 0..1 RGB per vertex) and `u32` indices; instances are
//! `{ id, interactionGranularityId, meshId, position, rotation [x,y,z,w], scale, label }` whose `id` is the scoped identity
//! `encode_scoped_identity([granularity, entityId])`; extra parts of one entity add `interactionId`.

#![allow(async_fn_in_trait, unused_imports)]

include!("../../📋️prelude/🦀️.rs");

use semio_s_plugin_norm_structural_scene as scene;

#[path = "🧭️pose/🦀️.rs"]
pub mod pose;

#[path = "🧮️mesh/🦀️.rs"]
pub mod mesh;

#[path = "🎚️config/🦀️.rs"]
pub mod config;

#[path = "🧱️parts/🦀️.rs"]
pub mod parts;

#[path = "🎨️colour/🦀️.rs"]
pub mod colour;

#[path = "📏️measure/🦀️.rs"]
pub mod measure;

#[path = "🎯️picking/🦀️.rs"]
pub mod picking;

#[path = "📷️camera/🦀️.rs"]
pub mod camera;

#[path = "🏷️overlays/🦀️.rs"]
pub mod overlays;

#[path = "♿️description/🦀️.rs"]
pub mod description;

#[path = "⏯️playback/🦀️.rs"]
pub mod playback;

#[path = "🌈️fields/🦀️.rs"]
pub mod fields;

#[path = "🎞️lowering/🦀️.rs"]
pub mod lowering;

#[path = "🖼️window/🦀️.rs"]
pub mod window;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
