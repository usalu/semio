use super::*;
use semio_framework_value::{ValueError,ValueRefusalKind};

fn refusal_kind(name:&str)->ValueRefusalKind{match name{
 "invalidValue"=>ValueRefusalKind::InvalidValue,"canceled"=>ValueRefusalKind::Canceled,
 "ownershipLimit"=>ValueRefusalKind::OwnershipLimit,"allocationFailed"=>ValueRefusalKind::AllocationFailed,
 "workLimit"=>ValueRefusalKind::WorkLimit,"depthLimit"=>ValueRefusalKind::DepthLimit,
 "unsupportedOwner"=>ValueRefusalKind::UnsupportedOwner,"invariantViolated"=>ValueRefusalKind::InvariantViolated,
 _=>panic!("unknown owned refusal kind")}}

struct RefusalControl{mode:&'static str,error:Option<ValueError>,checkpoints:usize,admissions:usize}
impl DeflateEncodeControl for RefusalControl{
 fn admit(&mut self,_bytes:usize)->Result<(),ValueError>{self.admissions+=1;if self.mode=="admit"{Err(self.error.take().unwrap())}else{Ok(())}}
 fn checkpoint(&mut self,_event:DeflateEncodeProgress)->Result<(),ValueError>{self.checkpoints+=1;if self.mode=="checkpoint"{Err(self.error.take().unwrap())}else{Ok(())}}
}

#[test]
fn deflate_controlled_retains_every_refusal_kind_and_owns_internal_causes(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧫️fixtures/⚠️refusal/🔣️.json")).unwrap();
 for row in fixture["callbacks"].as_array().unwrap(){
  let error=ValueError::new(refusal_kind(row["kind"].as_str().unwrap()),row["message"].as_str().unwrap().to_owned());let pointer=error.message.as_ptr();
  let mode=if row["mode"]=="admit"{"admit"}else{"checkpoint"};let mut control=RefusalControl{mode,error:Some(error),checkpoints:0,admissions:0};
  let actual:ValueError=deflate_controlled(b"owned control cause",&mut control).unwrap_err();
  assert_eq!(actual.kind,refusal_kind(row["kind"].as_str().unwrap()));assert_eq!(actual.message,row["message"].as_str().unwrap());assert_eq!(actual.message.as_ptr(),pointer);
  assert_eq!(control.checkpoints,row["checkpointCount"].as_u64().unwrap() as usize);assert_eq!(control.admissions,row["admissionCount"].as_u64().unwrap() as usize);
  assert_eq!(serde_json::json!({"kind":actual.kind.as_str(),"message":actual.message}),serde_json::json!({"kind":row["kind"],"message":row["message"]}));
 }
 for row in fixture["owned"].as_array().unwrap(){
  let mut control=RefusalControl{mode:"success",error:None,checkpoints:0,admissions:0};let mut completed=0;
  let actual:ValueError=if row["id"]=="match-work-overflow"{completed=usize::MAX;controlled_search_step(&mut control,&mut completed).unwrap_err()}else{controlled_positions(usize::MAX,&mut control,&mut completed,usize::MAX).unwrap_err()};
  assert_eq!(actual.kind,refusal_kind(row["kind"].as_str().unwrap()));assert_eq!(actual.message,row["message"].as_str().unwrap());
 }
 for row in fixture["samples"].as_array().unwrap(){
  let hex=row["hex"].as_str().unwrap();let raw:Vec<u8>=(0..hex.len()).step_by(2).map(|index|u8::from_str_radix(&hex[index..index+2],16).unwrap()).collect();let raw=raw.repeat(row["repeat"].as_u64().unwrap() as usize);
  let mut control=RefusalControl{mode:"success",error:None,checkpoints:0,admissions:0};let stored=deflate_controlled(&raw,&mut control).unwrap();
  assert_eq!(stored,deflate(&raw));assert_eq!(miniz_oxide::inflate::decompress_to_vec_with_limit(&stored,raw.len().max(1)).unwrap(),raw);
 }
 eprintln!("[DEBUG] Deflate refusal callbacks=16 owned causes=2 independent inflate samples=2");
}

#[test]
fn deflate_controlled_retained_allocation_errors_retain_the_owned_kind(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧫️fixtures/⚠️refusal/🔣️.json")).unwrap();
 for row in fixture["retained"].as_array().unwrap(){
  let requested=row["requestedBytes"].as_u64().unwrap() as usize;let mut history=RetainedInflateHistory::new(requested,requested).unwrap();
  if row["id"]=="allocator-refused"{DENY_ALLOCATION_SIZE.store(requested,std::sync::atomic::Ordering::Relaxed);}else{history.bytes=std::mem::ManuallyDrop::new(Vec::with_capacity(row["allocatedBytes"].as_u64().unwrap() as usize));}
  let result=history.reserve(requested);DENY_ALLOCATION_SIZE.store(0,std::sync::atomic::Ordering::Relaxed);
  let repeated=history.reserve(requested);let allocated=history.allocated_bytes();history.release(allocated);assert_eq!(history.allocated_bytes(),0);
  let error=result.unwrap_err();assert_eq!(error.kind,refusal_kind(row["kind"].as_str().unwrap()));assert_eq!(error.reason,row["reason"].as_str().unwrap());assert_eq!(error.allocated_bytes,row["allocatedBytes"].as_u64().unwrap() as usize);assert_eq!(repeated.unwrap_err(),error);
 }
 eprintln!("[DEBUG] Deflate retained allocator-refusal and physical-ceiling owned kinds=2");
}
