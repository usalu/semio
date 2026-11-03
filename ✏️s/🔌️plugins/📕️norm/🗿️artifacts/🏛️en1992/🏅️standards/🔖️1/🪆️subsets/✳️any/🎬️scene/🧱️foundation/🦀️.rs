//! 🧱️ EN 1992 scene foundation: the entity builder and kit, the layout, the section mechanics, the bar positions and the field tracks every projection and diagram of the scene stands on.

extern crate semio_s_artifact_norm_en1992_checks as checks;
extern crate semio_s_artifact_norm_en1992_model as model;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_structural_scene as scene;

#[path = "🧰️kit/🦀️.rs"]
pub mod kit;

#[path = "📍️layout/🦀️.rs"]
pub mod layout;

#[path = "🔬️mechanics/🦀️.rs"]
pub mod mechanics;

#[path = "🧱️reinforcement/🦀️.rs"]
pub mod reinforcement;

#[path = "🎞️tracks/🦀️.rs"]
pub mod tracks;

use layout::Placement;
use model::En1992Snapshot;
use norm::CheckReport;
use scene::SceneRevision;

/// 🎯️ Everything one projection reads: the document, its settled report, the persisted placements and the revision stamp.
#[derive(Clone, Copy)]
pub struct SceneInput<'a> {
    pub document: &'a En1992Snapshot,
    pub report: &'a CheckReport,
    pub placements: &'a [Placement],
    pub revision: SceneRevision,
}
