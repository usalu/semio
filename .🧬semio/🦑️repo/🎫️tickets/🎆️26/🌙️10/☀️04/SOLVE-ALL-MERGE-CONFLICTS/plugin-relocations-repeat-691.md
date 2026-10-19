## 50 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/📨️dispatch/🦀️.rs
```diff
diff --git a/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/📨️dispatch/🦀️.rs b/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/📨️dispatch/🦀️.rs
index a32a24c688e..74c60190279 100644
--- a/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/📨️dispatch/🦀️.rs
+++ b/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/📨️dispatch/🦀️.rs
@@ -565,7 +565,7 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
                 if !outcome.is_applicable(protocol::MergePolicy::default()) {
                     return Err(MediaTransfers::<A>::transaction_fault(FaultOrigin::App, "transaction.member-rejected", format!("mutation outcome was rejected: {:?}", outcome.messages())));
                 }
-                running = protocol::apply_diff(outcome.diff(), &running).map_err(|error| MediaTransfers::<A>::transaction_fault(FaultOrigin::App, "transaction.member-rejected", error.to_string()))?;
+                running = protocol::apply_diff_cold(outcome.diff(), &running).map_err(|error| MediaTransfers::<A>::transaction_fault(FaultOrigin::App, "transaction.member-rejected", error.to_string()))?;
             }
             if !foreign.is_empty() {
                 let mut local_ops = Vec::with_capacity(artifact_mutations.len());

```

## 51 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🚪️ingress/🦀️.rs
```diff
diff --git a/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🚪️ingress/🦀️.rs b/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🚪️ingress/🦀️.rs
index 23c4add6c73..0c6e02029e6 100644
--- a/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🚪️ingress/🦀️.rs
+++ b/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🚪️ingress/🦀️.rs
@@ -16,12 +16,13 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
 
     /// 🔬️ Exercises real reserved slot admission without exporting private producer authority.
     #[cfg(test)]
-    pub(crate) fn test_reserved_emit_admission_case(&mut self, occupied: usize, meta: &ActionMeta) -> bool {
+    pub(crate) fn test_reserved_emit_admission_case(&mut self, occupied: usize, meta: &ActionMeta,cx:&mut semio_framework_job::StepContext<'_>) -> bool {
         let before = self.tools.typed_operation_reservations;
         for slot in 0..occupied { self.tools.typed_operation_reservations[slot] = Some((slot + 128) as u64); }
         let original = self.tools.typed_operation_reservations;
-        let admitted = match self.admit_reserved_emit_producer("cut", meta) {
-            Ok(permit) => {
+        let mut result = Ok(None);for _ in 0..128{result= self.admit_reserved_emit_producer("cut", meta,cx) ;if !matches!(result,Ok(None)){break}}
+        let admitted = match result {
+            Ok(Some(permit)) => {
                 let id = permit.operation.operation.0;
                 let slot = id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS;
                 assert_eq!(slot, occupied);
@@ -35,7 +36,8 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
                 assert_eq!(self.tools.typed_operation_reservations, original);
                 true
             }
-            Err(_) => { assert_eq!(self.tools.typed_operation_reservations, original); false }
+
+            Ok(None)|Err(_) => { assert_eq!(self.tools.typed_operation_reservations, original); false }
         };
         self.tools.typed_operation_reservations = before;
         admitted
@@ -43,21 +45,34 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
 
     /// 🧪️ Keeps cancelled worker completion tied to the actual original permit through handoff.
     #[cfg(test)]
-    pub(crate) async fn test_reserved_emit_cancelled_worker_handoff(&mut self, meta: &ActionMeta) -> bool {
-        let permit = self.admit_reserved_emit_producer("cut", meta).expect("original cut producer admission");
-        let id = permit.operation.operation;
-        permit.lease.cancel();
-        let job = ArtifactReservedToolJob::new(FrameworkCutJob::new(Vec::new(), 0));
-        match self.tools.run_framework_reserved_job(&self.mounted_policy, &mut self.stores, "cut", &[], 0, job, meta, None, Some(permit)).await {
-            Ok(permit) => { assert_eq!(permit.operation.operation, id); assert_eq!(self.tools.typed_operation_reservations[id.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS], Some(id.0)); self.tools.finish_reserved_emit_producer(permit); true }
-            Err(_) => false,
-        }
+    pub(crate) async fn test_reserved_emit_cancelled_worker_handoff(&mut self,meta:&ActionMeta)->bool{
+        fn now()->Option<u64>{Some(1)}
+        let grant=crate::app_testing::artifact_app_laws::fixture_mounted_policy().preparation;
+        let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(899),semio_framework_job::Generation(3),grant,now,u64::MAX,1_000_000);
+        let mut permit=None;
+        for _ in 0..128{caller.turn(grant,|cx|{permit=self.admit_reserved_emit_producer("cut",meta,cx)?;Ok(())}).unwrap();if permit.is_some(){break}}
+        let permit=permit.expect("original cut producer admission");let id=permit.operation.operation;permit.lease.cancel();
+        let retained=permit.operation.operation==id&&self.tools.typed_operation_reservations[id.0 as usize%ARTIFACT_LIVE_OUTPUT_SLOTS]==Some(id.0);
+        self.tools.finish_reserved_emit_producer(permit);caller.retire(grant);retained
+    }
+
+    /// 🧷️ Preserves the mounted original reserved source across refused worker birth.
+    #[cfg(test)]
+    pub(crate) async fn test_reserved_original_before_birth(&mut self,meta:&ActionMeta,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
+        let mut permit=None;for _ in 0..128{permit=self.admit_reserved_emit_producer("cut",meta,cx)?;if permit.is_some(){break}}
+        let permit=permit.expect("original fixture reserved permit");let operation=permit.operation;let id=operation.operation.0;
+        let job=ArtifactReservedToolJob::new(FrameworkCutJob::new(Vec::new(),0));let original=job.test_source_identity();let completion=ArtifactToolCompletion::<A>::new();
+        self.dispatch_reserved_original_producer("cut",&[],0,job,completion,meta,None,permit,cx).await?;
+        let mounted=self.tools.tool_operations.get_mut(id).expect("actual original reserved producer mounted");assert!(mounted.reserved_producer.is_some()&&!mounted.terminal_seen);assert!(mounted.session.is_none()&&mounted.session_rejected.is_none());let before=cx.retained_progress();
+        let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(Default::default(),|same|mounted.drive_worker_step(same)));result?;assert_eq!(cx.retained_progress(),before);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(mounted.session_rejected.is_none());assert_eq!(mounted.reserved_producer.as_ref().unwrap().test_source_identity(),original);
+        mounted.drive_worker_step(cx)?;let retained=mounted.session_rejected.is_some()&&mounted.session.is_none()&&mounted.stage==MountedTypedCommandFullOperationStage::Worker&&mounted.reserved_producer.as_ref().unwrap().test_source_identity()==original&&mounted.reserved_producer.as_ref().unwrap().test_bound_operation()==Some(operation);
+        mounted.user_cancel_requested=true;mounted.terminal_seen=true;mounted.cancellation_lease.as_ref().unwrap().cancel();Ok(retained)
     }
 
     /// 🧫️ Observes busy original completion retention without exposing producer authority.
     #[cfg(test)]
     pub(crate) async fn test_reserved_emit_busy_completion_owner(&mut self, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> bool {
-        let permit=self.admit_reserved_emit_producer("cut",meta).expect("fixture cut admission");
+        let  mut permit=None;for _ in 0..128{permit=self.admit_reserved_emit_producer("cut",meta,cx).expect("fixture cut admission");if permit.is_some(){break}}let permit=permit.expect("fixture original cut permit");
         let id=permit.operation.operation.0;
         let completion=ArtifactToolCompletion::<A>::new();
         let mut emit=Emit::default();emit.artifact_mutations=Vec::with_capacity(3);
@@ -309,14 +324,14 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
                 return Ok(());
             }
             if self.tools.tool_operations.get(operation_id).is_some_and(|operation|operation.granted_retirement_ready()&&operation.granted_retirement_pending()){
-                let grant=crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.close,cx.retained_grant());
+                let grant=semio_framework_plugin_ownership::bounded_ownership::original_plugin_turn_grant(self.mounted_policy.close,cx.retained_grant());
                 let demand=self.tools.tool_operations.get(operation_id).unwrap().granted_retirement_demands(grant.maximum_copy_bytes)?;let step=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,actual|{let step=self.tools.tool_operations.get_mut(operation_id).unwrap().granted_retirement_step(same,actual)?;Ok((PluginLifecycleStep::Progress(step.progress()),false))})?;let step=RetainedCloneStep::Progress(step.progress().unwrap_or_default());
                 if !step.progress().fits(grant){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original typed operation retirement exceeded its issuer grant").with_retained_progress(step.progress()).into());}
                 let progress=step.progress();if progress==RetainedCloneProgress::default(){return Ok(());}
                 let owner=self.tools.tool_operations.get_mut(operation_id).unwrap();assert!(owner.original_retirement_receipt.is_none(),"original mounted Close receipt cannot be overwritten before acknowledgement");owner.original_retirement_receipt=Some((grant,progress));return Ok(());
 
             }
-            let grant = crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.close, cx.retained_grant());
+            let grant = semio_framework_plugin_ownership::bounded_ownership::original_plugin_turn_grant(self.mounted_policy.close, cx.retained_grant());
             let demand = self.tools.typed_operation_retirement_demands(&self.child_composition, operation_id, grant.maximum_copy_bytes)?;
             if !plugin_grant_funds(demand, grant) {
                 return Ok(());

```

