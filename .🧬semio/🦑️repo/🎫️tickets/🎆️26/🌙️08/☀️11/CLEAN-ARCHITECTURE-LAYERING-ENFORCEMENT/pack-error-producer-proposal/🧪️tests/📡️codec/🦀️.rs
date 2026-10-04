//! 🧪️ Unmounted actual retained parser syntax and owner lifecycle are distinct machine causes.
use super::*;
use semio_framework_value::ValueRefusalKind;

#[cfg(feature="deflate")]
fn close_authority_cursor(cursor:&mut DeflateRetainedCursor){
 for _ in 0..8{if cursor.close_step(8,usize::MAX)==RetainedInflateCloseStep::Complete{return}}
 panic!("bounded retained authority fixture failed to close")
}

#[cfg(feature="deflate")]
#[test]
fn actual_retained_deflate_syntax_lifecycle_and_requested_ceiling_are_distinct(){
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
 assert_eq!(ceiling.refusal_kind(),Some(ValueRefusalKind::OwnershipLimit));
 eprintln!("[DEBUG] Pack retained actual syntax/lifecycle/ceiling producers remained distinct");
}
