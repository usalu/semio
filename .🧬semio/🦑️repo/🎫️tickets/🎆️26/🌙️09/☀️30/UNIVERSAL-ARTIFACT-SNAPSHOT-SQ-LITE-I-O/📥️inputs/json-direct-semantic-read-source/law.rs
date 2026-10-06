
#[test]
fn retained_json_direct_semantic_source_moves_admitted_original_cells_without_tree_mirror(){
    use semio_framework_value::{DslValue,Number,NativeDecodeControl,list::PagedList};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🫳️read-source.json")).unwrap();
    let text=fixture["source"].as_str().unwrap();let source=PagedList::<u8,16384>::try_from_iter(text.bytes()).unwrap();
    let mut accepted=|_|true;let mut control=NativeDecodeControl::new(131072,&mut accepted);
    let mut cursor=JsonBorrowedDslCursor::new(&source,JsonMemberPolicy::Reject);
    let value=loop{if let Some(value)=cursor.step(1,&mut control).unwrap(){break value;}};
    assert_eq!(cursor.source().get(0).unwrap()as *const u8,source.get(0).unwrap()as *const u8);
    assert_eq!(serde_json::Value::from(&value),serde_json::from_str::<serde_json::Value>(text).unwrap());
    let DslValue::Object(fields)=&value else{panic!("original object")};
    let DslValue::Array(numbers)=&fields.iter().find(|(key,_)|key=="numbers").unwrap().1 else{panic!("original number array")};
    assert!(matches!(numbers[0],DslValue::Number(Number::UInt(u64::MAX))));
    assert!(matches!(numbers[1],DslValue::Number(Number::Int(i64::MIN))));
    assert!(matches!(numbers[2],DslValue::Number(Number::Float(value))if value.to_bits()==(-0.0f64).to_bits()));
    assert!(matches!(numbers[3],DslValue::Number(Number::Float(value))if value.to_bits()==1));
    let mut close=cursor.into_retirement();while !close.terminal_is_empty(){close.close_step(1,131072).unwrap();}
    let source=PagedList::<u8,16384>::try_from_iter(std::iter::once(b'"').chain(std::iter::repeat_n(b'x',fixture["largeStringBytes"].as_u64().unwrap()as usize)).chain(std::iter::once(b'"'))).unwrap();
    let mut cursor=JsonBorrowedDslCursor::new(&source,JsonMemberPolicy::Reject);let mut control=NativeDecodeControl::new(131072,&mut accepted);
    let value=loop{if let Some(value)=cursor.step(1,&mut control).unwrap(){break value;}};
    assert_eq!(control.owned_bytes(),fixture["largeStringBytes"].as_u64().unwrap()as usize);
    let DslValue::String(value)=value else{panic!("direct owned semantic string")};assert_eq!(value.len(),fixture["largeStringBytes"].as_u64().unwrap()as usize);assert!(value.bytes().all(|byte|byte==b'x'));
    let mut close=cursor.into_retirement();while !close.terminal_is_empty(){close.close_step(1,131072).unwrap();}
    eprintln!("[DEBUG] original retained JSON grammar moves native UInt/Int/Float words and one paid8194 String directly into DslValue; no JSON DOM mirror or source flatten");
}
