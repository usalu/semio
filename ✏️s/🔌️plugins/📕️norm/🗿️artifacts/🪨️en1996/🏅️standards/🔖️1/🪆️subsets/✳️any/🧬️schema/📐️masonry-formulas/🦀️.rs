//! 📐️ EN 1996 masonry formulas — the pure factors, geometry and EN 1990 design-effect functions below the check assembly of [`semio_s_artifact_norm_en1996_masonry`].

use semio_s_artifact_norm_en1996_model::{MasonryClass, MasonryWall, WallLoadCase};
use semio_s_plugin_norm_compliance::{AnnexChoice, ClauseId, DesignSituation, LocalizedCopy, SubjectRef};
use semio_s_artifact_norm_en1996_masonry_tables::*;

pub fn loc(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

pub fn clause(family: &str, part: &str, section: &str) -> ClauseId {
    ClauseId::new(family, part, section)
}

pub fn wall_subject(w: &MasonryWall, path: &str) -> SubjectRef {
    SubjectRef::new(&w.id, path, loc(&w.label_en, &w.label_de))
}

pub fn wall_path(wall: &MasonryWall, field: &str) -> String {
    format!("walls[id={}].{field}", wall.id)
}

pub fn load_case_path(wall: &MasonryWall, lc: &WallLoadCase, field: &str) -> String {
    format!("walls[id={}].loadCases[id={}].{field}", wall.id, lc.id)
}

pub fn concentrated_path(wall: &MasonryWall, lc: &WallLoadCase, cid: &str, field: &str) -> String {
    format!("walls[id={}].loadCases[id={}].concentrated[id={}].{field}", wall.id, lc.id, cid)
}

#[path = "📏️factors-and-geometry/🦀️.rs"]
mod factors_and_geometry;
pub use factors_and_geometry::*;

#[path = "🔥️fire-and-design-effects/🦀️.rs"]
mod fire_and_design_effects;
pub use fire_and_design_effects::*;

/// 📂️ Directory of the formula sources, scanned by the superseded-form tests.
pub const FORMULAS_SOURCE_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
