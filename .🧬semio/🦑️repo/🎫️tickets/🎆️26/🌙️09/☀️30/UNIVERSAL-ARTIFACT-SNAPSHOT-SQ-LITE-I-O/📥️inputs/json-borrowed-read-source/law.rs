
#[test]
fn retained_json_borrowed_source_uses_original_pages_utf8_and_refusal_owner(){
    use semio_framework_value::{NativeDecodeControl,ValueRefusalKind,list::PagedList};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🫳️read-source.json")).unwrap();
    let retire=|mut cursor:Box<dyn semio_framework_value::ErasedSnapshotRetirement>|{
        assert!(matches!(cursor.close_step(0,0).unwrap(),semio_framework_value::SnapshotRetirementStep::Pending{released_items:0,released_bytes:0}));
        for _ in 0..100000{if cursor.terminal_is_empty(){return;}cursor.close_step(1,131072).unwrap();}
        panic!("original parser candidate owner did not retire");
    };
    let text=fixture["source"].as_str().unwrap();
    let source=PagedList::<u8,16384>::try_from_iter(text.bytes()).unwrap();
    let pointer=source.get(0).unwrap()as *const u8;
    let mut accepted=|_|true;let mut control=NativeDecodeControl::new(fixture["decodeAllocationBytes"].as_u64().unwrap()as usize,&mut accepted);
    let mut cursor=JsonBorrowedParseCursor::new(&source,JsonMemberPolicy::Reject);
    let parsed=loop{if let Some(value)=cursor.step(fixture["stepUnits"].as_u64().unwrap()as usize,&mut control).unwrap(){break value;}};
    assert_eq!(cursor.source().get(0).unwrap()as *const u8,pointer);
    assert_eq!(cursor.position(),source.len());
    let oracle:serde_json::Value=serde_json::from_str(text).unwrap();
    assert_eq!(serde_json::from_str::<serde_json::Value>(&to_json_string(&parsed)).unwrap(),oracle);
    assert_eq!(parsed,parse(text,JsonMemberPolicy::Reject).unwrap());
    retire(cursor.into_retirement());
    let large=format!("\"{}\"", "x".repeat(fixture["largeStringBytes"].as_u64().unwrap()as usize));
    let source=PagedList::<u8,16384>::try_from_iter(large.bytes()).unwrap();
    let mut cursor=JsonBorrowedParseCursor::new(&source,JsonMemberPolicy::Reject);
    let live=std::cell::Cell::new(true);let mut accepted=|_|live.get();let mut control=NativeDecodeControl::new(131072,&mut accepted);
    while cursor.phase()!="materialize-string"{assert!(cursor.step(1,&mut control).unwrap().is_none());}
    for _ in 0..64{assert!(cursor.step(1,&mut control).unwrap().is_none());}
    let bytes=control.owned_bytes();assert!(bytes>=fixture["largeStringBytes"].as_u64().unwrap()as usize);
    let position=cursor.position();let pointer=cursor.source().get(0).unwrap()as *const u8;
    live.set(false);assert_eq!(cursor.step(1,&mut control).unwrap_err().into_value_error().kind,ValueRefusalKind::Canceled);
    assert_eq!(cursor.position(),position);assert_eq!(control.owned_bytes(),bytes);assert_eq!(cursor.source().get(0).unwrap()as *const u8,pointer);
    live.set(true);
    let actual=loop{if let Some(value)=cursor.step(1,&mut control).unwrap(){break value;}};
    assert_eq!(actual,parse(&large,JsonMemberPolicy::Reject).unwrap());retire(cursor.into_retirement());
    for bytes in[fixture["invalidUtf8"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as u8).collect::<Vec<_>>(),fixture["duplicate"].as_str().unwrap().as_bytes().to_vec()]{
        let source=PagedList::<u8,16384>::try_from_iter(bytes.iter().copied()).unwrap();let mut cursor=JsonBorrowedParseCursor::new(&source,JsonMemberPolicy::Reject);
        let error=loop{match cursor.step(1,&mut control){Err(error)=>break error,Ok(None)=>{},Ok(Some(_))=>panic!("invalid literal source accepted")}};
        if bytes[1]==237{assert!(matches!(error,JsonError::InvalidUtf8));}else{assert!(matches!(error,JsonError::DuplicateMember{..}));}
        retire(cursor.into_retirement());
    }
    eprintln!("[DEBUG] retained JSON binds original paged source, matches Serde and keeps paid parser/source on interior refusal; candidate retirement uses its separately declared131072 grant");
}
