//! 🛫️ Cumulative caller admission for owned native construction and physical output.
use crate::{ValueError, ValueRefusalKind};

/// ⏱️ Known encoding work and cumulative owned storage at a cancellation boundary.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct NativeEncodeProgress { pub completed:usize, pub total:usize, pub owned_bytes:usize }

/// 🧮️ One ownership ceiling persists through typed fields, physical output and its envelope.
pub struct NativeEncodeControl<'a> { maximum_bytes:usize, owned_bytes:usize, completed:usize, total:usize, started:bool, stage:u64, depth:usize, callback:&'a mut dyn FnMut(NativeEncodeProgress)->bool }

impl<'a> NativeEncodeControl<'a> {
    /// 🚦️ Binds the exact caller ceiling and cancellation callback.
    pub fn new(maximum_bytes:usize,callback:&'a mut dyn FnMut(NativeEncodeProgress)->bool)->Self{Self{maximum_bytes,owned_bytes:0,completed:0,total:0,started:false,stage:0,depth:0,callback}}
    /// 📏️ Returns the complete allowance and cumulative admitted ownership.
    pub fn maximum_bytes(&self)->usize{self.maximum_bytes}
    /// 📊️ Returns storage admitted across every nested stage.
    pub fn owned_bytes(&self)->usize{self.owned_bytes}
    /// 📐️ Applies a narrower physical stage ceiling without resetting ownership.
    pub fn scoped_maximum<T,E:From<ValueError>>(&mut self,maximum:usize,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{let parent=self.maximum_bytes;self.maximum_bytes=parent.min(maximum);if self.owned_bytes>self.maximum_bytes{self.maximum_bytes=parent;return Err(E::from(ValueError::new(ValueRefusalKind::OwnershipLimit, "native encoding ownership exceeds stage limit")));}let result=operation(self);self.maximum_bytes=parent;result}
    /// 🛑️ Checks cancellation before an expensive operation.
    pub fn checkpoint(&mut self)->Result<(),ValueError>{if(self.callback)(NativeEncodeProgress{completed:self.completed,total:self.total,owned_bytes:self.owned_bytes}){self.started=true;Ok(())}else{Err(ValueError::new(ValueRefusalKind::Canceled, "native encoding canceled"))}}
    /// 🧭️ Begins an exact workload without resetting admitted ownership.
    pub fn begin_stage(&mut self,total:usize)->Result<(),ValueError>{self.stage=self.stage.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "native encoding stage overflow"))?;self.completed=0;self.total=total;self.started=false;self.checkpoint()}
    /// 🪆️ Restores the enclosing workload while retaining child allocation charges.
    pub fn scoped_stage<T,E>(&mut self,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{let parent=(self.completed,self.total,self.started,self.stage);let result=operation(self);if self.stage!=parent.3{self.completed=parent.0;self.total=parent.1;self.started=parent.2;self.stage=parent.3;}result}
    /// 🌲️ Bounds borrowed recursive projection independently of the input's depth.
    pub fn scoped_depth<T,E:From<ValueError>>(&mut self,maximum:usize,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{if self.depth>=maximum{return Err(E::from(ValueError::new(ValueRefusalKind::DepthLimit, "native encoding exceeds depth limit")))}self.depth+=1;let result=operation(self);self.depth-=1;result}
    /// 📍️ Publishes interior checkpoints at256-unit boundaries and stage completion.
    pub fn advance(&mut self,units:usize)->Result<(),ValueError>{if !self.started{self.checkpoint()?;}let previous=self.completed;self.completed=self.completed.checked_add(units).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "native encoding work overflow"))?;if self.total!=0&&self.completed>self.total{return Err(ValueError::new(ValueRefusalKind::WorkLimit, "native encoding exceeds declared workload"))}if previous/256!=self.completed/256||(self.total!=0&&self.completed==self.total){self.checkpoint()?;}Ok(())}
    /// 🔢️ Advances one explicit owned field or collection item.
    pub fn step(&mut self)->Result<(),ValueError>{self.advance(1)}
    /// 📦️ Admits storage before an allocation, preserving all previous charges.
    pub fn charge(&mut self,bytes:usize)->Result<(),ValueError>{let next=self.owned_bytes.checked_add(bytes).filter(|next|*next<=self.maximum_bytes).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "native encoding ownership exceeds caller limit"))?;if !self.started||bytes>65536{self.checkpoint()?;}self.owned_bytes=next;Ok(())}
    /// 🗂️ Reserves a known typed frontier before creating its slots.
    pub fn allocate_vec<T>(&mut self,count:usize)->Result<Vec<T>,ValueError>{let bytes=count.checked_mul(std::mem::size_of::<T>()).filter(|bytes|*bytes<=isize::MAX as usize).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "native encoding collection size overflow"))?;self.charge(bytes)?;let mut output=Vec::new();output.try_reserve_exact(count).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "native encoding collection allocation failed"))?;Ok(output)}
    /// 🔤️ Owns exact UTF-8 through cancellable64KiB spans.
    pub fn copy_text(&mut self,text:&str)->Result<String,ValueError>{self.scoped_stage(|control|{control.begin_stage(text.len())?;control.charge(text.len())?;let mut output=String::new();output.try_reserve_exact(text.len()).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "native encoding text allocation failed"))?;let mut position=0;while position<text.len(){let mut end=position.saturating_add(65536).min(text.len());while !text.is_char_boundary(end){end-=1;}output.push_str(&text[position..end]);control.advance(end-position)?;position=end;}Ok(output)})}
    /// 🧬️ Owns intrinsic octets through cancellable64KiB spans.
    pub fn copy_bytes(&mut self,bytes:&[u8])->Result<Vec<u8>,ValueError>{self.scoped_stage(|control|{control.begin_stage(bytes.len())?;let mut output=control.allocate_vec(bytes.len())?;for chunk in bytes.chunks(65536){output.extend_from_slice(chunk);control.advance(chunk.len())?;}Ok(output)})}
    /// 📎️ Appends exact bytes with admission and cancellation inside materialization.
    pub fn append_bytes(&mut self,output:&mut Vec<u8>,bytes:&[u8])->Result<(),ValueError>{self.scoped_stage(|control|{control.begin_stage(bytes.len())?;for chunk in bytes.chunks(65536){control.charge(chunk.len())?;output.try_reserve_exact(chunk.len()).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "native encoding output allocation failed"))?;output.extend_from_slice(chunk);control.advance(chunk.len())?;}Ok(())})}
}
