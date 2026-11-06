const fs=require('fs');const f='🧰️framework/🔨️modules/🧵️job/⏱️context/📦️return/🎟️place/🧪️tests/🦀️.rs',s=fs.readFileSync(f,'utf8');const t=`
#[test]
fn sized_preissue_requires_paid_metadata_and_retains_capture(){
 let stock=RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:10000000,maximum_capacity_bytes:10000000,maximum_release_bytes:10000000,maximum_depth:16};
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(7817),Generation(2),stock,clock,100000,100000);
 let mut place=OriginalCallerReturnPlace::empty();let mut extra=0;let mut ready=false;let mut cell=0u64;let destination=&mut cell as *mut u64 as usize;let extent=std::mem::size_of::<u64>();
 for _ in 0..64{caller.turn(stock,|cx|{ready=place.admit_sized(cx,destination,extent)?;Ok(())}).unwrap();extra+=caller.receipt().retained_capacity_bytes;if ready{break}}assert!(ready);assert!(!place.capture_is_preissued());
 let(items,bytes,work)=OriginalCallerReturnPlace::preissue_demand();let low=RetainedCloneGrant{maximum_items:items-1,..stock};
 caller.turn(low,|cx|{let before=cx.retained_progress();assert!(!place.preissue_capture(cx,destination,extent)?);assert_eq!(cx.retained_progress(),before);assert!(!place.capture_is_preissued());Ok(())}).unwrap();
 caller.turn(stock,|cx|{let before=cx.retained_progress();let start=cx.original_work_units_spent();assert!(place.preissue_capture(cx,destination,extent)?);let after=cx.retained_progress();assert_eq!(after.copied_items-before.copied_items,items);assert_eq!(after.copied_bytes-before.copied_bytes,bytes);assert_eq!(cx.original_work_units_spent()-start,u128::from(work));assert!(place.capture_is_preissued());Ok(())}).unwrap();
 cell=7;assert_eq!(cell,7);for _ in 0..64{if place.terminal_is_empty(){break}caller.turn(stock,|cx|{place.settle_return(cx)?;Ok(())}).unwrap()}assert!(place.terminal_is_empty());retire_source(caller,stock,extra);
}
`;if(s.includes('fn sized_preissue_requires_paid_metadata'))throw Error('duplicate');const p=f+'.preissue-law';fs.writeFileSync(p,s+t,{flag:'wx'});if(fs.readFileSync(f,'utf8')!==s){fs.unlinkSync(p);throw Error('concurrent change')}fs.renameSync(p,f);