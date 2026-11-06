const fs=require('fs'),path='🧰️framework/🔨️modules/🧵️job/⏱️context/📦️return/🎟️place/🧪️tests/🦀️.rs',before=fs.readFileSync(path,'utf8');if(before.includes('fn sized_return_publisher_failed_post_is_fenced'))throw Error('law already exists');
const law=`
#[test]
fn sized_return_publisher_failed_post_is_fenced(){
 let stock=RetainedCloneGrant{maximum_items:64,maximum_copy_bytes:1048576,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:8};let q=RetainedCloneGrant{maximum_items:1,..stock};let mut caller=OriginalJobFixtureCaller::admit(OperationId(8840),Generation(9),stock,clock,10000,128);let mut source_extra=0;let mut table=false;
 for _ in 0..32{caller.turn(q,|cx|{cx.bind_original_source_epoch(1)?;if !cx.admit_original_core_wire_delivery()?{return Ok(())}table=cx.admit_original_return_table()?;Ok(())}).unwrap();source_extra+=caller.receipt().retained_capacity_bytes;if table{break}}assert!(table);
 let failure=caller.turn(q,|cx|{cx.bind_original_source_epoch(2)?;let maximum=testkit::retained_grant_mut(cx).maximum_items;testkit::set_loan_receipt_refusal(true);let result=cx.admit_original_escrow_publisher();testkit::retained_grant_mut(cx).maximum_items=maximum;let failure=result.expect_err("injected publisher receipt refusal");assert_eq!(failure.retained_progress().copied_items,1);let before=cx.retained_progress();assert!(cx.admit_original_escrow_publisher().is_err());assert_eq!(cx.retained_progress(),before);Ok(failure)}).unwrap();source_extra+=caller.receipt().retained_capacity_bytes;assert!(failure.primary_value().is_some());assert!(failure.retained_progress().retained_capacity_bytes>0);retire_source(caller,stock,source_extra);
}
`;
const tmp=path+'.guest-publisher-failure-law';fs.writeFileSync(tmp,before+law,{flag:'wx'});if(fs.readFileSync(path,'utf8')!==before){fs.unlinkSync(tmp);throw Error('concurrent source change')}fs.renameSync(tmp,path);
