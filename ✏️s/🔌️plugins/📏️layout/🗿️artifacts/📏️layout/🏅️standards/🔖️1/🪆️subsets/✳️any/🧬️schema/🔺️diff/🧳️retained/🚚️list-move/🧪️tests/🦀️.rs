use super::*;
use semio_framework_value_clone_source_testing::drive::drive_reply;
use semio_os_kernel::retained_conclude;
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn reserve(owner:&mut TypedListMove<String,String>,held:usize,inserted:usize,output:usize,grant:RetainedCloneGrant)->RetainedCloneStep{
    drive_reply(grant,1,|result|result.as_ref().map_or_else(ValueError::retained_progress,|step:&RetainedCloneStep|step.progress()),
        |effect,reply,recover|{
            assert!(!recover);
            Ok(reply.complete(owner.reserve_effect(held,inserted,output,effect)))
        },
    ).result.unwrap()
}
fn close(owner:&mut TypedListMove<String,String>){
    owner.begin_close();
    for _ in 0..100_000{
        if owner.terminal_is_empty(){return}
        let demand=owner.close_demand(4096).unwrap();
        let units=owner.close_work_demand(4096).unwrap();
        let mut close=|grant|{
            drive_reply(grant,units,|result|result.as_ref().map_or_else(semio_framework_value::OriginalRetirementFailure::retained_progress,|step:&RetainedCloneStep|step.progress()),
                |effect,reply,recover|{
                    let base=effect.performed_work();
                    let mut work=0;
                    let result=owner.close_step(None,4096,effect,recover,&mut work);
                    retained_conclude(effect,base,result,work,|step|step.progress(),reply)
                },
            ).result.unwrap()
        };
        assert_eq!(close(Default::default()).progress(),Default::default());
        close(grant(demand));
    }
    panic!("typed original close stalled")
}
#[test]
fn original_payload_allocations_survive_numeric_list_commit(){let corpus:serde_json::Value=serde_json::from_str(include_str!("../../📋️keyed-plan/🧪️fixtures/🔣️.json")).unwrap();for row in corpus["cases"].as_array().unwrap(){if row.get("error").is_some(){continue}let held:Vec<String>=row["held"].as_array().unwrap().iter().map(|id|id.as_str().unwrap().to_owned()).collect();let pointers:Vec<*const u8>=held.iter().map(|id|id.as_ptr()).collect();let inserted:Vec<String>=row["delta"]["inserted"].as_array().unwrap().iter().map(|row|row["id"].as_str().unwrap().to_owned()).collect();let origins:Vec<Option<Origin>>=row["origins"].as_array().unwrap().iter().map(|row|Some(Origin{inserted:row[0]=="inserted",index:row[1].as_u64().unwrap() as usize})).collect();let(mut post,mut authored)=(held,Some(inserted));let mut owner=TypedListMove::<String,String>::new();let d=owner.reserve_demand(post.len(),authored.as_ref().unwrap().len(),origins.len()).unwrap();assert_eq!(reserve(&mut owner,post.len(),authored.as_ref().unwrap().len(),origins.len(),Default::default()).progress(),Default::default());reserve(&mut owner,post.len(),authored.as_ref().unwrap().len(),origins.len(),grant(d));let d=owner.commit_demand().unwrap();assert_eq!(owner.capture_owned(&mut post,&mut authored,Default::default()).unwrap().progress(),Default::default());assert_eq!(post.len(),pointers.len());owner.capture_owned(&mut post,&mut authored,grant(d)).unwrap();for turn in 0..100000{let d=owner.commit_demand().unwrap();assert_eq!(owner.advance_owned(&mut post,&origins,Default::default()).unwrap().progress(),Default::default());if matches!(owner.advance_owned(&mut post,&origins,grant(d)).unwrap(),RetainedCloneStep::Complete(_)){break}assert!(turn<99999)}for(at,origin)in post.iter().zip(&origins){let source=origin.unwrap();if !source.inserted{assert_eq!(at.as_ptr(),pointers[source.index])}}close(&mut owner);}}
#[test]
fn every_partial_original_owner_closes_under_funded_retirement(){for cut in 0..12{let mut owner=TypedListMove::<String,String>::new();let(mut post,mut authored)=(vec!["first original payload".to_owned(),"second original payload".to_owned()],Some(vec!["new payload".to_owned()]));let origins=[Some(Origin{inserted:false,index:1}),Some(Origin{inserted:true,index:0})];let d=owner.reserve_demand(2,1,2).unwrap();reserve(&mut owner,2,1,2,grant(d));if cut>0{let d=owner.commit_demand().unwrap();owner.capture_owned(&mut post,&mut authored,grant(d)).unwrap();for _ in 1..cut{let d=owner.commit_demand().unwrap();if matches!(owner.advance_owned(&mut post,&origins,grant(d)).unwrap(),RetainedCloneStep::Complete(_)){break}}}close(&mut owner);}}
