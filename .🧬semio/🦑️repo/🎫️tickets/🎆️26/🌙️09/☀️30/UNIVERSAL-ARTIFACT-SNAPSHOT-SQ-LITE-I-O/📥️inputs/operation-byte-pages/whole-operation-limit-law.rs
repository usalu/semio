
#[test]
fn owned_operation_byte_pages_complete_operation_limit_counts_header_and_returns_refused_prefix(){
    use super::operation_bytes::{OperationByteLimitedOutput,OperationByteOutput};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let expected:Vec<u8>=fixture["source"]["prefixUtf8"].as_str().unwrap().bytes().chain(std::iter::repeat_n(32,8192)).collect();
    let close=|source:&mut OwnedOperationBytes|{for _ in 0..fixture["maximumCloseSteps"].as_u64().unwrap(){if source.close_one(1,4096).unwrap()==OperationByteCloseStep::Complete{break;}}assert!(source.terminal_is_empty());assert_eq!(source.allocated_bytes(),0);};
    for maximum in [expected.len(),expected.len()-1]{
        let mut source=OwnedOperationBytes::try_new(expected.len(),65536).unwrap();
        let mut allow=|_|true;
        let mut control=crate::value::NativeEncodeControl::new(65536,&mut allow);
        let completed={
            let mut limited=OperationByteLimitedOutput::new(&mut source,maximum as u64);
            limited.write_bytes(&expected[..2],&mut control).unwrap();
            let result=limited.write_bytes(&expected[2..],&mut control);
            if maximum==expected.len(){result.unwrap();assert_eq!(limited.completed_bytes(),expected.len()as u64);}else{
                let crate::PackRefusal::RetainedMalformed{kind,offset,..}=result.unwrap_err()else{panic!("whole operation typed ceiling lost")};
                assert_eq!(kind,crate::value::ValueRefusalKind::OwnershipLimit);
                assert_eq!(offset,2);
                assert!(limited.write_bytes(&[],&mut control).is_err());
                assert_eq!(limited.completed_bytes(),2);
            }
            limited.completed_bytes()as usize
        };
        assert!(source.iter().eq(expected[..completed].iter().copied()));
        assert_eq!(serde_json::to_value(&source).unwrap(),serde_json::to_value(&expected[..completed]).unwrap());
        close(&mut source);
    }
    let mut source=OwnedOperationBytes::try_new(expected.len(),65536).unwrap();
    let mut allow=|_|true;
    let mut control=crate::value::NativeEncodeControl::new(65536,&mut allow);
    {
        let mut limited=OperationByteLimitedOutput::new(&mut source,expected.len()as u64);
        limited.write_bytes(&expected[..2],&mut control).unwrap();
        let mut cancel=|progress:crate::value::native_encoding::NativeEncodeProgress|progress.completed<32;
        let mut control=crate::value::NativeEncodeControl::new(65536,&mut cancel);
        assert_eq!(limited.write_bytes(&expected[2..],&mut control).unwrap_err().kind(),crate::value::ValueRefusalKind::Canceled);
        assert_eq!(limited.completed_bytes(),2);
        assert!(limited.write_bytes(&[],&mut control).is_err());
    }
    assert_eq!(source.len(),34);
    assert!(source.iter().eq(expected[..34].iter().copied()));
    close(&mut source);
    println!("[DEBUG] Complete 8194 operation limit includes its header, one-byte-short refusal preserves the accepted whole fragment prefix, and inner cancellation retains its exact actual source for fixed 4096 terminal release");
}