## 52 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🤝️group/🦀️.rs
```diff
diff --git a/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🤝️group/🦀️.rs b/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🤝️group/🦀️.rs
index a7e4463e2eb..9736addcc11 100644
--- a/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🤝️group/🦀️.rs
+++ b/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🤝️group/🦀️.rs
@@ -25,14 +25,14 @@ impl<A:ArtifactApp,M:SpaceMember+MemberFactory+'static> VcsArtifactApp<A,M>{
         let reservation=self.tools.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS];
         if !self.tools.tool_operations.can_insert(operation_id)||reservation.is_some_and(|owner|owner!=operation_id){return Err(plugin_sdk_fault("original emission lost its exact pre-admitted publication slot"));}
         let mounted=MountedTypedCommandFullOperation::<A>{
-            verb:verb.into(),meta:meta.clone(),operation,source_identity:(cx.operation(),cx.generation()),canonical_revision:revision,artifact_generation:generation.0,
+            verb:verb.into(),meta:meta.clone(),operation,source_identity:semio_framework_job::OriginalSourceBinding::borrow(cx),canonical_revision:revision,artifact_generation:generation.0,
             config_generation:self.stores.config_store.generation_now(),draft_generation:self.stores.draft_store.generation_now(),presence_generation:0,transient_generation:0,
             window_config_authority:None,window_transient_authority:None,publication_lanes:&[],
-            session:None,session_rejected:None,reserved_producer,completion,completion_retirement:None,publication_retirement:None,output_retirement:None,raw_input:None,output_chunks:None,cancellation_lease:lease,identity,identity_progress:semio_framework_value::native_encoding::NativeEncodeProgress{completed:0,total:0,owned_bytes:0},terminal_seen:true,worker_semantic_pending:false,worker_outcome_pending:false,worker_fault_capture:None,pending_publication_outcome:PendingPublicationOutcome::new(),pending_window_config_receipt:None,cancellation_retirement:None,original_retirement_receipt:None,
+            session:None,session_rejected:None,producer_source:None,producer_request:None,producer_request_retirement:None,reserved_producer,completion,completion_retirement:None,publication_retirement:None,output_retirement:None,raw_input:None,output_chunks:None,cancellation_lease:lease,identity,identity_progress:semio_framework_value::native_encoding::NativeEncodeProgress{completed:0,total:0,owned_bytes:0},terminal_seen:true,worker_semantic_pending:false,worker_outcome_pending:false,worker_fault_capture:None,pending_publication_outcome:PendingPublicationOutcome::new(),pending_window_config_receipt:None,cancellation_retirement:None,original_retirement_receipt:None,
             publication,publication_ownership_progress:None,actor_capture:None,worker_resume_pending:false,pending_artifact_publication:None,pending_child_publication:None,
             owned_child_group:None,owned_child_committed:false,owned_child_result_pending:false,
             captured_child_content:Some(std::sync::Arc::new(ChildContentView::clone(&*self.child_composition.child_content_root))),captured_child_content_generation:self.child_composition.child_content_generation,
-            result_page:None,result_page_presented:false,result_sequence:0,publication_progress:0,publication_checkpoint:None,publication_attempt:0,
+            result_page:None, diagnostics:None,result_page_presented:false,result_sequence:0,publication_progress:0,publication_checkpoint:None,publication_attempt:0,
             ui_pending:true,progress:None,progress_pending:false,user_cancel_requested:false,published_artifact:false,published_config:false,published_window_config:false,
             command_logged:false,interaction_revalidated:false,retained_close_fault: None,retained_close_fault_retirement:None,retained_close_fault_refusal:None, terminal_fault:None,stage:MountedTypedCommandFullOperationStage::Publishing,
         };

```

