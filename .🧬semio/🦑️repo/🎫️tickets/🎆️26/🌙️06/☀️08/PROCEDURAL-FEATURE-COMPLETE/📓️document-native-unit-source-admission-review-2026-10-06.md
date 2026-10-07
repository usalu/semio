# Original Native Unit Admission Source Review

The retained `ce14732b5dd8166aea3f0d88150ca2ef8e319157d745a7b271a10d2160528444` source was transient and has no exact matching blob among the 32 read-only path revisions. This review uses the closest authoritative preceding original source, Git revision `5c7f51ee6430`, not an alleged exact reversal.

- Baseline SHA256: `9407f54861682ce90c5a6000c11e49e54324860f37cd8a1fd241dd7567e7c9e1`.
- Current SHA256: `55fefa69fc3b49222c4ab57bc34abea321d5fad890ee46c0c61690e53ba057af`.
- All 20 named original functions in that baseline remain, including every asserted native law and the original history acceptance macro. No function added or removed in the complete baseline/current delta.
- Every changed executable line moves a snapshot/operation/helper path to its current owning IO or host module. Assert expressions, literals, inputs, loops, retirement behavior, scoped mesh parameter fields and command rosters are unchanged. Two doc paths move with those same modules.

Surface additions already present in this authoritative baseline are the first-step TXT law, TXT physical parity/budget/cancel/stale law strengthened by a large escaped note and one-byte exact typed refusal, incoming-group extent law, and source-lease identity/cancellation disposal law. Their actual runtime state is separately reported; source admission is not acceptance of pending Generation3D runtime behavior. Other owners added scoped widget/mesh fields and mesh action vocabulary/parameter refusal assertions in the prior 5c7 revision; they remain intact.

## Complete Reviewed Delta

