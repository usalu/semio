#[test]
fn native_capacity_closed_vectors_preserve_the_consumed_admission_owner(){
    use crate::{NativeEncodeControl,ValueRefusalKind};
    fn number(value:&serde_json::Value)->usize{match value.as_str().unwrap(){"usizeMax"=>usize::MAX,"isizeMax"=>isize::MAX as usize,decimal=>decimal.parse().unwrap()}}
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let cases=fixture["capacityAdmission"]["cases"].as_array().unwrap();assert_eq!(cases.len(),12);
    for case in cases {
        let calls=std::cell::Cell::new(0);let event=std::cell::Cell::new(None);
        let mut callback=|value|{calls.set(calls.get()+1);event.set(Some(value));true};
        let mut control=NativeEncodeControl::new(13,&mut callback);control.begin_stage(9).unwrap();control.charge(number(&case["ownedBytes"])).unwrap();control.advance(4).unwrap();control.checkpoint().unwrap();
        let before=event.get().unwrap();let before_calls=calls.get();
        let result=control.admit_capacity(number(&case["sourceBytes"]),number(&case["multiples"]),number(&case["scaffoldBytes"]));
        assert_eq!(calls.get(),before_calls,"{}",case["id"]);
        let mut control=if case["accepted"].as_bool().unwrap(){let control=match result{Ok(control)=>control,Err(_)=>panic!("capacity unexpectedly refused: {}",case["id"])};assert_eq!(control.maximum_bytes(),number(&case["maximumBytes"]));control}else{let(control,error)=match result{Err(refusal)=>refusal,Ok(_)=>panic!("capacity unexpectedly admitted: {}",case["id"])};assert_eq!(error.kind,ValueRefusalKind::OwnershipLimit);assert_eq!(control.maximum_bytes(),13);control};
        assert_eq!(control.owned_bytes(),before.owned_bytes);control.checkpoint().unwrap();assert_eq!(event.get().unwrap(),before);control.advance(5).unwrap();let complete=event.get().unwrap();assert_eq!(complete.completed,9);assert_eq!(complete.total,9);assert_eq!(complete.owned_bytes,before.owned_bytes);
        let receipt=control.pause().unwrap();let canceled_event=std::cell::Cell::new(None);let mut canceled=|value|{canceled_event.set(Some(value));false};let mut control=NativeEncodeControl::resume(receipt,&mut canceled).unwrap();assert_eq!(control.step().unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!(canceled_event.get().unwrap(),complete);assert_eq!(control.owned_bytes(),before.owned_bytes);
    }
    eprintln!("[DEBUG] native source capacity twelve closed vectors preserve owner/counters/callback; checked overflow, signed ceiling, refusal and resumed cancellation");
}
