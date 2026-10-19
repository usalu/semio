#![allow(unused, dead_code)]
use semio_framework_value::{ValueError,NativeDecodeControl};
pub struct PackRefusal;
impl std::fmt::Display for PackRefusal{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.write_str("")}}
pub struct ByteReader<'a>{b:&'a [u8]}
impl<'a> ByteReader<'a>{
 pub fn new(bytes:&'a [u8])->Self{Self{b:bytes}}
 pub fn remaining(&self)->usize{0}
 pub fn read_bytes(&mut self,length:usize)->Result<&'a [u8],PackRefusal>{unimplemented!()}
 pub fn read_u8(&mut self)->Result<u8,PackRefusal>{unimplemented!()}
 pub fn read_u16_le(&mut self)->Result<u16,PackRefusal>{unimplemented!()}
 pub fn read_u32_le(&mut self)->Result<u32,PackRefusal>{unimplemented!()}
 pub fn read_u64_le(&mut self)->Result<u64,PackRefusal>{unimplemented!()}
 pub fn read_f64_le(&mut self)->Result<f64,PackRefusal>{unimplemented!()}
 pub fn read_varint_u64(&mut self)->Result<u64,PackRefusal>{unimplemented!()}
}
pub mod io{pub enum IoPayload{Binary(Vec<u8>),Text(String)}}
pub mod semio_format{
 use super::*;
 pub enum Component{Pack,Dsl}
 pub struct SemioError;
 impl SemioError{pub fn into_value_error(self)->ValueError{unimplemented!()}}
 pub type SemioResult<T>=Result<T,SemioError>;
 pub fn unwrap_binary_controlled<'input>(bytes:&'input[u8],envelope_id:&str,component:Component,version:u16,control:&mut NativeDecodeControl<'_>)->Result<&'input[u8],ValueError>{unimplemented!()}
 pub fn split_text_preamble_controlled<'input>(text:&'input str,envelope_id:&str,component:Component,version:u16,control:&mut NativeDecodeControl<'_>)->SemioResult<&'input str>{unimplemented!()}
}
pub mod sqlite_snapshot{
 use super::*;
 #[derive(Clone,Copy,Debug,PartialEq,Eq)] pub enum SnapshotEncoding{Binary,Text}
 pub mod artifact{use super::*;use semio_framework_value::retirement::RetireOwned;pub fn owned_workspace<T:RetireOwned,R>(control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase,empty:impl FnOnce()->T,build:impl FnOnce(&mut T,&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>,take:impl FnOnce(&mut T)->R)->Result<R,ValueError>{unimplemented!()}}
 #[derive(Clone,Copy,Debug)]
 pub struct SqliteDatabaseLimits{pub max_file_bytes:usize,pub max_value_bytes:usize,pub max_allocation_bytes:usize,pub max_schema_bytes:usize,pub max_rows:usize,pub max_columns:usize,pub max_tables:usize,pub max_pages:usize}
 #[derive(Clone,Copy,Debug,PartialEq,Eq)]
 pub enum SqliteSnapshotPhase{ReadPages,WritePages,ProjectSnapshot,ReconstructSnapshot,DecodeNative,EncodeNative}
 pub struct SqliteSnapshotControl<'a>{p:&'a ()}
 impl<'a> SqliteSnapshotControl<'a>{
  pub fn limits(&self)->SqliteDatabaseLimits{unimplemented!()}
  pub fn with_retirement_child<T,E:From<ValueError>>(&mut self,recipient:&mut semio_framework_value::native_decoding::NativeDecodeRetirementRecipient,operation:impl FnOnce(&mut SqliteSnapshotControl<'_>)->std::result::Result<T,E>)->std::result::Result<T,E>{unimplemented!()}
  pub fn allocation_stage_native<T,E>(&mut self,phase:SqliteSnapshotPhase,operation:impl FnOnce(usize,&mut dyn FnMut(usize,usize)->bool)->(std::result::Result<T,E>,usize))->std::result::Result<std::result::Result<T,E>,ValueError>{unimplemented!()}
 }
}
pub mod os_store{pub mod artifact_child{
 use std::marker::PhantomData;
 pub struct ArtifactChild<S>{pub child_id:String,pub target:semio_framework_artifact_reference::ArtifactRef,pub local_owner:Option<std::sync::Arc<dyn std::any::Any+Send+Sync>>,pub _snapshot:PhantomData<S>}
 impl<S> Clone for ArtifactChild<S>{fn clone(&self)->Self{unimplemented!()}}
 impl<S> std::fmt::Debug for ArtifactChild<S>{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{Ok(())}}
 impl<S> PartialEq for ArtifactChild<S>{fn eq(&self,o:&Self)->bool{true}}
 impl<S> ArtifactChild<S>{pub fn new(child_id:String,target:semio_framework_artifact_reference::ArtifactRef)->Self{Self{child_id,target,local_owner:None,_snapshot:PhantomData}}}
}}
#[derive(Clone,Debug,PartialEq)]
pub struct BlobRef{pub hash:String,pub size:u64,pub media_type:String}
#[derive(Clone,Debug,PartialEq)]
pub enum LinkPin{Head,Checkpoint{id:String},Snapshot{blob:BlobRef}}
#[derive(Clone,Debug,PartialEq)]
pub struct ArtifactLink{pub target:semio_framework_artifact_reference::ArtifactRef,pub pin:LinkPin,pub role:String}

pub struct NativeSnapshotEncodeOwner<'owner,'control>{native:&'owner mut semio_framework_value::native_encoding::NativeEncodeControl<'control>}
impl<'owner,'control> NativeSnapshotEncodeOwner<'owner,'control>{pub fn native(&mut self)->&mut semio_framework_value::native_encoding::NativeEncodeControl<'control>{self.native}}
