--- 
+++ 
@@ -270,2 +270,10 @@
     async fn drive_host_typed_continuations<P: PluginApp>(app: &mut P, receiver: u32, continuations: usize, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Vec<artifact_app_laws::TypedOperationFixtureOwnershipReceipt> {
+    /// 🔁️ The host's OWN typed-operation continuation, unit for unit: exactly what
+    /// `plugin_runtime::advance_typed_operation_output` drives per turn — one publication unit, the result
+    /// page for the bound receiver plus its mandatory ACK, then the effect, event, UI-scope and completion
+    /// outboxes. Deliberately calls NO `maintenance_step`: a slot the host cannot release on this path alone
+    /// is a leak, whatever the cooperative rotation would eventually have done about it.
+    #[cfg(test)]
+    async fn drive_host_typed_continuations<P: PluginApp>(app: &mut P, receiver: u32, continuations: usize, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Vec<artifact_app_laws::TypedOperationFixtureOwnershipReceipt> {
+        let mut receiving_job=crate::app_testing::artifact_app_laws::FixtureOriginalJob::new(crate::app_testing::artifact_app_laws::fixture_retained_turn());
         let mut ownership_receipts = Vec::new();
@@ -276,6 +284,6 @@
             }
-            app.advance_typed_operation_publication(identity,mounted_policy.preparation).await.expect("one host continuation unit");
+            receiving_job.receive(mounted_policy.preparation,|receiving|::semio_framework_async::poll::resolve_ready(app.advance_typed_operation_publication(identity,mounted_policy.preparation,receiving))).expect("one host continuation unit");
             artifact_app_laws::retain_original_ownership_receipts(app, mounted_policy, &mut ownership_receipts).expect("original fixture continuation receipts");
-            while let Some(page) = app.take_typed_operation_result_page(receiver) {
-                assert!(app.acknowledge_typed_operation_result(page.token).expect("presented result page accepts its exact token"));
+            while let Some(mut page) = crate::app_testing::artifact_app_laws::take_fixture_result_page(app,receiver,mounted_policy.preparation).expect("original fixture page presentation") {
+                assert!(app.acknowledge_typed_operation_result(crate::app_testing::artifact_app_laws::close_fixture_result_page(&mut page,mounted_policy.close)).expect("presented result page accepts its exact token"));
             }
@@ -290,2 +298,15 @@
     async fn drive_host_call_with_deferred_acks<P: PluginApp>(app: &mut P, receiver: u32, continuations: usize, stop_at_idle: bool, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> HostCallCensus {
+
+    /// 🕰️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B16: the host's OWN call, with the ACK round trip the
+    /// reactor really has. `poll` routes an `Event::Message` acknowledgement at the TOP of a turn and only
+    /// then advances the operation, so the ACK a turn's presented page earns is submitted with the NEXT
+    /// continuation, never inside the turn that produced it. `stop_at_idle` picks which host call this is:
+    /// `false` is the renderer's `settlePluginTurn`, which also continues while it owes an acknowledgement;
+    /// `true` is every call that can only read the turn's STATUS — the eventless `drainTypedOperations` poll
+    /// and a `refresh-ui` settle whose surfaces are already published. Answers
+    /// `(terminal completions, continuations spent, first parked continuation)`, where "parked" is a
+    /// continuation that ended owing typed-operation work while the turn reported none runnable.
+    #[cfg(test)]
+    async fn drive_host_call_with_deferred_acks<P: PluginApp>(app: &mut P, receiver: u32, continuations: usize, stop_at_idle: bool, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> HostCallCensus {
+        let mut receiving_job=crate::app_testing::artifact_app_laws::FixtureOriginalJob::new(crate::app_testing::artifact_app_laws::fixture_retained_turn());
         let mut ownership_receipts = Vec::new();
@@ -300,5 +321,5 @@
             }
-            app.advance_typed_operation_publication(identity,mounted_policy.preparation).await.expect("one host continuation unit");
+            receiving_job.receive(mounted_policy.preparation,|receiving|::semio_framework_async::poll::resolve_ready(app.advance_typed_operation_publication(identity,mounted_policy.preparation,receiving))).expect("one host continuation unit");
             artifact_app_laws::retain_original_ownership_receipts(app, mounted_policy, &mut ownership_receipts).expect("original deferred continuation receipts");
-            while let Some(page) = app.take_typed_operation_result_page(receiver) {
+            while let Some(mut page) = crate::app_testing::artifact_app_laws::take_fixture_result_page(app,receiver,mounted_policy.preparation).expect("original fixture page presentation") {
                 if matches!(page.lane, TypedOperationResultLane::Terminal | TypedOperationResultLane::Fault) {
@@ -306,3 +327,3 @@
                 }
-                acknowledgements.push(page.token);
+                acknowledgements.push(crate::app_testing::artifact_app_laws::close_fixture_result_page(&mut page,mounted_policy.close));
             }
@@ -327,2 +348,4 @@
     }
+
+    #[cfg(test)]
     struct HostCallCensus {
