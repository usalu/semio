const fs=require('fs'),f='🧰️framework/🔨️modules/🧵️job/⏱️context/📦️return/🎟️place/🧪️tests/🦀️.rs';const before=fs.readFileSync(f,'utf8');const test=`
#[test]
fn sized_return_cleanup_can_reserve_after_cancellation_on_the_original_retirement_loan(){
 let stock=RetainedCloneGrant{maximum_items:64,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:8};let q=RetainedCloneGrant{maximum_items:1,..stock};let mut caller=OriginalJobFixtureCaller::admit(OperationId(8810),Generation(5),stock,clock,10000,128);let mut place=OriginalCallerReturnPlace::empty();let mut destination=[0u8;48];let address=destination.as_mut_ptr() as usize;caller.cancel().cancel_now();let mut admitted=false;
 for _ in 0..24{caller.turn(q,|cx|cx.with_original_retirement(|cx|{admitted=place.admit_sized(cx,address,destination.len())?;Ok(())})).unwrap();if admitted{break}}assert!(admitted);
 place.record_sized_return(address,destination.len()).unwrap();let mut copied=0;for _ in 0..8{if place.terminal_is_empty(){break}caller.turn(q,|cx|cx.with_original_retirement(|cx|{let before=cx.retained_progress();place.settle_return(cx)?;copied+=cx.retained_progress().copied_bytes-before.copied_bytes;Ok(())})).unwrap();}assert!(place.terminal_is_empty());assert_eq!(copied,destination.len());caller.retire(stock);
}
`;
if(before.includes('fn sized_return_cleanup_can_reserve'))throw Error('exists');if(fs.readFileSync(f,'utf8')!==before)throw Error('changed');let t=f+'.guest-cancel-law.tmp';fs.writeFileSync(t,before+test);if(fs.readFileSync(f,'utf8')!==before){fs.unlinkSync(t);throw Error('changed before rename')}fs.renameSync(t,f);
