//! 💡️ inferences of `semio-s-artifact-shooting-shooting` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v1::subsets::any::schema::snapshot::ShootingSnapshot;
pub use crate::editor::shooting::config::io::binary::artifact_items::{SHOOTING_DOCUMENT_SCHEMA};
pub use crate::editor::shooting::config::io::binary::artifact_items_5::{ShootingAmbient, ShootingAsset, ShootingCamera, ShootingMaterial, ShootingSavedCamera, ShootingSceneLighting, ShootingShadow, ShootingShot, ShootingSun, default_camera_position, default_camera_target, default_fov, default_glb_format, one_f64, quat_from_axis_angle, quat_mul, shooting_asset_scale};
pub use crate::editor::shooting::config::io::binary::artifact_items_6::{empty_shooting_snapshot, shooting_resolve_shot_camera};
pub use crate::editor::shooting::config::io::binary::artifact_items_7::{ShootingEmblemChild, shooting_base64_decode, shooting_base64_encode, shooting_emblem_bytes, shooting_emblem_bytes_from_image, shooting_emblem_child_handle, shooting_emblem_image, shooting_emblem_image_from_bytes, shooting_set_emblem_from_base64};
pub use crate::editor::shooting::config::io::binary::artifact_items_9::{active_asset, active_shot, is_transparent_shooting_background, next_shooting_id};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
