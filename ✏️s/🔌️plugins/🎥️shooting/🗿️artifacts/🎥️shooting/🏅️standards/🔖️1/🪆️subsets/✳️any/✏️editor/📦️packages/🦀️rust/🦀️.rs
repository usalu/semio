//! ✏️ shooting of `semio-s-artifact-shooting-shooting` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

pub use crate::standards::v1::subsets::any::schema::snapshot::ShootingSnapshot;
#[cfg(test)]
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub use semio_s_artifact_shooting_shooting_inferences::{SHOOTING_DOCUMENT_SCHEMA, ShootingAsset, ShootingCamera, ShootingSavedCamera, ShootingShot, active_asset, active_shot, default_fov, empty_shooting_snapshot, is_transparent_shooting_background, next_shooting_id, shooting_asset_scale};
pub use semio_s_artifact_shooting_shooting_tests_dims_scene_sun_to_half::{SHOOTING_DIALECT};
pub use crate::editor::shooting::artifact_items_3::{artifact_kind};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
