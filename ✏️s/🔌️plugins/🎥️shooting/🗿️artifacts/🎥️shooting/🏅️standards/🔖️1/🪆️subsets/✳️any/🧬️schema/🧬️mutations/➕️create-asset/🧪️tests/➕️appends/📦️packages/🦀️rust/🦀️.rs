//! ➕️ mutations of `semio-s-artifact-shooting-shooting` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v1::subsets::any::schema::snapshot::ShootingSnapshot;
#[cfg(test)]
pub use crate::standards::v1::subsets::any::schema::diff::ShootingDiff;
#[cfg(test)]
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub use semio_s_artifact_shooting_shooting_inferences::{SHOOTING_DOCUMENT_SCHEMA, ShootingAmbient, ShootingAsset, ShootingCamera, ShootingMaterial, ShootingSavedCamera, ShootingSceneLighting, ShootingShadow, ShootingShot, ShootingSun, active_asset, active_shot, empty_shooting_snapshot, shooting_asset_scale, shooting_emblem_child_handle, shooting_emblem_image_from_bytes};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
