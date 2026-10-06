//! 🧯️ Known diagnostic owners retain every payload and allocation until a full physical grant.
use crate::{Fault,FaultScope};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum FaultCloseStep{Pending{released_items:usize,released_bytes:usize},Complete}

/// 🎯️ This concrete owner closes only the declared Fault fields; it accepts no opaque transport cause.
pub struct FaultCloseOwner{fault:Option<Fault>}
impl FaultCloseOwner{
    /// 🫙️ Creates an empty inline owner without allocating a retirement worker.
    pub const fn empty()->Self{Self{fault:None}}
    /// 📥️ Transfers the exact typed Fault without cloning its payload or allocating.
    pub fn new(fault:Fault)->Self{Self{fault:Some(fault)}}
    /// 👓️ Exposes the still-retained diagnostic identity.
    pub fn fault(&self)->Option<&Fault>{self.fault.as_ref()}
    /// 🏁️ Physical completion requires the actual scope allocation to have been released.
    pub fn terminal_is_empty(&self)->bool{self.fault.is_none()}
    /// ♻️ Releases at most one declared allocation under its whole actual capacity or Layout.
    pub fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize)->FaultCloseStep{
        if maximum_items==0||maximum_bytes==0{return FaultCloseStep::Pending{released_items:0,released_bytes:0};}
        let Some(fault)=self.fault.as_mut()else{return FaultCloseStep::Complete};
        let Fault{origin:_,code,severity:_,message,scope,span:_,causes,params,retryable:_}=fault;
        let crate::FaultCode(code)=code;
        for text in [message,code]{if let Some(step)=close_text(text,maximum_bytes){return step;}}
        let FaultScope{plugin_id,app_id,instance_id,module,body_key}=scope.as_mut();
        for text in [plugin_id,app_id,instance_id,module,body_key]{
            if let Some(value)=text.as_mut(){if let Some(step)=close_text(value,maximum_bytes){return step;}*text=None;return pending(0);}
        }
        if let Some(crate::FaultCause{message,code})=causes.last_mut(){
            if let Some(step)=close_text(message,maximum_bytes){return step;}
            if let Some(crate::FaultCode(text))=code.as_mut(){if let Some(step)=close_text(text,maximum_bytes){return step;}*code=None;return pending(0);}
            causes.pop();return pending(0);
        }
        if let Some(step)=close_empty_vec(causes,maximum_bytes){return step;}
        if let Some(owner)=params.as_mut(){
            let crate::FaultParams(values)=owner.as_mut();
            if let Some((key,value))=values.last_mut(){for text in [key,value]{if let Some(step)=close_text(text,maximum_bytes){return step;}}values.pop();return pending(0);}
            if let Some(step)=close_empty_vec(values,maximum_bytes){return step;}
            let bytes=std::mem::size_of_val(owner.as_ref());if bytes>maximum_bytes{return pending_blocked();}*params=None;return pending(bytes);
        }
        let bytes=std::mem::size_of::<FaultScope>();if bytes>maximum_bytes{return pending_blocked();}
        drop(self.fault.take());pending(bytes)
    }
}
impl Drop for FaultCloseOwner{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"typed diagnostic owner retains actual payload and scope allocation");}}
fn pending(bytes:usize)->FaultCloseStep{FaultCloseStep::Pending{released_items:1,released_bytes:bytes}}
fn pending_blocked()->FaultCloseStep{FaultCloseStep::Pending{released_items:0,released_bytes:0}}
fn close_text(text:&mut String,maximum_bytes:usize)->Option<FaultCloseStep>{let bytes=text.capacity();if bytes==0{return None;}if bytes>maximum_bytes{return Some(pending_blocked());}*text=String::new();Some(pending(bytes))}
fn close_empty_vec<T>(values:&mut Vec<T>,maximum_bytes:usize)->Option<FaultCloseStep>{let bytes=values.capacity().checked_mul(std::mem::size_of::<T>()).expect("actual empty diagnostic vector layout");if bytes==0{return None;}if bytes>maximum_bytes{return Some(pending_blocked());}*values=Vec::new();Some(pending(bytes))}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
