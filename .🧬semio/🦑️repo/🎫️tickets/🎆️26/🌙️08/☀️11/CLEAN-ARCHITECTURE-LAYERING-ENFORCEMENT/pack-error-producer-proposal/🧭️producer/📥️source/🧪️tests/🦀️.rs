//! 📥️ Unmounted Source typed metadata and actual private constructor laws.
use super::*;

#[test]
fn retained_source_preserves_actual_kind_reason_and_allocation_witness(){
 const REASON:&str="same deliberately misleading canceled/work/allocation prose";
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let kind=match row["cause"]["kind"].as_str().unwrap(){"invalidValue"=>ValueRefusalKind::InvalidValue,"canceled"=>ValueRefusalKind::Canceled,"ownershipLimit"=>ValueRefusalKind::OwnershipLimit,"allocationFailed"=>ValueRefusalKind::AllocationFailed,"workLimit"=>ValueRefusalKind::WorkLimit,"depthLimit"=>ValueRefusalKind::DepthLimit,"unsupportedOwner"=>ValueRefusalKind::UnsupportedOwner,"invariantViolated"=>ValueRefusalKind::InvariantViolated,_=>panic!("closed Source kind unknown")};
  let fault=match row["cause"]["type"].as_str().unwrap(){
   "value"=>RetainedPackSourceFault::refusal(kind,REASON),
   "paged"|"allocation"=>{
    let source_kind=match kind{ValueRefusalKind::OwnershipLimit=>PagedListRefusalKind::OwnershipLimit,ValueRefusalKind::AllocationFailed=>PagedListRefusalKind::AllocationFailed,ValueRefusalKind::InvariantViolated=>PagedListRefusalKind::InvariantViolated,_=>panic!("closed Source lower kind unknown")};
    if row["cause"]["type"]=="paged"{RetainedPackSourceFault::from_paged_refusal(PagedListError{kind:source_kind,reason:REASON})}else{RetainedPackSourceFault::from_paged_allocation(PagedListAllocationError{kind:source_kind,reason:REASON,allocated_bytes:row["cause"]["allocatedBytes"].as_u64().unwrap() as usize})}
   },_=>panic!("closed Source cause unknown"),
  };
  assert_eq!(fault.kind,kind);assert_eq!(fault.reason.as_ptr(),REASON.as_ptr());assert_eq!(fault.allocated_bytes,row["expected"]["allocatedBytes"].as_u64().map(|bytes|bytes as usize));
  let error=fault.into_pack_error("retained-pack-source",7);assert_eq!(error.refusal_kind(),Some(kind));match error{PackError::RetainedMalformed{offset,detail,..}=>{assert_eq!(fault.allocated_bytes,None);assert_eq!(offset,7);assert_eq!(detail.as_ptr(),REASON.as_ptr());},PackError::RetainedAllocation{allocated_bytes,offset,detail,..}=>{assert_eq!(Some(allocated_bytes),fault.allocated_bytes);assert_eq!(offset,7);assert_eq!(detail.as_ptr(),REASON.as_ptr());},_=>panic!("Source machine cause erased")}
 }
 assert_eq!(RetainedPackSourceCursor::try_new(0,1,1).err().unwrap().kind,ValueRefusalKind::InvalidValue);
 assert_eq!(RetainedPackSourceCursor::try_new(1,1,0).err().unwrap().kind,ValueRefusalKind::OwnershipLimit);
 assert_eq!(RetainedPackSourceCursor::try_new(RETAINED_PACK_MAXIMUM_PAGES+1,1,4096).err().unwrap().kind,ValueRefusalKind::OwnershipLimit);
 let mut source=RetainedPackSourceCursor::try_new(1,1,1).unwrap();assert_eq!(source.next_allocation_bytes().err().unwrap().kind,ValueRefusalKind::OwnershipLimit);source.request_cancel();assert_eq!(source.grant().err().unwrap().kind,ValueRefusalKind::Canceled);while source.close_step(64,usize::MAX).unwrap()!=RetainedPackCloseStep::Complete{}
 let mut source=RetainedPackSourceCursor::try_new(1,1,32768).unwrap();assert_eq!(source.preflight_page(0).err().unwrap().kind,ValueRefusalKind::InvalidValue);assert_eq!(source.preflight_page(2).err().unwrap().kind,ValueRefusalKind::WorkLimit);assert_eq!(source.preflight_page(1).err().unwrap().kind,ValueRefusalKind::InvariantViolated);while source.close_step(64,usize::MAX).unwrap()!=RetainedPackCloseStep::Complete{}
 assert_eq!(RetainedPackSegmentCursor::try_new(PackLimits{max_segment_len:0,..PackLimits::default()},4096).err().unwrap().refusal_kind(),Some(ValueRefusalKind::InvalidValue));
 assert_eq!(RetainedPackSegmentCursor::try_new(PackLimits::default(),usize::MAX).err().unwrap().refusal_kind(),Some(ValueRefusalKind::OwnershipLimit));
 let mut segment=RetainedPackSegmentCursor::try_new(PackLimits::default(),4096).unwrap();segment.admit(RetainedPackSourceEvent::Byte{offset:1,value:0}).unwrap();assert_eq!(segment.grant().err().unwrap().refusal_kind(),Some(ValueRefusalKind::InvariantViolated));while segment.close_step(64,usize::MAX)!=RetainedPackCloseStep::Complete{}
 let limits=PackLimits::default();let maximum=limits.max_file_len;let mut segment=RetainedPackSegmentCursor::try_new(limits,4096).unwrap();segment.total=maximum;segment.admit(RetainedPackSourceEvent::Byte{offset:maximum,value:0}).unwrap();assert_eq!(segment.grant().err().unwrap().refusal_kind(),Some(ValueRefusalKind::WorkLimit));while segment.close_step(64,usize::MAX)!=RetainedPackCloseStep::Complete{}
 assert_eq!(check_allocation_layout::<u64>(usize::MAX,"same deliberately misleading canceled/work/allocation prose").err().unwrap().refusal_kind(),Some(ValueRefusalKind::OwnershipLimit));
 let mut segment=RetainedPackSegmentCursor::try_new(PackLimits::default(),4096).unwrap();segment.pending=Some(RetainedPackSourceEvent::Byte{offset:0,value:0});assert!(matches!(segment.preflight(),Err(RetainedPackSegmentAdmission::Pending)));segment.pending=None;segment.fault=Some(PackError::ValueRefusal(semio_framework_value::ValueError::new(ValueRefusalKind::Canceled,"same misleading structural grammar prose")));match segment.preflight().err().unwrap(){RetainedPackSegmentAdmission::Fault(error)=>{assert_eq!(error.refusal_kind(),Some(ValueRefusalKind::Canceled));assert!(std::ptr::eq(error,segment.fault.as_ref().unwrap()));},_=>panic!("stored fault identity erased")};while segment.close_step(64,usize::MAX)!=RetainedPackCloseStep::Complete{}
 for row in fixture["admissions"].as_array().unwrap(){
  let state=&row["state"];let mut segment=RetainedPackSegmentCursor::try_new(PackLimits::default(),65536).unwrap();
  if state["fault"].as_bool().unwrap(){segment.fault=Some(PackError::ValueRefusal(semio_framework_value::ValueError::new(ValueRefusalKind::Canceled,"same misleading structural grammar prose")));}
  segment.closed=state["closed"].as_bool().unwrap();
  if state["complete"].as_bool().unwrap(){segment.phase=RetainedPackSegmentPhase::Complete;}
  if state["pending"].as_bool().unwrap(){segment.pending=Some(RetainedPackSourceEvent::Byte{offset:0,value:0});}
  #[cfg(feature="deflate")]
  if row["expected"]=="inflaterBackpressure"{segment.phase=RetainedPackSegmentPhase::Payload;segment.segment.flags=1;segment.inflater=Some(crate::codec::DeflateRetainedCursor::try_new(1,16,65536).unwrap());}
  #[cfg(not(feature="deflate"))]
  if row["expected"]=="inflaterBackpressure"{while segment.close_step(64,usize::MAX)!=RetainedPackCloseStep::Complete{}continue;}
  let actual=match segment.preflight(){Ok(())=>"ready",Err(RetainedPackSegmentAdmission::Fault(error))=>{assert!(std::ptr::eq(error,segment.fault.as_ref().unwrap()));assert_eq!(error.refusal_kind(),Some(ValueRefusalKind::Canceled));"fault"},Err(RetainedPackSegmentAdmission::Closed)=>"closed",Err(RetainedPackSegmentAdmission::Complete)=>"complete",Err(RetainedPackSegmentAdmission::Pending)=>"pending",Err(RetainedPackSegmentAdmission::InflaterBackpressure)=>"inflaterBackpressure"};
  assert_eq!(actual,row["expected"].as_str().unwrap());segment.closed=false;while segment.close_step(64,usize::MAX)!=RetainedPackCloseStep::Complete{}
 }
 eprintln!("[DEBUG] native Source precise factories/constructors/cancellation and borrowed admission laws preserve source metadata");
}
