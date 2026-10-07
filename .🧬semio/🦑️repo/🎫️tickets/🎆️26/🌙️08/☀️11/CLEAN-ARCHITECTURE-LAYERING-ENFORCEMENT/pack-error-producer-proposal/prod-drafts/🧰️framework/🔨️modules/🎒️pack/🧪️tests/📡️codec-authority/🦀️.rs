//! 🧪️ Actual retained parser syntax and owner lifecycle are distinct machine causes.
use crate::codec::*;
use semio_framework_value::ValueRefusalKind;

#[cfg(feature="deflate")]
fn close_authority_cursor(cursor:&mut DeflateRetainedCursor){
 for _ in 0..8{if cursor.close_step(8,usize::MAX)==RetainedInflateCloseStep::Complete{return}}
 panic!("bounded retained authority fixture failed to close")
}

#[cfg(feature="deflate")]
#[test]
fn actual_retained_deflate_syntax_lifecycle_and_requested_ceiling_are_distinct(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../⚠️error/🧫️fixtures/🧭️cause/📡️codec/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let mut grammar=DeflateRetainedCursor::try_new(0,0,0).unwrap();
  let mut terminal=None;
  for byte in row["stored"].as_array().unwrap(){
   grammar.admit_byte(u8::try_from(byte.as_u64().unwrap()).unwrap()).unwrap();
   for _ in 0..64{
    match grammar.grant(false){Ok(DeflateRetainedStep::NeedInput)=>{},Ok(DeflateRetainedStep::Complete)=>break,Ok(DeflateRetainedStep::Byte(_))=>panic!("empty grammar corpus produced a byte"),Err(error)=>{terminal=Some(error);break}}
    if grammar.can_admit(){break}
   }
   if terminal.is_some(){break}
  }
  if terminal.is_none(){
   let mut complete=false;
   for _ in 0..64{match grammar.grant(true){Ok(DeflateRetainedStep::Complete)=>{complete=true;break},Ok(DeflateRetainedStep::NeedInput)=>{},Ok(DeflateRetainedStep::Byte(_))=>panic!("empty grammar corpus produced a byte"),Err(error)=>{terminal=Some(error);break}}}
   assert!(complete||terminal.is_some(),"bounded grammar fixture did not terminate");
  }
  close_authority_cursor(&mut grammar);
  assert_eq!(terminal.and_then(|error|error.refusal_kind().map(ValueRefusalKind::as_str)),row["expectedKind"].as_str());
 }
 let mut lifecycle=DeflateRetainedCursor::try_new(0,0,0).unwrap();
 let reset=lifecycle.reset(0,0);
 close_authority_cursor(&mut lifecycle);
 assert_eq!(reset.unwrap_err().refusal_kind(),Some(ValueRefusalKind::InvariantViolated));
 let mut syntax=DeflateRetainedCursor::try_new(0,0,0).unwrap();
 let admitted=syntax.admit_byte(7);
 let refusal=syntax.grant(true);
 let repeated=syntax.grant(true);
 close_authority_cursor(&mut syntax);
 assert!(admitted.is_ok());
 let refusal=refusal.unwrap_err();
 assert_eq!(refusal.refusal_kind(),Some(ValueRefusalKind::InvalidValue));
 assert_eq!(repeated.unwrap_err(),refusal);
 let ceiling=match DeflateRetainedCursor::try_new(1,0,0){Ok(mut cursor)=>{close_authority_cursor(&mut cursor);panic!("declared retained ceiling accepted")},Err(error)=>error};
 assert_eq!(ceiling.refusal_kind(),Some(ValueRefusalKind::WorkLimit));
 for (name,kind) in [("physicalHistory",ValueRefusalKind::OwnershipLimit),("semanticLength",ValueRefusalKind::WorkLimit)]{
  let row=&fixture["retainedAdmission"][name];
  let expected=row["expectedRawBytes"].as_u64().unwrap();
  let semantic=row["semanticSegmentBytes"].as_u64().unwrap();
  let physical=usize::try_from(row["maximumAllocationBytes"].as_u64().unwrap()).unwrap();
  let refusal=match DeflateRetainedCursor::try_new(expected,semantic,physical){Ok(mut cursor)=>{close_authority_cursor(&mut cursor);panic!("closed retained admission ceiling accepted")},Err(error)=>error};
  assert_eq!(refusal.refusal_kind(),Some(kind));
  assert_eq!(refusal.refusal_kind().map(ValueRefusalKind::as_str),row["expectedKind"].as_str());
 }
 eprintln!("[DEBUG] Pack retained actual syntax/lifecycle/ceiling producers remained distinct");
}
