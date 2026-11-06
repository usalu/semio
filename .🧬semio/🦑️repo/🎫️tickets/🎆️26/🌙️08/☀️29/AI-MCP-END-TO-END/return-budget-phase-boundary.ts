import fs from"node:fs";const ticket=import.meta.dir,p="🧰️framework/🔨️modules/🧵️job/📬️outcome/⏸️continuation/🎟️source/🦀️.rs",owner="🧰️framework/🔨️modules/🧵️job/📬️outcome/🪙️budget/🦀️.rs";
const old=fs.readFileSync(p,"utf8"),base=fs.readFileSync(owner,"utf8");
const phaseStart=old.indexOf("#[derive(Clone,Copy,Debug,PartialEq,Eq)]\npub enum OriginalReturnPhase");
const phaseEnd=old.indexOf("\npub struct OriginalReturnClaimCredit",phaseStart);
const creditDefEnd=old.indexOf("\npub fn original_return_claim_copy_bytes",phaseEnd);
const methodsStart=old.indexOf("impl OriginalReturnClaimCredit{",creditDefEnd);
const qualify=old.indexOf(" pub fn qualify(",methodsStart);
const witnessStart=old.indexOf("pub struct OriginalReturnWitness{",qualify),witnessEnd=old.indexOf("\nimpl OriginalReturnWitness{",witnessStart);
if([phaseStart,phaseEnd,creditDefEnd,methodsStart,qualify,witnessStart,witnessEnd].some(n=>n<0))throw Error("phase boundary seams");
let data=old.slice(phaseStart,phaseEnd)+"\n"+old.slice(phaseEnd+1,creditDefEnd)+"\n"+old.slice(witnessStart,witnessEnd)+"\n"+old.slice(methodsStart,qualify)+"}\n";
data=data.replaceAll("OriginalReturnClaimCredit","OriginalReturnClaimState").replaceAll("OriginalReturnWitness","OriginalReturnWitnessState");
data=data.replace(/pub struct (OriginalReturnClaimState|OriginalReturnWitnessState)\{([^\n]+)\}/g,(_,name,fields)=>"pub struct "+name+"{"+fields.replace(/(^|,)(\w+):/g,"$1pub $2:")+"}");
const newOwner=base.replace("OriginalReturnEscrowMessages as E,","OriginalReturnEscrowMessages as E,OriginalReturnEscrowAccounting as M,")+"\n"+data;
const phaseAlias="pub use semio_framework_job_return_budget::OriginalReturnPhase;";
const credit=`#[repr(transparent)]
pub struct OriginalReturnClaimCredit{data:semio_framework_job_return_budget::OriginalReturnClaimState}
impl std::ops::Deref for OriginalReturnClaimCredit{type Target=semio_framework_job_return_budget::OriginalReturnClaimState;fn deref(&self)->&Self::Target{&self.data}}
`;
const delegates=`impl OriginalReturnClaimCredit{
 pub fn terminal_is_empty(&self)->bool{self.data.terminal_is_empty()}
 pub fn has_unused_capture(&self)->bool{self.data.has_unused_capture()}
 pub fn issue_return(&mut self,phase:OriginalReturnPhase,copy_bytes:usize)->Result<OriginalReturnWitness,ValueError>{self.data.issue_return(phase,copy_bytes).map(|data|OriginalReturnWitness{data})}
 pub fn cancel_preissued(&mut self,w:&mut OriginalReturnWitness)->Result<(),ValueError>{self.data.cancel_preissued(&mut w.data)}
 pub fn issue_success_return(&mut self,copy_bytes:usize)->Result<OriginalReturnWitness,ValueError>{self.data.issue_success_return(copy_bytes).map(|data|OriginalReturnWitness{data})}

`;
const witness=`#[repr(transparent)]
pub struct OriginalReturnWitness{data:semio_framework_job_return_budget::OriginalReturnWitnessState}
impl std::ops::Deref for OriginalReturnWitness{type Target=semio_framework_job_return_budget::OriginalReturnWitnessState;fn deref(&self)->&Self::Target{&self.data}}
`;
let next=old.slice(0,phaseStart)+phaseAlias+"\n"+credit+old.slice(creditDefEnd,methodsStart)+delegates+old.slice(qualify,witnessStart)+witness+old.slice(witnessEnd);
const literal="OriginalReturnClaimCredit{source:origin.source,serial:plan.serial,slot,local,capture:true,receive:true,capture_observed:false,receive_observed:false,live:true}";
if(!next.includes(literal))throw Error("sealed credit mint");
next=next.replace(literal,"OriginalReturnClaimCredit{data:semio_framework_job_return_budget::OriginalReturnClaimState{source:origin.source,serial:plan.serial,slot,local,capture:true,receive:true,capture_observed:false,receive_observed:false,live:true}}");
for(const field of["capture_observed","receive_observed","live"])next=next.replace(new RegExp("credit\\."+field+"=","g"),"credit.data."+field+"=");
for(const field of["redeemed","received"])next=next.replace(new RegExp("witness\\."+field+"=","g"),"witness.data."+field+"=");
if(fs.readFileSync(p,"utf8")!==old||fs.readFileSync(owner,"utf8")!==base)throw Error("concurrent boundary change");
for(const[file,suffix,text]of[[owner,"phase-owner",newOwner],[p,"phase-context",next]]){const tmp=ticket+"/🗑️generated/"+suffix+".rs";fs.writeFileSync(tmp,text);fs.renameSync(tmp,file);}
fs.appendFileSync(ticket+"/return-budget-boundary-2026-11-06.md","\nExtracted pure claim/witness phase DATA and arithmetic into the budget owner. The actual context Credit and Witness are transparent opaque wrappers with private data and read-only Deref; only context mints them after genuine Source qualification. There is no mutable dereference, public authority constructor or unchecked retag. Drop fences and all context qualification remain on the opaque wrappers. Existing real ReturnPlace laws exercise this same phase implementation; native compilation remains required.\n");
console.log(JSON.stringify({phaseDataBytes:Buffer.byteLength(data),contextSaved:Buffer.byteLength(old)-Buffer.byteLength(next)}));
