
#[test]
fn retained_json_borrowed_source_honors_complete_caller_extent_depth_and_paid_allocation(){
    use semio_framework_value::{DslValue,NativeDecodeControl,ValueRefusalKind,list::PagedList};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🫳️read-limits.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){
        let text=row["source"].as_str().unwrap();let source=PagedList::<u8,16384>::try_from_iter(text.bytes()).unwrap();
        let limits=JsonReadLimits{maximum_bytes:text.len()as u64,maximum_allocation_bytes:fixture["decodeAllocationBytes"].as_u64().unwrap()as usize,maximum_depth:row["maximumDepth"].as_u64().unwrap()as usize,maximum_items:row["maximumItems"].as_u64().unwrap()};
        let mut cursor=JsonBorrowedDslCursor::new_with_limits(&source,JsonMemberPolicy::Reject,limits).unwrap();let mut accepted=|_|true;let mut control=NativeDecodeControl::new(262144,&mut accepted);
        let result=loop{match cursor.step(1,&mut control){Ok(Some(value))=>break Ok(value),Err(error)=>break Err(error),Ok(None)=>{}}};
        assert_eq!(control.maximum_bytes(),262144);
        if row["accept"].as_bool().unwrap(){assert_eq!(serde_json::Value::from(&result.unwrap()),serde_json::from_str::<serde_json::Value>(text).unwrap());}
        else{assert_eq!(result.unwrap_err().into_value_error().kind,match row["kind"].as_str().unwrap(){"WorkLimit"=>ValueRefusalKind::WorkLimit,"DepthLimit"=>ValueRefusalKind::DepthLimit,_=>panic!("closed literal refusal")});}
        let mut close=cursor.into_retirement();while !close.terminal_is_empty(){close.close_step(1,131072).unwrap();}
        let short=JsonReadLimits{maximum_bytes:text.len()as u64-1,..limits};assert_eq!(JsonBorrowedDslCursor::new_with_limits(&source,JsonMemberPolicy::Reject,short).err().unwrap().kind,ValueRefusalKind::WorkLimit);
    }
    let source=PagedList::<u8,16384>::try_from_iter(std::iter::once(b'"').chain(std::iter::repeat_n(b'x',fixture["largeStringBytes"].as_u64().unwrap()as usize)).chain(std::iter::once(b'"'))).unwrap();
    let mut accepted=|_|true;let mut control=NativeDecodeControl::new(262144,&mut accepted);
    let limits=JsonReadLimits{maximum_bytes:source.len()as u64,maximum_allocation_bytes:fixture["refusedAllocationBytes"].as_u64().unwrap()as usize,maximum_depth:8,maximum_items:8};
    let mut cursor=JsonBorrowedDslCursor::new_with_limits(&source,JsonMemberPolicy::Reject,limits).unwrap();
    let error=loop{match cursor.step(1,&mut control){Err(error)=>break error,Ok(Some(_))=>panic!("original paid string must refuse"),Ok(None)=>{}}};
    assert_eq!(error.into_value_error().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(control.owned_bytes(),0);assert_eq!(control.maximum_bytes(),262144);
    assert_eq!(cursor.source().len(),source.len());let mut close=cursor.into_retirement();while !close.terminal_is_empty(){close.close_step(1,131072).unwrap();}
    let _=std::mem::size_of::<DslValue>();
    eprintln!("[DEBUG] original bound JSON source keeps complete byte limits, per-collection extents, depth and cumulative allocation with exact caller control restored");
}
