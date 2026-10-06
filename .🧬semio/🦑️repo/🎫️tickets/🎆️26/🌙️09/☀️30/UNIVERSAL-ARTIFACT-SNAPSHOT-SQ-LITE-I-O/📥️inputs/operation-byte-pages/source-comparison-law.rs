
#[test]
fn owned_operation_byte_pages_canonical_comparison_retains_exact_source_and_first_difference_without_allocation(){
    use super::bytes::{OperationByteComparison,OperationByteOutput};
    use crate::codec::ByteSpan;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let expected:Vec<u8>=fixture["source"]["prefixUtf8"].as_str().unwrap().bytes().chain(std::iter::repeat_n(32,8192)).collect();
    let mut source=OwnedOperationBytes::try_new(expected.len(),65536).unwrap();
    let mut allow=|_|true;
    let mut control=crate::value::NativeEncodeControl::new(65536,&mut allow);
    source.write_bytes(&expected,&mut control).unwrap();
    let retained=source.allocated_bytes();
    for span in [ByteSpan::from_slice(&expected),ByteSpan::from_source(&source)]{
        let mut allow=|_|true;
        let mut control=crate::value::NativeEncodeControl::new(65536,&mut allow);
        let(result,requested,released)=crate::test_allocation::observe_backing(||{
            let mut comparison=OperationByteComparison::new(span);
            comparison.write_bytes(&expected[..4095],&mut control)?;
            comparison.write_bytes(&expected[4095..],&mut control)?;
            comparison.finish()
        });
        result.unwrap();
        assert_eq!((requested,released),(0,0));
        let mut allow=|_|true;
        let mut control=crate::value::NativeEncodeControl::new(65536,&mut allow);
        let mut comparison=OperationByteComparison::new(span);
        comparison.write_bytes(&expected[..4095],&mut control).unwrap();
        let(error,requested,released)=crate::test_allocation::observe_backing(||comparison.write_bytes(&[255],&mut control));
        let crate::PackRefusal::RetainedMalformed{kind,offset,..}=error.unwrap_err()else{panic!("exact typed mismatch lost")};
        assert_eq!(kind,crate::value::ValueRefusalKind::InvalidValue);
        assert_eq!(offset,4095);
        assert_eq!(comparison.position(),4095);
        assert_eq!((requested,released),(0,0));
        assert!(comparison.write_bytes(&expected[4095..],&mut control).is_err());
        assert!(comparison.finish().is_err());
        let mut comparison=OperationByteComparison::new(span);
        comparison.write_bytes(&expected[..expected.len()-1],&mut control).unwrap();
        assert!(comparison.finish().is_err());
        let mut comparison=OperationByteComparison::new(span);
        comparison.write_bytes(&expected,&mut control).unwrap();
        assert!(comparison.write_bytes(&[0],&mut control).is_err());
        assert!(comparison.finish().is_err());
        let mut canceled=|progress:crate::value::native_encoding::NativeEncodeProgress|progress.completed<256;
        let mut control=crate::value::NativeEncodeControl::new(65536,&mut canceled);
        let mut comparison=OperationByteComparison::new(span);
        let refusal=comparison.write_bytes(&expected,&mut control).unwrap_err();
        assert_eq!(refusal.kind(),crate::value::ValueRefusalKind::Canceled);
        assert_eq!(comparison.position(),256);
    }
    assert_eq!(source.allocated_bytes(),retained);
    assert_eq!(serde_json::to_value(&source).unwrap(),serde_json::to_value(&expected).unwrap());
    for _ in 0..fixture["maximumCloseSteps"].as_u64().unwrap(){if source.close_one(1,4096).unwrap()==OperationByteCloseStep::Complete{break;}}
    assert!(source.terminal_is_empty());
    println!("[DEBUG] Exact whole 8194 source comparison has zero backing allocation, keeps first mismatch and cancellation position, and retains source until fixed 4096 terminal retirement");
}
