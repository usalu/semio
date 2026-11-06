import fs from"node:fs";import path from"node:path";
const ticket=import.meta.dir,root="🧰️framework/🔨️modules/🧵️job/📬️outcome/🪙️budget",source="🧰️framework/🔨️modules/🧵️job/📬️outcome/⏸️continuation/🎟️source/🦀️.rs";
const original=fs.readFileSync(source,"utf8");
const sourceStart=original.indexOf("/// 🔒️ See `README.md`."),snapshotStart=original.indexOf("pub use semio_framework_job_payload::OriginalReturnBudgetSnapshot;"),deliveryStart=original.indexOf("#[derive(Clone,Copy,PartialEq,Eq)]",snapshotStart),claimStart=original.indexOf("  fn claim(");
const actualClaimStart=original.indexOf(" fn claim(",deliveryStart);
if([sourceStart,snapshotStart,deliveryStart,actualClaimStart].some(n=>n<0))throw Error("boundary seams");
let sourceBlock=original.slice(sourceStart,snapshotStart);
sourceBlock=sourceBlock.replace("pub struct OriginalReturnSource{","pub struct OriginalReturnSource<I:OriginalReturnOrigin>{").replace("impl OriginalReturnSource{","impl<I:OriginalReturnOrigin> OriginalReturnSource<I>{").replace("impl Drop for OriginalReturnSource{","impl<I:OriginalReturnOrigin> Drop for OriginalReturnSource<I>{");
sourceBlock=sourceBlock.replaceAll("OriginalReturnBudget>","OriginalReturnBudget<I>>").replaceAll("&OriginalReturnBudget{","&OriginalReturnBudget<I>{").replaceAll("&mut OriginalReturnBudget{","&mut OriginalReturnBudget<I>{").replaceAll("OriginalReturnBudget::empty","OriginalReturnBudget::<I>::empty");
sourceBlock=sourceBlock.replace("pub enum OriginalReturnBudgetReference<'a>{","pub enum OriginalReturnBudgetReference<'a,I:OriginalReturnOrigin>{").replace("impl OriginalReturnBudgetReference<'_>{","impl<I:OriginalReturnOrigin> OriginalReturnBudgetReference<'_,I>{").replaceAll("Option<OriginalReturnBudgetReference<'_>>","Option<OriginalReturnBudgetReference<'_,I>>");
let pure=original.slice(deliveryStart,actualClaimStart);
pure=pure.replace("enum OriginalReturnDelivery{","pub enum OriginalReturnDelivery{");
pure=pure.replace("pub struct OriginalReturnBudget{","pub struct OriginalReturnBudget<I:OriginalReturnOrigin>{").replace("origin:Option<OriginalReturnSourceInput>","origin:Option<I>");
const structStart=pure.indexOf("pub struct OriginalReturnBudget"),structEnd=pure.indexOf("\nimpl OriginalReturnBudget",structStart);
pure=pure.slice(0,structStart)+pure.slice(structStart,structEnd).replace(/([,{])(\w+):/g,"$1pub $2:")+pure.slice(structEnd);
pure=pure.replace("impl OriginalReturnBudget{","impl<I:OriginalReturnOrigin> OriginalReturnBudget<I>{").replace(" const fn empty("," pub const fn empty(").replace("input:OriginalReturnSourceInput","input:I");
const comparison="origin.source!=input.source||origin.clock!=input.clock||!std::ptr::fn_addr_eq(origin.now,input.now)||origin.deadline!=input.deadline||origin.recipient!=input.recipient||origin.operation!=input.operation||origin.generation!=input.generation";
if(!pure.includes(comparison))throw Error("origin comparison");pure=pure.replace(comparison,"!origin.same_origin(&input)");
for(const name of["recipient","granted","fuel","spent","work"])for(const object of["input","origin"])pure=pure.replaceAll(object+"."+name,object+"."+name+"()");
pure+="}\n";
const trait=`/// 🪪️ Data-only origin observations; the context owns the actual Source-qualified input constructor.
pub trait OriginalReturnOrigin:Copy{
 fn same_origin(&self,other:&Self)->bool;
 fn recipient(&self)->usize;
 fn granted(&self)->RetainedCloneGrant;
 fn fuel(&self)->u64;
 fn spent(&self)->P;
 fn work(&self)->u128;
}
`;
const imports=`//! 🪙️ Typed return-budget storage, conservation and physical closure without context capability minting.
use semio_framework_value_retained_clone::{RetainedCloneGrant,RetainedCloneProgress as P,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_framework_value_retained_clone::retirement::controlled::{RetainedOwnerGate,RetainedOwnerGateGuard};
use semio_framework_job_return_escrow_publication::{OriginalEscrowPublisher,OriginalEscrowPublication};
use semio_framework_job_payload::{OriginalReturnEscrowTable,OriginalReturnEscrowMessages as E,ORIGINAL_RETURN_ESCROW_RECLAIM_COPY_BYTES};
pub use semio_framework_job_payload::OriginalReturnBudgetSnapshot;
fn refused(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
`;
const module=imports+trait+sourceBlock+pure+'\n#[cfg(test)]\n#[path="🧪️tests/🦀️.rs"]\nmod tests;\n';
if(module.includes("OriginalReturnSourceInput")||module.includes("StepContext"))throw Error("context dependency leaked");
const implementation=`
impl semio_framework_job_return_budget::OriginalReturnOrigin for OriginalReturnSourceInput{
 fn same_origin(&self,other:&Self)->bool{self.source==other.source&&self.clock==other.clock&&std::ptr::fn_addr_eq(self.now,other.now)&&self.deadline==other.deadline&&self.recipient==other.recipient&&self.operation==other.operation&&self.generation==other.generation}
 fn recipient(&self)->usize{self.recipient}
 fn granted(&self)->RetainedCloneGrant{self.granted}
 fn fuel(&self)->u64{self.fuel}
 fn spent(&self)->P{self.spent}
 fn work(&self)->u128{self.work}
}
pub type OriginalReturnBudget=semio_framework_job_return_budget::OriginalReturnBudget<OriginalReturnSourceInput>;
pub type OriginalReturnBudgetReference<'a>=semio_framework_job_return_budget::OriginalReturnBudgetReference<'a,OriginalReturnSourceInput>;
use semio_framework_job_return_budget::OriginalReturnDelivery;
/// 🔒️ Only context can capture this origin or mint a genuine claimed return.
pub struct OriginalReturnSource{core:semio_framework_job_return_budget::OriginalReturnSource<OriginalReturnSourceInput>}
impl OriginalReturnSource{pub const fn empty(birth:u64)->Self{Self{core:semio_framework_job_return_budget::OriginalReturnSource::empty(birth)}}}
impl std::ops::Deref for OriginalReturnSource{type Target=semio_framework_job_return_budget::OriginalReturnSource<OriginalReturnSourceInput>;fn deref(&self)->&Self::Target{&self.core}}
impl std::ops::DerefMut for OriginalReturnSource{fn deref_mut(&mut self)->&mut Self::Target{&mut self.core}}
trait OriginalReturnBudgetAuthority{
 fn claim(&mut self,copy_bytes:usize,destination:usize,epoch:u64)->Result<Option<OriginalReturnClaimCredit>,ValueError>;
 fn redeem(&mut self,credit:&mut OriginalReturnClaimCredit,witness:&mut OriginalReturnWitness)->Result<(),ValueError>;
}
impl OriginalReturnBudgetAuthority for OriginalReturnBudget{
`;
let next=original.slice(0,sourceStart)+implementation+original.slice(actualClaimStart);
next=next.replace("use semio_framework_value_retained_clone::retirement::controlled::{RetainedOwnerGate,RetainedOwnerGateGuard};\n","");
function writeNew(p:string,s:string){if(fs.existsSync(p))throw Error("exists "+p);if(path.resolve(p).length>256)throw Error("long path "+p);fs.mkdirSync(path.dirname(p),{recursive:true});fs.writeFileSync(p,s);}
writeNew(root+"/🦀️.rs",module);
writeNew(root+"/README.md","# Typed Return Budget\n\nOwns generic origin-indexed budget data, the guarded physical home, conservation, and paid physical close. Context owns its private Source input constructor and claimed-return authority extension. This package does not accept a StepContext or mint Source-bound credits.\n\nExisting Source rationale is preserved in [the Source notes](../⏸️continuation/🎟️source/README.md).\n");
const manifest=root+"/📦️packages/🦀️rust/Cargo.toml";const framework="🧰️framework";const rel=(base:string,target:string)=>path.relative(base,target).replaceAll("\\","/");const pkgdir=path.dirname(manifest);
writeNew(manifest,`[package]
workspace = "${rel(pkgdir,framework)}"
name = "semio-framework-job-return-budget"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "Typed original return-budget data, conservation and physical closure"

[package.metadata.semio]
role = "framework"
id = "job-return-budget"

[package.metadata.semio.rust-source-boundary]
maxSourceBytes = 262144
maxSourceFiles = 32
maxActiveLines = 4000

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[dependencies]
semio-framework-job-payload = { path = "${rel(pkgdir,"🧰️framework/🔨️modules/🧵️job/📬️outcome/📦️builder/📦️packages/🦀️rust")}" }
semio-framework-job-return-escrow-publication = { path = "${rel(pkgdir,"🧰️framework/🔨️modules/🧵️job/📬️outcome/📤️publication/🪙️escrow/📦️packages/🦀️rust")}" }
semio-framework-value-retained-clone = { path = "${rel(pkgdir,"🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/📦️packages/🦀️rust")}" }

[dev-dependencies]
serde_json = "1.0.140"
`);
function edit(p:string,fn:(s:string)=>string){const old=fs.readFileSync(p,"utf8"),next=fn(old);if(old===next)throw Error("no edit "+p);if(fs.readFileSync(p,"utf8")!==old)throw Error("stale "+p);const tmp=ticket+"/🗑️generated/budget-boundary.tmp";fs.writeFileSync(tmp,next);fs.renameSync(tmp,p);}
const contextManifest="🧰️framework/🔨️modules/🧵️job/⏱️context/📦️packages/🦀️rust/Cargo.toml";
edit(contextManifest,s=>s.replace("[dependencies]\n","[dependencies]\nsemio-framework-job-return-budget = { path = \""+rel(path.dirname(contextManifest),pkgdir)+"\" }\n"));
edit("🧰️framework/Cargo.toml",s=>{const member=rel("🧰️framework",pkgdir);if(s.includes('"'+member+'"'))throw Error("already member");return s.replace("members = [","members = [\n    \""+member+"\",");});
if(fs.readFileSync(source,"utf8")!==original)throw Error("source changed during split");const tmp=ticket+"/🗑️generated/return-budget-source.rs";fs.writeFileSync(tmp,next);fs.renameSync(tmp,source);
fs.appendFileSync(ticket+"/return-budget-boundary-2026-11-06.md","\nApplied generic budget and guarded storage owner. Context retains a local wrapper so header guard extensions remain actual context methods, and private Source input + actual claim/witness constructors stay in context. Budget state remains bound to the actual recipient address. Independent schema/AJV/SQLite law passed 1 test / 9 assertions. Native generic binding test and existing Source laws are not yet compiled.\n");
console.log(JSON.stringify({oldBytes:Buffer.byteLength(original),contextBytes:Buffer.byteLength(next),ownerBytes:Buffer.byteLength(module),saved:Buffer.byteLength(original)-Buffer.byteLength(next)}));
