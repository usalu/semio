//! 🎲️ Closed neutral Board documents prepare entity owners without constructing a host.
use super::board_host::{BoardHost,NormalPortError};
use crate::board::{CameraRecord,NodeRecord,RegionData,NodeShape};
use crate::board::graph;
use crate::board::ports::directed::{EdgeRecord,HandleRecord,RegionRecord,SceneRecord,WireRecord,NodeData,HandleData,EdgeData,WireData,GraphPortMode};
use semio_framework_value::{ErasedSnapshotRetirement,FromValue,NativeDecodeControl,SnapshotRetirementStep,ValueError,ValueRefusalKind};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor};
use std::collections::{BTreeMap,BTreeSet};
pub use semio_framework_pack::intrinsic::IntrinsicFormat;
pub use semio_framework_pack::record::DecodeOptions;

#[derive(semio_framework_value_derive::FromValue)]
#[value(rename_all="camelCase",deny_unknown_fields)]
struct Payload {
 mode:String,
 #[value(default)] camera:Option<CameraRecord>,
 nodes:Vec<NodeRecord>,
 handles:Vec<HandleRecord>,
 edges:Vec<EdgeRecord>,
 #[value(default)] wires:Vec<WireRecord>,
 #[value(default)] regions:Vec<RegionRecord>,
 #[value(default)] selection_exit_highlight_ids:Vec<String>,
}
impl RetireOwned for Payload{fn retirement(self)->Box<dyn RetirementCursor>{semio_framework_value::artifact_retirement_sequence!(self.mode,self.camera,self.nodes,self.handles,self.edges,self.wires,self.regions,self.selection_exit_highlight_ids)}}

#[derive(Default)]
pub(super) struct PreparedEntities {
 pub(super) nodes:BTreeMap<String,NodeData>,
 pub(super) handles:BTreeMap<String,HandleData>,
 pub(super) edges:BTreeMap<String,EdgeData>,
 pub(super) wires:BTreeMap<String,WireData>,
 pub(super) regions:BTreeMap<String,RegionData>,
 pub(super) selection:BTreeSet<String>,
 pub(super) preselect:BTreeSet<String>,
 pub(super) preselect_removed:BTreeSet<String>,
 pub(super) selection_exit_highlight:BTreeSet<String>,
}
impl RetireOwned for PreparedEntities{fn retirement(self)->Box<dyn RetirementCursor>{semio_framework_value::artifact_retirement_sequence!(self.nodes,self.handles,self.edges,self.wires,self.regions,self.selection,self.preselect,self.preselect_removed,self.selection_exit_highlight)}}

pub(super) struct SceneOwner {pub(super) prepared:PreparedEntities,pub(super) descriptor:SceneRecord,pub(super) camera:Option<CameraRecord>,pub(super) mode:GraphPortMode}
impl RetireOwned for SceneOwner{fn retirement(self)->Box<dyn RetirementCursor>{semio_framework_value::artifact_retirement_sequence!(self.prepared,self.descriptor,self.camera)}}

/// 🪑️ Only admitted decoding can create a scene receipt; every receipt must publish or retire.
pub struct BoardScene{owner:std::mem::ManuallyDrop<SceneOwner>,consumed:bool}
impl BoardScene {
 pub(super) fn into_owner(mut self)->SceneOwner{self.consumed=true;unsafe{std::mem::ManuallyDrop::take(&mut self.owner)}}
 /// 🚦️ Checks the final caller cancellation boundary before any exclusive host borrow.
 pub fn admit_publication(self,control:&mut NativeDecodeControl<'_>)->Result<BoardScenePublication,BoardSceneError>{let owner=self.into_owner();if let Err(cause)=control.checkpoint(){return Err(BoardSceneError::retain(NormalPortError::Scene(cause),owner));}Ok(BoardScenePublication{owner:std::mem::ManuallyDrop::new(owner),consumed:false})}
 /// ♻️ Transfers an unpublished receipt to the caller's bounded retirement scheduler.
 pub fn retirement(self)->BoardSceneRetirement{BoardSceneRetirement::new(self.into_owner())}
}
impl Drop for BoardScene{fn drop(&mut self){assert!(std::thread::panicking()||self.consumed,"BoardScene receipt must publish or retire");}}

/// 🎬️ A final admitted receipt performs no callbacks or expensive preparation under host mutation.
pub struct BoardScenePublication{owner:std::mem::ManuallyDrop<SceneOwner>,consumed:bool}
impl BoardScenePublication {
 pub(super) fn into_owner(mut self)->SceneOwner{self.consumed=true;unsafe{std::mem::ManuallyDrop::take(&mut self.owner)}}
 pub fn retirement(self)->BoardSceneRetirement{BoardSceneRetirement::new(self.into_owner())}
}
impl Drop for BoardScenePublication{fn drop(&mut self){assert!(std::thread::panicking()||self.consumed,"BoardScenePublication receipt must publish or retire");}}

/// ♻️ Actual entity, descriptor, string and property owners retire under item and byte credits.
pub struct BoardSceneRetirement{owner:Box<dyn ErasedSnapshotRetirement>}
impl BoardSceneRetirement {
 pub(super) fn new<T:RetireOwned>(value:T)->Self{Self{owner:semio_framework_value::retirement::owned_retirement(value)}}
 pub fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize)->Result<SnapshotRetirementStep,ValueError>{self.owner.close_step(maximum_items,maximum_bytes)}
 pub fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
 pub fn next_close_byte_demand(&self)->usize{self.owner.next_close_byte_demand()}
}

