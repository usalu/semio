import{readFileSync,writeFileSync}from"node:fs";
const core="🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/♻️original-close/🦀️.rs",ret="🧰️framework/🔨️modules/🌱️value/♻️retirement/🧬️contract/🦀️.rs";
const declarations=` fn failure_storage_birth(&mut self)->&mut OriginalFailureStorageBirth;
 fn prepare_failure_storage_work(&mut self,work:u64)->OriginalFailureStorageStep<bool,ValueError>;
 fn commit_failure_storage_work(&mut self,work:u64)->Result<(),ValueError>;
 fn cancel_failure_storage_work(&mut self)->Result<(),ValueError>;
 fn finish_failure_storage_work(&mut self)->Result<(),ValueError>;
`;
const loanMethods=` pub fn failure_storage_birth(&mut self)->&mut OriginalFailureStorageBirth{self.port.failure_storage_birth()}
 pub fn prepare_failure_storage_work(&mut self,work:u64)->OriginalFailureStorageStep<bool,ValueError>{self.port.prepare_failure_storage_work(work)}
 pub fn commit_failure_storage_work(&mut self,work:u64)->Result<(),ValueError>{self.port.commit_failure_storage_work(work)}
 pub fn cancel_failure_storage_work(&mut self)->Result<(),ValueError>{self.port.cancel_failure_storage_work()}
 pub fn finish_failure_storage_work(&mut self)->Result<(),ValueError>{self.port.finish_failure_storage_work()}
`;
const controlLoanMethods=` fn failure_storage_birth(&mut self)->&mut OriginalFailureStorageBirth{self.failure_storage_birth()}
 fn prepare_failure_storage_work(&mut self,work:u64)->OriginalFailureStorageStep<bool,ValueError>{self.prepare_failure_storage_work(work)}
 fn commit_failure_storage_work(&mut self,work:u64)->Result<(),ValueError>{self.commit_failure_storage_work(work)}
 fn cancel_failure_storage_work(&mut self)->Result<(),ValueError>{self.cancel_failure_storage_work()}
 fn finish_failure_storage_work(&mut self)->Result<(),ValueError>{self.finish_failure_storage_work()}
`;
let s=readFileSync(core,"utf8");s=s.replace("use crate::{ValueError,","use crate::{OriginalFailureStorageBirth,OriginalFailureStorageStep,ValueError,");
s=s.replace("pub trait OriginalRetirementClosePort{","pub trait OriginalRetirementClosePort{\n"+declarations);
s=s.replace(" pub fn quote_failure_storage(",""+loanMethods+" pub fn quote_failure_storage(");
s=s.split("\n").map(l=>/fn (quote_failure_storage|post_failure_storage|cancel_failure_storage)\(/.test(l)?l.replaceAll("Result<RetainedCloneGrant,NativeAllocationRefusal>","OriginalFailureStorageStep<RetainedCloneGrant>").replaceAll("Result<(),NativeAllocationRefusal>","OriginalFailureStorageStep<()>"):l).join("\n");writeFileSync(core,s);
s=readFileSync(ret,"utf8");s=s.replace("pub use semio_framework_value_control_contract::{NativeAllocationRefusal,","pub use semio_framework_value_control_contract::{OriginalFailureStorageBirth,OriginalFailureStorageBirthPhase,OriginalFailureStorageStep,OriginalFailureFunding,refill_original_failure_storage,NativeAllocationRefusal,");
s=s.replace("pub trait OriginalRetirementControl {","pub trait OriginalRetirementControl {\n"+declarations);
s=s.replace("impl OriginalRetirementControl for semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>{","impl OriginalRetirementControl for semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>{\n"+controlLoanMethods);
s=s.split("\n").map(l=>/fn (quote_failure_storage|post_failure_storage|cancel_failure_storage)\(/.test(l)?l.replaceAll("Result<RetainedCloneGrant,NativeAllocationRefusal>","OriginalFailureStorageStep<RetainedCloneGrant>").replaceAll("Result<(),NativeAllocationRefusal>","OriginalFailureStorageStep<()>"):l).join("\n");
const a=s.indexOf("pub trait OriginalFailureFunding{"),b=s.indexOf("pub trait ErasedSnapshotRetirement",a);if(a<0||b<a)throw Error("old funding section");
const implementation=`impl OriginalFailureFunding for dyn OriginalRetirementControl+'_{
 fn birth(&mut self)->&mut OriginalFailureStorageBirth{self.failure_storage_birth()}
 fn quote(&mut self,d:RetirementDemand,g:RetainedCloneGrant)->OriginalFailureStorageStep<RetainedCloneGrant>{self.quote_failure_storage(d,g)}
 fn post(&mut self,g:RetainedCloneGrant,p:RetainedCloneProgress)->OriginalFailureStorageStep<()>{self.post_failure_storage(g,p)}
 fn cancel(&mut self)->OriginalFailureStorageStep<()>{self.cancel_failure_storage()}
 fn pool(&mut self)->&mut semio_framework_value_control_contract::OriginalFailureStoragePool{self.failure_storage_pool()}
 fn prepare(&mut self,work:u64)->OriginalFailureStorageStep<bool,ValueError>{self.prepare_failure_storage_work(work)}
 fn commit(&mut self,work:u64)->Result<(),ValueError>{self.commit_failure_storage_work(work)}
 fn cancel_work(&mut self)->Result<(),ValueError>{self.cancel_failure_storage_work()}
 fn finish_work(&mut self)->Result<(),ValueError>{self.finish_failure_storage_work()}
}
`;s=s.slice(0,a)+implementation+s.slice(b);writeFileSync(ret,s);
const birth="🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/♻️failure/👶️birth/🦀️.rs";s=readFileSync(birth,"utf8");const idx=s.indexOf("/// 🧾️ Advances");if(idx<0)throw Error("driver header");
const coreimpl=`impl OriginalFailureFunding for crate::OriginalRetirementCloseLoan<'_>{
 fn birth(&mut self)->&mut OriginalFailureStorageBirth{self.failure_storage_birth()}
 fn quote(&mut self,d:RetirementDemand,g:RetainedCloneGrant)->OriginalFailureStorageStep<RetainedCloneGrant>{self.quote_failure_storage(d,g)}
 fn post(&mut self,g:RetainedCloneGrant,p:RetainedCloneProgress)->OriginalFailureStorageStep<()>{self.post_failure_storage(g,p)}
 fn cancel(&mut self)->OriginalFailureStorageStep<()>{self.cancel_failure_storage()}
 fn pool(&mut self)->&mut OriginalFailureStoragePool{self.failure_storage_pool()}
 fn prepare(&mut self,work:u64)->OriginalFailureStorageStep<bool,ValueError>{self.prepare_failure_storage_work(work)}
 fn commit(&mut self,work:u64)->Result<(),ValueError>{self.commit_failure_storage_work(work)}
 fn cancel_work(&mut self)->Result<(),ValueError>{self.cancel_failure_storage_work()}
 fn finish_work(&mut self)->Result<(),ValueError>{self.finish_failure_storage_work()}
}
`;s=s.slice(0,idx)+coreimpl+s.slice(idx);
s=s.replace(" if funding.birth().phase==Phase::Empty{"," if funding.birth().phase!=Phase::Empty&&funding.birth().requested!=Some(grant){return RetainedWorkOutcome{result:Err(invalid(\"failure birth requires its original requested grant\").into()),work:0}}\n if funding.birth().phase==Phase::Empty{");writeFileSync(birth,s);
