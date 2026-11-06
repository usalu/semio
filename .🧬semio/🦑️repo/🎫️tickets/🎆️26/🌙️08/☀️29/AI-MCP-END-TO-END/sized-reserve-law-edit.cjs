const fs=require('fs'),f='🧰️framework/🔨️modules/🧵️job/⏱️context/📦️return/🎟️place/🧪️tests/🦀️.rs',s=fs.readFileSync(f,'utf8');const add=`
#[test]
fn core_wire_sized_claim_reserve_checks_actual_target_and_overflow(){
 let reserve=OriginalCallerReturnPlace::core_wire_sized_claim_reserve(8).expect("actual target fits the portable CoreWire ceiling");assert_eq!((reserve.items,reserve.immediate_copy_bytes,reserve.lifetime_copy_bytes,reserve.work,reserve.depth),(3,512,520,3,1));assert!(OriginalCallerReturnPlace::core_wire_sized_claim_reserve(usize::MAX).is_err());
}
`;if(s.includes('fn core_wire_sized_claim_reserve_checks_actual_target'))throw Error('duplicate');let p=f+'.sized-reserve-law';fs.writeFileSync(p,s+add,{flag:'wx'});if(fs.readFileSync(f,'utf8')!==s){fs.unlinkSync(p);throw Error('race')}fs.renameSync(p,f);