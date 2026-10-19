--- 
+++ 
@@ -191,7 +191,18 @@
             }
-            async fn undo(&mut self, identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), $crate::os_store::VcsError> {
-                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::undo(m.as_mut(), identity).await),+ }
-            }
-            async fn redo(&mut self, identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), $crate::os_store::VcsError> {
-                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::redo(m.as_mut(), identity).await),+ }
+            async fn undo(&mut self, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> where M:ReferenceSpaceMember {
+        let mut candidates = Vec::new();
+        for (document_id, member) in self.members.iter() {
+                if let Some(timestamp) =member.last_local_edit_timestamp().await {
+                candidates.push((timestamp.cmp_key(),  document_id.clone()));
+            }
+            }
+        let target = candidates.into_iter().max_by_key(|(cmp_key, _)| *cmp_key).map(|(_, document_id)| document_id);
+        let document_id = target.ok_or(VcsError::NothingToUndo)?;
+        self.members.get_mut(&document_id).ok_or(VcsError::NothingToUndo)?.undo(identity).await }
+
+            async fn redo(&mut self, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> where M:ReferenceSpaceMember {
+        let mut candidates = Vec::new();
+        for (document_id, member) in self.members.iter() {
+                if let Some(timestamp) =member.last_undone_local_edit_timestamp().await {
+                candidates.push((timestamp.cmp_key(),  document_id.clone()));
             }
@@ -209,11 +220,25 @@
             }
-            async fn event_log_payload(&self) -> Result<Vec<u8>, $crate::os_store::VcsError> {
-                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::event_log_payload(m.as_ref()).await),+ }
-            }
-            async fn announce_tail_edit_payload(&mut self) -> Result<Vec<u8>, $crate::os_store::VcsError> {
-                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::announce_tail_edit_payload(m.as_mut()).await),+ }
-            }
-            async fn ingest_remote_payload(&mut self, envelopes: &[u8]) -> Result<(), $crate::os_store::VcsError> {
-                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::ingest_remote_payload(m.as_mut(), envelopes).await),+ }
-            }
+            async fn event_log_payload(&self) -> Result<Vec<u8>, VcsError> {
+        Ok(crate::os_spr::encode_envelopes(& self .event_log()?)) }
+
+            async fn announce_tail_edit_payload(&mut self) -> Result<Vec<u8>, VcsError> {
+        let Some(edit) = self.envelope.vcs.edits.last().filter(|edit| self.envelope.holds_committed_edit(&edit.id)) else { return Ok(Vec::new()) };
+        let document_id = ArtifactId(self.envelope.id.clone());
+        let schema = SchemaId(self.envelope.schema.clone());
+        let op_envelopes =crate::os_spr::mutation_envelope_from_edit::<P, Mutation>(edit, &document_id, &schema).map_err(|error| VcsError::Serialize(error.to_string()))?;
+        for op_envelope in &op_envelopes {
+                match self .dag.seed_applied(op_envelope.mutation_id.clone()) { Ok(()) => {}
+                Err(rejected) if rejected.error ==crate::os_spr::MutationDagError::Duplicate => {}
+                Err(rejected) => return Err(VcsError::ValidationFailed(rejected.error.to_string())), }
+            }
+            Ok(crate::os_spr::encode_envelopes(&op_envelopes))
+    }
+            async fn ingest_remote_payload(&mut self, envelopes: &[u8]) -> Result<Vec<crate::os_spr::MergeReport>, VcsError>  where P: Clone, Mutation: ColdMutation<P> {
+                let envelopes =crate::os_spr::decode_envelopes( envelopes).map_err(|error| VcsError::Deserialize(error.to_string()))?;
+        let mut reports = Vec::with_capacity(envelopes.len());
+        for envelope in envelopes {
+            reports.push(self.ingest_remote(envelope).await?); }
+
+        Ok(reports)
+    }
             async fn tail_group_id(&self) -> Option<String> {
