//! 🚪️ Native geometry compute admission with explicit physical source and interchange owners.
#![allow(clippy::result_large_err)]
use semio_s_artifact_procedural_3d_geometry_compute::GeometryComputeContext;
use semio_s_artifact_procedural_3d_geometry_compute::ComputeEntry;
use semio_s_artifact_procedural_3d_geometry_compute::StartFn;
use semio_s_artifact_procedural_3d_geometry::registry;
use std::{collections::BTreeMap,sync::{Arc,OnceLock}};
#[path="💾️brep-interchange/🦀️.rs"]
pub mod brep_interchange;
#[path="📼️mesh-interchange/🦀️.rs"]
pub mod mesh_interchange;
#[path="🥽️mesh-source/🦀️.rs"]
pub(crate) mod mesh_source;
/// 🗃️ Physical IO operations, separately admitted from semantic computations.
pub const TABLES:&[&[ComputeEntry]]=&[brep_interchange::COMPUTES,mesh_interchange::COMPUTES,mesh_source::COMPUTES];
/// 🏠️ The actual native compute capability composition of the generation3d host.
pub struct NativeGeometryComputeContext;
impl GeometryComputeContext for NativeGeometryComputeContext {
    fn lookup(&self,kind_id:&str)->Option<StartFn> {registry::lookup(kind_id).or_else(||index().get(kind_id).copied())}
    fn start_original(&self,kind:&semio_s_artifact_procedural_3d_model::catalogue::Kind,inputs:&mut Option<semio_s_artifact_procedural_3d_geometry_inputs::WidgetInputs>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<registry::OriginalFixedOutputJob>,semio_framework_job::OriginalTurnFailure>{registry::start_original_fixed_output(kind,inputs,cx)}
    fn start_original_widget(&self,widget:&semio_framework_artifact_flow_flow::Widget,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<registry::OriginalFixedOutputJob>,semio_framework_job::OriginalTurnFailure>{match widget{semio_framework_artifact_flow_flow::Widget::InputSlider{value,..}=>registry::OriginalFixedOutputJob::admit_slider(*value,cx),_=>Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"native source widget has no original geometry producer").into())}}
}
/// 🗺️ Native IO operation index; no schema owner mounts these tables.
pub fn index()->&'static BTreeMap<&'static str,StartFn> {
    static INDEX:OnceLock<BTreeMap<&'static str,StartFn>>=OnceLock::new();
    INDEX.get_or_init(||TABLES.iter().flat_map(|table|table.iter()).map(|entry|(entry.id,entry.start)).collect())
}
/// 🧩️ Admits the native capability context at a host boundary.
pub fn context()->Arc<dyn GeometryComputeContext> {Arc::new(NativeGeometryComputeContext)}
