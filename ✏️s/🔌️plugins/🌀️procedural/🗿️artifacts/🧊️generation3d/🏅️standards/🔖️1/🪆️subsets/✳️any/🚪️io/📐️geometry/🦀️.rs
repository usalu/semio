//! 🚪️ Native geometry compute admission with explicit physical source and interchange owners.
use crate::standards::v1::subsets::any::schema::inferences::geometry::{compute::{GeometryComputeContext,ComputeEntry,StartFn},registry};
use std::{collections::BTreeMap,sync::{Arc,OnceLock}};
#[path="💾️brep-interchange/🦀️.rs"]
mod brep_interchange;
#[path="📼️mesh-interchange/🦀️.rs"]
mod mesh_interchange;
#[path="🥽️mesh-source/🦀️.rs"]
pub(crate) mod mesh_source;
/// 🗃️ Physical IO operations, separately admitted from semantic computations.
pub const TABLES:&[&[ComputeEntry]]=&[brep_interchange::COMPUTES,mesh_interchange::COMPUTES,mesh_source::COMPUTES];
/// 🏠️ The actual native compute capability composition of the generation3d host.
pub struct NativeGeometryComputeContext;
impl GeometryComputeContext for NativeGeometryComputeContext {
    fn lookup(&self,kind_id:&str)->Option<StartFn> {registry::lookup(kind_id).or_else(||index().get(kind_id).copied())}
}
/// 🗺️ Native IO operation index; no schema owner mounts these tables.
pub fn index()->&'static BTreeMap<&'static str,StartFn> {
    static INDEX:OnceLock<BTreeMap<&'static str,StartFn>>=OnceLock::new();
    INDEX.get_or_init(||TABLES.iter().flat_map(|table|table.iter()).map(|entry|(entry.id,entry.start)).collect())
}
/// 🧩️ Admits the native capability context at a host boundary.
pub fn context()->Arc<dyn GeometryComputeContext> {Arc::new(NativeGeometryComputeContext)}
