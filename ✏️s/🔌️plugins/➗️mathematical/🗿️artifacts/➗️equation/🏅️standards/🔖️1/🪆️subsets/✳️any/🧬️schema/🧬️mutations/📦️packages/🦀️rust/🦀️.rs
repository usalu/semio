//! 🧬️ mutations of `semio-s-artifact-mathematical-equation` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_number as number;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::standards::v1::subsets::any::schema::snapshot::EquationSnapshot;
pub use crate::standards::v1::subsets::any::schema::diff::EquationDiff;
#[cfg(test)]
pub use crate::standards::v1::subsets::any::schema::mutations::EquationMutation;
pub mod op {
    pub use crate::standards::v1::subsets::any::schema::mutations::EquationMutation;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_mathematical_equation_snapshot::{EquationGraph, EquationNode, EquationPoint, equation_children, genesis_equation_child_pack};
pub use crate::viewer::equation::artifact_items::{EQUATION_DIALECT, MATH_DOCUMENT_SCHEMA};
pub use crate::viewer::equation::artifact_items_7::{equation_child_restore_projection};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
