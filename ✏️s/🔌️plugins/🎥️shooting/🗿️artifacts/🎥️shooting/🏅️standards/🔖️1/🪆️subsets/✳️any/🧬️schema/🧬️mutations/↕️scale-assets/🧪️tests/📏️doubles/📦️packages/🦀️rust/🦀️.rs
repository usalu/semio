//! 📏️ mut-any-3 of `semio-s-artifact-shooting-shooting` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as schema;

#[cfg(test)]
pub use crate::standards::v1::subsets::any::schema::diff::ShootingDiff;
#[cfg(test)]
pub use crate::standards::v1::subsets::any::schema::snapshot::ShootingSnapshot;
#[cfg(test)]
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
