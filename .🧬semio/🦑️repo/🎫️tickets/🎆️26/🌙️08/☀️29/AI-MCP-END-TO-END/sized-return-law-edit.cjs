const fs=require('fs');const f='🧰️framework/🔨️modules/🧵️job/⏱️context/📦️return/🎟️place/🧪️tests/🦀️.rs';const before=fs.readFileSync(f,'utf8');const test=`
#[test]
fn sized_return_receipt_charges_only_the_original_captured_destination(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../sized-case.json")).unwrap();
 for (index,row) in law["cases"].as_array().unwrap().iter().enumerate(){
  let stock=RetainedCloneGrant{maximum_items:64,maximum_copy_bytes:law["grantBytes"].as_u64().unwrap() as usize,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:8};let q=RetainedCloneGrant{maximum_items:1,..stock};
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(8800+index as u64),Generation(4),stock,clock,10000,128);let mut place=OriginalCallerReturnPlace::empty();let mut destination=[0u8;48];let address=destination.as_mut_ptr() as usize;let requested=row["requested"].as_u64().unwrap() as usize;
  let mut admitted=false;for _ in 0..24{caller.turn(q,|cx|{admitted=place.admit_sized(cx,address,requested)?;Ok(())}).unwrap();if admitted{break}}
  assert_eq!(admitted,row["admitted"].as_bool().unwrap());
  if admitted{
   assert!(place.record_sized_return(address+1,requested).is_err());
   if row["captured"].as_bool().unwrap(){place.record_sized_return(address,requested).unwrap();}
   let mut copied=0;for _ in 0..8{if place.terminal_is_empty(){break}caller.turn(q,|cx|{let before=cx.retained_progress();place.settle_return(cx)?;copied+=cx.retained_progress().copied_bytes-before.copied_bytes;Ok(())}).unwrap();}
   assert!(place.terminal_is_empty());assert_eq!(copied,row["spent"].as_u64().unwrap() as usize);
  }
  caller.retire(stock);
 }
}
`;
if(before.includes('fn sized_return_receipt_charges_only'))throw Error('exists');if(fs.readFileSync(f,'utf8')!==before)throw Error('changed');let t=f+'.guest-law.tmp';fs.writeFileSync(t,before+test);if(fs.readFileSync(f,'utf8')!==before){fs.unlinkSync(t);throw Error('changed before rename')}fs.renameSync(t,f);