## 53 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/🦀️.rs
```diff
diff --git a/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/🦀️.rs b/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/🦀️.rs
index c4d8bf35288..93513bfacd9 100644
--- a/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/🦀️.rs
+++ b/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/🦀️.rs
@@ -351,14 +351,16 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
     pub(super) async fn dispatch_reserved_original_producer(&mut self,verb:&str,raw:&[u8],decoded_items:usize,job:ArtifactReservedToolJob,completion:ArtifactToolCompletion<A>,meta:&ActionMeta,admission:Option<semio_framework::ToolWireAdmission>,mut permit:FrameworkReservedCommitPermit, cx:&mut semio_framework_job::StepContext<'_>)->Result<InvocationResult,Fault>{
         let id=permit.operation.operation.0;
         let started=self.mount_original_reserved_owners(verb,None,Some(job),Some(completion),meta,Some((permit.operation,permit.lease,permit.identity.take().ok_or_else(||plugin_sdk_fault("original reserved producer lost its authoring receipt"))?)), cx).await?;
-        let proof=self.tools.qualified_tool_proof(&self.stores, verb)?;
-        let key=proof.key();
-        let admitted=match admission{Some(admitted)=>admitted,None=>self.tools.tool_jobs.admit_exact_wire(key.controller_id.clone(),key.tool_id.clone(),proof.schema_id(),raw).map_err(|error|plugin_sdk_fault(error.to_string()))?};
-        if decoded_items>proof.contract().max_decoded_items||!proof.admits::<A>(&admitted){return Err(plugin_sdk_fault("original reserved producer failed its exact owner/factory/schema admission"));}
+        self.tools.tool_operations.get_mut(id).expect("original producer owns its admitted slot").producer_source=Some(semio_framework::action_bus::OriginalToolSourceBinding::borrow(cx));
+        let validation=(||->Result<(),Fault>{
+            let proof=self.tools.qualified_tool_proof(&self.stores,verb)?;
+            let key=proof.key();
+            let admitted=match admission{Some(admitted)=>admitted,None=>self.tools.tool_jobs.admit_exact_wire(key.controller_id.clone(),key.tool_id.clone(),proof.schema_id(),raw).map_err(|error|plugin_sdk_fault(error.to_string()))?};
+            if decoded_items>proof.contract().max_decoded_items||!proof.admits::<A>(&admitted){return Err(plugin_sdk_fault("original reserved producer failed its exact owner/factory/schema admission"));}
+            Ok(())
+        })();
         let mounted=self.tools.tool_operations.get_mut(id).expect("original producer owns its admitted slot");
-        let job=mounted.reserved_producer.as_ref().expect("original reserved producer remains retained").clone();
-        let dispatched=self.tools.tool_jobs.dispatch(semio_framework::ToolOperationSpec::new(key.controller_id,key.tool_id,proof.schema_id(),job,mounted.operation)).map_err(|error|plugin_sdk_fault(error.to_string()))?;
-        mounted.session_rejected=Some(semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource::pending(dispatched.job,mounted.source_identity));
+        if let Err(fault)=validation{mounted.user_cancel_requested=true;mounted.terminal_seen=true;mounted.stage=MountedTypedCommandFullOperationStage::Publishing;mounted.cancellation_lease.as_ref().expect("original reserved source cancellation lease").cancel();return Err(fault)}
         mounted.terminal_seen=false;
         mounted.stage=MountedTypedCommandFullOperationStage::Worker;
         Ok(started)
@@ -407,12 +409,12 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
     }
 
     pub(crate) async fn dispatch_action(&mut self, action: &str, source_args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>, semio_framework_job::OriginalTurnFailure> {
-        let Some(slot)=self.ingress.stage_action_source(action, source_args, original_fault, cx)?else{return Ok(None)};
-        let Some(mut source)=self.ingress.action_ingress_sources.get_mut(slot).unwrap().loan_source(cx)?else{return Ok(None)};
+        let Some(slot)=self.tool_machines.ingress.stage_action_source(action, source_args, original_fault, cx)?else{return Ok(None)};
+        let Some(mut source)=self.tool_machines.ingress.action_ingress_sources.get_mut(slot).unwrap().loan_source(cx)?else{return Ok(None)};
         let result=self.dispatch_action_received(action,&mut source,meta,identity,cx).await;
         let terminal=!matches!(&result,Ok(None));
-        self.ingress.action_ingress_sources.get_mut(slot).unwrap().return_source(std::mem::ManuallyDrop::into_inner(source),terminal);
-        if terminal{self.ingress.pending_action_ingress_identity=None;}
+        self.tool_machines.ingress.action_ingress_sources.get_mut(slot).unwrap().return_source(std::mem::ManuallyDrop::into_inner(source),terminal);
+        if terminal{self.tool_machines.ingress.pending_action_ingress_identity=None;}
         result
     }
 
@@ -526,8 +528,8 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
         };
         validate_ui_dispatch_classification(&owner, command_id, definition.semantics.execution.interactive_job)?;
         let kind = definition.kind;
-        let Some(slot)=self.ingress.stage_action_source(command_id, source_args, original_fault, cx)?else{return Ok(None)};
-        let Some(mut source)=self.ingress.action_ingress_sources.get_mut(slot).unwrap().loan_source(cx)?else{return Ok(None)};
+        let Some(slot)=self.tool_machines.ingress.stage_action_source(command_id, source_args, original_fault, cx)?else{return Ok(None)};
+        let Some(mut source)=self.tool_machines.ingress.action_ingress_sources.get_mut(slot).unwrap().loan_source(cx)?else{return Ok(None)};
         let result=async {
         let args = source.args.as_ref();
         match self.ingest.import_staging.admit_args(args).map_err(|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), refusal.message()))? {
@@ -559,8 +561,8 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
         result.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from)
         }.await;
         let terminal=!matches!(&result,Ok(None));
-        self.ingress.action_ingress_sources.get_mut(slot).unwrap().return_source(std::mem::ManuallyDrop::into_inner(source),terminal);
-        if terminal{self.ingress.pending_action_ingress_identity=None;}
+        self.tool_machines.ingress.action_ingress_sources.get_mut(slot).unwrap().return_source(std::mem::ManuallyDrop::into_inner(source),terminal);
+        if terminal{self.tool_machines.ingress.pending_action_ingress_identity=None;}
         result
     }
 
@@ -609,7 +611,7 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
     ///
     /// 🧭️ Why this cannot be `AppCommand::Command`. The shell's dispatch lane APPLIES: it runs
     /// `handle_action_invocation` → `dispatch_typed_command_inner`, which mounts a typed
-    /// operation whose publication ladder commits into `self.store` and mints an undo group. An
+    /// operation whose publication ladder commits into `self.stores.store` and mints an undo group. An
     /// agent's `action_prepare` must not do that — its own protocol commits separately
     /// (`TransactionPrepare{ops}` → `TransactionCommit`), and `RoutingArtifactChannel` caches ONE
     /// guest instance per plugin, so a prepare that applied would leave the commit applying the
@@ -662,10 +664,10 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
         validate_ui_dispatch_classification("action", &address.action_id, definition.semantics.execution.interactive_job)?;
         if self.tools.agent_lane_preview.as_ref().is_none_or(|state|state.starting){
             if !self.admit_agent_lane_preview_key(meta,cx)?{return Ok(None)}
-            if !self.ingress.action_source_capture_ready(&address.action_id, cx){return Ok(None)}
-            let mut original_args=if self.ingress.action_source_pending(){None}else{let mut arguments=invocation.arguments.clone();arguments.insert("windowId".into(),DslValue::String(address.window_instance_id.clone()));self.fill_agent_revisions(address,&mut arguments).await?;
+            if !self.tool_machines.ingress.action_source_capture_ready(&address.action_id, cx){return Ok(None)}
+            let mut original_args=if self.tool_machines.ingress.action_source_pending(){None}else{let mut arguments=invocation.arguments.clone();arguments.insert("windowId".into(),DslValue::String(address.window_instance_id.clone()));self.fill_agent_revisions(address,&mut arguments).await?;
             Some(DslValue::Object(arguments.into_iter().collect()))};
-            let Some(command)=self.ingress.receive_action_command(&address.action_id, &mut original_args, &mut None, cx).await?else{return Ok(None)};
+            let Some(command)=self.tool_machines.ingress.receive_action_command(&address.action_id, &mut original_args, &mut None, cx).await?else{return Ok(None)};
             let wire=<A::Command as protocol::OpBinary>::encode_op(&command).map_err(|source|source.into_fault())?;
             let admission=self.tools.admit_command_wire(&self.stores, &address.action_id, &wire, 1).await?;self.media.require_complete_tool_operation_pipeline(&admission)?;
             self.begin_agent_lane_preview(Box::new(command),admission,&ActionMeta{view_state:Some(view),..meta.clone()},cx).await?;
@@ -755,7 +757,7 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
             let Some(caller)=cx.original_source_parent()?.admit_loan(cx)?else{return Ok(false)};
             cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;
             let config=semio_framework_job_worker_authority::BatchDriveConfig{retained:grant,site:"plugin_agent_lane_preview",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:0,step_budget_us:0};
-            self.tools.agent_lane_preview=Some(crate::tool_jobs::AgentLanePreview{key:Some(key),caller:std::mem::ManuallyDrop::new(Some(caller)),starting:true,producer:None,children:None,completion:None,completion_close:None,lease:None,lease_close:None,config,maximum_bytes:0,steps:0,publication:None,complete:false,closing:false,publication_close:None});
+            self.tools.agent_lane_preview=Some(crate::tool_jobs::AgentLanePreview{key:Some(key),caller:std::mem::ManuallyDrop::new(Some(caller)),starting:true,producer_source:None,producer_request:None,producer_request_retirement:None,producer:None,children:None,completion:None,completion_close:None,lease:None,lease_close:None,config,maximum_bytes:0,steps:0,publication:None,complete:false,closing:false,publication_close:None});
         }
         let state=self.tools.agent_lane_preview.as_mut().unwrap();
         if state.closing{return Ok(false)}
@@ -800,10 +802,12 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
         );
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
             tool_id: key.tool_id,
@@ -813,7 +817,7 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
             app_instance_id: meta.instance_id,
             parent_document_id: self.stores.store.envelope().id.clone(),
             canonical_base_revision: roots.canonical_base_revision,
-            authoring_seed: self.stores.authoring_seed(&meta.actor),
+            authoring_seed: self.authoring_seed(&meta.actor),
             snapshot: roots.snapshot,
             snapshot_read: self.stores.store.snapshot_read().map_err(FaultFrom::into_fault)?,
             config: roots.config,
@@ -825,14 +829,9 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
             instance_operation_owner: self.tools.instance_operation_owner.clone(),
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
 

```

