
#[test]
fn owned_operation_byte_pages_fund_complete_backing_across_hops_before_actual_emission(){
    use super::operation_bytes::{OperationBytePreparation,OperationByteFundingStep,OperationByteOutput};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let expected:Vec<u8>=fixture["source"]["prefixUtf8"].as_str().unwrap().bytes().chain(std::iter::repeat_n(32,8192)).collect();
    let maximum=fixture["maximumBytes"].as_u64().unwrap()as usize;
    for canceled in [false,true]{
        let mut prepared=OperationBytePreparation::try_new(expected.len(),65536).unwrap();
        let mut allow=|_|true;let control=crate::value::NativeEncodeControl::new(65536,&mut allow);
        let mut receipt=control.pause().unwrap();
        for _ in 0..fixture["maximumCloseSteps"].as_u64().unwrap(){
            let mut allow=|_|true;let mut control=crate::value::NativeEncodeControl::resume(receipt,&mut allow).unwrap();
            let (zero,requested,released)=crate::test_allocation::observe_backing(||prepared.fund_one(1,0,&mut control));
            assert_eq!(zero.unwrap(),OperationByteFundingStep::Pending{prepared_items:0,allocated_bytes:0});assert_eq!((requested,released),(0,0));
            let required=prepared.next_funding_byte_demand().unwrap();
            let (narrow,requested,released)=crate::test_allocation::observe_backing(||prepared.fund_one(1,required.saturating_sub(1),&mut control));
            assert_eq!(narrow.unwrap(),OperationByteFundingStep::Pending{prepared_items:0,allocated_bytes:0});assert_eq!((requested,released),(0,0));
            let (step,requested,released)=crate::test_allocation::observe_backing(||prepared.fund_one(1,maximum,&mut control));
            assert_eq!(released,0);assert!(requested<=maximum);
            match step.unwrap(){OperationByteFundingStep::Pending{prepared_items,allocated_bytes}=>{assert!(prepared_items<=1);assert_eq!(allocated_bytes,requested);},OperationByteFundingStep::Ready=>{assert_eq!(requested,0);}}
            assert_eq!(prepared.accepted_prefix().unwrap().len(),0);
            assert_eq!(control.owned_bytes(),prepared.allocated_bytes());
            receipt=control.pause().unwrap();
            if prepared.is_funded(){break;}
        }
        assert!(prepared.is_funded());
        let retained=prepared.allocated_bytes();
        let mut callback=|progress:crate::value::native_encoding::NativeEncodeProgress|!canceled||progress.completed<256;
        let mut control=crate::value::NativeEncodeControl::resume(receipt,&mut callback).unwrap();
        if canceled{
            assert_eq!(prepared.write_bytes(&expected,&mut control).unwrap_err().kind(),crate::value::ValueRefusalKind::Canceled);
            let prefix=prepared.accepted_prefix().unwrap();assert_eq!(prefix.len(),256);assert!(prefix.iter().eq(expected[..256].iter().copied()));
            assert!(prepared.take_ready().is_none());
            for _ in 0..fixture["maximumCloseSteps"].as_u64().unwrap(){if prepared.close_one(1,maximum).unwrap()==OperationByteCloseStep::Complete{break;}}
            assert!(prepared.terminal_is_empty());assert_eq!(prepared.allocated_bytes(),0);
        }else{
            let (emitted,requested,released)=crate::test_allocation::observe_backing(||prepared.write_bytes(&expected,&mut control));
            emitted.unwrap();assert_eq!((requested,released),(0,0));assert_eq!(prepared.allocated_bytes(),retained);
            assert!(prepared.accepted_prefix().unwrap().iter().eq(expected.iter().copied()));
            let mut source=prepared.take_ready().unwrap();assert!(prepared.terminal_is_empty());
            assert_eq!(source.allocated_bytes(),retained);assert_eq!(serde_json::to_value(&source).unwrap(),serde_json::to_value(&expected).unwrap());
            for _ in 0..fixture["maximumCloseSteps"].as_u64().unwrap(){if source.close_one(1,maximum).unwrap()==OperationByteCloseStep::Complete{break;}}
            assert!(source.terminal_is_empty());assert_eq!(source.allocated_bytes(),0);
        }
    }
    println!("[DEBUG] Same8194 original octets use step-funded actual backing under4096 and same consuming Native admission receipt; actual emission allocates no backing, cancellation retains only genuine256 prefix and all unpaid-to-release ownership");
}
