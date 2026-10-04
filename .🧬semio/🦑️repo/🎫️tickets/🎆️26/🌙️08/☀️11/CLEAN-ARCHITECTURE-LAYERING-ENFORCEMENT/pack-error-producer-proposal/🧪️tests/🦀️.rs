//! 🧪️ Test-only canonical Pack cause floor; native mount and compilation remain pending.
use super::*;
use semio_framework_diagnostic::TextSpan;

fn authored_kind(text:&str)->ValueRefusalKind {
 match text {
  "invalidValue"=>ValueRefusalKind::InvalidValue,"canceled"=>ValueRefusalKind::Canceled,
  "ownershipLimit"=>ValueRefusalKind::OwnershipLimit,"allocationFailed"=>ValueRefusalKind::AllocationFailed,
  "workLimit"=>ValueRefusalKind::WorkLimit,"depthLimit"=>ValueRefusalKind::DepthLimit,
  "unsupportedOwner"=>ValueRefusalKind::UnsupportedOwner,"invariantViolated"=>ValueRefusalKind::InvariantViolated,
  _=>panic!("unknown source-authored kind")
 }
}

#[test]
fn canonical_pack_producer_authority_is_required_and_preserves_owned_causes() {
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let expected=authored_kind(row["expectedKind"].as_str().unwrap());
  let message=row["message"].as_str().unwrap();
  let what="deflate";
  let offset=row["offset"].as_str().unwrap().parse::<u64>().unwrap();
  let mut owned_pointer=None;
  let error=match row["variant"].as_str().unwrap(){
   "BadMagic"=>PackError::BadMagic,
   "UnsupportedVersion"=>PackError::UnsupportedVersion{major:2,minor:3},
   "UnknownRequiredFlags"=>PackError::UnknownRequiredFlags(4),
   "Truncated"=>PackError::Truncated(offset),
   "ChecksumMismatch"=>PackError::ChecksumMismatch{segment:what,offset},
   "ContentHashMismatch"=>PackError::ContentHashMismatch,
   "NonCanonical"=>PackError::NonCanonical("same deliberately misleading cancellation/limit prose"),
   "UnsupportedCodec"=>PackError::UnsupportedCodec(7),
   "LimitExceeded"=>PackError::LimitExceeded{kind:authored_kind(row["kind"].as_str().unwrap()),limit:"same deliberately misleading cancellation/limit prose"},
   "RetainedMalformed"=>PackError::RetainedMalformed{kind:authored_kind(row["kind"].as_str().unwrap()),what,offset,detail:"same deliberately misleading cancellation/limit prose"},
   "RetainedAllocation"=>PackError::RetainedAllocation{kind:authored_kind(row["kind"].as_str().unwrap()),allocated_bytes:usize::try_from(row["allocatedBytes"].as_u64().unwrap()).unwrap(),what,offset,detail:"same deliberately misleading cancellation/limit prose"},
   "Malformed"=>PackError::Malformed{kind:authored_kind(row["kind"].as_str().unwrap()),what,offset,detail:message.to_owned()},
   "ValueRefusal"=>{let cause=ValueError::new(authored_kind(row["kind"].as_str().unwrap()),message);owned_pointer=Some(cause.message.as_ptr());PackError::from(cause)},
   "TextRefusal"=>{let cause=TextError::new(authored_kind(row["kind"].as_str().unwrap()),message,TextSpan{line:2,column:3,length:1});owned_pointer=Some(cause.message.as_ptr());PackError::from(cause)},
   "Io"=>{let cause=ValueError::new(authored_kind(row["kind"].as_str().unwrap()),message);owned_pointer=Some(cause.message.as_ptr());PackError::Io{error:cause,retry:match row["retry"].as_str().unwrap(){"never"=>PackRetryDisposition::Never,"transient"=>PackRetryDisposition::Transient,_=>panic!("unknown source-authored retry policy")}}},
   _=>panic!("unknown authored Pack variant")
  };
  assert_eq!(error.refusal_kind(),Some(expected));
  assert_eq!(error.cause_kind(),PackCauseKind::Refusal(expected));
  let retry=match &error{PackError::Io{retry,..}=>Some(match retry{PackRetryDisposition::Never=>"never",PackRetryDisposition::Transient=>"transient"}),_=>None};
  let allocated_bytes=match &error{PackError::RetainedAllocation{allocated_bytes,..}=>Some(*allocated_bytes),_=>None};
  let actual=serde_json::json!({"kind":error.refusal_kind().map(ValueRefusalKind::as_str),"display":error.to_string(),"retry":retry,"allocatedBytes":allocated_bytes});
  let reference=serde_json::json!({"kind":row["expectedKind"],"display":row["display"],"retry":row["retry"],"allocatedBytes":row["allocatedBytes"]});
  assert_eq!(actual,reference);
  if let Some(pointer)=owned_pointer{
   let source=std::error::Error::source(&error).unwrap();
   match &error{
    PackError::ValueRefusal(cause)|PackError::Io{error:cause,..}=>{assert_eq!(cause.message.as_ptr(),pointer);assert!(std::ptr::eq(source.downcast_ref::<ValueError>().unwrap(),cause));},
    PackError::TextRefusal(cause)=>{assert_eq!(cause.message.as_ptr(),pointer);assert!(std::ptr::eq(source.downcast_ref::<TextError>().unwrap(),cause));},
    _=>panic!("owned source was erased")
   }
  }
  if let PackError::RetainedAllocation{allocated_bytes,..}=&error{assert_eq!(*allocated_bytes as u64,row["allocatedBytes"].as_u64().unwrap());}
 }
 eprintln!("[DEBUG] canonical Pack projected {} source-authored causes",fixture["cases"].as_array().unwrap().len());
}
