//! 🎬️ EN 1998 structural scene: the pure projection of the seismic snapshot and its check report to the locale-free `StructuralScene`, its `Diagram2d` set and its drag handles. See [`semio_s_plugin_norm_structural_scene`].

extern crate semio_s_artifact_norm_en1998_model as model;
extern crate semio_s_artifact_norm_en1998_schema as schema;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_structural_scene as scene;
extern crate semio_s_artifact_norm_en1998_scene_structures as structure_views;

pub use semio_s_artifact_norm_en1998_scene_core::*;

#[path = "📈️diagrams/🦀️.rs"]
pub mod diagrams;
pub use diagrams::diagrams;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit;

#[cfg(test)]
#[path = "🧪️tests/🔬️live/🦀️.rs"]
mod live_law;