/// 🫴️ Refused preparation or publication returns its partial owners explicitly.
pub struct BoardSceneError{cause:NormalPortError,retirement:Option<Box<BoardSceneRetirement>>}
impl BoardSceneError {
 pub(super) fn retain<T:RetireOwned>(cause:NormalPortError,value:T)->Self{Self{cause,retirement:Some(Box::new(BoardSceneRetirement::new(value)))}}
 pub fn take_retirement(&mut self)->Option<Box<BoardSceneRetirement>>{self.retirement.take()}
}
impl std::fmt::Debug for BoardSceneError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("BoardSceneError").field("cause",&self.cause).field("retained",&self.retirement.is_some()).finish()}}
impl std::fmt::Display for BoardSceneError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{std::fmt::Display::fmt(&self.cause,f)}}
impl std::error::Error for BoardSceneError{}
impl From<ValueError> for BoardSceneError{fn from(cause:ValueError)->Self{Self{cause:NormalPortError::Scene(cause),retirement:None}}}
impl From<NormalPortError> for BoardSceneError{fn from(cause:NormalPortError)->Self{Self{cause,retirement:None}}}

pub(super) fn copy_optional(value:&Option<String>,control:&mut NativeDecodeControl<'_>)->Result<Option<String>,ValueError>{value.as_ref().map(|value|control.copy_text(value)).transpose()}

fn prepared(payload:Payload,control:&mut NativeDecodeControl<'_>)->Result<BoardScene,BoardSceneError>{
 let mode=match payload.mode.as_str(){"normal"=>GraphPortMode::Normal,"ported"=>GraphPortMode::Ported,_=>return Err(BoardSceneError::retain(NormalPortError::Scene(ValueError::new(ValueRefusalKind::InvalidValue,"unknown neutral Board mode")),payload))};
 if payload.camera.as_ref().is_some_and(|camera|![camera.x,camera.y,camera.zoom].into_iter().all(f64::is_finite)||camera.zoom<=0.0){return Err(BoardSceneError::retain(NormalPortError::Scene(ValueError::new(ValueRefusalKind::InvalidValue,"invalid neutral Board camera")),payload));}
 let mut owner=SceneOwner{prepared:PreparedEntities::default(),camera:payload.camera,mode,descriptor:SceneRecord{nodes:payload.nodes,handles:payload.handles,edges:payload.edges,wires:payload.wires,regions:payload.regions,selection_exit_highlight_ids:payload.selection_exit_highlight_ids}};
 if let Err(cause)=owner.prepared.prepare_descriptor(&owner.descriptor,control){return Err(BoardSceneError::retain(cause,owner));}
 if let Err(cause)=control.checkpoint(){return Err(BoardSceneError::retain(NormalPortError::Scene(cause),owner));}
 Ok(BoardScene{owner:std::mem::ManuallyDrop::new(owner),consumed:false})
}

fn admission(control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{control.charge(std::mem::size_of::<SceneOwner>()+std::mem::size_of::<BoardSceneRetirement>()+std::mem::size_of::<BoardSceneError>()+16*std::mem::size_of::<usize>())?;control.checkpoint()}

/// 📦️ Decodes only the declared frame under cumulative caller authority.
pub fn decode(bytes:&[u8],format:IntrinsicFormat,options:&DecodeOptions,control:&mut NativeDecodeControl<'_>)->Result<BoardScene,BoardSceneError>{
 admission(control)?;
 let value=semio_framework_pack::intrinsic::decode(bytes,format,options,control).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;
 prepared(Payload::from_value_controlled(&value,control)?,control)
}

/// 🧾️ Decodes one neutral closed JSON document with duplicate members refused.
pub fn from_json(json:&str,control:&mut NativeDecodeControl<'_>)->Result<BoardScene,BoardSceneError>{admission(control)?;prepared(semio_framework_pack_json::from_json_str_controlled(json,semio_framework_pack_json::JsonMemberPolicy::Reject,control)?,control)}

#[path="📥️prepare/🦀️.rs"]
mod prepare;
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
