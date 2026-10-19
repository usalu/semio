# Plugin Incoming Source Hunks

### 0 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs
B/H/S/current: 1414/92/1435/100
--- 
+++ 
@@ -9,4 +9,5 @@
 
 use semio_framework_ui_contract::*;
+use store::ErasedSnapshotRetirement;
 #[cfg(test)] use semio_framework_os_kernel::HistoryPageStack;
 use semio_framework::kernel::{HistoryMutationEntry, HistoryMutationMessage, HistoryReprojection, HistoryReprojectionKind, HistoryTimeTravel, HistoryTimeTravelProblem, HistoryTimeTravelReview, HistoryTimeTravelStage, RequestId};
@@ -289,4 +290,6 @@
     /// 📨️ Borrows whether the mounted caller still owns an unconsumed original command receipt.
     pub fn has_command_receipt(&self)->bool{self.command_receipt.is_some()}
+    #[cfg(any(test, feature = "artifact-app-testing"))]
+    pub(crate) fn original_close_diagnostic(&self)->String{format!("closing={} sessionRetired={} sessionOwner={} sessionUntouched={} sessionEmpty={} sessionDemand={:?} commandReceipt={} pendingCommand={} discardCursor={} documentEmpty={} member={} authoring={} patch={} commandRetirementsEmpty={}",self.closing,self.session_retired,self.session_retirement.is_some(),self.session_retirement.as_ref().is_some_and(|owner|owner.original_is_untouched()),self.session_retirement.as_ref().is_none_or(|owner|owner.terminal_is_empty()),self.session_retirement_demands(32768),self.command_receipt.is_some(),self.pending_command.is_some(),self.discard_cursor.is_some(),self.document.terminal_is_empty(),self.member.is_some(),self.authoring.is_some(),self.patch.is_some(),self.command_retirements.terminal_is_empty())+&format!(" documentFrontier aliases={} snapshotClose={} issuers={} issuerClose={} discardedLen={} discardedCap={} discardedPending={} discardedActive={} discardedFactory={} discardedFactoryClose={} effects={} staged={} kind={} preview={} previewJob={} previewInput={} replay={} finished={} head={}",self.document.snapshot_aliases.iter().filter(|slot|slot.is_some()).count(),self.document.snapshot_close.is_some(),self.document.snapshot_issuers.is_some(),self.document.snapshot_issuer_close.iter().filter(|slot|slot.is_some()).count(),self.document.discarded.len(),self.document.discarded.capacity(),self.document.discarded_pending.is_some(),self.document.discarded_active.is_some(),self.document.discarded_factory.is_some(),self.document.discarded_factory_close.is_some(),self.document.discard_effects.is_some(),self.document.staged.is_some(),self.document.kind.is_some(),self.document.preview.is_some(),self.document.preview_job.is_some(),self.document.preview_input.is_some(),self.document.replay.is_some(),self.document.finished.is_some(),self.document.head.is_some())}
 
     /// ✏️ The draft editor while a mutation is being edited.
@@ -625,5 +628,5 @@
 /// its finished result and replayed head, the retirements of the snapshots they displaced (taken from the store at once,
 /// so they retire without it) and the operations a draft discarded.
-pub(crate) struct TimeTravelStoreState<P, Mu: ::protocol::Mutation<P>> {
+pub(crate) struct TimeTravelStoreState<P, Mu: ::protocol::Mutation<P>+Send+'static> {
     discard_effects: Option<Box<discard_decision::TimeTravelDiscardStoreEffects<P,Mu>>>,
     label_of: fn(&Mu) -> LocalizedLabel,
@@ -633,4 +636,12 @@
     preview_job: Option<store::os_store::snapshot_read::ArtifactDerivedHistoryPreview<P, Mu>>,
     preview_input: Option<protocol::InputReplacement>,
+    preview_base:Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<P>>,
+    preview_original:Option<Mu>,
+    preview_source:Option<store::os_store::ArtifactMutationSource<Mu>>,
+    preview_native_input:Option<semio_framework_value::retained_clone::RetainedOwnedProjection<Mu>>,
+    preview_derived:Option<store::os_store::ArtifactDerivedOperation<P,Mu>>,
+    preview_input_close:Option<Box<dyn store::ErasedSnapshotRetirement>>,
+    preview_original_close:Option<Box<dyn store::ErasedSnapshotRetirement>>,
+    preview_cancelled:bool,preview_job_closing:bool,replay_cancelled:bool,
     replay: Option<store::os_store::snapshot_read::ArtifactDerivedReplay<P, Mu>>,
     finished: Option<store::os_store::replay::EditReplayResult<P, Mu>>,
@@ -648,5 +659,5 @@
 }
 
-impl<P,Mu:Mutation<P>> Drop for TimeTravelStoreState<P,Mu>{
+impl<P,Mu:Mutation<P>+Send+'static> Drop for TimeTravelStoreState<P,Mu>{
     fn drop(&mut self){
         let empty=self.snapshot_aliases.iter().all(Option::is_none)&&self.snapshot_close.is_none()&&self.snapshot_issuers.is_none()&&self.snapshot_issuer_close.iter().all(Option::is_none);
@@ -658,5 +669,6 @@
 /// ⏭️ What one replay slice did: progress, a completed report (the head is swapped in), or a fault.
 pub(crate) enum TimeTravelReplayStep {
-    Pending { done: u32, total: u32 },
+    Pending { done: u32, total: u32, ownership:RetainedCloneProgress },
+    Refused(ValueError),
     Completed(protocol::ReplayReport),
     Faulted,
@@ -665,6 +677,8 @@
 /// 🖼️ One preview slice leaves progress or publishes a complete selected-mutation projection.
 pub(crate) enum TimeTravelPreviewStep {
+    Refused(ValueError),
     Pending,
-    Completed(Vec<protocol::MutationMessage>),
+    Progress(RetainedCloneProgress),
+    Completed(Vec<protocol::MutationMessage>,RetainedCloneProgress),
 }
 
@@ -685,8 +699,9 @@
     Rebuild(DslValue),
     Preview { target: MutationId, replacement: protocol::InputReplacement, drafts: protocol::HistoryInputDrafts, deadline_us: u64, clock: fn() -> Option<u64> },
-    StepPreview { deadline_us: u64, clock: fn() -> Option<u64> },
+    StepPreview { deadline_us: u64, clock: fn() -> Option<u64>,grant:RetainedCloneGrant },
     StartReplay { drafts: protocol::HistoryInputDrafts, from: MutationId },
     CancelReplay,
-    StepReplay { deadline_us: u64, clock: fn() -> Option<u64> },
+    CloseOriginalSources { grant: RetainedCloneGrant },
+    StepReplay { deadline_us: u64, clock: fn() -> Option<u64>,grant:RetainedCloneGrant },
     Read(TimeTravelStage),
 }
@@ -699,4 +714,5 @@
     ReplayStarted(bool),
     Stepped(TimeTravelReplayStep),
+    OriginalProgress(RetainedCloneProgress),
     Read(Option<(store::os_store::snapshot_read::ErasedSnapshotRead, [u8; 32])>),
     Done,
@@ -722,7 +738,10 @@
     Mu: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ::protocol::Mutation<P> + ::protocol::OpBinary + ::protocol::OpText + Send + 'static,
 {
+    fn discarded_has_physical_capacity(&self)->bool{size_of::<Mu>()!=0&&self.discarded.capacity()!=0}
+    fn discarded_terminal_is_empty(&self)->bool{self.discarded.is_empty()&&!self.discarded_has_physical_capacity()}
+
     /// 🏗️ Owners at rest, labelling the store's operations with `label_of`.
     fn new(label_of: fn(&Mu) -> LocalizedLabel) -> Self {
-        Self { discard_effects:None,label_of, staged: None, kind: None, preview: None, preview_job: None, preview_input: None, replay: None, finished: None, head: None, snapshot_aliases: ManuallyDrop::new(std::array::from_fn(|_|None)), snapshot_issuers: ManuallyDrop::new(None), snapshot_close: ManuallyDrop::new(None), snapshot_issuer_close: std::array::from_fn(|_|None), retirements: VecDeque::new(), discarded: Vec::new(), discarded_pending: None, discarded_active: None, discarded_factory: None, discarded_factory_close: None }
+        Self { discard_effects:None,label_of, staged: None, kind: None, preview: None, preview_job: None, preview_input: None, preview_base:None,preview_original:None,preview_source:None,preview_native_input:None,preview_derived:None,preview_input_close:None,preview_original_close:None,preview_cancelled:false,preview_job_closing:false,replay_cancelled:false,replay: None, finished: None, head: None, snapshot_aliases: ManuallyDrop::new(std::array::from_fn(|_|None)), snapshot_issuers: ManuallyDrop::new(None), snapshot_close: ManuallyDrop::new(None), snapshot_issuer_close: std::array::from_fn(|_|None), retirements: VecDeque::new(), discarded: Vec::new(), discarded_pending: None, discarded_active: None, discarded_factory: None, discarded_factory_close: None }
     }
 
@@ -760,13 +779,13 @@
 
     fn cancel_preview(&mut self, store: &ArtifactStore<P, Mu>) -> Result<(), Fault> {
-        let grant=history_retirement_frame_grant(ArtifactStore::<P,Mu>::history_read_retirement_birth_bytes());
-        if let Some((retirement,progress)) = store.retire_derived_history_preview(&mut self.preview_job,grant).map_err(ValueError::into_fault)? { assert!(progress.fits(grant));self.retirements.push_back(retirement); }
-        self.preview_input = None;
+        self.preview_cancelled=true;
+        if let Some(owner)=self.preview_derived.as_mut(){owner.cancel();}
+        let base=self.preview_base.take();self.retire(store,base)?;
+        if self.preview_original.is_some()&&self.discarded_factory.is_none(){self.discarded_factory=Some(Arc::clone(store.owned_mutation_retirement_factory().map_err(ValueError::into_fault)?));}
         Ok(())
     }
 
-    fn cancel_replay(&mut self, store: &ArtifactStore<P, Mu>) -> Result<(), Fault> {
-        let grant=history_retirement_frame_grant(ArtifactStore::<P,Mu>::history_read_retirement_birth_bytes());
-        if let Some((retirement,progress)) = store.retire_derived_report_replay(&mut self.replay,grant).map_err(ValueError::into_fault)? { assert!(progress.fits(grant));self.retirements.push_back(retirement); }
+    fn cancel_replay(&mut self, _store: &ArtifactStore<P, Mu>) -> Result<(), Fault> {
+        self.replay_cancelled=true;
         Ok(())
     }
@@ -818,5 +837,5 @@
 
     /// 📨️ Runs one typed command against `store`.
-    pub(crate) async fn run(&mut self, store: &mut ArtifactStore<P, Mu>, command: TimeTravelStoreCommand) -> Result<TimeTravelStoreOutput, Fault> {
+    pub(crate) async fn run(&mut self, store: &mut ArtifactStore<P, Mu>, command: TimeTravelStoreCommand,receiving:Option<&mut semio_framework_value::NativeDecodeControl<'_>>) -> Result<TimeTravelStoreOutput, Fault> {
         Ok(match command {
             TimeTravelStoreCommand::Open { target, current, withdraw } => TimeTravelStoreOutput::Opened(self.open(store, &target, current.as_ref(), withdraw)),
@@ -834,15 +853,17 @@
             TimeTravelStoreCommand::Rebuild(candidate) => TimeTravelStoreOutput::Rebuilt(self.rebuild(store, candidate)),
             TimeTravelStoreCommand::Preview { target, replacement, drafts, deadline_us, clock } => {
+                if self.preview_job.is_some()||self.preview_derived.is_some()||self.preview_original.is_some()||self.preview_source.is_some()||self.preview_native_input.is_some()||self.preview_input_close.is_some()||self.preview_original_close.is_some(){self.cancel_preview(store)?;return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original preview cancellation must settle before another selected input").into_fault());}
                 self.cancel_preview(store)?;
                 self.preview_job = Some(store.begin_derived_history_preview(target, drafts).map_err(|error| error.into_fault())?);
+                self.preview_cancelled=false;
                 self.preview_input = Some(replacement);
-                TimeTravelStoreOutput::Previewed(self.step_preview(store, deadline_us, clock)?)
+                TimeTravelStoreOutput::Previewed(TimeTravelPreviewStep::Pending)
             }
-            TimeTravelStoreCommand::StepPreview { deadline_us, clock } => TimeTravelStoreOutput::Previewed(self.step_preview(store, deadline_us, clock)?),
+            TimeTravelStoreCommand::StepPreview { deadline_us, clock,grant } => TimeTravelStoreOutput::Previewed(self.step_preview(store, deadline_us, clock,grant,receiving.ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"history preview requires its original Job receiving issuer").into_fault())?)?),
             TimeTravelStoreCommand::StartReplay { drafts, from } => {
                 self.cancel_replay(store)?;
                 match store.begin_derived_report_replay(drafts, Some(from)) {
                     Ok(replay) => {
-                        self.replay = Some(replay);
+                        self.replay = Some(replay);self.replay_cancelled=false;
                         TimeTravelStoreOutput::ReplayStarted(true)
                     }
@@ -850,9 +871,10 @@
                 }
             }
+            TimeTravelStoreCommand::CloseOriginalSources { grant } => TimeTravelStoreOutput::OriginalProgress(self.close_original_store_sources(store,grant).map_err(ValueError::into_fault)?),
             TimeTravelStoreCommand::CancelReplay => {
                 self.cancel_replay(store)?;
                 TimeTravelStoreOutput::Done
             }
-            TimeTravelStoreCommand::StepReplay { deadline_us, clock } => TimeTravelStoreOutput::Stepped(self.step_replay(store, deadline_us, clock)?),
+            TimeTravelStoreCommand::StepReplay { deadline_us, clock,grant } => TimeTravelStoreOutput::Stepped(self.step_replay(store, deadline_us, clock,grant,receiving.ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"history replay requires its original Job receiving issuer").into_fault())?)?),
             TimeTravelStoreCommand::Read(stage) => TimeTravelStoreOutput::Read(match self.shown(stage) {
                 Some(shown) => Some((store.snapshot_read_derived(shown).map_err(|error| error.into_fault())?, store.content_revision())),
@@ -961,43 +983,42 @@
     }
 
+    /// ♻️ Returns genuine history loans to the same Store ledger under original incoming cleanup grants.
+    fn close_original_store_sources(&mut self,store:&mut ArtifactStore<P,Mu>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
+      let preview=(self.preview_cancelled||self.preview_job_closing)&&self.preview_job.is_some();let replay=self.replay_cancelled&&self.replay.is_some();if !preview&&!replay{return Ok(Default::default())}
+      if self.retirements.len()==self.retirements.capacity(){let capacity=(self.retirements.len()+1)*size_of::<Box<dyn ErasedSnapshotRetirement>>();let released=self.retirements.capacity()*size_of::<Box<dyn ErasedSnapshotRetirement>>();if grant.maximum_items==0||grant.maximum_capacity_bytes<capacity||grant.maximum_release_bytes<released||grant.maximum_depth==0{return Ok(Default::default())}self.retirements.try_reserve_exact(1).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"history cleanup queue allocation refused"))?;return Ok(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:self.retirements.capacity()*size_of::<Box<dyn ErasedSnapshotRetirement>>(),released_bytes:released,..Default::default()})}
+      if preview{if let Some((owner,p))=store.retire_derived_history_preview(&mut self.preview_job,grant)?{self.retirements.push_back(owner);return Ok(p)}return Ok(self.preview_job.as_mut().map_or(Default::default(),|owner|owner.take_preparation_receipt()))}
+      if let Some((owner,p))=store.retire_derived_report_replay(&mut self.replay,grant)?{self.retirements.push_back(owner);return Ok(p)}Ok(self.replay.as_mut().map_or(Default::default(),|owner|owner.take_preparation_receipt()))
+    }
     /// 🖼️ One bounded prefix slice; only a completed preview displaces the last complete projection.
-    fn step_preview(&mut self, store: &mut ArtifactStore<P, Mu>, deadline_us: u64, clock: fn() -> Option<u64>) -> Result<TimeTravelPreviewStep, Fault> {
+    fn step_preview(&mut self,store:&mut ArtifactStore<P,Mu>,deadline_us:u64,clock:fn()->Option<u64>,grant:RetainedCloneGrant,receiving:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<TimeTravelPreviewStep,Fault>{
         self.snapshot_custody_available(2)?;
-        let Some(job) = self.preview_job.as_mut() else { return Ok(TimeTravelPreviewStep::Pending) };
-        let mut deadline = || clock().is_none_or(|now| now >= deadline_us);
-        loop {
-            let grant=history_planning_retirement_grant(job.planning_retirement_demands(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).map_err(ValueError::into_fault)?);
-            if matches!(store.step_derived_history_preview(job, grant, &mut deadline).map_err(|error| error.into_fault())?, store::os_store::replay::ReplayStep::Finished(_)) { break; }
-            if deadline() { return Ok(TimeTravelPreviewStep::Pending); }
-        }
-        let base = store.finish_derived_history_preview(&mut self.preview_job).map_err(|error| error.into_fault())?;
-        let replacement = self.preview_input.take().ok_or_else(|| Fault::new(FaultOrigin::Plugin, FaultCode::new("timeTravel.preview-input-absent"), "history preview lost its selected input"))?;
-        self.cancel_preview(store)?;
-        let (preview, outcome) = match replacement {
-            protocol::InputReplacement::Withdrawn => (base, Vec::new()),
-            protocol::InputReplacement::Input { payload, .. } => {
-                let op = <Mu as ::protocol::OpBinary>::decode_op(&payload).map_err(|error| error.into_fault())?;
-                let derived = store.derive_history_snapshot(&base, &op);
-                self.discard(Some(op));
-                let (next, messages) = derived.map_err(|error| error.into_fault())?;
-                match next {
-                    Some(next) => { self.retire(store, Some(base))?; (next, messages) }
-                    None => (base, messages),
-                }
-            }
-        };
-        let previous = self.preview.replace(preview);
-        self.retire(store, previous)?;
-        Ok(TimeTravelPreviewStep::Completed(outcome))
+        let idle=RetainedCloneProgress::default();let mut deadline=||clock().is_none_or(|now|now>=deadline_us);
+        if self.preview_cancelled{return Ok(TimeTravelPreviewStep::Pending);}
+        if self.preview_base.is_none(){let Some(job)=self.preview_job.as_mut()else{return Ok(TimeTravelPreviewStep::Pending)};
+            let step=match store.step_derived_history_preview(job,grant,receiving,&mut deadline){Ok(step)=>step,Err(vcs::VcsError::InverseRefused(error))=>return Ok(TimeTravelPreviewStep::Refused(error)),Err(error)=>return Err(error.into_fault())};if !matches!(step,store::os_store::replay::ReplayStep::Finished(_)){return Ok(TimeTravelPreviewStep::Progress(job.take_preparation_receipt()));}
+            if self.snapshot_issuers.is_none(){*self.snapshot_issuers=Some((Arc::clone(store.owned_snapshot_retirement_factory().map_err(ValueError::into_fault)?),Arc::clone(store.owned_mutation_retirement_factory().map_err(ValueError::into_fault)?)));}
+            self.preview_base=Some(store.finish_derived_history_preview(&mut self.preview_job).map_err(FaultFrom::into_fault)?);
+            self.preview_job_closing=true;return Ok(TimeTravelPreviewStep::Progress(idle));
+        }
+        let Some(replacement)=self.preview_input.as_ref()else{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"selected preview retains its original replacement").into_fault())};
+        if matches!(replacement,protocol::InputReplacement::Withdrawn){let copy=size_of::<Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<P>>>();if grant.maximum_items==0||grant.maximum_copy_bytes<copy{return Ok(TimeTravelPreviewStep::Pending);}let next=self.preview_base.take().unwrap();let previous=self.preview.replace(next);self.retire(store,previous)?;self.preview_cancelled=true;return Ok(TimeTravelPreviewStep::Completed(Vec::new(),RetainedCloneProgress{copied_items:1,copied_bytes:copy,..idle}));}
+        if self.preview_original.is_none()&&self.preview_source.is_none()&&self.preview_derived.is_none(){let protocol::InputReplacement::Input{payload,..}=replacement else{unreachable!()};if grant.maximum_items==0||grant.maximum_copy_bytes<payload.len()||payload.len()>4096{return Ok(TimeTravelPreviewStep::Pending);}if self.discarded_factory.is_none(){self.discarded_factory=Some(Arc::clone(store.owned_mutation_retirement_factory().map_err(ValueError::into_fault)?));}self.preview_original=Some(<Mu as ::protocol::OpBinary>::decode_op(payload).map_err(FaultFrom::into_fault)?);return Ok(TimeTravelPreviewStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:payload.len(),..idle}));}
+        if self.preview_derived.is_none(){
+            if self.preview_source.is_none(){let factory=store.owned_mutation_retirement_factory().map_err(ValueError::into_fault)?;let Some((source,p))=store::os_store::ArtifactMutationSource::<Mu>::admit(&mut self.preview_original,factory,grant).map_err(ValueError::into_fault)?else{return Ok(TimeTravelPreviewStep::Pending)};self.preview_source=Some(source);return Ok(TimeTravelPreviewStep::Progress(p));}
+            if self.preview_native_input.is_none(){let source=self.preview_source.as_mut().unwrap();let p=if source.ready(){source.take(&mut self.preview_native_input,grant).map_err(ValueError::into_fault)?.unwrap_or(idle)}else{source.advance(grant).map_err(ValueError::into_fault)?.progress()};return Ok(TimeTravelPreviewStep::Progress(p));}
+        }
+        if self.preview_derived.is_none(){let p=store.begin_derived_operation(&mut self.preview_derived,self.preview_base.as_ref().unwrap(),&mut self.preview_native_input,0,store::os_store::ArtifactDerivedOperationMode::History,grant).map_err(ValueError::into_fault)?.unwrap_or(idle);return Ok(TimeTravelPreviewStep::Progress(p));}
+        let owner=self.preview_derived.as_mut().unwrap();
+        if owner.is_finished(){let Some(((next,messages),p))=store.take_derived_operation(owner,grant).map_err(ValueError::into_fault)?else{return Ok(TimeTravelPreviewStep::Pending)};let base=self.preview_base.take().unwrap();let result=if let Some(next)=next{self.retire(store,Some(base))?;next}else{base};let previous=self.preview.replace(result);self.retire(store,previous)?;self.preview_cancelled=true;if self.discarded_factory.is_none(){self.discarded_factory=Some(Arc::clone(store.owned_mutation_retirement_factory().map_err(ValueError::into_fault)?));}return Ok(TimeTravelPreviewStep::Completed(messages,p));}
+        Ok(match store.step_derived_operation(owner,grant.maximum_copy_bytes,grant,receiving,&mut deadline){Ok(step)=>TimeTravelPreviewStep::Progress(step.progress()),Err(error)=>TimeTravelPreviewStep::Refused(error)})
     }
 
     /// ⏭️ Steps the replay until `deadline_us` on `clock`; completion swaps the replayed head in and keeps the finished result.
-    fn step_replay(&mut self, store: &ArtifactStore<P, Mu>, deadline_us: u64, clock: fn() -> Option<u64>) -> Result<TimeTravelReplayStep, Fault> {
+    fn step_replay(&mut self, store: &mut ArtifactStore<P, Mu>, deadline_us: u64, clock: fn() -> Option<u64>,grant:RetainedCloneGrant,receiving:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<TimeTravelReplayStep, Fault> {
         self.snapshot_custody_available(1)?;
         let Some(replay) = self.replay.as_mut() else { return Ok(TimeTravelReplayStep::Faulted) };
         let mut deadline = || clock().is_none_or(|now| now >= deadline_us);
-        let grant=history_planning_retirement_grant(replay.planning_retirement_demands(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).map_err(ValueError::into_fault)?);
-        Ok(match store.step_derived_report_replay(replay, grant, &mut deadline) {
-            Ok(store::os_store::replay::ReplayStep::Pending(progress)) => TimeTravelReplayStep::Pending { done: progress.done, total: progress.total },
+        Ok(match store.step_derived_report_replay(replay, grant, receiving,&mut deadline) {
+            Ok(store::os_store::replay::ReplayStep::Pending(progress)) => TimeTravelReplayStep::Pending { done: progress.done, total: progress.total,ownership:replay.take_preparation_receipt() },
             Ok(store::os_store::replay::ReplayStep::Finished(_)) => {
                 match store.finish_derived_report_replay(&mut self.replay).and_then(|(result, head)| store.replay_report(&result).map(|report| (result, head, report))) {
@@ -1030,5 +1051,5 @@
 
     fn preview_progress(&self) -> Option<store::os_store::replay::ReplayProgress> {
-        self.preview_job.as_ref().map(store::os_store::snapshot_read::ArtifactDerivedHistoryPreview::progress)
+        self.preview_job.as_ref().map(store::os_store::snapshot_read::ArtifactDerivedHistoryPreview::progress).or_else(||self.preview_derived.as_ref().filter(|owner|!owner.is_closing()).map(|owner|store::os_store::replay::ReplayProgress{done:u32::try_from(owner.progress().items).unwrap_or(u32::MAX),total:0}))
     }
 
@@ -1038,9 +1059,9 @@
 
     fn has_pending_work(&self) -> bool {
-        self.snapshot_aliases.iter().any(Option::is_some) || self.snapshot_close.is_some() || self.snapshot_issuers.is_some() || self.snapshot_issuer_close.iter().any(Option::is_some) || self.discard_effects.as_ref().is_some_and(|owner|owner.closing) || self.preview_job.is_some() || self.retirements.capacity()!=0 || self.discarded.capacity()!=0 || self.discarded_pending.is_some() || self.discarded_active.is_some() || self.discarded_factory.is_some() || self.discarded_factory_close.is_some()
+        self.snapshot_aliases.iter().any(Option::is_some) || self.snapshot_close.is_some() || self.snapshot_issuers.is_some() || self.snapshot_issuer_close.iter().any(Option::is_some) || self.discard_effects.as_ref().is_some_and(|owner|owner.closing) || self.preview_job.is_some()||self.preview_derived.as_ref().is_some_and(|owner|owner.is_closing())||self.preview_cancelled&&(self.preview_original.is_some()||self.preview_input.is_some()||self.preview_source.is_some()||self.preview_native_input.is_some())||self.preview_input_close.is_some()||self.preview_original_close.is_some() || self.retirements.capacity()!=0 || !self.discarded.is_empty() || self.discarded_has_physical_capacity() || self.discarded_pending.is_some() || self.discarded_active.is_some() || self.discarded_factory.is_some() || self.discarded_factory_close.is_some()
     }
 
     fn terminal_is_empty(&self) -> bool {
-        self.snapshot_aliases.iter().all(Option::is_none) && self.snapshot_close.is_none() && self.snapshot_issuers.is_none() && self.snapshot_issuer_close.iter().all(Option::is_none) && self.discard_effects.is_none() && self.staged.is_none() && self.kind.is_none() && self.preview.is_none() && self.preview_job.is_none() && self.preview_input.is_none() && self.replay.is_none() && self.finished.is_none() && self.head.is_none() && self.retirements.capacity()==0 && self.discarded.capacity()==0 && self.discarded_pending.is_none() && self.discarded_active.is_none() && self.discarded_factory.is_none() && self.discarded_factory_close.is_none()
+        self.snapshot_aliases.iter().all(Option::is_none) && self.snapshot_close.is_none() && self.snapshot_issuers.is_none() && self.snapshot_issuer_close.iter().all(Option::is_none) && self.discard_effects.is_none() && self.staged.is_none() && self.kind.is_none() && self.preview.is_none() && self.preview_job.is_none() && self.preview_input.is_none()&&self.preview_base.is_none()&&self.preview_original.is_none()&&self.preview_source.is_none()&&self.preview_native_input.is_none()&&self.preview_derived.is_none()&&self.preview_input_close.is_none()&&self.preview_original_close.is_none() && self.replay.is_none() && self.finished.is_none() && self.head.is_none() && self.retirements.capacity()==0 && self.discarded_terminal_is_empty() && self.discarded_pending.is_none() && self.discarded_active.is_none() && self.discarded_factory.is_none() && self.discarded_factory_close.is_none()
     }
 
@@ -1061,10 +1082,10 @@
 /// ✍️ One typed history-edit command against a composed member's store (design §12): creates the member's owners on the
 /// first visit, then runs the command on them.
-pub(crate) struct TimeTravelMemberRun<'a> {
+pub(crate) struct TimeTravelMemberRun<'a,'c> {
     pub owners: &'a mut Option<Box<dyn TimeTravelOwners>>,
-    pub command: TimeTravelStoreCommand,
-}
-
-impl store::MemberStoreVisitorMut for TimeTravelMemberRun<'_> {
+    pub command: TimeTravelStoreCommand,pub receiving:Option<&'a mut semio_framework_value::NativeDecodeControl<'c>>,
+}
+
+impl store::MemberStoreVisitorMut for TimeTravelMemberRun<'_,'_> {
     type Output = Result<TimeTravelStoreOutput, Fault>;
 
@@ -1079,5 +1100,5 @@
         let owners = self.owners.get_or_insert_with(|| Box::new(TimeTravelStoreState::<P, Mu>::new(|op| protocol::SemanticMutation::<P>::label(op))) as Box<dyn TimeTravelOwners>);
         let state = owners.as_any_mut().downcast_mut::<TimeTravelStoreState<P, Mu>>().ok_or_else(|| Fault::new(FaultOrigin::Plugin, FaultCode::new("timeTravel.member-store-kind"), "a composed member's history-edit owners belong to another store kind"))?;
-        state.run(store, self.command).await
+        state.run(store, self.command,self.receiving).await
     }
 }


### 1 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🧵️instance/🦀️.rs
B/H/S/current: 1220/81/1230/81
--- 
+++ 
@@ -35,13 +35,13 @@
     /// 📨️ Runs one typed command on the store the session edits: the document's own, or the composed member's through
     /// its typed visitor (design §12).
-    async fn time_travel_run(&mut self, command: TimeTravelStoreCommand) -> Result<TimeTravelStoreOutput, Fault> {
+    async fn time_travel_run(&mut self, command: TimeTravelStoreCommand,receiving:Option<&mut semio_framework_value::NativeDecodeControl<'_>>) -> Result<TimeTravelStoreOutput, Fault> {
         let TimeTravelCtx { stores, time_travel, child_composition, .. } = self;
         let (store, children) = (&mut stores.store, &mut child_composition.children);
         match time_travel.member.as_mut() {
-            None => time_travel.document.run(store, command).await,
+            None => time_travel.document.run(store, command,receiving).await,
             Some(member) => {
                 let key = member.key.clone();
                 let entry = children.member_mut(key.borrowed()).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new(TIME_TRAVEL_MEMBER_GONE_CODE), format!("the composed member {} a history edit targets is gone", member.store())))?;
-                entry.member.visit_member_mut(TimeTravelMemberRun { owners: &mut member.owners, command }).await
+                entry.member.visit_member_mut(TimeTravelMemberRun { owners: &mut member.owners, command,receiving }).await
             }
         }
@@ -90,5 +90,5 @@
         let (key, dialect) = (member.key.clone(), member.dialect.clone());
         let stage = self.time_travel.session.stage;
-        let next = match self.time_travel_run(TimeTravelStoreCommand::Read(stage)).await? {
+        let next = match self.time_travel_run(TimeTravelStoreCommand::Read(stage),None).await? {
             TimeTravelStoreOutput::Read(Some((read, revision))) => Some(self.child_composition.child_content_root.with_member_read(key.borrowed(), &dialect, revision, read)?),
             _ => None,
@@ -131,6 +131,11 @@
         if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
         if self.time_travel.has_command_receipt(){return Ok(RetainedCloneStep::Progress(Default::default()))}
-        self.time_travel.begin_close();
-        self.time_travel.document.cancel_prepared_command();if let Some(owners)=self.time_travel.member.as_mut().and_then(|member|member.owners.as_deref_mut()){owners.cancel_prepared_command()}
+        if !self.time_travel.closing{
+            let demand=self.time_travel_close_demands(grant.maximum_copy_bytes).map_err(ValueError::into_fault)?;
+            if !protocol::retained_mutation_demand_fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()))}
+            self.time_travel.begin_close();
+            self.time_travel.document.cancel_prepared_command();if let Some(owners)=self.time_travel.member.as_mut().and_then(|member|member.owners.as_deref_mut()){owners.cancel_prepared_command()}
+            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
+        }
         if self.drive_original_discard_command(grant)?{let (original,receipt)=self.time_travel.take_command_receipt().expect("close owns its original command turn receipt");assert_eq!(original,grant);return Ok(RetainedCloneStep::Progress(receipt))}
         if self.drive_original_review_children(grant)?{let (original,receipt)=self.time_travel.take_command_receipt().expect("close owns its original review view turn receipt");assert_eq!(original,grant);return Ok(RetainedCloneStep::Progress(receipt))}
@@ -410,5 +415,5 @@
                 TimeTravelEffect::StartReplay { drafts, from } => queue.extend(self.start_time_travel_replay(drafts, &from).await?),
                 TimeTravelEffect::CancelReplay => {
-                    self.time_travel_run(TimeTravelStoreCommand::CancelReplay).await?;
+                    self.time_travel_run(TimeTravelStoreCommand::CancelReplay,None).await?;
                 }
                 TimeTravelEffect::OpenFinalizePrompt => {
@@ -433,12 +438,15 @@
         let drafts = self.time_travel.accepted_drafts();
         let deadline_us = self.shared().time_travel_turn_deadline();
-        let TimeTravelStoreOutput::Previewed(step) = self.time_travel_run(TimeTravelStoreCommand::Preview { target: target.clone(), replacement: replacement.clone(), drafts, deadline_us, clock: self.time_travel.turn_clock }).await? else {
+        let TimeTravelStoreOutput::Previewed(step) = self.time_travel_run(TimeTravelStoreCommand::Preview { target: target.clone(), replacement: replacement.clone(), drafts, deadline_us, clock: self.time_travel.turn_clock },None).await? else {
             return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("timeTravel.preview-mismatch"), "a history-edit preview answered another command"));
         };
-        self.adopt_time_travel_preview_step(step).await
-    }
-
-    async fn adopt_time_travel_preview_step(&mut self, step: TimeTravelPreviewStep) -> Result<(), Fault> {
-        if let TimeTravelPreviewStep::Completed(outcome) = step {
+        self.adopt_time_travel_preview_step(step,None).await
+    }
+
+    async fn adopt_time_travel_preview_step(&mut self, step: TimeTravelPreviewStep,grant:Option<RetainedCloneGrant>) -> Result<(), Fault> {
+        let progress=match &step{TimeTravelPreviewStep::Progress(p)|TimeTravelPreviewStep::Completed(_,p)=>*p,TimeTravelPreviewStep::Pending=>Default::default(),TimeTravelPreviewStep::Refused(error)=>error.retained_progress()};
+        if let Some(grant)=grant{if !progress.fits(grant){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"preview producer exceeds its original turn grant").into_fault());}if progress!=RetainedCloneProgress::default(){if self.time_travel.command_receipt.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original preview receipt awaits its mounted parent").into_fault());}self.time_travel.command_receipt=Some((grant,progress));}}
+        if let TimeTravelPreviewStep::Refused(error)=step{return Err(error.into_fault())}
+        if let TimeTravelPreviewStep::Completed(outcome,_) = step {
             if let Some(editor) = self.time_travel.editor.as_mut() { editor.outcome = outcome; }
             self.time_travel.document_dirty = true;
@@ -455,5 +463,5 @@
         let drafts: protocol::HistoryInputDrafts = drafts.into_iter().map(|draft| (draft.target, draft.replacement)).collect();
         let generation = self.time_travel.session.generation;
-        Ok(match self.time_travel_run(TimeTravelStoreCommand::StartReplay { drafts, from: from.clone() }).await? {
+        Ok(match self.time_travel_run(TimeTravelStoreCommand::StartReplay { drafts, from: from.clone() },None).await? {
             TimeTravelStoreOutput::ReplayStarted(true) => Vec::new(),
             _ => self.time_travel.session.apply(TimeTravelEvent::ReplayFaulted { generation, code: TIME_TRAVEL_REPLAY_FAULTED_CODE.to_string() }).unwrap_or_default(),
@@ -548,5 +556,5 @@
         let target = MutationId(mutation.to_string());
         let current = if withdraw { Some(protocol::InputReplacement::Withdrawn) } else { self.time_travel.session.accepted_draft(&target).map(|draft| draft.replacement.clone()) };
-        let opened = match self.time_travel_run(TimeTravelStoreCommand::Open { target: target.clone(), current, withdraw }).await? {
+        let opened = match self.time_travel_run(TimeTravelStoreCommand::Open { target: target.clone(), current, withdraw },None).await? {
             TimeTravelStoreOutput::Opened(opened) => opened,
             _ => Err(TimeTravelActionRefusal::UnknownMutation),
@@ -565,5 +573,5 @@
         match self.time_travel.session.apply(event) {
             Ok(session_effects) => {
-                self.time_travel_run(TimeTravelStoreCommand::Adopt(true)).await?;
+                self.time_travel_run(TimeTravelStoreCommand::Adopt(true),None).await?;
                 self.time_travel.replace_editor(Some(editor));
                 self.note_time_travel_changed(true, true);
@@ -575,5 +583,5 @@
             }
             Err(refusal) => {
-                self.time_travel_run(TimeTravelStoreCommand::Adopt(false)).await?;
+                self.time_travel_run(TimeTravelStoreCommand::Adopt(false),None).await?;
                 if opening {
                     self.settle_time_travel_owners()?;
@@ -680,5 +688,5 @@
             .and_then(|instance| editor.validator.as_ref().map_err(String::clone).and_then(|validator| validator.validate_json(&time_travel_json(&instance)).map(|_| ()).map_err(|error| error.to_string())));
         let rebuilt = match verdict {
-            Ok(()) => match self.time_travel_run(TimeTravelStoreCommand::Rebuild(candidate.clone())).await? {
+            Ok(()) => match self.time_travel_run(TimeTravelStoreCommand::Rebuild(candidate.clone()),None).await? {
                 TimeTravelStoreOutput::Rebuilt(rebuilt) => rebuilt,
                 _ => Err("a history-edit rebuild answered another command".to_string()),
@@ -726,4 +734,5 @@
     pub(crate) async fn drive_time_travel_turn(&mut self, grant: RetainedCloneGrant, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
         if grant.maximum_items==0{return Ok(());}
+        if let TimeTravelStoreOutput::OriginalProgress(progress)=self.time_travel_run(TimeTravelStoreCommand::CloseOriginalSources{grant},None).await?{if progress!=RetainedCloneProgress::default(){assert!(self.time_travel.command_receipt.is_none());self.time_travel.command_receipt=Some((grant,progress));return Ok(())}}
         if self.drive_original_discard_command(grant)?{return Ok(())}
         if self.drive_original_review_children(grant)?{return Ok(())}
@@ -739,12 +748,12 @@
         let moved = self.watch_time_travel_base(identity).await?;
         if !moved && self.time_travel.session.stage == TimeTravelStage::Editing && self.time_travel.preview_progress().is_some() {
-            if let TimeTravelStoreOutput::Previewed(step) = self.time_travel_run(TimeTravelStoreCommand::StepPreview { deadline_us, clock: self.time_travel.turn_clock }).await? {
-                self.adopt_time_travel_preview_step(step).await?;
+            if let TimeTravelStoreOutput::Previewed(step) = self.time_travel_run(TimeTravelStoreCommand::StepPreview { deadline_us, clock: self.time_travel.turn_clock,grant },Some(receiving)).await? {
+                self.adopt_time_travel_preview_step(step,Some(grant)).await?;
             }
         }
         if !moved && self.time_travel.session.stage == TimeTravelStage::Replaying && self.time_travel.replaying() {
-            self.step_time_travel_replay(deadline_us, identity).await?;
-        }
-        if self.time_travel.authoring.is_some() && self.step_supersede_authoring(deadline_us, identity).await? != SupersedeAuthored::Pending {
+            self.step_time_travel_replay(grant,deadline_us, identity,receiving).await?;
+        }
+        if self.time_travel.authoring.is_some() && self.step_supersede_authoring(grant,deadline_us, identity,receiving).await? != SupersedeAuthored::Pending {
             self.note_time_travel_changed(true, true);
         }
@@ -820,12 +829,13 @@
     /// at most every [`TIME_TRAVEL_PROGRESS_REFRESH_MS`]; completion hands the report to the session and swaps the
     /// preview to the replayed head; a store refusal faults the replay.
-    async fn step_time_travel_replay(&mut self, deadline_us: u64, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
+    async fn step_time_travel_replay(&mut self,grant:RetainedCloneGrant, deadline_us: u64, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,receiving:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<(), Fault> {
         let generation = self.time_travel.session.generation;
-        let stepped = match self.time_travel_run(TimeTravelStoreCommand::StepReplay { deadline_us, clock: self.time_travel.turn_clock }).await? {
+        let stepped = match self.time_travel_run(TimeTravelStoreCommand::StepReplay { deadline_us, clock: self.time_travel.turn_clock,grant },Some(receiving)).await? {
             TimeTravelStoreOutput::Stepped(stepped) => stepped,
             _ => TimeTravelReplayStep::Faulted,
         };
         let event = match stepped {
-            TimeTravelReplayStep::Pending { done, total } => {
+            TimeTravelReplayStep::Pending { done, total,ownership } => {
+                if ownership!=RetainedCloneProgress::default(){assert!(self.time_travel.command_receipt.is_none());self.time_travel.command_receipt=Some((grant,ownership));}
                 let _ = self.time_travel.session.apply(TimeTravelEvent::ReplayProgressed { generation, done, total });
                 let now_ms = semio_framework_job::default_now_ms().unwrap_or(0);
@@ -839,4 +849,5 @@
             }
             TimeTravelReplayStep::Completed(report) => TimeTravelEvent::ReplayCompleted { generation, report },
+            TimeTravelReplayStep::Refused(error)=>{let progress=error.retained_progress();if progress!=RetainedCloneProgress::default(){assert!(self.time_travel.command_receipt.is_none());self.time_travel.command_receipt=Some((grant,progress));}return Err(error.into_fault())},
             TimeTravelReplayStep::Faulted => TimeTravelEvent::ReplayFaulted { generation, code: TIME_TRAVEL_REPLAY_FAULTED_CODE.to_string() },
         };
@@ -875,5 +886,5 @@
         if member.review_children.is_some()||member.children.is_some(){return Ok(Some(RetirementDemand{copy_bytes:size_of::<TimeTravelReviewChildren>()+size_of::<ChildContentRetirement>()+size_of::<u64>(),depth:1,..Default::default()}))}
         if let Some(owners)=member.owners.as_ref(){if !owners.terminal_is_empty(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"inactive original member retains live typed sources"))}return Ok(Some(RetirementDemand{copy_bytes:size_of_val(&member.owners),release_bytes:size_of_val(owners.as_ref()),depth:1,..Default::default()}))}
-        if let Some(owner)=member.metadata_close.as_ref(){return if owner.terminal_is_empty(){Ok(Some(RetirementDemand{copy_bytes:size_of_val(&member.metadata_close),depth:1,..Default::default()}))}else{Ok(Some(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original member metadata depth overflow"))?}))}}
+        if let Some(owner)=member.metadata_close.as_ref(){return if owner.terminal_is_empty(){Ok(Some(RetirementDemand{copy_bytes:size_of_val(&member.metadata_close),depth:1,..Default::default()}))}else{Ok(Some({let mut demand=owner.next_demand(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original member metadata depth overflow"))?;demand}))}}
         Ok(Some(RetirementDemand{copy_bytes:if member.metadata_retired{size_of::<TimeTravelMemberSubject>()}else{size_of::<TimeTravelMemberMetadata>()},depth:1,..Default::default()}))
     }
@@ -1045,10 +1056,10 @@
             Err(error) => return Err(error.into_fault()),
         }
-        self.step_supersede_authoring(self.shared().time_travel_turn_deadline(), identity).await
+        Ok(SupersedeAuthored::Pending)
     }
 
     /// ⏭️ One turn of the undo or redo of a finalize being authored: steps its replay until the turn deadline, then commits
     /// the finished replay in its scope; a store that moved meanwhile replays the same inputs again from the new base.
-    async fn step_supersede_authoring(&mut self, deadline_us: u64, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<SupersedeAuthored, Fault> {
+    async fn step_supersede_authoring(&mut self,grant:RetainedCloneGrant, deadline_us: u64, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,receiving:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<SupersedeAuthored, Fault> {
         let stepped = {
             let TimeTravelCtx { stores, time_travel, .. } = self;
@@ -1057,6 +1068,5 @@
             let Some(authoring) = time_travel.authoring.as_mut() else { return Ok(SupersedeAuthored::Refused) };
             let mut deadline = || clock().is_none_or(|now| now >= deadline_us);
-            let grant=history_planning_retirement_grant(authoring.replay.planning_retirement_demands(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).map_err(ValueError::into_fault)?);
-            store.step_derived_report_replay(&mut authoring.replay, grant, &mut deadline)
+            store.step_derived_report_replay(&mut authoring.replay, grant,receiving, &mut deadline)
         };
         match stepped {
@@ -1210,5 +1220,5 @@
             return Ok(false);
         }
-        let grant = crate::plugin_runtime::original_plugin_turn_grant(ctx.mounted_policy.maintenance, *retained);
+        let grant = semio_framework_plugin_ownership::bounded_ownership::original_plugin_turn_grant(ctx.mounted_policy.maintenance, *retained);
         ctx.drive_time_travel_turn(grant, identity, cx).await?;
         Ok(ctx.time_travel.has_command_receipt())


### 2 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🎞️tick/📦️payload/🧪️tests/🟦️.ts
B/H/S/current: 18/17/11/17
--- 
+++ 
@@ -1,18 +1,11 @@
-/** 📨️ Original OS payload source laws retain the declared Toy caller authority. */
-import {test,expect} from "bun:test";
-import {Database} from "bun:sqlite";
-import {readFileSync,existsSync} from "node:fs";
-
-test("original tick payload uses the same declared Toy caller policy and paid borrowed context",()=>{
- const policy=JSON.parse(readFileSync(new URL("../../../../🧪️tests/🔬️plugin-runtime-runtime-close-budget/🧫️fixtures/🧾️fixture-caller/🔣️.json",import.meta.url),"utf8"));
- expect(policy.native.grant).toEqual([1,4096,65536,262144,64]);
- const db=new Database(":memory:");try{
-  db.exec("CREATE TABLE policy(position INTEGER PRIMARY KEY,value INTEGER)");
-  for(const[i,value]of policy.native.grant.entries())db.query("INSERT INTO policy VALUES(?,?)").run(i,value);
-  expect(db.query("SELECT value FROM policy ORDER BY position").all().map(row=>(row as {value:number}).value)).toEqual(policy.native.grant);
- }finally{db.close();}
- const path=new URL("../🦀️.rs",import.meta.url);expect(existsSync(path)).toBe(true);const source=readFileSync(path,"utf8");
- for(const token of["advance_original_tick_payload","append_original","consume_retained","acknowledge_chunk","retained_grant"])expect(source).toContain(token);
- for(const token of["payload_from_bytes","Arc::new","StepContext::new",".finish()",".encode()","to_vec()"])expect(source).not.toContain(token);
- console.log("[DEBUG] Actual OS payload caller grants and complete source laws agree with independent SQLite");
+/** 🎞️ Immutable tick payload byte ranges agree with an independent SQLite BLOB oracle. */
+import{test,expect}from"bun:test";
+import{Database}from"bun:sqlite";
+import Ajv from"ajv";
+import{readFileSync}from"node:fs";
+const read=(path:string)=>JSON.parse(readFileSync(new URL(path,import.meta.url),"utf8"));
+test("writer and native tick sources preserve original bytes across bounded range relay",()=>{
+ const law=read("../🧫️fixtures/🔣️.json");expect(new Ajv({strict:true}).compile(read("../🧬️schema/🔣️.json"))(law)).toBe(true);
+ const db=new Database(":memory:");try{db.exec("CREATE TABLE original(mode TEXT PRIMARY KEY,payload BLOB)");for(const mode of law.sources){const original=Uint8Array.from({length:law.payloadBytes},()=>law.byte);db.query("INSERT INTO original VALUES(?,?)").run(mode,original);for(const cut of law.cancelCuts){const maximum=cut==="complete"?Infinity:cut,pages:Uint8Array[]=[];for(let offset=0;offset<original.length&&pages.length<maximum;offset+=64){const actual=original.slice(offset,Math.min(offset+64,original.length)),oracle=(db.query("SELECT substr(payload,?,64) AS bytes FROM original WHERE mode=?").get(offset+1,mode)as{bytes:Uint8Array}).bytes;expect(Buffer.from(actual)).toEqual(Buffer.from(oracle));pages.push(actual);}expect((db.query("SELECT payload FROM original WHERE mode=?").get(mode)as{payload:Uint8Array}).payload).toEqual(original);if(cut==="complete")expect(Buffer.concat(pages)).toEqual(Buffer.from(original));}}
+ }finally{db.close();}console.log("[DEBUG] strict Ajv/SQLite writer and borrowed native tick sources preserve every original64-byte range, cancellation prefix and full payload");
 });


### 3 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🎞️tick/📦️payload/🧪️tests/🦀️.rs
B/H/S/current: 17/17/25/25
--- 
+++ 
@@ -1,17 +1,25 @@
 //! 🧪️ Original product tick payload keeps caller authority, source identity and physical closure.
 use super::{advance_original_tick_payload,close_original_tick_payload};
-use semio_framework_tool_run::{ToolRunId,ToolRunIdentity,ToolRunTickWriter,ToolRunTickWireCursor,ToolRunTraceSubject,ToolRunVerdict};
+use semio_framework_tool_run::{ToolRunId,ToolRunIdentity,ToolRunTickWriter,ToolRunTickWireCursor,ToolRunTraceSubject,ToolRunVerdict,ToolRunTickPayloadSource};
 use semio_framework_value::RetainedCloneGrant as Grant;
 
+struct OriginalBytes<'a>(&'a[u8]);
+impl ToolRunTickPayloadSource for OriginalBytes<'_>{
+ fn original_identity(&self)->usize{self.0.as_ptr()as usize}
+ fn byte_len(&self)->usize{self.0.len()}
+ fn range_demands(&self,offset:usize,count:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{assert!(offset+count<=self.0.len());Ok(semio_framework_value::RetirementDemand{copy_bytes:count*2,depth:1,..Default::default()})}
+ fn write_range(&self,offset:usize,target:&mut[u8])->Result<(),semio_framework_value::ValueError>{target.copy_from_slice(&self.0[offset..offset+target.len()]);Ok(())}
+}
 #[test]
 fn original_tick_payload_preserves_same_context_authority_and_cancellation(){
  use semio_framework_job::{Generation,OperationId,JobPayloadAuthority,RetainedPayloadBuilder,JobPayloadStream,StepContext,StepBudget,JobOutcomeBorrow};
- let law:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️tests/🔬️plugin-runtime-runtime-close-budget/🧫️fixtures/🧾️fixture-caller/🔣️.json")).unwrap();let p=&law["native"]["grant"];let grant=Grant{maximum_items:p[0].as_u64().unwrap()as usize,maximum_copy_bytes:p[1].as_u64().unwrap()as usize,maximum_capacity_bytes:p[2].as_u64().unwrap()as usize,maximum_release_bytes:p[3].as_u64().unwrap()as usize,maximum_depth:p[4].as_u64().unwrap()as usize};let cancel=semio_framework_job::root_cancel_token();
- let identity=ToolRunIdentity::new(ToolRunId{app_instance_id:7,run:11},[17;32]);let mut writer=ToolRunTickWriter::new(identity);let bytes=vec![120;32769];let source_pointer=bytes.as_ptr();writer.payload(bytes);writer.append_op(vec![1,2,3]).unwrap();writer.append_op(vec![4,5,6]).unwrap();writer.append_entity(5);writer.append_entity(8);writer.upsert(5,ToolRunVerdict::Success,1,ToolRunTraceSubject::Entity{entity:5});let expected=writer.clone().finish().unwrap().encode().unwrap();assert_eq!(writer.payload_bytes().unwrap().as_ptr(),source_pointer);
- for cut in[0,1,8,64,256,usize::MAX]{let operation=OperationId(77341);let generation=Generation(3);let((mut authority,birth),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||JobPayloadAuthority::admit(operation,generation,grant).unwrap().unwrap());assert!(birth.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(birth.retained_capacity_bytes,birth.released_bytes));let mut born=heap.requested_bytes;let mut released=heap.released_bytes;let(mut cursor,receipt)=ToolRunTickWireCursor::admit(&writer,grant).unwrap();assert!(receipt.fits(grant));let mut payload=RetainedPayloadBuilder::new(JobPayloadStream::Preview);let mut offset=0;let mut sequence=0;let mut turns=0;let mut ready=false;
-  while !ready&&turns<cut{let mut receipt=Default::default();let mut cx=StepContext::with_payload_authority(operation,generation,StepBudget::new(1,u64::MAX,grant),&cancel,semio_framework_job::logical_now_us,&mut sequence,&mut receipt,&authority).unwrap();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||advance_original_tick_payload(&writer,&mut cursor,&mut payload,&mut offset,&mut cx).unwrap());ready=result;let progress=cx.retained_progress();assert!(progress.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;assert_eq!(writer.payload_bytes().unwrap().as_ptr(),source_pointer);turns+=1;assert!(turns<20000);}
+ let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let p=&law["grant"];let grant=Grant{maximum_items:p[0].as_u64().unwrap()as usize,maximum_copy_bytes:p[1].as_u64().unwrap()as usize,maximum_capacity_bytes:p[2].as_u64().unwrap()as usize,maximum_release_bytes:p[3].as_u64().unwrap()as usize,maximum_depth:p[4].as_u64().unwrap()as usize};let cancel=semio_framework_job::root_cancel_token();
+ for mode in law["sources"].as_array().unwrap(){let identity=ToolRunIdentity::new(ToolRunId{app_instance_id:7,run:11},[17;32]);let mut writer=ToolRunTickWriter::new(identity);let bytes=vec![law["byte"].as_u64().unwrap()as u8;law["payloadBytes"].as_u64().unwrap()as usize];let source_pointer=bytes.as_ptr();let original=OriginalBytes(&bytes);let native=(mode.as_str().unwrap()=="native").then_some(&original as &dyn ToolRunTickPayloadSource);if native.is_none(){writer.payload(bytes.clone());}let writer_pointer=writer.payload_bytes().map(|bytes|bytes.as_ptr());writer.append_op(vec![1,2,3]).unwrap();writer.append_op(vec![4,5,6]).unwrap();writer.append_entity(5);writer.append_entity(8);writer.upsert(5,ToolRunVerdict::Success,1,ToolRunTraceSubject::Entity{entity:5});let mut oracle=writer.clone();if native.is_some(){oracle.payload(bytes.clone());}let expected=oracle.finish().unwrap().encode().unwrap();assert_eq!(bytes.as_ptr(),source_pointer);assert_eq!(writer.payload_bytes().map(|bytes|bytes.as_ptr()),writer_pointer);
+ for cut in law["cancelCuts"].as_array().unwrap(){let cut=cut.as_u64().map(|n|n as usize).unwrap_or(usize::MAX);let operation=OperationId(77341);let generation=Generation(3);let((mut authority,birth),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||JobPayloadAuthority::admit(operation,generation,grant).unwrap().unwrap());assert!(birth.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(birth.retained_capacity_bytes,birth.released_bytes));let mut born=heap.requested_bytes;let mut released=heap.released_bytes;let(mut cursor,receipt)=ToolRunTickWireCursor::admit(&writer,native,grant).unwrap().unwrap();assert!(receipt.fits(grant));let mut payload=RetainedPayloadBuilder::new(JobPayloadStream::Preview);let mut offset=0;let mut sequence=0;let mut turns=0;let mut ready=false;
+  while !ready&&turns<cut{let mut receipt=Default::default();let mut cx=StepContext::with_payload_authority(operation,generation,StepBudget::new(1,u64::MAX,grant),&cancel,semio_framework_job::logical_now_us,&mut sequence,&mut receipt,&authority).unwrap();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||advance_original_tick_payload(&writer,native,&mut cursor,&mut payload,&mut offset,&mut cx).unwrap());ready=result;let progress=cx.retained_progress();assert!(progress.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;assert_eq!(bytes.as_ptr(),source_pointer);assert_eq!(writer.payload_bytes().map(|bytes|bytes.as_ptr()),writer_pointer);turns+=1;assert!(turns<20000);}
   if ready{let original=payload.published().unwrap();let pointer=original as*const _;let mut receipt=Default::default();let mut cx=StepContext::with_payload_authority(operation,generation,StepBudget::new(1,u64::MAX,grant),&cancel,semio_framework_job::logical_now_us,&mut sequence,&mut receipt,&authority).unwrap();let(outcome,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||JobOutcomeBorrow::admit_preview(&mut cx,original).unwrap().unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));match outcome{JobOutcomeBorrow::PreviewReady{payload,..}=>{assert_eq!(payload as*const _,pointer);let mut actual=Vec::new();for index in 0..payload.page_count(){actual.extend_from_slice(payload.page(index).unwrap());}assert_eq!(actual,expected);},_=>unreachable!()}assert!(cx.retained_progress().fits(grant));}
   let mut closed=false;while !closed{let mut receipt=Default::default();let mut cx=StepContext::with_payload_authority(operation,generation,StepBudget::new(1,u64::MAX,grant),&cancel,semio_framework_job::logical_now_us,&mut sequence,&mut receipt,&authority).unwrap();let(done,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||close_original_tick_payload(&mut cursor,&mut payload,&mut offset,&mut cx).unwrap());closed=done;let progress=cx.retained_progress();assert!(progress.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;}
-  let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||authority.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));released+=heap.released_bytes;assert_eq!(born,released);assert!(authority.terminal_is_empty());let((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{drop(cursor);drop(payload);drop(authority);});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(writer.payload_bytes().unwrap().as_ptr(),source_pointer);println!("[DEBUG] original Plugin tick payload cancellation={cut} turns={turns} sameOriginalContextAuthority=true wire={} physical={released} fixedCopy4096 existingToyCapacity65536 terminalDrop0",expected.len());
+  let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||authority.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));released+=heap.released_bytes;assert_eq!(born,released);assert!(authority.terminal_is_empty());let((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{drop(cursor);drop(payload);drop(authority);});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(bytes.as_ptr(),source_pointer);assert_eq!(writer.payload_bytes().map(|bytes|bytes.as_ptr()),writer_pointer);println!("[DEBUG] original Plugin tick payload source={mode} cancellation={cut} turns={turns} sameOriginalContextAuthority=true wire={} physical={released} fixedCopy4096 existingToyCapacity65536 terminalDrop0",expected.len());
  }
 }
+}


### 4 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs
B/H/S/current: 1698/118/1723/125
--- 
+++ 
@@ -30,4 +30,5 @@
 use semio_framework_plugin_window_config::WindowConfigOwner;
 use semio_framework_ui_contract::*;
+use store::ErasedSnapshotRetirement;
 use semio_framework_tool_run::{
     tool_run_format, ToolRunAction, ToolRunCounter, ToolRunDefinition, ToolRunEffect, ToolRunEvent, ToolRunId, ToolRunIdentity, ToolRunLabel, ToolRunLane, ToolRunMachine, ToolRunRebasePolicy,
@@ -455,5 +456,5 @@
             return None;
         }
-        protocol::apply_diff(outcome.diff(), snapshot).ok()
+        protocol::apply_diff_cold(outcome.diff(), snapshot).ok()
     }
 
@@ -488,5 +489,5 @@
 
     /// 🧬️ Applies one tick of the current identity: retract, append (O(k) fold), entities, steps, progress, trace.
-    fn apply_tick(&mut self, mut tick: ToolRunTick, discarded: &mut Vec<A::Mutation>) -> Result<ToolRunTickReceipt, Fault> {
+    pub(in crate::component) fn apply_tick(&mut self, mut tick: ToolRunTick, discarded: &mut Vec<A::Mutation>) -> Result<ToolRunTickReceipt, Fault> {
         let mut receipt = ToolRunTickReceipt::default();
         if tick.identity.id != self.identity.id || tick.identity.generation != self.identity.generation {
@@ -720,4 +721,6 @@
     }
 
+    pub(crate) fn poll_view_for(&self,window_id:Option<&str>)->Option<crate::ToolRunPollView<'_>>{self.window_index(window_id).or_else(||self.primary_index()).map(|index|{let entry=&self.entries[index];crate::ToolRunPollView{tool_id:&entry.tool_id,identity:entry.identity,state:entry.slot.state}})}
+
     fn window_index(&self, window_id: Option<&str>) -> Option<usize> {
         let window_id = window_id?;
@@ -1119,5 +1122,12 @@
 }
 
-struct ToolRunMemberState<P> {
+struct ToolRunMemberState<P,M:Send+Sync+'static> {
+    operation:Option<M>,
+    operation_source:Option<store::os_store::ArtifactMutationSource<M>>,
+    operation_input:Option<semio_framework_value::retained_clone::RetainedOwnedProjection<M>>,
+    operation_consumed:bool,
+    preparation:Option<store::os_store::ArtifactDerivedOperation<P,M>>,
+    operation_retirement:Option<Box<dyn store::ErasedSnapshotRetirement>>,
+    rebase_pending:bool,
     base: Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<P>>,
     overlay: Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<P>>,
@@ -1126,5 +1136,5 @@
 }
 
-impl<P: Send + Sync + 'static> ToolRunMemberOwners for ToolRunMemberState<P> {
+impl<P: Send + Sync + 'static,M:Send+Sync+'static> ToolRunMemberOwners for ToolRunMemberState<P,M> {
     fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
         self
@@ -1132,9 +1142,9 @@
 
     fn has_displaced(&self) -> bool {
-        !self.displaced.is_empty() || self.pending_alias.is_some()
+        !self.displaced.is_empty() || self.pending_alias.is_some()||self.operation_retirement.is_some()||((self.operation_consumed||self.rebase_pending)&&(self.operation_source.is_some()||self.operation_input.is_some()))||self.preparation.as_ref().is_some_and(|owner|owner.is_closing())
     }
 
     fn is_empty(&self) -> bool {
-        self.base.is_none() && self.overlay.is_none() && self.displaced.is_empty() && self.displaced.capacity()==0 && self.pending_alias.is_none()
+        self.operation.is_none()&&self.operation_source.is_none()&&self.operation_input.is_none()&&self.preparation.is_none()&&self.operation_retirement.is_none()&&self.base.is_none() && self.overlay.is_none() && self.displaced.is_empty() && self.displaced.capacity()==0 && self.pending_alias.is_none()
     }
 }
@@ -1151,5 +1161,5 @@
     pub(crate) shown: usize,
     pub(crate) rebase: bool,
-    pub(crate) owners: Option<Box<dyn ToolRunMemberOwners>>,
+    pub(in crate::component) owners: Option<Box<dyn ToolRunMemberOwners>>,
     pub(crate) children: Option<ChildContentView>,
     pub(crate) stale: Vec<ChildContentView>,
@@ -1193,4 +1203,6 @@
 pub(crate) struct ToolRunMemberFolded {
     pub(crate) shown: usize,
+    pub(crate) pending:bool,
+    pub(crate) progress:RetainedCloneProgress,
     pub(crate) conflicts: u32,
     pub(crate) read: Option<(store::os_store::snapshot_read::ErasedSnapshotRead, [u8; 32])>,
@@ -1206,6 +1218,6 @@
 }
 
-impl store::MemberStoreVisitor for ToolRunMemberFold<'_> {
-    type Output = Result<ToolRunMemberFolded, Fault>;
+impl store::MemberStoreVisitor for ToolRunMemberFold<'_,'_> {
+    type Output = Result<ToolRunMemberFolded, ToolRunMemberFoldRefusal>;
 
     fn visit<P, Mu>(self, store: &ArtifactStore<P, Mu>) -> Self::Output
@@ -1214,33 +1226,39 @@
         Mu: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ::protocol::Mutation<P> + protocol::SemanticMutation<P> + ::protocol::OpBinary + ::protocol::OpText + Send + 'static,
     {
-        let owners = self.owners.get_or_insert_with(|| Box::new(ToolRunMemberState::<P> { base: Some(store.derived_snapshot_head()), overlay: None, displaced: Vec::new(), pending_alias: None }));
-        let state = owners.as_any_mut().downcast_mut::<ToolRunMemberState<P>>().ok_or_else(|| Fault::new(FaultOrigin::Plugin, FaultCode::new("toolRun.member-store-kind"), "a member tool run's owners belong to another store kind"))?;
-        let mut shown = self.shown;
-        if self.rebase {
-            let previous = state.base.replace(store.derived_snapshot_head());
-            state.displaced.extend(previous.map(store::os_store::snapshot_read::ArtifactDerivedSnapshot::into_snapshot_owner));
-            state.displaced.extend(state.overlay.take().map(store::os_store::snapshot_read::ArtifactDerivedSnapshot::into_snapshot_owner));
-            shown = 0;
-        }
-        let mut conflicts = 0u32;
-        while shown < self.ops.len() {
-            let op = <Mu as ::protocol::OpBinary>::decode_op(&self.ops[shown]).map_err(|error| error.into_fault())?;
-            let source = state.overlay.as_ref().or(state.base.as_ref()).ok_or_else(|| Fault::new(FaultOrigin::Plugin, FaultCode::new("toolRun.member-base-lost"), "a member tool run lost its base"))?;
-            let next = store.derive_provisional_snapshot(source, &op);
-            <Mu as ::protocol::Mutation<P>>::retire_cold(op);
-            match next.map_err(|error| error.into_fault())? {
-                Some(next) => state.displaced.extend(state.overlay.replace(next).map(store::os_store::snapshot_read::ArtifactDerivedSnapshot::into_snapshot_owner)),
-                None => conflicts = conflicts.saturating_add(1),
+        let idle=RetainedCloneProgress::default();
+        if self.grant.maximum_items==0{return Ok(ToolRunMemberFolded{shown:self.shown,pending:true,progress:idle,conflicts:0,read:None});}
+        if self.owners.is_none(){let demand=RetirementDemand{capacity_bytes:size_of::<ToolRunMemberState<P,Mu>>(),depth:1,..Default::default()};if !tool_grant_funds(self.grant,demand){return Ok(ToolRunMemberFolded{shown:self.shown,pending:true,progress:idle,conflicts:0,read:None});}*self.owners=Some(Box::new(ToolRunMemberState::<P,Mu>{operation:None,operation_source:None,operation_input:None,operation_consumed:false,preparation:None,operation_retirement:None,rebase_pending:self.rebase,base:Some(store.derived_snapshot_head()),overlay:None,displaced:Vec::new(),pending_alias:None}));return Ok(ToolRunMemberFolded{shown:self.shown,pending:true,progress:RetainedCloneProgress{copied_items:1,retained_capacity_bytes:demand.capacity_bytes,..idle},conflicts:0,read:None});}
+        let state=self.owners.as_mut().unwrap().as_any_mut().downcast_mut::<ToolRunMemberState<P,Mu>>().ok_or_else(||plugin_sdk_fault("member fold owns another exact Store type"))?;
+        state.rebase_pending|=self.rebase;
+        let mut deadline=||semio_framework_job::default_now_us().is_none_or(|now|now>=self.deadline_us);
+        if state.rebase_pending{if let Some(owner)=state.preparation.as_mut(){owner.cancel();}}
+        if let Some(owner)=state.preparation.as_mut(){
+            if owner.is_closing(){let step=owner.close_step(self.grant).map_err(ToolRunMemberFoldRefusal::Native)?;if owner.terminal_is_empty(){drop(state.preparation.take());}return Ok(ToolRunMemberFolded{shown:self.shown,pending:true,progress:step.progress(),conflicts:0,read:None});}
+        }
+        if state.operation_consumed||state.rebase_pending{
+            if let Some(input)=state.operation_input.as_mut(){let step=input.close_step(self.grant).map_err(ToolRunMemberFoldRefusal::Native)?;if input.terminal_is_empty(){state.operation_input=None;}return Ok(ToolRunMemberFolded{shown:self.shown,pending:true,progress:step.progress(),conflicts:0,read:None});}
+            if let Some(source)=state.operation_source.as_mut(){source.begin_close();let step=source.close_step(self.grant).map_err(ToolRunMemberFoldRefusal::Native)?;if source.terminal_is_empty(){state.operation_source=None;}return Ok(ToolRunMemberFolded{shown:self.shown,pending:true,progress:step.progress(),conflicts:0,read:None});}
+        }
+        if state.operation_retirement.is_some(){let step=semio_framework_value::close_factory_ticket(&mut state.operation_retirement,self.grant).map_err(ToolRunMemberFoldRefusal::Native)?;return Ok(ToolRunMemberFolded{shown:self.shown,pending:true,progress:step.progress(),conflicts:0,read:None});}
+        if state.operation.is_some()&&(state.operation_consumed||state.rebase_pending){let p=store.retire_owned_mutation(&mut state.operation,self.grant).map_err(ToolRunMemberFoldRefusal::Native)?.map_or(idle,|(owner,p)|{state.operation_retirement=Some(owner);p});return Ok(ToolRunMemberFolded{shown:self.shown,pending:true,progress:p,conflicts:0,read:None});}
+        let mut shown=self.shown;
+        if state.rebase_pending{
+            let copy=3*size_of::<Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<P>>>();if !tool_grant_funds(self.grant,RetirementDemand{copy_bytes:copy,depth:1,..Default::default()}){return Ok(ToolRunMemberFolded{shown,pending:true,progress:idle,conflicts:0,read:None});}
+            if state.base.is_some()||state.overlay.is_some(){let needed=usize::from(state.base.is_some())+usize::from(state.overlay.is_some());if state.displaced.capacity()-state.displaced.len()<needed{let before=state.displaced.capacity();let bytes=(state.displaced.len()+needed)*size_of::<Arc<P>>();if self.grant.maximum_capacity_bytes<bytes||self.grant.maximum_release_bytes<before*size_of::<Arc<P>>(){return Ok(ToolRunMemberFolded{shown,pending:true,progress:idle,conflicts:0,read:None});}state.displaced.try_reserve_exact(needed).map_err(|_|plugin_sdk_fault("member original displaced slots cannot reserve"))?;return Ok(ToolRunMemberFolded{shown,pending:true,progress:RetainedCloneProgress{copied_items:1,retained_capacity_bytes:state.displaced.capacity()*size_of::<Arc<P>>(),released_bytes:before*size_of::<Arc<P>>(),..idle},conflicts:0,read:None});}}
+            state.displaced.extend(state.base.replace(store.derived_snapshot_head()).map(store::os_store::snapshot_read::ArtifactDerivedSnapshot::into_snapshot_owner));state.displaced.extend(state.overlay.take().map(store::os_store::snapshot_read::ArtifactDerivedSnapshot::into_snapshot_owner));state.rebase_pending=false;shown=0;return Ok(ToolRunMemberFolded{shown,pending:true,progress:RetainedCloneProgress{copied_items:1,copied_bytes:copy,..idle},conflicts:0,read:None});
+        }
+        if shown<self.ops.len(){
+            if state.operation.is_none()&&state.operation_source.is_none()&&state.preparation.is_none(){let bytes=self.ops[shown].len();if self.grant.maximum_copy_bytes<bytes||bytes>4096{return Ok(ToolRunMemberFolded{shown,pending:true,progress:idle,conflicts:0,read:None});}state.operation_consumed=false;state.operation=Some(<Mu as ::protocol::OpBinary>::decode_op(&self.ops[shown]).map_err(FaultFrom::into_fault)?);return Ok(ToolRunMemberFolded{shown,pending:true,progress:RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..idle},conflicts:0,read:None});}
+            if state.preparation.is_none(){
+                if state.operation_source.is_none(){let factory=store.owned_mutation_retirement_factory().map_err(ToolRunMemberFoldRefusal::Native)?;let Some((source,p))=store::os_store::ArtifactMutationSource::<Mu>::admit(&mut state.operation,&factory,self.grant).map_err(ToolRunMemberFoldRefusal::Native)?else{return Ok(ToolRunMemberFolded{shown,pending:true,progress:idle,conflicts:0,read:None});};state.operation_source=Some(source);return Ok(ToolRunMemberFolded{shown,pending:true,progress:p,conflicts:0,read:None});}
+                if state.operation_input.is_none(){let source=state.operation_source.as_mut().unwrap();let p=if source.ready(){source.take(&mut state.operation_input,self.grant).map_err(ToolRunMemberFoldRefusal::Native)?.unwrap_or(idle)}else{source.advance(self.grant).map_err(ToolRunMemberFoldRefusal::Native)?.progress()};return Ok(ToolRunMemberFolded{shown,pending:true,progress:p,conflicts:0,read:None});}
             }
-            shown += 1;
-            if shown % TOOL_RUN_REFOLD_CHECK_OPS == 0 && shown < self.ops.len() && semio_framework_job::default_now_us().is_none_or(|now| now >= self.deadline_us) {
-                return Ok(ToolRunMemberFolded { shown, conflicts, read: None });
-            }
-        }
-        let read = match state.overlay.as_ref().filter(|_| !self.ops.is_empty()) {
-            Some(overlay) => Some((store.snapshot_read_derived(overlay).map_err(|error| error.into_fault())?, store.content_revision())),
-            None => None,
-        };
-        Ok(ToolRunMemberFolded { shown, conflicts, read })
+            if state.preparation.is_none(){let source=state.overlay.as_ref().or(state.base.as_ref()).ok_or_else(||plugin_sdk_fault("member fold retains its original base"))?;let p=store.begin_derived_operation(&mut state.preparation,source,&mut state.operation_input,u32::try_from(shown).unwrap_or(u32::MAX),store::os_store::ArtifactDerivedOperationMode::Provisional,self.grant).map_err(ToolRunMemberFoldRefusal::Native)?.unwrap_or(idle);return Ok(ToolRunMemberFolded{shown,pending:true,progress:p,conflicts:0,read:None});}
+            let owner=state.preparation.as_mut().unwrap();
+            if owner.is_finished(){let Some(((next,_),p))=store.take_derived_operation(owner,self.grant).map_err(ToolRunMemberFoldRefusal::Native)?else{return Ok(ToolRunMemberFolded{shown,pending:true,progress:idle,conflicts:0,read:None});};state.operation_consumed=true;let conflicts=u32::from(next.is_none());if let Some(next)=next{if let Some(previous)=state.overlay.replace(next){state.pending_alias=Some(previous.into_snapshot_owner());}}return Ok(ToolRunMemberFolded{shown:shown+1,pending:true,progress:p,conflicts,read:None});}
+            let step=store.step_derived_operation(owner,self.grant.maximum_copy_bytes,self.grant,self.receiving,&mut deadline).map_err(ToolRunMemberFoldRefusal::Native)?;return Ok(ToolRunMemberFolded{shown,pending:true,progress:step.progress(),conflicts:0,read:None});
+        }
+        let read=state.overlay.as_ref().filter(|_|!self.ops.is_empty()).map(|overlay|store.snapshot_read_derived(overlay).map(|read|(read,store.content_revision()))).transpose().map_err(FaultFrom::into_fault)?;
+        Ok(ToolRunMemberFolded{shown,pending:false,progress:idle,conflicts:0,read})
     }
 }
@@ -1259,5 +1277,5 @@
  let Some(owner)=owner.as_ref()else{return Ok(Default::default())};
  if owner.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:size_of_val(owner.as_ref()),copy_bytes:size_of::<Option<Box<dyn store::ErasedSnapshotRetirement>>>(),depth:1,..Default::default()});}
- Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"tool retained close depth overflow"))?})
+ Ok({let mut demand=owner.next_demand(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"tool retained close depth overflow"))?;demand})
 }
 pub(crate) fn tool_grant_funds(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{
@@ -1298,5 +1316,5 @@
         if let Some(owner)=self.retirement_refusal_close.as_ref(){
             if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<ValueError>>>(),depth:1,..Default::default()});}
-            return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"member original refusal close depth overflow"))?});
+            return Ok({let mut demand=owner.next_demand(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"member original refusal close depth overflow"))?;demand});
         }
         if self.retirement_refusal.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<ControlledRetirement<ValueError>>()+size_of::<Option<ValueError>>(),depth:1,..Default::default()});}
@@ -1409,5 +1427,12 @@
     {
         if self.grant.maximum_items==0{return Ok((None,Default::default()));}
-        let state=self.owners.as_any_mut().downcast_mut::<ToolRunMemberState<P>>().ok_or_else(||plugin_sdk_fault("member alias retirement owns another exact Store type"))?;
+        let state=self.owners.as_any_mut().downcast_mut::<ToolRunMemberState<P,Mu>>().ok_or_else(||plugin_sdk_fault("member alias retirement owns another exact Store type"))?;
+        if let Some(owner)=state.preparation.as_mut(){if self.everything{owner.cancel();}if owner.is_closing(){let step=owner.close_step(self.grant).map_err(ValueError::into_fault)?;if owner.terminal_is_empty(){drop(state.preparation.take());}return Ok((None,step.progress()));}}
+        if self.everything||state.operation_consumed{
+            if let Some(input)=state.operation_input.as_mut(){let step=input.close_step(self.grant).map_err(ValueError::into_fault)?;if input.terminal_is_empty(){state.operation_input=None;}return Ok((None,step.progress()));}
+            if let Some(source)=state.operation_source.as_mut(){source.begin_close();let step=source.close_step(self.grant).map_err(ValueError::into_fault)?;if source.terminal_is_empty(){state.operation_source=None;}return Ok((None,step.progress()));}
+        }
+        if state.operation_retirement.is_some(){return semio_framework_value::close_factory_ticket(&mut state.operation_retirement,self.grant).map(|step|(None,step.progress())).map_err(ValueError::into_fault);}
+        if state.operation.is_some()&&(self.everything||state.operation_consumed){return Ok(store.retire_owned_mutation(&mut state.operation,self.grant).map_err(ValueError::into_fault)?.map_or((None,Default::default()),|(frame,p)|(Some(frame),p)));}
         if state.pending_alias.is_some(){return Ok(store.retire_snapshot_alias(&mut state.pending_alias,self.grant).map_err(FaultFrom::into_fault)?.map_or((None,Default::default()),|(owner,progress)|(Some(owner),progress)));}
         let copy=2*size_of::<Option<Arc<P>>>();


### 5 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🧵️instance/🦀️.rs
B/H/S/current: 1078/62/1080/62
--- 
+++ 
@@ -399,5 +399,5 @@
 
     /// ⏯️ The selected run's share of one driver turn.
-    async fn drive_selected_tool_run(&mut self, deadline: u64,grant:RetainedCloneGrant,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
+    async fn drive_selected_tool_run(&mut self, deadline: u64,grant:RetainedCloneGrant,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,receiving:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<(), Fault> {
         let Some(state) = self.tool_runs.state() else { return Ok(()) };
         match state {
@@ -418,5 +418,5 @@
             ToolRunState::Starting => self.admit_tool_run_job().await,
             ToolRunState::Running | ToolRunState::Paused | ToolRunState::Complete => {
-                if self.refold_tool_run(deadline) || self.refresh_tool_run_member(deadline)? {
+                if self.refold_tool_run(deadline) || self.refresh_tool_run_member(deadline,grant,receiving)? {
                     return Ok(());
                 }
@@ -436,10 +436,10 @@
             }
             ToolRunState::Finalizing => {
-                if self.refresh_tool_run_member(deadline)? {
+                if self.refresh_tool_run_member(deadline,grant,receiving)? {
                     return Ok(());
                 }
-                self.finalize_tool_run_turn(deadline,grant,identity).await
-            }
-            ToolRunState::Finalized | ToolRunState::Aborted | ToolRunState::Faulted => self.settle_tool_run_member(deadline),
+                self.finalize_tool_run_turn(deadline,grant,identity,receiving).await
+            }
+            ToolRunState::Finalized | ToolRunState::Aborted | ToolRunState::Faulted => self.settle_tool_run_member(deadline,grant),
         }
     }
@@ -577,5 +577,5 @@
 
     /// 🏁️ Finalize (§2.7.4): freshness refold on the head, optional revalidate job, one outbound batched `Edit`.
-    async fn finalize_tool_run_turn(&mut self, deadline: u64,grant:RetainedCloneGrant,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
+    async fn finalize_tool_run_turn(&mut self, deadline: u64,grant:RetainedCloneGrant,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,receiving:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<(), Fault> {
         let store_generation = self.stores.store.generation();
         let member_generation = self.shared().tool_run_member_generation();
@@ -634,5 +634,5 @@
             }
             ToolRunFinalizePhase::Revalidating => self.step_tool_run_job(deadline,grant),
-            ToolRunFinalizePhase::Publishing => self.publish_tool_run(deadline,grant,identity).await,
+            ToolRunFinalizePhase::Publishing => self.publish_tool_run(deadline,grant,identity,receiving).await,
             ToolRunFinalizePhase::Closing => self.close_tool_run_publication(grant),
         }
@@ -651,7 +651,7 @@
     /// ♻️ Each folded op leaves the root it was prepared against returned to the Store; reclaim it before the
     /// next fold, or a large finalize keeps one whole document per op (see `reclaim_document_snapshot_read_returns`).
-    async fn publish_tool_run(&mut self, deadline: u64,grant:RetainedCloneGrant,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
+    async fn publish_tool_run(&mut self, deadline: u64,grant:RetainedCloneGrant,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,receiving:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<(), Fault> {
         if selected_entry!(self.tool_runs).is_some_and(|entry| entry.member.is_some()) {
-            return self.publish_tool_run_member(deadline,grant).await;
+            return self.publish_tool_run_member(deadline,grant,receiving).await;
         }
         let entry = selected_entry_mut!(self.tool_runs).expect("finalizing slot");
@@ -678,4 +678,5 @@
                 Ok(Some((publication,progress)))=>{finalize.publication=Some(publication);self.tool_runs.retain_original_receipt(grant,progress)?;return Ok(());},
                 Ok(None)=>return Ok(()),
+                Err(store::os_vcs::VcsError::InverseRefused(error))=>{self.tool_runs.retain_original_receipt(grant,error.retained_progress())?;return Err(error.into_fault())},
                 Err(_) => return self.reject_tool_run_publication(run, generation),
             }
@@ -690,6 +691,6 @@
             let entry = selected_entry_mut!(self.tool_runs).expect("finalizing slot");
             let publication = entry.finalize.as_mut().and_then(|finalize| finalize.publication.as_mut()).expect("admitted publication");
-            match self.stores.store.advance_apply_batch(publication, store::os_store::one_item::ArtifactStoreOneItemGrant{maximum_items:grant.maximum_items.min(1),maximum_copy_bytes:grant.maximum_copy_bytes,maximum_capacity_bytes:grant.maximum_capacity_bytes,maximum_release_bytes:grant.maximum_release_bytes,maximum_depth:grant.maximum_depth}) {
-                Ok(store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Published(_)) => {
+            match self.stores.store.advance_apply_batch(publication, store::os_store::one_item::ArtifactStoreOneItemGrant{maximum_items:grant.maximum_items.min(1),maximum_copy_bytes:grant.maximum_copy_bytes,maximum_capacity_bytes:grant.maximum_capacity_bytes,maximum_release_bytes:grant.maximum_release_bytes,maximum_depth:grant.maximum_depth},receiving) {
+                Ok(store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Published(_, _)) => {
                     let group_id = entry.identity.id.group_id();
                     let tool_id = entry.tool_id.clone();
@@ -712,4 +713,5 @@
                 Ok(store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::PreparationProgress(_,progress))=>{self.tool_runs.retain_original_receipt(grant,progress)?;return Ok(());},
                 Ok(_) => {}
+                Err(store::os_vcs::VcsError::InverseRefused(error))=>{self.tool_runs.retain_original_receipt(grant,error.retained_progress())?;return Err(error.into_fault())},
                 Err(_) => return self.reject_tool_run_publication(run, generation),
             }
@@ -770,7 +772,7 @@
     /// of the reads it replaced is saturated (retryable), so the job waits instead of leasing more; a fold that cannot
     /// apply its ops (they do not decode as the member's vocabulary) faults the run.
-    fn refresh_tool_run_member(&mut self, deadline: u64) -> Result<bool, Fault> {
+    fn refresh_tool_run_member(&mut self, deadline: u64,grant:RetainedCloneGrant,receiving:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<bool, Fault> {
         let Some(mut member) = selected_entry_mut!(self.tool_runs).and_then(|entry| entry.member.take()) else { return Ok(false) };
-        let outcome = self.child_composition.refresh_member_overlay(&mut member, deadline);
+        let outcome = self.child_composition.refresh_member_overlay(&mut member, deadline,grant,receiving);
         let entry = selected_entry_mut!(self.tool_runs).expect("a member run keeps its slot");
         entry.member = Some(member);
@@ -800,8 +802,8 @@
     /// overlay rests on the member head again, so the run owes nothing. A member that is gone has no head to rest on; a
     /// saturated retirement ring (retryable) is retried on the next turn.
-    fn settle_tool_run_member(&mut self, deadline: u64) -> Result<(), Fault> {
+    fn settle_tool_run_member(&mut self, deadline: u64,grant:RetainedCloneGrant) -> Result<(), Fault> {
         let Some(mut member) = selected_entry_mut!(self.tool_runs).and_then(|entry| entry.member.take()) else { return Ok(()) };
         let outcome = if self.child_composition.children.get(&(member.slot.clone(), member.child_id.clone())).is_some() {
-            self.child_composition.refresh_member_overlay(&mut member, deadline).map(|_| ())
+            self.child_composition.refresh_member_overlay(&mut member, deadline,grant,receiving).map(|_| ())
         } else {
             member.rebase = false;
@@ -821,5 +823,5 @@
     /// that member store. A member that is gone or ops that do not decode reject the publication like a conflicting parent
     /// batch.
-    async fn publish_tool_run_member(&mut self, deadline: u64,grant:RetainedCloneGrant) -> Result<(), Fault> {
+    async fn publish_tool_run_member(&mut self, deadline: u64,grant:RetainedCloneGrant,receiving:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<(), Fault> {
         let app_instance_id = self.app.instance_id().await;
         let entry = selected_entry_mut!(self.tool_runs).expect("finalizing slot");


### 6 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/♻️lifecycle/🦀️.rs
B/H/S/current: 525/527/543/529
--- 
+++ 
@@ -206,4 +206,6 @@
         Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<BoundedConfigPreparation<C, M>>(), depth: 1 })
     }
+    fn begin_copy_byte_demand(&self,input:&M,lane:HistoryLane)->Result<usize,semio_framework_value::ValueError>{self.begin_demand(input,lane)?; std::mem::size_of::<store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<C, M, M>>().checked_add(std::mem::size_of::<BoundedConfigPreparation<C, M>>()).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original preparation header quote overflow"))}
+
     fn begin(&self, request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<C, M, M>, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<C, M>>, RetainedCloneProgress), (ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<C, M, M>)> {
         let demand = match self.begin_demand(&request.mutation, request.lane) { Ok(demand) => demand, Err(error) => return Err((error, request)) };
@@ -240,6 +242,6 @@
  fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
   let nested=|mut demand:RetirementDemand|->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"bounded config preparation depth overflow"))?;Ok(demand)};
-  if let Some(owner)=self.active_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of_val(owner.as_ref()),depth:1,..Default::default()})}else{nested(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})};}
-  if let Some(owner)=self.returned_read.as_ref(){return nested(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?});}
+  if let Some(owner)=self.active_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of_val(owner.as_ref()),depth:1,..Default::default()})}else{nested(owner.next_demand(body)?)};}
+  if let Some(owner)=self.returned_read.as_ref(){return nested(owner.next_demand(body)?);}
   if let Some(factory)=self.returned_factories.iter().find_map(Option::as_ref){return nested(factory.demands(body)?);}
   if self.prepared.is_some(){let birth=store::os_store::one_item_publication::ArtifactStoreOneItemPrepared::<C,M>::retirement_birth_demand();return Ok(RetirementDemand{capacity_bytes:birth.capacity_bytes,depth:birth.depth+1,..Default::default()});}
@@ -257,5 +259,5 @@
     M: Clone + OpBinary + Mutation<C> + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
 {
-    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, ValueError> {
+    fn advance(&mut self, body:usize,grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, ValueError> {
         if !grant.permits_one() || self.cancelled {
             return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
@@ -390,5 +392,5 @@
 pub struct ArtifactStoreInitializationJob<P, Mutation>
 where
-    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
+    P: semio_framework_value::ToValue,
     Mutation: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::Mutation<P>,
 {
@@ -401,5 +403,5 @@
 impl<P, Mutation> ArtifactStoreInitializationJob<P, Mutation>
 where
-    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
+    P: semio_framework_value::ToValue,
     Mutation: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::Mutation<P>,
 {
@@ -459,5 +461,5 @@
 impl<P, Mutation> semio_framework_job::InteractiveJob for ArtifactStoreInitializationJob<P, Mutation>
 where
-    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send,
+    P: semio_framework_value::ToValue + Send,
     Mutation: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::Mutation<P> + Send,
 {
@@ -473,5 +475,5 @@
                     authority.request_cancel();
                 }
-                authority.step(cx).map_err(semio_framework_job::OriginalTurnFailure::from)
+                authority.step(cx)
             }
             None if self.terminal_handoff => semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None).map_err(semio_framework_job::OriginalTurnFailure::from),
@@ -497,8 +499,24 @@
     fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
         use semio_framework_job::InteractiveJobCloseStep as Step;
-        self.begin_close();if self.terminal_is_empty(){return Ok(Step::Complete{progress:Default::default()})}let demand=self.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;
-        let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{
-            if let Some(authority)=self.authority.as_mut(){if authority.terminal_is_empty(){drop(self.authority.take());return Ok(semio_framework_value::RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))}let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};let step=authority.close_step(child)?;return semio_framework_value::retained_clone::admit_retained_clone_close(child,step,authority.terminal_is_empty(),"original initializer authority")}
-            unsafe{ManuallyDrop::drop(&mut self.cancel_requested)}self.cancel_arc_live=false;self.terminal_handoff=true;Ok(semio_framework_value::RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))
+        self.begin_close();
+        if self.terminal_is_empty(){return Ok(Step::Complete{progress:Default::default()})}
+        let demand=self.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;
+        if let Some(authority)=self.authority.as_mut(){
+            if !authority.terminal_is_empty(){
+                let remaining=cx.retained_grant();
+                if cx.retirement_should_yield()||remaining.maximum_items==0||remaining.maximum_depth<demand.depth||remaining.maximum_copy_bytes<demand.copy_bytes||remaining.maximum_capacity_bytes<demand.capacity_bytes||remaining.maximum_release_bytes<demand.release_bytes{return Ok(Step::Pending{progress:Default::default()})}
+                let child=RetainedCloneGrant{maximum_items:1,maximum_depth:remaining.maximum_depth-1,..remaining};
+                let step=cx.with_retained_loan(child,|same|authority.close_step(same))?;
+                return Ok(Step::Pending{progress:step.progress()})
+            }
+        }
+        let step=semio_framework_job::close_original_retained_step(cx,demand,|_|{
+            if self.authority.is_some(){
+                drop(self.authority.take());
+                return Ok(semio_framework_value::RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))
+            }
+            unsafe{ManuallyDrop::drop(&mut self.cancel_requested)}
+            self.cancel_arc_live=false;self.terminal_handoff=true;
+            Ok(semio_framework_value::RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))
         })?;
         Ok(match step{Step::Pending{progress}|Step::Complete{progress}=>if self.terminal_is_empty(){Step::Complete{progress}}else{Step::Pending{progress}},Step::Blocked=>Step::Blocked})
@@ -512,5 +530,5 @@
 impl<P, Mutation> Drop for ArtifactStoreInitializationJob<P, Mutation>
 where
-    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
+    P: semio_framework_value::ToValue,
     Mutation: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::Mutation<P>,
 {


### 7 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/⚖️failure-debit/🧪️tests/🟦️.ts
B/H/S/current: 132/124/132/124
--- 
+++ 
@@ -48,5 +48,5 @@
 test("original failure debit input admits exact receipt and original policy, separate expected outcomes",()=>{const ajv=new Ajv({strict:true});const authority=Object.values(schema.properties).find(v=>"$ref"in v&&v.$ref!=="#/$defs/progress")! as{$ref:string};const canonical=new URL("../🧬️schema/🔣️.json",import.meta.url).href;ajv.addSchema(grant,new URL(authority.$ref,canonical).href);const admit=ajv.compile({...schema,$id:canonical});for(const input of inputs)expect(admit(input)).toBe(true);const admitGrant=ajv.compile(grant);for(const policy of Object.values(closing))expect(admitGrant(policy)).toBe(true);expect(admit({...inputs[0],published:true})).toBe(false);});
 test("independent SQLite and integer arithmetic preserve real failed child currencies and saturating fuel",()=>{for(let index=0;index<inputs.length;index++){const input=inputs[index]!,output=expected[index]!,db=new Database(":memory:",{safeIntegers:true});try{const metrics=["copiedItems","copiedBytes","retainedCapacityBytes","releasedBytes"]as const;const axes=["maximumItems","maximumCopyBytes","maximumCapacityBytes","maximumReleaseBytes"]as const;let accepted=true;for(let axis=0;axis<4;axis++){const key=metrics[axis]!,limit=axes[axis]!,initial=BigInt(input.recipient[key]),child=BigInt(input.originalReceipt[key]);const observed=db.query("SELECT ? + ? AS spent").get(initial,child)as{spent:bigint};expect(observed.spent).toBe(initial+child);expect(observed.spent).toBe(BigInt(output.recipient[key]));accepted&&=observed.spent<=BigInt(input.grant[limit]);}const fuel=db.query("SELECT max(0,? - ?) AS remaining").get(input.fuel,input.originalReceipt.copiedItems)as{remaining:bigint};expect(fuel.remaining).toBe(BigInt(output.fuel));expect(accepted).toBe(output.accepted);expect(JSON.parse(JSON.stringify(input.originalReceipt))).toEqual(input.originalReceipt);}finally{db.close();}}console.log("[DEBUG] Original failure6inputs independently conserve all4actual child currencies on admitted/refused debit and canonical saturating fuel; runtime owner pointers require Native");});
-test("actual original owner retains canonical Job primary and receipt causes before publication",()=>{expect(original).toContain("close_fault:ManuallyDrop<Option<OriginalTurnFailure>>");expect(original).toContain("debit_original_error");expect(original).toContain("close_fault");expect(original).toContain("debit_fault");expect(original).not.toContain("failure.primary");const start=original.indexOf("fn debit_original_error"),end=original.indexOf("\n }",start),body=original.slice(start,end);expect(body.indexOf("consume_fuel")).toBeLessThan(body.indexOf("consume_retained"));expect(body).toContain("retained_progress()");expect(body).toContain("Some(OriginalTurnFailure::from(original))");expect(body.indexOf("**close=Some")).toBeLessThan(body.indexOf("consume_fuel"));expect(body).toContain("receipt_refusal=cx.consume_retained(progress).err()");expect(body).toContain("receipt_refusal.is_none()");});
+test("actual original owner retains canonical Job primary and receipt causes before publication",()=>{expect(original).toContain("close_fault:ManuallyDrop<Option<OriginalTurnFailure>>");expect(original).toContain("debit_original_error");expect(original).toContain("close_fault");expect(original).toContain("debit_fault");expect(original).not.toContain("failure.primary");const start=original.indexOf("fn debit_original_error"),end=original.indexOf("\n }",start),body=original.slice(start,end);expect(body).not.toContain("consume_fuel");expect(original.match(/cx\.consume_fuel\(actual_work\);Self::debit_original_error\(error,close_fault,cx\)/g)?.length).toBe(3);expect(body).toContain("retained_progress()");expect(body).toContain("Some(OriginalTurnFailure::from(original))");expect(body.indexOf("**close=Some")).toBeLessThan(body.indexOf("consume_retained"));expect(body).toContain("receipt_refusal=cx.consume_retained(progress).err()");expect(body).toContain("receipt_refusal.is_none()");});
 
 /** 🔁️ The existing retained request returns its whole failure while preserving every pending original owner. */


### 8 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs
B/H/S/current: 2321/145/2320/145
--- 
+++ 
@@ -1136,4 +1136,5 @@
     let close_cleanup_work = close_cleanup_work || extension_retirement_work;
     let mut effects: Vec<Effect> = document_backbone_effects;
+    let _=crate::plugin_runtime::plugin_drive_result_pages(runtime,&mut effects,budget.max_effects as usize,cx)?;
     effects.extend(inbound_request_effects);
     let mut cold_pair_ingress = semio_framework::kernel::ColdPairIngressStatus::Idle;
@@ -1480,4 +1481,5 @@
 
     trace_turn_phase_retention("continuation");
+    let result_page_work=crate::plugin_runtime::plugin_drive_result_pages(runtime,&mut effects,budget.max_effects as usize,cx)?;
     let now_ms = u64::try_from(semio_framework_plugin_actor_host::now_ms().await).unwrap_or(0);
 
@@ -1567,5 +1569,5 @@
         process_pool: process_pool_work,
         close_cleanup: close_cleanup_work,
-        typed_operation: typed_operation_scan.runnable,
+        typed_operation: typed_operation_scan.runnable || result_page_work,
         typed_operation_contended,
         reconcile: reconcile_work,
@@ -2116,5 +2118,5 @@
   if let Err(error)=cx.consume_retained(unposted){if refusal.is_none(){refusal=Some(error)}else if receipt_refusal.is_none(){receipt_refusal=Some(error)}}
  }
- match refusal{Some(error)=>Err(OriginalPublicationReceiptRefusal{receipts,error:semio_framework_job::OriginalTurnFailure{primary:error.into(),receipt_refusal,context_refusal:None}}),None=>Ok(())}
+ match refusal{Some(error)=>Err(OriginalPublicationReceiptRefusal{receipts,error:{let original=semio_framework_job::OriginalTurnFailure::from(error);semio_framework_job::OriginalTurnFailure{receipt_refusal,context_refusal:None,..original}}}),None=>Ok(())}
 }
 
@@ -2129,7 +2131,4 @@
  }
 
-    for page in output.typed_operation_results.iter() {
-        effects.push(Effect::SendMessage { target: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) }, payload: page.renderer_exchange_bytes() });
-    }
     for frame_bytes in output.frames {
         route_app_frame(instance, &frame_bytes, effects);


### 9 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌱️initialization/♻️retirement/🧪️tests/🟦️.ts
B/H/S/current: 11/11/27/22
--- 
+++ 
@@ -7,5 +7,21 @@
  const root=new URL("../",import.meta.url),read=(p:string)=>JSON.parse(readFileSync(new URL(p,root),"utf8")),law=read("🧪️tests/🧫️fixtures/🔣️.json");expect(new Ajv2020({strict:true}).compile(read("🧬️schema/🔣️.json"))(law)).toBe(true);
  const db=new Database(":memory:");try{db.exec("CREATE TABLE custody(frontier TEXT PRIMARY KEY,original TEXT,closed INTEGER)");for(const frontier of law.frontiers){const original=JSON.stringify({actor:"Ä\u0000original",id:"original-envelope",frontier});db.query("INSERT INTO custody VALUES(?,?,0)").run(frontier,original);for(const axis of Object.keys(law.grant)){const denied={...law.grant,[axis]:0};expect(denied[axis]).toBe(0);expect(db.query("SELECT original,closed FROM custody WHERE frontier=?").get(frontier)).toEqual({original,closed:0});}}}finally{db.close();}
- const source=readFileSync(new URL("../../🦀️.rs",root),"utf8"),start=source.indexOf("pub trait ArtifactStoreInitializationAuthority"),end=source.indexOf("fn drive_artifact_owned_disposer",start),owner=source.slice(start,end);expect(owner).toContain("retirement_demands");expect(owner).toContain("RetainedCloneGrant");expect(owner).toContain("admit_retained_clone_close");expect(owner).not.toContain("maximum_items: usize, maximum_bytes: usize");expect(owner).toContain("cancel_arc_release");expect(owner).toContain("retained_progress");
+ const plugin=new URL("../../../",import.meta.url),authority=readFileSync(new URL("../🏪️store/🏗️initialization/🤝️authority/🦀️.rs",plugin),"utf8"),owner=readFileSync(new URL("♻️lifecycle/🦀️.rs",plugin),"utf8");
+ expect(authority).toContain("retirement_demands");expect(authority).toContain("OriginalTurnFailure");expect(authority).toMatch(/fn close_step\(&mut self,\s*cx:/);
+ expect(authority).not.toContain("close_step(&mut self, grant:");
+ expect(owner).toContain("authority.close_step(same)");expect(owner).toContain("with_retained_loan");expect(owner).toContain("cancel_arc_release");
+ expect(owner).not.toContain("let step=authority.close_step(child)?");
+
  console.log("[DEBUG] SQLite initializer original custody conserved through eight independent grant frontiers; native physical acceptance remains separate");
 });
+
+/** 🎟️ Authored native fixtures retain one real source across pauses, cancellation and authority closure. */
+test("initializer fixture callers preserve original source and whole failure API",()=>{
+ const plugin=new URL("../../../",import.meta.url),read=(f:string)=>readFileSync(new URL(f,plugin),"utf8");
+ const reload=read("🧪️tests/🧪️bounded-reload/🦀️.rs"),replay=read("../🏪️store/🏗️initialization/🔁️replay/🧪️tests/🔬️unit/🦀️.rs"),native=read("🌱️initialization/♻️retirement/🧪️tests/🦀️.rs"),composition=read("🧪️tests/🧩️composition/🦀️.rs");
+ expect(reload).toContain("OriginalWorkFixture::with_identity(operation,generation)");expect(reload).toContain("original_caller.work_turn(original,1,|cx|");expect(reload).toContain("original_caller.finish()");expect(reload).not.toContain("StepContext::new");
+ expect(replay).toContain("turn_with_decoder(grant,&mut self.decoder,advance)");expect(replay).toContain("original.caller.as_ref().unwrap().cancel().cancel_now()");expect(replay).toContain("fn run_until(init: &mut Initializer, original:&mut ReplayOriginal");expect(replay).toContain("original.turn(WIDE,|cx|ArtifactStoreInitializationAuthority::close_step(init,cx))");expect(replay).toContain("self.caller.take().unwrap().retire(WIDE)");expect(replay).not.toContain("StepContext::new");expect(replay).not.toContain("StepContext::with_payload_authority");expect(replay).not.toContain("ArtifactStoreInitializationAuthority::close_step(init, WIDE)");
+ expect(native).toContain("source.turn(denied,|cx|InteractiveJob::close_step(&mut job,cx))");expect(native).toContain("source.turn(grant,|cx|InteractiveJob::close_step(&mut job,cx))");expect(native).toContain("source.finish()");
+ const owner=composition.slice(composition.indexOf("impl store::ArtifactStoreInitializationAuthority"),composition.indexOf("impl Drop for ReadyComposedParentInitialization"));expect(owner).toContain("fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)");expect(owner).not.toContain("fn close_step(&mut self, grant:");
+ console.log("[DEBUG] initializer authored native fixture source continuity verified; canonical native execution remains prerequisite-blocked");
+});


### 10 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌱️initialization/♻️retirement/🧪️tests/🦀️.rs
B/H/S/current: 30/36/37/43
--- 
+++ 
@@ -11,11 +11,18 @@
 struct OriginalAuthority{payload:Option<ControlledRetirement<String>>,pointer:usize,closing:bool}
 impl ArtifactStoreInitializationAuthority<TestSnapshot,TestMutation> for OriginalAuthority{
- fn step<'a>(&'a mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,ValueError>{semio_framework_job::JobOutcomeBorrow::admit_yield(cx)}
+ fn step<'a>(&'a mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure> {semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from)}
  fn borrow_outcome<'a>(&'a self,descriptor:&'a semio_framework_job::JobOutcomeDescriptor)->Result<semio_framework_job::JobOutcomeView<'a>,ValueError>{descriptor.yielded()}
  fn request_cancel(&mut self){self.closing=true;}
  fn begin_close(&mut self){self.closing=true;}
  fn take_candidate(&mut self)->Option<ArtifactStore<TestSnapshot,TestMutation>>{None}
- fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.payload.as_ref().map_or(Ok(Default::default()),|owner|if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<String>>>(),depth:1,..Default::default()})}else{nested(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})})}
- fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let demand=self.retirement_demands(grant.maximum_copy_bytes)?;if !self.closing||!funds(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}let Some(owner)=self.payload.as_mut()else{return Ok(RetainedCloneStep::Complete(Default::default()))};if owner.terminal_is_empty(){drop(self.payload.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}if let Some(original)=owner.original(){assert_eq!(original.as_ptr()as usize,self.pointer);}let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=owner.step(child)?;admit_retained_clone_close(child,step,owner.terminal_is_empty(),"native original initializer fixture").map(|step|RetainedCloneStep::Progress(step.progress()))}
+ fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.payload.as_ref().map_or(Ok(Default::default()),|owner|if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<String>>>(),depth:1,..Default::default()})}else{nested(owner.next_demand(body)?)})}
+ fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<RetainedCloneStep,semio_framework_job::OriginalTurnFailure> {
+        self.begin_close();
+        if self.terminal_is_empty(){return Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(Default::default()))}
+        let demand=self.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;
+        let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{let demand=self.retirement_demands(grant.maximum_copy_bytes)?;if !self.closing||!funds(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}let Some(owner)=self.payload.as_mut()else{return Ok(RetainedCloneStep::Complete(Default::default()))};if owner.terminal_is_empty(){drop(self.payload.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}if let Some(original)=owner.original(){assert_eq!(original.as_ptr()as usize,self.pointer);}let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=owner.step(child)?;admit_retained_clone_close(child,step,owner.terminal_is_empty(),"native original initializer fixture").map(|step|RetainedCloneStep::Progress(step.progress()))
+        })?;
+        Ok(if self.terminal_is_empty(){semio_framework_value::retained_clone::RetainedCloneStep::Complete(step.progress())}else{semio_framework_value::retained_clone::RetainedCloneStep::Progress(step.progress())})
+    }
  fn terminal_is_empty(&self)->bool{self.closing&&self.payload.is_none()}
 }
@@ -23,8 +30,8 @@
 fn original_initializer_job_keeps_source_authority_cancel_arc_and_full_receipts(){
  let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let grant:RetainedCloneGrant=serde_json::from_value(law["grant"].clone()).unwrap();
- for size in[0,1,8192]{let mut original=String::with_capacity(size);original.extend(std::iter::repeat_n('x',size));assert_eq!(serde_json::to_value(&original).unwrap(),serde_json::Value::String("x".repeat(size)));let pointer=original.as_ptr()as usize;let mut born=original.capacity()+size_of::<OriginalAuthority>()+semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<std::sync::atomic::AtomicBool>();let mut job=ArtifactStoreInitializationJob::new(Box::new(OriginalAuthority{payload:Some(ControlledRetirement::new(original).ok().unwrap()),pointer,closing:false}));let mut released=0;let mut turns=0;
-  while !job.terminal_is_empty(){let(demand,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.retirement_demands(grant.maximum_copy_bytes).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut denials=vec![RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}];if demand.copy_bytes>0{denials.push(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant});}if demand.capacity_bytes>0{denials.push(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant});}if demand.release_bytes>0{denials.push(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant});}for denied in denials{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||InteractiveJob::close_step(&mut job,denied));assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(!job.terminal_is_empty());}
-   let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||InteractiveJob::close_step(&mut job,grant));assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;turns+=1;assert!(turns<100000);
-  }assert_eq!(born,released);let((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(job));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));println!("[DEBUG] original initializer size={size} turns={turns} physical={released} unchanged5currencies originalBytes samePointer authorityFrame and cancelArc separate terminalDrop0");
+ for size in[0,1,8192]{let mut source=crate::app_testing::OriginalWorkFixture::new();let mut original=String::with_capacity(size);original.extend(std::iter::repeat_n('x',size));assert_eq!(serde_json::to_value(&original).unwrap(),serde_json::Value::String("x".repeat(size)));let pointer=original.as_ptr()as usize;let mut born=original.capacity()+size_of::<OriginalAuthority>()+semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<std::sync::atomic::AtomicBool>();let mut job=ArtifactStoreInitializationJob::new(Box::new(OriginalAuthority{payload:Some(ControlledRetirement::new(original).ok().unwrap()),pointer,closing:false}));let mut released=0;let mut turns=0;
+  while !job.terminal_is_empty(){let(demand,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.retirement_demands(grant.maximum_copy_bytes).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut denials=vec![RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}];if demand.copy_bytes>0{denials.push(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant});}if demand.capacity_bytes>0{denials.push(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant});}if demand.release_bytes>0{denials.push(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant});}for denied in denials{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||source.turn(denied,|cx|InteractiveJob::close_step(&mut job,cx)).expect("original initializer denied close"));assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(!job.terminal_is_empty());}
+   let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||source.turn(grant,|cx|InteractiveJob::close_step(&mut job,cx)).expect("original initializer funded close"));assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;turns+=1;assert!(turns<100000);
+  }assert_eq!(born,released);source.finish();let((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(job));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));println!("[DEBUG] original initializer size={size} turns={turns} physical={released} unchanged5currencies originalBytes samePointer authorityFrame and cancelArc separate terminalDrop0");
  }
 }


### 11 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️testing/🔗️dependency-contribution/🧬️mutations/➕️add-value/🦀️.rs
B/H/S/current: 56/58/69/65
--- 
+++ 
@@ -18,4 +18,15 @@
 impl protocol::MutationKind<DependencyTestSnapshot, DependencyTestOp> for AddValue {
     const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "value", kind: "add-value", record: "AddedValue" };
+
+
+    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
+        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add {} to value", self.delta), &format!("{} zu Wert hinzufügen", self.delta))
+    }
+    fn target(&self) -> Vec<String> {
+        vec!["value".into()]
+    }
+}
+
+impl protocol::ColdMutationKind<DependencyTestSnapshot, DependencyTestOp> for AddValue {
     fn diff(&self, _base: &DependencyTestSnapshot) -> protocol::MutationOutcome<DependencyTestDiff> {
         protocol::MutationOutcome::new(DependencyTestDiff { deltas: vec![self.delta] })
@@ -30,17 +41,9 @@
         })())
     }
-    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
-        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add {} to value", self.delta), &format!("{} zu Wert hinzufügen", self.delta))
-    }
-    fn target(&self) -> Vec<String> {
-        vec!["value".into()]
-    }
 }
 
 impl protocol::CompositeMutationKind<DependencyTestSnapshot, DependencyTestOp> for AddValue {
     const SEMANTICS: protocol::SemanticDescriptor = <Self as protocol::MutationKind<DependencyTestSnapshot, DependencyTestOp>>::SEMANTICS;
-    fn plan(&self, _base: &DependencyTestSnapshot, planner: &mut protocol::Planner<DependencyTestSnapshot, DependencyTestOp>) -> Result<(), protocol::PlanError> {
-        planner.call(DependencyTestOp::AddValue(self.clone()))
-    }
+
     fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
         <Self as protocol::MutationKind<DependencyTestSnapshot, DependencyTestOp>>::label(self)
@@ -51,6 +54,16 @@
 }
 
+impl protocol::ColdCompositeMutationKind<DependencyTestSnapshot, DependencyTestOp> for AddValue {
+    fn plan(&self, _base: &DependencyTestSnapshot, planner: &mut protocol::Planner<DependencyTestSnapshot, DependencyTestOp>) -> Result<(), protocol::PlanError> {
+        planner.call(DependencyTestOp::AddValue(self.clone()))
+    }
+}
+
 #[cfg(test)]
 #[path = "../../../../🧪️tests/➕️dependency-contribution-add-value-unit/🦀️.rs"]
 mod tests;
 //#endregion ➕️AddValue
+
+#[cfg(test)]
+#[path = "🧪️tests/🧪️sum-law/🦀️.rs"]
+mod sum_law_tests;


### 12 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🦀️.rs
B/H/S/current: 1062/77/1055/77
--- 
+++ 
@@ -11,6 +11,4 @@
 use semio_framework_job::JobOutcomeView;
 use semio_framework_job::JobPayloadStream;
-use semio_framework_job_publication::JobPublicationKind;
-use semio_framework_job_publication::RetainedJobPublication;
 use semio_framework_job::RetainedPayloadBuilder;
 use semio_framework_value::retained_clone::RetainedCloneProgress;
@@ -301,23 +299,5 @@
 }
 
-#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
-#[value(rename_all = "camelCase")]
-pub enum TypedOperationResultLane {
-    Artifact,
-    Config,
-    Draft,
-    Presence,
-    Transient,
-    WindowConfig,
-    WindowTransient,
-    Child,
-    Interaction,
-    Effect,
-    Event,
-    Ui,
-    Download,
-    Terminal,
-    Fault,
-}
+pub use semio_framework_actor::typed_operation_page::TypedOperationResultLane;
 
 pub(crate) struct OriginalAuthoringEffect {
@@ -659,5 +639,5 @@
     /// 👥️ Borrows immutable captured presence without projecting or granting publication authority.
     pub fn presence_view(&self) -> Option<PresenceView<'_, A::Presence>> {
-        Some(PresenceView { local: self.presence_local.as_ref()?.get(), peers: PresencePeersView { root: self.presence_peers.as_deref()? } })
+        Some(PresenceView { local: self.presence_local.as_ref()?.get(), peers: PresencePeersView::new(self.presence_peers.as_deref()?) })
     }
 
@@ -711,9 +691,15 @@
 }
 
+#[path="🎟️request/🔒️command/🦀️.rs"]
+mod original_command;
+pub use original_command::ArtifactOriginalToolCommand;
+
 /// 🎒️ Fully owned input handed to an app-defined factory builder after exact raw admission.
 pub struct ArtifactOwnedToolJobRequest<A: ArtifactApp> {
+    pub source_binding: semio_framework::action_bus::OriginalToolSourceBinding,
     pub retained:RetainedCloneGrant,
-    pub command: Box<A::Command>,
-    pub raw_wire: ArtifactToolRawInput,
+    pub command: ArtifactOriginalToolCommand<A::Command>,
+    pub input: Option<semio_framework::action_bus::RetainedToolWireInput>,
+    pub checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
     pub operation: semio_framework_job::Operation,
     pub controller_id: String,
@@ -739,4 +725,7 @@
     pub completion: ArtifactToolCompletion<A>,
 }
+#[path="🎟️request/♻️retirement/🦀️.rs"]
+mod original_request_retirement;
+
 
 /// 🧭️ Owned input for an app-defined implementation of a framework-reserved route. The runtime
@@ -803,8 +792,12 @@
     }
 
-    pub fn bind_operation(&mut self,operation:semio_framework_job::Operation)->Result<(),semio_framework::ToolJobFactoryError>{
-        let source=self.state.as_ref().ok_or_else(||semio_framework::ToolJobFactoryError::new("reserved job state already returned"))?;
-        let mut state=source.try_lock().map_err(|_|semio_framework::ToolJobFactoryError::new("reserved job operation authority is busy"))?;
-        if state.operation.is_some(){return Err(semio_framework::ToolJobFactoryError::new("reserved job operation authority was already bound"))}
+    #[cfg(test)]
+    pub(crate) fn test_source_identity(&self)->Option<usize>{self.state.as_ref().map(|state|std::sync::Arc::as_ptr(state)as usize)}
+    #[cfg(test)]
+    pub(crate) fn test_bound_operation(&self)->Option<semio_framework_job::Operation>{self.state.as_ref()?.try_lock().ok()?.operation}
+    pub fn bind_operation(&mut self,operation:semio_framework_job::Operation)->Result<(),ValueError>{
+        let source=self.state.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"reserved job state already returned"))?;
+        let mut state=source.try_lock().map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"reserved job operation authority is busy"))?;
+        if state.operation.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"reserved job operation authority was already bound"))}
         state.operation=Some(operation);Ok(())
     }
@@ -843,5 +836,5 @@
         use semio_framework_job::InteractiveJobCloseStep as Step;
         let Some(source)=self.state.as_ref()else{return Ok(Step::Complete{progress:Default::default()})};
-        if cx.should_yield(){return Ok(Step::Pending{progress:Default::default()})}let grant=cx.retained_grant();
+        if cx.retirement_should_yield(){return Ok(Step::Pending{progress:Default::default()})}let grant=cx.retained_grant();
         let mut state=match source.try_lock(){Ok(state)=>state,Err(_)=>return Ok(Step::Blocked)};
         if let Some(inner)=state.inner.as_mut(){


### 13 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🧩️app/🦀️.rs
B/H/S/current: 1167/159/1167/159
--- 
+++ 
@@ -111,5 +111,5 @@
 use semio_framework_plugin_decl_emission::HostEvent;
 use semio_framework_plugin_decl_views::AppRenderOperationContext;
-use semio_framework_plugin_decl_views::ArtifactView;
+use semio_framework_plugin_decl_views::{ArtifactView,ArtifactPendingView,MountedSnapshotReadPreparation};
 use semio_framework_plugin_decl_views::ConfigView;
 use semio_framework_plugin_decl_views::DraftView;
@@ -236,6 +236,6 @@
     /// 🔍️ Requests an opaque immutable snapshot lease only when the mounted owner has
     /// admitted genuinely new work. The default path issues no lease.
-    fn mounted_job_prepare_snapshot_read(_operation: AppRenderOperationContext, _snapshot: &Self::Snapshot) -> bool {
-        false
+    fn mounted_job_prepare_snapshot_read(_operation: AppRenderOperationContext, _snapshot: &Self::Snapshot, _windows: &WindowConfigOwnerRegistry, _view: Option<&ViewModel>, _grant: RetainedCloneGrant) -> crate::MountedSnapshotReadPreparation {
+        crate::MountedSnapshotReadPreparation::default()
     }
     /// 🧩️ Registers only app-owned exact factories. The empty default contributes no execution
@@ -302,6 +302,6 @@
     /// 🏗️ Builds one payload-owning operation for an explicitly registered app factory. Returning
     /// `None` is fail-closed; the framework never substitutes its bounded-first-step factory.
-    async fn build_tool_job(_request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
-        Ok(None)
+    fn build_tool_job(_request:&mut Option<ArtifactOwnedToolJobRequest<Self>>,_output:&mut Option<semio_framework::ErasedToolJob>,_cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
+        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"registered app tool lacks its original source-preserving producer").into())
     }
     /// 🧭️ Builds a persistent producer/import job for one explicitly registered reserved route.
@@ -655,5 +655,5 @@
     /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). `None` means no surface is mounted yet: an app
     /// with window-scoped work must arm NOTHING and wait for the first `Event::SurfaceVisible`.
-    async fn pending_effects(_owner: &ArtifactInstanceOperationOwnerHandle, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view: Option<&ViewModel>) -> Vec<Effect> {
+    async fn pending_effects(_owner: &ArtifactInstanceOperationOwnerHandle, _doc: &crate::ArtifactPendingView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view: Option<&ViewModel>) -> Vec<Effect> {
         Vec::new()
     }


### 14 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🪟️surface/🦀️.rs
B/H/S/current: 2093/185/2093/185
--- 
+++ 
@@ -126,5 +126,5 @@
 use semio_framework_plugin_decl_emission::HostEvent;
 use semio_framework_plugin_decl_views::AppRenderOperationContext;
-use semio_framework_plugin_decl_views::ArtifactView;
+use semio_framework_plugin_decl_views::{ArtifactView,ArtifactPendingView,MountedSnapshotReadPreparation};
 use semio_framework_plugin_decl_views::ConfigView;
 use semio_framework_plugin_decl_views::DraftView;
@@ -236,6 +236,6 @@
     }
 
-    fn mounted_job_prepare_snapshot_read(_operation: AppRenderOperationContext, _snapshot: &Self::Snapshot) -> bool {
-        false
+    fn mounted_job_prepare_snapshot_read(_operation: AppRenderOperationContext, _snapshot: &Self::Snapshot, _windows: &WindowConfigOwnerRegistry, _view: Option<&ViewModel>, _grant: RetainedCloneGrant) -> crate::MountedSnapshotReadPreparation {
+        crate::MountedSnapshotReadPreparation::default()
     }
 
@@ -312,6 +312,6 @@
 
     /// 🏗️ Author-owned payload builder paired with `register_tool_job_factories`.
-    fn build_tool_job(_request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
-        Ok(None)
+    fn build_tool_job(_request:&mut Option<ArtifactOwnedToolJobRequest<EditorApp<Self>>>,_output:&mut Option<semio_framework::ErasedToolJob>,_cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
+        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"registered surface tool lacks its original source-preserving producer").into())
     }
 
@@ -518,5 +518,5 @@
         Ok(Vec::new())
     }
-    fn pending_effects(_owner: &ArtifactInstanceOperationOwnerHandle, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view: Option<&ViewModel>) -> Vec<Effect> {
+    fn pending_effects(_owner: &ArtifactInstanceOperationOwnerHandle, _doc: &crate::ArtifactPendingView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view: Option<&ViewModel>) -> Vec<Effect> {
         Vec::new()
     }
@@ -920,6 +920,6 @@
     }
 
-    fn mounted_job_prepare_snapshot_read(_operation: AppRenderOperationContext, _snapshot: &Self::Snapshot) -> bool {
-        false
+    fn mounted_job_prepare_snapshot_read(_operation: AppRenderOperationContext, _snapshot: &Self::Snapshot, _windows: &WindowConfigOwnerRegistry, _view: Option<&ViewModel>, _grant: RetainedCloneGrant) -> crate::MountedSnapshotReadPreparation {
+        crate::MountedSnapshotReadPreparation::default()
     }
 
@@ -985,6 +985,6 @@
 
     /// 🏗️ Author-owned payload builder paired with `register_tool_job_factories`.
-    fn build_tool_job(_request: ArtifactOwnedToolJobRequest<ViewerApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
-        Ok(None)
+    fn build_tool_job(_request:&mut Option<ArtifactOwnedToolJobRequest<ViewerApp<Self>>>,_output:&mut Option<semio_framework::ErasedToolJob>,_cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
+        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"registered surface tool lacks its original source-preserving producer").into())
     }
 
@@ -1231,6 +1231,6 @@
         E::mounted_jobs_terminal_is_empty(instance_id)
     }
-    fn mounted_job_prepare_snapshot_read(operation: AppRenderOperationContext, snapshot: &Self::Snapshot) -> bool {
-        E::mounted_job_prepare_snapshot_read(operation, snapshot)
+    fn mounted_job_prepare_snapshot_read(operation: AppRenderOperationContext, snapshot: &Self::Snapshot, windows: &WindowConfigOwnerRegistry, view: Option<&ViewModel>, grant: RetainedCloneGrant) -> crate::MountedSnapshotReadPreparation {
+        E::mounted_job_prepare_snapshot_read(operation, snapshot, windows, view, grant)
     }
     fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> {
@@ -1264,6 +1264,6 @@
         E::build_instance_operation_owner()
     }
-    async fn build_tool_job(request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
-        E::build_tool_job(request)
+    fn build_tool_job(request:&mut Option<ArtifactOwnedToolJobRequest<Self>>,output:&mut Option<semio_framework::ErasedToolJob>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
+        E::build_tool_job(request,output,cx)
     }
     fn build_reserved_tool_job(request: ArtifactReservedToolJobRequest<Self>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
@@ -1443,5 +1443,5 @@
         E::paste_operations(doc, fragment, placement)
     }
-    async fn pending_effects(owner: &ArtifactInstanceOperationOwnerHandle, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view: Option<&ViewModel>) -> Vec<Effect> {
+    async fn pending_effects(owner: &ArtifactInstanceOperationOwnerHandle, doc: &crate::ArtifactPendingView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view: Option<&ViewModel>) -> Vec<Effect> {
         E::pending_effects(owner, doc, cfg, view)
     }
@@ -1654,6 +1654,6 @@
         V::mounted_jobs_terminal_is_empty(instance_id)
     }
-    fn mounted_job_prepare_snapshot_read(operation: AppRenderOperationContext, snapshot: &Self::Snapshot) -> bool {
-        V::mounted_job_prepare_snapshot_read(operation, snapshot)
+    fn mounted_job_prepare_snapshot_read(operation: AppRenderOperationContext, snapshot: &Self::Snapshot, windows: &WindowConfigOwnerRegistry, view: Option<&ViewModel>, grant: RetainedCloneGrant) -> crate::MountedSnapshotReadPreparation {
+        V::mounted_job_prepare_snapshot_read(operation, snapshot, windows, view, grant)
     }
     fn register_window_transient_owners(registry: &mut WindowTransientOwnerRegistry) -> Result<(), Fault> {
@@ -1690,6 +1690,6 @@
         V::build_instance_operation_owner()
     }
-    async fn build_tool_job(request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
-        V::build_tool_job(request)
+    fn build_tool_job(request:&mut Option<ArtifactOwnedToolJobRequest<Self>>,output:&mut Option<semio_framework::ErasedToolJob>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
+        V::build_tool_job(request,output,cx)
     }
     fn build_reserved_tool_job(request: ArtifactReservedToolJobRequest<Self>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
@@ -1782,5 +1782,5 @@
         V::render_with_request_context(owner, body_key, doc, cfg, view_state, transient, interaction,cx)
     }
-    async fn pending_effects(owner: &ArtifactInstanceOperationOwnerHandle, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view: Option<&ViewModel>) -> Vec<Effect> {
+    async fn pending_effects(owner: &ArtifactInstanceOperationOwnerHandle, doc: &crate::ArtifactPendingView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view: Option<&ViewModel>) -> Vec<Effect> {
         V::pending_effects(owner, doc, cfg, view)
     }
@@ -2021,11 +2021,5 @@
 );
 
-/// 🏗️ A generic [`ArtifactBuilder`] for a subset with no custom build logic — everything is one
-/// snapshot value and the ordinary `Mutation`/`MutationDiff` algebra, so a trivial subset writes
-/// zero builder boilerplate (Task 3): `type Construction = SnapshotBuilder<Snapshot, Mutation>;`
-/// where old code hand-wrote its own `empty`/`from_snapshot`/`from_text`/`from_binary`/`mutate`/
-/// `absorb`/`build`. `mutate` reuses `store::os_store::replay::apply_outcome` (this crate's own established
-/// "apply once, fold a rejection into a Fatal message" idiom) rather than re-deriving it, matching
-/// the trait's own doc: "the builder applies `outcome.diff()` once and returns that outcome intact".
+/// ❄️ Owns wire DTOs for explicitly selected reference decisions and synchronous diff application.
 pub struct SnapshotBuilder<S, M> {
     snapshot: S,
@@ -2047,5 +2041,5 @@
     type Snapshot = S;
     type Mutation = M;
-    type Diff = M::Diff;
+    type Diff = M::ReferenceDiff;
 
     fn empty() -> Self {
@@ -2061,13 +2055,15 @@
         Ok(Self::wrap(S::decode_pack(bytes)?))
     }
-    fn mutate(self, mutation: M) -> (Self, protocol::MutationOutcome<M::Diff>) {
+    fn mutate(self, mutation: M) -> (Self, protocol::MutationOutcome<M::ReferenceDiff>) {
         let (snapshot, outcome) = store::os_store::replay::apply_outcome(&self.snapshot, mutation.diff(&self.snapshot));
+        drop(mutation);
+        drop(self.snapshot);
         (Self::wrap(snapshot), outcome)
     }
-    fn absorb(self, diff: M::Diff) -> protocol::MutationApplyResult<Self> {
-        match protocol::apply_diff(&diff, &self.snapshot) {
-            Ok(snapshot) => Ok(Self::wrap(snapshot)),
-            Err(error) => Err(error),
-        }
+    fn absorb(self, diff: M::ReferenceDiff) -> protocol::MutationApplyResult<Self> {
+        let applied = protocol::apply_diff_cold(&diff, &self.snapshot);
+        drop(diff);
+        drop(self.snapshot);
+        applied.map(Self::wrap)
     }
     fn build(self) -> Result<S, Vec<semio_framework_diagnostic::Diagnostic>> {
@@ -2075,4 +2071,8 @@
     }
 }
+
+#[cfg(test)]
+#[path = "🏗️snapshot-builder/🧪️tests/🦀️.rs"]
+mod snapshot_builder_ownership_tests;
 
 fn subset_example_table() -> &'static std::sync::Mutex<HashMap<Dialect, Vec<ExampleSource>>> {


### 15 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml
B/H/S/current: 216/220/230/233
--- 
+++ 
@@ -21,4 +21,17 @@
 [lib]
 path = "🦀️.rs"
+
+[[test]]
+name = "snapshot_builder_ownership"
+path = "../../📜️declaration/🪟️surface/🏗️snapshot-builder/🧪️tests/🚪️public/🦀️.rs"
+
+[[test]]
+name = "mounted_snapshot_preparation"
+path = "../../📜️declaration/👁️view/📸️preparation/🧪️tests/🚪️public/🦀️.rs"
+required-features = ["artifact-app-testing"]
+
+[[test]]
+name = "original_tick_payload_sources"
+path = "../../⏯️tool-run/🎞️tick/📦️payload/🧪️tests/🚪️public/🦀️.rs"
 
 [features]
@@ -187,8 +200,9 @@
 
 [dev-dependencies]
+semio-framework-value-snapshot-read = { path = "../../../../../../🔨️modules/🌱️value/📸️snapshot/👁️read/📦️packages/🦀️rust" }
 semio-framework-job-worker = { path = "../../../../../../🔨️modules/🧵️job/👷️worker/📦️packages/🦀️rust", features = ["testkit"] }
 
 semio-framework-job = { path = "../../../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust", package = "semio-framework-job", features = ["testkit"] }
-semio-framework-os-kernel = { path = "../../../../📦️packages/🦀️rust", package = "semio-framework-os-kernel", features = ["mutation-testing"] }
+semio-framework-os-kernel = { path = "../../../../📦️packages/🦀️rust", package = "semio-framework-os-kernel", features = ["mutation-testing", "protocol-laws"] }
 semio-framework-async-macros = { path = "../../../../../../🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust" }
 # 🚪️ Test-only `entrypoint`: `one_framework_reserved_route_fits_a_bounded_thread_stack` drives one


### 16 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️.rs
B/H/S/current: 1432/82/1433/82
--- 
+++ 
@@ -440,5 +440,5 @@
         let grant=cx.retained_grant();
         if self.0.is_none(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
-        if cx.should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
+        if cx.retirement_should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
         let demand=self.close_demands(grant.maximum_copy_bytes).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?;
         if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
@@ -448,5 +448,6 @@
             cx.consume_retained(progress)?;drop(self.0.take());cx.consume_fuel(1);return Ok(InteractiveJobCloseStep::Complete{progress})
         }
-        let step=original.close_step(cx)?.admit(grant,original.terminal_is_empty()).map_err(OriginalTurnFailure::from)?;
+        let child=semio_framework_value::RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
+        let step=cx.with_retained_loan(child,|cx|original.close_step(cx))?.admit(grant,original.terminal_is_empty()).map_err(OriginalTurnFailure::from)?;
         Ok(match step{InteractiveJobCloseStep::Complete{progress}=>InteractiveJobCloseStep::Pending{progress},step=>step})
     }
@@ -531,5 +532,5 @@
     fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
         self.begin_close();
-        let grant=cx.retained_grant();if cx.should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
+        let grant=cx.retained_grant();if cx.retirement_should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
         let step=close_router_effect_sources(&mut self.writer,&mut self.detail,grant).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?;
         cx.consume_retained(step.progress())?;if step.progress().copied_items!=0{cx.consume_fuel(1)}Ok(step)
@@ -587,5 +588,5 @@
     fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
         self.begin_close();
-        let grant=cx.retained_grant();if cx.should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
+        let grant=cx.retained_grant();if cx.retirement_should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
         let step=close_router_effect_sources(&mut self.writer,&mut self.output,grant).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?;
         cx.consume_retained(step.progress())?;if step.progress().copied_items!=0{cx.consume_fuel(1)}Ok(step)


### 17 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🧪️tests/🔬️unit/🦀️.rs
B/H/S/current: 489/492/510/514
--- 
+++ 
@@ -1,3 +1,12 @@
 use super::*;
+
+const ROUTER_ORIGINAL_STOCK:semio_framework_job::RetainedCloneGrant=semio_framework_job::RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:4096};
+struct OriginalRouterFixture{caller:semio_framework_job::OriginalJobFixtureCaller,decoder:semio_framework_job::OriginalJobDecodeOwner}
+impl OriginalRouterFixture{
+ fn new()->Self{Self{caller:semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(7301),semio_framework_job::Generation(17),ROUTER_ORIGINAL_STOCK,||Some(1),u64::MAX,1<<20),decoder:semio_framework_job::OriginalJobDecodeOwner::empty()}}
+ fn close<J:InteractiveJob>(&mut self,job:&mut J,grant:semio_framework_job::RetainedCloneGrant)->InteractiveJobCloseStep{self.caller.turn_with_decoder(grant,&mut self.decoder,|cx|cx.with_original_retirement(|cx|job.close_step(cx))).expect("router original close preserves complete failure")}
+ fn finish(mut self){for _ in 0..4096{if self.decoder.terminal_is_empty(){break}self.caller.turn(ROUTER_ORIGINAL_STOCK,|cx|{let demand=self.decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|loan|self.decoder.close_step(loan))?;Ok(())}).expect("router original decoder close");}assert!(self.decoder.terminal_is_empty());self.caller.retire(ROUTER_ORIGINAL_STOCK);}
+}
+
 
 fn fixture_router_grant()->semio_framework_job::RetainedCloneGrant{let law:serde_json::Value=serde_json::from_str(include_str!("../../../../../🛎️services/🧪️tests/🧫️fixtures/🧮️compute-retained/🔣️.json")).unwrap();serde_json::from_value(law["callerGrant"].clone()).unwrap()}
@@ -73,5 +82,5 @@
     fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
         self.begin_close();
-        let grant=cx.retained_grant();if cx.should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
+        let grant=cx.retained_grant();if cx.retirement_should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
         use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep,RetirementTurnError};
         if let Some(writer)=self.writer.as_mut(){if writer.terminal_is_empty(){let copied_bytes=std::mem::size_of::<Option<RetainedPayloadBuilder>>();if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_depth==0{return Ok(InteractiveJobCloseStep::Blocked)}let progress=RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()};cx.consume_retained(progress)?;drop(self.writer.take());cx.consume_fuel(1);return Ok(InteractiveJobCloseStep::Pending{progress})}let step=writer.close_step_granted(grant).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?;cx.consume_retained(step.progress())?;cx.consume_fuel(1);return Ok(InteractiveJobCloseStep::Pending{progress:step.progress()})}
@@ -413,4 +422,5 @@
 fn router_effect_original_sources_keep_capacity_until_funded_close() {
     fn verify<J:InteractiveJob>(mut job:J,source:fn(&J)->&Option<Vec<u8>>,text:&str,grant:semio_framework_value::RetainedCloneGrant){
+        let mut original=OriginalRouterFixture::new();
         let pointer=source(&job).as_ref().unwrap().as_ptr();let capacity=source(&job).as_ref().unwrap().capacity();
         job.begin_close();let mut released=0;let mut copies=0;let mut turns=0;
@@ -421,16 +431,17 @@
                 let mut denied=grant;
                 match axis{0=>denied.maximum_items=0,1 if copy>0=>denied.maximum_copy_bytes=copy-1,2 if release>0=>denied.maximum_release_bytes=release-1,3 if depth>0=>denied.maximum_depth=depth-1,_=>continue}
-                let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.close_step(denied));
+                let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original.close(&mut job,denied));
                 assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
                 assert!(!matches!(step,InteractiveJobCloseStep::Complete{..}));
                 assert_eq!(source(&job).as_ref().unwrap().as_ptr(),pointer);assert_eq!(source(&job).as_deref().unwrap(),text.as_bytes());
             }
-            let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.close_step(grant));
+            let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original.close(&mut job,grant));
             assert!(step.progress().fits(grant));assert_eq!(step.progress().retained_capacity_bytes,heap.requested_bytes);assert_eq!(step.progress().released_bytes,heap.released_bytes);
-            assert!(!matches!(step,InteractiveJobCloseStep::Refused{..}|InteractiveJobCloseStep::Blocked));
+            assert!(!matches!(step,InteractiveJobCloseStep::Blocked));
             released+=step.progress().released_bytes;copies+=step.progress().copied_bytes;turns+=1;assert!(turns<16);
         }
         assert_eq!(released,capacity);assert!(copies>=std::mem::size_of::<Option<Vec<u8>>>());
         let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(job));assert_eq!(heap.released_bytes,0);
+        original.finish();
         let oracle:Vec<u8>=serde_json::from_str(&serde_json::to_string(text.as_bytes()).unwrap()).unwrap();assert_eq!(oracle,text.as_bytes());
         eprintln!("[DEBUG] original router source length{} capacity{capacity} close{turns} copied{copies} released{released} drop0",text.len());
@@ -449,4 +460,5 @@
 #[test]
 fn router_effect_original_box_frame_has_a_separate_funded_terminal_turn(){
+    let mut original_source=OriginalRouterFixture::new();
     let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
     let grant:semio_framework_value::RetainedCloneGrant=serde_json::from_value(fixture["grant"].clone()).unwrap();
@@ -463,10 +475,11 @@
             saw_frame=true;assert_eq!(demand.release_bytes,frame);assert_eq!(demand.depth,1);
             let denied=semio_framework_value::RetainedCloneGrant{maximum_release_bytes:frame-1,..grant};
-            let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(denied));assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(!owner.terminal_is_empty());assert_eq!((&**owner.0.as_ref().unwrap()as *const dyn InteractiveJob).cast::<()>(),pointer);
-        }
-        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(grant));assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(!matches!(step,InteractiveJobCloseStep::Refused{..}|InteractiveJobCloseStep::Blocked));released+=heap.released_bytes;
+            let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original_source.close(&mut owner,denied));assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(!owner.terminal_is_empty());assert_eq!((&**owner.0.as_ref().unwrap()as *const dyn InteractiveJob).cast::<()>(),pointer);
+        }
+        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original_source.close(&mut owner,grant));assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(!matches!(step,InteractiveJobCloseStep::Blocked));released+=heap.released_bytes;
         if terminal{assert!(matches!(step,InteractiveJobCloseStep::Complete{..}));}
     }
     assert!(saw_frame&&owner.terminal_is_empty());assert_eq!(released,source_capacity+frame);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!(heap.released_bytes,0);
+    original_source.finish();
     eprintln!("[DEBUG] original router Box frame{frame} source{source_capacity} released{released} separate terminal frame funded drop0; factory birth unqualified");
 }
@@ -476,14 +489,22 @@
     let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let grant:semio_framework_value::RetainedCloneGrant=serde_json::from_value(fixture["grant"].clone()).unwrap();
     for case in fixture["recordingLeases"].as_array().unwrap(){
+        let mut original=OriginalRouterFixture::new();
         let source=Arc::new(AtomicUsize::new(7));let bytes=semio_framework_value::shared_retirement_allocation_bytes::<AtomicUsize>();
         let mut alias=(case=="alias").then(||RecordingRouterJob{calls:Some(source.clone()),yielded:false,closing:false,cancelled:false,completed:false,cursor:0,writer:None});let weak=(case=="weak").then(||Arc::downgrade(&source));let mut owner=RecordingRouterJob{calls:Some(source),yielded:false,closing:false,cancelled:false,completed:false,cursor:0,writer:None};owner.begin_close();
         let demand=owner.close_demands();
-        for denied in [semio_framework_value::RetainedCloneGrant{maximum_items:0,..grant},semio_framework_value::RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant},semio_framework_value::RetainedCloneGrant{maximum_release_bytes:bytes-1,..grant},semio_framework_value::RetainedCloneGrant{maximum_depth:0,..grant}]{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(denied));assert_eq!(step.progress(),Default::default());assert!(!owner.terminal_is_empty());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
-        if weak.is_some(){let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(grant));assert_eq!(step.progress(),Default::default());assert!(!owner.terminal_is_empty());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
+        for denied in [semio_framework_value::RetainedCloneGrant{maximum_items:0,..grant},semio_framework_value::RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant},semio_framework_value::RetainedCloneGrant{maximum_release_bytes:bytes-1,..grant},semio_framework_value::RetainedCloneGrant{maximum_depth:0,..grant}]{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original.close(&mut owner,denied));assert_eq!(step.progress(),Default::default());assert!(!owner.terminal_is_empty());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
+        if weak.is_some(){let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original.close(&mut owner,grant));assert_eq!(step.progress(),Default::default());assert!(!owner.terminal_is_empty());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
         let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(weak));assert_eq!(heap.released_bytes,0);
         assert_eq!(owner.calls.as_ref().unwrap().load(Ordering::SeqCst),serde_json::from_str::<usize>("7").unwrap());
-        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(grant));assert!(owner.terminal_is_empty());assert!(step.progress().fits(grant));assert_eq!(step.progress().released_bytes,heap.released_bytes);let mut released=heap.released_bytes;
-        if let Some(alias)=alias.as_mut(){assert_eq!(released,0);alias.begin_close();let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||alias.close_step(grant));assert!(alias.terminal_is_empty());assert_eq!(step.progress().released_bytes,heap.released_bytes);released+=heap.released_bytes;}
-        assert_eq!(released,bytes);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{drop(owner);drop(alias);});assert_eq!(heap.released_bytes,0);eprintln!("[DEBUG] original recording lease {} backing{bytes} released{released} weak-gated terminal drop0",case.as_str().unwrap());
-    }
-}
+        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original.close(&mut owner,grant));assert!(owner.terminal_is_empty());assert!(step.progress().fits(grant));assert_eq!(step.progress().released_bytes,heap.released_bytes);let mut released=heap.released_bytes;
+        if let Some(alias)=alias.as_mut(){assert_eq!(released,0);alias.begin_close();let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original.close(alias,grant));assert!(alias.terminal_is_empty());assert_eq!(step.progress().released_bytes,heap.released_bytes);released+=heap.released_bytes;}
+        assert_eq!(released,bytes);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{drop(owner);drop(alias);});assert_eq!(heap.released_bytes,0);original.finish();eprintln!("[DEBUG] original recording lease {} backing{bytes} released{released} weak-gated terminal drop0",case.as_str().unwrap());
+    }
+}
+
+#[test]
+fn original_retirement_loan_closes_router_after_semantic_deadline(){
+ let mut original=OriginalRouterFixture::new();let mut job=FaultRouterEffectJob{detail:Some(Vec::with_capacity(4096)),writer:None,cursor:0,closing:true};
+ original.caller.turn_with_decoder(ROUTER_ORIGINAL_STOCK,&mut original.decoder,|cx|{cx.with_original_work_loan(64,0,|cx|{assert!(cx.should_yield());cx.with_original_retirement(|cx|{assert!(!cx.retirement_should_yield());job.close_step(cx)?;Ok(())})})}).unwrap();
+ assert!(job.terminal_is_empty());drop(job);original.finish();
+}


### 18 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🎟️grant/🧪️tests/🟦️.ts
B/H/S/current: 16/18/23/18
--- 
+++ 
@@ -4,13 +4,20 @@
 
 test("one original Host authority survives only-resource overrides and one neutral wire header",()=>{
- const law=fixture.dispatch,validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,dispatch:{...law,overrideAuthorityAccessible:true}})).toBe(false);expect(validate({...fixture,dispatch:{...law,grantWireAuthorityCount:2}})).toBe(false);
+ const law=fixture.dispatch,validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,dispatch:{...law,overrideAuthorityAccessible:true}})).toBe(false);expect(validate({...fixture,dispatch:{...law,grantWireAuthorityCount:2}})).toBe(false);expect(validate({...fixture,input:{...fixture.input,work:{...fixture.input.work,fuel:65}}})).toBe(false);expect(validate({...fixture,loan:{...fixture.loan,workFuel:65}})).toBe(false);
  const db=new Database(":memory:");try{db.run("CREATE TABLE authority(operation INTEGER,generation INTEGER,epoch INTEGER,items INTEGER,copy INTEGER,capacity INTEGER,release INTEGER,depth INTEGER,fuel INTEGER,wall INTEGER,memory INTEGER,nodes INTEGER,mailbox INTEGER,effects INTEGER,patch INTEGER)");const before=law.resourcesBefore,after=law.resourcesAfter;db.query("INSERT INTO authority VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)").run(71,3,19,1,0,128,97,8,before.fuel,before.wallMs,before.memoryBytes,before.uiNodes,before.mailboxLen,before.maxEffects,before.maxPatchBytes);db.query("UPDATE authority SET fuel=?,wall=?,memory=?,nodes=?,mailbox=?,effects=?,patch=? WHERE operation=?").run(after.fuel,after.wallMs,after.memoryBytes,after.uiNodes,after.mailboxLen,after.maxEffects,after.maxPatchBytes,71);expect(db.query("SELECT operation,generation,epoch,items,copy,capacity,release,depth FROM authority").get()).toEqual({operation:71,generation:3,epoch:19,items:1,copy:0,capacity:128,release:97,depth:8})}finally{db.close()}
- const buffer=Buffer.alloc(law.singleAuthorityWire.byteLength);let position=0;const u8=(value:number)=>{buffer.writeUInt8(value,position);position++},u16=(value:number)=>{buffer.writeUInt16LE(value,position);position+=2},u32=(value:number)=>{buffer.writeUInt32LE(value,position);position+=4},u64=(value:number)=>{buffer.writeBigUInt64LE(BigInt(value),position);position+=8};u8(2);u64(71);const budget=law.resourcesAfter;u64(budget.fuel);u32(budget.wallMs);u64(budget.memoryBytes);u32(budget.uiNodes);u16(budget.mailboxLen);u32(budget.maxEffects);u32(budget.maxPatchBytes);u32(budget.maxFrames);for(const value of[fixture.input.operation,fixture.input.generation,fixture.input.epoch,...Object.values(fixture.input.grant)])u64(value as number);u8(0);expect(position).toBe(law.singleAuthorityWire.byteLength);expect(buffer.toString("hex")).toBe(law.singleAuthorityWire.hex);
+ const buffer=Buffer.alloc(law.singleAuthorityWire.byteLength);let position=0;const u8=(value:number)=>{buffer.writeUInt8(value,position);position++},u16=(value:number)=>{buffer.writeUInt16LE(value,position);position+=2},u32=(value:number)=>{buffer.writeUInt32LE(value,position);position+=4},u64=(value:number)=>{buffer.writeBigUInt64LE(BigInt(value),position);position+=8};u8(2);u64(71);const budget=law.resourcesAfter;u64(budget.fuel);u32(budget.wallMs);u64(budget.memoryBytes);u32(budget.uiNodes);u16(budget.mailboxLen);u32(budget.maxEffects);u32(budget.maxPatchBytes);u32(budget.maxFrames);for(const value of[fixture.input.operation,fixture.input.generation,fixture.input.epoch,...Object.values(fixture.input.work),...Object.values(fixture.input.grant)])u64(value as number);for(const value of[...Object.values(fixture.loan.retainedPolicy),fixture.loan.workFuel,fixture.loan.deadlineUs])u64(value);u8(0);expect(position).toBe(law.singleAuthorityWire.byteLength);expect(buffer.toString("hex")).toBe(law.singleAuthorityWire.hex);
 
- const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8"),resources=source.slice(source.indexOf("pub struct ShardResourceBudget"),source.indexOf("impl ShardResourceBudget"));expect(resources.includes("RetainedTurnInput")).toBe(false);expect(resources.includes("retained")).toBe(false);expect(source.includes("pub fn apply(self, budget: &mut semio_framework_actor::Budget)")).toBe(true);const frame=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");expect(frame.includes("Grant { actor: ActorId, budget: semio_framework_actor::Budget, retained: semio_framework_actor::RetainedTurnInput, envelopes: Vec<Envelope> }")).toBe(true);expect(frame.includes("let mut budget = grant.budget;")).toBe(true);expect(frame.includes("resources.apply(&mut budget);")).toBe(true);expect(frame.includes("grant.original_input().pack_encode(out).await?")).toBe(false);expect(frame.includes("retained: semio_framework_actor::RetainedTurnInput::pack_decode")).toBe(true);const root=new URL("../../../../../../",import.meta.url);for(const path of["🖥️host/🎠️activation/🦀️.rs","🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎠️runtime/🦀️.rs"]){const runtime=readFileSync(new URL(path,root),"utf8");expect(runtime.includes("impl Fn(ActorId, semio_framework_plugin_host::shard::grant::ShardResourceBudget)")).toBe(true);expect(runtime.includes("ShardResourceBudget::from_original(&grant.budget)")).toBe(true)}console.log("[DEBUG] Independent Ajv/SQLite/Node Buffer resource-only Host callback preserves all8 original authority scalars and exact112byte one-header wire; owning Native behavior separate");
+ const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8"),resources=source.slice(source.indexOf("pub struct ShardResourceBudget"),source.indexOf("impl ShardResourceBudget"));expect(resources.includes("RetainedTurnInput")).toBe(false);expect(resources.includes("retained")).toBe(false);expect(source.includes("pub fn apply(self, budget: &mut semio_framework_actor::Budget)")).toBe(true);const frame=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");expect(frame.includes("Grant { actor: ActorId, budget: semio_framework_actor::Budget, retained: semio_framework_actor::RetainedTurnInput, retained_policy:semio_framework_value::RetainedCloneGrant,work_fuel:u64,deadline_us:u64, envelopes: Vec<Envelope> }")).toBe(true);expect(frame.includes("let mut budget = grant.budget;")).toBe(true);expect(frame.includes("resources.apply(&mut budget);")).toBe(true);expect(frame.includes("grant.original_input().pack_encode(out).await?")).toBe(false);expect(frame.includes("let retained=semio_framework_actor::RetainedTurnInput::pack_decode")).toBe(true);const root=new URL("../../../../../../",import.meta.url);for(const path of["🖥️host/🎠️activation/🦀️.rs","🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎠️runtime/🦀️.rs"]){const runtime=readFileSync(new URL(path,root),"utf8");expect(runtime.includes("impl Fn(ActorId, semio_framework_plugin_host::shard::grant::ShardResourceBudget)")).toBe(true);expect(runtime.includes("ShardResourceBudget::from_original(&grant.budget)")).toBe(true)}console.log("[DEBUG] Independent Ajv/SQLite/Node Buffer resource-only Host callback preserves original identity, five physical axes, clock/deadline/fuel and bounded loan in exact192byte one-header wire; owning Native behavior separate");
 });
-test("original Host Shard grants retain missing authority and use only the original issued input",()=>{const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,missing:{...fixture.missing,policyMinted:true}})).toBe(false);const db=new Database(":memory:");try{expect(db.query("SELECT ? AS operation,? AS generation,? AS epoch,? AS items,? AS copy,? AS capacity,? AS release,? AS depth").get(71,3,19,1,0,128,97,8)).toEqual({operation:71,generation:3,epoch:19,items:1,copy:0,capacity:128,release:97,depth:8})}finally{db.close()}expect(JSON.parse(JSON.stringify(fixture)).input).toEqual(fixture.input);const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");const start=source.indexOf("fn granted_budget("),end=source.indexOf("\n    }",start)+6;expect(source.slice(start,end).includes("Option<&semio_framework_actor::Budget>")).toBe(true);const lifecycle=readFileSync(new URL("../../🔁️lifecycle/🦀️.rs",import.meta.url),"utf8");expect(lifecycle.includes("budget: Option<semio_framework_actor::Budget>")).toBe(true);expect(source.includes("drive_unissued_authority")).toBe(true);expect(source.includes("type Target=semio_framework_actor::IssuedRetainedBatch")).toBe(true);const actor=readFileSync(new URL("../../../../../../../../🔨️modules/🎭️actor/🎟️retained-turn/📦️batch/🦀️.rs",import.meta.url),"utf8");expect(actor.includes("pub fn original_input(&self) -> &RetainedTurnInput")).toBe(true);expect(source.includes("budget.pack_encode(out).await?")).toBe(true);expect(source.includes("budget: semio_framework_actor::Budget::pack_decode(bytes, pos).await?")).toBe(true);expect(source.includes("or_insert_with(|| semio_framework_actor::lane_defaults::budget_for")).toBe(false);expect(source.includes("let input=*supplied.original_input();let budget=supplied.budget;")).toBe(true);console.log("[DEBUG] Independent Ajv/SQLite/JSON Host original71/3/19 missing grant preserves original owner without minted policy and forwards all5 supplied axes; actual native supplier is separate")});
+test("original Host Shard grants retain missing authority and use only the original issued input",()=>{const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,missing:{...fixture.missing,policyMinted:true}})).toBe(false);const db=new Database(":memory:");try{expect(db.query("SELECT ? AS operation,? AS generation,? AS epoch,? AS items,? AS copy,? AS capacity,? AS release,? AS depth").get(71,3,19,1,0,128,97,8)).toEqual({operation:71,generation:3,epoch:19,items:1,copy:0,capacity:128,release:97,depth:8})}finally{db.close()}expect(JSON.parse(JSON.stringify(fixture)).input).toEqual(fixture.input);const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");const start=source.indexOf("fn granted_budget("),end=source.indexOf("\n    }",start)+6;expect(source.slice(start,end).includes("Option<&semio_framework_actor::Budget>")).toBe(true);const lifecycle=readFileSync(new URL("../../🔁️lifecycle/🦀️.rs",import.meta.url),"utf8");expect(lifecycle.includes("budget: Option<semio_framework_actor::Budget>")).toBe(true);expect(source.includes("drive_unissued_authority")).toBe(true);expect(source.includes("type Target=semio_framework_actor::IssuedRetainedBatch")).toBe(true);const actor=readFileSync(new URL("../../../../../../../../🔨️modules/🎭️actor/🎟️retained-turn/📦️batch/🦀️.rs",import.meta.url),"utf8");expect(actor.includes("pub fn original_input(&self) -> &RetainedTurnInput")).toBe(true);expect(source.includes("budget.pack_encode(out).await?")).toBe(true);expect(source.includes("let budget=semio_framework_actor::Budget::pack_decode(bytes,pos).await?")).toBe(true);expect(source.includes("or_insert_with(|| semio_framework_actor::lane_defaults::budget_for")).toBe(false);expect(source.includes("let input=*supplied.original_input();let budget=supplied.budget;")).toBe(true);console.log("[DEBUG] Independent Ajv/SQLite/JSON Host original71/3/19 missing grant preserves original owner without minted policy and forwards all5 supplied axes; actual native supplier is separate")});
 
-test("one original issued shard batch returns cumulative conservation once",async()=>{const law=JSON.parse(readFileSync(retainedTurnBatchFixtureUrl,"utf8")),contract=JSON.parse(readFileSync(new URL("../🧬️schema/📦️batch.json",import.meta.url),"utf8"));expect(new Ajv({strict:true}).compile(contract)(law)).toBe(true);const db=new Database(":memory:");try{let grant=law.input.grant;for(const spent of law.spent){grant=db.query("SELECT ?-? AS maximumItems,?-? AS maximumCopyBytes,?-? AS maximumCapacityBytes,?-? AS maximumReleaseBytes,? AS maximumDepth").get(grant.maximumItems,spent.copiedItems,grant.maximumCopyBytes,spent.copiedBytes,grant.maximumCapacityBytes,spent.retainedCapacityBytes,grant.maximumReleaseBytes,spent.releasedBytes,grant.maximumDepth)}expect(grant).toEqual(law.remaining);expect(law.completionCount).toBe(1)}finally{db.close()}console.log("[DEBUG] Independent Ajv/SQLite whole issued batch conserves original remaining and one final return, native receiving separate")});
+test("one original issued shard batch returns cumulative conservation once",async()=>{const law=JSON.parse(readFileSync(retainedTurnBatchFixtureUrl,"utf8")),contract=JSON.parse(readFileSync(new URL("../🧬️schema/📦️batch.json",import.meta.url),"utf8"));expect(new Ajv({strict:true}).compile(contract)(law)).toBe(true);const db=new Database(":memory:");try{let grant=law.input.grant,fuel=law.input.work.fuel;for(const[index,spent]of law.spent.entries()){grant=db.query("SELECT ?-? AS maximumItems,?-? AS maximumCopyBytes,?-? AS maximumCapacityBytes,?-? AS maximumReleaseBytes,? AS maximumDepth").get(grant.maximumItems,spent.copiedItems,grant.maximumCopyBytes,spent.copiedBytes,grant.maximumCapacityBytes,spent.retainedCapacityBytes,grant.maximumReleaseBytes,spent.releasedBytes,grant.maximumDepth);fuel=(db.query("SELECT ?-? AS fuel").get(fuel,law.workSpent[index])as{fuel:number}).fuel;expect(grant).toEqual(law.activeRemaining[index]);expect(fuel).toBe(law.activeRemainingFuel[index]);}expect(grant).toEqual(law.remaining);expect(fuel).toBe(law.remainingFuel);expect(law.completionCount).toBe(1)}finally{db.close()}console.log("[DEBUG] Independent Ajv/SQLite whole issued batch conserves original remaining and one final return, native receiving separate")});
 
-test("checked Host dispatch retains the admitted original decision and exact typed refusal",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{db.run("CREATE TABLE dispatch(operation INTEGER,epoch INTEGER,cursor INTEGER,writes INTEGER)");db.query("INSERT INTO dispatch VALUES(?,?,?,?)").run(0,fixture.input.epoch,fixture.dispatch.cursor,fixture.dispatch.writes);expect(db.query("SELECT epoch,cursor,writes FROM dispatch WHERE operation=0").get()).toEqual({epoch:19,cursor:0,writes:0});expect(db.query("SELECT COUNT(*) AS count FROM dispatch WHERE operation<>0").get()).toEqual({count:0})}finally{db.close()}const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8"),frame=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");expect(source.includes("decision: Option<Decision>")).toBe(true);expect(source.includes("refusal: Option<PackError>")).toBe(true);expect(source.indexOf("self.decision = decision.take()")<source.indexOf("grant.original_input().validate()")).toBe(true);expect(source.includes("self.refusal = Some(error)")).toBe(true);expect(source.includes("self.refusal.is_some() || self.next().is_some()")).toBe(true);expect(frame.includes("pub async fn pack_encode_grant(grant: &semio_framework_actor::TurnGrant")).toBe(true);expect(frame.includes("write_vec(out, &grant.envelopes")).toBe(true);expect(frame.includes("grant.original_input().pack_encode(out).await?")).toBe(false);expect(frame.includes("resources.apply(&mut budget)")).toBe(true);const root=new URL("../../../../../../",import.meta.url);for(const path of["🖥️host/🎠️activation/🦀️.rs","🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎠️runtime/🦀️.rs"]){const runtime=readFileSync(new URL(path,root),"utf8");expect(runtime.includes("if self.dispatch.decision().is_none()")).toBe(true);expect(runtime.includes("self.dispatch.admit(&mut decision)?")).toBe(true);expect(runtime.includes("return Err(self.dispatch.refuse(error))")).toBe(true);expect(runtime.includes("envelopes: grant.envelopes.clone()")).toBe(false);expect(runtime.includes("pack_encode(&mut bytes).await?")).toBe(true)}const failure=readFileSync(new URL("🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs",root),"utf8");expect(failure.includes("pub frame: Option<semio_framework_actor::pack::PackError>")).toBe(true);expect(failure.includes("frame: Some(error)")).toBe(true);expect(failure.includes("text: String::new()")).toBe(true);expect(failure.includes("impl From<ProgramFault> for String")).toBe(false);const shell=readFileSync(new URL("🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs",root),"utf8");expect(shell.includes("refused_frame_fault: Option<semio_framework_actor::pack::PackError>")).toBe(true);expect(shell.includes("pub frame: Option<semio_framework_actor::pack::PackError>")).toBe(true);expect(shell.includes("self.chrome_build.refused_frame_fault = refusal.frame")).toBe(true);expect(shell.includes("Rejected(crate::program_bridge::ProgramFault)")).toBe(true);expect(shell.includes("ShellDirectoryHomePublicationOutcome::Rejected(error.text)")).toBe(false);expect(fixture.dispatch.shellTerminalOnFrameRefusal).toBe(true);const actor=readFileSync(new URL("../../🔨️modules/🎭️actor/🦀️.rs",root),"utf8");expect(actor.includes("pub fn original_input(&self) -> &RetainedTurnInput")).toBe(true);console.log("[DEBUG] Independent Ajv/SQLite checked Host frame refusal preserves original epoch19/cursor0/writes0; actual native pending Decision/PackError custody and borrowed grant path declared, runtime evidence separate")});
+test("checked Host dispatch retains the admitted original decision and exact typed refusal",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{db.run("CREATE TABLE dispatch(operation INTEGER,epoch INTEGER,cursor INTEGER,writes INTEGER)");db.query("INSERT INTO dispatch VALUES(?,?,?,?)").run(0,fixture.input.epoch,fixture.dispatch.cursor,fixture.dispatch.writes);expect(db.query("SELECT epoch,cursor,writes FROM dispatch WHERE operation=0").get()).toEqual({epoch:19,cursor:0,writes:0});expect(db.query("SELECT COUNT(*) AS count FROM dispatch WHERE operation<>0").get()).toEqual({count:0})}finally{db.close()}const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8"),frame=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");expect(source.includes("decision: Option<Decision>")).toBe(true);expect(source.includes("refusal: Option<PackError>")).toBe(true);expect(source.indexOf("self.decision = decision.take()")<source.indexOf("grant.original_input().validate()")).toBe(true);expect(source.includes("self.refusal = Some(error)")).toBe(true);expect(source.includes("self.refusal.is_some() || self.next().is_some()")).toBe(true);expect(frame.includes("pub async fn pack_encode_grant(grant: &semio_framework_actor::TurnGrant")).toBe(true);expect(frame.includes("write_vec(out, &grant.envelopes")).toBe(true);expect(frame.includes("grant.original_input().pack_encode(out).await?")).toBe(false);expect(frame.includes("resources.apply(&mut budget)")).toBe(true);const root=new URL("../../../../../../",import.meta.url);for(const path of["🖥️host/🎠️activation/🦀️.rs","🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎠️runtime/🦀️.rs"]){const runtime=readFileSync(new URL(path,root),"utf8");expect(runtime.includes("if self.dispatch.decision().is_none()")).toBe(true);expect(runtime.includes("self.dispatch.admit(&mut decision)?")).toBe(true);expect(runtime.includes("return Err(self.dispatch.refuse(error))")).toBe(true);expect(runtime.includes("envelopes: grant.envelopes.clone()")).toBe(false);expect(runtime.includes("pack_encode(&mut bytes).await?")).toBe(true)}const failure=readFileSync(new URL("🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs",root),"utf8");expect(failure.includes("pub frame: Option<semio_framework_actor::pack::PackError>")).toBe(true);expect(failure.includes("frame: Some(error)")).toBe(true);expect(failure.includes("text: String::new()")).toBe(true);expect(failure.includes("impl From<ProgramFault> for String")).toBe(false);const shellRoot=new URL("🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/",root),shellMount=readFileSync(new URL("🦀️.rs",shellRoot),"utf8");const shell=["🔖️identity-and-types","🏛️space-admin-and-settle","📨️command-dispatch","🧵️shell-async-runners"].map(owner=>{expect(shellMount).toContain(`${owner}/🦀️.rs`);return readFileSync(new URL(`${owner}/🦀️.rs`,shellRoot),"utf8")}).join("\n");expect(shell.includes("refused_frame_fault: Option<semio_framework_actor::pack::PackError>")).toBe(true);expect(shell.includes("pub frame: Option<semio_framework_actor::pack::PackError>")).toBe(true);expect(shell.includes("self.chrome_build.refused_frame_fault = refusal.frame")).toBe(true);expect(shell.includes("Rejected(crate::program_bridge::ProgramFault)")).toBe(true);expect(shell.includes("ShellDirectoryHomePublicationOutcome::Rejected(error.text)")).toBe(false);expect(fixture.dispatch.shellTerminalOnFrameRefusal).toBe(true);const actor=readFileSync(new URL("../../🔨️modules/🎭️actor/🦀️.rs",root),"utf8");expect(actor.includes("pub fn original_input(&self) -> &RetainedTurnInput")).toBe(true);console.log("[DEBUG] Independent Ajv/SQLite checked Host frame refusal preserves original epoch19/cursor0/writes0; actual native pending Decision/PackError custody and borrowed grant path declared, runtime evidence separate")});
+
+
+test("raw shard frames reject a foreign bounded loan before wire writes and envelope allocation",()=>{
+ expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{const input=fixture.input,base=fixture.loan;for(const row of fixture.loanCases){const policy={...base.retainedPolicy},work={workFuel:base.workFuel,deadlineUs:base.deadlineUs};if(row.axis in policy)policy[row.axis as keyof typeof policy]=row.value;else if(row.axis==="workFuel"||row.axis==="deadlineUs")work[row.axis]=row.value;const result=db.query("SELECT (?1<=?2 AND ?3<=?4 AND ?5<=?6 AND ?7<=?8 AND ?9<=?10 AND ?11<=?12 AND ?13>0 AND ?13<=?14) AS admitted").get(policy.maximumItems,input.grant.maximumItems,policy.maximumCopyBytes,input.grant.maximumCopyBytes,policy.maximumCapacityBytes,input.grant.maximumCapacityBytes,policy.maximumReleaseBytes,input.grant.maximumReleaseBytes,policy.maximumDepth,input.grant.maximumDepth,work.workFuel,input.work.fuel,work.deadlineUs,input.work.absoluteDeadlineUs)as{admitted:number};expect(Boolean(result.admitted),row.axis).toBe(row.admitted);}}finally{db.close();}
+ const frame=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8"),encode=frame.slice(frame.indexOf("pub async fn pack_encode(&self"),frame.indexOf("pub async fn pack_decode(bytes")),decode=frame.slice(frame.indexOf("pub async fn pack_decode(bytes"),frame.indexOf("//#endregion 📨️ShardFrame"));expect(encode).toContain("loan.validate_for(*retained)");expect(encode.indexOf("loan.validate_for(*retained)")).toBeLessThan(encode.indexOf("write_u8(out"));expect(decode).toContain("loan.validate_for(retained)");expect(decode.indexOf("loan.validate_for(retained)")).toBeLessThan(decode.indexOf('read_vec(bytes, pos, "ShardFrame::Grant::envelopes"'));
+ console.log("[DEBUG] Independent SQLite nine original-loan cases reject each overdrawn physical/work/deadline view before raw Grant publication and envelope allocation; native law unrun");
+});


### 19 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs
B/H/S/current: 2667/99/2670/99
--- 
+++ 
@@ -114,5 +114,9 @@
 
     pub async fn pack_encode(&self, out: &mut Vec<u8>) -> Result<(), semio_framework_actor::pack::PackError> {
-        if let ShardFrame::Grant { retained, .. } = self { retained.validate().map_err(|_| semio_framework_actor::pack::PackError::InvalidRetainedTurn("operation identity is absent"))?; }
+        if let ShardFrame::Grant { retained,retained_policy,work_fuel,deadline_us,.. } = self {
+            retained.validate().map_err(|_|semio_framework_actor::pack::PackError::InvalidRetainedTurn("operation identity is absent"))?;
+            let loan=semio_framework_actor::OriginalActorTurnLoan{retained_policy:*retained_policy,work_fuel:*work_fuel,deadline_us:*deadline_us};
+            loan.validate_for(*retained).map_err(|_|semio_framework_actor::pack::PackError::InvalidRetainedTurn("original grant semantic loan exceeds its source"))?;
+        }
         semio_framework_actor::pack::write_u8(out, self.tag().await).await;
         match self {
@@ -136,13 +140,12 @@
             0 => Ok(ShardFrame::Register { actor: ActorId::pack_decode(bytes, pos).await? }),
             1 => Ok(ShardFrame::Unregister { actor: ActorId::pack_decode(bytes, pos).await? }),
-            2 => Ok(ShardFrame::Grant {
-                actor: ActorId::pack_decode(bytes, pos).await?,
-                budget: semio_framework_actor::Budget::pack_decode(bytes, pos).await?,
-                retained: semio_framework_actor::RetainedTurnInput::pack_decode(bytes, pos).await?,
-                retained_policy:semio_framework_actor::retained_turn::pack_decode_grant(bytes,pos).await?,
-                work_fuel:semio_framework_actor::pack::read_u64(bytes,pos,"Grant::work-fuel").await?,
-                deadline_us:semio_framework_actor::pack::read_u64(bytes,pos,"Grant::deadline-us").await?,
-                envelopes: semio_framework_actor::pack::read_vec(bytes, pos, "ShardFrame::Grant::envelopes", Envelope::pack_decode).await?,
-            }),
+            2 => {
+                let actor=ActorId::pack_decode(bytes,pos).await?;
+                let budget=semio_framework_actor::Budget::pack_decode(bytes,pos).await?;
+                let retained=semio_framework_actor::RetainedTurnInput::pack_decode(bytes,pos).await?;
+                let loan=semio_framework_actor::OriginalActorTurnLoan::pack_decode(bytes,pos).await?;
+                loan.validate_for(retained).map_err(|_|semio_framework_actor::pack::PackError::InvalidRetainedTurn("original grant semantic loan exceeds its source"))?;
+                Ok(ShardFrame::Grant{actor,budget,retained,retained_policy:loan.retained_policy,work_fuel:loan.work_fuel,deadline_us:loan.deadline_us,envelopes:semio_framework_actor::pack::read_vec(bytes, pos, "ShardFrame::Grant::envelopes", Envelope::pack_decode).await?})
+            },
             3 => Ok(ShardFrame::Envelope(Envelope::pack_decode(bytes, pos).await?)),
             4=>{let actor=ActorId::pack_decode(bytes,pos).await?;let raw=semio_framework_actor::OriginalActorGuestReturn::pack_decode(bytes,pos).await?;let receiving_sequence=semio_framework_actor::pack::read_u64(bytes,pos,"MeshReceived::sequence").await?;let accepted=semio_framework_actor::pack::read_bool(bytes,pos,"MeshReceived::accepted").await?;Ok(Self::MeshReceived{actor,input:raw.input,receiving_sequence,spent:raw.spent,work_units_spent:raw.work_units_spent,accepted})},


### 20 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧪️tests/🔬️unit/🦀️.rs
B/H/S/current: 1660/28/1660/28
--- 
+++ 
@@ -1610,5 +1610,5 @@
     retained.operation = 0;
     let budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive);
-    let decision = semio_framework_actor::Decision { run: vec![semio_framework_actor::TurnGrant { actor: ActorId(71), shard: semio_framework_actor::ShardId(0), budget, retained, envelopes: Vec::new() }], wake_at: Some(19) };
+    let decision = semio_framework_actor::Decision { run: vec![semio_framework_actor::TurnGrant { actor: ActorId(71), shard: semio_framework_actor::ShardId(0), budget, retained, loan: serde_json::from_value(law["loan"].clone()).unwrap(), envelopes: Vec::new() }], wake_at: Some(19) };
     let pointer = decision.run.as_ptr();
     let mut input = Some(decision);


### 21 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🗃️document/🦀️.rs
B/H/S/current: 1899/89/1899/89
--- 
+++ 
@@ -1639,5 +1639,5 @@
 pub struct ActiveDocumentArchiveLoad<P, Mutation>
 where
-    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
+    P: semio_framework_value::ToValue,
     Mutation: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::Mutation<P>,
 {
@@ -1820,5 +1820,5 @@
 /// 🧭️ Selects the original archive operation eligible for one close turn.
 fn original_document_archive_close_slot<P,Mutation>(document_archive_loads:&ArtifactFixedRegistry<ActiveDocumentArchiveLoad<P,Mutation>>,closing:bool)->Option<u64>
-where P:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue,Mutation:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+store::Mutation<P>{
+where P: semio_framework_value::ToValue,Mutation:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+store::Mutation<P>{
     (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|index|document_archive_loads.entry(index).filter(|(_,active)|closing||active.terminal_target.is_some()&&!matches!(active.state,ActiveDocumentArchiveLoadState::Ready|ActiveDocumentArchiveLoadState::Cancelled|ActiveDocumentArchiveLoadState::Fault)).map(|(operation,_)|*operation))
 }
@@ -1863,5 +1863,5 @@
 impl<P, Mutation> Drop for ActiveDocumentArchiveLoad<P, Mutation>
 where
-    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
+    P: semio_framework_value::ToValue,
     Mutation: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::Mutation<P>,
 {


### 22 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🦀️.rs
B/H/S/current: 3172/126/3174/126
--- 
+++ 
@@ -2853,4 +2853,6 @@
 struct CheckpointOutputRetirement{bytes:std::mem::ManuallyDrop<Option<Vec<u8>>>}
 impl semio_framework_value::ErasedSnapshotRetirement for CheckpointOutputRetirement{
+ fn next_demand(&self,maximum_body_bytes:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{let _=maximum_body_bytes;Ok(semio_framework_value::RetirementDemand{copy_bytes:if self.bytes.is_some(){size_of::<Option<Vec<u8>>>()}else{0},release_bytes:self.bytes.as_ref().map_or(0,Vec::capacity),depth:usize::from(self.bytes.is_some()),..Default::default()})}
+
     fn close_step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,semio_framework_value::ValueError>{use semio_framework_value::retained_clone::{RetainedCloneStep,RetainedCloneProgress};let empty=RetainedCloneProgress::default();if self.bytes.is_none(){return Ok(RetainedCloneStep::Complete(empty));}let release=self.bytes.as_ref().unwrap().capacity();let copy=size_of::<Option<Vec<u8>>>();if grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_release_bytes<release{return Ok(RetainedCloneStep::Progress(empty));}if grant.maximum_depth==0{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"checkpoint owner release requires original depth"));}drop(self.bytes.take());Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:copy,released_bytes:release,..empty}))}
     fn next_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(if self.bytes.is_some(){size_of::<Option<Vec<u8>>>()}else{0})}


### 23 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/📡️contributed-mutation-wire/🧬️mutations/➕️add-value/🦀️.rs
B/H/S/current: 64/67/77/74
--- 
+++ 
@@ -20,17 +20,7 @@
     const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "value", kind: "add-value", record: "AddedValue" };
 
-    fn diff(&self, _base: &WireTestSnapshot) -> protocol::MutationOutcome<WireTestDiff> {
-        protocol::MutationOutcome::new(WireTestDiff { deltas: vec![self.delta] })
-    }
 
-    fn inverse(&self, _base: &WireTestSnapshot) -> Result<Vec<WireTestMutation>, semio_framework_value::ValueError> {
-        Ok((|| {
-            if self.delta == i32::MIN {
-                vec![WireTestMutation::AddValue(Self { delta: 1 }), WireTestMutation::AddValue(Self { delta: i32::MAX })]
-            } else {
-                vec![WireTestMutation::AddValue(Self { delta: -self.delta })]
-            }
-        })())
-    }
+
+
 
     fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
@@ -43,10 +33,23 @@
 }
 
+impl protocol::ColdMutationKind<WireTestSnapshot, WireTestMutation> for AddValue {
+    fn diff(&self, _base: &WireTestSnapshot) -> protocol::MutationOutcome<WireTestDiff> {
+        protocol::MutationOutcome::new(WireTestDiff { deltas: vec![self.delta] })
+    }
+    fn inverse(&self, _base: &WireTestSnapshot) -> Result<Vec<WireTestMutation>, semio_framework_value::ValueError> {
+        Ok((|| {
+            if self.delta == i32::MIN {
+                vec![WireTestMutation::AddValue(Self { delta: 1 }), WireTestMutation::AddValue(Self { delta: i32::MAX })]
+            } else {
+                vec![WireTestMutation::AddValue(Self { delta: -self.delta })]
+            }
+        })())
+    }
+}
+
 impl protocol::CompositeMutationKind<WireTestSnapshot, WireTestMutation> for AddValue {
     const SEMANTICS: protocol::SemanticDescriptor = <Self as protocol::MutationKind<WireTestSnapshot, WireTestMutation>>::SEMANTICS;
 
-    fn plan(&self, _base: &WireTestSnapshot, planner: &mut protocol::Planner<WireTestSnapshot, WireTestMutation>) -> Result<(), protocol::PlanError> {
-        planner.call(WireTestMutation::AddValue(self.clone()))
-    }
+
 
     fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
@@ -59,6 +62,16 @@
 }
 
+impl protocol::ColdCompositeMutationKind<WireTestSnapshot, WireTestMutation> for AddValue {
+    fn plan(&self, _base: &WireTestSnapshot, planner: &mut protocol::Planner<WireTestSnapshot, WireTestMutation>) -> Result<(), protocol::PlanError> {
+        planner.call(WireTestMutation::AddValue(self.clone()))
+    }
+}
+
 #[cfg(test)]
 #[path = "../../../../🧪️tests/➕️contributed-mutation-wire-add-value-unit/🦀️.rs"]
 mod tests;
 //#endregion ➕️ContributedWireAddValue
+
+#[cfg(test)]
+#[path = "🧪️tests/🧪️sum-law/🦀️.rs"]
+mod sum_law_tests;


### 24 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/📢️publication-fixtures/👥️presence/🧬️mutations/📝️change-publication/🦀️.rs
B/H/S/current: 71/72/80/84
--- 
+++ 
@@ -58,11 +58,7 @@
     const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "publication-presence", kind: "change-publication-presence", record: "ChangedPublicationPresence" };
 
-    fn diff(&self, _base: &PublicationPresence) -> MutationOutcome<super::super::PublicationPresenceDiff> {
-        MutationOutcome::new(super::super::PublicationPresenceDiff { revision: Some(self.revision) })
-    }
 
-    fn inverse(&self, base: &PublicationPresence) -> Result<Vec<PublicationPresenceMutation>, semio_framework_value::ValueError> {
-        Ok((|| vec![Self { revision: base.revision }.into()])())
-    }
+
+
 
     fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
@@ -70,2 +66,15 @@
     }
 }
+
+impl crate::os_spr::ColdMutationKind<PublicationPresence, PublicationPresenceMutation> for ChangePublicationPresence {
+    fn diff(&self, _base: &PublicationPresence) -> MutationOutcome<super::super::PublicationPresenceDiff> {
+        MutationOutcome::new(super::super::PublicationPresenceDiff { revision: Some(self.revision) })
+    }
+    fn inverse(&self, base: &PublicationPresence) -> Result<Vec<PublicationPresenceMutation>, semio_framework_value::ValueError> {
+        Ok((|| vec![Self { revision: base.revision }.into()])())
+    }
+}
+
+#[cfg(test)]
+#[path = "🧪️tests/🧪️sum-law/🦀️.rs"]
+mod sum_law_tests;


### 25 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/📢️publication-fixtures/🫧️transient/🧬️mutations/📝️change-publication/🦀️.rs
B/H/S/current: 71/72/80/84
--- 
+++ 
@@ -58,11 +58,7 @@
     const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "publication-transient", kind: "change-publication-transient", record: "ChangedPublicationTransient" };
 
-    fn diff(&self, _base: &PublicationTransient) -> MutationOutcome<super::super::PublicationTransientDiff> {
-        MutationOutcome::new(super::super::PublicationTransientDiff { revision: Some(self.revision) })
-    }
 
-    fn inverse(&self, base: &PublicationTransient) -> Result<Vec<PublicationTransientMutation>, semio_framework_value::ValueError> {
-        Ok((|| vec![Self { revision: base.revision }.into()])())
-    }
+
+
 
     fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
@@ -70,2 +66,15 @@
     }
 }
+
+impl crate::os_spr::ColdMutationKind<PublicationTransient, PublicationTransientMutation> for ChangePublicationTransient {
+    fn diff(&self, _base: &PublicationTransient) -> MutationOutcome<super::super::PublicationTransientDiff> {
+        MutationOutcome::new(super::super::PublicationTransientDiff { revision: Some(self.revision) })
+    }
+    fn inverse(&self, base: &PublicationTransient) -> Result<Vec<PublicationTransientMutation>, semio_framework_value::ValueError> {
+        Ok((|| vec![Self { revision: base.revision }.into()])())
+    }
+}
+
+#[cfg(test)]
+#[path = "🧪️tests/🧪️sum-law/🦀️.rs"]
+mod sum_law_tests;


### 26 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🖥️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config/🦀️.rs
B/H/S/current: 64/67/73/64
--- 
+++ 
@@ -33,13 +33,6 @@
 impl MutationKind<TestConfig, TestConfigMutation> for ChangeTestConfigSelection {
     const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "test-config-selection", kind: "change-test-config-selection", record: "ChangedTestConfigSelection" };
-    fn diff(&self, _: &TestConfig) -> MutationOutcome<TestConfigDiff> {
-        MutationOutcome::new(match &self.selected {
-            None => TestConfigDiff::Clear,
-            Some(value) => TestConfigDiff::Set(value.clone()),
-        })
-    }
-    fn inverse(&self, base: &TestConfig) -> Result<Vec<TestConfigMutation>, semio_framework_value::ValueError> {
-        Ok((|| vec![Self { selected: base.selected.clone() }.into()])())
-    }
+
+
     fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
         semio_framework_ui_locale::LocalizedLabel::native(
@@ -60,5 +53,21 @@
 }
 
+impl crate::os_spr::ColdMutationKind<TestConfig, TestConfigMutation> for ChangeTestConfigSelection {
+    fn diff(&self, _: &TestConfig) -> MutationOutcome<TestConfigDiff> {
+        MutationOutcome::new(match &self.selected {
+            None => TestConfigDiff::Clear,
+            Some(value) => TestConfigDiff::Set(value.clone()),
+        })
+    }
+    fn inverse(&self, base: &TestConfig) -> Result<Vec<TestConfigMutation>, semio_framework_value::ValueError> {
+        Ok((|| vec![Self { selected: base.selected.clone() }.into()])())
+    }
+}
+
 #[cfg(test)]
 #[path = "../../../../../🧪️tests/🎚️test-app-config-selection-unit/🦀️.rs"]
 mod tests;
+
+#[cfg(test)]
+#[path = "🧪️tests/🧪️sum-law/🦀️.rs"]
+mod sum_law_tests;


### 27 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🖥️test-app-mutations/🧬️document/🧬️mutations/🧒️set-slot-children/🦀️.rs
B/H/S/current: 51/54/60/52
--- 
+++ 
@@ -20,13 +20,7 @@
     const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "slot-children", kind: "set-slot-children", record: "SetSlotChildren" };
 
-    fn diff(&self, _: &TestSnapshot) -> MutationOutcome<TestDiff> {
-        MutationOutcome::new(TestDiff { count: None, label: None, slot: Some(self.children.clone()) })
-    }
 
-    fn inverse(&self, base: &TestSnapshot) -> Result<Vec<TestMutation>, semio_framework_value::ValueError> {
-use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};
 
-        Ok((|| vec![Self { children: base.slot.iter().map(|child| child.target.to_uri()).collect() }.into()])())
-    }
+
 
     fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
@@ -46,6 +40,21 @@
 }
 
+impl crate::os_spr::ColdMutationKind<TestSnapshot, TestMutation> for SetSlotChildren {
+    fn diff(&self, _: &TestSnapshot) -> MutationOutcome<TestDiff> {
+        MutationOutcome::new(TestDiff { count: None, label: None, slot: Some(self.children.clone()) })
+    }
+    fn inverse(&self, base: &TestSnapshot) -> Result<Vec<TestMutation>, semio_framework_value::ValueError> {
+use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};
+
+        Ok((|| vec![Self { children: base.slot.iter().map(|child| child.target.to_uri()).collect() }.into()])())
+    }
+}
+
 #[cfg(test)]
 #[path = "../../../../../🧪️tests/🧒️test-app-document-set-slot-children-unit/🦀️.rs"]
 mod tests;
+
+#[cfg(test)]
+#[path = "🧪️tests/🧪️sum-law/🦀️.rs"]
+mod sum_law_tests;
 //#endregion 🧒️SetSlotChildren


### 28 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🛰️declaration-channels/1standard/🌐️any/🧬️mutations/📝️set-value/🦀️.rs
B/H/S/current: 35/37/44/37
--- 
+++ 
@@ -16,10 +16,6 @@
 impl protocol::MutationKind<Std1AnySnapshot, Std1AnyMutation> for SetValue {
     const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "value", kind: "set-value", record: "SetValue" };
-    fn diff(&self, _base: &Std1AnySnapshot) -> protocol::MutationOutcome<Std1AnyDiff> {
-        protocol::MutationOutcome::new(Std1AnyDiff { value: Some(self.value) })
-    }
-    fn inverse(&self, base: &Std1AnySnapshot) -> Result<Vec<Std1AnyMutation>, semio_framework_value::ValueError> {
-        Ok((|| vec![Std1AnyMutation::SetValue(Self { value: base.value })])())
-    }
+
+
     fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
         semio_framework_ui_locale::LocalizedLabel::native(&format!("Set value to {}", self.value), &format!("Wert auf {} setzen", self.value))
@@ -30,6 +26,19 @@
 }
 
+impl protocol::ColdMutationKind<Std1AnySnapshot, Std1AnyMutation> for SetValue {
+    fn diff(&self, _base: &Std1AnySnapshot) -> protocol::MutationOutcome<Std1AnyDiff> {
+        protocol::MutationOutcome::new(Std1AnyDiff { value: Some(self.value) })
+    }
+    fn inverse(&self, base: &Std1AnySnapshot) -> Result<Vec<Std1AnyMutation>, semio_framework_value::ValueError> {
+        Ok((|| vec![Std1AnyMutation::SetValue(Self { value: base.value })])())
+    }
+}
+
 #[cfg(test)]
 #[path = "../../../../../../🧪️tests/🌐️standard-one-any-set-value-unit/🦀️.rs"]
 mod tests;
 //#endregion 📝️Std1AnySetValue
+
+#[cfg(test)]
+#[path = "🧪️tests/🧪️sum-law/🦀️.rs"]
+mod sum_law_tests;


### 29 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🛰️declaration-channels/1standard/🔒️strict/🧬️mutations/📝️set-value/🦀️.rs
B/H/S/current: 35/37/44/37
--- 
+++ 
@@ -16,10 +16,6 @@
 impl protocol::MutationKind<Std1StrictSnapshot, Std1StrictMutation> for SetValue {
     const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "value", kind: "set-value", record: "SetValue" };
-    fn diff(&self, _base: &Std1StrictSnapshot) -> protocol::MutationOutcome<Std1StrictDiff> {
-        protocol::MutationOutcome::new(Std1StrictDiff { value: Some(self.value) })
-    }
-    fn inverse(&self, base: &Std1StrictSnapshot) -> Result<Vec<Std1StrictMutation>, semio_framework_value::ValueError> {
-        Ok((|| vec![Std1StrictMutation::SetValue(Self { value: base.value })])())
-    }
+
+
     fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
         semio_framework_ui_locale::LocalizedLabel::native(&format!("Set value to {}", self.value), &format!("Wert auf {} setzen", self.value))
@@ -30,6 +26,19 @@
 }
 
+impl protocol::ColdMutationKind<Std1StrictSnapshot, Std1StrictMutation> for SetValue {
+    fn diff(&self, _base: &Std1StrictSnapshot) -> protocol::MutationOutcome<Std1StrictDiff> {
+        protocol::MutationOutcome::new(Std1StrictDiff { value: Some(self.value) })
+    }
+    fn inverse(&self, base: &Std1StrictSnapshot) -> Result<Vec<Std1StrictMutation>, semio_framework_value::ValueError> {
+        Ok((|| vec![Std1StrictMutation::SetValue(Self { value: base.value })])())
+    }
+}
+
 #[cfg(test)]
 #[path = "../../../../../../🧪️tests/🔒️standard-one-strict-set-value-unit/🦀️.rs"]
 mod tests;
 //#endregion 📝️Std1StrictSetValue
+
+#[cfg(test)]
+#[path = "🧪️tests/🧪️sum-law/🦀️.rs"]
+mod sum_law_tests;


### 30 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🛰️declaration-channels/2standard/🌐️any/🧬️mutations/📝️set-value/🦀️.rs
B/H/S/current: 35/37/44/37
--- 
+++ 
@@ -16,10 +16,6 @@
 impl protocol::MutationKind<Std2AnySnapshot, Std2AnyMutation> for SetValue {
     const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "value", kind: "set-value", record: "SetValue" };
-    fn diff(&self, _base: &Std2AnySnapshot) -> protocol::MutationOutcome<Std2AnyDiff> {
-        protocol::MutationOutcome::new(Std2AnyDiff { value: Some(self.value) })
-    }
-    fn inverse(&self, base: &Std2AnySnapshot) -> Result<Vec<Std2AnyMutation>, semio_framework_value::ValueError> {
-        Ok((|| vec![Std2AnyMutation::SetValue(Self { value: base.value })])())
-    }
+
+
     fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
         semio_framework_ui_locale::LocalizedLabel::native(&format!("Set value to {}", self.value), &format!("Wert auf {} setzen", self.value))
@@ -30,6 +26,19 @@
 }
 
+impl protocol::ColdMutationKind<Std2AnySnapshot, Std2AnyMutation> for SetValue {
+    fn diff(&self, _base: &Std2AnySnapshot) -> protocol::MutationOutcome<Std2AnyDiff> {
+        protocol::MutationOutcome::new(Std2AnyDiff { value: Some(self.value) })
+    }
+    fn inverse(&self, base: &Std2AnySnapshot) -> Result<Vec<Std2AnyMutation>, semio_framework_value::ValueError> {
+        Ok((|| vec![Std2AnyMutation::SetValue(Self { value: base.value })])())
+    }
+}
+
 #[cfg(test)]
 #[path = "../../../../../../🧪️tests/🌐️standard-two-any-set-value-unit/🦀️.rs"]
 mod tests;
 //#endregion 📝️Std2AnySetValue
+
+#[cfg(test)]
+#[path = "🧪️tests/🧪️sum-law/🦀️.rs"]
+mod sum_law_tests;


### 31 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🦀️.rs
B/H/S/current: 2746/174/2859/201
--- 
+++ 
@@ -1,3 +1,7 @@
 //! ⚖️ App laws and the test modules of the app vocabulary.
+#[path="🎟️caller/👷️work/🦀️.rs"]
+mod original_work_fixture;
+pub use original_work_fixture::OriginalWorkFixture;
+
 #[path="🎟️caller/🦀️.rs"]
 mod original_fixture_caller;
@@ -263,6 +267,16 @@
     //! `terminology_tests`/`panel_kit_tests` above for the sibling pattern of testing SDK primitives
     //! themselves inline.
-
-    pub use super::OriginalFixtureCaller;
+    use semio_framework_value::ErasedSnapshotRetirement;
+    /// 🧪️ Observes each production page presentation with the exact original fixture grant.
+    pub fn take_fixture_result_page<P:crate::plugin_app::PluginApp>(app:&mut P,receiver:u32,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<Option<crate::tool_contract::TypedOperationResultPage>,semio_framework::Fault>{
+        for _ in 0..2{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||app.take_typed_operation_result_page(receiver,grant));let(page,p)=result?;assert!(p.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));if page.is_some()||p==Default::default(){return Ok(page);}}
+        Err(semio_framework::Fault::from("original fixture page sealing omitted its next projection"))
+    }
+    /// ♻️ Returns a projected page's token only after its exact physical alias closure.
+    pub fn close_fixture_result_page(page:&mut crate::tool_contract::TypedOperationResultPage,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->crate::tool_contract::TypedOperationResultToken{
+        let token=page.token;for _ in 0..100000{if page.terminal_is_empty(){return token;}let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||page.close_step(grant).expect("original fixture receiver page closure"));assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert_ne!(step.progress(),Default::default(),"original fixture close grant must fund its unchanged page authority");}panic!("original fixture receiver page did not close");
+    }
+
+    pub use super::{OriginalFixtureCaller,OriginalWorkFixture};
     use super::{register_framework_reserved_factories, ActionMeta, App, AppActionRegistry, ArtifactApp, ArtifactToolFactoryRegistry, PluginApp, TypedOperationResultLane, VcsArtifactApp};
     use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
@@ -280,4 +294,27 @@
         close_fixture_action_inputs(original_args,None,cx,close_loan);
         result?.ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original action fixture dispatch exceeded its authored turn bound").into())
+    }
+
+    /// 📨️ Projects authored external unit and data variants before their fixture action enters the original source.
+    fn original_fixture_command_arguments(command:&impl semio_framework_value::ToValue)->Result<Option<super::DslValue>,semio_framework_value::ValueError>{
+        match semio_framework_value::ToValue::to_value(command){super::DslValue::String(_)=>Ok(None),super::DslValue::Object(mut rows)if rows.len()==1=>Ok(Some(rows.pop().unwrap().1)),_=>Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"typed fixture command requires its authored external variant"))}
+    }
+
+    #[cfg(test)]
+    #[test]
+    fn original_fixture_command_projection_matches_the_closed_unit_and_data_cases(){
+        #[derive(semio_framework_value_derive::ToValue)]
+        enum Command{Noop,Set{value:u64}}
+        let law:serde_json::Value=serde_json::from_str(include_str!("🎟️caller/🧪️tests/🧫️fixtures/📨️command.json")).unwrap();
+        for row in law["cases"].as_array().unwrap(){let command=match row["variant"].as_str().unwrap(){"Noop"=>Command::Noop,"Set"=>Command::Set{value:row["value"].as_u64().unwrap()},_=>panic!("closed fixture command variant")};let actual=original_fixture_command_arguments(&command).unwrap();if row["variant"]=="Noop"{assert!(actual.is_none());assert!(row["expectedArgs"].is_null());}else{assert_eq!(actual,Some(super::DslValue::object([("value".to_owned(),semio_framework_value::ToValue::to_value(&row["expectedArgs"]["value"].as_u64().unwrap()))])));}}
+        println!("[DEBUG] actual derived external unit/data fixture command projection preserves authored arguments");
+    }
+
+    /// 🎟️ Projects a cold typed fixture input into the actual retained action ingress under its persistent original caller.
+    pub async fn dispatch_original_fixture_command<A,M>(app:&mut VcsArtifactApp<A,M>,command:&A::Command,meta:&ActionMeta,mounted_policy:crate::MountedOwnerPolicyV1,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<super::InvocationResult,semio_framework_job::OriginalTurnFailure>
+    where A:ArtifactApp,A::Command:semio_framework_value::ToValue,M:super::SpaceMember+super::MemberFactory+Send+'static{
+        let action=A::command_id(command).await;
+        let args=original_fixture_command_arguments(command)?;
+        dispatch_fixture_action(app,action,args,meta,identity,cx,mounted_policy.maintenance).await
     }
 
@@ -333,4 +370,74 @@
     pub fn fixture_retained_turn() -> semio_framework::kernel::RetainedTurnInput {
         semio_framework::kernel::RetainedTurnInput { operation: 1, generation: 1, epoch: 1, grant: fixture_mounted_policy().maintenance }
+    }
+
+    /// 🧳️ A fixture job retains one original consuming decoder and closes it under its caller's supplied grant.
+    pub struct FixtureOriginalJob {
+        decoder: semio_framework_job::OriginalJobDecodeOwner,
+        cancel: Option<semio_framework_job::CancelToken>,
+        payload: semio_framework_job::JobPayloadAuthority,
+        turn: semio_framework::kernel::RetainedTurnInput,
+        sequence: u64,
+        progress: semio_framework_value::RetainedCloneProgress,
+    }
+
+    impl FixtureOriginalJob {
+        /// 🎟️ Captures an explicit original input without allocating decoder capacity.
+        pub fn new(turn: semio_framework::kernel::RetainedTurnInput) -> Self {
+            let ((cancel,progress),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_job::CancelToken::admit_root(turn.grant).expect("fixture original cancellation admission").expect("fixture original input funds cancellation"));
+            assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
+            assert!(progress.fits(turn.grant));
+            let ((payload,progress),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_job::JobPayloadAuthority::admit(semio_framework_job::OperationId(turn.operation),semio_framework_job::Generation(turn.generation),turn.grant).expect("fixture original payload admission").expect("fixture original input funds payload"));
+            assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
+            assert!(progress.fits(turn.grant));
+            Self { decoder: semio_framework_job::OriginalJobDecodeOwner::empty(),cancel:Some(cancel),payload,turn, sequence: 0, progress: Default::default() }
+        }
+
+        /// 🫴️ Reborrows the same issuer for one context carrying the caller's exact turn grant.
+        pub fn context(&mut self, grant: RetainedCloneGrant) -> semio_framework_job::StepContext<'_> {
+            self.progress = Default::default();
+            semio_framework_job::StepContext::with_payload_authority(semio_framework_job::OperationId(self.turn.operation), semio_framework_job::Generation(self.turn.generation), semio_framework_job::StepBudget::new(1_000_000, u64::MAX, grant), self.cancel.as_ref().expect("fixture original cancellation retained"), || Some(0), &mut self.sequence, &mut self.progress,&self.payload).expect("fixture original payload loan").with_original_decoder(&mut self.decoder)
+        }
+
+        /// 🛬️ Loans original receiving to a synchronous semantic unit through its real context recipient.
+        pub fn receive<T>(&mut self, grant: RetainedCloneGrant, operation: impl FnOnce(&mut semio_framework_value::NativeDecodeControl<'_>) -> T) -> T {
+            self.context(grant).with_native_receiving(|receiving| Ok::<_, semio_framework_value::ValueError>(operation(receiving))).expect("fixture original receiving loan")
+        }
+
+        /// ♻️ Pays every original receiving retirement action with the supplied fixture input.
+        pub fn close(&mut self) {
+            for _ in 0..100_000 {
+                if self.decoder.terminal_is_empty() { break; }
+                let (step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||self.decoder.close_step(self.turn.grant).expect("fixture original decoder paid closure"));
+                let progress = match step { semio_framework_value::RetainedCloneStep::Progress(progress) | semio_framework_value::RetainedCloneStep::Complete(progress) => progress };
+                assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
+                assert!(progress.fits(self.turn.grant), "fixture receiving closure exceeded its original input");
+                assert_ne!(progress, semio_framework_value::RetainedCloneProgress::default(), "fixture receiving closure stalled under its original input");
+            }
+            assert!(self.decoder.terminal_is_empty(),"fixture original receiving retirement did not finish");
+            for _ in 0..100_000 {
+                if self.payload.terminal_is_empty() { break; }
+                let (step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||self.payload.close_step(self.turn.grant).expect("fixture original payload paid closure"));let progress=step.progress();
+                assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
+                assert!(progress.fits(self.turn.grant));
+                assert_ne!(progress,semio_framework_value::RetainedCloneProgress::default(),"fixture original payload closure stalled");
+            }
+            assert!(self.payload.terminal_is_empty());
+            if let Some(cancel)=self.cancel.take() {
+                let mut owner=semio_framework_async::CancelTokenRetirement::from_token(cancel);
+                for _ in 0..100_000 {
+                    if owner.terminal_is_empty() { break; }
+                    let (step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(self.turn.grant).expect("fixture original cancellation paid closure"));let progress=step.progress();
+                    assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
+                    assert!(progress.fits(self.turn.grant));
+                    assert_ne!(progress,semio_framework_value::RetainedCloneProgress::default(),"fixture original cancellation closure stalled");
+                }
+                assert!(owner.terminal_is_empty());
+            }
+        }
+    }
+
+    impl Drop for FixtureOriginalJob {
+        fn drop(&mut self) { if !std::thread::panicking() { self.close(); } }
     }
 
@@ -1017,5 +1124,5 @@
             app.advance_typed_operation_publication(identity,cx).await?;
             retain_registered_ownership_receipts(app, mounted_policy, &mut receipt)?;
-            while let Some(page) = app.take_typed_operation_result_page(receiver) {
+            while let Some(mut page) = crate::app_testing::artifact_app_laws::take_fixture_result_page(app,receiver,mounted_policy.preparation).expect("original fixture page presentation") {
                 let lane = page.lane;
                 let fault = (lane == TypedOperationResultLane::Fault).then(|| crate::tool_contract::decode_typed_operation_fault_page(page.bytes()));
@@ -1267,4 +1374,5 @@
     pub async fn assert_undo_redo_round_trip<A, M, P>(app: &mut VcsArtifactApp<A, M>, command: A::Command, probe: impl Fn(&VcsArtifactApp<A, M>) -> P, before: P, after: P, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)
     where
+        A::Command:semio_framework_value::ToValue,
         A: ArtifactApp,
         M: super::SpaceMember + super::MemberFactory + Send + 'static,
@@ -1272,5 +1380,5 @@
     {
         let receiver = meta("local").instance_id;
-        app.dispatch_typed(command, &meta("local"), identity,cx).await.expect("apply command");
+        dispatch_original_fixture_command(app,&command,&meta("local"),mounted_policy,identity,cx).await.expect("apply command");
         settle_registered_typed_operation(app, receiver, mounted_policy, identity,cx).await.expect("apply command publication");
         assert_eq!(probe(app), after, "command did not produce the expected snapshot");
@@ -1632,4 +1740,5 @@
             TypedOperationResultLane::Terminal => "terminal",
             TypedOperationResultLane::Fault => "fault",
+            TypedOperationResultLane::Diagnostic => "diagnostic",
         }
     }
@@ -1651,5 +1760,5 @@
     /// handler's own `Fault` crossed the publication boundary, so it is the handler's refusal.
     fn declared_verb_refusal(failure:impl Into<crate::PluginLifecycleFailure>)->DeclaredVerbOutcome{
-        let failure=failure.into();let(code,detail)=match &failure.primary{crate::PluginLifecyclePrimary::Fault(fault)=>(fault.code.0.clone(),fault.message.clone()),crate::PluginLifecyclePrimary::Value(error)=>("interactive-job.owner-refused".to_string(),format!("{error:?}")),crate::PluginLifecyclePrimary::Domain(domain)=>(domain.code().to_owned(),domain.message().to_owned())};
+        let failure=failure.into();let(code,detail)=match &failure.primary{crate::PluginLifecyclePrimary::Fault(fault)=>(fault.code.0.clone(),fault.message.clone()),crate::PluginLifecyclePrimary::Value(error)=>("interactive-job.owner-refused".to_string(),format!("{error:?}")),crate::PluginLifecyclePrimary::Domain(domain)=>(domain.code().to_owned(),domain.message().to_owned()),crate::PluginLifecyclePrimary::Receiving(original)=>("interactive-job.receiving-refused".to_owned(),original.message().to_owned())};
         if(code.starts_with("interactive-job.")&&code!="interactive-job.app-owned-output")||code=="app.command.unsupported"{DeclaredVerbOutcome::Unreachable{code,detail,failure:Some(failure)}}else{DeclaredVerbOutcome::Refused{code,detail,failure:Some(failure)}}
     }
@@ -1806,5 +1915,5 @@
 
     /// 🎬️ Projects the same stationary render source without donating its native resources to the fixture.
-    pub(crate) async fn render_registered_fixture_app<A,M>(app:&mut VcsArtifactApp<A,M>,body_key:&str,view:&crate::ViewModel,cx:&mut semio_framework_job::StepContext<'_>)->Result<String,semio_framework_job::OriginalTurnFailure>
+    pub async fn render_registered_fixture_app<A,M>(app:&mut VcsArtifactApp<A,M>,body_key:&str,view:&crate::ViewModel,cx:&mut semio_framework_job::StepContext<'_>)->Result<String,semio_framework_job::OriginalTurnFailure>
     where A:ArtifactApp+Default,M:super::SpaceMember+super::MemberFactory+Send+'static{
         for _ in 0..OriginalFixtureCaller::maximum_turns(){
@@ -2087,4 +2196,5 @@
     pub async fn assert_two_instances_converge<A, P>(channel: &str, command_a: A::Command, command_b: A::Command, probe: impl Fn(&VcsArtifactApp<A>) -> P, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)
     where
+        A::Command:semio_framework_value::ToValue,
         A: ArtifactApp + Default,
         P: PartialEq + std::fmt::Debug,
@@ -2093,6 +2203,6 @@
         let (mut instance_a, mut instance_b) = paired_apps::<A>(channel, mounted_policy, identity).await;
         let genesis = probe(&instance_a);
-        instance_a.dispatch_typed(command_a, &meta("actor-a"), identity,cx).await.expect("a applies its edit");
-        instance_b.dispatch_typed(command_b, &meta("actor-b"), identity,cx).await.expect("b applies its edit");
+        dispatch_original_fixture_command(&mut instance_a,&command_a,&meta("actor-a"),mounted_policy,identity,cx).await.expect("a applies its edit");
+        dispatch_original_fixture_command(&mut instance_b,&command_b,&meta("actor-b"),mounted_policy,identity,cx).await.expect("b applies its edit");
         exchange_and_assert_convergence(&mut instance_a, &mut instance_b, "actor-a", &probe, mounted_policy, identity,cx).await;
         assert_ne!(probe(&instance_a), genesis, "the replicated edits must actually land, not converge on the untouched genesis");
@@ -2120,4 +2230,5 @@
     pub async fn assert_two_registered_instances_converge_with_members<A, M, P, Manifest, Build>(channel: &str, manifest: Build, command_a: A::Command, command_b: A::Command, probe: impl Fn(&VcsArtifactApp<A, M>) -> P, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)
     where
+        A::Command:semio_framework_value::ToValue,
         A: ArtifactApp + Default,
         M: super::SpaceMember + super::MemberFactory + Send + 'static,
@@ -2132,7 +2243,7 @@
         let genesis = probe(&instance_a);
         let receiver = meta("actor-a").instance_id;
-        instance_a.dispatch_typed(command_a, &meta("actor-a"), identity,cx).await.expect("a applies its edit");
+        dispatch_original_fixture_command(&mut instance_a,&command_a,&meta("actor-a"),mounted_policy,identity,cx).await.expect("a applies its edit");
         settle_registered_typed_operation(&mut instance_a, receiver, mounted_policy, identity,cx).await.expect("a's edit publishes");
-        instance_b.dispatch_typed(command_b, &meta("actor-b"), identity,cx).await.expect("b applies its edit");
+        dispatch_original_fixture_command(&mut instance_b,&command_b,&meta("actor-b"),mounted_policy,identity,cx).await.expect("b applies its edit");
         settle_registered_typed_operation(&mut instance_b, receiver, mounted_policy, identity,cx).await.expect("b's edit publishes");
         exchange_and_assert_convergence(&mut instance_a, &mut instance_b, "actor-a", &probe, mounted_policy, identity,cx).await;
@@ -2182,4 +2293,5 @@
     pub async fn assert_ingest_idempotent<A, P>(command: A::Command, probe: impl Fn(&VcsArtifactApp<A>) -> P, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)
     where
+        A::Command:semio_framework_value::ToValue,
         A: ArtifactApp + Default,
         P: PartialEq + std::fmt::Debug,
@@ -2189,5 +2301,5 @@
         let (near, mut far) = MemoryBackbone::pair("mem://testkit-idempotent", "mem://testkit-idempotent").await;
         sender.attach_backbone(store::os_store::backbone::Backbones::Memory(near)).await.expect("attach sender");
-        sender.dispatch_typed(command, &meta("local"), identity,cx).await.expect("apply command");
+        dispatch_original_fixture_command(&mut sender,&command,&meta("local"),mounted_policy,identity,cx).await.expect("apply command");
 
         let mut envelopes = Vec::new();
@@ -2224,4 +2336,5 @@
     pub async fn assert_registered_ingest_idempotent_with_members<A, M, P, Manifest, Build>(manifest: Build, command: A::Command, probe: impl Fn(&VcsArtifactApp<A, M>) -> P, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)
     where
+        A::Command:semio_framework_value::ToValue,
         A: ArtifactApp + Default,
         M: super::SpaceMember + super::MemberFactory + Send + 'static,
@@ -2237,5 +2350,5 @@
         let (near, mut far) = MemoryBackbone::pair("mem://testkit-idempotent", "mem://testkit-idempotent").await;
         sender.attach_backbone(store::os_store::backbone::Backbones::Memory(near)).await.expect("attach sender");
-        sender.dispatch_typed(command, &meta("local"), identity,cx).await.expect("apply command");
+        dispatch_original_fixture_command(&mut sender,&command,&meta("local"),mounted_policy,identity,cx).await.expect("apply command");
         settle_registered_typed_operation(&mut sender, meta("local").instance_id, mounted_policy, identity,cx).await.expect("the edit publishes");
         let mut envelopes = Vec::new();
@@ -2267,8 +2380,8 @@
     /// applying anything (contract §5.1) — the guest-side half of "propose instead of apply".
     /// Returns the draft so a test can inspect `local_ops`/`foreign`.
-    pub async fn assert_proposes_transaction<A: ArtifactApp>(app: &mut VcsArtifactApp<A>, command: A::Command, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> super::TransactionProposalDraft {
+    pub async fn assert_proposes_transaction<A: ArtifactApp>(app: &mut VcsArtifactApp<A>, command: A::Command, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> super::TransactionProposalDraft where A::Command:semio_framework_value::ToValue {
         let generation_before = app.stores.store.generation();
         let edits_before = app.stores.store.envelope().vcs.edits.len();
-        app.dispatch_typed(command, &meta("local"), identity,cx).await.expect("dispatch a command whose mutations carry foreign steps");
+        dispatch_original_fixture_command(app,&command,&meta("local"),mounted_policy,identity,cx).await.expect("dispatch a command whose mutations carry foreign steps");
         settle_registered_typed_operation(app, meta("local").instance_id, mounted_policy, identity,cx).await.expect("a proposing dispatch settles its own operation");
         assert_eq!(app.stores.store.generation(), generation_before, "a proposed transaction must not bump the document store's generation");
@@ -2404,6 +2517,6 @@
         }
 
-        fn mounted_job_prepare_snapshot_read(operation: super::AppRenderOperationContext, snapshot: &Self::Snapshot) -> bool {
-            V::mounted_job_prepare_snapshot_read(operation, snapshot)
+        fn mounted_job_prepare_snapshot_read(operation: super::AppRenderOperationContext, snapshot: &Self::Snapshot, windows: &super::WindowConfigOwnerRegistry, view: Option<&semio_framework::ViewModel>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> crate::MountedSnapshotReadPreparation {
+            V::mounted_job_prepare_snapshot_read(operation, snapshot, windows, view, grant)
         }
 
@@ -2502,5 +2615,5 @@
         }
 
-        fn pending_effects(owner: &super::ArtifactInstanceOperationOwnerHandle, doc: &super::ArtifactView<'_, Self::Snapshot>, cfg: &super::ConfigView<'_, Self::Config>, view: Option<&super::ViewModel>) -> Vec<super::Effect> {
+        fn pending_effects(owner: &super::ArtifactInstanceOperationOwnerHandle, doc: &crate::ArtifactPendingView<'_, Self::Snapshot>, cfg: &super::ConfigView<'_, Self::Config>, view: Option<&super::ViewModel>) -> Vec<super::Effect> {
             V::pending_effects(owner, doc, cfg, view)
         }
@@ -2604,5 +2717,5 @@
             &super::ArtifactView::new(snapshot, &history),
             &super::ConfigView { snapshot: &config, window: None },
-            &super::InteractionView { state: &state, hover: &hover, peers: &peers, draft_references: super::empty_draft_references() },
+            &super::InteractionView::new(&state, &hover, &peers, super::empty_draft_references()),
             None,
             &super::DraftView { snapshot: &draft },
@@ -2640,5 +2753,5 @@
             let hover = InteractionHoverState::new();
             let peers = PeerPresenceRoot::empty();
-            let interaction = InteractionView { state: &state, hover: &hover, peers: &peers, draft_references: super::empty_draft_references() };
+            let interaction = InteractionView::new(&state, &hover, &peers, super::empty_draft_references());
             ViewerApp::<V>::handle(&V::Command::default(), &doc, &cfg, &interaction, None, &draft, &semio_framework_2d::compute::EngineHandles::empty()).await.map(|emit| (emit.artifact_mutations.is_empty(), emit.draft_mutations.is_empty()))
         };


### 32 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/⏳️completion/🦀️.rs
B/H/S/current: 584/0/494/0
--- 
+++ 
@@ -88,72 +88,9 @@
 }
 
-/// 🧵️ `TestApp`'s app-owned reducer job. Deliberately the SAME minimal shape as `TestRestartJob` and
-/// `DummyFixtureJob` rather than the generic `retained_command::ArtifactRetainedCommandJob`: that
-/// type inlines an `Emit`, an `EphemeralEmit`, a 512-byte checkpoint buffer and two wire-page
-/// owners, and the framework-reserved dispatch generator this fixture also drives is already
-/// measured at ~1.75 MiB of the 2 MiB bounded thread stack
-/// (`one_framework_reserved_route_fits_a_bounded_thread_stack`).
-struct TestRawLease(crate::app::ArtifactToolRawInput);
-
-struct TestRawLeaseCursor {
-    lease: std::mem::ManuallyDrop<Option<crate::app::ArtifactToolRawInput>>,
-}
-
-impl semio_framework_value::retirement::RetireOwned for TestRawLease {
-    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
-        Box::new(TestRawLeaseCursor { lease: std::mem::ManuallyDrop::new(Some(self.0)) })
-    }
-
-    fn retirement_birth_bytes(&self) -> Option<usize> {
-        Some(std::mem::size_of::<TestRawLeaseCursor>())
-    }
-
-    fn controlled_retirement_supported() -> bool {
-        true
-    }
-}
-
-impl semio_framework_value::retirement::RetirementCursor for TestRawLeaseCursor {
-    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_value::retirement::RetirementStep {
-        use semio_framework_value::retirement::RetirementStep;
-        if self.lease.is_none() {
-            return RetirementStep::Complete;
-        }
-        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
-            return RetirementStep::BudgetExhausted;
-        }
-        drop(self.lease.take());
-        RetirementStep::Complete
-    }
-
-    fn terminal_is_empty(&self) -> bool {
-        self.lease.is_none()
-    }
-
-    fn next_birth_bytes(&self, _copy: usize) -> Option<usize> {
-        Some(0)
-    }
-
-    fn next_close_byte_demand(&self) -> Option<usize> {
-        Some(0)
-    }
-
-    fn terminal_release_bytes(&self) -> Option<usize> {
-        self.lease.is_none().then_some(std::mem::size_of::<Self>())
-    }
-}
-
-impl Drop for TestRawLeaseCursor {
-    fn drop(&mut self) {
-        assert!(std::thread::panicking() || self.lease.is_none(), "fixture raw wire lease reached Drop before it returned");
-        if self.lease.is_none() {
-            unsafe { std::mem::ManuallyDrop::drop(&mut self.lease) };
-        }
-    }
-}
-
 #[derive(semio_framework_value::RetireOwned)]
 struct TestAppCommandInputs {
-    raw_wire: TestRawLease,
+    controller_id:String,
+    tool_id:String,
+    schema_id:String,
     window_config: Option<crate::app::WindowConfigSnapshot>,
     interaction_state: semio_framework_value::retirement::shared::SharedControlledRetirement<InteractionState>,
@@ -305,20 +242,7 @@
         test_app_command_contract()
     }
-    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
-        Ok(payload)
-    }
-    fn create_job_from_wire_pages_with_payload(
-        &mut self,
-        _operation: semio_framework_job::Operation,
-        mut payload: Self::Payload,
-        input: action_bus::RetainedToolWireInput,
-        checkpoint: Option<action_bus::RetainedToolWireInput>,
-    ) -> Result<Self::Job, (ToolJobFactoryError, action_bus::RetainedToolWireInput, Option<action_bus::RetainedToolWireInput>)> {
-        if checkpoint.is_some() {
-            return Err((ToolJobFactoryError::new("test app command resume starts a fresh command owner"), input, checkpoint));
-        }
-        payload.raw = Some(input);
-        Ok(payload)
-    }
+    fn admission_demands(&self,_operation:&semio_framework_job::Operation,payload:&Self::Payload,input:Option<&semio_framework::RetainedToolWireInput>,checkpoint:Option<&semio_framework::RetainedToolWireInput>)->Result<semio_framework::action_bus::ToolJobAdmissionDemand,semio_framework_value::ValueError>{if checkpoint.is_some()||(input.is_some()&&payload.raw.is_some()){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original typed job refuses a replacement wire owner or restart checkpoint"));}Ok(semio_framework::action_bus::ToolJobAdmissionDemand::reused_job())}
+fn admit_job(&mut self,context:&mut semio_framework_job::StepContext<'_>,operation:&semio_framework_job::Operation,payload:&mut Option<Box<Self::Payload>>,input:&mut Option<semio_framework::RetainedToolWireInput>,checkpoint:&mut Option<semio_framework::RetainedToolWireInput>,output:&mut Option<Box<Self::Job>>)->Result<bool,semio_framework_job::OriginalTurnFailure>{let demand=self.admission_demands(operation,payload.as_deref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original typed job payload is absent"))?,input.as_ref(),checkpoint.as_ref())?;semio_framework::action_bus::admit_original_tool_job_controlled(context,operation,payload,input,checkpoint,output,demand,|_,payload,input,_|{if input.is_some(){payload.as_mut().expect("retained original job").raw=input.take();}let job=payload.take().expect("admitted same original job");Ok((job,semio_framework_job::RetainedCloneProgress{copied_items:1,..Default::default()}))})}
+
 }
 
@@ -365,24 +289,14 @@
 }
 
-async fn test_restart_build<const RETAINED: bool, const TOOLS: u8>(request: ArtifactOwnedToolJobRequest<TestApp<RETAINED, TOOLS>>) -> Result<Option<ToolOperationSpec>, Fault> {
-    if RETAINED {
-        assert!(matches!(request.command.as_ref(), TestCommand::ApplyCountFromTask { .. }));
-        assert!(request.snapshot.label.is_empty(), "restart fixture begins from its actual fresh snapshot");
-        return test_app_command_spec::<RETAINED, TOOLS>(request);
-    }
-    if TOOLS == TEST_APP_TOOLS_NONE || !TEST_APP_COMMAND_TOOL_IDS.contains(&request.tool_id.as_str()) {
-        return Ok(None);
-    }
-    test_app_command_spec::<RETAINED, TOOLS>(request)
-}
+fn test_restart_build<const RETAINED:bool,const TOOLS:u8>(request:&mut Option<ArtifactOwnedToolJobRequest<TestApp<RETAINED,TOOLS>>>,output:&mut Option<semio_framework::ErasedToolJob>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{let original=request.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original fixture request absent"))?;if RETAINED{assert!(matches!(original.command.as_ref(),TestCommand::ApplyCountFromTask{..}));assert!(original.snapshot.label.is_empty());}else if TOOLS==TEST_APP_TOOLS_NONE||!TEST_APP_COMMAND_TOOL_IDS.contains(&original.tool_id.as_str()){return Ok(false)}test_app_command_job(request,output,cx)}
 
 /// 🧵️ Builds the job on its OWN synchronous frame — every real editor's
 /// `ArtifactEditor::build_tool_job` is sync for the same bounded-stack reason.
 #[inline(never)]
-fn test_app_command_spec<const RETAINED: bool, const TOOLS: u8>(request: ArtifactOwnedToolJobRequest<TestApp<RETAINED, TOOLS>>) -> Result<Option<ToolOperationSpec>, Fault> {
+fn test_app_command_job<const RETAINED:bool,const TOOLS:u8>(request:&mut Option<ArtifactOwnedToolJobRequest<TestApp<RETAINED,TOOLS>>>,output:&mut Option<semio_framework::ErasedToolJob>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{crate::app_testing::mutation_fixture::job_close::admit_fixture_tool_job(request,output,cx,|mut request|{
     let job = TestAppCommandJob {
         rejected: None,
         admitted: false,
-        residual: Some(TestAppCommandInputs { raw_wire: TestRawLease(request.raw_wire), window_config: request.window_config, interaction_state: semio_framework_value::retirement::shared::SharedControlledRetirement::lease(request.interaction_state), interaction_hover: semio_framework_value::retirement::shared::SharedControlledRetirement::lease(request.interaction_hover), instance_operation_owner: request.instance_operation_owner, output_chunks: request.output_chunks }),
+        residual: Some(TestAppCommandInputs { controller_id:request.controller_id,tool_id:request.tool_id,schema_id:request.payload_schema_id, window_config: request.window_config, interaction_state: semio_framework_value::retirement::shared::SharedControlledRetirement::lease(request.interaction_state), interaction_hover: semio_framework_value::retirement::shared::SharedControlledRetirement::lease(request.interaction_hover), instance_operation_owner: request.instance_operation_owner, output_chunks: request.output_chunks }),
         metadata: Some(AppOperationContext { app_instance_id: request.app_instance_id, parent_document_id: request.parent_document_id, operation_id: request.operation.operation.0, generation: request.operation.generation.0, canonical_base_revision: request.canonical_base_revision, retained: request.retained, authoring_seed: request.authoring_seed }),
         command: Some(request.command),
@@ -394,10 +308,10 @@
         context: Some(request.context),
         completion: Some(request.completion),
-        raw: None,
+        raw: request.input.take(),
         page: 0,
         closing: false,
     };
-    Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, job, request.operation)))
-}
+
+job})}
 
 async fn test_restart_registry() -> AppActionRegistry {
@@ -501,10 +415,7 @@
     fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
     fn execution_contract(&self) -> ToolExecutionContract { ToolExecutionContract::resumable(4_096, 4, 1, 4_096, 7_500, 1, 1) }
-    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> { Ok(payload) }
-    fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: action_bus::RetainedToolWireInput, checkpoint: Option<action_bus::RetainedToolWireInput>) -> Result<Self::Job, (ToolJobFactoryError, action_bus::RetainedToolWireInput, Option<action_bus::RetainedToolWireInput>)> {
-        if checkpoint.is_some() { return Err((ToolJobFactoryError::new("restart resume starts a fresh command owner"), input, checkpoint)); }
-        payload.raw = Some(input);
-        Ok(payload)
-    }
+    fn admission_demands(&self,_operation:&semio_framework_job::Operation,payload:&Self::Payload,input:Option<&semio_framework::RetainedToolWireInput>,checkpoint:Option<&semio_framework::RetainedToolWireInput>)->Result<semio_framework::action_bus::ToolJobAdmissionDemand,semio_framework_value::ValueError>{if checkpoint.is_some()||(input.is_some()&&payload.raw.is_some()){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original typed job refuses a replacement wire owner or restart checkpoint"));}Ok(semio_framework::action_bus::ToolJobAdmissionDemand::reused_job())}
+fn admit_job(&mut self,context:&mut semio_framework_job::StepContext<'_>,operation:&semio_framework_job::Operation,payload:&mut Option<Box<Self::Payload>>,input:&mut Option<semio_framework::RetainedToolWireInput>,checkpoint:&mut Option<semio_framework::RetainedToolWireInput>,output:&mut Option<Box<Self::Job>>)->Result<bool,semio_framework_job::OriginalTurnFailure>{let demand=self.admission_demands(operation,payload.as_deref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original typed job payload is absent"))?,input.as_ref(),checkpoint.as_ref())?;semio_framework::action_bus::admit_original_tool_job_controlled(context,operation,payload,input,checkpoint,output,demand,|_,payload,input,_|{if input.is_some(){payload.as_mut().expect("retained original job").raw=input.take();}let job=payload.take().expect("admitted same original job");Ok((job,semio_framework_job::RetainedCloneProgress{copied_items:1,..Default::default()}))})}
+
 }
 
@@ -582,3 +493,2 @@
 }
 //#endregion 🫧️RestartTransientOwner
-                 


### 33 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures/🦀️.rs
B/H/S/current: 106/14/106/14
--- 
+++ 
@@ -13,5 +13,5 @@
     use super::*;
     use crate::app_views::{NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation};
-    use protocol::{Mutation, MutationDiff, MutationKind, MutationLeaf, OpBinary, OpText};
+    use protocol::{Mutation, ColdMutationDiff,MutationDiff, MutationKind, MutationLeaf, OpBinary, OpText};
 
     /// 🧾️ The committed wire witnesses decode through the aggregate's `FromValue` and re-encode to exactly the committed JSON.
@@ -43,23 +43,23 @@
         let presence = ChangePublicationPresence { revision: 7 };
         let presence_before = PublicationPresence { revision: 3 };
-        assert_eq!(protocol::apply_diff(&PublicationPresenceDiff::default(), &presence_before).unwrap(), presence_before);
+        assert_eq!(protocol::apply_diff_cold(&PublicationPresenceDiff::default(), &presence_before).unwrap(), presence_before);
         let mut presence_diff = PublicationPresenceDiff::default();
         presence_diff.absorb(PublicationPresenceDiff { revision: Some(7) });
-        assert_eq!(protocol::apply_diff(&presence_diff, &presence_before).unwrap(), PublicationPresence { revision: 7 });
-        let presence_after = protocol::apply_diff(presence.diff(&presence_before).diff(), &presence_before).unwrap();
+        assert_eq!(protocol::apply_diff_cold(&presence_diff, &presence_before).unwrap(), PublicationPresence { revision: 7 });
+        let presence_after = protocol::apply_diff_cold(presence.diff(&presence_before).diff(), &presence_before).unwrap();
         assert_eq!(presence_after, PublicationPresence { revision: 7 });
         let [presence_inverse] = <[PublicationPresenceMutation; 1]>::try_from(presence.inverse(&presence_before).expect("valid retained mutation inverse fixture")).unwrap();
-        assert_eq!(protocol::apply_diff(presence_inverse.diff(&presence_after).diff(), &presence_after).unwrap(), presence_before);
+        assert_eq!(protocol::apply_diff_cold(presence_inverse.diff(&presence_after).diff(), &presence_after).unwrap(), presence_before);
 
         let transient = ChangePublicationTransient { revision: 11 };
         let transient_before = PublicationTransient { revision: 5 };
-        assert_eq!(protocol::apply_diff(&PublicationTransientDiff::default(), &transient_before).unwrap(), transient_before);
+        assert_eq!(protocol::apply_diff_cold(&PublicationTransientDiff::default(), &transient_before).unwrap(), transient_before);
         let mut transient_diff = PublicationTransientDiff::default();
         transient_diff.absorb(PublicationTransientDiff { revision: Some(11) });
-        assert_eq!(protocol::apply_diff(&transient_diff, &transient_before).unwrap(), PublicationTransient { revision: 11 });
-        let transient_after = protocol::apply_diff(transient.diff(&transient_before).diff(), &transient_before).unwrap();
+        assert_eq!(protocol::apply_diff_cold(&transient_diff, &transient_before).unwrap(), PublicationTransient { revision: 11 });
+        let transient_after = protocol::apply_diff_cold(transient.diff(&transient_before).diff(), &transient_before).unwrap();
         assert_eq!(transient_after, PublicationTransient { revision: 11 });
         let [transient_inverse] = <[PublicationTransientMutation; 1]>::try_from(transient.inverse(&transient_before).expect("valid retained mutation inverse fixture")).unwrap();
-        assert_eq!(protocol::apply_diff(transient_inverse.diff(&transient_after).diff(), &transient_after).unwrap(), transient_before);
+        assert_eq!(protocol::apply_diff_cold(transient_inverse.diff(&transient_after).diff(), &transient_after).unwrap(), transient_before);
 
         assert_eq!(<PublicationPresenceMutation as Mutation<PublicationPresence>>::DESCRIPTORS, &[ChangePublicationPresence::DESCRIPTOR]);


### 34 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-child-member-registry/🦀️.rs
B/H/S/current: 650/628/650/628
--- 
+++ 
@@ -73,6 +73,6 @@
         let grant=|name:&str|{let v=law[name].as_array().unwrap();RetainedCloneGrant{maximum_items:v[0].as_u64().unwrap()as usize,maximum_copy_bytes:v[1].as_u64().unwrap()as usize,maximum_capacity_bytes:v[2].as_u64().unwrap()as usize,maximum_release_bytes:v[3].as_u64().unwrap()as usize,maximum_depth:v[4].as_u64().unwrap()as usize}};
         let policy=grant("phasePolicy");let incoming=grant("incomingRemaining");let expected=grant("admitted");
-        let(actual,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||crate::plugin_runtime::original_plugin_turn_grant(policy,incoming));assert_eq!(actual,expected);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
-        for mask in 0..32{let denied=RetainedCloneGrant{maximum_items:if mask&1!=0{0}else{incoming.maximum_items},maximum_copy_bytes:if mask&2!=0{0}else{incoming.maximum_copy_bytes},maximum_capacity_bytes:if mask&4!=0{0}else{incoming.maximum_capacity_bytes},maximum_release_bytes:if mask&8!=0{0}else{incoming.maximum_release_bytes},maximum_depth:if mask&16!=0{0}else{incoming.maximum_depth}};let(actual,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||crate::plugin_runtime::original_plugin_turn_grant(policy,denied));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(actual,RetainedCloneGrant{maximum_items:if mask&1!=0{0}else{expected.maximum_items},maximum_copy_bytes:if mask&2!=0{0}else{expected.maximum_copy_bytes},maximum_capacity_bytes:if mask&4!=0{0}else{expected.maximum_capacity_bytes},maximum_release_bytes:if mask&8!=0{0}else{expected.maximum_release_bytes},maximum_depth:if mask&16!=0{0}else{expected.maximum_depth}});}
+        let(actual,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_plugin_ownership::bounded_ownership::original_plugin_turn_grant(policy,incoming));assert_eq!(actual,expected);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
+        for mask in 0..32{let denied=RetainedCloneGrant{maximum_items:if mask&1!=0{0}else{incoming.maximum_items},maximum_copy_bytes:if mask&2!=0{0}else{incoming.maximum_copy_bytes},maximum_capacity_bytes:if mask&4!=0{0}else{incoming.maximum_capacity_bytes},maximum_release_bytes:if mask&8!=0{0}else{incoming.maximum_release_bytes},maximum_depth:if mask&16!=0{0}else{incoming.maximum_depth}};let(actual,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_plugin_ownership::bounded_ownership::original_plugin_turn_grant(policy,denied));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(actual,RetainedCloneGrant{maximum_items:if mask&1!=0{0}else{expected.maximum_items},maximum_copy_bytes:if mask&2!=0{0}else{expected.maximum_copy_bytes},maximum_capacity_bytes:if mask&4!=0{0}else{expected.maximum_capacity_bytes},maximum_release_bytes:if mask&8!=0{0}else{expected.maximum_release_bytes},maximum_depth:if mask&16!=0{0}else{expected.maximum_depth}});}
         eprintln!("[DEBUG] original exchange native authority axes=5 denialShapes=32 heapBirth=0 heapRelease=0 originalPolicyUnchanged=true");
     }


### 35 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs
B/H/S/current: 1951/36/1961/36
--- 
+++ 
@@ -185,5 +185,5 @@
             if self.prepared.is_none() {
                 let request = self.request.take().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "two-turn publication-presence preparation lost its owner bundle"))?;
-                let next_root = protocol::apply_diff(request.mutation.diff(request.base.as_ref()).diff(), request.base.as_ref()).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))?;
+                let next_root = protocol::apply_diff_cold(request.mutation.diff(request.base.as_ref()).diff(), request.base.as_ref()).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))?;
                 self.prepared = Some(store::ArtifactEphemeralOneItemPrepared { next_root: std::sync::Arc::new(next_root) });
                 self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: 2, digest: [2; 32] };
@@ -247,4 +247,6 @@
 
     impl store::ErasedSnapshotRetirement for PublicationPresenceLocalRootRetirement {
+ fn next_demand(&self,maximum_body_bytes:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{let _=maximum_body_bytes;Ok(semio_framework_value::RetirementDemand{depth:usize::from(self.root.is_some()),..Default::default()})}
+
         fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
             use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
@@ -373,5 +375,5 @@
                 publication_lanes: &[ArtifactToolPublicationLane::Presence],
                 session: None,
-                session_rejected: None,
+                session_rejected: None,producer_source:None,producer_request:None,producer_request_retirement:None,
                 reserved_producer: None, completion: None,
                 completion_retirement:None,publication_retirement:None,output_retirement:None,raw_input: None,
@@ -390,5 +392,5 @@
                 captured_child_content: Some(std::sync::Arc::new(ChildContentView::EMPTY)),
                 captured_child_content_generation: 0,
-                result_page: None,
+                result_page: None, diagnostics:None,
                 result_page_presented: false,
                 result_sequence: 0,
@@ -435,9 +437,10 @@
             mounted.cancellation_lease.as_ref().unwrap().cancel();
             if boundary == "awaitingAck" {
-                let page = mounted.result_page.as_ref().unwrap().clone();
+                let mut page = mounted.take_fixture_result_page(crate::app_testing::artifact_app_laws::fixture_mounted_policy().preparation).unwrap();
                 assert!(!mounted.reject_cancelled_publication().unwrap());
                 assert_eq!(mounted.result_page.as_ref().unwrap().token, page.token);
                 assert_eq!(mounted.result_page.as_ref().unwrap().bytes(), page.bytes());
-                assert!(mounted.acknowledge_result_page(page.token).unwrap());
+                assert!(mounted.acknowledge_result_page(crate::app_testing::artifact_app_laws::close_fixture_result_page(&mut page,crate::app_testing::artifact_app_laws::fixture_mounted_policy().close)).unwrap());
+                mounted.close_fixture_original_page(crate::app_testing::artifact_app_laws::fixture_mounted_policy().close);
             }
             app.typed_ctx().publish_mounted_typed_operation_unit(&mut mounted, crate::app_testing::artifact_app_laws::fixture_mounted_policy().maintenance).unwrap();
@@ -485,5 +488,5 @@
             }
             if permit.is_some() {
-                assert!(matches!(presence.advance_publish_one(&mut pending, grant).unwrap(), store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Published(_)));
+                assert!(matches!(presence.advance_publish_one(&mut pending, grant).unwrap(), store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Published(_, _)));
                 assert!(pending.acknowledge());
             }
@@ -595,5 +598,5 @@
                     publication_lanes: &[ArtifactToolPublicationLane::Artifact],
                     session: None,
-                    session_rejected: None,
+                    session_rejected: None,producer_source:None,producer_request:None,producer_request_retirement:None,
                     reserved_producer: None, completion: None,
                     completion_retirement:None,publication_retirement:None,output_retirement:None,raw_input: None,
@@ -612,5 +615,5 @@
                     captured_child_content: Some(std::sync::Arc::new(ChildContentView::EMPTY)),
                     captured_child_content_generation: 0,
-                    result_page: None,
+                    result_page: None, diagnostics:None,
                     result_page_presented: false,
                     result_sequence: 0,
@@ -657,25 +660,27 @@
                 mounted.cancellation_lease.as_ref().unwrap().cancel();
                 if boundary == "awaitingAck" {
-                    let receipt = mounted.result_page.as_ref().unwrap().clone();
+                    let receipt = {let original=mounted.result_page.as_ref().unwrap();(original.token,original.bytes().to_vec())};
                     assert!(!mounted.reject_cancelled_publication().unwrap());
-                    assert_eq!(mounted.result_page.as_ref().unwrap().bytes(), receipt.bytes());
+                    assert_eq!(mounted.result_page.as_ref().unwrap().bytes(), receipt.1.as_slice());
                     if delayed_ack {
-                        let presented = mounted.take_result_page().unwrap();
+                        let mut presented = mounted.take_fixture_result_page(crate::app_testing::artifact_app_laws::fixture_mounted_policy().preparation).unwrap();
                         assert_eq!(presented.token.attempt, fixture["resultAck"]["attempt"].as_u64().unwrap() as u8);
                         assert_eq!(mounted.stage, MountedTypedCommandFullOperationStage::AwaitingAck);
                         for _ in 0..fixture["resultAck"]["preAckPolls"].as_u64().unwrap() {
-                            assert!(mounted.take_result_page().is_none());
+                            assert!(mounted.take_fixture_result_page(crate::app_testing::artifact_app_laws::fixture_mounted_policy().preparation).is_none());
                         }
-                        assert!(mounted.acknowledge_result_page(presented.token).unwrap());
+                        assert!(mounted.acknowledge_result_page(crate::app_testing::artifact_app_laws::close_fixture_result_page(&mut presented,crate::app_testing::artifact_app_laws::fixture_mounted_policy().close)).unwrap());
+                mounted.close_fixture_original_page(crate::app_testing::artifact_app_laws::fixture_mounted_policy().close);
                     } else {
-                        assert!(mounted.acknowledge_result_page(receipt.token).unwrap());
+                        assert!(mounted.acknowledge_result_page(receipt.0).unwrap());
+                mounted.close_fixture_original_page(crate::app_testing::artifact_app_laws::fixture_mounted_policy().close);
                     }
                 }
                 app.typed_ctx().publish_mounted_typed_operation_unit(&mut mounted, crate::app_testing::artifact_app_laws::fixture_mounted_policy().maintenance).unwrap();
-                let final_page = if delayed_ack {
-                    let presented = mounted.take_result_page().unwrap();
+                let mut final_page = if delayed_ack {
+                    let mut presented = mounted.take_fixture_result_page(crate::app_testing::artifact_app_laws::fixture_mounted_policy().preparation).unwrap();
                     let mut deliveries = 1;
                     for _ in 0..fixture["resultAck"]["preAckPolls"].as_u64().unwrap() {
-                        deliveries += usize::from(mounted.take_result_page().is_some());
+                        deliveries += usize::from(mounted.take_fixture_result_page(crate::app_testing::artifact_app_laws::fixture_mounted_policy().preparation).is_some());
                     }
                     assert_eq!(deliveries, fixture["resultAck"]["deliveries"].as_u64().unwrap() as usize);
@@ -684,5 +689,5 @@
                     presented
                 } else {
-                    mounted.result_page.as_ref().unwrap().clone()
+                    mounted.take_fixture_result_page(crate::app_testing::artifact_app_laws::fixture_mounted_policy().preparation).unwrap()
                 };
                 assert_eq!(final_page.lane, TypedOperationResultLane::Fault);
@@ -973,5 +978,6 @@
                     publication_lanes: &[ArtifactToolPublicationLane::Presence],
                     session: None,
-                    session_rejected: None,
+                    session_rejected: None,producer_source:None,producer_request:None,producer_request_retirement:None,
+                    producer_source:None,producer_request:None,producer_request_retirement:None,
                     reserved_producer: None, completion: None,
                     completion_retirement:None,publication_retirement:None,output_retirement:None,raw_input: None,
@@ -990,5 +996,5 @@
                     captured_child_content: Some(std::sync::Arc::new(ChildContentView::EMPTY)),
                     captured_child_content_generation: 0,
-                    result_page: None,
+                    result_page: None, diagnostics: None, diagnostics:None,
                     result_page_presented: false,
                     result_sequence: 0,
@@ -1020,12 +1026,12 @@
         assert_eq!(stuck.stage, MountedTypedCommandFullOperationStage::AwaitingAck, "a refused worker step leaves its own operation holding its own terminal fault page");
         let mut pages: Vec<(u64, TypedOperationResultLane, String)> = Vec::new();
-        while let Some(page) = app.take_typed_operation_result_page(7) {
+        while let Some(mut page) = crate::app_testing::artifact_app_laws::take_fixture_result_page(&mut app,7,crate::app_testing::artifact_app_laws::fixture_mounted_policy().preparation).expect("original fixture page presentation") {
             let code = if page.lane == TypedOperationResultLane::Fault { crate::tool_contract::decode_typed_operation_fault_page(page.bytes()).code.0 } else { String::new() };
             let operation = page.token.operation;
             pages.push((operation, page.lane, code));
-            if operation == 1 {
-                let acknowledged = app.tools.tool_operations.get_mut(1).expect("the terminated operation stays mounted").acknowledge_result_page(page.token).expect("the terminated operation's own fault page is acknowledgeable");
-                assert!(acknowledged, "a terminated operation must accept the ACK for the page it minted");
-            }
+            let token =crate::app_testing::artifact_app_laws::close_fixture_result_page(&mut page,crate::app_testing::artifact_app_laws::fixture_mounted_policy().close);
+                let acknowledged = app.tools.acknowledge_typed_operation_result(token).expect("each original published page accepts its exact ACK after receiver closure");
+                assert!(acknowledged, "published operation {operation} must retain its original ACK authority");
+
         }
         assert!(
@@ -1053,5 +1059,5 @@
                     publication_lanes: &[],
                     session: None,
-                    session_rejected: None,
+                    session_rejected: None,producer_source:None,producer_request:None,producer_request_retirement:None,
                     reserved_producer: None, completion: None,
                     completion_retirement:None,publication_retirement:None,output_retirement:None,raw_input: None,
@@ -1465,6 +1471,7 @@
         let dispatch_end = source[dispatch_start..].find("pub fn dispatch_typed").map(|offset| dispatch_start + offset).expect("production dispatch end");
         let dispatch = &source[dispatch_start..dispatch_end];
-        assert!(dispatch.contains("dispatch_wire_retained_with_spec"));
-        assert!(dispatch.contains("retained_wire.take()"));
+        assert!(dispatch.contains("PendingLatestWinsCommand {"));
+        assert!(dispatch.contains("admission: Some(admission)"));
+        assert!(dispatch.contains("source_identity:(cx.operation(),cx.generation())"));
 
         let publisher_start = source.rfind("fn publish_mounted_typed_operation_unit").expect("production one-page publisher");
@@ -1498,6 +1505,9 @@
 
         let reactor = [module_source("⚛️reactor/🦀️.rs"), module_source("⚛️reactor/🔄️turn/🦀️.rs")].concat();
-        assert!(reactor.contains("output.typed_operation_results.iter()"));
-        assert!(reactor.contains("page.renderer_exchange_bytes()"));
+        let receiving=include_str!("../../⚛️reactor/🏃️runtime/📮️publication/🦀️.rs");
+        assert!(receiving.contains("ResultPageWireReceiver"));
+        assert!(receiving.contains("cx.consume_retained(p)"));
+        assert!(!reactor.contains("page.renderer_exchange_bytes()"));
+        assert!(reactor.contains("plugin_drive_result_pages"));
         assert!(reactor.contains("TypedOperationResultPage::renderer_ack_token"));
         assert!(reactor.contains("plugin_acknowledge_typed_operation_result(runtime, token)"));
@@ -1804,10 +1814,10 @@
             let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);let mut owner=registry::RetainedSourceRegistry::empty();
             let first=child(&mut control);let first_allocated=first.allocated_bytes();let first_pointer=first.operations[0].byte_ref(0).map(|byte|byte as *const u8).unwrap();let mut incoming=Some(first);let mut allow_zero=|_|true;let mut zero=NativeEncodeControl::new(0,&mut allow_zero);assert!(owner.insert(&mut incoming,&mut zero).is_err());assert_eq!(owner.retained_count(),0);assert!(owner.terminal_is_empty());assert_eq!(incoming.as_ref().unwrap().operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(first_pointer));let one=owner.insert(&mut incoming,&mut control).unwrap();assert!(incoming.is_none());let scalar_clone=one.clone();let another_scalar_clone=scalar_clone.clone();assert_eq!(another_scalar_clone,one);
-            ledger.send("window","tool",&actor,"base",ScrubInput::Tick{gesture:"press".into(),leaves:vec![one]},clock(1)).unwrap();reconcile(&mut owner,&ledger,&mut control).unwrap();
+            ledger.send("window","tool",&actor,"base", &mut Some(ScrubInput::Tick{gesture:"press".into(),leaves:vec![one]}),clock(1)).unwrap();reconcile(&mut owner,&ledger,&mut control).unwrap();
             assert_eq!(owner.borrow(one).unwrap().operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(first_pointer));assert_eq!(owner.retained_count(),1);
             let second=child(&mut control);let second_pointer=second.operations[0].byte_ref(0).map(|byte|byte as *const u8).unwrap();let second_allocated=second.allocated_bytes();let mut incoming=Some(second);let continuation=control.pause().unwrap();let mut reject=|_|false;let mut rejected=NativeEncodeControl::resume(continuation,&mut reject).unwrap();
             assert!(owner.insert(&mut incoming,&mut rejected).is_err());assert_eq!(incoming.as_ref().unwrap().operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(second_pointer));assert!(reconcile(&mut owner,&ledger,&mut rejected).is_err());assert_eq!(owner.borrow(one).unwrap().operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(first_pointer));
             let continuation=rejected.pause().unwrap();let mut allow=|_|true;let mut control=NativeEncodeControl::resume(continuation,&mut allow).unwrap();let two=owner.insert(&mut incoming,&mut control).unwrap();assert!(incoming.is_none());assert_eq!(owner.retained_count(),2);
-            ledger.send("window","tool",&actor,"base",ScrubInput::Tick{gesture:"press".into(),leaves:vec![two]},clock(2)).unwrap();
+            ledger.send("window","tool",&actor,"base", &mut Some(ScrubInput::Tick{gesture:"press".into(),leaves:vec![two]}),clock(2)).unwrap();
             assert!(owner.borrow(one).is_some());assert_eq!(ledger.provisional().copied().collect::<Vec<_>>(),vec![two]);reconcile(&mut owner,&ledger,&mut control).unwrap();assert!(owner.borrow(one).is_none());assert_eq!(owner.borrow(two).unwrap().operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(second_pointer));assert!(owner.take(one,&mut control).is_err());
             assert_eq!(owner.close_step(RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:4096,maximum_depth:64}).unwrap(),RetainedCloneStep::Progress(Default::default()));let mut displaced=0;
@@ -1818,11 +1828,11 @@
                 for _ in 0..16384{match owner.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:4096,maximum_depth:64}).unwrap(){RetainedCloneStep::Progress(progress)=>{let released_items=progress.copied_items;let released_bytes=progress.released_bytes;assert!(progress.copied_bytes<=4096);assert_eq!(progress.retained_capacity_bytes,0);assert!(released_items<=1&&released_bytes<=4096);second_released+=released_bytes;if released_items==0&&released_bytes==0{break}},RetainedCloneStep::Complete(_)=>break}}
                 assert_eq!(owner.retained_count(),0);let third_pointer=second_pointer;let third_allocated=second_allocated;let before_readmission=control.owned_bytes();let three=owner.insert(&mut incoming,&mut control).unwrap();assert!(incoming.is_none());assert!(control.owned_bytes()>=before_readmission&&control.owned_bytes()<=65536);
-                ledger.send("window","tool",&actor,"base",ScrubInput::Tick{gesture:"new-press".into(),leaves:vec![three]},clock(3)).unwrap();let retained=owner.borrow(three).expect("open registry lost monotone identity after abort and bounded source drain");assert_eq!(retained.operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(third_pointer));assert_eq!(second_released,0);assert!(owner.allocated_bytes()>=metadata);reconcile(&mut owner,&ledger,&mut control).unwrap();
-                let ToolStep::Committed(_,leaves)=ledger.send("window","tool",&actor,"base",ScrubInput::Commit{gesture:"new-press".into(),leaves:vec![three]},clock(4)).unwrap()else{panic!("reused open registry did not commit actual new press")};assert_eq!(leaves,vec![three]);let mut published=owner.take(three,&mut control).unwrap();assert_eq!(published.operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(third_pointer));assert_eq!(published.operations[0].len(),8194);reconcile(&mut owner,&ledger,&mut control).unwrap();assert!(owner.take(three,&mut control).is_err());assert_eq!(drain_retained(&mut published),third_allocated);let metadata=owner.allocated_bytes();owner.begin_close();assert_eq!(drain_retained(&mut owner),metadata);
-                assert!(matches!(ledger.send("window","tool",&actor,"base",ScrubInput::Commit{gesture:"new-press".into(),leaves:vec![three]},clock(5)).unwrap(),ToolStep::Idle));
+                ledger.send("window","tool",&actor,"base", &mut Some(ScrubInput::Tick{gesture:"new-press".into(),leaves:vec![three]}),clock(3)).unwrap();let retained=owner.borrow(three).expect("open registry lost monotone identity after abort and bounded source drain");assert_eq!(retained.operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(third_pointer));assert_eq!(second_released,0);assert!(owner.allocated_bytes()>=metadata);reconcile(&mut owner,&ledger,&mut control).unwrap();
+                let ToolStep::Committed(_,leaves)=ledger.send("window","tool",&actor,"base", &mut Some(ScrubInput::Commit{gesture:"new-press".into(),leaves:vec![three]}),clock(4)).unwrap()else{panic!("reused open registry did not commit actual new press")};assert_eq!(leaves,vec![three]);let mut published=owner.take(three,&mut control).unwrap();assert_eq!(published.operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(third_pointer));assert_eq!(published.operations[0].len(),8194);reconcile(&mut owner,&ledger,&mut control).unwrap();assert!(owner.take(three,&mut control).is_err());assert_eq!(drain_retained(&mut published),third_allocated);let metadata=owner.allocated_bytes();owner.begin_close();assert_eq!(drain_retained(&mut owner),metadata);
+                assert!(matches!(ledger.send("window","tool",&actor,"base", &mut Some(ScrubInput::Commit{gesture:"new-press".into(),leaves:vec![three]}),clock(5)).unwrap(),ToolStep::Idle));
             }else{
-                let ToolStep::Committed(_,leaves)=ledger.send("window","tool",&actor,"base",ScrubInput::Commit{gesture:"press".into(),leaves:vec![two]},clock(3)).unwrap()else{panic!("accepted release did not commit")};assert_eq!(leaves,vec![two]);assert!(ledger.is_empty());
+                let ToolStep::Committed(_,leaves)=ledger.send("window","tool",&actor,"base", &mut Some(ScrubInput::Commit{gesture:"press".into(),leaves:vec![two]}),clock(3)).unwrap()else{panic!("accepted release did not commit")};assert_eq!(leaves,vec![two]);assert!(ledger.is_empty());
                 let mut published=owner.take(leaves[0],&mut control).unwrap();assert_eq!(published.operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(second_pointer));assert_eq!(published.operations[0].len(),8194);assert_eq!(published.operations[0].byte_at(0),Some(b'1'));assert_eq!(published.operations[0].byte_at(1),Some(b'7'));for at in 2..8194{assert_eq!(published.operations[0].byte_at(at),Some(b' '));}
-                reconcile(&mut owner,&ledger,&mut control).unwrap();assert!(owner.take(two,&mut control).is_err());assert!(matches!(ledger.send("window","tool",&actor,"base",ScrubInput::Commit{gesture:"press".into(),leaves:vec![two]},clock(4)).unwrap(),ToolStep::Idle));
+                reconcile(&mut owner,&ledger,&mut control).unwrap();assert!(owner.take(two,&mut control).is_err());assert!(matches!(ledger.send("window","tool",&actor,"base", &mut Some(ScrubInput::Commit{gesture:"press".into(),leaves:vec![two]}),clock(4)).unwrap(),ToolStep::Idle));
                 assert_eq!(drain_retained(&mut published),second_allocated);let metadata=owner.allocated_bytes();owner.begin_close();assert_eq!(drain_retained(&mut owner),metadata);
             }


### 36 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/📨️owned-child-dispatch/🪪️admission/🦀️.rs
B/H/S/current: 38/61/47/70
--- 
+++ 
@@ -13,6 +13,7 @@
   let occupied=row["occupied"].as_u64().unwrap()as usize;
   let mut app=contract_composed_app_raw(crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
-  let admitted=app.dispatch_ctx().test_reserved_emit_admission_case(occupied,&meta());
-  drain_and_close_composed_fixture(&mut app, crate::app_testing::artifact_app_laws::fixture_mounted_policy());
+  let mut caller=crate::app_testing::OriginalFixtureCaller::admit().unwrap();let mut decoder=semio_framework_job::OriginalJobDecodeOwner::empty();let mut cx=caller.context().with_original_decoder(&mut decoder);
+  let admitted=app.dispatch_ctx().test_reserved_emit_admission_case(occupied,&meta(),&mut cx);
+  drain_and_close_composed_fixture(&mut app, crate::app_testing::artifact_app_laws::fixture_mounted_policy(),&mut cx);let fuel=cx.fuel_remaining();drop(cx);let fuel=caller.retire_decoder(&mut decoder,fuel).unwrap();caller.retire(fuel).unwrap();
   assert_eq!(admitted,row["admitted"].as_bool().unwrap());
   println!("[DEBUG] reserved producer occupied={occupied} exact original reservation+keyed lease, no producer handoff at saturation, foreign identity unchanged");
@@ -21,10 +22,11 @@
 
 #[semio_framework_async_macros::async_test]
-async fn framework_reserved_emit_cancelled_worker_keeps_original_permit_until_completion_handoff(){
+async fn framework_reserved_emit_cancelled_original_permit_keeps_its_exact_reservation_until_finish(){
  let mut app=contract_composed_app_raw(crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
+ let mut caller=crate::app_testing::OriginalFixtureCaller::admit().unwrap();let mut cx=caller.context();
  let retained=app.dispatch_ctx().test_reserved_emit_cancelled_worker_handoff(&meta()).await;
- drain_and_close_composed_fixture(&mut app, crate::app_testing::artifact_app_laws::fixture_mounted_policy());
- assert!(retained,"cancelled producer must return its original permit so completed original output can enter bounded mounted cancellation");
- println!("[DEBUG] cancelled reserved worker retains original operation+keyed lease through completion handoff, then exact finish");
+ drain_and_close_composed_fixture(&mut app, crate::app_testing::artifact_app_laws::fixture_mounted_policy(),&mut cx);let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).unwrap();
+ assert!(retained,"cancelled producer retains its original admitted permit and keyed reservation until explicit finish");
+ println!("[DEBUG] cancelled reserved producer retains original operation and keyed lease before worker birth, then exact finish");
 }
 
@@ -32,7 +34,14 @@
 async fn framework_reserved_emit_busy_completion_keeps_original_fixed_owner(){
  let mut app=contract_composed_app_raw(crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
- let retained=app.test_reserved_emit_busy_completion_owner(&meta()).await;
+ let mut caller=crate::app_testing::OriginalFixtureCaller::admit().unwrap();let mut cx=caller.context();
+ let retained=app.test_reserved_emit_busy_completion_owner(&meta(),&mut cx).await;
  drain_and_close_composed_fixture(&mut app, crate::app_testing::artifact_app_laws::fixture_mounted_policy());
  assert!(retained,"busy original completion remains in the same pre-admitted operation until bounded close");
  println!("[DEBUG] busy completion retains same operation, original cell/payload pointer and cancelled keyed lease");
 }
+
+#[semio_framework_async_macros::async_test]
+async fn reserved_mounted_source_survives_denied_first_birth_and_reaches_the_actual_worker_candidate(){
+ let policy=crate::app_testing::artifact_app_laws::fixture_mounted_policy();let mut app=contract_composed_app_raw(policy,&mut crate::app_testing::artifact_app_laws::fixture_identity()).await;let mut caller=crate::app_testing::OriginalFixtureCaller::admit().unwrap();let mut decoder=semio_framework_job::OriginalJobDecodeOwner::empty();let mut cx=caller.context().with_original_decoder(&mut decoder);
+ assert!(app.test_reserved_original_before_birth(&meta(),&mut cx).await.unwrap());drain_and_close_composed_fixture(&mut app,policy,&mut cx);let fuel=cx.fuel_remaining();drop(cx);let fuel=caller.retire_decoder(&mut decoder,fuel).unwrap();caller.retire(fuel).unwrap();println!("[DEBUG] actual reserved mounted guard preserves original source with zero denied effects, queues same semantic-bound paid candidate, then closes under the same source");
+}


### 37 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs
B/H/S/current: 10650/103/10701/103
--- 
+++ 
@@ -631,7 +631,6 @@
             ToolJobFactory::execution_contract(&self.0)
         }
-        fn create_job(&mut self, operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
-            ToolJobFactory::create_job(&mut self.0, operation, payload)
-        }
+        fn admission_demands(&self,operation:&semio_framework_job::Operation,payload:&Self::Payload,input:Option<&action_bus::RetainedToolWireInput>,checkpoint:Option<&action_bus::RetainedToolWireInput>)->Result<action_bus::ToolJobAdmissionDemand,semio_framework_value::ValueError>{ToolJobFactory::admission_demands(&self.0,operation,payload,input,checkpoint)}
+fn admit_job(&mut self,cx:&mut semio_framework_job::StepContext<'_>,operation:&semio_framework_job::Operation,payload:&mut Option<Box<Self::Payload>>,input:&mut Option<action_bus::RetainedToolWireInput>,checkpoint:&mut Option<action_bus::RetainedToolWireInput>,output:&mut Option<Box<Self::Job>>)->Result<bool,semio_framework_job::OriginalTurnFailure>{ToolJobFactory::admit_job(&mut self.0,cx,operation,payload,input,checkpoint,output)}
     }
 
@@ -670,23 +669,7 @@
         }
 
-        fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
-            Ok(crate::retained_command::ArtifactRetainedCommandJob::new(payload))
-        }
-
-        fn create_job_from_wire_pages_with_payload(
-            &mut self,
-            _operation: semio_framework_job::Operation,
-            payload: Self::Payload,
-            input: action_bus::RetainedToolWireInput,
-            checkpoint: Option<action_bus::RetainedToolWireInput>,
-        ) -> Result<Self::Job, (ToolJobFactoryError, action_bus::RetainedToolWireInput, Option<action_bus::RetainedToolWireInput>)> {
-            if input.declared_bytes() > TEST_RETAINED_COMMAND_RAW_BYTES || checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.declared_bytes() > crate::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
-                return Err((ToolJobFactoryError::new("test retained command extent"), input, checkpoint));
-            }
-            Ok(match checkpoint {
-                Some(checkpoint) => crate::retained_command::ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
-                None => crate::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input),
-            })
-        }
+        crate::retained_command_factory_admission!(maximum_raw_bytes:TEST_RETAINED_COMMAND_RAW_BYTES,checkpoint_resume:true);
+
+
     }
 
@@ -700,9 +683,12 @@
     const TEST_COMMAND_RETAINED_GRANT: semio_framework_value::RetainedCloneGrant = semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
 
-    async fn test_retained_command_payload(completion: ArtifactToolCompletion<TestApp>) -> crate::retained_command::ArtifactRetainedCommandPayload<TestApp> {
-        crate::retained_command::ArtifactRetainedCommandPayload::new(
+    async fn test_retained_command_payload(completion: ArtifactToolCompletion<TestApp>) -> (crate::retained_command::ArtifactRetainedCommandPayload<TestApp>,crate::app_testing::mutation_fixture::job_close::FixtureSnapshotOwner) {
+        let snapshot=std::sync::Arc::new(TestSnapshot::default());
+        let(snapshot_read,issuer)=crate::app_testing::mutation_fixture::job_close::FixtureSnapshotOwner::new(snapshot.clone(),3,[5;32]);
+        let payload=crate::retained_command::ArtifactRetainedCommandPayload::new(
             crate::retained_command::ArtifactRetainedCommandInputs {
                 command: TestCommand::SetLabel { value: "wire".into() },
-                snapshot: std::sync::Arc::new(TestSnapshot::default()),
+                snapshot,
+                snapshot_read,
                 config: std::sync::Arc::new(TestConfig::default()),
                 history: std::sync::Arc::new(HistoryView::empty()),
@@ -717,12 +703,15 @@
             3,
             Box::new(TestRetainedCommandWork { cursor: 0 }),
-        )
-    }
-
-    async fn test_retained_child_command_payload(completion: ArtifactToolCompletion<TestApp>, emit: Emit<TestMutation, TestConfigMutation, NoDraftMutation>) -> crate::retained_command::ArtifactRetainedCommandPayload<TestApp> {
-        crate::retained_command::ArtifactRetainedCommandPayload::new(
+        );(payload,issuer)
+    }
+
+    async fn test_retained_child_command_payload(completion: ArtifactToolCompletion<TestApp>, emit: Emit<TestMutation, TestConfigMutation, NoDraftMutation>) -> (crate::retained_command::ArtifactRetainedCommandPayload<TestApp>,crate::app_testing::mutation_fixture::job_close::FixtureSnapshotOwner) {
+        let snapshot=std::sync::Arc::new(TestSnapshot::default());
+        let(snapshot_read,issuer)=crate::app_testing::mutation_fixture::job_close::FixtureSnapshotOwner::new(snapshot.clone(),3,[5;32]);
+        let payload=crate::retained_command::ArtifactRetainedCommandPayload::new(
             crate::retained_command::ArtifactRetainedCommandInputs {
                 command: TestCommand::SetLabel { value: "wire".into() },
-                snapshot: std::sync::Arc::new(TestSnapshot::default()),
+                snapshot,
+                snapshot_read,
                 config: std::sync::Arc::new(TestConfig::default()),
                 history: std::sync::Arc::new(HistoryView::empty()),
@@ -737,5 +726,5 @@
             1,
             Box::new(TestRetainedChildCommandWork { emit: Some(emit) }),
-        )
+        );(payload,issuer)
     }
 
@@ -878,7 +867,5 @@
         }
 
-        async fn build_tool_job(request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<ToolOperationSpec>, Fault> {
-            test_restart_build::<RETAINED, TOOLS>(request).await
-        }
+        fn build_tool_job(request:&mut Option<ArtifactOwnedToolJobRequest<Self>>,output:&mut Option<semio_framework::ErasedToolJob>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{test_restart_build::<RETAINED,TOOLS>(request,output,cx)}
 
         fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
@@ -1262,5 +1249,7 @@
 
     struct KeyedTestJobSources {
-        raw_wire: Option<crate::tool_contract::ArtifactToolRawInput>,
+        controller_id:Option<String>,
+        tool_id:Option<String>,
+        schema_id:Option<String>,
         read: Option<store::os_store::snapshot_read::SnapshotRead<TestSnapshot>>,
         snapshot: Option<std::sync::Arc<TestSnapshot>>,
@@ -1282,7 +1271,7 @@
             macro_rules! owned { ($field:ident) => { if self.$field.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.$field).map(Some); } }; }
             macro_rules! lease { ($field:ident) => { if self.$field.is_some() { return store::os_store::snapshot_read::artifact_retirement_shared_lease_birth_demands(&self.$field).map(Some); } }; }
-            if self.raw_wire.is_some() { return Ok(Some(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() })); }
-            if self.window_config.is_some() { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "keyed fixture has no retirement lane for a window config snapshot")); }
-            owned!(read); owned!(context); owned!(instance_owner); owned!(output); owned!(parent_document_id); owned!(authoring_seed);
+
+
+            owned!(controller_id);owned!(tool_id);owned!(schema_id);owned!(window_config);owned!(read); owned!(context); owned!(instance_owner); owned!(output); owned!(parent_document_id); owned!(authoring_seed);
             lease!(snapshot); lease!(config); lease!(history); lease!(interaction); lease!(hover);
             Ok(None)
@@ -1292,6 +1281,6 @@
             macro_rules! owned { ($field:ident) => { if self.$field.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.$field, active, grant).map(Some); } }; }
             macro_rules! lease { ($field:ident) => { if self.$field.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_shared_lease(&mut self.$field, active, grant).map(Some); } }; }
-            if self.raw_wire.take().is_some() { return Ok(Some(semio_framework_value::retained_clone::RetainedCloneStep::Progress(semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() }))); }
-            owned!(read); owned!(context); owned!(instance_owner); owned!(output); owned!(parent_document_id); owned!(authoring_seed);
+
+            owned!(controller_id);owned!(tool_id);owned!(schema_id);owned!(window_config);owned!(read); owned!(context); owned!(instance_owner); owned!(output); owned!(parent_document_id); owned!(authoring_seed);
             lease!(snapshot); lease!(config); lease!(history); lease!(interaction); lease!(hover);
             Ok(None)
@@ -1421,18 +1410,7 @@
             ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1)
         }
-        fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
-            Ok(payload)
-        }
-        fn create_job_from_wire_pages_with_payload(
-            &mut self,
-            _operation: semio_framework_job::Operation,
-            mut payload: Self::Payload,
-            input: action_bus::RetainedToolWireInput,
-            checkpoint: Option<action_bus::RetainedToolWireInput>,
-        ) -> Result<Self::Job, (ToolJobFactoryError, action_bus::RetainedToolWireInput, Option<action_bus::RetainedToolWireInput>)> {
-            assert!(checkpoint.is_none());
-            payload.raw = Some(input);
-            Ok(payload)
-        }
+        fn admission_demands(&self,_operation:&semio_framework_job::Operation,payload:&Self::Payload,input:Option<&semio_framework::RetainedToolWireInput>,checkpoint:Option<&semio_framework::RetainedToolWireInput>)->Result<semio_framework::action_bus::ToolJobAdmissionDemand,semio_framework_value::ValueError>{if checkpoint.is_some()||(input.is_some()&&payload.raw.is_some()){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original typed job refuses a replacement wire owner or restart checkpoint"));}Ok(semio_framework::action_bus::ToolJobAdmissionDemand::reused_job())}
+fn admit_job(&mut self,context:&mut semio_framework_job::StepContext<'_>,operation:&semio_framework_job::Operation,payload:&mut Option<Box<Self::Payload>>,input:&mut Option<semio_framework::RetainedToolWireInput>,checkpoint:&mut Option<semio_framework::RetainedToolWireInput>,output:&mut Option<Box<Self::Job>>)->Result<bool,semio_framework_job::OriginalTurnFailure>{let demand=self.admission_demands(operation,payload.as_deref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original typed job payload is absent"))?,input.as_ref(),checkpoint.as_ref())?;semio_framework::action_bus::admit_original_tool_job_controlled(context,operation,payload,input,checkpoint,output,demand,|_,payload,input,_|{if input.is_some(){payload.as_mut().expect("retained original job").raw=input.take();}let job=payload.take().expect("admitted same original job");Ok((job,semio_framework_job::RetainedCloneProgress{copied_items:1,..Default::default()}))})}
+
     }
 
@@ -1479,11 +1457,11 @@
             registry.register(KeyedTestFactory { keys: vec![ToolFactoryKey::new(registry.controller_id(), "compositeEdit")] })
         }
-        async fn build_tool_job(request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<ToolOperationSpec>, Fault> {
+        fn build_tool_job(request:&mut Option<ArtifactOwnedToolJobRequest<Self>>,output:&mut Option<semio_framework::ErasedToolJob>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{crate::app_testing::mutation_fixture::job_close::admit_fixture_tool_job(request,output,cx,|mut request|{
             assert!(request.snapshot.label.is_empty());
             let base_count = request.snapshot.count;
-            let sources = KeyedTestJobSources { raw_wire: Some(request.raw_wire), read: Some(request.snapshot_read), snapshot: Some(request.snapshot), config: Some(request.config), history: Some(request.history), window_config: request.window_config, interaction: Some(request.interaction_state), hover: Some(request.interaction_hover), context: Some(request.context), instance_owner: Some(request.instance_operation_owner), output: Some(request.output_chunks), parent_document_id: Some(request.parent_document_id), authoring_seed: Some(request.authoring_seed) };
-            let job = KeyedTestJob { command: Some(request.command), completion: Some(request.completion), raw: None, page: 0, base_count, closing: false, sources: Some(sources), rejected: None, admitted: false, retirement: None };
-            Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, job, request.operation)))
-        }
+            let sources = KeyedTestJobSources { controller_id:Some(request.controller_id),tool_id:Some(request.tool_id),schema_id:Some(request.payload_schema_id), read: Some(request.snapshot_read), snapshot: Some(request.snapshot), config: Some(request.config), history: Some(request.history), window_config: request.window_config, interaction: Some(request.interaction_state), hover: Some(request.interaction_hover), context: Some(request.context), instance_owner: Some(request.instance_operation_owner), output: Some(request.output_chunks), parent_document_id: Some(request.parent_document_id), authoring_seed: Some(request.authoring_seed) };
+            let job = KeyedTestJob { command: Some(request.command), completion: Some(request.completion), raw: request.input.take(), page: 0, base_count, closing: false, sources: Some(sources), rejected: None, admitted: false, retirement: None };
+
+job})}
         fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
             Some(store::mutation_apply_preparation_factory::<TestSnapshot, TestMutation>())
@@ -1804,4 +1782,11 @@
     /// edits of one immutable document root make three of them rebase refusals, and a refusal is
     /// published as its own acknowledgeable `Fault` page on exactly the wire this law measures.
+    struct NativeResultWire{token:semio_framework_actor::typed_operation_page::TypedOperationResultToken,lane:TypedOperationResultLane,target:String,wire:Vec<u8>}
+    impl NativeResultWire{
+        fn close(self,grant:semio_framework_value::RetainedCloneGrant)->semio_framework_actor::typed_operation_page::TypedOperationResultToken{let token=self.token;let mut owner=semio_framework_value::retirement::controlled::ControlledRetirement::new((self.target,self.wire)).unwrap_or_else(|_|panic!("original renderer destination supports controlled retirement"));for _ in 0..100_000{if owner.terminal_is_empty(){return token;}let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(grant).expect("original renderer destination paid closure"));let p=step.progress();assert!(p.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));assert_ne!(p,Default::default(),"original renderer destination closure stalled");}panic!("original renderer destination did not close")}
+        fn bytes(&self)->&[u8]{&self.wire[semio_framework_actor::typed_operation_page::TYPED_OPERATION_RESULT_HEADER_BYTES..]}
+        fn decode(target:String,wire:Vec<u8>)->Self{use semio_framework_actor::typed_operation_page::TypedOperationResultPage as Page;let body=wire.strip_prefix(Page::RENDERER_PAGE_MAGIC).expect("actual original renderer page magic");assert!(body.len()>=30);let token=semio_framework_actor::typed_operation_page::TypedOperationResultToken{receiver:u32::from_le_bytes(body[..4].try_into().unwrap()),operation:u64::from_le_bytes(body[4..12].try_into().unwrap()),generation:u64::from_le_bytes(body[12..20].try_into().unwrap()),sequence:u32::from_le_bytes(body[20..24].try_into().unwrap()),attempt:body[24]};let lane=match body[25]{0=>TypedOperationResultLane::Artifact,1=>TypedOperationResultLane::Config,2=>TypedOperationResultLane::Draft,3=>TypedOperationResultLane::Presence,4=>TypedOperationResultLane::Transient,5=>TypedOperationResultLane::Child,6=>TypedOperationResultLane::Effect,7=>TypedOperationResultLane::Event,8=>TypedOperationResultLane::Ui,9=>TypedOperationResultLane::Download,10=>TypedOperationResultLane::Terminal,11=>TypedOperationResultLane::Fault,12=>TypedOperationResultLane::Interaction,13=>TypedOperationResultLane::WindowTransient,14=>TypedOperationResultLane::WindowConfig,15=>TypedOperationResultLane::Diagnostic,_=>panic!("original renderer lane is outside its closed ABI")};assert_eq!(body.len(),30+u32::from_le_bytes(body[26..30].try_into().unwrap())as usize);Self{token,lane,target,wire}}
+    }
+    fn receive_native_result_pages<PA:crate::plugin_app::PluginApp>(runtime:&super::PluginRuntime<PA>,cx:&mut semio_framework_job::StepContext<'_>)->(Vec<NativeResultWire>,bool){let mut effects=Vec::new();let pending=super::plugin_drive_result_pages(runtime,&mut effects,super::TYPED_OPERATION_PAGES_PER_TURN_MAXIMUM,cx).expect("actual original native renderer receiving");let pages=effects.into_iter().map(|effect|match effect{Effect::SendMessage{target:semio_framework::kernel::MessageEndpoint::Shell{instance},payload}=>NativeResultWire::decode(instance.0,payload),_=>panic!("original native result publisher emitted an unrelated effect")}).collect();(pages,pending)}
     #[semio_framework_async_macros::async_test]
     async fn concurrent_typed_operations_hand_every_presentable_page_to_one_turn() {
@@ -1823,5 +1808,5 @@
         app.bind_instance_id(id).await;
         let meta = ActionMeta { actor: "fixture".into(), instance_id: id, view_state: None };
-        for index in 0..OPERATIONS {
+        for index in 0..operations {
             let command = TestCommand::CompositeEdit { slot: String::new(), child_id: format!("batch-{index}").into(), child_value: index as i32 + 1 };
             app.dispatch_typed(command, &meta, &mut identity,cx).await.unwrap();
@@ -1837,21 +1822,23 @@
             if let Some((receiver, output)) = output {
                 assert_eq!(receiver, id);
-                if !output.typed_operation_results.is_empty() {
+            }
+            {
+                if !native_pages.is_empty() {
                     crossings += 1;
-                    widest = widest.max(output.typed_operation_results.len());
+                    widest = widest.max(native_pages.len());
                 }
-                for page in output.typed_operation_results {
+                for page in native_pages {
                     pages += 1;
-                    super::plugin_acknowledge_typed_operation_result(&runtime, page.token).await.unwrap();
+                    super::plugin_acknowledge_typed_operation_result(&runtime, page.close(issued.unwrap_or(mounted_policy.close))).await.unwrap();
                 }
             }
-            if !more {
+            if !more && (issued.is_none()||holding>=256) {
                 break;
             }
             std::thread::yield_now();
         }
-        assert!(pages >= OPERATIONS, "each of the {OPERATIONS} admitted operations owes at least one result page, saw {pages}");
-        assert!(widest >= 2, "{OPERATIONS} concurrently publishing operations must share a crossing; the widest batch was {widest}");
-        assert!(crossings < pages, "{pages} result pages retired in {crossings} crossings — a page per crossing is the cardinality this law removes");
+        assert!(pages >= operations, "each of the {operations} admitted operations owes at least one result page, saw {pages}");
+        if issued.is_some(){assert!(admitted>=8,"separately issued original authority must retain all8 fixed slots before output");assert!(widest>=2,"prepared original slots share the separately issued original crossing; widest={widest}");}else{assert_eq!(widest,1,"the unchanged original item1 fixture funds one output action per crossing");}
+        if issued.is_some(){assert!(crossings<pages,"{pages} actual renderer pages span{crossings} original batch crossings");}else{assert_eq!(crossings,pages);}
         let active = cell.instance.lock().unwrap();
         assert!(!active.app.has_pending_typed_operations());
@@ -1907,15 +1894,17 @@
             let (output, scan) = super::plugin_continue_typed_operations(&runtime, super::TypedOperationGrant::UNIT, &mut crate::app_testing::artifact_app_laws::fixture_identity(), cx).await.unwrap();
             units += 1;
-            if let Some((receiver, output)) = output {
+            if let Some((receiver, _output)) = output {
                 assert_eq!(receiver, id);
-                for page in output.typed_operation_results {
+            }
+            {
+                for page in native_pages {
                     if page.lane == TypedOperationResultLane::Fault {
                         faults.push(String::from_utf8_lossy(page.bytes()).into_owned());
                     }
                     terminal |= page.lane == TypedOperationResultLane::Terminal;
-                    super::plugin_acknowledge_typed_operation_result(&runtime, page.token).await.unwrap();
+                    super::plugin_acknowledge_typed_operation_result(&runtime, page.close(mounted_policy.close)).await.unwrap();
                 }
             }
-            if !(scan.runnable || scan.contended) {
+            if !(scan.runnable || scan.contended || pages_pending) {
                 break;
             }
@@ -2034,11 +2023,13 @@
             spent += u64::from(scan.runnable || !scan.contended);
             turns.push((scan.runnable, scan.contended, output.is_some()));
-            if let Some((receiver, output)) = output {
+            if let Some((receiver, _output)) = output {
                 assert_eq!(receiver, id);
-                for page in output.typed_operation_results {
+            }
+            {
+                for page in native_pages {
                     assert_ne!(page.lane, TypedOperationResultLane::Fault, "{}", String::from_utf8_lossy(page.bytes()));
                     terminal |= page.lane == TypedOperationResultLane::Terminal;
                     receipts += 1;
-                    super::plugin_acknowledge_typed_operation_result(&runtime, page.token).await.unwrap();
+                    super::plugin_acknowledge_typed_operation_result(&runtime, page.close(mounted_policy.close)).await.unwrap();
                 }
             }
@@ -2144,10 +2135,12 @@
             if let Some((receiver, output)) = output {
                 assert_eq!(receiver, id);
-                for page in output.typed_operation_results {
+            }
+            {
+                for page in native_pages {
                     assert_ne!(page.lane, TypedOperationResultLane::Fault, "{}", String::from_utf8_lossy(page.bytes()));
                     lanes.push(page.lane);
                     if page.lane == TypedOperationResultLane::Child {
                         assert_eq!(page.token.attempt, acknowledgement_fixture["resultAck"]["attempt"].as_u64().unwrap() as u8);
-                        let delayed_token = page.token;
+                        let delayed_token = page.close(mounted_policy.close);
                         {
                             let active = cell.instance.lock().expect("delayed ACK retained app");
@@ -2191,5 +2184,5 @@
                         more = true;
                     } else {
-                        super::plugin_acknowledge_typed_operation_result(&runtime, page.token).await.expect("ack exact retained result");
+                        super::plugin_acknowledge_typed_operation_result(&runtime, page.close(mounted_policy.close)).await.expect("ack exact retained result");
                         more = true;
                     }
@@ -2217,5 +2210,5 @@
             let skipped: Vec<String> = undone.diagnostics.iter().map(|diagnostic| diagnostic.message.clone()).collect();
             assert!(skipped.is_empty(), "the group undo skips no member of its own gesture: {skipped:?}");
-            assert_eq!(active.app.snapshot().expect("undone parent snapshot").count, 0, "the parent's edit of the gesture is undone with its group (parent tail group {:?})", store::SpaceMember::tail_group_id(&active.app.store).await);
+            assert_eq!(active.app.snapshot().expect("undone parent snapshot").count, 0, "the parent's edit of the gesture is undone with its group (parent tail group {:?})", store::SpaceMember::tail_group_id(&*active.app.store).await);
             let TestMembers::Child(child) = &mut active.app.children.get_mut(&("slot".to_string(), "child-1".to_string())).expect("undone child").member;
             assert_eq!(child.snapshot().expect("undone child snapshot").count, 0);
@@ -2240,5 +2233,5 @@
         assert!(runtime.close_quarantine.borrow().get(id).is_none());
         assert!(retired.upgrade().is_none());
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -2837,5 +2830,5 @@
         assert_eq!(platform.action_bus.dispatch_count(), before);
         artifact_app_laws::close_registered_fixture_app(&mut app, mounted_policy,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -2909,5 +2902,5 @@
         let mut app = app;
         artifact_app_laws::close_registered_fixture_app(&mut app, mounted_policy,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -2968,4 +2961,16 @@
     fn retained_latest_wins_raw_capacity_is_not_initialized_byte_retirement() {
         crate::retained_command::test_raw_allocation_close::<TestApp>();
+    }
+
+    #[test]
+    fn retained_rejected_completion_closes_original_artifact_and_ephemeral_owners(){
+        crate::retained_command::test_rejected_completion_close::<TestApp>(|row|{
+            let mut emit=Emit::default();
+            let mut ephemeral=EphemeralEmit::default();
+            for revision in 0..row["artifactMutations"].as_u64().unwrap(){emit.artifact_mutations.push(TestMutation::SetCount(SetCount{value:revision as _}));}
+            for revision in 0..row["presenceMutations"].as_u64().unwrap(){ephemeral.presence.push(ChangePublicationPresence{revision}.into());}
+            for revision in 0..row["transientMutations"].as_u64().unwrap(){ephemeral.transient.push(ChangePublicationTransient{revision}.into());}
+            (emit,ephemeral)
+        });
     }
 
@@ -3135,5 +3140,5 @@
         assert_eq!(app.take_segmented_download_chunk(91).await.expect_err("terminal none removes authority").code.0, "interactive-job.unknown-segmented-download");
         artifact_app_laws::close_registered_fixture_app(&mut *app, mounted_policy,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -3184,5 +3189,5 @@
         assert!(!app.media.test_segmented_closure_contains(operation_id.0));
         artifact_app_laws::close_registered_fixture_app(&mut *app, mounted_policy,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -3281,5 +3286,5 @@
         assert_eq!(app.ladder_ctx().drive_envelope_field_decoder_returns(semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES,maximum_capacity_bytes:0,maximum_release_bytes:store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES,maximum_depth:64},true).expect("close hierarchy observes exact terminal state"), PluginLifecycleStep::Complete(Default::default()));
         artifact_app_laws::close_registered_fixture_app(&mut app, mounted_policy,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -3390,5 +3395,5 @@
         assert_eq!(app.ladder_ctx().drive_envelope_completed_record_returns(semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES,maximum_capacity_bytes:0,maximum_release_bytes:store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES,maximum_depth:64},true).expect("close observes exact completed terminal state"), PluginLifecycleStep::Complete(Default::default()));
         artifact_app_laws::close_registered_fixture_app(&mut app, mounted_policy,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -3441,6 +3446,6 @@
         assert_eq!(app_close_turn(&mut *app,cx).expect("terminal close slice"), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(store::RetainedCloneProgress { copied_items: 1, ..Default::default() }));
         assert!(!app.media.segmented_downloads.contains(37));
-    
-        
+
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
     }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
@@ -3472,5 +3477,5 @@
         assert!(matches!(app.ladder_ctx().retained_fields_close_step(crate::app_testing::artifact_app_laws::fixture_mounted_policy().close).expect("repeated terminal turn"), store::RetainedCloneStep::Complete(progress) if progress == store::RetainedCloneProgress::default()));
         artifact_app_laws::close_registered_fixture_app(&mut app, crate::app_testing::artifact_app_laws::fixture_mounted_policy(),cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -3529,9 +3534,10 @@
         let make=||{let length=law["payloadBytes"].as_u64().unwrap()as usize;assert!(length>law["wireBytes"].as_u64().unwrap()as usize);let chunks=ArtifactOutputChunks::new(length);for index in 0..length.div_ceil(ArtifactOutputChunks::CHUNK_BYTES){let start=index*ArtifactOutputChunks::CHUNK_BYTES;let end=(start+ArtifactOutputChunks::CHUNK_BYTES).min(length);chunks.push((start..end).map(|index|(index%251)as u8).collect()).unwrap();}chunks.seal().unwrap();ArtifactDownloadOutput::new("original.txt","text/plain",None,chunks).unwrap()};
         for (busy,cancel) in [(false,false),(true,false),(false,true)]{
-            let completion=ArtifactToolCompletion::<TestApp>::new();let consumer=completion.clone();let mut payload=test_retained_command_payload(completion.clone()).await;payload.work=Box::new(DownloadWork{output:Some(make()),first:true,closing:false,retirement:None});let mut job=ArtifactRetainedCommandJob::new(payload);let mut yielded=false;let policy=crate::app_testing::artifact_app_laws::fixture_mounted_policy();let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(41),semio_framework_job::Generation(3),policy.preparation,||Some(0),u64::MAX,1);
+            let completion=ArtifactToolCompletion::<TestApp>::new();let consumer=completion.clone();let (mut payload,issuer)=test_retained_command_payload(completion.clone()).await;payload.work=Box::new(DownloadWork{output:Some(make()),first:true,closing:false,retirement:None});let mut job=ArtifactRetainedCommandJob::new(payload);let mut yielded=false;let policy=crate::app_testing::artifact_app_laws::fixture_mounted_policy();let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(41),semio_framework_job::Generation(3),crate::app_testing::mutation_fixture::job_close::original_fixture_stock(),||Some(0),u64::MAX,1<<24);
+            let mut decoder=semio_framework_job::OriginalJobDecodeOwner::empty();assert!(caller.turn_with_decoder(policy.preparation,&mut decoder,|cx|job.test_admit_original_source(cx)).unwrap());
             for _ in 0..128{let result=caller.turn(policy.maintenance,|cx|Ok(if busy{completion.with_busy_test_lock(||crate::app_testing::artifact_app_laws::fixture_step(&mut job,cx))}else{crate::app_testing::artifact_app_laws::fixture_step(&mut job,cx)})).unwrap();match result{crate::app_testing::artifact_app_laws::FixtureStepOutcome::Preview(_)=>{yielded=true;if cancel{caller.cancel().cancel_now()}},crate::app_testing::artifact_app_laws::FixtureStepOutcome::Checkpoint{..}=>{},crate::app_testing::artifact_app_laws::FixtureStepOutcome::Complete{..}=>break,crate::app_testing::artifact_app_laws::FixtureStepOutcome::Fault(_)=>break,crate::app_testing::artifact_app_laws::FixtureStepOutcome::Cancelled=>{assert!(cancel);break},_=>{}}}
             assert!(yielded);job.begin_close();for _ in 0..100000{if job.terminal_is_empty(){break}let grant=job_close_grant(&job);let before=caller.turn(semio_framework_value::RetainedCloneGrant{maximum_items:0,..grant},|cx|job.close_step(cx)).unwrap();assert!(matches!(before,InteractiveJobCloseStep::Pending{progress}if progress==Default::default())||matches!(before,InteractiveJobCloseStep::Blocked));let step=caller.turn(grant,|cx|job.close_step(cx)).unwrap();assert_eq!(step.progress(),caller.receipt());assert!(step.progress().fits(grant));assert!(!matches!(step,InteractiveJobCloseStep::Blocked))}assert!(job.terminal_is_empty());
-            if busy||cancel{assert!(consumer.test_take_download().unwrap().is_none())}else{let (Ok(output),_)=consumer.test_take_download().unwrap().unwrap()else{panic!("original download completion")};let mut actual=Vec::new();while let Some(chunk)=output.chunks.take_chunk().unwrap(){assert!(chunk.len()<=law["chunkBytes"].as_u64().unwrap()as usize);actual.extend(chunk)}let expected:Vec<u8>=serde_json::from_value(serde_json::json!((0..law["payloadBytes"].as_u64().unwrap()).map(|index|(index%251)as u8).collect::<Vec<_>>())).unwrap();assert_eq!(actual,expected);}
-            caller.retire(policy.close);
+            if busy||cancel{assert!(consumer.test_take_download().unwrap().is_none())}else{let (Ok(output),ephemeral)=consumer.test_take_download().unwrap().unwrap()else{panic!("original download completion")};let mut actual=Vec::new();while let Some(chunk)=output.chunks.take_chunk().unwrap(){assert!(chunk.len()<=law["chunkBytes"].as_u64().unwrap()as usize);actual.extend_from_slice(&chunk);crate::app_testing::mutation_fixture::job_close::retire_fixture_owned(chunk,&mut caller)}let expected:Vec<u8>=serde_json::from_value(serde_json::json!((0..law["payloadBytes"].as_u64().unwrap()).map(|index|(index%251)as u8).collect::<Vec<_>>())).unwrap();assert_eq!(actual,expected);crate::app_testing::mutation_fixture::job_close::retire_fixture_owned(output,&mut caller);crate::app_testing::mutation_fixture::job_close::retire_fixture_owned(ephemeral,&mut caller);}
+            crate::app_testing::mutation_fixture::job_close::retire_fixture_owned(consumer,&mut caller);crate::app_testing::mutation_fixture::job_close::retire_fixture_owned(completion,&mut caller);issuer.close(&mut caller);while !decoder.terminal_is_empty(){caller.turn(policy.close,|cx|{let step=decoder.close_step(cx.retained_grant())?;cx.consume_retained(step.progress())?;cx.consume_fuel(step.progress().copied_items as u64);Ok(())}).unwrap();}caller.retire(crate::app_testing::mutation_fixture::job_close::original_fixture_stock());
             eprintln!("[DEBUG] original Work download busy={busy} canceled={cancel} yields; source chunks preserved; exact quoted retirement");
         }
@@ -3559,17 +3565,14 @@
         let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(41), semio_framework_job::RevisionId(2), semio_framework_job::Generation(3), 5);
         let policy=crate::app_testing::artifact_app_laws::fixture_mounted_policy();
-        let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(operation.operation,operation.generation,policy.preparation,||Some(0),u64::MAX,1);
+        let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(operation.operation,operation.generation,crate::app_testing::mutation_fixture::job_close::original_fixture_stock(),||Some(0),u64::MAX,1<<24);
+        let mut decoder=semio_framework_job::OriginalJobDecodeOwner::empty();
         let original_completion = ArtifactToolCompletion::<TestApp>::new();
         let original_consumer = original_completion.clone();
-        let original_payload = test_retained_command_payload(original_completion).await;
-        let original_spec = ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, original_payload, operation);
+        let (original_payload,original_issuer) = test_retained_command_payload(original_completion).await;
         let (original_admission, original_input) = test_retained_wire_input(&bus, &wire);
-        let mut original = match bus.dispatch_wire_retained_with_spec(&original_admission, original_input, None, original_spec) {
-            Ok(dispatch) => dispatch,
-            Err(_) => panic!("test retained command initial dispatch"),
-        };
+        let mut original=crate::app_testing::mutation_fixture::job_close::admit_registered_fixture(&bus,&mut caller,&mut decoder,operation,action_bus::ToolFactoryKey::new(TEST_RETAINED_COMMAND_CONTROLLER,TEST_RETAINED_COMMAND_TOOL),TEST_RETAINED_COMMAND_SCHEMA.into(),original_payload,Some(original_input),None);
         let mut checkpoint_bytes = None;
         for _ in 0..64 {
-            match caller.turn(policy.maintenance,|cx|Ok(crate::app_testing::artifact_app_laws::fixture_step(&mut original.job,cx))).unwrap() {
+            match caller.turn(policy.maintenance,|cx|Ok(crate::app_testing::artifact_app_laws::fixture_step(&mut original,cx))).unwrap() {
                 crate::app_testing::artifact_app_laws::FixtureStepOutcome::Preview(_) => {},
                 crate::app_testing::artifact_app_laws::FixtureStepOutcome::Checkpoint { state: bytes, .. } => {
@@ -3590,23 +3593,20 @@
         }
         let checkpoint_bytes = checkpoint_bytes.expect("work checkpoint after one custom cursor step");
-        original.job.begin_close();
-        drive_job_close(&mut original.job,&mut caller);
-        assert!(original.job.terminal_is_empty(), "interrupted original retires every retained owner incrementally");
+        original.begin_close();
+        drive_job_close(&mut original,&mut caller);
+        assert!(original.terminal_is_empty(), "interrupted original retires every retained owner incrementally");
         assert!(original_consumer.take_emit().expect("original completion cell").is_none(), "interrupted original never publishes a result");
 
+        original_issuer.close(&mut caller);crate::app_testing::mutation_fixture::job_close::retire_fixture_owned(original_consumer,&mut caller);
         TEST_RETAINED_COMMAND_STEP_CALLS.store(0, std::sync::atomic::Ordering::SeqCst);
         let resumed_completion = ArtifactToolCompletion::<TestApp>::new();
         let resumed_consumer = resumed_completion.clone();
-        let resumed_payload = test_retained_command_payload(resumed_completion).await;
-        let resumed_spec = ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, resumed_payload, operation);
+        let (resumed_payload,resumed_issuer) = test_retained_command_payload(resumed_completion).await;
         let (resumed_admission, resumed_input) = test_retained_wire_input(&bus, &wire);
         let (_, resumed_checkpoint) = test_retained_wire_input(&bus, &checkpoint_bytes);
-        let mut resumed = match bus.dispatch_wire_retained_with_spec(&resumed_admission, resumed_input, Some(resumed_checkpoint), resumed_spec) {
-            Ok(dispatch) => dispatch,
-            Err(_) => panic!("test retained command resume dispatch"),
-        };
+        let mut resumed=crate::app_testing::mutation_fixture::job_close::admit_registered_fixture(&bus,&mut caller,&mut decoder,operation,action_bus::ToolFactoryKey::new(TEST_RETAINED_COMMAND_CONTROLLER,TEST_RETAINED_COMMAND_TOOL),TEST_RETAINED_COMMAND_SCHEMA.into(),resumed_payload,Some(resumed_input),Some(resumed_checkpoint));
         let mut completed = false;
         for _ in 0..96 {
-            match caller.turn(policy.maintenance,|cx|Ok(crate::app_testing::artifact_app_laws::fixture_step(&mut resumed.job,cx))).unwrap() {
+            match caller.turn(policy.maintenance,|cx|Ok(crate::app_testing::artifact_app_laws::fixture_step(&mut resumed,cx))).unwrap() {
                 crate::app_testing::artifact_app_laws::FixtureStepOutcome::Preview(_) => {},
                 crate::app_testing::artifact_app_laws::FixtureStepOutcome::Checkpoint { .. } => {},
@@ -3625,25 +3625,22 @@
         assert_eq!(TEST_RETAINED_COMMAND_STEP_CALLS.load(std::sync::atomic::Ordering::SeqCst), 2, "resume continues after cursor one instead of replaying completed custom work");
         let (result, _) = resumed_consumer.take_emit().expect("resumed completion cell").expect("resumed completion value");
-        assert_eq!(result.expect("resumed command emit").artifact_mutations, vec![TestMutation::SetLabel(SetLabel { value: "resumed".into() })]);
-        resumed.job.begin_close();
-        drive_job_close(&mut resumed.job,&mut caller);
-        assert!(resumed.job.terminal_is_empty(), "completed resume retires every retained owner incrementally");
-
+        let result=result.expect("resumed command emit");assert_eq!(result.artifact_mutations,vec![TestMutation::SetLabel(SetLabel{value:"resumed".into()})]);crate::app_testing::mutation_fixture::job_close::retire_fixture_owned(result,&mut caller);
+        resumed.begin_close();
+        drive_job_close(&mut resumed,&mut caller);
+        assert!(resumed.terminal_is_empty(), "completed resume retires every retained owner incrementally");
+
+        resumed_issuer.close(&mut caller);crate::app_testing::mutation_fixture::job_close::retire_fixture_owned(resumed_consumer,&mut caller);
         let cancelled_completion = ArtifactToolCompletion::<TestApp>::new();
         let cancelled_consumer = cancelled_completion.clone();
-        let cancelled_payload = test_retained_command_payload(cancelled_completion).await;
-        let cancelled_spec = ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, cancelled_payload, operation);
+        let (cancelled_payload,cancelled_issuer) = test_retained_command_payload(cancelled_completion).await;
         let (cancelled_admission, cancelled_input) = test_retained_wire_input(&bus, &wire);
-        let mut cancelled = match bus.dispatch_wire_retained_with_spec(&cancelled_admission, cancelled_input, None, cancelled_spec) {
-            Ok(dispatch) => dispatch,
-            Err(_) => panic!("test retained command cancellation dispatch"),
-        };
+        let mut cancelled=crate::app_testing::mutation_fixture::job_close::admit_registered_fixture(&bus,&mut caller,&mut decoder,operation,action_bus::ToolFactoryKey::new(TEST_RETAINED_COMMAND_CONTROLLER,TEST_RETAINED_COMMAND_TOOL),TEST_RETAINED_COMMAND_SCHEMA.into(),cancelled_payload,Some(cancelled_input),None);
         caller.cancel().cancel_now();
-        assert!(matches!(caller.turn(policy.maintenance,|cx|Ok(crate::app_testing::artifact_app_laws::fixture_step(&mut cancelled.job,cx))).unwrap(),crate::app_testing::artifact_app_laws::FixtureStepOutcome::Cancelled));
-        cancelled.job.begin_close();
-        drive_job_close(&mut cancelled.job,&mut caller);
-        assert!(cancelled.job.terminal_is_empty(), "cancelled retained command retires every owner incrementally");
+        assert!(matches!(caller.turn(policy.maintenance,|cx|Ok(crate::app_testing::artifact_app_laws::fixture_step(&mut cancelled,cx))).unwrap(),crate::app_testing::artifact_app_laws::FixtureStepOutcome::Cancelled));
+        cancelled.begin_close();
+        drive_job_close(&mut cancelled,&mut caller);
+        assert!(cancelled.terminal_is_empty(), "cancelled retained command retires every owner incrementally");
         assert!(cancelled_consumer.take_emit().expect("cancelled completion cell").is_none(), "cancelled retained command never publishes a result");
-        caller.retire(policy.close);
+        cancelled_issuer.close(&mut caller);crate::app_testing::mutation_fixture::job_close::retire_fixture_owned(cancelled_consumer,&mut caller);crate::app_testing::mutation_fixture::job_close::close_fixture_decoder(&mut caller,&mut decoder);caller.retire(crate::app_testing::mutation_fixture::job_close::original_fixture_stock());
     }
 
@@ -3669,5 +3666,6 @@
         let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(42), semio_framework_job::RevisionId(2), semio_framework_job::Generation(3), 5);
         let policy=crate::app_testing::artifact_app_laws::fixture_mounted_policy();
-        let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(operation.operation,operation.generation,policy.preparation,||Some(0),u64::MAX,1);
+        let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(operation.operation,operation.generation,crate::app_testing::mutation_fixture::job_close::original_fixture_stock(),||Some(0),u64::MAX,1<<24);
+        let mut decoder=semio_framework_job::OriginalJobDecodeOwner::empty();
         let drive_to_publish = |job:&mut crate::retained_command::ArtifactRetainedCommandJob<TestApp>,caller:&mut semio_framework_job::OriginalJobFixtureCaller| {
             for expected_stage in ["preflight", "work"] {
@@ -3726,14 +3724,18 @@
         let direct_completion = ArtifactToolCompletion::<TestApp>::new();
         let _direct_consumer = direct_completion.clone();
-        let mut direct = crate::retained_command::ArtifactRetainedCommandJob::new(test_retained_child_command_payload(direct_completion, build_emit()).await);
+        let (direct_payload,direct_issuer)=test_retained_child_command_payload(direct_completion,build_emit()).await;
+        let mut direct=crate::retained_command::ArtifactRetainedCommandJob::new(direct_payload);
+        assert!(caller.turn_with_decoder(policy.preparation,&mut decoder,|cx|direct.test_admit_original_source(cx)).unwrap());
         drive_to_publish(&mut direct,&mut caller);
-        close_and_observe(&mut direct,&mut caller);
+        close_and_observe(&mut direct,&mut caller);direct_issuer.close(&mut caller);crate::app_testing::mutation_fixture::job_close::retire_fixture_owned(_direct_consumer,&mut caller);
 
         let duplicate_completion = ArtifactToolCompletion::<TestApp>::new();
         let duplicate_consumer = duplicate_completion.clone();
         duplicate_completion.complete(Ok(Emit::default()), EphemeralEmit::default()).expect("prefill completion once");
-        let mut duplicate = crate::retained_command::ArtifactRetainedCommandJob::new(test_retained_child_command_payload(duplicate_completion, build_emit()).await);
+        let (duplicate_payload,duplicate_issuer)=test_retained_child_command_payload(duplicate_completion,build_emit()).await;
+        let mut duplicate=crate::retained_command::ArtifactRetainedCommandJob::new(duplicate_payload);
+        assert!(caller.turn_with_decoder(policy.preparation,&mut decoder,|cx|duplicate.test_admit_original_source(cx)).unwrap());
         drive_to_publish(&mut duplicate,&mut caller);
-        
+
         match caller.turn(policy.maintenance,|cx|Ok(crate::app_testing::artifact_app_laws::fixture_step(&mut duplicate,cx))).unwrap() {
             crate::app_testing::artifact_app_laws::FixtureStepOutcome::Fault(_) => {},
@@ -3741,14 +3743,16 @@
         }
         assert!(duplicate.test_pending_emit_shape().is_some(), "duplicate rejection returns the exact emit owner");
-        close_and_observe(&mut duplicate,&mut caller);
+        close_and_observe(&mut duplicate,&mut caller);duplicate_issuer.close(&mut caller);
         let (existing, _) = duplicate_consumer.take_emit().expect("prefilled completion remains readable").expect("prefilled completion remains assigned");
-        assert!(existing.expect("prefilled completion remains successful").child_emits.is_empty());
+        let existing=existing.expect("prefilled completion remains successful");assert!(existing.child_emits.is_empty());crate::app_testing::mutation_fixture::job_close::retire_fixture_owned(existing,&mut caller);crate::app_testing::mutation_fixture::job_close::retire_fixture_owned(duplicate_consumer,&mut caller);
 
         let busy_completion = ArtifactToolCompletion::<TestApp>::new();
         let busy_consumer = busy_completion.clone();
-        let mut busy = crate::retained_command::ArtifactRetainedCommandJob::new(test_retained_child_command_payload(busy_completion.clone(), build_emit()).await);
+        let (busy_payload,busy_issuer)=test_retained_child_command_payload(busy_completion.clone(),build_emit()).await;
+        let mut busy=crate::retained_command::ArtifactRetainedCommandJob::new(busy_payload);
+        assert!(caller.turn_with_decoder(policy.preparation,&mut decoder,|cx|busy.test_admit_original_source(cx)).unwrap());
         drive_to_publish(&mut busy,&mut caller);
         busy_completion.with_busy_test_lock(|| {
-            
+
             match caller.turn(policy.maintenance,|cx|Ok(crate::app_testing::artifact_app_laws::fixture_step(&mut busy,cx))).unwrap() {
                 crate::app_testing::artifact_app_laws::FixtureStepOutcome::Fault(_) => {},
@@ -3757,7 +3761,7 @@
         });
         assert!(busy.test_pending_emit_shape().is_some(), "busy rejection returns the exact emit owner");
-        close_and_observe(&mut busy,&mut caller);
+        close_and_observe(&mut busy,&mut caller);busy_issuer.close(&mut caller);
         assert!(busy_consumer.take_emit().expect("busy completion remains readable").is_none(), "busy completion remains untouched");
-        caller.retire(policy.close);
+        crate::app_testing::mutation_fixture::job_close::retire_fixture_owned(busy_consumer,&mut caller);crate::app_testing::mutation_fixture::job_close::retire_fixture_owned(busy_completion,&mut caller);crate::app_testing::mutation_fixture::job_close::close_fixture_decoder(&mut caller,&mut decoder);caller.retire(crate::app_testing::mutation_fixture::job_close::original_fixture_stock());
     }
 
@@ -3781,5 +3785,5 @@
         assert_eq!(app.stores.test_snapshot().await.count, 0, "the over-budget reducer never started");
         artifact_app_laws::close_registered_fixture_app(&mut app, mounted_policy,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -3895,20 +3899,20 @@
     // The laws below pin both halves: the reservation is the command's DECLARED page count, and every
     // refusal on this path is a `Fault` the host can display rather than a panic.
-    
+
     /// 🔁️ Commands driven through the real ingress — over the 184 the browser reached before the
     /// prologue started refusing, so a per-command owner the authority never gives back shows up here.
     const INGRESS_COMMANDS: u64 = 320;
-    
+
     /// 🧮️ Retained bytes one command may leave behind. The authority is transient by construction: its
     /// pages are released as the command is assembled and its deque dies with the command.
     const INGRESS_COMMAND_RETENTION_CEILING_BYTES: isize = 512;
-    
+
     /// ⏱️ Turns one command may take to reach its terminal ingress status before the law calls it stuck.
     const INGRESS_TURNS_PER_COMMAND: usize = 512;
-    
+
     fn command_page_authority_budget() -> Budget {
         Budget { retained: crate::app_testing::artifact_app_laws::fixture_retained_turn(), fuel: 64, deadline_ms: 1000, max_effects: 16, max_patch_bytes: 65536, max_frames: 16 }
     }
-    
+
     /// 📤️ Encodes one `AppCommand` into the exact page and cursor a host hands the guest — the bytes and
     /// the header come from `encode_app_command` and `CommandBatchDriver`, production code, not from a
@@ -3924,5 +3928,5 @@
         page
     }
-    
+
     /// 🚚️ Drives one command to its terminal ingress status through the real `poll_kernel`: page 0 on the
     /// first turn, then plain turns while the guest steps the owner it retained, until it answers
@@ -3942,5 +3946,5 @@
         }
     }
-    
+
     /// 🧱️ Page counts the page-set law drives. The host splits at `COMMAND_PAGE_MAXIMUM_BYTES`, so an
     /// 8-page command is the shape a `registerBrushMesh`-scale payload reaches — and 67 is the
@@ -3949,10 +3953,10 @@
     /// by a margin no constant explains.
     const INGRESS_PAGE_SETS: [usize; 6] = [1, 2, 4, 8, 67, 80];
-    
+
     /// ⏱️ Turns a page set may spend beyond one per page. The reactor wire admits exactly ONE page per
     /// turn, so `pages` is the protocol's own floor; the slack covers a single instance-authority
     /// contention retry (`plugin_exchange` hands the owner back when `try_lock` would block).
     const INGRESS_TURN_SLACK: usize = 1;
-    
+
     /// 📤️ The production host owner for one multi-page command — the `CommandBatchDriver` a shard keeps
     /// per command, which hands out its next page only once the guest's status has been observed.
@@ -3964,5 +3968,5 @@
         semio_framework::kernel::CommandBatchDriver::new(seq, batch)
     }
-    
+
     /// 🚚️ Drives a whole page set to its terminal ingress status exactly as a shard does: hand the owner's
     /// next page (the wire carries at most one per turn), observe the status it produced, repeat. Returns
@@ -3988,5 +3992,5 @@
         }
     }
-    
+
     /// ⚖️ LAW: a command's page set reaches its terminal ingress status in ONE TURN PER PAGE — the cost is
     /// the wire's own page count, never the number of internal moves the ingress owner makes.
@@ -4024,8 +4028,8 @@
         }
         reactor_native_lifecycle_finish(&runtime, lifetime, 9).await;
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original ingress fixture paid terminal retirement");result
     }
-    
+
     /// ⚖️ LAW: the prologue reserves EXACTLY the pages its command declares, and that reservation stays
     /// inside one guest-allocator growth unit.
@@ -4068,5 +4072,5 @@
         );
     }
-    
+
     /// ⚖️ LAW: a page authority refuses every over-declaration and every over-push with a `Fault` the
     /// host can display, and never panics. Exhaustion on this path reaches a user as a refused command,
@@ -4104,5 +4108,5 @@
         assert!(!wire.retryable, "a refusal the guest cannot serve is not something the host should retry");
     }
-    
+
     /// 📥️ The cross-language paged command-ingress contract — the rows the kernel's own reassembly law
     /// and the TypeScript host writer's twin read.
@@ -4111,5 +4115,5 @@
         disorder: Vec<CommandIngressDisorderRow>,
     }
-    
+
     #[derive(serde::Deserialize)]
     struct CommandIngressDisorderRow {
@@ -4120,5 +4124,5 @@
         rejected_delivery: usize,
     }
-    
+
     /// 📤️ Every page of one command with the cursor the host stamps on it — the bytes come from the real
     /// `encode_app_command`, cut at the page extent exactly as `CommandBatchDriver` cuts them, so a law
@@ -4153,5 +4157,5 @@
             .collect()
     }
-    
+
     /// ⚖️ LAW: the guest reassembles a multi-page command in ARRIVAL ORDER and refuses every page that is
     /// not the next one — a duplicate, a skip and a swap all answer `plugin.command-page-order`.
@@ -4194,5 +4198,5 @@
         }
         reactor_native_lifecycle_finish(&runtime, lifetime, 9).await;
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original ingress fixture paid terminal retirement");result
     }
@@ -4267,5 +4271,5 @@
         assert_eq!(crate::reactor::retained_command_ingress_occupancy(), 0, "the superseded owner retires on its own budget instead of pinning the authority");
         reactor_native_lifecycle_finish(&runtime, lifetime, 9).await;
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original ingress fixture paid terminal retirement");result
     }
@@ -4324,5 +4328,5 @@
         );
         reactor_native_lifecycle_finish(&runtime, lifetime, 9).await;
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original ingress fixture paid terminal retirement");result
     }
@@ -4374,5 +4378,5 @@
         assert!(bundle.create_app("unknown-app", actor, mounted_policy, &mut identity).is_none());
         artifact_app_laws::close_registered_fixture_app(&mut app, mounted_policy,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -4447,5 +4451,5 @@
         assert!(app.open_conflicts().await.is_empty());
         drain_and_close_fixture(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -4471,7 +4475,7 @@
         assert_eq!(result.inverse_group.mutations.len(), 1);
         assert_eq!(app.stores.test_snapshot().await.count, 1);
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -4526,7 +4530,7 @@
         assert_eq!(app.stores.transient_store.generation().await, 1, "undo must not rewind transient");
         assert_eq!(json!({ "presenceGeneration": app.stores.presence_store.generation().await, "transientGeneration": app.stores.transient_store.generation().await, "documentCount": app.stores.test_snapshot().await.count }), vectors["undone"]);
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -4549,7 +4553,7 @@
         assert_eq!(app.stores.presence_store.generation().await, 0);
         assert_eq!(app.stores.transient_store.generation().await, 0);
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -4588,7 +4592,7 @@
         assert_eq!(decoded.domains[0].domain, "items");
         assert_eq!(decoded.domains[0].selected, vec!["item-1".to_string()]);
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -4632,5 +4636,5 @@
                     artifact_app_laws::close_registered_fixture_app(&mut *app, mounted_policy,cx).expect("original contract fixture paid terminal close");
                 });
-            
+
                 let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original worker fixture paid terminal retirement");
             })
@@ -4727,6 +4731,6 @@
         assert!(!app.roster.peer_presence.contains_key("user:bob#s1"), "an actor absent from the roster must be dropped, not left stale");
         drain_and_close_fixture(&mut app, mounted_policy,cx);
-    
-        
+
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
     }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
@@ -4788,5 +4792,5 @@
         assert!(app.roster.presence_peer_retirements.is_empty(), "displaced app-typed peer reaches its domain-owned terminal witness");
         drain_and_close_fixture(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -4879,5 +4883,5 @@
         assert_eq!(stale.presence_store.peers_root().len(), 0, "stale roster never changes the typed peer root");
         drain_and_close_fixture(&mut stale, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -4900,5 +4904,5 @@
         let state = InteractionState::default();
         let hover = InteractionHoverState::new();
-        let view = InteractionView { state: &state, hover: &hover, peers: &peers, draft_references: &std::collections::BTreeMap::new() };
+        let view = InteractionView::new(&state, &hover, &peers, &std::collections::BTreeMap::new());
 
         let selecting = view.peers_selecting("items", "item-1");
@@ -4975,4 +4979,6 @@
 
     impl store::ErasedSnapshotRetirement for TestSnapshotRetirement {
+ fn next_demand(&self,maximum_body_bytes:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.demands(maximum_body_bytes)}
+
         fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
             use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
@@ -5151,5 +5157,5 @@
         assert!(matches!(app_maintenance_turn(&mut app,cx).unwrap(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(_)));
         close_member_admission_app(&mut app,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -5186,7 +5192,7 @@
         close_member_admission_fixture(&mut returned);
         close_member_admission_app(&mut app,cx);
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -5220,5 +5226,5 @@
         close_member_admission_app(&mut restored,cx);
         close_member_admission_app(&mut app,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -5286,7 +5292,7 @@
         assert_eq!(carrying[0].child_edit_ids.len(), 1, "the composite row names the child's own edit id exactly once: {:?}", carrying[0].child_edit_ids);
         assert!(carrying[0].edit_id.is_some(), "the row that names the child's edit also carries the parent's own edit: {:?}", carrying[0].edit_id);
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -5348,7 +5354,7 @@
         let refused = app.transaction_prepare("txn-agent-3", "", &[], &[], &stray, Some(protocol::MutationOrigin::Owner)).await;
         assert_eq!(refused.rejection.expect("a child this instance does not hold is refused").code.0, "transaction.member-rejected");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -5428,5 +5434,5 @@
         drain_and_close_composed_fixture(&mut reloaded, mounted_policy,cx);
         drain_and_close_composed_fixture(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -5460,5 +5466,5 @@
         assert_eq!(crate::inference_child_dependency(&heads[0].slot, &heads[0].child_id), "child:slot/child-1");
         drain_and_close_composed_fixture(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -5520,5 +5526,5 @@
         drain_and_close_composed_fixture(&mut consumer, mounted_policy,cx);
         drain_and_close_composed_fixture(&mut source, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -5566,5 +5572,5 @@
         drain_and_close_composed_fixture(&mut consumer, mounted_policy,cx);
         drain_and_close_composed_fixture(&mut source, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -5650,7 +5656,7 @@
         publication_caller.retire(mounted_policy.close);
 
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -5691,7 +5697,7 @@
         publication_caller.retire(mounted_policy.close);
 
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -5733,7 +5739,7 @@
         app.dispatch_action("checkoutCheckpoint", Some(&dv(serde_json::json!({ "checkpointId": pinned_checkpoint }))), &meta(), &mut identity,cx).await.expect("checkout");
         assert_eq!(reads_child_count(&app).await, 7, "checkout did not cascade to the pinned child");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -5776,5 +5782,5 @@
         drop(admitted_root);
         drain_and_close_composed_fixture(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -5807,5 +5813,5 @@
         install_test_member_owners(&mut app, "child-a");
         drain_and_close_composed_fixture(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -5890,5 +5896,5 @@
         assert!(!app.child_composition.child_content_retirements.is_empty(), "terminal witness failure retains the registry authority");
         drain_and_close_composed_fixture(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -5923,5 +5929,5 @@
         app.child_composition.publish_child_content_member(generation, "slot", "child-a").await.expect("later publication after bounded reclaim");
         drain_and_close_composed_fixture(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -5959,5 +5965,5 @@
         assert!(app.child_composition.child_content_retirements.len() < MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY, "pressure maintenance returns the child roots below their quarter-occupancy bound");
         drain_and_close_composed_fixture(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -5999,7 +6005,7 @@
         assert_eq!(MAXIMUM_CHILD_CLONES.load(std::sync::atomic::Ordering::Acquire), 0, "ChildContentView must clone only the existing Arc, never the maximum snapshot graph");
         assert_eq!(MAXIMUM_CHILD_ENCODINGS.load(std::sync::atomic::Ordering::Acquire), 0, "ChildContentView must not pack-encode maximum child content during public dispatch");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -6029,7 +6035,7 @@
         app.dispatch_action("redo", None, &meta(), &mut identity,cx).await.expect("redo");
         assert_eq!(reads_child_count(&app).await, 7, "the view must reflect the child's redone state");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -6074,7 +6080,7 @@
         assert!(result.diagnostics.iter().any(|diagnostic| diagnostic.message.contains("child-b")), "the skip diagnostic must name the actual skipped member");
 
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -6112,5 +6118,5 @@
         assert_eq!(entry.member.document_id().await, "genesis-child");
         drain_and_close_composed_fixture(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -6136,7 +6142,7 @@
         assert!(result.requested_effects.is_empty());
         assert_eq!(app.stores.test_snapshot().await, TestSnapshot::default());
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -6181,5 +6187,5 @@
         assert_eq!(app.stores.test_config().await.selected, Some("node-1".to_string()), "undo never steps over the config lane");
         close_reserved_app(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -6216,7 +6222,7 @@
         assert_eq!(after.commands.len(), 3, "the revert appends one History-kind row");
         assert_eq!(after.commands.first().map(|entry| entry.action_id.as_str()), Some(REVERT_TO_COMMAND_ACTION_ID));
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -6251,5 +6257,5 @@
         assert_eq!(app.render_ctx().test_history().await.commands.len(), history.commands.len(), "bubbling the effect logs nothing new by itself");
         artifact_app_laws::close_registered_fixture_app(&mut *app, mounted_policy,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -6272,7 +6278,7 @@
         assert!(result.mutations.is_empty());
         assert_eq!(result.requested_effects, vec![Effect::Navigate { uri: "semio://home".into() }]);
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -6299,7 +6305,7 @@
         assert_eq!(fragment.dsl_text, "hello");
         assert_eq!(fragment.source_app, TestApp::<false>::APP_ID);
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -6322,7 +6328,7 @@
         assert!(result.mutations.is_empty());
         assert!(result.requested_effects.is_empty());
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -6350,7 +6356,7 @@
         reserved_action(&mut app, "undo", None, &mut identity).await;
         assert_eq!(app.stores.test_snapshot().await.label, "hello");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -6381,7 +6387,7 @@
         reserved_action(&mut app, "paste", Some(&args), &mut identity).await;
         assert_eq!(app.stores.test_snapshot().await.label, "pasted");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -6412,7 +6418,7 @@
         reserved_action(&mut app, "paste", Some(&args), &mut identity).await;
         assert_eq!(app.stores.test_snapshot().await.label, format!("pasted-{:?}", PasteAnchor::Centroid));
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -6435,7 +6441,7 @@
         assert!(result.mutations.is_empty());
         assert_eq!(app.stores.test_snapshot().await.label, "");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -6478,7 +6484,7 @@
         reserved_action(&mut app, "undo", None, &mut identity).await;
         assert_eq!(app.stores.test_snapshot().await.label, "");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -6505,7 +6511,7 @@
         reserved_action(&mut app, "undo", None, &mut identity).await;
         assert_eq!(app.stores.test_snapshot().await.label, "ab");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -6540,7 +6546,7 @@
         assert!(checkpoint.mutations.is_empty());
         assert!(checkpoint.events.iter().any(|event| event.kind == "history-changed"));
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -6576,5 +6582,5 @@
         assert!(result.mutations.is_empty());
         artifact_app_laws::close_registered_fixture_app(&mut app.0, crate::app_testing::artifact_app_laws::fixture_mounted_policy(),cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -6630,5 +6636,5 @@
         assert!(app.stores.test_store().await.applied_edit_ids().len() < before);
         artifact_app_laws::close_registered_fixture_app(&mut app.0, crate::app_testing::artifact_app_laws::fixture_mounted_policy(),cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -6662,5 +6668,5 @@
         assert_eq!(app.stores.test_snapshot().await.count, 0, "driving the spawned job must run commit_framework_history_route");
         artifact_app_laws::close_registered_fixture_app(&mut app.0, crate::app_testing::artifact_app_laws::fixture_mounted_policy(),cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -6754,5 +6760,5 @@
         assert!(app.render_ctx().test_history().await.commands.iter().find(|entry| entry.action_id == "os.resizeWindow").is_some_and(|entry| entry.revertible));
         close_reserved_app(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -6807,5 +6813,5 @@
         assert!(settled.history_patch.is_some_and(|patch| patch.can_undo && patch.can_redo && !patch.upserts.is_empty()));
         close_reserved_app(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -6863,5 +6869,5 @@
         assert_eq!(app.stores.test_snapshot().await.count, 1, "redo must restore the document example before the shell");
         close_reserved_app(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -6924,5 +6930,5 @@
         let mut instance = cell.instance.lock().expect("export instance");
         artifact_app_laws::close_registered_fixture_app(&mut instance.app, crate::app_testing::artifact_app_laws::fixture_mounted_policy(),cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -6983,5 +6989,5 @@
         assert!(app.render_ctx().test_history().await.commands.iter().find(|entry| entry.action_id == replay_id).is_some_and(|entry| entry.revertible));
         close_reserved_app(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -7029,5 +7035,5 @@
         assert!(after.commands.iter().any(|entry| entry.action_id == "shell.windowActivate"), "the chrome row stays in the append-only log");
         close_reserved_app(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -7083,7 +7089,7 @@
         assert!(!entry.op_lines.is_empty(), "operation entry must carry printed op-text");
         assert!(entry.applied && entry.revertible);
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -7109,7 +7115,7 @@
         let stream_entries: Vec<&CommandView> = history.commands.iter().filter(|entry| entry.action_id == "streamLabel").collect();
         assert_eq!(stream_entries.len(), 1, "a streamed gesture must grow one entry's op_lines, not append new entries");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -7146,7 +7152,7 @@
         let exported = exported.upserts.iter().find(|entry| entry.action_id == "streamLabel").expect("the gesture's exported row");
         assert_eq!((exported.op_count, exported.op_lines.len()), (64, HISTORY_ROW_OPERATION_PREVIEW), "the exported row carries the bounded preview and its edit's operation count");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -7176,7 +7182,7 @@
         assert_eq!(after_redo.commands.len(), 3, "redo appends a third entry");
         assert!(after_redo.commands.iter().any(|entry| entry.action_id == "undo"));
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -7211,7 +7217,7 @@
         assert_eq!(history.commands.len(), before_len + 1, "exactly one entry appended for the revert itself");
         assert_eq!(history.commands.first().map(|entry| entry.action_id.as_str()), Some(REVERT_TO_COMMAND_ACTION_ID));
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -7250,9 +7256,9 @@
         assert!(history.commands[0].edit_id.is_some());
         assert!(!history.commands[0].op_lines.is_empty());
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut receiver.0,receiver.1,cx).expect("original contract fixture paid terminal close");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut sender.0,sender.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -7276,5 +7282,5 @@
         assert_eq!(app.render_ctx().test_history().await.command_filter, HistoryCommandFilter::OnlyMutations);
         artifact_app_laws::close_registered_fixture_app(&mut *app, mounted_policy,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -7569,7 +7575,7 @@
         assert!(app.render_ctx().test_history().await.commands.is_empty(), "a config-only View is never a row (design §20.13, L4)");
         assert_eq!(app.stores.test_config().await.selected, Some("node-1".to_string()), "while the config lane holds its edit");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -7594,7 +7600,7 @@
         assert!(app.render_ctx().test_history().await.commands.is_empty(), "three config-only Views add no row");
         assert_eq!(app.stores.test_config().await.selected, Some("node-3".to_string()));
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -7620,7 +7626,7 @@
         let history = app.render_ctx().test_history().await;
         assert_eq!(history.commands.iter().map(|entry| (entry.action_id.as_str(), entry.count)).collect::<Vec<_>>(), vec![("increment", 1)]);
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -7656,5 +7662,5 @@
         assert!(history.commands.iter().all(|entry| entry.count == 1));
         artifact_app_laws::close_registered_fixture_app(&mut *app, mounted_policy,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -7687,5 +7693,5 @@
         }
         artifact_app_laws::close_registered_fixture_app(&mut *app, mounted_policy,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -7707,7 +7713,7 @@
         let result = app.dispatch_typed(TestCommand::ViewNoScope, &meta(), &mut identity,cx).await.expect("viewNoScope");
         assert_eq!(result.ui_scope, UiDirtyScope::None);
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -7738,7 +7744,7 @@
         app.dispatch_typed(TestCommand::Increment, &under(host_event_view("main-2", None)), &mut identity,cx).await.expect("another window");
         assert_eq!(take_test_host_events(), Vec::new(), "main-2 is seen first, and this view names no utility for main-1");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -7774,7 +7780,7 @@
         let refused = app.handle_action(semio_framework::HOST_EVENT_ACTION_ID, Some(dv(json!({ "windowId": "main-1", "kind": "wobble" }))), &under, &mut identity).await;
         assert_eq!(refused.err().map(|fault| fault.code), Some(FaultCode::new("hostEvent.invalid")));
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -7801,7 +7807,7 @@
         let rows: Vec<&str> = history.commands.iter().map(|entry| entry.action_id.as_str()).collect();
         assert_eq!(rows, vec!["increment"], "pure view verbs never become rows");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -7842,7 +7848,7 @@
         assert_eq!(window_bodies, vec!["some.window".to_string()]);
         assert!(panel_bodies.is_empty());
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -7864,7 +7870,7 @@
         let result = app.dispatch_typed(TestCommand::Select { id: Some("x".into()) }, &meta(), &mut identity,cx).await.expect("select");
         assert_eq!(result.ui_scope, UiDirtyScope::Full);
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -7888,5 +7894,5 @@
         assert_eq!(app.render_ctx().test_history().await.commands.len(), before_len);
         close_reserved_app(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -7910,5 +7916,5 @@
         assert_eq!(app.render_ctx().test_history().await.commands.len(), before_len, "the filter's own chrome must not fill the list it filters");
         artifact_app_laws::close_registered_fixture_app(&mut *app, mounted_policy,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -7933,7 +7939,7 @@
         assert_eq!(rendered.root.children.len(), 2, "Actions + Commands");
         assert_eq!(rendered.root.children[1].children.len(), 1, "a log-only cache key change (no store generation bump) must still refresh the rendered panel");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -7961,7 +7967,7 @@
         assert!(entry.edit_id.is_none(), "no operations means no VCS edit, even though the action is Mutation-kind");
         assert_eq!(entry.count, 1);
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -7985,5 +7991,5 @@
         assert!(result.events.is_empty());
         close_reserved_app(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -8010,10 +8016,51 @@
         artifact_app_laws::load_document(&mut restored, &files, &mut identity).await.expect("load document pack");
         assert_eq!(restored.test_snapshot().await, TestSnapshot { count: 1, label: "hi".into(), slot: Vec::new() });
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut restored.0,restored.1,cx).expect("original contract fixture paid terminal close");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
-        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
+
+        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
+    }
+
+    /// 📥️ Every accepted remote prefix is observed even when a later wire-valid envelope refuses.
+    #[semio_framework_async_macros::async_test]
+    async fn accepted_remote_prefix_notifies_before_later_refusal() {
+        let law: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧫️fixtures/📥️remote-prefix/🔣️.json"))).unwrap();
+        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
+        let mounted_policy = crate::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
+        let mut identity = crate::app_testing::artifact_app_laws::fixture_identity();
+        let mut sender = contract_app(mounted_policy, &mut identity).await;
+        let (near, mut far) = MemoryBackbone::pair("mem://remote-prefix", "mem://remote-prefix").await;
+        sender.attach_backbone(store::os_store::backbone::Backbones::Memory(near)).await.expect("attach sender");
+        sender.dispatch_typed(TestCommand::Increment, &meta(), &mut identity).await.expect("sender edit");
+        let mut envelopes = Vec::new();
+        for message in far.receive().await.expect("receive sender edit") {
+            if let BackboneMessage::Mutations { envelopes: operations } = message {
+                envelopes.extend(protocol::decode_envelopes(&operations).expect("sender envelopes"));
+            }
+        }
+        assert_eq!(envelopes.len(), 1, "fixture owns one actual authored operation");
+        let valid = envelopes.pop().unwrap();
+        for row in law["cases"].as_array().unwrap() {
+            let mut receiver = contract_app(mounted_policy, &mut identity).await;
+            let under = ActionMeta { view_state: Some(host_event_view("main-1", Some("select"))), ..meta() };
+            receiver.dispatch_typed(TestCommand::ViewNoScope, &under, &mut identity).await.expect("admitted window context");
+            take_test_host_events();
+            let batch: Vec<_> = row["steps"].as_array().unwrap().iter().map(|step| {
+                let mut envelope = valid.clone();
+                if step == "malformed" { envelope.diff.payload = vec![0xff]; }
+                envelope
+            }).collect();
+            let result = receiver.ingest_operations(&protocol::encode_envelopes(&batch), &mut identity).await;
+            assert_eq!(result.is_err(), row["refused"].as_bool().unwrap(), "{}", row["name"]);
+            assert_eq!(receiver.test_snapshot().await.count, row["count"].as_i64().unwrap() as i32, "{}", row["name"]);
+            let events = take_test_host_events();
+            let expected = row["notifications"].as_u64().unwrap() as usize;
+            let base_moved: Vec<_> = events.into_iter().filter(|event| matches!(event, HostEvent::BaseMoved { .. })).collect();
+            assert_eq!(base_moved.len(), expected * 2, "{}", row["name"]);
+            if expected > 0 { assert_eq!(base_moved, vec![HostEvent::BaseMoved { window_id: "main-1".into() }, HostEvent::BaseMoved { window_id: "main-2".into() }]); }
+            eprintln!("[DEBUG] remote prefix case={} committed={} base_notifications={}", row["name"], row["count"], base_moved.len());
+        }
     }
 
@@ -8049,9 +8096,9 @@
         receiver.ingest_operations(&operations, &mut crate::app_testing::artifact_app_laws::fixture_identity()).await.expect("ingest twice");
         assert_eq!(receiver.test_snapshot().await.count, 1, "feeding the same operation twice must not double-apply");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut receiver.0,receiver.1,cx).expect("original contract fixture paid terminal close");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut sender.0,sender.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -8090,7 +8137,7 @@
         assert_eq!(app.stores.test_snapshot().await.count, 3);
         assert!(!far_again.receive().await.expect("receive after re-attach").is_empty(), "re-attaching resumes outbound convergence on the new backbone");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -8135,7 +8182,7 @@
         assert_eq!(with_label.len(), 2, "the guard must open once a label is set: {with_label:?}");
         assert_eq!(with_label[1], ContextMenuItemSpec { id: "incrementViaCommand".into(), label: Some("Increment".into()), icon: Some(increment_icon), action: Some("incrementViaCommand".into()), ..Default::default() });
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -8229,7 +8276,7 @@
         let actions_children: Vec<&str> = organized[6].children.as_ref().unwrap().iter().map(|child| child.id.as_str()).collect();
         assert_eq!(actions_children, vec!["flatLeaf9", "flatLeaf10"], "uncategorized overflow leaves default to menu.group.actions");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -8308,5 +8355,7 @@
         let app = contract_app_raw(mounted_policy, &mut identity).await;
         runtime.instances.borrow_mut().insert_admitted(7, std::sync::Arc::new(super::RuntimeAppCell::new(AppInstance { id: 7, app, surface_contexts: Default::default() }, runtime.mounted_policy)));
-        let response = super::plugin_refresh_ui(&runtime, 7, &serde_json::to_string(&json!({"viewState":focused,"panels":[{"key":"properties","bodyKey":"properties"}]})).unwrap()).await.unwrap();
+        let mut sequence = 0;
+        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(4_096, u64::MAX, mounted_policy.preparation), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, crate::artifact_app_laws::fixture_progress());
+        let response = super::plugin_refresh_ui(&runtime, 7, &serde_json::to_string(&json!({"viewState":focused,"panels":[{"key":"properties","bodyKey":"properties"}]})).unwrap(), &mut cx).await.unwrap();
         let response: Value = serde_json::from_str(&response).unwrap();
         assert_eq!(response["panels"][0]["key"], "properties");
@@ -8636,7 +8685,7 @@
         assert!(matches!(&error.primary,semio_framework_plugin_ownership::bounded_ownership::PluginLifecyclePrimary::Fault(fault) if fault.message.contains("must not emit operations")), "unexpected complete original error: {error:?}");
         assert_eq!(app.stores.test_snapshot().await, TestSnapshot::default());
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -8678,7 +8727,7 @@
         let history = app.render_ctx().test_history().await;
         assert!(history.commands.iter().any(|entry| entry.action_id == "incrementViaCommand"), "a command-log entry must have been recorded for the intent");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -8716,7 +8765,7 @@
         assert!(matches!(&error.primary,semio_framework_plugin_ownership::bounded_ownership::PluginLifecyclePrimary::Fault(fault) if fault.message.contains("must not emit operations")), "unexpected complete original error: {error:?}");
         assert_eq!(app.stores.test_snapshot().await, TestSnapshot::default(), "kind discipline must block the mutation");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -8760,7 +8809,7 @@
         let event = result.events.iter().find(|event| event.kind == "active-utility").expect("echoed active utility");
         assert_eq!(event.payload, semio_framework_value::DslValue::from(json!({ "utilityId": "brush" })));
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -8795,7 +8844,7 @@
         reserved_action(&mut app, "undo", None, &mut identity).await;
         assert_eq!(app.stores.test_snapshot().await.label, "x");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -8837,7 +8886,7 @@
         reserved_action(&mut app, "undo", None, &mut identity).await;
         assert_eq!(app.stores.test_snapshot().await.label, "");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -8862,7 +8911,7 @@
         assert_eq!(result.mutations[0].inverse.inverse_diff.payload, protocol::encode_ops_vec(&[::protocol::OpBinary::encode_op(&TestMutation::SetCount(SetCount { value: 0 })).unwrap()]));
         assert_eq!(app.stores.test_snapshot().await.count, 1);
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -8890,7 +8939,7 @@
         let error = app.handle_command(&mut unknown, None, Some("edit"), &meta(), &mut identity).await.expect_err("undeclared app command");
         assert!(error.message.contains("not owned by app"), "unexpected error: {}", error.message);
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -8935,5 +8984,5 @@
             artifact_app_laws::close_registered_fixture_app(&mut command_app, mounted_policy,cx).expect("original contract fixture paid terminal close");
         }
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -8963,7 +9012,7 @@
         let error = app.handle_command(&mut invocation, None, None, &meta(), &mut identity).await.expect_err("inactive mode command");
         assert!(error.message.contains("not owned by active mode edit"), "unexpected error: {}", error.message);
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -8991,7 +9040,7 @@
         settle_contract_app(&mut app,&mut identity,cx).await;
         assert_eq!(app.stores.test_snapshot().await.label, "main-instance-2");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -9048,7 +9097,7 @@
         reserved_action(&mut app, "undo", None, &mut identity).await;
         assert_eq!(app.stores.test_snapshot().await.count, 1);
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -9069,5 +9118,5 @@
         assert_eq!(app.stores.test_snapshot().await, TestSnapshot::default());
         artifact_app_laws::close_registered_fixture_app(&mut app, mounted_policy,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -9094,7 +9143,7 @@
         assert_eq!(selection.ids, vec!["item-1".to_string()]);
         assert_eq!(selection.granularity, "item");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -9126,7 +9175,7 @@
         reserved_action(&mut app, INTERACTION_HOVER_ACTION_ID, Some(&dv(json!({ "domainId": "items", "channel": "pointer", "targets": "[]" }))), &mut identity).await;
         assert!(app.interaction_state().await.hover.get("items").is_none(), "an emptied hover channel is removed, not left as an empty entry");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -9155,7 +9204,7 @@
         assert_eq!(app.stores.test_snapshot().await.label, "", "undo must revert the document edit (seeding the label)");
         assert_eq!(app.interaction_state().await.selection.get("items").map(|selection| selection.ids.clone()), Some(vec!["item-1".to_string()]), "the pick itself must survive an unrelated document undo — lane discipline");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -9187,7 +9236,7 @@
         let error = crate::artifact_instance::settle_framework_reserved_admission(&mut *app, admitted).await.expect_err("undeclared granularity must be rejected");
         assert!(error.message.contains("bogus"), "unexpected error: {}", error.message);
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -9214,7 +9263,7 @@
         reserved_action(&mut app, CLEAR_SELECTION_ACTION_ID, None, &mut identity).await;
         assert!(app.interaction_state().await.selection.get("items").is_none_or(|selection| selection.ids.is_empty()), "clearSelection must empty every declared domain's selection");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -9243,7 +9292,7 @@
 
         assert!(app.interaction_state().await.selection.get("items").is_none_or(|selection| selection.ids.is_empty()), "the deleted id must be pruned from selection automatically");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -9272,7 +9321,7 @@
         let kinds: Vec<(&str, ActionKind)> = history.commands.iter().map(|entry| (entry.action_id.as_str(), entry.kind)).collect();
         assert_eq!(kinds, vec![("setLabel", ActionKind::Mutation)], "only the seeding edit is a row");
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -9393,7 +9442,7 @@
         assert!(update.peers[0].selected);
         assert!(!update.peers[0].hovered);
-    
+
         artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -9469,5 +9518,5 @@
         assert_eq!(third, vec![("item-2".to_string(), true)], "an unchanged selection must cost exactly its own mark, never a repeated retirement");
         close_reserved_app(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -9497,5 +9546,5 @@
         assert_eq!(gumball.get("anchorId").and_then(semio_framework_value::DslValue::as_str), Some("item-1"));
         close_reserved_app(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -9525,5 +9574,5 @@
         assert!(ids.iter().any(|id| id.as_str() == Some("seed-left-001")), "leftover selected ids {ids:?}");
         close_reserved_app(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -9549,5 +9598,5 @@
         assert_eq!(hover.get("domain").and_then(semio_framework_value::DslValue::as_str), Some("items"));
         close_reserved_app(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -9580,5 +9629,5 @@
         assert!(!utilities && !tools && !engagements && !measures && !labels, "a hover cannot dirty the shell chrome");
         close_reserved_app(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -9614,5 +9663,5 @@
         );
         close_reserved_app(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -9644,5 +9693,5 @@
         );
         close_reserved_app(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -9700,5 +9749,5 @@
         assert_eq!(app.interaction_state().await.hover.get("items").map(|hover| hover.ids.as_slice()), Some(["item-1".to_string()].as_slice()));
         close_reserved_app(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -9726,5 +9775,5 @@
         assert_eq!(hover.get("id").and_then(semio_framework_value::DslValue::as_str), Some("item-1"));
         close_reserved_app(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -9774,5 +9823,5 @@
         assert!(!history.commands.iter().any(|entry| entry.action_id == INTERACTION_SELECT_ACTION_ID || entry.kind == ActionKind::Interaction), "the folded verb is never a history row");
         close_reserved_app(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -9812,5 +9861,5 @@
         assert!(*utilities, "the carrier's own section flag survives the merge");
         close_reserved_app(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -9844,5 +9893,5 @@
         assert!(snapshot.selection.values().all(|selection| selection.ids.is_empty()), "a refused verb moves nothing: {:?}", snapshot.selection);
         close_reserved_app(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -9869,5 +9918,5 @@
         assert!(result.diagnostics.is_empty());
         close_reserved_app(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -9908,5 +9957,5 @@
         assert!(!history.commands.iter().any(|entry| entry.action_id == INTERACTION_SELECT_ACTION_ID || entry.kind == ActionKind::Interaction), "the folded verb is never a history row");
         close_reserved_app(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -9937,5 +9986,5 @@
         assert!(snapshot.selection.values().all(|selection| selection.ids.is_empty()), "a non-interaction DispatchAction moves no selection: {:?}", snapshot.selection);
         close_reserved_app(&mut app, mounted_policy,cx);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -9993,10 +10042,10 @@
                 original_publication_turn(&mut app,&mut identity,&mut publication_caller,mounted_policy.maintenance).map_err(|error| format!("turn {turns}: {error:?}"))?;
                 turns += 1;
-                while let Some(page) = app.take_typed_operation_result_page(id) {
+                while let Some(mut page) = crate::app_testing::artifact_app_laws::take_fixture_result_page(&mut app,id,crate::app_testing::artifact_app_laws::fixture_mounted_policy().preparation).expect("original fixture page presentation") {
                     if page.lane == TypedOperationResultLane::Fault {
                         return Err(format!("turn {turns}: fault page {}", String::from_utf8_lossy(page.bytes())));
                     }
                     lanes.push(page.lane);
-                    if !app.acknowledge_typed_operation_result(page.token).map_err(|error| format!("ack: {error:?}"))? {
+                    if !app.acknowledge_typed_operation_result(crate::app_testing::artifact_app_laws::close_fixture_result_page(&mut page,crate::app_testing::artifact_app_laws::fixture_mounted_policy().close)).map_err(|error| format!("ack: {error:?}"))? {
                         return Err(format!("turn {turns}: exact result ACK rejected"));
                     }
@@ -10068,5 +10117,5 @@
         publication_caller.retire(mounted_policy.close);
 
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -10148,5 +10197,5 @@
             close_reserved_app(&mut app, mounted_policy,cx);
         }
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -10191,5 +10240,5 @@
             close_reserved_app(&mut app, mounted_policy,cx);
         }
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -10315,5 +10364,5 @@
         assert!(row.applied, "the follow-up is an APPLIED edit, not a staged preview");
         assert!(row.op_lines.iter().any(|line| line.contains("42")), "the applied edit carries the value the injected completion delivered: {:?}", row.op_lines);
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }
@@ -10347,4 +10396,6 @@
 
     impl<F: Send + 'static> semio_framework_value::retirement::RetirementCursor for TestTaskDrop<F> {
+ fn next_demand(&self,maximum_body_bytes:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{let _=maximum_body_bytes;Ok(semio_framework_value::RetirementDemand{depth:usize::from(self.0.is_some()),..Default::default()})}
+
         fn close_step(&mut self, grant: semio_framework_value::RetainedCloneGrant) -> semio_framework_value::retirement::RetirementStep {
             use semio_framework_value::retirement::RetirementStep;
@@ -10620,5 +10671,5 @@
         crate::child_content::test_mounted_private_child_frame(&mut app);
         crate::app_testing::artifact_app_laws::close_registered_fixture_app(&mut app, crate::app_testing::artifact_app_laws::fixture_mounted_policy(),cx).expect("original contract fixture paid terminal close");
-    
+
         }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
     }


### 38 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-coverage/🟦️.ts
B/H/S/current: 1858/62/1211/176
--- 
+++ 
@@ -1,5 +1,12 @@
 import { toolJobMemberStoreProducerExact } from "../../../🏪️store/🏗️initialization/🏠️owners/🏭️member/📥️source/🟦️.ts";
 import { memberStoreProducerSourceSelfTests } from "../../../🏪️store/🏗️initialization/🏠️owners/🏭️member/📥️source/🧪️tests/🟦️.ts";
-import { toolJobStaticRows, toolJobDispositions, toolJobProofs, toolJobAppOwnedRows, toolJobProductionProtocolSource, toolJobGlobalPayloadStoreInventory, toolJobGlobalPayloadStoreIsSharedFixedAuthority, toolJobScanThenMonolithRows, TOOL_JOB_RESERVED_IDS, toolJobPublicationContracts, toolJobPublicationAuthorityReady } from "../../../../../🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";
+import fundedBatchFixture from "./🧫️fixtures/🧾️funded-batch.json";
+import { spawnSync } from "node:child_process";
+import { readFileSync } from "node:fs";
+import { join } from "node:path";
+import { createRequire } from "node:module";
+import persistentFoundationFixture from "./🧫️fixtures/🧾️persistent-foundation.json";
+import contractExpressionFixture from "./🧫️fixtures/🧾️contract-expression.json";
+import { toolJobContractSource, toolJobStaticRows, toolJobDispositions, toolJobProofs, toolJobAppOwnedRows, toolJobProductionProtocolSource, toolJobGlobalPayloadStoreInventory, toolJobGlobalPayloadStoreIsSharedFixedAuthority, toolJobScanThenMonolithRows, TOOL_JOB_RESERVED_IDS, toolJobPublicationContracts, toolJobPublicationAuthorityReady } from "../../../../../🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";
 import { toolJobProofCatalogFailures, toolJobFrameworkReservedRoutesExact } from "../../../../../🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
 import { toolJobActorProgressOverlayExact, toolJobFem2dMountedSessionExact, toolJobUniversalRetainedOwnershipExact, toolJobLayoutColdRelayRetainedExact } from "../../../../../🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🔁️live-replay/🟦️.ts";
@@ -7,5 +14,6 @@
 import { toolJobImportPreparationBounded, toolJobFullOperationBounded, toolJobStoreBatchPublicationBounded, toolJobEphemeralOneItemPublicationBounded, toolJobPublicationFreshnessBeforeEveryTurn, toolJobTypedRouteFailsClosedBeforePreparation, toolJobTypedPersistentFoundation } from "../../../../../🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📤️publication/🟦️.ts";
 import { toolJobSegmentedQueueHardBounded, toolJobSegmentedTerminalDrainExact, toolJobPuzzleReservedRoutesExact } from "../../../../../🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🛣️routes/🟦️.ts";
-import { toolJobImmutableOperationRootsExact, toolJobChildContentRootExact, toolJobArtifactStoreStructuralOwnersExact, toolJobHistoryLedgerAdmissionExact, toolJobArtifactEventSourcedMergeExact, toolJobArtifactEditMessageLedgerExact, toolJobArtifactEnvelopeOwnedCodecExact, toolJobPresentationEnvelopeCallerRetainedExact, toolJobWriterEnvelopeCallerRetainedExact, toolJobJackEnvelopeCallerRetainedExact, toolJobTrinityRewriteEnvelopeCallerRetainedExact, toolJobGisMapEnvelopeCallerRetainedExact, toolJobRasterEnvelopeCallerRetainedExact, toolJobDrawingEnvelopeCallerRetainedExact, toolJobPeerInteractionRootsExact, toolJobPagedIngressExact } from "../../../../../🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🗄️artifact-store/🟦️.ts";
+import { toolJobRasterEnvelopeSourceEvidence, toolJobRasterEnvelopeCallerFailures, toolJobImmutableOperationRootsExact, toolJobChildContentRootExact, toolJobArtifactStoreStructuralOwnersExact, toolJobHistoryLedgerAdmissionExact, toolJobArtifactEventSourcedMergeExact, toolJobArtifactEditMessageLedgerExact, toolJobArtifactEnvelopeOwnedCodecExact, toolJobPresentationEnvelopeCallerRetainedExact, toolJobWriterEnvelopeCallerRetainedExact, toolJobJackEnvelopeCallerRetainedExact, toolJobTrinityRewriteEnvelopeCallerRetainedExact, toolJobGisMapEnvelopeCallerRetainedExact, toolJobRasterEnvelopeCallerRetainedExact, toolJobDrawingEnvelopeCallerRetainedExact, toolJobPeerInteractionRootsExact, toolJobPagedIngressExact } from "../../../../../🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🗄️artifact-store/🟦️.ts";
+import { WORKSPACE_ROOT } from "../../../../../🦑️repo/🔨️modules/📚️library/🗂️workspaces/📍️checkout-root/🟦️.ts";
 import { toolJobCheckpointSelfTests } from "../../🧵️retained-command/🧪️tests/🔬️tool-job-checkpoint/🟦️.ts";
 import { toolJobScalarConfigCohortSelfTests } from "../../🧵️retained-command/🧪️tests/🔬️tool-job-scalar-config-cohort/🟦️.ts";
@@ -21,6 +29,93 @@
 import { flowSelectedCopySelfTests } from "../../../🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🧪️tests/🔬️flow-selected-copy/🟦️.ts";
 
+/** 📬️ Checks paid original batch admission, folding, publication and retirement against independent expected denials. */
+export function toolJobFundedBatchSelfTests(): number {
+  const base = join(WORKSPACE_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules");
+  const roots = { store: readFileSync(join(base, "🏪️store/🦀️.rs"), "utf8"), plugin: readFileSync(join(base, "🔌️plugin/🦀️.rs"), "utf8") };
+  const owners = { store: ["🦀️.rs", "🎮️one-item/🦀️.rs", "🎮️one-item/📬️publication/🦀️.rs"], plugin: ["🦀️.rs", "🧵️instance/🦀️.rs", "🧵️retained-command/🔄️full-operation/🦀️.rs"] };
+  for (const kind of ["store", "plugin"] as const) for (const path of owners[kind].slice(1)) if (!roots[kind].includes(`#[path = "${path}"]`) && !roots[kind].includes(`#[path="${path}"]`)) throw new Error(`batch owner is not mounted: ${path}`);
+  const source = { store: owners.store.map(path => readFileSync(join(base, "🏪️store", path), "utf8")).join("\n"), plugin: owners.plugin.map(path => readFileSync(join(base, "🔌️plugin", path), "utf8")).join("\n") };
+  const Ajv = createRequire(import.meta.url)("ajv"), oracle = new Ajv({strict:true}).compile({const:"none"});
+  for (const law of fundedBatchFixture.cases) {
+    const kind = law.owner as "store" | "plugin";
+    if (law.id !== "none" && !source[kind].includes(law.before)) throw new Error(`batch hostile target is missing: ${law.id}`);
+    const candidate = {...source, [kind]: law.id === "none" ? source[kind] : source[kind].replaceAll(law.before,law.after)};
+    if (oracle(law.id) !== law.accepted || toolJobStoreBatchPublicationBounded(candidate.store,candidate.plugin) !== law.accepted) throw new Error(`funded batch authority law: ${law.id}`);
+  }
+  return fundedBatchFixture.cases.length;
+}
+
+/** 🪙️ Preserves exact producer, receipt, fairness and retirement authority across mounted current source owners. */
+export function toolJobPersistentFoundationSelfTests(): number {
+  const base = join(WORKSPACE_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin");
+  const owners = ["🦀️.rs", "🧵️instance/🦀️.rs", "🧵️retained-command/🔄️full-operation/🦀️.rs", "👷️job/🚪️session/🦀️.rs", "📜️declaration/👷️job/🦀️.rs", "🪜️maintenance-ladder/🦀️.rs"] as const;
+  const root = readFileSync(join(base, "🦀️.rs"), "utf8"), instance = readFileSync(join(base, "🧵️instance/🦀️.rs"), "utf8");
+  for (const path of owners.slice(1, 5)) if (!root.includes(`#[path = "${path}"]`) && !root.includes(`#[path="${path}"]`)) throw new Error(`persistent owner is not mounted: ${path}`);
+  if (!instance.includes('#[path = "../🪜️maintenance-ladder/🦀️.rs"]')) throw new Error("persistent maintenance ladder is not mounted");
+  const source = owners.map(path => readFileSync(join(base, path), "utf8")).join("\n");
+  const Ajv = createRequire(import.meta.url)("ajv"), oracle = new Ajv({strict:true}).compile({const:"none"});
+  for (const law of persistentFoundationFixture.cases) {
+    if (law.id !== "none" && !source.includes(law.before)) throw new Error(`persistent hostile target is missing: ${law.id}`);
+    const changed = law.id === "none" ? source : source.replaceAll(law.before, law.after);
+    if (oracle(law.id) !== law.accepted || toolJobTypedPersistentFoundation(changed) !== law.accepted) throw new Error(`persistent authority law refused: ${law.id}`);
+  }
+  return persistentFoundationFixture.cases.length;
+}
+
 /** 🧪️ Executes tool job coverage policy assertions. */
+export function toolJobContractSourceSelfTests(): number {
+  for (const row of contractExpressionFixture.cases) {
+    const actual = toolJobContractSource(row.source, row.expression) ?? null;
+    if (JSON.stringify(actual) !== JSON.stringify(row.expected)) throw new Error(`contract expression law ${row.id}`);
+  }
+  for (const row of contractExpressionFixture.publicationCases) {
+    const actual = toolJobPublicationContracts("", row.factory, new Map());
+    if (JSON.stringify(actual ? [...actual] : null) !== JSON.stringify(row.expected)) throw new Error(`publication contract law ${row.id}`);
+  }
+  return contractExpressionFixture.cases.length + contractExpressionFixture.publicationCases.length;
+}
+
+
+/** 🧪️ Exercises current mounted Raster owners against neutral hostile vectors and an independent acceptance oracle. */
+export function toolJobRasterSourceSelfTests(): number {
+  const evidence = toolJobRasterEnvelopeSourceEvidence(WORKSPACE_ROOT), fixture = JSON.parse(readFileSync(join(import.meta.dir, "🧫️fixtures/🖨️raster-source.json"), "utf8"));
+  const Ajv = createRequire(import.meta.url)("ajv"), schema = JSON.parse(readFileSync(join(import.meta.dir, "🧬️schema/🖨️raster-source.json"), "utf8")), validate = new Ajv({strict:true}).compile(schema);
+  if (!validate(fixture)) throw new Error("Raster neutral source schema: " + JSON.stringify(validate.errors));
+  const oracle = new Ajv({strict:true}).compile({const:fixture.acceptedIdentity});
+  const identities = new Set<string>(), observedObligations = new Set<string>();
+  for (const row of fixture.cases) {
+    if (identities.has(row.id)) throw new Error("duplicate Raster hostile identity: " + row.id);
+    identities.add(row.id);
+    const owner = row.owner as "store" | "raster" | "editor" | "plugin" | "laws", current = evidence[owner];
+    if (row.id !== fixture.acceptedIdentity && row.before && !current.includes(row.before)) throw new Error("Raster hostile has no current owner target: " + row.id);
+    const candidate = {...evidence, [owner]: row.id === fixture.acceptedIdentity ? current : row.before ? current.replaceAll(row.before,row.after) : current + "\n" + row.after};
+    if (row.id !== fixture.acceptedIdentity && candidate[owner] === current) throw new Error("Raster hostile did not change its owner: " + row.id);
+    const failures = toolJobRasterEnvelopeCallerFailures(candidate.store,candidate.raster,candidate.editor,candidate.plugin,candidate.laws);
+    for (const failure of failures) observedObligations.add(failure);
+    if (oracle(row.id) !== row.accepted || (failures.length === 0) !== row.accepted || toolJobRasterEnvelopeCallerRetainedExact(candidate.store,candidate.raster,candidate.editor,candidate.plugin,candidate.laws) !== row.accepted) throw new Error("Raster authority " + row.id + ": " + failures.join(", "));
+  }
+  for (const obligation of fixture.obligations) if (!observedObligations.has(obligation)) throw new Error("Raster source obligation lacks a rejecting hostile: " + obligation);
+  console.log("[DEBUG] Raster actual production owners=" + evidence.productionPaths.length + " mounted test owners=" + evidence.testPaths.length + " neutral authority cases=" + fixture.cases.length);
+  return fixture.cases.length;
+}
+
+/** 🧱️ Keeps production fields and cfg alternatives while independently validating the exact remaining Rust syntax. */
+export function toolJobRustProductionEvidenceSelfTests(): number {
+  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "🧫️fixtures/🖨️raster-source.json"), "utf8"));
+  const Ajv = createRequire(import.meta.url)("ajv");
+  for (const row of fixture.scrubberCases) {
+    const production = toolJobProductionProtocolSource(row.source), expected = {kept:row.kept.map(() => true),removed:row.removed.map(() => false)}, observed = {kept:row.kept.map((text:string) => production.includes(text)),removed:row.removed.map((text:string) => production.includes(text))};
+    const oracle = new Ajv({strict:true}).compile({const:expected});
+    if (!oracle(observed) || production.length !== row.source.length) throw new Error("Rust production exclusion " + row.id + ": " + JSON.stringify(observed));
+    const executable = process.platform === "win32" ? join(process.env.USERPROFILE!,".rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustfmt.exe") : "rustfmt";
+    const parsed = spawnSync(executable,["--edition","2021","--config","skip_children=true","--emit","stdout"],{input:production,encoding:"utf8",timeout:10000,stdio:["pipe","ignore","pipe"]});
+    if (parsed.error || parsed.status !== 0) throw new Error("independent Rust production parser " + row.id + ": " + (parsed.error?.message ?? parsed.stderr));
+  }
+  console.log("[DEBUG] Rust production exclusion neutral cases=" + fixture.scrubberCases.length + " independently parsed");
+  return fixture.scrubberCases.length;
+}
+
 export function toolJobCoverageSelfTests(): number {
+  const contractChecks = toolJobContractSourceSelfTests() + toolJobPersistentFoundationSelfTests();
   const evaluate = (sources: Record<string, string>) => {
     const files = new Map(Object.entries(sources));
@@ -195,5 +290,5 @@
   ].join("\n");
   const femMountedEditor = "fn pending_effects() { crate::editor::fem2d::session::reconcile(doc); } fn mounted_job_maintenance_step() {} fn mounted_job_close_step() {} fn mounted_jobs_terminal_is_empty() {} fn mounted_job_prepare_snapshot_read() {} session::with_live_visual(doc.render_operation(), |visual| model_window::render_with_progress(doc.snapshot, camera, visual));";
-  const femMountedModel = "pub struct Fem2dVisualJob {} Fem2dVisualJobStage::ReserveSnapshot; canvas2d_snapshot_begin(); self.output.admit_page(); canvas2d_snapshot_seal(token); Fem2dVisualJobStage::OrderRegionKey; Fem2dVisualJobStage::OrderElementKey; fn order_field_one(&mut self, visual: &Fem2dLiveVisual) {} Fem2dVisualJobStage::BuildRegion; fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {} fn mounted_visual_output_exact_maximum_plus_one_and_page_handback() {} fn fem2d_visual_job_maximum_plus_one_rejects_before_owner_transfer() {} fn fem2d_visual_job_stale_cancel_fault_and_device_close_preserve_last_valid() {} fn fem2d_visual_job_replay_accessibility_and_each_step_are_bounded() { assert!(packet_equal(&first, &second)); assert!(started.elapsed().as_micros() < 8_000); } pub fn render_with_progress() { progress.map(Fem2dMountedVisualLease::snapshot); }";
+  const femMountedModel = "pub struct Fem2dVisualJob {} Fem2dVisualJobStage::ReserveSnapshot; canvas2d_snapshot_begin(); self.output.admit_page(); canvas2d_snapshot_seal(token); Fem2dVisualJobStage::OrderRegionKey; Fem2dVisualJobStage::OrderElementKey; fn order_field_one(&mut self, visual: &Fem2dLiveVisual) {} Fem2dVisualJobStage::BuildRegion; fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {} fn mounted_visual_output_exact_maximum_plus_one_and_page_handback() {} fn fem2d_visual_job_maximum_plus_one_rejects_before_owner_transfer() {} fn fem2d_visual_job_stale_cancel_fault_and_device_close_preserve_last_valid() {} fn fem2d_visual_job_replay_accessibility_and_each_step_are_bounded() { assert!(packet_equal(&first, &second)); assert!(started.elapsed().as_micros() < 8_000); } pub fn render_with_progress() { progress.map(Fem2dMountedVisualLease::snapshot); fem2d_structure_layers(doc); }";
   const femMountedMesh = "MeshJobStage::ReservePreparation; MeshJobStage::ReserveEdgeAuthorities; fn advance_face_classification(&mut self) { triangulation.triangles.get(self.face_cursor); } MeshJobStage::Classify => { self.advance_face_classification(); } MeshJobStage::Finalize => {} fn mesh_mounted_classification_indexes_admit_maximum_reject_plus_one_and_close_exactly() {}";
   const femMountedRoot = 'pub fn plugin() { crate::editor::fem2d::session::initialize(); Plugin::<FemApps>::builder("fem"); }';
@@ -396,64 +491,5 @@
   const staleExposure = noStageWatchdog.replace("validate_commit(base_revision, generation);", "").replace("fn step() { ", "fn step() { cx.is_cancelled(); cx.should_yield(); ");
   if (toolJobFullOperationBounded(staleExposure)) throw new Error("[verify interactivity tool-jobs] self-test full-operation-without-stale-result-validation was falsely accepted.");
-  const retainedStorePublication = `
-pub trait ArtifactStoreOneItemPreparationFactory {}
-pub trait ArtifactStoreOneItemPreparation {}
-pub struct ArtifactStoreOneItemPrepared;
-trait ArtifactStoreBatchSource {}
-pub enum ArtifactStoreOneItemPublicationPhase { Prepare, Commit, Outbound, Retiring, Complete }
-pub struct ArtifactStoreBatchPublication<P, Mutation> { generation: u64, maximum_items: usize, maximum_bytes: usize, marker: PhantomData<(P, Mutation)> }
-impl<P, Mutation> ArtifactStoreBatchPublication<P, Mutation> {
-  fn begin_close(&mut self) {}
-  fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) { let items = maximum_items.min(1); if items > 0 && maximum_bytes > 0 { self.maximum_items = items; } }
-  fn terminal_is_empty(&self) -> bool { self.phase == ArtifactStoreOneItemPublicationPhase::Complete && self.marker.is_empty() && self.maximum_items == 0 && self.maximum_bytes == 0 && self.generation == 0 }
-}
-impl<P, Mutation> Drop for ArtifactStoreBatchPublication<P, Mutation> { fn drop(&mut self) { assert!(self.terminal_is_empty()); } }
-impl<P, Mutation> ArtifactStore<P, Mutation> {
-  fn begin_apply_batch(&mut self) -> ArtifactStoreBatchPublication<P, Mutation> { self.begin_apply_batch_owned() }
-  fn begin_apply_batch_owned(&mut self) -> ArtifactStoreBatchPublication<P, Mutation> { let footprint = source.footprint(lane); ArtifactStoreBatchPublication { generation, maximum_items, maximum_bytes, marker: PhantomData } }
-  fn batch_item_base(&self) -> SnapshotRead<P> { self.snapshot_read() }
-  fn advance_apply_batch(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mutation>) { publication.generation += 1; }
-  fn cancel_apply_batch(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mutation>) { publication.begin_close(); }
-  fn fold_batch_item(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mutation>) { authority.validate_prepared(candidate); inverse.reverse(); publication.staged += 1; }
-}`;
-  const retainedPluginPublication = `
-struct Host {
-  artifact_one_item_factory: Option<ArtifactFactory>,
-  config_one_item_factory: Option<ConfigFactory>,
-  draft_one_item_factory: Option<DraftFactory>,
-  unsupported_publication_contracts: Contracts,
-}
-struct MountedTypedCommandFullOperation<A> { pending_artifact_publication: Option<ArtifactStoreBatchPublication<A::Snapshot, A::Mutation>> }
-async fn publish_mounted_typed_operation_unit() {
-  self.artifact_one_item_factory.as_ref();
-  self.config_one_item_factory.as_ref();
-  self.draft_one_item_factory.as_ref();
-  mounted.pending_artifact_publication = Some(self.store.begin_apply_batch(std::mem::take(&mut emit.artifact_mutations)));
-  self.config_store.begin_apply_batch(std::mem::take(&mut emit.config_mutations));
-  self.draft_store.begin_apply_batch(std::mem::take(&mut emit.draft_mutations));
-  self.store.advance_apply_batch(mounted.pending_artifact_publication.as_mut().unwrap());
-}`;
-  if (!toolJobStoreBatchPublicationBounded(retainedStorePublication, retainedPluginPublication))
-    throw new Error("[verify interactivity tool-jobs] self-test retained Store batched publication was falsely rejected.");
-  if (toolJobStoreBatchPublicationBounded(retainedStorePublication.replace("pub trait ArtifactStoreOneItemPreparationFactory {}", ""), retainedPluginPublication))
-    throw new Error("[verify interactivity tool-jobs] self-test Store publication without a domain preparation factory was falsely accepted.");
-  if (toolJobStoreBatchPublicationBounded(retainedStorePublication.replace("publication.generation += 1;", "apply_command(publication);"), retainedPluginPublication))
-    throw new Error("[verify interactivity tool-jobs] self-test Store publication that restores whole apply_command was falsely accepted.");
-  if (toolJobStoreBatchPublicationBounded(retainedStorePublication.replace("publication.generation += 1;", "cursor.try_reserve_exact(total); publication.generation += 1;"), retainedPluginPublication))
-    throw new Error("[verify interactivity tool-jobs] self-test Store publication that hides whole-vector growth in a phase was falsely accepted.");
-  if (toolJobStoreBatchPublicationBounded(retainedStorePublication.replace("fn terminal_is_empty", "fn terminal_owner_remains"), retainedPluginPublication))
-    throw new Error("[verify interactivity tool-jobs] self-test Store publication without terminal-empty witness was falsely accepted.");
-  if (toolJobStoreBatchPublicationBounded(retainedStorePublication.replace("impl<P, Mutation> Drop for ArtifactStoreBatchPublication", "impl<P, Mutation> Closed for ArtifactStoreBatchPublication"), retainedPluginPublication))
-    throw new Error("[verify interactivity tool-jobs] self-test Store publication without terminal Drop enforcement was falsely accepted.");
-  if (toolJobStoreBatchPublicationBounded(retainedStorePublication.replace("self.maximum_items = items;", "owners.clear(); self.maximum_items = items;"), retainedPluginPublication))
-    throw new Error("[verify interactivity tool-jobs] self-test Store publication with whole-vector close was falsely accepted.");
-  if (toolJobStoreBatchPublicationBounded(retainedStorePublication, retainedPluginPublication.replace("advance_apply_batch", "apply_one")))
-    throw new Error("[verify interactivity tool-jobs] self-test plugin publisher that restores monolithic apply_one was falsely accepted.");
-  if (toolJobStoreBatchPublicationBounded(retainedStorePublication, retainedPluginPublication.replace("self.artifact_one_item_factory.as_ref();", "A::build_artifact_store_one_item_preparation_factory();")))
-    throw new Error("[verify interactivity tool-jobs] self-test per-publication durable factory reconstruction was falsely accepted.");
-  if (toolJobStoreBatchPublicationBounded(retainedStorePublication, retainedPluginPublication.replace("std::mem::take(&mut emit.artifact_mutations)", "emit.artifact_mutations.last().cloned()")))
-    throw new Error("[verify interactivity tool-jobs] self-test batched publisher that clones instead of draining its whole gesture lane was falsely accepted.");
-  if (toolJobStoreBatchPublicationBounded(retainedStorePublication.replace("inverse.reverse();", ""), retainedPluginPublication))
-    throw new Error("[verify interactivity tool-jobs] self-test batched fold without the replay_mutations inverse ordering was falsely accepted.");
+  const fundedBatchChecks = toolJobFundedBatchSelfTests();
   const retainedEphemeralPublication = `
 pub trait ArtifactEphemeralOneItemPreparationFactory {}
@@ -565,6 +601,4 @@
   const unguardedIncompleteRoute = "async fn dispatch_typed_command_inner() { refresh_cache().await; let session = WorkerJobSession::new(job); }";
   if (toolJobTypedRouteFailsClosedBeforePreparation(unguardedIncompleteRoute)) throw new Error("[verify interactivity tool-jobs] self-test incomplete-typed-route-without-preparation-guard was falsely accepted.");
-  const terminalLoopTypedFoundation = "struct ActiveToolCommand<A>; struct VcsArtifactApp<A> { tool_operations: ArtifactFixedRegistry<ActiveToolCommand<A>> } impl<A: ArtifactApp> ActiveToolCommand<A> { fn drive_worker_step() { session.try_submit_step(pool, semio_framework_async::Lane::Interactive); pending.try_recv(); ActiveToolCommandStage::CommitReady; } } async fn dispatch_typed_command_inner() { require_complete_tool_operation_pipeline(&admission)?; let operation_id = allocate_operation_id(); if !self.tool_operations.can_insert(operation_id.0) {} refresh_cache().await; let session = WorkerJobSession::new(job); self.tool_operations.insert_admitted(operation_id, active); let outcome = loop { session.step(&pool).await; }; active.drive_worker_step(&pool)?; DslValue::String(operation_id.0.to_string()); }";
-  if (toolJobTypedPersistentFoundation(terminalLoopTypedFoundation)) throw new Error("[verify interactivity tool-jobs] self-test typed-persistent-foundation-with-terminal-loop was falsely accepted.");
   const wholeMapDropCancellation = "impl<A: ArtifactApp> Drop for VcsArtifactApp<A> { fn drop() { resolve_ready(self.tool_cancellations.cancel_all()); operations.drain(); } }";
   if (toolJobDropCancellationBounded(wholeMapDropCancellation)) throw new Error("[verify interactivity tool-jobs] self-test whole-map-drop-cancellation was falsely accepted.");
@@ -926,376 +960,5 @@
   if (toolJobGisMapEnvelopeCallerRetainedExact(retainedJackStore, retainedGisMapCodec.replace("gis_map_all_twelve_mutation_variants_preserve_catalog_order_and_zero_grant_ownership", "gis_map_partial_mutation_catalog"), retainedGisMapEditor, retainedGisMapWasm, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test GIS-Map-zero-grant-catalog-fixture-missing was falsely accepted.");
   if (toolJobGisMapEnvelopeCallerRetainedExact(retainedJackStore, retainedGisMapCodec, retainedGisMapEditor, `${retainedGisMapWasm}\nenvelope_json: &str`, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test GIS-Map-whole-buffer-ingress-bypass was falsely accepted.");
-  const retainedRasterCodec = [
-    "pub struct RasterEnvelopeOwnedFieldCatalog",
-    "artifact_owned_spr_edit_history_decoder",
-    "RasterSnapshotDecodeAuthority",
-    "RasterMutationDecodeAuthority",
-    "OwnedSchemaHexAuthority<RASTER_OWNED_FIELD_BYTES>",
-    "struct RasterSnapshotBoundsAuthority",
-    "struct RasterSnapshotCloneAuthority",
-    "struct RasterMutationCandidateAuthority",
-    "RASTER_RETIREMENT_STACK_CAPACITY",
-    "pages: std::mem::ManuallyDrop<[Option<Box<RasterRetirementFramePage>>; RASTER_RETIREMENT_STACK_PAGE_COUNT]>",
-    "pending_push: std::mem::ManuallyDrop<Option<RasterRetirementOwner>>",
-    "struct RasterStoreInitializationAuthority",
-    "impl semio_framework_plugin::ArtifactStoreInitializationAuthority<RasterSnapshot, RasterMutation> for RasterStoreInitializationAuthority",
-    "RasterStoreInitializationPhase::ValidateEdit { index }",
-    "self.edit_index.admit(&envelope.vcs.edits, index,",
-    "RasterStoreInitializationPhase::SeedHistory",
-    "RasterStoreInitializationPhase::BuildCandidate",
-    "if cx.should_yield()",
-    "if cx.should_yield()",
-    "if cx.should_yield()",
-    "fn raster_reserve_unit(cx: &mut semio_framework_job::StepContext<'_>) -> bool",
-    "cx.consume_fuel(1)",
-    "self.cancel_requested || cx.is_cancelled()",
-    "ArtifactStoreInitializationRuntime::new",
-    "ArtifactStore::from_initialized_runtime_with_owners",
-    "self.generation.0.checked_add(1)",
-    "value.layers.pop()",
-    "value.assets.take_last_entry()",
-    "dsl::DslValue::Array(values)",
-    "dsl::DslValue::Object(values)",
-    "CreateLayer(payload)",
-    "DeleteLayer(payload)",
-    "ReorderLayers(payload)",
-    "RenameLayer(payload)",
-    "ChangeLayerVisible(payload)",
-    "ChangeLayerOpacity(payload)",
-    "ChangeLayerBlendMode(payload)",
-    "MoveLayer(payload)",
-    "ResizeLayer(payload)",
-    "ChangeLayerAdjustmentKind(payload)",
-    "AddLayerAsset(payload)",
-    "RemoveLayerAsset(payload)",
-    "Raster store initialization authority reached Drop before exact candidate handoff or retained rejection close",
-    "raster_store_initializer_publishes_next_generation_and_candidate_closes_incrementally",
-    "raster_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty",
-    "raster_store_initializer_zero_budget_advances_no_owner_or_phase",
-    "raster_nested_snapshot_and_child_handles_retire_one_owner_per_grant",
-    "raster_owner_caps_and_all_mutation_variants_retire_one_owner_per_grant",
-    "raster_envelope_caps_and_plus_one_page_return_the_exact_fixed_owner",
-    "raster_snapshot_bounds_and_clone_advance_one_pre_admitted_unit_with_low_nonzero_fuel",
-    "raster_expired_deadline_advances_no_bounds_clone_or_mutation_owner",
-    "raster_small_mutation_against_deep_snapshot_is_cursorized_and_atomic",
-    "raster_cancel_after_complete_retires_the_unclaimed_candidate_before_terminal",
-    "raster_retirement_uses_allocation_capacity_and_fixed_iterative_depth",
-    "raster_nested_owner_item_and_byte_capacity_plus_one_reject_before_clone",
-    "raster_empty_bounds_and_mounted_sixty_four_fuel_progress_across_second_map_page",
-    "raster_retirement_page_credit_is_claimed_before_allocation_and_returned_with_backing",
-    "raster_owned_map_removal_returns_exact_pair_and_populated_drop_refuses",
-    "const RASTER_MAXIMUM_CONTROL_BYTES: usize = RASTER_MAXIMUM_CONTROL_BACKINGS * RASTER_CONTROL_BACKING_BYTES",
-    "source_control_bytes: usize",
-    "self.source_control_bytes = RASTER_MAXIMUM_CONTROL_BYTES",
-    "raster-store.control-backing-double-reservation",
-    "static RASTER_RETIREMENT_PROCESS_PAGES: std::sync::atomic::AtomicUsize",
-    "static RASTER_INITIALIZATION_PROCESS_CONTROLS: std::sync::atomic::AtomicUsize",
-    "static RASTER_STANDALONE_PROCESS_CONTROLS: std::sync::atomic::AtomicUsize",
-    "held_items: usize",
-    "held_bytes: usize",
-    "control: std::mem::ManuallyDrop<Option<RasterStandaloneControlCredit>>",
-    "control: std::mem::ManuallyDrop<Option<RasterStandaloneControlCredit>>",
-    "let control = RasterStandaloneControlCredit::try_claim().ok();",
-    "let control = RasterStandaloneControlCredit::try_claim().ok();",
-    "fn claim_control_if_available(&mut self) -> Result<bool, String>",
-    "Err(\"raster-store.standalone-control-capacity\") => Ok(false)",
-    "control_returned: bool",
-    "remaining: RASTER_NON_STACK_CONTROL_BACKINGS",
-    "control_reservation: std::mem::ManuallyDrop<Option<RasterInitializationControlReservation>>",
-    "normal completion returns every non-stack process control credit",
-    "standalone Box control credit is returned before terminal-empty",
-    "standalone Arc and inner Box control credits return before terminal-empty",
-    "fn reserve_page_credit(&mut self, page_index: usize)",
-    "compare_exchange(current, next",
-    "self.return_page_credit(page_index)?",
-    "pages: std::mem::ManuallyDrop<[Option<Box<RasterOwnedMapPage<V>>>; RASTER_OWNED_MAP_PAGE_COUNT]>",
-    "Raster owned map reached Drop before every entry and page backing was explicitly retired",
-    "Populated Raster owned maps require the retained page clone authority",
-    "Raster maps require the retained page decoder",
-    "Populated Raster owned map DSL materialization is forbidden; interactive production routes require the retained page output authority",
-    "dsl::FieldValue::Map(Vec::new())",
-    "pub fn remove_entry(&mut self, key: &str) -> Option<RasterOwnedMapEntry<V>>",
-    "RasterOwnedMapInsert::Replaced",
-    "raster_empty_asset_map_retirement_has_no_hidden_allocation_release",
-    "value.assets.take_last_entry()",
-    "value.assets.take_empty_page_backing()",
-    "RasterRetirementOwner::AssetMapPage(page)",
-    "RasterRetirementOwner::ValueMapPage(page)",
-    "RasterOwnedMap::<V>::conservative_page_credit_bytes()",
-    "fn observe_candidate_capacity(",
-    "fn raster_exact_string_from_parts(",
-    "const RASTER_CONTROL_BACKING_BYTES: usize = RASTER_OWNED_FIELD_BYTES",
-    "const RASTER_MAXIMUM_CONTROL_BACKINGS: usize = RASTER_RETIREMENT_STACK_PAGE_COUNT + RASTER_NON_STACK_CONTROL_BACKINGS",
-    "RasterRetirementOwner::BoxedLayer(Some(layer))",
-    "const RASTER_RETIREMENT_ADMITTED_FRAME_CAPACITY: usize = RASTER_RETIREMENT_LAYER_FRAMES + RASTER_RETIREMENT_VALUE_FRAMES + RASTER_RETIREMENT_WRAPPER_FRAMES",
-    "const RASTER_RETIREMENT_STACK_CAPACITY: usize = RASTER_RETIREMENT_ADMITTED_FRAME_CAPACITY + RASTER_RETIREMENT_REJECTED_OWNER_MARGIN",
-    "raster_owned_map_cap_plus_one_returns_exact_owner_and_populated_pages_retire_explicitly",
-    "raster_observed_capacity_and_combined_retirement_depth_are_exact",
-    "raster_box_and_arc_control_backings_require_and_report_fixed_credit",
-    "raster_standalone_control_max_plus_one_returns_exact_owner_and_resumes_after_full_saturation",
-    "raster_arc_factory_full_saturation_preserves_exact_producer_through_every_control_phase",
-    "raster_populated_dsl_materialization_max_plus_one_nested_cancel_fault_panic_and_close_are_exact",
-    "impl<V> dsl::ToValue for RasterOwnedMap<V>",
-    "impl<V> dsl::FromValue for RasterOwnedMap<V>",
-    "Populated Raster owned map serialization is forbidden; interactive production routes require the retained page output authority",
-    "    pub assets: RasterOwnedMap<RasterAssetChild>,",
-    "    pub assets: RasterOwnedMap<RasterAssetChild>,",
-    "        params: RasterOwnedMap<dsl::DslValue>,",
-    "raster_populated_serde_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact",
-    "pub(crate) fn require_empty_output_shell(&self) -> Result<(), &'static str>",
-    "self.layers.is_empty() && self.assets.is_empty()",
-    "Populated Raster snapshot output is forbidden; interactive production routes require the retained page output authority",
-    "pub(crate) fn enc_asset_map(map: &RasterOwnedMap<RasterAssetChild>) -> String",
-    "pub(crate) fn enc_params(params: &RasterOwnedMap<dsl::DslValue>) -> String",
-    "fn write_asset_map(out: &mut Vec<u8>, map: &RasterOwnedMap<RasterAssetChild>)",
-    "fn write_params(out: &mut Vec<u8>, params: &RasterOwnedMap<dsl::DslValue>)",
-    "fn print_raster_snapshot_body(s: &RasterSnapshot) -> String",
-    "fn encode_raster_snapshot_binary(s: &RasterSnapshot) -> Vec<u8>",
-    'assert!(map.is_empty(), "{RASTER_POPULATED_OUTPUT_ERROR}");',
-    'assert!(params.is_empty(), "{RASTER_POPULATED_OUTPUT_ERROR}");',
-    'assert!(map.is_empty(), "{RASTER_POPULATED_OUTPUT_ERROR}");',
-    'assert!(params.is_empty(), "{RASTER_POPULATED_OUTPUT_ERROR}");',
-    'assert!(list.is_empty(), "{RASTER_POPULATED_OUTPUT_ERROR}");',
-    'assert!(list.is_empty(), "{RASTER_POPULATED_OUTPUT_ERROR}");',
-    "s.require_empty_output_shell().expect(RASTER_POPULATED_OUTPUT_ERROR);",
-    "s.require_empty_output_shell().expect(RASTER_POPULATED_OUTPUT_ERROR);",
-    "self.require_empty_output_shell().expect(RASTER_POPULATED_OUTPUT_ERROR);",
-    "self.require_empty_output_shell().map_err(|error| store::PackError::Schema(error.to_owned()))?;",
-    "fn raster_populated_snapshot_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact()",
-    "let plus_one_param_value_pointer = plus_one_param_value.as_ptr();",
-    "let rejected_param = params.insert",
-    "params.insert(plus_one_param_key, dsl::DslValue::String(plus_one_param_value))",
-    "let rejected_param_value = match &rejected_param.value",
-    'assert_eq!(rejected_param_value.as_ptr(), plus_one_param_value_pointer, "rejected output parameter returns the exact value allocation");',
-    "RasterOwnedRetirement::new(RasterRetirementOwner::ValueEntry { key: rejected_param.key, value: Some(rejected_param.value) })",
-    "let plus_one_asset_child_pointer = plus_one_asset_child.child_id.as_ptr();",
-    "let rejected_asset = assets",
-    "assets.insert(plus_one_asset_key, plus_one_asset_child)",
-    'assert_eq!(rejected_asset.value.child_id.as_ptr(), plus_one_asset_child_pointer, "rejected output asset returns the exact child allocation");',
-    "RasterOwnedRetirement::new(RasterRetirementOwner::AssetEntry { key: rejected_asset.key, child: Some(rejected_asset.value) })",
-    ...["bmp", "png", "tiff", "jpg"].map(
-      (format) =>
-        `pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> { let image = raster_composite_image(snapshot)?; let target = semio_image_to_format(&image, ${format.toUpperCase()}_DIALECT)?; semio_s_artifact_stdio_${format}::io::encode_${format}(&target) }`,
-    ),
-    "pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> { let image = raster_composite_image(snapshot)?; let gif89a = semio_image_to_format(&image, GIF89A_DIALECT)?; semio_s_artifact_stdio_gif::standards::v87a::subsets::any::io::encode_gif(&gif87a::from_89a(&gif89a)) }",
-    "pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> { let (svg, _width, _height) = crate::artifacts::raster::io::raster_document_json_to_svg(snapshot)?; Ok(svg.into_bytes()) }",
-    'pub const RASTER_PDF_EXPORT_UNSUPPORTED: &str = "pdf export not supported for a raster document";',
-    "pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> { let _ = snapshot; Err(RASTER_PDF_EXPORT_UNSUPPORTED.to_string()) }",
-    'pub const RASTER_DWG_EXPORT_UNSUPPORTED: &str = "dwg export not supported for a raster document";',
-    "pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> { let _ = snapshot; Err(RASTER_DWG_EXPORT_UNSUPPORTED.to_string()) }",
-    "std::mem::size_of::<RasterOwnedRetirement>() <= RASTER_CONTROL_BACKING_BYTES",
-    "std::mem::size_of::<RasterRetirementFramePage>() <= RASTER_CONTROL_BACKING_BYTES",
-    "fn raster_maximum_combined_layer_and_value_depth_retires_to_terminal()",
-    "Ok(Some((previous_key, previous)))",
-    "Ok(Some((previous_key, previous)))",
-    "Ok(Some((previous_key, previous)))",
-    "replacement returns the exact displaced pair",
-    "assert_eq!(previous_key.as_ptr(), old_key_pointer)",
-    "candidate_disposer",
-    "let released_bytes = value.capacity();",
-    "let bytes = value.capacity();",
-  ].join("\n");
-  const retainedRasterEditor = [
-    "fn build_envelope_decode_owner_bundle() raster_envelope_decode_owner_bundle() fn build_document_store_initialization_job( raster_document_store_initialization_job(envelope, operation, generation)",
-    "fn admit_raster_envelope(app: &mut VcsArtifactApp<EditorApp<RasterPlayApp>>, wire: &[u8]) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle {",
-    "let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect(\"Raster live envelope ingress credits\");",
-    "for chunk in wire.chunks(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {",
-    "let mut bytes = [0; store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];",
-    "let page = store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect(\"bounded Raster live envelope page\");",
-    "app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!(\"Raster live envelope page admission failed: {fault}\"));",
-    "assert!(app.seal_artifact_envelope_ingress(handle).expect(\"Raster live envelope seal/submit\"));",
-    "app.maintenance_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect(\"one Raster live maintenance turn\");",
-    "let poll = app.advance_artifact_envelope_load(handle).expect(\"Raster live load advancement\");",
-    "match retirement.close_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect(\"Raster fixture envelope retirement\") {",
-    "async fn raster_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed() {",
-    "assert_eq!(handle.generation, base_generation);",
-    "assert!(app.acknowledge_artifact_store_replacement(handle).expect(\"first exact Raster load acknowledgement\"));",
-    "assert!(!app.acknowledge_artifact_store_replacement(handle).expect(\"duplicate Raster load acknowledgement is a no-op\"));",
-    "async fn raster_live_envelope_cancel_closes_retained_pages_without_publication() {",
-    "app.cancel_artifact_envelope_load(handle).expect(\"cancel exact Raster ingress\");",
-  ].join("\n");
-  if (!toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test retained-Raster-envelope-route was falsely rejected.");
-  for (const signature of [
-    "pub(crate) fn enc_asset_map(map: &RasterOwnedMap<RasterAssetChild>) -> String",
-    "pub(crate) fn enc_params(params: &RasterOwnedMap<dsl::DslValue>) -> String",
-    "fn write_asset_map(out: &mut Vec<u8>, map: &RasterOwnedMap<RasterAssetChild>)",
-    "fn write_params(out: &mut Vec<u8>, params: &RasterOwnedMap<dsl::DslValue>)",
-    "fn print_raster_snapshot_body(s: &RasterSnapshot) -> String",
-    "fn encode_raster_snapshot_binary(s: &RasterSnapshot) -> Vec<u8>",
-  ]) {
-    const suspended = signature.replace("fn ", "async fn ");
-    if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace(signature, suspended), retainedRasterEditor, retainedWriterPlugin))
-      throw new Error(`[verify interactivity tool-jobs] self-test Raster-immediate-signature ${signature} was falsely accepted as async.`);
-  }
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace('let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("Raster live envelope ingress credits");\n', ""), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-page-admitted-without-ingress-credits was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace('let page = store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded Raster live envelope page");\napp.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("Raster live envelope page admission failed: {fault}"));', 'app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("Raster live envelope page admission failed: {fault}"));\nlet page = store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded Raster live envelope page");'), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-page-admitted-before-bounded-construction was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("let mut bytes = [0; store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES]", "let mut bytes = Vec::new()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-fixed-page-owner-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("assert_eq!(handle.generation, base_generation);", "let _ = handle;"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-generation-handle-erasure was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("-> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle {", "{"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-untyped-ingress-handle was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("for chunk in wire.chunks(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {", "let chunk = wire;"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-single-whole-wire-page was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("app.seal_artifact_envelope_ingress(handle)", "app.submit_all()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-unsealed-ingress-submit was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("app.maintenance_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)", "app.run_to_completion()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-unpumped-load-progress was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("app.advance_artifact_envelope_load(handle)", "app.await_load()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-unpolled-load-terminal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("duplicate Raster load acknowledgement is a no-op", "second Raster acknowledgement replaces the store again"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-repeatable-load-acknowledgement was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("async fn raster_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed()", "async fn raster_envelope_loads()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-live-submit-fixture-missing was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("async fn raster_live_envelope_cancel_closes_retained_pages_without_publication()", "async fn raster_envelope_cancels()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-live-cancel-fixture-missing was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("self.generation.0.checked_add(1)", "self.generation.0 + 1"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-unchecked-generation-publication was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replaceAll("if cx.should_yield()", "if false"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-deadline-fuel-guard-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("self.cancel_requested || cx.is_cancelled()", "self.cancel_requested"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-worker-cancel-guard-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_store_initializer_zero_budget_advances_no_owner_or_phase", "raster_zero_budget_advances_work"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-zero-budget-fixture-missing was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("RemoveLayerAsset(payload)", "drop(payload)"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-mutation-catalog-hole was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("dsl::DslValue::Object(values)", "drop(value)"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-nested-value-deep-drop was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_nested_snapshot_and_child_handles_retire_one_owner_per_grant", "raster_shallow_snapshot_fixture"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-nested-close-fixture-missing was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_owner_caps_and_all_mutation_variants_retire_one_owner_per_grant", "raster_partial_owner_fixture"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-cap-terminal-fixture-missing was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_envelope_caps_and_plus_one_page_return_the_exact_fixed_owner", "raster_unbounded_page_fixture"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-page-plus-one-fixture-missing was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len())", "store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage::try_from_vec(bytes.to_vec(), chunk.len())"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-dynamic-page-route was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, `${retainedRasterEditor}\nenvelope_json: &str`, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-whole-buffer-bypass was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replaceAll("acknowledge_artifact_store_replacement(handle)", "return Ready"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-terminal-without-exact-ack was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("app.cancel_artifact_envelope_load(handle)", "drop(handle)"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-cancel-owner-drop was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("close_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)", "close_all()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-bulk-close was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor, retainedWriterPlugin.replace("artifact_envelope_ingress_saturation_returns_exact_plus_one_owner_and_closes_fifo_slots", "artifact_envelope_ingress_drops_plus_one"))) throw new Error("[verify interactivity tool-jobs] self-test Raster-fixed-registry-exact-handback-fixture-missing was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor, retainedWriterPlugin.replace("artifact_envelope_ingress_cancel_and_interrupted_close_release_one_real_page_per_grant", "artifact_envelope_ingress_bulk_close"))) throw new Error("[verify interactivity tool-jobs] self-test Raster-one-page-close-fixture-missing was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("struct RasterSnapshotBoundsAuthority", "struct RasterPostCloneBoundsAuthority"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-recursive-preflight-authority-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("fn raster_reserve_unit(cx: &mut semio_framework_job::StepContext<'_>) -> bool", "fn raster_post_work_fuel(cx: &mut semio_framework_job::StepContext<'_>) -> bool"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-pre-work-fuel-authority-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("cx.consume_fuel(1)", "cx.consume_fuel(RASTER_CONTROL_BACKING_BYTES as u64)"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-mounted-64-fuel-byte-charge was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("self.source_control_bytes = RASTER_MAXIMUM_CONTROL_BYTES", "self.source_bytes = RASTER_MAXIMUM_NESTED_BYTES"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-control-payload-double-counting was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("static RASTER_INITIALIZATION_PROCESS_CONTROLS: std::sync::atomic::AtomicUsize", "static RASTER_UNTRACKED_CONTROLS: std::sync::atomic::AtomicUsize"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-non-stack-process-control-reservation-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("static RASTER_STANDALONE_PROCESS_CONTROLS: std::sync::atomic::AtomicUsize", "static RASTER_UNTRACKED_STANDALONE_CONTROLS: std::sync::atomic::AtomicUsize"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-standalone-Box-Arc-control-reservation-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replaceAll("control: std::mem::ManuallyDrop<Option<RasterStandaloneControlCredit>>", "control: Option<RasterStandaloneControlCredit>"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-standalone-control-terminal-shell-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replaceAll("let control = RasterStandaloneControlCredit::try_claim().ok();", "let control = RasterStandaloneControlCredit::try_claim().expect(\"saturation panic\");"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-saturated-constructor-panic-restoration was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("Err(\"raster-store.standalone-control-capacity\") => Ok(false)", "Err(code) => panic!(\"{code}\")"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-saturated-retirement-resume-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("control_returned: bool", "control_returned_without_witness: ()"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-Arc-control-return-witness-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("normal completion returns every non-stack process control credit", "normal completion leaks process control credit"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-process-control-zero-fixture-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("fn reserve_page_credit(&mut self, page_index: usize)", "fn allocate_page_without_credit(&mut self, page_index: usize)"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-retirement-page-credit-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("pages: std::mem::ManuallyDrop<[Option<Box<RasterOwnedMapPage<V>>>; RASTER_OWNED_MAP_PAGE_COUNT]>", "pages: [Option<Box<RasterOwnedMapPage<V>>>; RASTER_OWNED_MAP_PAGE_COUNT]"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-map-fail-closed-shell-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\npub fn remove(&mut self, key: &str)`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-key-discarding-remove-restoration was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("Populated Raster owned maps require the retained page clone authority", "Raster clone allocates map pages"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-map-whole-clone-restoration was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("Raster maps require the retained page decoder", "Raster serde allocates map pages"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-uncredited-map-serde-restoration was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("Populated Raster owned map DSL materialization is forbidden; interactive production routes require the retained page output authority", "ordinary populated DSL materialization is allowed"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-map-DSL-fail-closure-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("dsl::FieldValue::Map(Vec::new())", "let mut entries = Vec::with_capacity(self.length); for (key, value) in self { entries.push((key.clone(), value.to_value())); } dsl::FieldValue::Map(entries)"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-map-uncredited-DSL-loop-restoration was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\nserde_json::to_vec(source)`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-whole-recursive-encode-reintroduction was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\nsource.clone()`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-whole-recursive-clone-reintroduction was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\noperation.diff(current)\ndiff.apply(current)`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-monolithic-history-apply-reintroduction was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("candidate_disposer", "discarded_candidate"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-complete-candidate-retirement-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("let released_bytes = value.capacity();", "let released_bytes = value.len();"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-string-capacity-retirement-erasure was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("let bytes = value.capacity();", "let bytes = value.len();"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-byte-capacity-retirement-erasure was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("pages: std::mem::ManuallyDrop<[Option<Box<RasterRetirementFramePage>>; RASTER_RETIREMENT_STACK_PAGE_COUNT]>", "active: Box<RasterOwnedRetirement>"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-fixed-iterative-retirement-erasure was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_small_mutation_against_deep_snapshot_is_cursorized_and_atomic", "raster_small_mutation_clones_deep_snapshot"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-low-fuel-deep-mutation-fixture-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_cancel_after_complete_retires_the_unclaimed_candidate_before_terminal", "raster_cancel_drops_completed_candidate"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-cancel-after-complete-fixture-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_nested_owner_item_and_byte_capacity_plus_one_reject_before_clone", "raster_nested_owner_capacity_without_plus_one"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-nested-owner-capacity-plus-one-fixture-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_empty_asset_map_retirement_has_no_hidden_allocation_release", "raster_empty_asset_map_bulk_drop"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-empty-map-allocation-release-fixture-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("value.assets.take_empty_page_backing()", "drop(std::mem::take(&mut value.assets))"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-map-page-retirement-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("fn observe_candidate_capacity(", "fn trust_requested_candidate_capacity("), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-observed-capacity-admission-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("const RASTER_CONTROL_BACKING_BYTES: usize = RASTER_OWNED_FIELD_BYTES", "const RASTER_CONTROL_BACKING_BYTES: usize = 0"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-control-backing-credit-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("const RASTER_MAXIMUM_CONTROL_BACKINGS: usize = RASTER_RETIREMENT_STACK_PAGE_COUNT + RASTER_NON_STACK_CONTROL_BACKINGS", "const RASTER_MAXIMUM_CONTROL_BACKINGS: usize = 16"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-control-backing-count-regression was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("RASTER_RETIREMENT_LAYER_FRAMES + RASTER_RETIREMENT_VALUE_FRAMES + RASTER_RETIREMENT_WRAPPER_FRAMES", "RASTER_MAXIMUM_NESTED_DEPTH * 2 + 8"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-combined-retirement-depth-regression was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("RASTER_RETIREMENT_ADMITTED_FRAME_CAPACITY + RASTER_RETIREMENT_REJECTED_OWNER_MARGIN", "RASTER_RETIREMENT_ADMITTED_FRAME_CAPACITY"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-rejected-owner-retirement-margin-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_owned_map_cap_plus_one_returns_exact_owner_and_populated_pages_retire_explicitly", "raster_empty_map_only"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-map-cap-fixture-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_observed_capacity_and_combined_retirement_depth_are_exact", "raster_requested_capacity_only"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-observed-capacity-fixture-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_box_and_arc_control_backings_require_and_report_fixed_credit", "raster_unreported_control_drop"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-control-owner-fixture-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_standalone_control_max_plus_one_returns_exact_owner_and_resumes_after_full_saturation", "raster_standalone_saturation_panics"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-standalone-max-plus-one-fixture-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_arc_factory_full_saturation_preserves_exact_producer_through_every_control_phase", "raster_arc_saturation_drops_producer"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-Arc-every-control-phase-saturation-fixture-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_populated_dsl_materialization_max_plus_one_nested_cancel_fault_panic_and_close_are_exact", "raster_empty_dsl_map_only"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-DSL-hostile-fixture-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_populated_serde_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact", "raster_empty_serde_map_only"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-serde-hostile-fixture-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("    pub assets: RasterOwnedMap<RasterAssetChild>,", "    pub assets: std::collections::HashMap<String, RasterAssetChild>,"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-unguarded-map-field-escape was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("impl<V> dsl::ToValue for RasterOwnedMap<V>", "impl<V: dsl::ToValue> dsl::ToValue for RasterOwnedMapEntry<V>"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-owned-map-output-authority-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("impl<V> dsl::FromValue for RasterOwnedMap<V>", "impl<V: dsl::FromValue> dsl::FromValue for RasterOwnedMapEntry<V>"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-owned-map-decode-authority-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\n#[cfg_attr(test, serde(serialize_with = "crate::artifacts::raster::serialize_empty_owned_map"))]`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-dangling-serde-oracle-route was falsely accepted.");
-  const restoredRasterOwnedMapSerializeLoop = `impl<V: serde::Serialize> serde::Serialize for RasterOwnedMap<V> {
-    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
-        use serde::ser::SerializeMap;
-        let mut map = serializer.serialize_map(Some(self.length))?;
-        for (key, value) in self {
-            map.serialize_entry(key, value)?;
-        }
-        map.end()
-    }
-}`;
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\n${restoredRasterOwnedMapSerializeLoop}`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-public-populated-serde-loop-bound-restoration was falsely accepted.");
-  const restoredRasterEncAssetMapLoop = `pub(crate) fn enc_asset_map(map: &RasterOwnedMap<RasterAssetChild>) -> String {
-    format!("[{}]", map.iter().map(|(k, v)| format!("[{},{}]", enc_str(k), enc_child(v))).collect::<Vec<_>>().join(","))
-}`;
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\n${restoredRasterEncAssetMapLoop}`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-snapshot-text-asset-map-whole-loop-restoration was falsely accepted.");
-  const restoredRasterEncParamsLoop = `pub(crate) fn enc_params(params: &RasterOwnedMap<dsl::DslValue>) -> String {
-    format!("[{}]", params.iter().map(|(k, v)| format!("[{},{}]", enc_str(k), hex_encode(&serde_json::to_vec(v).unwrap_or_default()))).collect::<Vec<_>>().join(","))
-}`;
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\n${restoredRasterEncParamsLoop}`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-snapshot-text-parameter-map-whole-loop-restoration was falsely accepted.");
-  const restoredRasterWriteAssetMapLoop = `fn write_asset_map(out: &mut Vec<u8>, map: &RasterOwnedMap<RasterAssetChild>) {
-    store::pack_rt::write_varint_u64(out, map.len() as u64);
-    for (k, v) in map {
-        write_str_lp(out, k);
-        write_child(out, v);
-    }
-}`;
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\n${restoredRasterWriteAssetMapLoop}`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-snapshot-pack-asset-map-whole-loop-restoration was falsely accepted.");
-  const restoredRasterWriteParamsLoop = `fn write_params(out: &mut Vec<u8>, params: &RasterOwnedMap<dsl::DslValue>) {
-    store::pack_rt::write_varint_u64(out, params.len() as u64);
-    for (k, v) in params {
-        write_str_lp(out, k);
-        write_bytes_lp(out, &serde_json::to_vec(v).unwrap_or_default());
-    }
-}`;
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\n${restoredRasterWriteParamsLoop}`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-snapshot-pack-parameter-map-whole-loop-restoration was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("semio_s_artifact_stdio_png::io::encode_png(&target)", "Ok(<RasterSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(snapshot).into_bytes())"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-mounted-exporter-DSL-print-under-foreign-extension-restoration was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("let image = raster_composite_image(snapshot)?;", "let image = SemioImage::default();"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-mounted-exporter-document-composite-bypass was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("crate::artifacts::raster::io::raster_document_json_to_svg(snapshot)?", 'Ok::<_, String>((String::from("<svg/>"), 0u32, 0u32))?'), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-mounted-vector-exporter-composite-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("Err(RASTER_PDF_EXPORT_UNSUPPORTED.to_string())", "Ok(Vec::new())"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-mounted-exporter-silent-empty-output-instead-of-typed-decline was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("pub const RASTER_DWG_EXPORT_UNSUPPORTED: &str =", "const DWG_UNSUPPORTED: &str ="), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-mounted-exporter-decline-reason-constant-erasure was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> {", "fn unmounted_serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> {"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-mounted-exporter-entry-point-unmounting was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("self.require_empty_output_shell().expect(RASTER_POPULATED_OUTPUT_ERROR);", "let _ = self;"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-public-DSL-output-preflight-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("s.require_empty_output_shell().expect(RASTER_POPULATED_OUTPUT_ERROR);\n", ""), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-snapshot-body-output-preflight-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("self.require_empty_output_shell().map_err(|error| store::PackError::Schema(error.to_owned()))?;", "let _ = self;"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-public-pack-output-preflight-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_populated_snapshot_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact", "raster_empty_snapshot_output_only"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-snapshot-output-hostile-fixture-removal was falsely accepted.");
-  const rasterRejectedParamValueIdentity = 'assert_eq!(rejected_param_value.as_ptr(), plus_one_param_value_pointer, "rejected output parameter returns the exact value allocation");';
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace(rasterRejectedParamValueIdentity, ""), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-rejected-output-parameter-value-identity-removal was falsely accepted.");
-  if (
-    toolJobRasterEnvelopeCallerRetainedExact(
-      retainedJackStore,
-      retainedRasterCodec.replace(rasterRejectedParamValueIdentity, 'assert_eq!(rejected_param.key.as_ptr(), plus_one_param_pointer, "rejected output parameter returns only the key allocation");'),
-      retainedRasterEditor,
-      retainedWriterPlugin,
-    )
-  )
-    throw new Error("[verify interactivity tool-jobs] self-test Raster-rejected-output-parameter-value-identity-substitution was falsely accepted.");
-  const rasterRejectedParamMovedInsert = "params.insert(plus_one_param_key, dsl::DslValue::String(plus_one_param_value))";
-  const rasterRejectedParamValueBinding = "let rejected_param_value = match &rejected_param.value";
-  const rasterParamBindingBeforeInsertion = retainedRasterCodec
-    .replace(rasterRejectedParamValueBinding, "")
-    .replace(rasterRejectedParamMovedInsert, `${rasterRejectedParamValueBinding}\n${rasterRejectedParamMovedInsert}`);
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, rasterParamBindingBeforeInsertion, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-rejected-output-parameter-binding-before-insertion was falsely accepted.");
-  const rasterRejectedParamRetirement = "RasterOwnedRetirement::new(RasterRetirementOwner::ValueEntry { key: rejected_param.key, value: Some(rejected_param.value) })";
-  const rasterParamRetirementBeforeAssertion = retainedRasterCodec
-    .replace(rasterRejectedParamRetirement, "")
-    .replace(rasterRejectedParamValueIdentity, `${rasterRejectedParamRetirement}\n${rasterRejectedParamValueIdentity}`);
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, rasterParamRetirementBeforeAssertion, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-rejected-output-parameter-retirement-before-identity was falsely accepted.");
-  const rasterRejectedAssetChildIdentity = 'assert_eq!(rejected_asset.value.child_id.as_ptr(), plus_one_asset_child_pointer, "rejected output asset returns the exact child allocation");';
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace(rasterRejectedAssetChildIdentity, ""), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-rejected-output-asset-child-identity-removal was falsely accepted.");
-  if (
-    toolJobRasterEnvelopeCallerRetainedExact(
-      retainedJackStore,
-      retainedRasterCodec.replace(rasterRejectedAssetChildIdentity, 'assert_eq!(rejected_asset.key.as_ptr(), plus_one_asset_pointer, "rejected output asset returns only the key allocation");'),
-      retainedRasterEditor,
-      retainedWriterPlugin,
-    )
-  )
-    throw new Error("[verify interactivity tool-jobs] self-test Raster-rejected-output-asset-child-identity-substitution was falsely accepted.");
-  const rasterRejectedAssetMovedInsert = "assets.insert(plus_one_asset_key, plus_one_asset_child)";
-  const rasterAssetBindingBeforeInsertion = retainedRasterCodec
-    .replace(rasterRejectedAssetChildIdentity, "")
-    .replace(rasterRejectedAssetMovedInsert, `${rasterRejectedAssetChildIdentity}\n${rasterRejectedAssetMovedInsert}`);
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, rasterAssetBindingBeforeInsertion, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-rejected-output-asset-binding-before-insertion was falsely accepted.");
-  const rasterRejectedAssetRetirement = "RasterOwnedRetirement::new(RasterRetirementOwner::AssetEntry { key: rejected_asset.key, child: Some(rejected_asset.value) })";
-  const rasterAssetRetirementBeforeAssertion = retainedRasterCodec
-    .replace(rasterRejectedAssetRetirement, "")
-    .replace(rasterRejectedAssetChildIdentity, `${rasterRejectedAssetRetirement}\n${rasterRejectedAssetChildIdentity}`);
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, rasterAssetRetirementBeforeAssertion, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-rejected-output-asset-retirement-before-identity was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("std::mem::size_of::<RasterRetirementFramePage>() <= RASTER_CONTROL_BACKING_BYTES", "true"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-control-page-size-proof-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_maximum_combined_layer_and_value_depth_retires_to_terminal", "raster_separate_depth_only"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-combined-depth-fixture-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replaceAll("RasterOwnedMapInsert::Replaced", "RasterOwnedMapInsert::DiscardedKey"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-replacement-key-owner-handback-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("replacement returns the exact displaced pair", "replacement drops incoming key"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-replacement-pointer-fixture-removal was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\nsource.assets.clone()`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-retained-map-whole-clone-reintroduction was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\nserde_json::to_value(&source.assets)\nsource.assets.to_value()`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-retained-map-serde-dsl-loop-reintroduction was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\ndrop(populated_map)`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-map-ordinary-drop-reintroduction was falsely accepted.");
-  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor, retainedWriterPlugin.replace("artifact store initialization job reached Drop before exact candidate handoff or terminal retained close", "artifact initializer job silently drops retained candidate"))) throw new Error("[verify interactivity tool-jobs] self-test Raster-public-initializer-drop-refusal-removal was falsely accepted.");
+  const rasterChecks = toolJobRasterSourceSelfTests() + toolJobRustProductionEvidenceSelfTests();
   const retainedDrawingCodec = [
     "pub struct DrawingEnvelopeOwnedFieldCatalog",
@@ -1306,212 +969,9 @@
     "const DRAWING_MAXIMUM_NESTED_ITEMS: usize = 4_096",
     "const DRAWING_MAXIMUM_LAYER_DEPTH: usize = 64",
-    "struct DrawingSnapshotBoundsAuthority",
-    "struct DrawingFixedOwnerCensus",
-    "slots: [DrawingOwnerCreditSlot; DRAWING_MAXIMUM_NESTED_ITEMS]",
-    "const DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY: usize = 16",
-    "const DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY: usize = 64",
-    "const DRAWING_MUTATION_ARENA_POOL_CAPACITY: usize = 4",
-    "struct DrawingMutationArenaOwner",
-    "struct DrawingMutationArenaPool {",
-    "DrawingMutationArenaProcessState::Building",
-    "#[cfg(test)]\n    fn try_new()",
-    "struct DrawingMutationArenaOwnerBuilder",
-    "struct DrawingMutationArenaPoolBootstrap",
-    "struct DrawingMutationArenaBootstrapJob",
-    "fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingMutationArenaBootstrapStep",
-    "DrawingMutationArenaProcessState::Inert",
-    "DrawingMutationArenaBootstrapAdmission::fixed()",
-    "DrawingMutationArenaBootstrapStep::Blocked",
-    "pub fn request_drawing_mutation_arena_pool() {",
-    "DRAWING_MUTATION_ARENA_BOOTSTRAP_REQUESTED.store(true",
-    "DrawingMutationArenaProcessState::Inert",
-    "}",
-    "pub fn drawing_mutation_arena_pool_fault",
-    "fn borrow_drawing_mutation_arena() {",
-    "request_drawing_mutation_arena_pool()",
-    "DrawingMutationArenaBorrowError::NotReady",
-    "}",
-    "//#endregion",
-    "DrawingStoreInitializationPhase::InitializeArena",
-    "DrawingStoreInitializationPhase::InitializeArena => {",
-    "match self.arena_bootstrap_job.step(cx)",
-    "if let Some(undo) = self.source_undo {",
-    "self.start_rebuild(source, undo.parent, None, Some(undo.index), DrawingContainerRebuildRole::CloseSourceUndo)?;",
-    "DrawingMutationArenaProcessTransition::Retire",
-    "drawing_mutation_arena_pool_fault",
-    "borrow_drawing_mutation_arena_from",
-    "drawing-store.mutation-arena-pool-saturated",
-    "drawing-store.mutation-arena-stale-generation",
-    "owner.terminal_is_empty()",
-    "arena_return_phase: u8",
-    "Ok(Some(false)) => return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })",
-    "reverse.try_reserve_exact(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY)",
-    "pages.try_reserve_exact(DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY)",
-    "page.try_reserve_exact(DRAWING_MUTATION_RETAINED_PAGE_BYTES)",
-    "total.checked_add(page.capacity())",
-    "fn write_overlay_string",
-    "target.capacity() < source.len()",
-    "struct DrawingMutationOverlayPatch",
-    "drawing-store.mutation-overlay-owner-changed",
-    "struct DrawingAssetBoundsCursor",
-    "assets.range::<str, _>",
-    "drawing-store.preflight-depth-capacity",
-    "drawing-store.preflight-item-capacity",
-    "drawing-store.preflight-byte-capacity",
-    "struct DrawingLayerCloneAuthority",
-    "self.bounds.step(source, cx)",
-    "layer.step(source.layers.get(self.index)",
-    "target.children.push(Self::skeleton(child)?)",
-    "struct DrawingStoreInitializationAuthority",
-    "struct DrawingSemanticDigestCredit",
-    "semantic: Option<semio_framework_hash::Sha256>",
-    'digest.observe(b"drawing.semantic.sha256")',
-    "struct DrawingFillDigestAuthority",
-    "struct DrawingStrokeDigestAuthority",
-    "struct DrawingPathSegmentDigestAuthority",
-    "struct DrawingLayerVariantDigestAuthority",
-    "struct DrawingLayerDigestAuthority",
-    "struct DrawingMutationDigestAuthority",
-    "struct DrawingMutationAggregateReservation",
-    "mutation_source_items",
-    "mutation_derived_items",
-    "duplicate_candidate_items",
-    "source_owner_bytes",
-    "derived_owner_bytes",
-    "DRAWING_MUTATION_AGGREGATE_ITEMS",
-    "DRAWING_MUTATION_AGGREGATE_BYTES",
-    "DRAWING_MUTATION_RETAINED_PAGE_ITEMS",
-    "DRAWING_MUTATION_RETAINED_PAGE_BYTES",
-    "drawing-store.mutation-aggregate-item-capacity",
-    "drawing-store.mutation-aggregate-byte-capacity",
-    "struct DrawingMutationCandidateAuthority",
-    "drawing-store.mutation-candidate-stale-authority",
-    "drawing-store.mutation-candidate-cancelled",
-    "struct DrawingDuplicateRewriteAuthority",
-    "material: [u8; DRAWING_DUPLICATE_MATERIAL_BYTES]",
-    "self.hash_cursor + DRAWING_OWNED_FIELD_BYTES",
-    'update(b"semio.drawing.duplicate-id.v1")',
-    "update(&(self.id_len as u64).to_be_bytes())",
-    "update(&(self.name_len as u64).to_be_bytes())",
-    "pending_name: std::mem::ManuallyDrop<Option<String>>",
-    "drawing-store.duplicate-destination-capacity",
-    "DrawingMutationCandidatePhase::PreflightSource",
-    "DrawingMutationCandidatePhase::PreflightMutation",
-    "struct DrawingContainerRebuildAuthority",
-    "const DRAWING_CONTAINER_REBUILD_MOVE_CAPACITY",
-    "enum DrawingContainerRebuildMove",
-    "rebuild.rollback_step()?",
-    "DrawingContainerRebuildRole::CloseSourceUndo",
-    "source_undo: Option<DrawingContainerSourceUndo>",
-    "source.capacity() < output_capacity",
-    'self.source.as_mut().ok_or("drawing-store.container-source")?.push(value)',
-    "DrawingStoreInitializationPhase::ValidateEditId",
-    "DrawingStoreInitializationPhase::ValidateEditMeta",
-    "DrawingStoreInitializationPhase::PrepareApplied",
-    "DrawingStoreInitializationPhase::PrepareRedo",
-    "impl semio_framework_plugin::ArtifactStoreInitializationAuthority<DrawingSnapshot, DrawingMutation> for DrawingStoreInitializationAuthority",
-    "DrawingStoreInitializationPhase::ValidateEdit { index }",
-    "self.edit_index.admit(&envelope.vcs.edits, index,",
-    "DrawingStoreInitializationPhase::SeedHistory",
-    "DrawingStoreInitializationPhase::BuildCandidate",
-    "ArtifactStore::from_initialized_runtime_with_owners",
-    "ArtifactStoreInitializationOwnerCatalog::try_new()",
-    "ArtifactStoreInitializationRuntime::new_with_owner_catalog",
-    "DrawingStoreInitializationPhase::MoveInitialOwner",
-    "self.generation.0.checked_add(1)",
-    "DrawingRetirementOwner::Layer(value)",
-    "DrawingRetirementOwner::Fill(fill)",
-    "DrawingRetirementOwner::Stroke(stroke)",
-    "DrawingRetirementOwner::Segments(values)",
-    "DrawingRetirementOwner::Stops(values)",
-    "DrawingRetirementOwner::Points(values)",
-    "DrawingRetirementOwner::AssetEntry",
-    "FillStyle::Solid { color }",
-    "FillStyle::LinearGradient { x1, y1, x2, y2, stops }",
-    "FillStyle::RadialGradient { cx: center_x, cy: center_y, r, stops }",
-    "stop.offset",
-    "stop.color",
-    "value.color[(self.phase - 1) as usize]",
-    "value.width",
-    "observe_owned_string(digest, 246, &value.cap",
-    "observe_owned_string(digest, 247, &value.join",
-    "value.dash.is_some()",
-    "base.visible",
-    "base.locked",
-    "base.opacity",
-    "observe_owned_string(digest, 106, &base.blend_mode",
-    "base.transform.rotation",
-    "observe_owned_string(digest, 340, &value.shape_kind",
-    "value.rect.is_some()",
-    "value.ellipse.is_some()",
-    "value.circle.is_some()",
-    "value.line.is_some()",
-    "value.polygon.is_some()",
-    "PathSegment::Move",
-    "PathSegment::Line",
-    "PathSegment::Quad",
-    "PathSegment::Cubic",
-    "PathSegment::Arc",
-    "PathSegment::Close",
-    "observe_owned_string(digest, 382, &value.content",
-    "observe_owned_string(digest, 390, &value.image_key",
-    "observe_owned_string(digest, 410, &value.operation",
-    "value.children.len()",
-    "observe_owned_string(digest, 420, &value.source_key",
-    "value.params.threshold",
-    "Self::variant(mutation)",
-    "value.visible",
-    "value.locked",
-    "value.opacity",
-    "observe_owned_string(digest, 3, &value.blend_mode",
-    "observe_owned_string(digest, 3, &value.new_name",
-    "value.transform.rotation",
-    "value.fill.as_ref()",
-    "value.stroke.as_ref()",
-    "observe_owned_string(digest, 3, &value.boolean_operation",
-    "value.params.simplify_epsilon",
-    "value.parent_id.is_some()",
-    "value.index",
-    "value.layer",
-    "SetLayerVisible(payload)",
-    "SetLayerLocked(payload)",
-    "SetLayerOpacity(payload)",
-    "SetLayerBlendMode(payload)",
-    "RenameLayer(payload)",
-    "UpdateLayerTransform(payload)",
-    "ReplaceLayerFill(payload)",
-    "ReplaceLayerStroke(payload)",
-    "SetLayerBooleanOperation(payload)",
-    "UpdateLayerTraceParams(payload)",
-    "CreateLayer(payload)",
-    "DuplicateLayer(payload)",
-    "DeleteLayer(payload)",
-    "ReorderLayer(payload)",
-    "retained_drawing_mutation_candidate_covers_all_fourteen_variants_and_returns_exact_owners",
-    "retained_drawing_depth_plus_one_and_hostile_fields_fault_then_close_terminal_empty",
-    "retained_drawing_container_false_terminal_saturation_and_interrupted_close_preserve_exact_owner",
-    "retained_drawing_schema_digest_distinguishes_every_nested_semantic_field",
-    "retained_drawing_aggregate_credit_admits_exact_4096_rejects_plus_one_with_owner_handback",
-    "retained_drawing_duplicate_hash_frames_domain_id_and_name_lengths_without_concatenation_collision",
-    "retained_drawing_process_arena_pool_cap_plus_one_returns_exact_slots_and_rejects_stale_aba",
-    "retained_drawing_duplicate_name_uses_preadmitted_page_and_returns_exact_rejection_owner",
-    "retained_drawing_cancel_stale_each_replay_candidate_container_stage_preserves_last_valid",
-    "retained_drawing_rebuild_fault_after_every_phase_rolls_back_exact_container_and_reuses_pool_slot",
-    "retained_drawing_reorder_fault_after_source_handoff_restores_exact_nested_fifo_and_pool_roots",
-    "retained_drawing_arena_bootstrap_failure_at_each_allocation_retires_one_exact_root_per_grant",
-    "retained_drawing_arena_bootstrap_failure_after_each_bundle_keeps_every_root_until_terminal_close",
-    "retained_drawing_arena_bootstrap_advances_one_allocation_per_turn_and_withholds_incomplete_pool",
-    "retained_drawing_arena_bootstrap_exact_cap_and_plus_one_rejection_preserve_every_owner_until_close",
-    "retained_drawing_arena_default_second_app_and_borrow_only_request_without_allocation",
-    "retained_drawing_arena_bootstrap_job_cancel_budget_contention_and_saturation_are_governed",
+    "struct DrawingNativeCloneAuthority",
+    "struct DrawingLayerCloneWorkAuthority",
+    "struct DrawingLayerLocator",
   ].join("\n");
   const retainedDrawingEditor = [
-    "drawing_document_store_initialization_job(envelope, operation, generation)",
-    "impl Default for DrawingPlayApp {",
-    "request_drawing_mutation_arena_pool()",
-    "}",
-    "impl ArtifactEditor for DrawingPlayApp",
-    "arena_boot_fault: Option<&'static str>",
-    "drawing_mutation_arena_pool_fault",
     "drawing_live_envelope_submit_recursive_clone_swap_displaced_store_and_exact_ack_succeed",
     "drawing_live_envelope_cancel_closes_retained_pages_without_publication",
@@ -1528,154 +988,47 @@
     "acknowledge_artifact_store_replacement(handle.runtime_handle())",
     "cancel_artifact_envelope_load(handle.runtime_handle())",
+    "drawing_live_envelope_replay_equals_the_central_apply_fold_of_every_forward",
+    "protocol::apply_diff_cold(outcome.diff(), &expected)",
   ].join("\n");
-  if (!toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test retained-Drawing-envelope-route was falsely rejected.");
-  const retainedDrawingProductionEditor = retainedDrawingEditor
-    .replace("drawing_live_envelope_submit_recursive_clone_swap_displaced_store_and_exact_ack_succeed", "")
-    .replace("drawing_live_envelope_cancel_closes_retained_pages_without_publication", "")
-    .replace("drawing_live_envelope_rejects_single_and_final_edit_id_plus_one_before_mutation_candidate", "")
-    .replace("drawing_live_initializer_candidate_container_commit_ack_cancel_stale_preserve_last_valid_and_exact_handle", "")
-    .replace('"forwards": [crate::artifacts::drawing::mutations::DrawingMutation::RenameLayer', "");
-  if (!toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingProductionEditor, retainedWriterPlugin, retainedDrawingEditor)) {
-    throw new Error("[verify interactivity tool-jobs] self-test production-stripped Drawing editor with raw cfg(test) law witnesses was falsely rejected.");
+  const retainedDrawingDefaultInitializer = "Ok(bounded_document_store_initialization_job(envelope, Self::DOCUMENT_SCHEMA, operation, generation, actor))";
+  const retainedDrawingPlugin = [retainedWriterPlugin, retainedDrawingDefaultInitializer].join("\n");
+  if (!toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor, retainedDrawingPlugin)) throw new Error("[verify interactivity tool-jobs] self-test retained-Drawing-envelope-route was falsely rejected.");
+  const secondDrawingApplyLanes = [
+    "struct DrawingStoreInitializationAuthority",
+    "struct DrawingMutationCandidateAuthority",
+    "struct DrawingContainerRebuildAuthority",
+    "struct DrawingDuplicateRewriteAuthority",
+    "struct DrawingMutationArenaPool",
+    "fn node_at_mut",
+    "fn container_mut",
+    "fn apply_selection_transform",
+    "fn start_rebuild(&mut self, source: &mut DrawingSnapshot)",
+    "snapshot_mut",
+    "operation.diff(",
+    "diff.apply(",
+    "source.assets.remove(",
+  ];
+  for (const lane of secondDrawingApplyLanes) {
+    if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, `${retainedDrawingCodec}\n${lane}`, retainedDrawingEditor, retainedDrawingPlugin)) throw new Error(`[verify interactivity tool-jobs] self-test Drawing-second-apply-lane-${lane} was falsely accepted.`);
   }
-  const retainedDrawingSemanticFields = [
-    "FillStyle::Solid { color }",
-    "FillStyle::LinearGradient { x1, y1, x2, y2, stops }",
-    "FillStyle::RadialGradient { cx: center_x, cy: center_y, r, stops }",
-    "stop.offset",
-    "stop.color",
-    "value.color[(self.phase - 1) as usize]",
-    "value.width",
-    "observe_owned_string(digest, 246, &value.cap",
-    "observe_owned_string(digest, 247, &value.join",
-    "value.dash.is_some()",
-    "base.visible",
-    "base.locked",
-    "base.opacity",
-    "observe_owned_string(digest, 106, &base.blend_mode",
-    "base.transform.rotation",
-    "observe_owned_string(digest, 340, &value.shape_kind",
-    "value.rect.is_some()",
-    "value.ellipse.is_some()",
-    "value.circle.is_some()",
-    "value.line.is_some()",
-    "value.polygon.is_some()",
-    "PathSegment::Move",
-    "PathSegment::Line",
-    "PathSegment::Quad",
-    "PathSegment::Cubic",
-    "PathSegment::Arc",
-    "PathSegment::Close",
-    "observe_owned_string(digest, 382, &value.content",
-    "observe_owned_string(digest, 390, &value.image_key",
-    "observe_owned_string(digest, 410, &value.operation",
-    "value.children.len()",
-    "observe_owned_string(digest, 420, &value.source_key",
-    "value.params.threshold",
-    "Self::variant(mutation)",
-    "value.visible",
-    "value.locked",
-    "value.opacity",
-    "observe_owned_string(digest, 3, &value.blend_mode",
-    "observe_owned_string(digest, 3, &value.new_name",
-    "value.transform.rotation",
-    "value.fill.as_ref()",
-    "value.stroke.as_ref()",
-    "observe_owned_string(digest, 3, &value.boolean_operation",
-    "value.params.simplify_epsilon",
-    "value.parent_id.is_some()",
-    "value.index",
-    "value.layer",
+  const ownedDrawingInitializerHooks = [
+    "fn build_document_store_initialization_job",
+    "drawing_document_store_initialization_job(envelope, operation, generation, actor)",
+    "request_drawing_mutation_arena_pool()",
+    "drawing_mutation_arena_pool_fault",
+    "arena_boot_fault: Option<&'static str>",
   ];
-  for (const field of retainedDrawingSemanticFields) {
-    if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace(field, "omitted-semantic-field"), retainedDrawingEditor, retainedWriterPlugin)) {
-      throw new Error(`[verify interactivity tool-jobs] self-test Drawing-semantic-digest-omission-${field} was falsely accepted.`);
-    }
+  for (const hook of ownedDrawingInitializerHooks) {
+    if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, `${retainedDrawingEditor}\n${hook}`, retainedDrawingPlugin)) throw new Error(`[verify interactivity tool-jobs] self-test Drawing-owned-initializer-hook-${hook} was falsely accepted.`);
+    if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, `${retainedDrawingCodec}\n${hook}`, retainedDrawingEditor, retainedDrawingPlugin)) throw new Error(`[verify interactivity tool-jobs] self-test Drawing-host-initializer-hook-${hook} was falsely accepted.`);
   }
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("semantic: Option<semio_framework_hash::Sha256>", "semantic: u64"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-non-collision-resistant-digest was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace('digest.observe(b"drawing.semantic.sha256")', "digest.observe(&field)"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-unsealed-semantic-digest was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("struct DrawingMutationAggregateReservation", "struct DrawingPerAllocationGuess"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-missing-aggregate-reservation was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("struct DrawingFixedOwnerCensus", "struct DrawingResizableOwnerGuess"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-fixed-owner-census-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("const DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY: usize = 16", "let overlay_pages = Vec::new()"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-fixed-overlay-page-arena-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("page.try_reserve_exact(DRAWING_MUTATION_RETAINED_PAGE_BYTES)", "page.reserve(source.len())"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-overlay-page-not-preadmitted was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("total.checked_add(page.capacity())", "total.checked_add(page.len())"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-overlay-allocator-capacity-not-reconciled was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("fn write_overlay_string", "fn clone_overlay_string"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-overlay-owner-not-moved was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("reverse.try_reserve_exact(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY)", "let reverse = Vec::new()"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-fixed-container-owner-catalog-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("struct DrawingMutationArenaPool {", "struct PerCandidateArenaAllocation {"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-process-arena-pool-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DrawingMutationArenaProcessState::Building", "DrawingMutationArenaOwner::try_new()?"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-arena-allocation-before-process-admission was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, `${retainedDrawingCodec}\nDrawingMutationArenaOwner::try_new()?;`, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-question-mark-partial-bootstrap-drop was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("#[cfg(test)]\n    fn try_new()", "fn try_new()"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-synchronous-bootstrap-loop-was-production-reachable was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("borrow_drawing_mutation_arena_from", "allocate_drawing_mutation_arena_per_candidate"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-candidate-did-not-borrow-fixed-arena was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("owner.terminal_is_empty()", "true"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-nonterminal-arena-return-was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("arena_return_phase: u8", "return_whole_arena_bundle: bool"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-arena-return-was-not-one-root-per-grant was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("drawing-store.mutation-arena-stale-generation", "accept-stale-arena-return"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-arena-ABA-generation-check-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, `${retainedDrawingCodec}\nbase.name.try_reserve_exact(suffix.len());`, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-post-admission-duplicate-name-reserve was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("pending_name: std::mem::ManuallyDrop<Option<String>>", "pending_name: String"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-duplicate-name-owner-not-retained was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("source.capacity() < output_capacity", "false"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-live-container-owner-capacity-not-preflighted was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace('self.source.as_mut().ok_or("drawing-store.container-source")?.push(value)', 'self.output.as_mut().unwrap().push(value)'), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-container-scratch-owner-published-instead-of-returned was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_process_arena_pool_cap_plus_one_returns_exact_slots_and_rejects_stale_aba", "drawing-string-only-pool-predicate"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-live-process-pool-handback-fixture-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_duplicate_name_uses_preadmitted_page_and_returns_exact_rejection_owner", "drawing-string-only-name-predicate"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-live-duplicate-name-owner-fixture-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor.replace("request_drawing_mutation_arena_pool()", "initialize_drawing_mutation_arena_pool()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-app-default-advanced-bootstrap was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DRAWING_MUTATION_ARENA_BOOTSTRAP_REQUESTED.store(true", "Vec::<u8>::new().try_reserve_exact(64); DRAWING_MUTATION_ARENA_BOOTSTRAP_REQUESTED.store(true"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-request-path-allocated-before-governed-turn was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("fn borrow_drawing_mutation_arena() {\nrequest_drawing_mutation_arena_pool()", "fn borrow_drawing_mutation_arena() {\nbootstrap.step(cx); request_drawing_mutation_arena_pool()"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-borrow-path-advanced-bootstrap was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("struct DrawingMutationOverlayPatch", "struct DrawingWholeCandidateRebuild"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-retained-overlay-patch-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("mutation_derived_items", "semantic_totals_times_copies"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-derived-owner-credit-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, `${retainedDrawingCodec}\nfn exact_for_test() {}`, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-fabricated-boundary-authority was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, `${retainedDrawingCodec}\nsource.assets.iter().nth(index)`, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-asset-rescan-cursor was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore.replace("id_digest: [u8; 32]", "id: String"), retainedDrawingCodec, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-shared-history-id-clone-owner was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("struct DrawingDuplicateRewriteAuthority", "fn rewrite_duplicate_in_one_step"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-staged-duplicate-authority-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace('update(b"semio.drawing.duplicate-id.v1")', "update(&material_len.to_be_bytes())"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-duplicate-domain-frame-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("update(&(self.id_len as u64).to_be_bytes())", "update(&(self.material_len as u64).to_be_bytes())"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-duplicate-id-length-frame-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("update(&(self.name_len as u64).to_be_bytes())", "update(&[])"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-duplicate-name-length-frame-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("drawing-store.mutation-candidate-cancelled", "close-only-cancel"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-live-candidate-cancellation-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("drawing-store.mutation-aggregate-item-capacity", "unchecked-aggregate-items"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-aggregate-item-plus-one was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("drawing-store.mutation-aggregate-byte-capacity", "unchecked-aggregate-bytes"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-aggregate-byte-plus-one was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DRAWING_MUTATION_RETAINED_PAGE_ITEMS", "uncredited-retained-pages"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-retained-page-item-credit-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DRAWING_MUTATION_RETAINED_PAGE_BYTES", "uncredited-retained-page-bytes"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-retained-page-byte-credit-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DrawingMutationCandidatePhase::PreflightSource", "DrawingMutationCandidatePhase::Clone"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-clone-before-source-preflight was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DrawingMutationCandidatePhase::PreflightMutation", "DrawingMutationCandidatePhase::Clone"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-clone-before-mutation-preflight was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DrawingStoreInitializationPhase::ValidateEditMeta", "mutation_meta.iter().any"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-whole-metadata-scan was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DrawingStoreInitializationPhase::PrepareApplied", "DrawingStoreInitializationPhase::CommitApplied"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-applied-id-actor-multi-clone-grant was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DrawingStoreInitializationPhase::PrepareRedo", "DrawingStoreInitializationPhase::CommitRedo"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-redo-id-multi-clone-grant was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_schema_digest_distinguishes_every_nested_semantic_field", "drawing-string-name-predicate"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-semantic-authority-fixture-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_aggregate_credit_admits_exact_4096_rejects_plus_one_with_owner_handback", "drawing-aggregate-string-predicate"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-real-aggregate-saturation-fixture-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_duplicate_hash_frames_domain_id_and_name_lengths_without_concatenation_collision", "drawing-duplicate-name-only-predicate"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-duplicate-split-boundary-fixture-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_cancel_stale_each_replay_candidate_container_stage_preserves_last_valid", "drawing-only-cancels-before-start"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-stage-cancel-stale-fixture-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("rebuild.rollback_step()?", "rebuild.close_forward_step()?"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-active-rebuild-close-did-not-rollback was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("self.start_rebuild(source, undo.parent, None, Some(undo.index), DrawingContainerRebuildRole::CloseSourceUndo)?;", "self.start_rebuild(source, undo.parent, None, Some(undo.index), DrawingContainerRebuildRole::Destination)?;"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-live-source-undo-close-authority-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_rebuild_fault_after_every_phase_rolls_back_exact_container_and_reuses_pool_slot", "drawing-only-cancels-before-owner-move"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-every-rebuild-move-fault-fixture-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_reorder_fault_after_source_handoff_restores_exact_nested_fifo_and_pool_roots", "drawing-does-not-restore-source-handoff"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-reorder-source-undo-fixture-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("struct DrawingMutationArenaOwnerBuilder", "fn try_new_arena_owner()"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-retained-bootstrap-owner-builder-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("struct DrawingMutationArenaPoolBootstrap", "fn build_pool_with_question_mark()"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-retained-bootstrap-pool-owner-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("match self.arena_bootstrap_job.step(cx)", "match Ok::<bool, &'static str>(true)"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-live-bootstrap-not-driven-by-governed-store-turn was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_arena_bootstrap_failure_at_each_allocation_retires_one_exact_root_per_grant", "drawing-bootstrap-only-tests-success"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-bootstrap-allocation-fault-matrix-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_arena_bootstrap_failure_after_each_bundle_keeps_every_root_until_terminal_close", "drawing-bootstrap-drops-completed-bundles"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-bootstrap-bundle-fault-matrix-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_arena_bootstrap_advances_one_allocation_per_turn_and_withholds_incomplete_pool", "drawing-bootstrap-loops-to-completion"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-bootstrap-governed-progress-fixture-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_arena_bootstrap_exact_cap_and_plus_one_rejection_preserve_every_owner_until_close", "drawing-bootstrap-skips-aggregate-plus-one"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-bootstrap-aggregate-rejection-fixture-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor.replace("drawing_mutation_arena_pool_fault", "ignore_arena_boot_fault"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-app-bootstrap-fault-was-not-surfaced was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_arena_default_second_app_and_borrow_only_request_without_allocation", "drawing-default-allocates-on-second-app"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-default-borrow-no-allocation-fixture-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_arena_bootstrap_job_cancel_budget_contention_and_saturation_are_governed", "drawing-bootstrap-ignores-governed-cancellation"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-governed-bootstrap-adversarial-fixture-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, `${retainedDrawingCodec}\nmutation_meta.iter().any(|meta| true)`, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-resurrected-whole-metadata-scan was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("artifact_owned_spr_edit_history_decoder", "artifact_bounded_history_entry_decoder"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-domain-only-whole-edit-decoder was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("const DRAWING_MAXIMUM_LAYER_DEPTH: usize = 64", "let depth = Vec::new()"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-resizable-recursion-stack was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("drawing-store.preflight-byte-capacity", "unchecked_nested_bytes"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-nested-byte-preflight-hole was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("target.children.push(Self::skeleton(child)?)", "target.children.push(child.clone())"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-whole-recursive-layer-clone was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DrawingRetirementOwner::AssetEntry", "drop(asset)"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-asset-owner-drop was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("ReorderLayer(payload)", "drop(payload)"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-mutation-catalog-hole was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, `${retainedDrawingCodec}\noperation.encode_op()`, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-whole-operation-encode-before-credit was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, `${retainedDrawingCodec}\nlet next = snapshot.clone();`, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-whole-snapshot-clone-replay was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, `${retainedDrawingCodec}\nserde_json::from_value(value)`, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-serde-mutation-reconstruction was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DrawingStoreInitializationPhase::ValidateEditId", "skip_final_edit_id"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-single-final-edit-id-preflight-hole was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("ArtifactStoreInitializationOwnerCatalog::try_new()", "ArtifactStoreInitializationRuntime::new()"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-store-owner-catalog-not-preadmitted was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DrawingStoreInitializationPhase::MoveInitialOwner", "DrawingStoreInitializationPhase::CloneInitial"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-initial-owner-clone-resurrected was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("struct DrawingMutationCandidateAuthority", "struct SynchronousDrawDiffApply"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-retained-mutation-candidate-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_container_false_terminal_saturation_and_interrupted_close_preserve_exact_owner", "drawing_string_predicate_only"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-authority-close-fixture-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor.replace("drawing_live_envelope_rejects_single_and_final_edit_id_plus_one_before_mutation_candidate", "drawing_skips_final_edit_id"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-live-final-edit-id-fixture-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor.replace("drawing_live_initializer_candidate_container_commit_ack_cancel_stale_preserve_last_valid_and_exact_handle", "drawing-close-only-after-success"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-live-staged-cancel-stale-ack-fixture-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor.replace('"forwards": [crate::artifacts::drawing::mutations::DrawingMutation::RenameLayer', '"forwards": []'), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-live-populated-history-route-missing was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor.replace("source: &js_sys::Uint8Array", "source: &[u8]"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-post-lift-dynamic-page was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor.replace("acknowledge_artifact_store_replacement(handle.runtime_handle())", "return Ready"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-completion-without-exact-ack was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, `${retainedDrawingEditor}\nenvelope_json: Option<String>`, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-whole-buffer-constructor-bypass was falsely accepted.");
-  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor, retainedWriterPlugin.replace("self.jobs.insert_admitted(self.operation.0, ActiveArtifactStoreReplacement::new(self.operation, self.generation, job))", "drop(job)"))) throw new Error("[verify interactivity tool-jobs] self-test Drawing-false-terminal-initializer-drop was falsely accepted.");
+  for (const anchor of retainedDrawingCodec.split("\n")) {
+    if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace(anchor, "omitted-host-anchor"), retainedDrawingEditor, retainedDrawingPlugin)) throw new Error(`[verify interactivity tool-jobs] self-test Drawing-host-anchor-omission-${anchor} was falsely accepted.`);
+  }
+  for (const law of retainedDrawingEditor.split("\n")) {
+    if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor.replace(law, "omitted-law"), retainedDrawingPlugin)) throw new Error(`[verify interactivity tool-jobs] self-test Drawing-law-omission-${law} was falsely accepted.`);
+  }
+  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-without-framework-default-initializer was falsely accepted.");
+  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, `${retainedDrawingEditor}\nenvelope_json: Option<String>`, retainedDrawingPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-whole-string-envelope-ingress was falsely accepted.");
   const synchronousPeerRoster = "struct PeerPresenceRoot; struct VcsArtifactApp { peer_presence: Arc<PeerPresenceRoot> } impl PluginApp for VcsArtifactApp { async fn adopt_presence(&mut self, peers: &[PresencePeer]) { self.peer_presence = Arc::new(PeerPresenceRoot::from_peers(peers)); } } async fn dispatch_typed_command_inner() { let presence_peers = self.presence_store.peers().await.into_iter().map(|(actor, presence)| (actor.to_string(), presence.clone())).collect(); } let mut decoded: Vec<protocol::PresencePeer> = Vec::with_capacity(peers.len());";
   if (toolJobPeerInteractionRootsExact(synchronousPeerRoster, synchronousPeerRoster, synchronousPeerRoster)) throw new Error("[verify interactivity tool-jobs] self-test synchronous-whole-peer-roster-publication was falsely accepted.");
@@ -1855,4 +1208,4 @@
   const scalarConfig = toolJobScalarConfigCohortSelfTests();
   const sealer = storeCanonicalEditSealerSelfTests();
-  return fixtures.length + 394 + memberProducerChecks + checkpointChecks + toolJobOwnerFactoryResolutionSelfTests() + toolJobFactoryProofJoinSelfTests() + toolJobLatestWinsSelfTests() + toolJobMicrosecondBudgetSelfTests() + toolJobTelemetryContentionSelfTests() + toolJobCooperativeMaintenanceSelfTests() + proceduralGenerationRootSelfTests() + flowTypedRetirementSelfTests() + flowSelectedCopySelfTests() + scalarConfig.routes + scalarConfig.mutationOracles + scalarConfig.hostileCases + sealer.grants + sealer.editHostiles + sealer.sourceHostiles + sealer.digestOracles + sealer.mapGrants + sealer.mapEditHostiles + sealer.mapSourceHostiles + sealer.mapDigestOracles + sealer.readerChecks;
-}
+  return contractChecks + fundedBatchChecks + fixtures.length + (398 - 102) + rasterChecks + checkpointChecks + toolJobOwnerFactoryResolutionSelfTests() + toolJobFactoryProofJoinSelfTests() + toolJobLatestWinsSelfTests() + toolJobMicrosecondBudgetSelfTests() + toolJobTelemetryContentionSelfTests() + toolJobCooperativeMaintenanceSelfTests() + proceduralGenerationRootSelfTests() + flowTypedRetirementSelfTests() + flowSelectedCopySelfTests() + scalarConfig.routes + scalarConfig.mutationOracles + scalarConfig.hostileCases + sealer.grants + sealer.editHostiles + sealer.sourceHostiles + sealer.digestOracles + sealer.mapGrants + sealer.mapEditHostiles + sealer.mapSourceHostiles + sealer.mapDigestOracles + sealer.readerChecks;
+}


### 39 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts
B/H/S/current: 287/290/335/335
--- 
+++ 
@@ -35,5 +35,10 @@
   if (Buffer.byteLength(integration.keyRetirement.text, "utf8") !== integration.keyRetirement.utf8Bytes
     || JSON.stringify(Array.from(integration.keyRetirement.text as string).reverse().map((scalar) => Buffer.byteLength(scalar, "utf8"))) !== JSON.stringify(integration.keyRetirement.scalarBytes)) throw new Error("latest-wins UTF-8 retirement byte oracle");
-  const source = readFileSync(join(base, "🦀️.rs"), "utf8");
+  const owners = ["🦀️.rs", "🧵️instance/🦀️.rs", "♻️cancellation/🦀️.rs", "🧵️retained-command/🔄️full-operation/🦀️.rs", "📜️declaration/📨️emission/🦀️.rs", "📜️declaration/👷️job/🦀️.rs"] as const;
+  const rootSource = readFileSync(join(base, "🦀️.rs"), "utf8");
+  for (const owner of owners.slice(1)) if (!rootSource.includes(`#[path = "${owner}"]`)) throw new Error(`latest-wins owner is not mounted: ${owner}`);
+  const instance = readFileSync(join(base, "🧵️instance/🦀️.rs"), "utf8");
+  if (!instance.includes('#[path = "../🪜️maintenance-ladder/🦀️.rs"]')) throw new Error("latest-wins maintenance owner is not mounted");
+  const source = [...owners, "🪜️maintenance-ladder/🦀️.rs", "👷️job/🚪️session/🦀️.rs"].map(owner => readFileSync(join(base, owner), "utf8")).join("\n");
   const runtimeContractTests = readFileSync(join(base, "🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"), "utf8");
   const body = (text: string, name: string): string => {
@@ -44,4 +49,7 @@
     return start < 0 ? "" : toolJobRustBlock(scope, scope.indexOf("{", start))?.body ?? "";
   };
+  const claimSource = readFileSync(join(WORKSPACE_ROOT, "🧰️framework/🔨️modules/⏳️async/♻️cancel-return/📬️claim/🦀️.rs"), "utf8");
+  const claimIdentityExact = (text: string): boolean => body(text, "same_original").replace(/\s+/gu, "") === "Arc::ptr_eq(&left.inner,&right.inner)" && body(text, "try_claim").replace(/\s+/gu, "").includes("self.inner.state.compare_exchange(0,1,Ordering::AcqRel,Ordering::Acquire).ok()?");
+  if (!claimIdentityExact(claimSource) || claimIdentityExact(claimSource.replace("Arc::ptr_eq(&left.inner,&right.inner)", "true")) || claimIdentityExact(claimSource.replace("compare_exchange(0,1,Ordering::AcqRel,Ordering::Acquire).ok()?", "store(1,Ordering::Release)"))) throw new Error("publication claim identity accepts a different original issuer");
   const obligations: Array<[string, string]> = [
     ["publish_mounted_typed_operation_unit", "mounted.reject_cancelled_publication()?"],
@@ -90,5 +98,5 @@
     ["has_runnable_typed_operations", "!self.tool_operations.is_empty()"],
     ["cleanup_finished_slot", "scope.publication_claim.is_finished()"],
-    ["release_current", "std::sync::Arc::ptr_eq(&scope.publication_claim, &self.publication_claim)"],
+    ["release_current", "PublicationClaimHandle::same_original(&scope.publication_claim, &self.publication_claim)"],
     ["try_claim_publication", "self.handle.publication_scope.try_claim()?"],
     ["try_claim_publication", "self.publication_claim.try_claim()?"],
@@ -101,5 +109,5 @@
     && !body(text, "publish_mounted_typed_operation_unit").includes("dispatch_emit_group(")
     && text.includes("self.token.child_now()")
-    && text.includes("compare_exchange(0, 1, std::sync::atomic::Ordering::AcqRel")
+    && claimIdentityExact(claimSource)
     && text.includes("scope.operation")
     && ["retained_latest_wins_real_document_publication_cancellation_and_delayed_ack_close", "a_mounted_typed_operation_never_parks_a_turn_that_reports_no_runnable_work", "a_status_only_host_call_finishes_every_typed_operation_it_admitted"].every((name) => runtimeContractTests.includes(`async fn ${name}()`))
@@ -132,6 +140,6 @@
       && !handles.includes("pub async fn new(registry:") && handles.includes("pub fn new(registry:")
       && !admission.includes(".await") && !admission.includes("run(ctx)")
-      && execution.includes("let future = run.run(ctx);") && execution.includes("future.await")
-      && ordered(admission, ["instance_task_quota(instance)", "if live >= quota", "task.into_parts()", "executor.reserve()", "insert_admitted(task_id"]);
+      && execution.includes("let future = cold_run.take().run(ctx);") && execution.includes("future.await")
+      && ordered(admission, ["instance_task_quota(instance)", "if live >= quota", "task.into_parts()", "executor.reserve()", "insert_admitted(task_id", "let mut cold_run = crate::app_emission::ColdTaskRun::new(run)"]);
   };
   if (!taskAdmissionExact(source, reactor, host)) throw new Error("task publication invokes or polls user work before releasing its synchronous admission claim");
@@ -145,6 +153,7 @@
   const rawFixture = JSON.parse(readFileSync(join(base, "🧵️retained-command/🧪️tests/🧫️fixtures/🚪️raw-allocation-close.json"), "utf8"));
   for (const law of rawFixture.cases) {
-    const oracle = Buffer.alloc(law.capacity).subarray(0, law.initializedBytes);
-    if (law.initializedBytes > law.capacity || oracle.byteLength !== law.expectedCopiedBytes || oracle.buffer.byteLength !== law.expectedReleasedBytes) throw new Error(`retained raw allocation copy and physical release oracle: ${law.id}`);
+    const allocation = new ArrayBuffer(law.capacity);
+    const oracle = Buffer.from(allocation).subarray(0, law.initializedBytes);
+    if (law.initializedBytes > law.capacity || oracle.byteLength !== law.initializedBytes || oracle.buffer !== allocation || law.expectedCopiedBytes !== 0 || rawFixture.maximumCopyBytes !== 0 || oracle.buffer.byteLength !== law.expectedReleasedBytes) throw new Error(`retained raw allocation copy and physical release oracle: ${law.id}`);
   }
   const rawSource = readFileSync(join(base, "🧵️retained-command/🦀️.rs"), "utf8");
@@ -191,10 +200,33 @@
   if (JSON.stringify(childCloseFixture.children.map((child: { id: string }) => child.id).reverse()) !== JSON.stringify(childCloseFixture.expectedRetirementOrder)) throw new Error("retained child close LIFO oracle diverged");
   if (childCloseFixture.children.some((child: { slot: string; childId: string; value: string }) => [child.slot, child.childId, child.value].some(value => Buffer.byteLength(value, "utf8") <= value.length))) throw new Error("retained child close fixture lost its multibyte scalar oracle");
+  const completionHandbackExact = (text: string): boolean => {
+    const owner = text.indexOf("impl<A: ArtifactApp> InteractiveJob for ArtifactRetainedCommandJob<A>");
+    const start = owner < 0 ? -1 : text.indexOf("fn step", owner);
+    const publication = start < 0 ? "" : toolJobRustBlock(text, text.indexOf("{", start))?.body.replace(/\s+/gu, "") ?? "";
+    return ["Emit", "Download"].every(lane => new RegExp("self\\.rejected_completion=Some\\(ArtifactToolCompletionValue::" + lane + "\\(rejected\\." + lane.toLowerCase() + "\\.map_err\\((?:crate::completion_fault::)?ArtifactCompletionFault::new\\),rejected\\.ephemeral\\)\\);self\\.reducer_fault\\(rejected\\.fault\\);returnOk\\(None\\);").test(publication));
+  };
+  const rejectedCloseExact = (text: string): boolean => {
+    const close = rawCloseBlock(text)?.body;
+    const start = close?.indexOf("if self.rejected_completion.is_some()") ?? -1;
+    const admission = close && start >= 0 ? toolJobRustBlock(close, close.indexOf("{", start))?.body.replace(/\s+/gu, "") ?? "" : "";
+    return admission.includes("matchA::admit_completion_retirement(&mutself.rejected_completion,grant)")
+      && admission.includes("Ok(Some((owner,progress)))=>{self.rejected_retirement=Some(owner);InteractiveJobCloseStep::Pending{progress}}")
+      && admission.includes("Err(error)=>InteractiveJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()}");
+  };
+  const rejectedDepthExact = (text: string): boolean => {
+    const start = text.indexOf("if let Some(value)=self.rejected_completion.as_ref()");
+    const demand = start < 0 ? "" : toolJobRustBlock(text, text.indexOf("{", start))?.body.replace(/\s+/gu, "") ?? "";
+    return demand.includes("emit.child_close_demands(copy)?") && demand.includes('demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"retained rejected emitted child depth overflow"))?;returnOk(demand);'.replace(/\s+/gu, ""));
+  };
   const childCloseExact = (main: string, retained: string): boolean => main.includes("pub(crate) fn close_one(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>")
     && main.includes("pub fn close_child_one(&mut self, grant: RetainedCloneGrant) -> Result<Option<PluginLifecycleStep>, Fault>")
-    && retained.includes("emit.close_child_one(child)")
+    && rawCloseBlock(retained)?.body.split("emit.close_child_one(child)").length === 3
     && retained.includes("InteractiveJobCloseStep::Pending{progress}")
-    && retained.includes("self.emit = rejected.emit.ok()")
-    && retained.includes("self.ephemeral = Some(rejected.ephemeral)")
+    && rawCloseBlock(retained)?.body.split("if progress.fits(child)").length === 3
+    && completionHandbackExact(retained)
+    && rejectedCloseExact(retained)
+    && rejectedDepthExact(retained)
+    && retained.includes("artifact_retirement_box_close_step(&mut self.rejected_retirement,grant)")
+    && retained.includes("self.rejected_completion.is_none()") && retained.includes("self.rejected_retirement.is_none()")
     && main.split("return Err(ArtifactToolCompletionRejection { emit, ephemeral, fault:").length - 1 === 2
     && main.includes("mounted owned child group requires its one exact admitted typed source family");
@@ -203,5 +235,14 @@
     [source.replace("pub(crate) fn close_one(&mut self, grant: RetainedCloneGrant)", "fn close_one(&mut self, grant: RetainedCloneGrant)"), rawSource],
     [source, rawSource.replace("emit.close_child_one(child)", "drop(self.emit.take())")],
-    [source, rawSource.replace("self.emit = rejected.emit.ok()", "drop(rejected.emit)")],
+    [source, rawSource.replace("if progress.fits(child)", "if true")],
+    [source, rawSource.replace("rejected.emit.map_err", "discarded_emit.map_err")],
+    [source, rawSource.replace("rejected.download.map_err", "discarded_download.map_err")],
+    [source, rawSource.replace("rejected.ephemeral", "discarded_ephemeral")],
+    [source, rawSource.replace("A::admit_completion_retirement(&mut self.rejected_completion,grant)", "A::admit_completion_retirement(&mut discarded_completion,grant)")],
+    [source, rawSource.replace("self.rejected_retirement=Some(owner)", "drop(owner)")],
+    [source, rawSource.replace("self.rejected_retirement=Some(owner);InteractiveJobCloseStep::Pending{progress}", "self.rejected_retirement=Some(owner);InteractiveJobCloseStep::Pending{progress:Default::default()}")],
+    [source, mutateRawClose("Err(error)=>InteractiveJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()}", "Err(error)=>InteractiveJobCloseStep::Refused{kind:error.kind,progress:Default::default()}")],
+    [source, rawSource.replace('demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"retained rejected emitted child depth overflow"))?', "demand.depth+=1")],
+    [source, rawSource.replace("artifact_retirement_box_close_step(&mut self.rejected_retirement,grant)", "artifact_retirement_box_close_step(&mut self.rejected_retirement,Default::default())")],
     [source.replace("return Err(ArtifactToolCompletionRejection { emit, ephemeral, fault:", "return Err(ArtifactToolCompletionRejection { emit: discarded_emit, ephemeral, fault:"), rawSource],
     [source.replace("mounted owned child group requires its one exact admitted typed source family", "typed child output is accepted"), rawSource],
@@ -210,8 +251,12 @@
   for (const path of ["🦀️.rs", "🧩️composition/📨️emission/📦️preparation/🦀️.rs", "🧵️retained-command/🦀️.rs", "🧪️tests/🧩️composition/📨️emission/🦀️.rs"]) {
     const result = spawnSync("rustfmt", ["--edition", "2021", "--emit", "stdout", "--config", "skip_children=true", join(base, path)], { stdio: ["ignore", "ignore", "pipe"], timeout: 5000 });
-    if (result.status !== 0) throw new Error(`original child source grammar refused ${path}: ${result.stderr?.toString()}`);
+    if (result.status !== 0) throw new Error(`original child source grammar refused ${path}: ${result.error?.message ?? result.signal ?? result.status}; ${result.stderr?.toString()}`);
   }
   console.log("[DEBUG] Original ChildEmit full-grant/receipt source and rejected-owner hostiles passed; fixed caller Grant matched canonical Ajv and multibyte Buffer oracle; native System law remains unrun");
-  const storeSource = readFileSync(join(base, "../🏪️store/🦀️.rs"), "utf8");
+  const storeRoot = readFileSync(join(base, "../🏪️store/🦀️.rs"), "utf8");
+  if (!storeRoot.includes('#[path = "🎮️one-item/🦀️.rs"]')) throw new Error("latest-wins one-item store owner is not mounted");
+  const itemRoot = readFileSync(join(base, "../🏪️store/🎮️one-item/🦀️.rs"), "utf8");
+  if (!storeRoot.includes('#[path = "🎮️one-item/📬️publication/🦀️.rs"]')) throw new Error("latest-wins publication store owner is not mounted");
+  const storeSource = [storeRoot, itemRoot, readFileSync(join(base, "../🏪️store/🎮️one-item/📬️publication/🦀️.rs"), "utf8")].join("\n");
   const publisherStart = source.lastIndexOf("fn publish_mounted_typed_operation_unit(");
   const mutatePublisher = (before: string, after: string): string => source.slice(0, publisherStart) + source.slice(publisherStart).replace(before, after);
@@ -223,5 +268,5 @@
       return { text, lane, args, index: call.index } as const;
     });
-    return text.replace(/\s+/gu, "").includes("self.publish_mounted_typed_operation_unit(mounted,crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.preparation,retained))?") && calls.length > 0 && !publisher.includes(".await") && !publisher.includes(".last().cloned()") && Object.values(lanes).every(lane => calls.some(call => call.args.includes(`&mutemit.${lane},HistoryLane::Document`)) && !publisher.includes(`emit.${lane}.clone()`) && !publisher.includes(`emit.${lane}.clear()`) && !publisher.includes(`drop(std::mem::take(&mutemit.${lane}))`)) && calls.every(call => call.args.includes(`&mutemit.${lanes[call.lane]},HistoryLane::Document`) && call.args.endsWith("retained") && (toolJobRustBlock(publisher,publisher.indexOf("{",call.index+call.text.length))?.body??"").includes("mounted.publication_ownership_progress=Some((retained,ownership))"));
+    return text.replace(/\s+/gu, "").includes("self.publish_mounted_typed_operation_unit(mounted,crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.preparation,crate::plugin_runtime::remaining_plugin_turn_grant(turn,*spent)))?") && calls.length > 0 && !publisher.includes(".await") && !publisher.includes(".last().cloned()") && Object.values(lanes).every(lane => calls.some(call => call.args.includes(`&mutemit.${lane},HistoryLane::Document`)) && !publisher.includes(`emit.${lane}.clone()`) && !publisher.includes(`emit.${lane}.clear()`) && !publisher.includes(`drop(std::mem::take(&mutemit.${lane}))`)) && calls.every(call => call.args.includes(`&mutemit.${lanes[call.lane]},HistoryLane::Document`) && call.args.endsWith("retained") && (toolJobRustBlock(publisher,publisher.indexOf("{",call.index+call.text.length))?.body??"").includes("mounted.publication_ownership_progress=Some((retained,ownership))"));
   };
   const ingress = body(storeSource, "admit_apply_batch"), admission = ingress.indexOf("if grant.maximum_items==0"), transfer = ingress.indexOf("std::mem::take(mutations)");
@@ -232,8 +277,8 @@
     ["exact extracted setup", (text) => !!toolJobRetainedDispatchSetup(text), source.replace("self.start_typed_command_operation(command, admission, meta, operation_id, None).await", "self.unchecked_command_operation(command, admission, meta, operation_id, None).await")],
     ["unsupported generic reducer denial", toolJobTypedRouteFailsClosedBeforePreparation, source.replace("QualifiedToolProof::FrameworkOwned(_) | QualifiedToolProof::Bounded(_) => {", "QualifiedToolProof::FrameworkOwned(_) | QualifiedToolProof::Bounded(_) => { return Ok(());")],
-    ["mounted persistent operation", toolJobTypedPersistentFoundation, source.replace("session.pump_one(pool, semio_framework_async::Lane::Interactive)", "session.run_to_terminal(pool)")],
+    ["mounted persistent operation", toolJobTypedPersistentFoundation, source.replace("session.pump_one_for_interactive_turn(grant)", "session.run_to_terminal(pool)")],
     ["full maintenance eligibility scan", toolJobTypedPersistentFoundation, source.replace("for offset in 0..ARTIFACT_LIVE_OUTPUT_SLOTS", "for offset in 0..1")],
-    ["worker input-wait classification", toolJobTypedPersistentFoundation, source.replace('Ok(_) => Ok(PluginCloseStep::AwaitingInput { reason: "typed operation mounted worker awaits its next outcome" })', 'Ok(_) => Ok(PluginCloseStep::Blocked { reason: "typed operation mounted worker awaits its next outcome" })')],
-    ["transient scheduler-wait classification", toolJobTypedPersistentFoundation, source.replace('Ok(PluginCloseStep::AwaitingInput { reason: "typed operation mounted worker awaits transient scheduler authority" })', 'Ok(PluginCloseStep::Blocked { reason: "typed operation mounted worker awaits transient scheduler authority" })')],
+    ["worker input-wait classification", toolJobTypedPersistentFoundation, source.replace('Ok(_)=>Ok(PluginLifecycleStep::AwaitingInput{reason:"typed operation mounted worker awaits its next outcome"})', 'Ok(_) => Ok(PluginLifecycleStep::Blocked { reason: "typed operation mounted worker awaits its next outcome" })')],
+    ["transient scheduler-wait classification", toolJobTypedPersistentFoundation, source.replace('Ok(PluginLifecycleStep::AwaitingInput{reason:"typed operation mounted worker awaits transient scheduler authority"})', 'Ok(PluginLifecycleStep::Blocked { reason: "typed operation mounted worker awaits transient scheduler authority" })')],
     ["retained ephemeral publisher", (text) => toolJobEphemeralOneItemPublicationBounded(storeSource, text), source.replace("self.presence_one_item_factory.as_deref()", "A::build_presence_store_one_item_preparation_factory()")],
   ];
@@ -252,5 +297,8 @@
   }
   if (!toolJobStoreBatchPublicationBounded(storeSource, source)) throw new Error("mounted Store source binding lost its retained owned-preparation helper");
-  const replayingOwnedBegin = storeSource.replace("let footprint = match source.footprint(lane) {", "replay_mutations(); let footprint = match source.footprint(lane) {");
+  const advanceBatch = body(storeSource, "advance_apply_batch");
+  const itemGrant = "let item_grant = ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), ..grant };";
+  if (!advanceBatch.includes(itemGrant)) throw new Error("original owned batch advance grant probe is missing");
+  const replayingOwnedBegin = storeSource.replace(advanceBatch, advanceBatch.replace(itemGrant, `replay_mutations(); ${itemGrant}`));
   if (replayingOwnedBegin === storeSource || toolJobStoreBatchPublicationBounded(replayingOwnedBegin, source)) throw new Error("mounted Store source binding accepted replay inside extracted preparation");
   const dispatchFixture = JSON.parse(readFileSync(join(base, "🧵️retained-command/🧪️tests/🧫️fixtures/📌️mounted-dispatch-binding.json"), "utf8"));
@@ -271,9 +319,9 @@
     "duplicate-helper": (text) => `${text}\nasync fn start_typed_command_operation() {}`,
     "missing-pipeline-guard": (text) => mutateFunction(text, dispatchFixture.dispatcher, "self.require_complete_tool_operation_pipeline(&admission)?", "self.accept_incomplete_pipeline(&admission)?"),
-    "duplicate-session": (text) => mutateFunction(text, dispatchFixture.helper, "let (session, session_rejected) = match semio_framework_job::MountedWorkerJobSession::try_new", "semio_framework_job::MountedWorkerJobSession::try_new(extra, params); let (session, session_rejected) = match semio_framework_job::MountedWorkerJobSession::try_new"),
-    "duplicate-pump": (text) => mutateFunction(text, dispatchFixture.helper, "let _ = active.drive_worker_step(&pool,", "let _ = active.drive_worker_step(&pool, rejected_grant)?; let _ = active.drive_worker_step(&pool,"),
+    "duplicate-session": (text) => mutateFunction(text, dispatchFixture.helper, "let (session, session_rejected) = match crate::reserved_job_session::admit_mounted", "crate::reserved_job_session::admit_mounted(extra, params, grant); let (session, session_rejected) = match crate::reserved_job_session::admit_mounted"),
+    "duplicate-pump": (text) => mutateFunction(text, dispatchFixture.helper, "let _ = active.drive_worker_step(self.mounted_policy.maintenance)", "let _ = active.drive_worker_step(self.mounted_policy.maintenance)?; let _ = active.drive_worker_step(self.mounted_policy.maintenance)"),
     "direct-reducer": (text) => mutateFunction(text, dispatchFixture.dispatcher, "self.require_complete_tool_operation_pipeline(&admission)?", "A::handle(&command).await; self.require_complete_tool_operation_pipeline(&admission)?"),
     "direct-dispatch": (text) => mutateFunction(text, dispatchFixture.dispatcher, "self.require_complete_tool_operation_pipeline(&admission)?", "self.tool_jobs.dispatch(operation_spec); self.require_complete_tool_operation_pipeline(&admission)?"),
-    "run-to-completion": (text) => mutateFunction(text, dispatchFixture.helper, "let _ = active.drive_worker_step(&pool,", "let _ = active.run_to_completion(&pool)?; let _ = active.drive_worker_step(&pool,"),
+    "run-to-completion": (text) => mutateFunction(text, dispatchFixture.helper, "let _ = active.drive_worker_step(self.mounted_policy.maintenance)", "let _ = active.run_to_completion(&pool)?; let _ = active.drive_worker_step(self.mounted_policy.maintenance)"),
   };
   if (new Set(dispatchFixture.cases.map((law: { mutation: string }) => law.mutation)).size !== Object.keys(mutations).length) throw new Error("mounted dispatch fixture omits an exact hostile case");
@@ -283,5 +331,5 @@
     if (toolJobMountedDispatchOneTurnExact(changed) !== law.admitted) throw new Error(`mounted dispatch exact helper law: ${law.mutation}`);
   }
-  return fixture.cases.length + 3 + obligations.length + integration.cases.length + taskFixture.observations.length + taskHostiles.length + rawFixture.cases.length + 4 + rawHostiles.length + 3 + mountedChecks.length * 2 + 2 + dispatchFixture.cases.length * 2 + childCloseHostiles.length + 3;
+  return fixture.cases.length + 6 + obligations.length + integration.cases.length + taskFixture.observations.length + taskHostiles.length + rawFixture.cases.length + 4 + rawHostiles.length + 3 + mountedChecks.length * 2 + 2 + dispatchFixture.cases.length * 2 + childCloseHostiles.length + 3;
 }
 


### 40 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-run/🦀️.rs
B/H/S/current: 2382/701/2385/702
--- 
+++ 
@@ -137,5 +137,5 @@
 
 impl ToyRunJob {
-    fn retirement_demand<T:semio_framework_value::retirement::RetireOwned>(owner:&semio_framework_value::retirement::controlled::ControlledRetirement<T>,body:usize)->Result<RetirementDemand,ValueError>{if owner.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()})}Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?,..Default::default()})}
+    fn retirement_demand<T:semio_framework_value::retirement::RetireOwned>(owner:&semio_framework_value::retirement::controlled::ControlledRetirement<T>,body:usize)->Result<RetirementDemand,ValueError>{if owner.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()})}Ok(owner.next_demand(body)?)}
     fn auxiliary_demands(&self,body:usize)->Result<Option<RetirementDemand>,ValueError>{
      let child=if !self.unit.terminal_is_empty(){Some(self.unit.retirement_demands()?)}else if !self.emission.terminal_is_empty(){Some(self.emission.retirement_demands()?)}else if let Some(owner)=self.writer_retirement.as_ref(){Some(Self::retirement_demand(owner,body)?)}else if let Some(owner)=self.progress_retirement.as_ref(){Some(Self::retirement_demand(owner,body)?)}else if self.successor.is_some()||self.progress_source.is_some()||self.counter_source.is_some()||self.trace_source.is_some()||self.payload_source_ready||self.payload_position>0||self.retraction.is_some()||self.phase!=0{Some(RetirementDemand{depth:1,..Default::default()})}else{None};child.map(|mut demand|{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original Toy auxiliary depth overflow"))?;Ok(demand)}).transpose()
@@ -144,5 +144,5 @@
      use semio_framework_value::RetainedCloneStep as S;let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};if !self.unit.terminal_is_empty(){return self.unit.close_step(child)}if !self.emission.terminal_is_empty(){return self.emission.close_step(child)}if let Some(owner)=self.writer_retirement.as_mut(){if !owner.terminal_is_empty(){return owner.step(child)}drop(self.writer_retirement.take());return Ok(S::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}if let Some(owner)=self.progress_retirement.as_mut(){if !owner.terminal_is_empty(){return owner.step(child)}drop(self.progress_retirement.take());return Ok(S::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}if self.successor.is_some(){drop(self.successor.take());return Ok(S::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}if let Some(original)=self.progress_source.take(){match semio_framework_value::retirement::controlled::ControlledRetirement::new(original){Ok(owner)=>self.progress_retirement=Some(owner),Err((error,original))=>{self.progress_source=Some(original);return Err(error)}}return Ok(S::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}if self.counter_source.take().is_some()||self.trace_source.take().is_some(){return Ok(S::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}self.payload_source_ready=false;self.payload_position=0;self.retraction=None;self.phase=0;Ok(S::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))
     }
-    fn original_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(demand)=self.auxiliary_demands(body)?{return Ok(demand)}if let Some(owner)=self.original_close.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()});}return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"toy original writer depth overflow"))?});}Ok(if self.original_live{RetirementDemand{depth:1,..Default::default()}}else{Default::default()})}
+    fn original_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(demand)=self.auxiliary_demands(body)?{return Ok(demand)}if let Some(owner)=self.original_close.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()});}return Ok({let mut demand=owner.next_demand(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"toy original writer depth overflow"))?;demand});}Ok(if self.original_live{RetirementDemand{depth:1,..Default::default()}}else{Default::default()})}
 
     fn metadata(cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,ValueError>{let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;Ok(true)}
@@ -231,5 +231,5 @@
     fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
         let auxiliary=if !self.unit.terminal_is_empty(){Some(self.unit.retirement_demands()?)}else if !self.emission.terminal_is_empty(){Some(self.emission.retirement_demands()?)}else if let Some(owner)=self.action_close.as_ref(){Some(ToyRunJob::retirement_demand(owner,body)?)}else if self.successor.is_some()||self.entity.is_some()||self.trace.is_some()||self.action.is_some()||self.effect.is_some(){Some(RetirementDemand{depth:1,..Default::default()})}else{None};if let Some(mut demand)=auxiliary{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original toy port auxiliary depth overflow"))?;return Ok(demand)}
-        if let Some(owner)=self.writer_close.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()});}return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original toy port writer depth overflow"))?});}
+        if let Some(owner)=self.writer_close.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()});}return Ok({let mut demand=owner.next_demand(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original toy port writer depth overflow"))?;demand});}
         if self.writer_live{return Ok(RetirementDemand{depth:1,..Default::default()});}
         if let Some(owner)=self.port_close.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()});}let mut demand=owner.demands()?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original toy port depth overflow"))?;return Ok(demand);}
@@ -347,6 +347,7 @@
     fn build_retained_edit()->std::sync::Arc<Self::Edit>{std::sync::Arc::new(crate::component::test_app_mutation_fixture::config::preparation::SelectionRetainedEdit)}
 
-    fn build_store_owners() -> Result<store::os_store::document_store_owners::DocumentStoreOwners<Self::State, Self::Mutation>, semio_framework_value::ValueError> {
-        crate::window_config::bounded_window_config_store_owners::<Self>()
+    fn preparation_source_birth()->semio_framework_value::retained_clone::RetainedCloneBirthDemand{crate::window_config::bounded_window_config_preparation_source_birth::<Self>()}
+    fn build_store_owners(grant:semio_framework_value::RetainedCloneGrant) -> Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::State, Self::Mutation>,semio_framework_value::RetainedCloneProgress),store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::State,Self::Mutation>> {
+        crate::window_config::bounded_window_config_store_owners::<Self>(grant)
     }
 
@@ -437,5 +438,5 @@
             let run = doc
                 .tool_run()
-                .map_or_else(String::new, |run| format!(" completed={} steps={} payload={}", run.progress.completed, run.progress.steps.len(), run.payload.as_deref().and_then(|payload| payload.as_slice().try_into().ok()).map_or(0, u32::from_le_bytes)));
+                .map_or_else(String::new, |run| format!(" completed={} steps={} payload={}", run.progress.completed, run.progress.steps.len(), run.payload.as_deref().and_then(|payload| <[u8; 4]>::try_from(payload.as_slice()).ok()).map_or(0, u32::from_le_bytes)));
             return built_text_to_component_tree(semio_framework_ui_locale::Label::data(format!("count={}{run}", doc.snapshot.count)));
         }
@@ -1159,4 +1160,5 @@
             retract_to: None,
             payload: None,
+            payload_update:None,
         };
         let started = std::time::Instant::now();
@@ -1959,15 +1961,16 @@
             yielded_with_retained_receipt += 1;
         }
-        if let Some(found) = mounted.take_result_page() {
+        if let Some(found) = mounted.take_fixture_result_page(crate::app_testing::artifact_app_laws::fixture_mounted_policy().preparation) {
             page = Some(found);
             break;
         }
     }
-    let page = page.expect("the retained window config receipt reaches its result page");
+    let mut page = page.expect("the retained window config receipt reaches its result page");
     assert_eq!(page.lane, TypedOperationResultLane::WindowConfig);
     assert!(yielded_with_retained_receipt > 0, "the first refresh unit yields while the displaced snapshot awaits custody");
     assert!(mounted.pending_window_config_receipt.is_none() && mounted.published_window_config);
     assert!(mounted.window_config_authority.as_ref().is_some_and(|authority| authority.snapshot.generation() > generation_before), "the captured authority was refreshed to the published generation");
-    assert!(mounted.acknowledge_result_page(page.token).expect("exact ACK"));
+    assert!(mounted.acknowledge_result_page(crate::app_testing::artifact_app_laws::close_fixture_result_page(&mut page,crate::app_testing::artifact_app_laws::fixture_mounted_policy().close)).expect("exact ACK"));
+mounted.close_fixture_original_page(crate::app_testing::artifact_app_laws::fixture_mounted_policy().close);
     let mut terminal = false;
     for _ in 0..4096 {
@@ -1979,7 +1982,8 @@
             mounted.granted_retirement_step(quoted).expect("granted retirement of the acknowledged publication");
         }
-        if let Some(page) = mounted.take_result_page() {
+        if let Some(mut page) = mounted.take_fixture_result_page(crate::app_testing::artifact_app_laws::fixture_mounted_policy().preparation) {
             terminal = page.lane == TypedOperationResultLane::Terminal;
-            assert!(mounted.acknowledge_result_page(page.token).expect("exact ACK"));
+            assert!(mounted.acknowledge_result_page(crate::app_testing::artifact_app_laws::close_fixture_result_page(&mut page,crate::app_testing::artifact_app_laws::fixture_mounted_policy().close)).expect("exact ACK"));
+mounted.close_fixture_original_page(crate::app_testing::artifact_app_laws::fixture_mounted_policy().close);
             break;
         }
@@ -2380,3 +2384,2 @@
 #[path = "../🧪️tool-run-member/🦀️.rs"]
 mod member;
-                                                                                                                                                          

### 41 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs
B/H/S/current: 2521/0/2560/0
--- 
+++ 
@@ -22,7 +22,7 @@
 
 impl store::os_store::serialization::ArtifactSqliteSnapshotHost for ComposedParentSnapshot {
-    
-    
-    
+
+
+
 }
 impl pack::snapshot::ArtifactSqliteSnapshot for ComposedParentSnapshot {
@@ -130,21 +130,32 @@
 
 impl protocol::DiffAlgebra<ComposedParentSnapshot> for ComposedParentDiff {
+
+    fn is_empty(&self) -> bool {
+        self.revision.is_none()
+    }
+}
+
+impl protocol::ColdDiffAlgebra<ComposedParentSnapshot> for ComposedParentDiff {
     fn inverse(&self, base: &ComposedParentSnapshot) -> Self {
         Self { revision: self.revision.map(|_| base.revision) }
     }
-    fn is_empty(&self) -> bool {
-        self.revision.is_none()
-    }
 }
 
 impl protocol::MutationDiff<ComposedParentSnapshot> for ComposedParentDiff {
-    fn apply(&self, base: &ComposedParentSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<ComposedParentSnapshot> {
-        Ok(ComposedParentSnapshot { slot: base.slot.clone(), revision: self.revision.unwrap_or(base.revision) })
-    }
-
+
+
+
+}
+
+impl protocol::ColdMutationDiff<ComposedParentSnapshot> for ComposedParentDiff {
     fn absorb(&mut self, other: Self) {
         if other.revision.is_some() {
             self.revision = other.revision;
         }
+    }
+
+    fn apply(&self, base: &mut ComposedParentSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
+                if let Some(value) = self.revision { base.revision = value; }
+        Ok(())
     }
 }
@@ -154,7 +165,13 @@
     const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[RECURSIVE_FIXTURE_MUTATION_DESCRIPTOR];
     fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &RECURSIVE_FIXTURE_MUTATION_DESCRIPTOR }
-    fn diff(&self, _base: &ComposedParentSnapshot) -> protocol::MutationOutcome<Self::Diff> { protocol::MutationOutcome::new(ComposedParentDiff { revision: Some(self.value) }) }
+
+
+}
+
+impl crate::os_spr::ColdMutation<ComposedParentSnapshot> for RecursiveFixtureMutation {
+    type ReferenceDiff = ComposedParentDiff;
+    fn diff(&self, _base: &ComposedParentSnapshot) -> protocol::MutationOutcome<Self::ReferenceDiff> { protocol::MutationOutcome::new(ComposedParentDiff { revision: Some(self.value) }) }
     fn inverse(&self, base: &ComposedParentSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
-    Ok((|| { vec![Self { value: base.revision }] 
+    Ok((|| { vec![Self { value: base.revision }]
     })())
 }
@@ -174,5 +191,5 @@
 impl ArtifactPack for ComposedParentSnapshot {
     /// 🪶️ Publishes this owner's actual relational snapshot capability.
-    
+
 
     fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
@@ -597,7 +614,7 @@
 
 impl store::os_store::serialization::ArtifactSqliteSnapshotHost for RecursiveBranchSnapshot {
-    
-    
-    
+
+
+
 }
 impl pack::snapshot::ArtifactSqliteSnapshot for RecursiveBranchSnapshot {
@@ -642,25 +659,32 @@
 
 impl protocol::DiffAlgebra<RecursiveBranchSnapshot> for RecursiveBranchDiff {
+
+    fn is_empty(&self) -> bool {
+        self.count.is_none()
+    }
+}
+
+impl protocol::ColdDiffAlgebra<RecursiveBranchSnapshot> for RecursiveBranchDiff {
     fn inverse(&self, base: &RecursiveBranchSnapshot) -> Self {
         Self { count: self.count.map(|_| base.count) }
     }
-    fn is_empty(&self) -> bool {
-        self.count.is_none()
-    }
 }
 
 impl protocol::MutationDiff<RecursiveBranchSnapshot> for RecursiveBranchDiff {
-    fn apply(&self, base: &RecursiveBranchSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<RecursiveBranchSnapshot> {
-        Ok(RecursiveBranchSnapshot {
-            count: self.count.unwrap_or(base.count),
-            label: base.label.clone(),
-            nested: base.nested.clone(),
-        })
-    }
-
+
+
+
+}
+
+impl protocol::ColdMutationDiff<RecursiveBranchSnapshot> for RecursiveBranchDiff {
     fn absorb(&mut self, other: Self) {
         if other.count.is_some() {
             self.count = other.count;
         }
+    }
+
+    fn apply(&self, base: &mut RecursiveBranchSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
+                if let Some(value) = self.count { base.count = value; }
+        Ok(())
     }
 }
@@ -670,7 +694,13 @@
     const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[RECURSIVE_FIXTURE_MUTATION_DESCRIPTOR];
     fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &RECURSIVE_FIXTURE_MUTATION_DESCRIPTOR }
-    fn diff(&self, _base: &RecursiveBranchSnapshot) -> protocol::MutationOutcome<Self::Diff> { protocol::MutationOutcome::new(RecursiveBranchDiff { count: Some(self.value) }) }
+
+
+}
+
+impl crate::os_spr::ColdMutation<RecursiveBranchSnapshot> for RecursiveFixtureMutation {
+    type ReferenceDiff = RecursiveBranchDiff;
+    fn diff(&self, _base: &RecursiveBranchSnapshot) -> protocol::MutationOutcome<Self::ReferenceDiff> { protocol::MutationOutcome::new(RecursiveBranchDiff { count: Some(self.value) }) }
     fn inverse(&self, base: &RecursiveBranchSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
-    Ok((|| { vec![Self { value: base.count }] 
+    Ok((|| { vec![Self { value: base.count }]
     })())
 }
@@ -883,4 +913,6 @@
 
 impl store::ErasedSnapshotRetirement for RecursiveBranchSnapshotOpen {
+ fn next_demand(&self,maximum_body_bytes:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.close_demands(maximum_body_bytes)}
+
     fn close_step(&mut self, grant: store::RetainedCloneGrant) -> Result<store::RetainedCloneStep, semio_framework_value::ValueError> {
         let empty = store::RetainedCloneProgress::default();
@@ -1113,11 +1145,11 @@
     }
 
-    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
+    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure> {
         if self.closing || cx.is_cancelled() {
-            return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx);
+            return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
         }
         if self.retirement.is_some() {
             if cx.should_yield() {
-                return semio_framework_job::JobOutcomeBorrow::admit_yield(cx);
+                return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
             }
             let grant=cx.retained_grant();
@@ -1128,11 +1160,11 @@
                     cx.consume_retained(progress)?;
                     cx.consume_fuel((progress.copied_items + progress.released_bytes).max(1) as u64);
-                    semio_framework_job::JobOutcomeBorrow::admit_yield(cx)
+                    semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from)
                 }
-                _ => Err(Self::fault(b"recursive-parent-replay-retirement-rejected")),
+                _ => Err((Self::fault(b"recursive-parent-replay-retirement-rejected")).into()),
             };
         }
         if cx.should_yield() {
-            return semio_framework_job::JobOutcomeBorrow::admit_yield(cx);
+            return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
         }
         if let Some(runtime) = self.runtime.as_mut() {
@@ -1143,7 +1175,7 @@
                 Ok((grant, store::RetainedCloneStep::Progress(progress))) if progress.fits(grant) => {
                     cx.consume_fuel((progress.copied_items + progress.released_bytes).max(1) as u64);
-                    return semio_framework_job::JobOutcomeBorrow::admit_yield(cx);
+                    return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
                 }
-                _ => return Err(Self::fault(b"recursive-parent-current-retirement-rejected")),
+                _ => return Err((Self::fault(b"recursive-parent-current-retirement-rejected")).into()),
             }
         }
@@ -1152,7 +1184,7 @@
             Some(envelope) => envelope,
             None if self.phase == ReadyComposedParentInitializationPhase::Complete => {
-                return semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None);
+                return semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None).map_err(semio_framework_job::OriginalTurnFailure::from);
             }
-            None => return Err(Self::fault(b"recursive-parent-envelope-owner-missing")),
+            None => return Err((Self::fault(b"recursive-parent-envelope-owner-missing")).into()),
         };
         match self.phase {
@@ -1165,8 +1197,8 @@
             ReadyComposedParentInitializationPhase::Seed => {
                 if let Some(edit) = envelope.vcs.edits.get(self.edit_index) {
-                    let Some(runtime) = self.runtime.as_mut() else { return Err(Self::fault(b"recursive-parent-runtime-owner-missing")) };
+                    let Some(runtime) = self.runtime.as_mut() else { return Err((Self::fault(b"recursive-parent-runtime-owner-missing")).into()) };
                     if self.operation_index == 0 {
                         if runtime.seed_mutation(protocol::MutationId(edit.id.clone())).is_err() {
-                            return Err(Self::fault(b"recursive-parent-edit-seed-rejected"));
+                            return Err((Self::fault(b"recursive-parent-edit-seed-rejected")).into());
                         }
                         self.operation_index = 1;
@@ -1174,5 +1206,5 @@
                         if let Some(id) = &meta.mutation_id {
                             if runtime.seed_edit_operation(&edit.id, id.clone()).is_err() {
-                                return Err(Self::fault(b"recursive-parent-mutation-seed-rejected"));
+                                return Err((Self::fault(b"recursive-parent-mutation-seed-rejected")).into());
                             }
                         }
@@ -1191,18 +1223,18 @@
             }
             ReadyComposedParentInitializationPhase::Replay => {
-                let Some(cursor) = envelope.cursor.as_ref() else { return Err(Self::fault(b"recursive-parent-cursor-missing")) };
+                let Some(cursor) = envelope.cursor.as_ref() else { return Err((Self::fault(b"recursive-parent-cursor-missing")).into()) };
                 if let Some(edit_id) = cursor.applied_edit_ids.get(self.lane_index) {
-                    let Some(edit) = envelope.vcs.edits.iter().find(|edit| edit.id == *edit_id) else { return Err(Self::fault(b"recursive-parent-applied-edit-missing")) };
+                    let Some(edit) = envelope.vcs.edits.iter().find(|edit| edit.id == *edit_id) else { return Err((Self::fault(b"recursive-parent-applied-edit-missing")).into()) };
                     if let Some(operation) = edit.forwards.get(self.operation_index) {
                         let Some(runtime) = self.runtime.as_mut() else {
-                            return Err(Self::fault(b"recursive-parent-current-owner-missing"));
+                            return Err((Self::fault(b"recursive-parent-current-owner-missing")).into());
                         };
                         let current = runtime.current_ref();
-                        let next = match protocol::apply_diff(operation.diff(current).diff(), current) {
+                        let next = match protocol::apply_diff_cold(operation.diff(current).diff(), current) {
                             Ok(next) => next,
-                            Err(_) => return Err(Self::fault(b"recursive-parent-replay-rejected")),
+                            Err(_) => return Err((Self::fault(b"recursive-parent-replay-rejected")).into()),
                         };
                         if runtime.adopt_current_owned(next, std::sync::Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<ComposedParentSnapshot>::default())).is_err() {
-                            return Err(Self::fault(b"recursive-parent-current-adoption-rejected"));
+                            return Err((Self::fault(b"recursive-parent-current-adoption-rejected")).into());
                         }
                         self.operation_index += 1;
@@ -1217,12 +1249,12 @@
             }
             ReadyComposedParentInitializationPhase::Applied | ReadyComposedParentInitializationPhase::Redo => {
-                let Some(cursor) = envelope.cursor.as_ref() else { return Err(Self::fault(b"recursive-parent-cursor-missing")) };
+                let Some(cursor) = envelope.cursor.as_ref() else { return Err((Self::fault(b"recursive-parent-cursor-missing")).into()) };
                 let ids = if self.phase == ReadyComposedParentInitializationPhase::Applied { &cursor.applied_edit_ids } else { &cursor.redo_edit_ids };
                 if let Some(id) = ids.get(self.lane_index) {
-                    let Some((ledger_key, edit)) = envelope.vcs.edits.find_near_key(|edit| edit.id == *id) else { return Err(Self::fault(b"recursive-parent-lane-edit-missing")) };
-                    let Some(runtime) = self.runtime.as_mut() else { return Err(Self::fault(b"recursive-parent-runtime-owner-missing")) };
+                    let Some((ledger_key, edit)) = envelope.vcs.edits.find_near_key(|edit| edit.id == *id) else { return Err((Self::fault(b"recursive-parent-lane-edit-missing")).into()) };
+                    let Some(runtime) = self.runtime.as_mut() else { return Err((Self::fault(b"recursive-parent-runtime-owner-missing")).into()) };
                     let result = if self.phase == ReadyComposedParentInitializationPhase::Applied { runtime.push_applied_edit(edit, ledger_key) } else { runtime.push_redo_edit(edit, ledger_key) };
                     if result.is_err() {
-                        return Err(Self::fault(b"recursive-parent-lane-seed-rejected"));
+                        return Err((Self::fault(b"recursive-parent-lane-seed-rejected")).into());
                     }
                     self.lane_index += 1;
@@ -1247,8 +1279,8 @@
             }
             ReadyComposedParentInitializationPhase::Complete => {
-                return semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None);
+                return semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None).map_err(semio_framework_job::OriginalTurnFailure::from);
             }
         }
-        semio_framework_job::JobOutcomeBorrow::admit_yield(cx)
+        semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from)
     }
 
@@ -1269,5 +1301,9 @@
     }
 
-    fn close_step(&mut self, grant: store::RetainedCloneGrant) -> Result<store::RetainedCloneStep, semio_framework_value::ValueError> {
+    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>) -> Result<store::RetainedCloneStep,semio_framework_job::OriginalTurnFailure> {
+        self.begin_close();
+        if self.terminal_is_empty(){return Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(Default::default()))}
+        let demand=self.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;
+        let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{
         use semio_framework_value::{ValueError, ValueRefusalKind};
         let empty = store::RetainedCloneProgress::default();
@@ -1310,4 +1346,7 @@
         }
         Ok(store::RetainedCloneStep::Complete(empty))
+
+        })?;
+        Ok(if self.terminal_is_empty(){semio_framework_value::retained_clone::RetainedCloneStep::Complete(step.progress())}else{semio_framework_value::retained_clone::RetainedCloneStep::Progress(step.progress())})
     }
 


### 42 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️artifact-admission-and-completion-oracles/🟦️.ts
B/H/S/current: 90/91/101/95
--- 
+++ 
@@ -10,6 +10,11 @@
 }>;
 
-type CompletionCallerV1 = Readonly<{ family:string; source:string; sourceStart:string; sourceEnd:string; completion:string; retainedOwner:string; terminalGuard:string; incrementalClose:string; retainedPhysicalOwners:readonly string[]; legacyLossToken:string; bindings:readonly Readonly<{source:string;tokens:readonly string[]}>[] }>;
-type CompletionRejectionLawV1 = Readonly<{ schema:string; noImplicitRetry:boolean; cases:readonly Readonly<{id:string;cell:"empty"|"existing";busy:boolean;outcome:"accepted"|"rejected";finalCell:"empty"|"existing"|"submitted";submittedOwnerReturned:boolean}>[];callers:readonly CompletionCallerV1[];reservedCallers:readonly CompletionCallerV1[] }>;
+type CompletionRejectionLawV1 = Readonly<{
+  schema: string;
+  noImplicitRetry: boolean;
+  cases: readonly Readonly<{ id: string; cell: "empty" | "existing"; busy: boolean; outcome: "accepted" | "rejected"; finalCell: "empty" | "existing" | "submitted"; submittedOwnerReturned: boolean }>[];
+  callers: readonly Readonly<{ family: string; source: string; retainedOwner: string; terminalGuard: string; incrementalClose: string; retainedPhysicalOwners: readonly string[]; legacyLossToken: string }>[];
+  reservedCallers: readonly Readonly<{ family: string; source: string; sourceStart: string; sourceEnd: string; terminalGuard: string; retainedOwner: string; handoff: string; incrementalClose: string; legacyLossToken: string }>[];
+}>;
 
 type AwaitedCompletionFixtureV1 = Readonly<{ version: 1; ownerReturningRejection: CompletionRejectionLawV1 }>;
@@ -56,8 +61,7 @@
 /** ♻️ Independently models completion admission and checks the exact current caller's retained owners. */
 export function completionRejectionOracle(repoRoot?: string): number {
-  const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/⏳️completion/🔣️.json", import.meta.url), "utf8")) as AwaitedCompletionFixtureV1;
+  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/⏳️completion/🔣️.json", import.meta.url), "utf8")) as AwaitedCompletionFixtureV1;
   const law = fixture.ownerReturningRejection;
-  assert.equal(law.noImplicitRetry, true);
-  assert.equal(new Set(law.cases.map(row => row.id)).size, law.cases.length);
+  assert.equal(new Set(law.cases.map((row) => row.id)).size, law.cases.length);
   for (const row of law.cases) {
     const rejected = row.busy || row.cell !== "empty";
@@ -67,22 +71,29 @@
   }
   if (repoRoot) {
-    for (const rows of [law.callers, law.reservedCallers]) {
-      assert.equal(new Set(rows.map(row => row.family)).size, rows.length);
-      for (const row of rows) {
-        const source:string = readFileSync(resolve(repoRoot, row.source), "utf8");
-        const region:string|undefined = source.split(row.sourceStart, 2)[1]?.split(row.sourceEnd, 1)[0];
-        assert(region !== undefined, `${row.family} current producer region is absent`);
-        const guard = region.indexOf(row.terminalGuard), prepare = region.indexOf(row.completion), retain = region.indexOf(row.retainedOwner, prepare), close = region.indexOf(row.incrementalClose, retain);
-        assert(guard >= 0, `${row.family} can replay after a rejected completion`);
-        assert(prepare >= 0 && retain > prepare, `${row.family} loses the original returned completion owner`);
-        assert(close > retain, `${row.family} lacks its original full-grant child closure`);
-        assert(row.retainedPhysicalOwners.length > 0, `${row.family} omits its terminal custody witnesses`);
-        for (const owner of row.retainedPhysicalOwners) assert(region.includes(owner), `${row.family} lost original owner ${owner}`);
-        assert.equal(region.includes(row.legacyLossToken), false, `${row.family} uses a lossy completion handoff`);
-        for (const binding of row.bindings) {
-          const caller = readFileSync(resolve(repoRoot, binding.source), "utf8");
-          for (const token of binding.tokens) assert(caller.includes(token), `${row.family} no longer delegates through ${binding.source}: ${token}`);
-        }
-      }
+    assert.equal(new Set(law.callers.map((row) => row.family)).size, law.callers.length);
+    for (const row of law.callers) {
+      const source: string = readFileSync(resolve(repoRoot, row.source), "utf8");
+      const retain = source.indexOf(row.retainedOwner);
+      const guard = source.indexOf(row.terminalGuard);
+      const close = source.indexOf(row.incrementalClose);
+      assert(retain >= 0, `${row.family} loses the returned completion owner`);
+      assert(guard >= 0, `${row.family} can replay after terminal completion rejection`);
+      assert(close > retain, `${row.family} lacks its declared child-first rejection close operation`);
+      for (const owner of row.retainedPhysicalOwners) assert(source.includes(owner), `${row.family} lost a retained physical parent terminal guard: ${owner}`);
+      assert.equal(source.includes(row.legacyLossToken), false, `${row.family} retained the lossy is_err handoff`);
+    }
+    assert.equal(new Set(law.reservedCallers.map((row) => row.family)).size, law.reservedCallers.length);
+    for (const row of law.reservedCallers) {
+      const source: string = readFileSync(resolve(repoRoot, row.source), "utf8");
+      const region: string | undefined = source.split(row.sourceStart, 2)[1]?.split(row.sourceEnd, 1)[0];
+      assert(region !== undefined, `${row.family} source region is absent`);
+      const guard = region.indexOf(row.terminalGuard);
+      const prepare = region.indexOf(row.handoff);
+      const retain = region.indexOf(row.retainedOwner);
+      const close = region.indexOf(row.incrementalClose);
+      assert(guard >= 0 && guard < prepare, `${row.family} can replay after a rejected completion`);
+      assert(prepare >= 0 && prepare < retain, `${row.family} does not retain the exact returned owner`);
+      assert(close > retain, `${row.family} does not retire the returned owner incrementally`);
+      assert.equal(region.includes(row.legacyLossToken), false, `${row.family} retained the lossy completion handoff`);
     }
   }


### 43 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️bounded-reload/🦀️.rs
B/H/S/current: 233/248/249/265
--- 
+++ 
@@ -16,5 +16,5 @@
 
 impl ArtifactStoreInitializationAuthority<TestSnapshot, TestMutation> for PhysicalInitializerAuthority {
-    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, ValueError> { semio_framework_job::JobOutcomeBorrow::admit_yield(cx) }
+    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure> { semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from) }
     fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, ValueError> { crate::app_testing::artifact_app_laws::fixture_job_outcome(descriptor) }
     fn request_cancel(&mut self) {}
@@ -22,5 +22,12 @@
     fn retirement_demands(&self,_body:usize)->Result<RetirementDemand,ValueError>{Ok(self.buffer.as_ref().map_or(Default::default(),|buffer|RetirementDemand{copy_bytes:std::mem::size_of_val(&self.buffer),release_bytes:buffer.capacity(),depth:1,..Default::default()}))}
     fn begin_close(&mut self) {}
-    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let empty=Default::default();let Some(buffer)=self.buffer.as_ref()else{return Ok(RetainedCloneStep::Complete(empty))};let copied_bytes=std::mem::size_of_val(&self.buffer);let released_bytes=buffer.capacity();if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_release_bytes<released_bytes||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(empty));}drop(self.buffer.take());Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes,released_bytes,..empty}))}
+    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<RetainedCloneStep,semio_framework_job::OriginalTurnFailure> {
+        self.begin_close();
+        if self.terminal_is_empty(){return Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(Default::default()))}
+        let demand=self.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;
+        let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{let empty=Default::default();let Some(buffer)=self.buffer.as_ref()else{return Ok(RetainedCloneStep::Complete(empty))};let copied_bytes=std::mem::size_of_val(&self.buffer);let released_bytes=buffer.capacity();if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_release_bytes<released_bytes||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(empty));}drop(self.buffer.take());Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes,released_bytes,..empty}))
+        })?;
+        Ok(if self.terminal_is_empty(){semio_framework_value::retained_clone::RetainedCloneStep::Complete(step.progress())}else{semio_framework_value::retained_clone::RetainedCloneStep::Progress(step.progress())})
+    }
     fn terminal_is_empty(&self) -> bool { self.buffer.is_none() }
 }
@@ -37,4 +44,5 @@
     let admission = fixture["admissionBytes"].as_u64().unwrap() as usize;
     for row in fixture["cases"].as_array().unwrap() {
+        let mut original=crate::app_testing::OriginalWorkFixture::new();
         let extent = row["physicalBytes"].as_u64().unwrap() as usize;
         let mut buffer = Vec::new(); buffer.try_reserve_exact(extent).unwrap();
@@ -44,6 +52,6 @@
         let demand = job.next_close_release_byte_demand().unwrap();
         let original_grant=|release|RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:release,maximum_depth:4096};
-        let first = job.close_step(original_grant(row["callerBytes"].as_u64().unwrap()as usize));
-        if row["releasedBytes"].as_u64().unwrap() == 0 { job.close_step(original_grant(admission)); }
+        let first = original.turn(original_grant(row["callerBytes"].as_u64().unwrap()as usize),|cx|job.close_step(cx)).expect("first original initializer close");
+        if row["releasedBytes"].as_u64().unwrap() == 0 { original.turn(original_grant(admission),|cx|job.close_step(cx)).expect("original initializer close"); }
         assert!(job.accept_terminal_failure());
         assert!(job.authority.is_some());
@@ -51,7 +59,7 @@
         let box_bytes = std::mem::size_of::<PhysicalInitializerAuthority>();
         let terminal_demand = job.next_close_release_byte_demand().unwrap();
-        let denied = job.close_step(original_grant(box_bytes-1));
+        let denied = original.turn(original_grant(box_bytes-1),|cx|job.close_step(cx)).expect("denied original authority release");
         let retained = job.authority.is_some();
-        let released = job.close_step(original_grant(box_bytes));
+        let released = original.turn(original_grant(box_bytes),|cx|job.close_step(cx)).expect("funded original authority release");
         for _ in 0..8 { if job.terminal_is_empty() { break; } job.close_step(original_grant(admission)); }
         assert!(job.terminal_is_empty());
@@ -62,4 +70,5 @@
         assert!(retained);
         assert_eq!(released, InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<Option<Box<dyn ArtifactStoreInitializationAuthority<TestSnapshot,TestMutation>>>>(),released_bytes:box_bytes,..Default::default()}});
+        original.finish();
         eprintln!("[DEBUG] initializer job physical backing={extent} erased-authority-box={box_bytes} retained-denied-owner=true");
     }
@@ -122,12 +131,7 @@
     }
 
-    fn diff(&self, base: &TestSnapshot) -> ::protocol::MutationOutcome<Self::Diff> {
-        RELOAD_FOLDS.with(|folds| folds.set(folds.get() + 1));
-        store::Mutation::<TestSnapshot>::diff(&self.0, base)
-    }
-
-    fn inverse(&self, base: &TestSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
-        Ok({ store::Mutation::<TestSnapshot>::inverse(&self.0, base)?.into_iter().map(Self).collect() })
-    }
+
+
+
 
     fn conflict_target(&self) -> Vec<String> {
@@ -137,4 +141,15 @@
     fn may_emit_foreign_steps(&self) -> bool {
         store::Mutation::<TestSnapshot>::may_emit_foreign_steps(&self.0)
+    }
+}
+
+impl store::ColdMutation<TestSnapshot> for ReloadCountedOp {
+    type ReferenceDiff = <TestMutation as store::ColdMutation<TestSnapshot>>::ReferenceDiff;
+    fn diff(&self, base: &TestSnapshot) -> ::protocol::MutationOutcome<Self::ReferenceDiff> {
+        RELOAD_FOLDS.with(|folds| folds.set(folds.get() + 1));
+        store::ColdMutation::<TestSnapshot>::diff(&self.0, base)
+    }
+    fn inverse(&self, base: &TestSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
+        Ok({ store::ColdMutation::<TestSnapshot>::inverse(&self.0, base)?.into_iter().map(Self).collect() })
     }
 }
@@ -174,16 +189,16 @@
     let (operation, generation) = (semio_framework_job::OperationId(97), semio_framework_job::Generation(0));
     let mut job = bounded_document_store_initialization_job::<_,_,::semio_framework_os_kernel::os_store::document_store_owners::BoundedDocumentStoreOwnerProducer>(envelope, RELOAD_DOCUMENT_SCHEMA, operation, generation, source.local_actor_id().clone());
-    let mut sequence = 0u64;
+    let mut original_caller=crate::app_testing::OriginalWorkFixture::with_identity(operation,generation);
     let mut steps = 0usize;
     let mut folded = 0;
     loop {
         let before = RELOAD_FOLDS.with(std::cell::Cell::get);
-        let mut ownership=RetainedCloneProgress::default();
-        let mut cx=semio_framework_job::StepContext::new(operation,generation,semio_framework_job::StepBudget::new(1,u64::MAX,original),semio_framework_job::CancelToken::root_now(),semio_framework_job::default_now_us,&mut sequence,&mut ownership);
-        let finished = match semio_framework_job::InteractiveJob::step(&mut job, &mut cx) {
-            Ok(None) | Ok(Some(semio_framework_job::JobOutcomeBorrow::Yield { .. })) => false,
-            Ok(Some(semio_framework_job::JobOutcomeBorrow::Complete { .. })) => true,
-            _ => panic!("the reload stopped at initializer step {steps}"),
-        };
+        let finished=original_caller.work_turn(original,1,|cx|{
+            Ok(match semio_framework_job::InteractiveJob::step(&mut job,cx)?{
+                None|Some(semio_framework_job::JobOutcomeBorrow::Yield{..})=>false,
+                Some(semio_framework_job::JobOutcomeBorrow::Complete{..})=>true,
+                _=>panic!("the reload stopped at initializer step {steps}"),
+            })
+        }).expect("same original bounded reload turn");
         assert!(RELOAD_FOLDS.with(std::cell::Cell::get) - before <= 1, "initializer step {steps} folded more than one operation");
         let progress = job.progress();
@@ -208,5 +223,5 @@
     let handoff_demand=semio_framework_job::InteractiveJob::next_close_release_byte_demand(&job).unwrap();
     assert!(job.authority.is_some());
-    assert_eq!(semio_framework_job::InteractiveJob::close_step(&mut job,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:authority_bytes-1,maximum_depth:4096}), semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()});
+    assert_eq!(original_caller.turn(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:authority_bytes-1,maximum_depth:4096},|cx|semio_framework_job::InteractiveJob::close_step(&mut job,cx)).expect("denied original reload authority release"), semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()});
     for _ in 0..65_536 {
         if job.terminal_is_empty() {
@@ -214,5 +229,5 @@
         }
         let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:admission,maximum_depth:4096};
-        let step=semio_framework_job::InteractiveJob::close_step(&mut job,grant);
+        let step=original_caller.turn(grant,|cx|semio_framework_job::InteractiveJob::close_step(&mut job,cx)).expect("funded original reload close");
         assert!(step.progress().fits(grant));
         if let semio_framework_job::InteractiveJobCloseStep::Pending{progress}=step{assert!(progress.copied_items<=1&&progress.released_bytes<=admission);}
@@ -228,4 +243,5 @@
     assert_eq!(reloaded.content_revision_now(), whole.content_revision_now(), "the retained reload names the revision a whole load names");
     drop(job);
+    original_caller.finish();
     close_reload_counted_store(&mut reloaded);
     close_reload_counted_store(&mut whole);


### 44 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs
B/H/S/current: 1956/200/1956/200
--- 
+++ 
@@ -100,5 +100,5 @@
     use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind};
     let expected:serde_json::Value=serde_json::from_str(include_str!("../../🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🧾️expected.json")).unwrap();let policy:serde_json::Value=serde_json::from_str(include_str!("../../🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();let phase:RetainedCloneGrant=serde_json::from_value(policy["retirementLoan"].clone()).unwrap();let mut caller=artifact_app_laws::OriginalFixtureCaller::admit().unwrap();let mut cx=caller.context();let identity=cx.original_recipient_identity();
-    let((failure,pointers),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let primary=ValueError::new(ValueRefusalKind::AllocationFailed,"original acceptance cause 源");let receipt=ValueError::new(ValueRefusalKind::InvariantViolated,"original acceptance receipt 拒");let context=ValueError::new(ValueRefusalKind::Canceled,"original acceptance context 断");let pointers=(primary.message.as_ptr(),receipt.message.as_ptr(),context.message.as_ptr());(semio_framework_job::OriginalTurnFailure{primary:semio_framework_job::OriginalTurnPrimary::Value(primary),receipt_refusal:Some(receipt),context_refusal:Some(context)},pointers)});let(mut born,mut released)=(heap.requested_bytes,heap.released_bytes);cx.consume_retained(RetainedCloneProgress{copied_items:3,retained_capacity_bytes:born,released_bytes:released,..Default::default()}).unwrap();
+    let((failure,pointers),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let primary=ValueError::new(ValueRefusalKind::AllocationFailed,"original acceptance cause 源");let receipt=ValueError::new(ValueRefusalKind::InvariantViolated,"original acceptance receipt 拒");let context=ValueError::new(ValueRefusalKind::Canceled,"original acceptance context 断");let pointers=(primary.message.as_ptr(),receipt.message.as_ptr(),context.message.as_ptr());({let original=semio_framework_job::OriginalTurnFailure::from(semio_framework_job::OriginalTurnPrimary::Value(primary));semio_framework_job::OriginalTurnFailure{receipt_refusal:Some(receipt),context_refusal:Some(context),..original}},pointers)});let(mut born,mut released)=(heap.requested_bytes,heap.released_bytes);cx.consume_retained(RetainedCloneProgress{copied_items:3,retained_capacity_bytes:born,released_bytes:released,..Default::default()}).unwrap();
     let(error,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||AcceptanceSeedFault::Original(failure.into()).into_error().context("the seed does not land".into()).context("original reload annotation".into()));born+=heap.requested_bytes;released+=heap.released_bytes;cx.consume_retained(RetainedCloneProgress{copied_items:2,retained_capacity_bytes:heap.requested_bytes,released_bytes:heap.released_bytes,..Default::default()}).unwrap();let original=error.original.as_ref().unwrap();let crate::PluginLifecyclePrimary::Value(primary)=&original.primary else{panic!("original acceptance primary retains its variant")};assert_eq!((primary.message.as_ptr(),original.receipt_refusal.as_ref().unwrap().message.as_ptr(),original.context_refusal.as_ref().unwrap().message.as_ptr()),pointers);assert_eq!(serde_json::json!({"callerRetainsFailure":true})["callerRetainsFailure"],expected["callerRetainsFailure"]);
     let(mut owner,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_value::retirement::controlled::ControlledRetirement::new(error).unwrap_or_else(|_|panic!("complete acceptance error supports paid retirement")));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));


### 45 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs
B/H/S/current: 3393/410/3401/411
--- 
+++ 
@@ -2188,4 +2188,5 @@
 /// 🪶️ Publishes one edit with an explicit author while preserving the opened instance actor.
 fn publish_as(app: &mut ToyApp, actor: &str, mutation: TestMutation) {
+    let mut receiving_job=crate::app_testing::artifact_app_laws::FixtureOriginalJob::new(semio_framework::kernel::RetainedTurnInput{operation:71,generation:3,epoch:1,grant:app.mounted_policy.close});
     let grant = store::os_store::one_item::ArtifactStoreOneItemGrant::from_retained(app.mounted_policy.preparation);
     let mut publication = app
@@ -3116,17 +3117,7 @@
     }
 
-    fn diff(&self, base: &TestSnapshot) -> ::protocol::MutationOutcome<Self::Diff> {
-        match &self.0 {
-            TestMutation::SetLabel(_) if base.count < 0 => ::protocol::MutationOutcome::error("mutation.target-mismatch", "a label needs a count that is not negative", ["count"]),
-            operation => store::Mutation::<TestSnapshot>::diff(operation, base),
-        }
-    }
-
-    fn inverse(&self, base: &TestSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
-        if matches!(self.0, TestMutation::SetSlotChildren(_)) && base.count < -1 {
-            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "children have no inverse under a count below -1"));
-        }
-        Ok(store::Mutation::<TestSnapshot>::inverse(&self.0, base)?.into_iter().map(Self).collect())
-    }
+
+
+
 
     fn conflict_target(&self) -> Vec<String> {
@@ -3155,10 +3146,27 @@
 }
 
+impl store::ColdMutation<TestSnapshot> for InertLabelOp {
+    type ReferenceDiff = <TestMutation as store::Mutation<TestSnapshot>>::Diff;
+    fn diff(&self, base: &TestSnapshot) -> ::protocol::MutationOutcome<Self::ReferenceDiff> {
+        match &self.0 {
+            TestMutation::SetLabel(_) if base.count < 0 => ::protocol::MutationOutcome::error("mutation.target-mismatch", "a label needs a count that is not negative", ["count"]),
+            operation => store::Mutation::<TestSnapshot>::diff(operation, base),
+        }
+    }
+    fn inverse(&self, base: &TestSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
+        if matches!(self.0, TestMutation::SetSlotChildren(_)) && base.count < -1 {
+            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "children have no inverse under a count below -1"));
+        }
+        Ok(store::Mutation::<TestSnapshot>::inverse(&self.0, base)?.into_iter().map(Self).collect())
+    }
+}
+
 /// ⏭️ Replays `drafts` from `from` on `document` to completion through its history-edit owners and answers the report.
 async fn replayed_report(owners: &mut time_travel::TimeTravelStoreState<TestSnapshot, InertLabelOp>, document: &mut ArtifactStore<TestSnapshot, InertLabelOp>, drafts: &protocol::HistoryInputDrafts, from: &MutationId) -> protocol::ReplayReport {
-    let started = owners.run(document, time_travel::TimeTravelStoreCommand::StartReplay { drafts: drafts.clone(), from: from.clone() }).await.expect("the replay starts");
+    let (policy,_)=original_time_travel_fixture_caller();let grant=policy.preparation;let mut receiving_job=crate::app_testing::artifact_app_laws::FixtureOriginalJob::new(semio_framework::kernel::RetainedTurnInput{operation:71,generation:3,epoch:1,grant:policy.close});
+    let started = owners.run(document, time_travel::TimeTravelStoreCommand::StartReplay { drafts: drafts.clone(), from: from.clone() },None).await.expect("the replay starts");
     assert!(matches!(started, time_travel::TimeTravelStoreOutput::ReplayStarted(true)), "the store starts the Report replay");
     for _ in 0..65_536 {
-        match owners.run(document, time_travel::TimeTravelStoreCommand::StepReplay { deadline_us: u64::MAX, clock: semio_framework_job::default_now_us }).await.expect("a replay slice") {
+        match receiving_job.receive(grant,|receiving|semio_framework_async::poll::resolve_ready(owners.run(document, time_travel::TimeTravelStoreCommand::StepReplay { deadline_us: u64::MAX, clock: semio_framework_job::default_now_us,grant },Some(receiving)))).expect("a replay slice") {
             time_travel::TimeTravelStoreOutput::Stepped(time_travel::TimeTravelReplayStep::Completed(report)) => return report,
             time_travel::TimeTravelStoreOutput::Stepped(time_travel::TimeTravelReplayStep::Pending { .. }) => {}
@@ -3221,5 +3229,5 @@
 #[semio_framework_async_macros::async_test]
 async fn a_blocking_mutation_without_editable_inputs_is_withdrawn_and_the_review_becomes_ready() {
-    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
+    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();let mut receiving_job=crate::app_testing::artifact_app_laws::FixtureOriginalJob::new(semio_framework::kernel::RetainedTurnInput{operation:71,generation:3,epoch:1,grant:mounted_policy.close});
     let mut observer=|_|true;
     let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
@@ -3248,13 +3256,13 @@
 
     for (target, why) in [(&label, "an editor with zero rows never opens"), (&children, "an operation without an input schema has no editor")] {
-        let refused = owners.run(&mut document, TimeTravelStoreCommand::Open { target: target.clone(), current: None, withdraw: false }).await.expect("the open runs");
+        let refused = owners.run(&mut document, TimeTravelStoreCommand::Open { target: target.clone(), current: None, withdraw: false },None).await.expect("the open runs");
         assert!(matches!(refused, TimeTravelStoreOutput::Opened(Err(TimeTravelActionRefusal::NotEditable))), "{why}");
     }
-    let TimeTravelStoreOutput::Opened(Ok((editor, original))) = owners.run(&mut document, TimeTravelStoreCommand::Open { target: label.clone(), current: Some(protocol::InputReplacement::Withdrawn), withdraw: true }).await.expect("the open runs") else {
+    let TimeTravelStoreOutput::Opened(Ok((editor, original))) = owners.run(&mut document, TimeTravelStoreCommand::Open { target: label.clone(), current: Some(protocol::InputReplacement::Withdrawn), withdraw: true },None).await.expect("the open runs") else {
         panic!("the store's supersede law admits withdrawing an operation without editable inputs");
     };
     assert!(editor.inputs.is_empty() && editor.inputs_refused.is_none() && editor.target == label, "its editor holds no inputs and names no schema fault");
     assert!(matches!(original, protocol::InputReplacement::Input { .. }), "its original input is its recorded operation");
-    owners.run(&mut document, TimeTravelStoreCommand::Adopt(true)).await.expect("the session adopts the kind");
+    owners.run(&mut document, TimeTravelStoreCommand::Adopt(true),None).await.expect("the session adopts the kind");
 
     drafts.insert(label.clone(), protocol::InputReplacement::Withdrawn);
@@ -3277,5 +3285,5 @@
 #[semio_framework_async_macros::async_test]
 async fn an_inverse_refusal_is_one_mutations_fatal_that_its_row_names_and_resolves() {
-    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
+    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();let mut receiving_job=crate::app_testing::artifact_app_laws::FixtureOriginalJob::new(semio_framework::kernel::RetainedTurnInput{operation:71,generation:3,epoch:1,grant:mounted_policy.close});
     let mut observer=|_|true;
     let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
@@ -3300,8 +3308,8 @@
     }
 
-    let TimeTravelStoreOutput::Opened(Ok(_)) = owners.run(&mut document, TimeTravelStoreCommand::Open { target: children.clone(), current: Some(protocol::InputReplacement::Withdrawn), withdraw: true }).await.expect("the open runs") else {
+    let TimeTravelStoreOutput::Opened(Ok(_)) = owners.run(&mut document, TimeTravelStoreCommand::Open { target: children.clone(), current: Some(protocol::InputReplacement::Withdrawn), withdraw: true },None).await.expect("the open runs") else {
         panic!("the store's supersede law admits withdrawing the operation without an inverse");
     };
-    owners.run(&mut document, TimeTravelStoreCommand::Adopt(true)).await.expect("the session adopts the kind");
+    owners.run(&mut document, TimeTravelStoreCommand::Adopt(true),None).await.expect("the session adopts the kind");
     drafts.insert(children.clone(), protocol::InputReplacement::Withdrawn);
     let half = replayed_report(&mut owners, &mut document, &drafts, &count).await;


### 46 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧫️fixtures/⏳️completion/🔣️.json
B/H/S/current: 260/260/180/205
--- 
+++ 
@@ -89,88 +89,32 @@
         "family": "puzzle",
         "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs",
+        "retainedOwner": "self.rejected_completion = Some(ArtifactToolCompletionValue::Emit(rejected.emit.map_err(ArtifactCompletionFault::new),rejected.ephemeral))",
         "terminalGuard": "ArtifactRetainedCommandPhase::Fault =>",
         "legacyLossToken": "completion.complete(Ok(emit), EphemeralEmit::default()).is_err()",
-        "incrementalClose": "self.close_one_original(same,loan)?",
+        "incrementalClose": "A::admit_completion_retirement(&mut self.rejected_completion,grant)",
         "retainedPhysicalOwners": [
-          "self.ephemeral = Some(rejected.ephemeral)",
-          "self.reducer_fault(rejected.fault)",
-          "&& self.emit.is_none()",
-          "&& self.ephemeral.is_none()",
-          "&& self.fault_retirement.is_none()"
-        ],
-        "sourceStart": "impl<A: ArtifactApp> ArtifactRetainedCommandJob<A> {",
-        "sourceEnd": "//#endregion 🧵️Job",
-        "completion": "completion.complete(Ok(emit), ephemeral)",
-        "retainedOwner": "self.emit = rejected.emit.ok();",
-        "bindings": [
-          {
-            "source": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
-            "tokens": [
-              "type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<EditorApp<Puzzle3dPlayApp>>;",
-              "ArtifactRetainedCommandJob::from_wire(payload, input)"
-            ]
-          },
-          {
-            "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs",
-            "tokens": [
-              "self.advance_original(cx)",
-              "emit.close_child_one(child)",
-              "fn close_one_original(&mut self,cx:&mut StepContext"
-            ]
-          }
+          "&& self.rejected_completion.is_none()",
+          "&& self.rejected_retirement.is_none()"
         ]
       },
       {
         "family": "draw",
+        "retainedOwner": "self.pending_completion_rejection = Some(rejected)",
         "source": "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
         "terminalGuard": "if self.pending_completion_rejection.is_some()",
         "legacyLossToken": "payload.completion.complete(emit, semio_framework_plugin::EphemeralEmit::default()).is_err()",
         "incrementalClose": "emit.close_child_one(grant)",
-        "retainedPhysicalOwners": [
-          "self.pending_completion_rejection.is_none()",
-          "self.payload.is_none()",
-          "self.raw_input.is_none()",
-          "self.decoder.is_none()",
-          "self.outcomes.terminal_is_empty()"
-        ],
-        "sourceStart": "struct DrawingGestureOperationJob {",
-        "sourceEnd": "struct DrawingGestureOperationJobFactory {",
-        "completion": "payload.completion.complete(Ok(emit), ephemeral)",
-        "retainedOwner": "self.pending_completion_rejection = Some(rejected)",
-        "bindings": []
+        "retainedPhysicalOwners": []
       },
       {
         "family": "writer",
+        "retainedOwner": "self.rejected_completion = Some(ArtifactToolCompletionValue::Emit(rejected.emit.map_err(ArtifactCompletionFault::new),rejected.ephemeral))",
         "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs",
         "terminalGuard": "ArtifactRetainedCommandPhase::Fault =>",
         "legacyLossToken": "completion.complete(Ok(emit), EphemeralEmit::default()).is_err()",
-        "incrementalClose": "self.close_one_original(same,loan)?",
+        "incrementalClose": "A::admit_completion_retirement(&mut self.rejected_completion,grant)",
         "retainedPhysicalOwners": [
-          "self.ephemeral = Some(rejected.ephemeral)",
-          "self.reducer_fault(rejected.fault)",
-          "&& self.emit.is_none()",
-          "&& self.ephemeral.is_none()",
-          "&& self.fault_retirement.is_none()"
-        ],
-        "sourceStart": "impl<A: ArtifactApp> ArtifactRetainedCommandJob<A> {",
-        "sourceEnd": "//#endregion 🧵️Job",
-        "completion": "completion.complete(Ok(emit), ephemeral)",
-        "retainedOwner": "self.emit = rejected.emit.ok();",
-        "bindings": [
-          {
-            "source": "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
-            "tokens": [
-              "type Job = ArtifactRetainedCommandJob<EditorApp<WriterPlayApp>>;",
-              "ArtifactRetainedCommandJob::from_wire(payload, input)"
-            ]
-          },
-          {
-            "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs",
-            "tokens": [
-              "self.advance_original(cx)",
-              "emit.close_child_one(child)",
-              "fn close_one_original(&mut self,cx:&mut StepContext"
-            ]
-          }
+          "&& self.rejected_completion.is_none()",
+          "&& self.rejected_retirement.is_none()"
         ]
       }
@@ -179,69 +123,45 @@
       {
         "family": "puzzle5d-copy",
-        "source": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖌️session/🦀️.rs",
-        "sourceStart": "struct Puzzle5dClipboardJob {",
-        "sourceEnd": "enum Puzzle5dPasteStage {",
+        "source": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
+        "sourceStart": "\nstruct Puzzle5dClipboardJob {",
+        "sourceEnd": "\nstruct Puzzle5dPasteOwners {",
         "terminalGuard": "if self.rejection.is_some()",
         "retainedOwner": "self.rejection = Some(rejected)",
+        "handoff": "completion.complete(",
         "incrementalClose": "crate::puzzle_job::job_close_step(&mut self.outbox, &mut self.owners, grant)",
-        "legacyLossToken": "if let Err(error) = completion.complete",
-        "bindings": [],
-        "completion": "completion.complete(",
-        "retainedPhysicalOwners": [
-          "self.outbox.terminal_is_empty()",
-          "self.owners.is_empty()",
-          "rejection: self.rejection.take()"
-        ]
+        "legacyLossToken": "completion.complete(Ok(emit), EphemeralEmit::default()).is_err()"
       },
       {
         "family": "puzzle5d-cut",
-        "source": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖌️session/🦀️.rs",
-        "sourceStart": "struct Puzzle5dClipboardJob {",
-        "sourceEnd": "enum Puzzle5dPasteStage {",
+        "source": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
+        "sourceStart": "\nstruct Puzzle5dClipboardJob {",
+        "sourceEnd": "\nstruct Puzzle5dPasteOwners {",
         "terminalGuard": "if self.rejection.is_some()",
         "retainedOwner": "self.rejection = Some(rejected)",
+        "handoff": "completion.complete(",
         "incrementalClose": "crate::puzzle_job::job_close_step(&mut self.outbox, &mut self.owners, grant)",
-        "legacyLossToken": "if let Err(error) = completion.complete",
-        "bindings": [],
-        "completion": "completion.complete(",
-        "retainedPhysicalOwners": [
-          "self.outbox.terminal_is_empty()",
-          "self.owners.is_empty()",
-          "rejection: self.rejection.take()"
-        ]
+        "legacyLossToken": "completion.complete(Ok(emit), EphemeralEmit::default()).is_err()"
       },
       {
         "family": "puzzle5d-paste",
-        "source": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖌️session/🦀️.rs",
-        "sourceStart": "\npub(super) struct Puzzle5dPasteJob {",
-        "sourceEnd": "\nenum Puzzle5dImportStage {",
+        "source": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
+        "sourceStart": "\nstruct Puzzle5dPasteJob {",
+        "sourceEnd": "\nstruct Puzzle5dImportOwners {",
         "terminalGuard": "puzzle5d paste completion remains rejected",
         "retainedOwner": "self.pending_completion_rejection = Some(rejected)",
+        "handoff": "completion.complete(",
         "incrementalClose": "crate::puzzle_job::job_close_step(&mut self.outbox, &mut self.owners, grant)",
-        "legacyLossToken": "if let Err(error) = completion.complete",
-        "bindings": [],
-        "completion": "completion.complete(",
-        "retainedPhysicalOwners": [
-          "self.outbox.terminal_is_empty()",
-          "self.owners.is_empty()",
-          "pending_completion_rejection: self.pending_completion_rejection.take()"
-        ]
+        "legacyLossToken": "completion.complete(Ok(emit), EphemeralEmit::default()).is_err()"
       },
       {
         "family": "puzzle5d-import",
-        "source": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖌️session/🦀️.rs",
-        "sourceStart": "\npub(super) struct Puzzle5dImportJob {",
+        "source": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
+        "sourceStart": "\nstruct Puzzle5dImportJob {",
         "sourceEnd": "\n//#endregion 🧵️ReservedJobs",
         "terminalGuard": "puzzle5d import completion remains rejected",
         "retainedOwner": "self.pending_completion_rejection = Some(rejected)",
+        "handoff": "completion.complete(",
         "incrementalClose": "crate::puzzle_job::job_close_step(&mut self.outbox, &mut self.owners, grant)",
-        "legacyLossToken": "if let Err(error) = completion.complete",
-        "bindings": [],
-        "completion": "completion.complete(",
-        "retainedPhysicalOwners": [
-          "self.outbox.terminal_is_empty()",
-          "self.owners.is_empty()",
-          "pending_completion_rejection: self.pending_completion_rejection.take()"
-        ]
+        "legacyLossToken": "completion.complete(Ok(emit), EphemeralEmit::default()).is_err()"
       }
     ]


### 47 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧫️fixtures/🕹️selection-set.json
B/H/S/current: 57/71/59/71
--- 
+++ 
@@ -50,8 +50,10 @@
   "wiring": {
     "why": "Source-level anchors the Node twin re-checks, so the two flatten sites can never drift back apart into one deduping and one not — which is exactly how the live defect was introduced.",
-    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
-    "projection": "pub(crate) fn leftover_selected_ids_of(state: &protocol::InteractionState) -> Vec<String> {",
-    "callSites": ["            leftover_selected_ids_of(self.state)", "            let selected_ids = leftover_selected_ids_of(state);"],
-    "lockedFromSelectedIds": "            let locked: Vec<(String, DslValue)> = selected_ids.iter().map(|id| (id.clone(), DslValue::Bool(false))).collect();"
+    "projection": { "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/🧭️topology/🦀️.rs", "source": "pub(crate) fn leftover_selected_ids_of(state: &protocol::InteractionState) -> Vec<String> {" },
+    "callSites": [
+      { "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🦀️.rs", "source": "leftover_selected_ids_of(self.state)" },
+      { "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs", "source": "let selected_ids = leftover_selected_ids_of(state);" }
+    ],
+    "lockedFromSelectedIds": { "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs", "source": "let locked: Vec<(String, DslValue)> = selected_ids.iter().map(|id| (id.clone(), DslValue::Bool(false))).collect();" }
   }
 }


### 48 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs
B/H/S/current: 1448/45/1445/46
--- 
+++ 
@@ -31,4 +31,5 @@
 use semio_framework::{MediaForm, ViewModel, REVERT_TO_COMMAND_ACTION_ID};
 use protocol::MutationDiff;
+use protocol::ColdMutationDiff;
 use {semio_framework::action_bus,semio_framework::ActionKind,semio_framework_artifact_reference::Dialect,semio_framework::Fault,semio_framework::FaultOrigin,semio_framework::IconName,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework::ToolExecutionContract,semio_framework::ToolFactoryKey,semio_framework::ToolJobFactory,semio_framework::ToolOperationSpec};
 use semio_framework_2d::compute::EngineHandles;
@@ -156,7 +157,7 @@
 
 impl store::os_store::serialization::ArtifactSqliteSnapshotHost for SurfaceSnapshot {
-    
-    
-    
+
+
+
 }
 impl pack::snapshot::ArtifactSqliteSnapshot for SurfaceSnapshot {
@@ -203,5 +204,5 @@
 impl pack::snapshot::ArtifactPack for SurfaceSnapshot {
     /// 🪶️ Publishes this owner's actual relational snapshot capability.
-    
+
 
     fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
@@ -227,20 +228,31 @@
 
 impl protocol::DiffAlgebra<SurfaceSnapshot> for SurfaceDiff {
+
+    fn is_empty(&self) -> bool {
+        self.count.is_none()
+    }
+}
+
+impl protocol::ColdDiffAlgebra<SurfaceSnapshot> for SurfaceDiff {
     fn inverse(&self, base: &SurfaceSnapshot) -> Self {
         Self { count: self.count.map(|_| base.count) }
     }
-    fn is_empty(&self) -> bool {
-        self.count.is_none()
-    }
 }
 
 impl MutationDiff<SurfaceSnapshot> for SurfaceDiff {
-    fn apply(&self, snapshot: &SurfaceSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<SurfaceSnapshot> {
-        Ok(SurfaceSnapshot { count: self.count.unwrap_or(snapshot.count) })
-    }
+
+
+}
+
+impl ColdMutationDiff<SurfaceSnapshot> for SurfaceDiff {
     fn absorb(&mut self, other: Self) {
         if other.count.is_some() {
             self.count = other.count;
         }
+    }
+
+    fn apply(&self, snapshot: &mut SurfaceSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
+                if let Some(value) = self.count { snapshot.count = value; }
+        Ok(())
     }
 }
@@ -339,5 +351,5 @@
             return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(Into::into);
         }
-        let Some(SurfaceEditorCommand::Increment) = self.owners.command.as_deref() else {
+        let Some(SurfaceEditorCommand::Increment) = self.owners.command() else {
             return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into);
         };
@@ -397,20 +409,7 @@
         SURFACE_TOOL_CONTRACT
     }
-    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
-        Ok(payload)
-    }
-    fn create_job_from_wire_pages_with_payload(
-        &mut self,
-        _operation: semio_framework_job::Operation,
-        mut payload: Self::Payload,
-        input: action_bus::RetainedToolWireInput,
-        checkpoint: Option<action_bus::RetainedToolWireInput>,
-    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, action_bus::RetainedToolWireInput, Option<action_bus::RetainedToolWireInput>)> {
-        if checkpoint.is_some() {
-            return Err((semio_framework::ToolJobFactoryError::new("surface fixture resume starts a fresh command owner"), input, checkpoint));
-        }
-        payload.owners.raw = Some(input);
-        Ok(payload)
-    }
+    fn admission_demands(&self,_operation:&semio_framework_job::Operation,payload:&Self::Payload,input:Option<&semio_framework::RetainedToolWireInput>,checkpoint:Option<&semio_framework::RetainedToolWireInput>)->Result<semio_framework::action_bus::ToolJobAdmissionDemand,semio_framework_value::ValueError>{if checkpoint.is_some()||(input.is_some()&&payload.owners.raw.is_some()){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original typed job refuses a replacement wire owner or restart checkpoint"));}Ok(semio_framework::action_bus::ToolJobAdmissionDemand::reused_job())}
+fn admit_job(&mut self,context:&mut semio_framework_job::StepContext<'_>,operation:&semio_framework_job::Operation,payload:&mut Option<Box<Self::Payload>>,input:&mut Option<semio_framework::RetainedToolWireInput>,checkpoint:&mut Option<semio_framework::RetainedToolWireInput>,output:&mut Option<Box<Self::Job>>)->Result<bool,semio_framework_job::OriginalTurnFailure>{let demand=self.admission_demands(operation,payload.as_deref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original typed job payload is absent"))?,input.as_ref(),checkpoint.as_ref())?;semio_framework::action_bus::admit_original_tool_job_controlled(context,operation,payload,input,checkpoint,output,demand,|_,payload,input,_|{if input.is_some(){payload.as_mut().expect("retained original job").owners.raw=input.take();}let job=payload.take().expect("admitted same original job");Ok((job,semio_framework_job::RetainedCloneProgress{copied_items:1,..Default::default()}))})}
+
 }
 
@@ -594,8 +593,5 @@
     }
 
-    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
-        let job = SurfaceFixtureJob { owners: crate::app_testing::mutation_fixture::job_close::FixtureJobOwners::new(request.command, request.completion).with_context(request.context), count: request.snapshot.count, page: 0 };
-        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, job, request.operation)))
-    }
+    fn build_tool_job(request:&mut Option<ArtifactOwnedToolJobRequest<EditorApp<Self>>>,output:&mut Option<semio_framework::ErasedToolJob>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{crate::app_testing::mutation_fixture::job_close::admit_fixture_tool_job(request,output,cx,|request|{let count=request.snapshot.count;SurfaceFixtureJob{owners:crate::app_testing::mutation_fixture::job_close::FixtureJobOwners::from_request(request),count,page:0}})}
 
     fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
@@ -1191,5 +1187,5 @@
             cx.consume_fuel(self.consumes);
             Ok(ArtifactCommandWorkStep::Complete(Emit::default()))
-        
+
         })();
         original.map_err(semio_framework_job::OriginalTurnFailure::from)
@@ -1210,5 +1206,5 @@
     }
     for row in fixture["cases"].as_array().unwrap() {
-        let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().unwrap();let mut cx=original_caller.context();let recipient=cx.original_recipient_identity();
+        let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().unwrap();let mut decoder=semio_framework_job::OriginalJobDecodeOwner::empty();let mut cx=original_caller.context().with_original_decoder(&mut decoder);let recipient=cx.original_recipient_identity();
         let mut app=new_app::<EditorApp<SurfaceEditorFixture>>(protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()),crate::app_testing::artifact_app_laws::fixture_mounted_policy(),&mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
         let observed = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(u64::MAX));
@@ -1224,4 +1220,5 @@
         }, |_| SURFACE_TOOL_ID, 32, 1, Box::new(work));
         let mut job = ArtifactRetainedCommandJob::new(payload);
+        assert!(job.test_admit_original_source(&mut cx).expect("actual original source binds the preborn fixture job"));
         let mut observed_work=false;
         for _ in 0..crate::app_testing::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){
@@ -1237,5 +1234,5 @@
         assert!(consumer.take_emit().unwrap().is_none());
         crate::app_testing::mutation_fixture::job_close::retire_completion(consumer,&mut cx,SURFACE_ONE_UNIT_POLICY);
-        close_registered_fixture_app(&mut app,crate::app_testing::artifact_app_laws::fixture_mounted_policy(),&mut cx).unwrap();assert_eq!(cx.original_recipient_identity(),recipient);let fuel=cx.fuel_remaining();drop(cx);original_caller.retire(fuel).unwrap();
+        close_registered_fixture_app(&mut app,crate::app_testing::artifact_app_laws::fixture_mounted_policy(),&mut cx).unwrap();assert_eq!(cx.original_recipient_identity(),recipient);let fuel=cx.fuel_remaining();drop(cx);let fuel=original_caller.retire_decoder(&mut decoder,fuel).unwrap();original_caller.retire(fuel).unwrap();
         println!("[DEBUG] Retained command original work fuel case={} domainDebit={} fallbackOnce=true sameRecipient=true",row["id"],row["domainConsumes"]);
     }
@@ -1313,5 +1310,5 @@
             let request = store::MemberStoreOwnedBatchRequest { operation: semio_framework_job::OperationId(7101), expected_generation: generation, expected_revision: revision, actor: protocol::LOCAL_ACTOR_ID.into(), group_id: Some("private-neutral-three-lane".into()), transaction: None, mutations };
             let mut lane = PrivateOwnedPublicationLane::new(request);
-            let admitted = lane.next_capacity_byte_demand(&app.stores.store).unwrap();
+            let admitted = lane.next_capacity_byte_demand(&*app.stores.store).unwrap();
             for (items, bytes) in [(0, admitted), (1, admitted - 1)] {
                 let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| lane.advance(&mut app.stores.store, &visibility, store::os_store::one_item::ArtifactStoreOneItemGrant { maximum_items: items, maximum_capacity_bytes: bytes, ..grant }).unwrap());
@@ -1327,5 +1324,5 @@
         let mut index = 0;
         while index < lanes.len() && stop.is_none_or(|stop| turns < stop) {
-            lanes[index].advance(&mut apps[index].store, &visibility, grant).unwrap();
+            lanes[index].advance(&mut *apps[index].store, &visibility, grant).unwrap();
             if lanes[index].prepared_for_staging() {
                 let identity = lanes[index].prepared_edit_id().unwrap();
@@ -1370,5 +1367,5 @@
                 assert!(demand.copy_bytes <= grant.maximum_copy_bytes && demand.capacity_bytes <= grant.maximum_capacity_bytes && demand.release_bytes <= grant.maximum_release_bytes && demand.depth <= grant.maximum_depth);
                 let exact = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
-                let step = lanes[index].close_step(&mut apps[index].store, exact).unwrap();
+                let step = lanes[index].close_step(&mut *apps[index].store, exact).unwrap();
                 assert!(step.progress().fits(exact));
                 if matches!(step, semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) { break; }


### 49 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧾️document-archive-load-legs/🦀️.rs
B/H/S/current: 362/0/362/0
--- 
+++ 
@@ -174,5 +174,5 @@
     let mut caller=crate::app_testing::OriginalFixtureCaller::admit().expect("original composition caller input");let mut cx=caller.context();
     let mut app = Box::pin(VcsArtifactApp::<SingleDocumentApp, TestMembers>::new(SingleDocumentApp, protocol::ActorId(crate::app::LOCAL_ACTOR_ID.into()), crate::app::artifact_app_laws::fixture_mounted_policy(), &mut crate::app::artifact_app_laws::fixture_identity())).await;
-    let effects = Box::pin(app.pending_effects(None)).await;
+    let effects = Box::pin(crate::artifact_app_laws::fixture_pending_effects(&mut app, None)).await.unwrap();
     let Some(Effect::LoadDocument { pack, spr }) = effects.first().cloned() else { panic!("the refresh poll lane dropped the LoadDocument effect: {effects:?}") };
     let archive = protocol::DocumentArchivePack { parent_pack: pack, parent_spr: spr, members: Vec::new() };


### 50 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/👷️jobs/📥️staging/🧾️typed-operations/🦀️.rs
B/H/S/current: 272/297/272/297
--- 
+++ 
@@ -5,9 +5,9 @@
 
 impl<A: ArtifactApp> PendingReserved<A> {
-    pub(crate) fn receive_configuration_failure(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{if self.pending_configuration_failure.is_none(){return Ok(false)}if self.pending_configuration_identity!=Some((cx.operation(),cx.generation())){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"configuration failure requires its unchanged original caller").into())}if let Some(failure)=semio_framework_job::admit_original_domain_failure(&mut self.pending_configuration_failure,cx)?{self.pending_configuration_identity=None;return Err(failure)}Ok(true)}
-
-    pub(crate) fn configuration_failure_terminal_is_empty(&self)->bool{self.pending_configuration_completion.is_none()&&self.pending_configuration_command.is_none()&&self.pending_configuration_command_close.is_none()&&self.pending_configuration_permit.is_none()&&self.pending_configuration_failure.is_none()&&self.pending_configuration_retirement.is_none()&&self.pending_configuration_identity.is_none()}
-
-    pub(crate) fn configuration_failure_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(owner)=self.pending_configuration_completion.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{owner.demand(body)}}if let Some(owner)=self.pending_configuration_command_close.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{owner.selected_demand(body)}}self.pending_configuration_retirement.as_ref().map_or(Ok(RetirementDemand{depth:usize::from(!self.configuration_failure_terminal_is_empty()),..Default::default()}),|owner|if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{owner.selected_demand(body)})}
+    pub(crate) fn receive_configuration_failure(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{if self.pending_configuration_failure.is_none(){return Ok(false)}if self.pending_configuration_identity!=Some((cx.operation(),cx.generation())){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"configuration failure requires its unchanged original caller").into())}if let Some(failure)=semio_framework_job::admit_original_domain_failure(&mut self.pending_configuration_failure,cx)?{return Err(failure)}Ok(true)}
+
+    pub(crate) fn configuration_failure_terminal_is_empty(&self)->bool{!self.pending_configuration_rejected&&self.pending_configuration_source.is_none()&&self.pending_configuration_worker.is_none()&&self.pending_configuration_completion.is_none()&&self.pending_configuration_command.is_none()&&self.pending_configuration_command_close.is_none()&&self.pending_configuration_permit.is_none()&&self.pending_configuration_failure.is_none()&&self.pending_configuration_retirement.is_none()&&self.pending_configuration_identity.is_none()}
+
+    pub(crate) fn configuration_failure_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(worker)=self.pending_configuration_worker.as_ref(){return worker.close_demands(body)}if let Some(owner)=self.pending_configuration_completion.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{owner.demand(body)}}if let Some(owner)=self.pending_configuration_command_close.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{owner.selected_demand(body)}}self.pending_configuration_retirement.as_ref().map_or(Ok(RetirementDemand{depth:usize::from(!self.configuration_failure_terminal_is_empty()),..Default::default()}),|owner|if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{owner.selected_demand(body)})}
 }
 
@@ -34,11 +34,11 @@
             let Some(operation)=self.tool_operations.get_mut(operation_id)else{break};
             if operation.stage!=MountedTypedCommandFullOperationStage::Worker{break;}
-            let closing=operation.terminal_seen||operation.session_rejected.is_some()||operation.worker_outcome_pending;
+            let closing=operation.terminal_seen||operation.user_cancel_requested||cx.is_cancelled()||operation.producer_request_retirement.is_some()||operation.session_rejected.is_some()||operation.worker_outcome_pending;
             if turn.is_some_and(|(kind,_)|kind!=closing){break;}
             let policy=if closing{mounted_policy.close}else{mounted_policy.preparation};
-            let (_,turn_grant)=*turn.get_or_insert((closing,crate::plugin_runtime::original_plugin_turn_grant(policy,retained)));
-            let grant=crate::plugin_runtime::original_plugin_turn_grant(policy,crate::plugin_runtime::remaining_plugin_turn_grant(turn_grant,spent));
+            let (_,turn_grant)=*turn.get_or_insert((closing,semio_framework_plugin_ownership::bounded_ownership::original_plugin_turn_grant(policy,retained)));
+            let grant=semio_framework_plugin_ownership::bounded_ownership::original_plugin_turn_grant(policy,crate::plugin_runtime::remaining_plugin_turn_grant(turn_grant,spent));
             let stepped=cx.with_retained_loan(grant,|same|operation.drive_worker_step(same));
-            let progress=match &stepped{Ok(step)=>step.progress().unwrap_or_default(),Err(failure)=>failure.primary.retained_progress()};
+            let progress=match &stepped{Ok(step)=>step.progress().unwrap_or_default(),Err(failure)=>failure.retained_progress()};
             spent=spent.checked_add(progress)?;
             if let Err(fault)=stepped{failure=Some(fault);break;}
@@ -243,5 +243,5 @@
     }
 
-    pub(crate) fn close_configuration_failure(&mut self, pending: &mut PendingReserved<A>,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,crate::PluginLifecycleFailure>{let demand=pending.configuration_failure_demands(grant.maximum_copy_bytes)?;semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|_,grant|{if let Some(owner)=pending.pending_configuration_completion.as_mut(){if owner.terminal_is_empty(){pending.pending_configuration_completion.take();return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))}let step=owner.step(grant)?;return Ok((PluginLifecycleStep::Progress(step.progress()),false))}if let Some(owner)=pending.pending_configuration_command_close.as_mut(){if owner.terminal_is_empty(){pending.pending_configuration_command_close.take();return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))}let step=owner.step(grant)?;return Ok((PluginLifecycleStep::Progress(step.progress()),false))}if let Some(command)=pending.pending_configuration_command.take(){match ControlledRetirement::new(command){Ok(owner)=>pending.pending_configuration_command_close=Some(owner),Err((error,command))=>{*pending.pending_configuration_command=Some(command);return Err(error.into())}}return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))}if let Some(permit)=pending.pending_configuration_permit.take(){self.finish_reserved_emit_producer(permit);return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))}if let Some(owner)=pending.pending_configuration_retirement.as_mut(){if owner.terminal_is_empty(){pending.pending_configuration_retirement.take();return Ok((PluginLifecycleStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}),true))}let step=owner.step(grant)?;return Ok((PluginLifecycleStep::Progress(step.progress()),false))}if let Some(source)=pending.pending_configuration_failure.take(){match ControlledRetirement::new(source){Ok(owner)=>pending.pending_configuration_retirement=Some(owner),Err((error,source))=>{*pending.pending_configuration_failure=Some(source);return Err(error.into())}}return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))}pending.pending_configuration_identity=None;Ok((PluginLifecycleStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}),true))})}
+    pub(crate) fn close_configuration_failure(&mut self, pending: &mut PendingReserved<A>,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,crate::PluginLifecycleFailure>{if pending.pending_configuration_source.is_some_and(|source|!source.matches(cx)){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"configuration retirement refused different original source roots or clock").into())}if let Some(worker)=pending.pending_configuration_worker.as_mut(){worker.cancel();let permit=pending.pending_configuration_permit.as_ref().expect("configuration close retains its original permit");let done=cx.with_retained_loan(grant,|same|worker.close_step(same,permit.lease.cancel_token())).map_err(crate::PluginLifecycleFailure::from)?;if done{let mut worker=pending.pending_configuration_worker.take().unwrap();if let Some(failure)=worker.take_failure(){assert!(pending.pending_configuration_failure.is_none());*pending.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Original(failure));}assert!(worker.terminal_is_empty());}return Ok(PluginLifecycleStep::Progress(Default::default()))}let demand=pending.configuration_failure_demands(grant.maximum_copy_bytes)?;semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|_,grant|{if let Some(owner)=pending.pending_configuration_completion.as_mut(){if owner.terminal_is_empty(){pending.pending_configuration_completion.take();return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))}let step=owner.step(grant)?;return Ok((PluginLifecycleStep::Progress(step.progress()),false))}if let Some(owner)=pending.pending_configuration_command_close.as_mut(){if owner.terminal_is_empty(){pending.pending_configuration_command_close.take();return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))}let step=owner.step(grant)?;return Ok((PluginLifecycleStep::Progress(step.progress()),false))}if let Some(command)=pending.pending_configuration_command.take(){match ControlledRetirement::new(command){Ok(owner)=>pending.pending_configuration_command_close=Some(owner),Err((error,command))=>{*pending.pending_configuration_command=Some(command);return Err(error.into())}}return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))}if let Some(permit)=pending.pending_configuration_permit.take(){self.finish_reserved_emit_producer(permit);return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))}if let Some(owner)=pending.pending_configuration_retirement.as_mut(){if owner.terminal_is_empty(){pending.pending_configuration_retirement.take();return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))}let step=owner.step(grant)?;return Ok((PluginLifecycleStep::Progress(step.progress()),false))}if let Some(source)=pending.pending_configuration_failure.take(){match ControlledRetirement::new(source){Ok(owner)=>pending.pending_configuration_retirement=Some(owner),Err((error,source))=>{*pending.pending_configuration_failure=Some(source);return Err(error.into())}}return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))}pending.pending_configuration_identity=None;pending.pending_configuration_source=None;pending.pending_configuration_rejected=false;Ok((PluginLifecycleStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}),true))})}
 
     /// 🪪️ The immutable identity this instance's live document revision is observed under — the ONE


### 51 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📨️dispatch/📥️staging/⌨️typed-command/🦀️.rs
B/H/S/current: 427/460/400/436
--- 
+++ 
@@ -50,5 +50,5 @@
                         meta: meta.clone(),
                         operation,
-                        source_identity:(cx.operation(),cx.generation()),
+                        source_identity:semio_framework_job::OriginalSourceBinding::borrow(cx),
                         revision,
                         lease: None,
@@ -144,4 +144,5 @@
         assert!(self.tools.tool_operations.can_insert(operation_id.0), "typed worker handoff retains its exact empty operation slot");
         assert_eq!(self.tools.typed_operation_reservations[operation_id.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS], Some(operation_id.0), "worker handoff owns its exact pending reservation");
+        if !matches!(&admission.proof,QualifiedToolProof::AppOwned(_)){return Err(plugin_sdk_fault("typed worker handoff requires its exact registered application producer"));}
         let verb = admission.verb.clone();
         let TypedCommandRoots {
@@ -179,8 +180,6 @@
         drop(identity_observer);
         completion.install_authoring_identity(identity, &cancellation_lease)?;
-        let retained_wire_admission = admission.wire_admission.clone();
         let app_owned_retained_route = matches!(&admission.proof, QualifiedToolProof::AppOwned(_));
         let mut retained_wire = Some(admission.raw_wire);
-        let raw_input = if app_owned_retained_route { ArtifactToolRawInput::transferred_to_factory() } else { ArtifactToolRawInput::new(retained_wire.take().expect("generic retained wire owner remains mounted")) };
         let output_chunks = ArtifactOutputChunks::new(admission.proof.contract().max_output_bytes);
         let mut request_snapshot_read = if app_owned_retained_route { Some(self.stores.store.snapshot_read().map_err(FaultFrom::into_fault)?) } else { None };
@@ -211,39 +210,10 @@
             artifact_owned_tool_birth_grant(),
         ).0) } else { None };
-        let operation_spec = match admission.proof.clone() {
-            QualifiedToolProof::Bounded(_) => {
-                let job = TypedCommandFullOperationJob::<A> {
-                    operation: None,
-                    parent_document_id: Some(parent_document_id.clone()),
-                    command: Some(std::sync::Arc::from(command)),
-                    snapshot: Some(snapshot),
-                    config: Some(config),
-                    history: Some(history),
-                    children: Some(std::sync::Arc::clone(&children)),
-                    draft: Some(draft_snapshot),
-                    interaction_state: Some(interaction_state),
-                    interaction_hover: Some(std::sync::Arc::new(interaction_hover)),
-                    peer_presence: Some(peer_presence),
-                    presence_local: Some(self.stores.presence_store.local_read().map_err(Fault::from)?),
-                    presence_peers: Some(presence_peers),
-                    transient: Some(transient),
-                    contract: admission.proof.contract(),
-                    decoded_items: admission.decoded_items,
-                    phase: TypedCommandFullOperationPhase::Prepare,
-                    prepare_cursor: 0,
-                    reducer_cursor: 0,
-                    output: Some(completion.clone()),
-                    state: [0; 3],
-                    fault_message: b"",
-                    pending: None,
-                    outcomes: crate::tool_contract::ReservedJobOutcomes::new(),
-                    closing: false,
-                };
-                semio_framework::ToolOperationSpec::new(self.tools.tool_job_controller_id.clone(), verb.clone(), payload_schema_id, job, operation)
-            }
-            QualifiedToolProof::AppOwned(_) => A::build_tool_job(ArtifactOwnedToolJobRequest {
+        let producer_request = Some(ArtifactOwnedToolJobRequest {
+            source_binding: semio_framework::action_bus::OriginalToolSourceBinding::borrow(cx),
                 retained:self.mounted_policy.maintenance,
-                command,
-                raw_wire: raw_input.clone(),
+                command: crate::tool_contract::ArtifactOriginalToolCommand::from_admitted_op_binary(command,retained_wire.as_ref().expect("original typed wire remains retained"),semio_framework::action_bus::OriginalToolSourceBinding::borrow(cx)),
+                input: retained_wire.take(),
+                checkpoint: None,
                 operation,
                 controller_id: qualified_key.controller_id,
@@ -267,19 +237,6 @@
                 output_chunks: output_chunks.clone(),
                 completion: completion.clone(),
-            })
-            .await?
-            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.missing-owned-builder"), format!("app-owned tool '{verb}' registered a factory but supplied no exact payload builder")))?,
-            QualifiedToolProof::FrameworkOwned(_) => {
-                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.framework-route"), format!("framework-owned tool '{verb}' cannot enter the application typed-command reducer")));
-            }
-        };
-        let dispatch = if app_owned_retained_route {
-            self.tools.tool_jobs
-                .dispatch_wire_retained_with_spec(&retained_wire_admission, retained_wire.take().expect("app-owned retained wire owner transfers once"), None, operation_spec)
-                .map_err(|rejected| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), rejected.error.to_string()))?
-        } else {
-            self.tools.tool_jobs.dispatch(operation_spec).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), error.to_string()))?
-        };
-        let session=None;let session_rejected=Some(semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource::pending(dispatch.job,(cx.operation(),cx.generation())));
+            });
+        let session=None;let session_rejected=None;
         self.tools.tool_operations.insert_admitted(
             operation_id.0,
@@ -288,5 +245,5 @@
                 meta: meta.clone(),
                 operation,
-                source_identity:(cx.operation(),cx.generation()),
+                source_identity:semio_framework_job::OriginalSourceBinding::borrow(cx),
                 canonical_revision: canonical_base_revision,
                 artifact_generation: generation.0,
@@ -300,9 +257,12 @@
                 session,
                 session_rejected,
+                producer_source: producer_request.as_ref().map(|request|request.source_binding),
+                producer_request,
+                producer_request_retirement: None,
                 reserved_producer: None,
                 completion: Some(completion),
                 completion_retirement:None,
                 publication_retirement:None,
-                output_retirement:None,raw_input: (!app_owned_retained_route).then_some(raw_input),
+                output_retirement:None,raw_input: None,
                 output_chunks: Some(output_chunks),
                 cancellation_lease: Some(cancellation_lease),
@@ -391,9 +351,25 @@
     /// `apply_config_bytes` whole-record-replace legacy path).
     pub(super) async fn dispatch_config_command_inner(&mut self,command_bytes:&mut Vec<u8>,meta:&ActionMeta,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure>{
+        if self.pending.pending_configuration_source.is_some_and(|source|!source.matches(cx)){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"configuration source refused different original roots or clock").into())}
+        if self.pending.pending_configuration_rejected{self.tools.close_configuration_failure(&mut self.pending,cx,self.mounted_policy.close).map_err(semio_framework_job::OriginalTurnFailure::from)?;return Ok(None)}
+        if cx.is_cancelled(){if self.pending.pending_configuration_source.is_some(){self.pending.pending_configuration_rejected=true;self.tools.close_configuration_failure(&mut self.pending,cx,self.mounted_policy.close).map_err(semio_framework_job::OriginalTurnFailure::from)?;return Ok(None)}return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled,"configuration caller cancelled before any source was admitted").into())}
+        if self.pending.pending_configuration_worker.is_some(){
+            let permit=self.pending.pending_configuration_permit.as_ref().expect("configuration source retains its original permit");
+            let done=self.pending.pending_configuration_worker.as_mut().unwrap().advance(cx,permit.lease.cancel_token())?;
+            if !done{return Ok(None)}
+            let mut worker=self.pending.pending_configuration_worker.take().unwrap();let operation=worker.semantic_operation();let failure=worker.take_failure();assert!(worker.terminal_is_empty());drop(worker);
+            if let Some(failure)=failure{self.pending.pending_configuration_rejected=true;*self.pending.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Original(failure));let _=self.pending.receive_configuration_failure(cx)?;return Ok(None)}
+            let live=self.stores.store.content_revision();let revision=semio_framework_job::RevisionId(u64::from_be_bytes(live[..8].try_into().expect("revision lane width")));
+            if !matches!(semio_framework_job::validate_commit(&operation,revision,semio_framework_job::Generation(self.stores.store.generation())),semio_framework_job::CommitValidation::Accepted){self.pending.pending_configuration_rejected=true;*self.pending.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Original(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"configuration worker completed against a stale revision or generation").into()));let _=self.pending.receive_configuration_failure(cx)?;return Ok(None)}
+            let command=self.pending.pending_configuration_completion.as_ref().unwrap().original.as_ref().unwrap().lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"configuration binary job completed without a decoded command"))?;
+            match command{Ok(command)=>*self.pending.pending_configuration_command=Some(command),Err(source)=>{self.pending.pending_configuration_rejected=true;self.pending.pending_configuration_source=Some(semio_framework::action_bus::OriginalToolSourceBinding::borrow(cx));*self.pending.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Original(source));let _=self.pending.receive_configuration_failure(cx)?;return Ok(None)}}
+            return self.finish_configuration_command(meta,identity,cx).await
+        }
         if self.pending.receive_configuration_failure(cx)?{return Ok(None)}
-        if self.pending.pending_configuration_permit.is_some()&&self.pending.pending_configuration_completion.is_some(){return self.finish_configuration_command(meta,identity,cx).await}
-        let proof=self.tools.qualified_tool_proof(&*self.stores, "configuration-binary")?;let key=proof.key();
+        if self.pending.pending_configuration_permit.is_some()&&self.pending.pending_configuration_command.is_some(){return self.finish_configuration_command(meta,identity,cx).await}
+        let proof=self.tools.qualified_tool_proof(&*self.stores,"configuration-binary")?;let key=proof.key();
         let admission=match self.tools.tool_jobs.admit_exact_wire(key.controller_id,key.tool_id,proof.schema_id(),command_bytes){Ok(admission)=>admission,Err(source)=>{*self.pending.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Admission(source));self.pending.pending_configuration_identity=Some((cx.operation(),cx.generation()));let _=self.pending.receive_configuration_failure(cx)?;return Ok(None)}};
-        if self.pending.pending_configuration_permit.is_none(){let Some(authority)=self.admit_reserved_emit_producer("configuration-binary",meta,cx)?else{return Ok(None)};self.pending.pending_configuration_permit=Some(authority);self.pending.pending_configuration_identity=Some((cx.operation(),cx.generation()));return Ok(None)}
+        if !matches!(proof,QualifiedToolProof::FrameworkOwned(_)|QualifiedToolProof::AppOwned(_))||proof.contract().max_decoded_items<1||!proof.admits::<A>(&admission){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"configuration producer failed its exact owner/factory/schema/contract admission").into())}
+        if self.pending.pending_configuration_permit.is_none(){let Some(authority)=self.admit_reserved_emit_producer("configuration-binary",meta,cx)?else{return Ok(None)};self.pending.pending_configuration_permit=Some(authority);self.pending.pending_configuration_source=Some(semio_framework::action_bus::OriginalToolSourceBinding::borrow(cx));self.pending.pending_configuration_identity=Some((cx.operation(),cx.generation()));return Ok(None)}
         if self.pending.pending_configuration_identity!=Some((cx.operation(),cx.generation())){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"configuration admission requires its unchanged original receiving caller").into())}
         type Completion<M>=std::sync::Mutex<Option<Result<ArtifactCommand<M>,semio_framework_job::OriginalTurnFailure>>>;
@@ -402,11 +378,8 @@
         self.pending.pending_configuration_identity=Some((cx.operation(),cx.generation()));
         self.pending.pending_configuration_completion=Some(crate::reserved_jobs::ConfigurationDecodeRetirement{original:ManuallyDrop::new(Some(decoded.clone())),payload:None});
-        let job=FrameworkConfigurationBinaryJob::<A>{raw:std::mem::take(command_bytes),cursor:0,stage:FrameworkConfigurationBinaryStage::Envelope,operation:None,decoded:ManuallyDrop::new(Some(decoded)),decoded_close:None,receiving:ManuallyDrop::new(None),native:semio_framework_job::OriginalNativeDecodeSource::empty(semio_framework_job::OriginalNativeDecodePolicy{maximum_owned_bytes:crate::reserved_jobs::FRAMEWORK_CONFIGURATION_NATIVE_OWNED_BYTES}),failure:ManuallyDrop::new(None),state:[0;8],pending:None,outcomes:crate::tool_contract::ReservedJobOutcomes::new(),closing:false};
-        let authority=self.pending.pending_configuration_permit.take().unwrap();
-        let permit=match self.tools.run_framework_reserved_job(&self.mounted_policy, &mut *self.stores, "configuration-binary", &[], 1, job, meta, Some(admission), authority, cx).await{Ok(permit)=>permit,Err((source,permit))=>{self.pending.pending_configuration_permit=Some(permit);*self.pending.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Original(source));let _=self.pending.receive_configuration_failure(cx)?;return Ok(None)}};
-        self.pending.pending_configuration_permit=Some(permit);
-        let command=self.pending.pending_configuration_completion.as_ref().unwrap().original.as_ref().unwrap().lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().ok_or_else(||Fault::new(FaultOrigin::Framework,FaultCode::new("interactive-job.missing-output"),"configuration binary job completed without a decoded command"))?;
-        match command{Ok(command)=>*self.pending.pending_configuration_command=Some(command),Err(source)=>{*self.pending.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Original(source));let _=self.pending.receive_configuration_failure(cx)?;return Ok(None)}}
-        self.finish_configuration_command(meta,identity,cx).await
+        let job=FrameworkConfigurationBinaryJob::<A>{raw:std::mem::take(command_bytes),cursor:0,stage:FrameworkConfigurationBinaryStage::Envelope,operation:Some(self.pending.pending_configuration_permit.as_ref().unwrap().operation),decoded:ManuallyDrop::new(Some(decoded)),decoded_close:None,receiving:ManuallyDrop::new(None),native:semio_framework_job::OriginalNativeDecodeSource::empty(semio_framework_job::OriginalNativeDecodePolicy{maximum_owned_bytes:crate::reserved_jobs::FRAMEWORK_CONFIGURATION_NATIVE_OWNED_BYTES}),failure:ManuallyDrop::new(None),state:[0;8],pending:None,outcomes:crate::tool_contract::ReservedJobOutcomes::new(),closing:false};
+        let operation=self.pending.pending_configuration_permit.as_ref().unwrap().operation;
+        self.pending.pending_configuration_worker=Some(crate::reserved_jobs::configuration_worker::ConfigurationWorker::new(job,operation,proof.contract().max_output_bytes,proof.contract().max_work_units_per_step,self.mounted_policy.preparation,self.mounted_policy.close,cx));
+        Ok(None)
     }
 
@@ -423,5 +396,5 @@
         let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
         let command=self.pending.pending_configuration_command.take().expect("configuration commit retains its original typed command");cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);
-        let dispatched=self.stores.config_store.dispatch(command,identity).await;let permit=self.pending.pending_configuration_permit.take().unwrap();self.tools.finish_reserved_emit_producer(permit);if let Err(source)=dispatched{*self.pending.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Store(source));let _=self.pending.receive_configuration_failure(cx)?;return Ok(None)}self.pending.pending_configuration_identity=None;self.history.cache=None;Ok(Some(ToolOperations::<A>::empty_result("configCommand", meta, Vec::new(), Vec::new(), UiDirtyScope::Full).await))
+        let dispatched=self.stores.config_store.dispatch(command,identity).await;let permit=self.pending.pending_configuration_permit.take().unwrap();self.tools.finish_reserved_emit_producer(permit);if let Err(source)=dispatched{*self.pending.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Store(source));let _=self.pending.receive_configuration_failure(cx)?;return Ok(None)}self.pending.pending_configuration_identity=None;self.pending.pending_configuration_source=None;self.pending.pending_configuration_rejected=false;self.history.cache=None;Ok(Some(ToolOperations::<A>::empty_result("configCommand", meta, Vec::new(), Vec::new(), UiDirtyScope::Full).await))
     }
 }


### 52 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🦀️.rs
B/H/S/current: 1678/931/1663/935
--- 
+++ 
@@ -150,5 +150,5 @@
             && self.tools.agent_lane_preview.is_none()
             && self.media.media_submission_refusal_terminal_is_empty()
-            && self.ingress.action_ingress_sources_closed()
+            && self.tool_machines.ingress.action_ingress_sources_closed()
             && self.media.media_exports.is_empty()
             && self.pending.pending_reserved.is_empty()
@@ -231,5 +231,5 @@
         if self.interaction.local_interaction_query_receipt.is_some(){return Ok(())}
         if self.constructor.has_constructor_receipt() { return Ok(()); }
-        if self.constructor.store_constructor_is_pending() { ({ let __a0 = crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.preparation,retained); self.stores.drive_store_constructor_unit(&mut self.constructor, __a0) })?; return Ok(()); }
+        if self.constructor.store_constructor_is_pending() { ({ let __a0 = semio_framework_plugin_ownership::bounded_ownership::original_plugin_turn_grant(self.mounted_policy.preparation,retained); self.stores.drive_store_constructor_unit(&mut self.constructor, __a0) })?; return Ok(()); }
         if InstanceExtensions::holds_receipt(&self.time_travel, &self.tool_runs) || (0..ARTIFACT_LIVE_OUTPUT_SLOTS).any(|index|self.tools.tool_operations.entry(index).is_some_and(|(_,owner)|owner.publication_ownership_progress.is_some()||owner.original_retirement_receipt.is_some())){return Ok(());}
         if !self.pending.reserved_commits.is_empty() && self.pending.reserved_commit_outcome.is_none() {
@@ -413,11 +413,15 @@
     }
 
-    fn take_typed_operation_result_page(&mut self, receiver: u32) -> Option<TypedOperationResultPage> {
-        let (index, operation_id) = (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|offset| {
+    fn take_typed_operation_result_page(&mut self, receiver: u32,grant:RetainedCloneGrant) -> Result<(Option<TypedOperationResultPage> ,RetainedCloneProgress),Fault>{
+        let selected = (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|offset| {
             let index = (self.tools.typed_result_cursor + offset) % ARTIFACT_LIVE_OUTPUT_SLOTS;
             self.tools.tool_operations.entry(index).filter(|(_, operation)| operation.meta.instance_id == receiver && !operation.result_page_presented && operation.result_page.is_some()).map(|(id, _)| (index, *id))
-        })?;
-        self.tools.typed_result_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
-        self.tools.tool_operations.get_mut(operation_id)?.take_result_page()
+        });
+        let Some((index,operation_id))=selected else{return Ok((None,Default::default()))};let demand=self.typed_operation_result_page_demands(receiver)?;
+        if !plugin_turn_admitted(demand,grant)?{return Ok((None,Default::default()))}
+        let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-std::mem::size_of::<usize>(),..grant};let(page,mut progress)=self.tools.tool_operations.get_mut(operation_id).expect("selected result owner remains mounted").take_result_page(child)?;
+        if page.is_some(){self.tools.typed_result_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
+        progress.tools.copied_bytes+=std::mem::size_of::<usize>()
+    ;}Ok((page,progress))
     }
 
@@ -560,5 +564,5 @@
             None => reserved_kind.ok_or_else(|| plugin_sdk_fault(format!("window kind {} does not own action {}", address.window_kind_id, address.action_id)))?,
         };
-        if !self.ingress.action_source_capture_ready(&address.action_id, cx){return Ok(None)}
+        if !self.tool_machines.ingress.action_source_capture_ready(&address.action_id, cx){return Ok(None)}
         let log_generation_before = self.history.log_generation;
         let previous_kind = self.host_events.invocation_kind.replace(kind);
@@ -570,9 +574,9 @@
 
     async fn handle_response_action(&mut self, action: &str, args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>, semio_framework_job::OriginalTurnFailure> {
-        if !self.ingress.action_source_capture_ready(action, cx){return Ok(None)}
+        if !self.tool_machines.ingress.action_source_capture_ready(action, cx){return Ok(None)}
         let definition = self.tools.registry.app_command(action).await.ok_or_else(|| plugin_sdk_fault(format!("continuation command '{action}' is not owned by the receiving app")))?;
         validate_ui_dispatch_classification("receiving app", action, definition.semantics.execution.interactive_job)?;
         let kind = definition.kind;
-        let Some(command) = self.ingress.receive_action_command(action, args, original_fault, cx).await? else { return Ok(None) };
+        let Some(command) = self.tool_machines.ingress.receive_action_command(action, args, original_fault, cx).await? else { return Ok(None) };
         let wire = <A::Command as ::protocol::OpBinary>::encode_op(&command).map_err(|error| error.into_fault())?;
         let admission = self.tools.admit_command_wire(&self.stores, action, &wire, 1).await?;
@@ -610,7 +614,7 @@
         }
         let _ = self.tools.qualified_tool_proof(&self.stores, verb)?;
-        if !self.ingress.action_source_capture_ready(verb, cx){return Ok(None)}
-        let mut merged = if self.ingress.action_source_pending(){None}else{merge_ui_values(intent.args.as_ref(), intent.input.as_ref()).await?};
-        let Some(command)=self.ingress.receive_action_command(verb, &mut merged, &mut None, cx).await?else{return Ok(None)};
+        if !self.tool_machines.ingress.action_source_capture_ready(verb, cx){return Ok(None)}
+        let mut merged = if self.tool_machines.ingress.action_source_pending(){None}else{merge_ui_values(intent.args.as_ref(), intent.input.as_ref()).await?};
+        let Some(command)=self.tool_machines.ingress.receive_action_command(verb, &mut merged, &mut None, cx).await?else{return Ok(None)};
         self.dispatch_ctx().dispatch_typed(command, meta, identity, cx).await.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from)
     }
@@ -721,5 +725,5 @@
                 return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(MediaTransfers::<A>::transaction_fault(FaultOrigin::App, "transaction.member-rejected", format!("mutation outcome was rejected: {:?}", outcome.messages()))) };
             }
-            running = match protocol::apply_diff(outcome.diff(), &running) {
+            running = match protocol::apply_diff_cold(outcome.diff(), &running) {
                 Ok(next) => next,
                 Err(error) => return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(MediaTransfers::<A>::transaction_fault(FaultOrigin::App, "transaction.member-rejected", error.to_string())) },
@@ -1386,35 +1390,16 @@
     /// stays the COMMITTED snapshot: a poll decides about work over what has landed, never over a
     /// run's own provisional overlay.
-    async fn pending_effects(&mut self, view: Option<&ViewModel>) -> Vec<Effect> {
-        if self.render_ctx().refresh_cache().await.is_err() {
-            return Vec::new();
-        }
-        let render_operation = self.tools.live_render_operation(&self.mounted_policy, &self.stores);
-        let parent_document_id = self.stores.store.envelope().id.clone();
-        let snapshot_is_admitted = {
-            let (_, snapshot, _, _) = self.history.cache.as_ref().expect("cache refreshed above");
-            A::mounted_job_prepare_snapshot_read(render_operation, snapshot.as_ref())
+    async fn pending_effects(&mut self,view:Option<&ViewModel>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Vec<Effect>,Fault>{
+        if cx.should_yield(){return Ok(Vec::new())}
+        self.render_ctx().refresh_cache().await?;
+        let render_operation=self.tools.live_render_operation(&self.mounted_policy, &self.stores);
+        let snapshot_read={let(_,snapshot,_,_)=self.history.cache.as_ref().ok_or_else(||plugin_sdk_fault("pending poll cache unavailable"))?;if A::mounted_job_prepare_snapshot_read(render_operation,snapshot.as_ref()){Some(self.stores.store.snapshot_read().map_err(|error|error.into_fault())?)}else{None}};
+        let effects={
+            let(_,snapshot,config,_)=self.history.cache.as_ref().ok_or_else(||plugin_sdk_fault("pending poll cache unavailable"))?;
+            let doc=crate::ArtifactPendingView::new(snapshot.as_ref(),&self.child_composition.child_content_root,render_operation,&self.stores.store.envelope().id,snapshot_read,self.tool_runs.poll_view_for(view.and_then(|view|view.window_id.as_deref())),self.tool_machines.provisional_generation(),&self.tools.instance_operation_owner);
+            let cfg=ConfigView{snapshot:config.as_ref(),window:None};
+            A::pending_effects(&self.tools.instance_operation_owner,&doc,&cfg,view).await
         };
-        let snapshot_read = if snapshot_is_admitted {
-            match self.stores.store.snapshot_read() {
-                Ok(snapshot_read) => Some(snapshot_read),
-                Err(_) => return Vec::new(),
-            }
-        } else {
-            None
-        };
-        let instance_operation_owner = self.tools.instance_operation_owner.clone();
-        let effects = {
-            let VcsArtifactApp { app: _, cache, child_content_root, tool_runs, tool_machines, .. } = self;
-            let (_, snapshot, config, history) = cache.as_ref().expect("cache refreshed above");
-            let doc = ArtifactView::with_render_context(snapshot.as_ref(), history.as_ref(), ChildContentView::clone(child_content_root), render_operation, parent_document_id, snapshot_read)
-                .await
-                .with_instance(instance_operation_owner.clone())
-                .with_tool_run(tool_runs.view_for(view.and_then(|view| view.window_id.as_deref())))
-                .with_provisional_generation(tool_machines.provisional_generation());
-            let cfg = ConfigView { snapshot: config.as_ref(), window: None };
-            A::pending_effects(&instance_operation_owner, &doc, &cfg, view).await
-        };
-        self.stores.stamp_polled_load_document_effects(effects)
+        Ok(self.stamp_polled_load_document_effects(effects))
     }
 


### 53 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs
B/H/S/current: 752/372/752/372
--- 
+++ 
@@ -110,7 +110,7 @@
 use crate::child_content::MountedPrivateChildGroup;
 use crate::child_content::MountedCommandPrune;
-use crate::child_content::PagedCommandLog;
-use crate::document_ingress::OriginalCatalogConstructor;
-use crate::child_content::private_publication_group::PRIVATE_CHILD_GROUP_MAXIMUM_CHILDREN;
+use crate::child_content::mounted_child_group_receipt_group::command::PagedCommandLog;
+use crate::document_ingress::original_catalog_constructor::OriginalCatalogConstructor;
+use crate::child_content::private_publication_group::owner::PRIVATE_CHILD_GROUP_MAXIMUM_CHILDREN;
 use crate::window_kits::AppActionRegistry;
 use crate::app_contract::ArtifactApp;


### 54 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🚦️actions/🦀️.rs
B/H/S/current: 454/487/451/484
--- 
+++ 
@@ -211,5 +211,5 @@
     /// 🧭️ Why this cannot be `AppCommand::Command`. The shell's dispatch lane APPLIES: it runs
     /// `handle_action_invocation` → `dispatch_typed_command_inner`, which mounts a typed
-    /// operation whose publication ladder commits into `self.store` and mints an undo group. An
+    /// operation whose publication ladder commits into `self.stores.store` and mints an undo group. An
     /// agent's `action_prepare` must not do that — its own protocol commits separately
     /// (`TransactionPrepare{ops}` → `TransactionCommit`), and `RoutingArtifactChannel` caches ONE
@@ -357,5 +357,5 @@
             cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;
             let config=semio_framework_job_worker_authority::BatchDriveConfig{retained:grant,site:"plugin_agent_lane_preview",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:0,step_budget_us:0};
-            self.tools.agent_lane_preview=Some(crate::tool_jobs::AgentLanePreview{key:Some(key),caller:std::mem::ManuallyDrop::new(Some(caller)),starting:true,producer:None,children:None,completion:None,completion_close:None,lease:None,lease_close:None,config,maximum_bytes:0,steps:0,publication:None,complete:false,closing:false,publication_close:None});
+            self.tools.agent_lane_preview=Some(crate::tool_jobs::AgentLanePreview{key:Some(key),caller:std::mem::ManuallyDrop::new(Some(caller)),starting:true,producer_request:None,producer_request_retirement:None,producer:None,children:None,completion:None,completion_close:None,lease:None,lease_close:None,config,maximum_bytes:0,steps:0,publication:None,complete:false,closing:false,publication_close:None});
         }
         let state=self.tools.agent_lane_preview.as_mut().unwrap();
@@ -402,8 +402,10 @@
         let key = admission.proof.key();
         let contract = admission.proof.contract();
-        let spec = A::build_tool_job(ArtifactOwnedToolJobRequest {
+        let request = ArtifactOwnedToolJobRequest {
+            source_binding: semio_framework::action_bus::OriginalToolSourceBinding::borrow(cx),
                 retained:self.mounted_policy.maintenance,
-            command,
-            raw_wire: ArtifactToolRawInput::transferred_to_factory(),
+            command: crate::tool_contract::ArtifactOriginalToolCommand::from_admitted_op_binary(command,&admission.raw_wire,semio_framework::action_bus::OriginalToolSourceBinding::borrow(cx)),
+            input: Some(admission.raw_wire),
+            checkpoint: None,
             operation,
             controller_id: key.controller_id,
@@ -427,12 +429,7 @@
             output_chunks: ArtifactOutputChunks::new(contract.max_output_bytes),
             completion: completion.clone(),
-        })
-        .await?
-        .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.missing-owned-builder"), format!("app-owned tool '{verb}' registered a factory but supplied no exact payload builder")))?;
-        let wire_admission = admission.wire_admission.clone();
-        let dispatch =
-            self.tools.tool_jobs.dispatch_wire_retained_with_spec(&wire_admission, admission.raw_wire, None, spec).map_err(|rejected| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), rejected.error.to_string()))?;
+        };
         let config=semio_framework_job_worker_authority::BatchDriveConfig{retained:cx.retained_grant(),site:"plugin_agent_lane_preview",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:contract.max_work_units_per_step,step_budget_us:u64::from(contract.max_step_micros)};
-        let state=self.tools.agent_lane_preview.as_mut().expect("original preview lease remains received");state.producer=Some(crate::tool_jobs::OriginalPreviewDrive::new(dispatch.job,config,cx));state.completion=Some(completion);state.config=config;state.maximum_bytes=contract.max_output_bytes;state.starting=false;
+        let state=self.tools.agent_lane_preview.as_mut().expect("original preview lease remains received");state.producer_request=Some(request);state.completion=Some(completion);state.config=config;state.maximum_bytes=contract.max_output_bytes;state.starting=false;
         Ok(())
     }


### 55 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/♻️retirement/🦀️.rs
B/H/S/current: 93/109/93/109
--- 
+++ 
@@ -62,5 +62,5 @@
             }
             if self.tools.tool_operations.get(operation_id).is_some_and(|operation|operation.granted_retirement_ready()&&operation.granted_retirement_pending()){
-                let grant=crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.close,cx.retained_grant());
+                let grant=semio_framework_plugin_ownership::bounded_ownership::original_plugin_turn_grant(self.mounted_policy.close,cx.retained_grant());
                 let demand=self.tools.tool_operations.get(operation_id).unwrap().granted_retirement_demands(grant.maximum_copy_bytes)?;let step=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,actual|{let step=self.tools.tool_operations.get_mut(operation_id).unwrap().granted_retirement_step(same,actual)?;Ok((PluginLifecycleStep::Progress(step.progress()),false))})?;let step=RetainedCloneStep::Progress(step.progress().unwrap_or_default());
                 if !step.progress().fits(grant){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original typed operation retirement exceeded its issuer grant").with_retained_progress(step.progress()).into());}
@@ -69,5 +69,5 @@
 
             }
-            let grant = crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.close, cx.retained_grant());
+            let grant = semio_framework_plugin_ownership::bounded_ownership::original_plugin_turn_grant(self.mounted_policy.close, cx.retained_grant());
             let demand = self.tools.typed_operation_retirement_demands(&*self.child_composition, operation_id, grant.maximum_copy_bytes)?;
             if !plugin_grant_funds(demand, grant) {


### 56 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🦀️.rs
B/H/S/current: 977/56/977/56
--- 
+++ 
@@ -95,5 +95,5 @@
         if self.tools.tool_operations.get(operation_id).is_some_and(|mounted|mounted.cancellation_lease.is_none()&&mounted.cancellation_retirement.is_none()&&mounted.stage==MountedTypedCommandFullOperationStage::Publishing){
             let mounted=self.tools.tool_operations.get_mut(operation_id).unwrap();
-            if mounted.source_identity!=(cx.operation(),cx.generation()){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original emission lease requires its mounted receiving caller").into())}
+            if !mounted.source_identity.matches(cx){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original emission lease requires its mounted receiving caller").into())}
             let key=ToolOperationKey{app_instance_id:mounted.meta.instance_id,document:ArtifactDocumentAuthority(mounted.meta.instance_id),operation_id:mounted.operation.operation,base_revision:mounted.operation.base_revision,generation:mounted.operation.generation};
             mounted.cancellation_lease=self.tools.tool_cancellations.begin_keyed(&key,cx)?;
@@ -191,5 +191,5 @@
         if spent != RetainedCloneProgress::default() {
             assert!(mounted.publication_ownership_progress.is_none(), "typed preparation cannot overwrite an unreturned original receipt");
-            mounted.publication_ownership_progress = Some((crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.preparation, retained), spent));
+            mounted.publication_ownership_progress = Some((semio_framework_plugin_ownership::bounded_ownership::original_plugin_turn_grant(self.mounted_policy.preparation, retained), spent));
         }
         outcome
@@ -199,5 +199,5 @@
     async fn publish_mounted_typed_operation_pump(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, retained:RetainedCloneGrant, spent: &mut RetainedCloneProgress,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(),crate::PluginLifecycleFailure> {
         let started_us=cx.now_us();
-        let turn = crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.preparation, retained);
+        let turn = semio_framework_plugin_ownership::bounded_ownership::original_plugin_turn_grant(self.mounted_policy.preparation, retained);
         for _ in 0..TYPED_OPERATION_PUBLICATION_PUMPS {
             if cx.should_yield(){return Ok(())}
@@ -217,5 +217,5 @@
             } else {
                 let before = (mounted.publication_progress, mounted.publication.is_some(), mounted.pending_artifact_publication.is_some(), mounted.stage);
-                self.publish_mounted_typed_operation_unit(mounted,crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.preparation,semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_grant(cx,crate::plugin_runtime::remaining_plugin_turn_grant(turn,*spent))))?;
+                self.publish_mounted_typed_operation_unit(mounted,semio_framework_plugin_ownership::bounded_ownership::original_plugin_turn_grant(self.mounted_policy.preparation,semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_grant(cx,crate::plugin_runtime::remaining_plugin_turn_grant(turn,*spent))))?;
                 let receipt = mounted.publication_ownership_progress.take();
                 if let Some((_, unit)) = receipt {
@@ -272,5 +272,5 @@
         }
         let token = mounted.next_token();
-        let page = TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::Interaction, &("accepted", writes.len()))?;
+        let page = TypedOperationResultPage::try_serialize_cold(token, TypedOperationResultLane::Interaction, &("accepted", writes.len()))?;
         mounted.queue_page(page)
     }
@@ -377,5 +377,5 @@
                 publication_lanes: &[],
                 session: None,
-                session_rejected: None,
+                session_rejected: None,producer_source:None,producer_request:None,producer_request_retirement:None,
                 reserved_producer: None,
                 completion: None,
@@ -451,5 +451,5 @@
                 self.as_dispatch().record_typed_operation_lane(mounted);
                 mounted.pending_child_publication = Some(pending);
-                return mounted.queue_page(TypedOperationResultPage::try_serialize(mounted.next_token(), TypedOperationResultLane::Child, &receipt)?);
+                return mounted.queue_page(TypedOperationResultPage::try_serialize_cold(mounted.next_token(), TypedOperationResultLane::Child, &receipt)?);
             }
             PendingChildGroupPublicationPhase::Faulted => {
@@ -555,5 +555,5 @@
                 }
                 mounted.pending_child_publication = Some(pending);
-                mounted.queue_page(TypedOperationResultPage::try_serialize(mounted.next_token(), TypedOperationResultLane::Child, &receipt)?)
+                mounted.queue_page(TypedOperationResultPage::try_serialize_cold(mounted.next_token(), TypedOperationResultLane::Child, &receipt)?)
             }
             Err(fault) => {
@@ -650,5 +650,5 @@
                     }
                     let token = mounted.next_token();
-                    mounted.queue_page(TypedOperationResultPage::try_serialize(token, pending_lane, &receipt)?)
+                    mounted.queue_page(TypedOperationResultPage::try_serialize_cold(token, pending_lane, &receipt)?)
                 }
                 store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Progress(checkpoint) => {
@@ -736,5 +736,5 @@
                     }
                 } else {
-                    TypedOperationResultPage::try_serialize(
+                    TypedOperationResultPage::try_serialize_cold(
                         token,
                         TypedOperationResultLane::Download,
@@ -786,5 +786,5 @@
                                 return Err(MediaTransfers::<A>::transaction_fault(FaultOrigin::App, "transaction.member-rejected", format!("mutation outcome was rejected: {:?}", outcome.messages())));
                             }
-                            running = protocol::apply_diff(outcome.diff(), &running).map_err(|error| MediaTransfers::<A>::transaction_fault(FaultOrigin::App, "transaction.member-rejected", error.to_string()))?;
+                            running = protocol::apply_diff_cold(outcome.diff(), &running).map_err(|error| MediaTransfers::<A>::transaction_fault(FaultOrigin::App, "transaction.member-rejected", error.to_string()))?;
                         }
                         if !foreign.is_empty() {
@@ -929,5 +929,5 @@
                             return Err(plugin_sdk_fault("typed-operation effect receiver is saturated"));
                         }
-                        TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::Effect, &("accepted", self.tools.typed_effect_outbox.len()))?
+                        TypedOperationResultPage::try_serialize_cold(token, TypedOperationResultLane::Effect, &("accepted", self.tools.typed_effect_outbox.len()))?
                     } else if let Some(event) = emit.events.pop() {
                         if let Err(event) = self.tools.typed_event_outbox.push(event) {
@@ -935,5 +935,5 @@
                             return Err(plugin_sdk_fault("typed-operation event receiver is saturated"));
                         }
-                        TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::Event, &("accepted", self.tools.typed_event_outbox.len()))?
+                        TypedOperationResultPage::try_serialize_cold(token, TypedOperationResultLane::Event, &("accepted", self.tools.typed_event_outbox.len()))?
                     } else {
                         match () {
@@ -948,5 +948,5 @@
                                 }
                                 mounted.ui_pending = false;
-                                TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::Ui, &emit.ui_scope)?
+                                TypedOperationResultPage::try_serialize_cold(token, TypedOperationResultLane::Ui, &emit.ui_scope)?
                             }
                             () => {


### 57 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/🦀️.rs
B/H/S/current: 656/661/707/711
--- 
+++ 
@@ -2,4 +2,6 @@
 use semio_framework_plugin_operation::{ToolCancellationLeaseRetirement, original_lease_cursor_admit, original_lease_cursor_release};
 use semio_framework_value::retirement::controlled::ControlledRetirement;
+pub(crate) use semio_framework_actor::typed_operation_diagnostics as diagnostic_publication;
+
 use semio_framework_value::DslValue;
 use semio_framework::{Fault, FaultCode, FaultOrigin};
@@ -14,5 +16,5 @@
 #[cfg(test)]
 use semio_framework_plugin_decl_emission::ChildEmit;
-use semio_framework_plugin_ownership::bounded_ownership::plugin_grant_funds;
+use semio_framework_plugin_ownership::bounded_ownership::{plugin_grant_funds, plugin_retirement_fault, plugin_turn_admitted};
 use semio_framework_plugin_decl_app::ArtifactApp;
 use semio_framework_plugin_decl_emission::ActionMeta;
@@ -51,5 +53,5 @@
     pub meta: ActionMeta,
     pub operation: semio_framework_job::Operation,
-    pub source_identity:(semio_framework_job::OperationId,semio_framework_job::Generation),
+    pub source_identity:semio_framework_job::OriginalSourceBinding,
     pub canonical_revision: [u8; 32],
     pub artifact_generation: u64,
@@ -63,4 +65,7 @@
     pub session: Option<semio_framework_job_worker::MountedWorkerJobSession<semio_framework::ErasedToolJob>>,
     pub session_rejected: Option<semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource<semio_framework::ErasedToolJob>>,
+    pub(crate) producer_source: Option<semio_framework::action_bus::OriginalToolSourceBinding>,
+    pub(crate) producer_request: Option<semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobRequest<A>>,
+    pub(crate) producer_request_retirement: Option<ControlledRetirement<semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobRequest<A>>>,
     pub reserved_producer: Option<ArtifactReservedToolJob>,
     pub completion: Option<ArtifactToolCompletion<A>>,
@@ -90,4 +95,5 @@
     pub captured_child_content_generation: u64,
     pub result_page: Option<TypedOperationResultPage>,
+    pub diagnostics: Option<diagnostic_publication::OperationDiagnosticPublication>,
     pub result_page_presented: bool,
     pub result_sequence: u32,
@@ -125,5 +131,5 @@
 
 impl<A:ArtifactApp> MountedTypedCommandFullOperation<A>{
- pub fn lease_retirement_ready(&self)->bool{self.stage==MountedTypedCommandFullOperationStage::Retiring&&self.session.is_none()&&self.session_rejected.is_none()&&!self.worker_semantic_pending&&!self.worker_outcome_pending&&self.worker_fault_capture.is_none()&&self.publication.is_none()&&self.publication_retirement.is_none()&&self.pending_artifact_publication.is_none()&&!self.pending_publication_outcome.pending()&&self.pending_child_publication.is_none()&&self.owned_child_group.is_none()&&self.captured_child_content.is_none()&&self.result_page.is_none()&&self.pending_window_config_receipt.is_none()&&self.raw_input.is_none()&&self.output_chunks.is_none()&&self.output_retirement.is_none()&&self.reserved_producer.is_none()&&self.completion.is_none()&&self.completion_retirement.is_none()&&self.meta.view_state.is_none()&&self.window_config_authority.is_none()&&self.window_transient_authority.is_none()&&self.actor_capture.is_none()&&self.verb.capacity()==0&&!self.meta.actor.has_owner()&&self.retained_close_fault.is_none()&&(self.cancellation_lease.is_some()||self.cancellation_retirement.is_some())}
+ pub fn lease_retirement_ready(&self)->bool{self.stage==MountedTypedCommandFullOperationStage::Retiring&&self.session.is_none()&&self.producer_request.is_none()&&self.producer_request_retirement.is_none()&&self.session_rejected.is_none()&&!self.worker_semantic_pending&&!self.worker_outcome_pending&&self.worker_fault_capture.is_none()&&self.publication.is_none()&&self.publication_retirement.is_none()&&self.pending_artifact_publication.is_none()&&!self.pending_publication_outcome.pending()&&self.pending_child_publication.is_none()&&self.owned_child_group.is_none()&&self.captured_child_content.is_none()&&self.result_page.is_none()&&self.pending_window_config_receipt.is_none()&&self.raw_input.is_none()&&self.output_chunks.is_none()&&self.output_retirement.is_none()&&self.reserved_producer.is_none()&&self.completion.is_none()&&self.completion_retirement.is_none()&&self.meta.view_state.is_none()&&self.window_config_authority.is_none()&&self.window_transient_authority.is_none()&&self.actor_capture.is_none()&&self.verb.capacity()==0&&!self.meta.actor.has_owner()&&self.retained_close_fault.is_none()&&(self.cancellation_lease.is_some()||self.cancellation_retirement.is_some())}
  pub fn lease_retirement_demands(&self,canonical:&ToolCancellationHandle,body:usize)->Result<RetirementDemand,ValueError>{if let Some(owner)=self.cancellation_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of::<ToolCancellationLeaseRetirement>(),depth:1,..Default::default()})}else{owner.retirement_demands(canonical,body)};}Ok(RetirementDemand{copy_bytes:size_of::<ToolCancellationLease>(),capacity_bytes:size_of::<ToolCancellationLeaseRetirement>(),depth:1,..Default::default()})}
  pub fn lease_retirement_step(&mut self,canonical:&ToolCancellationHandle,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
@@ -185,12 +191,16 @@
     /// cancel the lease so the next publication unit mints the exact fault page.
     pub fn drive_worker_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<PluginLifecycleStep, semio_framework_job::OriginalTurnFailure> {
-        if self.source_identity!=(cx.operation(),cx.generation()){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"mounted worker requires its original source identity independently of its registry record").into())}
-        let grant=cx.retained_grant();let demand=if cx.is_cancelled()&&self.session_rejected.as_ref().is_some_and(|source|source.admission_pending()){self.worker_retirement_demands(grant.maximum_copy_bytes)?}else{self.worker_step_demands(grant.maximum_copy_bytes)?};
+        self.validate_original_producer_source(cx)?;
+        if self.stage != MountedTypedCommandFullOperationStage::Worker {return Ok(PluginLifecycleStep::Blocked{reason:"typed command awaits its bounded revision-validated publication or ACK turn"});}
+        if !self.source_identity.matches(cx){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"mounted worker requires its original source identity independently of its registry record").into())}
+        if (self.producer_request.is_some()||self.original_reserved_producer_pending())&&!self.user_cancel_requested&&!self.terminal_seen&&!cx.is_cancelled(){return self.drive_original_producer(cx);}
+        let grant=cx.retained_grant();let demand=if cx.is_cancelled()&&(self.original_producer_pending()||self.session_rejected.as_ref().is_some_and(|source|source.admission_pending())){self.worker_retirement_demands(grant.maximum_copy_bytes)?}else{self.worker_step_demands(grant.maximum_copy_bytes)?};
         semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant|{
             let step=(||->Result<PluginLifecycleStep,semio_framework_job::OriginalTurnFailure>{
         if self.stage != MountedTypedCommandFullOperationStage::Worker {return Ok(PluginLifecycleStep::Blocked{reason:"typed command awaits its bounded revision-validated publication or ACK turn"});}
         if grant.maximum_items==0||grant.maximum_depth==0{return Ok(PluginLifecycleStep::Progress(Default::default()));}
+        if self.original_producer_pending(){return self.retire_original_producer(grant);}
         if let Some(rejected)=self.session_rejected.as_mut(){
-            if rejected.admission_pending()&&!cx.is_cancelled(){
+            if rejected.admission_pending()&&!cx.is_cancelled()&&!self.user_cancel_requested&&!self.terminal_seen{
                 let config=semio_framework_job_worker_authority::BatchDriveConfig{retained:grant,site:"plugin_framework_reserved_route",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:cx.fuel_remaining(),step_budget_us:0};
                 let before=cx.retained_progress();
@@ -349,10 +359,33 @@
     }
 
-    pub fn take_result_page(&mut self) -> Option<TypedOperationResultPage> {
-        if self.result_page_presented {
-            return None;
-        }
-        self.result_page_presented = true;
-        self.result_page.clone()
+    pub(crate) fn result_page_presentation_demands(&self)->Result<RetirementDemand,ValueError>{self.result_page.as_ref().map_or(Ok(Default::default()),|page|{let mut d=page.presentation_demand()?;d.copy_bytes+=size_of::<bool>();Ok(d)})}
+    pub fn take_result_page(&mut self,grant:RetainedCloneGrant) -> Result<(Option<TypedOperationResultPage>,RetainedCloneProgress),Fault> {
+        if self.result_page_presented { return Ok((None,Default::default())); }
+        let demand=self.result_page_presentation_demands().map_err(plugin_retirement_fault)?;
+        if !plugin_turn_admitted(demand,grant)?{return Ok((None,Default::default()));}
+        let Some(page)=self.result_page.as_mut()else{return Ok((None,Default::default()))};
+        let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<bool>(),..grant};
+        let(result,mut progress)=page.present(child).map_err(plugin_retirement_fault)?;
+        if result.is_some(){self.result_page_presented=true;progress.copied_bytes+=size_of::<bool>();}
+        Ok((result,progress))
+    }
+
+    /// 🧪️ Supplies each production presentation turn with the fixture issuer's unchanged grant.
+    #[cfg(test)]
+    pub(crate) fn take_fixture_result_page(&mut self,grant:RetainedCloneGrant)->Option<TypedOperationResultPage>{
+        for _ in 0..2{let((page,progress),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||self.take_result_page(grant).expect("original fixture page presentation"));assert!(progress.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));if page.is_some()||progress==RetainedCloneProgress::default(){return page;}}
+        panic!("original fixture page sealing omitted its next projection");
+    }
+    /// 🧪️ Pays the original page retirement through the production close port after exact ACK.
+    #[cfg(test)]
+    pub(crate) fn close_fixture_original_page(&mut self,grant:RetainedCloneGrant){for _ in 0..100000{if !self.result_page_retirement_pending(){return;}let(p,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||self.result_page_retirement_step(grant).expect("original fixture producer page close"));assert!(p.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));assert_ne!(p,Default::default(),"original fixture close grant funds the exact producer page");}panic!("original fixture producer page did not close");}
+    /// ♻️ Exact ACK retains the original page until its funded physical closure.
+    pub(crate) fn result_page_retirement_pending(&self)->bool{self.result_page.is_some()&&self.stage!=MountedTypedCommandFullOperationStage::AwaitingAck}
+    pub(crate) fn result_page_retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.result_page.as_ref().map_or(Ok(Default::default()),|page|{let mut d=page.next_close_demand(body.saturating_sub(size_of::<Option<TypedOperationResultPage>>()+size_of::<bool>()))?;d.copy_bytes+=size_of::<Option<TypedOperationResultPage>>()+size_of::<bool>();Ok(d)})}
+    pub(crate) fn result_page_retirement_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,Fault>{
+        if !self.result_page_retirement_pending(){return Ok(Default::default());}
+        let demand=self.result_page_retirement_demands(grant.maximum_copy_bytes).map_err(plugin_retirement_fault)?;if !plugin_turn_admitted(demand,grant)?{return Ok(Default::default());}let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<Option<TypedOperationResultPage>>()-size_of::<bool>(),..grant};let page=self.result_page.as_mut().unwrap();let mut progress=page.close_step(child).map_err(plugin_retirement_fault)?.progress();
+        if page.terminal_is_empty(){self.result_page=None;self.result_page_presented=false;progress.copied_bytes+=size_of::<Option<TypedOperationResultPage>>()+size_of::<bool>();}
+        Ok(progress)
     }
 
@@ -379,7 +412,8 @@
             return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-publication-ack"), "child-group result ACK lost its exact committed publication authority"));
         }
-        self.result_page = None;
+        if lane == TypedOperationResultLane::Diagnostic && !self.diagnostics.as_mut().is_some_and(|owner|owner.acknowledge(token)) {
+            return Err(plugin_sdk_fault("diagnostic result ACK lost its exact original source authority"));
+        }
         if lane == TypedOperationResultLane::Child && self.owned_child_committed { self.owned_child_result_pending = false; }
-        self.result_page_presented = false;
         self.result_sequence = self.result_sequence.checked_add(1).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.result-sequence"), "typed-operation result sequence exhausted"))?;
         self.stage = if matches!(lane, TypedOperationResultLane::Terminal | TypedOperationResultLane::Fault | TypedOperationResultLane::Download) {
@@ -388,4 +422,5 @@
             MountedTypedCommandFullOperationStage::Publishing
         };
+        if self.stage==MountedTypedCommandFullOperationStage::Retiring{if let Some(owner)=self.diagnostics.as_mut(){owner.begin_close();}}
         Ok(true)
     }
@@ -395,4 +430,5 @@
         let item = RetirementDemand { depth: 1, ..Default::default() };
         if self.stage != MountedTypedCommandFullOperationStage::Worker { return Ok(Default::default()); }
+        if self.original_producer_pending(){return if self.user_cancel_requested||self.terminal_seen{self.original_producer_retirement_demands(body)}else{Ok(item)};}
         if let Some(source)=self.session_rejected.as_ref(){return if source.admission_pending(){source.admission_demands()}else{self.worker_retirement_demands(body)};}
         let Some(session) = self.session.as_ref() else { return Ok(item) };
@@ -408,5 +444,5 @@
 
     fn worker_retirement_pending(&self) -> bool {
-        self.session.is_some() || self.session_rejected.is_some() || self.worker_semantic_pending || self.worker_outcome_pending || self.worker_fault_capture.is_some()
+        self.original_producer_pending() || self.session.is_some() || self.session_rejected.is_some() || self.worker_semantic_pending || self.worker_outcome_pending || self.worker_fault_capture.is_some()
     }
 
@@ -414,4 +450,5 @@
     fn worker_retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
         let item = RetirementDemand { depth: 1, ..Default::default() };
+        if self.original_producer_pending(){return self.original_producer_retirement_demands(body);}
         if let Some(rejected) = self.session_rejected.as_ref() {
             if rejected.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: size_of_val(rejected), ..item }); }
@@ -431,8 +468,10 @@
     /// ♻️ Closes the original mounted worker session, its refused admission and its inline capture markers under the unchanged grant.
     fn retire_worker_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<PluginLifecycleStep, semio_framework_job::OriginalTurnFailure> {
+        self.validate_original_producer_source(cx)?;
         let grant=cx.retained_grant();let demand=self.worker_retirement_demands(grant.maximum_copy_bytes)?;
         semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant|{
             let step=(||->Result<PluginLifecycleStep,semio_framework_job::OriginalTurnFailure>{
         let item = RetainedCloneProgress { copied_items: 1, ..Default::default() };
+        if self.original_producer_pending(){return self.retire_original_producer(grant);}
         if let Some(rejected) = self.session_rejected.as_mut() {
             if rejected.terminal_is_empty() {
@@ -487,6 +526,8 @@
     pub fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
         let item = RetirementDemand { depth: 1, ..Default::default() };
+        if self.result_page_retirement_pending(){return self.result_page_retirement_demands(body);}
         if self.close_fault_pending() { return self.close_fault_demands(body); }
         if self.worker_retirement_pending() { return self.worker_retirement_demands(body); }
+        if let Some(owner)=self.diagnostics.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<diagnostic_publication::OperationDiagnosticPublication>>(),..item})}else{owner.next_demand(body)}}
         if self.pending_window_config_receipt.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<store::os_store::presence::LaneItemReceipt>(), ..item }); }
         if self.granted_retirement_ready() && self.granted_retirement_pending() { return self.granted_retirement_demands(body); }
@@ -505,4 +546,5 @@
     /// 🎟️ Returns each original mounted owner under the caller's unchanged grant.
     pub fn retirement_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<PluginLifecycleStep, semio_framework_job::OriginalTurnFailure> {
+        self.validate_original_producer_source(cx)?;
         let grant=cx.retained_grant();let demand=self.retirement_demands(grant.maximum_copy_bytes)?;
         semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant|{
@@ -511,4 +553,5 @@
         let item = RetainedCloneProgress { copied_items: 1, ..empty };
         if self.stage != MountedTypedCommandFullOperationStage::Retiring { return Ok(PluginLifecycleStep::Blocked { reason: "typed operation is not admitted for retirement" }); }
+        if self.result_page_retirement_pending(){return self.result_page_retirement_step(grant).map(PluginLifecycleStep::Progress);}
         if self.result_page.is_some() { return Ok(PluginLifecycleStep::Blocked { reason: "typed operation retains an unacknowledged result page" }); }
         if self.terminal_is_empty() { return Ok(PluginLifecycleStep::Complete(empty)); }
@@ -564,4 +607,5 @@
         if self.stage == MountedTypedCommandFullOperationStage::Retiring { return self.retirement_step(cx); }
         self.user_cancel_requested = true;
+        if let Some(owner)=self.diagnostics.as_mut(){owner.begin_close();}
         self.result_page = None;
         self.result_page_presented = false;
@@ -578,4 +622,6 @@
         self.stage == MountedTypedCommandFullOperationStage::Retiring
             && self.session.is_none()
+            && self.producer_request.is_none()
+            && self.producer_request_retirement.is_none()
             && self.session_rejected.is_none()
             && !self.worker_semantic_pending
@@ -589,4 +635,5 @@
             && self.captured_child_content.is_none()
             && self.result_page.is_none()
+            && self.diagnostics.is_none()
             && self.raw_input.is_none()
             && self.output_chunks.is_none()
@@ -654,3 +701,7 @@
 
 
+
+
 pub use semio_framework_plugin_job::lifted_full_operation_job::*;
+
+include!("🏭️producer/🦀️.rs");


### 58 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs
B/H/S/current: 1023/65/1075/70
--- 
+++ 
@@ -29,5 +29,10 @@
 pub mod admission;
 pub use crate::admission::ArtifactRetainedAdmissionRefusal;
+#[path="🏭️factory/🦀️.rs"]
+pub mod factory;
+#[path="🏭️producer/🦀️.rs"]
+pub mod producer;
 #[doc(hidden)]
+
 #[path="📸️checkpoint/👣️cursor/🦀️.rs"]
 pub mod checkpoint_cursor;
@@ -114,4 +119,5 @@
 
 pub enum ArtifactCommandWorkStep<A: ArtifactApp> {
+    Pending { stage: &'static str, preview: &'static [u8] },
     Replay { stage: &'static str, preview: &'static [u8] },
     Progress { stage: &'static str, preview: &'static [u8] },
@@ -309,4 +315,10 @@
     pub admission_refusal:Option<ArtifactRetainedAdmissionRefusal>,
     pub work: Box<dyn ArtifactCommandWork<A>>,
+    pub(crate) producer_window_config:Option<crate::window_config::WindowConfigSnapshot>,
+    pub(crate) producer_instance_operation_owner:Option<crate::tool_contract::ArtifactInstanceOperationOwnerHandle>,
+    pub(crate) producer_output_chunks:Option<crate::tool_contract::ArtifactOutputChunks>,
+    pub(crate) producer_controller_id:Option<String>,
+    pub(crate) producer_tool_id:Option<String>,
+    pub(crate) producer_payload_schema_id:Option<String>,
 }
 
@@ -315,5 +327,5 @@
         let admission_refusal=admission::admit_limits(maximum_raw_bytes,maximum_work_items);
         let ArtifactRetainedCommandInputs { command, snapshot, snapshot_read, config, history, interaction_state, interaction_hover, context, operation, completion } = inputs;
-        Self { command, snapshot, snapshot_read, config, history, interaction_state, interaction_hover, context, operation, completion, command_id, maximum_raw_bytes, maximum_work_items, admission_refusal,work }
+        Self { command, snapshot, snapshot_read, config, history, interaction_state, interaction_hover, context, operation, completion, command_id, maximum_raw_bytes, maximum_work_items, admission_refusal,work,producer_window_config:None,producer_instance_operation_owner:None,producer_output_chunks:None,producer_controller_id:None,producer_tool_id:None,producer_payload_schema_id:None }
 
     }
@@ -342,5 +354,7 @@
 
 pub struct ArtifactRetainedCommandJob<A: ArtifactApp> {
+    source_binding:Option<semio_framework::action_bus::OriginalToolSourceBinding>,
     pub command: Option<A::Command>,
+
     command_received:bool,
     pub snapshot: Option<Arc<A::Snapshot>>,
@@ -386,4 +400,10 @@
     work_progress: u64,
     pub(crate) closing: bool,
+    producer_window_config:Option<crate::window_config::WindowConfigSnapshot>,
+    producer_instance_operation_owner:Option<crate::tool_contract::ArtifactInstanceOperationOwnerHandle>,
+    producer_output_chunks:Option<crate::tool_contract::ArtifactOutputChunks>,
+    producer_controller_id:Option<String>,
+    producer_tool_id:Option<String>,
+    producer_payload_schema_id:Option<String>,
 }
 
@@ -415,5 +435,5 @@
             else if self.interaction_hover.is_some(){Some(store::os_store::snapshot_read::artifact_retirement_admit_shared_lease(&mut self.interaction_hover,&mut self.input_retirement,grant))}
             else if self.operation.is_some(){Some(store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.operation,&mut self.input_retirement,grant))}
-            else if self.context.is_some(){Some(store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.context,&mut self.input_retirement,grant))}else{None};
+            else if self.context.is_some(){Some(store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.context,&mut self.input_retirement,grant))}else if self.producer_controller_id.is_some(){Some(store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.producer_controller_id,&mut self.input_retirement,grant))}else if self.producer_tool_id.is_some(){Some(store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.producer_tool_id,&mut self.input_retirement,grant))}else if self.producer_payload_schema_id.is_some(){Some(store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.producer_payload_schema_id,&mut self.input_retirement,grant))}else if self.producer_window_config.is_some(){Some(store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.producer_window_config,&mut self.input_retirement,grant))}else if self.producer_instance_operation_owner.is_some(){Some(store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.producer_instance_operation_owner,&mut self.input_retirement,grant))}else if self.producer_output_chunks.is_some(){Some(store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.producer_output_chunks,&mut self.input_retirement,grant))}else{None};
         if let Some(step)=admitted{return Self::controlled_close_step(step)}
         if let Some(completion)=self.completion.take(){match ControlledRetirement::new(completion){Ok(owner)=>self.completion_retirement=Some(owner),Err((error,original))=>{self.completion=Some(original);return Err(error.into())}}return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}})}
@@ -453,4 +473,10 @@
         if self.operation.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.operation);}
         if self.context.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.context);}
+        if self.producer_window_config.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.producer_window_config);}
+        if self.producer_instance_operation_owner.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.producer_instance_operation_owner);}
+        if self.producer_output_chunks.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.producer_output_chunks);}
+        if self.producer_controller_id.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.producer_controller_id);}
+        if self.producer_tool_id.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.producer_tool_id);}
+        if self.producer_payload_schema_id.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.producer_payload_schema_id);}
         if self.completion.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()});}
         if let Some(owner)=self.completion_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{Ok(owner.selected_demand(copy)?)};}
@@ -471,5 +497,10 @@
     }
 
+    #[cfg(any(test, feature = "artifact-app-testing"))]
+    pub fn test_admit_original_source(&mut self,cx:&mut StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
+        if self.source_binding.is_some(){return Ok(self.source_binding.unwrap().matches(cx))}let mut output=None;let admitted=semio_framework::action_bus::admit_original_tool_construction(cx,&mut output,semio_framework::action_bus::ToolJobAdmissionDemand::reused_job(),|identity|Ok((identity.source_binding(),RetainedCloneProgress{copied_items:1,..Default::default()})))?;if admitted{self.source_binding=output;}Ok(admitted)
+    }
     pub fn from_payload(payload: ArtifactRetainedCommandPayload<A>, raw_input: Option<RetainedToolWireInput>, checkpoint_input: Option<RetainedToolWireInput>) -> Self {
+
         let (raw,admission_refusal)=match (payload.admission_refusal,raw_input.as_ref()) {
             (None,Some(input))=>admission::admit_raw(input.declared_bytes(),payload.maximum_raw_bytes),
@@ -484,4 +515,5 @@
         };
         Self {
+            source_binding:None,
             command: Some(payload.command),
             command_received:false,
@@ -499,4 +531,10 @@
             maximum_work_items: payload.maximum_work_items,
             work: Some(payload.work),
+            producer_controller_id:payload.producer_controller_id,
+            producer_tool_id:payload.producer_tool_id,
+            producer_payload_schema_id:payload.producer_payload_schema_id,
+            producer_window_config:payload.producer_window_config,
+            producer_instance_operation_owner:payload.producer_instance_operation_owner,
+            producer_output_chunks:payload.producer_output_chunks,
             checkpoint_input,
             checkpoint_bytes: [0; ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES],
@@ -785,5 +823,5 @@
                 if !self.command_received{
                     let (Some(command),Some(operation),Some(work))=(self.command.as_mut(),self.operation.as_ref(),self.work.as_mut())else{return Err(Self::unadmitted_input().into())};
-                    if cx.operation().0!=operation.operation_id||cx.generation().0!=operation.generation{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original command receiving requires its admitted business identity").into())}
+                    if !self.source_binding.is_some_and(|source|source.matches(cx)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original command receiving requires its original physical source binding").into())}
                     let before=cx.retained_progress();let fuel=cx.fuel_remaining();
                     if !work.receive_original_command(command,cx)?{return Ok(None)}
@@ -795,8 +833,9 @@
                 let before = cx.retained_progress();
                 let fuel = cx.fuel_remaining();
-                let step = work.step(&ArtifactCommandInputs { command, snapshot, snapshot_owner: Some(snapshot), snapshot_read:ArtifactCommandSnapshotRead{original:&self.snapshot_read,operation}, config, history, interaction, hover, context: self.context.as_deref(), operation }, cx)?;
+                let step = work.step(&ArtifactCommandInputs { command, snapshot, snapshot_owner: Some(snapshot), snapshot_read:ArtifactCommandSnapshotRead{original:Some(&self.snapshot_read),operation,source:self.source_binding}, config, history, interaction, hover, context: self.context.as_deref(), operation }, cx)?;
                 if cx.retained_progress() == before { Self::metadata(cx)?; }
                 if cx.fuel_remaining() == fuel { cx.consume_fuel(1); }
                 match step {
+                    ArtifactCommandWorkStep::Pending { stage, preview } => { cx.set_stage(stage); self.checkpoint_due(); self.preview(preview); }
                     ArtifactCommandWorkStep::Replay { stage, preview } => { cx.set_stage(stage); self.checkpoint_due(); self.preview(preview); }
                     ArtifactCommandWorkStep::Progress { stage, preview } => { cx.set_stage(stage); self.work_progress = self.work_progress.saturating_add(1); self.checkpoint_due(); self.preview(preview); }
@@ -855,5 +894,5 @@
 
 impl<A: ArtifactApp> InteractiveJob for ArtifactRetainedCommandJob<A> {
-    fn step<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{self.advance_original(cx)}
+    fn step<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{if !self.source_binding.is_some_and(|source|source.matches(cx)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original retained command source binding changed before work").into())}self.advance_original(cx)}
 
     fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
@@ -886,4 +925,5 @@
     fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.depth)}
     fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
+        if !self.source_binding.is_some_and(|source|source.matches(cx)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original retained command source binding changed before retirement").into())}
         if !self.closing{return Ok(InteractiveJobCloseStep::Blocked)}
         let grant=cx.retained_grant();let demand=self.close_demands(grant.maximum_copy_bytes)?;
@@ -928,4 +968,10 @@
             && self.completion.is_none()
             && self.completion_retirement.is_none()
+            && self.producer_controller_id.is_none()
+            && self.producer_tool_id.is_none()
+            && self.producer_payload_schema_id.is_none()
+            && self.producer_window_config.is_none()
+            && self.producer_instance_operation_owner.is_none()
+            && self.producer_output_chunks.is_none()
     }
 }
@@ -955,4 +1001,10 @@
             maximum_work_items: 1,
             work: None,
+            producer_controller_id:None,
+            producer_tool_id:None,
+            producer_payload_schema_id:None,
+            producer_window_config:None,
+            producer_instance_operation_owner:None,
+            producer_output_chunks:None,
             checkpoint_input: None,
             checkpoint_bytes: [0; ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES],


### 59 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-scalar-config-cohort/🟦️.ts
B/H/S/current: 64/86/74/96
--- 
+++ 
@@ -34,14 +34,24 @@
       }
     }
+    for (const lane of ["WindowConfig", "WindowTransient"] as const) {
+      const routed = evidence.appOwned.filter((row) => row.ownerFile === owner.file && row.publicationLanes.includes(lane));
+      if (!routed.length) continue;
+      const name = lane === "WindowConfig" ? "register_window_config_owners" : "register_window_transient_owners";
+      const missing = new Map(files);
+      missing.set(owner.file, files.get(owner.file)!.replaceAll(name, "missing_window_registry_authority"));
+      const rejected = toolJobOwnerSourceEvidence(missing);
+      if (routed.some((row) => rejected.appOwned.find((candidate) => candidate.ownerFile === row.ownerFile && candidate.toolId === row.toolId)?.publicationReady)) throw new Error(`window registry authority omission ${owner.id}/${lane}`);
+      hostileCases++;
+    }
     const source = files.get(owner.file)!;
-    const witnesses = [
+    const witnesses = owner.preparationLane === "WindowConfig" ? ["fn register_window_config_owners(", "fn register_window_transient_owners(", ...owner.registrationWitnesses] : [
       `const ${owner.prefix}_CONFIG_TEXT_MAXIMUM_BYTES: usize = 128;`,
       `const ${owner.prefix}_CONFIG_PUBLICATION_MAXIMUM_BYTES: usize = 4_096;`,
       "request.operation != request.authority.operation()", "request.generation != request.authority.generation()", "request.base_revision != request.authority.base_revision()",
-      "request.authority.actor().len() > 64", "self.preflight(&request.mutation, request.description.as_deref(), request.lane).is_err()",
+      "request.authority.actor().len() > 64", "self.preflight(&request.mutation, request.lane).is_err()",
       `_config_text_bytes(request.base.get()) > ${owner.prefix}_CONFIG_TEXT_MAXIMUM_BYTES`,
-      `grant.maximum_bytes < ${owner.prefix}_CONFIG_PUBLICATION_MAXIMUM_BYTES || self.cancelled || self.closing`,
-      "if self.checkpoint.cursor != 0", "inverse: vec![inverse]", "protocol::UndoPolicy::ExactBaseOnly", "authority.prepare_one_item(edit, std::sync::Arc::new(next))?",
-      "if !base.return_to_registry()", "self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some()",
+      `!grant.permits_one() || grant.maximum_copy_bytes < ${owner.prefix}_CONFIG_PUBLICATION_MAXIMUM_BYTES || self.cancelled || self.owners.is_closing()`,
+      "if self.checkpoint.cursor != 0", "authority.next_edit(mutation.clone(), vec![inverse])", "match authority.prepare_one_item(edit, std::sync::Arc::new(next))",
+      "*self.owners.refused = Some((edit, post))", "self.owners.begin_close()", "self.owners.close_step(grant.retained_grant())", "self.owners.terminal_is_empty()",
     ];
     const exact = (candidate: string) => witnesses.every((witness) => candidate.includes(witness));


### 60 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🧫️fixtures/🎚️scalar-config-cohort.json
B/H/S/current: 905/1175/1216/1188
--- 
+++ 
@@ -11,5 +11,31 @@
     "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🦀️.rs",
     "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🦀️.rs",
-    "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🦀️.rs"
+    "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🦀️.rs",
+    "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🎚️config/🦀️.rs",
+    "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🎚️config/🦀️.rs",
+    "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🎚️config/🦀️.rs",
+    "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️blueprint/🎚️config/🦀️.rs",
+    "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs",
+    "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs",
+    "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🦀️.rs",
+    "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🦀️.rs",
+    "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🦀️.rs",
+    "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🚪️io/🦀️.rs",
+    "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/🦀️.rs",
+    "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/⏱️set-playback-clock/🦀️.rs",
+    "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🚪️io/💾️binary/🦀️.rs",
+    "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🚪️io/📝️text/🦀️.rs",
+    "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs",
+    "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🦀️.rs",
+    "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs",
+    "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/📝️form/🦀️.rs",
+    "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️.rs",
+    "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs",
+    "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs",
+    "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs",
+    "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs",
+    "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️blueprint/🦀️.rs",
+    "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️blueprint/🪛️utilities/🔄️transform/🦀️.rs",
+    "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️blueprint/🫧️transient/🦀️.rs"
   ],
   "owners": [
@@ -116,4 +142,108 @@
         },
         {
+          "id": "canvasPointerDown",
+          "disposition": "Migrated",
+          "lanes": [
+            "HostOnly"
+          ],
+          "blocker": ""
+        },
+        {
+          "id": "canvasPointerMove",
+          "disposition": "Migrated",
+          "lanes": [
+            "HostOnly"
+          ],
+          "blocker": ""
+        },
+        {
+          "id": "canvasPointerUp",
+          "disposition": "Migrated",
+          "lanes": [
+            "HostOnly"
+          ],
+          "blocker": ""
+        },
+        {
+          "id": "focusEntity",
+          "disposition": "Migrated",
+          "lanes": [
+            "WindowConfig"
+          ],
+          "blocker": ""
+        },
+        {
+          "id": "patchCombination",
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact"
+          ],
+          "blocker": ""
+        },
+        {
+          "id": "patchElement",
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact"
+          ],
+          "blocker": ""
+        },
+        {
+          "id": "patchLoad",
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact"
+          ],
+          "blocker": ""
+        },
+        {
+          "id": "patchLoadCase",
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact"
+          ],
+          "blocker": ""
+        },
+        {
+          "id": "patchMaterial",
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact"
+          ],
+          "blocker": ""
+        },
+        {
+          "id": "patchNode",
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact"
+          ],
+          "blocker": ""
+        },
+        {
+          "id": "patchRegion",
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact"
+          ],
+          "blocker": ""
+        },
+        {
+          "id": "patchSection",
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact"
+          ],
+          "blocker": ""
+        },
+        {
+          "id": "patchSupport",
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact"
+          ],
+          "blocker": ""
+        },
+        {
           "id": "removeSelection",
           "disposition": "Migrated",
@@ -124,8 +254,35 @@
         },
         {
+          "id": "resultAnimationTick",
+          "disposition": "Migrated",
+          "lanes": [
+            "WindowConfig",
+            "WindowTransient"
+          ],
+          "blocker": ""
+        },
+        {
+          "id": "rotateSelection",
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact",
+            "Transient"
+          ],
+          "blocker": ""
+        },
+        {
+          "id": "scaleSelection",
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact",
+            "Transient"
+          ],
+          "blocker": ""
+        },
+        {
           "id": "setActiveExample",
           "disposition": "Migrated",
           "lanes": [
-            "Config"
+            "HostOnly"
           ],
           "blocker": ""
@@ -143,5 +300,14 @@
           "disposition": "Migrated",
           "lanes": [
-            "Config"
+            "WindowConfig"
+          ],
+          "blocker": ""
+        },
+        {
+          "id": "setResultAnimation",
+          "disposition": "Migrated",
+          "lanes": [
+            "WindowConfig",
+            "WindowTransient"
           ],
           "blocker": ""
@@ -151,5 +317,5 @@
           "disposition": "Migrated",
           "lanes": [
-            "Config"
+            "WindowConfig"
           ],
           "blocker": ""
@@ -162,5 +328,28 @@
           ],
           "blocker": ""
+        },
+        {
+          "id": "setTransformGumballFlag",
+          "disposition": "Migrated",
+          "lanes": [
+            "HostOnly"
+          ],
+          "blocker": ""
+        },
+        {
+          "id": "translateSelection",
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact",
+            "Transient"
+          ],
+          "blocker": ""
         }
+      ],
+      "preparationLane": "WindowConfig",
+      "registrationWitnesses": [
+        "registry.register::<model_window::config::Fem2dModelWindowConfigOwner>()",
+        "registry.register::<results_window::config::Fem2dResultsWindowConfigOwner>()",
+        "registry.register::<results_window::transient::Fem2dResultsWindowTransientOwner>()"
       ]
     },
@@ -172,13 +361,18 @@
         {
           "id": "addGeneration",
-          "disposition": "BatchOnlyPendingRewrite",
-          "lanes": [],
-          "blocker": "Generation projection and preview evaluate/clone the full generation state; needs retained generation/preview computation and exact mixed Artifact/Config preparation."
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact",
+            "Config"
+          ],
+          "blocker": ""
         },
         {
           "id": "addWidget",
-          "disposition": "BatchOnlyPendingRewrite",
-          "lanes": [],
-          "blocker": "Whole FlowHostSnapshot/graph clone and host operation diff remain monolithic; needs granular graph root and Artifact Store preparation."
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact"
+          ],
+          "blocker": ""
         },
         {
@@ -216,13 +410,25 @@
         {
           "id": "connectMediaPorts",
-          "disposition": "BatchOnlyPendingRewrite",
-          "lanes": [],
-          "blocker": "Whole FlowHostSnapshot/graph clone and host operation diff remain monolithic; needs granular graph root and Artifact Store preparation."
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact"
+          ],
+          "blocker": ""
+        },
+        {
+          "id": "flowEvalResolve",
+          "disposition": "Migrated",
+          "lanes": [
+            "HostOnly"
+          ],
+          "blocker": ""
         },
         {
           "id": "flowEvalTick",
-          "disposition": "BatchOnlyPendingRewrite",
-          "lanes": [],
-          "blocker": "FlowEvalSession is reconstructed per call; app-instance evaluator authority and retained evaluator state must own this route before it can be interactive."
+          "disposition": "Migrated",
+          "lanes": [
+            "Transient"
+          ],
+          "blocker": ""
         },
         {
@@ -236,13 +442,17 @@
         {
           "id": "moveMediaNode",
-          "disposition": "BatchOnlyPendingRewrite",
-          "lanes": [],
-          "blocker": "Whole FlowHostSnapshot/graph clone and host operation diff remain monolithic; needs granular graph root and Artifact Store preparation."
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact"
+          ],
+          "blocker": ""
         },
         {
           "id": "nodeGraphEdit",
-          "disposition": "BatchOnlyPendingRewrite",
-          "lanes": [],
-          "blocker": "Whole FlowHostSnapshot/graph clone and host operation diff remain monolithic; needs granular graph root and Artifact Store preparation."
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact"
+          ],
+          "blocker": ""
         },
         {
@@ -250,4 +460,13 @@
           "disposition": "Migrated",
           "lanes": [
+            "WindowConfig"
+          ],
+          "blocker": ""
+        },
+        {
+          "id": "removeGeneration",
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact",
             "Config"
           ],
@@ -255,38 +474,60 @@
         },
         {
-          "id": "removeGeneration",
-          "disposition": "BatchOnlyPendingRewrite",
-          "lanes": [],
-          "blocker": "Generation projection and preview evaluate/clone the full generation state; needs retained generation/preview computation and exact mixed Artifact/Config preparation."
-        },
-        {
           "id": "removeWidget",
-          "disposition": "BatchOnlyPendingRewrite",
-          "lanes": [],
-          "blocker": "Whole FlowHostSnapshot/graph clone and host operation diff remain monolithic; needs granular graph root and Artifact Store preparation."
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact"
+          ],
+          "blocker": ""
         },
         {
           "id": "renameGeneration",
-          "disposition": "BatchOnlyPendingRewrite",
-          "lanes": [],
-          "blocker": "Generation projection and preview evaluate/clone the full generation state; needs retained generation/preview computation and exact mixed Artifact/Config preparation."
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact",
+            "Config"
+          ],
+          "blocker": ""
         },
         {
           "id": "reorganize",
-          "disposition": "BatchOnlyPendingRewrite",
-          "lanes": [],
-          "blocker": "Whole FlowHostSnapshot/graph clone and host operation diff remain monolithic; needs granular graph root and Artifact Store preparation."
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact"
+          ],
+          "blocker": ""
         },
         {
           "id": "selectGeneration",
-          "disposition": "BatchOnlyPendingRewrite",
-          "lanes": [],
-          "blocker": "Generation projection and preview evaluate/clone the full generation state; needs retained generation/preview computation and exact mixed Artifact/Config preparation."
+          "disposition": "Migrated",
+          "lanes": [
+            "Config"
+          ],
+          "blocker": ""
+        },
+        {
+          "id": "setActiveExample",
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact",
+            "Config"
+          ],
+          "blocker": ""
+        },
+        {
+          "id": "setContributions",
+          "disposition": "Migrated",
+          "lanes": [
+            "HostOnly"
+          ],
+          "blocker": ""
         },
         {
           "id": "setEvalOutputs",
-          "disposition": "BatchOnlyPendingRewrite",
-          "lanes": [],
-          "blocker": "FlowEvalSession is reconstructed per call; app-instance evaluator authority and retained evaluator state must own this route before it can be interactive."
+          "disposition": "Migrated",
+          "lanes": [
+            "HostOnly"
+          ],
+          "blocker": ""
         },
         {
@@ -300,9 +541,14 @@
         {
           "id": "updateGenerationValues",
-          "disposition": "BatchOnlyPendingRewrite",
-          "lanes": [],
-          "blocker": "Generation projection and preview evaluate/clone the full generation state; needs retained generation/preview computation and exact mixed Artifact/Config preparation."
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact",
+            "Config"
+          ],
+          "blocker": ""
         }
-      ]
+      ],
+      "preparationLane": "Config",
+      "registrationWitnesses": []
     },
     {
@@ -313,13 +559,18 @@
         {
           "id": "addFrame",
-          "disposition": "BatchOnlyPendingRewrite",
-          "lanes": [],
-          "blocker": "LayoutSnapshot owns deep page/frame/style roots; mutation cloning/diff and Artifact Store publication require retained per-item preparation."
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact"
+          ],
+          "blocker": ""
         },
         {
           "id": "addPage",
-          "disposition": "BatchOnlyPendingRewrite",
-          "lanes": [],
-          "blocker": "LayoutSnapshot owns deep page/frame/style roots; mutation cloning/diff and Artifact Store publication require retained per-item preparation."
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact",
+            "WindowConfig"
+          ],
+          "blocker": ""
         },
         {
@@ -327,5 +578,5 @@
           "disposition": "Migrated",
           "lanes": [
-            "Config"
+            "WindowTransient"
           ],
           "blocker": ""
@@ -335,5 +586,5 @@
           "disposition": "Migrated",
           "lanes": [
-            "Config"
+            "WindowTransient"
           ],
           "blocker": ""
@@ -341,19 +592,27 @@
         {
           "id": "canvasDrop",
-          "disposition": "BatchOnlyPendingRewrite",
-          "lanes": [],
-          "blocker": "LayoutSnapshot owns deep page/frame/style roots; mutation cloning/diff and Artifact Store publication require retained per-item preparation."
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact",
+            "WindowConfig",
+            "WindowTransient"
+          ],
+          "blocker": ""
         },
         {
           "id": "canvasPointerDown",
-          "disposition": "BatchOnlyPendingRewrite",
-          "lanes": [],
-          "blocker": "Hit testing builds a full page display list before selection/hover effects; requires retained scene/hit-test cursor."
+          "disposition": "Migrated",
+          "lanes": [
+            "HostOnly"
+          ],
+          "blocker": ""
         },
         {
           "id": "canvasPointerMove",
-          "disposition": "BatchOnlyPendingRewrite",
-          "lanes": [],
-          "blocker": "Hit testing builds a full page display list before selection/hover effects; requires retained scene/hit-test cursor."
+          "disposition": "Migrated",
+          "lanes": [
+            "HostOnly"
+          ],
+          "blocker": ""
         },
         {
@@ -366,8 +625,16 @@
         },
         {
+          "id": "deleteSelection",
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact"
+          ],
+          "blocker": ""
+        },
+        {
           "id": "engagementInput",
           "disposition": "Migrated",
           "lanes": [
-            "Config"
+            "WindowTransient"
           ],
           "blocker": ""
@@ -417,5 +684,13 @@
           "disposition": "Migrated",
           "lanes": [
-            "Config"
+            "WindowConfig"
+          ],
+          "blocker": ""
+        },
+        {
+          "id": "patchDocument",
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact"
           ],
           "blocker": ""
@@ -423,13 +698,35 @@
         {
           "id": "patchFrame",
-          "disposition": "BatchOnlyPendingRewrite",
-          "lanes": [],
-          "blocker": "LayoutSnapshot owns deep page/frame/style roots; mutation cloning/diff and Artifact Store publication require retained per-item preparation."
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact"
+          ],
+          "blocker": ""
         },
         {
           "id": "patchPage",
-          "disposition": "BatchOnlyPendingRewrite",
-          "lanes": [],
-          "blocker": "LayoutSnapshot owns deep page/frame/style roots; mutation cloning/diff and Artifact Store publication require retained per-item preparation."
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact"
+          ],
+          "blocker": ""
+        },
+        {
+          "id": "rotateSelection",
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact",
+            "WindowTransient"
+          ],
+          "blocker": ""
+        },
+        {
+          "id": "scaleSelection",
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact",
+            "WindowTransient"
+          ],
+          "blocker": ""
         },
         {
@@ -437,5 +734,5 @@
           "disposition": "Migrated",
           "lanes": [
-            "Config"
+            "WindowConfig"
           ],
           "blocker": ""
@@ -445,8 +742,22 @@
           "disposition": "Migrated",
           "lanes": [
-            "Config"
+            "WindowConfig"
+          ],
+          "blocker": ""
+        },
+        {
+          "id": "translateSelection",
+          "disposition": "Migrated",
+          "lanes": [
+            "Artifact",
+            "WindowTransient"
           ],
           "blocker": ""
         }
+      ],
+      "preparationLane": "WindowConfig",
+      "registrationWitnesses": [
+        "blueprint::config::register(registry)",
+        "blueprint::transient::register(registry)"
       ]
     }


### 61 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪆️child/🦀️.rs
B/H/S/current: 1097/56/1098/57
--- 
+++ 
@@ -1,3 +1,4 @@
 //! 🪆️ Child members and child content: member keys and paths, the member registry, owned document ingress and child content views.
+use semio_framework_value::ErasedSnapshotRetirement as _;
 extern crate semio_framework_os_kernel as store;
 use std::mem::ManuallyDrop;
@@ -523,5 +524,5 @@
         if let Some(request) = self.request.as_ref() {
             if request.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: size_of_val(&*self.request), depth: 1, ..Default::default() }); }
-            return Ok(RetirementDemand { copy_bytes: request.next_copy_byte_demand()?, capacity_bytes: request.next_capacity_byte_demand(body)?, release_bytes: request.next_release_byte_demand()?, depth: request.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original member request parent depth overflow"))? });
+            return Ok({let mut demand=request.next_demand(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original member request parent depth overflow"))?;demand});
         }
         let Some((_, reference, owner)) = self.identity.as_ref() else { return Ok(RetirementDemand::default()) };
@@ -867,5 +868,5 @@
     pub fn retirement_demands<M: SpaceMember>(&self, children: &ChildMemberRegistry<M>, retiring: Option<&ArtifactFixedRegistry<ChildMemberRetirement<M>>>, body: usize) -> Result<RetirementDemand, ValueError> {
         if let Some(active) = self.active.as_ref() { let parent_copy=if active.terminal_is_empty(){size_of_val(&*self.active)}else{0};return Self::parent_snapshot_demand(store::os_store::snapshot_read::artifact_retirement_box_demands(active,body.saturating_sub(parent_copy))?,parent_copy); }
-        if let Some(owner)=self.metadata_close.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of_val(&*self.metadata_close),depth:1,..Default::default()})}else{Self::parent_snapshot_demand(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?},0)}}
+        if let Some(owner)=self.metadata_close.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of_val(&*self.metadata_close),depth:1,..Default::default()})}else{Self::parent_snapshot_demand(owner.next_demand(body)?,0)}}
         if let Some(original) = self.active_member.as_ref() {
             if self.require_member_terminal{
@@ -1066,5 +1067,5 @@
 
     pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
-        if let Some(owner)=self.metadata.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of_val(&*self.metadata),depth:1,..Default::default()})}else{ChildContentRetirement::parent_snapshot_demand(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?},0)}}
+        if let Some(owner)=self.metadata.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of_val(&*self.metadata),depth:1,..Default::default()})}else{ChildContentRetirement::parent_snapshot_demand(owner.next_demand(body)?,0)}}
         let Some(entry)=self.entry.as_ref()else{return Ok(Default::default())};
         if !entry.member.snapshot_read_leases_terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()})}


### 62 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/📥️retained/🦀️.rs
B/H/S/current: 1596/36/1611/36
--- 
+++ 
@@ -13,5 +13,5 @@
 #[doc(hidden)]
 pub fn controlled_demands<T: RetireOwned>(owner: &ControlledRetirement<T>, body: usize) -> Result<RetirementDemand, ValueError> {
-    Ok(RetirementDemand { copy_bytes: owner.next_copy_byte_demand()?, capacity_bytes: owner.next_capacity_byte_demand(body)?, release_bytes: owner.next_release_byte_demand()?, depth: owner.next_depth_demand()? })
+    Ok(owner.next_demand(body)?)
 }
 fn pack_close_receipt(step: store::mounted_pack_rt::RetainedPackCloseStep, demand: RetirementDemand) -> RetainedCloneStep {
@@ -43,4 +43,5 @@
     InputRetirement,
     StoreHydration,
+    StoreAddress,
     Ready,
     RetiringRejectedCandidate,
@@ -59,4 +60,5 @@
 pub enum WindowConfigPackLoadStep {
     Pending(WindowConfigPackLoadProgress),
+    OwnershipProgress(WindowConfigPackLoadProgress,RetainedCloneProgress),
     Ready,
     Rejected(WindowConfigPackLoadDiagnostic),
@@ -1074,4 +1076,6 @@
 
 impl<O:WindowConfigOwner> store::ErasedSnapshotRetirement for RetainedWindowConfigStateDecode<O>{
+ fn next_demand(&self,maximum_body_bytes:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.retirement_demands(maximum_body_bytes)}
+
  fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.retirement_demands(0).map(|demand|demand.copy_bytes)}
  fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.retirement_demands(body).map(|demand|demand.capacity_bytes)}
@@ -1115,5 +1119,5 @@
     history_decode: ManuallyDrop<Option<store::RetainedHistoryDecode>>,
     history: ManuallyDrop<Option<store::HistoryLog>>,
-    hydration: ManuallyDrop<Option<store::RetainedConfigStoreHydration<O::State, O::Mutation>>>,
+    hydration: ManuallyDrop<Option<store::ReferenceConfigStoreHydration<O::State, O::Mutation>>>,
     owners: ManuallyDrop<Option<store::os_store::document_store_owners::DocumentStoreOwners<O::State, O::Mutation>>>,
     candidate: ManuallyDrop<Option<WindowConfigPartition<O>>>,
@@ -1123,4 +1127,5 @@
     registry_lifetime: u64,
     partition_generation: Option<u64>,
+    candidate_authority: Option<super::WindowConfigAuthorityIdentity>,
     phase: WindowConfigPackLoadPhase,
     diagnostic: Option<WindowConfigPackLoadDiagnostic>,
@@ -1137,9 +1142,9 @@
         let total_bytes = files.pack.len().saturating_mul(3).saturating_add(files.spr.len()).saturating_add(files.ops.len()) as u64;
         let expected_id = format!("window-config:{}:{window_id}", O::WINDOW_KIND_ID);
-        let (owners, diagnostic) = match O::build_store_owners() {
-            Ok(owners) => (Some(owners), None),
-            Err(_) => (None, Some(WindowConfigPackLoadDiagnostic::Capacity)),
-        };
+        let candidate_authority = super::WindowConfigAuthorityIdentity::issue();
+        let owners=None;let mut diagnostic=None;
+        if candidate_authority.is_none() { diagnostic = Some(WindowConfigPackLoadDiagnostic::Capacity); }
         Self {
+            candidate_authority,
             completed_states:ManuallyDrop::new(std::array::from_fn(|_|None)),
             pending_files:ManuallyDrop::new(None),
@@ -1332,5 +1337,5 @@
             let expected_id = self.expected_id.take().expect("exact window config partition id remains");
             let owners = self.owners.take().expect("window config store owners remain");
-            *self.hydration = Some(store::RetainedConfigStoreHydration::from_snapshots(
+            *self.hydration = Some(store::ReferenceConfigStoreHydration::from_snapshots(
                 genesis,
                 validation,
@@ -1358,9 +1363,13 @@
             store::ConfigStoreHydrationStep::Ready(store) => {
 
-                *self.candidate = Some(WindowConfigPartition { store: *store, disposer: Some(O::build_store_disposer()), pending_preview:None,pending_preview_reads:None,pending_preview_projection:None,pending_preview_projection_read:None,preview_retirement:None,returned_read_retirement:None });
-                self.phase = WindowConfigPackLoadPhase::Ready;
-                WindowConfigPackLoadStep::Ready
-            }
-        }
+                *self.candidate = Some(WindowConfigPartition { catalog:None,address:None,semantic_preview:None,semantic_epoch:0,published_preview:None,finish_preview_base:None,finish_preview_read:None, authority: self.candidate_authority.expect("admitted original candidate authority"), store: *store, disposer: Some(O::build_store_disposer()), pending_preview:None,pending_preview_reads:None,pending_preview_projection:None,pending_preview_projection_read:None,preview_retirement:None,returned_read_retirement:None });
+                self.phase = WindowConfigPackLoadPhase::StoreAddress;
+                self.pending()
+            }
+        }
+    }
+
+    fn advance_address(&mut self,grant:RetainedCloneGrant)->WindowConfigPackLoadStep{
+        let demand=semio_framework_value::SharedUtf8::admission_demand();if !super::registry::fits(grant,demand){return self.pending();}let original=self.window_id.take().expect("original hydrated Window keeps its exact address");match semio_framework_value::SharedUtf8::admit(original,grant){Ok((address,p))=>{self.candidate.as_mut().expect("original hydrated Window candidate remains").address=Some(ControlledRetirement::new(address).expect("original Window address supports controlled closure"));self.phase=WindowConfigPackLoadPhase::Ready;WindowConfigPackLoadStep::OwnershipProgress(self.progress_now(),p)},Err((_,original))=>{*self.window_id=Some(original);self.reject(WindowConfigPackLoadDiagnostic::Capacity)}}
     }
 
@@ -1372,4 +1381,6 @@
             return self.pending();
         }
+        if self.owners.is_none()&&self.hydration.is_none()&&self.candidate.is_none()&&matches!(self.phase,WindowConfigPackLoadPhase::EnvelopeIdentity){return match O::build_store_owners(grant){Ok((owners,p))=>{*self.owners=Some(owners);WindowConfigPackLoadStep::OwnershipProgress(self.progress_now(),p)},Err(error)=>{*self.owners=error.owners;if self.owners.is_none()&&error.progress==Default::default(){self.pending()}else{self.diagnostic=Some(WindowConfigPackLoadDiagnostic::Capacity);WindowConfigPackLoadStep::OwnershipProgress(self.progress_now(),error.progress)}}}}
+        if let Some(owners)=self.owners.as_mut(){if !owners.constructor_is_complete(){return match owners.admit_constructor(grant){Ok(p)=>WindowConfigPackLoadStep::OwnershipProgress(self.progress_now(),p),Err((_,p))=>{self.diagnostic=Some(WindowConfigPackLoadDiagnostic::Capacity);WindowConfigPackLoadStep::OwnershipProgress(self.progress_now(),p)}}}}
         match self.phase {
             WindowConfigPackLoadPhase::EnvelopeIdentity | WindowConfigPackLoadPhase::PackIngress | WindowConfigPackLoadPhase::PackReplay => self.advance_decode(grant),
@@ -1378,4 +1389,5 @@
             WindowConfigPackLoadPhase::InputRetirement => self.advance_input_retirement(grant),
             WindowConfigPackLoadPhase::StoreHydration => self.advance_hydration(grant),
+            WindowConfigPackLoadPhase::StoreAddress => self.advance_address(grant),
             WindowConfigPackLoadPhase::Ready => WindowConfigPackLoadStep::Ready,
             WindowConfigPackLoadPhase::RetiringRejectedCandidate => WindowConfigPackLoadStep::Rejected(self.diagnostic.unwrap_or(WindowConfigPackLoadDiagnostic::Cancelled)),
@@ -1393,11 +1405,12 @@
             return self.reject_stale();
         }
-        let window_id = self.window_id.as_ref().expect("ready window config load retains its address");
+        let window_id = self.candidate.as_ref().and_then(|candidate|candidate.address.as_ref()).and_then(ControlledRetirement::original).expect("ready window config load retains its original shared address").as_str();
         let live_generation = partitions.get(window_id).map(|partition| partition.store.generation());
         if live_generation != self.partition_generation {
             return self.reject_stale();
         }
+        let address=window_id.to_string();
         let candidate = self.candidate.take().expect("ready window config candidate remains");
-        *self.displaced = partitions.insert(window_id.clone(), candidate);
+        *self.displaced = partitions.insert(address, candidate);
         self.committed = true;
         self.phase = WindowConfigPackLoadPhase::RetiringDisplacedStore;
@@ -1406,6 +1419,7 @@
 
     fn partition_demands(partition: &WindowConfigPartition<O>, body: usize) -> Result<RetirementDemand, ValueError> {
-        if partition.preview_retirement_pending() { return partition.preview_retirement_demand(); }
+        if partition.preview_close_pending() { return partition.preview_close_demand(body); }
         if partition.returned_read_pending() { return partition.returned_read_demands(body); }
+        if let Some(address)=partition.address.as_ref(){return if address.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<semio_framework_value::SharedUtf8>>>(),depth:1,..Default::default()})}else{address.next_demand(body)};}
         partition.disposer.as_ref().map_or(Ok(RetirementDemand { copy_bytes: size_of::<WindowConfigPartition<O>>(), depth: 1, ..Default::default() }), |owner| {
             if owner.terminal_is_empty(&partition.store) { Ok(RetirementDemand { release_bytes: size_of_val(owner.as_ref()), depth: 1, ..Default::default() }) }
@@ -1416,6 +1430,7 @@
     fn close_partition(slot: &mut Option<WindowConfigPartition<O>>, grant: RetainedCloneGrant, demand: RetirementDemand) -> Result<PluginLifecycleStep, String> {
         let partition = slot.as_mut().expect("granted original partition remains");
-        if partition.preview_retirement_pending() { return partition.preview_retirement_step(grant).map(|step| PluginLifecycleStep::retained(step, false)).map_err(ValueError::into_message); }
+        if partition.preview_close_pending() { return partition.preview_close_step(grant).map(|step| PluginLifecycleStep::retained(step, false)).map_err(ValueError::into_message); }
         if partition.returned_read_pending() { return partition.returned_read_step(grant).map(|step| PluginLifecycleStep::retained(step, false)).map_err(ValueError::into_message); }
+        if let Some(address)=partition.address.as_mut(){if address.terminal_is_empty(){partition.address=None;return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}return address.step(grant).map(|step|PluginLifecycleStep::retained(step,false)).map_err(ValueError::into_message);}
         if let Some(owner) = partition.disposer.as_mut() {
             if !owner.terminal_is_empty(&partition.store) { return owner.close_step(&mut partition.store, grant).map(|step| match step { PluginLifecycleStep::Complete(progress) => PluginLifecycleStep::Progress(progress), other => other }).map_err(|fault| fault.message); }
@@ -1436,5 +1451,5 @@
         if self.history_decode.is_some() { return frame(size_of::<store::RetainedHistoryDecode>().checked_add(Self::original_demand().copy_bytes).ok_or_else(|| ValueError::literal(ValueRefusalKind::WorkLimit, "window history handoff size overflow"))?); }
         if let Some(owner) = self.hydration.as_ref() {
-            return if store::ErasedSnapshotRetirement::terminal_is_empty(owner) { frame(size_of::<store::RetainedConfigStoreHydration<O::State, O::Mutation>>()) }
+            return if store::ErasedSnapshotRetirement::terminal_is_empty(owner) { frame(size_of::<store::ReferenceConfigStoreHydration<O::State, O::Mutation>>()) }
             else { Ok(RetirementDemand { copy_bytes: store::ErasedSnapshotRetirement::next_copy_byte_demand(owner)?, capacity_bytes: store::ErasedSnapshotRetirement::next_capacity_byte_demand(owner, body)?, release_bytes: store::ErasedSnapshotRetirement::next_release_byte_demand(owner)?, depth: store::ErasedSnapshotRetirement::next_depth_demand(owner)? }) };
         }


### 63 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs
B/H/S/current: 1013/53/1130/61
--- 
+++ 
@@ -16,4 +16,8 @@
 use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
 
+#[path="🧬️preview/🦀️.rs"]
+mod semantic_preview;
+use semantic_preview::WindowSemanticPreview;
+
 #[path="🧬️preparation/🔁️replace/🦀️.rs"]
 mod replace_edit;
@@ -49,13 +53,17 @@
     fn build_mutation_retirement_factory()->Arc<dyn store::ArtifactOwnedValueRetirementFactory<Self::Mutation>>{Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<Self::Mutation>::default())}
     fn build_snapshot_retirement_factory()->Arc<dyn store::SnapshotRetirementFactory<Self::State>>{Arc::new(semio_framework_value::factory::SharedValueRetirementFactory::<Self::State>::default())}
-    fn build_store_owners() -> Result<store::os_store::document_store_owners::DocumentStoreOwners<Self::State, Self::Mutation>, ValueError>;
+    fn preparation_source_birth()->semio_framework_value::retained_clone::RetainedCloneBirthDemand;
+    fn build_store_owners(grant:RetainedCloneGrant) -> Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::State, Self::Mutation>,RetainedCloneProgress),store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::State,Self::Mutation>>;
     fn build_one_item_preparation_factory() -> Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>>;
     fn build_store_disposer() -> Box<dyn ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>>;
 }
 
-pub fn bounded_window_config_store_owners<O: WindowConfigOwner>() -> Result<store::os_store::document_store_owners::DocumentStoreOwners<O::State, O::Mutation>, ValueError> {
-    store::os_store::document_store_owners::funded_bounded_artifact_store_owners_with_one_item_preparation::<O::State, O::Mutation>(O::build_one_item_preparation_factory())
-}
-
+/// 📏️ Quotes the concrete zero-payload issuer tree before any original Window factory allocates.
+pub fn bounded_window_config_preparation_source_birth<O:WindowConfigOwner>()->semio_framework_value::retained_clone::RetainedCloneBirthDemand{semio_framework_value::retained_clone::RetainedCloneBirthDemand{capacity_bytes:semio_framework_value::factory_arc_birth_bytes::<O::Edit>()+semio_framework_value::factory_arc_birth_bytes::<semio_framework_value::factory::OwnedValueRetirementFactory<O::Mutation>>()+semio_framework_value::factory_arc_birth_bytes::<semio_framework_value::factory::SharedValueRetirementFactory<O::State>>(),depth:1}}
+/// 🎟️ Constructs the exact paired native Window factory only after the caller funds its declared original sources.
+pub fn bounded_window_config_store_owners<O:WindowConfigOwner>(grant:RetainedCloneGrant)->Result<(store::os_store::document_store_owners::DocumentStoreOwners<O::State,O::Mutation>,RetainedCloneProgress),store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<O::State,O::Mutation>>{
+ let copy=2*std::mem::size_of::<store::os_store::document_store_owners::DocumentStoreOwners<O::State,O::Mutation>>();if grant.maximum_copy_bytes<copy{return Err(store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError{error:ValueError::literal(ValueRefusalKind::WorkLimit,"original Window catalog header transfer not funded"),owners:None,progress:Default::default()})}
+ store::os_store::document_store_owners::bounded_artifact_store_owners_with_shared_semantic_preparation(grant,O::preparation_source_birth(),copy,||store::snapshot_clone_preparation::RetainedClonePreparationFactory::new(O::build_retained_edit(),O::build_mutation_retirement_factory(),O::build_snapshot_retirement_factory(),O::MAXIMUM_PREPARATION_DEPTH).expect("original Window native depth declared before admission"))
+}
 /// 🧬️ Uses the original domain edit issuer and Store's admitted clone, metadata and canonical-seal owners.
 pub fn bounded_window_config_preparation_factory<O: WindowConfigOwner>() -> Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<O::State, O::Mutation>> {
@@ -127,5 +135,5 @@
 
 impl crate::window_mutation::refresh::WindowRefreshSnapshot for WindowConfigSnapshot{fn swap_address(&mut self,other:&mut Self){std::mem::swap(&mut self.window_id,&mut other.window_id);}}
-impl WindowConfigSnapshot{pub fn try_duplicate(&self)->Result<Self,ValueError>{Ok(Self{window_id:self.window_id.clone(),window_kind_id:self.window_kind_id,generation:self.generation,revision:self.revision,snapshot:self.snapshot.try_duplicate()?})}}
+impl WindowConfigSnapshot{pub fn try_duplicate(&self)->Result<Self,ValueError>{Ok(Self{authority:self.authority,window_id:self.window_id.clone(),window_kind_id:self.window_kind_id,generation:self.generation,revision:self.revision,snapshot:self.snapshot.try_duplicate()?})}}
 /// 📖️ Immutable snapshot of one exact window-owned config partition.
 #[derive(semio_framework_value::RetireOwned)]
@@ -143,5 +151,35 @@
 }
 
+/// 🪪️ Checked process-unique identity issued to one original window partition.
+#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, semio_framework_value::RetireOwned)]
+pub struct WindowConfigAuthorityIdentity(u64);
+
+impl WindowConfigAuthorityIdentity {
+    fn issue() -> Option<Self> {
+        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
+        NEXT.fetch_update(std::sync::atomic::Ordering::Relaxed, std::sync::atomic::Ordering::Relaxed, |next| next.checked_add(1)).ok().map(Self)
+    }
+}
+
+/// 👁️ Borrows one original committed window partition without issuing or copying an owner.
+pub struct WindowConfigBorrow<'a> {
+    pub authority: WindowConfigAuthorityIdentity,
+    pub window_id: &'a str,
+    pub window_kind_id: &'static str,
+    pub generation: u64,
+    pub revision: [u8; 32],
+    state: &'a dyn std::any::Any,
+}
+
+impl WindowConfigBorrow<'_> {
+    /// 🧬️ Lends only the state of the exact declared window owner.
+    pub fn get<O: WindowConfigOwner>(&self) -> Option<&O::State> {
+        (self.window_kind_id == O::WINDOW_KIND_ID).then(|| self.state.downcast_ref::<O::State>()).flatten()
+    }
+}
+
 impl WindowConfigSnapshot {
+    /// 🪪️ Returns the exact original partition authority captured with this snapshot.
+    pub fn authority(&self) -> WindowConfigAuthorityIdentity { self.authority }
     pub fn generation(&self) -> u64 {
         self.generation
@@ -224,5 +262,5 @@
     fn pending_bytes(&self)->usize{self.pending.capacity().checked_mul(size_of::<O::Mutation>()).expect("admitted original window backing remains bounded")}
     fn controlled_demand<T:semio_framework_value::retirement::RetireOwned>(owner:&ControlledRetirement<T>,body:usize)->Result<RetirementDemand,ValueError>{
-        Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})
+        Ok(owner.next_demand(body)?)
     }
     fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
@@ -292,5 +330,5 @@
     }
 
-    fn ingress_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:self.ingress.next_copy_byte_demand()?,capacity_bytes:self.ingress.next_capacity_byte_demand(body)?,release_bytes:self.ingress.next_release_byte_demand()?,depth:self.ingress.next_depth_demand()?})}
+    fn ingress_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{Ok(self.ingress.next_demand(body)?)}
     fn close_ingress(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.ingress.step(grant)}
     fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demand(body)}
@@ -305,4 +343,12 @@
 
 struct WindowConfigPartition<O: WindowConfigOwner> {
+    catalog:Option<store::os_store::document_store_owners::DocumentStoreOwners<O::State,O::Mutation>>,
+    address:Option<ControlledRetirement<semio_framework_value::SharedUtf8>>,
+    semantic_preview:Option<WindowSemanticPreview<O>>,
+    semantic_epoch:u64,
+    published_preview:Option<WindowConfigSnapshot>,
+    finish_preview_base:Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<O::State>>,
+    finish_preview_read:Option<store::os_store::snapshot_read::ErasedSnapshotRead>,
+    authority: WindowConfigAuthorityIdentity,
     store: WindowConfigStore<O>,
     disposer: Option<Box<dyn ArtifactOwnedDisposer<WindowConfigStore<O>>>>,
@@ -324,11 +370,54 @@
         Ok(RetainedCloneStep::Progress(progress))
     }
-    fn preview_retirement_pending(&self)->bool{self.pending_preview.is_some()||self.pending_preview_reads.is_some()||self.pending_preview_projection.is_some()||self.pending_preview_projection_read.is_some()||self.preview_retirement.is_some()}
-    fn preview_retirement_demand(&self)->Result<RetirementDemand,ValueError>{
-        if self.pending_preview_projection.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()});}
+    fn catalog_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
+        if self.store.owned_disposer_installed(){return Ok(Default::default());}
+        if let Some(catalog)=self.catalog.as_ref(){return if catalog.constructor_is_complete(){Ok(RetirementDemand{copy_bytes:2*size_of::<store::os_store::document_store_owners::DocumentStoreOwners<O::State,O::Mutation>>(),depth:1,..Default::default()})}else{catalog.constructor_demands(body)};}
+        let mut d=store::os_store::document_store_owners::bounded_artifact_store_owners_source_demands::<O::State,O::Mutation>()?;let extra=O::preparation_source_birth();d.capacity_bytes=d.capacity_bytes.checked_add(extra.capacity_bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original Window source catalog birth overflow"))?;d.copy_bytes=2*size_of::<store::os_store::document_store_owners::DocumentStoreOwners<O::State,O::Mutation>>();d.depth=d.depth.max(extra.depth);Ok(d)
+    }
+    fn catalog_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
+        if self.store.owned_disposer_installed(){return Ok(RetainedCloneStep::Complete(Default::default()));}let d=self.catalog_demands(grant.maximum_copy_bytes)?;if !registry::fits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
+        if self.catalog.is_none(){return match O::build_store_owners(grant){Ok((owners,p))=>{self.catalog=Some(owners);Ok(RetainedCloneStep::Progress(p))},Err(error)=>{self.catalog=error.owners;Err(error.error.with_retained_progress(error.progress))}};}
+        let owners=self.catalog.as_mut().unwrap();if !owners.constructor_is_complete(){return owners.admit_constructor(grant).map(RetainedCloneStep::Progress).map_err(|(error,p)|error.with_retained_progress(p));}
+        let original=self.catalog.take().unwrap();match self.store.install_document_store_owners_exact(original){Ok(())=>Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),Err((error,owners))=>{self.catalog=Some(owners);Err(error)}}
+    }
+    fn preview_finish_demands(&self,epoch:u64,body:usize)->Result<RetirementDemand,ValueError>{
+        if self.pending_preview.is_some()||self.preview_retirement.is_some(){return self.preview_retirement_demand(body);}
+        if self.semantic_epoch!=epoch||self.semantic_preview.is_none(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<WindowConfigSnapshot>>(),depth:1,..Default::default()});}
+        if let Some(base)=self.finish_preview_base.as_ref(){if self.finish_preview_read.is_none(){return self.store.retained_derived_read_demands(base);}return Ok(RetirementDemand{copy_bytes:size_of::<WindowConfigSnapshot>()+size_of::<store::os_store::snapshot_read::ArtifactDerivedSnapshot<O::State>>(),depth:1,..Default::default()});}
+        if self.semantic_preview.as_ref().is_some_and(WindowSemanticPreview::leaf_ready){return Ok(RetirementDemand{copy_bytes:size_of::<store::os_store::snapshot_read::ArtifactDerivedSnapshot<O::State>>(),depth:1,..Default::default()});}
+        self.preview_retirement_demand(body)
+    }
+    fn preview_finish_step(&mut self,epoch:u64,grant:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),ValueError>{let d=self.preview_finish_demands(epoch,grant.maximum_copy_bytes)?;if !registry::fits(grant,d){return Ok((false,Default::default()));}
+        if self.pending_preview.is_some()||self.preview_retirement.is_some(){return self.preview_retirement_step(grant).map(|step|(false,step.progress()));}
+        if self.semantic_epoch!=epoch||self.semantic_preview.is_none(){self.pending_preview=self.published_preview.take();return Ok((true,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}
+        if let Some(base)=self.finish_preview_base.as_ref(){if self.finish_preview_read.is_none(){return self.store.admit_retained_derived_read(base,&mut self.finish_preview_read,grant).map(|step|(false,step.progress()));}
+            let read=self.finish_preview_read.take().unwrap();let snapshot=WindowConfigSnapshot{authority:self.authority,window_id:self.address.as_ref().and_then(ControlledRetirement::original).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original Window preview lost its address"))?.clone(),window_kind_id:O::WINDOW_KIND_ID,generation:self.store.generation(),revision:self.store.content_revision_now(),snapshot:read};self.pending_preview=self.published_preview.replace(snapshot);drop(self.finish_preview_base.take());return Ok((true,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}
+        let owner=self.semantic_preview.as_mut().unwrap();if owner.leaf_ready(){if let Some((base,p))=owner.take_base(grant)?{self.finish_preview_base=Some(base);return Ok((false,p));}}self.preview_retirement_step(grant).map(|step|(false,step.progress()))
+    }
+    fn preview_retirement_pending(&self)->bool{self.semantic_preview.as_ref().is_some_and(WindowSemanticPreview::work_pending)||self.pending_preview.is_some()||self.pending_preview_reads.is_some()||self.pending_preview_projection.is_some()||self.pending_preview_projection_read.is_some()||self.preview_retirement.is_some()}
+    fn preview_close_pending(&self)->bool{self.semantic_preview.is_some()||self.published_preview.is_some()||self.finish_preview_base.is_some()||self.finish_preview_read.is_some()||self.preview_retirement_pending()}
+    fn preview_close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.preview_cancel_demand(body,true)}
+    fn preview_cancel_demand(&self,body:usize,clear_published:bool)->Result<RetirementDemand,ValueError>{
+        if self.pending_preview.is_some()||self.pending_preview_projection_read.is_some()||self.pending_preview_projection.is_some()||self.preview_retirement.is_some(){return self.preview_retirement_demand(body);}
+        if clear_published&&self.published_preview.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<WindowConfigSnapshot>>(),depth:1,..Default::default()});}
+        if self.finish_preview_read.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<store::os_store::snapshot_read::ErasedSnapshotRead>>(),depth:1,..Default::default()});}
+        if self.finish_preview_base.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<O::State>>>(),depth:1,..Default::default()});}
+        if self.semantic_preview.as_ref().is_some_and(WindowSemanticPreview::needs_cancellation){return Ok(RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()});}self.preview_retirement_demand(body)
+    }
+    fn preview_close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.preview_cancel_step(grant,true)}
+    fn preview_cancel_step(&mut self,grant:RetainedCloneGrant,clear_published:bool)->Result<RetainedCloneStep,ValueError>{let d=self.preview_cancel_demand(grant.maximum_copy_bytes,clear_published)?;if !registry::fits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
+        if self.pending_preview.is_some()||self.pending_preview_projection_read.is_some()||self.pending_preview_projection.is_some()||self.preview_retirement.is_some(){return self.preview_retirement_step(grant);}
+        let p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
+        if clear_published&&self.published_preview.is_some(){self.pending_preview=self.published_preview.take();return Ok(RetainedCloneStep::Progress(p));}
+        if self.finish_preview_read.is_some(){self.pending_preview_projection_read=self.finish_preview_read.take();return Ok(RetainedCloneStep::Progress(p));}
+        if self.finish_preview_base.is_some(){self.pending_preview_projection=self.finish_preview_base.take();return Ok(RetainedCloneStep::Progress(p));}
+        if self.semantic_preview.as_ref().is_some_and(WindowSemanticPreview::needs_cancellation){self.semantic_preview.as_mut().unwrap().cancel();return Ok(RetainedCloneStep::Progress(p));}self.preview_retirement_step(grant)
+    }
+    fn preview_retirement_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
+        if let Some(owner)=self.semantic_preview.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<WindowSemanticPreview<O>>>(),depth:1,..Default::default()})}else{owner.demand(&self.store,body)};}
+        if let Some(projection)=self.pending_preview_projection.as_ref(){let mut d=self.store.retained_derived_read_demands(projection)?;d.copy_bytes+=size_of::<Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<O::State>>>();return Ok(d);}
         if let Some(owner)=self.preview_retirement.as_ref(){
             if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<WindowPreviewOwners>>>(),depth:1,..Default::default()});}
-            let copy=owner.next_copy_byte_demand()?;let release=owner.next_release_byte_demand()?;
-            return Ok(RetirementDemand{copy_bytes:copy,capacity_bytes:owner.next_capacity_byte_demand(if copy>0{copy}else{release})?,release_bytes:release,depth:owner.next_depth_demand()?});
+            return owner.next_demand(body);
         }
         Ok(RetirementDemand{copy_bytes:size_of::<WindowPreviewOwners>(),depth:1,..Default::default()})
@@ -336,10 +425,14 @@
     fn preview_retirement_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
         if !self.preview_retirement_pending(){return Ok(RetainedCloneStep::Complete(Default::default()));}
-        let demand=self.preview_retirement_demand()?;
+        let demand=self.preview_retirement_demand(grant.maximum_copy_bytes)?;
         if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()));}
+        if let Some(owner)=self.semantic_preview.as_mut(){if owner.terminal_is_empty(){self.semantic_preview=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}return owner.step(&self.store,grant);}
         if let Some(projection)=self.pending_preview_projection.as_ref(){
-            let read=match self.store.retained_derived_read(projection){Ok(read)=>read,Err(_)=>return Ok(RetainedCloneStep::Progress(Default::default()))};
-            self.pending_preview_projection_read=Some(read);drop(self.pending_preview_projection.take());
-            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));
+            let overhead=size_of::<Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<O::State>>>();
+            let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-overhead,..grant};
+            let step=self.store.admit_retained_derived_read(projection,&mut self.pending_preview_projection_read,child)?;
+            let mut progress=step.progress();
+            if matches!(step,RetainedCloneStep::Complete(_)){drop(self.pending_preview_projection.take());progress.copied_bytes+=overhead;}
+            return Ok(RetainedCloneStep::Progress(progress));
         }
         if let Some(owner)=self.preview_retirement.as_mut(){
@@ -356,8 +449,10 @@
 }
 trait ErasedWindowConfigStoreOwner: Send {
+    fn admit_catalog(&mut self,authority:&WindowConfigAuthority,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>;
+    fn borrow<'a>(&'a self, window_id: &'a str) -> Option<WindowConfigBorrow<'a>>;
     fn capture<'a>(&'a mut self, window_id: &'a str) -> Pin<Box<dyn Future<Output = Result<WindowConfigAuthority, Fault>> + 'a>>;
     fn dispatch<'a, 'b>(&'a mut self, actor: &'a str, mutation: WindowConfigMutation, identity: &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>) -> Pin<Box<dyn Future<Output = Result<(), Fault>> + 'a>>;
     fn admit_begin(&mut self, operation: semio_framework_job::OperationId, actor: &mut Option<ControlledRetirement<semio_framework_value::SharedUtf8>>, mutations: &mut Vec<WindowConfigMutation>, authority: &WindowConfigAuthority, grant: RetainedCloneGrant) -> Result<Option<(Box<dyn ErasedWindowConfigPublication>,RetainedCloneProgress)>,ValueError>;
-    fn advance(&mut self, publication: &mut dyn ErasedWindowConfigPublication, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item_publication::ArtifactStoreOneItemAdvance, Fault>;
+    fn advance(&mut self, publication: &mut dyn ErasedWindowConfigPublication, body:usize, grant: store::os_store::one_item::ArtifactStoreOneItemGrant,receiving:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<store::os_store::one_item_publication::ArtifactStoreOneItemAdvance, Fault>;
     fn refresh_demands(&self,authority:&WindowConfigAuthority,body:usize)->Result<RetirementDemand,ValueError>;
     fn refresh(&mut self,authority:&mut WindowConfigAuthority,grant:RetainedCloneGrant)->Result<crate::window_mutation::WindowAuthorityRefreshStep,Fault>;
@@ -377,10 +472,17 @@
     fn pointer_values(&self, pointers: &[String]) -> Vec<(String, Vec<Option<semio_framework_value::DslValue>>)>;
     fn snapshot(&self, window_id: &str) -> Option<WindowConfigSnapshot>;
-    fn preview(&mut self, window_id: &str, mutations: &[&WindowConfigMutation]) -> Option<WindowConfigSnapshot>;
+    fn preview_input_demands(&self,window:&str,epoch:u64,body:usize)->Result<RetirementDemand,ValueError>;
+    fn preview_input(&mut self,input:semio_framework_value::retained_clone::RetainedCloneRef<'_,WindowConfigMutation>,epoch:u64,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>;
+    fn preview_leaf_ready(&self,window:&str,epoch:u64)->bool;
+    fn preview_input_held(&self,window:&str,epoch:u64)->bool;
+    fn preview_partition_count(&self)->usize;
+    fn preview_partition_demands(&self,index:usize,epoch:u64,closing:bool,body:usize)->Result<RetirementDemand,ValueError>;
+    fn preview_partition_step(&mut self,index:usize,epoch:u64,closing:bool,grant:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),ValueError>;
+    fn preview_or<'a>(&'a self,committed:&'a WindowConfigSnapshot)->&'a WindowConfigSnapshot;
     fn preview_available(&self,window_id:&str)->bool;
     fn can_retire_preview(&self,preview:&WindowConfigSnapshot)->bool;
     fn retire_preview(&mut self,preview:&mut Option<WindowConfigSnapshot>)->bool;
     fn preview_retirement_pending(&self)->bool;
-    fn preview_retirement_demand(&self)->Result<RetirementDemand,ValueError>;
+    fn preview_retirement_demand(&self,body:usize)->Result<RetirementDemand,ValueError>;
     fn preview_retirement_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 }
@@ -409,13 +511,11 @@
     pub async fn partition(&mut self, window_id: &str) -> Result<&mut WindowConfigPartition<O>, Fault> {
         if !self.partitions.contains_key(window_id) {
+            let authority = WindowConfigAuthorityIdentity::issue().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.authority-exhausted"), "window config authority identities exhausted"))?;
             let id = format!("window-config:{}:{window_id}", O::WINDOW_KIND_ID);
             let envelope = store::create_config_envelope::<O::State, O::Mutation>(O::SCHEMA, &id, O::State::default(), None).await;
             let config = store::ConfigStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, self.actor.as_ref().expect("live window owner retains its opened actor").clone()).await.map_err(|error| error.into_fault())?;
-            self.partitions.insert(window_id.to_string(), WindowConfigPartition { store: config, disposer: Some(O::build_store_disposer()), pending_preview:None,pending_preview_reads:None,pending_preview_projection:None,pending_preview_projection_read:None,preview_retirement:None,returned_read_retirement:None });
+            self.partitions.insert(window_id.to_string(), WindowConfigPartition { catalog:None,address:Some(ControlledRetirement::new(semio_framework_value::SharedUtf8::from(window_id)).expect("original registered Window address supports controlled closure")),semantic_preview:None,semantic_epoch:0,published_preview:None,finish_preview_base:None,finish_preview_read:None, authority, store: config, disposer: Some(O::build_store_disposer()), pending_preview:None,pending_preview_reads:None,pending_preview_projection:None,pending_preview_projection_read:None,preview_retirement:None,returned_read_retirement:None });
         }
         let partition = self.partitions.get_mut(window_id).expect("initialized window config partition remains owned");
-        if !partition.store.owned_disposer_installed() {
-            store::install_unscheduled_catalog(&mut partition.store, O::build_store_owners()).map_err(|error| error.into_fault())?;
-        }
         Ok(partition)
     }
@@ -423,4 +523,8 @@
 
 impl<O: WindowConfigOwner> ErasedWindowConfigStoreOwner for TypedWindowConfigStoreOwner<O> {
+    fn borrow<'a>(&'a self, window_id: &'a str) -> Option<WindowConfigBorrow<'a>> {
+        let partition = self.partitions.get(window_id)?;
+        Some(WindowConfigBorrow { authority: partition.authority, window_id, window_kind_id: O::WINDOW_KIND_ID, generation: partition.store.generation(), revision: partition.store.content_revision_now(), state: partition.store.snapshot_ref() })
+    }
     fn capture<'a>(&'a mut self, window_id: &'a str) -> Pin<Box<dyn Future<Output = Result<WindowConfigAuthority, Fault>> + 'a>> {
         Box::pin(async move {
@@ -433,5 +537,6 @@
                 pending_snapshot:None,retired_snapshots:Default::default(),
                 snapshot: WindowConfigSnapshot {
-                    window_id: window_id.to_string(),
+                    authority: partition.authority,
+                    window_id: partition.address.as_ref().and_then(ControlledRetirement::original).expect("original Window address remains registered").clone(),
                     window_kind_id: O::WINDOW_KIND_ID,
                     generation: partition.store.generation(),
@@ -451,4 +556,5 @@
         })
     }
+    fn admit_catalog(&mut self,authority:&WindowConfigAuthority,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{let partition=self.partitions.get_mut(&authority.window_id).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original Window catalog requires its captured partition"))?;if partition.store.owned_disposer_installed(){return Ok(None);}partition.catalog_step(grant).map(|step|Some(step.progress()))}
     /// 🎟️ Admits one original typed frame and stops before the separately paid Store constructor.
     fn admit_begin(
@@ -475,5 +581,5 @@
     }
 
-    fn advance(&mut self, publication: &mut dyn ErasedWindowConfigPublication, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item_publication::ArtifactStoreOneItemAdvance, Fault> {
+    fn advance(&mut self, publication: &mut dyn ErasedWindowConfigPublication, body:usize, grant: store::os_store::one_item::ArtifactStoreOneItemGrant,receiving:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<store::os_store::one_item_publication::ArtifactStoreOneItemAdvance, Fault> {
         let publication = publication
             .as_any_mut()
@@ -489,5 +595,5 @@
             };
         }
-        let result=partition.store.advance_apply_batch(publication.publication.as_mut().expect("admitted original Store publication remains"),grant).map_err(|error|error.into_fault());
+        let result=partition.store.advance_apply_batch(publication.publication.as_mut().expect("admitted original Store publication remains"),body,grant,receiving).map_err(|error|error.into_fault());
         if publication.publication.as_ref().is_some_and(|owner|owner.phase()==store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase::Closing){publication.closing=true;}
         result
@@ -502,5 +608,5 @@
             let partition=self.partitions.get(&authority.window_id).ok_or_else(||Fault::new(FaultOrigin::Framework,FaultCode::new("window-config.partition"),"window config authority lost its exact partition"))?;
             let snapshot=partition.store.snapshot_read().map_err(|error|error.into_fault())?.into_erased();
-            authority.pending_snapshot=Some(WindowConfigSnapshot{window_id:String::new(),window_kind_id:O::WINDOW_KIND_ID,generation:partition.store.generation(),revision:partition.store.content_revision_now(),snapshot});
+            authority.pending_snapshot=Some(WindowConfigSnapshot{authority:partition.authority,window_id:semio_framework_value::SharedUtf8::default(),window_kind_id:O::WINDOW_KIND_ID,generation:partition.store.generation(),revision:partition.store.content_revision_now(),snapshot});
             return Ok(crate::window_mutation::WindowAuthorityRefreshStep::Pending(RetainedCloneProgress{copied_items:1,..Default::default()}));
         }
@@ -531,26 +637,15 @@
     fn snapshot(&self, window_id: &str) -> Option<WindowConfigSnapshot> {
         let partition = self.partitions.get(window_id)?;
-        Some(WindowConfigSnapshot { window_id: window_id.to_string(), window_kind_id: O::WINDOW_KIND_ID, generation: partition.store.generation(), revision: partition.store.content_revision_now(), snapshot: partition.store.snapshot_read().ok()?.into_erased() })
-    }
-
-    fn preview(&mut self, window_id: &str, mutations: &[&WindowConfigMutation]) -> Option<WindowConfigSnapshot> {
-        let partition=self.partitions.get_mut(window_id)?;
-        if partition.preview_retirement_pending(){return None;}
-        let mut projection=partition.store.derived_snapshot_head();
-        let mut reads=Vec::with_capacity(mutations.len());
-        let mut current=None;
-        for mutation in mutations.iter().filter_map(|mutation|mutation.mutation.as_any().downcast_ref::<O::Mutation>()){
-            let next=match partition.store.derive_provisional_snapshot(&projection,mutation){Ok(Some(next))=>next,_=>continue};
-            let read=match partition.store.retained_derived_read(&next){
-                Ok(read)=>read,
-                Err(_)=>{partition.pending_preview_projection=Some(next);partition.pending_preview_reads=Some(reads);partition.pending_preview_projection_read=current;return None;}
-            };
-            if let Some(previous)=current.replace(read){reads.push(previous);}
-            projection=next;
-        }
-        partition.pending_preview_reads=Some(reads);
-        let snapshot=current?;
-        Some(WindowConfigSnapshot{window_id:window_id.to_string(),window_kind_id:O::WINDOW_KIND_ID,generation:partition.store.generation(),revision:partition.store.content_revision_now(),snapshot})
-    }
+        Some(WindowConfigSnapshot { authority: partition.authority, window_id: partition.address.as_ref().and_then(ControlledRetirement::original).expect("original Window address remains registered").clone(), window_kind_id: O::WINDOW_KIND_ID, generation: partition.store.generation(), revision: partition.store.content_revision_now(), snapshot: partition.store.snapshot_read().ok()?.into_erased() })
+    }
+
+    fn preview_input_demands(&self,window:&str,epoch:u64,body:usize)->Result<RetirementDemand,ValueError>{let p=self.partitions.get(window).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original Window preview partition is absent"))?;if !p.store.owned_disposer_installed(){return p.catalog_demands(body);}if let Some(owner)=p.semantic_preview.as_ref(){if p.semantic_epoch!=epoch||owner.work_pending(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original Window preview waits for its previous source to close"));}return Ok(WindowSemanticPreview::<O>::input_demand());}let mut d=WindowSemanticPreview::<O>::birth_demand();d.copy_bytes+=size_of::<u64>();Ok(d)}
+    fn preview_input(&mut self,input:semio_framework_value::retained_clone::RetainedCloneRef<'_,WindowConfigMutation>,epoch:u64,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{let address=input.get().window_id();let demand=self.preview_input_demands(address,epoch,grant.maximum_copy_bytes)?;if !registry::fits(grant,demand){return Ok(Some(Default::default()));}let p=self.partitions.get_mut(address).unwrap();if !p.store.owned_disposer_installed(){return p.catalog_step(grant).map(|step|Some(step.progress()));}if p.semantic_preview.is_none(){let overhead=size_of::<u64>();let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-overhead,..grant};let Some((owner,mut receipt))=WindowSemanticPreview::begin(&p.store,child)?else{return Ok(Some(Default::default()));};p.semantic_preview=Some(owner);p.semantic_epoch=epoch;receipt.copied_bytes+=overhead;return Ok(Some(receipt));}p.semantic_preview.as_mut().unwrap().admit_input(input,grant)}
+    fn preview_leaf_ready(&self,window:&str,epoch:u64)->bool{self.partitions.get(window).is_some_and(|p|p.semantic_epoch==epoch&&p.semantic_preview.as_ref().is_some_and(WindowSemanticPreview::leaf_ready))}
+    fn preview_input_held(&self,window:&str,epoch:u64)->bool{self.partitions.get(window).is_some_and(|p|p.semantic_epoch==epoch&&p.semantic_preview.as_ref().is_some_and(WindowSemanticPreview::work_pending))}
+    fn preview_partition_count(&self)->usize{self.partitions.len()}
+    fn preview_partition_demands(&self,index:usize,epoch:u64,closing:bool,body:usize)->Result<RetirementDemand,ValueError>{let Some(p)=self.partitions.get_index(index)else{return Ok(Default::default());};if closing{p.preview_cancel_demand(body,false)}else{p.preview_finish_demands(epoch,body)}}
+    fn preview_partition_step(&mut self,index:usize,epoch:u64,closing:bool,grant:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),ValueError>{let Some(p)=self.partitions.get_index_mut(index)else{return Ok((true,Default::default()));};if closing{let pending=p.semantic_preview.is_some()||p.finish_preview_base.is_some()||p.finish_preview_read.is_some()||p.preview_retirement_pending();if !pending{return Ok((true,Default::default()));}let step=p.preview_cancel_step(grant,false)?;let pending=p.semantic_preview.is_some()||p.finish_preview_base.is_some()||p.finish_preview_read.is_some()||p.preview_retirement_pending();return Ok((!pending,step.progress()));}p.preview_finish_step(epoch,grant)}
+    fn preview_or<'a>(&'a self,committed:&'a WindowConfigSnapshot)->&'a WindowConfigSnapshot{self.partitions.get(committed.window_id()).and_then(|p|p.published_preview.as_ref()).filter(|preview|preview.generation()==committed.generation()).unwrap_or(committed)}
     fn preview_available(&self,window_id:&str)->bool{self.partitions.get(window_id).is_some_and(|partition|!partition.preview_retirement_pending())}
     fn can_retire_preview(&self,preview:&WindowConfigSnapshot)->bool{preview.window_kind_id==O::WINDOW_KIND_ID&&preview.snapshot.get::<O::State>().is_some()&&self.partitions.get(&preview.window_id).is_some_and(|partition|partition.pending_preview.is_none()&&partition.pending_preview_projection.is_none()&&partition.pending_preview_projection_read.is_none())}
@@ -561,5 +656,5 @@
     }
     fn preview_retirement_pending(&self)->bool{self.partitions.values().any(WindowConfigPartition::preview_retirement_pending)}
-    fn preview_retirement_demand(&self)->Result<RetirementDemand,ValueError>{self.partitions.values().find(|partition|partition.preview_retirement_pending()).map_or(Ok(Default::default()),WindowConfigPartition::preview_retirement_demand)}
+    fn preview_retirement_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.partitions.values().find(|partition|partition.preview_retirement_pending()).map_or(Ok(Default::default()),|partition|partition.preview_retirement_demand(body))}
     fn preview_retirement_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}self.partitions.values_mut().find(|partition|partition.preview_retirement_pending()).map_or(Ok(RetainedCloneStep::Complete(Default::default())),|partition|partition.preview_retirement_step(grant))}
 
@@ -578,6 +673,8 @@
         if let Some(owner) = self.partition_address_retirement.as_ref() { return controlled_owner_demands(owner, body); }
         if let Some(partition) = self.partitions.get_index(self.partition_close_cursor) {
-            if partition.preview_retirement_pending() { return partition.preview_retirement_demand(); }
+            if let Some(catalog)=partition.catalog.as_ref(){return if catalog.uninstalled_owners_terminal_is_empty(){Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Option<store::os_store::document_store_owners::DocumentStoreOwners<O::State,O::Mutation>>>(),depth:1,..Default::default()})}else{catalog.uninstalled_owners_demands(body)}}
+            if partition.preview_close_pending() { return partition.preview_close_demand(body); }
             if partition.returned_read_pending() { return partition.returned_read_demands(body); }
+            if let Some(address)=partition.address.as_ref(){return if address.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<semio_framework_value::SharedUtf8>>>(),depth:1,..Default::default()})}else{address.next_demand(body)};}
             return partition.disposer.as_ref().map_or(Ok(RetirementDemand { copy_bytes: size_of::<usize>(), depth: 1, ..Default::default() }), |disposer| {
                 if disposer.terminal_is_empty(&partition.store) { Ok(RetirementDemand { release_bytes: size_of_val(disposer.as_ref()), depth: 1, ..Default::default() }) }
@@ -600,6 +697,8 @@
         }
         if let Some(partition) = self.partitions.get_index_mut(self.partition_close_cursor) {
-            if partition.preview_retirement_pending() { return partition.preview_retirement_step(grant).map(|step| PluginLifecycleStep::retained(step, false)).map_err(ValueError::into_fault); }
+            if let Some(catalog)=partition.catalog.as_mut(){if catalog.uninstalled_owners_terminal_is_empty(){partition.catalog=None;return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}let step=catalog.close_uninstalled_owners_step(grant).map_err(ValueError::into_fault)?;return Ok(PluginLifecycleStep::retained(step,false));}
+            if partition.preview_close_pending() { return partition.preview_close_step(grant).map(|step| PluginLifecycleStep::retained(step, false)).map_err(ValueError::into_fault); }
             if partition.returned_read_pending() { return partition.returned_read_step(grant).map(|step| PluginLifecycleStep::retained(step, false)).map_err(ValueError::into_fault); }
+            if let Some(address)=partition.address.as_mut(){if address.terminal_is_empty(){partition.address=None;return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}return address.step(grant).map(|step|PluginLifecycleStep::retained(step,false)).map_err(ValueError::into_fault);}
             if let Some(disposer) = partition.disposer.as_mut() {
                 if disposer.terminal_is_empty(&partition.store) {
@@ -624,5 +723,5 @@
     }
 
-    fn direct_ingress_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:self.direct_ingress.next_copy_byte_demand()?,capacity_bytes:self.direct_ingress.next_capacity_byte_demand(body)?,release_bytes:self.direct_ingress.next_release_byte_demand()?,depth:self.direct_ingress.next_depth_demand()?})}
+    fn direct_ingress_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{Ok(self.direct_ingress.next_demand(body)?)}
     fn close_direct_ingress(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.direct_ingress.step(grant)}
     fn direct_ingress_terminal_is_empty(&self)->bool{self.direct_ingress.terminal_is_empty()}
@@ -672,5 +771,5 @@
 
 fn controlled_owner_demands<T: semio_framework_value::retirement::RetireOwned>(owner: &ControlledRetirement<T>, body: usize) -> Result<RetirementDemand, ValueError> {
-    Ok(RetirementDemand { copy_bytes: owner.next_copy_byte_demand()?, capacity_bytes: owner.next_capacity_byte_demand(body)?, release_bytes: owner.next_release_byte_demand()?, depth: owner.next_depth_demand()? })
+    Ok(owner.next_demand(body)?)
 }
 fn window_actor_demands(actor: &Option<protocol::ActorId>, retirement: &Option<ControlledRetirement<semio_framework_value::SharedUtf8>>, body: usize) -> Result<RetirementDemand, ValueError> {
@@ -692,4 +791,12 @@
 
 impl WindowConfigOwnerRegistry {
+    /// 🧪️ Opens a real declared fixture partition and returns its original issued authority.
+    #[cfg(any(test, feature = "artifact-app-testing"))]
+    pub async fn fixture_partition_authority(&mut self, kind: &str, id: &str) -> Result<WindowConfigAuthorityIdentity, Fault> {
+        let owner = self.owners.get_mut(kind).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.fixture-owner"), "fixture partition requires its declared original owner"))?;
+        let original = owner.capture(id).await?;
+        Ok(original.snapshot.authority)
+    }
+
     pub fn new(actor: protocol::ActorId) -> Self {
         static NEXT_LIFETIME: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
@@ -716,4 +823,9 @@
     pub fn snapshot(&self, window_kind_id: &str, window_id: &str) -> Option<WindowConfigSnapshot> {
         self.owners.get(window_kind_id)?.snapshot(window_id)
+    }
+
+    /// 👁️ Lends committed window state without a new snapshot lease or address allocation.
+    pub fn borrow<'a>(&'a self, window_kind_id: &str, window_id: &'a str) -> Option<WindowConfigBorrow<'a>> {
+        self.owners.get(window_kind_id)?.borrow(window_id)
     }
 
@@ -776,4 +888,5 @@
     }
 
+    pub(crate) fn admit_catalog(&mut self,authority:&WindowConfigAuthority,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{self.owners.get_mut(&authority.window_kind_id).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original Window catalog owner is absent"))?.admit_catalog(authority,grant)}
     #[doc(hidden)]
     pub fn admit_begin(
@@ -830,5 +943,5 @@
             }
             match self.advance_retained_load(&mut load, grant) {
-                WindowConfigPackLoadStep::Pending(_) => {}
+                WindowConfigPackLoadStep::Pending(_)|WindowConfigPackLoadStep::OwnershipProgress(_,_) => {}
                 WindowConfigPackLoadStep::Ready => {
                     if let WindowConfigPackLoadStep::Rejected(diagnostic) = self.commit_retained_load(&mut load) {
@@ -1012,2 +1125,6 @@
     }
 }
+
+#[cfg(test)]
+#[path="🏠️catalog/🧪️tests/🦀️.rs"]
+mod original_catalog_tests;
