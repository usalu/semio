#[test]
fn retained_snapshot_patch_reader_moves_original_cells_and_retains_refused_candidate(){
    use kernel::operation_bytes::{OwnedOperationBytes,OperationByteOutput,OperationByteComparison,OperationByteCloseStep};
    use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,ValueRefusalKind};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📦️operation-source.json")).unwrap();
    let read_fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📦️operation-read.json")).unwrap();
    let allocation=fixture["allocationBytes"].as_u64().unwrap()as usize;let cleanup=read_fixture["maximumCleanupBytes"].as_u64().unwrap()as usize;
    let close_source=|source:&mut OwnedOperationBytes|{for _ in 0..source.len()+128{match source.close_one(1,4096).unwrap(){OperationByteCloseStep::Complete=>break,OperationByteCloseStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1);assert!(released_bytes<=4096);}}}assert!(source.terminal_is_empty());};
    let mut cases=fixture["cases"].as_array().unwrap().iter().map(|case|case.as_str().unwrap().to_owned()).collect::<Vec<_>>();
    cases.push(serde_json::json!({"operation":"set","path":"/title","value":fixture["word"].as_str().unwrap().repeat(fixture["payloadBytes"].as_u64().unwrap()as usize)}).to_string());
    for text in &cases{
        let mut source=OwnedOperationBytes::try_new(text.len(),allocation).unwrap();let mut allowed=|_|true;let mut encoding=NativeEncodeControl::new(allocation,&mut allowed);source.write_bytes(text.as_bytes(),&mut encoding).unwrap();
        let span=kernel::codec::ByteSpan::from_source(&source);assert!(span.contiguous().is_none());let pointer=span.get(0).unwrap()as *const u8;
        let mut options=kernel::codec::PackDecodeOptions::default();options.limits.max_file_len=text.len()as u64;options.limits.max_total_alloc=allocation as u64;
        let mut reader=SnapshotPatchReadCursor::new(span,&options).unwrap();let live=std::cell::Cell::new(true);let mut allowed=|_|live.get();let mut decode=NativeDecodeControl::new(allocation,&mut allowed);
        assert!(!reader.step(0,&mut decode).unwrap());let before=reader.position();live.set(false);assert_eq!(reader.step(1,&mut decode).unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!(reader.position(),before);live.set(true);
        while !reader.step(1,&mut decode).unwrap(){}let paid=decode.owned_bytes();
        let value_pointer=match reader.candidate(){Some(DslValue::Object(fields))=>fields.iter().find_map(|(key,value)|if key=="value"{match value{DslValue::String(text)=>Some(text.as_ptr()),_=>None}}else{None}),_=>None};
        live.set(false);assert_eq!(reader.admit_patch(&mut decode).unwrap_err().kind,ValueRefusalKind::Canceled);assert!(reader.candidate().is_some());assert!(reader.take_patch().is_none());assert_eq!(decode.owned_bytes(),paid);live.set(true);
        reader.admit_patch(&mut decode).unwrap();assert_eq!(decode.owned_bytes(),paid);let patch=reader.take_patch().unwrap();assert!(reader.take_patch().is_none());assert_eq!(patch,SnapshotPatch::decode_op(text.as_bytes()).unwrap());
        if let Some(pointer)=value_pointer{let (SnapshotPatch::Set{value:DslValue::String(value),..}|SnapshotPatch::Insert{value:DslValue::String(value),..})= &patch else{panic!("original text value moved")};assert_eq!(value.as_ptr(),pointer);}
        assert_eq!(serde_json::from_str::<serde_json::Value>(&patch.print_op()).unwrap(),serde_json::from_str::<serde_json::Value>(text).unwrap());assert_eq!(span.get(0).unwrap()as *const u8,pointer);
        let mut close=reader.into_retirement();while !close.terminal_is_empty(){close.close_step(1,cleanup).unwrap();}
        let mut canonical=kernel::codec::PackEncodeOptions::default();canonical.limits.max_file_len=patch.encode_op().unwrap().len()as u64;let canonical_text=patch.encode_op().unwrap();let mut comparison=OperationByteComparison::new(kernel::codec::ByteSpan::from_slice(&canonical_text));let mut allowed=|_|true;let mut encoding=NativeEncodeControl::new(allocation,&mut allowed);patch.encode_op_into(&canonical,&mut comparison,&mut encoding).unwrap();comparison.finish().unwrap();
        let mut close=semio_framework_value::retirement::owned_retirement(patch);while !close.terminal_is_empty(){close.close_step(1,cleanup).unwrap();}close_source(&mut source);
    }
    for row in read_fixture["invalid"].as_array().unwrap(){
        let text=row["source"].as_str().unwrap();assert!(serde_json::from_str::<serde_json::Value>(text).is_ok());assert!(SnapshotPatch::decode_op(text.as_bytes()).is_err());
        let mut options=kernel::codec::PackDecodeOptions::default();options.limits.max_file_len=text.len()as u64;options.limits.max_total_alloc=allocation as u64;let mut reader=SnapshotPatchReadCursor::new(kernel::codec::ByteSpan::from_slice(text.as_bytes()),&options).unwrap();let mut allowed=|_|true;let mut decode=NativeDecodeControl::new(allocation,&mut allowed);while !reader.step(1,&mut decode).unwrap(){}let paid=decode.owned_bytes();
        assert_eq!(reader.admit_patch(&mut decode).unwrap_err().kind,ValueRefusalKind::InvalidValue);assert!(reader.candidate().is_some());assert!(reader.take_patch().is_none());assert_eq!(decode.owned_bytes(),paid);
        assert_eq!(serde_json::Value::from(reader.candidate().unwrap()),serde_json::from_str::<serde_json::Value>(text).unwrap());let mut close=reader.into_retirement();while !close.terminal_is_empty(){close.close_step(1,cleanup).unwrap();}
    }
    eprintln!("[DEBUG] original six-kind Patch reader retains paged source, cancellation/refused candidate and semantic pointer through typed admission;8194 value moves once with no paid mirror");
}