```diff
--- 5c7f51ee6430
+++ current
@@ -79,5 +79,5 @@
     let export=crate::standards::v1::subsets::any::io::document_io::export_document(&incoming).unwrap();
     let command=Generation3dCommand::ImportDocument(import_document::ImportDocument{name:export.filename,payload:export.data,widget_id:None,channel:None,texture_id:None});
-    let expected=crate::standards::v1::subsets::any::schema::mutations::text::generation3d_host_snapshot_operations(&snapshot.host_snapshot,&incoming.host_snapshot);
+    let expected=crate::standards::v1::subsets::any::schema::mutations::generation3d_host_snapshot_operations(&snapshot.host_snapshot,&incoming.host_snapshot);
     let owner=semio_framework_plugin::ArtifactInstanceOperationOwnerHandle::new(<Generation3dPlayApp as ArtifactEditor>::build_instance_operation_owner());let mut work=Generation3dDocumentIoWork::new("importDocument",owner.clone());
     let extent=work.extent(&command,&snapshot,&protocol::InteractionState::default(),None).unwrap();let required=GENERATION3D_RETAINED_CAPACITY.rows(expected.len());
@@ -347,5 +347,5 @@
 fn production_semantic_digest(snapshot: &Generation3dSnapshot) -> [u8; 32] {
     let mut digest = store::ArtifactStoreInitializationDigest::new(b"generation3d.production-law.semantic");
-    digest.observe(&crate::standards::v1::subsets::any::schema::snapshot::binary::encode(snapshot));
+    digest.observe(&crate::standards::v1::subsets::any::io::binary::snapshot::encode(snapshot));
     digest.finish()
 }
@@ -358,8 +358,8 @@
     mutation_hex.try_reserve_exact(mutations.len()).expect("P3 production mutation owner preflight");
     for mutation in &mutations {
-        mutation_hex.push(production_hex(&crate::standards::v1::subsets::any::schema::mutations::binary::encode_op(mutation).expect("P3 production mutation encoding")));
+        mutation_hex.push(production_hex(&crate::standards::v1::subsets::any::io::binary::mutations::encode_op(mutation).expect("P3 production mutation encoding")));
     }
     let mut expected = production_initial_snapshot(label);
-    crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_apply_retained_mutations_for_test(&mut expected, &mutations);
+    crate::host::generation3d_apply_retained_mutations_for_test(&mut expected, &mutations);
     let expected_digest = production_semantic_digest(&expected);
     let wire = serde_json::to_vec(&serde_json::json!({
@@ -367,5 +367,5 @@
         "id": "generation3d-production-mounted-law",
         "vcs": {
-            "initialSnapshot": production_hex(&crate::standards::v1::subsets::any::schema::snapshot::binary::encode_mounted(&snapshot)),
+            "initialSnapshot": production_hex(&crate::standards::v1::subsets::any::io::binary::snapshot::encode_mounted(&snapshot)),
             "edits": [{
                 "id": "generation3d-production-all14-edit",
@@ -397,5 +397,5 @@
 /// 🔐️ Owns the publication lease `admit_production_envelope` took and releases it even when the law
 /// panics before its explicit release. The lease table is a PROCESS-GLOBAL 4-slot
-/// `FixedOperationRegistry` (`🧬️schema/🧬️mutations/💾️binary/🦀️.rs:211`), so one leaked slot turns every
+/// `FixedOperationRegistry` (`🚪️io/💾️binary/🧬️mutations/🦀️.rs:211`), so one leaked slot turns every
 /// later law in the same binary into `generation3d-publication.saturated` — an order-dependent red
 /// that has nothing to do with what those laws assert
@@ -412,5 +412,5 @@
         }
         self.released = true;
-        crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_release_publication_authority(self.handle.operation, self.handle.generation)
+        crate::host::generation3d_release_publication_authority(self.handle.operation, self.handle.generation)
     }
 }
@@ -425,5 +425,5 @@
     let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
     let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("P3 production ingress credits");
-    crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_admit_publication_authority(handle.operation, handle.generation, handle.generation.0, handle.generation.0, handle.generation.0, crate::standards::v1::subsets::any::schema::mutations::binary::Generation3dPublicationCredits { maximum_items: 8_192, maximum_output_pages: crate::standards::v1::subsets::any::schema::mutations::binary::GENERATION3D_MOUNTED_OUTPUT_CHANNELS, maximum_controls: crate::standards::v1::subsets::any::schema::mutations::binary::GENERATION3D_MOUNTED_CONTROL_CREDITS })
+    crate::host::generation3d_admit_publication_authority(handle.operation, handle.generation, handle.generation.0, handle.generation.0, handle.generation.0, crate::host::Generation3dPublicationCredits { maximum_items: 8_192, maximum_output_pages: crate::host::GENERATION3D_MOUNTED_OUTPUT_CHANNELS, maximum_controls: crate::host::GENERATION3D_MOUNTED_CONTROL_CREDITS })
     .expect("P3 production publication authority");
     for chunk in wire.chunks(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
@@ -449,5 +449,5 @@
 fn drive_production_envelope(app: &mut semio_framework_plugin::VcsArtifactApp<EditorApp<Generation3dPlayApp>>, handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll {
     for _ in 0..300_000 {
-        crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_refresh_publication_authority(handle.operation, handle.generation, app.artifact_generation_now().0)
+        crate::host::generation3d_refresh_publication_authority(handle.operation, handle.generation, app.artifact_generation_now().0)
             .expect("P3 authority refresh immediately before production maintenance");
         PluginApp::maintenance_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("one P3 production maintenance turn");
@@ -486,5 +486,5 @@
     expected.retire_cold();
 
-    use crate::standards::v1::subsets::any::schema::mutations::binary::Generation3dPublicationHostile::{Missing, WrongBase, WrongGeneration, WrongOperation, WrongParent};
+    use crate::host::Generation3dPublicationHostile::{Missing, WrongBase, WrongGeneration, WrongOperation, WrongParent};
     for (hostile, expected_code) in [
         (Missing, "generation3d-publication.authority-missing"),
@@ -502,7 +502,7 @@
         let mut lease = admit_production_envelope(&mut app, &wire);
         let handle = lease.handle;
-        crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_arm_publication_hostile(handle.operation, hostile);
+        crate::host::generation3d_arm_publication_hostile(handle.operation, hostile);
         assert_eq!(drive_production_envelope(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
-        assert_eq!(crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_take_publication_hostile_observed(handle.operation), Some(expected_code));
+        assert_eq!(crate::host::generation3d_take_publication_hostile_observed(handle.operation), Some(expected_code));
         assert_eq!(app.artifact_generation_now(), base_generation);
         let retained = context::snapshot(&app);
```
