use super::*;
use semio_framework_pack_error::{PackTransportCategory,PackTransportPhase,TransportContextRefusalCause};

#[test]
fn standalone_policy_language_neutral_cases(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap(){
        let args:Vec<String>=case["args"].as_array().unwrap().iter().map(|value|value.as_str().unwrap().to_owned()).collect();
        match parse(&args){
            Ok(Invocation::Help)=>assert_eq!(case["result"],"help","{}",case["name"]),
            Ok(Invocation::Run{arguments,options})=>{assert_eq!(case["result"],"run","{}",case["name"]);assert_eq!(u64::from(options.credit),case["credit"].as_u64().unwrap());assert_eq!(options.input_credit(),case["inputCredit"].as_u64().unwrap());assert_eq!(u64::from(options.timeout_ms),case["timeoutMs"].as_u64().unwrap());assert_eq!(serde_json::to_value(arguments).unwrap(),case["command"]);},
            Err(refusal)=>{assert_eq!(case["result"],"refusal","{}",case["name"]);assert_eq!(format!("{refusal:?}"),case["refusal"].as_str().unwrap());},
        }
    }
    println!("[DEBUG] standalone policy literal cases={}",fixture["cases"].as_array().unwrap().len());
}

#[test]
fn standalone_policy_preserves_shared_cancellation_and_admission(){
    let options=TransportOptions{credit:1048576,input_credit:1048576,timeout_ms:30000};
    let context=options.create_context(CancelToken::root_now()).ok().unwrap();let alias=context.clone();
    context.cancel();assert!(matches!(alias.checkpoint(PackTransportPhase::BeforeOperation,PackTransportCategory::NativeIo,1),Err(semio_framework_pack_error::PackError::Refusal(semio_framework_pack_error::PackRefusal::TransportAdmission{refusal,..}))if refusal.kind==ValueRefusalKind::Canceled));
    let cancellation=CancelToken::root_now();let context=options.create_context(cancellation.clone()).ok().unwrap();cancellation.cancel_now();
    assert!(matches!(context.checkpoint(PackTransportPhase::BeforeOperation,PackTransportCategory::NativeIo,1),Err(semio_framework_pack_error::PackError::Refusal(semio_framework_pack_error::PackRefusal::ValueRefusal(error)))if error.kind==ValueRefusalKind::Canceled));
    let refusal=TransportOptions{credit:0,input_credit:1048576,timeout_ms:30000}.create_context(CancelToken::root_now()).err().unwrap();
    assert!(matches!(refusal.cause,TransportContextRefusalCause::Admission(refusal)if refusal.kind==ValueRefusalKind::OwnershipLimit));
    println!("[DEBUG] standalone shared cancellation and zero-credit refusal observed");
}

#[test]
fn standalone_deadline_refuses_exact_expiration(){
    let policy=TransportDeadline{started:Instant::now(),timeout:Duration::from_millis(10),cancellation:CancelToken::root_now()};
    assert!(policy.check_elapsed(Duration::from_millis(9)).is_ok());
    assert_eq!(policy.check_elapsed(Duration::from_millis(10)).unwrap_err().kind,ValueRefusalKind::Canceled);
    assert_eq!(policy.check_elapsed(Duration::from_millis(11)).unwrap_err().kind,ValueRefusalKind::Canceled);
    println!("[DEBUG] standalone finite deadline before/at/after observed");
}
