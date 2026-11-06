import fs from"node:fs";const ticket=import.meta.dir,p="🧰️framework/🔨️modules/🧵️job/📬️outcome/⏸️continuation/🎟️source/🦀️.rs",owner="🧰️framework/🔨️modules/🧵️job/📬️outcome/🪙️budget/🦀️.rs";const old=fs.readFileSync(p,"utf8"),base=fs.readFileSync(owner,"utf8");
const traitStart=old.indexOf("trait OriginalReturnBudgetAuthority{"),implStart=old.indexOf("impl OriginalReturnBudgetAuthority for OriginalReturnBudget{",traitStart),bodyStart=old.indexOf(" fn claim(",implStart),phaseStart=old.indexOf("pub use semio_framework_job_return_budget::OriginalReturnPhase;",bodyStart);
if([traitStart,implStart,bodyStart,phaseStart].some(n=>n<0))throw Error("authority seams");
let body=old.slice(bodyStart,phaseStart).trimEnd();if(!body.endsWith("}"))throw Error("budget impl end");body=body.slice(0,-1);
body=body.replace(" fn claim("," pub fn claim(").replace(" fn redeem("," pub fn redeem(").replaceAll("OriginalReturnClaimCredit","OriginalReturnClaimCredit<I>").replaceAll("OriginalReturnWitness","OriginalReturnWitness<I>");
body=body.replace("OriginalReturnClaimCredit<I>{data:","OriginalReturnClaimCredit::<I>{_origin:std::marker::PhantomData,data:").replaceAll("semio_framework_job_return_budget::OriginalReturnClaimState","OriginalReturnClaimState");
body=body.replaceAll("original_return_claim_copy_bytes()","original_return_claim_copy_bytes::<I>()");
for(const field of["source","recipient"])body=body.replaceAll("origin."+field,"origin."+field+"_identity()");
body=body.replace('let origin=self.origin.ok_or_else(||refused("original return claim requires its admitted source"))?;', 'let origin=self.origin.ok_or_else(||refused("original return claim requires its admitted source"))?;if origin.recipient_identity()!=self as*const Self as usize{return Err(refused("original return claim requires its actual bound budget address"))}');
body=body.replace('let origin=self.origin.ok_or_else(||refused("original return source already closed"))?;', 'let origin=self.origin.ok_or_else(||refused("original return source already closed"))?;if origin.recipient_identity()!=self as*const Self as usize{return Err(refused("original return redemption requires its actual bound budget address"))}');
let newOwner=base.replace("OriginalReturnEscrowAccounting as M,","OriginalReturnEscrowAccounting as M,OriginalReturnEscrowRow,OriginalReturnEscrowIdentity,OriginalReturnEscrowAmounts,");
newOwner=newOwner.replace(" fn recipient(&self)->usize;"," fn recipient(&self)->usize;\n fn source_identity(&self)->usize;\n fn recipient_identity(&self)->usize{self.recipient()}");
newOwner+="\nimpl<I:OriginalReturnOrigin> OriginalReturnBudget<I>{\n"+body+"}\n";
newOwner+=`
/// 🎟️ Nominal origin typing keeps claimed authority distinct from freely constructible phase data.
#[repr(transparent)]
pub struct OriginalReturnClaimCredit<I:OriginalReturnOrigin>{data:OriginalReturnClaimState,_origin:std::marker::PhantomData<fn()->I>}
impl<I:OriginalReturnOrigin> std::ops::Deref for OriginalReturnClaimCredit<I>{type Target=OriginalReturnClaimState;fn deref(&self)->&Self::Target{&self.data}}
impl<I:OriginalReturnOrigin> OriginalReturnClaimCredit<I>{
 pub fn terminal_is_empty(&self)->bool{self.data.terminal_is_empty()}
 pub fn has_unused_capture(&self)->bool{self.data.has_unused_capture()}
 pub fn issue_return(&mut self,phase:OriginalReturnPhase,copy_bytes:usize)->Result<OriginalReturnWitness<I>,ValueError>{self.data.issue_return(phase,copy_bytes).map(|data|OriginalReturnWitness{data,_origin:std::marker::PhantomData})}
 pub fn cancel_preissued(&mut self,w:&mut OriginalReturnWitness<I>)->Result<(),ValueError>{self.data.cancel_preissued(&mut w.data)}
 pub fn issue_success_return(&mut self,copy_bytes:usize)->Result<OriginalReturnWitness<I>,ValueError>{self.data.issue_success_return(copy_bytes).map(|data|OriginalReturnWitness{data,_origin:std::marker::PhantomData})}
}
impl<I:OriginalReturnOrigin> Drop for OriginalReturnClaimCredit<I>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original return claim retains unspent original currencies");}}
/// 🧾️ Only a genuine typed claim can mint this held physical-effect witness.
#[repr(transparent)]
pub struct OriginalReturnWitness<I:OriginalReturnOrigin>{data:OriginalReturnWitnessState,_origin:std::marker::PhantomData<fn()->I>}
impl<I:OriginalReturnOrigin> std::ops::Deref for OriginalReturnWitness<I>{type Target=OriginalReturnWitnessState;fn deref(&self)->&Self::Target{&self.data}}
impl<I:OriginalReturnOrigin> OriginalReturnWitness<I>{
 pub fn is_capture(&self)->bool{self.phase==OriginalReturnPhase::Capture}
 pub fn releases_unused_capture(&self)->bool{self.release_capture}
 pub fn mark_received(&mut self)->Result<(),ValueError>{if !self.redeemed||self.received{return Err(refused("original held return witness has not been redeemed exactly once"))}self.data.received=true;Ok(())}
}
impl<I:OriginalReturnOrigin> Drop for OriginalReturnWitness<I>{fn drop(&mut self){assert!(std::thread::panicking()||self.received,"original held return witness requires its actual receiving caller");}}
pub fn original_return_claim_copy_bytes<I:OriginalReturnOrigin>()->usize{std::mem::size_of::<OriginalReturnEscrowRow>()+std::mem::size_of::<Option<OriginalReturnClaimCredit<I>>>()+2*std::mem::size_of::<u64>()+4*std::mem::size_of::<u8>()+std::mem::size_of::<usize>()}
`;
let next=old.slice(0,traitStart)+old.slice(phaseStart);
const creditStart=next.indexOf("#[repr(transparent)]\npub struct OriginalReturnClaimCredit"),copyStart=next.indexOf("pub fn original_return_claim_copy_bytes",creditStart),copyEnd=next.indexOf("\npub struct OriginalReturnBodyIdentity",copyStart);
next=next.slice(0,creditStart)+"pub type OriginalReturnClaimCredit=semio_framework_job_return_budget::OriginalReturnClaimCredit<OriginalReturnSourceInput>;\n"+"pub fn original_return_claim_copy_bytes()->usize{semio_framework_job_return_budget::original_return_claim_copy_bytes::<OriginalReturnSourceInput>()}"+next.slice(copyEnd);
const qualImpl=next.indexOf("impl OriginalReturnClaimCredit{"),qualStart=next.indexOf(" pub fn qualify(",qualImpl),dropStart=next.indexOf("impl Drop for OriginalReturnClaimCredit",qualStart),witnessEnd=next.indexOf("/// 🌐️",dropStart);
if([qualImpl,qualStart,dropStart,witnessEnd].some(n=>n<0))throw Error("qualification boundary");
const qualBody=next.slice(qualStart,dropStart).replace(" pub fn qualify("," fn qualify(").replace(" pub fn qualify_source("," fn qualify_source(");
const trait=`pub trait OriginalReturnCreditQualification{
 fn qualify(&self,cx:&StepContext<'_>,identity:&OriginalReturnBodyIdentity)->Result<(),OriginalTurnFailure>;
 fn qualify_source(&self,cx:&StepContext<'_>,binding:crate::OriginalSourceBinding)->Result<(),OriginalTurnFailure>;
 fn qualify_context(&self,cx:&StepContext<'_>,context:Option<ValueError>)->Result<(),OriginalTurnFailure>;
}
impl OriginalReturnCreditQualification for OriginalReturnClaimCredit{
`;
next=next.slice(0,qualImpl)+trait+qualBody+"pub type OriginalReturnWitness=semio_framework_job_return_budget::OriginalReturnWitness<OriginalReturnSourceInput>;\n"+next.slice(witnessEnd);
next=next.replace(" fn recipient(&self)->usize{self.recipient}"," fn recipient(&self)->usize{self.recipient}\n fn source_identity(&self)->usize{self.source}");
if(!next.includes("witness.data.received=true;"))throw Error("observed witness seam");next=next.replace("witness.data.received=true;","witness.mark_received()?;");
if(next.includes("OriginalReturnBudgetAuthority")||next.includes("impl OriginalReturnClaimCredit{")||next.includes("witness.data.")||next.includes("credit.data."))throw Error("private phase escape");
const test=owner.replace("/🦀️.rs","/🧪️tests/🦀️.rs"),testOld=fs.readFileSync(test,"utf8"),testNext=testOld.replace(" fn recipient(&self)->usize{self.recipient}"," fn recipient(&self)->usize{self.recipient}\n fn source_identity(&self)->usize{1}");
const place="🧰️framework/🔨️modules/🧵️job/⏱️context/📦️return/🎟️place/🦀️.rs",placeOld=fs.readFileSync(place,"utf8"),placeNext='use crate::original_job_continuation::source::OriginalReturnCreditQualification;\n'+placeOld;
if(fs.readFileSync(p,"utf8")!==old||fs.readFileSync(owner,"utf8")!==base||fs.readFileSync(place,"utf8")!==placeOld||fs.readFileSync(test,"utf8")!==testOld)throw Error("concurrent typed authority split");
for(const[file,name,value]of[[owner,"typed-budget",newOwner],[p,"typed-context",next],[test,"typed-budget-law",testNext],[place,"typed-place",placeNext]]){const temp=ticket+"/🗑️generated/"+name+".rs";fs.writeFileSync(temp,value);fs.renameSync(temp,file);}
fs.appendFileSync(ticket+"/return-budget-boundary-2026-11-06.md","\nThe completed boundary now owns typed Credit<Origin> and Witness<Origin> conservation/mint mechanics as well. Context's private captured Origin type nominally indexes each opaque token; fake Origin implementations cannot retag authority. Mint and redemption independently verify the bound actual budget address and table Source/recipient identity. Token fields remain private, phase data is read-only, there is no public raw constructor, and Source/clock/context qualification stays in a local context trait before operations. Drop fences remain. This replaces the intermediate data-wrapper split with one actual owner, reducing duplicated production code. Native compilation remains required.\n");console.log(JSON.stringify({contextSaved:Buffer.byteLength(old)-Buffer.byteLength(next),ownerBytes:Buffer.byteLength(newOwner)}));
