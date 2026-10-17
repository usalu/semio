//! 🔬️ leaf-tests of `semio-s-artifact-mathematical-equation` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::standards::v1::subsets::any::schema::mutations::EquationMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::EquationSnapshot;
pub use crate::snapshot::schema::EquationCarrierSnapshot;
pub use crate::snapshot::schema::EquationExprSnapshot;
#[cfg(test)]
pub use crate::standards::v1::subsets::any::schema::diff::EquationDiff;
pub use semio_s_artifact_mathematical_equation_snapshot::{EquationGeometry, EquationGraph, EquationNode, EquationPoint, equation_children, equation_snapshot_from_host_snapshot, equation_snapshot_with_state};
pub use semio_s_artifact_mathematical_equation_mutations::{EQUATION_DIALECT, MATH_DOCUMENT_SCHEMA};
pub use crate::standards::v1::artifact_items_6::{EquationApplication, artifact, definition, pilot_languages};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
