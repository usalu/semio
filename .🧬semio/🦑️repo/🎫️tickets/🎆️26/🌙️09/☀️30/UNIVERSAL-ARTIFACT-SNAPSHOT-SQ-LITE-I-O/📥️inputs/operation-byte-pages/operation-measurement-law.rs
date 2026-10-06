
#[test]
fn owned_operation_byte_pages_measure_complete_operation_without_retaining_payload_or_resetting_control(){
    use super::operation_bytes::{OperationByteMeasurement,OperationByteOutput};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let expected:Vec<u8>=fixture["source"]["prefixUtf8"].as_str().unwrap().bytes().chain(std::iter::repeat_n(32,8192)).collect();
    for maximum in [expected.len(),expected.len()-1]{
        let mut measured=OperationByteMeasurement::new(maximum as u64);
        let mut allow=|_|true;let mut control=crate::value::NativeEncodeControl::new(65536,&mut allow);
        let (result,requested,released)=crate::test_allocation::observe_backing(||{
            measured.write_bytes(&expected[..2],&mut control)?;
            measured.write_bytes(&expected[2..],&mut control)
        });
        assert_eq!((requested,released),(0,0));assert_eq!(control.owned_bytes(),0);
        if maximum==expected.len(){result.unwrap();assert_eq!(measured.exact_length().unwrap(),expected.len());}else{
            assert_eq!(result.unwrap_err().kind(),crate::value::ValueRefusalKind::OwnershipLimit);
            assert!(measured.exact_length().is_err());assert!(measured.write_bytes(&[],&mut control).is_err());
        }
    }
    let mut measured=OperationByteMeasurement::new(expected.len()as u64);
    let mut allow=|_|true;let mut control=crate::value::NativeEncodeControl::new(65536,&mut allow);
    measured.write_bytes(&expected[..2],&mut control).unwrap();
    let receipt=control.pause().unwrap();
    let mut cancel=|progress:crate::value::native_encoding::NativeEncodeProgress|progress.completed==0;
    let mut control=crate::value::NativeEncodeControl::resume(receipt,&mut cancel).unwrap();
    assert_eq!(measured.write_bytes(&expected[2..],&mut control).unwrap_err().kind(),crate::value::ValueRefusalKind::Canceled);
    assert_eq!(control.owned_bytes(),0);assert!(measured.exact_length().is_err());
    assert_eq!(serde_json::to_value(&expected).unwrap().as_array().unwrap().len(),fixture["expected"]["payloadBytes"].as_u64().unwrap()as usize);
    println!("[DEBUG] Genuine operation measurement preserves complete8194 whole-frame ceiling and same Native control without any payload backing; finite refusal/cancellation latch prevents a false funding length");
}
