//! 🧪️ Portable grammar outcomes and actual retained refusal ownership.
use super::*;
use semio_framework_value::{ErasedSnapshotRetirement,SnapshotRetirementStep};
fn drain(mut owner:Box<dyn ErasedSnapshotRetirement>){
 for _ in 0..1_048_576{
  let bytes=owner.next_close_byte_demand().max(4096);
  if matches!(owner.close_step(1,bytes).expect("actual cursor retirement"),SnapshotRetirementStep::Complete){assert!(owner.terminal_is_empty());return;}
 }
 panic!("retained cursor must reach terminal retirement");
}
#[test]
fn controlled_board_json_matches_portable_serde_grammar(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for case in corpus["cases"].as_array().unwrap(){
  let source=case["source"].as_str().unwrap();
  let mut callbacks=0_usize;let mut progress=|_|{callbacks+=1;callbacks<=65_536};
  let mut cursor=BoardJsonCursor::<semio_framework_pack_json::Value>::new();
  let result=cursor.decode(source,&mut NativeDecodeControl::new(1024*1024,&mut progress));
  let oracle=serde_json::from_str::<serde_json::Value>(source);
  assert_eq!(result.is_ok(),case["accepted"].as_bool().unwrap());
  assert_eq!(result.is_ok(),oracle.is_ok());
  if let Ok(value)=result{
   let actual:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_string(&value)).unwrap();
   assert_eq!(actual,case["expected"]);assert_eq!(actual,oracle.unwrap());
  }
  drain(semio_framework_value::retirement::owned_retirement(cursor));
 }
}
#[test]
fn strict_cursor_refuses_duplicate_zero_owned_budget_and_callback_revocation(){
 let mut calls=0;let mut progress=|_|{calls+=1;calls<=65_536};
 let mut duplicate=BoardJsonCursor::<semio_framework_pack_json::Value>::new();
 assert!(duplicate.decode(r#"{"id":1,"id":2}"#,&mut NativeDecodeControl::new(1024*1024,&mut progress)).is_err());
 drain(semio_framework_value::retirement::owned_retirement(duplicate));
 let mut empty=BoardJsonCursor::<semio_framework_pack_json::Value>::new();
 assert!(empty.decode(r#"{"id":"owned"}"#,&mut NativeDecodeControl::new(0,&mut progress)).is_err());
 drain(semio_framework_value::retirement::owned_retirement(empty));
 for frontier in 0..32{
  let mut calls=0;let mut progress=|_|{calls+=1;calls<=frontier};
  let mut cursor=BoardJsonCursor::<semio_framework_pack_json::Value>::new();
  assert!(cursor.decode(r#"{"nodes":[{"id":"retained","x":1.5}]}"#,&mut NativeDecodeControl::new(1024*1024,&mut progress)).is_err());
  drain(semio_framework_value::retirement::owned_retirement(cursor));
 }
}

