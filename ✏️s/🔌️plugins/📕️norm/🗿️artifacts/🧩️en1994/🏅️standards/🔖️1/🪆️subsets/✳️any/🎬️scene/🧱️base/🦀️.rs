//! 🧱️ EN 1994 scene base: statics, facts, layout, kit and handles shared by every projection. See [`super`].

extern crate semio_s_artifact_norm_en1994_checks as checks;
extern crate semio_s_artifact_norm_en1994_model as model;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_structural_scene as scene;

#[path = "../🌀️statics/🦀️.rs"]
pub mod statics;
#[path = "../🧮️facts/🦀️.rs"]
pub mod facts;
#[path = "../📍️layout/🦀️.rs"]
pub mod layout;
#[path = "../🧰️kit/🦀️.rs"]
pub mod kit;
#[path = "../🕹️handles/🦀️.rs"]
pub mod handles;
