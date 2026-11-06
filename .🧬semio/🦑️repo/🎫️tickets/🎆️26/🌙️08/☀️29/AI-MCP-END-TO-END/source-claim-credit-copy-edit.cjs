const fs=require('fs');function edit(f,changes){const before=fs.readFileSync(f,'utf8');let s=before;for(const[a,b,n=1]of changes){if(s.split(a).length-1!==n)throw Error(a);s=s.split(a).join(b)}if(fs.readFileSync(f,'utf8')!==before)throw Error('changed');let t=f+'.guest-credit-copy.tmp';fs.writeFileSync(t,s);if(fs.readFileSync(f,'utf8')!==before){fs.unlinkSync(t);throw Error('changed before rename')}fs.renameSync(t,f)}
edit('🧰️framework/🔨️modules/🧵️job/📬️outcome/⏸️continuation/🎟️source/🦀️.rs',[
 ['std::mem::size_of::<OriginalReturnEscrowRow>()','original_return_claim_copy_bytes()',2],
 ['pub struct OriginalReturnBodyIdentity{','pub fn original_return_claim_copy_bytes()->usize{std::mem::size_of::<OriginalReturnEscrowRow>()+std::mem::size_of::<OriginalReturnClaimCredit>()}\npub struct OriginalReturnBodyIdentity{']
]);
edit('🧰️framework/🔨️modules/🧵️job/⏱️context/📦️return/🎟️place/🧪️tests/🦀️.rs',[["std::mem::size_of::<semio_framework_job_payload::OriginalReturnEscrowRow>()+2*metadata","semio_framework_job_context::testkit::original_return_source::original_return_claim_copy_bytes()+2*metadata"]]);
