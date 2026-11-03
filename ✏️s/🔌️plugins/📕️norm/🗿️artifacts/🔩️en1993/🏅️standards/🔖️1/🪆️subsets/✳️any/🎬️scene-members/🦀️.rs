//! 🧩️ EN 1993 scene members: the kit, layout, module families and drag handles below the scene facade [`semio_s_artifact_norm_en1993_scene`]; every item the facade, the diagrams and the tests reach is `pub`.

extern crate semio_s_artifact_norm_en1993_checks as checks;
extern crate semio_s_artifact_norm_en1993_model as model;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_mechanics as mechanics;
extern crate semio_s_plugin_norm_structural_scene as scene;

#[path = "🧰️kit/🦀️.rs"]
pub mod kit;
pub use kit::{EntityKindSpec, ENTITY_KINDS, entities_of, entity_dependencies, entity_kind_of};

#[path = "📍️layout/🦀️.rs"]
pub mod layout;
pub use layout::{Layout, Placement};

#[path = "🔩️members/🦀️.rs"]
pub mod members;

#[path = "🌀️eigen-modes/🦀️.rs"]
pub mod eigen;

#[path = "🎞️tracks/🦀️.rs"]
pub mod tracks;

#[path = "🔗️joints/🦀️.rs"]
pub mod joints;

#[path = "🔩️joint-rows/🦀️.rs"]
pub mod joint_rows;

#[path = "🔩️plate-layout/🦀️.rs"]
pub mod plate_layout;

#[path = "🎚️scalar-grips/🦀️.rs"]
pub mod scalar_grips;

#[path = "🕹️handles/🦀️.rs"]
pub mod handles;
pub use handles::handles;

use model::En1993Snapshot;
use norm::CheckReport;
use scene::SceneRevision;

/// 🎯️ Everything one projection reads: the document, its settled report, the persisted placements and the revision stamp.
#[derive(Clone)]
pub struct SceneInput<'a> {
    pub document: &'a En1993Snapshot,
    pub report: &'a CheckReport,
    pub placements: &'a [Placement],
    pub revision: SceneRevision,
}
