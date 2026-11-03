//! 📚️ EN 1996 masonry normative tables and numerical models: pure functions over the snapshot enums, re-exported by `semio-s-artifact-norm-en1996-masonry`.

use semio_s_artifact_norm_en1996_model::{ExposureClass, MortarClass, MortarType, UnitGroup, UnitMaterial};
use semio_s_plugin_norm_compliance::AnnexChoice;

#[path = "📏️limits/🦀️.rs"]
mod limits;
pub use limits::*;

#[path = "🕳️chase-tables/🦀️.rs"]
mod chase_tables;
pub use chase_tables::*;

#[path = "📚️normative-tables/🦀️.rs"]
mod normative_tables;
pub use normative_tables::*;

#[path = "🔬️numerical-models/🦀️.rs"]
pub mod numerical_models;

#[path = "📐️annex-e/🦀️.rs"]
pub mod annex_e;
