#![allow(unused, dead_code)]
pub use native_decoding::NativeDecodeControl;
pub use native_decoding::NativeDecodeProgress;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum ValueRefusalKind{InvalidValue,Canceled,OwnershipLimit,AllocationFailed,WorkLimit,DepthLimit,UnsupportedOwner,InvariantViolated}
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct ValueError{pub kind:ValueRefusalKind,pub message:std::borrow::Cow<'static,str>}
impl ValueError{
 pub fn new(kind:ValueRefusalKind,message:impl Into<String>)->Self{Self{kind,message:std::borrow::Cow::Owned(message.into())}}
 pub const fn literal(kind:ValueRefusalKind,message:&'static str)->Self{Self{kind,message:std::borrow::Cow::Borrowed(message)}}
}
impl std::fmt::Display for ValueError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.write_str(&self.message)}}
impl std::error::Error for ValueError{}
pub mod retirement{pub trait RetireOwned:Send+'static{} impl<T:Send+'static> RetireOwned for T{}}
pub trait ErasedSnapshotRetirement{}
pub mod native_decoding{
 use super::*;
 pub struct NativeDecodeProgress{pub completed:usize,pub total:usize,pub owned_bytes:usize}
 pub struct NativeDecodeRetirementRecipient;
 impl NativeDecodeRetirementRecipient{pub fn new()->Self{Self}}
 pub struct NativeDecodeControl<'a>{p:&'a ()}
 impl<'a> NativeDecodeControl<'a>{
  pub fn scoped_observer<T,E>(&mut self,observer:&mut dyn FnMut(NativeDecodeProgress)->bool,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,E>)->Result<T,E>{unimplemented!()}
  pub fn scoped_depth<T,E:From<ValueError>>(&mut self,maximum:usize,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{unimplemented!()}
  pub fn scoped_stage<T,E>(&mut self,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{unimplemented!()}
  pub fn scoped_maximum<T,E:From<ValueError>>(&mut self,maximum:usize,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{unimplemented!()}
  pub fn new(maximum_bytes:usize,callback:&'a mut dyn FnMut(NativeDecodeProgress)->bool)->Self{unimplemented!()}
  pub fn owned_bytes(&self)->usize{0}
  pub fn checkpoint(&mut self)->Result<(),ValueError>{Ok(())}
  pub fn begin_stage(&mut self,total:usize)->Result<(),ValueError>{Ok(())}
  pub fn advance(&mut self,units:usize)->Result<(),ValueError>{Ok(())}
  pub fn step(&mut self)->Result<(),ValueError>{Ok(())}
  pub fn charge(&mut self,bytes:usize)->Result<(),ValueError>{Ok(())}
  pub fn allocate_vec<T>(&mut self,count:usize)->Result<Vec<T>,ValueError>{unimplemented!()}
  pub fn borrow_text<'text>(&mut self,bytes:&'text[u8])->Result<&'text str,ValueError>{unimplemented!()}
  pub fn copy_text(&mut self,text:&str)->Result<String,ValueError>{unimplemented!()}
  pub fn copy_text_into(&mut self,text:&str,output:&mut String)->Result<(),ValueError>{unimplemented!()}
  pub fn copy_bytes_into(&mut self,bytes:&[u8],output:&mut Vec<u8>)->Result<(),ValueError>{unimplemented!()}
  pub fn copy_bytes(&mut self,bytes:&[u8])->Result<Vec<u8>,ValueError>{unimplemented!()}
 }
}

pub mod native_encoding{
 use super::*;
 pub struct NativeEncodeProgress{pub completed:usize,pub total:usize,pub owned_bytes:usize}
 pub struct NativeEncodeControl<'a>{p:&'a ()}
 impl<'a> NativeEncodeControl<'a>{
  pub fn scoped_observer<T,E>(&mut self,observer:&mut dyn FnMut(NativeEncodeProgress)->bool,operation:impl FnOnce(&mut NativeEncodeControl<'_>)->Result<T,E>)->Result<T,E>{unimplemented!()}
  pub fn scoped_maximum<T,E:From<ValueError>>(&mut self,maximum:usize,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{unimplemented!()}
  pub fn scoped_stage<T,E>(&mut self,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{unimplemented!()}
  pub fn owned_bytes(&self)->usize{0}
  pub fn checkpoint(&mut self)->Result<(),ValueError>{Ok(())}
  pub fn begin_stage(&mut self,total:usize)->Result<(),ValueError>{Ok(())}
  pub fn advance(&mut self,units:usize)->Result<(),ValueError>{Ok(())}
  pub fn allocate_vec<T>(&mut self,count:usize)->Result<Vec<T>,ValueError>{unimplemented!()}
 }
}
