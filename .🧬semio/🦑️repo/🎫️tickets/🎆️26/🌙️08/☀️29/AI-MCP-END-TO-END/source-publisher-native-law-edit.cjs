const fs=require('fs'),path='🧰️framework/🔨️modules/🧵️job/⏱️context/📦️return/🎟️place/🧪️tests/🦀️.rs',before=fs.readFileSync(path,'utf8');if(before.includes('fn sized_return_core_wire_publisher_is_source_owned'))throw Error('law already exists');
const law=`
#[test]
fn sized_return_core_wire_publisher_is_source_owned(){
 let stock=RetainedCloneGrant{maximum_items:64,maximum_copy_bytes:1048576,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:8};let q=RetainedCloneGrant{maximum_items:1,..stock};let mut caller=OriginalJobFixtureCaller::admit(OperationId(8830),Generation(8),stock,clock,10000,128);let mut ready=false;for _ in 0..32{caller.turn(q,|cx|{cx.bind_original_source_epoch(1)?;if !cx.admit_original_core_wire_delivery()?{return Ok(())}ready=cx.admit_original_escrow_publisher()?;Ok(())}).unwrap();if ready{break}}assert!(ready);caller.turn(q,|cx|{cx.bind_original_source_epoch(2)?;let before=cx.retained_progress();assert!(cx.admit_original_escrow_publisher()?);assert_eq!(cx.retained_progress(),before);Ok(())}).unwrap();caller.retire(stock);
}
`;
const tmp=path+'.guest-publisher-law';fs.writeFileSync(tmp,before+law,{flag:'wx'});if(fs.readFileSync(path,'utf8')!==before){fs.unlinkSync(tmp);throw Error('concurrent source change')}fs.renameSync(tmp,path);
