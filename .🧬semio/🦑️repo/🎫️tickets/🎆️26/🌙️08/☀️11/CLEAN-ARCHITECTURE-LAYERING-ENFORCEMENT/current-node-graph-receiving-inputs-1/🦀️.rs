//! 📥️ Retained source reception separates structural births, bounded copies and materialization.
use semio_framework_pack::intrinsic::{IntrinsicFormat,RetainedIntrinsicInput};
use semio_framework_surface::node_graph::{NodeGraphScenePayload,SceneDecodeCursor,SceneDecodeLimits,SceneDecodeStep};
use semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneStep,RetainedCloneProgress},retirement::controlled::ControlledRetirement};
use std::mem::{ManuallyDrop,MaybeUninit};

/// 🎟️ One reception turn carries distinct work and backing-capacity authorities.
#[derive(Clone,Copy,Debug)]
pub struct SceneInputGrant{pub maximum_units:usize,pub maximum_capacity_bytes:usize,pub cancelled:bool}
/// 📊️ Copied bytes exclude the separately charged structural source birth.
#[derive(Clone,Copy,Debug,Default)]
pub struct SceneInputStep{pub units:usize,pub copied_bytes:usize,pub admitted_bytes:usize}
/// 🧵️ Original source and active decoder remain owned through refusal and explicit terminal close.
pub struct SceneInputCursor{source:ManuallyDrop<Vec<u8>>,total:usize,format:IntrinsicFormat,limits:SceneDecodeLimits,decoder:Option<SceneDecodeCursor<'static>>,retiring:Option<ControlledRetirement<Vec<u8>>>,admitted:usize,closing:bool,closed:bool,fault:Option<ValueError>}
fn refusal(kind:ValueRefusalKind,message:&'static str)->ValueError{ValueError::literal(kind,message)}
impl SceneInputCursor{
 /// 📏️ Scalar admission creates no source allocation or complete transport copy.
 pub fn new(total:usize,format:IntrinsicFormat,limits:SceneDecodeLimits)->Result<Self,ValueError>{if total>limits.maximum_owned_bytes||total>isize::MAX as usize{return Err(refusal(ValueRefusalKind::OwnershipLimit,"scene source exceeds caller ownership ceiling"))}if limits.maximum_depth==0{return Err(refusal(ValueRefusalKind::DepthLimit,"scene source requires a positive depth authority"))}if limits.maximum_items==0{return Err(refusal(ValueRefusalKind::WorkLimit,"scene source requires a positive item authority"))}Ok(Self{source:ManuallyDrop::new(Vec::new()),total,format,limits,decoder:None,retiring:None,admitted:0,closing:false,closed:false,fault:None})}
 fn fail<T>(&mut self,error:ValueError)->Result<T,ValueError>{self.fault=Some(error.clone());Err(error)}
 fn ready(&mut self,cancelled:bool)->Result<(),ValueError>{if let Some(error)=&self.fault{return Err(error.clone())}if cancelled{return self.fail(refusal(ValueRefusalKind::Canceled,"scene reception canceled"))}if self.closing{return Err(refusal(ValueRefusalKind::InvalidValue,"scene reception is retiring"))}Ok(())}
 /// 📋️ Native spans initialize only the bytes covered by the current finite work authority.
 pub fn append(&mut self,input:&[u8],grant:SceneInputGrant)->Result<SceneInputStep,ValueError>{unsafe{self.append_initialized(input.len(),grant,|destination|{for(destination,source)in destination.iter_mut().zip(input){destination.write(*source);}})}}
 /// 🛂️ The caller initializes every cell in the supplied admitted span before returning.
 unsafe fn append_initialized(&mut self,length:usize,grant:SceneInputGrant,copy:impl FnOnce(&mut[MaybeUninit<u8>]))->Result<SceneInputStep,ValueError>{self.ready(grant.cancelled)?;let mut step=SceneInputStep{admitted_bytes:self.admitted,..Default::default()};if grant.maximum_units==0{return Ok(step)}if self.decoder.is_some()||length>self.total-self.source.len(){return self.fail(refusal(ValueRefusalKind::InvalidValue,"scene received span exceeds remaining source"))}if length==0&&self.source.len()!=self.total{return self.fail(refusal(ValueRefusalKind::InvalidValue,"scene received span is empty"))}
  if self.source.capacity()==0&&self.total!=0{if grant.maximum_capacity_bytes<self.total{return self.fail(refusal(ValueRefusalKind::OwnershipLimit,"scene source birth exceeds current capacity grant"))}if self.source.try_reserve_exact(self.total).is_err(){return self.fail(refusal(ValueRefusalKind::AllocationFailed,"scene source backing allocation failed"))}self.admitted=self.source.capacity();step.units=1;step.admitted_bytes=self.admitted;}
  let count=length.min(grant.maximum_units-step.units);if count!=0{let before=self.source.len();copy(&mut self.source.spare_capacity_mut()[..count]);unsafe{self.source.set_len(before+count)};step.units+=count;step.copied_bytes=count;}Ok(step)
 }
 #[cfg(all(target_arch="wasm32",not(target_env="p2")))]
 pub(crate) fn append_browser(&mut self,input:&js_sys::Uint8Array,grant:SceneInputGrant)->Result<SceneInputStep,ValueError>{let length=input.length()as usize;unsafe{self.append_initialized(length,grant,|destination|{input.subarray(0,destination.len()as u32).copy_to_uninit(destination);})}}
 /// 👁️ The same admitted backing is borrowed before and after decoder transfer.
 pub fn input(&self)->&[u8]{if let Some(decoder)=&self.decoder{decoder.input().unwrap_or(&[])}else{&self.source}}
 pub fn admitted_bytes(&self)->usize{self.admitted}
 pub fn total_input_bytes(&self)->usize{self.total}
 pub fn phase(&self)->&'static str{if self.closing{"retirement"}else if let Some(decoder)=&self.decoder{decoder.phase().as_str()}else{"input-copy"}}
 pub fn next_work_demand(&self)->usize{if self.fault.is_some()||self.closing{0}else{self.decoder.as_ref().map_or(1,SceneDecodeCursor::next_work_demand)}}
 /// 📤️ One charged transition moves original source custody into its defining General decoder.
 pub fn advance(&mut self,maximum_units:usize,cancelled:bool)->Result<SceneDecodeStep,ValueError>{self.ready(cancelled)?;if self.decoder.is_none(){if maximum_units==0{return Ok(SceneDecodeStep{admitted_bytes:self.admitted,..Default::default()})}if self.source.len()!=self.total{return self.fail(refusal(ValueRefusalKind::InvalidValue,"scene source reception is incomplete"))}let source=std::mem::take(&mut*self.source);match SceneDecodeCursor::new(RetainedIntrinsicInput::OwnedBytes(source),self.format,self.limits){Ok(decoder)=>self.decoder=Some(decoder),Err((error,input))=>{let RetainedIntrinsicInput::OwnedBytes(source)=input else{unreachable!()};*self.source=source;return self.fail(error)}}return Ok(SceneDecodeStep{units:1,admitted_bytes:self.admitted,complete:false})}let result=self.decoder.as_mut().unwrap().advance(maximum_units,false);self.admitted=self.decoder.as_ref().unwrap().admitted_bytes();match result{Ok(step)=>Ok(step),Err(error)=>self.fail(error)}}
 pub fn take_output(&mut self)->Option<NodeGraphScenePayload>{if self.fault.is_none()&&!self.closing{self.decoder.as_mut().and_then(SceneDecodeCursor::take_output)}else{None}}
 pub fn next_close_copy_byte_demand(&self)->usize{if let Some(decoder)=&self.decoder{decoder.next_close_copy_byte_demand()}else{self.retiring.as_ref().map_or(0,ControlledRetirement::next_copy_byte_demand)}}
 pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{if let Some(decoder)=&self.decoder{decoder.next_close_capacity_byte_demand(body)}else{self.retiring.as_ref().map_or(Ok(0),|owner|owner.next_capacity_byte_demand(body))}}
 pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{if let Some(decoder)=&self.decoder{decoder.next_close_release_byte_demand()}else{self.retiring.as_ref().map_or(Ok(0),ControlledRetirement::next_release_byte_demand)}}
 pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{if let Some(decoder)=&self.decoder{decoder.next_close_depth_demand()}else{self.retiring.as_ref().map_or(Ok(1),ControlledRetirement::next_depth_demand)}}
 /// ♻️ Full independent retirement grants forward to the actual current defining owner.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if self.closed{return Ok(RetainedCloneStep::Complete(Default::default()))}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}if let Some(decoder)=&mut self.decoder{self.closing=true;let step=decoder.close_step(grant)?;if decoder.terminal_is_empty(){self.decoder.take();self.closed=true;}return Ok(step)}if self.retiring.is_none(){if grant.maximum_depth==0{return Err(refusal(ValueRefusalKind::DepthLimit,"scene source retirement requires one structural depth"))}let source=std::mem::take(&mut*self.source);match ControlledRetirement::new(source){Ok(owner)=>self.retiring=Some(owner),Err((error,source))=>{*self.source=source;return Err(error)}}self.closing=true;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}let owner=self.retiring.as_mut().unwrap();let step=owner.step(grant)?;if owner.terminal_is_empty(){self.retiring.take();self.closed=true;}Ok(step)}
 pub fn terminal_is_empty(&self)->bool{self.closed&&self.decoder.is_none()&&self.retiring.is_none()&&self.source.is_empty()&&self.source.capacity()==0}
}
impl Drop for SceneInputCursor{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"scene reception abandoned before terminal close");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.source)}}}}