## 54 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/🛠️tool-machine/🦀️.rs
```diff
diff --git a/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/🛠️tool-machine/🦀️.rs b/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/🛠️tool-machine/🦀️.rs
index 1bb51f10a90..a81b078f360 100644
--- a/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/🛠️tool-machine/🦀️.rs
+++ b/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/🛠️tool-machine/🦀️.rs
@@ -177,11 +177,11 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
         let frozen=InstanceExtensions::freezes_document(self);
         let Some(original)=self.tool_machines.gesture_operations.iter_mut().flatten().find(|original|original.operation==operation&&!original.settled)else{return false};
         original.settled=true;
-        original.decision=original.slot.take();
+        if original.decision.is_none(){original.decision=original.slot.take();}
         let Some(decision)=original.decision.as_mut()else{return false};
         if publishes&&!frozen&&self.tool_machines.gestures.open(original.slot.window())==decision.admitted.as_ref(){
             if let Some(press)=decision.closing.take(){self.tool_machines.gestures.close(original.slot.window(),press);}
-            if let Some(decided)=decision.decided.take(){self.tool_machines.gestures.settle(original.slot.window(),decided);self.follow_tool_machines(true);}
+            if let Some(decided)=decision.decided.take(){if let Err(decided)=self.tool_machines.gestures.settle(original.slot.window(),decided){decision.decided=Some(decided);original.settled=false;self.tool_machines.refold_owed=true;return false;}self.follow_tool_machines(true);}
         }
         true
     }
@@ -195,7 +195,7 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
     fn gesture_retirement_pending(&self)->bool{self.tool_machines.gesture_retirement.is_some()||self.tool_machines.gesture_operations.iter().flatten().any(|original|original.settled)}
 
     fn gesture_retirement_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
-        if let Some(owner)=self.tool_machines.gesture_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<GestureOperation<A::Mutation>>>>(),depth:1,..Default::default()})}else{Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})};}
+        if let Some(owner)=self.tool_machines.gesture_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<GestureOperation<A::Mutation>>>>(),depth:1,..Default::default()})}else{Ok(owner.next_demand(body)?)};}
         Ok(RetirementDemand{copy_bytes:size_of::<GestureOperation<A::Mutation>>(),depth:1,..Default::default()})
     }
 
@@ -293,7 +293,7 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
         if self.tool_machines.pending_overlay_aliases.is_some(){return Ok(ToolOriginalAliasBatch::<A::Snapshot>::constructor_demand());}
         if let Some(owner)=self.tool_machines.config_alias_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<ToolOriginalAliasBatch<A::Config>>>(),depth:1,..Default::default()})}else{owner.next_demand(ArtifactStore::<A::Config,A::ConfigMutation>::snapshot_alias_retirement_birth_bytes())};}
         if self.tool_machines.pending_config_aliases.is_some(){return Ok(ToolOriginalAliasBatch::<A::Config>::constructor_demand());}
-        self.stores.window_config_store.preview_retirement_demand()
+        self.stores.window_config_store.preview_retirement_demand(body)
     }
 
     /// 🎟️ Original refold Vec/Arc owners wait for this actual full grant; no cold-drain fallback is admitted.
@@ -326,6 +326,8 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
     /// of the press the window already closed stays silent on every lane. One ledger step decides every lane, so a refused
     /// input leaves the press, its overlays and its held config exactly as they were until a retry or a host abort decides.
     /// Answers whether `emit` publishes the press.
+    pub(crate) fn tool_source_busy(&self,operation:u64)->bool{self.tool_machines.operation_tag(operation).is_some_and(|tag|match tag{ToolTag::Scrub(tag)=>self.tool_machines.presses.retained_read_pending(&tag.window),ToolTag::Typing(_)=>false})}
+
     pub(crate) fn settle_press<D>(&mut self, tag: &ScrubTag, base: &[u8], actor: &str, emit: &mut Emit<A::Mutation, A::ConfigMutation, D>) -> Result<bool, Fault> {
         let base: String = base.iter().map(|byte| format!("{byte:02x}")).collect();
         let leaves = std::mem::take(&mut emit.artifact_mutations)
@@ -335,12 +337,13 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
             .chain(std::mem::take(&mut emit.config_mutations).into_iter().map(PressLeaf::Config))
             .chain(std::mem::take(&mut emit.window_config_mutations).into_iter().map(PressLeaf::WindowConfig))
             .collect();
-        emit.transaction = None;
+        let transaction=emit.transaction.take();
+        let mut input=Some(tag.phase.clone().input(leaves));
         let step = self
             .tool_machines
             .presses
-            .send(&tag.window, &tag.tool, &ActorId(actor.into()), &base, tag.phase.clone().input(leaves), semio_framework_tool_machine::authoring_clock(0))
-            .map_err(|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), format!("continuous control {:?} refused its press", tag.tool)))?;
+            .send(&tag.window, &tag.tool, &ActorId(actor.into()), &base, &mut input, semio_framework_tool_machine::authoring_clock(0));
+        let step=match step{Ok(step)=>step,Err(refusal)=>{if let Some(semio_framework_tool_machine::ScrubInput::Tick{leaves,..}|semio_framework_tool_machine::ScrubInput::Commit{leaves,..})=input.take(){for leaf in leaves{match leaf{PressLeaf::Member(leaf)=>emit.artifact_mutations.push(leaf),PressLeaf::Child(leaf)=>emit.child_emits.push(leaf),PressLeaf::Config(leaf)=>emit.config_mutations.push(leaf),PressLeaf::WindowConfig(leaf)=>emit.window_config_mutations.push(leaf)}}emit.transaction=transaction;}return Err(Fault::new(FaultOrigin::Framework,FaultCode::new(refusal.code()),"original continuous-control input waits for its source owners"));}};
         let ToolStep::Committed(transaction, leaves) = step else { return Ok(false) };
         for leaf in leaves {
             match leaf {

```

