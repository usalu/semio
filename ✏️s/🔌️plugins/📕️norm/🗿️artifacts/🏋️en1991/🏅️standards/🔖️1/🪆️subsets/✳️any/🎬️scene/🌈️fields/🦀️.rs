//! 🌈️ EN 1991 scene fields: pure builders of colour channels, parameter tracks, diagrams and profiles over the structural scene, kept apart from the projection crate so that crate stays inside the source boundary. See [`semio_s_plugin_norm_structural_scene`].

extern crate semio_s_artifact_norm_en1991_checks as checks;
extern crate semio_s_artifact_norm_en1991_model as model;
extern crate semio_s_plugin_norm_structural_scene as scene;

#[path = "🌉️travel/🦀️.rs"]
pub mod travel;

#[path = "🌬️pressure/🦀️.rs"]
pub mod pressure;

#[path = "❄️snow/🦀️.rs"]
pub mod snow;

#[path = "🧯fire-load/🦀️.rs"]
pub mod fire_load;

#[path = "🎚️gauges/🦀️.rs"]
pub mod gauges;
