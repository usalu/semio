//! 🧩️ EN 1991 scene members: the kit, layout, module families and drag handles below the scene facade [`semio_s_artifact_norm_en1991_scene`]; every item the facade, the diagrams and the tests reach is `pub`.

extern crate semio_s_artifact_norm_en1991_checks as checks;
extern crate semio_s_artifact_norm_en1991_model as model;
extern crate semio_s_artifact_norm_en1991_scene_fields as fields;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_mechanics as mechanics;
extern crate semio_s_plugin_norm_structural_scene as scene;

extern crate semio_s_artifact_norm_en1991_scene_registry as registry;

#[path = "🧰️kit/🦀️.rs"]
pub mod kit;
pub use kit::{binding_of_owner, entities_of, entity_dependencies, entity_kind_of, family_of, focus_of_outcome, inert_reference, is_outcome, owner_entity, report_owner, route_of, scalar_owner, scene_report, CheckRoute, EntityKindSpec, DOCUMENT_CHECKS, DRAWN_FAMILIES, ENTITY_KINDS, OUTCOME_FAMILIES};
#[path = "📍️layout/🦀️.rs"]
pub mod layout;
pub use layout::{Layout, Placement};
#[path = "🏢️envelope/🦀️.rs"]
pub mod envelope;
#[path = "🌬️wind/🦀️.rs"]
pub mod wind;
#[path = "🏗️gravity/🦀️.rs"]
pub mod gravity;
#[path = "🔥️thermal-fire/🦀️.rs"]
pub mod thermal_fire;
#[path = "💥accidental/🦀️.rs"]
pub mod accidental;
#[path = "🧱️collections/🦀️.rs"]
pub mod collections;
#[path = "🫙silo/🦀️.rs"]
pub mod silo;
#[path = "🗂️cases/🦀️.rs"]
pub mod cases;
#[path = "🌪️wave3/🦀️.rs"]
pub mod wave3;
#[path = "🧷️wave3-rows/🦀️.rs"]
pub mod wave3_rows;
#[path = "🎞️tracks/🦀️.rs"]
pub mod dynamics;
#[path = "🕹️handles/🦀️.rs"]
pub mod handles;
pub use handles::handles;

use model::En1991Snapshot;
use norm::CheckReport;
use scene::SceneRevision;

/// 🎯️ Everything one projection reads: the document, its settled report, the persisted placements and the revision stamp.
#[derive(Clone, Copy)]
pub struct SceneInput<'a> {
    pub document: &'a En1991Snapshot,
    pub report: &'a CheckReport,
    pub placements: &'a [Placement],
    pub revision: SceneRevision,
}

pub struct Ctx<'a> {
    pub input: &'a SceneInput<'a>,
    pub layout: &'a Layout,
}
