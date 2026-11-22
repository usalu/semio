//! 🧪️ The real receiving slot and borrowed producer agree with the independent frame corpus.
use super::*;
use semio_framework::kernel::{Event,OwnedCorePollLimits};
use semio_framework_job::{Generation,OperationId};
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
use semio_framework_value::RetainedCloneGrant;
use semio_plugin_abi::original_open_frame::OpenView;
use serde_json::{Value,json};

fn clock()->Option<u64>{Some(1)}

#[test]
fn compact_instance_open_has_exact_semantics_and_paid_retirement() {
    super::tests::with_input_decode_stack(check_open);
}

fn check_open(){
    let law:Value=serde_json::from_str(include_str!("__LAW__")).unwrap();
    for row in law["cases"].as_array().unwrap(){
        let mut event=row["event"].clone();
        normalize(&mut event["request"]["instanceId"]);
        normalize(&mut event["request"]["requestSequence"]);
        for capability in event["capabilities"].as_array_mut().unwrap(){
            if let Some(expiry)=capability.get_mut("expiresMs"){normalize(expiry);}
        }
        for quota in event["quotas"].as_object_mut().unwrap().values_mut(){normalize(quota);}
        let mut limits=row["limits"].clone();
        for value in limits.as_array_mut().unwrap(){normalize(value);}
        let input:Event=serde_json::from_value(json!({"instanceOpen":event})).unwrap();
        let request:OwnedCorePollLimits=serde_json::from_value(limits.clone()).unwrap();
        let source=OpenView::select(&input,request).unwrap();
        let wire=hex(row["hex"].as_str().unwrap());
        let mut encoded=Vec::new();
        for segment in 0..source.segments().unwrap(){
            encoded.extend(source.segment(segment,wire.len()).unwrap().bytes());
        }
        assert_eq!(encoded,wire);
        let semantic=json!({"Poll":{"events":[{"instanceOpen":event}],"command_page":null,"cold_pair_page":null,"limits":limits}});
        receive(&wire,Some(&semantic),false);
        receive(&wire,Some(&semantic),true);
    }
    let cases=law["cases"].as_array().unwrap();
    for row in law["refusals"].as_array().unwrap(){
        if row["index"]==0{continue}
        let mut wire=hex(cases[row["case"].as_u64().unwrap()as usize]["hex"].as_str().unwrap());
        wire[row["index"].as_u64().unwrap()as usize]=row["replacement"].as_u64().unwrap()as u8;
        receive(&wire,None,false);
    }
}

fn normalize(value:&mut Value){*value=Value::from(value.as_str().unwrap().parse::<u64>().unwrap());}
fn hex(value:&str)->Vec<u8>{(0..value.len()).step_by(2).map(|offset|u8::from_str_radix(&value[offset..offset+2],16).unwrap()).collect()}

fn receive(wire:&[u8], semantic:Option<&Value>, early_cancel:bool){
    let stock=RetainedCloneGrant{maximum_items:1_000_000,maximum_copy_bytes:32_000_000,maximum_capacity_bytes:32_000_000,maximum_release_bytes:32_000_000,maximum_depth:64};
    let slice=RetainedCloneGrant{maximum_items:100,maximum_copy_bytes:16_384,..stock};
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(953),Generation(1),stock,clock,1_000_000,1_000_000);
    caller.retain_dispatch().unwrap();
    caller.admit_return_table();
    caller.audit_heap();
    let mut receiver=caller.turn(stock,|cx|{
        assert!(charge(cx,false,size_of::<OriginalActorInputReceiving>())?);
        Ok(OriginalActorInputReceiving::new_bound(
            OriginalNativeDecodePolicy{maximum_owned_bytes:stock.maximum_capacity_bytes},
            JsonReadLimits{maximum_bytes:stock.maximum_copy_bytes as u64,maximum_allocation_bytes:stock.maximum_capacity_bytes,maximum_depth:64,maximum_items:stock.maximum_items as u64},
            wire,cx,
        ))
    }).unwrap();
    let substitute=wire.to_vec();
    assert!(caller.turn(slice,|cx|receiver.advance_in_turn(&substitute,cx)).is_err());
    let denied=RetainedCloneGrant{maximum_copy_bytes:0,..slice};
    assert_eq!(caller.turn(denied,|cx|receiver.advance(Some(wire),cx)).unwrap(),InputAdmission::Pending);
    assert!(receiver.input().is_none());
    let mut state=InputAdmission::Pending;
    for _ in 0..128{
        state=caller.turn(slice,|cx|receiver.advance(Some(wire),cx)).unwrap();
        if state!=InputAdmission::Pending || early_cancel&&receiver.poll_event_pending(){break}
    }
    if early_cancel{
        assert_eq!(state,InputAdmission::Pending);
        assert!(receiver.poll_event_pending()&&!receiver.input_returned());
    }else if let Some(semantic)=semantic{
        assert_eq!(state,InputAdmission::Ready);
        assert_eq!(serde_json::to_value(receiver.input().unwrap()).unwrap(),*semantic);
        assert!(receiver.input_returned());
        assert!(caller.turn(stock,|cx|receiver.prepare_poll_event(cx)).unwrap());
    }else{
        assert_eq!(state,InputAdmission::Failed);
        assert_eq!(receiver.failure_kind(),Some(ValueRefusalKind::InvalidValue));
        assert!(!receiver.input_returned());
    }
    caller.cancel().cancel_now();
    for _ in 0..512{
        if receiver.terminal_is_empty(){break}
        let held=(!receiver.input_returned()).then_some(wire);
        caller.turn(stock,|cx|cx.with_original_retirement(|cx|{
            if receiver.begin_close(held,cx)?{receiver.advance(held,cx)?;}
            Ok(())
        })).unwrap();
    }
    assert!(receiver.input_returned()&&receiver.terminal_is_empty());
    caller.retire(stock);
}
