//! 🧊️ Structural scene meshing: colour maps and tessellation over the scene foundation; re-exported by `semio-s-plugin-norm-structural-scene`.

extern crate semio_framework_value as value;
extern crate semio_framework_value_derive as value_derive;
extern crate semio_s_plugin_norm_compliance as norm;

pub use semio_s_plugin_norm_structural_scene_foundation::*;

#[path = "../🎨️colour-maps/🦀️.rs"]
pub mod colour_maps;

#[path = "../🧮️tessellation/🦀️.rs"]
pub mod tessellation;
