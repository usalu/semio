# Current UI Eight Source Authorities

3 completecaptured14initial/currentauthority before-after body/hash transitions refreshed. Same166040descriptorfullpair/alloriginal assertions/budgets/caps/owners conserved. Full currentdeclaredprovider bodies are bound; no exhaustivehistoricalABI attribution orphysicalsize acceptance inferred. Freshoriginalwholemandatory.

```diff
--- UI7-🧰️framework/🔨️modules/🌱️value/📦️paged/🎮️append/🦀️.rs
+++ UI8-🧰️framework/🔨️modules/🌱️value/📦️paged/🎮️append/🦀️.rs
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
--- UI7-🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs
+++ UI8-🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs
@@ -119,6 +119,7 @@
     fn terminal_is_empty(&self) -> bool {
         self.0.is_empty()
     }
+    fn next_close_byte_demand(&self) -> Option<usize> { Some(usize::from(!self.0.is_empty())) }
     fn next_birth_bytes(&self, _: usize) -> Option<usize> { Some(0) }
     fn terminal_release_bytes(&self) -> Option<usize> { size_of::<Self>().checked_add(self.0.capacity()) }
 }
```
```diff
--- UI7-🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs
+++ UI8-🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs
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

ExactGUI272001–002/plannedreceipt 🗑️generated/current-native-ui-layout-8/green/admission.json. Source22methodready, actualcontrols next; noRootwrite/compiler.
