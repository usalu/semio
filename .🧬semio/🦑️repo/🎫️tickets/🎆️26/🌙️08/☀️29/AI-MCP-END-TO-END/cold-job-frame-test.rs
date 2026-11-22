
#[test]
fn compact_job_source_matches_serde_and_retires_its_partial_and_complete_owners() {
    with_input_decode_stack(check_compact_job_source);
}

fn check_compact_job_source() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("cold-job.json")).unwrap();
    let cases=fixture["cases"].as_array().unwrap();
    for row in cases {
        let wire=compact_hex(row["hex"].as_str().unwrap());
        let mut source=row["source"].clone();
        source["job"]=source["job"].as_str().unwrap().parse::<u64>().unwrap().into();
        check_compact_job_case(&wire,Some(source),false);
    }
    check_compact_job_case(b"SJS1",None,false);
    check_compact_job_case(br#"{"Job":{"job":19,"kind":"snapshot","input":null,"checkpoint":null}}"#,None,false);
    for row in fixture["malformed"].as_array().unwrap() {
        let mut wire=compact_hex(cases[row["case"].as_u64().unwrap() as usize]["hex"].as_str().unwrap());
        wire[row["index"].as_u64().unwrap() as usize]=row["value"].as_u64().unwrap() as u8;
        check_compact_job_case(&wire,None,false);
    }

    let large=&fixture["largeInput"];
    let pattern:Vec<u8>=serde_json::from_value(large["pattern"].clone()).unwrap();
    let payload=pattern.repeat(large["repeat"].as_u64().unwrap() as usize);
    let job=large["job"].as_u64().unwrap();
    let kind=large["kind"].as_str().unwrap();
    let mut wire=b"SJS1\x01\x00\x00\x00".to_vec();
    for word in [job,kind.len() as u64,payload.len() as u64,0] { wire.extend(word.to_le_bytes()); }
    wire.extend(kind.as_bytes());
    wire.extend(&payload);
    let input=serde_json::json!({"job":job,"kind":kind,"input":payload,"checkpoint":null});
    check_compact_job_case(&wire,Some(input.clone()),false);
    check_compact_job_case(&wire,Some(input),true);
}

fn compact_hex(hex:&str)->Vec<u8> {
    (0..hex.len()).step_by(2).map(|index|u8::from_str_radix(&hex[index..index+2],16).unwrap()).collect()
}

fn check_compact_job_case(wire:&[u8], expected:Option<serde_json::Value>, cancel_partial:bool) {
    let receiver_bytes=size_of::<OriginalActorInputReceiving>();
    let json=wire.starts_with(b"{");
    let stock=if json{
        RetainedCloneGrant{maximum_items:1_000_000,maximum_copy_bytes:32_000_000,maximum_capacity_bytes:32_000_000,maximum_release_bytes:32_000_000,maximum_depth:128}
    }else{
        RetainedCloneGrant{maximum_items:20_000,maximum_copy_bytes:(1<<20)+receiver_bytes,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64}
    };
    let admission=RetainedCloneGrant{maximum_copy_bytes:receiver_bytes+16_384,..stock};
    let slice=if json{
        RetainedCloneGrant{maximum_items:256,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64}
    }else{
        RetainedCloneGrant{maximum_items:100,maximum_copy_bytes:16_384,..stock}
    };
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(952),Generation(2),stock,clock,5_000_000,100_000);
    caller.retain_dispatch().unwrap();
    caller.admit_return_table();
    let mut receiver=funded(&mut caller,admission,|cx|{
        assert!(charge(cx,false,receiver_bytes)?);
        Ok(OriginalActorInputReceiving::new_bound(
            OriginalNativeDecodePolicy{maximum_owned_bytes:stock.maximum_capacity_bytes},
            JsonReadLimits{maximum_bytes:stock.maximum_copy_bytes as u64,maximum_allocation_bytes:stock.maximum_capacity_bytes,maximum_depth:64,maximum_items:stock.maximum_items as u64},
            wire,cx,
        ))
    });
    let substitute=wire.to_vec();
    assert!(caller.turn(slice,|cx|receiver.advance_in_turn(&substitute,cx)).is_err());
    let mut opened=0;
    let mut closed=0;
    let mut items=0;
    let wanted=usize::from(expected.is_some()&&!cancel_partial);
    let mut reads=0;
    let mut final_state=InputAdmission::Pending;
    for _ in 0..100 {
        final_state=funded(&mut caller,slice,|cx|receiver.advance_in_turn(wire,cx));
        let receipt=caller.receipt();
        opened+=receipt.retained_capacity_bytes;
        closed+=receipt.released_bytes;
        items+=receipt.copied_items;
        if final_state!=InputAdmission::Pending { break; }
        if let Some(ActorInput::Job(job))=receiver.input.as_ref() {
            reads=job.input.as_ref().map_or(0,Vec::len);
            if cancel_partial&&reads>0 { break; }
        }
    }
    assert_eq!(usize::from(final_state==InputAdmission::Ready),wanted);
    if cancel_partial {
        assert!(matches!(final_state,InputAdmission::Pending)&&reads>0&&reads<wire.len()-64);
    }else if let Some(expected)=expected {
        let job=expected["job"].as_u64().unwrap();
        let expected:ActorInput=serde_json::from_value(serde_json::json!({"Job":expected})).unwrap();
        assert_eq!(serde_json::to_value(receiver.input().unwrap()).unwrap(),serde_json::to_value(expected).unwrap());
        if wire.len()>16_384 { assert!(items<1_000,"retained item count grew with the payload octets: {items}"); }
        let denied=RetainedCloneGrant{maximum_copy_bytes:0,..stock};
        assert!(!funded(&mut caller,denied,|cx|receiver.prepare_job(job,cx)));
        assert!(funded(&mut caller,stock,|cx|receiver.prepare_job(job,cx)));
        let original=receiver.job_slot(job).unwrap().as_ref().unwrap().kind.as_ptr();
        assert!(funded(&mut caller,stock,|cx|receiver.prepare_job(job,cx)));
        assert_eq!(receiver.job_slot(job).unwrap().as_ref().unwrap().kind.as_ptr(),original);
    }else {
        assert_eq!(final_state,InputAdmission::Failed);
        assert_eq!(receiver.failure_kind(),Some(ValueRefusalKind::InvalidValue));
        assert!(!receiver.input_returned());
    }
    caller.cancel().cancel_now();
    for _ in 0..(if json{1024}else{100}) {
        if receiver.terminal_is_empty(){break;}
        let held=(!receiver.input_returned()).then_some(wire);
        funded(&mut caller,stock,|cx|cx.with_original_retirement(|cx|{
            if receiver.begin_close(held,cx)?{receiver.advance(held,cx)?;}
            Ok(())
        }));
        closed+=caller.receipt().released_bytes;
        opened+=caller.receipt().retained_capacity_bytes;
    }
    assert!(receiver.input_returned()&&receiver.terminal_is_empty());
    assert_eq!(opened,closed);
    caller.retire(stock);
}
