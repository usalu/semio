D='/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/'
def sub(path,pairs):
    s=open(D+path).read()
    for a,b in pairs:
        assert s.count(a)==1,(path,a[:60],s.count(a))
        s=s.replace(a,b)
    open(D+path,'w').write(s)
P='🧬️native/🔗️input/🦀️.rs'
sub(P,[
(" authority:ManuallyDrop<Option<FactoryOwnedSourceAuthority<M>>>,\n"," authority:ManuallyDrop<Option<FactoryOwnedSourceAuthority<M>>>,\n admission:ManuallyDrop<Option<RetainedCloneSourceAdmission<M,FactoryOwnedSourceAuthority<M>>>>,\n"),
("Ok(Some((Self{authority:ManuallyDrop::new(Some(FactoryOwnedSourceAuthority::new(original,Arc::clone(factory)))),source:","Ok(Some((Self{authority:ManuallyDrop::new(None),admission:ManuallyDrop::new(Some(RetainedCloneSource::admission_borrowed(FactoryOwnedSourceAuthority::new(original,Arc::clone(factory)),project_authority::<M>))),source:"),
("impl<M:Send+Sync+'static> ArtifactMutationSource<M>{\n pub fn constructor_demand","fn project_authority<M:Send+Sync+'static>(authority:&FactoryOwnedSourceAuthority<M>)->&M{authority.get()}\nimpl<M:Send+Sync+'static> ArtifactMutationSource<M>{\n pub fn constructor_demand"),
("  if self.closing{return self.close_demand(body);}if self.authority.is_some(){let birth=RetainedCloneSource::<M,FactoryOwnedSourceAuthority<M>>::borrowed_constructor_demand::<FactoryOwnedSourceAuthority<M>>();return Ok(RetirementDemand{copy_bytes:RetainedCloneSource::<M,FactoryOwnedSourceAuthority<M>>::constructor_copy_bytes(),capacity_bytes:birth.capacity_bytes,depth:birth.depth,..Default::default()});}\n",
 "  if self.closing{return self.close_demand(body);}if let Some(admission)=self.admission.as_ref(){return admission.next_demand();}\n"),
("  if let Some(authority)=self.authority.take(){return match RetainedCloneSource::<M,FactoryOwnedSourceAuthority<M>>::admit_borrowed(authority,|owner:&FactoryOwnedSourceAuthority<M>|owner.get(),grant){Ok((source,p))=>{*self.source=Some(source);Ok(RetainedCloneStep::Progress(p))},Err((error,authority))=>{*self.authority=Some(authority);Err(error)}};}\n",
 "  if self.admission.is_some(){return self.admit_source(grant).map(RetainedCloneStep::Progress);}\n"),
("source.as_ref().ok_or_else(||invalid(\"original mutation source absent\")).result?.retain_original(grant)?;","source.as_ref().ok_or_else(||invalid(\"original mutation source absent\"))?.retain_original(grant).result?;"),
(" pub fn take(&mut self,output:&mut Option<RetainedOwnedProjection<M>>",
 " fn admit_source(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let admission=self.admission.as_mut().ok_or_else(||invalid(\"original mutation source admission absent\"))?;if admission.phase()!=\"ready\"{return Ok(admission.advance(grant).result?.progress());}let mut output=None;let step=admission.take_output(&mut output,grant).result?;if let Some(source)=output{*self.source=Some(source);self.admission.take();}Ok(step.progress())}\n fn close_admission(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{if self.admission.as_ref().is_some_and(|admission|admission.phase()==\"ready\"){return self.admit_source(grant);}let admission=self.admission.as_mut().ok_or_else(||invalid(\"original mutation source admission absent\"))?;let mut inputs=None;let step=admission.return_inputs(&mut inputs,grant).result?;if let Some((authority,()))=inputs{*self.authority=Some(authority);self.admission.take();}Ok(step.progress())}\n pub fn take(&mut self,output:&mut Option<RetainedOwnedProjection<M>>"),
("  if let Some(projection)=self.projection.as_ref(){return if projection.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<M>>>())}else{projection.close_demand(body)};}\n",
 "  if let Some(admission)=self.admission.as_ref(){return if admission.phase()==\"ready\"{admission.next_demand()}else{admission.next_return_demand()};}\n  if let Some(projection)=self.projection.as_ref(){return if projection.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<M>>>())}else{projection.close_demand(body)};}\n"),
("  if !self.retirement.is_empty(){return self.retirement.step_granted(grant);}\n","  if !self.retirement.is_empty(){return self.retirement.step_granted(grant).result;}\n  if self.admission.is_some(){return self.close_admission(grant).map(RetainedCloneStep::Progress);}\n"),
("self.closing&&self.authority.is_none()&&self.source.is_none()","self.closing&&self.authority.is_none()&&self.admission.is_none()&&self.source.is_none()"),
])
