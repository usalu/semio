
#[test]
fn paged_pdf14_original_source_preserves_seven_frames_raw_words_and_fixed_page_grants(){
    use protocol::{OpBinary,operation_bytes::{OwnedOperationBytes,OperationByteMeasurement,OperationBytePreparation,OperationByteCloseStep}};
    use semio_framework_value::{NativeEncodeControl,ValueRefusalKind};
    use crate::standards::v1_4::subsets::base::schema::snapshot::PageDoc;
    let neutral:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🫳️operation-source/🔣️.json")).unwrap();
    let index=|case:&serde_json::Value,key:&str|usize::try_from(case[key].as_u64().unwrap()).unwrap();
    let word=|case:&serde_json::Value,key:&str|f64::from_bits(case[key].as_str().unwrap().parse().unwrap());
    let mut cases=Vec::new();
    for case in neutral["cases"].as_array().unwrap(){
        let operation=match case["kind"].as_str().unwrap(){
            "insert-page"=>PdfMutation::InsertPage(InsertPage{index:index(case,"index"),page:PageDoc{width:word(case,"widthBits"),height:word(case,"heightBits"),text:case["text"].as_str().unwrap().into()}}),
            "remove-page"=>PdfMutation::RemovePage(RemovePage{index:index(case,"index")}),
            "move-page"=>PdfMutation::MovePage(MovePage{from:index(case,"from"),to:index(case,"to")}),
            "resize-page"=>PdfMutation::ResizePage(ResizePage{index:index(case,"index"),width:word(case,"widthBits"),height:word(case,"heightBits")}),
            "replace-page-text"=>PdfMutation::ReplacePageText(ReplacePageText{index:index(case,"index"),text:case["text"].as_str().unwrap().into()}),
            "set-snapshot"=>PdfMutation::SetSnapshot(SetSnapshot{snapshot:PdfSnapshot{schema:case["identity"].as_str().unwrap().into(),pages:case["pages"].as_array().unwrap().iter().map(|page|PageDoc{width:word(page,"widthBits"),height:word(page,"heightBits"),text:page["text"].as_str().unwrap().into()}).collect()}}),
            "patch-snapshot"=>PdfMutation::PatchSnapshot(PatchSnapshot{patch:semio_s_artifact_stdio_contract::editing::SnapshotPatch::Remove{path:case["patch"]["path"].as_str().unwrap().into()}}),
            kind=>panic!("undeclared literal kind {kind}"),
        };
        let hex=case["frameHex"].as_str().unwrap();let literal=(0..hex.len()).step_by(2).map(|index|u8::from_str_radix(&hex[index..index+2],16).unwrap()).collect::<Vec<_>>();
        assert_eq!(literal[1],case["tag"].as_u64().unwrap()as u8);assert_eq!(operation.encode_op().unwrap(),literal);cases.push((operation,literal));
    }
    assert_eq!(cases.len(),binary::REGISTRY.len());
    let large=PdfMutation::ReplacePageText(ReplacePageText{index:0,text:"x".repeat(neutral["payloadBytes"].as_u64().unwrap()as usize)});let literal=large.encode_op().unwrap();cases.push((large,literal));
    let allocation=neutral["allocationBytes"].as_u64().unwrap()as usize;let items=neutral["maximumCloseItems"].as_u64().unwrap()as usize;let bytes=neutral["maximumCloseBytes"].as_u64().unwrap()as usize;
    let close=|owner:&mut OwnedOperationBytes|{let allocation=owner.allocated_bytes();let turns=owner.len()+256;let mut released=0;for _ in 0..turns{match owner.close_one(items,bytes).unwrap(){OperationByteCloseStep::Complete=>break,OperationByteCloseStep::Pending{released_items,released_bytes}=>{assert!(released_items<=items);assert!(released_bytes<=bytes);released+=released_bytes;}}}assert!(owner.terminal_is_empty());assert_eq!(owner.allocated_bytes(),0);assert_eq!(released,allocation);};
    let refusal=|error:protocol::ProtocolError|match error{protocol::ProtocolError::Pack(protocol::PackError::Refusal(refusal))=>refusal.kind(),other=>panic!("expected exact typed policy refusal: {other:?}")};
    for(operation,literal)in &cases{
        let mut options=protocol::codec::PackEncodeOptions::default();options.limits.max_file_len=literal.len()as u64;
        let mut measure=OperationByteMeasurement::new(options.limits.max_file_len);let mut accept=|_|true;let mut control=NativeEncodeControl::new(0,&mut accept);operation.encode_op_into(&options,&mut measure,&mut control).unwrap();assert_eq!(control.owned_bytes(),0);assert_eq!(measure.exact_length().unwrap(),literal.len());
        let mut preparation=OperationBytePreparation::try_new(literal.len(),allocation).unwrap();let mut accept=|_|true;let mut control=NativeEncodeControl::new(allocation,&mut accept);for _ in 0..literal.len()+256{preparation.fund_one(items,bytes,&mut control).unwrap();if preparation.is_funded(){break;}}assert!(preparation.is_funded());let charged=control.owned_bytes();let backing=preparation.allocated_bytes();operation.encode_op_into(&options,&mut preparation,&mut control).unwrap();assert_eq!(charged,control.owned_bytes());assert_eq!(backing,preparation.allocated_bytes());let mut owner=preparation.take_ready().unwrap();assert!(owner.iter().eq(literal.iter().copied()));assert_eq!(serde_json::to_value(&owner).unwrap(),serde_json::to_value(literal).unwrap());assert_eq!(owner.close_one(0,bytes).unwrap(),OperationByteCloseStep::Pending{released_items:0,released_bytes:0});assert_eq!(owner.len(),literal.len());close(&mut owner);
        options.limits.max_file_len-=1;let mut prefix=OwnedOperationBytes::try_new(literal.len(),allocation).unwrap();let mut accept=|_|true;let mut control=NativeEncodeControl::new(allocation,&mut accept);assert_eq!(refusal(operation.encode_op_into(&options,&mut prefix,&mut control).unwrap_err()),ValueRefusalKind::OwnershipLimit);assert!(prefix.len()<literal.len());assert!(prefix.iter().eq(literal[..prefix.len()].iter().copied()));close(&mut prefix);
    }
    let(operation,literal)=cases.last().unwrap();let options=protocol::codec::PackEncodeOptions::default();let mut prefix=OwnedOperationBytes::try_new(literal.len(),allocation).unwrap();let mut cancel=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|progress.completed<neutral["cancelAt"].as_u64().unwrap()as usize;let mut control=NativeEncodeControl::new(allocation,&mut cancel);assert_eq!(refusal(operation.encode_op_into(&options,&mut prefix,&mut control).unwrap_err()),ValueRefusalKind::Canceled);assert!(prefix.iter().eq(literal[..prefix.len()].iter().copied()));close(&mut prefix);
    for key in ["max_items","max_depth"]{let mut options=protocol::codec::PackEncodeOptions::default();if key=="max_items"{options.limits.max_items=0;}else{options.limits.max_depth=0;}let mut measure=OperationByteMeasurement::new(options.limits.max_file_len);let mut accept=|_|true;let mut control=NativeEncodeControl::new(0,&mut accept);let kind=refusal(cases[5].0.encode_op_into(&options,&mut measure,&mut control).unwrap_err());assert_eq!(kind,if key=="max_items"{ValueRefusalKind::WorkLimit}else{ValueRefusalKind::DepthLimit});}
    let mut options=protocol::codec::PackEncodeOptions::default();options.limits.max_total_alloc=0;let mut owner=OwnedOperationBytes::try_new(literal.len(),allocation).unwrap();let mut accept=|_|true;let mut control=NativeEncodeControl::new(allocation,&mut accept);assert_eq!(refusal(operation.encode_op_into(&options,&mut owner,&mut control).unwrap_err()),ValueRefusalKind::OwnershipLimit);assert_eq!(owner.len(),0);assert_eq!(control.owned_bytes(),0);assert_eq!(control.maximum_bytes(),allocation);close(&mut owner);
    let invalid=PdfMutation::InsertPage(InsertPage{index:0,page:PageDoc{width:f64::NAN,height:1.0,text:String::new()}});assert!(invalid.encode_op().is_err());let options=protocol::codec::PackEncodeOptions::default();let mut measure=OperationByteMeasurement::new(options.limits.max_file_len);let mut accept=|_|true;let mut control=NativeEncodeControl::new(0,&mut accept);assert!(matches!(invalid.encode_op_into(&options,&mut measure,&mut control),Err(protocol::ProtocolError::Malformed{what:"PDF 1.4 mutation",..})));
    eprintln!("[DEBUG] PDF1.4 seven neutral literal frames, NaN payload/signed-zero words, actual8194 text, same-control caller ceilings and exact physical4096 page retirement");
}
