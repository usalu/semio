import{readFileSync,writeFileSync,renameSync}from"node:fs";import{dirname,join}from"node:path";const t=dirname(import.meta.path),f="🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🦀️.rs",s=readFileSync(f,"utf8");let n=s;function rep(a:string,b:string){if(n.split(a).length!==2)throw Error("anchor "+a.slice(0,120));n=n.replace(a,b)}
n=n.replaceAll("grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::OriginalRetirementFailure>","grant:RetainedCloneGrant,actual:&mut Option<usize>)->Result<RetainedCloneStep,crate::OriginalRetirementFailure>").replaceAll("grant:RetainedCloneGrant,control:&mut dyn crate::OriginalRetirementControl)->","grant:RetainedCloneGrant,control:&mut dyn crate::OriginalRetirementControl,actual:&mut Option<usize>)->").replaceAll("grant:RetainedCloneGrant)->crate::OriginalRetirementOutcome","grant:RetainedCloneGrant,actual:&mut Option<usize>)->crate::OriginalRetirementOutcome").replaceAll("grant:RetainedCloneGrant)->RetirementStep","grant:RetainedCloneGrant,actual:&mut Option<usize>)->RetirementStep");
n=n.replaceAll("owned.step(grant)","owned.step(grant,actual)").replaceAll("(owner,grant,control)","(owner,grant,control,actual)").replaceAll("(owner,original,grant)","(owner,original,grant,actual)").replaceAll("self.step(grant)","self.step(grant,actual)").replaceAll("(self,grant,control)","(self,grant,control,actual)").replaceAll("(self,original,grant)","(self,original,grant,actual)");
rep("fn retirement(self)->Box<dyn RetirementCursor> {Box::new(SharedControlledRetirement::new(self))}","type Cursor=SharedControlledRetirement<T>;\n    fn retirement(self)->Self::Cursor{SharedControlledRetirement::new(self)}");
rep("fn retirement(self)->Box<dyn RetirementCursor>{Box::new(self)}","type Cursor=Self;\n    fn retirement(self)->Self::Cursor{self}");
const a=n.indexOf("pub fn admit_shared_retirement<"),b=n.indexOf("\nimpl<T:RetireOwned+Sync> Drop",a);if(a<0||b<0)throw Error("shared frame helper");
n=n.slice(0,a)+`pub fn shared_retirement_birth_demand<T:RetireOwned+Sync>()->Result<crate::RetirementDemand,ValueError>{
 let copy_bytes=size_of::<SharedControlledRetirement<T>>().checked_add(size_of::<Option<Arc<T>>>()).and_then(|n|n.checked_add(size_of::<Option<Box<dyn crate::ErasedSnapshotRetirement>>>())).ok_or_else(||ValueError::invariant("shared retirement frame placement extent overflow"))?;
 Ok(crate::RetirementDemand{copy_bytes,capacity_bytes:size_of::<SharedControlledRetirement<T>>(),depth:1,..Default::default()})
}
pub fn admit_shared_retirement<T:RetireOwned+Sync>(source:&mut Option<Arc<T>>,target:&mut Option<Box<dyn crate::ErasedSnapshotRetirement>>,grant:RetainedCloneGrant,lease_only:bool,actual:&mut Option<usize>)->crate::RetainedWorkOutcome{
 let pending=||crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Progress(Default::default())),work:0};
 let demand=match shared_retirement_birth_demand::<T>(){Ok(d)=>d,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};
 if source.is_none()||target.is_some()||actual.is_some(){return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("shared frame requires its original source and empty destination and marker")),work:0}}
 if !T::controlled_retirement_supported(){return crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"shared payload has no controlled retirement authority")),work:0}}
 if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return pending()}
 let layout=std::alloc::Layout::new::<SharedControlledRetirement<T>>();*actual=Some(0);
 let Some(pointer)=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<SharedControlledRetirement<T>>())else{return crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"shared frame allocation failed")),work:1}};
 *actual=Some(layout.size());
 let original=source.take().expect("original shared lease was validated before allocation");
 unsafe{pointer.as_ptr().write(SharedControlledRetirement::with_custody(original,lease_only));*target=Some(Box::from_raw(pointer.as_ptr()));}
 crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:layout.size(),..Default::default()})),work:4}
}
`+n.slice(b);
rep("depth:1,..Default::default()});}\n        self.owned","copy_bytes:size_of::<Option<Arc<T>>>()+size_of::<Option<ControlledRetirement<T>>>(),depth:1,..Default::default()});}\n        self.owned");
n=n.replaceAll('ControlledRetirement::new(value).unwrap_or_else(|(error,_)|panic!("admitted shared payload refused: {error}"))',"ControlledRetirement::retain(value)");
n=n.replaceAll("RetainedCloneProgress {copied_items:1,released_bytes,..empty}","RetainedCloneProgress {copied_items:1,copied_bytes:size_of::<Option<Arc<T>>>()+size_of::<Option<ControlledRetirement<T>>>(),released_bytes,..empty}").replaceAll("RetainedCloneProgress {copied_items:1,released_bytes:extent,..empty}","RetainedCloneProgress {copied_items:1,copied_bytes:size_of::<Option<Arc<T>>>()+size_of::<Option<ControlledRetirement<T>>>(),released_bytes:extent,..empty}");
rep("self.last_work=0;let empty=RetainedCloneProgress::default();","self.last_work=0;let empty=RetainedCloneProgress::default();let demand=self.next_demand(grant.maximum_copy_bytes)?;if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(empty))}");
const q=join(t,"🗑️generated/shared-retirement-frame.rs");writeFileSync(q,n);if(readFileSync(f,"utf8")!==s)throw Error("concurrent arc");renameSync(q,f);console.log("Shared frame original-slot allocator markers and inline Arc cursor construction authored");
