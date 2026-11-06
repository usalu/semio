import{readFileSync,writeFileSync,renameSync}from'node:fs';import{dirname,join}from'node:path';
const ticket=dirname(import.meta.path),root='🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/',base=root+'🔁️replay/🎮️operation/';
function rep(s:string,a:string,b:string,count=1){if(s.split(a).length!==count+1)throw Error('anchor '+a);return s.replaceAll(a,b)}
function edit(file:string,fn:(s:string)=>string){const old=readFileSync(file,'utf8'),next=fn(old),tmp=join(ticket,'🗑️generated/replay-work-'+Date.now()+'.rs');writeFileSync(tmp,next);if(readFileSync(file,'utf8')!==old)throw Error('concurrent '+file);renameSync(tmp,file);console.log(file)}
edit(base+'🧬️native/🔌️drivers/🦀️.rs',s=>{
 s=rep(s,"fn demands(&self,s:RetainedCloneRef<'_,P>,b:usize)","fn demands(&self,s:RetainedCloneRef<'_,P>,b:usize,maximum_work_units:u64)",2);
 s=rep(s,"fn advance_work_demand(&self,s:RetainedCloneRef<'_,P>,body:usize)","fn advance_work_demand(&self,s:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64)",2);
 s=rep(s,"fn advance(&mut self,s:RetainedCloneRef<'_,P>,body:usize,g:","fn advance(&mut self,s:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64,g:",2);
 s=rep(s,'.demands(s,b)','.demands(s,b,maximum_work_units)');s=rep(s,'.advance_demands(s,b)','.advance_demands(s,b,maximum_work_units)');s=rep(s,'.advance_work_demand(s,body)','.advance_work_demand(s,body,maximum_work_units)',2);return rep(s,'.advance(s,body,g)','.advance(s,body,maximum_work_units,g)',2);
});
edit(base+'🧬️native/🦀️.rs',s=>{
 s=rep(s,"fn demands(&self,source:RetainedCloneRef<'_,P>,body:usize)","fn demands(&self,source:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64)");
 s=rep(s,"fn advance_work_demand(&self,_source:RetainedCloneRef<'_,P>,_body:usize)","fn advance_work_demand(&self,_source:RetainedCloneRef<'_,P>,_body:usize,_maximum_work_units:u64)");
 s=rep(s,"fn advance(&mut self,source:RetainedCloneRef<'_,P>,body:usize,grant:","fn advance(&mut self,source:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64,grant:");
 s=rep(s,'pub fn demands(&self,body:usize)','pub fn demands(&self,body:usize,maximum_work_units:u64)');
 s=rep(s,'"semantic original snapshot source absent"))?.try_borrow()?,body)?','"semantic original snapshot source absent"))?.try_borrow()?,body,maximum_work_units.checked_sub(1).unwrap_or(0))?');
 s=rep(s,'  if self.closing||!retained_clone_grant_admits(grant,self.demands(body)?)','  let maximum_work_units=receiving.available_work_units()?;if maximum_work_units==0{return Ok(ArtifactReplayPreparationStep::Blocked)}if self.closing||!retained_clone_grant_admits(grant,self.demands(body,maximum_work_units)?)');
 s=rep(s,'"semantic source absent"))?.try_borrow()?,body)?.map','"semantic source absent"))?.try_borrow()?,body,maximum_work_units.checked_sub(1).unwrap_or(0))?.map');
 s=rep(s,'self.advance_original(body,grant,','self.advance_original(body,maximum_work_units,grant,',2);
 s=rep(s,'fn advance_original(&mut self,body:usize,grant:','fn advance_original(&mut self,body:usize,maximum_work_units:u64,grant:');
 s=rep(s,'let d=self.demands(body)?','let d=self.demands(body,maximum_work_units)?');
 s=rep(s,'self.projection.advance(self.source.as_ref().unwrap().try_borrow()?,body,grant)','self.projection.advance(self.source.as_ref().unwrap().try_borrow()?,body,maximum_work_units.checked_sub(1).unwrap_or(0),grant)');
 return rep(s,'fn advance_demands(&self,b:usize)->Result<RetirementDemand,ValueError>{self.demands(b)}','fn advance_demands(&self,b:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{self.demands(b,maximum_work_units)}');
});
edit(base+'🦀️.rs',s=>{s=rep(s,'fn advance_demands(&self, body: usize)','fn advance_demands(&self, body: usize, maximum_work_units:u64)');s=rep(s,'revision: [u8; 32],body:usize)','revision: [u8; 32],body:usize,maximum_work_units:u64)');return rep(s,'self.cursor.advance_demands(body)','self.cursor.advance_demands(body,maximum_work_units)')});
edit(base+'🌱️derived/🦀️.rs',s=>{s=rep(s,'pub fn demands(&self,generation:u64,revision:[u8;32],body:usize)','pub fn demands(&self,generation:u64,revision:[u8;32],body:usize,maximum_work_units:u64)');s=rep(s,'task.demands(generation,revision,body)','task.demands(generation,revision,body,maximum_work_units)');return rep(s,'let demand=self.demands(generation,revision,body)?','let maximum_work_units=match &mut turn{DerivedOperationTurn::Advance(control,_)=>control.available_work_units()?,_=>0};let demand=self.demands(generation,revision,body,maximum_work_units)?')});
edit(base+'🌱️derived/🧩️store/🦀️.rs',s=>{s=rep(s,'owner:&ArtifactDerivedOperation<P,M>,body:usize)','owner:&ArtifactDerivedOperation<P,M>,body:usize,maximum_work_units:u64)');return rep(s,'owner.demands(self.generation(),self.content_revision(),body)','owner.demands(self.generation(),self.content_revision(),body,maximum_work_units)')});
edit(root+'🏗️initialization/🏠️owners/🦀️.rs',s=>rep(s,'task.demands(generation,revision,body)?','task.demands(generation,revision,body,cx.fuel_remaining())?'));
edit(root+'🏗️initialization/🔁️replay/🧬️native/🦀️.rs',s=>rep(s,'owner.demands(cx.generation().0,self.revision,grant.maximum_copy_bytes)?','owner.demands(cx.generation().0,self.revision,grant.maximum_copy_bytes,cx.fuel_remaining())?'));
