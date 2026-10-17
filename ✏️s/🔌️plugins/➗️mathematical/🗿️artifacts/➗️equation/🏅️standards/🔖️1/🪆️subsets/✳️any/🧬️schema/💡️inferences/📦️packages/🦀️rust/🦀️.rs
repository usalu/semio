//! 💡️ inferences of `semio-s-artifact-mathematical-equation` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_number as number;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as framework_schema;

pub use crate::standards::v1::subsets::any::schema::snapshot::EquationSnapshot;
pub use crate::standards::v1::subsets::any::schema::diff::EquationDiff;
pub use semio_s_artifact_mathematical_equation_snapshot::{EquationEdge, EquationGeometry, EquationGraph, EquationNode, EquationPoint, equation_carrier_snapshot, equation_children, equation_snapshot_with_state};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
