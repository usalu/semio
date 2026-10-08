# Current Renderer Thirteen Complete Source Authorities

Source22methodready; prior draft assembly refused changed pagedappend binding without executable invocation. Fresh13 now carries 3completecurrentfullauthority refreshes (Store, pagedappend, retainedclone asreached) fromtheir retainedphysicalcaptured baselines withcomplete before/after bodies/hashes/fullreplacementrecipes. Specific declaredfieldclaims remain exact and are not an exhaustive ABI or physicalsize attribution; full native providers capture and originalwhole remainsmandatory. Fixturefullpair/all budgets/capacities/owners/wholeassertions conserved. No Rootwrite/compiler/layoutacceptance.

```diff
--- Renderer12-🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs
+++ Renderer13-🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs
@@ -1892,19 +1892,8 @@
         if maximum_items == 0 {
             return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
         }
-        if let Some(active) = self.active.as_mut() {
-            return match active.close_step(maximum_items, maximum_bytes)? {
-                SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= maximum_items && released_bytes <= maximum_bytes => Ok(SnapshotRetirementStep::Pending { released_items, released_bytes }),
-                SnapshotRetirementStep::Pending { .. } => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "displaced envelope nested owner exceeded its exact close grant")),
-                SnapshotRetirementStep::Blocked => Ok(SnapshotRetirementStep::Blocked),
-                SnapshotRetirementStep::Complete => {
-                    if !active.terminal_is_empty() {
-                        return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "displaced envelope nested owner reported Complete without its terminal-empty witness"));
-                    }
-                    drop(self.active.take());
-                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
-                }
-            };
+        if self.active.is_some() {
+            return artifact_retirement_box_close_step(&mut self.active, maximum_items, maximum_bytes);
         }
         let Some(envelope) = self.envelope.as_mut() else {
             self.phase = ArtifactStoreEnvelopeRetirementPhase::Complete;
@@ -2059,7 +2048,7 @@
     /// driver that only ever sees the ERASED envelope can still pay the innermost owner's physical
     /// minimum instead of under-granting it forever.
     fn next_close_byte_demand(&self) -> usize {
-        self.active.as_ref().map_or(1, |active| active.next_close_byte_demand())
+        self.active.as_ref().map_or(usize::from(self.envelope.is_some()), artifact_retirement_box_byte_demand)
     }
 }
 
@@ -2088,17 +2077,11 @@
 
 impl ErasedSnapshotRetirement for ArtifactGenesisRetirement {
     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
-        if maximum_items == 0 { return Ok(SnapshotRetirementStep::Blocked); }
         let slot = if self.snapshot.is_some() { &mut self.snapshot } else { &mut self.pack };
-        let Some(active) = slot.as_mut() else { return Ok(SnapshotRetirementStep::Complete) };
-        match active.close_step(maximum_items.min(1), maximum_bytes)? {
-            SnapshotRetirementStep::Complete if active.terminal_is_empty() => { drop(slot.take()); Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }) }
-            SnapshotRetirementStep::Complete => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "genesis retirement requires its exact terminal-empty witness")),
-            step => Ok(step),
-        }
+        artifact_retirement_box_close_step(slot, maximum_items.min(1), maximum_bytes)
     }
     fn terminal_is_empty(&self) -> bool { self.snapshot.is_none() && self.pack.is_none() }
-    fn next_close_byte_demand(&self) -> usize { self.snapshot.as_ref().or(self.pack.as_ref()).map_or(1, |active| active.next_close_byte_demand()) }
+    fn next_close_byte_demand(&self) -> usize { self.snapshot.as_ref().or(self.pack.as_ref()).map_or(0, artifact_retirement_box_byte_demand) }
 }
 
 impl Drop for ArtifactGenesisRetirement {
@@ -2562,7 +2545,7 @@
     }
 
     fn close_phase_witness(&self) -> String {
-        format!("{:?}/started={}/active={}/demand={}/terminal={}/cursorBytes={}", self.phase, self.started, self.active.is_some(), self.active.as_ref().map_or(0, |owner| owner.next_close_byte_demand()), self.active.as_ref().is_some_and(|owner| owner.terminal_is_empty()), self.active.as_ref().map_or(0, |owner| std::mem::size_of_val(owner.as_ref())))
+        format!("{:?}/started={}/active={}", self.phase, self.started, self.active.is_some())
     }
 }
 
@@ -26565,6 +26548,8 @@
     fn adopt_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String>;
     /// 🪞️ Reads an exact private staged root before its common visibility decision.
     fn snapshot_read_erased_for_publication(&self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<(u64, [u8; 32], ErasedSnapshotRead), String>;
+    /// ↩️ Returns a private candidate lease before its exact staged snapshot may be aborted.
+    fn return_prepared_snapshot_read_erased(&self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, snapshot: ErasedSnapshotRead, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, SnapshotRetirementRejected>;
     fn abort_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError>;
     /// 🧵️ The live typed snapshot behind an opaque immutable ownership boundary.
     fn snapshot_read_erased_now(&self) -> Result<ErasedSnapshotRead, String>;
@@ -26906,6 +26891,20 @@
         Ok((root.generation, root.content_revision, ErasedSnapshotRead::new(owner, lease)))
     }
 
+    fn return_prepared_snapshot_read_erased(&self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, snapshot: ErasedSnapshotRead, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, SnapshotRetirementRejected> {
+        let reject = |snapshot, reason: &str| Err(SnapshotRetirementRejected { snapshot, reason: reason.into() });
+        let Some(publication) = publication.as_any_mut().downcast_mut::<MemberStoreOneItemPublication<P, Mutation>>() else { return reject(snapshot, "prepared read return concrete publication type does not match"); };
+        if !publication.member.as_ref().is_some_and(|member| Arc::ptr_eq(member, &self.snapshot_read_leases)) { return reject(snapshot, "prepared read return belongs to a different exact member"); }
+        let Some(root) = self.durable_group_root.as_ref() else { return reject(snapshot, "prepared read return requires its retained staged root"); };
+        if visibility.committed() || !Arc::ptr_eq(&root.visibility, visibility) { return reject(snapshot, "prepared read return requires its exact undecided or aborted visibility"); }
+        if !root.current.as_ref().is_some_and(|candidate| snapshot.get::<P>().is_some_and(|borrowed| std::ptr::eq(candidate.as_ref(), borrowed))) { return reject(snapshot, "prepared read return does not retain this exact private candidate"); }
+        if !grant.permits_one() || grant.maximum_bytes < ErasedSnapshotRead::LEASE_ALLOCATION_BYTES { return reject(snapshot, "prepared read return whole lease allocation is not funded"); }
+        match snapshot.into_typed::<P>(&self.snapshot_read_leases) {
+            Ok(owner) => { drop(owner); Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: ErasedSnapshotRead::LEASE_ALLOCATION_BYTES }) }
+            Err(snapshot) => reject(snapshot, "prepared read return exact lease registry is busy or stale"),
+        }
+    }
+
     fn snapshot_read_erased_now(&self) -> Result<ErasedSnapshotRead, String> {
         if !self.snapshot_read_leases.publish_authority(self.generation(), self.content_revision()) {
             return Err("snapshot read commit authority is busy or exhausted".into());
@@ -27243,6 +27242,7 @@
     fn adopt_one_item_publication(&mut self, _publication: &mut dyn ErasedMemberStoreOneItemPublication, _visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, _grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> { match *self {} }
 
     fn snapshot_read_erased_for_publication(&self, _publication: &mut dyn ErasedMemberStoreOneItemPublication, _visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, _grant: ArtifactStoreOneItemGrant) -> Result<(u64, [u8; 32], ErasedSnapshotRead), String> { match *self {} }
+    fn return_prepared_snapshot_read_erased(&self, _publication: &mut dyn ErasedMemberStoreOneItemPublication, _visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, _snapshot: ErasedSnapshotRead, _grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, SnapshotRetirementRejected> { match *self {} }
 
     fn abort_one_item_publication(&mut self, _publication: &mut dyn ErasedMemberStoreOneItemPublication, _grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
         match *self {}
@@ -27463,6 +27463,10 @@
                 match self { $(Self::$variant(open) => $crate::os_store::MemberOpenOperation::close_step(open.as_mut(), maximum_items, maximum_bytes)),+ }
             }
 
+            fn next_close_byte_demand(&self) -> usize {
+                match self { $(Self::$variant(open) => $crate::os_store::MemberOpenOperation::next_close_byte_demand(open.as_ref())),+ }
+            }
+
             fn terminal_is_empty(&self) -> bool {
                 match self { $(Self::$variant(open) => $crate::os_store::MemberOpenOperation::terminal_is_empty(open.as_ref())),+ }
             }
@@ -27510,6 +27514,9 @@
             }
             fn snapshot_read_erased_for_publication(&self, publication: &mut dyn $crate::os_store::ErasedMemberStoreOneItemPublication, visibility: &::std::sync::Arc<$crate::os_vcs::ArtifactGroupVisibility>, grant: $crate::os_store::ArtifactStoreOneItemGrant) -> Result<(u64, [u8; 32], $crate::os_store::ErasedSnapshotRead), String> {
                 match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::snapshot_read_erased_for_publication(m.as_ref(), publication, visibility, grant)),+ }
+            }
+            fn return_prepared_snapshot_read_erased(&self, publication: &mut dyn $crate::os_store::ErasedMemberStoreOneItemPublication, visibility: &::std::sync::Arc<$crate::os_vcs::ArtifactGroupVisibility>, snapshot: $crate::os_store::ErasedSnapshotRead, grant: $crate::os_store::ArtifactStoreOneItemGrant) -> Result<$crate::os_store::SnapshotRetirementStep, $crate::os_store::SnapshotRetirementRejected> {
+                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::return_prepared_snapshot_read_erased(m.as_ref(), publication, visibility, snapshot, grant)),+ }
             }
             fn abort_one_item_publication(&mut self, publication: &mut dyn $crate::os_store::ErasedMemberStoreOneItemPublication, grant: $crate::os_store::ArtifactStoreOneItemGrant) -> Result<$crate::os_store::SnapshotRetirementStep, ::semio_framework_value::ValueError> {
                 match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::abort_one_item_publication(m.as_mut(), publication, grant)),+ }
```
```diff
--- Renderer12-🧰️framework/🔨️modules/🌱️value/📦️paged/🎮️append/🦀️.rs
+++ Renderer13-🧰️framework/🔨️modules/🌱️value/📦️paged/🎮️append/🦀️.rs
@@ -71,11 +71,22 @@
         Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: 0, released_bytes: 0 }))
     }
     pub fn begin_close(&mut self) -> bool { if self.closing { return false; } self.closing = true; true }
+    /// 📐️ Reads the next native retirement birth while retaining the pending chunk.
+    pub fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, ValueError> {
+        if self.retirement.is_some() { return Err(ValueError::new(ValueRefusalKind::UnsupportedOwner, "cold UTF-8 append retirement has no controlled birth demand")); }
+        if !self.controlled_close.is_empty() { return self.controlled_close.next_capacity_byte_demand(maximum_release_bytes); }
+        Ok(if self.pending.is_some() { std::mem::size_of::<crate::retirement::controlled::ControlledRetirement<String>>() } else { 0 })
+    }
+    /// 📐️ Reads the next physical chunk or scaffold release without closing it.
+    pub fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
+        if self.retirement.is_some() { return Err(ValueError::new(ValueRefusalKind::UnsupportedOwner, "cold UTF-8 append retirement has no controlled release demand")); }
+        self.controlled_close.next_release_byte_demand()
+    }
     pub fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
         if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
         if !self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "native UTF-8 append must begin close before granted retirement")); }
         if self.retirement.is_some() { return Err(ValueError::new(ValueRefusalKind::UnsupportedOwner, "cold UTF-8 append retirement cannot enter granted closure")); }
-        if !self.controlled_close.is_empty() { return self.controlled_close.step_granted(grant); }
+        if !self.controlled_close.is_empty() { return self.controlled_close.step_granted(grant).map(|step|RetainedCloneStep::Progress(step.progress())); }
         if let Some(step) = self.controlled_close.begin_granted(&mut self.pending, grant)? { return Ok(step); }
         if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
         self.source = None;
```
```diff
--- Renderer12-🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs
+++ Renderer13-🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs
@@ -269,11 +269,15 @@
 trait ControlledCloneRetirement: Send {
     fn step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError>;
     fn terminal_is_empty(&self) -> bool;
+    fn next_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError>;
+    fn next_release_byte_demand(&self) -> Result<usize, crate::ValueError>;
 }
 
 impl<T: RetireOwned> ControlledCloneRetirement for crate::retirement::controlled::ControlledRetirement<T> {
     fn step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> { self.step(grant) }
     fn terminal_is_empty(&self) -> bool { self.terminal_is_empty() }
+    fn next_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> { self.next_capacity_byte_demand(maximum_release_bytes) }
+    fn next_release_byte_demand(&self) -> Result<usize, crate::ValueError> { self.next_release_byte_demand() }
 }
 
 #[derive(Default)]
@@ -284,6 +288,18 @@
 }
 
 impl RetainedCloneClose {
+    /// 📐️ Observes the next controlled child birth without allocating or moving an owner.
+    pub fn next_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
+        if self.retirement.is_some() { return Err(crate::ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "cold clone retirement has no controlled birth demand")); }
+        self.controlled.as_ref().map_or(Ok(0), |owner| owner.next_capacity_byte_demand(maximum_release_bytes))
+    }
+
+    /// 📐️ Observes the next physical body or terminal scaffold release independently of copying.
+    pub fn next_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
+        if self.retirement.is_some() { return Err(crate::ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "cold clone retirement has no controlled release demand")); }
+        self.controlled.as_ref().map_or(Ok(0), |owner| if owner.terminal_is_empty() { Ok(self.controlled_bytes) } else { owner.next_release_byte_demand() })
+    }
+
     pub fn begin<T: RetireOwned>(&mut self, value: T) -> Result<(), crate::ValueError> {
         if !self.is_empty() {
             return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained clone retirement frontier is occupied"));
```

ExactGUI271001/plannedreceipt 🗑️generated/current-engine-layout-13/green-1/admission.json.
