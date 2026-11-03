//! ⚖️ EN 1996 masonry checks — DIN EN 1996-1-1/NA, 1-2, 2, 3/NA with EN 1990 combinations.

use semio_s_plugin_norm_compliance::incremental_evaluation::Scope;
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckReport, CheckResult, CheckStatus, ClauseId, DesignSituation, LocalizedCopy, Quantity, QuantityKind, Remedy, Station, SubjectRef};
use semio_s_artifact_norm_en1996_model::{BarKind, ConcentratedLoad, ExposureClass, FrostClass, TieKind, WallBar, WallTie, MasonryClass, MasonryWall, MortarClass, MortarType, UnitGroup, UnitMaterial, WallLoadCase, WallType};

pub use semio_s_artifact_norm_en1996_masonry_tables::*;

pub use semio_s_artifact_norm_en1996_masonry_formulas::*;

#[path = "🛡️conformance-and-integrity/🦀️.rs"]
mod conformance_and_integrity;
pub use conformance_and_integrity::*;

#[path = "🧱️evaluate-wall/🦀️.rs"]
mod evaluate_wall;
use evaluate_wall::*;

#[path = "🧮️units/🦀️.rs"]
mod units;
pub use units::*;

pub use semio_s_artifact_norm_en1996_masonry_catalogue::*;

#[path = "🪚️detailing/🦀️.rs"]
mod detailing;
pub use detailing::*;


/// 📂️ Directory of this crate sources, scanned by the superseded-form tests.
pub const SOURCE_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
