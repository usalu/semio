# Renderer4 Current Store and Retained Clone Transitions

Read-only complete actual Product11 initial bodies compared with current Root physical bodies. Product11 failed and supplies no historical acceptance. Descriptor values remain proposed, with fresh whole required.

## 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs

Before SHA256 `e2ffb24cd54e5ec263e7cbf6bc6f91eab220a4f9ba3a7239a8f946da1162385a`; current SHA256 `d15c17b3d53b5f66e954bd738db58182ce39f1321b02989bc03acd13ef5c8edb`.

```diff
--- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/current-native-origin/epoch-11/product/initial-captured-input/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs
+++ 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs
@@ -20589,6 +20589,20 @@
                 if stage.edit.id.is_empty() || stage.edit.id.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES || authority.base_applied_edit_count != self.applied_edit_ids.len() {
                     return Err(VcsError::ValidationFailed("domain-prepared batched candidate failed its live cursor authority".into()));
                 }
+                if let Some(edit_id) = self.batch_amend_target(publication.transaction.as_ref()) {
+                    let existing = self.envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id).ok_or_else(|| VcsError::ValidationFailed("batched cursor preparation lost its open edit".into()))?;
+                    let capacity = existing.inverse.len().checked_add(stage.edit.inverse.len()).ok_or_else(|| VcsError::ValidationFailed("batched inverse capacity overflow".into()))?;
+                    if let Some(bytes) = existing.inverse.next_capacity_allocation_bytes(capacity).map_err(|error| VcsError::ValidationFailed(error.reason.into()))? {
+                        if bytes > grant.maximum_bytes {
+                            return Err(VcsError::ValidationFailed("batched inverse backing cannot fit the admitted allocation grant".into()));
+                        }
+                        let step = existing.inverse.reserve_capacity_one(capacity, grant.maximum_bytes).map_err(|error| VcsError::ValidationFailed(error.reason.into()))?;
+                        if !step.progressed || step.allocated_bytes > grant.maximum_bytes {
+                            return Err(VcsError::ValidationFailed("batched inverse backing failed its exact allocation admission".into()));
+                        }
+                        return Ok(ArtifactStoreOneItemAdvance::Progress(publication.progress()));
+                    }
+                }
                 publication.phase = ArtifactStoreOneItemPublicationPhase::PreflightingCommit;
                 Ok(ArtifactStoreOneItemAdvance::Progress(publication.progress()))
             }
@@ -20806,6 +20820,7 @@
         }
         if publication.stage.is_none() {
             let mut staged = Box::new(Self::empty_batch_stage(&authority));
+            staged.edit.inverse = semio_framework_value::list::PagedList::with_payload_page_bytes(grant.maximum_bytes).map_err(|error| VcsError::ValidationFailed(error.reason.into()))?;
             staged.mutation_retirement = (*self.mutation_retirement_factory).clone();
             staged.snapshot_retirement = (*self.snapshot_retirement_factory).clone();
             if staged.mutation_retirement.is_none() || staged.snapshot_retirement.is_none() {
@@ -20817,11 +20832,15 @@
             publication.stage = Some(staged);
         }
         let inverse = &mut publication.stage.as_mut().expect("validated staged batch edit").edit.inverse;
-        if let Some(bytes) = inverse.next_exact_capacity_allocation_bytes(publication.footprint.work_items).map_err(|error| VcsError::ValidationFailed(error.reason.into()))? {
+        let backing = if publication.transaction_open { inverse.next_capacity_allocation_bytes(publication.footprint.work_items) } else { inverse.next_exact_capacity_allocation_bytes(publication.footprint.work_items) };
+        if let Some(bytes) = backing.map_err(|error| VcsError::ValidationFailed(error.reason.into()))? {
             if bytes > grant.maximum_bytes {
-                return Ok(ArtifactStoreOneItemAdvance::Blocked);
-            }
-            inverse.reserve_exact_capacity_one(publication.footprint.work_items, grant.maximum_bytes).map_err(|error| VcsError::ValidationFailed(error.reason.into()))?;
+                return Err(VcsError::ValidationFailed("batched inverse backing cannot fit the admitted allocation grant".into()));
+            }
+            let step = if publication.transaction_open { inverse.reserve_capacity_one(publication.footprint.work_items, grant.maximum_bytes) } else { inverse.reserve_exact_capacity_one(publication.footprint.work_items, grant.maximum_bytes) }.map_err(|error| VcsError::ValidationFailed(error.reason.into()))?;
+            if !step.progressed || step.allocated_bytes > grant.maximum_bytes {
+                return Err(VcsError::ValidationFailed("batched inverse backing failed its exact allocation admission".into()));
+            }
             return Ok(ArtifactStoreOneItemAdvance::Progress(publication.progress()));
         }
         let owner = publication.preparation.as_mut().expect("validated batched fold preparation owner");
```

## 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs

Before SHA256 `a51897f27ba16072ae73c886834ea600371bbb2b03edd09191ba7b260c81201a`; current SHA256 `ce4e3e61f7f1581d2d3027b60da3e1cd755bcd396fef6d323899c1bdf0fb9238`.

```diff
--- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/current-native-origin/epoch-11/product/initial-captured-input/🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs
+++ 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs
@@ -59,7 +59,7 @@
     /// 🔗️ Retains a projected native field through the actual immutable root lease.
     pub fn project_owned<U: ?Sized + Sync, F>(&self, discriminator: usize, project: F) -> RetainedOwnedProjection<U>
     where F: for<'source> FnOnce(&'source T) -> &'source U {
-        RetainedOwnedProjection::from_source(self.borrow().project(discriminator, project))
+        unsafe { RetainedOwnedProjection::from_source(self.borrow().project(discriminator, project)) }
     }
 }
 
```