## 55 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/🦀️.rs
```diff
diff --git a/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/🦀️.rs b/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/🦀️.rs
index e6b24c996b9..cea0df58681 100644
--- a/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/🦀️.rs
+++ b/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/🦀️.rs
@@ -69,7 +69,7 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
         self.tools.typed_publication_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
         if self.tools.tool_operations.get(operation_id).is_some_and(|mounted|mounted.cancellation_lease.is_none()&&mounted.cancellation_retirement.is_none()&&mounted.stage==MountedTypedCommandFullOperationStage::Publishing){
             let mounted=self.tools.tool_operations.get_mut(operation_id).unwrap();
-            if mounted.source_identity!=(cx.operation(),cx.generation()){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original emission lease requires its mounted receiving caller").into())}
+            if !mounted.source_identity.matches(cx){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original emission lease requires its mounted receiving caller").into())}
             let key=ToolOperationKey{app_instance_id:mounted.meta.instance_id,document:ArtifactDocumentAuthority(mounted.meta.instance_id),operation_id:mounted.operation.operation,base_revision:mounted.operation.base_revision,generation:mounted.operation.generation};
             mounted.cancellation_lease=self.tools.tool_cancellations.begin_keyed(&key,cx)?;
             if let Some(lease)=mounted.cancellation_lease.as_ref(){
@@ -165,7 +165,7 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
         let outcome = self.publish_mounted_typed_operation_pump(mounted, identity, retained, &mut spent,cx).await;
         if spent != RetainedCloneProgress::default() {
             assert!(mounted.publication_ownership_progress.is_none(), "typed preparation cannot overwrite an unreturned original receipt");
-            mounted.publication_ownership_progress = Some((crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.preparation, retained), spent));
+            mounted.publication_ownership_progress = Some((semio_framework_plugin_ownership::bounded_ownership::original_plugin_turn_grant(self.mounted_policy.preparation, retained), spent));
         }
         outcome
     }
@@ -173,7 +173,7 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
     /// 🏃️ Spends ONE host turn's grant on consecutive publication units: each unit receives what the turn still owns, its receipt is summed into `spent`, and the pump hands the turn back as soon as the grant is exhausted — so a grant of one item still publishes one unit per turn, and a larger grant batches exactly as many units as it pays for.
     async fn publish_mounted_typed_operation_pump(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, retained:RetainedCloneGrant, spent: &mut RetainedCloneProgress,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(),crate::PluginLifecycleFailure> {
         let started_us=cx.now_us();
-        let turn = crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.preparation, retained);
+        let turn = semio_framework_plugin_ownership::bounded_ownership::original_plugin_turn_grant(self.mounted_policy.preparation, retained);
         for _ in 0..TYPED_OPERATION_PUBLICATION_PUMPS {
             if cx.should_yield(){return Ok(())}
             let before=cx.retained_progress();let fuel=cx.fuel_remaining();
@@ -191,7 +191,7 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
                 self.publish_mounted_typed_inline_interaction_unit(mounted, identity).await?;
             } else {
                 let before = (mounted.publication_progress, mounted.publication.is_some(), mounted.pending_artifact_publication.is_some(), mounted.stage);
-                self.publish_mounted_typed_operation_unit(mounted,crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.preparation,semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_grant(cx,crate::plugin_runtime::remaining_plugin_turn_grant(turn,*spent))))?;
+                self.publish_mounted_typed_operation_unit(mounted,semio_framework_plugin_ownership::bounded_ownership::original_plugin_turn_grant(self.mounted_policy.preparation,semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_grant(cx,crate::plugin_runtime::remaining_plugin_turn_grant(turn,*spent))))?;
                 let receipt = mounted.publication_ownership_progress.take();
                 if let Some((_, unit)) = receipt {
                     cx.consume_retained(unit)?;
@@ -375,7 +375,8 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
                 window_transient_authority: None,
                 publication_lanes: &[],
                 session: None,
-                session_rejected: None,
+                session_rejected: None,producer_source:None,producer_request:None,producer_request_retirement:None,
+                producer_source:None,producer_request:None,producer_request_retirement:None,
                 reserved_producer: None,
                 completion: None,
                 completion_retirement:None,
@@ -784,7 +785,7 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
                             if !outcome.is_applicable(protocol::MergePolicy::default()) {
                                 return Err(MediaTransfers::<A>::transaction_fault(FaultOrigin::App, "transaction.member-rejected", format!("mutation outcome was rejected: {:?}", outcome.messages())));
                             }
-                            running = protocol::apply_diff(outcome.diff(), &running).map_err(|error| MediaTransfers::<A>::transaction_fault(FaultOrigin::App, "transaction.member-rejected", error.to_string()))?;
+                            running = protocol::apply_diff_cold(outcome.diff(), &running).map_err(|error| MediaTransfers::<A>::transaction_fault(FaultOrigin::App, "transaction.member-rejected", error.to_string()))?;
                         }
                         if !foreign.is_empty() {
                             let mut local_ops = Vec::with_capacity(emit.artifact_mutations.len());
@@ -1004,7 +1005,7 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
                         admission: Some(admission),
                         meta: meta.clone(),
                         operation,
-                        source_identity:(cx.operation(),cx.generation()),
+                        source_identity:semio_framework_job::OriginalSourceBinding::borrow(cx),
                         revision,
                         lease: None,
                         key_copy: None,
@@ -1098,6 +1099,7 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
      cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
         assert!(self.tools.tool_operations.can_insert(operation_id.0), "typed worker handoff retains its exact empty operation slot");
         assert_eq!(self.tools.typed_operation_reservations[operation_id.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS], Some(operation_id.0), "worker handoff owns its exact pending reservation");
+        if !matches!(&admission.proof,QualifiedToolProof::AppOwned(_)){return Err(plugin_sdk_fault("typed worker handoff requires its exact registered application producer"));}
         let verb = admission.verb.clone();
         let TypedCommandRoots {
             snapshot,
@@ -1133,10 +1135,8 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
         let identity = semio_framework_value::NativeEncodeControl::new(admission.proof.contract().max_output_bytes, &mut identity_observer).pause().map_err(|error| plugin_sdk_fault(error.to_string()))?;
         drop(identity_observer);
         completion.install_authoring_identity(identity, &cancellation_lease)?;
-        let retained_wire_admission = admission.wire_admission.clone();
         let app_owned_retained_route = matches!(&admission.proof, QualifiedToolProof::AppOwned(_));
         let mut retained_wire = Some(admission.raw_wire);
-        let raw_input = if app_owned_retained_route { ArtifactToolRawInput::transferred_to_factory() } else { ArtifactToolRawInput::new(retained_wire.take().expect("generic retained wire owner remains mounted")) };
         let output_chunks = ArtifactOutputChunks::new(admission.proof.contract().max_output_bytes);
         let mut request_snapshot_read = if app_owned_retained_route { Some(self.stores.store.snapshot_read().map_err(FaultFrom::into_fault)?) } else { None };
         let mut request_window_config = if app_owned_retained_route { window_config_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())? } else { None };
@@ -1165,41 +1165,12 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
             .with_gesture(gesture),
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
                 tool_id: qualified_key.tool_id,
@@ -1209,7 +1180,7 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
                 app_instance_id: meta.instance_id,
                 parent_document_id,
                 canonical_base_revision,
-                authoring_seed: self.stores.authoring_seed(&meta.actor),
+                authoring_seed: self.authoring_seed(&meta.actor),
                 snapshot,
                 snapshot_read: request_snapshot_read.take().expect("app-owned route retains its request snapshot read"),
                 config,
@@ -1221,28 +1192,15 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
                 instance_operation_owner: self.tools.instance_operation_owner.clone(),
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
             MountedTypedCommandFullOperation {
                 verb: verb.clone(),
                 meta: meta.clone(),
                 operation,
-                source_identity:(cx.operation(),cx.generation()),
+                source_identity:semio_framework_job::OriginalSourceBinding::borrow(cx),
                 canonical_revision: canonical_base_revision,
                 artifact_generation: generation.0,
                 config_generation,
@@ -1254,11 +1212,14 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
                 publication_lanes,
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
                 identity: None,
@@ -1300,7 +1261,7 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
             },
         );
         self.tools.typed_operation_reservations[operation_id.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = None;
-        let mut started = ToolOperations::<A>::empty_result(&verb, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await;
+        let mut started = Self::empty_result(&verb, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await;
         started.output = DslValue::Object(vec![("operationId".into(), DslValue::String(operation_id.0.to_string())), ("generation".into(), DslValue::String(generation.0.to_string()))]);
         Ok(started)
     }
@@ -1345,24 +1306,37 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
     /// the config store — real work for `AppCommand::ConfigCommand` (replaces the deleted
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
-        let proof=self.tools.qualified_tool_proof(&self.stores, "configuration-binary")?;let key=proof.key();
+        if self.pending.pending_configuration_permit.is_some()&&self.pending.pending_configuration_command.is_some(){return self.finish_configuration_command(meta,identity,cx).await}
+        let proof=self.tools.qualified_tool_proof(&self.stores,"configuration-binary")?;let key=proof.key();
         let admission=match self.tools.tool_jobs.admit_exact_wire(key.controller_id,key.tool_id,proof.schema_id(),command_bytes){Ok(admission)=>admission,Err(source)=>{*self.pending.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Admission(source));self.pending.pending_configuration_identity=Some((cx.operation(),cx.generation()));let _=self.pending.receive_configuration_failure(cx)?;return Ok(None)}};
-        if self.pending.pending_configuration_permit.is_none(){let Some(authority)=self.admit_reserved_emit_producer("configuration-binary",meta,cx)?else{return Ok(None)};self.pending.pending_configuration_permit=Some(authority);self.pending.pending_configuration_identity=Some((cx.operation(),cx.generation()));return Ok(None)}
+        if !matches!(proof,QualifiedToolProof::FrameworkOwned(_)|QualifiedToolProof::AppOwned(_))||proof.contract().max_decoded_items<1||!proof.admits::<A>(&admission){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"configuration producer failed its exact owner/factory/schema/contract admission").into())}
+        if self.pending.pending_configuration_permit.is_none(){let Some(authority)=self.admit_reserved_emit_producer("configuration-binary",meta,cx)?else{return Ok(None)};self.pending.pending_configuration_permit=Some(authority);self.pending.pending_configuration_source=Some(semio_framework::action_bus::OriginalToolSourceBinding::borrow(cx));self.pending.pending_configuration_identity=Some((cx.operation(),cx.generation()));return Ok(None)}
         if self.pending.pending_configuration_identity!=Some((cx.operation(),cx.generation())){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"configuration admission requires its unchanged original receiving caller").into())}
         type Completion<M>=std::sync::Mutex<Option<Result<ArtifactCommand<M>,semio_framework_job::OriginalTurnFailure>>>;
         let bytes=semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<Completion<A::ConfigMutation>>();let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<bytes{return Ok(None)}
         let decoded=std::sync::Arc::new(std::sync::Mutex::new(None));cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()}).expect("exact original configuration completion preflight remains exclusive");cx.consume_fuel(1);
         self.pending.pending_configuration_identity=Some((cx.operation(),cx.generation()));
         self.pending.pending_configuration_completion=Some(crate::reserved_jobs::ConfigurationDecodeRetirement{original:ManuallyDrop::new(Some(decoded.clone())),payload:None});
-        let job=FrameworkConfigurationBinaryJob::<A>{raw:std::mem::take(command_bytes),cursor:0,stage:FrameworkConfigurationBinaryStage::Envelope,operation:None,decoded:ManuallyDrop::new(Some(decoded)),decoded_close:None,receiving:ManuallyDrop::new(None),native:semio_framework_job::OriginalNativeDecodeSource::empty(semio_framework_job::OriginalNativeDecodePolicy{maximum_owned_bytes:crate::reserved_jobs::FRAMEWORK_CONFIGURATION_NATIVE_OWNED_BYTES}),failure:ManuallyDrop::new(None),state:[0;8],pending:None,outcomes:crate::tool_contract::ReservedJobOutcomes::new(),closing:false};
-        let authority=self.pending.pending_configuration_permit.take().unwrap();
-        let permit=match self.tools.run_framework_reserved_job(&self.mounted_policy, &mut self.stores, "configuration-binary", &[], 1, job, meta, Some(admission), authority, cx).await{Ok(permit)=>permit,Err((source,permit))=>{self.pending.pending_configuration_permit=Some(permit);*self.pending.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Original(source));let _=self.pending.receive_configuration_failure(cx)?;return Ok(None)}};
-        self.pending.pending_configuration_permit=Some(permit);
-        let command=self.pending.pending_configuration_completion.as_ref().unwrap().original.as_ref().unwrap().lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().ok_or_else(||Fault::new(FaultOrigin::Framework,FaultCode::new("interactive-job.missing-output"),"configuration binary job completed without a decoded command"))?;
-        match command{Ok(command)=>*self.pending.pending_configuration_command=Some(command),Err(source)=>{*self.pending.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Original(source));let _=self.pending.receive_configuration_failure(cx)?;return Ok(None)}}
-        self.finish_configuration_command(meta,identity,cx).await
+        let job=FrameworkConfigurationBinaryJob::<A>{raw:std::mem::take(command_bytes),cursor:0,stage:FrameworkConfigurationBinaryStage::Envelope,operation:Some(self.pending.pending_configuration_permit.as_ref().unwrap().operation),decoded:ManuallyDrop::new(Some(decoded)),decoded_close:None,receiving:ManuallyDrop::new(None),native:semio_framework_job::OriginalNativeDecodeSource::empty(semio_framework_job::OriginalNativeDecodePolicy{maximum_owned_bytes:crate::reserved_jobs::FRAMEWORK_CONFIGURATION_NATIVE_OWNED_BYTES}),failure:ManuallyDrop::new(None),state:[0;8],pending:None,outcomes:crate::tool_contract::ReservedJobOutcomes::new(),closing:false};
+        let operation=self.pending.pending_configuration_permit.as_ref().unwrap().operation;
+        self.pending.pending_configuration_worker=Some(crate::reserved_jobs::configuration_worker::ConfigurationWorker::new(job,operation,proof.contract().max_output_bytes,proof.contract().max_work_units_per_step,self.mounted_policy.preparation,self.mounted_policy.close,cx));
+        Ok(None)
     }
 
     async fn finish_configuration_command(&mut self,meta:&ActionMeta,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure>{
@@ -1377,7 +1351,7 @@ impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A,
         if permit.is_cancelled().await{return Err(Fault::new(FaultOrigin::Framework,FaultCode::new("interactive-job.cancelled"),"configuration binary was cancelled before config-store commit").into())}
         let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
         let command=self.pending.pending_configuration_command.take().expect("configuration commit retains its original typed command");cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);
-        let dispatched=self.stores.config_store.dispatch(command,identity).await;let permit=self.pending.pending_configuration_permit.take().unwrap();self.tools.finish_reserved_emit_producer(permit);if let Err(source)=dispatched{*self.pending.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Store(source));let _=self.pending.receive_configuration_failure(cx)?;return Ok(None)}self.pending.pending_configuration_identity=None;self.history.cache=None;Ok(Some(ToolOperations::<A>::empty_result("configCommand", meta, Vec::new(), Vec::new(), UiDirtyScope::Full).await))
+        let dispatched=self.stores.config_store.dispatch(command,identity).await;let permit=self.pending.pending_configuration_permit.take().unwrap();self.tools.finish_reserved_emit_producer(permit);if let Err(source)=dispatched{*self.pending.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Store(source));let _=self.pending.receive_configuration_failure(cx)?;return Ok(None)}self.pending.pending_configuration_identity=None;self.pending.pending_configuration_source=None;self.pending.pending_configuration_rejected=false;self.history.cache=None;Ok(Some(ToolOperations::<A>::empty_result("configCommand", meta, Vec::new(), Vec::new(), UiDirtyScope::Full).await))
     }
 
     /// 🧾️ Whether a verb of this declared kind may dirty the history panel. `dispatch_emit` records a

```
