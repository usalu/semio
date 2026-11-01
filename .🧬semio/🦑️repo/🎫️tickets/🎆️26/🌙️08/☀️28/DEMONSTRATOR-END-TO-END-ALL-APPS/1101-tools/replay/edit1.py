import sys
D='/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/'
def sub(path,pairs):
    s=open(D+path).read()
    for a,b in pairs:
        assert s.count(a)==1,(path,a[:60],s.count(a))
        s=s.replace(a,b)
    open(D+path,'w').write(s)
sub('🧬️native/🦀️.rs',[
("RetainedCloneClose, retained_clone_grant_admits","RetainedCloneClose, RetainedCloneSourceAdmission, retained_clone_grant_admits"),
("source_authority:ManuallyDrop<Option<CapturedSnapshotSource<P>>>,","source_authority:ManuallyDrop<Option<CapturedSnapshotSource<P>>>,source_admission:ManuallyDrop<Option<RetainedCloneSourceAdmission<P,CapturedSnapshotSource<P>>>>,"),
("source_authority:ManuallyDrop::new(None),input:","source_authority:ManuallyDrop::new(None),source_admission:ManuallyDrop::new(None),input:"),
("fn invalid(reason:&'static str)->ValueError{","fn project_captured<P>(source:&CapturedSnapshotSource<P>)->&P{source.get()}\nfn invalid(reason:&'static str)->ValueError{"),
("Stage::Source=>{let d=RetainedCloneSource::<P,CapturedSnapshotSource<P>>::borrowed_constructor_demand::<CapturedSnapshotSource<P>>();RetirementDemand{copy_bytes:RetainedCloneSource::<P,CapturedSnapshotSource<P>>::constructor_copy_bytes(),capacity_bytes:d.capacity_bytes,depth:d.depth,..Default::default()}},",
 "Stage::Source=>self.source_admission.as_ref().ok_or_else(||invalid(\"semantic original source admission absent\"))?.next_demand()?,"),
("*self.source_authority=Some(source);self.stage=Stage::Source;p},","*self.source_admission=Some(RetainedCloneSource::admission_borrowed(source,project_captured::<P>));self.stage=Stage::Source;p},"),
("Stage::Source=>{let authority=self.source_authority.take().ok_or_else(||invalid(\"semantic original captured snapshot authority absent\"))?;match RetainedCloneSource::<P,CapturedSnapshotSource<P>>::admit_borrowed(authority,|source:&CapturedSnapshotSource<P>|source.get(),grant){Ok((source,p))=>{*self.source=Some(source);self.stage=Stage::ProjectionBirth;p},Err((error,authority))=>{*self.source_authority=Some(authority);return Err(error);}}},",
 "Stage::Source=>self.admit_source(grant)?,"),
("pub fn take(&mut self)->Option<ArtifactReplayPrepared<P,M>>{if self.closing||",
 "fn admit_source(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let admission=self.source_admission.as_mut().ok_or_else(||invalid(\"semantic original source admission absent\"))?;if admission.phase()!=\"ready\"{return Ok(admission.advance(grant).result?.progress());}let mut output=None;let step=admission.take_output(&mut output,grant).result?;if let Some(source)=output{*self.source=Some(source);self.source_admission.take();self.stage=Stage::ProjectionBirth;}Ok(step.progress())}\n fn close_admission(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let admission=self.source_admission.as_mut().ok_or_else(||invalid(\"semantic original source admission absent\"))?;if admission.phase()==\"ready\"{let mut output=None;let step=admission.take_output(&mut output,grant).result?;if let Some(source)=output{*self.source=Some(source);self.source_admission.take();}return Ok(step.progress());}let mut inputs=None;let step=admission.return_inputs(&mut inputs,grant).result?;if let Some((authority,()))=inputs{*self.source_authority=Some(authority);self.source_admission.take();}Ok(step.progress())}\n pub fn take(&mut self)->Option<ArtifactReplayPrepared<P,M>>{if self.closing||"),
("  if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok(header::<Option<RetainedCloneSource<P,CapturedSnapshotSource<P>>>>())}else{source.next_close_demand(body)};}\n",
 "  if let Some(admission)=self.source_admission.as_ref(){return if admission.phase()==\"ready\"{admission.next_demand()}else{admission.next_return_demand()};}\n  if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok(header::<Option<RetainedCloneSource<P,CapturedSnapshotSource<P>>>>())}else{source.next_close_demand(body)};}\n"),
("  if let Some(source)=self.source.as_mut(){if source.terminal_is_empty(){self.source.take();return Ok(unit());}",
 "  if self.source_admission.is_some(){return self.close_admission(grant).map(RetainedCloneStep::Progress);}\n  if let Some(source)=self.source.as_mut(){if source.terminal_is_empty(){self.source.take();return Ok(unit());}"),
("&&self.source_authority.is_none()&&self.input.is_none()","&&self.source_authority.is_none()&&self.source_admission.is_none()&&self.input.is_none()"),
])
