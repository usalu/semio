//! 🎬️ EN 1999 structural scene: the pure projection of the aluminium snapshot and its check report to the locale-free `StructuralScene`, its `Diagram2d` set and its drag handles. See [`semio_s_plugin_norm_structural_scene`].

extern crate semio_s_artifact_norm_en1999_checks as checks;
extern crate semio_s_artifact_norm_en1999_model as model;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_mechanics as mechanics;
extern crate semio_s_plugin_norm_structural_scene as scene;

pub use semio_s_artifact_norm_en1999_scene_core::*;
use semio_s_artifact_norm_en1999_scene_core::{kit, layout, facts, handles};

#[path = "📈️diagrams/🦀️.rs"]
pub mod diagrams;
pub use diagrams::diagrams;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit;

#[cfg(test)]
#[path = "🧪️tests/🔬️live/🦀️.rs"]
mod live_law;

#[cfg(test)]
#[path = "🧪️tests/🔭️wave3/🦀️.rs"]
mod wave3;

#[cfg(test)]
#[path = "🧪️tests/🎨️visual-pass/🦀️.rs"]
mod visual_pass;
