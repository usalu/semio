//! 🎬️ EN 1997 structural scene: the pure projection of the geotechnical snapshot and its check report to the locale-free `StructuralScene`, its `Diagram2d` set and its drag handles. See [`semio_s_plugin_norm_structural_scene`].

extern crate semio_s_artifact_norm_en1997_checks as checks;
extern crate semio_s_artifact_norm_en1997_model as model;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_structural_scene as scene;

pub use semio_s_artifact_norm_en1997_scene_core::*;
use semio_s_artifact_norm_en1997_scene_core::{kit, mechanics, layout, slopes, settlement};

#[path = "📈️diagrams/🦀️.rs"]
pub mod diagrams;
pub use diagrams::diagrams;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit;

#[cfg(test)]
#[path = "🧪️tests/🔬️live/🦀️.rs"]
mod live_law;
