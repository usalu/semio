# UI Ten Current Authorities

UI9latercurrentretainedclone/fielddrift requires2freshfullcaptured14/currentauthoritytransitions, 6total. Same166040descriptorfixturefullpair/alloriginalwholeassertions/owners/caps/stacks/threshold conserved. Rootsourcechanges do not establishphysicalsizeinvariance or historicalfieldattribution, freshnativewholes mandatory.

```diff
--- UI9-🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs
+++ UI10-🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs
@@ -260,6 +260,14 @@
     fn begin_close(&mut self) -> bool;
     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, crate::ValueError>;
     fn terminal_is_empty(&self) -> bool;
+    /// 📐️ Observes the next granted close birth; unsupported cursors retain their owners.
+    fn next_close_capacity_byte_demand(&self, _maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
+        Err(crate::ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "clone cursor has no controlled close birth demand"))
+    }
+    /// 📐️ Observes the next body or scaffold release independently of payload copying.
+    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
+        Err(crate::ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "clone cursor has no controlled close release demand"))
+    }
     fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
         if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
         Err(crate::ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "clone cursor has no capacity-bearing close authority"))
@@ -288,6 +296,11 @@
 }
 
 impl RetainedCloneClose {
+    /// 📐️ Observes the actual controlled owner Box required by the next handoff.
+    pub fn next_owner_capacity_byte_demand<T: RetireOwned>(&self, present: bool, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
+        if !self.is_empty() { return self.next_capacity_byte_demand(maximum_release_bytes); }
+        Ok(if present { size_of::<crate::retirement::controlled::ControlledRetirement<T>>() } else { 0 })
+    }
     /// 📐️ Observes the next controlled child birth without allocating or moving an owner.
     pub fn next_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
         if self.retirement.is_some() { return Err(crate::ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "cold clone retirement has no controlled birth demand")); }
```
```diff
--- UI9-🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️field/🦀️.rs
+++ UI10-🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️field/🦀️.rs
@@ -58,6 +58,21 @@
         }
         close_retained_binding(&mut self.source, grant)
     }
+    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
+        if !self.closing { return Ok(0); }
+        match self.child.as_ref() {
+            Some(child) if !child.terminal_is_empty() => child.next_close_capacity_byte_demand(maximum_release_bytes),
+            _ => Ok(0),
+        }
+    }
+    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
+        if !self.closing { return Ok(0); }
+        match self.child.as_ref() {
+            Some(child) if child.terminal_is_empty() => Ok(size_of::<T::Cursor>()),
+            Some(child) => child.next_close_release_byte_demand(),
+            None => Ok(0),
+        }
+    }
     fn terminal_is_empty(&self) -> bool { self.closing && self.child.is_none() && self.source.is_none() }
 }
 
```

ExactGUI275001–002/plannedreceipt 🗑️generated/current-native-ui-layout-10/green/admission.json. Source22methodimmutable; freshchoice2 maybinddataafteractualcontrols/currentLowadmission. NoRootwrite/compiler.
