//! 💡️ tests-dims-scene-sun-to-half of `semio-s-artifact-shooting-shooting` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

pub use crate::standards::v1::subsets::any::schema::snapshot::ShootingSnapshot;
#[cfg(test)]
pub use crate::standards::v1::subsets::any::schema::diff::ShootingDiff;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub use semio_s_artifact_shooting_shooting_inferences::{SHOOTING_DOCUMENT_SCHEMA, ShootingAsset, ShootingCamera, ShootingSavedCamera, ShootingShot, active_shot, empty_shooting_snapshot, next_shooting_id};
pub use crate::viewer::shooting::artifact_items_2::{SHOOTING_DIALECT};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
