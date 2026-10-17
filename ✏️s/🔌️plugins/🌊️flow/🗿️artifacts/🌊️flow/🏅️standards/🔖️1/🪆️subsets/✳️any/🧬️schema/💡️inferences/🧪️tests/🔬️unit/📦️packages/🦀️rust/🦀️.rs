//! 🔬️ tests of `semio-s-artifact-flow-flow` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v1::subsets::any::schema::snapshot::FlowSnapshot;
use semio_framework_artifact_playbook_playbook as playbook;
#[cfg(test)]
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
