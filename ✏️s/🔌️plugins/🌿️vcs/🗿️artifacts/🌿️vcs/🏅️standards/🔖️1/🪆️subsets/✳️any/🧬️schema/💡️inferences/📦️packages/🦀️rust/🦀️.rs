//! 💡️ inferences of `semio-s-artifact-vcs-vcs` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

pub use crate::standards::v1::subsets::any::schema::snapshot::VcsSnapshot;
pub use crate::standards::v1::subsets::any::io::binary::snapshot::artifact_items::{VCS_DOCUMENT_SCHEMA};
pub use crate::standards::v1::subsets::any::io::binary::snapshot::artifact_items_4::{definition};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
