import{readFileSync,writeFileSync,renameSync,appendFileSync}from"node:fs";import{dirname,join}from"node:path";
const t=dirname(import.meta.path),b="🧰️framework/🔨️modules/🌱️value/🛬️decode/";
function edit(f:string,fn:(s:string)=>string){let s=readFileSync(f,"utf8"),n=fn(s);if(n===s)throw Error("no change "+f);let q=join(t,"🗑️generated/shared-quotes-"+Date.now()+".rs");writeFileSync(q,n);if(readFileSync(f,"utf8")!==s)throw Error("concurrent "+f);renameSync(q,f)}
function rep(s:string,a:string,z:string){if(s.split(a).length!==2)throw Error("anchor "+a);return s.replace(a,z)}
edit(b+"🏭️construction/🦀️.rs",s=>rep(s," fn publication_demand()->RetirementDemand",` /// ⛽️ Quotes the original sealed birth or publication without advancing it.
 pub fn shared_work_demand(&self)->Result<u64,ValueError>{if !self.can_advance()||self.arc.is_some(){return Err(ValueError::invariant("sealed admission requires its resolved original frame"))}Ok(if self.sealed.is_some(){1}else{SealedShared::<T>::birth_work_demand()})}
 /// 🖼️ Quotes the original Arc allocation or publication.
 pub fn arc_work_demand(&self)->Result<u64,ValueError>{if !self.can_advance()||self.sealed.is_some(){return Err(ValueError::invariant("Arc admission requires its resolved original frame"))}Ok(1)}
 fn publication_demand()->RetirementDemand`));
edit(b+"🏭️construction/🧳️slots/🦀️.rs",s=>{s=rep(s," /// 🧷️ Advances the original sealed owner",` /// ⛽️ Quotes work only for the actual original sealed input and empty destination.
 pub fn advance_work_demand(&self)->Result<u64,ValueError>{if self.input.is_none()||self.output.is_some(){return Err(ValueError::invariant("sealed slots require original input and empty output"))}self.admission.shared_work_demand()}
 /// 🧷️ Advances the original sealed owner`);return rep(s," /// 🖼️ Advances the original Arc owner",` /// ⛽️ Quotes work only for the actual original Arc input and empty destination.
 pub fn advance_work_demand(&self)->Result<u64,ValueError>{if self.input.is_none()||self.output.is_some(){return Err(ValueError::invariant("Arc slots require original input and empty output"))}self.admission.arc_work_demand()}
 /// 🖼️ Advances the original Arc owner`)});
appendFileSync(join(t,"native-work-view-2026-11-06.md"),"\nSharedAdmission now quotes its actual sealed allocator birth or one-step publication and Arc allocation/publication. It rejects unresolved or wrong-kind frames. SharedSlots validates its actual input/output custody before delegating; these queries neither advance nor create a work loan. Native compilation remains pending.\n");
console.log("Mounted SharedAdmission and SharedSlots actual work quotes");
