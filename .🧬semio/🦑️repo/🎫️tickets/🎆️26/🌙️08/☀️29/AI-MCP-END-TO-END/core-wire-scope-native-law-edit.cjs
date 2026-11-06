const fs=require('fs'),path='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🧪️tests/🤝️admission/🦀️.rs',before=fs.readFileSync(path,'utf8');if(before.includes('fn original_core_wire_scope_withholds_real_reserve'))throw Error('law exists');
const law=`
#[test]
fn original_core_wire_scope_withholds_real_reserve(){
 use semio_framework_job_testing_caller::OriginalJobFixtureCaller;use semio_framework_job::{OperationId,Generation};use semio_framework_job_return_escrow_publication::OriginalEscrowPublisher;
 let demand=OriginalEscrowPublisher::core_wire_reserve(144).unwrap();let stock=RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:1048576,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:64};let mut caller=OriginalJobFixtureCaller::admit(OperationId(8860),Generation(10),stock,||Some(1),10000,10000);let mut called=false;
 let denied=RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..stock};caller.turn(denied,|cx|{let before=cx.retained_progress();assert!(guest_context::with_core_wire_reserve(cx,144,|_|{called=true;Ok(())}).is_err());assert_eq!(cx.retained_progress(),before);Ok(())}).unwrap();assert!(!called);
 caller.turn(stock,|cx|{let before=cx.retained_progress();let grant=cx.retained_grant();let fuel=cx.fuel_remaining();guest_context::with_core_wire_reserve(cx,144,|inner|{called=true;assert_eq!(inner.retained_grant().maximum_items,grant.maximum_items-demand.items);assert_eq!(inner.retained_grant().maximum_copy_bytes,grant.maximum_copy_bytes-demand.copy_bytes);assert!(inner.fuel_remaining()<=fuel-demand.work);Ok(())})?;assert_eq!(cx.retained_progress(),before);Ok(())}).unwrap();assert!(called);caller.retire(stock);
}
`;
const tmp=path+'.guest-core-scope-law';fs.writeFileSync(tmp,before+law,{flag:'wx'});if(fs.readFileSync(path,'utf8')!==before){fs.unlinkSync(tmp);throw Error('concurrent test change')}fs.renameSync(tmp,path);
