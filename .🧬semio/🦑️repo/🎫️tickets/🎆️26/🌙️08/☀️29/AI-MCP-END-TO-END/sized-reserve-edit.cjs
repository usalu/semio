const fs=require('fs'),a='🧰️framework/🔨️modules/🧵️job/⏱️context/📦️return/🎟️place/🦀️.rs',b='🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs';let s=fs.readFileSync(a,'utf8');const anchor='pub struct OriginalCallerReturnPlace{',v=`/// 📏️ Portable upper bound for one already-table-bound sized return claim.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalSizedClaimReserve{pub items:usize,pub immediate_copy_bytes:usize,pub lifetime_copy_bytes:usize,pub work:u64,pub depth:usize}
`;if(s.split(anchor).length!==2)throw Error('type anchor');let t=s.replace(anchor,v+anchor);const method='impl OriginalCallerReturnPlace{',insert=`impl OriginalCallerReturnPlace{
 /// 🧮️ See \`README.md\`.
 pub fn sized_claim_reserve(extent:usize)->Result<OriginalSizedClaimReserve,ValueError>{
  use semio_framework_job_payload::OriginalReturnEscrowAccounting as A;
  let actual=crate::original_job_continuation::source::original_return_claim_copy_bytes().checked_add(std::mem::size_of::<Option<OriginalSourceBinding>>()).and_then(|v|v.checked_add(std::mem::size_of::<Option<(usize,usize)>>())).and_then(|v|v.checked_add(2*A::UPDATE+A::FINAL)).ok_or_else(||ValueError::invariant("sized claim reserve overflow"))?;
  if actual>512{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"sized claim target exceeds portable metadata ceiling"))}
  Ok(OriginalSizedClaimReserve{items:3,immediate_copy_bytes:512,lifetime_copy_bytes:extent.checked_add(512).ok_or_else(||ValueError::invariant("sized claim reserve overflow"))?,work:3,depth:1})
 }
`;if(t.split(method).length!==2)throw Error('method anchor');t=t.replace(method,insert);let p=a+'.sized-reserve';fs.writeFileSync(p,t,{flag:'wx'});if(fs.readFileSync(a,'utf8')!==s){fs.unlinkSync(p);throw Error('race place')}fs.renameSync(p,a);s=fs.readFileSync(b,'utf8');const x='pub use original_return_place::OriginalCallerReturnPlace;',y='pub use original_return_place::{OriginalCallerReturnPlace,OriginalSizedClaimReserve};';if(s.split(x).length!==2)throw Error('export anchor');p=b+'.sized-reserve';fs.writeFileSync(p,s.replace(x,y),{flag:'wx'});if(fs.readFileSync(b,'utf8')!==s){fs.unlinkSync(p);throw Error('race context')}fs.renameSync(p,b);