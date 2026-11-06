import{readFileSync,writeFileSync,renameSync}from"node:fs";import{dirname,join}from"node:path";const t=dirname(import.meta.path),b="🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/";
function edit(f:string,fn:(s:string)=>string){const s=readFileSync(f,"utf8"),n=fn(s),tmp=join(t,"🗑️generated/child-projection-"+Date.now()+".rs");writeFileSync(tmp,n);if(readFileSync(f,"utf8")!==s)throw Error("concurrent "+f);renameSync(tmp,f)}
edit(b+"🧪️tests/📌️retain/🦀️.rs",s=>s+`
#[test]
fn fallible_child_retention_is_funded_before_accessor_and_preserves_original_alias(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let rows:Vec<String>=serde_json::from_value(law["rows"].clone()).unwrap();let rank=law["selected"].as_u64().unwrap()as usize;
 let mut bank=Bank::new();let mut root=source(rows,&mut bank);let d=RetainedOwnedProjection::<String>::constructor_demand();let g=grant(d,&bank);let calls=std::cell::Cell::new(0);
 for denied in[RetainedCloneGrant{maximum_items:0,..g},RetainedCloneGrant{maximum_depth:0,..g}]{let(outcome,heap)=observe_retirement_allocations(||root.borrow().try_project_owned(rank,|rows|{calls.set(calls.get()+1);rows.get(rank).ok_or_else(||crate::ValueError::invariant("absent original child"))},denied));assert!(outcome.result.is_err());assert_eq!(outcome.work,0);assert_eq!(calls.get(),0);assert_eq!(heap,(0,0));}
 let(original,heap)=bank.run(g,||root.borrow().try_project_owned(rank,|rows|{calls.set(calls.get()+1);rows.get(rank).ok_or_else(||crate::ValueError::invariant("absent original child"))},g),|(_,p)|*p);
 let(mut retained,p)=original.unwrap();assert_eq!(calls.get(),1);assert_eq!(heap,(0,0));assert!(p.fits(g));assert_eq!(retained.borrow().unwrap().get().as_str(),law["retained"].as_str().unwrap());assert_eq!(retained.borrow().unwrap().get().as_ptr(),root.borrow().get()[rank].as_ptr());assert_eq!(retained.borrow().unwrap().source_identity(),root.borrow().source_identity());
 let(failed,heap)=observe_retirement_allocations(||root.borrow().try_project_owned(usize::MAX,|rows|{calls.set(calls.get()+1);rows.get(usize::MAX).ok_or_else(||crate::ValueError::invariant("absent original child"))},g));assert!(failed.result.is_err());assert_eq!(failed.work,0);assert_eq!(calls.get(),2);assert_eq!(heap,(0,0));
 for _ in 0..10000{if retained.terminal_is_empty(){break}let g=grant(retained.close_demand(4096).unwrap(),&bank);let(result,_)=bank.run(g,||retained.close_step(g),|s|s.progress());result.unwrap();}
 assert!(retained.terminal_is_empty());close_source(&mut root,&mut bank);
}
`);
edit(b+"📦️packages/🦀️rust/🦀️.rs",s=>{let a=" pub fn retain_owned(self,grant:RetainedCloneGrant)->RetainedWorkOutcome<(RetainedOwnedProjection<T>,RetainedCloneProgress)>{unsafe{RetainedOwnedProjection::from_source(self,grant)}}";if(s.split(a).length!==2)throw Error("canonical ref anchor");return s.replace(a,a+`
 /// 🌿️ Retains a fallible original child only after its actual alias grant is admitted.
 pub fn try_project_owned<U:?Sized+Sync,F>(self,discriminator:usize,project:F,grant:RetainedCloneGrant)->RetainedWorkOutcome<(RetainedOwnedProjection<U>,RetainedCloneProgress)>where F:for<'source>FnOnce(&'source T)->Result<&'source U,ValueError>{
  if let Err(error)=RetainedOwnedProjection::<U>::admit_constructor(grant){return RetainedWorkOutcome{result:Err(error),work:0}}
  match self.try_project(discriminator,project){Ok(child)=>unsafe{RetainedOwnedProjection::from_source(child,grant)},Err(error)=>RetainedWorkOutcome{result:Err(error),work:0}}
 }
`)});
console.log("Canonical funded fallible child projection port + native neutral-fixture law authored");
