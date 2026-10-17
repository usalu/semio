//! 🔬️ Tests of `semio-s-plugin-flow-brep` over its public API: the unit laws, the mesh laws, the evaluate-budget law and the extension-guest laws share one helper set.

pub use semio_s_plugin_flow_brep::{extension_guest, extension_manifest_json, geometry_inference, module_registry, operation_quality, register, NODE_KERNEL_METHOD};
use semio_s_spatial_kernel_semio_session::*;
use flow_extension_sdk::mesh::*;
use flow_extension_sdk::build_manifest_json;
use neural_engine::{channel_output, ChannelSpec, Dictionary, EvalError, Operator, OperatorImpl, OperatorInfo, Registry, Value};
use semio_framework_3d_brep_boolean::BooleanOp;
use semio_framework_3d_brep_engine::{BrepBooleanAdmission, BrepBooleanJob, BrepBooleanStep, BrepKernel};
use semio_framework_3d_brep_engine::{Brep, GeometryHandle};
use semio_framework_3d_brep_representation::contract::BREP_KERNEL_OPERATIONS;

#[cfg(test)]
#[path = "../../🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "../../../🔬️extension-guest-standalone/🦀️.rs"]
mod extension_guest_tests;
#[cfg(test)]
#[path = "../../../../🥽️mesh/🧪️tests/🔬️unit/🦀️.rs"]
mod mesh_tests;
