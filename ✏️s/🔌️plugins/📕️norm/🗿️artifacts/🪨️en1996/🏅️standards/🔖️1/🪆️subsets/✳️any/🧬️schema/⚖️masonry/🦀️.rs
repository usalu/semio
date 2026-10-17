//! ⚖️ EN 1996 masonry checks — DIN EN 1996-1-1/NA, 1-2, 2, 3/NA with EN 1990 combinations.

use semio_s_plugin_norm_compliance::{AnnexChoice, CheckReport, CheckResult, CheckStatus, ClauseId, DesignSituation, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
use semio_s_artifact_norm_en1996_model::{ExposureClass, MasonryClass, MasonryWall, MortarClass, MortarType, UnitGroup, UnitMaterial, WallLoadCase, WallType};

fn loc(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

fn clause(family: &str, part: &str, section: &str) -> ClauseId {
    ClauseId::new(family, part, section)
}

fn wall_subject(w: &MasonryWall, path: &str) -> SubjectRef {
    SubjectRef::new(&w.id, path, loc(&w.label_en, &w.label_de))
}

fn wall_path(wall: &MasonryWall, field: &str) -> String {
    format!("walls[id={}].{field}", wall.id)
}

fn load_case_path(wall: &MasonryWall, lc: &WallLoadCase, field: &str) -> String {
    format!("walls[id={}].loadCases[id={}].{field}", wall.id, lc.id)
}

fn concentrated_path(wall: &MasonryWall, lc: &WallLoadCase, cid: &str, field: &str) -> String {
    format!("walls[id={}].loadCases[id={}].concentrated[id={}].{field}", wall.id, lc.id, cid)
}

#[path = "📏️factors-and-geometry/🦀️.rs"]
mod factors_and_geometry;
pub use factors_and_geometry::*;

#[path = "🔥️fire-and-design-effects/🦀️.rs"]
mod fire_and_design_effects;
pub use fire_and_design_effects::*;

#[path = "🛡️conformance-and-integrity/🦀️.rs"]
mod conformance_and_integrity;
pub use conformance_and_integrity::*;

#[path = "🧱️evaluate-wall/🦀️.rs"]
mod evaluate_wall;
use evaluate_wall::*;
