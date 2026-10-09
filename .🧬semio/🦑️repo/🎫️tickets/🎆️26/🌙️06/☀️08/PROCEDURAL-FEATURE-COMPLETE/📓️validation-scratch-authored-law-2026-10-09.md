# Original Validation Scratch Law

Authored full-grant law before the direct-drop baseline. Runtime qualification pending.

```rust
#[test]
fn original_validation_scratch_retains_all_physical_buffers_until_full_grants() {
    use crate::brep::queries::validation::BodyValidationJob;
    use semio_framework_value::{retained_clone::RetainedCloneGrant,retirement::controlled::ControlledRetirement};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎟️ownership/🔣️.json")).unwrap();let laws=&fixture["validationScratch"];
    let mut body=Body::new();build_unit_box(&mut body,&mut OpRecorder::new());
    let (mut job,constructor)=observe_tessellation_system(||BodyValidationJob::new(&body));assert_eq!(constructor,(laws["constructorCapacityBytes"].as_u64().unwrap() as usize,laws["constructorReleaseBytes"].as_u64().unwrap() as usize));
    let (_,source)=observe_tessellation_system(||job.step(&body,laws["warmTurns"].as_u64().unwrap() as usize));assert!(source.0-source.1>=laws["minimumRetainedBytes"].as_u64().unwrap() as usize);
    let (mut owner,handoff)=observe_tessellation_system(||ControlledRetirement::new(job).unwrap_or_else(|_|panic!("original validation scratch requires typed retirement")));assert_eq!(handoff,(laws["handoffCapacityBytes"].as_u64().unwrap() as usize,laws["handoffReleaseBytes"].as_u64().unwrap() as usize));
    let (mut born,mut freed,mut refusals,mut turns)=(0,0,0,0);
    while !owner.terminal_is_empty() {
        let copy=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
        if grant.maximum_release_bytes>8 {for _ in 0..2 {let (step,heap)=observe_tessellation_system(||owner.step(RetainedCloneGrant {maximum_release_bytes:8,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(owner.next_release_byte_demand().unwrap(),grant.maximum_release_bytes);refusals+=1;}}
        let (step,heap)=observe_tessellation_system(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;freed+=heap.1;turns+=1;assert!(turns<1000000);
    }
    assert!(refusals>0);assert_eq!(source.0-source.1+born,freed);eprintln!("[DEBUG] Original validation scratch source={} admittedBirth={born} physicalRelease={freed} repeatedEightByteRefusals={refusals} turns={turns}",source.0-source.1);
}

```
