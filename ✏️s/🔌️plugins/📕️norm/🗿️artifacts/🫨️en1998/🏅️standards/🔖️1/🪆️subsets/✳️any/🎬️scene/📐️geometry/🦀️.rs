//! 📐️ EN 1998 scene geometry: kit, layout, tracks, handles, horizontal elements, ground and bridges and walls shared by every layer. See [`semio_s_artifact_norm_en1998_scene_core`].

extern crate semio_s_artifact_norm_en1998_model as model;
extern crate semio_s_artifact_norm_en1998_schema as schema;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_structural_scene as scene;
extern crate semio_s_artifact_norm_en1998_scene_structures as structure_views;

#[path = "../🧰️kit/🦀️.rs"]
pub mod kit;
#[path = "../📍️layout/🦀️.rs"]
pub mod layout;
#[path = "../🎞️tracks/🦀️.rs"]
pub mod tracks;
#[path = "../🧱️horizontal-elements/🦀️.rs"]
pub mod horizontal_elements;
#[path = "../🌉️ground-and-bridges/🦀️.rs"]
pub mod ground_and_bridges;
#[path = "../🕹️handles/🦀️.rs"]
pub mod handles;
#[path = "../🧱️walls/🦀️.rs"]
pub mod walls;
pub use kit::{entities_of, entity_dependencies, entity_kind_of, EntityKindSpec, ENTITY_KINDS, Out};
pub use layout::{Layout, Placement};
pub use handles::handles;
pub use semio_s_artifact_norm_en1998_scene_analysis::{assessment, capacity, geotechnical, response, storey_response};
pub use model::En1998Snapshot;
pub use norm::CheckReport;
pub use scene::projection::{project_whole, Fragment, Kit, KitDoc, Part, ProjectionContext, SceneKit};
pub use scene::projection_kit;
pub use scene::tessellation::Lod;
pub use scene::{live_scene, EntityRef, LocalizedCopy, SceneRevision, StructuralScene};
pub use std::collections::BTreeMap;

/// 🎯️ Everything one projection reads: the document, its settled report, the persisted placements and the revision stamp.
#[derive(Clone, Copy)]
pub struct SceneInput<'a> {
    pub document: &'a En1998Snapshot,
    pub report: &'a CheckReport,
    pub placements: &'a [Placement],
    pub revision: SceneRevision,
}
