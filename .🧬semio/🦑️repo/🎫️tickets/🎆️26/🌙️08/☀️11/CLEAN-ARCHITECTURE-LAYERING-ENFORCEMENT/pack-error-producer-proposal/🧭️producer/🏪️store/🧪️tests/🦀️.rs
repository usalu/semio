//! 🏪️ Unmounted higher owner machine cause and distinct allocation witness laws.
use super::*;

#[test]
fn higher_owner_keeps_source_cause_and_reservation_charge(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let kind=match row["kind"].as_str().unwrap(){"invalidValue"=>ValueRefusalKind::InvalidValue,"canceled"=>ValueRefusalKind::Canceled,"ownershipLimit"=>ValueRefusalKind::OwnershipLimit,"allocationFailed"=>ValueRefusalKind::AllocationFailed,"workLimit"=>ValueRefusalKind::WorkLimit,"depthLimit"=>ValueRefusalKind::DepthLimit,"unsupportedOwner"=>ValueRefusalKind::UnsupportedOwner,"invariantViolated"=>ValueRefusalKind::InvariantViolated,_=>panic!("closed higher owner kind unknown")};
  let original=match row["sourceBytes"].as_u64(){Some(bytes)=>PackError::RetainedAllocation{kind,allocated_bytes:bytes as usize,what:"actual producer",offset:7,detail:"same canceled allocation grammar prose"},None=>PackError::ValueRefusal(semio_framework_value::ValueError::new(kind,"same canceled allocation grammar prose"))};
  let bytes=row["reservationBytes"].as_u64().unwrap() as usize;let (error,wrapper_bytes)=crate::test_allocation::observe(||RetainedTypedPackAllocationError{allocated_bytes:bytes,fault:original});assert_eq!(wrapper_bytes,0);
  assert_eq!(error.allocated_bytes,bytes);assert_eq!(error.fault.refusal_kind(),Some(kind));
  match (&error.fault,row["sourceBytes"].as_u64()){(PackError::RetainedAllocation{allocated_bytes,..},Some(source))=>assert_eq!(*allocated_bytes,source as usize),(PackError::ValueRefusal(_),None)=>{},_=>panic!("higher source witness erased")}
 }
 struct Owner;
 impl RetainedTypedPackOwner for Owner{
  type Value=();
  fn accept(&mut self,_:mounted::RetainedValueToken,_:&mounted::RetainedPackCatalogCursor,control:&mut NativeDecodeControl<'_>)->Result<(),PackError>{control.checkpoint().map_err(PackError::from)}
  fn grant_symbol(&mut self,_:&mounted::RetainedPackCatalogCursor,control:&mut NativeDecodeControl<'_>)->Result<bool,PackError>{control.checkpoint().map_err(PackError::from)?;Ok(false)}
  fn take(&mut self)->Option<()>{None}
  fn close_step(&mut self)->bool{true}
  fn terminal_is_empty(&self)->bool{true}
 }
 fn owner(control:&mut NativeDecodeControl<'_>)->Result<Owner,PackError>{control.checkpoint().map_err(PackError::from)?;Ok(Owner)}
 let allowed=std::cell::Cell::new(true);let mut policy=|_|allowed.get();let mut control=NativeDecodeControl::new(4096,&mut policy);
 let mut session=RetainedTypedPackSession::<Owner>::new(Vec::new(),1,1,1,owner,&mut control).unwrap();
 allowed.set(false);assert_eq!(session.admit_byte(37,&mut control),Err(37));
 let (kind,projection_bytes)=crate::test_allocation::observe(||session.fault().unwrap().refusal_kind());assert_eq!(projection_bytes,0);assert_eq!(kind,Some(ValueRefusalKind::Canceled));assert_eq!(session.admitted,0);assert!(!session.semantic_allocated());
 allowed.set(true);assert_eq!(session.grant(&mut control).err().unwrap().refusal_kind(),Some(ValueRefusalKind::Canceled));while session.close_step(1,4096).unwrap()!=RetainedTypedPackCloseStep::Complete{}assert!(session.terminal_is_empty());
 assert_eq!(RetainedTypedPackSession::<Owner>::new(Vec::new(),0,1,1,owner,&mut control).err().unwrap().refusal_kind(),Some(ValueRefusalKind::InvalidValue));
 assert_eq!(RetainedTypedPackSession::<Owner>::new(Vec::new(),super::super::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES+1,1,1,owner,&mut control).err().unwrap().refusal_kind(),Some(ValueRefusalKind::WorkLimit));
 eprintln!("[DEBUG] native higher owner retains causes/charges, moves without allocation, and preserves canceled byte handback through final close");
}
