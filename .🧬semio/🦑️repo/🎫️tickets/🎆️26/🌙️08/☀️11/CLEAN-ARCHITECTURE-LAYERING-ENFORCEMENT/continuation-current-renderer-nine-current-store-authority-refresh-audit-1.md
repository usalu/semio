# Current Renderer Nine Store Authority Refresh

Source17 capture refused the changed Store body. Renderer8 and its negative applicability remain immutable. This successor conserves the exact three-descriptor full fixture pair and every original bounded slot law. It admits only held verification; physicalLayoutAccepted remains false and all three fresh original wholes remain required.

The complete captured11 body is retained at `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/current-native-origin/epoch-11/product/initial-captured-input/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`; its SHA256 is `e2ffb24cd54e5ec263e7cbf6bc6f91eab220a4f9ba3a7239a8f946da1162385a`. Current Root Store full UTF-8 SHA256 is `3f1d99cf503fbd12fcf43981f747cd7687c74f4064654c25c9c7106c204f73be`. The successor proposal carries both full bodies, their full replacement recipe and unchanged declared EditReplay.operation_progress field claim. The prior Renderer8 current authority hash was `0b2942bf7b092048f1d046f1c487feb0a43febe32bb688572301c4575304281d`. No historical size attribution or actor/cause is inferred.

Current observed Renderer8-after to Renderer9-after complete diff follows.

```diff
--- Renderer8-current-authority
+++ Renderer9-current-authority
@@ -727,6 +727,9 @@
 }
 
 impl ErasedSnapshotRead {
+    /// 📏️ Whole allocation retained by one immutable read's shared return flag.
+    pub const LEASE_ALLOCATION_BYTES: usize = std::mem::size_of::<(std::sync::atomic::AtomicUsize, std::sync::atomic::AtomicUsize, std::sync::atomic::AtomicBool)>();
+
     fn new<T: Send + Sync + 'static>(owner: Arc<T>, lease: SnapshotReadLease) -> Self {
         Self { owner: Some(owner), lease: Some(lease) }
     }
@@ -26514,6 +26517,8 @@
     fn prepare_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String>;
     fn stage_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String>;
     fn adopt_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String>;
+    /// 🪞️ Reads an exact private staged root before its common visibility decision.
+    fn snapshot_read_erased_for_publication(&self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<(u64, [u8; 32], ErasedSnapshotRead), String>;
     fn abort_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError>;
     /// 🧵️ The live typed snapshot behind an opaque immutable ownership boundary.
     fn snapshot_read_erased_now(&self) -> Result<ErasedSnapshotRead, String>;
@@ -26844,6 +26849,18 @@
         publication.close_step(ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes })
     }
 
+    fn snapshot_read_erased_for_publication(&self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<(u64, [u8; 32], ErasedSnapshotRead), String> {
+        let publication = publication.as_any_mut().downcast_mut::<MemberStoreOneItemPublication<P, Mutation>>().ok_or_else(|| "prepared member read concrete owner type does not match".to_string())?;
+        if !publication.member.as_ref().is_some_and(|member| Arc::ptr_eq(member, &self.snapshot_read_leases)) { return Err("prepared member read belongs to a different exact store authority".into()); }
+        if !visibility.pending() || self.generation != publication.publication.expected_generation || self.content_revision != publication.publication.expected_revision { return Err("prepared member read requires its pending unchanged store authority".into()); }
+        let root = self.durable_group_root.as_ref().ok_or_else(|| "prepared member read requires a fully staged common root".to_string())?;
+        if !Arc::ptr_eq(&root.visibility, visibility) { return Err("prepared member read visibility authority does not match".into()); }
+        if !grant.permits_one() || grant.maximum_bytes < ErasedSnapshotRead::LEASE_ALLOCATION_BYTES { return Err("prepared member read lease allocation is not funded".into()); }
+        let owner = Arc::clone(root.current.as_ref().ok_or_else(|| "prepared member read lost its immutable candidate".to_string())?);
+        let lease = self.snapshot_read_leases.try_issue(Arc::clone(&owner)).map_err(|_| "prepared member read lease registry is busy, saturated, or exhausted".to_string())?;
+        Ok((root.generation, root.content_revision, ErasedSnapshotRead::new(owner, lease)))
+    }
+
     fn snapshot_read_erased_now(&self) -> Result<ErasedSnapshotRead, String> {
         if !self.snapshot_read_leases.publish_authority(self.generation(), self.content_revision()) {
             return Err("snapshot read commit authority is busy or exhausted".into());
@@ -27180,6 +27197,8 @@
 
     fn adopt_one_item_publication(&mut self, _publication: &mut dyn ErasedMemberStoreOneItemPublication, _visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, _grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> { match *self {} }
 
+    fn snapshot_read_erased_for_publication(&self, _publication: &mut dyn ErasedMemberStoreOneItemPublication, _visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, _grant: ArtifactStoreOneItemGrant) -> Result<(u64, [u8; 32], ErasedSnapshotRead), String> { match *self {} }
+
     fn abort_one_item_publication(&mut self, _publication: &mut dyn ErasedMemberStoreOneItemPublication, _grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
         match *self {}
     }
@@ -27443,6 +27462,9 @@
             }
             fn adopt_one_item_publication(&mut self, publication: &mut dyn $crate::os_store::ErasedMemberStoreOneItemPublication, visibility: &::std::sync::Arc<$crate::os_vcs::ArtifactGroupVisibility>, grant: $crate::os_store::ArtifactStoreOneItemGrant) -> Result<$crate::os_store::ArtifactStoreOneItemPreparationStep, String> {
                 match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::adopt_one_item_publication(m.as_mut(), publication, visibility, grant)),+ }
+            }
+            fn snapshot_read_erased_for_publication(&self, publication: &mut dyn $crate::os_store::ErasedMemberStoreOneItemPublication, visibility: &::std::sync::Arc<$crate::os_vcs::ArtifactGroupVisibility>, grant: $crate::os_store::ArtifactStoreOneItemGrant) -> Result<(u64, [u8; 32], $crate::os_store::ErasedSnapshotRead), String> {
+                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::snapshot_read_erased_for_publication(m.as_ref(), publication, visibility, grant)),+ }
             }
             fn abort_one_item_publication(&mut self, publication: &mut dyn $crate::os_store::ErasedMemberStoreOneItemPublication, grant: $crate::os_store::ArtifactStoreOneItemGrant) -> Result<$crate::os_store::SnapshotRetirementStep, ::semio_framework_value::ValueError> {
                 match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::abort_one_item_publication(m.as_mut(), publication, grant)),+ }
```

The exact helper is cargo-inputs/📥️current-engine-layout-9/📜️script.ts. Registered GUI source control row is 900.244001. Planned fresh receipt is 🗑️generated/current-engine-layout-9/green-1/admission.json. Source controls have not executed at this report creation.
