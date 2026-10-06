#[test]
fn retained_json_cursor_owns_exact_borrowed_source_view(){
    use semio_framework_value::{DslValue,NativeDecodeControl,list::PagedList};
    #[derive(Clone,Copy)]
    struct OriginalView<'source>{owner:&'source PagedList<u8,16384>,start:usize,length:usize}
    impl JsonReadSource for OriginalView<'_>{fn byte_len(&self)->usize{self.length}fn byte_at(&self,index:usize)->Option<u8>{(index<self.length).then(||self.owner.get(self.start+index).copied()).flatten()}}
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🫳️read-source.json")).unwrap();let text=fixture["source"].as_str().unwrap();
    let source=PagedList::<u8,16384>::try_from_iter(std::iter::once(b'!').chain(text.bytes()).chain(std::iter::once(b'?'))).unwrap();
    let pointer=source.get(1).unwrap()as *const u8;
    let view=OriginalView{owner:&source,start:1,length:text.len()};
    let limits=JsonReadLimits{maximum_bytes:text.len()as u64,maximum_allocation_bytes:131072,maximum_depth:8,maximum_items:4};
    let mut cursor=JsonSourceCursor::<_,DslValue>::new_with_limits(view,JsonMemberPolicy::Reject,limits).unwrap();
    let live=std::cell::Cell::new(true);let mut accepted=|_|live.get();let mut control=NativeDecodeControl::new(262144,&mut accepted);
    for _ in 0..4{assert!(cursor.step(1,&mut control).unwrap().is_none());}
    let position=cursor.position();let owned=control.owned_bytes();live.set(false);assert_eq!(cursor.step(1,&mut control).unwrap_err().kind(),ValueRefusalKind::Canceled);assert_eq!(cursor.position(),position);assert_eq!(control.owned_bytes(),owned);live.set(true);
    let value=loop{if let Some(value)=cursor.step(1,&mut control).unwrap(){break value}};
    assert_eq!(serde_json::Value::from(&value),serde_json::from_str::<serde_json::Value>(text).unwrap());
    assert_eq!(cursor.source_ref().owner.get(cursor.source_ref().start).unwrap()as *const u8,pointer);assert_eq!(cursor.source_ref().length,text.len());assert_eq!(control.maximum_bytes(),262144);
    let mut close=cursor.into_retirement();while !close.terminal_is_empty(){close.close_step(1,131072).unwrap();}
    assert_eq!(*source.get(0).unwrap(),b'!');assert_eq!(*source.get(source.len()-1).unwrap(),b'?');
    eprintln!("[DEBUG] retained JSON cursor owns the exact immutable bounded view, preserving original pointer and source sentinels across cancellation without flattening");
}
